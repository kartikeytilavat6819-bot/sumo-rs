//! Port of microsim/devices/MSDevice_GLOSA.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_GLOSA` (microsim/devices/MSDevice_GLOSA.h)
#[repr(C)]
pub struct MSDevice_GLOSA {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// myHolder cast to needed type
    pub myVeh: Ptr<MSVehicle>,
    /// the upcoming traffic light
    pub myNextTLSLink: Ptr<MSLink>,
    /// the distance to the upcoming traffic light
    pub myDistance: f64,
    /// minimum approach speed towards red light
    pub myMinSpeed: f64,
    /// maximum communication range
    pub myRange: f64,
    /// maximum speed factor when trying to reach green light
    pub myMaxSpeedFactor: f64,
    /// Additional time the vehicle shall need to reach the intersection after the signal turns green
    pub myAddSwitchTime: f64,
    /// if true ignore the current light state, always follow GLOSA's predicted state
    pub myOverrideSafety: bool,
    /// if true ignore non-critical speed calculations from the CF model, follow GLOSA's perfect speed calculation
    pub myIgnoreCFModel: bool,
    /// original speed factor
    pub myOriginalSpeedFactor: f64,
    /// If speedFactor is currently beeing changed by the GLOSA device
    pub mySpeedAdviceActive: bool,
    /// if true the queue in front of the TLS is used for calculation
    pub myUseQueue: bool,
}

/// C++ `MIN2` macro
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// C++ `MAX2` macro
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// C++ `SIMSTEP`
fn sim_step() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}

impl std::ops::Deref for MSDevice_GLOSA {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_GLOSA {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_GLOSA {
    /// C++: `void MSDevice_GLOSA::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_GLOSA-options
    // void
    // MSDevice_GLOSA::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("GLOSA Device");
    //     insertDefaultAssignmentOptions("glosa", "GLOSA Device", oc);
    //
    //     oc.doRegister("device.glosa.range", new Option_Float(100.0));
    //     oc.addDescription("device.glosa.range", "GLOSA Device", TL("The communication range to the traffic light"));
    //
    //     oc.doRegister("device.glosa.max-speedfactor", new Option_Float(1.1));
    //     oc.addDescription("device.glosa.max-speedfactor", "GLOSA Device", TL("The maximum speed factor when approaching a green light"));
    //
    //     oc.doRegister("device.glosa.min-speed", new Option_Float(5.0));
    //     oc.addDescription("device.glosa.min-speed", "GLOSA Device", TL("Minimum speed when coasting towards a red light"));
    //
    //     oc.doRegister("device.glosa.add-switchtime", new Option_Float(0.0));
    //     oc.addDescription("device.glosa.add-switchtime", "GLOSA Device", TL("Additional time the vehicle shall need to reach the intersection after the signal turns green"));
    //
    //     oc.doRegister("device.glosa.use-queue", new Option_Bool(false));
    //     oc.addDescription("device.glosa.use-queue", "GLOSA Device", TL("Use queue in front of the tls for GLOSA calculation"));
    //
    //     oc.doRegister("device.glosa.override-safety", new Option_Bool(false));
    //     oc.addDescription("device.glosa.override-safety", "GLOSA Device", TL("Override safety features - ignore the current light state, always follow GLOSA's predicted state"));
    //
    //     oc.doRegister("device.glosa.ignore-cfmodel", new Option_Bool(false));
    //     oc.addDescription("device.glosa.ignore-cfmodel", "GLOSA Device", TL("Vehicles follow a perfect speed calculation - ignore speed calculations from the CF model if not safety critical"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        oc.add_option_sub_topic("GLOSA Device");
        MSDevice::insert_default_assignment_options("glosa", "GLOSA Device", oc, false);

        oc.do_register("device.glosa.range", SumoOption::float(100.0));
        oc.add_description("device.glosa.range", "GLOSA Device", "The communication range to the traffic light");

        oc.do_register("device.glosa.max-speedfactor", SumoOption::float(1.1));
        oc.add_description("device.glosa.max-speedfactor", "GLOSA Device", "The maximum speed factor when approaching a green light");

        oc.do_register("device.glosa.min-speed", SumoOption::float(5.0));
        oc.add_description("device.glosa.min-speed", "GLOSA Device", "Minimum speed when coasting towards a red light");

        oc.do_register("device.glosa.add-switchtime", SumoOption::float(0.0));
        oc.add_description("device.glosa.add-switchtime", "GLOSA Device", "Additional time the vehicle shall need to reach the intersection after the signal turns green");

        oc.do_register("device.glosa.use-queue", SumoOption::bool(false));
        oc.add_description("device.glosa.use-queue", "GLOSA Device", "Use queue in front of the tls for GLOSA calculation");

        oc.do_register("device.glosa.override-safety", SumoOption::bool(false));
        oc.add_description("device.glosa.override-safety", "GLOSA Device", "Override safety features - ignore the current light state, always follow GLOSA's predicted state");

        oc.do_register("device.glosa.ignore-cfmodel", SumoOption::bool(false));
        oc.add_description("device.glosa.ignore-cfmodel", "GLOSA Device", "Vehicles follow a perfect speed calculation - ignore speed calculations from the CF model if not safety critical");
    }

