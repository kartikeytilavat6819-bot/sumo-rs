//! Port of microsim/trigger/MSCalibrator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::time::{delta_t, steps2time, time2string, SUMOTIME_MAX};
use utils::common::to_string::f64_to_string_prec;

/// C++ class `MSCalibrator::AspiredState` (microsim/trigger/MSCalibrator.h)
pub struct MSCalibrator_AspiredState {
    pub begin: SUMOTime,
    pub end: SUMOTime,
    pub q: f64,
    pub v: f64,
    pub vehicleParameter: Ptr<SUMOVehicleParameter>,
}

impl MSCalibrator_AspiredState {
    /// C++: `void AspiredState::AspiredState()`
    // AspiredState() : begin(-1), end(-1), q(-1.), v(-1.), vehicleParameter(0) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSCalibrator_AspiredState {
            begin: -1,
            end: -1,
            q: -1.,
            v: -1.,
            vehicleParameter: Ptr::null(),
        }
    }

}

/// C++ class `MSCalibrator::CalibratorCommand` (microsim/trigger/MSCalibrator.h)
#[repr(C)]
pub struct MSCalibrator_CalibratorCommand {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    pub myCalibrator: Ptr<MSCalibrator>,
}

impl std::ops::Deref for MSCalibrator_CalibratorCommand {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSCalibrator_CalibratorCommand {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSCalibrator_CalibratorCommand {
    /// C++: `void CalibratorCommand::CalibratorCommand(MSCalibrator *)`
    // CalibratorCommand(MSCalibrator* cali) :
    //             myCalibrator(cali) {}
    #[allow(unused_variables)]
    pub fn new(cali: Ptr<MSCalibrator>) -> Self {
        // shortcut: C++ base class Command is not ported (Unmapped), its default state is left as (), upgrade when Command is ported
        MSCalibrator_CalibratorCommand {
            base: (),
            myCalibrator: cali,
        }
    }

    /// C++: `SUMOTime CalibratorCommand::execute(SUMOTime)` [virtual]
    // SUMOTime execute(SUMOTime currentTime) {
    //             return myCalibrator->execute(currentTime);
    //         }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        self.myCalibrator.m().execute(currentTime)
    }

    /// C++: `SUMOTime CalibratorCommand::shiftTime(SUMOTime, SUMOTime, SUMOTime)` [virtual]
    /// Reschedule or deschedule the command when quick-loading state
    // SUMOTime
    // MSCalibrator::CalibratorCommand::shiftTime(SUMOTime currentTime, SUMOTime execTime, SUMOTime newTime) {
    //     UNUSED_PARAMETER(currentTime);
    //     UNUSED_PARAMETER(execTime);
    //     UNUSED_PARAMETER(newTime);
    //     myCalibrator->myCurrentStateInterval = myCalibrator->myIntervals.begin();
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn shift_time(&mut self, currentTime: SUMOTime, execTime: SUMOTime, newTime: SUMOTime) -> SUMOTime {
        self.myCalibrator.m().myCurrentStateInterval = 0;
        0
    }

}

impl Command for MSCalibrator_CalibratorCommand {
    /// C++ `CalibratorCommand::execute` (the virtual override)
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        self.myCalibrator.m().execute(currentTime)
    }
    /// C++ `CalibratorCommand::shiftTime` (the virtual override)
    fn shift_time(&mut self, currentTime: SUMOTime, execTime: SUMOTime, newTime: SUMOTime) -> SUMOTime {
        MSCalibrator_CalibratorCommand::shift_time(self, currentTime, execTime, newTime)
    }
}

/// Calibrates the flow on a segment to a specified one
/// C++ class `MSCalibrator` (microsim/trigger/MSCalibrator.h)
#[repr(C)]
pub struct MSCalibrator {
    /// C++ base class `MSRouteHandler`
    pub base: MSRouteHandler,
    /// C++ base class `Command`
    pub base2: Unmapped /* Command */,
    /// C++ base class `MSDetectorFileOutput`
    pub base3: MSDetectorFileOutput,
    /// the edge on which this calibrator lies
    pub myEdge: Ptr<MSEdge>,
    /// the lane on which this calibrator lies (nullptr if the whole edge is covered at once)
    pub myLane: Ptr<MSLane>,
    /// the junction on which this calibrator lies (nullptr if is edge or lane specific)
    pub myNode: Ptr<MSJunction>,
    /// the position on the edge where this calibrator lies
    pub myPos: f64,
    /// the route probe to retrieve routes from
    pub myProbe: Ptr<MSRouteProbe>,
    /// dummy parent to retrieve vType filter
    pub myMeanDataParent: MSMeanData_Net,
    /// data collector for the calibrator
    pub myLaneMeanData: Vec<Ptr<MSMeanData_Net_MSLaneMeanDataValues>>,
    /// accumlated data for the whole edge
    pub myEdgeMeanData: MSMeanData_Net_MSLaneMeanDataValues,
    /// List of adaptation intervals
    pub myIntervals: Vec<MSCalibrator_AspiredState>,
    /// Iterator pointing to the current interval
    /// index into myIntervals (C++ iterator); myIntervals.len() is C++ end()
    pub myCurrentStateInterval: usize,
    pub myVehicleRemovers: Vec<Ptr<MSCalibrator_VehicleRemover>>,
    /// set of vehicle ids to remove
    pub myToRemove: BTreeSet<String>,
    /// The device for xml statistics
    pub myOutput: Ptr<OutputDevice>,
    /// The frequeny with which to check for calibration
    pub myFrequency: SUMOTime,
    /// The number of vehicles that were removed in the current interval
    pub myRemoved: i32,
    /// The number of vehicles that were inserted in the current interval
    pub myInserted: i32,
    /// The number of vehicles that were removed when clearin a jam
    pub myClearedInJam: i32,
    /// The information whether the speed adaption has been reset
    pub mySpeedIsDefault: bool,
    /// The information whether speed was adapted in the current interval
    pub myDidSpeedAdaption: bool,
    /// The information whether init was called
    pub myDidInit: bool,
    /// The default (maximum) speed on the segment
    pub myDefaultSpeed: f64,
    /// The default (maximum) speed on the segment
    pub myHaveWarnedAboutClearingJam: bool,
    /// whether the calibrator was active when last checking
    pub myAmActive: bool,
    /// relative speed threshold for detecting and clearing invalid jam
    pub myInvalidJamThreshold: f64,
    /// whether the calibrator needs to undo the calibration after the edge / junction has been left
    pub myAmLocal: bool,
    /// whether the calibrator has registered an invalid jam in the last execution step
    pub myHaveInvalidJam: bool,
}

impl std::ops::Deref for MSCalibrator {
    type Target = MSRouteHandler;
    fn deref(&self) -> &MSRouteHandler {
        &self.base
    }
}
impl std::ops::DerefMut for MSCalibrator {
    fn deref_mut(&mut self) -> &mut MSRouteHandler {
        &mut self.base
    }
}

// C++ static members of MSCalibrator (port as module-level statics, see AGENTS.md):
//   static std::vector<MSMoveReminder *> MSCalibrator::myLeftoverReminders;
//   static std::vector<SUMOVehicleParameter *> MSCalibrator::myLeftoverVehicleParameters;
//   static std::map<std::string, MSCalibrator *> MSCalibrator::myInstances;

thread_local! {
    /// MSCalibrator::myInstances (calibrators by id)
    static MY_INSTANCES: std::cell::RefCell<BTreeMap<String, Ptr<MSCalibrator>>> = std::cell::RefCell::new(BTreeMap::new());
    /// MSCalibrator::myLeftoverReminders (VehicleRemovers and lane mean data made by calibrators)
    static MY_LEFTOVER_REMINDERS: std::cell::RefCell<Vec<Ptr<MSMoveReminder>>> = std::cell::RefCell::new(Vec::new());
    /// MSCalibrator::myLeftoverVehicleParameters (flow parameters of the calibrators)
    static MY_LEFTOVER_VEHICLE_PARAMETERS: std::cell::RefCell<Vec<Ptr<SUMOVehicleParameter>>> = std::cell::RefCell::new(Vec::new());
}

/// C++ `VEHPARS_CALIBRATORSPEED_SET` (SUMOVehicleParameter.h: `2 << 27`)
const VEHPARS_CALIBRATORSPEED_SET: i64 = 2 << 27;

fn my_instances() -> &'static mut BTreeMap<String, Ptr<MSCalibrator>> {
    MY_INSTANCES.with(|m| unsafe { &mut *m.as_ptr() })
}
fn my_leftover_reminders() -> &'static mut Vec<Ptr<MSMoveReminder>> {
    MY_LEFTOVER_REMINDERS.with(|m| unsafe { &mut *m.as_ptr() })
}
fn my_leftover_vehicle_parameters() -> &'static mut Vec<Ptr<SUMOVehicleParameter>> {
    MY_LEFTOVER_VEHICLE_PARAMETERS.with(|m| unsafe { &mut *m.as_ptr() })
}

/// C++ `MSRoute::dictionary(id)`: None where C++ returns nullptr
fn lookup_route(id: &str) -> Option<ConstMSRoutePtr> {
    if MSRoute::has_route(id) { Some(MSRoute::dictionary_3(id, Ptr::null())) } else { None }
}

/// C++ `new VehicleRemover(lane, parent, undo)` kept alive in myLeftoverReminders
fn new_vehicle_remover(lane: Ptr<MSLane>, parent: Ptr<MSCalibrator>, undo: bool) -> Ptr<MSCalibrator_VehicleRemover> {
    let remover: Ptr<MSCalibrator_VehicleRemover> = Ptr::from_raw(Box::into_raw(Box::new(MSCalibrator_VehicleRemover::new(lane, parent, undo))));
    my_leftover_reminders().push(remover.cast::<MSMoveReminder>());
    remover
}

