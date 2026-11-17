//! Port of mesosim/METriggeredCalibrator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Calibrates the flow on a segment to a specified one
/// C++ class `METriggeredCalibrator` (mesosim/METriggeredCalibrator.h)
#[repr(C)]
pub struct METriggeredCalibrator {
    /// C++ base class `MSCalibrator` (boxed: MSCalibrator::new registers `this`, so it must not move)
    pub base: Box<MSCalibrator>,
    /// mesoscopic edge segment the calibrator lies on
    pub mySegment: Ptr<MESegment>,
}

impl std::ops::Deref for METriggeredCalibrator {
    type Target = MSCalibrator;
    fn deref(&self) -> &MSCalibrator {
        &self.base
    }
}
impl std::ops::DerefMut for METriggeredCalibrator {
    fn deref_mut(&mut self) -> &mut MSCalibrator {
        &mut self.base
    }
}

impl METriggeredCalibrator {
    /// C++: `void METriggeredCalibrator::METriggeredCalibrator(const std::string &, MSEdge *const, const double, const std::string &, const std::string &, const SUMOTime, const double, const MSRouteProbe *, const double, const std::string &)`
    /// constructor
    // METriggeredCalibrator::METriggeredCalibrator(const std::string& id,
    //         MSEdge* const edge, const double pos,
    //         const std::string& aXMLFilename,
    //         const std::string& outputFilename,
    //         const SUMOTime freq, const double length,
    //         const MSRouteProbe* probe,
    //         const double invalidJamThreshold,
    //         const std::string& vTypes) :
    //     MSCalibrator(id, edge, nullptr, nullptr, pos, aXMLFilename, outputFilename, freq, length, probe, invalidJamThreshold, vTypes, false, false),
    //     mySegment(edge == nullptr ? nullptr : MSGlobals::gMesoNet->getSegmentForEdge(*edge, pos)) {
    //     myEdgeMeanData.setDescription("meandata_calibrator_" + getID());
    //     if (mySegment != nullptr) {
    //         mySegment->addDetector(&myEdgeMeanData);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, edge: Ptr<MSEdge>, pos: f64, aXMLFilename: &str, outputFilename: &str, freq: SUMOTime, length: f64, probe: Ptr<MSRouteProbe>, invalidJamThreshold: f64, vTypes: &str) -> Box<Self> {
        // Boxed: the base MSCalibrator registers `this`, and the detector registration below stores pointers into it
        let base = MSCalibrator::new(id, edge, Ptr::null(), Ptr::null(), pos, aXMLFilename, outputFilename, freq, length, probe, invalidJamThreshold, vTypes, false, false);
        let mySegment = if edge.is_null() { Ptr::null() } else { meso_net().m().get_segment_for_edge(&edge, pos) };
        let mut b = Box::new(METriggeredCalibrator { base, mySegment });
        let desc = format!("meandata_calibrator_{}", b.base3.get_id());
        b.base.myEdgeMeanData.base.base.set_description(&desc);
        if !b.mySegment.is_null() {
            let seg = b.mySegment;
            seg.m().add_detector(Ptr::from_mut(&mut b.base.myEdgeMeanData.base.base), -1);
        }
        b
    }

    /// C++: `void METriggeredCalibrator::~METriggeredCalibrator() noexcept` [virtual]
    /// destructor
    // METriggeredCalibrator::~METriggeredCalibrator() {
    //     if (myCurrentStateInterval != myIntervals.end()) {
    //         // need to do it here and not in MSCalibrator because otherwise meandata is gone
    //         intervalEnd();
    //         // but avoid to call it again in MSCalibrator
    //         myCurrentStateInterval = myIntervals.end();
    //     }
    //     // TODO this is just commented out to work around https://github.com/eclipse/sumo/issues/7861
    //     //mySegment->removeDetector(&myEdgeMeanData);
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if self.myCurrentStateInterval != self.myIntervals.len() {
            // need to do it here and not in MSCalibrator because otherwise meandata is gone
            self.interval_end();
            // but avoid to call it again in MSCalibrator
            self.myCurrentStateInterval = self.myIntervals.len();
        }
    }

    /// C++: `SUMOTime METriggeredCalibrator::execute(SUMOTime)` [virtual]
    /// the implementation of the MSTrigger / Command interface. Calibrating takes place here.
    // SUMOTime
    // METriggeredCalibrator::execute(SUMOTime currentTime) {
    //     // get current simulation values (valid for the last simulation second)
    //     // XXX could we miss vehicle movements if this is called less often than every DELTA_T (default) ?
    //     mySegment->prepareDetectorForWriting(myEdgeMeanData);
    //
    //     // check whether an adaptation value exists
    //     if (isCurrentStateActive(currentTime)) {
    //         // all happens in isCurrentStateActive()
    //         myAmActive = true;
    //     } else {
    //         myAmActive = false;
    //         myEdgeMeanData.reset(); // discard collected values
    //         if (!mySpeedIsDefault) {
    //             // if not, reset adaptation values
    //             const double jamThresh = OptionsCont::getOptions().getFloat("meso-jam-threshold");
    //             myEdge->setMaxSpeed(myDefaultSpeed, false, jamThresh);
    //             mySpeedIsDefault = true;
    //         }
    //         if (myCurrentStateInterval == myIntervals.end()) {
    //             // keep calibrator alive but do not call again
    //             return SUMOTime_DAY;
    //         }
    //         return myFrequency;
    //     }
    //     const bool calibrateFlow = myCurrentStateInterval->q >= 0;
    //     const bool calibrateSpeed = myCurrentStateInterval->v >= 0;
    //     // we are active
    //     if (!myDidSpeedAdaption && calibrateSpeed && myCurrentStateInterval->v != mySegment->getEdge().getSpeedLimit()) {
    //         myEdge->setMaxSpeed(myCurrentStateInterval->v);
    //         mySpeedIsDefault = false;
    //         myDidSpeedAdaption = true;
    //     }
    //     // clear invalid jams
    //     bool hadInvalidJam = false;
    //     while ((calibrateFlow || calibrateSpeed) && invalidJam()) {
    //         hadInvalidJam = true;
    //         if (!myHaveWarnedAboutClearingJam) {
    //             WRITE_WARNINGF(TL("Clearing jam at calibrator '%' at time=%."), getID(), time2string(currentTime));
    //         }
    //         // remove one vehicle currently on the segment
    //         if (mySegment->vaporizeAnyCar(currentTime, this)) {
    //             myClearedInJam++;
    //         } else {
    //             if (!myHaveWarnedAboutClearingJam) {
    //                 // this frequenly happens for very short edges
    //                 WRITE_WARNINGF(TL("Could not clear jam at calibrator '%' at time=%."), getID(), time2string(currentTime));
    //             }
    //             break;
    //         }
    //         myHaveWarnedAboutClearingJam = true;
    //     }
    //     if (calibrateFlow) {
    //         // flow calibration starts here ...
    //         // compute the number of vehicles that should have passed the calibrator within the time
    //         // rom begin of the interval
    //         const double totalHourFraction = STEPS2TIME(myCurrentStateInterval->end - myCurrentStateInterval->begin) / (double) 3600.;
    //         const int totalWishedNum = (int)std::floor(myCurrentStateInterval->q * totalHourFraction + 0.5); // round to closest int
    //         int adaptedNum = passed() + myClearedInJam;
    //         if (!hadInvalidJam) {
    //             // only add vehicles if we do not have an invalid upstream jam to prevent spill-back
    //             const double hourFraction = STEPS2TIME(currentTime - myCurrentStateInterval->begin + DELTA_T) / (double) 3600.;
    //             const int wishedNum = (int)std::floor(myCurrentStateInterval->q * hourFraction + 0.5); // round to closest int
    //             // only the difference between inflow and aspiredFlow should be added, thus
    //             // we should not count vehicles vaporized from a jam here
    //             // if we have enough time left we can add missing vehicles later
    //             const int relaxedInsertion = (int)std::floor(STEPS2TIME(myCurrentStateInterval->end - currentTime) / 3);
    //             const int insertionSlack = MAX2(0, adaptedNum + relaxedInsertion - totalWishedNum);
    //             // increase number of vehicles
    //             //std::cout << "time:" << STEPS2TIME(currentTime) << " w:" << wishedNum << " s:" << insertionSlack << " before:" << adaptedNum;
    //             MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //             while (wishedNum > adaptedNum + insertionSlack && remainingVehicleCapacity() > maximumInflow()) {
    //                 SUMOVehicleParameter* pars = myCurrentStateInterval->vehicleParameter;
    //                 ConstMSRoutePtr route = myProbe != nullptr ? myProbe->sampleRoute() : nullptr;
    //                 if (route == nullptr) {
    //                     route = MSRoute::dictionary(pars->routeid);
    //                 }
    //                 if (route == nullptr) {
    //                     WRITE_WARNINGF(TL("No valid routes in calibrator '%'."), getID());
    //                     break;
    //                 }
    //                 if (!route->contains(myEdge)) {
    //                     WRITE_WARNINGF(TL("Route '%' in calibrator '%' does not contain edge '%'."), route->getID(), getID(), myEdge->getID());
    //                     break;
    //                 }
    //                 MSVehicleType* vtype = vc.getVType(pars->vtypeid);
    //                 assert(route != 0 && vtype != 0);
    //                 // build the vehicle
    //                 const SUMOTime depart = mySegment->getNextInsertionTime(currentTime);
    //                 SUMOVehicleParameter* newPars = new SUMOVehicleParameter(*pars);
    //                 newPars->id = getNewVehicleID();
    //                 newPars->depart = depart;
    //                 newPars->routeid = route->getID();
    //                 MEVehicle* vehicle;
    //                 try {
    //                     vehicle = static_cast<MEVehicle*>(vc.buildVehicle(newPars, route, vtype, false, MSVehicleControl::VehicleDefinitionSource::TRIGGER));
    //                     std::string msg;
    //                     if (!vehicle->hasValidRouteStart(msg)) {
    //                         throw ProcessError(msg);
    //                     }
    //                 } catch (const ProcessError& e) {
    //                     if (!MSGlobals::gCheckRoutes) {
    //                         WRITE_WARNING(e.what());
    //                         vehicle = nullptr;
    //                         break;
    //                     } else {
    //                         throw;
    //                     }
    //                 }
    //                 const bool duplicate = vc.getVehicle(newPars->id) != nullptr;
    //                 // duplicate ids could come from loading state
    //                 if (duplicate) {
    //                     vc.deleteVehicle(vehicle, true);
    //                     continue;
    //                 }
    //                 vehicle->setSegment(mySegment); // needed or vehicle will not be registered (XXX why?)
    //                 vehicle->setEventTime(currentTime); // XXX superfluous?
    //                 // move vehicle forward when the route does not begin at the calibrator's edge
    //                 const MSEdge* myedge = &mySegment->getEdge();
    //                 bool atDest = false;
    //                 while (vehicle->getEdge() != myedge) {
    //                     // let the vehicle move to the next edge
    //                     atDest = vehicle->moveRoutePointer();
    //                 }
    //                 // insert vehicle into the net
    //                 if (atDest || !tryEmit(mySegment, vehicle)) {
    //                     //std::cout << "F ";
    //                     vc.deleteVehicle(vehicle, true);
    //                     break;
    //                 }
    //                 //std::cout << "I ";
    //                 myInserted++;
    //                 adaptedNum++;
    //             }
    //         }
    //         //std::cout << " after:" << adaptedNum << "\n";
    //         // we only remove vehicles once we really have to
    //         while (totalWishedNum < adaptedNum) {
    //             if (!mySegment->vaporizeAnyCar(currentTime, this)) {
    //                 // @bug: short edges may be jumped in a single step, giving us no chance to remove a vehicle
    //                 break;
    //             }
    //             myRemoved++;
    //             adaptedNum--;
    //         }
    //     }
    //     if (myCurrentStateInterval->end <= currentTime + myFrequency) {
    //         intervalEnd();
    //     }
    //     //assert(!invalidJam());
    //     if (invalidJam()) {
    //         WRITE_WARNINGF("DEBUG: Could not clear jam at calibrator '%' at time=%.", getID(), time2string(currentTime));
    //     }
    //     return myFrequency;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        // get current simulation values (valid for the last simulation second)
        let seg = self.mySegment;
        seg.m().prepare_detector_for_writing(&mut self.myEdgeMeanData.base.base, -1);

        // check whether an adaptation value exists
        if self.is_current_state_active(currentTime) {
            // all happens in isCurrentStateActive()
            self.myAmActive = true;
        } else {
            self.myAmActive = false;
            self.myEdgeMeanData.reset(false); // discard collected values
            if !self.mySpeedIsDefault {
                // if not, reset adaptation values
                let jamThresh = utils::options::options_cont::get_float("meso-jam-threshold");
                self.myEdge.m().set_max_speed(self.myDefaultSpeed, false, jamThresh);
                self.mySpeedIsDefault = true;
            }
            if self.myCurrentStateInterval == self.myIntervals.len() {
                // keep calibrator alive but do not call again
                return utils::common::time::SUMOTIME_DAY;
            }
            return self.myFrequency;
        }
        let idx = self.myCurrentStateInterval;
        let calibrateFlow = self.myIntervals[idx].q >= 0.;
        let calibrateSpeed = self.myIntervals[idx].v >= 0.;
        // we are active
        if !self.myDidSpeedAdaption && calibrateSpeed && self.myIntervals[idx].v != self.mySegment.get_edge().get_speed_limit() {
            self.myEdge.m().set_max_speed(self.myIntervals[idx].v, true, -1.);
            self.mySpeedIsDefault = false;
            self.myDidSpeedAdaption = true;
        }
        // clear invalid jams
        let mut hadInvalidJam = false;
        while (calibrateFlow || calibrateSpeed) && self.invalid_jam() {
            hadInvalidJam = true;
            if !self.myHaveWarnedAboutClearingJam {
                write_warningf!("Clearing jam at calibrator '%' at time=%.", self.base3.get_id(), utils::common::time::time2string(currentTime));
            }
            // remove one vehicle currently on the segment
            let seg = self.mySegment;
            if seg.m().vaporize_any_car(currentTime, Ptr::from_mut(&mut self.base3)) {
                self.myClearedInJam += 1;
            } else {
                if !self.myHaveWarnedAboutClearingJam {
                    // this frequenly happens for very short edges
                    write_warningf!("Could not clear jam at calibrator '%' at time=%.", self.base3.get_id(), utils::common::time::time2string(currentTime));
                }
                break;
            }
            self.myHaveWarnedAboutClearingJam = true;
        }
        if calibrateFlow {
            // flow calibration starts here ...
            // compute the number of vehicles that should have passed the calibrator within the time
            // rom begin of the interval
            let totalHourFraction = utils::common::time::steps2time(self.myIntervals[idx].end - self.myIntervals[idx].begin) / 3600.;
            let totalWishedNum = (self.myIntervals[idx].q * totalHourFraction + 0.5).floor() as i32; // round to closest int
            let mut adaptedNum = self.passed() + self.myClearedInJam;
            if !hadInvalidJam {
                // only add vehicles if we do not have an invalid upstream jam to prevent spill-back
                let hourFraction = utils::common::time::steps2time(currentTime - self.myIntervals[idx].begin + utils::common::time::delta_t()) / 3600.;
                let wishedNum = (self.myIntervals[idx].q * hourFraction + 0.5).floor() as i32; // round to closest int
                // only the difference between inflow and aspiredFlow should be added, thus
                // we should not count vehicles vaporized from a jam here
                // if we have enough time left we can add missing vehicles later
                let relaxedInsertion = (utils::common::time::steps2time(self.myIntervals[idx].end - currentTime) / 3.).floor() as i32;
                let insertionSlack = std::cmp::max(0, adaptedNum + relaxedInsertion - totalWishedNum);
                // increase number of vehicles
                let net = MSNet::get_instance();
                let vc = net.m().get_vehicle_control();
                while wishedNum > adaptedNum + insertionSlack && self.remaining_vehicle_capacity() > self.maximum_inflow() {
                    let pars = self.myIntervals[idx].vehicleParameter;
                    let mut route: Option<ConstMSRoutePtr> = if !self.myProbe.is_null() { self.myProbe.sample_route(true) } else { None };
                    if route.is_none() {
                        // C++ `route = MSRoute::dictionary(pars->routeid);`; an unknown id yields the null route
                        route = Some(MSRoute::dictionary_3(&pars.routeid, Ptr::null()));
                    }
                    let Some(route) = route.filter(|r| !crate::microsim::ms_route::is_null_route(r)) else {
                        write_warningf!("No valid routes in calibrator '%'.", self.base3.get_id());
                        break;
                    };
                    if !route.contains(self.myEdge) {
                        write_warningf!("Route '%' in calibrator '%' does not contain edge '%'.", route.get_id(), self.base3.get_id(), self.myEdge.get_id());
                        break;
                    }
                    let vtype = vc.get_v_type(&pars.vtypeid, Ptr::null(), false);
                    assert!(!vtype.is_null());
                    // build the vehicle
                    let depart = self.mySegment.get_next_insertion_time(currentTime);
                    let mut newPars = (*pars).clone();
                    newPars.id = self.get_new_vehicle_id();
                    newPars.depart = depart;
                    newPars.routeid = route.get_id().to_string();
                    let newParsPtr: Ptr<SUMOVehicleParameter> = Ptr::from_raw(Box::into_raw(Box::new(newPars)));
                    let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let v = vc.build_vehicle(newParsPtr, route.clone(), vtype, false, MSVehicleControl_VehicleDefinitionSource::TRIGGER, true).cast::<MEVehicle>();
                        let mut msg = String::new();
                        if !v.m().has_valid_route_start(&mut msg) {
                            std::panic::panic_any(ProcessError::new(msg));
                        }
                        v
                    }));
                    let vehicle = match built {
                        Ok(v) => v,
                        Err(payload) => match payload.downcast::<ProcessError>() {
                            Ok(e) => {
                                if !MSGlobals::check_routes() {
                                    utils::common::msg_handler::write_warning(&e.to_string());
                                    break;
                                } else {
                                    std::panic::resume_unwind(e);
                                }
                            }
                            Err(other) => std::panic::resume_unwind(other),
                        },
                    };
                    let duplicate = !vc.get_vehicle(&newParsPtr.id).is_null();
                    // duplicate ids could come from loading state
                    if duplicate {
                        vc.delete_vehicle(vehicle.cast::<SUMOVehicle>(), true, false);
                        continue;
                    }
                    vehicle.m().set_segment(self.mySegment, 0); // needed or vehicle will not be registered (XXX why?)
                    vehicle.m().set_event_time(currentTime, true); // XXX superfluous?
                    // move vehicle forward when the route does not begin at the calibrator's edge
                    let myedge: Ptr<MSEdge> = Ptr::new(self.mySegment.get_edge());
                    let mut atDest = false;
                    while vehicle.m().get_edge() != myedge {
                        // let the vehicle move to the next edge
                        atDest = vehicle.m().move_route_pointer();
                    }
                    // insert vehicle into the net
                    if atDest || !self.try_emit(self.mySegment, vehicle) {
                        vc.delete_vehicle(vehicle.cast::<SUMOVehicle>(), true, false);
                        break;
                    }
                    self.myInserted += 1;
                    adaptedNum += 1;
                }
            }
            // we only remove vehicles once we really have to
            while totalWishedNum < adaptedNum {
                let seg = self.mySegment;
                if !seg.m().vaporize_any_car(currentTime, Ptr::from_mut(&mut self.base3)) {
                    // @bug: short edges may be jumped in a single step, giving us no chance to remove a vehicle
                    break;
                }
                self.myRemoved += 1;
                adaptedNum -= 1;
            }
        }
        if self.myIntervals[idx].end <= currentTime + self.myFrequency {
            self.interval_end();
        }
        //assert(!invalidJam());
        if self.invalid_jam() {
            write_warningf!("DEBUG: Could not clear jam at calibrator '%' at time=%.", self.base3.get_id(), utils::common::time::time2string(currentTime));
        }
        self.myFrequency
    }

    /// C++: `bool METriggeredCalibrator::tryEmit(MESegment *, MEVehicle *)` [protected]
    // bool
    // METriggeredCalibrator::tryEmit(MESegment* s, MEVehicle* vehicle) {
    //     if (s->initialise(vehicle, vehicle->getParameter().depart)) {
    //         if (!MSNet::getInstance()->getVehicleControl().addVehicle(vehicle->getID(), vehicle)) {
    //             throw ProcessError(TLF("Emission of vehicle '%' in calibrator '%' failed!", vehicle->getID(), getID()));
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn try_emit(&mut self, s: Ptr<MESegment>, vehicle: Ptr<MEVehicle>) -> bool {
        if s.m().initialise(vehicle, vehicle.m().get_parameter().depart) {
            let id = vehicle.m().get_id().to_string();
            if !MSNet::get_instance().m().get_vehicle_control().add_vehicle(&id, vehicle.cast::<SUMOVehicle>()) {
                std::panic::panic_any(ProcessError::new(format!("Emission of vehicle '{}' in calibrator '{}' failed!", id, self.base3.get_id())));
            }
            return true;
        }
        false
    }

    /// C++: `int METriggeredCalibrator::passed() const` [const, virtual, protected]
    // inline int passed() const {
    //         // calibrator measures at start of segment
    //         return myEdgeMeanData.nVehEntered + myEdgeMeanData.nVehDeparted - myEdgeMeanData.nVehVaporized;
    //     }
    #[allow(unused_variables)]
    pub fn passed(&self) -> i32 {
        // calibrator measures at start of segment
        self.myEdgeMeanData.nVehEntered + self.myEdgeMeanData.nVehDeparted - self.myEdgeMeanData.nVehVaporized
    }

    /// C++: `bool METriggeredCalibrator::invalidJam() const` [const, protected]
    /// returns whether the segment is jammed although it should not be
    // bool
    // METriggeredCalibrator::invalidJam() const {
    //     if (mySegment->getBruttoOccupancy() == 0.) {
    //         return false;
    //     }
    //     // maxSpeed reflects the calibration target
    //     const bool toSlow = mySegment->getMeanSpeed() < myInvalidJamThreshold * mySegment->getEdge().getSpeedLimit();
    //     return toSlow && remainingVehicleCapacity() < maximumInflow();
    // }
    #[allow(unused_variables)]
    pub fn invalid_jam(&self) -> bool {
        if self.mySegment.get_brutto_occupancy() == 0. {
            return false;
        }
        // maxSpeed reflects the calibration target
        let toSlow = self.mySegment.get_mean_speed(true) < self.myInvalidJamThreshold * self.mySegment.get_edge().get_speed_limit();
        toSlow && self.remaining_vehicle_capacity() < self.maximum_inflow()
    }

    /// C++: `int METriggeredCalibrator::remainingVehicleCapacity() const` [const, protected]
    /// returns the number of vehicles (of the current type) that still fit onto the segment
    // int
    // METriggeredCalibrator::remainingVehicleCapacity() const {
    //     const SUMOVehicleParameter* pars = myCurrentStateInterval->vehicleParameter;
    //     const MSVehicleType* vtype = MSNet::getInstance()->getVehicleControl().getVType(pars->vtypeid);
    //     return mySegment->remainingVehicleCapacity(vtype->getLengthWithGap());
    // }
    #[allow(unused_variables)]
    pub fn remaining_vehicle_capacity(&self) -> i32 {
        let pars = self.myIntervals[self.myCurrentStateInterval].vehicleParameter;
        let vtype = MSNet::get_instance().m().get_vehicle_control().get_v_type(&pars.vtypeid, Ptr::null(), false);
        self.mySegment.remaining_vehicle_capacity(vtype.get_length_with_gap())
    }

    /// C++: `void METriggeredCalibrator::reset()` [virtual, protected]
    /// reset collected vehicle data
    // void
    // METriggeredCalibrator::reset() {
    //     myEdgeMeanData.reset();
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.myEdgeMeanData.reset(false);
    }

    /// C++: `void METriggeredCalibrator::updateMeanData()` [virtual, protected]
    /// do nothing
    // void updateMeanData() {}
    #[allow(unused_variables)]
    pub fn update_mean_data(&mut self) {
        // C++ body is empty
    }

    /// C++: `int METriggeredCalibrator::maximumInflow() const` [const, protected]
    /// returns the maximum number of vehicles that could enter from upstream until the calibrator is activated again
    // inline int maximumInflow() const {
    //         return (int)std::ceil((double)myFrequency / (double)mySegment->getMinimumHeadwayTime());
    //     }
    #[allow(unused_variables)]
    pub fn maximum_inflow(&self) -> i32 {
        ((self.myFrequency as f64) / (self.mySegment.get_minimum_headway_time() as f64)).ceil() as i32
    }

}