    /// C++: `void MSDevice_GLOSA::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_GLOSA::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (!MSGlobals::gUseMesoSim && equippedByDefaultAssignmentOptions(oc, "glosa", v, false)) {
    //         MSDevice_GLOSA* device = new MSDevice_GLOSA(v, "glosa_" + v.getID(),
    //                 v.getFloatParam("device.glosa.min-speed", true),
    //                 v.getFloatParam("device.glosa.range", true),
    //                 v.getFloatParam("device.glosa.max-speedfactor", true),
    //                 v.getFloatParam("device.glosa.add-switchtime", true),
    //                 v.getBoolParam("device.glosa.use-queue", true),
    //                 v.getBoolParam("device.glosa.override-safety", true),
    //                 v.getBoolParam("device.glosa.ignore-cfmodel", true));
    //         into.push_back(device);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        let equipped = !MSGlobals::use_meso_sim() && utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "glosa", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), false, false)
        });
        if equipped {
            let id = format!("glosa_{}", v.get_id());
            let minSpeed = v.get_float_param("device.glosa.min-speed", true, utils::common::std_defs::INVALID_DOUBLE, true);
            let range = v.get_float_param("device.glosa.range", true, utils::common::std_defs::INVALID_DOUBLE, true);
            let maxSpeedFactor = v.get_float_param("device.glosa.max-speedfactor", true, utils::common::std_defs::INVALID_DOUBLE, true);
            let addSwitchTime = v.get_float_param("device.glosa.add-switchtime", true, utils::common::std_defs::INVALID_DOUBLE, true);
            let useQueue = v.get_bool_param("device.glosa.use-queue", true, false);
            let overrideSafety = v.get_bool_param("device.glosa.override-safety", true, false);
            let ignoreCFModel = v.get_bool_param("device.glosa.ignore-cfmodel", true, false);
            // shortcut: the device is leaked like the C++ new, so the vehicle's device list keeps a stable pointer; upgrade when MSVehicle owns its devices as Box
            let device: &mut MSDevice_GLOSA = Box::leak(Box::new(MSDevice_GLOSA::new(v, &id, minSpeed, range, maxSpeedFactor, addSwitchTime, useQueue, overrideSafety, ignoreCFModel)));
            into.push(Ptr::from_mut(&mut device.base));
        }
    }

    /// C++: `void MSDevice_GLOSA::cleanup()` [static]
    /// resets counters
    // void
    // MSDevice_GLOSA::cleanup() {
    //     // cleaning up global state (if any)
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        // C++ body only cleans up global state (none in this port)
    }

    /// C++: `void MSDevice_GLOSA::~MSDevice_GLOSA() noexcept` [virtual]
    /// Destructor.
    // MSDevice_GLOSA::~MSDevice_GLOSA() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool MSDevice_GLOSA::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// updates distance and computes speed advice
    // bool
    // MSDevice_GLOSA::notifyMove(SUMOTrafficObject& /*tObject*/, double oldPos,
    //                            double newPos, double /*newSpeed*/) {
    //     myDistance -= (newPos - oldPos);
    //     if (myNextTLSLink != nullptr && myDistance <= myRange) {
    //         const double vMax = myVeh.getLane()->getVehicleMaxSpeed(&myVeh);
    //         double timeToJunction = earliest_arrival(myDistance, vMax);
    //         int countOld = 0;
    //         // calculate "first" next phase, all coming Phases are calculated via "getTimeToNextSwitch"
    //         double timeToSwitch = getTimeToSwitch(myNextTLSLink, countOld);
    //         bool currentPhaseGreen = false;
    //         bool currentPhaseStop = false;
    //         bool solved = false;
    //         double nextSwitch = 0;
    //         nextSwitch = timeToSwitch;
    //         double QueueLength = 0;
    //         double greenTime = 0;
    //         double additionalJunctionTime = 0;
    //         // It takes factor [seconds/per meter queue] to dissolve the queue at drive off
    //         // experimental value from calibrated drone data
    //         double factor = 0.21;
    //         // Additional time (offset) for the leading vehicle to accelerate
    //         // experimental value from calibrated drone data
    //         double addition = 3;
    //         // Vehicles are often not able to drive with their desired speed vMax.
    //         // The further away from the junction, the more uncertain the arrival prediction becomes.
    //         // For a vehicle trying to reach the begin/end of a green phase, this is critical.
    //         // The vehicle should then rather opt for the next phase (trade travel time for traffic flow)
    //         int switchOffset = 0;
    //
    //         if (myNextTLSLink->haveGreen()) {
    //             currentPhaseGreen = true;
    //         } else if (myNextTLSLink->haveRed() || myNextTLSLink->haveYellow()) {
    //             currentPhaseStop = true;
    //         }
    //         // else if any other phase, GLOSA does not interfere
    // #if defined DEBUG_GLOSA || defined DEBUG_QUEUE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << myVeh.getID() << " d=" << myDistance << " ttJ=" << timeToJunction << " ttS=" << timeToSwitch << "\n";
    //         }
    // #endif
    //         if (myUseQueue) {
    //             // Detect queue length at tls
    //             QueueLength = myNextTLSLink->getTLLogic()->getTLQueueLength(myNextTLSLink->getLaneBefore()->getID());
    // #ifdef DEBUG_QUEUE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " veh=" << myVeh.getID() << " Queuelength=" << QueueLength << "\n";
    //             }
    // #endif
    //             if (currentPhaseGreen) {
    //                 // how long has it already been green in this phase
    //                 greenTime = timeGreen(myNextTLSLink);
    //                 additionalJunctionTime = (QueueLength * factor + addition) - greenTime;
    //                 if ((additionalJunctionTime > 0) && (additionalJunctionTime < nextSwitch)) {
    //                     // do note use queue system if queue is to long
    //                     timeToJunction += additionalJunctionTime;
    //                 } else {
    //                     // important for case: "speed can be increased to arrive at tls while green"
    //                     additionalJunctionTime = 0;
    //                 }
    // #ifdef DEBUG_QUEUE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " veh=" << myVeh.getID() << " Additonaljunctiontime=" << additionalJunctionTime << " Greentime=" << greenTime << " TimetoJunction(GreenQueue)=" << timeToJunction << "\n";
    //                 }
    // #endif
    //             }
    //
    //             if (currentPhaseStop) {
    //                 nextSwitch += QueueLength * factor + addition;
    // #ifdef DEBUG_QUEUE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " veh=" << myVeh.getID() << " Nextswitch(RedQueue)=" << nextSwitch << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //
    //         // Search for the next passable phase, maximum 10 Phases
    //         for (int countwhile = 1; countwhile < 11; countwhile++) {
    //             if (currentPhaseGreen) {
    //                 // reset nextSwitch because the queue matters only for the current Phase
    //                 if (countwhile == 2 && myUseQueue) {
    //                     nextSwitch -= QueueLength * factor + addition;
    //                 }
    //                 if (mySpeedAdviceActive && myOriginalSpeedFactor > myVeh.getChosenSpeedFactor()) {
    //                     myVeh.setChosenSpeedFactor(myOriginalSpeedFactor);
    //                     mySpeedAdviceActive = false;
    //                 }
    // #ifdef DEBUG_GLOSA
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " veh=" << myVeh.getID() << " traffic light will be/is green" << "\n";
    //                     std::cout << SIMTIME << " veh=" << myVeh.getID()
    //                               << " timeToJunction=" << timeToJunction
    //                               << " nextSwitch=" << nextSwitch
    //                               << " myDistance=" << myDistance << "\n";
    //                 }
    // #endif
    //                 // if we arrive at the tls after it switched to red (else do nothing)
    //                 if (timeToJunction > nextSwitch) {
    //                     // if speed can be increased to arrive at tls while green (else look for next phases)
    //                     if (myMaxSpeedFactor > myOriginalSpeedFactor) {
    //                         const double vMax2 = vMax / myVeh.getChosenSpeedFactor() * myMaxSpeedFactor;
    //                         const double timetoJunction2 = earliest_arrival(myDistance, vMax2) + additionalJunctionTime;
    //                         // reaching the signal at yellow might be sufficient
    //                         const double yellowSlack = myVeh.getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_DRIVE_AFTER_YELLOW_TIME, 0);
    // #ifdef DEBUG_GLOSA
    //                         if (DEBUG_COND) {
    //                             std::cout << SIMTIME << " veh=" << myVeh.getID()
    //                                       << " vMax2=" << vMax2
    //                                       << " timetoJunction2=" << timetoJunction2
    //                                       << " yellowSlack=" << yellowSlack << "\n";
    //                         }
    // #endif
    //                         // if increased speed is fast enough to arrive at tls while green (else look for next phases)
    //                         if (timetoJunction2 <= (nextSwitch + yellowSlack)) {
    //                             // increase speed factor up to a maximum if necessary and useful
    //                             // XXX could compute optimal speed factor here
    //                             myVeh.setChosenSpeedFactor(myMaxSpeedFactor);
    //                             mySpeedAdviceActive = true;
    //                             break; // solved
    //                         } else {
    //                             // speed can not be increased to arrive at tls while green
    //                             // next switch is calculated at the end of the for-loop
    //                         }
    //                     } else {
    //                         // speed increase is not enough to reach tls while green
    //                         // next switch is calculated at the end of the for-loop
    //                     }
    //                 } else {
    //                     // vehicle will arrive at tls while green
    //                     break; // solved
    //                 }
    //             } else if (currentPhaseStop) {
    //                 // tls is red at the moment
    // #ifdef DEBUG_GLOSA
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " veh=" << myVeh.getID() << " traffic light will be/is red" << "\n";
    //                     std::cout << SIMTIME << " veh=" << myVeh.getID()
    //                               << " timeToJunction=" << timeToJunction
    //                               << " nextSwitch=" << nextSwitch
    //                               << " myDistance=" << myDistance << "\n";
    //                 }
    // #endif
    //                 if (countwhile == 2 && myUseQueue) {
    //                     // take queue into account if current green phase can not be passed and next red phase is used instead
    //                     nextSwitch += QueueLength * factor + addition;
    // #ifdef DEBUG_QUEUE
    //                     if (DEBUG_COND) {
    //                         std::cout << SIMTIME << " veh=" << myVeh.getID() << " nextSwitch(redadditon): " << nextSwitch << "\n";
    //                     }
    // #endif
    //                 }
    //                 adaptSpeed(myDistance, timeToJunction, nextSwitch + myAddSwitchTime, solved);
    //                 if (countwhile == 2 && myUseQueue) {
    //                     // reset queue because following phases should not calculate with current queue
    //                     nextSwitch -= QueueLength * factor + addition;
    //                     timeToJunction -= additionalJunctionTime;
    //                 }
    //                 if (solved) {
    //                     break; // solved
    //                 }
    //             }
    //             // calculate next Phase
    //             nextSwitch += getTimeToNextSwitch(myNextTLSLink, currentPhaseGreen, currentPhaseStop, countOld);
    //             // For vehicles far away from the junction we add an offset
    //             if (nextSwitch > 80.) {
    //                 nextSwitch += (double)switchOffset;
    //                 switchOffset = 6;
    //                 nextSwitch -= (double)switchOffset;
    //             } else if (nextSwitch > 60.) {
    //                 nextSwitch += (double)switchOffset;
    //                 switchOffset = 4;
    //                 nextSwitch -= (double)switchOffset;
    //             } else if (nextSwitch > 40.) {
    //                 nextSwitch += (double)switchOffset;
    //                 switchOffset = 3;
    //                 nextSwitch -= (double)switchOffset;
    //             } else if (nextSwitch > 20.) {
    //                 nextSwitch += (double)switchOffset;
    //                 switchOffset = 2;
    //                 nextSwitch -= (double)switchOffset;
    //             }
    //         }
    //     }
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        self.myDistance -= newPos - oldPos;
        if !self.myNextTLSLink.is_null() && self.myDistance <= self.myRange {
            let vMax = self.myVeh.get_lane().get_vehicle_max_speed(self.myVeh.cast::<SUMOTrafficObject>());
            let mut timeToJunction = self.earliest_arrival(self.myDistance, vMax);
            let mut countOld: i32 = 0;
            let timeToSwitch = Self::get_time_to_switch(self.myNextTLSLink, &mut countOld);
            let mut currentPhaseGreen = false;
            let mut currentPhaseStop = false;
            let mut solved = false;
            let mut nextSwitch: f64 = 0.;
            nextSwitch = timeToSwitch;
            let mut QueueLength = 0.;
            let mut greenTime = 0.;
            let mut additionalJunctionTime = 0.;
            // It takes factor [seconds/per meter queue] to dissolve the queue at drive off
            // experimental value from calibrated drone data
            let factor = 0.21;
            // Additional time (offset) for the leading vehicle to accelerate
            // experimental value from calibrated drone data
            let addition = 3.;
            // Vehicles are often not able to drive with their desired speed vMax.
            // The further away from the junction, the more uncertain the arrival prediction becomes.
            // For a vehicle trying to reach the begin/end of a green phase, this is critical.
            // The vehicle should then rather opt for the next phase (trade travel time for traffic flow)
            let mut switchOffset: i32 = 0;

            if self.myNextTLSLink.have_green() {
                currentPhaseGreen = true;
            } else if self.myNextTLSLink.have_red() || self.myNextTLSLink.have_yellow() {
                currentPhaseStop = true;
            }
            // else if any other phase, GLOSA does not interfere

            if self.myUseQueue {
                // Detect queue length at tls
                QueueLength = self.myNextTLSLink.get_tl_logic().get_tl_queue_length(self.myNextTLSLink.get_lane_before().get_id().to_string());

                if currentPhaseGreen {
                    // how long has it already been green in this phase
                    greenTime = Self::time_green(self.myNextTLSLink);
                    additionalJunctionTime = (QueueLength * factor + addition) - greenTime;
                    if (additionalJunctionTime > 0.) && (additionalJunctionTime < nextSwitch) {
                        // do note use queue system if queue is to long
                        timeToJunction += additionalJunctionTime;
                    } else {
                        // important for case: "speed can be increased to arrive at tls while green"
                        additionalJunctionTime = 0.;
                    }
                }

                if currentPhaseStop {
                    nextSwitch += QueueLength * factor + addition;
                }
            }

            // Search for the next passable phase, maximum 10 Phases
            for countwhile in 1..11 {
                if currentPhaseGreen {
                    // reset nextSwitch because the queue matters only for the current Phase
                    if countwhile == 2 && self.myUseQueue {
                        nextSwitch -= QueueLength * factor + addition;
                    }
                    if self.mySpeedAdviceActive && self.myOriginalSpeedFactor > self.myVeh.get_chosen_speed_factor() {
                        self.myVeh.m().set_chosen_speed_factor(self.myOriginalSpeedFactor);
                        self.mySpeedAdviceActive = false;
                    }
                    // if we arrive at the tls after it switched to red (else do nothing)
                    if timeToJunction > nextSwitch {
                        // if speed can be increased to arrive at tls while green (else look for next phases)
                        if self.myMaxSpeedFactor > self.myOriginalSpeedFactor {
                            let vMax2 = vMax / self.myVeh.get_chosen_speed_factor() * self.myMaxSpeedFactor;
                            let timetoJunction2 = self.earliest_arrival(self.myDistance, vMax2) + additionalJunctionTime;
                            // reaching the signal at yellow might be sufficient
                            let yellowSlack = self.myVeh.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_DRIVE_AFTER_YELLOW_TIME, 0.);
                            // if increased speed is fast enough to arrive at tls while green (else look for next phases)
                            if timetoJunction2 <= (nextSwitch + yellowSlack) {
                                // increase speed factor up to a maximum if necessary and useful
                                // XXX could compute optimal speed factor here
                                self.myVeh.m().set_chosen_speed_factor(self.myMaxSpeedFactor);
                                self.mySpeedAdviceActive = true;
                                break; // solved
                            }
                            // else: speed can not be increased to arrive at tls while green
                            // next switch is calculated at the end of the for-loop
                        }
                        // else: speed increase is not enough to reach tls while green
                        // next switch is calculated at the end of the for-loop
                    } else {
                        // vehicle will arrive at tls while green
                        break; // solved
                    }
                } else if currentPhaseStop {
                    // tls is red at the moment
                    if countwhile == 2 && self.myUseQueue {
                        // take queue into account if current green phase can not be passed and next red phase is used instead
                        nextSwitch += QueueLength * factor + addition;
                    }
                    self.adapt_speed(self.myDistance, timeToJunction, nextSwitch + self.myAddSwitchTime, &mut solved);
                    if countwhile == 2 && self.myUseQueue {
                        // reset queue because following phases should not calculate with current queue
                        nextSwitch -= QueueLength * factor + addition;
                        timeToJunction -= additionalJunctionTime;
                    }
                    if solved {
                        break; // solved
                    }
                }
                // calculate next Phase
                nextSwitch += Self::get_time_to_next_switch(self.myNextTLSLink, &mut currentPhaseGreen, &mut currentPhaseStop, &mut countOld);
                // For vehicles far away from the junction we add an offset
                if nextSwitch > 80. {
                    nextSwitch += switchOffset as f64;
                    switchOffset = 6;
                    nextSwitch -= switchOffset as f64;
                } else if nextSwitch > 60. {
                    nextSwitch += switchOffset as f64;
                    switchOffset = 4;
                    nextSwitch -= switchOffset as f64;
                } else if nextSwitch > 40. {
                    nextSwitch += switchOffset as f64;
                    switchOffset = 3;
                    nextSwitch -= switchOffset as f64;
                } else if nextSwitch > 20. {
                    nextSwitch += switchOffset as f64;
                    switchOffset = 2;
                    nextSwitch -= switchOffset as f64;
                }
            }
        }
        true // keep the device
    }

    /// C++: `bool MSDevice_GLOSA::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// updates next tls link
    // bool
    // MSDevice_GLOSA::notifyEnter(SUMOTrafficObject& /*veh*/, MSMoveReminder::Notification /*reason*/, const MSLane* /* enteredLane */) {
    //     const MSLink* prevLink = myNextTLSLink;
    //     myNextTLSLink = nullptr;
    //     const MSLane* lane = myVeh.getLane();
    //     if (myVeh.getDeparture() < SIMSTEP) {
    //         // no need to call at insertion
    //         myVeh.updateBestLanes();
    //     }
    //     const std::vector<MSLane*>& bestLaneConts = myVeh.getBestLanesContinuation(lane);
    //     double seen = lane->getLength() - myVeh.getPositionOnLane();
    //     int view = 1;
    //     std::vector<MSLink*>::const_iterator linkIt = MSLane::succLinkSec(myVeh, view, *lane, bestLaneConts);
    //     while (!lane->isLinkEnd(linkIt)) {
    //         if (!lane->getEdge().isInternal()) {
    //             if ((*linkIt)->isTLSControlled()) {
    //                 myNextTLSLink = *linkIt;
    //                 myDistance = seen;
    //                 break;
    //             }
    //         }
    //         lane = (*linkIt)->getViaLaneOrLane();
    //         if (!lane->getEdge().isInternal()) {
    //             view++;
    //         }
    //         seen += lane->getLength();
    //         linkIt = MSLane::succLinkSec(myVeh, view, *lane, bestLaneConts);
    //     }
    //     if (prevLink != nullptr && myNextTLSLink == nullptr) {
    //         // moved pass tls
    //         myVeh.setChosenSpeedFactor(myOriginalSpeedFactor);
    //         mySpeedAdviceActive = false;
    //     } else if (myNextTLSLink != nullptr && prevLink != myNextTLSLink) {
    //         // approaching new tls
    //         double tlsRange = 1e10;
    //         const std::string val = myNextTLSLink->getTLLogic()->getParameter("device.glosa.range", "1e10");
    //         try {
    //             tlsRange = StringUtils::toDouble(val);
    //         } catch (const NumberFormatException&) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for parameter 'device.glosa.range' of traffic light '%'"),
    //                            val, myNextTLSLink->getTLLogic()->getID());
    //         }
    //         myRange = MIN2(myVeh.getFloatParam("device.glosa.range", true), tlsRange);
    //     }
    //
    // #ifdef DEBUG_GLOSA
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << myVeh.getID() << " enter=" << myVeh.getLane()->getID() << " tls=" << (myNextTLSLink == nullptr ? "NULL" : myNextTLSLink->getTLLogic()->getID()) << " dist=" << myDistance << "\n";
    //     }
    // #endif
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        let prevLink = self.myNextTLSLink;
        self.myNextTLSLink = Ptr::null();
        let mut lane = self.myVeh.get_lane();
        if self.myVeh.get_departure() < sim_step() {
            // no need to call at insertion
            self.myVeh.m().update_best_lanes(false, Ptr::null());
        }
        let bestLaneConts = self.myVeh.get_best_lanes_continuation_2(lane);
        let mut seen = lane.get_length() - self.myVeh.get_position_on_lane();
        let mut view: i32 = 1;
        let mut linkIt: usize = MSLane::succ_link_sec(&*self.myVeh, view, &*lane, bestLaneConts);
        while !lane.is_link_end(&mut linkIt) {
            if !lane.myEdge.is_internal() {
                let link = lane.myLinks[linkIt];
                if link.is_tls_controlled() {
                    self.myNextTLSLink = link;
                    self.myDistance = seen;
                    break;
                }
            }
            lane = lane.myLinks[linkIt].get_via_lane_or_lane();
            if !lane.myEdge.is_internal() {
                view += 1;
            }
            seen += lane.get_length();
            linkIt = MSLane::succ_link_sec(&*self.myVeh, view, &*lane, bestLaneConts);
        }
        if !prevLink.is_null() && self.myNextTLSLink.is_null() {
            // moved pass tls
            self.myVeh.m().set_chosen_speed_factor(self.myOriginalSpeedFactor);
            self.mySpeedAdviceActive = false;
        } else if !self.myNextTLSLink.is_null() && prevLink != self.myNextTLSLink {
            // approaching new tls
            let mut tlsRange = 1e10;
            let val = self.myNextTLSLink.get_tl_logic().base2.get_parameter("device.glosa.range", "1e10");
            match utils::common::string_utils::to_double(&val) {
                Ok(d) => tlsRange = d,
                Err(_) => {
                    write_warningf!("Invalid value '%' for parameter 'device.glosa.range' of traffic light '%'", val, self.myNextTLSLink.get_tl_logic().base.get_id());
                }
            }
            self.myRange = min2(self.myVeh.get_float_param("device.glosa.range", true, utils::common::std_defs::INVALID_DOUBLE, true), tlsRange);
        }
        true // keep the device
    }

    /// C++: `const std::string MSDevice_GLOSA::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "glosa";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "glosa".to_string()
    }

    /// C++: `double MSDevice_GLOSA::getOriginalSpeedFactor() const` [const]
    /// Returns the precomputed, original factor by which the driver wants to be faster than the speed limit
    // inline double getOriginalSpeedFactor() const {
    //         return myOriginalSpeedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_original_speed_factor(&self) -> f64 {
        self.myOriginalSpeedFactor
    }

    /// C++: `bool MSDevice_GLOSA::getOverrideSafety() const` [const]
    /// Returns if the traffic light stop calculation of the CF model shall be ignored
    // inline bool getOverrideSafety() const {
    //         return myOverrideSafety;
    //     }
    #[allow(unused_variables)]
    pub fn get_override_safety(&self) -> bool {
        self.myOverrideSafety
    }

    /// C++: `bool MSDevice_GLOSA::isSpeedAdviceActive() const` [const]
    /// Returns if the GLOSA device is currently changing the speedFactor
    // inline bool isSpeedAdviceActive() const {
    //         return mySpeedAdviceActive;
    //     }
    #[allow(unused_variables)]
    pub fn is_speed_advice_active(&self) -> bool {
        self.mySpeedAdviceActive
    }

    /// C++: `std::string MSDevice_GLOSA::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_GLOSA::getParameter(const std::string& key) const {
    //     if (key == "minSpeed") {
    //         return toString(myMinSpeed);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> Result<String, ProcessError> {
        if key == "minSpeed" {
            return Ok(utils::common::to_string::to_string(&self.myMinSpeed));
        }
        Err(ProcessError::InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())))
    }

    /// C++: `void MSDevice_GLOSA::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_GLOSA::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    //     if (key == "minSpeed") {
    //         myMinSpeed = doubleValue;
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<(), ProcessError> {
        let doubleValue = match utils::common::string_utils::to_double(value) {
            Ok(d) => d,
            Err(_) => return Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name()))),
        };
        if key == "minSpeed" {
            self.myMinSpeed = doubleValue;
        } else {
            return Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name())));
        }
        Ok(())
    }

    /// C++: `void MSDevice_GLOSA::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSDevice_GLOSA::generateOutput(OutputDevice* /*tripinfoOut*/) const {
    //     /*
    //     if (tripinfoOut != nullptr) {
    //         tripinfoOut->openTag("glosa_device");
    //         tripinfoOut->closeTag();
    //     }
    //     */
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        // C++ body is commented out: nothing is written to the tripinfo output
    }

    /// C++: `double MSDevice_GLOSA::getTimeToSwitch(const MSLink *, int &)` [static, private]
    /// compute time to next (relevant) switch
    // double
    // MSDevice_GLOSA::getTimeToSwitch(const MSLink* tlsLink, int& countOld) {
    //     assert(tlsLink != nullptr);
    //     const MSTrafficLightLogic* const tl = tlsLink->getTLLogic();
    //     assert(tl != nullptr);
    //     const auto& phases = tl->getPhases();
    //     const int n = (int)phases.size();
    //     const int cur = tl->getCurrentPhaseIndex();
    //     SUMOTime result = tl->getNextSwitchTime() - SIMSTEP;
    //
    //     for (int i = 1; i < n; i++) {
    //         const auto& phase = phases[(cur + i) % n];
    //         const char ls = phase->getState()[tlsLink->getTLIndex()];
    //         if (((tlsLink->haveRed() || tlsLink->haveYellow()) && (ls == 'g' || ls == 'G'))
    //                 || (tlsLink->haveGreen() && ls != 'g' && ls != 'G')) {
    //             countOld = cur + i;
    //             break;
    //         }
    //         result += phase->duration;
    //     }
    //     return STEPS2TIME(result);
    // }
    #[allow(unused_variables)]
    pub fn get_time_to_switch(tlsLink: Ptr<MSLink>, countOld: &mut i32) -> f64 {
        assert!(!tlsLink.is_null());
        let tl = tlsLink.get_tl_logic();
        assert!(!tl.is_null());
        let phases = tl.get_phases();
        let n = phases.len() as i32;
        let cur = tl.get_current_phase_index();
        let mut result: SUMOTime = tl.get_next_switch_time() - sim_step();

        for i in 1..n {
            let phase = &phases[((cur + i) % n) as usize];
            let ls = phase.get_state().as_bytes()[tlsLink.get_tl_index() as usize] as char;
            if ((tlsLink.have_red() || tlsLink.have_yellow()) && (ls == 'g' || ls == 'G'))
                    || (tlsLink.have_green() && ls != 'g' && ls != 'G') {
                *countOld = cur + i;
                break;
            }
            result += phase.duration;
        }
        utils::common::time::steps2time(result)
    }

    /// C++: `double MSDevice_GLOSA::getTimeToNextSwitch(const MSLink *, bool &, bool &, int &)` [static, private]
    /// compute time to next (relevant) switch the vehicle can reach
    // double
    // MSDevice_GLOSA::getTimeToNextSwitch(const MSLink* tlsLink, bool& currentPhaseGreen, bool& currentPhaseStop, int& countOld) {
    //     // get time till the tls switches to a phase the vehicle can reach
    //     assert(tlsLink != nullptr);
    //     const MSTrafficLightLogic* const tl = tlsLink->getTLLogic();
    //     assert(tl != nullptr);
    //     const auto& phases = tl->getPhases();
    //     const int n = (int)phases.size();
    //     const int cur = countOld;
    //     SUMOTime result = 0;
    //
    //     for (int i = 0; i < n; i++) {
    //         const auto& phase = phases[(cur + i) % n];
    //         const char ls = phase->getState()[tlsLink->getTLIndex()];
    //         if (currentPhaseGreen && (ls == 'g' || ls == 'G')) {
    //             countOld = (cur + i) % n;
    //             break;
    //         }
    //         if (currentPhaseStop && (ls != 'g' && ls != 'G')) {
    //             countOld = (cur + i) % n;
    //             break;
    //         }
    //         result += phase->duration;
    //     }
    //     currentPhaseGreen = !currentPhaseGreen;
    //     currentPhaseStop = !currentPhaseStop;
    //     return STEPS2TIME(result);
    // }
    #[allow(unused_variables)]
    pub fn get_time_to_next_switch(tlsLink: Ptr<MSLink>, currentPhaseGreen: &mut bool, currentPhaseStop: &mut bool, countOld: &mut i32) -> f64 {
        // get time till the tls switches to a phase the vehicle can reach
        assert!(!tlsLink.is_null());
        let tl = tlsLink.get_tl_logic();
        assert!(!tl.is_null());
        let phases = tl.get_phases();
        let n = phases.len() as i32;
        let cur = *countOld;
        let mut result: SUMOTime = 0;

        for i in 0..n {
            let phase = &phases[((cur + i) % n) as usize];
            let ls = phase.get_state().as_bytes()[tlsLink.get_tl_index() as usize] as char;
            if *currentPhaseGreen && (ls == 'g' || ls == 'G') {
                *countOld = (cur + i) % n;
                break;
            }
            if *currentPhaseStop && (ls != 'g' && ls != 'G') {
                *countOld = (cur + i) % n;
                break;
            }
            result += phase.duration;
        }
        *currentPhaseGreen = !*currentPhaseGreen;
        *currentPhaseStop = !*currentPhaseStop;
        utils::common::time::steps2time(result)
    }

    /// C++: `double MSDevice_GLOSA::timeGreen(const MSLink *)` [static, private]
    // double
    // MSDevice_GLOSA::timeGreen(const MSLink* tlsLink) {
    //     assert(tlsLink != nullptr);
    //     const MSTrafficLightLogic* const tl = tlsLink->getTLLogic();
    //     assert(tl != nullptr);
    //     const auto& phases = tl->getPhases();
    //     const int n = (int)phases.size();
    //     const int cur = tl->getCurrentPhaseIndex();
    //     // As there are multiple "microphases G" in one Greenphase this function only gives back
    //     // the already spent time in the current microphase
    //     SUMOTime result = tl->getSpentDuration();
    //
    //     for (int i = 1; i < n; i++) {
    //         const auto& phase = phases[(cur - i) % n];
    //         const char ls = phase->getState()[tlsLink->getTLIndex()];
    //         if (ls != 'g' && ls != 'G') {
    //             break;
    //         }
    //         result += phase->duration;
    //     }
    //     return STEPS2TIME(result);
    // }
    #[allow(unused_variables)]
    pub fn time_green(tlsLink: Ptr<MSLink>) -> f64 {
        assert!(!tlsLink.is_null());
        let tl = tlsLink.get_tl_logic();
        assert!(!tl.is_null());
        let phases = tl.get_phases();
        let n = phases.len() as i32;
        let cur = tl.get_current_phase_index();
        // As there are multiple "microphases G" in one Greenphase this function only gives back
        // the already spent time in the current microphase
        let mut result: SUMOTime = tl.get_spent_duration(-1);

        for i in 1..n {
            // shortcut: C++ indexes phases[(cur - i) % n], which is a negative index (undefined behaviour) when cur < i; the port wraps it cyclically with rem_euclid, upgrade if the undefined case is ever reproduced
            let phase = &phases[((cur - i).rem_euclid(n)) as usize];
            let ls = phase.get_state().as_bytes()[tlsLink.get_tl_index() as usize] as char;
            if ls != 'g' && ls != 'G' {
                break;
            }
            result += phase.duration;
        }
        utils::common::time::steps2time(result)
    }

    /// C++: `double MSDevice_GLOSA::earliest_arrival(double, double)` [private]
    /// return minimum number of seconds to reach the junction
    // double
    // MSDevice_GLOSA::earliest_arrival(double distance, double vMax) {
    //     // assume we keep acceleration until we hit maximum speed
    //     const double v = myVeh.getSpeed();
    //     const double a = myVeh.getCarFollowModel().getMaxAccel();
    //     const double accel_time = MIN2((vMax - v) / a, time_to_junction_at_continuous_accel(distance, v));
    //     const double remaining_dist = distance - distance_at_continuous_accel(v, accel_time);
    //     const double remaining_time = remaining_dist / vMax;
    //     return accel_time + remaining_time;
    // }
    #[allow(unused_variables)]
    pub fn earliest_arrival(&mut self, distance: f64, vMax: f64) -> f64 {
        // assume we keep acceleration until we hit maximum speed
        let v = self.myVeh.get_speed();
        let a = self.myVeh.get_car_follow_model().get_max_accel();
        let accel_time = min2((vMax - v) / a, self.time_to_junction_at_continuous_accel(distance, v));
        let remaining_dist = distance - self.distance_at_continuous_accel(v, accel_time);
        let remaining_time = remaining_dist / vMax;
        accel_time + remaining_time
    }

    /// C++: `double MSDevice_GLOSA::latest_arrival(double, double, double)` [private]
    /// return maximum number of seconds to reach the junction
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn latest_arrival(&mut self, speed: f64, distance: f64, earliest: f64) -> f64 {
        // C++ defines latest_arrival only inside a /* */ block of pseudo-code, so the compiled C++ has no such method
        panic!("MSDevice_GLOSA::latest_arrival has no C++ implementation (commented-out pseudo-code)")
    }

    /// C++: `double MSDevice_GLOSA::distance_at_continuous_accel(double, double)` [private]
    // double
    // MSDevice_GLOSA::distance_at_continuous_accel(double speed, double time) {
    //     const double v = speed;
    //     const double t = time;
    //     const double a = myVeh.getCarFollowModel().getMaxAccel();
    //     // integrated area composed of a rectangle and a triangle
    //     return v * t + a * t * t / 2;
    // }
    #[allow(unused_variables)]
    pub fn distance_at_continuous_accel(&mut self, speed: f64, time: f64) -> f64 {
        let v = speed;
        let t = time;
        let a = self.myVeh.get_car_follow_model().get_max_accel();
        // integrated area composed of a rectangle and a triangle
        v * t + a * t * t / 2.
    }

    /// C++: `double MSDevice_GLOSA::time_to_junction_at_continuous_accel(double, double)` [private]
    // double
    // MSDevice_GLOSA::time_to_junction_at_continuous_accel(double d, double v) {
    //     // see distance_at_continuous_accel
    //     // t^2 + (2v/a)t - 2d/a = 0
    //     const double a = myVeh.getCarFollowModel().getMaxAccel();
    //     const double p_half = v / a;
    //     const double t = -p_half + sqrt(p_half * p_half + 2 * d / a);
    //     return t;
    // }
    #[allow(unused_variables)]
    pub fn time_to_junction_at_continuous_accel(&mut self, d: f64, v: f64) -> f64 {
        // see distance_at_continuous_accel
        // t^2 + (2v/a)t - 2d/a = 0
        let a = self.myVeh.get_car_follow_model().get_max_accel();
        let p_half = v / a;
        let t = -p_half + (p_half * p_half + 2. * d / a).sqrt();
        t
    }

    /// C++: `void MSDevice_GLOSA::adaptSpeed(double, double, double, bool &)` [private]
    /// adapt speed to reach junction at green
    // void
    // MSDevice_GLOSA::adaptSpeed(double distance, double /*timeToJunction*/, double timeToSwitch, bool& solved) {
    //     // ensure that myVehicle arrives at the
    //     // junction with maximum speed when it switches to green
    //     // car performs a slowDown at time z to speed x for duration y
    //     // there are two basic strategies
    //     // a) maximize z -> this saves road space but leads to low x and thus excessive braking
    //     // b) maximize x -> this saves fuel but wastes road
    //     // c) compromise: b) but only when distance to junction is below a threshold
    //
    //     const double vMax = myVeh.getLane()->getSpeedLimit() * myOriginalSpeedFactor;
    //
    //     // need to start/continue maneuver
    //     const double t = timeToSwitch;
    //     const double a = myVeh.getCarFollowModel().getMaxAccel();
    //     const double u = myMinSpeed;
    //     const double w = vMax;
    //     const double s = distance;
    //     const double v = myVeh.getSpeed();
    //     // x : target speed
    //     // y : slow down duration
    //     // s is composed of 1 trapezoid (decel), 1 rectangle (maintain), 1 trapezoid (accel)
    //     // s = (v^2-x^2)/2d + x*(t-y-(w-x)/a) + (w^2-x^2)/2a
    //     // y = (v-x)/d
    //     // solution for x curtesy of mathomatic.org
    //
    //     // First, we calculate targetSpeed assuming we are driving that speed already (v=x)
    //     // If this results in targetSpeed < currentSpeed, then we need to decelerate (and vice versa)
    //     const double rootConst = a * a * t * t - 2.0 * a * w * t + 2 * a * s;
    //     double vConst = 0;
    //     if (rootConst >= 0) {
    //         vConst = sqrt(rootConst) - a * t + w;
    //     }
    //     double d = myVeh.getCarFollowModel().getMaxDecel();
    //     if (v < vConst) {
    //         d = a;
    //     }
    //
    //     // Second, we calculate the correct targetSpeed, knowing if we need to accelerate or decelerate
    //     const double sign0 = -1; // quadratic formula solution x1 (choose solution with higher speed)
    //     const double root_argument = a * d * ((2.0 * d * (s - (w * t))) - ((v - w) * (v - w)) + (a * ((d * (t * t)) + (2.0 * (s - (t * v))))));
    //     if (root_argument < 0) {
    // #ifdef DEBUG_GLOSA
    //         WRITE_WARNINGF("GLOSA error 1 root_argument=% s=% t=% v=%", root_argument, s, t, v);
    // #endif
    //         // no reset of speedFactor because in this case, current speed can be kept
    //         solved = true;
    //
    //         return;
    //     }
    //     const double x = (((a * (v - (d * t))) + (d * w) - sign0 * sqrt(root_argument)) / (d + a));
    //     double y = (v - x) / d;
    //     if (v < x) {
    //         y = (x - v) / d;
    //     }
    //     if (s < (w * w - x * x) / 2.0 / a) {
    //         // end maneuver
    //         if (myIgnoreCFModel) {
    //             std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //             speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), myVeh.getSpeed()));
    //             speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(w - x / a), vMax));
    //             myVeh.getInfluencer().setSpeedTimeLine(speedTimeLine);
    //         } else {
    //             myVeh.setChosenSpeedFactor(myOriginalSpeedFactor);
    //         }
    //         mySpeedAdviceActive = false;
    //         return;
    //     }
    //     if (!(x >= u && x <= w && y < t)) {
    // #ifdef DEBUG_GLOSA
    //         WRITE_WARNINGF("GLOSA error 2 x=% y=% s=% t=% v=%", x, y, s, t, v);
    // #endif
    //         if (x < u) {
    // #ifdef DEBUG_GLOSA
    //             if (DEBUG_COND) {
    //                 std::cout << "veh=" << myVeh.getID() << " cant go slow enough" << "\n";
    //             }
    // #endif
    //             // no reset of speedFactor because in this case, current speed can be kept
    //             //myVeh.setChosenSpeedFactor(myOriginalSpeedFactor);
    //             //mySpeedAdviceActive = false;
    //             solved = true;
    //         }
    //         if (x > w) {
    // #ifdef DEBUG_GLOSA
    //             if (DEBUG_COND) {
    //                 std::cout << "veh=" << myVeh.getID() << " cant go fast enough" << "\n";
    //             }
    // #endif
    //         }
    //         return;
    //     }
    //     const double targetSpeed = MAX2(x, myVeh.getCarFollowModel().minNextSpeed(myVeh.getSpeed(), &myVeh));
    //     const double duration = MAX2(y, TS);
    //     solved = true;
    // #ifdef DEBUG_GLOSA
    //     if (DEBUG_COND) {
    //         std::cout << "  targetSpeed=" << targetSpeed << " duration=" << duration << "\n";
    //     }
    // #endif
    //     if (myIgnoreCFModel) {
    //         std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //         speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), myVeh.getSpeed()));
    //         speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(duration), targetSpeed));
    //         myVeh.getInfluencer().setSpeedTimeLine(speedTimeLine);
    //     } else {
    //         myVeh.setChosenSpeedFactor(targetSpeed / myVeh.getLane()->getSpeedLimit());
    //     }
    //     mySpeedAdviceActive = true;
    // }
    #[allow(unused_variables)]
    pub fn adapt_speed(&mut self, distance: f64, timeToJunction: f64, timeToSwitch: f64, solved: &mut bool) {
        // ensure that myVehicle arrives at the
        // junction with maximum speed when it switches to green
        // car performs a slowDown at time z to speed x for duration y
        // there are two basic strategies
        // a) maximize z -> this saves road space but leads to low x and thus excessive braking
        // b) maximize x -> this saves fuel but wastes road
        // c) compromise: b) but only when distance to junction is below a threshold

        let vMax = self.myVeh.get_lane().get_speed_limit() * self.myOriginalSpeedFactor;

        // need to start/continue maneuver
        let t = timeToSwitch;
        let a = self.myVeh.get_car_follow_model().get_max_accel();
        let u = self.myMinSpeed;
        let w = vMax;
        let s = distance;
        let v = self.myVeh.get_speed();
        // x : target speed
        // y : slow down duration
        // s is composed of 1 trapezoid (decel), 1 rectangle (maintain), 1 trapezoid (accel)
        // s = (v^2-x^2)/2d + x*(t-y-(w-x)/a) + (w^2-x^2)/2a
        // y = (v-x)/d
        // solution for x curtesy of mathomatic.org

        // First, we calculate targetSpeed assuming we are driving that speed already (v=x)
        // If this results in targetSpeed < currentSpeed, then we need to decelerate (and vice versa)
        let rootConst = a * a * t * t - 2.0 * a * w * t + 2. * a * s;
        let mut vConst = 0.;
        if rootConst >= 0. {
            vConst = rootConst.sqrt() - a * t + w;
        }
        let mut d = self.myVeh.get_car_follow_model().get_max_decel();
        if v < vConst {
            d = a;
        }

        // Second, we calculate the correct targetSpeed, knowing if we need to accelerate or decelerate
        let sign0 = -1.; // quadratic formula solution x1 (choose solution with higher speed)
        let root_argument = a * d * ((2.0 * d * (s - (w * t))) - ((v - w) * (v - w)) + (a * ((d * (t * t)) + (2.0 * (s - (t * v))))));
        if root_argument < 0. {
            // no reset of speedFactor because in this case, current speed can be kept
            *solved = true;
            return;
        }
        let x = ((a * (v - (d * t))) + (d * w) - sign0 * root_argument.sqrt()) / (d + a);
        let mut y = (v - x) / d;
        if v < x {
            y = (x - v) / d;
        }
        if s < (w * w - x * x) / 2.0 / a {
            // end maneuver
            if self.myIgnoreCFModel {
                let mut speedTimeLine: Vec<(SUMOTime, f64)> = Vec::new();
                speedTimeLine.push((MSNet::get_instance().get_current_time_step(), self.myVeh.get_speed()));
                speedTimeLine.push((MSNet::get_instance().get_current_time_step() + utils::common::time::time2steps(w - x / a), vMax));
                self.myVeh.m().get_influencer().set_speed_time_line(&speedTimeLine);
            } else {
                self.myVeh.m().set_chosen_speed_factor(self.myOriginalSpeedFactor);
            }
            self.mySpeedAdviceActive = false;
            return;
        }
        if !(x >= u && x <= w && y < t) {
            if x < u {
                // no reset of speedFactor because in this case, current speed can be kept
                //myVeh.setChosenSpeedFactor(myOriginalSpeedFactor);
                //mySpeedAdviceActive = false;
                *solved = true;
            }
            // if x > w: the vehicle can not go fast enough (debug output only)
            return;
        }
        let targetSpeed = max2(x, self.myVeh.get_car_follow_model().min_next_speed(self.myVeh.get_speed(), self.myVeh));
        let duration = max2(y, utils::common::time::ts());
        *solved = true;
        if self.myIgnoreCFModel {
            let mut speedTimeLine: Vec<(SUMOTime, f64)> = Vec::new();
            speedTimeLine.push((MSNet::get_instance().get_current_time_step(), self.myVeh.get_speed()));
            speedTimeLine.push((MSNet::get_instance().get_current_time_step() + utils::common::time::time2steps(duration), targetSpeed));
            self.myVeh.m().get_influencer().set_speed_time_line(&speedTimeLine);
        } else {
            self.myVeh.m().set_chosen_speed_factor(targetSpeed / self.myVeh.get_lane().get_speed_limit());
        }
        self.mySpeedAdviceActive = true;
    }

    /// C++: `void MSDevice_GLOSA::MSDevice_GLOSA(SUMOVehicle &, const std::string &, double, double, double, double, bool, bool, bool)` [private]
    /// Constructor
    // MSDevice_GLOSA::MSDevice_GLOSA(SUMOVehicle& holder, const std::string& id, double minSpeed, double range, double maxSpeedFactor,
    //                                double addSwitchTime, bool useQueue, bool overrideSafety, bool ignoreCFModel) :
    //     MSVehicleDevice(holder, id),
    //     myVeh(dynamic_cast<MSVehicle&>(holder)),
    //     myNextTLSLink(nullptr),
    //     myDistance(0),
    //     myMinSpeed(minSpeed),
    //     myRange(range),
    //     myMaxSpeedFactor(maxSpeedFactor),
    //     myAddSwitchTime(addSwitchTime),
    //     myOverrideSafety(overrideSafety),
    //     myIgnoreCFModel(ignoreCFModel),
    //     mySpeedAdviceActive(false),
    //     myUseQueue(useQueue)
    //
    // {
    //     myOriginalSpeedFactor = myVeh.getChosenSpeedFactor();
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, minSpeed: f64, range: f64, maxSpeedFactor: f64, addSwitchTime: f64, useQueue: bool, overrideSafety: bool, ignoreCFModel: bool) -> Self {
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_GLOSA>().m().notify_enter(veh, reason, enteredLane)
        }
        fn mr_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_GLOSA>().m().notify_move(veh, oldPos, newPos, newSpeed)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: mr_notify_move,
            notify_leave: crate::microsim::ms_move_reminder::default_notify_leave,
        };
        let mut this = Self::new_device(holder, id, minSpeed, range, maxSpeedFactor, addSwitchTime, useQueue, overrideSafety, ignoreCFModel);
        this.base.base.vt = &MS_MOVE_REMINDER_VT;
        this
    }

    fn new_device(holder: &mut SUMOVehicle, id: &str, minSpeed: f64, range: f64, maxSpeedFactor: f64, addSwitchTime: f64, useQueue: bool, overrideSafety: bool, ignoreCFModel: bool) -> Self {
        let mut b = MSDevice_GLOSA {
            myVeh: Ptr::from_mut(holder).cast::<MSVehicle>(),
            base: MSVehicleDevice::new(holder, id),
            myNextTLSLink: Ptr::null(),
            myDistance: 0.,
            myMinSpeed: minSpeed,
            myRange: range,
            myMaxSpeedFactor: maxSpeedFactor,
            myAddSwitchTime: addSwitchTime,
            myOverrideSafety: overrideSafety,
            myIgnoreCFModel: ignoreCFModel,
            myOriginalSpeedFactor: 0.,
            mySpeedAdviceActive: false,
            myUseQueue: useQueue,
        };
        b.myOriginalSpeedFactor = b.myVeh.get_chosen_speed_factor();
        b
    }

    /// C++: `void MSDevice_GLOSA::MSDevice_GLOSA(const MSDevice_GLOSA &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_GLOSA) -> Self {
        panic!("MSDevice_GLOSA copy constructor is private and invalid")
    }

    /// C++: `MSDevice_GLOSA & MSDevice_GLOSA::operator=(const MSDevice_GLOSA &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_GLOSA) -> &mut MSDevice_GLOSA {
        panic!("MSDevice_GLOSA assignment operator is private and invalid")
    }

}

