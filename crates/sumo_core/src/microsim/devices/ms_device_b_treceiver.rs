//! Port of microsim/devices/MSDevice_BTreceiver.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A global update performer
/// C++ class `MSDevice_BTreceiver::BTreceiverUpdate` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSDevice_BTreceiver_BTreceiverUpdate {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
}

impl std::ops::Deref for MSDevice_BTreceiver_BTreceiverUpdate {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_BTreceiver_BTreceiverUpdate {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSDevice_BTreceiver_BTreceiverUpdate {
    /// C++: `void BTreceiverUpdate::BTreceiverUpdate()`
    /// Constructor
    // MSDevice_BTreceiver::BTreceiverUpdate::BTreceiverUpdate() {
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this);
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port BTreceiverUpdate::BTreceiverUpdate")
    }

    /// C++: `void BTreceiverUpdate::~BTreceiverUpdate() noexcept` [virtual]
    /// Destructor
    // MSDevice_BTreceiver::BTreceiverUpdate::~BTreceiverUpdate() {
    //     for (const auto& vehicleInfo : MSDevice_BTsender::sVehicles) {
    //         vehicleInfo.second->amOnNet = false;
    //         vehicleInfo.second->haveArrived = true;
    //     }
    //     for (const auto& vehicleInfo : MSDevice_BTreceiver::sVehicles) {
    //         vehicleInfo.second->amOnNet = false;
    //         vehicleInfo.second->haveArrived = true;
    //     }
    //     execute(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port BTreceiverUpdate::~BTreceiverUpdate")
    }

    /// C++: `SUMOTime BTreceiverUpdate::execute(SUMOTime)` [virtual]
    /// Performs the update
    // SUMOTime
    // MSDevice_BTreceiver::BTreceiverUpdate::execute(SUMOTime /*currentTime*/) {
    //     // loop over equipped persons to update their positions
    //     if (myHasPersons && MSNet::getInstance()->hasPersons()) {  // the check whether the net has persons is only important in the final cleanup
    //         MSTransportableControl& c = MSNet::getInstance()->getPersonControl();
    //         for (MSTransportableControl::constVehIt i = c.loadedBegin(); i != c.loadedEnd(); ++i) {
    //             MSTransportable* t = i->second;
    //             if (t->getCurrentStageType() != MSStageType::WAITING_FOR_DEPART) {
    //                 MSDevice_BTsender* snd = dynamic_cast<MSDevice_BTsender*>(t->getDevice(typeid(MSTransportableDevice_BTsender)));
    //                 MSDevice_BTreceiver* rec = dynamic_cast<MSDevice_BTreceiver*>(t->getDevice(typeid(MSTransportableDevice_BTreceiver)));
    //                 if (snd) {
    //                     snd->notifyMove(*t, t->getPositionOnLane(), t->getPositionOnLane(), t->getSpeed());
    //                     if (MSDevice_BTsender::sVehicles[t->getID()]->route.back() != t->getEdge()) {
    //                         MSDevice_BTsender::sVehicles[t->getID()]->route.push_back(t->getEdge());
    //                     }
    //                 }
    //                 if (rec) {
    //                     rec->notifyMove(*t, t->getPositionOnLane(), t->getPositionOnLane(), t->getSpeed());
    //                     if (sVehicles[t->getID()]->route.back() != t->getEdge()) {
    //                         sVehicles[t->getID()]->route.push_back(t->getEdge());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     // build rtree with senders
    //     NamedRTree rt;
    //     for (std::map<std::string, MSDevice_BTsender::VehicleInformation*>::const_iterator i = MSDevice_BTsender::sVehicles.begin(); i != MSDevice_BTsender::sVehicles.end(); ++i) {
    //         MSDevice_BTsender::VehicleInformation* vi = (*i).second;
    //         Boundary b = vi->getBoxBoundary();
    //         b.grow(POSITION_EPS);
    //         const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //         const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //         rt.Insert(cmin, cmax, vi);
    //     }
    //
    //     // check visibility for all receivers
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool allRecognitions = oc.getBool("device.btreceiver.all-recognitions");
    //     bool haveOutput = oc.isSet("bt-output");
    //     for (std::map<std::string, MSDevice_BTreceiver::VehicleInformation*>::iterator i = MSDevice_BTreceiver::sVehicles.begin(); i != MSDevice_BTreceiver::sVehicles.end();) {
    //         // collect surrounding vehicles
    //         MSDevice_BTreceiver::VehicleInformation* vi = (*i).second;
    //         Boundary b = vi->getBoxBoundary();
    //         b.grow(vi->range);
    //         const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //         const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //         std::set<const Named*> surroundingVehicles;
    //         Named::StoringVisitor sv(surroundingVehicles);
    //         rt.Search(cmin, cmax, sv);
    //
    //         // loop over surrounding vehicles, check visibility status
    //         for (const Named* vehicle : surroundingVehicles) {
    //             if ((*i).first == vehicle->getID()) {
    //                 // seeing oneself? skip
    //                 continue;
    //             }
    //             updateVisibility(*vi, *MSDevice_BTsender::sVehicles.find(vehicle->getID())->second);
    //         }
    //
    //         if (vi->haveArrived) {
    //             // vehicle has left the simulation; remove
    //             if (haveOutput) {
    //                 writeOutput((*i).first, vi->seen, allRecognitions);
    //             }
    //             delete vi;
    //             MSDevice_BTreceiver::sVehicles.erase(i++);
    //         } else {
    //             // vehicle is still in the simulation; reset state
    //             vi->updates.erase(vi->updates.begin(), vi->updates.end() - 1);
    //             ++i;
    //         }
    //     }
    //
    //     // remove arrived senders / reset state
    //     for (std::map<std::string, MSDevice_BTsender::VehicleInformation*>::iterator i = MSDevice_BTsender::sVehicles.begin(); i != MSDevice_BTsender::sVehicles.end();) {
    //         MSDevice_BTsender::VehicleInformation* vi = (*i).second;
    //         if (vi->haveArrived) {
    //             delete vi;
    //             MSDevice_BTsender::sVehicles.erase(i++);
    //         } else {
    //             vi->updates.erase(vi->updates.begin(), vi->updates.end() - 1);
    //             ++i;
    //         }
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port BTreceiverUpdate::execute")
    }

    /// C++: `void BTreceiverUpdate::updateVisibility(VehicleInformation &, MSDevice_BTsender::VehicleInformation &)`
    /// Rechecks the visibility for a given receiver/sender pair
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn update_visibility(&mut self, receiver: &mut MSVehicleTransfer_VehicleInformation, sender: &mut MSDevice_BTsender_VehicleInformation) {
        todo!("port BTreceiverUpdate::updateVisibility")
    }

    /// C++: `void BTreceiverUpdate::enterRange(double, const MSDevice_BTsender::VehicleState &, const std::string &, const MSDevice_BTsender::VehicleState &, int &)`
    /// Informs the receiver about a sender entering it's radius
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn enter_range(&mut self, atOffset: f64, receiverState: &MSDevice_BTsender_VehicleState, senderID: &str, senderState: &MSDevice_BTsender_VehicleState, currentlySeen: &mut i32) {
        todo!("port BTreceiverUpdate::enterRange")
    }

    /// C++: `void BTreceiverUpdate::leaveRange(VehicleInformation &, const MSDevice_BTsender::VehicleState &, MSDevice_BTsender::VehicleInformation &, const MSDevice_BTsender::VehicleState &, double)`
    /// Removes the sender from the currently seen devices to past episodes
    // void
    // MSDevice_BTreceiver::BTreceiverUpdate::leaveRange(VehicleInformation& receiverInfo, const MSDevice_BTsender::VehicleState& receiverState,
    //         MSDevice_BTsender::VehicleInformation& senderInfo, const MSDevice_BTsender::VehicleState& senderState,
    //         double tOffset) {
    //     std::map<std::string, SeenDevice*>::iterator i = receiverInfo.currentlySeen.find(senderInfo.getID());
    //     // check whether the other was recognized
    //     addRecognitionPoint(SIMTIME + tOffset, receiverState, senderState, i->second);
    //     // build leaving point
    //     i->second->meetingEnd = new MeetingPoint(STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep()) + tOffset, receiverState, senderState);
    //     ConstMSEdgeVector::const_iterator begin = receiverInfo.route.begin() + i->second->meetingBegin.observerState.routePos;
    //     ConstMSEdgeVector::const_iterator end = receiverInfo.route.begin() + receiverState.routePos + 1;
    //     i->second->receiverRoute = toString<const MSEdge>(begin, end);
    //     begin = senderInfo.route.begin() + i->second->meetingBegin.seenState.routePos;
    //     end = senderInfo.route.begin() + senderState.routePos + 1;
    //     i->second->senderRoute = toString<const MSEdge>(begin, end);
    //     receiverInfo.seen[senderInfo.getID()].push_back(i->second);
    //     receiverInfo.currentlySeen.erase(i);
    // }
    #[allow(unused_variables)]
    pub fn leave_range(&mut self, receiverInfo: &mut MSVehicleTransfer_VehicleInformation, receiverState: &MSDevice_BTsender_VehicleState, senderInfo: &mut MSDevice_BTsender_VehicleInformation, senderState: &MSDevice_BTsender_VehicleState, tOffset: f64) {
        todo!("port BTreceiverUpdate::leaveRange")
    }

    /// C++: `void BTreceiverUpdate::addRecognitionPoint(const double, const MSDevice_BTsender::VehicleState &, const MSDevice_BTsender::VehicleState &, SeenDevice *) const` [const]
    /// Adds a point of recognition
    // void
    // MSDevice_BTreceiver::BTreceiverUpdate::addRecognitionPoint(const double tEnd, const MSDevice_BTsender::VehicleState& receiverState,
    //         const MSDevice_BTsender::VehicleState& senderState,
    //         SeenDevice* senderDevice) const {
    //     if (senderDevice->nextView == -1.) {
    //         senderDevice->nextView = senderDevice->lastView + inquiryDelaySlots(int(myOffTime / 0.000625 + .5)) * 0.000625;
    //     }
    //     if (tEnd > senderDevice->nextView) {
    //         senderDevice->lastView = senderDevice->nextView;
    //         MeetingPoint* mp = new MeetingPoint(tEnd, receiverState, senderState);
    //         senderDevice->recognitionPoints.push_back(mp);
    //         senderDevice->nextView = senderDevice->lastView + inquiryDelaySlots(int(myOffTime / 0.000625 + .5)) * 0.000625;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_recognition_point(&self, tEnd: f64, receiverState: &MSDevice_BTsender_VehicleState, senderState: &MSDevice_BTsender_VehicleState, senderDevice: Ptr<MSDevice_BTreceiver_SeenDevice>) {
        todo!("port BTreceiverUpdate::addRecognitionPoint")
    }

    /// C++: `void BTreceiverUpdate::writeOutput(const std::string &, const int &, bool)`
    /// Writes the output
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn write_output(&mut self, id: &str, seen: &i32, allRecognitions: bool) {
        todo!("port BTreceiverUpdate::writeOutput")
    }

}