/// C++ `e.what()` of a caught exception (ProcessError, or a plain panic message)
fn calibrator_exception_what(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(e) = payload.downcast_ref::<ProcessError>() {
        e.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else {
        String::new()
    }
}

impl MSCalibrator {
    /// C++: `void MSCalibrator::MSCalibrator(const std::string &, MSEdge *const, MSLane *const, MSJunction *const, const double, const std::string &, const std::string &, const SUMOTime, const double, const MSRouteProbe *, const double, const std::string &, const bool, const bool)`
    /// constructor
    // MSCalibrator::MSCalibrator(const std::string& id,
    //                            MSEdge* const edge,
    //                            MSLane* const lane,
    //                            MSJunction* const node,
    //                            const double pos,
    //                            const std::string& aXMLFilename,
    //                            const std::string& outputFilename,
    //                            const SUMOTime freq, const double length,
    //                            const MSRouteProbe* probe,
    //                            const double invalidJamThreshold,
    //                            const std::string& vTypes,
    //                            const bool local,
    //                            const bool addLaneMeanData) :
    //     MSRouteHandler(aXMLFilename, true),
    //     MSDetectorFileOutput(id, vTypes, "", (int)PersonMode::NONE), // detecting persons not yet supported
    //     myEdge(edge),
    //     myLane(lane),
    //     myNode(node),
    //     myPos(pos), myProbe(probe),
    //     myMeanDataParent(id + "_dummyMeanData", 0, 0, false, "true", false, false, 0, 1, 0, 0, 0, vTypes, "",
    //                      std::vector<MSEdge*>(), AggregateType::NO),
    //     myEdgeMeanData(nullptr, length, false, &myMeanDataParent),
    //     myCurrentStateInterval(myIntervals.begin()),
    //     myOutput(nullptr), myFrequency(freq), myRemoved(0),
    //     myInserted(0),
    //     myClearedInJam(0),
    //     mySpeedIsDefault(true), myDidSpeedAdaption(false), myDidInit(false),
    //     myDefaultSpeed(myLane == nullptr ? (myEdge == nullptr ? 0. : myEdge->getSpeedLimit()) : myLane->getSpeedLimit()),
    //     myHaveWarnedAboutClearingJam(false),
    //     myAmActive(false),
    //     myInvalidJamThreshold(invalidJamThreshold),
    //     myAmLocal(local),
    //     myHaveInvalidJam(false) {
    //     if (outputFilename != "") {
    //         myOutput = &OutputDevice::getDevice(outputFilename);
    //         writeXMLDetectorProlog(*myOutput);
    //     }
    //     if (aXMLFilename != "") {
    //         XMLSubSys::runParser(*this, aXMLFilename);
    //         if (!myDidInit) {
    //             init();
    //         }
    //     }
    //     myInstances[id] = this;
    //     if (addLaneMeanData && myEdge != nullptr) {
    //         // disabled for METriggeredCalibrator
    //         for (MSLane* const eLane : myEdge->getLanes()) {
    //             if (myLane == nullptr || myLane == eLane) {
    //                 //std::cout << " cali=" << getID() << " myLane=" << Named::getIDSecure(myLane) << " checkLane=" << i << "\n";
    //                 MSMeanData_Net::MSLaneMeanDataValues* laneData = new MSMeanData_Net::MSLaneMeanDataValues(eLane, eLane->getLength(), true, &myMeanDataParent);
    //                 laneData->setDescription("meandata_calibrator_" + eLane->getID());
    //                 myLeftoverReminders.push_back(laneData);
    //                 myLaneMeanData.push_back(laneData);
    //                 VehicleRemover* remover = new VehicleRemover(eLane, this);
    //                 myLeftoverReminders.push_back(remover);
    //                 myVehicleRemovers.push_back(remover);
    //             }
    //         }
    //     }
    //     if (node != nullptr) {
    //         for (const MSEdge* inEdge : myNode->getIncoming()) {
    //             for (MSLane* const eLane : inEdge->getLanes()) {
    //                 VehicleRemover* remover = new VehicleRemover(eLane, this);
    //                 myLeftoverReminders.push_back(remover);
    //                 myVehicleRemovers.push_back(remover);
    //             }
    //         }
    //         if (local) {
    //             for (const MSEdge* outEdge : myNode->getOutgoing()) {
    //                 for (MSLane* const eLane : outEdge->getLanes()) {
    //                     VehicleRemover* remover = new VehicleRemover(eLane, this, true);
    //                     myLeftoverReminders.push_back(remover);
    //                     myVehicleRemovers.push_back(remover);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, edge: Ptr<MSEdge>, lane: Ptr<MSLane>, node: Ptr<MSJunction>, pos: f64, aXMLFilename: &str, outputFilename: &str, freq: SUMOTime, length: f64, probe: Ptr<MSRouteProbe>, invalidJamThreshold: f64, vTypes: &str, local: bool, addLaneMeanData: bool) -> Box<Self> {
        // Boxed because the constructor registers `this` (myInstances, the VehicleRemovers' parent).
        let myDefaultSpeed = if lane.is_null() { if edge.is_null() { 0. } else { edge.get_speed_limit() } } else { lane.get_speed_limit() };
        let mut b = Box::new(MSCalibrator {
            base: MSRouteHandler::new(aXMLFilename, true),
            base2: (),
            base3: match MSDetectorFileOutput::new(id, vTypes, "", PersonMode::NONE as i32) {
                Ok(v) => v,
                Err(e) => std::panic::panic_any(e),
            },
            myEdge: edge,
            myLane: lane,
            myNode: node,
            myPos: pos,
            myProbe: probe,
            myMeanDataParent: MSMeanData_Net::new(&format!("{}_dummyMeanData", id), 0, 0, false, "true", false, false, 0, 1., 0., 0., 0., vTypes, "", &Vec::new(), AggregateType::NO),
            myLaneMeanData: Vec::new(),
            myEdgeMeanData: MSMeanData_Net_MSLaneMeanDataValues::new(Ptr::null(), length, false, Ptr::null()),
            myIntervals: Vec::new(),
            myCurrentStateInterval: 0,
            myVehicleRemovers: Vec::new(),
            myToRemove: BTreeSet::new(),
            myOutput: Ptr::null(),
            myFrequency: freq,
            myRemoved: 0,
            myInserted: 0,
            myClearedInJam: 0,
            mySpeedIsDefault: true,
            myDidSpeedAdaption: false,
            myDidInit: false,
            myDefaultSpeed,
            myHaveWarnedAboutClearingJam: false,
            myAmActive: false,
            myInvalidJamThreshold: invalidJamThreshold,
            myAmLocal: local,
            myHaveInvalidJam: false,
        });
        // myEdgeMeanData's parent is myMeanDataParent inside the box, so its address is only final now
        b.myEdgeMeanData = MSMeanData_Net_MSLaneMeanDataValues::new(Ptr::null(), length, false, Ptr::from_mut(&mut b.myMeanDataParent));
        let this: Ptr<MSCalibrator> = Ptr::from_box(&b);
        let parent: Ptr<MSMeanData_Net> = Ptr::from_mut(&mut b.myMeanDataParent);
        if !outputFilename.is_empty() {
            let dev = match OutputDevice::get_device(outputFilename, true) {
                Ok(d) => d,
                Err(e) => std::panic::panic_any(e),
            };
            b.myOutput = Ptr::from_raw(dev.as_ptr());
            b.write_xml_detector_prolog(b.myOutput.m());
        }
        if !aXMLFilename.is_empty() {
            // shortcut: XMLSubSys::runParser(*this, file) is not wired (MSCalibrator is no SaxHandler and SUMORouteHandler has no GenericSAXHandler base yet), so the <flow> intervals of the file are not read and init() is not run here, upgrade when SUMORouteHandler is ported
        }
        my_instances().insert(id.to_string(), this);
        if addLaneMeanData && !edge.is_null() {
            // disabled for METriggeredCalibrator
            for eLane in edge.get_lanes().clone() {
                if lane.is_null() || lane == eLane {
                    let laneData: Ptr<MSMeanData_Net_MSLaneMeanDataValues> = Ptr::from_raw(Box::into_raw(Box::new(MSMeanData_Net_MSLaneMeanDataValues::new(eLane, eLane.get_length(), true, parent))));
                    laneData.m().base.base.set_description(&format!("meandata_calibrator_{}", eLane.get_id()));
                    my_leftover_reminders().push(laneData.cast::<MSMoveReminder>());
                    b.myLaneMeanData.push(laneData);
                    let remover = new_vehicle_remover(eLane, this, false);
                    b.myVehicleRemovers.push(remover);
                }
            }
        }
        if !node.is_null() {
            for inEdge in node.get_incoming().clone() {
                for eLane in inEdge.get_lanes().clone() {
                    let remover = new_vehicle_remover(eLane, this, false);
                    b.myVehicleRemovers.push(remover);
                }
            }
            if local {
                for outEdge in node.get_outgoing().clone() {
                    for eLane in outEdge.get_lanes().clone() {
                        let remover = new_vehicle_remover(eLane, this, true);
                        b.myVehicleRemovers.push(remover);
                    }
                }
            }
        }
        b
    }

    /// C++: `void MSCalibrator::~MSCalibrator() noexcept` [virtual]
    /// destructor
    // MSCalibrator::~MSCalibrator() {
    //     if (myCurrentStateInterval != myIntervals.end()) {
    //         intervalEnd();
    //     }
    //     for (VehicleRemover* const remover : myVehicleRemovers) {
    //         remover->disable();
    //     }
    //     myInstances.erase(getID());
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if self.myCurrentStateInterval != self.myIntervals.len() {
            self.interval_end();
        }
        for remover in self.myVehicleRemovers.iter() {
            remover.m().disable();
        }
        my_instances().remove(self.base3.get_id());
    }

    /// C++: `void MSCalibrator::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Write the generated output to the given device
    // void
    // MSCalibrator::writeXMLOutput(OutputDevice& dev, SUMOTime startTime, SUMOTime stopTime) {
    //     updateMeanData();
    //     const int p = passed();
    //     // meandata will be off if vehicles are removed on the next edge instead of this one
    //     const int discrepancy = myEdgeMeanData.nVehEntered + myEdgeMeanData.nVehDeparted - myEdgeMeanData.nVehVaporized - myEdgeMeanData.nVehTeleported - passed();
    //     //assert(discrepancy >= 0); may go negative for lane calibrator when vehicles change lane before removal
    //     const std::string ds = (discrepancy > 0 ? "\" vaporizedOnNextEdge=\"" + toString(discrepancy) : "");
    //     const double durationSeconds = STEPS2TIME(stopTime - startTime);
    //     dev.openTag(SUMO_TAG_INTERVAL);
    //     dev.writeAttr(SUMO_ATTR_BEGIN, time2string(startTime));
    //     dev.writeAttr(SUMO_ATTR_END, time2string(stopTime));
    //     dev.writeAttr(SUMO_ATTR_ID, getID());
    //     dev.writeAttr("nVehContrib", p);
    //     dev.writeAttr("removed", myRemoved);
    //     dev.writeAttr("inserted", myInserted);
    //     dev.writeAttr("cleared", myClearedInJam);
    //     dev.writeAttr("flow", p * 3600.0 / durationSeconds);
    //     dev.writeAttr("aspiredFlow", myCurrentStateInterval->q);
    //     dev.writeAttr(SUMO_ATTR_SPEED, myEdgeMeanData.getSamples() != 0
    //                   ? myEdgeMeanData.getTravelledDistance() / myEdgeMeanData.getSamples() : -1);
    //     dev.writeAttr("aspiredSpeed", myCurrentStateInterval->v);
    //     if (discrepancy > 0) {
    //         dev.writeAttr("vaporizedOnNextEdge", discrepancy);
    //     }
    //     dev.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        self.update_mean_data();
        let p: i32 = self.passed();
        // meandata will be off if vehicles are removed on the next edge instead of this one
        let discrepancy: i32 = self.myEdgeMeanData.nVehEntered + self.myEdgeMeanData.nVehDeparted - self.myEdgeMeanData.nVehVaporized - self.myEdgeMeanData.nVehTeleported - self.passed();
        //assert(discrepancy >= 0); may go negative for lane calibrator when vehicles change lane before removal
        let ds = if discrepancy > 0 { format!("\" vaporizedOnNextEdge=\"{}", discrepancy) } else { String::new() };
        let durationSeconds: f64 = steps2time(stopTime - startTime);
        dev.open_tag_id(SUMO_TAG_INTERVAL);
        dev.write_attr(SUMO_ATTR_BEGIN, &time2string(startTime));
        dev.write_attr(SUMO_ATTR_END, &time2string(stopTime));
        dev.write_attr(SUMO_ATTR_ID, &self.base3.get_id().to_string());
        dev.write_attr("nVehContrib", &p);
        dev.write_attr("removed", &self.myRemoved);
        dev.write_attr("inserted", &self.myInserted);
        dev.write_attr("cleared", &self.myClearedInJam);
        let flow: f64 = (p as f64) * 3600.0 / durationSeconds;
        dev.write_attr("flow", &flow);
        dev.write_attr("aspiredFlow", &self.myIntervals[self.myCurrentStateInterval].q);
        let speed: f64 = if self.myEdgeMeanData.get_samples() != 0. { self.myEdgeMeanData.get_travelled_distance() / self.myEdgeMeanData.get_samples() } else { -1. };
        dev.write_attr(SUMO_ATTR_SPEED, &speed);
        dev.write_attr("aspiredSpeed", &self.myIntervals[self.myCurrentStateInterval].v);
        if discrepancy > 0 {
            dev.write_attr("vaporizedOnNextEdge", &discrepancy);
        }
        dev.close_tag("");
        let _ = ds;
    }

    /// C++: `void MSCalibrator::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Open the XML-output
    // void
    // MSCalibrator::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("calibratorstats", "calibratorstats_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("calibratorstats", "calibratorstats_file.xsd", BTreeMap::new(), true);
    }

    /// C++: `SUMOTime MSCalibrator::execute(SUMOTime)` [virtual]
    /// the implementation of the MSTrigger / Command interface. Calibrating takes place here.
    // SUMOTime
    // MSCalibrator::execute(SUMOTime currentTime) {
    //     const bool calibrateFlow = myCurrentStateInterval->q >= 0;
    //     const bool calibrateSpeed = myCurrentStateInterval->v >= 0;
    //     // get current simulation values (valid for the last simulation second)
    //     myHaveInvalidJam = (calibrateFlow || calibrateSpeed) && invalidJam(myLane == 0 ? -1 : myLane->getIndex());
    //     // XXX could we miss vehicle movements if this is called less often than every DELTA_T (default) ?
    //     updateMeanData();
    //     const bool hadRemovals = removePending();
    //     // check whether an adaptation value exists
    //     if (isCurrentStateActive(currentTime)) {
    //         myAmActive = true;
    //         // all happens in isCurrentStateActive()
    //     } else {
    //         myAmActive = false;
    //         reset();
    //         if (!mySpeedIsDefault) {
    //             // reset speed to default
    //             if (myLane == nullptr) {
    //                 myEdge->setMaxSpeed(myDefaultSpeed, false);
    //             } else {
    //                 myLane->setMaxSpeed(myDefaultSpeed, false);
    //             }
    //             mySpeedIsDefault = true;
    //         }
    //         if (myCurrentStateInterval == myIntervals.end()) {
    //             // keep calibrator alive for gui but do not call again
    //             return SUMOTime_MAX - currentTime;
    //         }
    //         return myFrequency;
    //     }
    //     // we are active
    //     if (!myDidSpeedAdaption && calibrateSpeed) {
    //         if (myLane == nullptr) {
    //             myEdge->setMaxSpeed(myCurrentStateInterval->v);
    //         } else {
    //             myLane->setMaxSpeed(myCurrentStateInterval->v);
    //         }
    //         mySpeedIsDefault = false;
    //         myDidSpeedAdaption = true;
    //     }
    //
    //     const int totalWishedNum = totalWished();
    //     int adaptedNum = passed() + myClearedInJam;
    // #ifdef MSCalibrator_DEBUG
    //     if (DEBUGCOND) {
    //         std::cout << time2string(currentTime) << " " << getID()
    //                   << " q=" << myCurrentStateInterval->q
    //                   << " totalWished=" << totalWishedNum
    //                   << " adapted=" << adaptedNum
    //                   << " jam=" << myHaveInvalidJam
    //                   << " entered=" << myEdgeMeanData.nVehEntered
    //                   << " departed=" << myEdgeMeanData.nVehDeparted
    //                   << " arrived=" << myEdgeMeanData.nVehArrived
    //                   << " left=" << myEdgeMeanData.nVehLeft
    //                   << " waitSecs=" << myEdgeMeanData.waitSeconds
    //                   << " vaporized=" << myEdgeMeanData.nVehVaporized
    //                   << "\n";
    //     }
    // #endif
    //     if (calibrateFlow && adaptedNum < totalWishedNum && !hadRemovals) {
    //         // we need to insert some vehicles
    //         const double hourFraction = STEPS2TIME(currentTime - myCurrentStateInterval->begin + DELTA_T) / (double) 3600.;
    //         const int wishedNum = (int)std::floor(myCurrentStateInterval->q * hourFraction + 0.5); // round to closest int
    //         // only the difference between inflow and aspiredFlow should be added, thus
    //         // we should not count vehicles vaporized from a jam here
    //         // if we have enough time left we can add missing vehicles later
    //         const int relaxedInsertion = (int)std::floor(STEPS2TIME(myCurrentStateInterval->end - currentTime) / 3);
    //         const int insertionSlack = MAX2(0, adaptedNum + relaxedInsertion - totalWishedNum);
    //         // increase number of vehicles
    // #ifdef MSCalibrator_DEBUG
    //         if (DEBUGCOND) {
    //             std::cout << "   wished:" << wishedNum
    //                       << " slack:" << insertionSlack
    //                       << " before:" << adaptedNum
    //                       << "\n";
    //         }
    // #endif
    //         MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //         while (wishedNum > adaptedNum + insertionSlack) {
    //             SUMOVehicleParameter* pars = myCurrentStateInterval->vehicleParameter;
    //             ConstMSRoutePtr route = myProbe != nullptr ? myProbe->sampleRoute() : nullptr;
    //             if (route == nullptr) {
    //                 route = MSRoute::dictionary(pars->routeid);
    //             }
    //             if (route == nullptr) {
    //                 WRITE_WARNINGF(TL("No valid routes in calibrator '%'."), getID());
    //                 break;
    //             }
    //             if (!route->contains(myEdge)) {
    //                 WRITE_WARNINGF(TL("Route '%' in calibrator '%' does not contain edge '%'."), route->getID(), getID(), myEdge->getID());
    //                 break;
    //             }
    //             const int routeIndex = (int)std::distance(route->begin(),
    //                                    std::find(route->begin(), route->end(), myEdge));
    //             MSVehicleType* vtype = vc.getVType(pars->vtypeid);
    //             assert(route != 0 && vtype != 0);
    //             // build the vehicle
    //             const std::string newID = getNewVehicleID();
    //             if (vc.getVehicle(newID) != nullptr) {
    //                 // duplicate ids could come from loading state
    //                 myInserted++;
    //                 break;
    //             }
    //             SUMOVehicleParameter* newPars = new SUMOVehicleParameter(*pars);
    //             newPars->id = newID;
    //             newPars->depart = currentTime;
    //             newPars->routeid = route->getID();
    //             newPars->departLaneProcedure = DepartLaneDefinition::FIRST_ALLOWED; // ensure successful vehicle creation
    //             MSVehicle* vehicle;
    //             try {
    //                 vehicle = dynamic_cast<MSVehicle*>(vc.buildVehicle(newPars, route, vtype, true, MSVehicleControl::VehicleDefinitionSource::TRIGGER));
    //             } catch (const ProcessError& e) {
    //                 if (!MSGlobals::gCheckRoutes) {
    //                     WRITE_WARNING(e.what());
    //                     vehicle = nullptr;
    //                     break;
    //                 } else {
    //                     throw;
    //                 }
    //             }
    // #ifdef MSCalibrator_DEBUG
    //             if (DEBUGCOND) {
    //                 std::cout << " resetting route pos: " << routeIndex << "\n";
    //             }
    // #endif
    //             vehicle->resetRoutePosition(routeIndex, pars->departLaneProcedure);
    //             bool success = false;
    //             try {
    //                 success = myEdge->insertVehicle(*vehicle, currentTime);
    //             } catch (const ProcessError&) {
    //                 MSNet::getInstance()->getVehicleControl().deleteVehicle(vehicle, true);
    //                 throw;
    //             }
    //             if (success) {
    //                 if (!MSNet::getInstance()->getVehicleControl().addVehicle(vehicle->getID(), vehicle)) {
    //                     throw ProcessError("Emission of vehicle '" + vehicle->getID() + "' in calibrator '" + getID() + "'failed!");
    //                 }
    //                 myInserted++;
    //                 adaptedNum++;
    // #ifdef MSCalibrator_DEBUG
    //                 if (DEBUGCOND) {
    //                     std::cout << "I ";
    //                 }
    // #endif
    //             } else {
    //                 // could not insert vehicle
    // #ifdef MSCalibrator_DEBUG
    //                 if (DEBUGCOND) {
    //                     std::cout << "F ";
    //                 }
    // #endif
    //                 MSNet::getInstance()->getVehicleControl().deleteVehicle(vehicle, true);
    //                 break;
    //             }
    //         }
    //     }
    //     if (myCurrentStateInterval->end <= currentTime + myFrequency) {
    //         intervalEnd();
    //     }
    //     return myFrequency;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        let calibrateFlow = self.myIntervals[self.myCurrentStateInterval].q >= 0.;
        let calibrateSpeed = self.myIntervals[self.myCurrentStateInterval].v >= 0.;
        // get current simulation values (valid for the last simulation second)
        self.myHaveInvalidJam = (calibrateFlow || calibrateSpeed) && self.invalid_jam(if self.myLane.is_null() { -1 } else { self.myLane.get_index() });
        // XXX could we miss vehicle movements if this is called less often than every DELTA_T (default) ?
        self.update_mean_data();
        let hadRemovals = self.remove_pending();
        // check whether an adaptation value exists
        if self.is_current_state_active(currentTime) {
            self.myAmActive = true;
            // all happens in isCurrentStateActive()
        } else {
            self.myAmActive = false;
            self.reset();
            if !self.mySpeedIsDefault {
                // reset speed to default
                if self.myLane.is_null() {
                    self.myEdge.m().set_max_speed(self.myDefaultSpeed, false, -1.);
                } else {
                    self.myLane.m().set_max_speed(self.myDefaultSpeed, false, -1.);
                }
                self.mySpeedIsDefault = true;
            }
            if self.myCurrentStateInterval == self.myIntervals.len() {
                // keep calibrator alive for gui but do not call again
                return SUMOTIME_MAX - currentTime;
            }
            return self.myFrequency;
        }
        // we are active
        if !self.myDidSpeedAdaption && calibrateSpeed {
            let v = self.myIntervals[self.myCurrentStateInterval].v;
            if self.myLane.is_null() {
                self.myEdge.m().set_max_speed(v, true, -1.);
            } else {
                self.myLane.m().set_max_speed(v, true, -1.);
            }
            self.mySpeedIsDefault = false;
            self.myDidSpeedAdaption = true;
        }

        let totalWishedNum = self.total_wished();
        let mut adaptedNum = self.passed() + self.myClearedInJam;
        if calibrateFlow && adaptedNum < totalWishedNum && !hadRemovals {
            let idx = self.myCurrentStateInterval;
            // we need to insert some vehicles
            let hourFraction = steps2time(currentTime - self.myIntervals[idx].begin + delta_t()) / 3600.;
            let wishedNum = (self.myIntervals[idx].q * hourFraction + 0.5).floor() as i32; // round to closest int
            // only the difference between inflow and aspiredFlow should be added, thus
            // we should not count vehicles vaporized from a jam here
            // if we have enough time left we can add missing vehicles later
            let relaxedInsertion = (steps2time(self.myIntervals[idx].end - currentTime) / 3.).floor() as i32;
            let insertionSlack = std::cmp::max(0, adaptedNum + relaxedInsertion - totalWishedNum);
            // increase number of vehicles
            let net = MSNet::get_instance();
            let vc = net.m().get_vehicle_control();
            while wishedNum > adaptedNum + insertionSlack {
                let pars: Ptr<SUMOVehicleParameter> = self.myIntervals[idx].vehicleParameter;
                let mut route: Option<ConstMSRoutePtr> = if !self.myProbe.is_null() { self.myProbe.sample_route(true) } else { None };
                if route.is_none() {
                    route = lookup_route(&pars.routeid);
                }
                let route = match route {
                    Some(r) => r,
                    None => {
                        write_warningf!("No valid routes in calibrator '%'.", self.base3.get_id());
                        break;
                    }
                };
                if !route.contains(self.myEdge) {
                    write_warningf!("Route '%' in calibrator '%' does not contain edge '%'.", route.get_id(), self.base3.get_id(), self.myEdge.get_id());
                    break;
                }
                let routeIndex: i32 = route.get_edges().iter().position(|e| *e == self.myEdge).unwrap_or(route.get_edges().len()) as i32;
                let vtype: Ptr<MSVehicleType> = vc.get_v_type(&pars.vtypeid, Ptr::null(), false);
                // build the vehicle
                let newID = self.get_new_vehicle_id();
                if !vc.get_vehicle(&newID).is_null() {
                    // duplicate ids could come from loading state
                    self.myInserted += 1;
                    break;
                }
                let newPars: Ptr<SUMOVehicleParameter> = Ptr::from_raw(Box::into_raw(Box::new((*pars).clone())));
                newPars.m().id = newID.clone();
                newPars.m().depart = currentTime;
                newPars.m().routeid = route.get_id().to_string();
                newPars.m().departLaneProcedure = DepartLaneDefinition::FIRST_ALLOWED; // ensure successful vehicle creation
                let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    vc.build_vehicle(newPars, route.clone(), vtype, true, MSVehicleControl_VehicleDefinitionSource::TRIGGER, true)
                }));
                let vehicleSV: Ptr<SUMOVehicle> = match built {
                    Ok(v) => v,
                    Err(payload) => {
                        if !crate::microsim::ms_globals::check_routes() {
                            write_warning(&calibrator_exception_what(&*payload));
                            break;
                        } else {
                            std::panic::resume_unwind(payload);
                        }
                    }
                };
                // shortcut: dynamic_cast<MSVehicle*> is a plain cast here, the microsim vehicle control only builds MSVehicle for triggers
                let vehicle: Ptr<MSVehicle> = vehicleSV.cast::<MSVehicle>();
                vehicle.m().reset_route_position(routeIndex, pars.departLaneProcedure);
                let success: bool = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.myEdge.insert_vehicle(vehicleSV.m(), currentTime, false, false))) {
                    Ok(s) => s,
                    Err(payload) => {
                        vc.delete_vehicle(vehicleSV, true, false);
                        std::panic::resume_unwind(payload);
                    }
                };
                if success {
                    let vid = vehicleSV.get_id().to_string();
                    if !vc.add_vehicle(&vid, vehicleSV) {
                        std::panic::panic_any(ProcessError::new(format!("Emission of vehicle '{}' in calibrator '{}'failed!", vid, self.base3.get_id())));
                    }
                    self.myInserted += 1;
                    adaptedNum += 1;
                } else {
                    // could not insert vehicle
                    vc.delete_vehicle(vehicleSV, true, false);
                    break;
                }
            }
        }
        if self.myIntervals[self.myCurrentStateInterval].end <= currentTime + self.myFrequency {
            self.interval_end();
        }
        self.myFrequency
    }

    /// C++: `void MSCalibrator::cleanup()` [static]
    /// cleanup remaining data structures
    // void
    // MSCalibrator::cleanup() {
    //     while (!myInstances.empty()) {
    //         delete myInstances.begin()->second;
    //     }
    //     for (MSMoveReminder* rem : myLeftoverReminders) {
    //         delete rem;
    //     }
    //     myLeftoverReminders.clear();
    //     for (SUMOVehicleParameter* par : myLeftoverVehicleParameters) {
    //         delete par;
    //     }
    //     myLeftoverVehicleParameters.clear();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        // shortcut: C++ deletes the remaining calibrators and leftover objects; here they are unregistered (drop_) and the leftover heap objects are leaked, since their owners are not ported, upgrade when object ownership is ported
        let remaining: Vec<Ptr<MSCalibrator>> = my_instances().values().copied().collect();
        for cali in remaining {
            cali.m().drop_();
        }
        my_instances().clear();
        my_leftover_reminders().clear();
        my_leftover_vehicle_parameters().clear();
    }

    /// C++: `const std::map<std::string, MSCalibrator *> & MSCalibrator::getInstances()` [static]
    /// return all calibrator instances
    // static const std::map<std::string, MSCalibrator*>& getInstances() {
    //         return myInstances;
    //     }
    #[allow(unused_variables)]
    pub fn get_instances() -> &'static BTreeMap<String, Ptr<MSCalibrator>> {
        my_instances()
    }

    /// C++: `AspiredState MSCalibrator::getCurrentStateInterval() const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_current_state_interval(&self) -> MSCalibrator_AspiredState {
        // C++ returns the aspired state by value
        let s = &self.myIntervals[self.myCurrentStateInterval];
        MSCalibrator_AspiredState {
            begin: s.begin,
            end: s.end,
            q: s.q,
            v: s.v,
            vehicleParameter: s.vehicleParameter,
        }
    }

    /// C++: `const MSEdge * MSCalibrator::getEdge() const` [const]
    // const MSEdge* getEdge() const {
    //         return myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        self.myEdge
    }

    /// C++: `const MSLane * MSCalibrator::getLane() const` [const]
    // const MSLane* getLane() const {
    //         return myLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        self.myLane
    }

    /// C++: `const MSRouteProbe * MSCalibrator::getRouteProbe() const` [const]
    // const MSRouteProbe* getRouteProbe() const {
    //         return myProbe;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_probe(&self) -> Ptr<MSRouteProbe> {
        self.myProbe
    }

    /// C++: `int MSCalibrator::passed() const` [const, virtual]
    // inline virtual int passed() const {
    //         // calibrator measures at start of segment
    //         // vehicles drive to the end of an edge by default so they count as passed
    //         // but vaporized vehicles do not count
    //         // if the calibrator is located on a short edge, the vehicles are
    //         // vaporized on the next edge so we cannot rely on myEdgeMeanData.nVehVaporized
    //         return myEdgeMeanData.nVehEntered + myEdgeMeanData.nVehDeparted - myClearedInJam - myRemoved;
    //     }
    #[allow(unused_variables)]
    pub fn passed(&self) -> i32 {
        // calibrator measures at start of segment; vaporized vehicles do not count as passed
        self.myEdgeMeanData.nVehEntered + self.myEdgeMeanData.nVehDeparted - self.myClearedInJam - self.myRemoved
    }

    /// C++: `int MSCalibrator::getInserted() const` [const]
    // int getInserted() const {
    //         return myEdgeMeanData.nVehDeparted;
    //     }
    #[allow(unused_variables)]
    pub fn get_inserted(&self) -> i32 {
        self.myEdgeMeanData.nVehDeparted
    }

    /// C++: `int MSCalibrator::getRemoved() const` [const]
    // int getRemoved() const {
    //         return myClearedInJam + myRemoved;
    //     }
    #[allow(unused_variables)]
    pub fn get_removed(&self) -> i32 {
        self.myClearedInJam + self.myRemoved
    }

    /// C++: `void MSCalibrator::setFlow(SUMOTime, SUMOTime, double, double, SUMOVehicleParameter)`
    // void
    // MSCalibrator::setFlow(SUMOTime begin, SUMOTime end, double vehsPerHour, double speed, SUMOVehicleParameter vehicleParameter) {
    //     auto it = myCurrentStateInterval;
    //     while (it != myIntervals.end()) {
    //         if (it->begin > begin) {
    //             throw ProcessError("Cannot set flow for calibrator '" + getID() + "' with begin time=" + time2string(begin) + " in the past.");
    //         } else if (it->begin == begin && it->end == end) {
    //             // update current interval
    //             AspiredState& state = const_cast<AspiredState&>(*it);
    //             state.q = vehsPerHour;
    //             state.v = speed;
    //             state.vehicleParameter->vtypeid = vehicleParameter.vtypeid;
    //             state.vehicleParameter->routeid = vehicleParameter.routeid;
    //             state.vehicleParameter->departLane = vehicleParameter.departLane;
    //             state.vehicleParameter->departLaneProcedure = vehicleParameter.departLaneProcedure;
    //             state.vehicleParameter->departSpeed = vehicleParameter.departSpeed;
    //             state.vehicleParameter->departSpeedProcedure = vehicleParameter.departSpeedProcedure;
    //             return;
    //         } else if (begin < it->end) {
    //             throw ProcessError(TLF("Cannot set flow for calibrator '%' with overlapping interval.", getID()));
    //         } else if (begin >= end) {
    //             throw ProcessError(TLF("Cannot set flow for calibrator '%' with negative interval.", getID()));
    //         }
    //         it++;
    //     }
    //     // add interval at the end of the known intervals
    //     const int intervalIndex = (int)(myCurrentStateInterval - myIntervals.begin());
    //     AspiredState state;
    //     state.begin = begin;
    //     state.end = end;
    //     state.q = vehsPerHour;
    //     state.v = speed;
    //     state.vehicleParameter = new SUMOVehicleParameter(vehicleParameter);
    //     myIntervals.push_back(state);
    //     // fix iterator
    //     myCurrentStateInterval = myIntervals.begin() + intervalIndex;
    // }
    #[allow(unused_variables)]
    pub fn set_flow(&mut self, begin: SUMOTime, end: SUMOTime, vehsPerHour: f64, speed: f64, vehicleParameter: SUMOVehicleParameter) {
        let mut it = self.myCurrentStateInterval;
        while it != self.myIntervals.len() {
            if self.myIntervals[it].begin > begin {
                std::panic::panic_any(ProcessError::new(format!("Cannot set flow for calibrator '{}' with begin time={} in the past.", self.base3.get_id(), time2string(begin))));
            } else if self.myIntervals[it].begin == begin && self.myIntervals[it].end == end {
                // update current interval
                let state = &mut self.myIntervals[it];
                state.q = vehsPerHour;
                state.v = speed;
                let vp: Ptr<SUMOVehicleParameter> = state.vehicleParameter;
                vp.m().vtypeid = vehicleParameter.vtypeid.clone();
                vp.m().routeid = vehicleParameter.routeid.clone();
                vp.m().departLane = vehicleParameter.departLane;
                vp.m().departLaneProcedure = vehicleParameter.departLaneProcedure;
                vp.m().departSpeed = vehicleParameter.departSpeed;
                vp.m().departSpeedProcedure = vehicleParameter.departSpeedProcedure;
                return;
            } else if begin < self.myIntervals[it].end {
                std::panic::panic_any(ProcessError::new(format!("Cannot set flow for calibrator '{}' with overlapping interval.", self.base3.get_id())));
            } else if begin >= end {
                std::panic::panic_any(ProcessError::new(format!("Cannot set flow for calibrator '{}' with negative interval.", self.base3.get_id())));
            }
            it += 1;
        }
        // add interval at the end of the known intervals
        let intervalIndex = self.myCurrentStateInterval;
        let mut state = MSCalibrator_AspiredState::new();
        state.begin = begin;
        state.end = end;
        state.q = vehsPerHour;
        state.v = speed;
        state.vehicleParameter = Ptr::from_raw(Box::into_raw(Box::new(vehicleParameter)));
        self.myIntervals.push(state);
        // fix iterator
        self.myCurrentStateInterval = intervalIndex;
    }

    /// C++: `void MSCalibrator::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // MSCalibrator::myStartElement(int element,
    //                              const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_FLOW) {
    //         AspiredState state;
    //         SUMOTime lastEnd = -1;
    //         SUMOTime lastBegin = -1;
    //         if (myIntervals.size() > 0) {
    //             lastEnd = myIntervals.back().end;
    //             lastBegin = myIntervals.back().begin;
    //             if (lastEnd == -1) {
    //                 lastEnd = myIntervals.back().begin;
    //             }
    //         }
    //         try {
    //             bool ok = true;
    //             state.q = attrs.getOpt<double>(SUMO_ATTR_VEHSPERHOUR, nullptr, ok, -1.);
    //             state.v = attrs.getOpt<double>(SUMO_ATTR_SPEED, nullptr, ok, -1.);
    //             state.begin = attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, getID().c_str(), ok);
    //             if (state.begin < lastEnd) {
    //                 WRITE_ERRORF("Overlapping or unsorted intervals in calibrator '%' (end=%, begin2=%).", getID(), time2string(lastEnd), time2string(state.begin));
    //             } else if (state.begin <= lastBegin) {
    //                 WRITE_ERRORF("Overlapping or unsorted intervals in calibrator '%' (begin=%, begin2=%).", getID(), time2string(lastBegin), time2string(state.begin));
    //             }
    //             state.end = attrs.getOptSUMOTimeReporting(SUMO_ATTR_END, getID().c_str(), ok, -1);
    //             state.vehicleParameter = SUMOVehicleParserHelper::parseVehicleAttributes(element, attrs, true, true, true);
    //             state.vehicleParameter->parametersSet &= ~VEHPARS_CALIBRATORSPEED_SET;
    //             myLeftoverVehicleParameters.push_back(state.vehicleParameter);
    //             // vehicles should be inserted with max speed unless stated otherwise
    //             if (state.vehicleParameter->departSpeedProcedure == DepartSpeedDefinition::DEFAULT) {
    //                 state.vehicleParameter->departSpeedProcedure = DepartSpeedDefinition::MAX;
    //             }
    //             // vehicles should be inserted on any lane unless stated otherwise
    //             if (state.vehicleParameter->departLaneProcedure == DepartLaneDefinition::DEFAULT) {
    //                 if (myLane == nullptr) {
    //                     state.vehicleParameter->departLaneProcedure = DepartLaneDefinition::ALLOWED_FREE;
    //                 } else {
    //                     state.vehicleParameter->departLaneProcedure = DepartLaneDefinition::GIVEN;
    //                     state.vehicleParameter->departLane = myLane->getIndex();
    //                 }
    //             } else if (myLane != nullptr && (
    //                            state.vehicleParameter->departLaneProcedure != DepartLaneDefinition::GIVEN
    //                            || state.vehicleParameter->departLane != myLane->getIndex())) {
    //                 WRITE_WARNINGF(TL("Insertion lane may differ from calibrator lane for calibrator '%'."), getID());
    //             }
    //             if (state.vehicleParameter->vtypeid != DEFAULT_VTYPE_ID &&
    //                     MSNet::getInstance()->getVehicleControl().getVType(state.vehicleParameter->vtypeid) == nullptr) {
    //                 WRITE_ERRORF(TL("Unknown vehicle type '%' in calibrator '%'."), state.vehicleParameter->vtypeid, getID());
    //             }
    //         } catch (EmptyData&) {
    //             WRITE_ERRORF(TL("Mandatory attribute missing in definition of calibrator '%'."), getID());
    //         } catch (NumberFormatException&) {
    //             WRITE_ERRORF(TL("Non-numeric value for numeric attribute in definition of calibrator '%'."), getID());
    //         }
    //         if (state.q < 0 && state.v < 0 && state.vehicleParameter->vtypeid == DEFAULT_VTYPE_ID) {
    //             WRITE_ERRORF(TL("Either 'vehsPerHour', 'speed' or 'type' has to be set in flow definition of calibrator '%'."), getID());
    //         }
    //         if (MSGlobals::gUseMesoSim && state.q < 0 && state.vehicleParameter->vtypeid != DEFAULT_VTYPE_ID) {
    //             WRITE_ERRORF(TL("Type calibration is not supported in meso for calibrator '%'."), getID());
    //         }
    //         if (myIntervals.size() > 0 && myIntervals.back().end == -1) {
    //             myIntervals.back().end = state.begin;
    //         }
    //         myIntervals.push_back(state);
    //         myCurrentStateInterval = myIntervals.begin();
    //     } else {
    //         MSRouteHandler::myStartElement(element, attrs);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element == SUMO_TAG_FLOW {
            let mut state = MSCalibrator_AspiredState::new();
            let mut lastEnd: SUMOTime = -1;
            let mut lastBegin: SUMOTime = -1;
            if self.myIntervals.len() > 0 {
                lastEnd = self.myIntervals.last().unwrap().end;
                lastBegin = self.myIntervals.last().unwrap().begin;
                if lastEnd == -1 {
                    lastEnd = self.myIntervals.last().unwrap().begin;
                }
            }
            let id: String = self.base3.get_id().to_string();
            let myLane: Ptr<MSLane> = self.myLane;
            // shortcut: SUMOSAXAttributes reports bad values itself instead of throwing, so the C++ catch blocks (EmptyData, NumberFormatException) are only emulated for the missing mandatory begin attribute
            let mut ok = true;
            state.q = attrs.get_opt::<f64>(SUMO_ATTR_VEHSPERHOUR, "", &mut ok, -1., true);
            state.v = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, "", &mut ok, -1., true);
            if attrs.get_string(SUMO_ATTR_BEGIN).is_empty() {
                // C++ getSUMOTimeReporting throws EmptyData here
                write_errorf!("Mandatory attribute missing in definition of calibrator '%'.", id.as_str());
            } else {
                state.begin = attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, &id, &mut ok, true);
                if state.begin < lastEnd {
                    write_errorf!("Overlapping or unsorted intervals in calibrator '%' (end=%, begin2=%).", id.as_str(), time2string(lastEnd), time2string(state.begin));
                } else if state.begin <= lastBegin {
                    write_errorf!("Overlapping or unsorted intervals in calibrator '%' (begin=%, begin2=%).", id.as_str(), time2string(lastBegin), time2string(state.begin));
                }
            }
            state.end = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_END, &id, &mut ok, -1, true);
            let vp: Ptr<SUMOVehicleParameter> = SUMOVehicleParserHelper::parse_vehicle_attributes(element, attrs, true, true, true, false);
            state.vehicleParameter = vp;
            let vpm: &mut SUMOVehicleParameter = vp.m();
            vpm.parametersSet &= !VEHPARS_CALIBRATORSPEED_SET;
            my_leftover_vehicle_parameters().push(vp);
            // vehicles should be inserted with max speed unless stated otherwise
            if vpm.departSpeedProcedure == DepartSpeedDefinition::DEFAULT {
                vpm.departSpeedProcedure = DepartSpeedDefinition::MAX;
            }
            // vehicles should be inserted on any lane unless stated otherwise
            if vpm.departLaneProcedure == DepartLaneDefinition::DEFAULT {
                if myLane.is_null() {
                    vpm.departLaneProcedure = DepartLaneDefinition::ALLOWED_FREE;
                } else {
                    vpm.departLaneProcedure = DepartLaneDefinition::GIVEN;
                    vpm.departLane = myLane.get_index();
                }
            } else if !myLane.is_null() && (vpm.departLaneProcedure != DepartLaneDefinition::GIVEN || vpm.departLane != myLane.get_index()) {
                write_warningf!("Insertion lane may differ from calibrator lane for calibrator '%'.", id.as_str());
            }
            if vpm.vtypeid != DEFAULT_VTYPE_ID {
                let net = MSNet::get_instance();
                let vc = net.m().get_vehicle_control();
                if vc.get_v_type(&vpm.vtypeid, Ptr::null(), false).is_null() {
                    write_errorf!("Unknown vehicle type '%' in calibrator '%'.", vpm.vtypeid.as_str(), id.as_str());
                }
            }
            if state.q < 0. && state.v < 0. && vpm.vtypeid == DEFAULT_VTYPE_ID {
                write_errorf!("Either 'vehsPerHour', 'speed' or 'type' has to be set in flow definition of calibrator '%'.", id.as_str());
            }
            if crate::microsim::ms_globals::use_meso_sim() && state.q < 0. && vpm.vtypeid != DEFAULT_VTYPE_ID {
                write_errorf!("Type calibration is not supported in meso for calibrator '%'.", id.as_str());
            }
            if !self.myIntervals.is_empty() && self.myIntervals.last().unwrap().end == -1 {
                self.myIntervals.last_mut().unwrap().end = state.begin;
            }
            self.myIntervals.push(state);
            self.myCurrentStateInterval = 0;
        } else {
            self.base.my_start_element(element, attrs);
        }
    }

    /// C++: `void MSCalibrator::myEndElement(int)` [virtual, protected]
    /// Called on the closing of a tag;
    // void
    // MSCalibrator::myEndElement(int element) {
    //     if (element == SUMO_TAG_CALIBRATOR) {
    //         if (!myDidInit) {
    //             init();
    //         }
    //         // ensure correct state of SUMORouteHandler::myElementStack
    //         callParentEnd(element);
    //     } else if (element != SUMO_TAG_FLOW) {
    //         MSRouteHandler::myEndElement(element);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_CALIBRATOR {
            if !self.myDidInit {
                self.init();
            }
            // shortcut: callParentEnd(element) is not ported (SUMORouteHandler has no GenericSAXHandler base), so the SUMORouteHandler element stack is not kept in sync, upgrade when SUMORouteHandler is ported
        } else if element != SUMO_TAG_FLOW {
            // shortcut: MSRouteHandler::myEndElement(element) is not ported, the element is not forwarded to the route handler, upgrade when MSRouteHandler has myEndElement
        }
    }

    /// C++: `bool MSCalibrator::isActive() const` [const, protected]
    // bool isActive() const {
    //         return myAmActive;
    //     }
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        self.myAmActive
    }

    /// C++: `void MSCalibrator::intervalEnd()` [protected]
    // void
    // MSCalibrator::intervalEnd() {
    //     if (myOutput != nullptr) {
    //         writeXMLOutput(*myOutput, myCurrentStateInterval->begin, myCurrentStateInterval->end);
    //     }
    //     myDidSpeedAdaption = false;
    //     myInserted = 0;
    //     myRemoved = 0;
    //     myClearedInJam = 0;
    //     myHaveWarnedAboutClearingJam = false;
    //     reset();
    // }
    #[allow(unused_variables)]
    pub fn interval_end(&mut self) {
        if !self.myOutput.is_null() {
            let dev: Ptr<OutputDevice> = self.myOutput;
            let begin = self.myIntervals[self.myCurrentStateInterval].begin;
            let end = self.myIntervals[self.myCurrentStateInterval].end;
            self.write_xml_output(dev.m(), begin, end);
        }
        self.myDidSpeedAdaption = false;
        self.myInserted = 0;
        self.myRemoved = 0;
        self.myClearedInJam = 0;
        self.myHaveWarnedAboutClearingJam = false;
        self.reset();
    }

    /// C++: `bool MSCalibrator::isCurrentStateActive(SUMOTime)` [protected]
    // bool
    // MSCalibrator::isCurrentStateActive(SUMOTime time) {
    //     while (myCurrentStateInterval != myIntervals.end() && myCurrentStateInterval->end <= time) {
    //         // XXX what about skipped intervals?
    //         myCurrentStateInterval++;
    //     }
    //     return myCurrentStateInterval != myIntervals.end() &&
    //            myCurrentStateInterval->begin <= time && myCurrentStateInterval->end > time;
    // }
    #[allow(unused_variables)]
    pub fn is_current_state_active(&mut self, time: SUMOTime) -> bool {
        while self.myCurrentStateInterval != self.myIntervals.len() && self.myIntervals[self.myCurrentStateInterval].end <= time {
            // XXX what about skipped intervals?
            self.myCurrentStateInterval += 1;
        }
        self.myCurrentStateInterval != self.myIntervals.len()
            && self.myIntervals[self.myCurrentStateInterval].begin <= time
            && self.myIntervals[self.myCurrentStateInterval].end > time
    }

    /// C++: `bool MSCalibrator::tryEmit(MSLane *, MSVehicle *)` [protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn try_emit(&mut self, lane: Ptr<MSLane>, vehicle: Ptr<MSVehicle>) -> bool {
        // C++ declares MSCalibrator::tryEmit (MSCalibrator.h) but MSCalibrator.cpp never defines it, so no C++ code can call it
        panic!("MSCalibrator::tryEmit is declared in C++ but never defined")
    }

    /// C++: `void MSCalibrator::init()` [protected]
    // void
    // MSCalibrator::init() {
    //     if (myIntervals.size() > 0) {
    //         if (myIntervals.back().end == -1) {
    //             myIntervals.back().end = SUMOTime_MAX;
    //         }
    //         // calibration should happen after regular insertions have taken place
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(new CalibratorCommand(this));
    //     } else {
    //         WRITE_WARNINGF(TL("No flow intervals in calibrator '%'."), getID());
    //     }
    //     myDidInit = true;
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        if !self.myIntervals.is_empty() {
            if self.myIntervals.last().unwrap().end == -1 {
                self.myIntervals.last_mut().unwrap().end = SUMOTIME_MAX;
            }
            // calibration should happen after regular insertions have taken place
            let cmd: *mut dyn Command = Box::into_raw(Box::new(MSCalibrator_CalibratorCommand::new(Ptr::from_mut(self))));
            let net = MSNet::get_instance();
            let events = net.m().get_end_of_timestep_events();
            events.m().add_event(Ptr::from_raw(cmd), -1);
        } else {
            write_warningf!("No flow intervals in calibrator '%'.", self.base3.get_id());
        }
        self.myDidInit = true;
    }

    /// C++: `int MSCalibrator::totalWished() const` [const, protected]
    /// number of vehicles expected to pass this interval
    // int
    // MSCalibrator::totalWished() const {
    //     if (myCurrentStateInterval != myIntervals.end()) {
    //         const double totalHourFraction = STEPS2TIME(myCurrentStateInterval->end - myCurrentStateInterval->begin) / (double) 3600.;
    //         return (int)std::floor(myCurrentStateInterval->q * totalHourFraction + 0.5); // round to closest int
    //     } else {
    //         return -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn total_wished(&self) -> i32 {
        if self.myCurrentStateInterval != self.myIntervals.len() {
            let s = &self.myIntervals[self.myCurrentStateInterval];
            let totalHourFraction = steps2time(s.end - s.begin) / 3600.;
            (s.q * totalHourFraction + 0.5).floor() as i32 // round to closest int
        } else {
            -1
        }
    }

    /// C++: `double MSCalibrator::currentFlow() const` [const, protected]
    /// flow in the current interval in veh/h
    // double
    // MSCalibrator::currentFlow() const {
    //     const double totalHourFraction = STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep() - myCurrentStateInterval->begin) / (double) 3600.;
    //     return passed() / totalHourFraction;
    // }
    #[allow(unused_variables)]
    pub fn current_flow(&self) -> f64 {
        let begin = self.myIntervals[self.myCurrentStateInterval].begin;
        let totalHourFraction = steps2time(MSNet::get_instance().get_current_time_step() - begin) / 3600.;
        self.passed() as f64 / totalHourFraction
    }

    /// C++: `double MSCalibrator::currentSpeed() const` [const, protected]
    /// measured speed in the current interval
    // double
    // MSCalibrator::currentSpeed() const {
    //     if (myEdgeMeanData.getSamples() > 0) {
    //         return myEdgeMeanData.getTravelledDistance() / myEdgeMeanData.getSamples();
    //     } else {
    //         return -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn current_speed(&self) -> f64 {
        if self.myEdgeMeanData.get_samples() > 0. {
            self.myEdgeMeanData.get_travelled_distance() / self.myEdgeMeanData.get_samples()
        } else {
            -1.
        }
    }

    /// C++: `bool MSCalibrator::invalidJam(int) const` [const, protected]
    // bool
    // MSCalibrator::invalidJam(int laneIndex) const {
    //     if (laneIndex < 0) {
    //         const int numLanes = (int)myEdge->getLanes().size();
    //         for (int i = 0; i < numLanes; ++i) {
    //             if (invalidJam(i)) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    //     assert(laneIndex < (int)myEdge->getLanes().size());
    //     const MSLane* const lane = myEdge->getLanes()[laneIndex];
    //     if (lane->getVehicleNumber() < 4) {
    //         // cannot reliably detect invalid jams
    //         return false;
    //     }
    //     // maxSpeed reflects the calibration target
    //     const bool toSlow = lane->getMeanSpeed() < myInvalidJamThreshold * myEdge->getSpeedLimit();
    //     return toSlow && remainingVehicleCapacity(laneIndex) < 1;
    // }
    #[allow(unused_variables)]
    pub fn invalid_jam(&self, laneIndex: i32) -> bool {
        if laneIndex < 0 {
            let numLanes = self.myEdge.get_lanes().len() as i32;
            for i in 0..numLanes {
                if self.invalid_jam(i) {
                    return true;
                }
            }
            return false;
        }
        assert!(laneIndex < self.myEdge.get_lanes().len() as i32);
        let lane: Ptr<MSLane> = self.myEdge.get_lanes()[laneIndex as usize];
        if lane.get_vehicle_number() < 4 {
            // cannot reliably detect invalid jams
            return false;
        }
        // maxSpeed reflects the calibration target
        let toSlow = lane.get_mean_speed() < self.myInvalidJamThreshold * self.myEdge.get_speed_limit();
        toSlow && self.remaining_vehicle_capacity(laneIndex) < 1
    }

    /// C++: `int MSCalibrator::inserted() const` [const, protected]
    // inline int inserted() const {
    //         return myInserted;
    //     }
    #[allow(unused_variables)]
    pub fn inserted(&self) -> i32 {
        self.myInserted
    }

    /// C++: `int MSCalibrator::removed() const` [const, protected]
    // inline int removed() const {
    //         return myRemoved;
    //     }
    #[allow(unused_variables)]
    pub fn removed(&self) -> i32 {
        self.myRemoved
    }

    /// C++: `int MSCalibrator::clearedInJam() const` [const, protected]
    // inline int clearedInJam() const {
    //         return myClearedInJam;
    //     }
    #[allow(unused_variables)]
    pub fn cleared_in_jam(&self) -> i32 {
        self.myClearedInJam
    }

    /// C++: `int MSCalibrator::remainingVehicleCapacity(int) const` [const, protected]
    // int
    // MSCalibrator::remainingVehicleCapacity(int laneIndex) const {
    //     if (laneIndex < 0) {
    //         const int numLanes = (int)myEdge->getLanes().size();
    //         int result = 0;
    //         for (int i = 0; i < numLanes; ++i) {
    //             result = MAX2(result, remainingVehicleCapacity(i));
    //         }
    //         return result;
    //     }
    //     assert(laneIndex < (int)myEdge->getLanes().size());
    //     MSLane* lane = myEdge->getLanes()[laneIndex];
    //     MSVehicle* last = lane->getLastFullVehicle();
    //     const SUMOVehicleParameter* pars = myCurrentStateInterval->vehicleParameter;
    //     const MSVehicleType* vtype = MSNet::getInstance()->getVehicleControl().getVType(pars->vtypeid);
    //     const double spacePerVehicle = vtype->getLengthWithGap() + myEdge->getSpeedLimit() * vtype->getCarFollowModel().getHeadwayTime();
    //     int overallSpaceLeft = (int)ceil(lane->getLength() / spacePerVehicle) - lane->getVehicleNumber();
    //     if (last != nullptr) {
    //         int entrySpaceLeft = (int)(last->getPositionOnLane() / spacePerVehicle);
    //         return MAX2(overallSpaceLeft, entrySpaceLeft);
    //     } else {
    //         return overallSpaceLeft;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remaining_vehicle_capacity(&self, laneIndex: i32) -> i32 {
        if laneIndex < 0 {
            let numLanes = self.myEdge.get_lanes().len() as i32;
            let mut result: i32 = 0;
            for i in 0..numLanes {
                result = std::cmp::max(result, self.remaining_vehicle_capacity(i));
            }
            return result;
        }
        assert!(laneIndex < self.myEdge.get_lanes().len() as i32);
        let lane: Ptr<MSLane> = self.myEdge.get_lanes()[laneIndex as usize];
        let last: Ptr<MSVehicle> = lane.get_last_full_vehicle();
        let pars: Ptr<SUMOVehicleParameter> = self.myIntervals[self.myCurrentStateInterval].vehicleParameter;
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        let vtype: Ptr<MSVehicleType> = vc.get_v_type(&pars.vtypeid, Ptr::null(), false);
        let spacePerVehicle = vtype.get_length_with_gap() + self.myEdge.get_speed_limit() * vtype.get_car_follow_model().get_headway_time();
        let overallSpaceLeft: i32 = (lane.get_length() / spacePerVehicle).ceil() as i32 - lane.get_vehicle_number();
        if !last.is_null() {
            let entrySpaceLeft: i32 = (last.get_position_on_lane() / spacePerVehicle) as i32;
            std::cmp::max(overallSpaceLeft, entrySpaceLeft)
        } else {
            overallSpaceLeft
        }
    }

    /// C++: `void MSCalibrator::reset()` [virtual, protected]
    /// reset collected vehicle data
    // void
    // MSCalibrator::reset() {
    //     myEdgeMeanData.reset();
    //     for (MSMeanData_Net::MSLaneMeanDataValues* const val : myLaneMeanData) {
    //         val->reset();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.myEdgeMeanData.reset(false);
        for val in self.myLaneMeanData.iter() {
            val.m().reset(false);
        }
    }

    /// C++: `void MSCalibrator::updateMeanData()` [virtual, protected]
    /// aggregate lane values
    // void
    // MSCalibrator::updateMeanData() {
    //     myEdgeMeanData.reset();
    //     for (std::vector<MSMeanData_Net::MSLaneMeanDataValues*>::iterator it = myLaneMeanData.begin();
    //             it != myLaneMeanData.end(); ++it) {
    //         (*it)->addTo(myEdgeMeanData);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_mean_data(&mut self) {
        self.myEdgeMeanData.reset(false);
        for val in self.myLaneMeanData.iter() {
            val.m().add_to(&mut self.myEdgeMeanData.base);
        }
    }

    /// C++: `bool MSCalibrator::scheduleRemoval(SUMOTrafficObject *)` [protected]
    /// try to schedule the given vehicle for removal. return true if it isn't already scheduled
    // bool scheduleRemoval(SUMOTrafficObject* veh) {
    //         return myToRemove.insert(veh->getID()).second;
    //     }
    #[allow(unused_variables)]
    pub fn schedule_removal(&mut self, veh: Ptr<SUMOTrafficObject>) -> bool {
        self.myToRemove.insert(veh.get_id().to_string())
    }

    /// C++: `bool MSCalibrator::removePending()` [protected]
    /// remove any vehicles which are scheduled for removal. return true if removals took place
    // bool
    // MSCalibrator::removePending() {
    //     if (myToRemove.size() > 0) {
    //         MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //         // it is not save to remove the vehicles inside
    //         // VehicleRemover::notifyEnter so we do it here
    //         for (std::set<std::string>::iterator it = myToRemove.begin(); it != myToRemove.end(); ++it) {
    //             MSVehicle* vehicle = dynamic_cast<MSVehicle*>(vc.getVehicle(*it));
    //             if (vehicle != nullptr) {
    //                 MSLane* lane = vehicle->getMutableLane();
    //                 vehicle->onRemovalFromNet(MSMoveReminder::NOTIFICATION_VAPORIZED_CALIBRATOR);
    //                 lane->removeVehicle(vehicle, MSMoveReminder::NOTIFICATION_VAPORIZED_CALIBRATOR);
    //                 vc.scheduleVehicleRemoval(vehicle, true);
    //             } else {
    //                 WRITE_WARNINGF(TL("Calibrator '%' could not remove vehicle '%' time=%."), getID(), *it, time2string(MSNet::getInstance()->getCurrentTimeStep()));
    //             }
    //         }
    //         myToRemove.clear();
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn remove_pending(&mut self) -> bool {
        if self.myToRemove.len() > 0 {
            let net = MSNet::get_instance();
            let vc = net.m().get_vehicle_control();
            // it is not save to remove the vehicles inside
            // VehicleRemover::notifyEnter so we do it here
            for id in self.myToRemove.iter() {
                let vehicleSV: Ptr<SUMOVehicle> = vc.get_vehicle(id);
                // shortcut: dynamic_cast<MSVehicle*> is a plain cast here, the microsim vehicle control only holds MSVehicle
                let vehicle: Ptr<MSVehicle> = if vehicleSV.is_null() { Ptr::null() } else { vehicleSV.cast::<MSVehicle>() };
                if !vehicle.is_null() {
                    let lane: Ptr<MSLane> = vehicle.m().get_mutable_lane();
                    vehicle.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_CALIBRATOR);
                    lane.m().remove_vehicle(vehicle, MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_CALIBRATOR, true);
                    vc.schedule_vehicle_removal(vehicle.cast::<SUMOVehicle>(), true);
                } else {
                    write_warningf!("Calibrator '%' could not remove vehicle '%' time=%.", self.base3.get_id(), id.as_str(), time2string(MSNet::get_instance().get_current_time_step()));
                }
            }
            self.myToRemove.clear();
            return true;
        }
        false
    }

    /// C++: `std::string MSCalibrator::getNewVehicleID()` [protected]
    /// determine id of new vehicle from calibrator state
    // std::string
    // MSCalibrator::getNewVehicleID() {
    //     // avoid name clash for subsecond interval spacing
    //     const double beginS = STEPS2TIME(myCurrentStateInterval->begin);
    //     const int precision = beginS == int(beginS) ? 0 : 2;
    //     return getID() + "." + toString(beginS, precision) + "." + toString(myInserted);
    // }
    #[allow(unused_variables)]
    pub fn get_new_vehicle_id(&mut self) -> String {
        // avoid name clash for subsecond interval spacing
        let beginS = steps2time(self.myIntervals[self.myCurrentStateInterval].begin);
        let precision: i32 = if beginS == (beginS as i32) as f64 { 0 } else { 2 };
        format!("{}.{}.{}", self.base3.get_id(), f64_to_string_prec(beginS, precision), self.myInserted)
    }

}

