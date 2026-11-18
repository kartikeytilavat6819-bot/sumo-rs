//! Port of microsim/devices/MSDevice_BTreceiver.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::{Cell, RefCell};

thread_local! {
    /// C++ `static bool MSDevice_BTreceiver::myWasInitialised`
    static MY_WAS_INITIALISED: Cell<bool> = const { Cell::new(false) };
    /// The single BTreceiverUpdate; C++ ~MSNet deletes it so its destructor runs the final pass + bt-output.
    static BT_UPDATE: RefCell<Option<Ptr<MSDevice_BTreceiver_BTreceiverUpdate>>> = RefCell::new(None);
    /// C++ `static bool MSDevice_BTreceiver::myHasPersons`
    static MY_HAS_PERSONS: Cell<bool> = const { Cell::new(true) };
    /// C++ `static double MSDevice_BTreceiver::myRange`
    static MY_RANGE: Cell<f64> = const { Cell::new(-1.) };
    /// C++ `static double MSDevice_BTreceiver::myOffTime`
    static MY_OFF_TIME: Cell<f64> = const { Cell::new(-1.) };
    /// C++ `static SumoRNG MSDevice_BTreceiver::sRecognitionRNG("btreceiver")`
    static S_RECOGNITION_RNG: Ptr<SumoRNG> = Ptr::from_raw(Box::into_raw(Box::new(SumoRNG::new("btreceiver"))));
    /// C++ `static std::map<std::string, MSDevice_BTreceiver::VehicleInformation*> sVehicles`
    static S_VEHICLES: RefCell<BTreeMap<String, Ptr<MSDevice_BTreceiver_VehicleInformation>>> = const { RefCell::new(BTreeMap::new()) };
}

fn vehicle_info(id: &str) -> Ptr<MSDevice_BTreceiver_VehicleInformation> {
    S_VEHICLES.with(|m| *m.borrow().get(id).expect("btreceiver: no VehicleInformation for vehicle"))
}

fn has_vehicle_info(id: &str) -> bool {
    S_VEHICLES.with(|m| m.borrow().contains_key(id))
}

/// Snapshot of C++ `MSDevice_BTsender::sVehicles` in map (ID) order, so the map can be changed while iterating
fn sender_snapshot() -> Vec<(String, Ptr<MSDevice_BTsender_VehicleInformation>)> {
    crate::microsim::devices::ms_device_b_tsender::S_VEHICLES.with(|m| m.borrow().iter().map(|(id, vi)| (id.clone(), *vi)).collect())
}

/// Snapshot of C++ `MSDevice_BTreceiver::sVehicles` in map (ID) order, so the map can be changed while iterating
fn receiver_snapshot() -> Vec<(String, Ptr<MSDevice_BTreceiver_VehicleInformation>)> {
    S_VEHICLES.with(|m| m.borrow().iter().map(|(id, vi)| (id.clone(), *vi)).collect())
}

fn copy_state(s: &MSDevice_BTsender_VehicleState) -> MSDevice_BTsender_VehicleState {
    MSDevice_BTsender_VehicleState::new(s.speed, &s.position, &s.laneID, s.lanePos, s.routePos)
}

fn copy_meeting_point(m: &MSDevice_BTreceiver_MeetingPoint) -> MSDevice_BTreceiver_MeetingPoint {
    MSDevice_BTreceiver_MeetingPoint::new(m.t, &m.observerState, &m.seenState)
}

fn delete_seen(sd: Ptr<MSDevice_BTreceiver_SeenDevice>) {
    sd.m().drop_();
    drop(unsafe { Box::from_raw(sd.as_raw()) });
}

fn delete_vehicle_info(vi: Ptr<MSDevice_BTreceiver_VehicleInformation>) {
    vi.m().drop_();
    drop(unsafe { Box::from_raw(vi.as_raw()) });
}

fn delete_sender_info(vi: Ptr<MSDevice_BTsender_VehicleInformation>) {
    vi.m().drop_();
    drop(unsafe { Box::from_raw(vi.as_raw()) });
}

/// C++ `SIMTIME` (seconds)
fn simtime() -> f64 {
    steps2time(MSNet::get_instance().get_current_time_step())
}

/// C++ `TS` (DELTA_T in seconds)
fn ts() -> f64 {
    delta_t() as f64 / 1000.
}

