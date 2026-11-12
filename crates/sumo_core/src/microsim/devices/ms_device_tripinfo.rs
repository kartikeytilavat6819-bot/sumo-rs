//! Port of microsim/devices/MSDevice_Tripinfo.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Tripinfo` (microsim/devices/MSDevice_Tripinfo.h)
#[repr(C)]
pub struct MSDevice_Tripinfo {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// The lane the vehicle departed at
    pub myDepartLane: String,
    /// The speed on departure
    pub myDepartSpeed: f64,
    /// The lateral depart position
    pub myDepartPosLat: f64,
    /// The overall waiting time
    pub myWaitingTime: SUMOTime,
    /// Whether the vehicle is currently waiting
    pub myAmWaiting: bool,
    /// The overall number of unintended stops
    pub myWaitingCount: i32,
    /// The overall intentional stopping time
    pub myStoppingTime: SUMOTime,
    /// The time when parking started
    pub myParkingStarted: SUMOTime,
    /// The vehicle's arrival time
    pub myArrivalTime: SUMOTime,
    /// The lane the vehicle arrived at
    pub myArrivalLane: String,
    /// The position on the lane the vehicle arrived at
    pub myArrivalPos: f64,
    /// The lateral position on the lane the vehicle arrived at
    pub myArrivalPosLat: f64,
    /// The speed when arriving
    pub myArrivalSpeed: f64,
    /// The reason for vehicle arrival
    pub myArrivalReason: MSMoveReminder_Notification,
    /// The time loss when compared to the desired and allowed speed
    pub myMesoTimeLoss: SUMOTime,
    /// The route length
    pub myRouteLength: f64,
}

impl std::ops::Deref for MSDevice_Tripinfo {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Tripinfo {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_Tripinfo (port as module-level statics, see AGENTS.md):
//   static int MSDevice_Tripinfo::myPendingOutput;
//   static int MSDevice_Tripinfo::myVehicleCount;
//   static int MSDevice_Tripinfo::myUndepartedVehicleCount;
//   static double MSDevice_Tripinfo::myTotalRouteLength;
//   static double MSDevice_Tripinfo::myTotalSpeed;
//   static SUMOTime MSDevice_Tripinfo::myTotalDuration;
//   static SUMOTime MSDevice_Tripinfo::myTotalWaitingTime;
//   static SUMOTime MSDevice_Tripinfo::myTotalTimeLoss;
//   static SUMOTime MSDevice_Tripinfo::myTotalDepartDelay;
//   static SUMOTime MSDevice_Tripinfo::myWaitingDepartDelay;
//   static int MSDevice_Tripinfo::myBikeCount;
//   static double MSDevice_Tripinfo::myTotalBikeRouteLength;
//   static double MSDevice_Tripinfo::myTotalBikeSpeed;
//   static SUMOTime MSDevice_Tripinfo::myTotalBikeDuration;
//   static SUMOTime MSDevice_Tripinfo::myTotalBikeWaitingTime;
//   static SUMOTime MSDevice_Tripinfo::myTotalBikeTimeLoss;
//   static SUMOTime MSDevice_Tripinfo::myTotalBikeDepartDelay;
//   static int MSDevice_Tripinfo::myWalkCount;
//   static double MSDevice_Tripinfo::myTotalWalkRouteLength;
//   static SUMOTime MSDevice_Tripinfo::myTotalWalkDuration;
//   static SUMOTime MSDevice_Tripinfo::myTotalWalkTimeLoss;
//   static int MSDevice_Tripinfo::myRideCount;
//   static int MSDevice_Tripinfo::myRideBusCount;
//   static int MSDevice_Tripinfo::myRideRailCount;
//   static int MSDevice_Tripinfo::myRideTaxiCount;
//   static int MSDevice_Tripinfo::myRideBikeCount;
//   static int MSDevice_Tripinfo::myRideAbortCount;
//   static int MSDevice_Tripinfo::myTotalRideWaitingTime;
//   static int MSDevice_Tripinfo::myTotalRideRouteLength;
//   static int MSDevice_Tripinfo::myTotalRideDuration;

impl MSDevice_Tripinfo {
    /// C++: `void MSDevice_Tripinfo::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Tripinfo-options
    // void
    // MSDevice_Tripinfo::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("Tripinfo Device");
    //     insertDefaultAssignmentOptions("tripinfo", "Tripinfo Device", oc);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_Tripinfo::insertOptions")
    }

    /// C++: `void MSDevice_Tripinfo::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_Tripinfo::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Tripinfo::generateOutputForUnfinished()` [static]
    /// generate output for vehicles which are still in the network
    // void
    // MSDevice_Tripinfo::generateOutputForUnfinished() {
    //     MSNet* net = MSNet::getInstance();
    //     OutputDevice* tripinfoOut = (OptionsCont::getOptions().isSet("tripinfo-output") ?
    //                                  &OutputDevice::getDeviceByOption("tripinfo-output") : nullptr);
    //     myWaitingDepartDelay = 0;
    //     myUndepartedVehicleCount = 0;
    //     const bool writeUndeparted = OptionsCont::getOptions().getBool("tripinfo-output.write-undeparted");
    //     const SUMOTime t = net->getCurrentTimeStep();
    //     while (myPendingOutput.size() > 0) {
    //         const MSDevice_Tripinfo* d = *myPendingOutput.begin();
    //         const bool departed = d->myHolder.hasDeparted();
    //         const bool departDelayed = d->myHolder.getParameter().depart <= t;
    //         if (!departed && departDelayed) {
    //             myUndepartedVehicleCount++;
    //             myWaitingDepartDelay += (t - d->myHolder.getParameter().depart);
    //         }
    //         if (departed || (writeUndeparted && departDelayed)) {
    //             const_cast<MSDevice_Tripinfo*>(d)->updateParkingStopTime();
    //             d->generateOutput(tripinfoOut);
    //             if (tripinfoOut != nullptr) {
    //                 for (MSVehicleDevice* const dev : d->myHolder.getDevices()) {
    //                     if (typeid(*dev) == typeid(MSDevice_Tripinfo) || typeid(*dev) == typeid(MSDevice_Vehroutes)) {
    //                         // tripinfo is special and vehroute has its own write-unfinished option
    //                         continue;
    //                     }
    //                     dev->generateOutput(tripinfoOut);
    //                 }
    //                 OutputDevice::getDeviceByOption("tripinfo-output").closeTag();
    //             }
    //         } else {
    //             myPendingOutput.erase(d);
    //         }
    //     }
    //     // unfinished persons
    //     if (net->hasPersons()) {
    //         net->getPersonControl().eraseAll();
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn generate_output_for_unfinished() {
        todo!("port MSDevice_Tripinfo::generateOutputForUnfinished")
    }

