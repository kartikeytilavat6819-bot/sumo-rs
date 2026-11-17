//! Port of microsim/devices/MSDevice_Transportable.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::time::{delta_t, time2steps, time2string};

/// C++ `dynamic_cast<MSStageDriving*>(stage)`
// shortcut: the stage hierarchy has no runtime downcast yet (MSStage carries no subtype tag), so no stage is
// treated as MSStageDriving; passenger boarding/unboarding through this path is inert until the downcast exists
fn stage_driving(stage: Ptr<MSStage>) -> Option<Ptr<MSStageDriving>> {
    let _ = stage;
    None
}

/// C++ `dynamic_cast<MSBaseVehicle*>(...)` on a SUMOVehicle (the base is embedded, so this is an upcast of a container)
// shortcut: SUMOVehicle -> MSBaseVehicle downcast is not expressible yet; returns null, so the join-vehicle and
// changeAttached paths are skipped until the vehicle hierarchy provides the downcast
fn base_vehicle_of(veh: Ptr<SUMOVehicle>) -> Ptr<MSBaseVehicle> {
    let _ = veh;
    Ptr::default()
}

/// C++ `static_cast<MSDevice_Taxi*>(veh.getDevice(typeid(MSDevice_Taxi)))`
fn taxi_device_of(veh: Ptr<SUMOVehicle>) -> Ptr<MSDevice_Taxi> {
    MSDevice::get_device_by_name(&veh.cast::<MSVehicle>().m().myDevices, "taxi").cast::<MSDevice_Taxi>()
}

/// C++ `istringstream >> x`: whitespace token parsed as T, default on failure
fn next_as<T: std::str::FromStr + Default>(bis: &mut std::str::SplitWhitespace) -> T {
    bis.next().and_then(|s| s.parse::<T>().ok()).unwrap_or_default()
}

/// C++ class `MSDevice_Transportable` (microsim/devices/MSDevice_Transportable.h)
// repr(C): the base sits at offset 0 so device pointers can be upcast to MSVehicleDevice (C++ implicit upcast)
#[repr(C)]
pub struct MSDevice_Transportable {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// Whether it is a container device
    pub myAmContainer: bool,
    /// The passengers of the vehicle
    pub myTransportables: Vec<Ptr<MSTransportable>>,
    pub myStopped: bool,
    /// unboarding positions of passengers if vehicle is a train
    pub myUnboardingPositions: Vec<Position>,
    /// the original type to set when nothing is being transported
    pub myOriginalType: Ptr<MSVehicleType>,
    /// the type to consult if loading/unloading changes vehicle properties
    pub myLoadedType: Ptr<MSVehicleType>,
}

impl std::ops::Deref for MSDevice_Transportable {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Transportable {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_Transportable {
    /// C++: `MSDevice_Transportable * MSDevice_Transportable::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &, const bool)` [static]
    /// Build devices for the given vehicle, if needed
    // MSDevice_Transportable*
    // MSDevice_Transportable::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into, const bool isContainer) {
    //     MSDevice_Transportable* device = new MSDevice_Transportable(v, isContainer ? "container_" + v.getID() : "person_" + v.getID(), isContainer);
    //     into.push_back(device);
    //     return device;
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>, isContainer: bool) -> Ptr<MSDevice_Transportable> {
        let id = if isContainer { format!("container_{}", v.get_id()) } else { format!("person_{}", v.get_id()) };
        let device = Ptr::from_mut(Box::leak(Box::new(MSDevice_Transportable::new(v, &id, isContainer))));
        into.push(Ptr::from_raw(device.as_raw() as *mut MSVehicleDevice));
        device
    }

    /// C++: `void MSDevice_Transportable::~MSDevice_Transportable() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Transportable::~MSDevice_Transportable() {
    //     // flush any unfortunate riders still remaining
    //     for (auto it = myTransportables.begin(); it != myTransportables.end();) {
    //         MSTransportable* transportable = *it;
    //         WRITE_WARNING((myAmContainer ? "Removing container '" : "Removing person '") + transportable->getID() +
    //                       "' at removal of vehicle '" + myHolder.getID() + "'");
    //         MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(transportable->getCurrentStage());
    //         if (stage != nullptr) {
    //             stage->setVehicle(nullptr);
    //         }
    //         if (myAmContainer) {
    //             MSNet::getInstance()->getContainerControl().erase(transportable);
    //         } else {
    //             MSNet::getInstance()->getPersonControl().erase(transportable);
    //         }
    //         it = myTransportables.erase(it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // flush any unfortunate riders still remaining
        while let Some(&transportable) = self.myTransportables.first() {
            let kind = if self.myAmContainer { "Removing container '" } else { "Removing person '" };
            write_warning(&format!("{}{}' at removal of vehicle '{}'", kind, transportable.get_id(), self.myHolder.get_id()));
            if let Some(stage) = stage_driving(transportable.get_current_stage()) {
                stage.m().set_vehicle(Ptr::default());
            }
            if self.myAmContainer {
                MSNet::get_instance().m().get_container_control().erase(transportable);
            } else {
                MSNet::get_instance().m().get_person_control().erase(transportable);
            }
            self.myTransportables.remove(0);
        }
    }