/// C++ `toString<const MSEdge>(begin, end)`: edge IDs separated by spaces
fn route_string(route: &ConstMSEdgeVector, begin: usize, end: usize) -> String {
    route[begin..end]
        .iter()
        .map(|e| if e.is_null() { String::new() } else { e.m().get_id().to_string() })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Float box overlap test (stands in for the NamedRTree search)
fn boxes_overlap(amin: [f32; 2], amax: [f32; 2], bmin: [f32; 2], bmax: [f32; 2]) -> bool {
    (0..2).all(|d| amin[d] <= bmax[d] && bmin[d] <= amax[d])
}

/// Initialises the bt-system from the options the first time a device is built
fn init_once_from_options() {
    if !MY_WAS_INITIALISED.with(|c| c.get()) {
        // C++ ~MSNet deletes the end-of-timestep events, running ~BTreceiverUpdate (drop_: final pass + bt-output for vehicles still running).
        // The event control here never drops its commands, so MSNet invokes finish() to run that final pass.
        let update = Box::leak(MSDevice_BTreceiver_BTreceiverUpdate::new());
        BT_UPDATE.with(|c| *c.borrow_mut() = Some(Ptr::from_mut(update)));
        MY_WAS_INITIALISED.with(|c| c.set(true));
        MY_RANGE.with(|c| c.set(utils::options::options_cont::get_float("device.btreceiver.range")));
        MY_OFF_TIME.with(|c| c.set(utils::options::options_cont::get_float("device.btreceiver.offtime")));
        S_RECOGNITION_RNG.with(|r| r.m().set_seed(utils::options::options_cont::get_int("seed")));
    }
}

/// Runs the C++ `~BTreceiverUpdate` final pass (final visibility updates + bt-output for vehicles still running).
pub fn finish() {
    if let Some(u) = BT_UPDATE.with(|c| c.borrow_mut().take()) {
        u.m().drop_();
    }
}

/// A global update performer
/// C++ class `MSDevice_BTreceiver::BTreceiverUpdate` (microsim/devices/MSDevice_BTreceiver.h)
pub struct MSDevice_BTreceiver_BTreceiverUpdate {
}

impl Command for MSDevice_BTreceiver_BTreceiverUpdate {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSDevice_BTreceiver_BTreceiverUpdate::execute(self, currentTime)
    }
}

