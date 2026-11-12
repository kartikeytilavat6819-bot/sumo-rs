//! Port of microsim/output/MSMeanData_Net.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::INVALID_DOUBLE;
use utils::geom::NUMERICAL_EPS;

/// C++ MAX2 (StdDefs.h): (a) > (b) ? (a) : (b)
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
/// C++ MIN2 (StdDefs.h): (a) < (b) ? (a) : (b)
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
/// C++ macro `SIMSTEP`: MSNet::getInstance()->getCurrentTimeStep()
fn sim_step() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}
/// C++ `static_cast<MSVehicle&>(veh).getLane()` for a vehicle (callers check isVehicle() first)
fn sumo_vehicle_lane(veh: &SUMOTrafficObject) -> Ptr<MSLane> {
    Ptr::new(veh).cast::<MSVehicle>().get_lane()
}
/// C++ virtual `SUMOTrafficObject::getLane()`: dispatched by hand on the object kind
// shortcut: containers are not dispatched (null lane), upgrade when containers reach meandata
fn traffic_object_lane(veh: &SUMOTrafficObject) -> Ptr<MSLane> {
    if veh.is_vehicle() {
        Ptr::new(veh).cast::<MSVehicle>().get_lane()
    } else if veh.is_person() {
        Ptr::new(veh).cast::<MSTransportable>().get_lane()
    } else {
        Ptr::null()
    }
}

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData_Net::MSLaneMeanDataValues` (microsim/output/MSMeanData_Net.h)
#[repr(C)]
pub struct MSMeanData_Net_MSLaneMeanDataValues {
    /// C++ base class `MSMeanData::MeanDataValues`
    pub base: MSMeanData_MeanDataValues,
    /// The number of vehicles that were emitted on the lane
    pub nVehDeparted: i32,
    /// The number of vehicles that finished on the lane
    pub nVehArrived: i32,
    /// The number of vehicles that entered this lane within the sample interval
    pub nVehEntered: i32,
    /// The number of vehicles that left this lane within the sample interval
    pub nVehLeft: i32,
    /// The number of vehicles that left this lane via vaporization within the sample interval
    pub nVehVaporized: i32,
    /// The number of vehicles that left this lane via teleporting within the sample interval
    pub nVehTeleported: i32,
    /// The number of vehicle probes with small speed
    pub waitSeconds: f64,
    /// The time loss accrued by vehicle probes
    pub timeLoss: f64,
    /// The number of vehicles that changed from this lane
    pub nVehLaneChangeFrom: i32,
    /// The number of vehicles that changed to this lane
    pub nVehLaneChangeTo: i32,
    /// The number of vehicle probes regarding the vehicle front
    pub frontSampleSeconds: f64,
    /// The travelled distance regarding the vehicle front
    pub frontTravelledDistance: f64,
    /// The sum of the lengths the vehicles had
    pub vehLengthSum: f64,
    /// The sum of the occupation of the lane
    pub occupationSum: f64,
    /// minimal vehicle length in the current interval (used to determine a maximal density, see #3265)
    pub minimalVehicleLength: f64,
    /// The meandata parent
    pub myParent: Ptr<MSMeanData_Net>,
}

impl std::ops::Deref for MSMeanData_Net_MSLaneMeanDataValues {
    type Target = MSMeanData_MeanDataValues;
    fn deref(&self) -> &MSMeanData_MeanDataValues {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Net_MSLaneMeanDataValues {
    fn deref_mut(&mut self) -> &mut MSMeanData_MeanDataValues {
        &mut self.base
    }
}

impl MSMeanData_Net_MSLaneMeanDataValues {
    /// C++: `void MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane *const, const double, const bool, const MSMeanData_Net *)`
    /// Constructor
    // MSMeanData_Net::MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane* const lane,
    //         const double length,
    //         const bool doAdd,
    //         const MSMeanData_Net* parent)
    //     : MSMeanData::MeanDataValues(lane, length, doAdd, parent),
    //       nVehDeparted(0), nVehArrived(0), nVehEntered(0), nVehLeft(0),
    //       nVehVaporized(0), nVehTeleported(0), waitSeconds(0), timeLoss(0),
    //       nVehLaneChangeFrom(0), nVehLaneChangeTo(0),
    //       frontSampleSeconds(0), frontTravelledDistance(0),
    //       vehLengthSum(0), occupationSum(0),
    //       minimalVehicleLength(INVALID_DOUBLE),
    //       myParent(parent) {}
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, length: f64, doAdd: bool, parent: Ptr<MSMeanData_Net>) -> Self {
        Self {
            base: MSMeanData_MeanDataValues::new(lane, length, doAdd, parent.cast::<MSMeanData>()),
            nVehDeparted: 0,
            nVehArrived: 0,
            nVehEntered: 0,
            nVehLeft: 0,
            nVehVaporized: 0,
            nVehTeleported: 0,
            waitSeconds: 0.0,
            timeLoss: 0.0,
            nVehLaneChangeFrom: 0,
            nVehLaneChangeTo: 0,
            frontSampleSeconds: 0.0,
            frontTravelledDistance: 0.0,
            vehLengthSum: 0.0,
            occupationSum: 0.0,
            minimalVehicleLength: INVALID_DOUBLE,
            myParent: parent,
        }
    }

    /// C++: `void MSLaneMeanDataValues::~MSLaneMeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Net::MSLaneMeanDataValues::~MSLaneMeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty.
    }

    /// C++: `void MSLaneMeanDataValues::reset(bool)` [virtual]
    /// Resets values so they may be used for the next interval
    // void
    // MSMeanData_Net::MSLaneMeanDataValues::reset(bool) {
    //     nVehDeparted = 0;
    //     nVehArrived = 0;
    //     nVehEntered = 0;
    //     nVehLeft = 0;
    //     nVehVaporized = 0;
    //     nVehTeleported = 0;
    //     nVehLaneChangeFrom = 0;
    //     nVehLaneChangeTo = 0;
    //     sampleSeconds = 0.;
    //     travelledDistance = 0;
    //     waitSeconds = 0;
    //     timeLoss = 0;
    //     frontSampleSeconds = 0;
    //     frontTravelledDistance = 0;
    //     vehLengthSum = 0;
    //     occupationSum = 0;
    //     minimalVehicleLength = INVALID_DOUBLE;
    //     resetTime = SIMSTEP;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self, afterWrite: bool) {
        let _ = afterWrite;
        self.nVehDeparted = 0;
        self.nVehArrived = 0;
        self.nVehEntered = 0;
        self.nVehLeft = 0;
        self.nVehVaporized = 0;
        self.nVehTeleported = 0;
        self.nVehLaneChangeFrom = 0;
        self.nVehLaneChangeTo = 0;
        self.sampleSeconds = 0.0;
        self.travelledDistance = 0.0;
        self.waitSeconds = 0.0;
        self.timeLoss = 0.0;
        self.frontSampleSeconds = 0.0;
        self.frontTravelledDistance = 0.0;
        self.vehLengthSum = 0.0;
        self.occupationSum = 0.0;
        self.minimalVehicleLength = INVALID_DOUBLE;
        self.resetTime = sim_step();
    }

    /// C++: `void MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues &) const` [const, virtual]
    /// Add the values of this to the given one and store them there
    // void
    // MSMeanData_Net::MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues& val) const {
    //     MSLaneMeanDataValues& v = (MSLaneMeanDataValues&) val;
    //     v.nVehDeparted += nVehDeparted;
    //     v.nVehArrived += nVehArrived;
    //     v.nVehEntered += nVehEntered;
    //     v.nVehLeft += nVehLeft;
    //     v.nVehVaporized += nVehVaporized;
    //     v.nVehTeleported += nVehTeleported;
    //     v.nVehLaneChangeFrom += nVehLaneChangeFrom;
    //     v.nVehLaneChangeTo += nVehLaneChangeTo;
    //     v.sampleSeconds += sampleSeconds;
    //     v.travelledDistance += travelledDistance;
    //     v.waitSeconds += waitSeconds;
    //     v.timeLoss += timeLoss;
    //     v.frontSampleSeconds += frontSampleSeconds;
    //     v.frontTravelledDistance += frontTravelledDistance;
    //     v.vehLengthSum += vehLengthSum;
    //     v.occupationSum += occupationSum;
    //     if (v.minimalVehicleLength == INVALID_DOUBLE) {
    //         v.minimalVehicleLength = minimalVehicleLength;
    //     } else {
    //         v.minimalVehicleLength = MIN2(minimalVehicleLength, v.minimalVehicleLength);
    //     }
    //     if (mySubData != nullptr) {
    //         mySubData->addTo(*val.getSubData());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        let v: Ptr<MSMeanData_Net_MSLaneMeanDataValues> = Ptr::from_mut(val).cast::<MSMeanData_Net_MSLaneMeanDataValues>();
        let v = v.m();
        v.nVehDeparted += self.nVehDeparted;
        v.nVehArrived += self.nVehArrived;
        v.nVehEntered += self.nVehEntered;
        v.nVehLeft += self.nVehLeft;
        v.nVehVaporized += self.nVehVaporized;
        v.nVehTeleported += self.nVehTeleported;
        v.nVehLaneChangeFrom += self.nVehLaneChangeFrom;
        v.nVehLaneChangeTo += self.nVehLaneChangeTo;
        v.sampleSeconds += self.sampleSeconds;
        v.travelledDistance += self.travelledDistance;
        v.waitSeconds += self.waitSeconds;
        v.timeLoss += self.timeLoss;
        v.frontSampleSeconds += self.frontSampleSeconds;
        v.frontTravelledDistance += self.frontTravelledDistance;
        v.vehLengthSum += self.vehLengthSum;
        v.occupationSum += self.occupationSum;
        if v.minimalVehicleLength == INVALID_DOUBLE {
            v.minimalVehicleLength = self.minimalVehicleLength;
        } else {
            v.minimalVehicleLength = min2(self.minimalVehicleLength, v.minimalVehicleLength);
        }
        if !self.mySubData.is_null() {
            // shortcut: C++ dispatches mySubData->addTo() virtually; MeanDataValues::add_to has no dispatch and panics, upgrade when sub-data types are known
            self.mySubData.add_to(val.get_sub_data().m());
        }
    }

    /// C++: `bool MSLaneMeanDataValues::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Called if the vehicle leaves the reminder's lane
    // bool
    // MSMeanData_Net::MSLaneMeanDataValues::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if ((myParent == nullptr || myParent->vehicleApplies(veh)) && (
    //                 getLane() == nullptr || !veh.isVehicle() || getLane() == static_cast<MSVehicle&>(veh).getLane())) {
    // #ifdef HAVE_FOX
    //         ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //         if (MSGlobals::gUseMesoSim) {
    //             removeFromVehicleUpdateValues(veh);
    //         }
    //         if (reason == MSMoveReminder::NOTIFICATION_ARRIVED) {
    //             ++nVehArrived;
    //         } else if (reason == MSMoveReminder::NOTIFICATION_LANE_CHANGE) {
    //             ++nVehLaneChangeFrom;
    //         } else if (myParent == nullptr || reason != MSMoveReminder::NOTIFICATION_SEGMENT) {
    //             if (reason != MSMoveReminder::NOTIFICATION_TELEPORT_CONTINUATION && reason != MSMoveReminder::NOTIFICATION_TELEPORT_CONTINUATION_ARRIVED) {
    //                 // if the entered this edge via teleport, we do not want to count it
    //                 ++nVehLeft;
    //             }
    //             if (reason == MSMoveReminder::NOTIFICATION_TELEPORT || reason == MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED) {
    //                 ++nVehTeleported;
    //             } else if (reason >= MSMoveReminder::NOTIFICATION_VAPORIZED_CALIBRATOR) {
    //                 ++nVehVaporized;
    //             }
    //         }
    //     }
    //     if (MSGlobals::gUseMesoSim || !veh.isVehicle()) {
    //         // only microsim vehicles keep the reminder until their back has fully left the edge/lane
    //         return false;
    //     }
    //     return reason == MSMoveReminder::NOTIFICATION_JUNCTION;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if (self.myParent.is_null() || self.myParent.vehicle_applies(veh))
            && (self.get_lane().is_null() || !veh.is_vehicle() || self.get_lane() == sumo_vehicle_lane(veh))
        {
            if MSGlobals::use_meso_sim() {
                self.remove_from_vehicle_update_values(veh);
            }
            if reason == MSMoveReminder_Notification::NOTIFICATION_ARRIVED {
                self.nVehArrived += 1;
            } else if reason == MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE {
                self.nVehLaneChangeFrom += 1;
            } else if self.myParent.is_null() || reason != MSMoveReminder_Notification::NOTIFICATION_SEGMENT {
                if reason != MSMoveReminder_Notification::NOTIFICATION_TELEPORT_CONTINUATION && reason != MSMoveReminder_Notification::NOTIFICATION_TELEPORT_CONTINUATION_ARRIVED {
                    // if the entered this edge via teleport, we do not want to count it
                    self.nVehLeft += 1;
                }
                if reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT || reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED {
                    self.nVehTeleported += 1;
                } else if reason >= MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_CALIBRATOR {
                    self.nVehVaporized += 1;
                }
            }
        }
        if MSGlobals::use_meso_sim() || !veh.is_vehicle() {
            // only microsim vehicles keep the reminder until their back has fully left the edge/lane
            return false;
        }
        reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION
    }

    /// C++: `bool MSLaneMeanDataValues::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Computes current values and adds them to their sums
    // bool
    // MSMeanData_Net::MSLaneMeanDataValues::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_NOTIFY_ENTER
    //     std::cout << "\n" << SIMTIME << " MSMeanData_Net::MSLaneMeanDataValues: veh '" << veh.getID() << "' enters lane '" << enteredLane->getID() << "'" << std::endl;
    // #else
    //     UNUSED_PARAMETER(enteredLane);
    // #endif
    //     if (myParent == nullptr || myParent->vehicleApplies(veh)) {
    //         if (getLane() == nullptr || !veh.isVehicle() || getLane() == static_cast<MSVehicle&>(veh).getLane()) {
    // #ifdef HAVE_FOX
    //             ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //             if (reason == MSMoveReminder::NOTIFICATION_DEPARTED) {
    //                 ++nVehDeparted;
    //             } else if (reason == MSMoveReminder::NOTIFICATION_LANE_CHANGE) {
    //                 ++nVehLaneChangeTo;
    //             } else if (reason != MSMoveReminder::NOTIFICATION_TELEPORT
    //                     && (myParent == nullptr || reason != MSMoveReminder::NOTIFICATION_SEGMENT)) {
    //                 ++nVehEntered;
    //             }
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if self.myParent.is_null() || self.myParent.vehicle_applies(veh) {
            if self.get_lane().is_null() || !veh.is_vehicle() || self.get_lane() == sumo_vehicle_lane(veh) {
                if reason == MSMoveReminder_Notification::NOTIFICATION_DEPARTED {
                    self.nVehDeparted += 1;
                } else if reason == MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE {
                    self.nVehLaneChangeTo += 1;
                } else if reason != MSMoveReminder_Notification::NOTIFICATION_TELEPORT && (self.myParent.is_null() || reason != MSMoveReminder_Notification::NOTIFICATION_SEGMENT) {
                    self.nVehEntered += 1;
                }
            }
            return true;
        }
        false
    }

    /// C++: `bool MSLaneMeanDataValues::isEmpty() const` [const, virtual]
    // bool
    // MSMeanData_Net::MSLaneMeanDataValues::isEmpty() const {
    //     return sampleSeconds == 0 && nVehDeparted == 0 && nVehArrived == 0 && nVehEntered == 0
    //            && nVehLeft == 0 && nVehVaporized == 0 && nVehTeleported == 0 && nVehLaneChangeFrom == 0 && nVehLaneChangeTo == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        self.sampleSeconds == 0. && self.nVehDeparted == 0 && self.nVehArrived == 0 && self.nVehEntered == 0
            && self.nVehLeft == 0 && self.nVehVaporized == 0 && self.nVehTeleported == 0 && self.nVehLaneChangeFrom == 0 && self.nVehLaneChangeTo == 0
    }

    /// C++: `void MSLaneMeanDataValues::write(OutputDevice &, const SumoXMLAttrMask &, const SUMOTime, const int, const double, const double, const int) const` [const, virtual]
    /// Writes output values into the given stream
    // void
    // MSMeanData_Net::MSLaneMeanDataValues::write(OutputDevice& dev, const SumoXMLAttrMask& attributeMask, const SUMOTime period,
    //         const int numLanes, const double speedLimit, const double defaultTravelTime, const int numVehicles) const {
    //
    //     double density = frontSampleSeconds / STEPS2TIME(period) * 1000. / myLaneLength;
    //     double overlapDensity = sampleSeconds / STEPS2TIME(period) * 1000. / myLaneLength;
    //     if (MSGlobals::gLateralResolution < 0) {
    //         // avoid exceeding upper bound
    //         density = MIN2(density, 1000 * (double)numLanes / MAX2(minimalVehicleLength, NUMERICAL_EPS));
    //         overlapDensity = MIN2(overlapDensity, 1000 * (double)numLanes / MAX2(minimalVehicleLength, NUMERICAL_EPS));
    //     }
    //     const double laneDensity = density / (double)numLanes;
    //     const double occupancy = getOccupancy(period, numLanes);
    // #ifdef DEBUG_OCCUPANCY2
    //     // tests #3264
    //     if (occupancy > 100) {
    //         std::cout << SIMTIME << " Encountered bad occupancy: " << occupancy
    //                   << ", myLaneLength=" << myLaneLength << ", period=" << STEPS2TIME(period) << ", occupationSum=" << occupationSum
    //                   << std::endl;
    //     }
    //     // refs #3265
    //     std::cout << SIMTIME << "ID: " << getDescription() << " minVehicleLength=" << minimalVehicleLength
    //               << "\ndensity=" << density << "\n";
    // #endif
    //
    //     if (myParent == nullptr) {
    //         const double speed = sampleSeconds == 0 ? 0. : travelledDistance / sampleSeconds;
    //         const double frontSpeed = frontSampleSeconds == 0 ? 0. : frontTravelledDistance / frontSampleSeconds;
    //         dev.writeOptionalAttr(SUMO_ATTR_DENSITY, density, attributeMask, sampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_LANEDENSITY, laneDensity, attributeMask, sampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_OCCUPANCY, occupancy, attributeMask, sampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_WAITINGTIME, waitSeconds, attributeMask, sampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_TIMELOSS, timeLoss, attributeMask, sampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_SPEED, speed, attributeMask, sampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_SPEEDREL, speedLimit == 0. || sampleSeconds == 0 ? 0. : travelledDistance / sampleSeconds / speedLimit,
    //                               attributeMask, sampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_DEPARTED, nVehDeparted, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_ARRIVED, nVehArrived, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_ENTERED, nVehEntered, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_LEFT, nVehLeft, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_VAPORIZED, nVehVaporized, attributeMask, nVehVaporized == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_TELEPORTED, nVehTeleported, attributeMask, nVehTeleported == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_FLOW, density * frontSpeed * 3.6, attributeMask, frontSampleSeconds == 0);
    //         dev.writeOptionalAttr(SUMO_ATTR_DISTANCE, frontTravelledDistance, attributeMask);
    //         dev.closeTag();
    //         return;
    //     }
    //     const bool haveSamples = sampleSeconds > myParent->myMinSamples;
    //     const bool haveFrontSamples = frontSampleSeconds > myParent->myMinSamples;
    //     const bool haveSamplesOrDefault = haveSamples || defaultTravelTime >= 0.;
    //     bool haveTravelTime = haveSamplesOrDefault;
    //     double traveltime = myParent->myMaxTravelTime;
    //     if (haveSamples) {
    //         if (numVehicles > 0) {
    //             traveltime = sampleSeconds / numVehicles;
    //         } else {
    //             traveltime = myParent->myMaxTravelTime;
    //             if (frontTravelledDistance > NUMERICAL_EPS) {
    //                 traveltime = MIN2(traveltime, myLaneLength * frontSampleSeconds / frontTravelledDistance);
    //             } else if (defaultTravelTime >= 0.) {
    //                 traveltime = defaultTravelTime;
    //             } else {
    //                 haveTravelTime = false;
    //             }
    //         }
    //     } else if (defaultTravelTime >= 0.) {
    //         traveltime = defaultTravelTime;
    //     }
    //     dev.writeOptionalAttr(SUMO_ATTR_TRAVELTIME, traveltime, attributeMask, !haveTravelTime);
    //     double overlapTraveltime = myParent->myMaxTravelTime;
    //     if (travelledDistance > 0.) {
    //         // one vehicle has to drive lane length + vehicle length before it has left the lane
    //         // thus we need to scale with an extended length, approximated by lane length + average vehicle length
    //         overlapTraveltime = MIN2(overlapTraveltime, (myLaneLength + vehLengthSum / sampleSeconds) * sampleSeconds / travelledDistance);
    //     }
    //     dev.writeOptionalAttr(SUMO_ATTR_OVERLAPTRAVELTIME, overlapTraveltime, attributeMask, !haveSamples || numVehicles > 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_DENSITY, density, attributeMask, !haveSamples || numVehicles > 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_OVERLAPDENSITY, overlapDensity, attributeMask, sampleSeconds == 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_LANEDENSITY, laneDensity, attributeMask, !haveSamples || numVehicles > 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_OCCUPANCY, occupancy, attributeMask, !haveSamples || numVehicles > 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_WAITINGTIME, waitSeconds, attributeMask, !haveSamples);
    //     dev.writeOptionalAttr(SUMO_ATTR_TIMELOSS, timeLoss, attributeMask, !haveSamples);
    //     double speed = 0.;
    //     double frontSpeed = 0.;
    //     if (haveSamples) {
    //         speed = travelledDistance / sampleSeconds;
    //     } else if (defaultTravelTime > 0.) {
    //         speed = myLaneLength / defaultTravelTime;
    //     }
    //     if (haveFrontSamples) {
    //         frontSpeed = frontTravelledDistance / frontSampleSeconds;
    //     } else if (defaultTravelTime > 0.) {
    //         frontSpeed = myLaneLength / defaultTravelTime;
    //     }
    //     dev.writeOptionalAttr(SUMO_ATTR_SPEED, speed, attributeMask, !haveSamplesOrDefault);
    //     dev.writeOptionalAttr(SUMO_ATTR_SPEEDREL, speedLimit == 0. ? 0. : speed / speedLimit, attributeMask, !haveSamplesOrDefault);
    //     dev.writeOptionalAttr(SUMO_ATTR_DEPARTED, nVehDeparted, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_ARRIVED, nVehArrived, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_ENTERED, nVehEntered, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_LEFT, nVehLeft, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_LANECHANGEDFROM, nVehLaneChangeFrom, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_LANECHANGEDTO, nVehLaneChangeTo, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_VAPORIZED, nVehVaporized, attributeMask, nVehVaporized == 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_TELEPORTED, nVehTeleported, attributeMask, nVehTeleported == 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_FLOW, density * frontSpeed * 3.6, attributeMask, !haveSamples || numVehicles == 0);
    //     dev.writeOptionalAttr(SUMO_ATTR_DISTANCE, frontTravelledDistance, attributeMask);
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        let mask = *attributeMask;
        let mut density = self.frontSampleSeconds / steps2time(period) * 1000. / self.myLaneLength;
        let mut overlapDensity = self.sampleSeconds / steps2time(period) * 1000. / self.myLaneLength;
        if MSGlobals::lateral_resolution() < 0. {
            // avoid exceeding upper bound
            density = min2(density, 1000. * numLanes as f64 / max2(self.minimalVehicleLength, NUMERICAL_EPS));
            overlapDensity = min2(overlapDensity, 1000. * numLanes as f64 / max2(self.minimalVehicleLength, NUMERICAL_EPS));
        }
        let laneDensity = density / numLanes as f64;
        let occupancy = self.get_occupancy(period, numLanes);

        if self.myParent.is_null() {
            let speed = if self.sampleSeconds == 0. { 0. } else { self.travelledDistance / self.sampleSeconds };
            let frontSpeed = if self.frontSampleSeconds == 0. { 0. } else { self.frontTravelledDistance / self.frontSampleSeconds };
            dev.write_optional_attr_mask(SUMO_ATTR_DENSITY, &density, mask, self.sampleSeconds == 0.);
            dev.write_optional_attr_mask(SUMO_ATTR_LANEDENSITY, &laneDensity, mask, self.sampleSeconds == 0.);
            dev.write_optional_attr_mask(SUMO_ATTR_OCCUPANCY, &occupancy, mask, self.sampleSeconds == 0.);
            dev.write_optional_attr_mask(SUMO_ATTR_WAITINGTIME, &self.waitSeconds, mask, self.sampleSeconds == 0.);
            dev.write_optional_attr_mask(SUMO_ATTR_TIMELOSS, &self.timeLoss, mask, self.sampleSeconds == 0.);
            dev.write_optional_attr_mask(SUMO_ATTR_SPEED, &speed, mask, self.sampleSeconds == 0.);
            let speedRel = if speedLimit == 0. || self.sampleSeconds == 0. { 0. } else { self.travelledDistance / self.sampleSeconds / speedLimit };
            dev.write_optional_attr_mask(SUMO_ATTR_SPEEDREL, &speedRel, mask, self.sampleSeconds == 0.);
            dev.write_optional_attr_mask(SUMO_ATTR_DEPARTED, &self.nVehDeparted, mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_ARRIVED, &self.nVehArrived, mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_ENTERED, &self.nVehEntered, mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_LEFT, &self.nVehLeft, mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_VAPORIZED, &self.nVehVaporized, mask, self.nVehVaporized == 0);
            dev.write_optional_attr_mask(SUMO_ATTR_TELEPORTED, &self.nVehTeleported, mask, self.nVehTeleported == 0);
            let flow = density * frontSpeed * 3.6;
            dev.write_optional_attr_mask(SUMO_ATTR_FLOW, &flow, mask, self.frontSampleSeconds == 0.);
            dev.write_optional_attr_mask(SUMO_ATTR_DISTANCE, &self.frontTravelledDistance, mask, false);
            dev.close_tag("");
            return;
        }
        let haveSamples = self.sampleSeconds > self.myParent.myMinSamples;
        let haveFrontSamples = self.frontSampleSeconds > self.myParent.myMinSamples;
        let haveSamplesOrDefault = haveSamples || defaultTravelTime >= 0.;
        let mut haveTravelTime = haveSamplesOrDefault;
        let mut traveltime = self.myParent.myMaxTravelTime;
        if haveSamples {
            if numVehicles > 0 {
                traveltime = self.sampleSeconds / numVehicles as f64;
            } else {
                traveltime = self.myParent.myMaxTravelTime;
                if self.frontTravelledDistance > NUMERICAL_EPS {
                    traveltime = min2(traveltime, self.myLaneLength * self.frontSampleSeconds / self.frontTravelledDistance);
                } else if defaultTravelTime >= 0. {
                    traveltime = defaultTravelTime;
                } else {
                    haveTravelTime = false;
                }
            }
        } else if defaultTravelTime >= 0. {
            traveltime = defaultTravelTime;
        }
        dev.write_optional_attr_mask(SUMO_ATTR_TRAVELTIME, &traveltime, mask, !haveTravelTime);
        let mut overlapTraveltime = self.myParent.myMaxTravelTime;
        if self.travelledDistance > 0. {
            // one vehicle has to drive lane length + vehicle length before it has left the lane
            // thus we need to scale with an extended length, approximated by lane length + average vehicle length
            overlapTraveltime = min2(overlapTraveltime, (self.myLaneLength + self.vehLengthSum / self.sampleSeconds) * self.sampleSeconds / self.travelledDistance);
        }
        dev.write_optional_attr_mask(SUMO_ATTR_OVERLAPTRAVELTIME, &overlapTraveltime, mask, !haveSamples || numVehicles > 0);
        dev.write_optional_attr_mask(SUMO_ATTR_DENSITY, &density, mask, !haveSamples || numVehicles > 0);
        dev.write_optional_attr_mask(SUMO_ATTR_OVERLAPDENSITY, &overlapDensity, mask, self.sampleSeconds == 0.);
        dev.write_optional_attr_mask(SUMO_ATTR_LANEDENSITY, &laneDensity, mask, !haveSamples || numVehicles > 0);
        dev.write_optional_attr_mask(SUMO_ATTR_OCCUPANCY, &occupancy, mask, !haveSamples || numVehicles > 0);
        dev.write_optional_attr_mask(SUMO_ATTR_WAITINGTIME, &self.waitSeconds, mask, !haveSamples);
        dev.write_optional_attr_mask(SUMO_ATTR_TIMELOSS, &self.timeLoss, mask, !haveSamples);
        let mut speed = 0.;
        let mut frontSpeed = 0.;
        if haveSamples {
            speed = self.travelledDistance / self.sampleSeconds;
        } else if defaultTravelTime > 0. {
            speed = self.myLaneLength / defaultTravelTime;
        }
        if haveFrontSamples {
            frontSpeed = self.frontTravelledDistance / self.frontSampleSeconds;
        } else if defaultTravelTime > 0. {
            frontSpeed = self.myLaneLength / defaultTravelTime;
        }
        dev.write_optional_attr_mask(SUMO_ATTR_SPEED, &speed, mask, !haveSamplesOrDefault);
        let speedRel = if speedLimit == 0. { 0. } else { speed / speedLimit };
        dev.write_optional_attr_mask(SUMO_ATTR_SPEEDREL, &speedRel, mask, !haveSamplesOrDefault);
        dev.write_optional_attr_mask(SUMO_ATTR_DEPARTED, &self.nVehDeparted, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_ARRIVED, &self.nVehArrived, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_ENTERED, &self.nVehEntered, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_LEFT, &self.nVehLeft, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_LANECHANGEDFROM, &self.nVehLaneChangeFrom, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_LANECHANGEDTO, &self.nVehLaneChangeTo, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_VAPORIZED, &self.nVehVaporized, mask, self.nVehVaporized == 0);
        dev.write_optional_attr_mask(SUMO_ATTR_TELEPORTED, &self.nVehTeleported, mask, self.nVehTeleported == 0);
        let flow = density * frontSpeed * 3.6;
        dev.write_optional_attr_mask(SUMO_ATTR_FLOW, &flow, mask, !haveSamples || numVehicles == 0);
        dev.write_optional_attr_mask(SUMO_ATTR_DISTANCE, &self.frontTravelledDistance, mask, false);
    }

    /// C++: `double MSLaneMeanDataValues::getAttributeValue(SumoXMLAttr, const SUMOTime, const double, const double) const` [const, virtual]
    /// return attribute value
    // double
    // MSMeanData_Net::MSLaneMeanDataValues::getAttributeValue(SumoXMLAttr a,
    //         const SUMOTime period, const double numLanes, const double speedLimit) const {
    //     /// @todo: remove redundancy in derived values (density, laneDensity)
    //     switch (a) {
    //         case SUMO_ATTR_DENSITY:
    //             return MIN2(frontSampleSeconds / STEPS2TIME(period) * (double) 1000 / myLaneLength,
    //                         1000. * numLanes / MAX2(minimalVehicleLength, NUMERICAL_EPS));
    //         case SUMO_ATTR_LANEDENSITY: {
    //             const double density = MIN2(frontSampleSeconds / STEPS2TIME(period) * (double) 1000 / myLaneLength,
    //                                         1000. * numLanes / MAX2(minimalVehicleLength, NUMERICAL_EPS));
    //             return density / numLanes;
    //         }
    //         case SUMO_ATTR_OVERLAPDENSITY: {
    //             const double overlapDensity = MIN2(sampleSeconds / STEPS2TIME(period) * (double) 1000 / myLaneLength,
    //                                                1000. * numLanes / MAX2(minimalVehicleLength, NUMERICAL_EPS));
    //             return overlapDensity / numLanes;
    //         }
    //         case SUMO_ATTR_OCCUPANCY:
    //             return occupationSum / STEPS2TIME(period) / myLaneLength / numLanes * (double) 1000;
    //         case SUMO_ATTR_WAITINGTIME:
    //             return waitSeconds;
    //         case SUMO_ATTR_TIMELOSS:
    //             return timeLoss;
    //         case SUMO_ATTR_SPEED:
    //             return travelledDistance / sampleSeconds;
    //         case SUMO_ATTR_SPEEDREL:
    //             return speedLimit == 0. ? 0. : travelledDistance / sampleSeconds / speedLimit;
    //         case SUMO_ATTR_DEPARTED:
    //             return nVehDeparted;
    //         case SUMO_ATTR_ARRIVED:
    //             return nVehArrived;
    //         case SUMO_ATTR_ENTERED:
    //             return nVehEntered;
    //         case SUMO_ATTR_LEFT:
    //             return nVehLeft;
    //         case SUMO_ATTR_VAPORIZED:
    //             return nVehVaporized;
    //         case SUMO_ATTR_TELEPORTED:
    //             return nVehTeleported;
    //         case SUMO_ATTR_FLOW: {
    //             const double density = MIN2(frontSampleSeconds / STEPS2TIME(period) * (double) 1000 / myLaneLength,
    //                                         1000. * numLanes / MAX2(minimalVehicleLength, NUMERICAL_EPS));
    //             const double speed = frontTravelledDistance / frontSampleSeconds;
    //             return density * speed * 3.6;
    //         }
    //         case SUMO_ATTR_DISTANCE:
    //             return frontTravelledDistance;
    //         default:
    //             return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_value(&self, a: SumoXMLAttr, period: SUMOTime, numLanes: f64, speedLimit: f64) -> f64 {
        match a {
            SUMO_ATTR_DENSITY => min2(self.frontSampleSeconds / steps2time(period) * 1000. / self.myLaneLength, 1000. * numLanes / max2(self.minimalVehicleLength, NUMERICAL_EPS)),
            SUMO_ATTR_LANEDENSITY => {
                let density = min2(self.frontSampleSeconds / steps2time(period) * 1000. / self.myLaneLength, 1000. * numLanes / max2(self.minimalVehicleLength, NUMERICAL_EPS));
                density / numLanes
            }
            SUMO_ATTR_OVERLAPDENSITY => {
                let overlapDensity = min2(self.sampleSeconds / steps2time(period) * 1000. / self.myLaneLength, 1000. * numLanes / max2(self.minimalVehicleLength, NUMERICAL_EPS));
                overlapDensity / numLanes
            }
            SUMO_ATTR_OCCUPANCY => self.occupationSum / steps2time(period) / self.myLaneLength / numLanes * 1000.,
            SUMO_ATTR_WAITINGTIME => self.waitSeconds,
            SUMO_ATTR_TIMELOSS => self.timeLoss,
            SUMO_ATTR_SPEED => self.travelledDistance / self.sampleSeconds,
            SUMO_ATTR_SPEEDREL => if speedLimit == 0. { 0. } else { self.travelledDistance / self.sampleSeconds / speedLimit },
            SUMO_ATTR_DEPARTED => self.nVehDeparted as f64,
            SUMO_ATTR_ARRIVED => self.nVehArrived as f64,
            SUMO_ATTR_ENTERED => self.nVehEntered as f64,
            SUMO_ATTR_LEFT => self.nVehLeft as f64,
            SUMO_ATTR_VAPORIZED => self.nVehVaporized as f64,
            SUMO_ATTR_TELEPORTED => self.nVehTeleported as f64,
            SUMO_ATTR_FLOW => {
                let density = min2(self.frontSampleSeconds / steps2time(period) * 1000. / self.myLaneLength, 1000. * numLanes / max2(self.minimalVehicleLength, NUMERICAL_EPS));
                let speed = self.frontTravelledDistance / self.frontSampleSeconds;
                density * speed * 3.6
            }
            SUMO_ATTR_DISTANCE => self.frontTravelledDistance,
            _ => 0.,
        }
    }

    /// C++: `double MSLaneMeanDataValues::getOccupancy(SUMOTime, int) const` [const]
    // double
    // MSMeanData_Net::MSLaneMeanDataValues::getOccupancy(SUMOTime period, int numLanes) const {
    //     return occupationSum / STEPS2TIME(period) / myLaneLength / (double)numLanes * 100.;
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy(&self, period: SUMOTime, numLanes: i32) -> f64 {
        self.occupationSum / steps2time(period) / self.myLaneLength / numLanes as f64 * 100.
    }

    /// C++: `void MSLaneMeanDataValues::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves
    // void
    // MSMeanData_Net::MSLaneMeanDataValues::notifyMoveInternal(
    //     const SUMOTrafficObject& veh, const double frontOnLane,
    //     const double timeOnLane, const double /* meanSpeedFrontOnLane */,
    //     const double meanSpeedVehicleOnLane,
    //     const double travelledDistanceFrontOnLane,
    //     const double travelledDistanceVehicleOnLane,
    //     const double meanLengthOnLane) {
    // #ifdef DEBUG_OCCUPANCY
    //     if (DEBUG_COND2) {
    //         std::cout << SIMTIME << "\n  MSMeanData_Net::MSLaneMeanDataValues::notifyMoveInternal()\n"
    //                   << "  veh '" << veh.getID() << "' on edge '" << veh.getEdge()->getID() << "'"
    //                   << ", timeOnLane=" << timeOnLane
    //                   << ", meanSpeedVehicleOnLane=" << meanSpeedVehicleOnLane
    //                   << ",\ntravelledDistanceFrontOnLane=" << travelledDistanceFrontOnLane
    //                   << ", travelledDistanceVehicleOnLane=" << travelledDistanceVehicleOnLane
    //                   << ", meanLengthOnLane=" << meanLengthOnLane
    //                   << std::endl;
    //     }
    // #endif
    //     if (myParent != nullptr && !myParent->vehicleApplies(veh)) {
    //         return;
    //     }
    //     sampleSeconds += timeOnLane;
    //     travelledDistance += travelledDistanceVehicleOnLane;
    //     vehLengthSum += veh.getVehicleType().getLength() * timeOnLane;
    //     if (MSGlobals::gUseMesoSim) {
    //         // For the mesosim case no information on whether the vehicle was occupying
    //         // the lane with its whole length is available. We assume the whole length
    //         // Therefore this increment is taken out with more information on the vehicle movement.
    //         occupationSum += veh.getVehicleType().getLength() * timeOnLane;
    //     } else {
    //         // for the microsim case more elaborate calculation of the average length on the lane,
    //         // is taken out in notifyMove(), refs #153
    //         occupationSum += meanLengthOnLane * TS;
    //     }
    //     if (!veh.isStopped()) {
    //         if (myParent != nullptr && (meanSpeedVehicleOnLane < myParent->myHaltSpeed
    //                                     || (myParent->myHaltSpeedRel > 0 && meanSpeedVehicleOnLane / veh.getCurrentEdge()->getSpeedLimit(veh.getVClass()) < myParent->myHaltSpeedRel))) {
    //             waitSeconds += timeOnLane;
    //         } else if (MSGlobals::gUseMesoSim) {
    //             waitSeconds += STEPS2TIME(veh.getWaitingTime());
    //         }
    //         const double vmax = veh.getLane() == nullptr ? veh.getEdge()->getVehicleMaxSpeed(&veh) : veh.getLane()->getVehicleMaxSpeed(&veh);
    //         if (vmax > 0) {
    //             timeLoss += timeOnLane * MAX2(0.0, vmax - meanSpeedVehicleOnLane) / vmax;
    //         }
    //     }
    //     frontSampleSeconds += frontOnLane;
    //     frontTravelledDistance += travelledDistanceFrontOnLane;
    //     if (minimalVehicleLength == INVALID_DOUBLE) {
    //         minimalVehicleLength = veh.getVehicleType().getLengthWithGap();
    //     } else {
    //         minimalVehicleLength = MIN2(minimalVehicleLength, veh.getVehicleType().getLengthWithGap());
    //     }
    // #ifdef DEBUG_OCCUPANCY2
    //     // refs #3265
    //     std::cout << SIMTIME << "ID: " << getDescription() << " minVehicleLength=" << minimalVehicleLength << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, arg3: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        if !self.myParent.is_null() && !self.myParent.vehicle_applies(veh) {
            return;
        }
        self.sampleSeconds += timeOnLane;
        self.travelledDistance += travelledDistanceVehicleOnLane;
        self.vehLengthSum += veh.get_vehicle_type().get_length() * timeOnLane;
        if MSGlobals::use_meso_sim() {
            // For the mesosim case no information on whether the vehicle was occupying
            // the lane with its whole length is available. We assume the whole length
            // Therefore this increment is taken out with more information on the vehicle movement.
            self.occupationSum += veh.get_vehicle_type().get_length() * timeOnLane;
        } else {
            // for the microsim case more elaborate calculation of the average length on the lane,
            // is taken out in notifyMove(), refs #153
            self.occupationSum += meanLengthOnLane * ts();
        }
        if !veh.is_stopped() {
            if !self.myParent.is_null() && (meanSpeedVehicleOnLane < self.myParent.myHaltSpeed
                || (self.myParent.myHaltSpeedRel > 0. && meanSpeedVehicleOnLane / veh.get_current_edge().get_speed_limit_2(veh.get_v_class()) < self.myParent.myHaltSpeedRel))
            {
                self.waitSeconds += timeOnLane;
            } else if MSGlobals::use_meso_sim() {
                self.waitSeconds += steps2time(veh.get_waiting_time(false));
            }
            let vlane = traffic_object_lane(veh);
            let vmax = if vlane.is_null() {
                veh.get_edge().get_vehicle_max_speed(Ptr::new(veh))
            } else {
                vlane.get_vehicle_max_speed(Ptr::new(veh))
            };
            if vmax > 0. {
                self.timeLoss += timeOnLane * max2(0.0, vmax - meanSpeedVehicleOnLane) / vmax;
            }
        }
        self.frontSampleSeconds += frontOnLane;
        self.frontTravelledDistance += travelledDistanceFrontOnLane;
        let lengthWithGap = veh.get_vehicle_type().get_length_with_gap();
        if self.minimalVehicleLength == INVALID_DOUBLE {
            self.minimalVehicleLength = lengthWithGap;
        } else {
            self.minimalVehicleLength = min2(self.minimalVehicleLength, lengthWithGap);
        }
        let _ = arg3;
    }

}

