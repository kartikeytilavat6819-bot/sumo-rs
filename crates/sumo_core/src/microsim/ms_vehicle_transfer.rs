//! Port of microsim/MSVehicleTransfer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::xml::sumo_xml_definitions::{SUMO_ATTR_DEPART, SUMO_ATTR_ID, SUMO_ATTR_JUMP, SUMO_ATTR_PARKING, SUMO_TAG_VEHICLETRANSFER};

// C++ static members, ported as module-level items
pub const MSVehicleTransfer_TeleportMinSpeed: f64 = 1.0;
thread_local! {
    static MSVehicleTransfer_MY_INSTANCE: std::cell::Cell<Ptr<MSVehicleTransfer>> = const { std::cell::Cell::new(Ptr::null()) };
}

/// This object (each simulation owns exactly one) is responsible for the transfer of vehicles that got stuck within the network due to grid locks. It also manages vehicles that are removed from the network because of stops with the parking attribute.
/// C++ class `MSVehicleTransfer` (microsim/MSVehicleTransfer.h)
pub struct MSVehicleTransfer {
    /// The information about stored vehicles to move virtually
    // Unreachable here: MFXSynchQue is only used when gNumSimThreads > 1, and --threads > 1 is rejected at startup (ms_frame.rs); a plain Vec is the serial behaviour
    pub myVehicles: Vec<MSVehicleTransfer_VehicleInformation>,
}

// C++ static members of MSVehicleTransfer (port as module-level statics, see AGENTS.md):
//   static const double MSVehicleTransfer::TeleportMinSpeed;
//   static MSVehicleTransfer * MSVehicleTransfer::myInstance;