impl MSDevice_BTreceiver_BTreceiverUpdate {
    /// C++: `void BTreceiverUpdate::BTreceiverUpdate()`
    /// Constructor
    // MSDevice_BTreceiver::BTreceiverUpdate::BTreceiverUpdate() {
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this);
    // }
    #[allow(unused_variables)]
    pub fn new() -> Box<Self> {
        let mut b = Box::new(MSDevice_BTreceiver_BTreceiverUpdate {});
        MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(Ptr::from_mut(&mut *b as &mut dyn Command), -1);
        b
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
        for (_, vi) in sender_snapshot() {
            vi.m().amOnNet = false;
            vi.m().haveArrived = true;
        }
        for (_, vi) in receiver_snapshot() {
            vi.m().amOnNet = false;
            vi.m().haveArrived = true;
        }
        self.execute(MSNet::get_instance().get_current_time_step());
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
        // the person loop has no effect in C++: a person carries MSTransportableDevice_BT* devices, and
        // dynamic_cast<MSDevice_BTsender*> / <MSDevice_BTreceiver*> of those (sibling classes) is always nullptr
        // NamedRTree::Search over float boxes = a linear scan with the same float boxes and inclusive overlap test
        let mut rt: Vec<(String, Ptr<MSDevice_BTsender_VehicleInformation>, [f32; 2], [f32; 2])> = Vec::new();
        for (id, vi) in sender_snapshot() {
            let mut b = vi.m().get_box_boundary();
            b.grow(utils::geom::POSITION_EPS);
            let cmin = [b.xmin() as f32, b.ymin() as f32];
            let cmax = [b.xmax() as f32, b.ymax() as f32];
            rt.push((id, vi, cmin, cmax));
        }

        let allRecognitions = utils::options::options_cont::get_bool("device.btreceiver.all-recognitions");
        let haveOutput = utils::options::options_cont::is_set("bt-output");
        for (id, vi) in receiver_snapshot() {
            let v = vi.m();
            let mut b = v.get_box_boundary();
            b.grow(v.range);
            let cmin = [b.xmin() as f32, b.ymin() as f32];
            let cmax = [b.xmax() as f32, b.ymax() as f32];
            // determinism: C++ std::set<const Named*> orders surrounding vehicles by address; here they are visited in ID order
            let surrounding: Vec<(String, Ptr<MSDevice_BTsender_VehicleInformation>)> = rt
                .iter()
                .filter(|e| boxes_overlap(e.2, e.3, cmin, cmax))
                .map(|e| (e.0.clone(), e.1))
                .collect();
            for (vid, svi) in surrounding {
                if id == vid {
                    // seeing oneself? skip
                    continue;
                }
                self.update_visibility(v, svi.m());
            }

            if v.haveArrived {
                // vehicle has left the simulation; remove
                if haveOutput {
                    self.write_output(&id, &v.seen, allRecognitions);
                }
                delete_vehicle_info(vi);
                S_VEHICLES.with(|m| m.borrow_mut().remove(&id));
            } else {
                // vehicle is still in the simulation; reset state
                let n = v.updates.len();
                v.updates.drain(..n.saturating_sub(1));
            }
        }

        // remove arrived senders / reset state
        for (id, vi) in sender_snapshot() {
            if vi.m().haveArrived {
                delete_sender_info(vi);
                crate::microsim::devices::ms_device_b_tsender::S_VEHICLES.with(|m| m.borrow_mut().remove(&id));
            } else {
                let n = vi.m().updates.len();
                vi.m().updates.drain(..n.saturating_sub(1));
            }
        }
        delta_t()
    }

    /// C++: `void BTreceiverUpdate::updateVisibility(VehicleInformation &, MSDevice_BTsender::VehicleInformation &)`
    /// Rechecks the visibility for a given receiver/sender pair
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn update_visibility(&mut self, receiver: &mut MSDevice_BTreceiver_VehicleInformation, sender: &mut MSDevice_BTsender_VehicleInformation) {
        let receiverData = copy_state(receiver.updates.last().unwrap());
        let senderData = copy_state(sender.updates.last().unwrap());
        if !receiver.amOnNet || !sender.amOnNet {
            // at least one of the vehicles has left the simulation area for any reason
            if receiver.currentlySeen.contains_key(sender.get_id()) {
                self.leave_range(receiver, &receiverData, sender, &senderData, 0.);
            }
        }

        let oldReceiverPosition = receiver.updates.first().unwrap().position;
        let oldSenderPosition = sender.updates.first().unwrap().position;

        // let the other's current position be the one obtained by applying the relative direction vector to the initial position
        let senderDelta = senderData.position - oldSenderPosition;
        let receiverDelta = receiverData.position - oldReceiverPosition;
        let translatedSender = senderData.position - receiverDelta;
        // find crossing points
        let mut intersections: Vec<f64> = Vec::new();
        utils::geom::geom_helper::find_line_circle_intersections(&oldReceiverPosition, receiver.range, &oldSenderPosition, &translatedSender, &mut intersections);
        match intersections.len() {
            0 => {
                // no intersections -> other vehicle either stays within or beyond range
                if receiver.amOnNet && sender.amOnNet && receiverData.position.distance_to(&senderData.position) < receiver.range {
                    if !receiver.currentlySeen.contains_key(sender.get_id()) {
                        self.enter_range(0., &receiverData, sender.get_id(), &senderData, &mut receiver.currentlySeen);
                    } else {
                        let sd = receiver.currentlySeen[sender.get_id()];
                        self.add_recognition_point(simtime(), &receiverData, &senderData, sd);
                    }
                } else if receiver.currentlySeen.contains_key(sender.get_id()) {
                    self.leave_range(receiver, &receiverData, sender, &senderData, 0.);
                }
            }
            1 => {
                // one intersection -> other vehicle either enters or leaves the range
                let mut intersection1ReceiverData = copy_state(&receiverData);
                intersection1ReceiverData.position = oldReceiverPosition + receiverDelta * intersections[0];
                let mut intersection1SenderData = copy_state(&senderData);
                intersection1SenderData.position = oldSenderPosition + senderDelta * intersections[0];
                if receiver.currentlySeen.contains_key(sender.get_id()) {
                    self.leave_range(receiver, &intersection1ReceiverData, sender, &intersection1SenderData, (intersections[0] - 1.) * ts());
                } else {
                    self.enter_range((intersections[0] - 1.) * ts(), &intersection1ReceiverData, sender.get_id(), &intersection1SenderData, &mut receiver.currentlySeen);
                }
            }
            2 => {
                // two intersections -> other vehicle enters and leaves the range
                if !receiver.currentlySeen.contains_key(sender.get_id()) {
                    let mut intersectionReceiverData = copy_state(&receiverData);
                    intersectionReceiverData.position = oldReceiverPosition + receiverDelta * intersections[0];
                    let mut intersectionSenderData = copy_state(&senderData);
                    intersectionSenderData.position = oldSenderPosition + senderDelta * intersections[0];
                    self.enter_range((intersections[0] - 1.) * ts(), &intersectionReceiverData, sender.get_id(), &intersectionSenderData, &mut receiver.currentlySeen);
                    let last = intersections[intersections.len() - 1];
                    intersectionReceiverData.position = oldReceiverPosition + receiverDelta * last;
                    intersectionSenderData.position = oldSenderPosition + senderDelta * last;
                    self.leave_range(receiver, &intersectionReceiverData, sender, &intersectionSenderData, (last - 1.) * ts());
                } else {
                    utils::common::msg_handler::write_warning(&utils::tlf!(
                        "The vehicle '%' cannot be in the range of vehicle '%', leave, and enter it in one step.",
                        sender.get_id(),
                        receiver.get_id()
                    ));
                }
            }
            _ => {
                utils::common::msg_handler::write_warning("Nope, a circle cannot be crossed more often than twice by a line.");
            }
        }
    }

    /// C++: `void BTreceiverUpdate::enterRange(double, const MSDevice_BTsender::VehicleState &, const std::string &, const MSDevice_BTsender::VehicleState &, int &)`
    /// Informs the receiver about a sender entering it's radius
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn enter_range(&mut self, atOffset: f64, receiverState: &MSDevice_BTsender_VehicleState, senderID: &str, senderState: &MSDevice_BTsender_VehicleState, currentlySeen: &mut BTreeMap<String, Ptr<MSDevice_BTreceiver_SeenDevice>>) {
        let mp = MSDevice_BTreceiver_MeetingPoint::new(simtime() + atOffset, receiverState, senderState);
        let sd = Ptr::from_mut(Box::leak(Box::new(MSDevice_BTreceiver_SeenDevice::new(&mp))));
        currentlySeen.insert(senderID.to_string(), sd);
        self.add_recognition_point(simtime(), receiverState, senderState, sd);
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
    pub fn leave_range(&mut self, receiverInfo: &mut MSDevice_BTreceiver_VehicleInformation, receiverState: &MSDevice_BTsender_VehicleState, senderInfo: &mut MSDevice_BTsender_VehicleInformation, senderState: &MSDevice_BTsender_VehicleState, tOffset: f64) {
        let senderID = senderInfo.get_id().to_string();
        let sd = receiverInfo.currentlySeen[&senderID];
        // check whether the other was recognized
        self.add_recognition_point(simtime() + tOffset, receiverState, senderState, sd);
        // build leaving point
        sd.m().meetingEnd = Ptr::from_mut(Box::leak(Box::new(MSDevice_BTreceiver_MeetingPoint::new(steps2time(MSNet::get_instance().get_current_time_step()) + tOffset, receiverState, senderState))));
        let begin = sd.m().meetingBegin.observerState.routePos as usize;
        let end = (receiverState.routePos + 1) as usize;
        sd.m().receiverRoute = route_string(&receiverInfo.route, begin, end);
        let begin = sd.m().meetingBegin.seenState.routePos as usize;
        let end = (senderState.routePos + 1) as usize;
        sd.m().senderRoute = route_string(&senderInfo.route, begin, end);
        receiverInfo.seen.entry(senderID.clone()).or_default().push(sd);
        receiverInfo.currentlySeen.remove(&senderID);
    }

    /// C++: `void BTreceiverUpdate::addRecognitionPoint(const double, const MSDevice_BTsender::VehicleState &, const MSDevice_BTsender::VehicleState &, SeenDevice *) const` [const]
    /// Adds a point of recognition
    // void
    // MSDevice_BTreceiver::BTreceiverUpdate::addRecognitionPoint(const double tEnd, const MSDevice_BTsender::VehicleState& receiverState,
    //         const MSDevice_BTsender::VehicleState& senderState,
    //         SeenDevice* senderDevice) const {
    //     if (senderDevice->nextView == -1.) {
    //         senderDevice->nextView = senderDevice->lastView + inquiryDelaySlots(int(myOffTime / 0.000625 + 0.5)) * 0.000625;
    //     }
    //     if (tEnd > senderDevice->nextView) {
    //         senderDevice->lastView = senderDevice->nextView;
    //         MeetingPoint* mp = new MeetingPoint(tEnd, receiverState, senderState);
    //         senderDevice->recognitionPoints.push_back(mp);
    //         senderDevice->nextView = senderDevice->lastView + inquiryDelaySlots(int(myOffTime / 0.000625 + 0.5)) * 0.000625;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_recognition_point(&self, tEnd: f64, receiverState: &MSDevice_BTsender_VehicleState, senderState: &MSDevice_BTsender_VehicleState, senderDevice: Ptr<MSDevice_BTreceiver_SeenDevice>) {
        let sd = senderDevice.m();
        let backoffLimit = (MY_OFF_TIME.with(|c| c.get()) / 0.000625 + 0.5) as i32;
        if sd.nextView == -1. {
            sd.nextView = sd.lastView + MSDevice_BTreceiver::inquiry_delay_slots(backoffLimit) * 0.000625;
        }
        if tEnd > sd.nextView {
            sd.lastView = sd.nextView;
            let mp = Ptr::from_raw(Box::into_raw(Box::new(MSDevice_BTreceiver_MeetingPoint::new(tEnd, receiverState, senderState))));
            sd.recognitionPoints.push(mp);
            sd.nextView = sd.lastView + MSDevice_BTreceiver::inquiry_delay_slots(backoffLimit) * 0.000625;
        }
    }

    /// C++: `void BTreceiverUpdate::writeOutput(const std::string &, const int &, bool)`
    /// Writes the output
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn write_output(&mut self, id: &str, seen: &BTreeMap<String, Vec<Ptr<MSDevice_BTreceiver_SeenDevice>>>, allRecognitions: bool) {
        let os = OutputDevice::get_device_by_option("bt-output").unwrap();
        let mut os = os.borrow_mut();
        os.open_tag("bt").write_attr("id", id);
        for (itemID, devices) in seen {
            for device in devices {
                let device: &MSDevice_BTreceiver_SeenDevice = device.m();
                os.open_tag("seen").write_attr("id", itemID.as_str());
                let obsBeg = &device.meetingBegin.observerState;
                let seenBeg = &device.meetingBegin.seenState;
                os.write_attr("tBeg", &device.meetingBegin.t)
                    .write_attr("observerPosBeg", &obsBeg.position)
                    .write_attr("observerSpeedBeg", &obsBeg.speed)
                    .write_attr("observerLaneIDBeg", &obsBeg.laneID)
                    .write_attr("observerLanePosBeg", &obsBeg.lanePos)
                    .write_attr("seenPosBeg", &seenBeg.position)
                    .write_attr("seenSpeedBeg", &seenBeg.speed)
                    .write_attr("seenLaneIDBeg", &seenBeg.laneID)
                    .write_attr("seenLanePosBeg", &seenBeg.lanePos);
                let meetingEnd: &MSDevice_BTreceiver_MeetingPoint = device.meetingEnd.m();
                let obsEnd = &meetingEnd.observerState;
                let seenEnd = &meetingEnd.seenState;
                os.write_attr("tEnd", &meetingEnd.t)
                    .write_attr("observerPosEnd", &obsEnd.position)
                    .write_attr("observerSpeedEnd", &obsEnd.speed)
                    .write_attr("observerLaneIDEnd", &obsEnd.laneID)
                    .write_attr("observerLanePosEnd", &obsEnd.lanePos)
                    .write_attr("seenPosEnd", &seenEnd.position)
                    .write_attr("seenSpeedEnd", &seenEnd.speed)
                    .write_attr("seenLaneIDEnd", &seenEnd.laneID)
                    .write_attr("seenLanePosEnd", &seenEnd.lanePos)
                    .write_attr("observerRoute", &device.receiverRoute)
                    .write_attr("seenRoute", &device.senderRoute);
                for mp in device.recognitionPoints.iter() {
                    let mp: &MSDevice_BTreceiver_MeetingPoint = mp.m();
                    os.open_tag("recognitionPoint")
                        .write_attr("t", &mp.t)
                        .write_attr("observerPos", &mp.observerState.position)
                        .write_attr("observerSpeed", &mp.observerState.speed)
                        .write_attr("observerLaneID", &mp.observerState.laneID)
                        .write_attr("observerLanePos", &mp.observerState.lanePos)
                        .write_attr("seenPos", &mp.seenState.position)
                        .write_attr("seenSpeed", &mp.seenState.speed)
                        .write_attr("seenLaneID", &mp.seenState.laneID)
                        .write_attr("seenLanePos", &mp.seenState.lanePos)
                        .close_tag("");
                    if !allRecognitions {
                        break;
                    }
                }
                os.close_tag("");
            }
        }
        os.close_tag("");
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
        MY_RANGE.with(|c| c.get())
    }

    /// C++: `SumoRNG * MSDevice_BTreceiver::getRNG()` [static]
    // static SumoRNG* getRNG() {
    //         return &sRecognitionRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_rng() -> Ptr<SumoRNG> {
        S_RECOGNITION_RNG.with(|r| *r)
    }

    /// C++: `void MSDevice_BTreceiver::~MSDevice_BTreceiver() noexcept`
    /// Destructor.
    // MSDevice_BTreceiver::~MSDevice_BTreceiver() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        let id: String = veh.get_id().to_string();
        if reason == MSMoveReminder_Notification::NOTIFICATION_DEPARTED && !has_vehicle_info(&id) {
            let vi = Ptr::from_raw(Box::into_raw(Box::new(MSDevice_BTreceiver_VehicleInformation::new(&id, MY_RANGE.with(|c| c.get())))));
            S_VEHICLES.with(|m| m.borrow_mut().insert(id.clone(), vi));
            vehicle_info(&id).m().route.push(veh.get_edge());
        }
        if reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT && has_vehicle_info(&id) {
            vehicle_info(&id).m().amOnNet = true;
        }
        if reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT || reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            vehicle_info(&id).m().route.push(veh.get_edge());
        }
        let location: String = MSDevice_BTsender::get_location(veh);
        vehicle_info(&id).m().updates.push(MSDevice_BTsender_VehicleState::new(veh.get_speed(), &veh.get_position(0.0), &location, veh.get_position_on_lane(), veh.get_route_position()));
        true
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
        let id: String = veh.get_id().to_string();
        if !has_vehicle_info(&id) {
            utils::common::msg_handler::write_warning(&utils::tlf!("btreceiver: Can not update position of vehicle '%' which is not on the road.", veh.get_id()));
            return true;
        }
        let location: String = MSDevice_BTsender::get_location(veh);
        vehicle_info(&id).m().updates.push(MSDevice_BTsender_VehicleState::new(newSpeed, &veh.get_position(0.0), &location, newPos, veh.get_route_position()));
        true
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
        if reason < MSMoveReminder_Notification::NOTIFICATION_TELEPORT {
            return true;
        }
        let id: String = veh.get_id().to_string();
        if !has_vehicle_info(&id) {
            utils::common::msg_handler::write_warning(&utils::tlf!("btreceiver: Can not update position of vehicle '%' which is not on the road.", veh.get_id()));
            return true;
        }
        let location: String = MSDevice_BTsender::get_location(veh);
        vehicle_info(&id).m().updates.push(MSDevice_BTsender_VehicleState::new(veh.get_speed(), &veh.get_position(0.0), &location, veh.get_position_on_lane(), veh.get_route_position()));
        if reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT {
            vehicle_info(&id).m().amOnNet = false;
        }
        if reason >= MSMoveReminder_Notification::NOTIFICATION_ARRIVED {
            vehicle_info(&id).m().amOnNet = false;
            vehicle_info(&id).m().haveArrived = true;
        }
        true
    }

    /// C++: `void MSDevice_BTreceiver::cleanUp(int &, int &)` [static]
    /// Clears the given containers deleting the stored items
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn clean_up(c: &mut BTreeMap<String, Ptr<MSDevice_BTreceiver_SeenDevice>>, s: &mut BTreeMap<String, Vec<Ptr<MSDevice_BTreceiver_SeenDevice>>>) {
        // C++ only declares cleanUp (no definition in MSDevice_BTreceiver.cpp); body follows the header doc
        for (_, sd) in std::mem::take(c) {
            delete_seen(sd);
        }
        for (_, list) in std::mem::take(s) {
            for sd in list {
                delete_seen(sd);
            }
        }
    }

    /// C++: `SumoRNG * MSDevice_BTreceiver::getRecognitionRNG()` [static]
    // static SumoRNG* getRecognitionRNG() {
    //         return &sRecognitionRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_recognition_rng() -> Ptr<SumoRNG> {
        S_RECOGNITION_RNG.with(|r| *r)
    }

    /// C++: `void MSDevice_BTreceiver::hasSendingPersons()` [static]
    // static void hasSendingPersons() {
    //         myHasPersons = true;
    //     }
    #[allow(unused_variables)]
    pub fn has_sending_persons() {
        MY_HAS_PERSONS.with(|c| c.set(true));
    }

    /// C++: `void MSDevice_BTreceiver::MSDevice_BTreceiver()` [protected]
    /// Constructor
    // MSDevice_BTreceiver() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSDevice_BTreceiver {}
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
        let rng = Self::get_recognition_rng();
        let phaseOffset = rng.m().rand_int(2047);
        let interlaced = rng.m().rand() < 0.7;
        let delaySlots = rng.m().rand() * 15.;
        let backoff = rng.m().rand_int(backoffLimit);
        if interlaced {
            return rng.m().rand() * 31. + backoff as f64;
        }
        if rng.m().rand_int(31) < 16 {
            // correct train for f0
            return delaySlots + backoff as f64;
        }
        if rng.m().rand_int(30) < 16 {
            // correct train for f1
            return (2048 - phaseOffset) as f64 + delaySlots + backoff as f64;
        }
        if rng.m().rand_int(29) < 16 {
            // f2 is in train A but has overlap with both trains
            if 2 * 2048 - phaseOffset + backoff < 4096 {
                return (2 * 2048 - phaseOffset) as f64 + delaySlots + backoff as f64;
            }
            // the following is wrong but should only happen in about 3% of the non-interlaced cases
            return (2 * 2048 - phaseOffset) as f64 + delaySlots + backoff as f64;
        }
        (2 * 2048) as f64 + delaySlots + backoff as f64
    }

    /// C++: `void MSDevice_BTreceiver::MSDevice_BTreceiver(const MSDevice_BTreceiver &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_BTreceiver) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("MSDevice_BTreceiver copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `MSDevice_BTreceiver & MSDevice_BTreceiver::operator=(const MSDevice_BTreceiver &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTreceiver) -> &mut MSDevice_BTreceiver {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MSDevice_BTreceiver assignment operator is invalidated in C++ and must not be used");
    }

}