/// A BT receiver
/// C++ class `MSDevice_BTreceiver` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSDevice_BTreceiver {
}

// C++ static members of MSDevice_BTreceiver (port as module-level statics, see AGENTS.md):
//   static bool MSDevice_BTreceiver::myWasInitialised;
//   static bool MSDevice_BTreceiver::myHasPersons;
//   static double MSDevice_BTreceiver::myRange;
//   static double MSDevice_BTreceiver::myOffTime;
//   static SumoRNG MSDevice_BTreceiver::sRecognitionRNG;
//   static int MSDevice_BTreceiver::sVehicles;

impl MSDevice_BTreceiver {
    /// C++: `double MSDevice_BTreceiver::getRange()` [static]
    /// Returns the configured range
    // static double getRange() {
    //         return myRange;
    //     }
    #[allow(unused_variables)]
    pub fn get_range() -> f64 {
        todo!("port MSDevice_BTreceiver::getRange")
    }

    /// C++: `SumoRNG * MSDevice_BTreceiver::getRNG()` [static]
    // static SumoRNG* getRNG() {
    //         return &sRecognitionRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_rng() -> Ptr<SumoRNG> {
        todo!("port MSDevice_BTreceiver::getRNG")
    }

    /// C++: `void MSDevice_BTreceiver::~MSDevice_BTreceiver() noexcept`
    /// Destructor.
    // MSDevice_BTreceiver::~MSDevice_BTreceiver() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_BTreceiver::~MSDevice_BTreceiver")
    }

    /// C++: `bool MSDevice_BTreceiver::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)`
    /// Adds the vehicle to running vehicles if it (re-) enters the network
    // bool
    // MSDevice_BTreceiver::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason == MSMoveReminder::NOTIFICATION_DEPARTED && sVehicles.find(veh.getID()) == sVehicles.end()) {
    //         sVehicles[veh.getID()] = new VehicleInformation(veh.getID(), myRange);
    //         sVehicles[veh.getID()]->route.push_back(veh.getEdge());
    //     }
    //     if (reason == MSMoveReminder::NOTIFICATION_TELEPORT && sVehicles.find(veh.getID()) != sVehicles.end()) {
    //         sVehicles[veh.getID()]->amOnNet = true;
    //     }
    //     if (reason == MSMoveReminder::NOTIFICATION_TELEPORT || reason == MSMoveReminder::NOTIFICATION_JUNCTION) {
    //         sVehicles[veh.getID()]->route.push_back(veh.getEdge());
    //     }
    //     const std::string location = MSDevice_BTsender::getLocation(veh);
    //     sVehicles[veh.getID()]->updates.push_back(MSDevice_BTsender::VehicleState(veh.getSpeed(), veh.getPosition(), location, veh.getPositionOnLane(), veh.getRoutePosition()));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_BTreceiver::notifyEnter")
    }

    /// C++: `bool MSDevice_BTreceiver::notifyMove(SUMOTrafficObject &, double, double, double)`
    /// Checks whether the reminder still has to be notified about the vehicle moves
    // bool
    // MSDevice_BTreceiver::notifyMove(SUMOTrafficObject& veh, double /* oldPos */, double newPos, double newSpeed) {
    //     if (sVehicles.find(veh.getID()) == sVehicles.end()) {
    //         WRITE_WARNINGF(TL("btreceiver: Can not update position of vehicle '%' which is not on the road."), veh.getID());
    //         return true;
    //     }
    //     const std::string location = MSDevice_BTsender::getLocation(veh);
    //     sVehicles[veh.getID()]->updates.push_back(MSDevice_BTsender::VehicleState(newSpeed, veh.getPosition(), location, newPos, veh.getRoutePosition()));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_BTreceiver::notifyMove")
    }

    /// C++: `bool MSDevice_BTreceiver::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)`
    /// Moves (the known) vehicle from running to arrived vehicles' list
    // bool
    // MSDevice_BTreceiver::notifyLeave(SUMOTrafficObject& veh, double /* lastPos */, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason < MSMoveReminder::NOTIFICATION_TELEPORT) {
    //         return true;
    //     }
    //     if (sVehicles.find(veh.getID()) == sVehicles.end()) {
    //         WRITE_WARNINGF(TL("btreceiver: Can not update position of vehicle '%' which is not on the road."), veh.getID());
    //         return true;
    //     }
    //     const std::string location = MSDevice_BTsender::getLocation(veh);
    //     sVehicles[veh.getID()]->updates.push_back(MSDevice_BTsender::VehicleState(veh.getSpeed(), veh.getPosition(), location, veh.getPositionOnLane(), veh.getRoutePosition()));
    //     if (reason == MSMoveReminder::NOTIFICATION_TELEPORT) {
    //         sVehicles[veh.getID()]->amOnNet = false;
    //     }
    //     if (reason >= MSMoveReminder::NOTIFICATION_ARRIVED) {
    //         sVehicles[veh.getID()]->amOnNet = false;
    //         sVehicles[veh.getID()]->haveArrived = true;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_BTreceiver::notifyLeave")
    }

    /// C++: `void MSDevice_BTreceiver::cleanUp(int &, int &)` [static]
    /// Clears the given containers deleting the stored items
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn clean_up(c: &mut i32, s: &mut i32) {
        todo!("port MSDevice_BTreceiver::cleanUp")
    }

    /// C++: `SumoRNG * MSDevice_BTreceiver::getRecognitionRNG()` [static]
    // static SumoRNG* getRecognitionRNG() {
    //         return &sRecognitionRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_recognition_rng() -> Ptr<SumoRNG> {
        todo!("port MSDevice_BTreceiver::getRecognitionRNG")
    }

    /// C++: `void MSDevice_BTreceiver::hasSendingPersons()` [static]
    // static void hasSendingPersons() {
    //         myHasPersons = true;
    //     }
    #[allow(unused_variables)]
    pub fn has_sending_persons() {
        todo!("port MSDevice_BTreceiver::hasSendingPersons")
    }

    /// C++: `void MSDevice_BTreceiver::MSDevice_BTreceiver()` [protected]
    /// Constructor
    // MSDevice_BTreceiver() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MSDevice_BTreceiver::MSDevice_BTreceiver")
    }

    /// C++: `double MSDevice_BTreceiver::inquiryDelaySlots(const int)` [static, protected]
    // double
    // MSDevice_BTreceiver::inquiryDelaySlots(const int backoffLimit) {
    //     const int phaseOffset = RandHelper::rand(2047, &sRecognitionRNG);
    //     const bool interlaced = RandHelper::rand(&sRecognitionRNG) < 0.7;
    //     const double delaySlots = RandHelper::rand(&sRecognitionRNG) * 15;
    //     const int backoff = RandHelper::rand(backoffLimit, &sRecognitionRNG);
    //     if (interlaced) {
    //         return RandHelper::rand(&sRecognitionRNG) * 31 + backoff;
    //     }
    //     if (RandHelper::rand(31, &sRecognitionRNG) < 16) {
    //         // correct train for f0
    //         return delaySlots + backoff;
    //     }
    //     if (RandHelper::rand(30, &sRecognitionRNG) < 16) {
    //         // correct train for f1
    //         return 2048 - phaseOffset + delaySlots + backoff;
    //     }
    //     if (RandHelper::rand(29, &sRecognitionRNG) < 16) {
    //         // f2 is in train A but has overlap with both trains
    //         if (2 * 2048 - phaseOffset + backoff < 4096) {
    //             return 2 * 2048 - phaseOffset + delaySlots + backoff;
    //         }
    //         // the following is wrong but should only happen in about 3% of the non-interlaced cases
    //         return 2 * 2048 - phaseOffset + delaySlots + backoff;
    //     }
    //     return 2 * 2048 + delaySlots + backoff;
    // }
    #[allow(unused_variables)]
    pub fn inquiry_delay_slots(backoffLimit: i32) -> f64 {
        todo!("port MSDevice_BTreceiver::inquiryDelaySlots")
    }

    /// C++: `void MSDevice_BTreceiver::MSDevice_BTreceiver(const MSDevice_BTreceiver &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_BTreceiver) -> Self {
        todo!("port MSDevice_BTreceiver::MSDevice_BTreceiver")
    }

    /// C++: `MSDevice_BTreceiver & MSDevice_BTreceiver::operator=(const MSDevice_BTreceiver &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTreceiver) -> &mut MSDevice_BTreceiver {
        todo!("port MSDevice_BTreceiver::operator=")
    }

}

