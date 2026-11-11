//! Port of microsim/devices/MSDevice_StationFinder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSDevice_StationFinder::ChargeType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_StationFinder_ChargeType {
    CHARGETYPE_CHARGING = 0,
    CHARGETYPE_BIDIRECTIONAL = 1,
    CHARGETYPE_BATTERYEXCHANGE = 2,
    CHARGETYPE_FUEL = 3,
}

/// C++ enum `MSDevice_StationFinder::ChargingStrategy`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_StationFinder_ChargingStrategy {
    CHARGINGSTRATEGY_NONE = 0,
    CHARGINGSTRATEGY_BALANCED = 1,
    CHARGINGSTRATEGY_LATEST = 2,
}

/// C++ enum `MSDevice_StationFinder::RescueAction`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_StationFinder_RescueAction {
    RESCUEACTION_NONE = 0,
    RESCUEACTION_REMOVE = 1,
    RESCUEACTION_TOW = 2,
}

/// C++ enum `MSDevice_StationFinder::SearchState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_StationFinder_SearchState {
    SEARCHSTATE_NONE = 0,
    SEARCHSTATE_SUCCESSFUL = 1,
    SEARCHSTATE_UNSUCCESSFUL = 2,
    SEARCHSTATE_CHARGING = 3,
    SEARCHSTATE_WAITING = 4,
    SEARCHSTATE_BROKEN_DOWN = 5,
}

/// A device which triggers rerouting to nearby charging stations
/// C++ class `MSDevice_StationFinder` (microsim/devices/MSDevice_StationFinder.h)
pub struct MSDevice_StationFinder {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// C++ base class `MSStoppingPlaceRerouter`
    pub base2: MSStoppingPlaceRerouter,
    /// myHolder cast to needed type
    pub myVeh: Ptr<MSVehicle>,
    /// The corresponding battery device
    pub myBattery: Ptr<MSDevice_Battery>,
    /// To which station we are currently travelling
    pub myChargingStation: Ptr<MSStoppingPlace>,
    /// The command responsible for rescue actions
    pub myRescueCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_StationFinder> */>,
    /// The command responsible for limiting the charging rate (~ implement charging strategies)
    pub myChargeLimitCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_StationFinder> */>,
    /// The next charging rates to set via myChargingRateCommand
    pub myChargeLimits: Vec<(SUMOTime, f64)>,
    /// Last time the SoC was checked
    pub myLastChargeCheck: SUMOTime,
    /// Time interval after which the SoC has to be checked
    pub myCheckInterval: SUMOTime,
    /// Arrival time in the vicinity of the target charging station (to track the waiting time before accessing it)
    pub myArrivalAtChargingStation: SUMOTime,
    /// Last time charging stations have been searched
    pub myLastSearch: SUMOTime,
    /// Last time charging stations have been searched for opportunistic charging
    pub myLastOpportunisticSearch: SUMOTime,
    /// The time to wait for a rescue vehicle in case the battery is empty
    pub myRescueTime: f64,
    /// The safety buffer when calculating expected consumption
    pub myReserveFactor: f64,
    /// The state of charge threshold below which rescue mode is activated
    pub myEmptySoC: f64,
    /// The maximum euclidean distance between the vehicle and the charging station (-1 deactivates the condition)
    pub myMaxEuclideanDistance: f64,
    /// The max travel time to the next charging station
    pub myRadius: SUMOTime,
    /// Time interval to search again for a charging station if the first attempt failed
    pub myRepeatInterval: SUMOTime,
    /// Accepted waiting time at the charging station before a place becomes available
    pub myWaitForCharge: SUMOTime,
    /// Minimal expected stop duration to allow for opportunistic charging (not needed to complete the route)
    pub myMinOpportunisticTime: SUMOTime,
    /// SoC the last time the station finder algorithm was run completely
    pub myUpdateSoC: f64,
    /// The target state of charge where the vehicle stops charging
    pub myTargetSoC: f64,
    /// The state of charge at which the vehicle starts looking for charging stations
    pub mySearchSoC: f64,
    /// The state of charge at/below which the vehicle is interested in charging although it may still be sufficient to terminate its route
    pub myOpportunitySoC: f64,
    /// The share of stopping time a charging stop should take from the next regular (non-charging) stop under certain conditions
    pub myReplacePlannedStop: f64,
    /// The distance in meters to the original stop replaced by the charging stop (models charging close to the activity location) - used as well for opportunistic charging
    pub myDistanceToOriginalStop: f64,
    /// The type of charging permitted by the battery (charging, bidirectional, battery exchange)
    pub myChargeType: ChargeType,
    /// The chosen charging strategy
    pub myChargingStrategy: MSDevice_StationFinder_ChargingStrategy,
    /// What to do when the state of charge gets very low
    pub myRescueAction: MSDevice_StationFinder_RescueAction,
    /// The current state of the charging search (remember for decision logic)
    pub mySearchState: MSDevice_StationFinder_SearchState,
    /// Whether to skip searching charging stations if the battery charge is sufficient to complete the current route
    pub myCheckEnergyForRoute: bool,
}

impl std::ops::Deref for MSDevice_StationFinder {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_StationFinder {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_StationFinder {
    /// C++: `void MSDevice_StationFinder::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_StationFinder-options
    // void
    // MSDevice_StationFinder::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("stationfinder", "Battery", oc);
    //     oc.doRegister("device.stationfinder.rescueTime", new Option_String("1800", "TIME"));
    //     oc.addDescription("device.stationfinder.rescueTime", "Battery", TL("Time to wait for a rescue vehicle on the road side when the battery is empty"));
    //     oc.doRegister("device.stationfinder.rescueAction", new Option_String("remove"));
    //     oc.addDescription("device.stationfinder.rescueAction", "Battery", TL("How to deal with a vehicle which has to stop due to low battery: [none, remove, tow]"));
    //     oc.doRegister("device.stationfinder.reserveFactor", new Option_Float(1.1));
    //     oc.addDescription("device.stationfinder.reserveFactor", "Battery", TL("Scale battery need with this factor to account for unexpected traffic situations"));
    //     oc.doRegister("device.stationfinder.emptyThreshold", new Option_Float(0.05));
    //     oc.addDescription("device.stationfinder.emptyThreshold", "Battery", TL("Battery percentage to go into rescue mode"));
    //     oc.doRegister("device.stationfinder.radius", new Option_String("180", "TIME"));
    //     oc.addDescription("device.stationfinder.radius", "Battery", TL("Search radius in travel time seconds"));
    //     oc.doRegister("device.stationfinder.maxEuclideanDistance", new Option_Float(-1));
    //     oc.addDescription("device.stationfinder.maxEuclideanDistance", "Battery", TL("Euclidean search distance in meters (a negative value disables the restriction)"));
    //     oc.doRegister("device.stationfinder.repeat", new Option_String("60", "TIME"));
    //     oc.addDescription("device.stationfinder.repeat", "Battery", TL("When to trigger a new search if no station has been found"));
    //     oc.doRegister("device.stationfinder.chargeType", new Option_String("charging"));
    //     oc.addDescription("device.stationfinder.chargeType", "Battery", TL("Type of energy transfer"));
    //     oc.doRegister("device.stationfinder.waitForCharge", new Option_String("600", "TIME"));
    //     oc.addDescription("device.stationfinder.waitForCharge", "Battery", TL("After this waiting time vehicle searches for a new station when the initial one is blocked"));
    //     oc.doRegister("device.stationfinder.minOpportunityDuration", new Option_String("3600", "TIME"));
    //     oc.addDescription("device.stationfinder.minOpportunityDuration", "Battery", TL("Only stops with a predicted duration of at least the given threshold are considered for opportunistic charging."));
    //     oc.doRegister("device.stationfinder.saturatedChargeLevel", new Option_Float(0.8));
    //     oc.addDescription("device.stationfinder.saturatedChargeLevel", "Battery", TL("Target state of charge after which the vehicle stops charging"));
    //     oc.doRegister("device.stationfinder.needToChargeLevel", new Option_Float(0.4));
    //     oc.addDescription("device.stationfinder.needToChargeLevel", "Battery", TL("State of charge the vehicle begins searching for charging stations"));
    //     oc.doRegister("device.stationfinder.opportunisticChargeLevel", new Option_Float(0.));
    //     oc.addDescription("device.stationfinder.opportunisticChargeLevel", "Battery", TL("State of charge below which the vehicle may look for charging opportunities along its planned stops"));
    //     oc.doRegister("device.stationfinder.replacePlannedStop", new Option_Float(0.));
    //     oc.addDescription("device.stationfinder.replacePlannedStop", "Battery", TL("Share of stopping time of the next independently planned stop to use for charging instead"));
    //     oc.doRegister("device.stationfinder.maxDistanceToReplacedStop", new Option_Float(300.));
    //     oc.addDescription("device.stationfinder.maxDistanceToReplacedStop", "Battery", TL("Maximum distance in meters from the original stop to be replaced by the charging stop"));
    //     oc.doRegister("device.stationfinder.chargingStrategy", new Option_String("none"));
    //     oc.addDescription("device.stationfinder.chargingStrategy", "Battery", TL("Set a charging strategy to alter time and charging load from the set: [none, balanced, latest]"));
    //     oc.doRegister("device.stationfinder.checkEnergyForRoute", new Option_Bool(true));
    //     oc.addDescription("device.stationfinder.checkEnergyForRoute", "Battery", TL("Only search for charging stations if the battery charge is not estimated sufficient to complete the current route"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_StationFinder::insertOptions")
    }

