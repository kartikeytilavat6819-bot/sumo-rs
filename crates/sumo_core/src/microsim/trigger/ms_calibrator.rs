//! Port of microsim/trigger/MSCalibrator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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

/// Calibrates the flow on a segment to a specified one
/// C++ class `MSCalibrator` (microsim/trigger/MSCalibrator.h)
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
    pub fn new(id: &str, edge: Ptr<MSEdge>, lane: Ptr<MSLane>, node: Ptr<MSJunction>, pos: f64, aXMLFilename: &str, outputFilename: &str, freq: SUMOTime, length: f64, probe: Ptr<MSRouteProbe>, invalidJamThreshold: f64, vTypes: &str, local: bool, addLaneMeanData: bool) -> Self {
        todo!("port MSCalibrator::MSCalibrator")
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
        todo!("port MSCalibrator::~MSCalibrator")
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
        todo!("port MSCalibrator::writeXMLOutput")
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
        todo!("port MSCalibrator::execute")
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
        todo!("port MSCalibrator::cleanup")
    }

    /// C++: `const std::map<std::string, MSCalibrator *> & MSCalibrator::getInstances()` [static]
    /// return all calibrator instances
    // static const std::map<std::string, MSCalibrator*>& getInstances() {
    //         return myInstances;
    //     }
    #[allow(unused_variables)]
    pub fn get_instances() -> &'static BTreeMap<String, Ptr<MSCalibrator>> {
        todo!("port MSCalibrator::getInstances")
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
        todo!("port MSCalibrator::setFlow")
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
        todo!("port MSCalibrator::myStartElement")
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
        todo!("port MSCalibrator::myEndElement")
    }

    /// C++: `bool MSCalibrator::isActive() const` [const, protected]
    // bool isActive() const {
    //         return myAmActive;
    //     }
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        todo!("port MSCalibrator::isActive")
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
        todo!("port MSCalibrator::intervalEnd")
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
        todo!("port MSCalibrator::isCurrentStateActive")
    }

    /// C++: `bool MSCalibrator::tryEmit(MSLane *, MSVehicle *)` [protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn try_emit(&mut self, lane: Ptr<MSLane>, vehicle: Ptr<MSVehicle>) -> bool {
        todo!("port MSCalibrator::tryEmit")
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
        todo!("port MSCalibrator::init")
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
        todo!("port MSCalibrator::currentSpeed")
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
        todo!("port MSCalibrator::invalidJam")
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
        todo!("port MSCalibrator::remainingVehicleCapacity")
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
        todo!("port MSCalibrator::reset")
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
        todo!("port MSCalibrator::updateMeanData")
    }

    /// C++: `bool MSCalibrator::scheduleRemoval(SUMOTrafficObject *)` [protected]
    /// try to schedule the given vehicle for removal. return true if it isn't already scheduled
    // bool scheduleRemoval(SUMOTrafficObject* veh) {
    //         return myToRemove.insert(veh->getID()).second;
    //     }
    #[allow(unused_variables)]
    pub fn schedule_removal(&mut self, veh: Ptr<SUMOTrafficObject>) -> bool {
        todo!("port MSCalibrator::scheduleRemoval")
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
        todo!("port MSCalibrator::removePending")
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
        todo!("port MSCalibrator::getNewVehicleID")
    }

}

/// C++ class `MSCalibrator::VehicleRemover` (microsim/trigger/MSCalibrator.h)
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
        todo!("port VehicleRemover::VehicleRemover")
    }

    /// C++: `bool VehicleRemover::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port VehicleRemover::notifyEnter")
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
        todo!("port VehicleRemover::notifyLeave")
    }

    /// C++: `void VehicleRemover::disable()`
    // void disable() {
    //             myParent = nullptr;
    //         }
    #[allow(unused_variables)]
    pub fn disable(&mut self) {
        todo!("port VehicleRemover::disable")
    }

}