/// C++ class `MSTransportableDevice_BTreceiver` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSTransportableDevice_BTreceiver {
    /// C++ base class `MSDevice_BTreceiver`
    pub base: MSDevice_BTreceiver,
    /// C++ base class `MSTransportableDevice`
    pub base2: MSTransportableDevice,
}

impl std::ops::Deref for MSTransportableDevice_BTreceiver {
    type Target = MSDevice_BTreceiver;
    fn deref(&self) -> &MSDevice_BTreceiver {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportableDevice_BTreceiver {
    fn deref_mut(&mut self) -> &mut MSDevice_BTreceiver {
        &mut self.base
    }
}

impl MSTransportableDevice_BTreceiver {
    /// C++: `void MSTransportableDevice_BTreceiver::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_BTreceiver-options
    // void
    // MSTransportableDevice_BTreceiver::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("btreceiver", "Communication", oc, true);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSTransportableDevice_BTreceiver::insertOptions")
    }

    /// C++: `void MSTransportableDevice_BTreceiver::buildDevices(MSTransportable &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_devices(t: &mut MSTransportable, into: &mut i32) {
        todo!("port MSTransportableDevice_BTreceiver::buildDevices")
    }

    /// C++: `const std::string MSTransportableDevice_BTreceiver::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btreceiver";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSTransportableDevice_BTreceiver::deviceName")
    }

    /// C++: `bool MSTransportableDevice_BTreceiver::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSTransportableDevice_BTreceiver::notifyEnter")
    }

    /// C++: `bool MSTransportableDevice_BTreceiver::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTreceiver::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSTransportableDevice_BTreceiver::notifyMove")
    }

    /// C++: `bool MSTransportableDevice_BTreceiver::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSTransportableDevice_BTreceiver::notifyLeave")
    }

    /// C++: `void MSTransportableDevice_BTreceiver::MSTransportableDevice_BTreceiver(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_BTreceiver(MSTransportable& holder, const std::string& id) :
    //         MSTransportableDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        todo!("port MSTransportableDevice_BTreceiver::MSTransportableDevice_BTreceiver")
    }

}