/// Network state mean data collector for edges/lanes
/// C++ class `MSMeanData_Net` (microsim/output/MSMeanData_Net.h)
#[repr(C)]
pub struct MSMeanData_Net {
    /// C++ base class `MSMeanData`
    pub base: MSMeanData,
    /// the minimum speed to consider a vehicle halting
    pub myHaltSpeed: f64,
    /// the minimum relative speed to consider a vehicle halting
    pub myHaltSpeedRel: f64,
}

impl std::ops::Deref for MSMeanData_Net {
    type Target = MSMeanData;
    fn deref(&self) -> &MSMeanData {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Net {
    fn deref_mut(&mut self) -> &mut MSMeanData {
        &mut self.base
    }
}

impl MSMeanData_Net {
    /// C++: `void MSMeanData_Net::MSMeanData_Net(const std::string &, const SUMOTime, const SUMOTime, const bool, const std::string &, const bool, const bool, const int, const double, const double, const double, const double, const std::string &, const std::string &, const std::vector<MSEdge *> &, AggregateType)`
    /// Constructor
    // MSMeanData_Net::MSMeanData_Net(const std::string& id,
    //                                const SUMOTime dumpBegin,
    //                                const SUMOTime dumpEnd, const bool useLanes,
    //                                const std::string& excludeEmpty,
    //                                const bool withInternal,
    //                                const bool trackVehicles,
    //                                const int detectPersons,
    //                                const double maxTravelTime,
    //                                const double minSamples,
    //                                const double haltSpeed,
    //                                const double haltSpeedRel,
    //                                const std::string& vTypes,
    //                                const std::string& writeAttributes,
    //                                const std::vector<MSEdge*>& edges,
    //                                AggregateType aggregate) :
    //     MSMeanData(id, dumpBegin, dumpEnd, useLanes, excludeEmpty,
    //                withInternal, trackVehicles, detectPersons, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate),
    //     myHaltSpeed(haltSpeed),
    //     myHaltSpeedRel(haltSpeedRel)
    // { }
    #[allow(unused_variables)]
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, detectPersons: i32, maxTravelTime: f64, minSamples: f64, haltSpeed: f64, haltSpeedRel: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        Self {
            base: MSMeanData::new(id, dumpBegin, dumpEnd, useLanes, excludeEmpty, withInternal, trackVehicles, detectPersons, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate),
            myHaltSpeed: haltSpeed,
            myHaltSpeedRel: haltSpeedRel,
        }
    }

    /// C++: `void MSMeanData_Net::~MSMeanData_Net() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Net::~MSMeanData_Net() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty.
    }