    /// C++: `void MSDevice_Tripinfo::addPedestrianData(double, SUMOTime, SUMOTime)` [static]
    /// record tripinfo data for pedestrians
    // void
    // MSDevice_Tripinfo::addPedestrianData(double walkLength, SUMOTime walkDuration, SUMOTime walkTimeLoss) {
    //     myWalkCount++;
    //     myTotalWalkRouteLength += walkLength;
    //     myTotalWalkDuration += walkDuration;
    //     myTotalWalkTimeLoss += walkTimeLoss;
    // }
    #[allow(unused_variables)]
    pub fn add_pedestrian_data(walkLength: f64, walkDuration: SUMOTime, walkTimeLoss: SUMOTime) {
        todo!("port MSDevice_Tripinfo::addPedestrianData")
    }

    /// C++: `void MSDevice_Tripinfo::addRideTransportData(const bool, const double, const SUMOTime, const SUMOVehicleClass, const std::string &, const SUMOTime)` [static]
    /// record tripinfo data for rides and transports
    // void
    // MSDevice_Tripinfo::addRideTransportData(const bool isPerson, const double distance, const SUMOTime duration,
    //                                         const SUMOVehicleClass vClass, const std::string& line, const SUMOTime waitingTime) {
    //     const int index = isPerson ? 0 : 1;
    //     myRideCount[index]++;
    //     if (duration > 0) {
    //         myTotalRideWaitingTime[index] += waitingTime;
    //         myTotalRideRouteLength[index] += distance;
    //         myTotalRideDuration[index] += duration;
    //         if (vClass == SVC_BICYCLE) {
    //             myRideBikeCount[index]++;
    //         } else if (!line.empty()) {
    //             if (isRailway(vClass)) {
    //                 myRideRailCount[index]++;
    //             } else if (vClass == SVC_TAXI) {
    //                 myRideTaxiCount[index]++;
    //             } else {
    //                 // some kind of road vehicle
    //                 myRideBusCount[index]++;
    //             }
    //         }
    //     } else {
    //         myRideAbortCount[index]++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_ride_transport_data(isPerson: bool, distance: f64, duration: SUMOTime, vClass: SUMOVehicleClass, line: &str, waitingTime: SUMOTime) {
        todo!("port MSDevice_Tripinfo::addRideTransportData")
    }

    /// C++: `std::string MSDevice_Tripinfo::printStatistics()` [static]
    /// get statistics for printing to stdout
    // std::string
    // MSDevice_Tripinfo::printStatistics() {
    //     std::ostringstream msg;
    //     msg.setf(msg.fixed);
    //     msg.precision(gPrecision);
    //     if (myBikeCount == 0 || myVehicleCount > 0) {
    //         msg << "Statistics (avg of " << myVehicleCount << "):\n";
    //         msg << " RouteLength: " << getAvgRouteLength() << "\n"
    //             << " Speed: " << getAvgTripSpeed() << "\n"
    //             << " Duration: " << getAvgDuration() << "\n"
    //             << " WaitingTime: " << getAvgWaitingTime() << "\n"
    //             << " TimeLoss: " << getAvgTimeLoss() << "\n"
    //             << " DepartDelay: " << getAvgDepartDelay() << "\n";
    //     }
    //     if (myBikeCount > 0) {
    //         msg << "Bike Statistics (avg of " << myBikeCount << "):\n"
    //             << " RouteLength: " << getAvgBikeRouteLength() << "\n"
    //             << " Speed: " << getAvgBikeTripSpeed() << "\n"
    //             << " Duration: " << getAvgBikeDuration() << "\n"
    //             << " WaitingTime: " << getAvgBikeWaitingTime() << "\n"
    //             << " TimeLoss: " << getAvgBikeTimeLoss() << "\n"
    //             << " DepartDelay: " << getAvgBikeDepartDelay() << "\n";
    //         if (myVehicleCount > 0 && myWaitingDepartDelay >= 0) {
    //             msg << "Statistics (avg of " << (myVehicleCount + myBikeCount) << "):\n";
    //         }
    //     }
    //     if (myWaitingDepartDelay >= 0) {
    //         msg << " DepartDelayWaiting: " << getAvgDepartDelayWaiting() << "\n";
    //     }
    //     if (myWalkCount > 0) {
    //         msg << "Pedestrian Statistics (avg of " << myWalkCount << " walks):\n"
    //             << " RouteLength: " << getAvgWalkRouteLength() << "\n"
    //             << " Duration: " << getAvgWalkDuration() << "\n"
    //             << " TimeLoss: " << getAvgWalkTimeLoss() << "\n";
    //     }
    //     printRideStatistics(msg, "Ride", "rides", 0);
    //     printRideStatistics(msg, "Transport", "transports", 1);
    //     return msg.str();
    // }
    #[allow(unused_variables)]
    pub fn print_statistics() -> String {
        todo!("port MSDevice_Tripinfo::printStatistics")
    }