/// C++ class `MSVehicleDevice_BTreceiver` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSVehicleDevice_BTreceiver {
    /// C++ base class `MSDevice_BTreceiver`
    pub base: MSDevice_BTreceiver,
    /// C++ base class `MSVehicleDevice`
    pub base2: MSVehicleDevice,
}

impl std::ops::Deref for MSVehicleDevice_BTreceiver {
    type Target = MSDevice_BTreceiver;
    fn deref(&self) -> &MSDevice_BTreceiver {
        &self.base
    }
}
impl std::ops::DerefMut for MSVehicleDevice_BTreceiver {
    fn deref_mut(&mut self) -> &mut MSDevice_BTreceiver {
        &mut self.base
    }
}

impl MSVehicleDevice_BTreceiver {
    /// C++: `void MSVehicleDevice_BTreceiver::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_BTreceiver-options
    // void
    // MSVehicleDevice_BTreceiver::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("btreceiver", "Communication", oc);
    //
    //     oc.doRegister("device.btreceiver.range", new Option_Float(300));
    //     oc.addDescription("device.btreceiver.range", "Communication", TL("The range of the bt receiver"));
    //
    //     oc.doRegister("device.btreceiver.all-recognitions", new Option_Bool(false));
    //     oc.addDescription("device.btreceiver.all-recognitions", "Communication", TL("Whether all recognition point shall be written"));
    //
    //     oc.doRegister("device.btreceiver.offtime", new Option_Float(0.64));
    //     oc.addDescription("device.btreceiver.offtime", "Communication", TL("The offtime used for calculating detection probability (in seconds)"));
    //
    //     myWasInitialised = false;
    //     myHasPersons = false;
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSVehicleDevice_BTreceiver::insertOptions")
    }

    /// C++: `void MSVehicleDevice_BTreceiver::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut i32) {
        todo!("port MSVehicleDevice_BTreceiver::buildVehicleDevices")
    }

    /// C++: `const std::string MSVehicleDevice_BTreceiver::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btreceiver";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSVehicleDevice_BTreceiver::deviceName")
    }

    /// C++: `bool MSVehicleDevice_BTreceiver::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSVehicleDevice_BTreceiver::notifyEnter")
    }

    /// C++: `bool MSVehicleDevice_BTreceiver::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTreceiver::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSVehicleDevice_BTreceiver::notifyMove")
    }

    /// C++: `bool MSVehicleDevice_BTreceiver::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSVehicleDevice_BTreceiver::notifyLeave")
    }

    /// C++: `void MSVehicleDevice_BTreceiver::MSVehicleDevice_BTreceiver(SUMOVehicle &, const std::string &)`
    /// Constructor
    // MSVehicleDevice_BTreceiver(SUMOVehicle& holder, const std::string& id) :
    //         MSVehicleDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        todo!("port MSVehicleDevice_BTreceiver::MSVehicleDevice_BTreceiver")
    }

}