    /// C++: `std::vector<std::string> MSMeanData_Net::getAttributeNames() const` [const, virtual]
    /// return all attributes that are (potentially) written by this output
    // std::vector<std::string>
    // MSMeanData_Net::getAttributeNames() const {
    //     std::vector<std::string> result;
    //     result.push_back(toString(SUMO_ATTR_DENSITY));
    //     result.push_back(toString(SUMO_ATTR_LANEDENSITY));
    //     result.push_back(toString(SUMO_ATTR_OVERLAPDENSITY));
    //     result.push_back(toString(SUMO_ATTR_OCCUPANCY));
    //     result.push_back(toString(SUMO_ATTR_WAITINGTIME));
    //     result.push_back(toString(SUMO_ATTR_TIMELOSS));
    //     result.push_back(toString(SUMO_ATTR_SPEED));
    //     result.push_back(toString(SUMO_ATTR_SPEEDREL));
    //     result.push_back(toString(SUMO_ATTR_DEPARTED));
    //     result.push_back(toString(SUMO_ATTR_ARRIVED));
    //     result.push_back(toString(SUMO_ATTR_ENTERED));
    //     result.push_back(toString(SUMO_ATTR_LEFT));
    //     result.push_back(toString(SUMO_ATTR_LANECHANGEDFROM));
    //     result.push_back(toString(SUMO_ATTR_LANECHANGEDTO));
    //     result.push_back(toString(SUMO_ATTR_VAPORIZED));
    //     result.push_back(toString(SUMO_ATTR_TELEPORTED));
    //     result.push_back(toString(SUMO_ATTR_FLOW));
    //     result.push_back(toString(SUMO_ATTR_DISTANCE));
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_names(&self) -> Vec<String> {
        let attrs = [
            SUMO_ATTR_DENSITY, SUMO_ATTR_LANEDENSITY, SUMO_ATTR_OVERLAPDENSITY, SUMO_ATTR_OCCUPANCY,
            SUMO_ATTR_WAITINGTIME, SUMO_ATTR_TIMELOSS, SUMO_ATTR_SPEED, SUMO_ATTR_SPEEDREL,
            SUMO_ATTR_DEPARTED, SUMO_ATTR_ARRIVED, SUMO_ATTR_ENTERED, SUMO_ATTR_LEFT,
            SUMO_ATTR_LANECHANGEDFROM, SUMO_ATTR_LANECHANGEDTO, SUMO_ATTR_VAPORIZED, SUMO_ATTR_TELEPORTED,
            SUMO_ATTR_FLOW, SUMO_ATTR_DISTANCE,
        ];
        attrs.iter().map(|a| utils::xml::sumo_xml_definitions::Attrs.get_string(*a).to_string()).collect()
    }

