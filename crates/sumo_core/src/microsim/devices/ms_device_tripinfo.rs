//! Port of microsim/devices/MSDevice_Tripinfo.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::Cell;
use utils::common::std_defs::g_precision;
use utils::common::string_utils::starts_with;
use utils::common::sumo_vehicle_class::{is_railway, SVC_BICYCLE, SVC_TAXI};
use utils::common::time::{delta_t, steps2time, time2steps, time2string};
use utils::common::to_string::{f64_to_string_default, f64_to_string_prec, to_string};
use utils::iodevices::output_device::AttrName;
use utils::options::options_cont::{get_bool, is_set, with_options};
use utils::xml::sumo_xml_definitions::{
    SUMO_ATTR_ARRIVALLANE, SUMO_ATTR_ARRIVALPOS, SUMO_ATTR_ARRIVALPOS_LAT, SUMO_ATTR_ARRIVALSPEED,
    SUMO_ATTR_ARRIVALTIME, SUMO_ATTR_COUNT, SUMO_ATTR_DURATION, SUMO_ATTR_ID, SUMO_ATTR_NUMBER,
    SUMO_ATTR_SPEED, SUMO_ATTR_STATE, SUMO_ATTR_STOPTIME, SUMO_ATTR_TIMELOSS, SUMO_ATTR_WAITINGCOUNT,
    SUMO_ATTR_WAITINGTIME, SUMO_TAG_DEVICE,
};

/// C++ `SUMO_const_haltingSpeed` (utils/common/StdDefs.h)
const SUMO_CONST_HALTING_SPEED: f64 = 0.1;

/// C++ `#define NOT_ARRIVED TIME2STEPS(-1)`
fn not_arrived() -> SUMOTime {
    time2steps(-1.0)
}

/// C++ `SIMSTEP`
fn sim_step() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}

/// C++ `#define STATE_EMPTY_ARRIVALLANE "NONE"`
const STATE_EMPTY_ARRIVALLANE: &str = "NONE";

/// The C++ static members of MSDevice_Tripinfo (listed above the impl block), kept in one record per thread.
/// The record is leaked once so that the pointers in myPendingOutput stay valid (as in ms_device_vehroutes.rs).
pub struct MSDevice_Tripinfo_Statics {
    /// C++ `myPendingOutput`, ordered by the holder's numerical ID (ComparatorNumericalIdLess)
    pub myPendingOutput: BTreeMap<SUMOTrafficObject_NumericalID, Ptr<MSDevice_Tripinfo>>,
    pub myVehicleCount: i32,
    pub myUndepartedVehicleCount: i32,
    pub myTotalRouteLength: f64,
    pub myTotalSpeed: f64,
    pub myTotalDuration: SUMOTime,
    pub myTotalWaitingTime: SUMOTime,
    pub myTotalTimeLoss: SUMOTime,
    pub myTotalDepartDelay: SUMOTime,
    pub myWaitingDepartDelay: SUMOTime,
    pub myBikeCount: i32,
    pub myTotalBikeRouteLength: f64,
    pub myTotalBikeSpeed: f64,
    pub myTotalBikeDuration: SUMOTime,
    pub myTotalBikeWaitingTime: SUMOTime,
    pub myTotalBikeTimeLoss: SUMOTime,
    pub myTotalBikeDepartDelay: SUMOTime,
    pub myWalkCount: i32,
    pub myTotalWalkRouteLength: f64,
    pub myTotalWalkDuration: SUMOTime,
    pub myTotalWalkTimeLoss: SUMOTime,
    /// index 0: persons, index 1: transports
    pub myRideCount: [i32; 2],
    pub myRideBusCount: [i32; 2],
    pub myRideRailCount: [i32; 2],
    pub myRideTaxiCount: [i32; 2],
    pub myRideBikeCount: [i32; 2],
    pub myRideAbortCount: [i32; 2],
    pub myTotalRideWaitingTime: [SUMOTime; 2],
    pub myTotalRideRouteLength: [f64; 2],
    pub myTotalRideDuration: [SUMOTime; 2],
}

thread_local! {
    static MY_STATICS: Cell<Ptr<MSDevice_Tripinfo_Statics>> = Cell::new(Ptr::default());
}

/// Access to the C++ statics of MSDevice_Tripinfo
fn st() -> Ptr<MSDevice_Tripinfo_Statics> {
    MY_STATICS.with(|c| {
        if c.get().is_null() {
            let s = MSDevice_Tripinfo_Statics {
                myPendingOutput: BTreeMap::new(),
                myVehicleCount: 0,
                myUndepartedVehicleCount: 0,
                myTotalRouteLength: 0.0,
                myTotalSpeed: 0.0,
                myTotalDuration: 0,
                myTotalWaitingTime: 0,
                myTotalTimeLoss: 0,
                myTotalDepartDelay: 0,
                myWaitingDepartDelay: -1,
                myBikeCount: 0,
                myTotalBikeRouteLength: 0.0,
                myTotalBikeSpeed: 0.0,
                myTotalBikeDuration: 0,
                myTotalBikeWaitingTime: 0,
                myTotalBikeTimeLoss: 0,
                myTotalBikeDepartDelay: 0,
                myWalkCount: 0,
                myTotalWalkRouteLength: 0.0,
                myTotalWalkDuration: 0,
                myTotalWalkTimeLoss: 0,
                myRideCount: [0; 2],
                myRideBusCount: [0; 2],
                myRideRailCount: [0; 2],
                myRideTaxiCount: [0; 2],
                myRideBikeCount: [0; 2],
                myRideAbortCount: [0; 2],
                myTotalRideWaitingTime: [0; 2],
                myTotalRideRouteLength: [0.0; 2],
                myTotalRideDuration: [0; 2],
            };
            c.set(Ptr::from_mut(Box::leak(Box::new(s))));
        }
        c.get()
    })
}