/// C++ class `MSTransportableDevice_BTreceiver` (microsim/devices/MSDevice_BTreceiver.h)
#[repr(C)]
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
        MSDevice::insert_default_assignment_options("btreceiver", "Communication", oc, true);
    }

    /// C++: `void MSTransportableDevice_BTreceiver::buildDevices(MSTransportable &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_devices(t: &mut MSTransportable, into: &mut Vec<Ptr<MSTransportableDevice>>) {
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "btreceiver", t.get_id(), t.get_parameter(), t.get_vehicle_type().get_parameter(), false, true)
        });
        if equipped {
            let id = format!("btreceiver_{}", t.get_id());
            let device = Box::leak(Box::new(MSTransportableDevice_BTreceiver::new(t, &id)));
            into.push(Ptr::from_mut(&mut device.base2));
            MY_HAS_PERSONS.with(|c| c.set(true));
            init_once_from_options();
        }
    }

    /// C++: `const std::string MSTransportableDevice_BTreceiver::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btreceiver";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "btreceiver".to_string()
    }

    /// C++: `bool MSTransportableDevice_BTreceiver::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_enter(veh, reason, enteredLane)
    }

    /// C++: `bool MSTransportableDevice_BTreceiver::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTreceiver::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        self.base.notify_move(veh, oldPos, newPos, newSpeed)
    }

    /// C++: `bool MSTransportableDevice_BTreceiver::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_leave(veh, lastPos, reason, enteredLane)
    }

    /// C++: `void MSTransportableDevice_BTreceiver::MSTransportableDevice_BTreceiver(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_BTreceiver(MSTransportable& holder, const std::string& id) :
    //         MSTransportableDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        MSTransportableDevice_BTreceiver { base: MSDevice_BTreceiver::new(), base2: MSTransportableDevice::new(holder, id) }
    }

}

