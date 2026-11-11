//! Port of mesosim/METriggeredCalibrator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Calibrates the flow on a segment to a specified one
/// C++ class `METriggeredCalibrator` (mesosim/METriggeredCalibrator.h)
pub struct METriggeredCalibrator {
    /// C++ base class `MSCalibrator`
    pub base: MSCalibrator,
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
    pub fn new(id: &str, edge: Ptr<MSEdge>, pos: f64, aXMLFilename: &str, outputFilename: &str, freq: SUMOTime, length: f64, probe: Ptr<MSRouteProbe>, invalidJamThreshold: f64, vTypes: &str) -> Self {
        todo!("port METriggeredCalibrator::METriggeredCalibrator")
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
        todo!("port METriggeredCalibrator::~METriggeredCalibrator")
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
        todo!("port METriggeredCalibrator::execute")
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
        todo!("port METriggeredCalibrator::tryEmit")
    }

    /// C++: `int METriggeredCalibrator::passed() const` [const, virtual, protected]
    // inline int passed() const {
    //         // calibrator measures at start of segment
    //         return myEdgeMeanData.nVehEntered + myEdgeMeanData.nVehDeparted - myEdgeMeanData.nVehVaporized;
    //     }
    #[allow(unused_variables)]
    pub fn passed(&self) -> i32 {
        todo!("port METriggeredCalibrator::passed")
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
        todo!("port METriggeredCalibrator::invalidJam")
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
        todo!("port METriggeredCalibrator::remainingVehicleCapacity")
    }

    /// C++: `void METriggeredCalibrator::reset()` [virtual, protected]
    /// reset collected vehicle data
    // void
    // METriggeredCalibrator::reset() {
    //     myEdgeMeanData.reset();
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port METriggeredCalibrator::reset")
    }

    /// C++: `void METriggeredCalibrator::updateMeanData()` [virtual, protected]
    /// do nothing
    // void updateMeanData() {}
    #[allow(unused_variables)]
    pub fn update_mean_data(&mut self) {
        todo!("port METriggeredCalibrator::updateMeanData")
    }

    /// C++: `int METriggeredCalibrator::maximumInflow() const` [const, protected]
    /// returns the maximum number of vehicles that could enter from upstream until the calibrator is activated again
    // inline int maximumInflow() const {
    //         return (int)std::ceil((double)myFrequency / (double)mySegment->getMinimumHeadwayTime());
    //     }
    #[allow(unused_variables)]
    pub fn maximum_inflow(&self) -> i32 {
        todo!("port METriggeredCalibrator::maximumInflow")
    }

}

