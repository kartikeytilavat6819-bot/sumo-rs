//! Port of microsim/devices/MSDevice_Routing.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device that performs vehicle rerouting based on current edge speeds
/// C++ class `MSDevice_Routing` (microsim/devices/MSDevice_Routing.h)
pub struct MSDevice_Routing {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// The period with which a vehicle shall be rerouted
    pub myPeriod: SUMOTime,
    /// The period with which a vehicle shall be rerouted before insertion
    pub myPreInsertionPeriod: SUMOTime,
    /// The last time a routing took place
    pub myLastRouting: SUMOTime,
    /// The time for which routing may be skipped because we cannot be inserted
    pub mySkipRouting: SUMOTime,
    /// The (optional) command responsible for rerouting
    pub myRerouteCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_Routing> */>,
    /// Whether the equipped vehicle may receive dispatch information at a rail signal
    pub myRerouteRailSignal: bool,
    /// the previous time that a vehicle entered a lane
    pub myLastLaneEntryTime: SUMOTime,
    /// Whether the equipped vehicle missed a reroute while stopping and should do so after the stop has ended
    pub myRerouteAfterStop: bool,
    pub myThresholdFactor: f64,
    pub myThresholdTime: f64,
    /// Whether the equipped vehicle may perform rerouting
    pub myActive: bool,
    /// Whether to gate periodic reroutes on a cheap route-drift check
    pub myInvalidation: bool,
    /// remaining-route cost at the last (re)routing, -1 if unknown
    pub myLastRouteCost: f64,
}

impl std::ops::Deref for MSDevice_Routing {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Routing {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_Routing {
    /// C++: `void MSDevice_Routing::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Routing-options
    // void
    // MSDevice_Routing::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("rerouting", "Routing", oc);
    //
    //     oc.doRegister("device.rerouting.period", new Option_String("0", "TIME"));
    //     oc.addSynonyme("device.rerouting.period", "device.routing.period", true);
    //     oc.addDescription("device.rerouting.period", "Routing", TL("The period with which the vehicle shall be rerouted"));
    //
    //     oc.doRegister("device.rerouting.pre-period", new Option_String("60", "TIME"));
    //     oc.addSynonyme("device.rerouting.pre-period", "device.routing.pre-period", true);
    //     oc.addDescription("device.rerouting.pre-period", "Routing", TL("The rerouting period before depart"));
    //
    //     oc.doRegister("device.rerouting.adaptation-weight", new Option_Float(0));
    //     oc.addSynonyme("device.rerouting.adaptation-weight", "device.routing.adaptation-weight", true);
    //     oc.addDescription("device.rerouting.adaptation-weight", "Routing", TL("The weight of prior edge weights for exponential moving average"));
    //
    //     oc.doRegister("device.rerouting.adaptation-steps", new Option_Integer(180));
    //     oc.addSynonyme("device.rerouting.adaptation-steps", "device.routing.adaptation-steps", true);
    //     oc.addDescription("device.rerouting.adaptation-steps", "Routing", TL("The number of steps for moving average weight of prior edge weights"));
    //
    //     oc.doRegister("device.rerouting.adaptation-interval", new Option_String("1", "TIME"));
    //     oc.addSynonyme("device.rerouting.adaptation-interval", "device.routing.adaptation-interval", true);
    //     oc.addDescription("device.rerouting.adaptation-interval", "Routing", TL("The interval for updating the edge weights"));
    //
    //     oc.doRegister("device.rerouting.cch-update-threshold.factor", new Option_Float(1));
    //     oc.addDescription("device.rerouting.cch-update-threshold.factor", "Routing", TL("Only propagate an arc weight into the CCH metric if it changed by more than the given factor"));
    //
    //     oc.doRegister("device.rerouting.cch-update-threshold.constant", new Option_String("0", "TIME"));
    //     oc.addDescription("device.rerouting.cch-update-threshold.constant", "Routing", TL("Only propagate an arc weight into the CCH metric if it changed by more than the given TIME"));
    //     oc.doRegister("device.rerouting.cch-ensemble", new Option_Integer(1));
    //     oc.addDescription("device.rerouting.cch-ensemble", "Routing", TL("Keep this many frozen weights.random-factor realizations (CCH metrics) per vehicle type and assign vehicles to them by a stable id hash"));
    //
    //     oc.doRegister("device.rerouting.threshold.factor", new Option_Float(1));
    //     oc.addDescription("device.rerouting.threshold.factor", "Routing", TL("Only reroute if the new route is faster than the current route by the given factor"));
    //
    //     oc.doRegister("device.rerouting.threshold.constant", new Option_String("0", "TIME"));
    //     oc.addDescription("device.rerouting.threshold.constant", "Routing", TL("Only reroute if the new route is faster than the current route by the given TIME"));
    //
    //     oc.doRegister("device.rerouting.invalidation", new Option_Bool(false));
    //     oc.addDescription("device.rerouting.invalidation", "Routing", TL("Only perform a periodic reroute when the current route's remaining cost has drifted beyond the threshold (cheap re-scoring gate before the full reroute)"));
    //
    //     oc.doRegister("device.rerouting.with-taz", new Option_Bool(false));
    //     oc.addSynonyme("device.rerouting.with-taz", "device.routing.with-taz", true);
    //     oc.addSynonyme("device.rerouting.with-taz", "with-taz");
    //     oc.addDescription("device.rerouting.with-taz", "Routing", TL("Use zones (districts) as routing start- and endpoints"));
    //
    //     oc.doRegister("device.rerouting.mode", new Option_String("0"));
    //     oc.addDescription("device.rerouting.mode", "Routing", TL("Set routing flags (8 ignores temporary blockages)"));
    //
    //     oc.doRegister("device.rerouting.init-with-loaded-weights", new Option_Bool(false));
    //     oc.addDescription("device.rerouting.init-with-loaded-weights", "Routing", TL("Use weight files given with option --weight-files for initializing edge weights"));
    //
    //     oc.doRegister("device.rerouting.threads", new Option_Integer(0));
    //     oc.addSynonyme("device.rerouting.threads", "routing-threads");
    //     oc.addDescription("device.rerouting.threads", "Routing", TL("The number of parallel execution threads used for rerouting"));
    //
    //     oc.doRegister("device.rerouting.synchronize", new Option_Bool(false));
    //     oc.addDescription("device.rerouting.synchronize", "Routing", TL("Let rerouting happen at the same time for all vehicles"));
    //
    //     oc.doRegister("device.rerouting.railsignal", new Option_Bool(false));
    //     oc.addDescription("device.rerouting.railsignal", "Routing", TL("Allow rerouting triggered by rail signals."));
    //
    //     oc.doRegister("device.rerouting.bike-speeds", new Option_Bool(false));
    //     oc.addDescription("device.rerouting.bike-speeds", "Routing", TL("Compute separate average speeds for bicycles"));
    //
    //     oc.doRegister("device.rerouting.output", new Option_FileName());
    //     oc.addDescription("device.rerouting.output", "Routing", TL("Save adapting weights to FILE"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_Routing::insertOptions")
    }