    /// C++: `void MSDevice_StationFinder::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_StationFinder::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "stationfinder", v, false)) {
    //         into.push_back(new MSDevice_StationFinder(v));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_StationFinder::buildVehicleDevices")
    }

    /// C++: `void MSDevice_StationFinder::MSDevice_StationFinder(SUMOVehicle &)`
    /// Constructor
    // MSDevice_StationFinder::MSDevice_StationFinder(SUMOVehicle& holder)
    //     : MSVehicleDevice(holder, "stationfinder_" + holder.getID()),
    //       MSStoppingPlaceRerouter("device.stationfinder.charging", true, {
    //     {"waitingTime", 1.}, {"chargingTime", 1.}
    // }, { {"waitingTime", false}, {"chargingTime", false} }),
    // myVeh(dynamic_cast<MSVehicle&>(holder)),
    // myBattery(nullptr), myChargingStation(nullptr), myRescueCommand(nullptr), myChargeLimitCommand(nullptr),
    // myLastChargeCheck(0), myCheckInterval(1000), myArrivalAtChargingStation(-1), myLastSearch(-1),
    // myLastOpportunisticSearch(-1) {
    //     // consider whole path to/from a charging station in the search
    //     myEvalParams["distanceto"] = 0.;
    //     myEvalParams["timeto"] = 1.;
    //     myEvalParams["timefrom"] = 1.;
    //     myNormParams["chargingTime"] = true;
    //     myNormParams["waitingTime"] = true;
    //     myRescueTime = STEPS2TIME(holder.getTimeParam("device.stationfinder.rescueTime"));
    //     const std::string chargingStrategy = holder.getStringParam("device.stationfinder.chargingStrategy");
    //     if (chargingStrategy == "balanced") {
    //         myChargingStrategy = CHARGINGSTRATEGY_BALANCED;
    //     } else if (chargingStrategy == "latest") {
    //         myChargingStrategy = CHARGINGSTRATEGY_LATEST;
    //     } else if (chargingStrategy == "none") {
    //         myChargingStrategy = CHARGINGSTRATEGY_NONE;
    //     } else {
    //         WRITE_ERRORF(TL("Invalid device.stationfinder.chargingStrategy '%'."), chargingStrategy);
    //     }
    //     const std::string rescueAction = holder.getStringParam("device.stationfinder.rescueAction");
    //     if (rescueAction == "remove") {
    //         myRescueAction = RESCUEACTION_REMOVE;
    //     }  else if (rescueAction == "tow") {
    //         myRescueAction = RESCUEACTION_TOW;
    //     } else if (rescueAction == "none") {
    //         myRescueAction = RESCUEACTION_NONE;
    //     } else {
    //         WRITE_ERRORF(TL("Invalid device.stationfinder.rescueAction '%'."), rescueAction);
    //     }
    //     initRescueCommand();
    //     myReserveFactor = MAX2(1., holder.getFloatParam("device.stationfinder.reserveFactor"));
    //     myEmptySoC = MAX2(0., MIN2(holder.getFloatParam("device.stationfinder.emptyThreshold"), 1.));
    //     myRadius = holder.getTimeParam("device.stationfinder.radius");
    //     myMaxEuclideanDistance = holder.getFloatParam("device.stationfinder.maxEuclideanDistance");
    //     myRepeatInterval = holder.getTimeParam("device.stationfinder.repeat");
    //     myChargeType = CHARGETYPE_CHARGING;
    //
    //     myWaitForCharge = holder.getTimeParam("device.stationfinder.waitForCharge");
    //     myMinOpportunisticTime = holder.getTimeParam("device.stationfinder.minOpportunityDuration");
    //     myTargetSoC = MAX2(0., MIN2(holder.getFloatParam("device.stationfinder.saturatedChargeLevel"), 1.));
    //     mySearchSoC = MAX2(0., MIN2(holder.getFloatParam("device.stationfinder.needToChargeLevel"), 1.));
    //     if (mySearchSoC <= myEmptySoC) {
    //         WRITE_WARNINGF(TL("Vehicle '%' searches for charging stations only in the rescue case due to search threshold % <= rescue threshold %."), myHolder.getID(), mySearchSoC, myEmptySoC);
    //     }
    //     myOpportunitySoC = MAX2(0., MIN2(holder.getFloatParam("device.stationfinder.opportunisticChargeLevel"), 1.));
    //     if (myOpportunitySoC > 0. && myOpportunitySoC < MIN2(mySearchSoC + NUMERICAL_EPS, 1.)) {
    //         myOpportunitySoC = 0.;
    //         WRITE_WARNINGF(TL("Vehicle '%' won't do opportunistic charging as the threshold % is too close to the regular one %."), myHolder.getID(), myOpportunitySoC, mySearchSoC);
    //     }
    //     myReplacePlannedStop = MAX2(0., holder.getFloatParam("device.stationfinder.replacePlannedStop"));
    //     myDistanceToOriginalStop = holder.getFloatParam("device.stationfinder.maxDistanceToReplacedStop");
    //     myUpdateSoC = 2; // check once at the beginning
    //     myCheckEnergyForRoute = holder.getBoolParam("device.stationfinder.checkEnergyForRoute");
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle) -> Self {
        todo!("port MSDevice_StationFinder::MSDevice_StationFinder")
    }

    /// C++: `void MSDevice_StationFinder::~MSDevice_StationFinder() noexcept` [virtual]
    /// Destructor.
    // MSDevice_StationFinder::~MSDevice_StationFinder() {
    //     // make the rescue command invalid if there is one
    //     if (myRescueCommand != nullptr) {
    //         myRescueCommand->deschedule();
    //     }
    //     if (myChargeLimitCommand != nullptr) {
    //         myChargeLimitCommand->deschedule();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_StationFinder::~MSDevice_StationFinder")
    }

    /// C++: `bool MSDevice_StationFinder::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Computes current emission values and adds them to their sums
    // bool
    // MSDevice_StationFinder::notifyMove(SUMOTrafficObject& veh, double /*oldPos*/, double /*newPos*/, double /*newSpeed*/) {
    //     if (myBattery->getEnergyCharged() > 0. && myChargingStation != nullptr) {
    //         myArrivalAtChargingStation = -1;
    //         myChargingStation = nullptr;
    //         mySearchState = SEARCHSTATE_CHARGING;
    //         return true;
    //     } else if (mySearchState == SEARCHSTATE_CHARGING) {
    //         if (myBattery->getChargingStation() == nullptr) {
    //             mySearchState = SEARCHSTATE_NONE;
    //         } else {
    //             return true;
    //         }
    //     }
    //     // check if the vehicle travels at most an edge length to the charging station after jump/teleport
    //     if (mySearchState == SEARCHSTATE_BROKEN_DOWN && myVeh.hasStops() && myVeh.getStop(0).chargingStation != nullptr && myVeh.getStop(0).chargingStation->getLane().getEdge().getID() == myVeh.getLane()->getEdge().getID()) {
    //         return true;
    //     }
    //     const SUMOTime now = SIMSTEP;
    //     if (myChargingStation != nullptr) {
    //         if (myArrivalAtChargingStation > 0 && now - myArrivalAtChargingStation > myWaitForCharge) {
    //             // waited for too long, try another charging station
    //             if (rerouteToChargingStation(true)) {
    //                 WRITE_MESSAGE(TLF("Rerouted vehicle '%' after waiting too long at the previous charging station at time=%.", veh.getID(), toString(SIMTIME)));
    //             }
    //         } else if (myArrivalAtChargingStation < 0 && myVeh.willStop() && myVeh.getDistanceToPosition(myChargingStation->getBeginLanePosition(), myVeh.getLane()) < DEFAULT_CHARGINGSTATION_VIEW_DIST) {
    //             // remember when the vehicle arrived close to the target charging station
    //             mySearchState = SEARCHSTATE_WAITING;
    //             myArrivalAtChargingStation = now;
    //         }
    //     }
    //     const double currentSoC = myBattery->getActualBatteryCapacity() / myBattery->getMaximumBatteryCapacity();
    //     if (currentSoC < myOpportunitySoC && currentSoC < myTargetSoC && mySearchState == SEARCHSTATE_NONE) {
    //         // battery SoC is low enough to allow opportunistic charging (charging whenever - wherever)
    //         planOpportunisticCharging();
    //         myLastOpportunisticSearch = now;
    //         return true;
    //     } else if (currentSoC > mySearchSoC || mySearchState == SEARCHSTATE_BROKEN_DOWN) {
    //         // battery SoC is too high to look for charging facilities or the vehicle is already in rescue mode
    //         return true;
    //     }
    //     // only check once per second
    //     if (now - myLastChargeCheck < 1000) {
    //         return true;
    //     } else if (myRescueAction != RESCUEACTION_NONE  && (currentSoC < myEmptySoC || currentSoC < NUMERICAL_EPS)) {
    //
    //         // vehicle has to stop at the end of the  because battery SoC is too low
    //         double brakeGap = myVeh.getCarFollowModel().brakeGap(myVeh.getSpeed());
    //         std::pair<const MSLane*, double> stopPos = myVeh.getLanePosAfterDist(brakeGap);
    //         if (stopPos.first != nullptr) {
    //             const MSLane* stopLane = (stopPos.first->isInternal()) ? stopPos.first->getNormalSuccessorLane() : stopPos.first;
    //             double endPos = stopPos.second;
    //             if (stopLane != stopPos.first) {
    //                 endPos = MIN2(POSITION_EPS, stopLane->getLength());
    //             }
    //             // remove possibly scheduled charging stop
    //             if (myVeh.hasStops() && myVeh.getStop(0).chargingStation != nullptr) {
    //                 myVeh.abortNextStop();
    //             }
    //
    //             // schedule the rescue stop
    //             SUMOVehicleParameter::Stop rescueStop;
    //             rescueStop.index = 0;
    //             rescueStop.edge = stopLane->getEdge().getID();
    //             rescueStop.lane = stopLane->getID();
    //             rescueStop.startPos = MAX2(endPos - 2 * myHolder.getVehicleType().getLength(), 0.);
    //             rescueStop.endPos = endPos;
    //             rescueStop.parametersSet |= STOP_START_SET | STOP_END_SET;
    //             WRITE_MESSAGEF(TL("Vehicle '%' wants to stop on lane % at pos % because of low battery charge % at time=%."), myHolder.getID(), rescueStop.lane, toString(rescueStop.endPos), toString(currentSoC), toString(SIMTIME));
    //
    //             if (myRescueAction == RESCUEACTION_REMOVE) {
    //                 // remove vehicle from network
    //                 rescueStop.until = SUMOTime_MAX;
    //                 rescueStop.breakDown = true;
    //                 std::string errorMsg = "Could not insert the rescue stop.";
    //                 if (!myVeh.insertStop(0, rescueStop, "stationfinder:rescue", false, errorMsg)) {
    //                     WRITE_ERROR(errorMsg);
    //                 }
    //                 mySearchState = SEARCHSTATE_BROKEN_DOWN;
    //                 return true;
    //             } else if (myRescueAction == RESCUEACTION_TOW) {
    //                 // wait next to the road and get teleported to a charging station
    //                 SUMOTime rescueTime = TIME2STEPS(myRescueTime);
    //                 rescueStop.duration = rescueTime;
    //                 rescueStop.parking = ParkingType::ONROAD;
    //                 rescueStop.jump = 0;
    //                 std::string errorMsg = "Could not insert the rescue stop.";
    //                 if (!myVeh.insertStop(0, rescueStop, "stationfinder:rescue", false, errorMsg)) {
    //                     WRITE_ERROR(errorMsg);
    //                 }
    //                 initRescueCommand();
    //                 MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myRescueCommand, SIMSTEP + rescueStop.duration - DELTA_T);
    //                 mySearchState = SEARCHSTATE_BROKEN_DOWN;
    //                 return true;
    //             }
    //         }
    //     } else if (myChargingStation == nullptr &&
    //                (currentSoC < myUpdateSoC || (mySearchState == SEARCHSTATE_UNSUCCESSFUL &&
    //                        now - myLastSearch >= myRepeatInterval && !myHolder.isStopped()))) {
    //         // check if a charging stop is already planned without the device, otherwise reroute inside this device
    //         if (!alreadyPlannedCharging() && now > myHolder.getDeparture()) {
    //             rerouteToChargingStation();
    //         }
    //         myUpdateSoC = currentSoC - MAX2(0.1 * currentSoC, 0.01);
    // #ifdef DEBUG_STATIONFINDER_REROUTE
    //         std::cout << SIMTIME << " " << myHolder.getID() << " currentSoC=" << currentSoC << " nextUpdateSoC=" << myUpdateSoC << "\n";
    // #endif
    //     }
    //     myLastChargeCheck = SIMSTEP;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_StationFinder::notifyMove")
    }

    /// C++: `bool MSDevice_StationFinder::notifyIdle(SUMOTrafficObject &)` [virtual]
    /// Computes idling emission values and adds them to the emission sums
    // bool
    // MSDevice_StationFinder::notifyIdle(SUMOTrafficObject& /*veh*/) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_idle(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        todo!("port MSDevice_StationFinder::notifyIdle")
    }

    /// C++: `void MSDevice_StationFinder::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice_StationFinder::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     std::vector<std::string> internals;
    //     internals.push_back(toString(myLastChargeCheck));
    //     internals.push_back(toString(myUpdateSoC));
    //     internals.push_back(toString(mySearchSoC));
    //     internals.push_back(toString(myTargetSoC));
    //     internals.push_back(toString(myWaitForCharge));
    //     internals.push_back(toString(myRepeatInterval));
    //     internals.push_back(toString(myRadius));
    //     internals.push_back(toString(myLastSearch));
    //     internals.push_back(toString(myReserveFactor));
    //     internals.push_back(toString(mySearchState));
    //     internals.push_back(toString(myArrivalAtChargingStation));
    //     internals.push_back((myChargingStation == nullptr) ? "NULL" : myChargingStation->getID());
    //     internals.push_back(toString(myChargeLimits.size()));
    //     for (auto chargeLimit : myChargeLimits) {
    //         internals.push_back(toString(chargeLimit.first));
    //         internals.push_back(toString(chargeLimit.second));
    //     }
    //     internals.push_back(toString(myOpportunitySoC));
    //     internals.push_back(toString(myMinOpportunisticTime));
    //     internals.push_back(toString(myCheckEnergyForRoute));
    //     out.writeAttr(SUMO_ATTR_STATE, toString(internals));
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        todo!("port MSDevice_StationFinder::saveState")
    }

    /// C++: `void MSDevice_StationFinder::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice_StationFinder::loadState(const SUMOSAXAttributes& attrs) {
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> myLastChargeCheck;
    //     bis >> myUpdateSoC;
    //     bis >> mySearchSoC;
    //     bis >> myTargetSoC;
    //     bis >> myWaitForCharge;
    //     bis >> myRepeatInterval;
    //     bis >> myRadius;
    //     bis >> myLastSearch;
    //     bis >> myReserveFactor;
    //     int searchState;
    //     bis >> searchState;
    //     mySearchState = (SearchState)searchState;
    //     bis >> myArrivalAtChargingStation;
    //     std::string csID;
    //     bis >> csID;
    //     if (csID != "NULL") {
    //         myChargingStation = dynamic_cast<MSChargingStation*>(MSNet::getInstance()->getStoppingPlace(csID, SUMO_TAG_CHARGING_STATION));
    //     }
    //     int chargeLimitCount = 0;
    //     bis >> chargeLimitCount;
    //     for (int i = 0; i < chargeLimitCount; ++i) {
    //         SUMOTime t = 0;
    //         double limit = 0.;
    //         bis >> t;
    //         bis >> limit;
    //         myChargeLimits.push_back({ t, limit });
    //     }
    //     bis >> myOpportunitySoC;
    //     bis >> myMinOpportunisticTime;
    //     bis >> myCheckEnergyForRoute;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port MSDevice_StationFinder::loadState")
    }

    /// C++: `const std::string MSDevice_StationFinder::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const override {
    //         return "stationfinder";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_StationFinder::deviceName")
    }

    /// C++: `const std::string MSDevice_StationFinder::getChargingStrategy() const` [const]
    /// return the string representation of the chosen charging strategy
    // const std::string getChargingStrategy() const {
    //         if (myChargingStrategy == CHARGINGSTRATEGY_NONE) {
    //             return "none";
    //         } else if (myChargingStrategy == CHARGINGSTRATEGY_BALANCED) {
    //             return "balanced";
    //         } else {
    //             return "latest";
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_charging_strategy(&self) -> String {
        todo!("port MSDevice_StationFinder::getChargingStrategy")
    }

    /// C++: `void MSDevice_StationFinder::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSDevice_StationFinder::generateOutput(OutputDevice* tripinfoOut) const {
    //     if (tripinfoOut != nullptr && myChargingStation != nullptr) {
    //         tripinfoOut->openTag("stationfinder");
    //         tripinfoOut->writeAttr("chargingStation", myChargingStation->getID());
    //         tripinfoOut->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        todo!("port MSDevice_StationFinder::generateOutput")
    }

    /// C++: `void MSDevice_StationFinder::setBattery(MSDevice_Battery *)`
    // void setBattery(MSDevice_Battery* battery) {
    //         myBattery = battery;
    //     }
    #[allow(unused_variables)]
    pub fn set_battery(&mut self, battery: Ptr<MSDevice_Battery>) {
        todo!("port MSDevice_StationFinder::setBattery")
    }

    /// C++: `std::string MSDevice_StationFinder::getParameter(const std::string &) const` [const, virtual]
    // std::string
    // MSDevice_StationFinder::getParameter(const std::string& key) const {
    //     if (key == "chargingStation") { // eventually abstract with enum
    //         return (myChargingStation == nullptr) ? "" : myChargingStation->getID();
    //     } else if (key == "batteryNeed") {
    //         return toString(estimateConsumption() * myReserveFactor);
    //     } else if (key == "needToChargeLevel") {
    //         return toString(mySearchSoC);
    //     } else if (key == "saturatedChargeLevel") {
    //         return toString(myTargetSoC);
    //     } else if (key == "waitForCharge") {
    //         return toString(myWaitForCharge);
    //     } else if (key == "repeat") {
    //         return toString(myRepeatInterval);
    //     } else if (key == "radius") {
    //         return toString(myRadius);
    //     } else if (key == "reserveFactor") {
    //         return toString(myReserveFactor);
    //     }
    //     throw InvalidArgument(TLF("Parameter '%' is not supported for device of type '%'", key, deviceName()));
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_StationFinder::getParameter")
    }

    /// C++: `void MSDevice_StationFinder::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_StationFinder::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument(TLF("Setting parameter '%' requires a number for device of type '%'", key, deviceName()));
    //     }
    //     if (key == "needToChargeLevel") {
    //         mySearchSoC = MAX2(0., MIN2(1., doubleValue));
    //     } else if (key == "saturatedChargeLevel") {
    //         myTargetSoC = MAX2(0., MIN2(1., doubleValue));
    //     } else if (key == "waitForCharge") {
    //         myWaitForCharge = TIME2STEPS(MAX2(0., doubleValue));
    //     } else if (key == "repeat") {
    //         myRepeatInterval = TIME2STEPS(MAX2(0., doubleValue));
    //     } else if (key == "radius") {
    //         myRadius = TIME2STEPS(MAX2(0., doubleValue));
    //     } else if (key == "reserveFactor") {
    //         myReserveFactor = MAX2(1., doubleValue);
    //     } else {
    //         throw InvalidArgument(TLF("Setting parameter '%' is not supported for device of type '%'", key, deviceName()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice_StationFinder::setParameter")
    }

    /// C++: `bool MSDevice_StationFinder::evaluateCustomComponents(SUMOVehicle &, double, bool, MSStoppingPlace *, double, double, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, StoppingPlaceParamMap_t &, ConstMSEdgeVector &, ConstMSEdgeVector &, StoppingPlaceParamMap_t &, StoppingPlaceParamMap_t &)` [virtual]
    /// Compute some custom target function components
    // bool
    // MSDevice_StationFinder::evaluateCustomComponents(SUMOVehicle& /* veh */, double /* brakeGap */, bool /* newDestination */,
    //         MSStoppingPlace* alternative, double /* occupancy */, double /* prob */,
    //         SUMOAbstractRouter<MSEdge, SUMOVehicle>& /* router */,
    //         StoppingPlaceParamMap_t& stoppingPlaceValues,
    //         ConstMSEdgeVector& /* newRoute */, ConstMSEdgeVector& /* stoppingPlaceApproach */,
    //         StoppingPlaceParamMap_t& /* maxValues */, StoppingPlaceParamMap_t& addInput) {
    //     // estimated waiting time and charging time
    //     MSChargingStation* cs = dynamic_cast<MSChargingStation*>(alternative);
    //     double parkingCapacity = (cs->getParkingArea() != nullptr) ? cs->getParkingArea()->getCapacity() : (cs->getEndLanePosition() - cs->getBeginLanePosition()) / myHolder.getVehicleType().getParameter().length;
    //     double freeParkingCapacity = freeSpaceAtChargingStation(cs);
    //     stoppingPlaceValues["waitingTime"] = (freeParkingCapacity < 1.) ? DEFAULT_AVG_WAITING_TIME / parkingCapacity : 0.;
    //     stoppingPlaceValues["chargingTime"] = STEPS2TIME(cs->getChargeDelay()) + addInput["expectedConsumption"] / cs->getChargingPower(false);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn evaluate_custom_components(&mut self, veh: &mut SUMOVehicle, brakeGap: f64, newDestination: bool, alternative: Ptr<MSStoppingPlace>, occupancy: f64, prob: f64, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, stoppingPlaceValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, newRoute: &mut ConstMSEdgeVector, stoppingPlaceApproach: &mut ConstMSEdgeVector, maxValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, addInput: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool {
        todo!("port MSDevice_StationFinder::evaluateCustomComponents")
    }

    /// C++: `bool MSDevice_StationFinder::validComponentValues(StoppingPlaceParamMap_t &)` [virtual]
    /// Whether the stopping place should be discarded due to its results from the component evaluation
    // bool
    // MSDevice_StationFinder::validComponentValues(StoppingPlaceParamMap_t& stoppingPlaceValues) {
    //     if (stoppingPlaceValues["timeto"] > STEPS2TIME(myRadius)) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn valid_component_values(&mut self, stoppingPlaceValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool {
        todo!("port MSDevice_StationFinder::validComponentValues")
    }

    /// C++: `bool MSDevice_StationFinder::useStoppingPlace(MSStoppingPlace *)` [virtual]
    /// Whether the stopping place should be included in the search (can be used to add an additional filter)
    // bool
    // MSDevice_StationFinder::useStoppingPlace(MSStoppingPlace* /* stoppingPlace */) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn use_stopping_place(&mut self, stoppingPlace: Ptr<MSStoppingPlace>) -> bool {
        todo!("port MSDevice_StationFinder::useStoppingPlace")
    }

    /// C++: `SUMOAbstractRouter<MSEdge, SUMOVehicle> & MSDevice_StationFinder::getRouter(SUMOVehicle &, const Prohibitions &)` [virtual]
    /// Provide the router to use (MSNet::getRouterTT or MSRoutingEngine)
    // SUMOAbstractRouter<MSEdge, SUMOVehicle>& MSDevice_StationFinder::getRouter(SUMOVehicle& veh, const Prohibitions& prohibited) {
    //     return MSRoutingEngine::getRouterTT(veh.getRNGIndex(), veh.getVClass(), prohibited);
    // }
    #[allow(unused_variables)]
    pub fn get_router(&mut self, veh: &mut SUMOVehicle, prohibited: &SUMOAbstractRouter_Prohibitions) -> &mut SUMOAbstractRouter<MSEdge, SUMOVehicle> {
        todo!("port MSDevice_StationFinder::getRouter")
    }

    /// C++: `double MSDevice_StationFinder::getStoppingPlaceOccupancy(MSStoppingPlace *, const SUMOVehicle *)` [virtual]
    /// Return the number of occupied places of the StoppingPlace
    // double
    // MSDevice_StationFinder::getStoppingPlaceOccupancy(MSStoppingPlace* stoppingPlace, const SUMOVehicle*) {
    //     MSChargingStation* cs = dynamic_cast<MSChargingStation*>(stoppingPlace);
    //     if (cs->getParkingArea() != nullptr) {
    //         return cs->getParkingArea()->getOccupancy();
    //     }
    //     return (cs->getEndLanePosition() - cs->getLastFreePos()) / (myHolder.getLength() + myHolder.getVehicleType().getMinGap());
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_occupancy(&mut self, stoppingPlace: Ptr<MSStoppingPlace>, veh: Ptr<SUMOVehicle>) -> f64 {
        todo!("port MSDevice_StationFinder::getStoppingPlaceOccupancy")
    }

    /// C++: `double MSDevice_StationFinder::getLastStepStoppingPlaceOccupancy(MSStoppingPlace *, const SUMOVehicle *)` [virtual]
    /// Return the number of occupied places of the StoppingPlace from the previous time step
    // double
    // MSDevice_StationFinder::getLastStepStoppingPlaceOccupancy(MSStoppingPlace* stoppingPlace, const SUMOVehicle*) {
    //     MSChargingStation* cs = dynamic_cast<MSChargingStation*>(stoppingPlace);
    //     if (cs->getParkingArea() != nullptr) {
    //         return cs->getParkingArea()->getLastStepOccupancy();
    //     }
    //     return (cs->getEndLanePosition() - cs->getLastFreePos()) / (myHolder.getLength() + myHolder.getVehicleType().getMinGap());
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_stopping_place_occupancy(&mut self, stoppingPlace: Ptr<MSStoppingPlace>, veh: Ptr<SUMOVehicle>) -> f64 {
        todo!("port MSDevice_StationFinder::getLastStepStoppingPlaceOccupancy")
    }

    /// C++: `double MSDevice_StationFinder::getStoppingPlaceCapacity(MSStoppingPlace *)` [virtual]
    /// Return the number of places the StoppingPlace provides
    // double
    // MSDevice_StationFinder::getStoppingPlaceCapacity(MSStoppingPlace* stoppingPlace) {
    //     MSChargingStation* cs = dynamic_cast<MSChargingStation*>(stoppingPlace);
    //     if (cs->getParkingArea() != nullptr) {
    //         return cs->getParkingArea()->getCapacity();
    //     }
    //     return (cs->getEndLanePosition() - cs->getBeginLanePosition()) / (myHolder.getLength() + myHolder.getVehicleType().getMinGap());
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_capacity(&mut self, stoppingPlace: Ptr<MSStoppingPlace>) -> f64 {
        todo!("port MSDevice_StationFinder::getStoppingPlaceCapacity")
    }

    /// C++: `void MSDevice_StationFinder::rememberBlockedStoppingPlace(SUMOVehicle &, const MSStoppingPlace *, bool)` [virtual]
    /// store the blocked stopping place in the vehicle
    // void
    // MSDevice_StationFinder::rememberBlockedStoppingPlace(SUMOVehicle& veh, const MSStoppingPlace* stoppingPlace, bool blocked) {
    //     veh.rememberBlockedChargingStation(stoppingPlace, blocked);
    // }
    #[allow(unused_variables)]
    pub fn remember_blocked_stopping_place(&mut self, veh: &mut SUMOVehicle, stoppingPlace: Ptr<MSStoppingPlace>, blocked: bool) {
        todo!("port MSDevice_StationFinder::rememberBlockedStoppingPlace")
    }

    /// C++: `void MSDevice_StationFinder::rememberStoppingPlaceScore(SUMOVehicle &, MSStoppingPlace *, const std::string &)` [virtual]
    /// store the stopping place score in the vehicle
    // void
    // MSDevice_StationFinder::rememberStoppingPlaceScore(SUMOVehicle& veh, MSStoppingPlace* place, const std::string& score) {
    //     veh.rememberChargingStationScore(place, score);
    // }
    #[allow(unused_variables)]
    pub fn remember_stopping_place_score(&mut self, veh: &mut SUMOVehicle, place: Ptr<MSStoppingPlace>, score: &str) {
        todo!("port MSDevice_StationFinder::rememberStoppingPlaceScore")
    }

    /// C++: `void MSDevice_StationFinder::resetStoppingPlaceScores(SUMOVehicle &)` [virtual]
    /// forget all stopping place score for this vehicle
    // void
    // MSDevice_StationFinder::resetStoppingPlaceScores(SUMOVehicle& veh) {
    //     veh.resetChargingStationScores();
    // }
    #[allow(unused_variables)]
    pub fn reset_stopping_place_scores(&mut self, veh: &mut SUMOVehicle) {
        todo!("port MSDevice_StationFinder::resetStoppingPlaceScores")
    }

    /// C++: `SUMOTime MSDevice_StationFinder::sawBlockedStoppingPlace(SUMOVehicle &, MSStoppingPlace *, bool)` [virtual]
    /// ask the vehicle when it has seen the stopping place
    // SUMOTime
    // MSDevice_StationFinder::sawBlockedStoppingPlace(SUMOVehicle& veh, MSStoppingPlace* place, bool local) {
    //     return veh.sawBlockedChargingStation(place, local);
    // }
    #[allow(unused_variables)]
    pub fn saw_blocked_stopping_place(&mut self, veh: &mut SUMOVehicle, place: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        todo!("port MSDevice_StationFinder::sawBlockedStoppingPlace")
    }

    /// C++: `int MSDevice_StationFinder::getNumberStoppingPlaceReroutes(SUMOVehicle &)` [virtual]
    /// ask how many times already the vehicle has been rerouted to another stopping place
    // int
    // MSDevice_StationFinder::getNumberStoppingPlaceReroutes(SUMOVehicle& /* veh */) {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_number_stopping_place_reroutes(&mut self, veh: &mut SUMOVehicle) -> i32 {
        todo!("port MSDevice_StationFinder::getNumberStoppingPlaceReroutes")
    }

    /// C++: `void MSDevice_StationFinder::setNumberStoppingPlaceReroutes(SUMOVehicle &, int)` [virtual]
    /// update the number of reroutes for the vehicle
    // void
    // MSDevice_StationFinder::setNumberStoppingPlaceReroutes(SUMOVehicle& /* veh */, int /* value */) {
    // }
    #[allow(unused_variables)]
    pub fn set_number_stopping_place_reroutes(&mut self, veh: &mut SUMOVehicle, value: i32) {
        todo!("port MSDevice_StationFinder::setNumberStoppingPlaceReroutes")
    }

    /// C++: `void MSDevice_StationFinder::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves, see MSMoveReminder::notifyMoveInternal()
    // void
    // MSDevice_StationFinder::notifyMoveInternal(const SUMOTrafficObject& /*veh*/,
    //         const double /* frontOnLane */,
    //         const double /* timeOnLane */,
    //         const double /* meanSpeedFrontOnLane */,
    //         const double /* meanSpeedVehicleOnLane */,
    //         const double /* travelledDistanceFrontOnLane */,
    //         const double /* travelledDistanceVehicleOnLane */,
    //         const double /* meanLengthOnLane */) {
    //
    //     // called by meso (see MSMeanData_Emissions::MSLaneMeanDataValues::notifyMoveInternal)
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        todo!("port MSDevice_StationFinder::notifyMoveInternal")
    }

    /// C++: `MSChargingStation * MSDevice_StationFinder::findChargingStation(SUMOAbstractRouter<MSEdge, SUMOVehicle> &, double, StoppingPlaceParamMap_t &, bool, bool, bool, bool)` [private]
    /// central search function for close charging stations
    // MSChargingStation*
    // MSDevice_StationFinder::findChargingStation(SUMOAbstractRouter<MSEdge, SUMOVehicle>& /*router*/, double expectedConsumption, StoppingPlaceParamMap_t& scores, bool constrainTT, bool skipVisited, bool skipOccupied, bool visible) {
    //     MSChargingStation* minStation = nullptr;
    //     std::vector<StoppingPlaceVisible> candidates;
    //     const StoppingPlaceMemory* chargingMemory = myVeh.getChargingMemory();
    //     if (chargingMemory == nullptr) {
    //         skipVisited = false;
    //     }
    //     const SUMOTime stoppingPlaceMemory = TIME2STEPS(getWeight(myHolder, "memory", 600));
    //     for (const auto& stop : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_CHARGING_STATION)) {
    //         MSChargingStation* cs = static_cast<MSChargingStation*>(stop.second);
    //         if (cs->getEfficency() < NUMERICAL_EPS || cs->getChargingPower(false) < NUMERICAL_EPS) {
    //             continue;
    //         }
    //         if (cs->getChargeType() != myBattery->getChargeType()) {
    //             continue;
    //         }
    //         if (cs->getParkingArea() != nullptr && !cs->getParkingArea()->accepts(&myVeh)) {
    //             // skip stations where the linked parking area does not grant access to the device holder
    //             continue;
    //         }
    //         if (skipOccupied && freeSpaceAtChargingStation(cs) < 1.) {
    //             continue;
    //         }
    //         if (skipVisited && chargingMemory->sawBlockedStoppingPlace(cs, false) > 0 && SIMSTEP - chargingMemory->sawBlockedStoppingPlace(cs, false) < stoppingPlaceMemory) {
    //             // skip recently visited
    //             continue;
    //         }
    //         if (constrainTT && myMaxEuclideanDistance > 0 && stop.second->getLane().geometryPositionAtOffset(stop.second->getBeginLanePosition()).distanceTo2D(myHolder.getPosition()) > myMaxEuclideanDistance) {
    //             // skip probably too distant charging stations
    //             continue;
    //         }
    //         if (visible && myHolder.getEdge()->getID() != stop.second->getLane().getEdge().getID()) {
    //             continue;
    //         }
    //         candidates.push_back({cs, false});
    //     }
    //     ConstMSEdgeVector newRoute;
    //     scores["expectedConsumption"] = expectedConsumption;
    //     std::vector<double> probs(candidates.size(), 1.);
    //     bool newDestination;
    //     myCheckValidity = constrainTT;
    //     MSStoppingPlace* bestCandidate = rerouteStoppingPlace(nullptr, candidates, probs, myHolder, newDestination, newRoute, scores);
    //     myCheckValidity = true;
    //     minStation = dynamic_cast<MSChargingStation*>(bestCandidate);
    //     return minStation;
    // }
    #[allow(unused_variables)]
    pub fn find_charging_station(&mut self, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, expectedConsumption: f64, scores: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, constrainTT: bool, skipVisited: bool, skipOccupied: bool, visible: bool) -> Ptr<MSChargingStation> {
        todo!("port MSDevice_StationFinder::findChargingStation")
    }

    /// C++: `bool MSDevice_StationFinder::rerouteToChargingStation(bool)` [private]
    /// reroute to a charging station
    // bool
    // MSDevice_StationFinder::rerouteToChargingStation(bool replace) {
    //     double expectedConsumption = (myCheckEnergyForRoute) ? MIN2(estimateConsumption() * myReserveFactor, myBattery->getMaximumBatteryCapacity() * myTargetSoC) :
    //                                  myBattery->getMaximumBatteryCapacity() * MAX2(myTargetSoC - myBattery->getActualBatteryCapacity() / myBattery->getMaximumBatteryCapacity(), 0.);
    // #ifdef DEBUG_STATIONFINDER_REROUTE
    //     std::cout << SIMTIME << " " << myHolder.getID() << " expectedConsumption=" << expectedConsumption << " reserve=" << (myEmptySoC * myBattery->getMaximumBatteryCapacity()) << " chargeLevel=" << myBattery->getActualBatteryCapacity() << "\n";
    // #endif
    //     if (!myCheckEnergyForRoute || myBattery->getActualBatteryCapacity() < expectedConsumption + myEmptySoC * myBattery->getMaximumBatteryCapacity()) {
    //         myLastSearch = SIMSTEP;
    //         MSVehicleRouter& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //         StoppingPlaceParamMap_t scores = {};
    //         MSChargingStation* cs = findChargingStation(router, expectedConsumption, scores);
    //         if (cs != nullptr) {
    //             // integrate previously planned stops which do not have charging facilities
    //             myChargingStation = cs;
    //             SUMOVehicleParameter::Stop stopPar;
    //             stopPar.chargingStation = cs->getID();
    //             if (cs->getParkingArea() != nullptr) {
    //                 stopPar.parkingarea = cs->getParkingArea()->getID();
    //                 stopPar.parking = (cs->getParkingArea()->parkOnRoad()) ? ParkingType::ONROAD : ParkingType::OFFROAD;
    //             }
    //             stopPar.edge = cs->getLane().getEdge().getID();
    //             stopPar.lane = cs->getLane().getID();
    //             stopPar.duration = TIME2STEPS(expectedConsumption / (cs->getChargingPower(false) * cs->getEfficency()));
    //             stopPar.parametersSet = STOP_DURATION_SET;
    //             if (myReplacePlannedStop > 0) {
    //                 // "reuse" a previously planned stop (stop at charging station instead of a different stop)
    //                 // what if the charging station is skipped due to long waiting time?
    //                 if (myReplacePlannedStop > 0. && myHolder.hasStops() && myHolder.getNextStopParameter()->chargingStation.empty()) {
    //                     // compare the distance to the original target
    //                     if (scores["distfrom"] < myDistanceToOriginalStop /*actualDist < myDistanceToOriginalStop*/) {
    //                         // compute the arrival time at the original stop
    //                         const SUMOTime timeToOriginalStop = TIME2STEPS(scores["timefrom"]);
    //                         const SUMOTime originalUntil = myHolder.getNextStopParameter()->until;
    //                         if (timeToOriginalStop + myLastSearch < originalUntil) {
    //                             const SUMOTime delta = originalUntil - (timeToOriginalStop + myLastSearch);
    //                             stopPar.until = timeToOriginalStop + myLastSearch + (SUMOTime)((double)delta * MIN2(myReplacePlannedStop, 1.));
    //                             stopPar.parametersSet |= STOP_UNTIL_SET;
    //                             if (myReplacePlannedStop > 1.) {
    //                                 myHolder.abortNextStop();
    //                             }
    //                             // optionally implement a charging strategy by adjusting the accepted charging rates
    //                             if (myChargingStrategy != CHARGINGSTRATEGY_NONE) {
    //                                 // the charging strategy should actually only be computed at the arrival at the charging station
    //                                 implementChargingStrategy(myLastSearch + TIME2STEPS(scores["timeto"]), stopPar.until, expectedConsumption, cs);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //             stopPar.startPos = cs->getBeginLanePosition();
    //             stopPar.endPos = cs->getEndLanePosition();
    //             std::string errorMsg;
    // #ifdef DEBUG_STATIONFINDER_REROUTE
    //             std::ostringstream os;
    //             const ConstMSEdgeVector edgesBefore = myVeh.getRoute().getEdges();
    //             for (auto edge : edgesBefore) {
    //                 os << edge->getID() << " ";
    //             }
    //             std::cout << "MSDevice_StationFinder::rerouteToChargingStation: \n\tRoute before scheduling the charging station: " << os.str() << "\n";
    // #endif
    //             if ((replace && !myVeh.replaceStop(0, stopPar, "stationfinder:search", false, errorMsg)) || (!replace && !myVeh.insertStop(0, stopPar, "stationfinder:search", false, errorMsg))) {
    //                 WRITE_MESSAGE(TLF("Problem with inserting the charging station stop for vehicle %.", myHolder.getID()));
    //                 WRITE_ERROR(errorMsg);
    //             }
    //
    // #ifdef DEBUG_STATIONFINDER_REROUTE
    //             std::ostringstream os2;
    //             const ConstMSEdgeVector edgesAfter = myVeh.getRoute().getEdges();
    //             for (auto edge : edgesAfter) {
    //                 os2 << edge->getID() << " ";
    //             }
    //             std::cout << "\tRoute after scheduling the charging station: " << os2.str() << "\n";
    // #endif
    //             myArrivalAtChargingStation = -1;
    //             mySearchState = SEARCHSTATE_SUCCESSFUL;
    // #ifdef DEBUG_STATIONFINDER_REROUTE
    //             std::cout << "\tVehicle " << myHolder.getID() << " gets rerouted to charging station " << cs->getID()  << " on edge " << stopPar.edge  << " at time " << SIMTIME << "\n";
    // #endif
    //             return true;
    //         }
    //         mySearchState = SEARCHSTATE_UNSUCCESSFUL;
    //         WRITE_MESSAGEF(TL("Vehicle '%' wants to charge at time=% but does not find any charging station nearby."), myHolder.getID(), toString(SIMTIME));
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn reroute_to_charging_station(&mut self, replace: bool) -> bool {
        todo!("port MSDevice_StationFinder::rerouteToChargingStation")
    }

    /// C++: `bool MSDevice_StationFinder::planOpportunisticCharging()` [private]
    /// check which stop is suited for opportunistic charging and try to plan charging stops
    // bool
    // MSDevice_StationFinder::planOpportunisticCharging() {
    //     // check next stop
    //     double capacityDelta = MAX2(0., myTargetSoC * myBattery->getMaximumBatteryCapacity() - myBattery->getActualBatteryCapacity());
    //     if (myHolder.hasStops() && capacityDelta > 0.) {
    //         const MSStop& nextStop = myHolder.getNextStop();
    //         if (myHolder.isStopped() || nextStop.chargingStation != nullptr || myHolder.getCurrentRouteEdge() != nextStop.edge ||
    //                 nextStop.getMinDuration(SIMSTEP) < myMinOpportunisticTime) {
    //             return false;
    //         }
    //         // find charging station on same edge
    //         MSVehicleRouter& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //         StoppingPlaceParamMap_t scores = {};
    //         MSChargingStation* cs = findChargingStation(router, 0., scores, true, true, true, true);
    //         if (cs != nullptr) {
    //             // replace next stop by charging stop
    //             myChargingStation = cs;
    //             SUMOVehicleParameter::Stop stopPar;
    //             stopPar.chargingStation = cs->getID();
    //             if (cs->getParkingArea() != nullptr) {
    //                 stopPar.parkingarea = cs->getParkingArea()->getID();
    //                 stopPar.parking = (cs->getParkingArea()->parkOnRoad()) ? ParkingType::ONROAD : ParkingType::OFFROAD;
    //             }
    //             stopPar.edge = cs->getLane().getEdge().getID();
    //             stopPar.lane = cs->getLane().getID();
    //             stopPar.startPos = cs->getBeginLanePosition();
    //             stopPar.endPos = cs->getEndLanePosition();
    //             // copy over depart time from previously planned stop
    //             SUMOTime oldUntil = nextStop.getUntil();
    //             if (oldUntil > 0) {
    //                 stopPar.until = oldUntil;
    //                 stopPar.duration = 0;
    //             } else { // set a duration needed to charge to the target SoC
    //                 stopPar.duration = nextStop.duration;
    //             }
    //             std::string errorMsg;
    //             if (!myVeh.replaceStop(0, stopPar, "stationfinder:opportunisticSearch", false, errorMsg)) {
    //                 WRITE_ERROR(errorMsg);
    //             }
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn plan_opportunistic_charging(&mut self) -> bool {
        todo!("port MSDevice_StationFinder::planOpportunisticCharging")
    }

    /// C++: `SUMOTime MSDevice_StationFinder::teleportToChargingStation(const SUMOTime)` [private]
    /// search for a charging station and teleport the vehicle there as a rescue measure
    // SUMOTime
    // MSDevice_StationFinder::teleportToChargingStation(const SUMOTime /*currentTime*/) {
    //     // find closest charging station
    //     MSVehicleRouter& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //     double expectedConsumption = MIN2(estimateConsumption(nullptr, true, STEPS2TIME(myVeh.getStops().front().pars.duration)) * myReserveFactor, myBattery->getMaximumBatteryCapacity() * myTargetSoC);
    //     StoppingPlaceParamMap_t scores = {};
    //     MSChargingStation* cs = findChargingStation(router, expectedConsumption, scores, false, false, true);
    //     if (cs == nullptr) {
    //         // continue waiting if all charging stations are occupied
    // #ifdef DEBUG_STATIONFINDER_RESCUE
    //         std::cout << "MSDevice_StationFinder::teleportToChargingStation: No charging station available to teleport the broken-down vehicle " << myHolder.getID() << " to at time " << SIMTIME << ".\n.";
    // #endif
    //         // remove the vehicle if teleport to a charging station fails
    //         if (myHolder.isStopped()) {
    //             MSStop& currentStop = myHolder.getNextStopMutable();
    //             currentStop.duration += DELTA_T;
    //             SUMOVehicleParameter::Stop& stopPar = const_cast<SUMOVehicleParameter::Stop&>(currentStop.pars);
    //             stopPar.jump = -1;
    //             stopPar.breakDown = true;
    //             mySearchState = SEARCHSTATE_BROKEN_DOWN;
    //             WRITE_WARNINGF(TL("There is no charging station available to teleport the vehicle '%' to at time=%. Thus the vehicle will be removed."), myHolder.getID(), toString(SIMTIME));
    //         }
    // #ifdef DEBUG_STATIONFINDER_RESCUE
    //         else {
    // #ifdef DEBUG_STATIONFINDER_RESCUE
    //             std::cout << "MSDevice_StationFinder::teleportToChargingStation: Rescue stop of " << myHolder.getID() << " ended prematurely before regular end at " << SIMTIME << ".\n.";
    // #endif
    //         }
    // #endif
    //         return myRepeatInterval;
    //     }
    //
    //     // teleport to the charging station, stop there for charging
    //     myChargingStation = cs;
    //     SUMOVehicleParameter::Stop stopPar;
    //     stopPar.chargingStation = cs->getID();
    //     if (cs->getParkingArea() != nullptr) {
    //         stopPar.parkingarea = cs->getParkingArea()->getID();
    //         stopPar.parking = (cs->getParkingArea()->parkOnRoad()) ? ParkingType::ONROAD : ParkingType::OFFROAD;
    //     }
    //     stopPar.edge = cs->getLane().getEdge().getID();
    //     stopPar.lane = cs->getLane().getID();
    //     stopPar.startPos = cs->getBeginLanePosition();
    //     stopPar.endPos = cs->getEndLanePosition();
    //     stopPar.duration = TIME2STEPS(expectedConsumption / (cs->getChargingPower(false) * cs->getEfficency()));
    //     std::string errorMsg;
    //     if (!myVeh.insertStop(1, stopPar, "stationfinder:search", true, errorMsg)) {
    //         WRITE_ERROR(errorMsg);
    //     }
    //     myRescueCommand->deschedule();
    //     myRescueCommand = nullptr;
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn teleport_to_charging_station(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_StationFinder::teleportToChargingStation")
    }

    /// C++: `double MSDevice_StationFinder::estimateConsumption(const MSEdge *, const bool, const double) const` [const, private]
    /// estimate the energy needed for the planned route / up to a target edge
    // double
    // MSDevice_StationFinder::estimateConsumption(const MSEdge* target, const bool includeEmptySoC, const double stopDiscount) const {
    //     const SUMOTime now = SIMSTEP;
    //     MSVehicleRouter& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //     const ConstMSEdgeVector& route = myHolder.getRoute().getEdges();
    //     ConstMSEdgeVector::const_iterator targetIt = (target == nullptr) ? route.end() : std::find(route.begin(), route.end(), target) + 1;
    //     const ConstMSEdgeVector remainingRoute(route.begin() + myHolder.getRoutePosition(), targetIt);
    //     const double remainingTime = router.recomputeCosts(remainingRoute, &myHolder, now);
    //     if (now > myHolder.getDeparture()) {
    //         const double totalConsumption = myBattery->getTotalConsumption();
    //         double expectedConsumption = 0.;
    //         double passedTime = STEPS2TIME(now - myHolder.getDeparture());
    //         if (totalConsumption > 0. && passedTime - stopDiscount > DEFAULT_CONSUMPTION_ESTIMATE_HISTORY) {
    //             expectedConsumption = totalConsumption / (passedTime - stopDiscount) * remainingTime;
    //         } else {
    //             // fallback consumption rate for vehicles starting with low battery
    //             if (!myHolder.getVehicleType().getParameter().wasSet(VTYPEPARS_EMISSIONCLASS_SET)) {
    //                 WRITE_ERRORF("The stationfinder device needs emission parameters for range estimation but no emission class has been set for the vehicle '%'", myHolder.getID());
    //             }
    //             const double speed = MIN2(myHolder.getMaxSpeed(), myHolder.getLane()->getSpeedLimit());
    //             EnergyParams* const params = myHolder.getEmissionParameters();
    //             PollutantsInterface::EmissionType emType = myBattery->tracksFuel() ? PollutantsInterface::FUEL : PollutantsInterface::ELEC;
    //             expectedConsumption = PollutantsInterface::compute(myVeh.getVehicleType().getEmissionClass(), emType,
    //                                   speed * 0.8, 0., 0., params) * (remainingTime - passedTime);
    //         }
    //         if (includeEmptySoC) {
    //             expectedConsumption += MAX2(0., myEmptySoC * myBattery->getMaximumBatteryCapacity() - myBattery->getActualBatteryCapacity());
    //         }
    //         expectedConsumption /= myHolder.getEmissionParameters()->getDoubleOptional(SUMO_ATTR_PROPULSIONEFFICIENCY, 1.);
    //         return expectedConsumption;
    //     }
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn estimate_consumption(&self, target: Ptr<MSEdge>, includeEmptySoC: bool, stopDiscount: f64) -> f64 {
        todo!("port MSDevice_StationFinder::estimateConsumption")
    }

    /// C++: `double MSDevice_StationFinder::freeSpaceAtChargingStation(MSChargingStation *) const` [const, private]
    /// compute the free space at a charging station
    // double
    // MSDevice_StationFinder::freeSpaceAtChargingStation(MSChargingStation* cs) const {
    //     return (cs->getParkingArea() != nullptr) ? cs->getParkingArea()->getCapacity() - cs->getParkingArea()->getOccupancy() : (cs->getLastFreePos() - cs->getBeginLanePosition()) / myHolder.getVehicleType().getParameter().length;
    // }
    #[allow(unused_variables)]
    pub fn free_space_at_charging_station(&self, cs: Ptr<MSChargingStation>) -> f64 {
        todo!("port MSDevice_StationFinder::freeSpaceAtChargingStation")
    }

    /// C++: `bool MSDevice_StationFinder::alreadyPlannedCharging()` [private]
    /// adopt a planned charging stop outside of the device
    // bool
    // MSDevice_StationFinder::alreadyPlannedCharging() {
    //     if (myChargingStation == nullptr) {
    //         auto stops = myHolder.getStops();
    //         for (auto stop : stops) {
    //             if (stop.chargingStation != nullptr) {
    //                 // compare whether we'll make it there without intermediate charging
    //                 double expectedConsumption = estimateConsumption(*stop.edge);
    //                 if (myBattery->getActualBatteryCapacity() < expectedConsumption) {
    //                     myChargingStation = stop.chargingStation;
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn already_planned_charging(&mut self) -> bool {
        todo!("port MSDevice_StationFinder::alreadyPlannedCharging")
    }

    /// C++: `void MSDevice_StationFinder::initRescueCommand()` [private]
    /// create the event command for teleporting in case of brake-down
    // void
    // MSDevice_StationFinder::initRescueCommand() {
    //     if (myRescueAction == RESCUEACTION_TOW && myRescueCommand == nullptr) {
    //         myRescueCommand = new WrappingCommand<MSDevice_StationFinder>(this, &MSDevice_StationFinder::teleportToChargingStation);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_rescue_command(&mut self) {
        todo!("port MSDevice_StationFinder::initRescueCommand")
    }

    /// C++: `void MSDevice_StationFinder::initChargeLimitCommand()` [private]
    /// create the event command for changing charging rates
    // void
    // MSDevice_StationFinder::initChargeLimitCommand() {
    //     if (myChargingStrategy != CHARGINGSTRATEGY_NONE && myChargeLimitCommand == nullptr) {
    //         myChargeLimitCommand = new WrappingCommand<MSDevice_StationFinder>(this, &MSDevice_StationFinder::updateChargeLimit);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_charge_limit_command(&mut self) {
        todo!("port MSDevice_StationFinder::initChargeLimitCommand")
    }

    /// C++: `SUMOTime MSDevice_StationFinder::updateChargeLimit(const SUMOTime)` [private]
    /// update the maximum charge rate of the battery to simulate charging strategies
    // SUMOTime
    // MSDevice_StationFinder::updateChargeLimit(const SUMOTime currentTime) {
    //     if (myChargeLimits.size() > 0 && myChargeLimits.begin()->first < currentTime - DELTA_T) {
    //         myChargeLimits.clear();
    //     }
    //     if (myChargeLimits.size() > 0) {
    //         double chargeLimit = myChargeLimits.begin()->second;
    //         myBattery->setChargeLimit(chargeLimit);
    //         if (chargeLimit < 0) {
    //             WRITE_MESSAGEF(TL("The charging rate limit of vehicle '%' is lifted at time=%"), myHolder.getID(), STEPS2TIME(SIMSTEP));
    //         } else {
    //             WRITE_MESSAGEF(TL("The charging rate of vehicle '%' is limited to % at time=%"), myHolder.getID(), chargeLimit, STEPS2TIME(SIMSTEP));
    //         }
    //         myChargeLimits.erase(myChargeLimits.begin());
    //     }
    //     if (myChargeLimits.size() == 0) {
    //         myChargeLimitCommand->deschedule();
    //         myChargeLimitCommand = nullptr;
    //         return 0;
    //     } else {
    //         return myChargeLimits.begin()->first - currentTime;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_charge_limit(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_StationFinder::updateChargeLimit")
    }

    /// C++: `void MSDevice_StationFinder::implementChargingStrategy(SUMOTime, SUMOTime, const double, const MSChargingStation *)` [private]
    // void
    // MSDevice_StationFinder::implementChargingStrategy(SUMOTime begin, SUMOTime end, const double plannedCharge, const MSChargingStation* cs) {
    //     myChargeLimits.clear();
    //     if (myChargingStrategy == CHARGINGSTRATEGY_BALANCED) {
    //         const double balancedChargeRate = plannedCharge / STEPS2TIME(end - begin) * 3600.;
    //         myChargeLimits.push_back({ begin, balancedChargeRate });
    //         myChargeLimits.push_back({ end, -1});
    //     } else { // CHARGINGSTRATEGY_LATEST
    //         SUMOTime expectedDuration = myBattery->estimateChargingDuration(plannedCharge, cs->getChargingPower(false) * cs->getEfficency());
    //         if (end - expectedDuration > begin) {
    //             myChargeLimits.push_back({ begin, 0 });
    //             myChargeLimits.push_back({ end - expectedDuration, -1 });
    //         }
    //     }
    //     if (myChargeLimits.size() > 0) {
    //         initChargeLimitCommand();
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myChargeLimitCommand, begin);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn implement_charging_strategy(&mut self, begin: SUMOTime, end: SUMOTime, plannedCharge: f64, cs: Ptr<MSChargingStation>) {
        todo!("port MSDevice_StationFinder::implementChargingStrategy")
    }

    /// C++: `void MSDevice_StationFinder::MSDevice_StationFinder(const MSDevice_StationFinder &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_StationFinder) -> Self {
        todo!("port MSDevice_StationFinder::MSDevice_StationFinder")
    }

    /// C++: `MSDevice_StationFinder & MSDevice_StationFinder::operator=(const MSDevice_StationFinder &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_StationFinder) -> &mut MSDevice_StationFinder {
        todo!("port MSDevice_StationFinder::operator=")
    }

}