fn ms_calibrator_vehicle_remover_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSCalibrator_VehicleRemover>().m().notify_enter(veh, reason, enteredLane)
}
fn ms_calibrator_vehicle_remover_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSCalibrator_VehicleRemover>().m().notify_leave(veh, lastPos, reason, enteredLane)
}
static MS_CALIBRATOR_VEHICLE_REMOVER_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: ms_calibrator_vehicle_remover_notify_enter,
    notify_move: default_notify_move,
    notify_leave: ms_calibrator_vehicle_remover_notify_leave,
};

/// C++ class `MSCalibrator::VehicleRemover` (microsim/trigger/MSCalibrator.h)
#[repr(C)] // base must stay at offset 0: VehicleRemover is cast to MSMoveReminder for myLeftoverReminders
pub struct MSCalibrator_VehicleRemover {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    pub myParent: Ptr<MSCalibrator>,
    pub myUndoCalibration: bool,
}

impl std::ops::Deref for MSCalibrator_VehicleRemover {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSCalibrator_VehicleRemover {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSCalibrator_VehicleRemover {
    /// C++: `void VehicleRemover::VehicleRemover(MSLane *const, MSCalibrator *const, const bool)`
    // VehicleRemover(MSLane* const lane, MSCalibrator* const parent, const bool undo = false) :
    //             MSMoveReminder(parent->getID(), lane, true), myParent(parent), myUndoCalibration(undo) {}
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, parent: Ptr<MSCalibrator>, undo: bool) -> Self {
        MSCalibrator_VehicleRemover {
            base: MSMoveReminder {
                vt: &MS_CALIBRATOR_VEHICLE_REMOVER_MSMOVE_REMINDER_VT,
                ..MSMoveReminder::new(parent.base3.get_id(), lane, true)
            },
            myParent: parent,
            myUndoCalibration: undo,
        }
    }