    /// C++: `bool MSDevice_Routing::checkOptions(OptionsCont &)` [static]
    /// checks MSDevice_Routing-options
    // bool
    // MSDevice_Routing::checkOptions(OptionsCont& oc) {
    //     bool ok = true;
    //     if (!oc.isDefault("device.rerouting.adaptation-steps") && !oc.isDefault("device.rerouting.adaptation-weight")) {
    //         WRITE_ERROR(TL("Only one of the options 'device.rerouting.adaptation-steps' or 'device.rerouting.adaptation-weight' may be given."));
    //         ok = false;
    //     }
    //     if (oc.getFloat("weights.random-factor") < 1) {
    //         WRITE_ERROR(TL("weights.random-factor cannot be less than 1"));
    //         ok = false;
    //     }
    //     if (string2time(oc.getString("device.rerouting.adaptation-interval")) < 0) {
    //         WRITE_ERROR(TL("Negative value for device.rerouting.adaptation-interval!"));
    //         ok = false;
    //     }
    //     if (oc.getFloat("device.rerouting.adaptation-weight") < 0.  ||
    //             oc.getFloat("device.rerouting.adaptation-weight") > 1.) {
    //         WRITE_ERROR(TL("The value for device.rerouting.adaptation-weight must be between 0 and 1!"));
    //         ok = false;
    //     }
    // #ifndef HAVE_FOX
    //     if (oc.getInt("device.rerouting.threads") > 1) {
    //         WRITE_ERROR(TL("Parallel routing is only possible when compiled with Fox."));
    //         ok = false;
    //     }
    // #endif
    //     if (oc.getInt("threads") > 1 && oc.getInt("device.rerouting.threads") > 1 && oc.getInt("threads") != oc.getInt("device.rerouting.threads")) {
    //         WRITE_WARNING(TL("Adapting number of routing threads to number of simulation threads."));
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        todo!("port MSDevice_Routing::checkOptions")
    }