impl MSVehicleTransfer {
    /// C++: `void MSVehicleTransfer::~MSVehicleTransfer()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // myInstance = nullptr; the myVehicles Vec drops with the struct
        MSVehicleTransfer_MY_INSTANCE.with(|cell| cell.set(Ptr::null()));
    }

    /// C++: `void MSVehicleTransfer::add(const SUMOTime, MSVehicle *)`
    /// Adds a vehicle to this transfer object
    // void
    // MSVehicleTransfer::add(const SUMOTime t, MSVehicle* veh) {
    //     const bool jumping = veh->isJumping();
    //     const SUMOTime proceed = jumping ? MAX2(t + veh->getPastStops().back().jump, veh->getPastStops().back().jumpUntil) : -1;
    //     if (veh->isParking()) {
    //         veh->getLaneChangeModel().endLaneChangeManeuver(MSMoveReminder::NOTIFICATION_PARKING);
    //         MSNet::getInstance()->informVehicleStateListener(veh, MSNet::VehicleState::STARTING_PARKING);
    //         veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_PARKING);
    //     } else {
    //         veh->getLaneChangeModel().endLaneChangeManeuver(MSMoveReminder::NOTIFICATION_TELEPORT);
    //         MSNet::getInstance()->informVehicleStateListener(veh, MSNet::VehicleState::STARTING_TELEPORT);
    //         if (veh->succEdge(1) == nullptr) {
    //             WRITE_WARNINGF(TL("Vehicle '%' teleports beyond arrival edge '%', time=%."), veh->getID(), veh->getEdge()->getID(), time2string(t));
    //             veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED);
    //             MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //             return;
    //         }
    //         veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_TELEPORT);
    //         veh->enterLaneAtMove(veh->succEdge(1)->getLanes()[0], true);
    //     }
    //     myVehicles.push_back(VehicleInformation(t, veh, proceed, veh->isParking(), jumping));
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, t: SUMOTime, veh: Ptr<MSVehicle>) {
        let jumping = veh.is_jumping();
        let proceed: SUMOTime = if jumping {
            let last = veh.get_past_stops().last().unwrap();
            std::cmp::max(t + last.jump, last.jumpUntil)
        } else {
            -1
        };
        if veh.is_parking() {
            veh.m().get_lane_change_model().end_lane_change_maneuver(MSMoveReminder_Notification::NOTIFICATION_PARKING);
            MSNet::get_instance().m().inform_vehicle_state_listener(Ptr::from_mut(&mut veh.m().base.base), MSNet_VehicleState::STARTING_PARKING, "");
            veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_PARKING);
        } else {
            veh.m().get_lane_change_model().end_lane_change_maneuver(MSMoveReminder_Notification::NOTIFICATION_TELEPORT);
            MSNet::get_instance().m().inform_vehicle_state_listener(Ptr::from_mut(&mut veh.m().base.base), MSNet_VehicleState::STARTING_TELEPORT, "");
            if veh.succ_edge(1).is_null() {
                write_warning(&format!("Vehicle '{}' teleports beyond arrival edge '{}', time={}.", veh.get_id(), veh.get_edge().get_id(), time2string(t)));
                veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED);
                MSNet::get_instance().m().get_vehicle_control().schedule_vehicle_removal(Ptr::from_mut(&mut veh.m().base.base), false);
                return;
            }
            veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_TELEPORT);
            let lane0 = veh.succ_edge(1).get_lanes()[0];
            veh.m().enter_lane_at_move(lane0, true);
        }
        let parking = veh.is_parking();
        self.myVehicles.push(MSVehicleTransfer_VehicleInformation::new(t, veh, proceed, parking, jumping));
    }

    /// C++: `void MSVehicleTransfer::remove(MSVehicle *)`
    /// Remove a vehicle from this transfer object
    // void
    // MSVehicleTransfer::remove(MSVehicle* veh) {
    //     auto& vehInfos = myVehicles.getContainer();
    //     for (auto i = vehInfos.begin(); i != vehInfos.end(); ++i) {
    //         if (i->myVeh == veh) {
    //             if (i->myParking) {
    //                 veh->getMutableLane()->removeParking(veh);
    //             }
    //             vehInfos.erase(i);
    //             break;
    //         }
    //     }
    //     myVehicles.unlock();
    // }
    #[allow(unused_variables)]
    pub fn remove(&mut self, veh: Ptr<MSVehicle>) {
        if let Some(pos) = self.myVehicles.iter().position(|i| i.myVeh == veh) {
            if self.myVehicles[pos].myParking {
                veh.m().get_mutable_lane().m().remove_parking(Ptr::from_mut(&mut veh.m().base));
            }
            self.myVehicles.remove(pos);
        }
    }

    /// C++: `void MSVehicleTransfer::checkInsertions(SUMOTime)`
    /// Checks "movement" of stored vehicles
    // void
    // MSVehicleTransfer::checkInsertions(SUMOTime time) {
    //     // go through vehicles
    //     auto& vehInfos = myVehicles.getContainer();
    //     std::sort(vehInfos.begin(), vehInfos.end());
    //     for (auto i = vehInfos.begin(); i != vehInfos.end();) {
    //         // vehicle information cannot be const because we need to assign the proceed time
    //         VehicleInformation& desc = *i;
    //
    //         if (desc.myParking) {
    //             // handle parking vehicles
    //             if (time != desc.myTransferTime) {
    //                 // avoid calling processNextStop twice in the transfer step
    //                 const MSLane* lane = desc.myVeh->getLane();
    //                 // lane must be locked because pedestrians may be added in during stop processing while existing passengers are being drawn simultaneously
    //                 if (lane != nullptr) {
    //                     lane->getVehiclesSecure();
    //                 }
    //                 desc.myVeh->processNextStop(1);
    //                 desc.myVeh->updateParkingState();
    //                 if (lane != nullptr) {
    //                     lane->releaseVehicles();
    //                 }
    //             }
    //             if (desc.myVeh->keepStopping(true)) {
    //                 i++;
    //                 continue;
    //             }
    //             // parking finished, head back into traffic
    //         }
    //         const SUMOVehicleClass vclass = desc.myVeh->getVehicleType().getVehicleClass();
    //         const MSEdge* e = desc.myVeh->getEdge();
    //         const MSEdge* nextEdge = desc.myVeh->succEdge(1);
    //
    //
    //         if (desc.myParking) {
    //             MSParkingArea* pa = desc.myVeh->getCurrentParkingArea();
    //             const double departPos = pa != nullptr ? pa->getInsertionPosition(*desc.myVeh) : desc.myVeh->getPositionOnLane();
    //             // handle parking vehicles
    //             desc.myVeh->setIdling(true);
    //             if (desc.myVeh->getMutableLane()->isInsertionSuccess(desc.myVeh, 0, departPos, desc.myVeh->getLateralPositionOnLane(),
    //                     false, MSMoveReminder::NOTIFICATION_PARKING)) {
    //                 MSNet::getInstance()->informVehicleStateListener(desc.myVeh, MSNet::VehicleState::ENDING_PARKING);
    //                 desc.myVeh->getMutableLane()->removeParking(desc.myVeh);
    //                 // at this point we are in the lane, blocking traffic & if required we configure the exit manoeuvre
    //                 if (MSGlobals::gModelParkingManoeuver && desc.myVeh->setExitManoeuvre()) {
    //                     MSNet::getInstance()->informVehicleStateListener(desc.myVeh, MSNet::VehicleState::MANEUVERING);
    //                 }
    //                 desc.myVeh->setIdling(false);
    //                 i = vehInfos.erase(i);
    //             } else {
    //                 // blocked from entering the road - engine assumed to be idling.
    //                 desc.myVeh->workOnIdleReminders();
    //                 if (!desc.myVeh->signalSet(MSVehicle::VEH_SIGNAL_BLINKER_LEFT | MSVehicle::VEH_SIGNAL_BLINKER_RIGHT)) {
    //                     // signal wish to re-enter the road
    //                     desc.myVeh->switchOnSignal(MSGlobals::gLefthand ? MSVehicle::VEH_SIGNAL_BLINKER_RIGHT : MSVehicle::VEH_SIGNAL_BLINKER_LEFT);
    //                     if (pa) {
    //                         // update freePosition so other vehicles can help with insertion
    //                         desc.myVeh->getCurrentParkingArea()->notifyEgressBlocked();
    //                     }
    //                 }
    //                 i++;
    //             }
    //         } else if (desc.myJumping && desc.myProceedTime > time) {
    //             ++i;
    //         } else {
    //             const double departPos = 0;
    //             // get the lane on which this vehicle should continue
    //             // first select all the lanes which allow continuation onto nextEdge
    //             //   then pick the one which is least occupied
    //             MSLane* l = (nextEdge != nullptr ? e->getFreeLane(e->allowedLanes(*nextEdge, vclass), vclass, departPos) :
    //                          e->getFreeLane(nullptr, vclass, departPos));
    //             // handle teleporting vehicles, lane may be 0 because permissions were modified by a closing rerouter or TraCI
    //             const bool busyBidi = l != nullptr && l->getBidiLane() != nullptr && l->getBidiLane()->getVehicleNumberWithPartials() > 0;
    //             if (l != nullptr && !busyBidi && l->freeInsertion(*(desc.myVeh), MIN2(l->getSpeedLimit(), desc.myVeh->getMaxSpeed()), 0, MSMoveReminder::NOTIFICATION_TELEPORT)) {
    //                 if (!desc.myJumping) {
    //                     WRITE_WARNINGF(TL("Vehicle '%' ends teleporting on edge '%', time=%."), desc.myVeh->getID(), e->getID(), time2string(time));
    //                 }
    //                 MSNet::getInstance()->informVehicleStateListener(desc.myVeh, MSNet::VehicleState::ENDING_TELEPORT);
    //                 i = vehInfos.erase(i);
    //             } else if (desc.myJumping) {
    //                 // try again later
    //                 ++i;
    //             } else {
    //                 // vehicle is visible while show-route is active. Make its state more obvious
    //                 desc.myVeh->computeAngle();
    //                 desc.myVeh->setLateralPositionOnLane(-desc.myVeh->getLane()->getWidth() / 2);
    //                 desc.myVeh->invalidateCachedPosition();
    //                 // could not insert. maybe we should proceed in virtual space
    //                 if (desc.myProceedTime < 0) {
    //                     // initialize proceed time (delayed to avoid lane-order dependency in executeMove)
    //                     desc.myProceedTime = time + TIME2STEPS(e->getCurrentTravelTime(TeleportMinSpeed));
    //                 } else if (desc.myProceedTime < time) {
    //                     if (desc.myVeh->succEdge(1) == nullptr) {
    //                         if (desc.myVeh->getDevice(typeid(MSDevice_Taxi)) != nullptr) {
    //                             // never teleport a taxi beyond the end of it's route
    //                             desc.myProceedTime = time + TIME2STEPS(e->getCurrentTravelTime(TeleportMinSpeed));
    //                         } else {
    //                             WRITE_WARNINGF(TL("Vehicle '%' teleports beyond arrival edge '%', time=%."), desc.myVeh->getID(), e->getID(), time2string(time));
    //                             desc.myVeh->leaveLane(MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED);
    //                             MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(desc.myVeh);
    //                             i = vehInfos.erase(i);
    //                         }
    //                         continue;
    //                     }
    //                     // let the vehicle move to the next edge
    //                     desc.myVeh->leaveLane(MSMoveReminder::NOTIFICATION_TELEPORT_CONTINUATION);
    //                     // active move reminders (i.e. rerouters)
    //                     const std::vector<MSLane*>* allowedLanes = nextEdge->allowedLanes(vclass);
    //                     MSLane* laneToEnter = (allowedLanes != nullptr) ? allowedLanes->at(0) : nextEdge->getLanes()[0];
    //                     desc.myVeh->enterLaneAtMove(laneToEnter, true);
    //                     // use current travel time to determine when to move the vehicle forward
    //                     desc.myProceedTime = time + TIME2STEPS(e->getCurrentTravelTime(TeleportMinSpeed));
    //                 }
    //                 ++i;
    //             }
    //         }
    //     }
    //     myVehicles.unlock();
    // }
    #[allow(unused_variables)]
    pub fn check_insertions(&mut self, time: SUMOTime) {
        // determinism: C++ std::sort is unstable; stable sort_by is used here
        self.myVehicles.sort_by(|a, b| {
            if a.lt_op(b) {
                std::cmp::Ordering::Less
            } else if b.lt_op(a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let mut i: usize = 0;
        while i < self.myVehicles.len() {
            // vehicle information cannot be const because we need to assign the proceed time
            let desc = &mut self.myVehicles[i];

            if desc.myParking {
                // handle parking vehicles
                if time != desc.myTransferTime {
                    // avoid calling processNextStop twice in the transfer step
                    let lane = desc.myVeh.get_lane();
                    // lane must be locked because pedestrians may be added in during stop processing while existing passengers are being drawn simultaneously
                    if !lane.is_null() {
                        lane.get_vehicles_secure();
                    }
                    desc.myVeh.m().process_next_stop(1.0);
                    desc.myVeh.m().update_parking_state();
                    if !lane.is_null() {
                        lane.release_vehicles();
                    }
                }
                if desc.myVeh.keep_stopping(true) {
                    i += 1;
                    continue;
                }
                // parking finished, head back into traffic
            }
            let vclass = desc.myVeh.get_vehicle_type().get_vehicle_class();
            let e = desc.myVeh.get_edge();
            let nextEdge = desc.myVeh.succ_edge(1);

            if desc.myParking {
                let pa = desc.myVeh.get_current_parking_area();
                let departPos = if !pa.is_null() {
                    pa.get_insertion_position(&desc.myVeh.base.base)
                } else {
                    desc.myVeh.get_position_on_lane()
                };
                // handle parking vehicles
                desc.myVeh.m().set_idling(true);
                let lane = desc.myVeh.get_mutable_lane();
                if lane.m().is_insertion_success(desc.myVeh, 0.0, departPos, desc.myVeh.get_lateral_position_on_lane(), false, MSMoveReminder_Notification::NOTIFICATION_PARKING) {
                    MSNet::get_instance().m().inform_vehicle_state_listener(Ptr::from_mut(&mut desc.myVeh.m().base.base), MSNet_VehicleState::ENDING_PARKING, "");
                    lane.m().remove_parking(Ptr::from_mut(&mut desc.myVeh.m().base));
                    // at this point we are in the lane, blocking traffic & if required we configure the exit manoeuvre
                    let gModelParkingManoeuver = crate::microsim::ms_globals::model_parking_manoeuver();
                    if gModelParkingManoeuver && desc.myVeh.m().set_exit_manoeuvre() {
                        MSNet::get_instance().m().inform_vehicle_state_listener(Ptr::from_mut(&mut desc.myVeh.m().base.base), MSNet_VehicleState::MANEUVERING, "");
                    }
                    desc.myVeh.m().set_idling(false);
                    self.myVehicles.remove(i);
                } else {
                    // blocked from entering the road - engine assumed to be idling.
                    desc.myVeh.m().work_on_idle_reminders();
                    if !desc.myVeh.signal_set(MSVehicle_Signalling::VEH_SIGNAL_BLINKER_LEFT as i32 | MSVehicle_Signalling::VEH_SIGNAL_BLINKER_RIGHT as i32) {
                        // signal wish to re-enter the road
                        let gLefthand = crate::microsim::ms_globals::lefthand();
                        desc.myVeh.m().switch_on_signal(if gLefthand { MSVehicle_Signalling::VEH_SIGNAL_BLINKER_RIGHT as i32 } else { MSVehicle_Signalling::VEH_SIGNAL_BLINKER_LEFT as i32 });
                        if !pa.is_null() {
                            // update freePosition so other vehicles can help with insertion
                            pa.m().notify_egress_blocked();
                        }
                    }
                    i += 1;
                }
            } else if desc.myJumping && desc.myProceedTime > time {
                i += 1;
            } else {
                let departPos = 0.0;
                // get the lane on which this vehicle should continue
                // first select all the lanes which allow continuation onto nextEdge
                //   then pick the one which is least occupied
                let l: Ptr<MSLane> = if !nextEdge.is_null() {
                    e.get_free_lane(e.allowed_lanes(&nextEdge, vclass, false), vclass, departPos)
                } else {
                    e.get_free_lane(Ptr::null(), vclass, departPos)
                };
                // handle teleporting vehicles, lane may be 0 because permissions were modified by a closing rerouter or TraCI
                let busyBidi = !l.is_null() && !l.get_bidi_lane().is_null() && l.get_bidi_lane().get_vehicle_number_with_partials() > 0;
                if !l.is_null() && !busyBidi && l.m().free_insertion(desc.myVeh.m(), l.get_speed_limit().min(desc.myVeh.get_max_speed()), 0.0, MSMoveReminder_Notification::NOTIFICATION_TELEPORT) {
                    if !desc.myJumping {
                        write_warning(&format!("Vehicle '{}' ends teleporting on edge '{}', time={}.", desc.myVeh.get_id(), e.get_id(), time2string(time)));
                    }
                    MSNet::get_instance().m().inform_vehicle_state_listener(Ptr::from_mut(&mut desc.myVeh.m().base.base), MSNet_VehicleState::ENDING_TELEPORT, "");
                    self.myVehicles.remove(i);
                } else if desc.myJumping {
                    // try again later
                    i += 1;
                } else {
                    // vehicle is visible while show-route is active. Make its state more obvious
                    let _ = desc.myVeh.compute_angle();
                    let halfWidth = desc.myVeh.get_lane().get_width() / 2.0;
                    desc.myVeh.m().set_lateral_position_on_lane(-halfWidth);
                    desc.myVeh.m().invalidate_cached_position();
                    // could not insert. maybe we should proceed in virtual space
                    if desc.myProceedTime < 0 {
                        // initialize proceed time (delayed to avoid lane-order dependency in executeMove)
                        desc.myProceedTime = time + time2steps(e.get_current_travel_time(MSVehicleTransfer_TeleportMinSpeed));
                    } else if desc.myProceedTime < time {
                        if desc.myVeh.succ_edge(1).is_null() {
                            let isTaxi = !desc.myVeh.get_device("taxi").is_null();
                            if isTaxi {
                                // never teleport a taxi beyond the end of it's route
                                desc.myProceedTime = time + time2steps(e.get_current_travel_time(MSVehicleTransfer_TeleportMinSpeed));
                            } else {
                                write_warning(&format!("Vehicle '{}' teleports beyond arrival edge '{}', time={}.", desc.myVeh.get_id(), e.get_id(), time2string(time)));
                                desc.myVeh.m().leave_lane(MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED, Ptr::null());
                                MSNet::get_instance().m().get_vehicle_control().schedule_vehicle_removal(Ptr::from_mut(&mut desc.myVeh.m().base.base), false);
                                self.myVehicles.remove(i);
                            }
                            continue;
                        }
                        // let the vehicle move to the next edge
                        desc.myVeh.m().leave_lane(MSMoveReminder_Notification::NOTIFICATION_TELEPORT_CONTINUATION, Ptr::null());
                        // active move reminders (i.e. rerouters)
                        let allowedLanes = nextEdge.allowed_lanes_2(vclass);
                        let laneToEnter = if !allowedLanes.is_null() { allowedLanes.m()[0] } else { nextEdge.get_lanes()[0] };
                        desc.myVeh.m().enter_lane_at_move(laneToEnter, true);
                        // use current travel time to determine when to move the vehicle forward
                        desc.myProceedTime = time + time2steps(e.get_current_travel_time(MSVehicleTransfer_TeleportMinSpeed));
                    }
                    i += 1;
                }
            }
        }
    }

    /// C++: `void MSVehicleTransfer::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSVehicleTransfer::saveState(OutputDevice& out) {
    //     for (const VehicleInformation& vehInfo : myVehicles.getContainer()) {
    //         out.openTag(SUMO_TAG_VEHICLETRANSFER);
    //         out.writeAttr(SUMO_ATTR_ID, vehInfo.myVeh->getID());
    //         out.writeAttr(SUMO_ATTR_DEPART, vehInfo.myProceedTime);
    //         if (vehInfo.myParking) {
    //             out.writeAttr(SUMO_ATTR_PARKING, vehInfo.myVeh->getLane()->getID());
    //         }
    //         if (vehInfo.myJumping) {
    //             out.writeAttr(SUMO_ATTR_JUMP, true);
    //         }
    //         out.closeTag();
    //     }
    //     myVehicles.unlock();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        for vehInfo in self.myVehicles.iter() {
            out.open_tag_id(SUMO_TAG_VEHICLETRANSFER);
            out.write_attr(SUMO_ATTR_ID, vehInfo.myVeh.get_id());
            out.write_attr(SUMO_ATTR_DEPART, &vehInfo.myProceedTime);
            if vehInfo.myParking {
                out.write_attr(SUMO_ATTR_PARKING, vehInfo.myVeh.get_lane().get_id());
            }
            if vehInfo.myJumping {
                out.write_attr(SUMO_ATTR_JUMP, &true);
            }
            out.close_tag("");
        }
    }

    /// C++: `void MSVehicleTransfer::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MSVehicleTransfer::clearState() {
    //     myVehicles.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myVehicles.clear();
    }

    /// C++: `void MSVehicleTransfer::loadState(const SUMOSAXAttributes &, const SUMOTime, MSVehicleControl &)`
    /// Loads one transfer vehicle state from the given description
    // void
    // MSVehicleTransfer::loadState(const SUMOSAXAttributes& attrs, const SUMOTime offset, MSVehicleControl& vc) {
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vc.getVehicle(attrs.getString(SUMO_ATTR_ID)));
    //     if (veh == nullptr) {
    //         // deleted
    //         return;
    //     }
    //     SUMOTime proceedTime = (SUMOTime)attrs.getLong(SUMO_ATTR_DEPART);
    //     MSLane* parkingLane = attrs.hasAttribute(SUMO_ATTR_PARKING) ? MSLane::dictionary(attrs.getString(SUMO_ATTR_PARKING)) : nullptr;
    //     bool ok = true;
    //     const bool jumping = attrs.getOpt<bool>(SUMO_ATTR_JUMP, veh->getID().c_str(), ok, false);
    //     myVehicles.push_back(VehicleInformation(-1, veh, proceedTime - offset, parkingLane != nullptr, jumping));
    //     if (parkingLane != nullptr) {
    //         parkingLane->addParking(veh);
    //         veh->setTentativeLaneAndPosition(parkingLane, veh->getPositionOnLane());
    //         veh->processNextStop(veh->getSpeed());
    //     }
    //     MSNet::getInstance()->getInsertionControl().alreadyDeparted(veh);
    // }
    #[allow(unused_variables)]
    // Returns Result: C++ attrs.getLong() throws ProcessError, which must propagate to the SAX handler.
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes, offset: SUMOTime, vc: &mut MSVehicleControl) -> Result<(), ProcessError> {
        // shortcut: dynamic_cast<MSVehicle*> has no runtime check here; a null result still means "deleted" as in C++
        let veh: Ptr<MSVehicle> = vc.get_vehicle(attrs.get_string(SUMO_ATTR_ID).as_str()).cast::<MSVehicle>();
        if veh.is_null() {
            // deleted
            return Ok(());
        }
        let proceedTime: SUMOTime = attrs.get_long(SUMO_ATTR_DEPART)? as SUMOTime;
        let parkingLane: Ptr<MSLane> = if attrs.has_attribute(SUMO_ATTR_PARKING) { MSLane::dictionary_2(attrs.get_string(SUMO_ATTR_PARKING).as_str()) } else { Ptr::null() };
        let mut ok = true;
        let jumping: bool = attrs.get_opt::<bool>(SUMO_ATTR_JUMP, veh.get_id(), &mut ok, false, true);
        self.myVehicles.push(MSVehicleTransfer_VehicleInformation::new(-1, veh, proceedTime - offset, !parkingLane.is_null(), jumping));
        if !parkingLane.is_null() {
            parkingLane.m().add_parking(veh.cast::<MSBaseVehicle>());
            veh.m().set_tentative_lane_and_position(parkingLane, veh.get_position_on_lane(), 0.0);
            veh.m().process_next_stop(veh.get_speed());
        }
        MSNet::get_instance().m().get_insertion_control().already_departed(veh.cast::<SUMOVehicle>());
        Ok(())
    }

    /// C++: `MSVehicleTransfer * MSVehicleTransfer::getInstance()` [static]
    /// Returns the instance of this object
    // MSVehicleTransfer*
    // MSVehicleTransfer::getInstance() {
    //     if (myInstance == nullptr) {
    //         myInstance = new MSVehicleTransfer();
    //     }
    //     return myInstance;
    // }
    #[allow(unused_variables)]
    pub fn get_instance() -> Ptr<MSVehicleTransfer> {
        // shortcut: no MSGlobals/ownership teardown, the instance is leaked like the C++ singleton
        MSVehicleTransfer_MY_INSTANCE.with(|cell| {
            if cell.get().is_null() {
                cell.set(Ptr::from_mut(Box::leak(Box::new(MSVehicleTransfer::new()))));
            }
            cell.get()
        })
    }

    /// C++: `void MSVehicleTransfer::MSVehicleTransfer()` [private]
    /// Constructor
    // MSVehicleTransfer::MSVehicleTransfer() : myVehicles(MSGlobals::gNumSimThreads > 1) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            // Unreachable here: MFXSynchQue (used only when gNumSimThreads > 1) is not needed while --threads > 1 is rejected at startup
            myVehicles: Vec::new(),
        }
    }

}

