//! Port of microsim/devices/MSDevice_Routing.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::options::options_cont::{with_options, OptionsCont};
use utils::options::option::SumoOption;
use utils::common::msg_handler::{write_error, write_warning, write_message};
use utils::common::time::{string2time, time2string, time2steps, steps2time, SUMOTIME_MIN};
use utils::common::to_string::to_string;
use crate::microsim::ms_event_control::Command;
use crate::utils::wrapping_command::WrappingCommand;

const INVALID_DOUBLE: f64 = f64::MAX;

/// A device that performs vehicle rerouting based on current edge speeds
/// C++ class `MSDevice_Routing` (microsim/devices/MSDevice_Routing.h)
#[repr(C)]
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
    pub myRerouteCommand: Ptr<MSDevice_Routing_WrappingCommand>,
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

/// C++ `WrappingCommand<MSDevice_Routing>` bound to `wrappedRerouteCommandExecute` (utils/common/WrappingCommand.h)
pub type MSDevice_Routing_WrappingCommand = WrappingCommand<MSDevice_Routing>;

fn ms_device_routing_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    let dev = Ptr::from_mut(mr).cast::<MSDevice_Routing>().get_mut().unwrap();
    dev.notify_enter(veh, reason, enteredLane)
}

static MS_DEVICE_ROUTING_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: ms_device_routing_notify_enter,
    notify_move: default_notify_move,
    notify_leave: default_notify_leave,
};

fn ms_device_routing_notify_stop_ended(mr: &mut MSMoveReminder) {
    Ptr::from_mut(mr).cast::<MSDevice_Routing>().m().notify_stop_ended()
}

static MS_DEVICE_ROUTING_MSMOVE_REMINDER_IDLE_VT: crate::microsim::ms_move_reminder::MSMoveReminderIdleVt = crate::microsim::ms_move_reminder::MSMoveReminderIdleVt {
    notify_idle: crate::microsim::ms_move_reminder::default_notify_idle,
    notify_parking: crate::microsim::ms_move_reminder::default_notify_parking,
    notify_stop_ended: ms_device_routing_notify_stop_ended,
};