/// C++ class `MSVehicleDevice_BTreceiver` (microsim/devices/MSDevice_BTreceiver.h)
#[repr(C)]
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
        MSDevice::insert_default_assignment_options("btreceiver", "Communication", oc, false);

        oc.do_register("device.btreceiver.range", SumoOption::float(300.));
        oc.add_description("device.btreceiver.range", "Communication", "The range of the bt receiver");

        oc.do_register("device.btreceiver.all-recognitions", SumoOption::bool(false));
        oc.add_description("device.btreceiver.all-recognitions", "Communication", "Whether all recognition point shall be written");

        oc.do_register("device.btreceiver.offtime", SumoOption::float(0.64));
        oc.add_description("device.btreceiver.offtime", "Communication", "The offtime used for calculating detection probability (in seconds)");

        MY_WAS_INITIALISED.with(|c| c.set(false));
        MY_HAS_PERSONS.with(|c| c.set(false));
    }

    /// C++: `void MSVehicleDevice_BTreceiver::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "btreceiver", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), false, false)
        });
        if equipped {
            let id = format!("btreceiver_{}", v.get_id());
            let device = Box::leak(Box::new(MSVehicleDevice_BTreceiver::new(v, &id)));
            into.push(Ptr::from_mut(&mut device.base2));
            init_once_from_options();
        }
    }

    /// C++: `const std::string MSVehicleDevice_BTreceiver::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btreceiver";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "btreceiver".to_string()
    }

    /// C++: `bool MSVehicleDevice_BTreceiver::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_enter(veh, reason, enteredLane)
    }

    /// C++: `bool MSVehicleDevice_BTreceiver::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTreceiver::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        self.base.notify_move(veh, oldPos, newPos, newSpeed)
    }

    /// C++: `bool MSVehicleDevice_BTreceiver::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTreceiver::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_leave(veh, lastPos, reason, enteredLane)
    }

    /// C++: `void MSVehicleDevice_BTreceiver::MSVehicleDevice_BTreceiver(SUMOVehicle &, const std::string &)`
    /// Constructor
    // MSVehicleDevice_BTreceiver(SUMOVehicle& holder, const std::string& id) :
    //         MSVehicleDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSVehicleDevice_BTreceiver>().m().notify_enter(veh, reason, enteredLane)
        }
        fn mr_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
            Ptr::from_mut(mr).cast::<MSVehicleDevice_BTreceiver>().m().notify_move(veh, oldPos, newPos, newSpeed)
        }
        fn mr_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSVehicleDevice_BTreceiver>().m().notify_leave(veh, lastPos, reason, enteredLane)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: mr_notify_move,
            notify_leave: mr_notify_leave,
        };
        let mut this = MSVehicleDevice_BTreceiver { base: MSDevice_BTreceiver::new(), base2: MSVehicleDevice::new(holder, id) };
        this.base2.base.vt = &MS_MOVE_REMINDER_VT;
        this
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
        MSDevice_BTreceiver_MeetingPoint { t: _t, observerState: copy_state(_observerState), seenState: copy_state(_seenState) }
    }

    /// C++: `void MeetingPoint::~MeetingPoint() noexcept`
    /// Destructor
    // ~MeetingPoint() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `MeetingPoint & MeetingPoint::operator=(const MeetingPoint &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTreceiver_MeetingPoint) -> &mut MSDevice_BTreceiver_MeetingPoint {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MeetingPoint assignment operator is invalidated in C++ and must not be used");
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
    pub recognitionPoints: Vec<Ptr<MSDevice_BTreceiver_MeetingPoint>>,
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
        MSDevice_BTreceiver_SeenDevice {
            meetingBegin: copy_meeting_point(meetingBegin_),
            meetingEnd: Ptr::null(),
            lastView: meetingBegin_.t,
            nextView: -1.,
            recognitionPoints: Vec::new(),
            receiverRoute: String::new(),
            senderRoute: String::new(),
        }
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
        if !self.meetingEnd.is_null() {
            drop(unsafe { Box::from_raw(self.meetingEnd.as_raw()) });
        }
        for mp in self.recognitionPoints.drain(..) {
            drop(unsafe { Box::from_raw(mp.as_raw()) });
        }
    }

    /// C++: `SeenDevice & SeenDevice::operator=(const SeenDevice &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTreceiver_SeenDevice) -> &mut MSDevice_BTreceiver_SeenDevice {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("SeenDevice assignment operator is invalidated in C++ and must not be used");
    }

}

