//! Port of microsim/devices/MSDevice_Transportable.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSDevice_Transportable` (microsim/devices/MSDevice_Transportable.h)
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
        todo!("port MSDevice_Transportable::buildVehicleDevices")
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
        todo!("port MSDevice_Transportable::~MSDevice_Transportable")
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
        todo!("port MSDevice_Transportable::notifyMove")
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
        todo!("port MSDevice_Transportable::notifyEnter")
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
        todo!("port MSDevice_Transportable::notifyLeave")
    }

    /// C++: `const std::string MSDevice_Transportable::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return myAmContainer ? "container" : "person";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Transportable::deviceName")
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
        todo!("port MSDevice_Transportable::getParameter")
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
        todo!("port MSDevice_Transportable::addTransportable")
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
        todo!("port MSDevice_Transportable::removeTransportable")
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
        todo!("port MSDevice_Transportable::anyLeavingAtStop")
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
        todo!("port MSDevice_Transportable::transferAtSplitOrJoin")
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
        todo!("port MSDevice_Transportable::saveState")
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
        todo!("port MSDevice_Transportable::loadState")
    }

    /// C++: `int MSDevice_Transportable::size() const` [const]
    /// Return the number of passengers / containers
    // int size() const {
    //         return (int)myTransportables.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        todo!("port MSDevice_Transportable::size")
    }

    /// C++: `const std::vector<MSTransportable *> & MSDevice_Transportable::getTransportables() const` [const]
    /// Returns the list of transportables using this vehicle
    // const std::vector<MSTransportable*>& getTransportables() const {
    //         return myTransportables;
    //     }
    #[allow(unused_variables)]
    pub fn get_transportables(&self) -> &Vec<Ptr<MSTransportable>> {
        todo!("port MSDevice_Transportable::getTransportables")
    }

    /// C++: `std::vector<Position> & MSDevice_Transportable::getUnboardingPositions()`
    // std::vector<Position>& getUnboardingPositions() {
    //         return myUnboardingPositions;
    //     }
    #[allow(unused_variables)]
    pub fn get_unboarding_positions(&mut self) -> &mut Vec<Position> {
        todo!("port MSDevice_Transportable::getUnboardingPositions")
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
        todo!("port MSDevice_Transportable::willTransferAtJoin")
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
        todo!("port MSDevice_Transportable::notifyMoveInternal")
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
        todo!("port MSDevice_Transportable::changeAttached")
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
        todo!("port MSDevice_Transportable::MSDevice_Transportable")
    }

    /// C++: `void MSDevice_Transportable::MSDevice_Transportable(const MSDevice_Transportable &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Transportable) -> Self {
        todo!("port MSDevice_Transportable::MSDevice_Transportable")
    }

    /// C++: `MSDevice_Transportable & MSDevice_Transportable::operator=(const MSDevice_Transportable &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Transportable) -> &mut MSDevice_Transportable {
        todo!("port MSDevice_Transportable::operator=")
    }

}