    /// C++: `void MSDevice_Tripinfo::writeStatistics(OutputDevice &)` [static]
    /// write statistic output to (xml) file
    // void
    // MSDevice_Tripinfo::writeStatistics(OutputDevice& od) {
    //     od.setPrecision(gPrecision);
    //     od.openTag("vehicleTripStatistics");
    //     od.writeAttr("count", myVehicleCount);
    //     od.writeAttr("routeLength", getAvgRouteLength());
    //     od.writeAttr("speed", getAvgTripSpeed());
    //     od.writeAttr("duration", getAvgDuration());
    //     od.writeAttr("waitingTime", getAvgWaitingTime());
    //     od.writeAttr("timeLoss", getAvgTimeLoss());
    //     od.writeAttr("departDelay", getAvgDepartDelay());
    //     od.writeAttr("departDelayWaiting", getAvgDepartDelayWaiting());
    //     od.writeAttr("totalTravelTime", time2string(myTotalDuration));
    //     od.writeAttr("totalDepartDelay", time2string(TIME2STEPS(getTotalDepartDelay() + getTotalBikeDepartDelay())));
    //     od.closeTag();
    //     if (myBikeCount > 0) {
    //         od.openTag("bikeTripStatistics");
    //         od.writeAttr("count", myBikeCount);
    //         od.writeAttr("routeLength", getAvgBikeRouteLength());
    //         od.writeAttr("speed", getAvgBikeTripSpeed());
    //         od.writeAttr("duration", getAvgBikeDuration());
    //         od.writeAttr("waitingTime", getAvgBikeWaitingTime());
    //         od.writeAttr("timeLoss", getAvgBikeTimeLoss());
    //         od.writeAttr("departDelay", getAvgBikeDepartDelay());
    //         od.writeAttr("totalTravelTime", time2string(myTotalBikeDuration));
    //         od.closeTag();
    //     }
    //     od.openTag("pedestrianStatistics");
    //     od.writeAttr("number", myWalkCount);
    //     od.writeAttr("routeLength", getAvgWalkRouteLength());
    //     od.writeAttr("duration", getAvgWalkDuration());
    //     od.writeAttr("timeLoss", getAvgWalkTimeLoss());
    //     od.closeTag();
    //     writeRideStatistics(od, "rideStatistics", 0);
    //     writeRideStatistics(od, "transportStatistics", 1);
    // }
    #[allow(unused_variables)]
    pub fn write_statistics(od: &mut OutputDevice) {
        todo!("port MSDevice_Tripinfo::writeStatistics")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgRouteLength()` [static]
    /// accessors for GUINet-Parameters
    // double
    // MSDevice_Tripinfo::getAvgRouteLength() {
    //     if (myVehicleCount > 0) {
    //         return myTotalRouteLength / myVehicleCount;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_route_length() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgRouteLength")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgTripSpeed()` [static]
    // double
    // MSDevice_Tripinfo::getAvgTripSpeed() {
    //     if (myVehicleCount > 0) {
    //         return myTotalSpeed / myVehicleCount;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_trip_speed() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgTripSpeed")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgDuration()` [static]
    // double
    // MSDevice_Tripinfo::getAvgDuration() {
    //     if (myVehicleCount > 0) {
    //         return STEPS2TIME(myTotalDuration / myVehicleCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_duration() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgDuration")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgWaitingTime()` [static]
    // double
    // MSDevice_Tripinfo::getAvgWaitingTime() {
    //     if (myVehicleCount > 0) {
    //         return STEPS2TIME(myTotalWaitingTime / myVehicleCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_waiting_time() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgWaitingTime")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgTimeLoss()` [static]
    // double
    // MSDevice_Tripinfo::getAvgTimeLoss() {
    //     if (myVehicleCount > 0) {
    //         return STEPS2TIME(myTotalTimeLoss / myVehicleCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_time_loss() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgTimeLoss")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgDepartDelay()` [static]
    // double
    // MSDevice_Tripinfo::getAvgDepartDelay() {
    //     if (myVehicleCount > 0) {
    //         return STEPS2TIME(myTotalDepartDelay / myVehicleCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_depart_delay() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgDepartDelay")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgDepartDelayWaiting()` [static]
    // double
    // MSDevice_Tripinfo::getAvgDepartDelayWaiting() {
    //     if (myWaitingDepartDelay >= 0) {
    //         return STEPS2TIME(myWaitingDepartDelay / MAX2(1, myUndepartedVehicleCount));
    //     } else {
    //         return -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_depart_delay_waiting() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgDepartDelayWaiting")
    }

    /// C++: `double MSDevice_Tripinfo::getTotalDepartDelay()` [static]
    // double
    // MSDevice_Tripinfo::getTotalDepartDelay() {
    //     return STEPS2TIME(myTotalDepartDelay + MAX2((SUMOTime)0, myWaitingDepartDelay));
    // }
    #[allow(unused_variables)]
    pub fn get_total_depart_delay() -> f64 {
        todo!("port MSDevice_Tripinfo::getTotalDepartDelay")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgBikeRouteLength()` [static]
    // double
    // MSDevice_Tripinfo::getAvgBikeRouteLength() {
    //     if (myBikeCount > 0) {
    //         return myTotalBikeRouteLength / myBikeCount;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_bike_route_length() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgBikeRouteLength")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgBikeTripSpeed()` [static]
    // double
    // MSDevice_Tripinfo::getAvgBikeTripSpeed() {
    //     if (myBikeCount > 0) {
    //         return myTotalBikeSpeed / myBikeCount;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_bike_trip_speed() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgBikeTripSpeed")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgBikeDuration()` [static]
    // double
    // MSDevice_Tripinfo::getAvgBikeDuration() {
    //     if (myBikeCount > 0) {
    //         return STEPS2TIME(myTotalBikeDuration / myBikeCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_bike_duration() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgBikeDuration")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgBikeWaitingTime()` [static]
    // double
    // MSDevice_Tripinfo::getAvgBikeWaitingTime() {
    //     if (myBikeCount > 0) {
    //         return STEPS2TIME(myTotalBikeWaitingTime / myBikeCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_bike_waiting_time() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgBikeWaitingTime")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgBikeTimeLoss()` [static]
    // double
    // MSDevice_Tripinfo::getAvgBikeTimeLoss() {
    //     if (myBikeCount > 0) {
    //         return STEPS2TIME(myTotalBikeTimeLoss / myBikeCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_bike_time_loss() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgBikeTimeLoss")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgBikeDepartDelay()` [static]
    // double
    // MSDevice_Tripinfo::getAvgBikeDepartDelay() {
    //     if (myBikeCount > 0) {
    //         return STEPS2TIME(myTotalBikeDepartDelay / myBikeCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_bike_depart_delay() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgBikeDepartDelay")
    }