/// Stores the information of a vehicle
/// C++ class `MSDevice_BTreceiver::VehicleInformation` (microsim/devices/MSDevice_BTreceiver.h)
#[repr(C)]
pub struct MSDevice_BTreceiver_VehicleInformation {
    /// C++ base class `MSDevice_BTsender::VehicleInformation`
    pub base: MSDevice_BTsender_VehicleInformation,
    /// Recognition range of the vehicle
    pub range: f64,
    /// The map of devices seen by the vehicle at removal time
    pub currentlySeen: BTreeMap<String, Ptr<MSDevice_BTreceiver_SeenDevice>>,
    /// The past episodes of removed vehicle
    pub seen: BTreeMap<String, Vec<Ptr<MSDevice_BTreceiver_SeenDevice>>>,
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
        MSDevice_BTreceiver_VehicleInformation {
            base: MSDevice_BTsender_VehicleInformation::new(id),
            range: _range,
            currentlySeen: BTreeMap::new(),
            seen: BTreeMap::new(),
        }
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
        for (_, sd) in std::mem::take(&mut self.currentlySeen) {
            delete_seen(sd);
        }
        for (_, list) in std::mem::take(&mut self.seen) {
            for sd in list {
                delete_seen(sd);
            }
        }
    }

    /// C++: `void VehicleInformation::VehicleInformation(const VehicleInformation &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_BTreceiver_VehicleInformation) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("VehicleInformation copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `VehicleInformation & VehicleInformation::operator=(const VehicleInformation &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTreceiver_VehicleInformation) -> &mut MSDevice_BTreceiver_VehicleInformation {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("VehicleInformation assignment operator is invalidated in C++ and must not be used");
    }

}