impl Drop for MSDevice_Routing {
    /// C++: `MSDevice_Routing::~MSDevice_Routing()`: make the rerouting command invalid if there is one
    fn drop(&mut self) {
        if !self.myRerouteCommand.is_null() {
            self.myRerouteCommand.m().deschedule();
        }
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
        MSDevice::insert_default_assignment_options("rerouting", "Routing", oc, false);

        oc.do_register("device.rerouting.period", SumoOption::string_typed("0", "TIME"));
        oc.add_synonyme("device.rerouting.period", "device.routing.period", true);
        oc.add_description("device.rerouting.period", "Routing", "The period with which the vehicle shall be rerouted");

        oc.do_register("device.rerouting.pre-period", SumoOption::string_typed("60", "TIME"));
        oc.add_synonyme("device.rerouting.pre-period", "device.routing.pre-period", true);
        oc.add_description("device.rerouting.pre-period", "Routing", "The rerouting period before depart");

        oc.do_register("device.rerouting.adaptation-weight", SumoOption::float(0.0));
        oc.add_synonyme("device.rerouting.adaptation-weight", "device.routing.adaptation-weight", true);
        oc.add_description("device.rerouting.adaptation-weight", "Routing", "The weight of prior edge weights for exponential moving average");

        oc.do_register("device.rerouting.adaptation-steps", SumoOption::integer(180));
        oc.add_synonyme("device.rerouting.adaptation-steps", "device.routing.adaptation-steps", true);
        oc.add_description("device.rerouting.adaptation-steps", "Routing", "The number of steps for moving average weight of prior edge weights");

        oc.do_register("device.rerouting.adaptation-interval", SumoOption::string_typed("1", "TIME"));
        oc.add_synonyme("device.rerouting.adaptation-interval", "device.routing.adaptation-interval", true);
        oc.add_description("device.rerouting.adaptation-interval", "Routing", "The interval for updating the edge weights");

        oc.do_register("device.rerouting.cch-update-threshold.factor", SumoOption::float(1.0));
        oc.add_description("device.rerouting.cch-update-threshold.factor", "Routing", "Only propagate an arc weight into the CCH metric if it changed by more than the given factor");

        oc.do_register("device.rerouting.cch-update-threshold.constant", SumoOption::string_typed("0", "TIME"));
        oc.add_description("device.rerouting.cch-update-threshold.constant", "Routing", "Only propagate an arc weight into the CCH metric if it changed by more than the given TIME");
        oc.do_register("device.rerouting.cch-ensemble", SumoOption::integer(1));
        oc.add_description("device.rerouting.cch-ensemble", "Routing", "Keep this many frozen weights.random-factor realizations (CCH metrics) per vehicle type and assign vehicles to them by a stable id hash");

        oc.do_register("device.rerouting.threshold.factor", SumoOption::float(1.0));
        oc.add_description("device.rerouting.threshold.factor", "Routing", "Only reroute if the new route is faster than the current route by the given factor");

        oc.do_register("device.rerouting.threshold.constant", SumoOption::string_typed("0", "TIME"));
        oc.add_description("device.rerouting.threshold.constant", "Routing", "Only reroute if the new route is faster than the current route by the given TIME");

        oc.do_register("device.rerouting.invalidation", SumoOption::bool(false));
        oc.add_description("device.rerouting.invalidation", "Routing", "Only perform a periodic reroute when the current route's remaining cost has drifted beyond the threshold (cheap re-scoring gate before the full reroute)");

        oc.do_register("device.rerouting.with-taz", SumoOption::bool(false));
        oc.add_synonyme("device.rerouting.with-taz", "device.routing.with-taz", true);
        oc.add_synonyme("device.rerouting.with-taz", "with-taz", false);
        oc.add_description("device.rerouting.with-taz", "Routing", "Use zones (districts) as routing start- and endpoints");

        oc.do_register("device.rerouting.mode", SumoOption::string_with("0"));
        oc.add_description("device.rerouting.mode", "Routing", "Set routing flags (8 ignores temporary blockages)");

        oc.do_register("device.rerouting.init-with-loaded-weights", SumoOption::bool(false));
        oc.add_description("device.rerouting.init-with-loaded-weights", "Routing", "Use weight files given with option --weight-files for initializing edge weights");

        oc.do_register("device.rerouting.threads", SumoOption::integer(0));
        oc.add_synonyme("device.rerouting.threads", "routing-threads", false);
        oc.add_description("device.rerouting.threads", "Routing", "The number of parallel execution threads used for rerouting");

        oc.do_register("device.rerouting.synchronize", SumoOption::bool(false));
        oc.add_description("device.rerouting.synchronize", "Routing", "Let rerouting happen at the same time for all vehicles");

        oc.do_register("device.rerouting.railsignal", SumoOption::bool(false));
        oc.add_description("device.rerouting.railsignal", "Routing", "Allow rerouting triggered by rail signals.");

        oc.do_register("device.rerouting.bike-speeds", SumoOption::bool(false));
        oc.add_description("device.rerouting.bike-speeds", "Routing", "Compute separate average speeds for bicycles");

        oc.do_register("device.rerouting.output", SumoOption::file_name());
        oc.add_description("device.rerouting.output", "Routing", "Save adapting weights to FILE");
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
        let mut ok = true;
        if !oc.is_default("device.rerouting.adaptation-steps") && !oc.is_default("device.rerouting.adaptation-weight") {
            write_error("Only one of the options 'device.rerouting.adaptation-steps' or 'device.rerouting.adaptation-weight' may be given.");
            ok = false;
        }
        if oc.get_float("weights.random-factor").unwrap() < 1. {
            write_error("weights.random-factor cannot be less than 1");
            ok = false;
        }
        if string2time(&oc.get_string("device.rerouting.adaptation-interval").unwrap()).unwrap() < 0 {
            write_error("Negative value for device.rerouting.adaptation-interval!");
            ok = false;
        }
        if oc.get_float("device.rerouting.adaptation-weight").unwrap() < 0. || oc.get_float("device.rerouting.adaptation-weight").unwrap() > 1. {
            write_error("The value for device.rerouting.adaptation-weight must be between 0 and 1!");
            ok = false;
        }
        // shortcut: the "not HAVE_FOX" thread-count error is omitted, this port is always built with threads, upgrade if a non-threaded build is needed
        if oc.get_int("threads").unwrap() > 1 && oc.get_int("device.rerouting.threads").unwrap() > 1 && oc.get_int("threads").unwrap() != oc.get_int("device.rerouting.threads").unwrap() {
            write_warning("Adapting number of routing threads to number of simulation threads.");
        }
        ok
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
        let equip = with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(
                oc,
                "rerouting",
                &v.get_id(),
                &v.get_parameter().base,
                &v.get_v_type_parameter().base,
                false,
                false,
            )
        });
        if v.get_parameter().was_set(VEHPARS_FORCE_REROUTE) || equip {
            // route computation is enabled
            // for implicitly equipped vehicles (trips, flows), option probability
            // can still be used to disable periodic rerouting after insertion for
            // parts of the fleet
            let probability = with_options(|oc| oc.get_float("device.rerouting.probability").unwrap());
            let probability_is_default = with_options(|oc| oc.is_default("device.rerouting.probability"));
            let period: SUMOTime = if equip || (probability_is_default && v.get_float_param("device.rerouting.probability", false, INVALID_DOUBLE, true) == probability) {
                v.get_time_param("device.rerouting.period", false, SUMOTIME_MIN)
            } else {
                0
            };
            let prePeriod: SUMOTime = std::cmp::max(0, v.get_time_param("device.rerouting.pre-period", false, SUMOTIME_MIN));
            MSRoutingEngine::init_weight_update(-1);
            // build the device
            let id = format!("routing_{}", v.get_id());
            into.push(Ptr::from_raw(Box::into_raw(MSDevice_Routing::new(v, &id, period, prePeriod)) as *mut MSVehicleDevice));
        }
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
        // make the rerouting command invalid if there is one
        if !self.myRerouteCommand.is_null() {
            self.myRerouteCommand.m().deschedule();
        }
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
        if reason == MSMoveReminder_Notification::NOTIFICATION_DEPARTED {
            let now = MSNet::get_instance().m().get_current_time_step();
            if self.myRerouteCommand.is_null() && self.myPreInsertionPeriod > 0 && self.myHolder.m().get_depart_delay() > self.myPreInsertionPeriod {
                // pre-insertion rerouting was disabled. Reroute from the departure edge if insertion was delayed
                // this is happening in the run thread (not inbeginOfTimestepEvents) so we cannot safely use the threadPool
                let router = MSRoutingEngine::get_router_tt(self.myHolder.m().get_rng_index(), self.myHolder.m().get_v_class(), &Default::default());
                self.myHolder.m().reroute(now, "device.rerouting", router, true, MSRoutingEngine::with_taz(), false, Ptr::default());
            }
            // build repetition trigger if routing shall be done more often
            self.rebuild_reroute_command(now + self.myPeriod);
        }
        if MSGlobals::weights_separate_turns() > 0. && !enteredLane.is_null() {
            if reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
                let t = MSNet::get_instance().m().get_current_time_step();
                if self.myLastLaneEntryTime >= 0 && enteredLane.m().is_internal() {
                    // record travel time on the previous edge but store on the internal ledge
                    MSRoutingEngine::add_edge_travel_time(&enteredLane.m().get_edge(), t - self.myLastLaneEntryTime);
                }
                self.myLastLaneEntryTime = t;
            }
            true
        } else {
            false
        }
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
        if self.myRerouteAfterStop {
            self.reroute(MSNet::get_instance().m().get_current_time_step(), false);
            self.myRerouteAfterStop = false;
        }
    }

    /// C++: `const std::string MSDevice_Routing::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "rerouting";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "rerouting".to_string()
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
        out.open_tag_id(SUMO_TAG_DEVICE);
        out.write_attr(SUMO_ATTR_ID, &self.base.base2.base.get_id());
        let internals = vec![format!("{}", self.myPeriod), format!("{}", self.myLastRouting)];
        out.write_attr(SUMO_ATTR_STATE, &internals.join(" "));
        out.close_tag("");
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
        let state = attrs.get_string(SUMO_ATTR_STATE);
        // istringstream >> SUMOTime: whitespace-separated integers, 0 on failed extraction
        let mut tokens = state.split_whitespace();
        self.myPeriod = tokens.next().and_then(|s| s.parse::<SUMOTime>().ok()).unwrap_or(0);
        self.myLastRouting = tokens.next().and_then(|s| s.parse::<SUMOTime>().ok()).unwrap_or(0);
        let now = MSNet::get_instance().m().get_current_time_step();
        if self.myHolder.m().has_departed() {
            let mut offset = self.myPeriod;
            if self.myPeriod > 0 {
                offset = (now - self.myHolder.m().get_departure()) % self.myPeriod;
                if offset != 0 {
                    offset = self.myPeriod - offset;
                }
            }
            self.rebuild_reroute_command(now + offset);
        }
    }

    /// C++: `void MSDevice_Routing::reroute(const SUMOTime, const bool)`
    /// initiate the rerouting, create router / thread pool on first use
    // void
    // MSDevice_Routing::reroute(const SUMOTime currentTime, const bool onInit) {
    //     MSRoutingEngine::initEdgeWeights(myHolder.getVClass());
    //     //check whether the weights did change since the last reroute
    //     if (myLastRouting >= MSRoutingEngine::getLastAdaptation() || !myActive) {
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
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        MSRoutingEngine::init_edge_weights(holder.get_v_class(), -1, -1);
        //check whether the weights did change since the last reroute
        if self.myLastRouting >= MSRoutingEngine::get_last_adaptation() || !self.myActive {
            return;
        }
        let canGate = self.myInvalidation && !onInit && holder.is_on_road();
        if canGate && self.myLastRouteCost >= 0. {
            let router = MSRoutingEngine::get_router_tt(holder.get_rng_index(), holder.get_v_class(), &Default::default());
            let remaining: Vec<Ptr<MSEdge>> = holder.get_route().get_edges()[*holder.get_current_route_edge()..].to_vec();
            if !remaining.is_empty() {
                let costNow = router.recompute_costs(&remaining, holderPtr, currentTime, Ptr::null());
                if costNow - self.myLastRouteCost <= self.myThresholdTime && costNow <= self.myLastRouteCost * self.myThresholdFactor {
                    self.myLastRouting = currentTime;
                    self.myLastRouteCost = costNow;
                    return;
                }
            }
        }
        self.myLastRouting = currentTime;
        MSRoutingEngine::reroute(holder, currentTime, "device.rerouting", onInit, false, &Default::default());
        if self.myInvalidation && holder.is_on_road() {
            let router = MSRoutingEngine::get_router_tt(holder.get_rng_index(), holder.get_v_class(), &Default::default());
            let remaining: Vec<Ptr<MSEdge>> = holder.get_route().get_edges()[*holder.get_current_route_edge()..].to_vec();
            self.myLastRouteCost = if remaining.is_empty() { -1. } else { router.recompute_costs(&remaining, holderPtr, currentTime, Ptr::null()) };
        }
    }

    /// C++: `void MSDevice_Routing::skipRouting(const SUMOTime)`
    /// Labels the current time step as "unroutable".
    // void skipRouting(const SUMOTime currentTime) {
    //         mySkipRouting = currentTime;
    //     }
    #[allow(unused_variables)]
    pub fn skip_routing(&mut self, currentTime: SUMOTime) {
        self.mySkipRouting = currentTime;
    }

    /// C++: `SUMOTime MSDevice_Routing::getPeriod() const` [const]
    // SUMOTime getPeriod() const {
    //         return myPeriod;
    //     }
    #[allow(unused_variables)]
    pub fn get_period(&self) -> SUMOTime {
        self.myPeriod
    }

    /// C++: `bool MSDevice_Routing::mayRerouteRailSignal() const` [const]
    /// return whether the equipped vehicle may receive dispatch information at a rail signal
    // bool mayRerouteRailSignal() const {
    //         return myRerouteRailSignal;
    //     }
    #[allow(unused_variables)]
    pub fn may_reroute_rail_signal(&self) -> bool {
        self.myRerouteRailSignal
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
    pub fn get_parameter(&self, key: &str) -> Result<String, ProcessError> {
        if let Some(edgeID) = key.strip_prefix("edge:") {
            let edge = MSEdge::dictionary_2(edgeID);
            if edge.is_null() {
                return Err(ProcessError::InvalidArgument(format!("Edge '{}' is invalid for parameter retrieval of '{}'", edgeID, self.device_name())));
            }
            return Ok(to_string(&MSRoutingEngine::get_effort(edge, self.myHolder, 0.)));
        } else if key == "period" {
            return Ok(time2string(self.myPeriod));
        }
        Err(ProcessError::InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())))
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
    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<(), ProcessError> {
        let doubleValue: f64 = match value.parse::<f64>() {
            Ok(v) => v,
            Err(_) => return Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name()))),
        };
        if let Some(edgeID) = key.strip_prefix("edge:") {
            let edge = MSEdge::dictionary_2(edgeID);
            if edge.is_null() {
                return Err(ProcessError::InvalidArgument(format!("Edge '{}' is invalid for parameter setting of '{}'", edgeID, self.device_name())));
            }
            MSRoutingEngine::set_edge_travel_time(edge, doubleValue);
        } else if key == "period" {
            self.myPeriod = time2steps(doubleValue);
            // re-schedule routing command
            let now = MSNet::get_instance().m().get_current_time_step();
            self.rebuild_reroute_command(now + self.myPeriod);
        } else {
            return Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name())));
        }
        Ok(())
    }

    /// C++: `void MSDevice_Routing::setActive(bool)`
    // void setActive(bool active) {
    //         myActive = active;
    //     }
    #[allow(unused_variables)]
    pub fn set_active(&mut self, active: bool) {
        self.myActive = active;
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
        if newCost == 0. {
            return true;
        }
        (oldCost / newCost > self.myThresholdFactor) && (oldCost - newCost > self.myThresholdTime)
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
    pub fn new(holder: &mut SUMOVehicle, id: &str, period: SUMOTime, preInsertionPeriod: SUMOTime) -> Box<Self> {
        let base = MSVehicleDevice::new(holder, id);
        let mut dev = Box::new(MSDevice_Routing {
            base,
            myPeriod: period,
            myPreInsertionPeriod: preInsertionPeriod,
            myLastRouting: -1,
            mySkipRouting: -1,
            myRerouteCommand: Ptr::default(),
            myRerouteRailSignal: holder.get_bool_param("device.rerouting.railsignal", true, false),
            myLastLaneEntryTime: -1,
            myRerouteAfterStop: false,
            myThresholdFactor: holder.get_float_param("device.rerouting.threshold.factor", true, 1., true),
            myThresholdTime: steps2time(holder.get_time_param("device.rerouting.threshold.constant", true, 0)),
            myActive: true,
            myInvalidation: with_options(|oc| oc.get_bool("device.rerouting.invalidation").unwrap()),
            myLastRouteCost: -1.,
        });
        dev.base.base.vt = &MS_DEVICE_ROUTING_MSMOVE_REMINDER_VT;
        dev.base.base.idle_vt = &MS_DEVICE_ROUTING_MSMOVE_REMINDER_IDLE_VT;
        // we do always a pre insertion reroute for trips to fill the best lanes of the vehicle with somehow meaningful values (especially for deaprtLane="best")
        if preInsertionPeriod > 0 || holder.get_parameter().was_set(VEHPARS_FORCE_REROUTE) {
            let cmd: Ptr<MSDevice_Routing_WrappingCommand> = Ptr::from_raw(Box::into_raw(Box::new(WrappingCommand::new(Ptr::from_mut(&mut *dev), MSDevice_Routing::pre_insertion_reroute))));
            dev.myRerouteCommand = cmd;
            // if we don't update the edge weights, we might as well reroute now and hopefully use our threads better
            let execTime = if MSRoutingEngine::has_edge_updates() { holder.get_parameter().depart } else { -1 };
            MSNet::get_instance().m().get_insertion_events().m().add_event(Ptr::from_raw(cmd.as_raw() as *mut dyn Command), execTime);
        }
        dev
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
        if self.mySkipRouting == currentTime {
            return utils::common::time::delta_t();
        }
        if self.myPreInsertionPeriod == 0 {
            // the event will deschedule and destroy itself so it does not need to be stored
            self.myRerouteCommand = Ptr::default();
        }
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        let source = holder.get_route().get_first_edge();
        let dest = holder.get_route().get_last_edge();
        if source.is_taz_connector() && dest.is_taz_connector() {
            let cached = MSRoutingEngine::get_cached_route(&(source, dest));
            if let Some(cached) = cached.filter(|c| c.size() > 2) {
                holder.replace_route(cached, "device.rerouting", true, 0, true, true, Ptr::default());
                return self.myPreInsertionPeriod;
            }
        }
        // C++ try/catch resets myRerouteCommand on ProcessError and rethrows; a panic here unwinds the same way
        let mut msg = String::new();
        if holder.has_valid_route_start(&mut msg) {
            self.reroute(currentTime, true);
        }
        // avoid repeated pre-insertion rerouting when the departure edge is fix and
        // the departure lane does not depend on the route
        if self.myPreInsertionPeriod > 0 && !source.is_taz_connector() && holder.get_parameter().departLaneProcedure != DepartLaneDefinition::BEST_FREE {
            self.myRerouteCommand = Ptr::default();
            return 0;
        }
        self.myPreInsertionPeriod
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
        if self.myHolder.m().is_stopped() {
            self.myRerouteAfterStop = true;
        } else {
            self.reroute(currentTime, false);
        }
        self.myPeriod
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
        let mut start = start;
        if !self.myRerouteCommand.is_null() {
            self.myRerouteCommand.m().deschedule();
            self.myRerouteCommand = Ptr::default();
        }
        if self.myPeriod > 0 {
            let raw = Box::into_raw(Box::new(WrappingCommand::new(Ptr::from_mut(self), MSDevice_Routing::wrapped_reroute_command_execute)));
            self.myRerouteCommand = Ptr::from_raw(raw);
            if with_options(|oc| oc.get_bool("device.rerouting.synchronize").unwrap()) {
                start -= start % self.myPeriod;
            }
            // determinism: the event priority (numerical holder id) orders events at equal time
            self.myRerouteCommand.m().myPriority = self.myHolder.m().get_numerical_id() as i32;
            MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(raw as *mut dyn Command), start);
        }
    }

    /// C++: `void MSDevice_Routing::MSDevice_Routing(const MSDevice_Routing &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Routing) -> Self {
        // C++ copy constructor is private and invalidated, so it can never be called
        panic!("MSDevice_Routing copy constructor is invalid (private in C++)")
    }

    /// C++: `MSDevice_Routing & MSDevice_Routing::operator=(const MSDevice_Routing &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Routing) -> &mut MSDevice_Routing {
        // C++ assignment operator is private and invalidated, so it can never be called
        panic!("MSDevice_Routing::operator= is invalid (private in C++)")
    }

}