    /// C++: `double MSDevice_Tripinfo::getTotalBikeDepartDelay()` [static]
    // double
    // MSDevice_Tripinfo::getTotalBikeDepartDelay() {
    //     return STEPS2TIME(myTotalBikeDepartDelay);
    // }
    #[allow(unused_variables)]
    pub fn get_total_bike_depart_delay() -> f64 {
        todo!("port MSDevice_Tripinfo::getTotalBikeDepartDelay")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgWalkRouteLength()` [static]
    // double
    // MSDevice_Tripinfo::getAvgWalkRouteLength() {
    //     if (myWalkCount > 0) {
    //         return myTotalWalkRouteLength / myWalkCount;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_walk_route_length() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgWalkRouteLength")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgWalkDuration()` [static]
    // double
    // MSDevice_Tripinfo::getAvgWalkDuration() {
    //     if (myWalkCount > 0) {
    //         return STEPS2TIME(myTotalWalkDuration / myWalkCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_walk_duration() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgWalkDuration")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgWalkTimeLoss()` [static]
    // double
    // MSDevice_Tripinfo::getAvgWalkTimeLoss() {
    //     if (myWalkCount > 0) {
    //         return STEPS2TIME(myTotalWalkTimeLoss / myWalkCount);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_walk_time_loss() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgWalkTimeLoss")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgRideRouteLength()` [static]
    // double
    // MSDevice_Tripinfo::getAvgRideRouteLength() {
    //     if (myRideCount[0] > 0) {
    //         return myTotalRideRouteLength[0] / myRideCount[0];
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_ride_route_length() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgRideRouteLength")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgRideWaitingTime()` [static]
    // double
    // MSDevice_Tripinfo::getAvgRideWaitingTime() {
    //     if (myRideCount[0] > 0) {
    //         return STEPS2TIME(myTotalRideWaitingTime[0] / myRideCount[0]);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_ride_waiting_time() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgRideWaitingTime")
    }

