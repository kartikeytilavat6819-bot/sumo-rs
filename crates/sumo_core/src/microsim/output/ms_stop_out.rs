//! Port of microsim/output/MSStopOut.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping the complete network state
/// C++ class `MSStopOut` (microsim/output/MSStopOut.h)
pub struct MSStopOut {
    pub myStopped: BTreeMap<Ptr<SUMOVehicle>, MSStopOut_StopInfo>, // C++: std::map<const SUMOVehicle *, StopInfo, ComparatorNumericalIdLess>
    pub myDevice: Ptr<OutputDevice>,
}

// C++ static members of MSStopOut (port as module-level statics, see AGENTS.md):
//   static MSStopOut * MSStopOut::myInstance;

impl MSStopOut {
    /// C++: `void MSStopOut::init()` [static]
    /// Static intialization
    // void
    // MSStopOut::init() {
    //     if (OptionsCont::getOptions().isSet("stop-output")) {
    //         myInstance = new MSStopOut(OutputDevice::getDeviceByOption("stop-output"));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init() {
        todo!("port MSStopOut::init")
    }

    /// C++: `bool MSStopOut::active()` [static]
    // static bool active() {
    //         return myInstance != 0;
    //     }
    #[allow(unused_variables)]
    pub fn active() -> bool {
        todo!("port MSStopOut::active")
    }

    /// C++: `void MSStopOut::cleanup()` [static]
    // void
    // MSStopOut::cleanup() {
    //     delete myInstance;
    //     myInstance = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSStopOut::cleanup")
    }

    /// C++: `MSStopOut * MSStopOut::getInstance()` [static]
    // static MSStopOut* getInstance() {
    //         return myInstance;
    //     }
    #[allow(unused_variables)]
    pub fn get_instance() -> Ptr<MSStopOut> {
        todo!("port MSStopOut::getInstance")
    }

    /// C++: `void MSStopOut::MSStopOut(OutputDevice &)`
    /// constructor.
    // MSStopOut::MSStopOut(OutputDevice& dev) :
    //     myDevice(dev) {
    // }
    #[allow(unused_variables)]
    pub fn new(dev: &mut OutputDevice) -> Self {
        todo!("port MSStopOut::MSStopOut")
    }

    /// C++: `void MSStopOut::~MSStopOut()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSStopOut::~MSStopOut")
    }

    /// C++: `void MSStopOut::stopBlocked(const SUMOVehicle *, SUMOTime)`
    // void
    // MSStopOut::stopBlocked(const SUMOVehicle* veh, SUMOTime time) {
    //     assert(veh != nullptr);
    //     if (myStopped.count(veh) == 0) {
    //         myStopped.emplace(veh, StopInfo(-time, -1, -1));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn stop_blocked(&mut self, veh: Ptr<SUMOVehicle>, time: SUMOTime) {
        todo!("port MSStopOut::stopBlocked")
    }

    /// C++: `void MSStopOut::stopNotStarted(const SUMOVehicle *)`
    // void
    // MSStopOut::stopNotStarted(const SUMOVehicle* veh) {
    //     assert(veh != nullptr);
    //     myStopped.erase(veh);
    // }
    #[allow(unused_variables)]
    pub fn stop_not_started(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSStopOut::stopNotStarted")
    }

    /// C++: `void MSStopOut::stopStarted(const SUMOVehicle *, int, int, SUMOTime)`
    // void
    // MSStopOut::stopStarted(const SUMOVehicle* veh, int numPersons, int numContainers, SUMOTime time) {
    //     assert(veh != nullptr);
    //     if (myStopped.count(veh) == 0) {
    //         myStopped.emplace(veh, StopInfo(0, numPersons, numContainers));
    //     } else {
    //         MSStopOut::StopInfo& info = myStopped.find(veh)->second;
    //         info.blockTime += time;
    //         info.initialNumPersons = numPersons;
    //         info.initialNumContainers = numContainers;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn stop_started(&mut self, veh: Ptr<SUMOVehicle>, numPersons: i32, numContainers: i32, time: SUMOTime) {
        todo!("port MSStopOut::stopStarted")
    }

    /// C++: `void MSStopOut::loadedPersons(const SUMOVehicle *, int)`
    // void
    // MSStopOut::loadedPersons(const SUMOVehicle* veh, int n) {
    //     // ignore triggered vehicles
    //     if (veh->hasDeparted()) {
    //         if (myStopped.count(veh) == 0) {
    //             WRITE_WARNINGF(TL("Vehicle '%' loads persons on edge '%', time=% without starting the stop."),
    //                            veh->getID(), veh->getEdge()->getID(), time2string(SIMSTEP));
    //         } else {
    //             myStopped.find(veh)->second.loadedPersons += n;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn loaded_persons(&mut self, veh: Ptr<SUMOVehicle>, n: i32) {
        todo!("port MSStopOut::loadedPersons")
    }

    /// C++: `void MSStopOut::unloadedPersons(const SUMOVehicle *, int)`
    // void
    // MSStopOut::unloadedPersons(const SUMOVehicle* veh, int n) {
    //     if (myStopped.count(veh) == 0) {
    //         WRITE_WARNINGF(TL("Vehicle '%' unloads persons on edge '%', time=% without starting the stop."),
    //                        veh->getID(), veh->getEdge()->getID(), time2string(SIMSTEP));
    //     } else {
    //         myStopped.find(veh)->second.unloadedPersons += n;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn unloaded_persons(&mut self, veh: Ptr<SUMOVehicle>, n: i32) {
        todo!("port MSStopOut::unloadedPersons")
    }

    /// C++: `void MSStopOut::loadedContainers(const SUMOVehicle *, int)`
    // void
    // MSStopOut::loadedContainers(const SUMOVehicle* veh, int n) {
    //     // ignore triggered vehicles
    //     if (veh->hasDeparted()) {
    //         if (myStopped.count(veh) == 0) {
    //             WRITE_WARNINGF(TL("Vehicle '%' loads container on edge '%', time=% without starting the stop."),
    //                            veh->getID(), veh->getEdge()->getID(), time2string(SIMSTEP));
    //         } else {
    //             myStopped.find(veh)->second.loadedContainers += n;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn loaded_containers(&mut self, veh: Ptr<SUMOVehicle>, n: i32) {
        todo!("port MSStopOut::loadedContainers")
    }

    /// C++: `void MSStopOut::unloadedContainers(const SUMOVehicle *, int)`
    // void
    // MSStopOut::unloadedContainers(const SUMOVehicle* veh, int n) {
    //     if (myStopped.count(veh) == 0) {
    //         WRITE_WARNINGF(TL("Vehicle '%' unloads container on edge '%', time=% without starting the stop."),
    //                        veh->getID(), veh->getEdge()->getID(), time2string(SIMSTEP));
    //     } else {
    //         myStopped.find(veh)->second.unloadedContainers += n;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn unloaded_containers(&mut self, veh: Ptr<SUMOVehicle>, n: i32) {
        todo!("port MSStopOut::unloadedContainers")
    }

    /// C++: `void MSStopOut::stopEnded(const SUMOVehicle *, const MSStop &, bool)`
    // void
    // MSStopOut::stopEnded(const SUMOVehicle* veh, const MSStop& stop, bool simEnd) {
    //     assert(veh != nullptr);
    //     if (myStopped.count(veh) == 0) {
    //         WRITE_WARNINGF(TL("Vehicle '%' ends stop on edge '%', time=% without entering the stop."),
    //                        veh->getID(), veh->getEdge()->getID(), time2string(SIMSTEP));
    //         return;
    //     }
    //     const SUMOVehicleParameter::Stop& pars = stop.pars;
    //     const StopInfo& si = myStopped.find(veh)->second;
    //     double delay = -1;
    //     double arrivalDelay = -1;
    //     if (pars.until >= 0 && !simEnd) {
    //         delay = STEPS2TIME(SIMSTEP - pars.until);
    //     }
    //     if (pars.arrival >= 0) {
    //         arrivalDelay = STEPS2TIME(pars.started - pars.arrival);
    //     }
    //     myDevice.openTag("stopinfo");
    //     myDevice.writeAttr(SUMO_ATTR_ID, veh->getID());
    //     myDevice.writeAttr(SUMO_ATTR_TYPE, veh->getVehicleType().getID());
    //     if (MSGlobals::gUseMesoSim) {
    //         myDevice.writeAttr(SUMO_ATTR_EDGE, veh->getEdge()->getID());
    //     } else {
    //         myDevice.writeAttr(SUMO_ATTR_LANE, stop.lane->getID());
    //     }
    //     myDevice.writeAttr(SUMO_ATTR_POSITION, veh->getPositionOnLane());
    //     myDevice.writeAttr(SUMO_ATTR_PARKING, pars.parking);
    //     myDevice.writeAttr(SUMO_ATTR_STARTED, time2string(pars.started));
    //     myDevice.writeAttr(SUMO_ATTR_ENDED, simEnd ? "-1" : time2string(SIMSTEP));
    //     if (pars.until >= 0) {
    //         myDevice.writeAttr("delay", delay);
    //     }
    //     if (pars.arrival >= 0) {
    //         myDevice.writeAttr(SUMO_ATTR_ARRIVALDELAY, arrivalDelay);
    //     }
    //     if (pars.busstop != "") {
    //         myDevice.writeAttr(SUMO_ATTR_BUS_STOP, pars.busstop);
    //     }
    //     if (pars.containerstop != "") {
    //         myDevice.writeAttr(SUMO_ATTR_CONTAINER_STOP, pars.containerstop);
    //     }
    //     if (pars.parkingarea != "") {
    //         myDevice.writeAttr(SUMO_ATTR_PARKING_AREA, pars.parkingarea);
    //     }
    //     if (pars.chargingStation != "") {
    //         myDevice.writeAttr(SUMO_ATTR_CHARGING_STATION, pars.chargingStation);
    //     }
    //     if (pars.overheadWireSegment != "") {
    //         myDevice.writeAttr(SUMO_ATTR_OVERHEAD_WIRE_SEGMENT, pars.overheadWireSegment);
    //     }
    //     if (pars.tripId != "") {
    //         myDevice.writeAttr(SUMO_ATTR_TRIP_ID, pars.tripId);
    //     }
    //     if (pars.line != "") {
    //         myDevice.writeAttr(SUMO_ATTR_LINE, pars.line);
    //     }
    //     if (pars.split != "") {
    //         myDevice.writeAttr(SUMO_ATTR_SPLIT, pars.split);
    //     }
    //     if (MSGlobals::gUseStopEnded) {
    //         myDevice.writeAttr(SUMO_ATTR_USED_ENDED, pars.ended >= 0);
    //     }
    //     myDevice.writeAttr("initialPersons", si.initialNumPersons);
    //     myDevice.writeAttr("loadedPersons", si.loadedPersons);
    //     myDevice.writeAttr("unloadedPersons", si.unloadedPersons);
    //     myDevice.writeAttr("initialContainers", si.initialNumContainers);
    //     myDevice.writeAttr("loadedContainers", si.loadedContainers);
    //     myDevice.writeAttr("unloadedContainers", si.unloadedContainers);
    //     myDevice.writeAttr("blockedDuration", time2string(si.blockTime));
    //
    //     if (pars.actType != "") {
    //         myDevice.writeAttr(SUMO_ATTR_ACTTYPE, pars.actType);
    //     }
    //     if (stop.pars.speed > 0) {
    //         if (stop.waypointWithStop) {
    //             myDevice.writeAttr(SUMO_ATTR_STATE, "waypointStopped");
    //         } else {
    //             myDevice.writeAttr(SUMO_ATTR_STATE, "waypoint");
    //         }
    //     } else if (stop.skipOnDemand) {
    //         myDevice.writeAttr(SUMO_ATTR_STATE, "skippedOnDemand");
    //     }
    //
    //     myDevice.closeTag();
    //     myStopped.erase(veh);
    // }
    #[allow(unused_variables)]
    pub fn stop_ended(&mut self, veh: Ptr<SUMOVehicle>, stop: &MSStop, simEnd: bool) {
        todo!("port MSStopOut::stopEnded")
    }

    /// C++: `void MSStopOut::generateOutputForUnfinished()`
    /// generate output for vehicles which are still stopped at simulation end
    // void
    // MSStopOut::generateOutputForUnfinished() {
    //     while (!myStopped.empty()) {
    //         const auto& item = *myStopped.begin();
    //         const SUMOVehicle* veh = item.first;
    //         assert(veh->isStopped());
    //         const MSStop& stop = veh->getNextStop();
    //         // erases item from myStopped
    //         stopEnded(veh, stop, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output_for_unfinished(&mut self) {
        todo!("port MSStopOut::generateOutputForUnfinished")
    }

    /// C++: `void MSStopOut::MSStopOut(const MSStopOut &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStopOut) -> Self {
        todo!("port MSStopOut::MSStopOut")
    }

    /// C++: `MSStopOut & MSStopOut::operator=(const MSStopOut &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStopOut) -> &mut MSStopOut {
        todo!("port MSStopOut::operator=")
    }

}

/// C++ class `MSStopOut::StopInfo` (microsim/output/MSStopOut.h)
pub struct MSStopOut_StopInfo {
    pub blockTime: SUMOTime,
    pub initialNumPersons: i32,
    pub loadedPersons: i32,
    pub unloadedPersons: i32,
    pub initialNumContainers: i32,
    pub loadedContainers: i32,
    pub unloadedContainers: i32,
}

impl MSStopOut_StopInfo {
    /// C++: `void StopInfo::StopInfo(SUMOTime, int, int)`
    // StopInfo(SUMOTime now, int numPersons, int numContainers) :
    //             blockTime(now),
    //             initialNumPersons(numPersons),
    //             loadedPersons(0),
    //             unloadedPersons(0),
    //             initialNumContainers(numContainers),
    //             loadedContainers(0),
    //             unloadedContainers(0) {
    //         }
    #[allow(unused_variables)]
    pub fn new(now: SUMOTime, numPersons: i32, numContainers: i32) -> Self {
        todo!("port StopInfo::StopInfo")
    }

}