/// C++ `istringstream >> x` over the whitespace-separated state string: once an extraction fails, the stream stays failed
struct StateTokens<'a> {
    toks: std::str::SplitAsciiWhitespace<'a>,
    ok: bool,
}

impl<'a> StateTokens<'a> {
    fn read<T: std::str::FromStr>(&mut self, x: &mut T) {
        if self.ok {
            match self.toks.next().and_then(|t| t.parse::<T>().ok()) {
                Some(v) => *x = v,
                None => self.ok = false,
            }
        }
    }

    /// C++ `bool` extraction reads the integers 0 and 1
    fn read_bool(&mut self, x: &mut bool) {
        if self.ok {
            match self.toks.next().and_then(|t| t.parse::<i32>().ok()) {
                Some(0) => *x = false,
                Some(1) => *x = true,
                _ => self.ok = false,
            }
        }
    }
}

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
        oc.add_option_sub_topic("Tripinfo Device");
        MSDevice::insert_default_assignment_options("tripinfo", "Tripinfo Device", oc, false);
    }

    /// C++: `void MSDevice_Tripinfo::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        let enableByOutputOption = is_set("tripinfo-output") || get_bool("duration-log.statistics");
        let vp = Ptr::from_mut(v);
        let equipped = with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(
                oc,
                "tripinfo",
                &vp.get_id(),
                &vp.get_parameter().base,
                &vp.get_v_type_parameter().base,
                enableByOutputOption,
                false,
            )
        });
        if equipped {
            let id = format!("tripinfo_{}", vp.get_id());
            let device: Ptr<MSDevice_Tripinfo> = Ptr::from_mut(Box::leak(Box::new(MSDevice_Tripinfo::new(vp.m(), &id))));
            into.push(device.cast::<MSVehicleDevice>());
            st().m().myPendingOutput.insert(vp.get_numerical_id(), device);
        }
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
        let net = MSNet::get_instance();
        // the device handle must stay alive while the output is written
        let tripinfoOutRef = if is_set("tripinfo-output") { Some(OutputDevice::get_device_by_option("tripinfo-output").unwrap()) } else { None };
        let tripinfoOut: Ptr<OutputDevice> = match &tripinfoOutRef {
            Some(r) => Ptr::from_raw(r.as_ptr()),
            None => Ptr::null(),
        };
        st().m().myWaitingDepartDelay = 0;
        st().m().myUndepartedVehicleCount = 0;
        let writeUndeparted = get_bool("tripinfo-output.write-undeparted");
        let t = net.get_current_time_step();
        while !st().myPendingOutput.is_empty() {
            let d: Ptr<MSDevice_Tripinfo> = *st().myPendingOutput.values().next().unwrap();
            let departed = d.myHolder.has_departed();
            let departDelayed = d.myHolder.get_parameter().depart <= t;
            if !departed && departDelayed {
                st().m().myUndepartedVehicleCount += 1;
                st().m().myWaitingDepartDelay += t - d.myHolder.get_parameter().depart;
            }
            if departed || (writeUndeparted && departDelayed) {
                d.m().update_parking_stop_time();
                d.generate_output(tripinfoOut);
                if !tripinfoOut.is_null() {
                    for dev in d.myHolder.get_devices().iter() {
                        // typeid(*dev) comparison: the device names are unique per class
                        let name = dev.base2.device_name();
                        if name == "tripinfo" || name == "vehroutes" {
                            // tripinfo is special and vehroute has its own write-unfinished option
                            continue;
                        }
                        dev.generate_output(tripinfoOut);
                    }
                    tripinfoOut.m().close_tag("");
                }
            } else {
                st().m().myPendingOutput.remove(&d.myHolder.get_numerical_id());
            }
        }
        // unfinished persons
        if net.has_persons() {
            net.m().get_person_control().erase_all();
        }
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
        let sp = st(); let s = sp.m();
        s.myWalkCount += 1;
        s.myTotalWalkRouteLength += walkLength;
        s.myTotalWalkDuration += walkDuration;
        s.myTotalWalkTimeLoss += walkTimeLoss;
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
        let index: usize = if isPerson { 0 } else { 1 };
        let sp = st(); let s = sp.m();
        s.myRideCount[index] += 1;
        if duration > 0 {
            s.myTotalRideWaitingTime[index] += waitingTime;
            s.myTotalRideRouteLength[index] += distance;
            s.myTotalRideDuration[index] += duration;
            if vClass == SVC_BICYCLE {
                s.myRideBikeCount[index] += 1;
            } else if !line.is_empty() {
                if is_railway(vClass) {
                    s.myRideRailCount[index] += 1;
                } else if vClass == SVC_TAXI {
                    s.myRideTaxiCount[index] += 1;
                } else {
                    // some kind of road vehicle
                    s.myRideBusCount[index] += 1;
                }
            }
        } else {
            s.myRideAbortCount[index] += 1;
        }
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
        let p = g_precision();
        let f = |v: f64| f64_to_string_prec(v, p);
        let s = st();
        let mut msg = String::new();
        if s.myBikeCount == 0 || s.myVehicleCount > 0 {
            msg.push_str(&format!("Statistics (avg of {}):\n", s.myVehicleCount));
            msg.push_str(&format!(" RouteLength: {}\n", f(Self::get_avg_route_length())));
            msg.push_str(&format!(" Speed: {}\n", f(Self::get_avg_trip_speed())));
            msg.push_str(&format!(" Duration: {}\n", f(Self::get_avg_duration())));
            msg.push_str(&format!(" WaitingTime: {}\n", f(Self::get_avg_waiting_time())));
            msg.push_str(&format!(" TimeLoss: {}\n", f(Self::get_avg_time_loss())));
            msg.push_str(&format!(" DepartDelay: {}\n", f(Self::get_avg_depart_delay())));
        }
        if s.myBikeCount > 0 {
            msg.push_str(&format!("Bike Statistics (avg of {}):\n", s.myBikeCount));
            msg.push_str(&format!(" RouteLength: {}\n", f(Self::get_avg_bike_route_length())));
            msg.push_str(&format!(" Speed: {}\n", f(Self::get_avg_bike_trip_speed())));
            msg.push_str(&format!(" Duration: {}\n", f(Self::get_avg_bike_duration())));
            msg.push_str(&format!(" WaitingTime: {}\n", f(Self::get_avg_bike_waiting_time())));
            msg.push_str(&format!(" TimeLoss: {}\n", f(Self::get_avg_bike_time_loss())));
            msg.push_str(&format!(" DepartDelay: {}\n", f(Self::get_avg_bike_depart_delay())));
            if s.myVehicleCount > 0 && s.myWaitingDepartDelay >= 0 {
                msg.push_str(&format!("Statistics (avg of {}):\n", s.myVehicleCount + s.myBikeCount));
            }
        }
        if s.myWaitingDepartDelay >= 0 {
            msg.push_str(&format!(" DepartDelayWaiting: {}\n", f(Self::get_avg_depart_delay_waiting())));
        }
        if s.myWalkCount > 0 {
            msg.push_str(&format!("Pedestrian Statistics (avg of {} walks):\n", s.myWalkCount));
            msg.push_str(&format!(" RouteLength: {}\n", f(Self::get_avg_walk_route_length())));
            msg.push_str(&format!(" Duration: {}\n", f(Self::get_avg_walk_duration())));
            msg.push_str(&format!(" TimeLoss: {}\n", f(Self::get_avg_walk_time_loss())));
        }
        Self::print_ride_statistics(&mut msg, "Ride", "rides", 0);
        Self::print_ride_statistics(&mut msg, "Transport", "transports", 1);
        msg
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
        let s = st();
        od.set_precision(g_precision());
        od.open_tag("vehicleTripStatistics");
        od.write_attr("count", &s.myVehicleCount);
        od.write_attr("routeLength", &Self::get_avg_route_length());
        od.write_attr("speed", &Self::get_avg_trip_speed());
        od.write_attr("duration", &Self::get_avg_duration());
        od.write_attr("waitingTime", &Self::get_avg_waiting_time());
        od.write_attr("timeLoss", &Self::get_avg_time_loss());
        od.write_attr("departDelay", &Self::get_avg_depart_delay());
        od.write_attr("departDelayWaiting", &Self::get_avg_depart_delay_waiting());
        od.write_attr("totalTravelTime", &time2string(s.myTotalDuration));
        od.write_attr("totalDepartDelay", &time2string(time2steps(Self::get_total_depart_delay() + Self::get_total_bike_depart_delay())));
        od.close_tag("");
        if s.myBikeCount > 0 {
            od.open_tag("bikeTripStatistics");
            od.write_attr("count", &s.myBikeCount);
            od.write_attr("routeLength", &Self::get_avg_bike_route_length());
            od.write_attr("speed", &Self::get_avg_bike_trip_speed());
            od.write_attr("duration", &Self::get_avg_bike_duration());
            od.write_attr("waitingTime", &Self::get_avg_bike_waiting_time());
            od.write_attr("timeLoss", &Self::get_avg_bike_time_loss());
            od.write_attr("departDelay", &Self::get_avg_bike_depart_delay());
            od.write_attr("totalTravelTime", &time2string(s.myTotalBikeDuration));
            od.close_tag("");
        }
        od.open_tag("pedestrianStatistics");
        od.write_attr("number", &s.myWalkCount);
        od.write_attr("routeLength", &Self::get_avg_walk_route_length());
        od.write_attr("duration", &Self::get_avg_walk_duration());
        od.write_attr("timeLoss", &Self::get_avg_walk_time_loss());
        od.close_tag("");
        Self::write_ride_statistics(od, "rideStatistics", 0);
        Self::write_ride_statistics(od, "transportStatistics", 1);
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
        let s = st();
        if s.myVehicleCount > 0 {
            return s.myTotalRouteLength / s.myVehicleCount as f64;
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myVehicleCount > 0 {
            return s.myTotalSpeed / s.myVehicleCount as f64;
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myVehicleCount > 0 {
            return steps2time(s.myTotalDuration / s.myVehicleCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myVehicleCount > 0 {
            return steps2time(s.myTotalWaitingTime / s.myVehicleCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myVehicleCount > 0 {
            return steps2time(s.myTotalTimeLoss / s.myVehicleCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myVehicleCount > 0 {
            return steps2time(s.myTotalDepartDelay / s.myVehicleCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myWaitingDepartDelay >= 0 {
            return steps2time(s.myWaitingDepartDelay / std::cmp::max(1, s.myUndepartedVehicleCount) as i64);
        } else {
            return -1.0;
        }
    }

    /// C++: `double MSDevice_Tripinfo::getTotalDepartDelay()` [static]
    // double
    // MSDevice_Tripinfo::getTotalDepartDelay() {
    //     return STEPS2TIME(myTotalDepartDelay + MAX2((SUMOTime)0, myWaitingDepartDelay));
    // }
    #[allow(unused_variables)]
    pub fn get_total_depart_delay() -> f64 {
        let s = st();
        return steps2time(s.myTotalDepartDelay + std::cmp::max(0, s.myWaitingDepartDelay));
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
        let s = st();
        if s.myBikeCount > 0 {
            return s.myTotalBikeRouteLength / s.myBikeCount as f64;
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myBikeCount > 0 {
            return s.myTotalBikeSpeed / s.myBikeCount as f64;
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myBikeCount > 0 {
            return steps2time(s.myTotalBikeDuration / s.myBikeCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myBikeCount > 0 {
            return steps2time(s.myTotalBikeWaitingTime / s.myBikeCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myBikeCount > 0 {
            return steps2time(s.myTotalBikeTimeLoss / s.myBikeCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myBikeCount > 0 {
            return steps2time(s.myTotalBikeDepartDelay / s.myBikeCount as i64);
        } else {
            return 0.0;
        }
    }

    /// C++: `double MSDevice_Tripinfo::getTotalBikeDepartDelay()` [static]
    // double
    // MSDevice_Tripinfo::getTotalBikeDepartDelay() {
    //     return STEPS2TIME(myTotalBikeDepartDelay);
    // }
    #[allow(unused_variables)]
    pub fn get_total_bike_depart_delay() -> f64 {
        steps2time(st().myTotalBikeDepartDelay)
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
        let s = st();
        if s.myWalkCount > 0 {
            return s.myTotalWalkRouteLength / s.myWalkCount as f64;
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myWalkCount > 0 {
            return steps2time(s.myTotalWalkDuration / s.myWalkCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myWalkCount > 0 {
            return steps2time(s.myTotalWalkTimeLoss / s.myWalkCount as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myRideCount[0] > 0 {
            return s.myTotalRideRouteLength[0] / s.myRideCount[0] as f64;
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myRideCount[0] > 0 {
            return steps2time(s.myTotalRideWaitingTime[0] / s.myRideCount[0] as i64);
        } else {
            return 0.0;
        }
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
        let s = st();
        if s.myRideCount[0] > 0 {
            return steps2time(s.myTotalRideDuration[0] / s.myRideCount[0] as i64);
        } else {
            return 0.0;
        }
    }

    /// C++: `void MSDevice_Tripinfo::~MSDevice_Tripinfo()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // ensure clean up for vaporized vehicles which do not generate output
        st().m().myPendingOutput.remove(&self.myHolder.get_numerical_id());
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
        let sp = st(); let s = sp.m();
        s.myVehicleCount = 0;
        s.myTotalRouteLength = 0.0;
        s.myTotalSpeed = 0.0;
        s.myTotalDuration = 0;
        s.myTotalWaitingTime = 0;
        s.myTotalTimeLoss = 0;
        s.myTotalDepartDelay = 0;
        s.myWaitingDepartDelay = -1;

        s.myBikeCount = 0;
        s.myTotalBikeRouteLength = 0.0;
        s.myTotalBikeSpeed = 0.0;
        s.myTotalBikeDuration = 0;
        s.myTotalBikeWaitingTime = 0;
        s.myTotalBikeTimeLoss = 0;
        s.myTotalBikeDepartDelay = 0;

        s.myWalkCount = 0;
        s.myTotalWalkRouteLength = 0.0;
        s.myTotalWalkDuration = 0;
        s.myTotalWalkTimeLoss = 0;

        s.myRideCount = [0, 0];
        s.myRideBusCount = [0, 0];
        s.myRideRailCount = [0, 0];
        s.myRideTaxiCount = [0, 0];
        s.myRideBikeCount = [0, 0];
        s.myRideAbortCount = [0, 0];
        s.myTotalRideWaitingTime = [0, 0];
        s.myTotalRideRouteLength = [0.0, 0.0];
        s.myTotalRideDuration = [0, 0];
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
        if veh.is_stopped() {
            if newSpeed <= SUMO_CONST_HALTING_SPEED {
                self.myStoppingTime += delta_t();
            }
        } else if newSpeed <= SUMO_CONST_HALTING_SPEED && Self::low_acceleration(veh) {
            self.myWaitingTime += delta_t();
            if !self.myAmWaiting {
                self.myWaitingCount += 1;
                self.myAmWaiting = true;
            }
        } else {
            self.myAmWaiting = false;
        }
        true
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
        if veh.is_vehicle() {
            self.myWaitingTime += delta_t();
            if !self.myAmWaiting {
                self.myWaitingCount += 1;
                self.myAmWaiting = true;
            }
        }
        true
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
        if std::env::var("MESO_TRACE").is_ok() { eprintln!("TRACE tripinfo notify {} departed={} sv={} vtget={:x} obj={:x}", veh.get_id(), matches!(reason, MSMoveReminder_Notification::NOTIFICATION_DEPARTED), veh.get_speed(), veh.vt.get_speed as usize, veh as *const SUMOTrafficObject as usize); }
        if matches!(reason, MSMoveReminder_Notification::NOTIFICATION_DEPARTED) {
            if !crate::microsim::ms_globals::use_meso_sim() {
                // C++ static_cast<MSVehicle&>(veh): the notified object is the holder vehicle
                self.myDepartLane = Ptr::from_mut(veh).cast::<MSVehicle>().get_lane().get_id().to_string();
                self.myDepartPosLat = Ptr::from_mut(veh).cast::<MSVehicle>().get_lateral_position_on_lane();
            } else {
                self.myDepartLane = veh.get_edge().get_first_allowed(veh.get_v_class(), true, 0).get_id().to_string();
            }
            self.myDepartSpeed = veh.get_speed();
            self.myRouteLength = -veh.get_position_on_lane();
        } else if matches!(reason, MSMoveReminder_Notification::NOTIFICATION_PARKING) {
            // notifyMove is not called while parking
            // @note insertion delay when resuming after parking is included
            self.update_parking_stop_time();
        }
        true
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
    //         if (reason > MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED) {
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
        if (reason as i32) >= (MSMoveReminder_Notification::NOTIFICATION_ARRIVED as i32) {
            self.myArrivalTime = MSNet::get_instance().get_current_time_step();
            self.myArrivalReason = reason;
            if !crate::microsim::ms_globals::use_meso_sim() {
                // C++ static_cast<MSVehicle&>(veh): the notified object is the holder vehicle
                self.myArrivalLane = Ptr::from_mut(veh).cast::<MSVehicle>().get_lane().get_id().to_string();
                self.myArrivalPosLat = Ptr::from_mut(veh).cast::<MSVehicle>().get_lateral_position_on_lane();
            } else {
                self.myArrivalLane = veh.get_edge().get_first_allowed(veh.get_v_class(), true, 0).get_id().to_string();
            }
            // @note vehicle may have moved past its arrivalPos during the last step
            // due to non-zero arrivalspeed but we consider it as arrived at the desired position
            // However, vaporization may happen anywhere (via TraCI)
            if (reason as i32) > (MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED as i32) {
                // vaporized
                self.myArrivalPos = veh.get_position_on_lane();
            } else {
                self.myArrivalPos = self.myHolder.get_arrival_pos();
            }
            self.myArrivalSpeed = veh.get_speed();
            self.update_parking_stop_time();
        } else if matches!(reason, MSMoveReminder_Notification::NOTIFICATION_PARKING) {
            self.myParkingStarted = MSNet::get_instance().get_current_time_step();
        } else if matches!(
            reason,
            MSMoveReminder_Notification::NOTIFICATION_JUNCTION
                | MSMoveReminder_Notification::NOTIFICATION_TELEPORT
                | MSMoveReminder_Notification::NOTIFICATION_TELEPORT_CONTINUATION
        ) {
            if crate::microsim::ms_globals::use_meso_sim() {
                self.myRouteLength += self.myHolder.get_current_edge().get_length();
            } else {
                let lane = Ptr::from_mut(veh).cast::<MSVehicle>().get_lane();
                if !lane.is_null() {
                    self.myRouteLength += lane.get_length();
                }
            }
        }
        true
    }

    /// C++: `const std::string MSDevice_Tripinfo::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "tripinfo";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "tripinfo".to_string()
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
        let timeLoss: SUMOTime = if crate::microsim::ms_globals::use_meso_sim() {
            self.myMesoTimeLoss
        } else {
            self.myHolder.cast::<MSVehicle>().get_time_loss()
        };
        let routeLength = self.myRouteLength + if self.myArrivalTime == not_arrived() { self.myHolder.get_position_on_lane() } else { self.myArrivalPos };
        let mut duration: SUMOTime = 0;
        if self.myHolder.has_departed() {
            duration = (if self.myArrivalTime == not_arrived() { sim_step() } else { self.myArrivalTime }) - self.myHolder.get_departure();
            let drivingDuration = duration - self.myStoppingTime;
            let speed = if drivingDuration > 0 { routeLength / steps2time(drivingDuration) } else { 0.0 };
            if self.myHolder.get_v_class() == SVC_BICYCLE {
                let sp = st(); let s = sp.m();
                s.myBikeCount += 1;
                s.myTotalBikeRouteLength += routeLength;
                s.myTotalBikeSpeed += speed;
                s.myTotalBikeDuration += duration;
                s.myTotalBikeWaitingTime += self.myWaitingTime;
                s.myTotalBikeTimeLoss += timeLoss;
                s.myTotalBikeDepartDelay += self.myHolder.get_depart_delay();
            } else {
                let sp = st(); let s = sp.m();
                s.myVehicleCount += 1;
                s.myTotalRouteLength += routeLength;
                s.myTotalSpeed += speed;
                s.myTotalDuration += duration;
                s.myTotalWaitingTime += self.myWaitingTime;
                s.myTotalTimeLoss += timeLoss;
                s.myTotalDepartDelay += self.myHolder.get_depart_delay();
            }
        }

        st().m().myPendingOutput.remove(&self.myHolder.get_numerical_id());
        if tripinfoOut.is_null() {
            return;
        }
        // write
        let os = tripinfoOut.m();
        os.open_tag("tripinfo").write_attr("id", self.myHolder.get_id());
        let depart = if self.myHolder.has_departed() { time2string(self.myHolder.get_departure()) } else { "-1".to_string() };
        os.write_attr("depart", &depart);
        os.write_attr("departLane", &self.myDepartLane);
        os.write_attr("departPos", &self.myHolder.get_depart_pos());
        if crate::microsim::ms_globals::lateral_resolution() > 0.0 {
            os.write_attr("departPosLat", &self.myDepartPosLat);
        }
        os.write_attr("departSpeed", &self.myDepartSpeed);
        let mut departDelay = self.myHolder.get_depart_delay();
        let param = self.myHolder.get_parameter();
        if !self.myHolder.has_departed() {
            // C++ assert(param.depart <= SIMSTEP || param.departProcedure != DepartDefinition::GIVEN) is compiled out of release builds
            departDelay = sim_step() - param.depart;
        }
        os.write_attr("departDelay", &time2string(departDelay));
        os.write_attr("arrival", &time2string(self.myArrivalTime));
        os.write_attr("arrivalLane", &self.myArrivalLane);
        os.write_attr("arrivalPos", &self.myArrivalPos);
        if crate::microsim::ms_globals::lateral_resolution() > 0.0 {
            os.write_attr("arrivalPosLat", &self.myArrivalPosLat);
        }
        os.write_attr("arrivalSpeed", &self.myArrivalSpeed);
        os.write_attr("duration", &time2string(duration));
        os.write_attr("routeLength", &routeLength);
        os.write_attr(SUMO_ATTR_WAITINGTIME, &time2string(self.myWaitingTime));
        os.write_attr(SUMO_ATTR_WAITINGCOUNT, &self.myWaitingCount);
        os.write_attr(SUMO_ATTR_STOPTIME, &time2string(self.myStoppingTime));
        os.write_attr(SUMO_ATTR_TIMELOSS, &time2string(timeLoss));
        os.write_attr("rerouteNo", &self.myHolder.get_number_reroutes());
        // C++ toString(vector of devices): the device ids separated by spaces
        let devices: Vec<String> = self.myHolder.get_devices().iter().map(|d| d.base2.get_id().to_string()).collect();
        os.write_attr("devices", &devices.join(" "));
        os.write_attr("vType", self.myHolder.get_vehicle_type().get_id());
        os.write_attr("speedFactor", &self.myHolder.get_chosen_speed_factor());
        let vaporized: &str = match self.myArrivalReason {
            MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_CALIBRATOR => "calibrator",
            MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_GUI => "gui",
            MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_COLLISION => "collision",
            MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_VAPORIZER => "vaporizer",
            MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_TRACI => "traci",
            MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED => "teleport",
            _ => {
                let param = self.myHolder.get_parameter();
                if self.myHolder.get_edge() == self.myHolder.get_route().get_last_edge()
                    || (param.arrivalEdge >= 0 && self.myHolder.get_route_position() >= param.arrivalEdge)
                {
                    ""
                } else {
                    "end"
                }
            }
        };
        os.write_attr("vaporized", vaporized);
        // cannot close tag because emission device output might follow
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
    //         internals << myDepartLane << " ";
    //         if (!MSGlobals::gUseMesoSim) {
    //             internals << myDepartPosLat << " ";
    //         }
    //         std::string state_arrivalLane = myArrivalLane == "" ? STATE_EMPTY_ARRIVALLANE : myArrivalLane;
    //         internals << myDepartSpeed << " " << myRouteLength << " " << myWaitingTime << " " << myAmWaiting << " " << myWaitingCount << " ";
    //         internals << myStoppingTime << " " << myParkingStarted << " ";
    //         internals << myArrivalTime << " " << state_arrivalLane << " " << myArrivalPos << " " << myArrivalPosLat << " " << myArrivalSpeed;
    //         out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        // always write device id to replicate stochastic assignment
        out.open_tag_id(SUMO_TAG_DEVICE);
        out.write_attr(SUMO_ATTR_ID, self.base.base2.get_id());
        if self.myHolder.has_departed() {
            let mut internals = String::new();
            internals.push_str(&format!("{} ", self.myDepartLane));
            if !crate::microsim::ms_globals::use_meso_sim() {
                internals.push_str(&format!("{} ", f64_to_string_default(self.myDepartPosLat)));
            }
            let state_arrivalLane: &str = if self.myArrivalLane.is_empty() { STATE_EMPTY_ARRIVALLANE } else { &self.myArrivalLane };
            internals.push_str(&format!(
                "{} {} {} {} {} ",
                f64_to_string_default(self.myDepartSpeed),
                f64_to_string_default(self.myRouteLength),
                self.myWaitingTime,
                self.myAmWaiting as i32,
                self.myWaitingCount
            ));
            internals.push_str(&format!("{} {} ", self.myStoppingTime, self.myParkingStarted));
            internals.push_str(&format!(
                "{} {} {} {} {}",
                self.myArrivalTime,
                state_arrivalLane,
                f64_to_string_default(self.myArrivalPos),
                f64_to_string_default(self.myArrivalPosLat),
                f64_to_string_default(self.myArrivalSpeed)
            ));
            out.write_attr(SUMO_ATTR_STATE, &internals);
        }
        out.close_tag("");
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
    //         if (myArrivalLane == STATE_EMPTY_ARRIVALLANE) {
    //             myArrivalLane = "";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        if attrs.has_attribute(SUMO_ATTR_STATE) {
            let state = attrs.get_string(SUMO_ATTR_STATE);
            let mut bis = StateTokens { toks: state.split_ascii_whitespace(), ok: true };
            bis.read(&mut self.myDepartLane);
            if !crate::microsim::ms_globals::use_meso_sim() {
                bis.read(&mut self.myDepartPosLat);
            }
            bis.read(&mut self.myDepartSpeed);
            bis.read(&mut self.myRouteLength);
            bis.read(&mut self.myWaitingTime);
            bis.read_bool(&mut self.myAmWaiting);
            bis.read(&mut self.myWaitingCount);
            bis.read(&mut self.myStoppingTime);
            bis.read(&mut self.myParkingStarted);
            bis.read(&mut self.myArrivalTime);
            bis.read(&mut self.myArrivalLane);
            bis.read(&mut self.myArrivalPos);
            bis.read(&mut self.myArrivalPosLat);
            bis.read(&mut self.myArrivalSpeed);
            if self.myArrivalLane == STATE_EMPTY_ARRIVALLANE {
                self.myArrivalLane = String::new();
            }
        }
    }

    /// C++: `void MSDevice_Tripinfo::updateStopTime(const SUMOTime)`
    /// update stopping time for meso
    // void updateStopTime(const SUMOTime time) {
    //         myStoppingTime += time;
    //     }
    #[allow(unused_variables)]
    pub fn update_stop_time(&mut self, time: SUMOTime) {
        self.myStoppingTime += time;
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
        if key == SUMO_ATTR_WAITINGTIME.attr_name() {
            to_string(&steps2time(self.myWaitingTime))
        } else if key == SUMO_ATTR_WAITINGCOUNT.attr_name() {
            to_string(&self.myWaitingCount)
        } else if key == SUMO_ATTR_STOPTIME.attr_name() {
            to_string(&steps2time(self.myStoppingTime))
        } else if key == SUMO_ATTR_ARRIVALTIME.attr_name() {
            to_string(&steps2time(self.myArrivalTime))
        } else if key == SUMO_ATTR_ARRIVALLANE.attr_name() {
            self.myArrivalLane.clone()
        } else if key == SUMO_ATTR_ARRIVALPOS.attr_name() {
            to_string(&self.myArrivalPos)
        } else if key == SUMO_ATTR_ARRIVALPOS_LAT.attr_name() {
            to_string(&self.myArrivalPosLat)
        } else if key == SUMO_ATTR_ARRIVALSPEED.attr_name() {
            to_string(&self.myArrivalSpeed)
        } else {
            panic!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())
        }
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
        let mut key: String = prefixedKey.to_string(); // by default, assume vehicleTripStatistics
        let err = format!("Parameter '{}' is not supported for device of type 'tripinfo'", prefixedKey);
        let s = st();
        if starts_with(&key, "vehicleTripStatistics.") {
            key = prefixedKey[22..].to_string();
        } else if starts_with(&key, "bikeTripStatistics.") {
            key = prefixedKey[19..].to_string();
            if key == SUMO_ATTR_COUNT.attr_name() {
                return to_string(&s.myBikeCount);
            } else if key == "routeLength" {
                return to_string(&Self::get_avg_bike_route_length());
            } else if key == SUMO_ATTR_SPEED.attr_name() {
                return to_string(&Self::get_avg_bike_trip_speed());
            } else if key == SUMO_ATTR_DURATION.attr_name() {
                return to_string(&Self::get_avg_bike_duration());
            } else if key == SUMO_ATTR_WAITINGTIME.attr_name() {
                return to_string(&Self::get_avg_bike_waiting_time());
            } else if key == SUMO_ATTR_TIMELOSS.attr_name() {
                return to_string(&Self::get_avg_bike_time_loss());
            } else if key == "departDelay" {
                return to_string(&Self::get_avg_bike_depart_delay());
            } else if key == "totalTravelTime" {
                // avoid human readable output
                return to_string(&steps2time(s.myTotalBikeDuration));
            }
            panic!("{}", err);
        } else if starts_with(&key, "pedestrianStatistics.") {
            key = prefixedKey[21..].to_string();
            if key == SUMO_ATTR_NUMBER.attr_name() || key == SUMO_ATTR_COUNT.attr_name() {
                return to_string(&s.myWalkCount);
            } else if key == "routeLength" {
                return to_string(&Self::get_avg_walk_route_length());
            } else if key == SUMO_ATTR_DURATION.attr_name() {
                return to_string(&Self::get_avg_walk_duration());
            } else if key == SUMO_ATTR_TIMELOSS.attr_name() {
                return to_string(&Self::get_avg_walk_time_loss());
            }
            panic!("{}", err);
        } else if starts_with(&key, "rideStatistics.") || starts_with(&key, "transportStatistics.") {
            let mut index: usize = 0;
            if starts_with(&key, "rideStatistics.") {
                key = prefixedKey[15..].to_string();
            } else {
                index = 1;
                key = prefixedKey[20..].to_string();
            }
            if key == SUMO_ATTR_NUMBER.attr_name() || key == SUMO_ATTR_COUNT.attr_name() {
                return to_string(&s.myRideCount[index]);
            } else if key == SUMO_ATTR_WAITINGTIME.attr_name() {
                return to_string(&steps2time(s.myTotalRideWaitingTime[index] / std::cmp::max(1, s.myRideCount[index]) as i64));
            } else if key == "routeLength" {
                return to_string(&(s.myTotalRideRouteLength[index] / std::cmp::max(1, s.myRideCount[index]) as f64));
            } else if key == SUMO_ATTR_DURATION.attr_name() {
                // C++ uses the route length here as well
                return to_string(&(s.myTotalRideRouteLength[index] / std::cmp::max(1, s.myRideCount[index]) as f64));
            } else if key == "bus" {
                return to_string(&s.myRideBusCount[index]);
            } else if key == "train" {
                return to_string(&s.myRideRailCount[index]);
            } else if key == "taxi" {
                return to_string(&s.myRideTaxiCount[index]);
            } else if key == "bike" {
                return to_string(&s.myRideBikeCount[index]);
            } else if key == "aborted" {
                return to_string(&s.myRideAbortCount[index]);
            }
            panic!("{}", err);
        }
        // vehicleTripStatistics
        if key == SUMO_ATTR_COUNT.attr_name() {
            to_string(&s.myVehicleCount)
        } else if key == "routeLength" {
            to_string(&Self::get_avg_route_length())
        } else if key == SUMO_ATTR_SPEED.attr_name() {
            to_string(&Self::get_avg_trip_speed())
        } else if key == SUMO_ATTR_DURATION.attr_name() {
            to_string(&Self::get_avg_duration())
        } else if key == SUMO_ATTR_WAITINGTIME.attr_name() {
            to_string(&Self::get_avg_waiting_time())
        } else if key == SUMO_ATTR_TIMELOSS.attr_name() {
            to_string(&Self::get_avg_time_loss())
        } else if key == "departDelay" {
            to_string(&Self::get_avg_depart_delay())
        } else if key == "departDelayWaiting" {
            to_string(&Self::get_avg_depart_delay_waiting())
        } else if key == "totalTravelTime" {
            // avoid human readable output
            to_string(&steps2time(s.myTotalDuration))
        } else if key == "totalDepartDelay" {
            to_string(&Self::get_total_depart_delay())
        } else {
            panic!("{}", err)
        }
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
        self.myMesoTimeLoss += waitingTime;
        self.myWaitingTime += waitingTime;
        self.myWaitingCount += 1;
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
        let mut this = MSDevice_Tripinfo {
            base: MSVehicleDevice::new(holder, id),
            myDepartLane: String::new(),
            myDepartSpeed: -1.0,
            myDepartPosLat: 0.0,
            myWaitingTime: 0,
            myAmWaiting: false,
            myWaitingCount: 0,
            myStoppingTime: 0,
            myParkingStarted: -1,
            myArrivalTime: not_arrived(),
            myArrivalLane: String::new(),
            myArrivalPos: -1.0,
            myArrivalPosLat: 0.0,
            myArrivalSpeed: -1.0,
            myArrivalReason: MSMoveReminder_Notification::NOTIFICATION_ARRIVED,
            myMesoTimeLoss: 0,
            myRouteLength: 0.0,
        };
        this.base.base.vt = &MS_DEVICE_TRIPINFO_MSMOVE_REMINDER_VT;
        this.base.base.idle_vt = &MS_DEVICE_TRIPINFO_MSMOVE_REMINDER_IDLE_VT;
        this
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
        // called by meso
        let vmax = veh.get_edge().get_vehicle_max_speed(Ptr::new(veh));
        if vmax > 0.0 {
            self.myMesoTimeLoss += time2steps(timeOnLane * (vmax - meanSpeedVehicleOnLane) / vmax);
        }
        self.myWaitingTime += veh.get_waiting_time(false);
        if veh.get_waiting_time(false) >= time2steps(1.0) {
            // waiting counts the time spent waiting to enter the next link (when it's occupied).
            self.myWaitingCount += 1;
        }
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
        if self.myParkingStarted >= 0 {
            self.myStoppingTime += sim_step() - self.myParkingStarted;
            self.myParkingStarted = -1;
        }
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
    pub fn print_ride_statistics(msg: &mut String /* std::ostringstream */, category: &str, modeName: &str, index: i32) {
        let p = g_precision();
        let f = |v: f64| f64_to_string_prec(v, p);
        let i = index as usize;
        let s = st();
        if s.myRideCount[i] > 0 {
            msg.push_str(&format!("{} Statistics (avg of {} {}):\n", category, s.myRideCount[i], modeName));
            msg.push_str(&format!(" WaitingTime: {}\n", f(steps2time(s.myTotalRideWaitingTime[i] / s.myRideCount[i] as i64))));
            msg.push_str(&format!(" RouteLength: {}\n", f(s.myTotalRideRouteLength[i] / s.myRideCount[i] as f64)));
            msg.push_str(&format!(" Duration: {}\n", f(steps2time(s.myTotalRideDuration[i] / s.myRideCount[i] as i64))));
            if s.myRideBusCount[i] > 0 {
                msg.push_str(&format!(" Bus: {}\n", s.myRideBusCount[i]));
            }
            if s.myRideRailCount[i] > 0 {
                msg.push_str(&format!(" Train: {}\n", s.myRideRailCount[i]));
            }
            if s.myRideTaxiCount[i] > 0 {
                msg.push_str(&format!(" Taxi: {}\n", s.myRideTaxiCount[i]));
            }
            if s.myRideBikeCount[i] > 0 {
                msg.push_str(&format!(" Bike: {}\n", s.myRideBikeCount[i]));
            }
            if s.myRideAbortCount[i] > 0 {
                msg.push_str(&format!(" Aborted: {}\n", s.myRideAbortCount[i]));
            }
        }
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
        let i = index as usize;
        let s = st();
        od.open_tag(category);
        od.write_attr("number", &s.myRideCount[i]);
        if s.myRideCount[i] > 0 {
            od.write_attr("waitingTime", &steps2time(s.myTotalRideWaitingTime[i] / s.myRideCount[i] as i64));
            od.write_attr("routeLength", &(s.myTotalRideRouteLength[i] / s.myRideCount[i] as f64));
            od.write_attr("duration", &steps2time(s.myTotalRideDuration[i] / s.myRideCount[i] as i64));
            od.write_attr("bus", &s.myRideBusCount[i]);
            od.write_attr("train", &s.myRideRailCount[i]);
            od.write_attr("taxi", &s.myRideTaxiCount[i]);
            od.write_attr("bike", &s.myRideBikeCount[i]);
            od.write_attr("aborted", &s.myRideAbortCount[i]);
        }
        od.close_tag("");
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
        if crate::microsim::ms_globals::use_meso_sim() {
            // acceleration is not modelled
            return false;
        } else {
            let v = Ptr::new(veh).cast::<MSVehicle>();
            return v.get_acceleration() <= v.accel_threshold_for_waiting();
        }
    }

    /// C++: `void MSDevice_Tripinfo::MSDevice_Tripinfo(const MSDevice_Tripinfo &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Tripinfo) -> Self {
        // C++ declares the copy constructor private and never defines it: any use is a link error there
        panic!("MSDevice_Tripinfo is not copyable (C++ declares the copy constructor private and undefined)")
    }

    /// C++: `MSDevice_Tripinfo & MSDevice_Tripinfo::operator=(const MSDevice_Tripinfo &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Tripinfo) -> &mut MSDevice_Tripinfo {
        // C++ declares the assignment operator private and never defines it
        panic!("MSDevice_Tripinfo is not assignable (C++ declares operator= private and undefined)")
    }

}

fn ms_device_tripinfo_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    let dev = Ptr::from_mut(mr).cast::<MSDevice_Tripinfo>().get_mut().unwrap();
    dev.notify_enter(veh, reason, enteredLane)
}
fn ms_device_tripinfo_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
    let dev = Ptr::from_mut(mr).cast::<MSDevice_Tripinfo>().get_mut().unwrap();
    dev.notify_move(veh, oldPos, newPos, newSpeed)
}
fn ms_device_tripinfo_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    let dev = Ptr::from_mut(mr).cast::<MSDevice_Tripinfo>().get_mut().unwrap();
    dev.notify_leave(veh, lastPos, reason, enteredLane)
}

static MS_DEVICE_TRIPINFO_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: ms_device_tripinfo_notify_enter,
    notify_move: ms_device_tripinfo_notify_move,
    notify_leave: ms_device_tripinfo_notify_leave,
};
fn ms_device_tripinfo_notify_idle(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject) -> bool {
    let dev = Ptr::from_mut(mr).cast::<MSDevice_Tripinfo>().get_mut().unwrap();
    dev.notify_idle(veh)
}

static MS_DEVICE_TRIPINFO_MSMOVE_REMINDER_IDLE_VT: MSMoveReminderIdleVt = MSMoveReminderIdleVt {
    notify_idle: ms_device_tripinfo_notify_idle,
    notify_parking: crate::microsim::ms_move_reminder::default_notify_parking,
    notify_stop_ended: crate::microsim::ms_move_reminder::default_notify_stop_ended,
};