/// Holds the information needed to move the vehicle over the network
/// C++ class `MSVehicleTransfer::VehicleInformation` (microsim/MSVehicleTransfer.h)
pub struct MSVehicleTransfer_VehicleInformation {
    /// the time at which this vehicle was removed from the network
    pub myTransferTime: SUMOTime,
    /// The vehicle itself
    pub myVeh: Ptr<MSVehicle>,
    /// The time at which the vehicle should be moved virtually one edge further
    pub myProceedTime: SUMOTime,
    /// whether the vehicle is or was parking
    pub myParking: bool,
    /// whether the vehicle is or was jumping
    pub myJumping: bool,
}

impl MSVehicleTransfer_VehicleInformation {
    /// C++: `void VehicleInformation::VehicleInformation(SUMOTime, MSVehicle *, SUMOTime, bool, bool)`
    /// Constructor
    // VehicleInformation(SUMOTime t, MSVehicle* veh, SUMOTime proceedTime, bool parking, bool jumping)
    //             : myTransferTime(t), myVeh(veh), myProceedTime(proceedTime), myParking(parking), myJumping(jumping) { }
    #[allow(unused_variables)]
    pub fn new(t: SUMOTime, veh: Ptr<MSVehicle>, proceedTime: SUMOTime, parking: bool, jumping: bool) -> Self {
        Self {
            myTransferTime: t,
            myVeh: veh,
            myProceedTime: proceedTime,
            myParking: parking,
            myJumping: jumping,
        }
    }

    /// C++: `bool VehicleInformation::operator<(const VehicleInformation &) const` [const]
    /// sort by vehicle ID for repeatable parallel simulation
    // bool
    // MSVehicleTransfer::VehicleInformation::operator<(const VehicleInformation& v2) const {
    //     return myVeh->getNumericalID() < v2.myVeh->getNumericalID();
    // }
    #[allow(unused_variables)]
    pub fn lt_op(&self, v2: &MSVehicleTransfer_VehicleInformation) -> bool {
        self.myVeh.get_numerical_id() < v2.myVeh.get_numerical_id()
    }

}