    /// C++: `double MSDevice_Tripinfo::getAvgRideDuration()` [static]
    // double
    // MSDevice_Tripinfo::getAvgRideDuration() {
    //     if (myRideCount[0] > 0) {
    //         return STEPS2TIME(myTotalRideDuration[0] / myRideCount[0]);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_avg_ride_duration() -> f64 {
        todo!("port MSDevice_Tripinfo::getAvgRideDuration")
    }

    /// C++: `void MSDevice_Tripinfo::~MSDevice_Tripinfo()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Tripinfo::~MSDevice_Tripinfo")
    }

    /// C++: `void MSDevice_Tripinfo::cleanup()` [static]
    /// resets counters
    // void
    // MSDevice_Tripinfo::cleanup() {
    //     myVehicleCount = 0;
    //     myTotalRouteLength = 0;
    //     myTotalSpeed = 0;
    //     myTotalDuration = 0;
    //     myTotalWaitingTime = 0;
    //     myTotalTimeLoss = 0;
    //     myTotalDepartDelay = 0;
    //     myWaitingDepartDelay = -1;
    //
    //     myBikeCount = 0;
    //     myTotalBikeRouteLength = 0;
    //     myTotalBikeSpeed = 0;
    //     myTotalBikeDuration = 0;
    //     myTotalBikeWaitingTime = 0;
    //     myTotalBikeTimeLoss = 0;
    //     myTotalBikeDepartDelay = 0;
    //
    //     myWalkCount = 0;
    //     myTotalWalkRouteLength = 0;
    //     myTotalWalkDuration = 0;
    //     myTotalWalkTimeLoss = 0;
    //
    //     myRideCount = {0, 0};
    //     myRideBusCount = {0, 0};
    //     myRideRailCount = {0, 0};
    //     myRideTaxiCount = {0, 0};
    //     myRideBikeCount = {0, 0};
    //     myRideAbortCount = {0, 0};
    //     myTotalRideWaitingTime = {0, 0};
    //     myTotalRideRouteLength = {0., 0.};
    //     myTotalRideDuration = {0, 0};
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSDevice_Tripinfo::cleanup")
    }

    /// C++: `bool MSDevice_Tripinfo::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks for waiting steps when the vehicle moves
    // bool
    // MSDevice_Tripinfo::notifyMove(SUMOTrafficObject& veh, double /*oldPos*/,
    //                               double /*newPos*/, double newSpeed) {
    //     if (veh.isStopped()) {
    //         if (newSpeed <= SUMO_const_haltingSpeed) {
    //             myStoppingTime += DELTA_T;
    //         }
    //     } else if (newSpeed <= SUMO_const_haltingSpeed && lowAcceleration(veh)) {
    //         myWaitingTime += DELTA_T;
    //         if (!myAmWaiting) {
    //             myWaitingCount++;
    //             myAmWaiting = true;
    //         }
    //     } else {
    //         myAmWaiting = false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_Tripinfo::notifyMove")
    }

    /// C++: `bool MSDevice_Tripinfo::notifyIdle(SUMOTrafficObject &)` [virtual]
    /// record idling as waiting time - cf issue 2233
    // bool
    // MSDevice_Tripinfo::notifyIdle(SUMOTrafficObject& veh) {
    //     if (veh.isVehicle()) {
    //         myWaitingTime += DELTA_T;
    //         if (!myAmWaiting) {
    //             myWaitingCount++;
    //             myAmWaiting = true;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_idle(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        todo!("port MSDevice_Tripinfo::notifyIdle")
    }

    /// C++: `bool MSDevice_Tripinfo::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Saves departure info on insertion
    // bool
    // MSDevice_Tripinfo::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason == MSMoveReminder::NOTIFICATION_DEPARTED) {
    //         if (!MSGlobals::gUseMesoSim) {
    //             myDepartLane = static_cast<MSVehicle&>(veh).getLane()->getID();
    //             myDepartPosLat = static_cast<MSVehicle&>(veh).getLateralPositionOnLane();
    //         } else {
    //             myDepartLane = veh.getEdge()->getFirstAllowed(veh.getVClass(), true)->getID();
    //         }
    //         myDepartSpeed = veh.getSpeed();
    //         myRouteLength = -veh.getPositionOnLane();
    //     } else if (reason == MSMoveReminder::NOTIFICATION_PARKING) {
    //         // notifyMove is not called while parking
    //         // @note insertion delay when resuming after parking is included
    //         updateParkingStopTime();
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Tripinfo::notifyEnter")
    }

    /// C++: `bool MSDevice_Tripinfo::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Saves arrival info
    // bool
    // MSDevice_Tripinfo::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/,
    //                                MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason >= MSMoveReminder::NOTIFICATION_ARRIVED) {
    //         myArrivalTime = MSNet::getInstance()->getCurrentTimeStep();
    //         myArrivalReason = reason;
    //         if (!MSGlobals::gUseMesoSim) {
    //             myArrivalLane = static_cast<MSVehicle&>(veh).getLane()->getID();
    //             myArrivalPosLat = static_cast<MSVehicle&>(veh).getLateralPositionOnLane();
    //         } else {
    //             myArrivalLane = veh.getEdge()->getFirstAllowed(veh.getVClass(), true)->getID();
    //         }
    //         // @note vehicle may have moved past its arrivalPos during the last step
    //         // due to non-zero arrivalspeed but we consider it as arrived at the desired position
    //         // However, vaporization may happen anywhere (via TraCI)
    //         if (reason > MSMoveReminder::NOTIFICATION_TELEPORT_CONTINUATION_ARRIVED) {
    //             // vaporized
    //             myArrivalPos = veh.getPositionOnLane();
    //         } else {
    //             myArrivalPos = myHolder.getArrivalPos();
    //         }
    //         myArrivalSpeed = veh.getSpeed();
    //         updateParkingStopTime();
    //     } else if (reason == MSMoveReminder::NOTIFICATION_PARKING) {
    //         myParkingStarted = MSNet::getInstance()->getCurrentTimeStep();
    //     } else if (reason == NOTIFICATION_JUNCTION
    //                || reason == NOTIFICATION_TELEPORT
    //                || reason == NOTIFICATION_TELEPORT_CONTINUATION) {
    //         if (MSGlobals::gUseMesoSim) {
    //             myRouteLength += myHolder.getCurrentEdge()->getLength();
    //         } else {
    //             const MSLane* lane = static_cast<MSVehicle&>(veh).getLane();
    //             if (lane != nullptr) {
    //                 myRouteLength += lane->getLength();
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Tripinfo::notifyLeave")
    }

    /// C++: `const std::string MSDevice_Tripinfo::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "tripinfo";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Tripinfo::deviceName")
    }

    /// C++: `void MSDevice_Tripinfo::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSDevice_Tripinfo::generateOutput(OutputDevice* tripinfoOut) const {
    //     const SUMOTime timeLoss = MSGlobals::gUseMesoSim ? myMesoTimeLoss : static_cast<MSVehicle&>(myHolder).getTimeLoss();
    //     const double routeLength = myRouteLength + (myArrivalTime == NOT_ARRIVED ? myHolder.getPositionOnLane() : myArrivalPos);
    //     SUMOTime duration = 0;
    //     if (myHolder.hasDeparted()) {
    //         duration = (myArrivalTime == NOT_ARRIVED ? SIMSTEP : myArrivalTime) - myHolder.getDeparture();
    //         const SUMOTime drivingDuration = duration - myStoppingTime;
    //         const double speed = drivingDuration > 0 ? routeLength / STEPS2TIME(drivingDuration) : 0;
    //         if (myHolder.getVClass() == SVC_BICYCLE) {
    //             myBikeCount++;
    //             myTotalBikeRouteLength += routeLength;
    //             myTotalBikeSpeed += speed;
    //             myTotalBikeDuration += duration;
    //             myTotalBikeWaitingTime += myWaitingTime;
    //             myTotalBikeTimeLoss += timeLoss;
    //             myTotalBikeDepartDelay += myHolder.getDepartDelay();
    //         } else {
    //             myVehicleCount++;
    //             myTotalRouteLength += routeLength;
    //             myTotalSpeed += speed;
    //             myTotalDuration += duration;
    //             myTotalWaitingTime += myWaitingTime;
    //             myTotalTimeLoss += timeLoss;
    //             myTotalDepartDelay += myHolder.getDepartDelay();
    //         }
    //     }
    //
    //     myPendingOutput.erase(this);
    //     if (tripinfoOut == nullptr) {
    //         return;
    //     }
    //     // write
    //     OutputDevice& os = *tripinfoOut;
    //     os.openTag("tripinfo").writeAttr("id", myHolder.getID());
    //     os.writeAttr("depart", myHolder.hasDeparted() ? time2string(myHolder.getDeparture()) : "-1");
    //     os.writeAttr("departLane", myDepartLane);
    //     os.writeAttr("departPos", myHolder.getDepartPos());
    //     if (MSGlobals::gLateralResolution > 0) {
    //         os.writeAttr("departPosLat", myDepartPosLat);
    //     }
    //     os.writeAttr("departSpeed", myDepartSpeed);
    //     SUMOTime departDelay = myHolder.getDepartDelay();
    //     const SUMOVehicleParameter& param = myHolder.getParameter();
    //     if (!myHolder.hasDeparted()) {
    //         assert(param.depart <= SIMSTEP || param.departProcedure != DepartDefinition::GIVEN);
    //         departDelay = SIMSTEP - param.depart;
    //     }
    //     os.writeAttr("departDelay", time2string(departDelay));
    //     os.writeAttr("arrival", time2string(myArrivalTime));
    //     os.writeAttr("arrivalLane", myArrivalLane);
    //     os.writeAttr("arrivalPos", myArrivalPos);
    //     if (MSGlobals::gLateralResolution > 0) {
    //         os.writeAttr("arrivalPosLat", myArrivalPosLat);
    //     }
    //     os.writeAttr("arrivalSpeed", myArrivalSpeed);
    //     os.writeAttr("duration", time2string(duration));
    //     os.writeAttr("routeLength", routeLength);
    //     os.writeAttr(SUMO_ATTR_WAITINGTIME, time2string(myWaitingTime));
    //     os.writeAttr(SUMO_ATTR_WAITINGCOUNT, myWaitingCount);
    //     os.writeAttr(SUMO_ATTR_STOPTIME, time2string(myStoppingTime));
    //     os.writeAttr(SUMO_ATTR_TIMELOSS, time2string(timeLoss));
    //     os.writeAttr("rerouteNo", myHolder.getNumberReroutes());
    //     os.writeAttr("devices", toString(myHolder.getDevices()));
    //     os.writeAttr("vType", myHolder.getVehicleType().getID());
    //     os.writeAttr("speedFactor", myHolder.getChosenSpeedFactor());
    //     std::string vaporized;
    //     switch (myArrivalReason) {
    //         case MSMoveReminder::NOTIFICATION_VAPORIZED_CALIBRATOR:
    //             vaporized = "calibrator";
    //             break;
    //         case MSMoveReminder::NOTIFICATION_VAPORIZED_GUI:
    //             vaporized = "gui";
    //             break;
    //         case MSMoveReminder::NOTIFICATION_VAPORIZED_COLLISION:
    //             vaporized = "collision";
    //             break;
    //         case MSMoveReminder::NOTIFICATION_VAPORIZED_VAPORIZER:
    //             vaporized = "vaporizer";
    //             break;
    //         case MSMoveReminder::NOTIFICATION_VAPORIZED_TRACI:
    //             vaporized = "traci";
    //             break;
    //         case MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED:
    //         case MSMoveReminder::NOTIFICATION_TELEPORT_CONTINUATION_ARRIVED:
    //             vaporized = "teleport";
    //             break;
    //         default:
    //             if (myHolder.getEdge() == myHolder.getRoute().getLastEdge() ||
    //                     (param.arrivalEdge >= 0 && myHolder.getRoutePosition() >= param.arrivalEdge)) {
    //                 vaporized = "";
    //             } else {
    //                 vaporized = "end";
    //             }
    //             break;
    //     }
    //     os.writeAttr("vaporized", vaporized);
    //     // cannot close tag because emission device output might follow
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        todo!("port MSDevice_Tripinfo::generateOutput")
    }

    /// C++: `void MSDevice_Tripinfo::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice_Tripinfo::saveState(OutputDevice& out) const {
    //     // always write device id to replicate stochastic assignment
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     if (myHolder.hasDeparted()) {
    //         std::ostringstream internals;
    //         // the depart lane is empty if the device was added when loading state
    //         internals << (myDepartLane == "" ? STATE_EMPTY_LANE : myDepartLane) << " ";
    //         if (!MSGlobals::gUseMesoSim) {
    //             internals << myDepartPosLat << " ";
    //         }
    //         std::string state_arrivalLane = myArrivalLane == "" ? STATE_EMPTY_LANE : myArrivalLane;
    //         internals << myDepartSpeed << " " << myRouteLength << " " << myWaitingTime << " " << myAmWaiting << " " << myWaitingCount << " ";
    //         internals << myStoppingTime << " " << myParkingStarted << " ";
    //         internals << myArrivalTime << " " << state_arrivalLane << " " << myArrivalPos << " " << myArrivalPosLat << " " << myArrivalSpeed;
    //         if (MSGlobals::gUseMesoSim) {
    //             internals << " " << myMesoTimeLoss;
    //         }
    //         out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        todo!("port MSDevice_Tripinfo::saveState")
    }

    /// C++: `void MSDevice_Tripinfo::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice_Tripinfo::loadState(const SUMOSAXAttributes& attrs) {
    //     if (attrs.hasAttribute(SUMO_ATTR_STATE)) {
    //         std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //         bis >> myDepartLane;
    //         if (!MSGlobals::gUseMesoSim) {
    //             bis >> myDepartPosLat;
    //         }
    //         bis >> myDepartSpeed >> myRouteLength >> myWaitingTime >> myAmWaiting >> myWaitingCount;
    //         bis >> myStoppingTime >> myParkingStarted;
    //         bis >> myArrivalTime >> myArrivalLane >> myArrivalPos >> myArrivalPosLat >> myArrivalSpeed;
    //         if (MSGlobals::gUseMesoSim) {
    //             bis >> myMesoTimeLoss;
    //         }
    //         if (myDepartLane == STATE_EMPTY_LANE) {
    //             myDepartLane = "";
    //         }
    //         if (myArrivalLane == STATE_EMPTY_LANE) {
    //             myArrivalLane = "";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port MSDevice_Tripinfo::loadState")
    }

    /// C++: `void MSDevice_Tripinfo::updateStopTime(const SUMOTime)`
    /// update stopping time for meso
    // void updateStopTime(const SUMOTime time) {
    //         myStoppingTime += time;
    //     }
    #[allow(unused_variables)]
    pub fn update_stop_time(&mut self, time: SUMOTime) {
        todo!("port MSDevice_Tripinfo::updateStopTime")
    }

    /// C++: `std::string MSDevice_Tripinfo::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Tripinfo::getParameter(const std::string& key) const {
    //     if (key == toString(SUMO_ATTR_WAITINGTIME)) {
    //         return toString(STEPS2TIME(myWaitingTime));
    //     } else if (key == toString(SUMO_ATTR_WAITINGCOUNT)) {
    //         return toString(myWaitingCount);
    //     } else if (key == toString(SUMO_ATTR_STOPTIME)) {
    //         return toString(STEPS2TIME(myStoppingTime));
    //     } else if (key == toString(SUMO_ATTR_ARRIVALTIME)) {
    //         return toString(STEPS2TIME(myArrivalTime));
    //     } else if (key == toString(SUMO_ATTR_ARRIVALLANE)) {
    //         return toString(myArrivalLane);
    //     } else if (key == toString(SUMO_ATTR_ARRIVALPOS)) {
    //         return toString(myArrivalPos);
    //     } else if (key == toString(SUMO_ATTR_ARRIVALPOS_LAT)) {
    //         return toString(myArrivalPosLat);
    //     } else if (key == toString(SUMO_ATTR_ARRIVALSPEED)) {
    //         return toString(myArrivalSpeed);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_Tripinfo::getParameter")
    }

    /// C++: `std::string MSDevice_Tripinfo::getGlobalParameter(const std::string &)` [static]
    /// try to retrieve the given parameter from the global statistics. Throw exception for unsupported key
    // std::string
    // MSDevice_Tripinfo::getGlobalParameter(const std::string& prefixedKey) {
    //     std::string key = prefixedKey; // by default, assume vehicleTripStatistics;
    //     const std::string err = "Parameter '" + prefixedKey + "' is not supported for device of type 'tripinfo'";
    //     if (StringUtils::startsWith(key, "vehicleTripStatistics.")) {
    //         key = prefixedKey.substr(22);
    //     } else if (StringUtils::startsWith(key, "bikeTripStatistics.")) {
    //         key = prefixedKey.substr(19);
    //         if (key == toString(SUMO_ATTR_COUNT)) {
    //             return toString(myBikeCount);
    //         } else if (key == "routeLength") {
    //             return toString(getAvgBikeRouteLength());
    //         } else if (key == toString(SUMO_ATTR_SPEED)) {
    //             return toString(getAvgBikeTripSpeed());
    //         } else if (key == toString(SUMO_ATTR_DURATION)) {
    //             return toString(getAvgBikeDuration());
    //         } else if (key == toString(SUMO_ATTR_WAITINGTIME)) {
    //             return toString(getAvgBikeWaitingTime());
    //         } else if (key == toString(SUMO_ATTR_TIMELOSS)) {
    //             return toString(getAvgBikeTimeLoss());
    //         } else if (key == "departDelay") {
    //             return toString(getAvgBikeDepartDelay());
    //         } else if (key == "totalTravelTime") {
    //             // avoid human readable output
    //             return toString(STEPS2TIME((myTotalBikeDuration)));
    //         }
    //         throw InvalidArgument(err);
    //
    //     } else if (StringUtils::startsWith(key, "pedestrianStatistics.")) {
    //         key = prefixedKey.substr(21);
    //         if (key == toString(SUMO_ATTR_NUMBER) || key == toString(SUMO_ATTR_COUNT)) {
    //             return toString(myWalkCount);
    //         } else if (key == "routeLength") {
    //             return toString(getAvgWalkRouteLength());
    //         } else if (key == toString(SUMO_ATTR_DURATION)) {
    //             return toString(getAvgWalkDuration());
    //         } else if (key == toString(SUMO_ATTR_TIMELOSS)) {
    //             return toString(getAvgWalkTimeLoss());
    //         }
    //         throw InvalidArgument(err);
    //
    //     } else if (StringUtils::startsWith(key, "rideStatistics.") ||
    //                StringUtils::startsWith(key, "transportStatistics.")) {
    //         int index = 0;
    //         if (StringUtils::startsWith(key, "rideStatistics.")) {
    //             key = prefixedKey.substr(15);
    //         } else {
    //             index = 1;
    //             key = prefixedKey.substr(20);
    //         }
    //         if (key == toString(SUMO_ATTR_NUMBER) || key == toString(SUMO_ATTR_COUNT)) {
    //             return toString(myRideCount[index]);
    //         } else if (key == toString(SUMO_ATTR_WAITINGTIME)) {
    //             return toString(STEPS2TIME(myTotalRideWaitingTime[index] / MAX2(1, myRideCount[index])));
    //         } else if (key == "routeLength") {
    //             return toString(myTotalRideRouteLength[index] / MAX2(1, myRideCount[index]));
    //         } else if (key == toString(SUMO_ATTR_DURATION)) {
    //             return toString(myTotalRideRouteLength[index] / MAX2(1, myRideCount[index]));
    //         } else if (key == "bus") {
    //             return toString(myRideBusCount[index]);
    //         } else if (key == "train") {
    //             return toString(myRideRailCount[index]);
    //         } else if (key == "taxi") {
    //             return toString(myRideTaxiCount[index]);
    //         } else if (key == "bike") {
    //             return toString(myRideBikeCount[index]);
    //         } else if (key == "aborted") {
    //             return toString(myRideAbortCount[index]);
    //         }
    //         throw InvalidArgument(err);
    //     }
    //     // vehicleTripStatistics
    //     if (key == toString(SUMO_ATTR_COUNT)) {
    //         return toString(myVehicleCount);
    //     } else if (key == "routeLength") {
    //         return toString(getAvgRouteLength());
    //     } else if (key == toString(SUMO_ATTR_SPEED)) {
    //         return toString(getAvgTripSpeed());
    //     } else if (key == toString(SUMO_ATTR_DURATION)) {
    //         return toString(getAvgDuration());
    //     } else if (key == toString(SUMO_ATTR_WAITINGTIME)) {
    //         return toString(getAvgWaitingTime());
    //     } else if (key == toString(SUMO_ATTR_TIMELOSS)) {
    //         return toString(getAvgTimeLoss());
    //     } else if (key == "departDelay") {
    //         return toString(getAvgDepartDelay());
    //     } else if (key == "departDelayWaiting") {
    //         return toString(getAvgDepartDelayWaiting());
    //     } else if (key == "totalTravelTime") {
    //         // avoid human readable output
    //         return toString(STEPS2TIME((myTotalDuration)));
    //     } else if (key == "totalDepartDelay") {
    //         return toString(getTotalDepartDelay());
    //     }
    //     throw InvalidArgument(err);
    // }
    #[allow(unused_variables)]
    pub fn get_global_parameter(prefixedKey: &str) -> String {
        todo!("port MSDevice_Tripinfo::getGlobalParameter")
    }

    /// C++: `void MSDevice_Tripinfo::recordMesoParkingTimeLoss(SUMOTime)`
    // void
    // MSDevice_Tripinfo::recordMesoParkingTimeLoss(SUMOTime waitingTime) {
    //     myMesoTimeLoss += waitingTime;
    //     myWaitingTime += waitingTime;
    //     myWaitingCount++;
    // }
    #[allow(unused_variables)]
    pub fn record_meso_parking_time_loss(&mut self, waitingTime: SUMOTime) {
        todo!("port MSDevice_Tripinfo::recordMesoParkingTimeLoss")
    }

    /// C++: `void MSDevice_Tripinfo::MSDevice_Tripinfo(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSDevice_Tripinfo::MSDevice_Tripinfo(SUMOVehicle& holder, const std::string& id) :
    //     MSVehicleDevice(holder, id),
    //     myDepartLane(""),
    //     myDepartSpeed(-1),
    //     myDepartPosLat(0),
    //     myWaitingTime(0),
    //     myAmWaiting(false),
    //     myWaitingCount(0),
    //     myStoppingTime(0),
    //     myParkingStarted(-1),
    //     myArrivalTime(NOT_ARRIVED),
    //     myArrivalLane(""),
    //     myArrivalPos(-1),
    //     myArrivalPosLat(0.),
    //     myArrivalSpeed(-1),
    //     myArrivalReason(MSMoveReminder::NOTIFICATION_ARRIVED),
    //     myMesoTimeLoss(0),
    //     myRouteLength(0.) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        todo!("port MSDevice_Tripinfo::MSDevice_Tripinfo")
    }

    /// C++: `void MSDevice_Tripinfo::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves, see MSMoveReminder::notifyMoveInternal()
    // void
    // MSDevice_Tripinfo::notifyMoveInternal(const SUMOTrafficObject& veh,
    //                                       const double /* frontOnLane */,
    //                                       const double timeOnLane,
    //                                       const double /* meanSpeedFrontOnLane */,
    //                                       const double meanSpeedVehicleOnLane,
    //                                       const double /* travelledDistanceFrontOnLane */,
    //                                       const double /* travelledDistanceVehicleOnLane */,
    //                                       const double /* meanLengthOnLane */) {
    //
    //     // called by meso
    //     const double vmax = veh.getEdge()->getVehicleMaxSpeed(&veh);
    //     if (vmax > 0) {
    //         myMesoTimeLoss += TIME2STEPS(timeOnLane * (vmax - meanSpeedVehicleOnLane) / vmax);
    //     }
    //     myWaitingTime += veh.getWaitingTime();
    //     if (veh.getWaitingTime() >= TIME2STEPS(1)) {
    //         // waiting counts the time spent waiting to enter the next link (when it's occupied).
    //         myWaitingCount++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        todo!("port MSDevice_Tripinfo::notifyMoveInternal")
    }

    /// C++: `void MSDevice_Tripinfo::updateParkingStopTime()` [protected]
    /// update stopping time after parking
    // void
    // MSDevice_Tripinfo::updateParkingStopTime() {
    //     if (myParkingStarted >= 0) {
    //         myStoppingTime += (MSNet::getInstance()->getCurrentTimeStep() - myParkingStarted);
    //         myParkingStarted = -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_parking_stop_time(&mut self) {
        todo!("port MSDevice_Tripinfo::updateParkingStopTime")
    }

    /// C++: `void MSDevice_Tripinfo::printRideStatistics(std::ostringstream &, const std::string &, const std::string &, const int)` [static, protected]
    // void
    // MSDevice_Tripinfo::printRideStatistics(std::ostringstream& msg, const std::string& category, const std::string& modeName, const int index) {
    //     if (myRideCount[index] > 0) {
    //         msg << category << " Statistics (avg of " << myRideCount[index] << " " << modeName << "):\n";
    //         msg << " WaitingTime: " << STEPS2TIME(myTotalRideWaitingTime[index] / myRideCount[index]) << "\n";
    //         msg << " RouteLength: " << myTotalRideRouteLength[index] / myRideCount[index] << "\n";
    //         msg << " Duration: " << STEPS2TIME(myTotalRideDuration[index] / myRideCount[index]) << "\n";
    //         if (myRideBusCount[index] > 0) {
    //             msg << " Bus: " << myRideBusCount[index] << "\n";
    //         }
    //         if (myRideRailCount[index] > 0) {
    //             msg << " Train: " << myRideRailCount[index] << "\n";
    //         }
    //         if (myRideTaxiCount[index] > 0) {
    //             msg << " Taxi: " << myRideTaxiCount[index] << "\n";
    //         }
    //         if (myRideBikeCount[index] > 0) {
    //             msg << " Bike: " << myRideBikeCount[index] << "\n";
    //         }
    //         if (myRideAbortCount[index] > 0) {
    //             msg << " Aborted: " << myRideAbortCount[index] << "\n";
    //         }
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn print_ride_statistics(msg: &mut Unmapped /* std::ostringstream */, category: &str, modeName: &str, index: i32) {
        todo!("port MSDevice_Tripinfo::printRideStatistics")
    }

    /// C++: `void MSDevice_Tripinfo::writeRideStatistics(OutputDevice &, const std::string &, const int)` [static, protected]
    // void
    // MSDevice_Tripinfo::writeRideStatistics(OutputDevice& od, const std::string& category, const int index) {
    //     od.openTag(category);
    //     od.writeAttr("number", myRideCount[index]);
    //     if (myRideCount[index] > 0) {
    //         od.writeAttr("waitingTime", STEPS2TIME(myTotalRideWaitingTime[index] / myRideCount[index]));
    //         od.writeAttr("routeLength", myTotalRideRouteLength[index] / myRideCount[index]);
    //         od.writeAttr("duration", STEPS2TIME(myTotalRideDuration[index] / myRideCount[index]));
    //         od.writeAttr("bus", myRideBusCount[index]);
    //         od.writeAttr("train", myRideRailCount[index]);
    //         od.writeAttr("taxi", myRideTaxiCount[index]);
    //         od.writeAttr("bike", myRideBikeCount[index]);
    //         od.writeAttr("aborted", myRideAbortCount[index]);
    //     }
    //     od.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_ride_statistics(od: &mut OutputDevice, category: &str, index: i32) {
        todo!("port MSDevice_Tripinfo::writeRideStatistics")
    }

    /// C++: `bool MSDevice_Tripinfo::lowAcceleration(const SUMOTrafficObject &)` [static, protected]
    // bool
    // MSDevice_Tripinfo::lowAcceleration(const SUMOTrafficObject& veh) {
    //     if (MSGlobals::gUseMesoSim) {
    //         // acceleration is not modelled
    //         return false;
    //     } else {
    //         const MSVehicle& v = dynamic_cast<const MSVehicle&>(veh);
    //         return v.getAcceleration() <= v.accelThresholdForWaiting();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn low_acceleration(veh: &SUMOTrafficObject) -> bool {
        todo!("port MSDevice_Tripinfo::lowAcceleration")
    }

    /// C++: `void MSDevice_Tripinfo::MSDevice_Tripinfo(const MSDevice_Tripinfo &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Tripinfo) -> Self {
        todo!("port MSDevice_Tripinfo::MSDevice_Tripinfo")
    }

    /// C++: `MSDevice_Tripinfo & MSDevice_Tripinfo::operator=(const MSDevice_Tripinfo &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Tripinfo) -> &mut MSDevice_Tripinfo {
        todo!("port MSDevice_Tripinfo::operator=")
    }

}