    /// C++: `bool MSMeanData_Net::supports(const SumoXMLAttrMask &)` [static]
    /// check whether any of the given attributes can be calculated by this meandata
    // bool
    // MSMeanData_Net::supports(const SumoXMLAttrMask& attributeMask) {
    //     return
    //         attributeMask.test(SUMO_ATTR_DENSITY) ||
    //         attributeMask.test(SUMO_ATTR_LANEDENSITY) ||
    //         attributeMask.test(SUMO_ATTR_OVERLAPDENSITY) ||
    //         attributeMask.test(SUMO_ATTR_OCCUPANCY) ||
    //         attributeMask.test(SUMO_ATTR_WAITINGTIME) ||
    //         attributeMask.test(SUMO_ATTR_TIMELOSS) ||
    //         attributeMask.test(SUMO_ATTR_SPEED) ||
    //         attributeMask.test(SUMO_ATTR_SPEEDREL) ||
    //         attributeMask.test(SUMO_ATTR_DEPARTED) ||
    //         attributeMask.test(SUMO_ATTR_ARRIVED) ||
    //         attributeMask.test(SUMO_ATTR_ENTERED) ||
    //         attributeMask.test(SUMO_ATTR_LEFT) ||
    //         attributeMask.test(SUMO_ATTR_LANECHANGEDFROM) ||
    //         attributeMask.test(SUMO_ATTR_LANECHANGEDTO) ||
    //         attributeMask.test(SUMO_ATTR_VAPORIZED) ||
    //         attributeMask.test(SUMO_ATTR_TELEPORTED) ||
    //         attributeMask.test(SUMO_ATTR_FLOW) ||
    //         attributeMask.test(SUMO_ATTR_DISTANCE);
    // }
    #[allow(unused_variables)]
    pub fn supports(attributeMask: &SumoXMLAttrMask) -> bool {
        attributeMask.test(SUMO_ATTR_DENSITY)
            || attributeMask.test(SUMO_ATTR_LANEDENSITY)
            || attributeMask.test(SUMO_ATTR_OVERLAPDENSITY)
            || attributeMask.test(SUMO_ATTR_OCCUPANCY)
            || attributeMask.test(SUMO_ATTR_WAITINGTIME)
            || attributeMask.test(SUMO_ATTR_TIMELOSS)
            || attributeMask.test(SUMO_ATTR_SPEED)
            || attributeMask.test(SUMO_ATTR_SPEEDREL)
            || attributeMask.test(SUMO_ATTR_DEPARTED)
            || attributeMask.test(SUMO_ATTR_ARRIVED)
            || attributeMask.test(SUMO_ATTR_ENTERED)
            || attributeMask.test(SUMO_ATTR_LEFT)
            || attributeMask.test(SUMO_ATTR_LANECHANGEDFROM)
            || attributeMask.test(SUMO_ATTR_LANECHANGEDTO)
            || attributeMask.test(SUMO_ATTR_VAPORIZED)
            || attributeMask.test(SUMO_ATTR_TELEPORTED)
            || attributeMask.test(SUMO_ATTR_FLOW)
            || attributeMask.test(SUMO_ATTR_DISTANCE)
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData_Net::createValues(MSLane *const, const double, const bool) const` [const, virtual, protected]
    /// Create an instance of MeanDataValues
    // MSMeanData::MeanDataValues*
    // MSMeanData_Net::createValues(MSLane* const lane, const double length, const bool doAdd) const {
    //     return new MSLaneMeanDataValues(lane, length, doAdd, this);
    // }
    #[allow(unused_variables)]
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        Ptr::from_raw(Box::into_raw(Box::new(MSMeanData_Net_MSLaneMeanDataValues::new(lane, length, doAdd, Ptr::new(self))))).cast::<MSMeanData_MeanDataValues>()
    }

    /// C++: `void MSMeanData_Net::resetOnly(SUMOTime)` [protected]
    /// Resets network value in order to allow processing of the next interval
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn reset_only(&mut self, stopTime: SUMOTime) {
        // C++ MSMeanData_Net does not override resetOnly; the inherited MSMeanData::resetOnly applies
        self.base.reset_only(stopTime);
    }

    /// C++: `void MSMeanData_Net::MSMeanData_Net(const MSMeanData_Net &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData_Net) -> Self {
        panic!("MSMeanData_Net copy constructor is private and invalidated in C++")
    }

    /// C++: `MSMeanData_Net & MSMeanData_Net::operator=(const MSMeanData_Net &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData_Net) -> &mut MSMeanData_Net {
        panic!("MSMeanData_Net assignment operator is private and invalidated in C++")
    }

}