/// Holds the information about exact positions/speeds/time of the begin/end of a meeting
/// C++ class `MSDevice_BTreceiver::MeetingPoint` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSDevice_BTreceiver_MeetingPoint {
    /// The time of the meeting
    pub t: f64,
    /// The state the observer had at the time
    pub observerState: MSDevice_BTsender_VehicleState,
    /// The state the seen vehicle had at the time
    pub seenState: MSDevice_BTsender_VehicleState,
}

impl MSDevice_BTreceiver_MeetingPoint {
    /// C++: `void MeetingPoint::MeetingPoint(double, const MSDevice_BTsender::VehicleState &, const MSDevice_BTsender::VehicleState &)`
    /// Constructor
    // MeetingPoint(double _t, const MSDevice_BTsender::VehicleState& _observerState,
    //                      const MSDevice_BTsender::VehicleState& _seenState)
    //             : t(_t), observerState(_observerState), seenState(_seenState) {}
    #[allow(unused_variables)]
    pub fn new(_t: f64, _observerState: &MSDevice_BTsender_VehicleState, _seenState: &MSDevice_BTsender_VehicleState) -> Self {
        todo!("port MeetingPoint::MeetingPoint")
    }

    /// C++: `void MeetingPoint::~MeetingPoint() noexcept`
    /// Destructor
    // ~MeetingPoint() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MeetingPoint::~MeetingPoint")
    }

    /// C++: `MeetingPoint & MeetingPoint::operator=(const MeetingPoint &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTreceiver_MeetingPoint) -> &mut MSDevice_BTreceiver_MeetingPoint {
        todo!("port MeetingPoint::operator=")
    }

}

