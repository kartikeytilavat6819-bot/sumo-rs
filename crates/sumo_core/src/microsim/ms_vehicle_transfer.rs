//! Port of microsim/MSVehicleTransfer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This object (each simulation owns exactly one) is responsible for the transfer of vehicles that got stuck within the network due to grid locks. It also manages vehicles that are removed from the network because of stops with the parking attribute.
/// C++ class `MSVehicleTransfer` (microsim/MSVehicleTransfer.h)
pub struct MSVehicleTransfer {
    /// The information about stored vehicles to move virtually
    pub myVehicles: i32,
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
        todo!("port MSVehicleTransfer::~MSVehicleTransfer")
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
        todo!("port MSVehicleTransfer::add")
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
        todo!("port MSVehicleTransfer::remove")
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
    //                             desc.myVeh->leaveLane(MSMoveReminder::NOTIFICATION_TELEPORT_CONTINUATION_ARRIVED);
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
        todo!("port MSVehicleTransfer::checkInsertions")
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
        todo!("port MSVehicleTransfer::saveState")
    }

    /// C++: `void MSVehicleTransfer::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MSVehicleTransfer::clearState() {
    //     myVehicles.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        todo!("port MSVehicleTransfer::clearState")
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
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes, offset: SUMOTime, vc: &mut MSVehicleControl) {
        todo!("port MSVehicleTransfer::loadState")
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
        todo!("port MSVehicleTransfer::getInstance")
    }

    /// C++: `void MSVehicleTransfer::MSVehicleTransfer()` [private]
    /// Constructor
    // MSVehicleTransfer::MSVehicleTransfer() : myVehicles(MSGlobals::gNumSimThreads > 1) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MSVehicleTransfer::MSVehicleTransfer")
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
        todo!("port VehicleInformation::VehicleInformation")
    }

    /// C++: `bool VehicleInformation::operator<(const VehicleInformation &) const` [const]
    /// sort by vehicle ID for repeatable parallel simulation
    // bool
    // MSVehicleTransfer::VehicleInformation::operator<(const VehicleInformation& v2) const {
    //     return myVeh->getNumericalID() < v2.myVeh->getNumericalID();
    // }
    #[allow(unused_variables)]
    pub fn lt_op(&self, v2: &MSVehicleTransfer_VehicleInformation) -> bool {
        todo!("port VehicleInformation::operator<")
    }

}