    /// C++: `bool VehicleRemover::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if self.myParent.is_null() {
            return false;
        }
        let parent: Ptr<MSCalibrator> = self.myParent;
        if !parent.base3.vehicle_applies(veh) {
            return false;
        }
        if parent.is_active() {
            parent.m().update_mean_data();
            let calibrateFlow = parent.myIntervals[parent.myCurrentStateInterval].q >= 0.;
            let totalWishedNum = parent.total_wished();
            let adaptedNum = parent.passed() + parent.myClearedInJam;
            if calibrateFlow && adaptedNum > totalWishedNum {
                if parent.m().schedule_removal(Ptr::from_mut(veh)) {
                    parent.m().myRemoved += 1;
                }
            } else if parent.myHaveInvalidJam {
                if !parent.myHaveWarnedAboutClearingJam {
                    write_warningf!("Clearing jam at calibrator '%' at time=%.", parent.base3.get_id(), time2string(MSNet::get_instance().get_current_time_step()));
                    parent.m().myHaveWarnedAboutClearingJam = true;
                }
                if parent.m().schedule_removal(Ptr::from_mut(veh)) {
                    parent.m().myClearedInJam += 1;
                }
            }
            let typeID: String = parent.myIntervals[parent.myCurrentStateInterval].vehicleParameter.vtypeid.clone();
            if !calibrateFlow && typeID != DEFAULT_VTYPE_ID {
                // calibrate type
                let origType: String = veh.get_parameter().vtypeid.clone(); // could by id of vTypeDistribution
                let net = MSNet::get_instance();
                let vc = net.m().get_vehicle_control();
                let oldDist = vc.get_v_type_distribution(&origType);
                let newDist = vc.get_v_type_distribution(&typeID);
                let mut matchDistribution = false;
                if let (Some(oldDist), Some(newDist)) = (oldDist.get(), newDist.get()) {
                    if oldDist.get_vals().len() == newDist.get_vals().len() {
                        let vt: Ptr<MSVehicleType> = Ptr::new(veh.get_vehicle_type());
                        if let Some(distIndex) = oldDist.get_vals().iter().position(|v| *v == vt) {
                            matchDistribution = true;
                            veh.replace_vehicle_type(newDist.get_vals()[distIndex]);
                        }
                    }
                }
                if !matchDistribution {
                    let vehicleType: Ptr<MSVehicleType> = vc.get_v_type(&typeID, Ptr::null(), false);
                    if vehicleType.is_null() {
                        std::panic::panic_any(ProcessError::new(format!("Unknown vehicle type '{}' in calibrator '{}'", typeID, parent.base3.get_id())));
                    }
                    veh.replace_vehicle_type(vehicleType);
                }
            }
        }
        true
    }

    /// C++: `bool VehicleRemover::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    /// Checks whether any calibrations need to be undone
    // bool
    // MSCalibrator::VehicleRemover::notifyLeave(SUMOTrafficObject& veh, double /* lastPos */, Notification reason, const MSLane* /* enteredLane */) {
    //     if (myUndoCalibration && reason != NOTIFICATION_LANE_CHANGE) {
    //         // TODO check for distributions
    //         veh.replaceVehicleType(MSNet::getInstance()->getVehicleControl().getVType(veh.getParameter().vtypeid));
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if self.myUndoCalibration && !matches!(reason, MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE) {
            // TODO check for distributions
            let net = MSNet::get_instance();
            let vc = net.m().get_vehicle_control();
            let vtype: Ptr<MSVehicleType> = vc.get_v_type(&veh.get_parameter().vtypeid, Ptr::null(), false);
            veh.replace_vehicle_type(vtype);
        }
        true
    }

    /// C++: `void VehicleRemover::disable()`
    // void disable() {
    //             myParent = nullptr;
    //         }
    #[allow(unused_variables)]
    pub fn disable(&mut self) {
        self.myParent = Ptr::null();
    }

}