/// Class representing a single seen device
/// C++ class `MSDevice_BTreceiver::SeenDevice` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSDevice_BTreceiver_SeenDevice {
    /// Description of the meeting's begin
    pub meetingBegin: MSDevice_BTreceiver_MeetingPoint,
    /// Description of the meeting's end
    pub meetingEnd: Ptr<MSDevice_BTreceiver_MeetingPoint>,
    /// Last recognition point
    pub lastView: f64,
    /// Next possible recognition point
    pub nextView: f64,
    /// List of recognition points
    pub recognitionPoints: i32,
    /// string of travelled receiver edges
    pub receiverRoute: String,
    /// string of travelled sender edges
    pub senderRoute: String,
}

impl MSDevice_BTreceiver_SeenDevice {
    /// C++: `void SeenDevice::SeenDevice(const MeetingPoint &)`
    /// Constructor
    // SeenDevice(const MeetingPoint& meetingBegin_)
    //             : meetingBegin(meetingBegin_), meetingEnd(0), lastView(meetingBegin_.t), nextView(-1.) {}
    #[allow(unused_variables)]
    pub fn new(meetingBegin_: &MSDevice_BTreceiver_MeetingPoint) -> Self {
        todo!("port SeenDevice::SeenDevice")
    }

    /// C++: `void SeenDevice::~SeenDevice() noexcept`
    /// Destructor
    // ~SeenDevice() {
    //             delete meetingEnd;
    //             for (std::vector<MeetingPoint*>::iterator i = recognitionPoints.begin(); i != recognitionPoints.end(); ++i) {
    //                 delete *i;
    //             }
    //             recognitionPoints.clear();
    //         }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SeenDevice::~SeenDevice")
    }

    /// C++: `SeenDevice & SeenDevice::operator=(const SeenDevice &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTreceiver_SeenDevice) -> &mut MSDevice_BTreceiver_SeenDevice {
        todo!("port SeenDevice::operator=")
    }

}