    /// C++: `void MSDevice_Routing::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_Routing::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const bool equip = equippedByDefaultAssignmentOptions(oc, "rerouting", v, false);
    //     if (v.getParameter().wasSet(VEHPARS_FORCE_REROUTE) || equip) {
    //         // route computation is enabled
    //         // for implicitly equipped vehicles (trips, flows), option probability
    //         // can still be used to disable periodic rerouting after insertion for
    //         // parts of the fleet
    //         const SUMOTime period = (equip || (
    //                                      oc.isDefault("device.rerouting.probability") &&
    //                                      v.getFloatParam("device.rerouting.probability") == oc.getFloat("device.rerouting.probability"))
    //                                  ? v.getTimeParam("device.rerouting.period") : 0);
    //         const SUMOTime prePeriod = MAX2((SUMOTime)0, v.getTimeParam("device.rerouting.pre-period"));
    //         MSRoutingEngine::initWeightUpdate();
    //         // build the device
    //         into.push_back(new MSDevice_Routing(v, "routing_" + v.getID(), period, prePeriod));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_Routing::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Routing::~MSDevice_Routing() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Routing::~MSDevice_Routing() {
    //     // make the rerouting command invalid if there is one
    //     if (myRerouteCommand != nullptr) {
    //         myRerouteCommand->deschedule();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Routing::~MSDevice_Routing")
    }

    /// C++: `bool MSDevice_Routing::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Computes a new route on vehicle insertion
    // bool
    // MSDevice_Routing::notifyEnter(SUMOTrafficObject& /*veh*/, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    //     if (reason == MSMoveReminder::NOTIFICATION_DEPARTED) {
    //         if (myRerouteCommand == nullptr && myPreInsertionPeriod > 0 && myHolder.getDepartDelay() > myPreInsertionPeriod) {
    //             // pre-insertion rerouting was disabled. Reroute from the departure edge if insertion was delayed
    //             // this is happening in the run thread (not inbeginOfTimestepEvents) so we cannot safely use the threadPool
    //             myHolder.reroute(MSNet::getInstance()->getCurrentTimeStep(), "device.rerouting",
    //                              MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass()),
    //                              true, MSRoutingEngine::withTaz(), false);
    //         }
    //         // build repetition trigger if routing shall be done more often
    //         rebuildRerouteCommand(SIMSTEP + myPeriod);
    //     }
    //     if (MSGlobals::gWeightsSeparateTurns > 0 && enteredLane != nullptr) {
    //         if (reason == MSMoveReminder::NOTIFICATION_JUNCTION) {
    //             const SUMOTime t = SIMSTEP;
    //             if (myLastLaneEntryTime >= 0 && enteredLane->isInternal()) {
    //                 // record travel time on the previous edge but store on the internal ledge
    //                 MSRoutingEngine::addEdgeTravelTime(enteredLane->getEdge(), t - myLastLaneEntryTime);
    //             }
    //             myLastLaneEntryTime = t;
    //         }
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Routing::notifyEnter")
    }

    /// C++: `void MSDevice_Routing::notifyStopEnded()` [virtual]
    /// called to do the rerouting we missed whuile stopping
    // void
    // MSDevice_Routing::notifyStopEnded() {
    //     if (myRerouteAfterStop) {
    //         reroute(SIMSTEP);
    //         myRerouteAfterStop = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_stop_ended(&mut self) {
        todo!("port MSDevice_Routing::notifyStopEnded")
    }

    /// C++: `const std::string MSDevice_Routing::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "rerouting";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Routing::deviceName")
    }

    /// C++: `void MSDevice_Routing::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice_Routing::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     std::vector<std::string> internals;
    //     internals.push_back(toString(myPeriod));
    //     internals.push_back(toString(myLastRouting));
    //     out.writeAttr(SUMO_ATTR_STATE, toString(internals));
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        todo!("port MSDevice_Routing::saveState")
    }

    /// C++: `void MSDevice_Routing::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice_Routing::loadState(const SUMOSAXAttributes& attrs) {
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> myPeriod;
    //     bis >> myLastRouting;
    //     if (myHolder.hasDeparted()) {
    //         SUMOTime offset = myPeriod;
    //         if (myPeriod > 0) {
    //             offset = ((SIMSTEP - myHolder.getDeparture()) % myPeriod);
    //             if (offset != 0) {
    //                 offset = myPeriod - offset;
    //             }
    //         }
    //         rebuildRerouteCommand(SIMSTEP + offset);
    //     } else if (myRerouteCommand != nullptr && myLastRouting >= 0) {
    //         // the pre-insertion reroute scheduled by the constructor already happened before saving
    //         myRerouteCommand->deschedule();
    //         myRerouteCommand = nullptr;
    //         const MSEdge* source = *myHolder.getRoute().begin();
    //         if (myPreInsertionPeriod > 0 && (source->isTazConnector() || myHolder.getParameter().departLaneProcedure == DepartLaneDefinition::BEST_FREE)) {
    //             // repeated pre-insertion rerouting continues with its original phase
    //             myRerouteCommand = new WrappingCommand<MSDevice_Routing>(this, &MSDevice_Routing::preInsertionReroute);
    //             MSNet::getInstance()->getInsertionEvents()->addEvent(myRerouteCommand, myLastRouting + myPreInsertionPeriod);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port MSDevice_Routing::loadState")
    }

    /// C++: `void MSDevice_Routing::reroute(const SUMOTime, const bool)`
    /// initiate the rerouting, create router / thread pool on first use
    // void
    // MSDevice_Routing::reroute(const SUMOTime currentTime, const bool onInit) {
    //     MSRoutingEngine::initEdgeWeights(myHolder.getVClass());
    //     //check whether the weights did change since the last reroute
    //     if (myLastRouting > MSRoutingEngine::getLastAdaptation() || !myActive) {
    //         return;
    //     }
    //     // Invalidation gate: even when the metric changed, skip the (expensive)
    //     // reroute unless THIS vehicle's remaining route has actually degraded.
    //     // recomputeCosts is O(remaining-route length) array reads, far cheaper
    //     // than a shortest-path search. Only meaningful once the vehicle is on the
    //     // road with a concrete route (getCurrentRouteEdge is only valid then);
    //     // pre-insertion / onInit reroutes always run and don't track cost.
    //     const bool canGate = myInvalidation && !onInit && myHolder.isOnRoad();
    //     if (canGate && myLastRouteCost >= 0.) {
    //         MSVehicleRouter& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //         const ConstMSEdgeVector remaining(myHolder.getCurrentRouteEdge(), myHolder.getRoute().end());
    //         if (!remaining.empty()) {
    //             const double costNow = router.recomputeCosts(remaining, &myHolder, currentTime);
    //             if (costNow - myLastRouteCost <= myThresholdTime && costNow <= myLastRouteCost * myThresholdFactor) {
    //                 myLastRouting = currentTime;  // route still good enough; skip the search
    //                 myLastRouteCost = costNow;
    //                 return;
    //             }
    //         }
    //     }
    //     myLastRouting = currentTime;
    //     MSRoutingEngine::reroute(myHolder, currentTime, "device.rerouting", onInit);
    //     if (myInvalidation && myHolder.isOnRoad()) {
    //         MSVehicleRouter& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //         const ConstMSEdgeVector remaining(myHolder.getCurrentRouteEdge(), myHolder.getRoute().end());
    //         myLastRouteCost = remaining.empty() ? -1. : router.recomputeCosts(remaining, &myHolder, currentTime);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, currentTime: SUMOTime, onInit: bool) {
        todo!("port MSDevice_Routing::reroute")
    }

    /// C++: `void MSDevice_Routing::skipRouting(const SUMOTime)`
    /// Labels the current time step as "unroutable".
    // void skipRouting(const SUMOTime currentTime) {
    //         mySkipRouting = currentTime;
    //     }
    #[allow(unused_variables)]
    pub fn skip_routing(&mut self, currentTime: SUMOTime) {
        todo!("port MSDevice_Routing::skipRouting")
    }

    /// C++: `SUMOTime MSDevice_Routing::getPeriod() const` [const]
    // SUMOTime getPeriod() const {
    //         return myPeriod;
    //     }
    #[allow(unused_variables)]
    pub fn get_period(&self) -> SUMOTime {
        todo!("port MSDevice_Routing::getPeriod")
    }

    /// C++: `bool MSDevice_Routing::mayRerouteRailSignal() const` [const]
    /// return whether the equipped vehicle may receive dispatch information at a rail signal
    // bool mayRerouteRailSignal() const {
    //         return myRerouteRailSignal;
    //     }
    #[allow(unused_variables)]
    pub fn may_reroute_rail_signal(&self) -> bool {
        todo!("port MSDevice_Routing::mayRerouteRailSignal")
    }

    /// C++: `std::string MSDevice_Routing::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Routing::getParameter(const std::string& key) const {
    //     if (StringUtils::startsWith(key, "edge:")) {
    //         const std::string edgeID = key.substr(5);
    //         const MSEdge* edge = MSEdge::dictionary(edgeID);
    //         if (edge == nullptr) {
    //             throw InvalidArgument("Edge '" + edgeID + "' is invalid for parameter retrieval of '" + deviceName() + "'");
    //         }
    //         return toString(MSRoutingEngine::getEffort(edge, &myHolder, 0));
    //     } else if (key == "period") {
    //         return time2string(myPeriod);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_Routing::getParameter")
    }

    /// C++: `void MSDevice_Routing::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_Routing::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    //     if (StringUtils::startsWith(key, "edge:")) {
    //         const std::string edgeID = key.substr(5);
    //         const MSEdge* edge = MSEdge::dictionary(edgeID);
    //         if (edge == nullptr) {
    //             throw InvalidArgument("Edge '" + edgeID + "' is invalid for parameter setting of '" + deviceName() + "'");
    //         }
    //         MSRoutingEngine::setEdgeTravelTime(edge, doubleValue);
    //     } else if (key == "period") {
    //         myPeriod = TIME2STEPS(doubleValue);
    //         // re-schedule routing command
    //         rebuildRerouteCommand(SIMSTEP + myPeriod);
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice_Routing::setParameter")
    }

    /// C++: `void MSDevice_Routing::setActive(bool)`
    // void setActive(bool active) {
    //         myActive = active;
    //     }
    #[allow(unused_variables)]
    pub fn set_active(&mut self, active: bool) {
        todo!("port MSDevice_Routing::setActive")
    }

    /// C++: `bool MSDevice_Routing::sufficientSaving(double, double)`
    /// whether the change in saving is enough to trigger rerouting
    // bool
    // MSDevice_Routing::sufficientSaving(double oldCost, double newCost) {
    //     if (newCost == 0) {
    //         return true;
    //     }
    //     return (oldCost / newCost > myThresholdFactor) && (oldCost - newCost > myThresholdTime);
    // }
    #[allow(unused_variables)]
    pub fn sufficient_saving(&mut self, oldCost: f64, newCost: f64) -> bool {
        todo!("port MSDevice_Routing::sufficientSaving")
    }

    /// C++: `void MSDevice_Routing::MSDevice_Routing(SUMOVehicle &, const std::string &, SUMOTime, SUMOTime)` [private]
    /// Constructor
    // MSDevice_Routing::MSDevice_Routing(SUMOVehicle& holder, const std::string& id,
    //                                    SUMOTime period, SUMOTime preInsertionPeriod) :
    //     MSVehicleDevice(holder, id),
    //     myPeriod(period),
    //     myPreInsertionPeriod(preInsertionPeriod),
    //     myLastRouting(-1),
    //     mySkipRouting(-1),
    //     myRerouteCommand(nullptr),
    //     myRerouteRailSignal(holder.getBoolParam("device.rerouting.railsignal", true)),
    //     myLastLaneEntryTime(-1),
    //     myRerouteAfterStop(false),
    //     myThresholdFactor(holder.getFloatParam("device.rerouting.threshold.factor", true, 1)),
    //     myThresholdTime(STEPS2TIME(holder.getTimeParam("device.rerouting.threshold.constant", true, 0))),
    //     myActive(true),
    //     myInvalidation(OptionsCont::getOptions().getBool("device.rerouting.invalidation")),
    //     myLastRouteCost(-1.) {
    //     if (myPreInsertionPeriod > 0 || holder.getParameter().wasSet(VEHPARS_FORCE_REROUTE)) {
    //         // we do always a pre insertion reroute for trips to fill the best lanes of the vehicle with somehow meaningful values (especially for deaprtLane="best")
    //         myRerouteCommand = new WrappingCommand<MSDevice_Routing>(this, &MSDevice_Routing::preInsertionReroute);
    //         // if we don't update the edge weights, we might as well reroute now and hopefully use our threads better
    //         const SUMOTime execTime = MSRoutingEngine::hasEdgeUpdates() ? holder.getParameter().depart : -1;
    //         MSNet::getInstance()->getInsertionEvents()->addEvent(myRerouteCommand, execTime);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, period: SUMOTime, preInsertionPeriod: SUMOTime) -> Self {
        todo!("port MSDevice_Routing::MSDevice_Routing")
    }

    /// C++: `SUMOTime MSDevice_Routing::preInsertionReroute(const SUMOTime)` [private]
    /// Performs rerouting before insertion into the network
    // SUMOTime
    // MSDevice_Routing::preInsertionReroute(const SUMOTime currentTime) {
    //     if (mySkipRouting == currentTime) {
    //         return DELTA_T;
    //     }
    //     if (myPreInsertionPeriod == 0) {
    //         // the event will deschedule and destroy itself so it does not need to be stored
    //         myRerouteCommand = nullptr;
    //     }
    //     const MSEdge* source = *myHolder.getRoute().begin();
    //     const MSEdge* dest = myHolder.getRoute().getLastEdge();
    //     if (source->isTazConnector() && dest->isTazConnector()) {
    //         ConstMSRoutePtr cached = MSRoutingEngine::getCachedRoute(std::make_pair(source, dest));
    //         if (cached != nullptr && cached->size() > 2) {
    //             myHolder.replaceRoute(cached, "device.rerouting", true);
    //             return myPreInsertionPeriod;
    //         }
    //     }
    //     try {
    //         std::string msg;
    //         if (myHolder.hasValidRouteStart(msg)) {
    //             reroute(currentTime, true);
    //         }
    //     } catch (ProcessError&) {
    //         myRerouteCommand = nullptr;
    //         throw;
    //     }
    //     // avoid repeated pre-insertion rerouting when the departure edge is fix and
    //     // the departure lane does not depend on the route
    //     if (myPreInsertionPeriod > 0 && !source->isTazConnector() && myHolder.getParameter().departLaneProcedure != DepartLaneDefinition::BEST_FREE) {
    //         myRerouteCommand = nullptr;
    //         return 0;
    //     }
    //     return myPreInsertionPeriod;
    // }
    #[allow(unused_variables)]
    pub fn pre_insertion_reroute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_Routing::preInsertionReroute")
    }

    /// C++: `SUMOTime MSDevice_Routing::wrappedRerouteCommandExecute(SUMOTime)` [private]
    /// Performs rerouting after a period
    // SUMOTime
    // MSDevice_Routing::wrappedRerouteCommandExecute(SUMOTime currentTime) {
    //     if (myHolder.isStopped()) {
    //         myRerouteAfterStop = true;
    //     } else {
    //         reroute(currentTime);
    //     }
    //     return myPeriod;
    // }
    #[allow(unused_variables)]
    pub fn wrapped_reroute_command_execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_Routing::wrappedRerouteCommandExecute")
    }

    /// C++: `void MSDevice_Routing::rebuildRerouteCommand(SUMOTime)` [private]
    /// rebuild reroute command according to period
    // void
    // MSDevice_Routing::rebuildRerouteCommand(SUMOTime start) {
    //     if (myRerouteCommand != nullptr) {
    //         myRerouteCommand->deschedule();
    //         myRerouteCommand = nullptr;
    //     }
    //     if (myPeriod > 0) {
    //         myRerouteCommand = new WrappingCommand<MSDevice_Routing>(this, &MSDevice_Routing::wrappedRerouteCommandExecute);
    //         if (OptionsCont::getOptions().getBool("device.rerouting.synchronize")) {
    //             start -= start % myPeriod;
    //         }
    //         // ensure stable sorting of events (for repeatable routing with randomness)
    //         myRerouteCommand->priority = (int)myHolder.getNumericalID();
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myRerouteCommand, start);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn rebuild_reroute_command(&mut self, start: SUMOTime) {
        todo!("port MSDevice_Routing::rebuildRerouteCommand")
    }

    /// C++: `void MSDevice_Routing::MSDevice_Routing(const MSDevice_Routing &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Routing) -> Self {
        todo!("port MSDevice_Routing::MSDevice_Routing")
    }

    /// C++: `MSDevice_Routing & MSDevice_Routing::operator=(const MSDevice_Routing &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Routing) -> &mut MSDevice_Routing {
        todo!("port MSDevice_Routing::operator=")
    }

}