    /// C++: `bool MSDevice_Transportable::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks whether the vehicle is at a stop and transportable action is needed.
    // bool
    // MSDevice_Transportable::notifyMove(SUMOTrafficObject& /*tObject*/, double /*oldPos*/, double newPos, double newSpeed) {
    //     SUMOVehicle& veh = myHolder;
    //     const SUMOTime currentTime = MSNet::getInstance()->getCurrentTimeStep();
    //     if (myStopped) {
    //         if (!veh.isStopped()) {
    //             const SUMOTime freeFlowTimeCorrection = MSGlobals::gUseMesoSim ? TIME2STEPS(newPos / newSpeed) : 0;
    //             for (MSTransportable* const transportable : myTransportables) {
    //                 transportable->setDeparted(currentTime - freeFlowTimeCorrection);
    //             }
    //             myStopped = false;
    //         }
    //     } else {
    //         if (veh.isStopped()) {
    //             myStopped = true;
    //             MSStop& stop = veh.getNextStopMutable();
    //             const MSVehicle* joinVeh = dynamic_cast<MSVehicle*>(MSNet::getInstance()->getVehicleControl().getVehicle(stop.pars.join));
    //             const SUMOTime boardingDuration = veh.getVehicleType().getLoadingDuration(!myAmContainer);
    //             int numUnloaded = 0;
    //             for (std::vector<MSTransportable*>::iterator i = myTransportables.begin(); i != myTransportables.end();) {
    //                 MSTransportable* transportable = *i;
    //                 MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(transportable->getCurrentStage());
    //                 if (stage->canLeaveVehicle(transportable, myHolder, stop) && !willTransferAtJoin(transportable, joinVeh)) {
    //                     SUMOTime& timeForNext = myAmContainer ? stop.timeToLoadNextContainer : stop.timeToBoardNextPerson;
    //                     MSDevice_Taxi* taxiDevice = static_cast<MSDevice_Taxi*>(myHolder.getDevice(typeid(MSDevice_Taxi)));
    //                     if (taxiDevice != nullptr && timeForNext == 0 && !MSGlobals::gUseMesoSim) {
    //                         // taxi passengers must leave at the end of the stop duration
    //                         timeForNext = stop.pars.started + stop.pars.duration;
    //                     }
    //                     if (timeForNext - DELTA_T > currentTime) {
    //                         // try deboarding again in the next step
    //                         myStopped = false;
    //                         break;
    //                     }
    //                     if (stage->getDestinationStop() != nullptr) {
    //                         stage->getDestinationStop()->addTransportable(transportable);
    //                     }
    //
    //                     SUMOTime arrivalTime = currentTime;
    //                     if (MSGlobals::gUseMesoSim) {
    //                         // no boarding / unboarding time in meso
    //                         arrivalTime += 1;
    //                     } else {
    //                         const SUMOTime boardingTime = (SUMOTime)((double)boardingDuration * transportable->getVehicleType().getBoardingFactor());
    //                         if (timeForNext > currentTime - DELTA_T) {
    //                             timeForNext += boardingTime;
    //                         } else {
    //                             timeForNext = currentTime + boardingTime;
    //                         }
    //                     }
    //                     //ensure that vehicle stops long enough for deboarding
    //                     stop.duration = MAX2(stop.duration, timeForNext - currentTime);
    //
    //                     veh.removeTransportableMass(transportable);
    //                     auto locker = myHolder.getScopeLock();
    //                     i = myTransportables.erase(i); // erase first in case proceed throws an exception
    //                     numUnloaded++;
    //                     if (taxiDevice != nullptr) {
    //                         taxiDevice->customerArrived(transportable);
    //                     }
    //                     if (!transportable->proceed(MSNet::getInstance(), arrivalTime)) {
    //                         if (myAmContainer) {
    //                             MSNet::getInstance()->getContainerControl().erase(transportable);
    //                         } else {
    //                             MSNet::getInstance()->getPersonControl().erase(transportable);
    //                         }
    //                     }
    //                     if (MSStopOut::active()) {
    //                         SUMOVehicle* vehicle = dynamic_cast<SUMOVehicle*>(&veh);
    //                         if (myAmContainer) {
    //                             MSStopOut::getInstance()->unloadedContainers(vehicle, 1);
    //                         } else {
    //                             MSStopOut::getInstance()->unloadedPersons(vehicle, 1);
    //                         }
    //                     }
    //                     continue;
    //                 }
    //                 ++i;
    //             }
    //             if (numUnloaded != 0) {
    //                 changeAttached();
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        let holder = self.myHolder;
        let net = MSNet::get_instance();
        let currentTime = net.get_current_time_step();
        if self.myStopped {
            if !holder.is_stopped() {
                let freeFlowTimeCorrection = if crate::microsim::ms_globals::use_meso_sim() { time2steps(newPos / newSpeed) } else { 0 };
                for transportable in self.myTransportables.iter() {
                    transportable.m().set_departed(currentTime - freeFlowTimeCorrection);
                }
                self.myStopped = false;
            }
        } else {
            if holder.is_stopped() {
                self.myStopped = true;
                let stop = holder.m().get_next_stop_mutable();
                let joinVeh = base_vehicle_of(net.m().get_vehicle_control().get_vehicle(&stop.pars.join));
                let boardingDuration = holder.get_vehicle_type().get_loading_duration(!self.myAmContainer);
                let mut numUnloaded = 0;
                let mut i = 0usize;
                while i < self.myTransportables.len() {
                    let transportable = self.myTransportables[i];
                    let stage = stage_driving(transportable.get_current_stage());
                    let canLeave = stage.is_some_and(|s| s.m().can_leave_vehicle(transportable, &*holder, &*stop));
                    if canLeave && !Self::will_transfer_at_join(transportable, joinVeh) {
                        let timeForNext: &mut SUMOTime = if self.myAmContainer { &mut stop.timeToLoadNextContainer } else { &mut stop.timeToBoardNextPerson };
                        let taxiDevice = taxi_device_of(holder);
                        if !taxiDevice.is_null() && *timeForNext == 0 && !crate::microsim::ms_globals::use_meso_sim() {
                            // taxi passengers must leave at the end of the stop duration
                            *timeForNext = stop.pars.started + stop.pars.duration;
                        }
                        if *timeForNext - delta_t() > currentTime {
                            // try deboarding again in the next step
                            self.myStopped = false;
                            break;
                        }
                        if let Some(stage) = stage {
                            let destinationStop = stage.get_destination_stop();
                            if !destinationStop.is_null() {
                                destinationStop.m().add_transportable(transportable, -1);
                            }
                        }

                        let mut arrivalTime = currentTime;
                        if crate::microsim::ms_globals::use_meso_sim() {
                            // no boarding / unboarding time in meso
                            arrivalTime += 1;
                        } else {
                            let boardingTime = ((boardingDuration as f64) * transportable.get_vehicle_type().get_boarding_factor()) as SUMOTime;
                            if *timeForNext > currentTime - delta_t() {
                                *timeForNext += boardingTime;
                            } else {
                                *timeForNext = currentTime + boardingTime;
                            }
                        }
                        //ensure that vehicle stops long enough for deboarding
                        stop.duration = std::cmp::max(stop.duration, *timeForNext - currentTime);

                        holder.m().remove_transportable_mass(transportable);
                        self.myTransportables.remove(i); // erase first in case proceed throws an exception
                        numUnloaded += 1;
                        if !taxiDevice.is_null() {
                            taxiDevice.m().customer_arrived(transportable);
                        }
                        if !transportable.m().proceed(net, arrivalTime, false) {
                            if self.myAmContainer {
                                net.m().get_container_control().erase(transportable);
                            } else {
                                net.m().get_person_control().erase(transportable);
                            }
                        }
                        if MSStopOut::active() {
                            let vehicle = holder;
                            if self.myAmContainer {
                                MSStopOut::get_instance().m().unloaded_containers(vehicle, 1);
                            } else {
                                MSStopOut::get_instance().m().unloaded_persons(vehicle, 1);
                            }
                        }
                        continue;
                    }
                    i += 1;
                }
                if numUnloaded != 0 {
                    self.change_attached();
                }
            }
        }
        true
    }

    /// C++: `bool MSDevice_Transportable::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Adds passengers on vehicle insertion
    // bool
    // MSDevice_Transportable::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason == MSMoveReminder::NOTIFICATION_DEPARTED) {
    //         const SUMOTime currentTime = MSNet::getInstance()->getCurrentTimeStep();
    //         for (MSTransportable* const transportable : myTransportables) {
    //             transportable->setDeparted(currentTime);
    //         }
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         // to trigger vehicle leaving
    //         notifyMove(veh, -1., -1., -1.);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if matches!(reason, MSMoveReminder_Notification::NOTIFICATION_DEPARTED) {
            let currentTime = MSNet::get_instance().get_current_time_step();
            for transportable in self.myTransportables.iter() {
                transportable.m().set_departed(currentTime);
            }
        }
        if crate::microsim::ms_globals::use_meso_sim() {
            // to trigger vehicle leaving
            self.notify_move(veh, -1., -1., -1.);
        }
        true
    }

    /// C++: `bool MSDevice_Transportable::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Passengers leaving on arrival
    // bool
    // MSDevice_Transportable::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/,
    //                                     MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason >= MSMoveReminder::NOTIFICATION_ARRIVED) {
    //         auto locker = myHolder.getScopeLock();
    //         for (std::vector<MSTransportable*>::iterator i = myTransportables.begin(); i != myTransportables.end();) {
    //             MSTransportableControl& tc = myAmContainer ? MSNet::getInstance()->getContainerControl() : MSNet::getInstance()->getPersonControl();
    //             MSTransportable* transportable = *i;
    //             if (transportable->getDestination() != veh.getEdge()) {
    //                 WRITE_WARNINGF("Teleporting % '%' from vehicle destination edge '%' to intended destination edge '%' time=%",
    //                                myAmContainer ? "container" : "person", transportable->getID(), veh.getEdge()->getID(),
    //                                transportable->getDestination()->getID(), time2string(SIMSTEP));
    //                 tc.registerTeleportWrongDest();
    //             }
    //             if (!transportable->proceed(MSNet::getInstance(), MSNet::getInstance()->getCurrentTimeStep(), true)) {
    //                 tc.erase(transportable);
    //             }
    //             i = myTransportables.erase(i);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if (reason as i32) >= (MSMoveReminder_Notification::NOTIFICATION_ARRIVED as i32) {
            let net = MSNet::get_instance();
            while !self.myTransportables.is_empty() {
                let transportable = self.myTransportables[0];
                let tc = if self.myAmContainer { net.m().get_container_control() } else { net.m().get_person_control() };
                if transportable.get_destination() != veh.get_edge() {
                    write_warningf!(
                        "Teleporting % '%' from vehicle destination edge '%' to intended destination edge '%' time=%",
                        if self.myAmContainer { "container" } else { "person" },
                        transportable.get_id(),
                        veh.get_edge().get_id(),
                        transportable.get_destination().get_id(),
                        time2string(net.get_current_time_step())
                    );
                    tc.register_teleport_wrong_dest();
                }
                if !transportable.m().proceed(net, net.get_current_time_step(), true) {
                    tc.erase(transportable);
                }
                self.myTransportables.remove(0);
            }
        }
        true
    }

    /// C++: `const std::string MSDevice_Transportable::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return myAmContainer ? "container" : "person";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        if self.myAmContainer { "container".to_string() } else { "person".to_string() }
    }

    /// C++: `std::string MSDevice_Transportable::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Transportable::getParameter(const std::string& key) const {
    //     if (key == "IDList") {
    //         std::vector<std::string> ids;
    //         for (const MSTransportable* t : myTransportables) {
    //             ids.push_back(t->getID());
    //         }
    //         return toString(ids);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        if key == "IDList" {
            let ids: Vec<String> = self.myTransportables.iter().map(|t| t.get_id().to_string()).collect();
            return ids.join(" ");
        }
        // C++ throws InvalidArgument here; the skeleton signature has no error channel
        panic!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())
    }

    /// C++: `void MSDevice_Transportable::addTransportable(MSTransportable *)`
    /// Add a passenger
    // void
    // MSDevice_Transportable::addTransportable(MSTransportable* transportable) {
    //     auto locker = myHolder.getScopeLock();
    //     if (myTransportables.empty()) {
    //         myOriginalType = &myHolder.getVehicleType();
    //     }
    //     myTransportables.push_back(transportable);
    //     if (MSStopOut::active()) {
    //         if (myAmContainer) {
    //             MSStopOut::getInstance()->loadedContainers(&myHolder, 1);
    //         } else {
    //             MSStopOut::getInstance()->loadedPersons(&myHolder, 1);
    //         }
    //     }
    //     MSDevice_Taxi* taxiDevice = static_cast<MSDevice_Taxi*>(myHolder.getDevice(typeid(MSDevice_Taxi)));
    //     if (taxiDevice != nullptr) {
    //         taxiDevice->customerEntered(transportable);
    //     }
    //     changeAttached();
    // }
    #[allow(unused_variables)]
    pub fn add_transportable(&mut self, transportable: Ptr<MSTransportable>) {
        let holder = self.myHolder;
        if self.myTransportables.is_empty() {
            self.myOriginalType = Ptr::new(holder.get_vehicle_type());
        }
        self.myTransportables.push(transportable);
        if MSStopOut::active() {
            if self.myAmContainer {
                MSStopOut::get_instance().m().loaded_containers(holder, 1);
            } else {
                MSStopOut::get_instance().m().loaded_persons(holder, 1);
            }
        }
        let taxiDevice = taxi_device_of(holder);
        if !taxiDevice.is_null() {
            taxiDevice.m().customer_entered(transportable);
        }
        self.change_attached();
    }

    /// C++: `void MSDevice_Transportable::removeTransportable(MSTransportable *)`
    /// Remove a passenger (TraCI)
    // void
    // MSDevice_Transportable::removeTransportable(MSTransportable* transportable) {
    //     auto it = std::find(myTransportables.begin(), myTransportables.end(), transportable);
    //     if (it != myTransportables.end()) {
    //         auto locker = myHolder.getScopeLock();
    //         myTransportables.erase(it);
    //         if (MSStopOut::active() && myHolder.isStopped()) {
    //             if (myAmContainer) {
    //                 MSStopOut::getInstance()->unloadedContainers(&myHolder, 1);
    //             } else {
    //                 MSStopOut::getInstance()->unloadedPersons(&myHolder, 1);
    //             }
    //         }
    //         MSDevice_Taxi* taxiDevice = static_cast<MSDevice_Taxi*>(myHolder.getDevice(typeid(MSDevice_Taxi)));
    //         if (taxiDevice != nullptr) {
    //             taxiDevice->customerArrived(transportable);
    //         }
    //         changeAttached();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_transportable(&mut self, transportable: Ptr<MSTransportable>) {
        if let Some(pos) = self.myTransportables.iter().position(|t| *t == transportable) {
            let holder = self.myHolder;
            self.myTransportables.remove(pos);
            if MSStopOut::active() && holder.is_stopped() {
                if self.myAmContainer {
                    MSStopOut::get_instance().m().unloaded_containers(holder, 1);
                } else {
                    MSStopOut::get_instance().m().unloaded_persons(holder, 1);
                }
            }
            let taxiDevice = taxi_device_of(holder);
            if !taxiDevice.is_null() {
                taxiDevice.m().customer_arrived(transportable);
            }
            self.change_attached();
        }
    }

    /// C++: `bool MSDevice_Transportable::anyLeavingAtStop(const MSStop &) const` [const]
    // bool
    // MSDevice_Transportable::anyLeavingAtStop(const MSStop& stop) const {
    //     for (const MSTransportable* t : myTransportables) {
    //         MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(t->getCurrentStage());
    //         if (stage->canLeaveVehicle(t, myHolder, stop)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn any_leaving_at_stop(&self, stop: &MSStop) -> bool {
        for t in self.myTransportables.iter() {
            let stage = stage_driving(t.get_current_stage());
            if stage.is_some_and(|s| s.m().can_leave_vehicle(*t, &*self.myHolder, stop)) {
                return true;
            }
        }
        false
    }

    /// C++: `void MSDevice_Transportable::transferAtSplitOrJoin(MSBaseVehicle *)`
    /// transfers transportables that want to continue in the other train part (without boarding/loading delays)
    // void
    // MSDevice_Transportable::transferAtSplitOrJoin(MSBaseVehicle* otherVeh) {
    //     const MSStop& stop = myHolder.getNextStop();
    //     for (auto it = myTransportables.begin(); it != myTransportables.end();) {
    //         MSTransportable* t = *it;
    //         if (t->getNumRemainingStages() > 1) {
    //             MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(t->getCurrentStage());
    //             if (stage->canLeaveVehicle(t, myHolder, stop)) {
    //                 MSStageDriving* const stage2 = dynamic_cast<MSStageDriving*>(t->getNextStage(1));
    //                 if (stage2 && stage2->isWaitingFor(otherVeh)) {
    //                     auto locker = myHolder.getScopeLock();
    //                     it = myTransportables.erase(it);
    //                     // proceeding registers t as waiting on edge
    //                     t->proceed(MSNet::getInstance(), SIMSTEP);
    //                     MSTransportableControl& tc = (t->isPerson() ?
    //                                                   MSNet::getInstance()->getPersonControl() :
    //                                                   MSNet::getInstance()->getContainerControl());
    //                     tc.abortWaitingForVehicle(t);
    //                     t->getEdge()->removeTransportable(t);
    //                     otherVeh->addTransportable(t);
    //                     stage2->setVehicle(otherVeh);
    //                     continue;
    //                 }
    //             }
    //         }
    //         it++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn transfer_at_split_or_join(&mut self, otherVeh: Ptr<MSBaseVehicle>) {
        let net = MSNet::get_instance();
        let holder = self.myHolder;
        let stop = holder.get_next_stop();
        // C++ SUMOVehicle base view of otherVeh (MSBaseVehicle embeds its SUMOVehicle as `base`)
        let otherSV: Ptr<SUMOVehicle> = Ptr::from_mut(&mut otherVeh.m().base);
        let mut i = 0usize;
        while i < self.myTransportables.len() {
            let t = self.myTransportables[i];
            if t.get_num_remaining_stages() > 1 {
                let stage = stage_driving(t.get_current_stage());
                if stage.is_some_and(|s| s.m().can_leave_vehicle(t, &*holder, stop)) {
                    let stage2 = stage_driving(t.get_next_stage(1));
                    if let Some(stage2) = stage2.filter(|s| s.is_waiting_for(otherSV)) {
                        self.myTransportables.remove(i); // erase first
                        // proceeding registers t as waiting on edge
                        t.m().proceed(net, net.get_current_time_step(), false);
                        let tc = if t.is_person() { net.m().get_person_control() } else { net.m().get_container_control() };
                        tc.abort_waiting_for_vehicle(t);
                        t.get_edge().remove_transportable(t);
                        otherVeh.m().add_transportable(t);
                        stage2.m().set_vehicle(otherSV);
                        continue;
                    }
                }
            }
            i += 1;
        }
    }

    /// C++: `void MSDevice_Transportable::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice_Transportable::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     SUMOTime nextPerson = 0;
    //     SUMOTime nextContainer = 0;
    //     if (myHolder.isStopped()) {
    //         nextPerson = myHolder.getNextStop().timeToBoardNextPerson;
    //         nextContainer = myHolder.getNextStop().timeToLoadNextContainer;
    //     }
    //     std::ostringstream internals;
    //     internals << myStopped
    //         << " " << nextPerson
    //         << " " << nextContainer;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_DEVICE);
        out.write_attr(SUMO_ATTR_ID, self.base.base2.base.get_id());
        let mut nextPerson: SUMOTime = 0;
        let mut nextContainer: SUMOTime = 0;
        if self.myHolder.is_stopped() {
            nextPerson = self.myHolder.get_next_stop().timeToBoardNextPerson;
            nextContainer = self.myHolder.get_next_stop().timeToLoadNextContainer;
        }
        // C++ `internals << myStopped` writes a bool as 0/1
        let internals = format!("{} {} {}", self.myStopped as i32, nextPerson, nextContainer);
        out.write_attr(SUMO_ATTR_STATE, &internals);
        out.close_tag("");
    }

    /// C++: `void MSDevice_Transportable::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice_Transportable::loadState(const SUMOSAXAttributes& attrs) {
    //     SUMOTime nextPerson;
    //     SUMOTime nextContainer;
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> myStopped;
    //     bis >> nextPerson;
    //     bis >> nextContainer;
    //     if (nextPerson != 0 || nextContainer !=0) {
    //         assert(myHolder.hasStops());
    //         MSStop& stop = myHolder.getNextStopMutable();
    //         stop.timeToBoardNextPerson = nextPerson;
    //         stop.timeToLoadNextContainer = nextContainer;
    //         stop.duration = MAX2(stop.duration, MAX2(nextPerson, nextContainer) - SIMSTEP);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        // C++ `bis >> bool` reads a 0/1 integer
        self.myStopped = next_as::<i32>(&mut bis) != 0;
        let nextPerson: SUMOTime = next_as(&mut bis);
        let nextContainer: SUMOTime = next_as(&mut bis);
        if nextPerson != 0 || nextContainer != 0 {
            debug_assert!(self.myHolder.has_stops());
            let stop = self.myHolder.m().get_next_stop_mutable();
            stop.timeToBoardNextPerson = nextPerson;
            stop.timeToLoadNextContainer = nextContainer;
            stop.duration = std::cmp::max(stop.duration, std::cmp::max(nextPerson, nextContainer) - MSNet::get_instance().get_current_time_step());
        }
    }

    /// C++: `int MSDevice_Transportable::size() const` [const]
    /// Return the number of passengers / containers
    // int size() const {
    //         return (int)myTransportables.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myTransportables.len() as i32
    }

    /// C++: `const std::vector<MSTransportable *> & MSDevice_Transportable::getTransportables() const` [const]
    /// Returns the list of transportables using this vehicle
    // const std::vector<MSTransportable*>& getTransportables() const {
    //         return myTransportables;
    //     }
    #[allow(unused_variables)]
    pub fn get_transportables(&self) -> &Vec<Ptr<MSTransportable>> {
        &self.myTransportables
    }

    /// C++: `std::vector<Position> & MSDevice_Transportable::getUnboardingPositions()`
    // std::vector<Position>& getUnboardingPositions() {
    //         return myUnboardingPositions;
    //     }
    #[allow(unused_variables)]
    pub fn get_unboarding_positions(&mut self) -> &mut Vec<Position> {
        &mut self.myUnboardingPositions
    }

    /// C++: `bool MSDevice_Transportable::willTransferAtJoin(const MSTransportable *, const MSBaseVehicle *)` [static]
    /// check if boardingDuration should be applied
    // bool
    // MSDevice_Transportable::willTransferAtJoin(const MSTransportable* t, const MSBaseVehicle* joinVeh) {
    //     if (joinVeh && t->getNumRemainingStages() > 1) {
    //         MSStageDriving* const stage2 = dynamic_cast<MSStageDriving*>(t->getNextStage(1));
    //         return stage2->isWaitingFor(joinVeh);
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn will_transfer_at_join(t: Ptr<MSTransportable>, joinVeh: Ptr<MSBaseVehicle>) -> bool {
        if !joinVeh.is_null() && t.get_num_remaining_stages() > 1 {
            // C++ dereferences dynamic_cast<MSStageDriving*>(getNextStage(1)) without a null check
            return stage_driving(t.get_next_stage(1)).is_some_and(|s| s.is_waiting_for(Ptr::from_mut(&mut joinVeh.m().base)));
        }
        false
    }

    /// C++: `void MSDevice_Transportable::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves, see MSMoveReminder::notifyMoveInternal()
    // void
    // MSDevice_Transportable::notifyMoveInternal(const SUMOTrafficObject& veh,
    //         const double /* frontOnLane */,
    //         const double /* timeOnLane */,
    //         const double /* meanSpeedFrontOnLane */,
    //         const double /* meanSpeedVehicleOnLane */,
    //         const double travelledDistanceFrontOnLane,
    //         const double /* travelledDistanceVehicleOnLane */,
    //         const double /* meanLengthOnLane */) {
    //     notifyMove(const_cast<SUMOTrafficObject&>(veh), -1, travelledDistanceFrontOnLane, veh.getEdge()->getVehicleMaxSpeed(&veh));
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        // C++ const_cast<SUMOTrafficObject&>(veh): the object is the holder vehicle, which the device owns
        let vp: Ptr<SUMOTrafficObject> = Ptr::from_raw(veh as *const SUMOTrafficObject as *mut SUMOTrafficObject);
        let maxSpeed = veh.get_edge().get_vehicle_max_speed(vp);
        self.notify_move(vp.m(), -1., travelledDistanceFrontOnLane, maxSpeed);
    }

    /// C++: `void MSDevice_Transportable::changeAttached()` [protected]
    /// modifiy vehicle properties when loading/unloading (optional)
    // void
    // MSDevice_Transportable::changeAttached() {
    //     if (myLoadedType != nullptr) {
    //         int perAttached = myAmContainer ? myLoadedType->getContainerCapacity() : myLoadedType->getPersonCapacity();
    //         if (perAttached > 0) {
    //             MSBaseVehicle& veh = dynamic_cast<MSBaseVehicle&>(myHolder);
    //             SUMOVehicleClass oldVC = myHolder.getVClass();
    //             const double numAttached = ceil(myTransportables.size() / perAttached);
    //             if (numAttached > 0.) {
    //                 MSVehicleType* stype = &veh.getSingularType();
    //                 stype->setVClass(myLoadedType->getVehicleClass());
    //                 stype->setGUIShape(myLoadedType->getGuiShape());
    //                 stype->setLength(myOriginalType->getLength() + numAttached * myLoadedType->getLength());
    //                 stype->setMass(myOriginalType->getMass() + numAttached * myLoadedType->getMass());
    //                 SUMOVTypeParameter& sparam = const_cast<SUMOVTypeParameter&>(stype->getParameter());
    //                 sparam.carriageLength = myLoadedType->getParameter().carriageLength;
    //                 sparam.locomotiveLength = myLoadedType->getParameter().locomotiveLength;
    //                 sparam.carriageGap = myLoadedType->getParameter().carriageGap;
    //             } else {
    //                 myHolder.replaceVehicleType(myOriginalType);
    //             }
    //             if (oldVC != myHolder.getVClass()) {
    //                 veh.reroute(SIMSTEP, "device." + deviceName() + ".loadedType", veh.getRouterTT());
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn change_attached(&mut self) {
        if !self.myLoadedType.is_null() {
            let loaded = self.myLoadedType;
            let perAttached = if self.myAmContainer { loaded.get_container_capacity() } else { loaded.get_person_capacity() };
            if perAttached > 0 {
                let holder = self.myHolder;
                let veh = base_vehicle_of(holder);
                let oldVC = holder.get_v_class();
                // C++ integer division before ceil()
                let numAttached = ((self.myTransportables.len() as i32 / perAttached) as f64).ceil();
                if numAttached > 0. {
                    // shortcut: the singular-type update needs the MSBaseVehicle downcast (see base_vehicle_of)
                    if !veh.is_null() {
                        let stype = veh.m().get_singular_type();
                        stype.set_v_class(loaded.get_vehicle_class());
                        stype.set_gui_shape(loaded.get_gui_shape());
                        let length = self.myOriginalType.get_length() + numAttached * loaded.get_length();
                        stype.set_length(&length);
                        stype.set_mass(self.myOriginalType.get_mass() + numAttached * loaded.get_mass());
                        let mut sparamPtr = Ptr::new(stype.get_parameter());
                        let sparam = sparamPtr.m();
                        sparam.carriageLength = loaded.get_parameter().carriageLength;
                        sparam.locomotiveLength = loaded.get_parameter().locomotiveLength;
                        sparam.carriageGap = loaded.get_parameter().carriageGap;
                    }
                } else {
                    holder.m().replace_vehicle_type(self.myOriginalType);
                }
                if oldVC != holder.get_v_class() && !veh.is_null() {
                    let info = format!("device.{}.loadedType", self.device_name());
                    let router = holder.get_router_tt();
                    veh.m().reroute(MSNet::get_instance().get_current_time_step(), &info, router, false, false, false, Ptr::default());
                }
            }
        }
    }

    /// C++: `void MSDevice_Transportable::MSDevice_Transportable(SUMOVehicle &, const std::string &, const bool)` [private]
    /// Constructor
    // MSDevice_Transportable::MSDevice_Transportable(SUMOVehicle& holder, const std::string& id, const bool isContainer) :
    //     MSVehicleDevice(holder, id),
    //     myAmContainer(isContainer),
    //     myTransportables(),
    //     myStopped(holder.isStopped()),
    //     myOriginalType(&holder.getVehicleType()),
    //     myLoadedType(nullptr) {
    //     const std::string key = "device." + deviceName() + ".loadedType";
    //     const std::string loadedTypeID = holder.getStringParam(key);
    //     if (loadedTypeID != "") {
    //         myLoadedType = MSNet::getInstance()->getVehicleControl().getVType(loadedTypeID, getEquipmentRNG());
    //         if (myLoadedType == nullptr) {
    //             throw InvalidArgument(TLF("Vehicle type '%' in parameter '%' of vehicle '%' is not known.", loadedTypeID, key, holder.getID()));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, isContainer: bool) -> Self {
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Transportable>().m().notify_enter(veh, reason, enteredLane)
        }
        fn mr_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Transportable>().m().notify_move(veh, oldPos, newPos, newSpeed)
        }
        fn mr_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Transportable>().m().notify_leave(veh, lastPos, reason, enteredLane)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: mr_notify_move,
            notify_leave: mr_notify_leave,
        };
        fn mr_notify_move_internal(mr: &mut MSMoveReminder, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
            Ptr::from_mut(mr).cast::<MSDevice_Transportable>().m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
        }
        static MS_MOVE_REMINDER_INTERNAL_VT: crate::microsim::ms_move_reminder::MSMoveReminderInternalVt = crate::microsim::ms_move_reminder::MSMoveReminderInternalVt {
            notify_move_internal: mr_notify_move_internal,
        };
        let mut this = Self::new_device(holder, id, isContainer);
        this.base.base.vt = &MS_MOVE_REMINDER_VT;
        this.base.base.internal_vt = &MS_MOVE_REMINDER_INTERNAL_VT;
        this
    }

    fn new_device(holder: &mut SUMOVehicle, id: &str, isContainer: bool) -> Self {
        let deviceName = if isContainer { "container" } else { "person" };
        let key = format!("device.{}.loadedType", deviceName);
        let loadedTypeID = holder.get_string_param(&key, false, "");
        let mut myLoadedType: Ptr<MSVehicleType> = Ptr::default();
        if loadedTypeID != "" {
            myLoadedType = MSNet::get_instance().m().get_vehicle_control().get_v_type(&loadedTypeID, MSDevice::get_equipment_rng(), false);
            if myLoadedType.is_null() {
                // C++ throws InvalidArgument (TLF text); the skeleton signature has no error channel
                panic!("Vehicle type '{}' in parameter '{}' of vehicle '{}' is not known.", loadedTypeID, key, holder.get_id());
            }
        }
        let myStopped = holder.is_stopped();
        let myOriginalType = Ptr::new(holder.get_vehicle_type());
        Self {
            base: MSVehicleDevice::new(holder, id),
            myAmContainer: isContainer,
            myTransportables: Vec::new(),
            myStopped,
            myUnboardingPositions: Vec::new(),
            myOriginalType,
            myLoadedType,
        }
    }

    /// C++: `void MSDevice_Transportable::MSDevice_Transportable(const MSDevice_Transportable &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Transportable) -> Self {
        // C++ declares this copy constructor private and never defines it: any use is a link error there
        panic!("MSDevice_Transportable is not copyable (C++ declares the copy constructor private and undefined)")
    }

    /// C++: `MSDevice_Transportable & MSDevice_Transportable::operator=(const MSDevice_Transportable &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Transportable) -> &mut MSDevice_Transportable {
        // C++ declares the assignment operator private and never defines it
        panic!("MSDevice_Transportable is not assignable (C++ declares operator= private and undefined)")
    }

}