/// Stores the information of a vehicle
/// C++ class `MSDevice_BTreceiver::VehicleInformation` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSDevice_BTreceiver_VehicleInformation {
    /// C++ base class `MSDevice_BTsender::VehicleInformation`
    pub base: MSDevice_BTsender_VehicleInformation,
    /// Recognition range of the vehicle
    pub range: f64,
    /// The map of devices seen by the vehicle at removal time
    pub currentlySeen: i32,
    /// The past episodes of removed vehicle
    pub seen: i32,
}

impl std::ops::Deref for MSDevice_BTreceiver_VehicleInformation {
    type Target = MSDevice_BTsender_VehicleInformation;
    fn deref(&self) -> &MSDevice_BTsender_VehicleInformation {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_BTreceiver_VehicleInformation {
    fn deref_mut(&mut self) -> &mut MSDevice_BTsender_VehicleInformation {
        &mut self.base
    }
}

impl MSDevice_BTreceiver_VehicleInformation {
    /// C++: `void VehicleInformation::VehicleInformation(const std::string &, const double)`
    /// Constructor
    // VehicleInformation(const std::string& id, const double _range) : MSDevice_BTsender::VehicleInformation(id), range(_range) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, _range: f64) -> Self {
        todo!("port VehicleInformation::VehicleInformation")
    }

    /// C++: `void VehicleInformation::~VehicleInformation() noexcept` [virtual]
    /// Destructor
    // ~VehicleInformation() {
    //             std::map<std::string, SeenDevice*>::iterator i;
    //             for (i = currentlySeen.begin(); i != currentlySeen.end(); i++) {
    //                 delete i->second;
    //             }
    //             std::map<std::string, std::vector<SeenDevice*> >::iterator j;
    //             std::vector<SeenDevice*>::iterator k;
    //             for (j = seen.begin(); j != seen.end(); j++) {
    //                 for (k = j->second.begin(); k != j->second.end(); k++) {
    //                     delete *k;
    //                 }
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port VehicleInformation::~VehicleInformation")
    }

    /// C++: `void VehicleInformation::VehicleInformation(const VehicleInformation &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSVehicleTransfer_VehicleInformation) -> Self {
        todo!("port VehicleInformation::VehicleInformation")
    }

    /// C++: `VehicleInformation & VehicleInformation::operator=(const VehicleInformation &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVehicleTransfer_VehicleInformation) -> &mut MSVehicleTransfer_VehicleInformation {
        todo!("port VehicleInformation::operator=")
    }

}

