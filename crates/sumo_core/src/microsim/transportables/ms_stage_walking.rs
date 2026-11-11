//! Port of microsim/transportables/MSStageWalking.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A stage performing walking on a sequence of edges. The real movements are part of the pedestrian model.
/// C++ class `MSStageWalking` (microsim/transportables/MSStageWalking.h)
pub struct MSStageWalking {
    /// C++ base class `MSStageMoving`
    pub base: MSStageMoving,
    /// the time the person is walking
    pub myWalkingTime: SUMOTime,
    /// the time the person entered the edge
    pub myLastEdgeEntryTime: SUMOTime,
    /// the MoveReminders encountered while walking
    pub myMoveReminders: Vec<Ptr<MSMoveReminder>>,
    /// optional exit time tracking for vehroute output
    pub myExitTimes: Ptr<Vec<SUMOTime>>,
    /// distance walked on non-normal edges (i.e. walkingareas)
    pub myInternalDistance: f64,
}

impl std::ops::Deref for MSStageWalking {
    type Target = MSStageMoving;
    fn deref(&self) -> &MSStageMoving {
        &self.base
    }
}
impl std::ops::DerefMut for MSStageWalking {
    fn deref_mut(&mut self) -> &mut MSStageMoving {
        &mut self.base
    }
}

// C++ static members of MSStageWalking (port as module-level statics, see AGENTS.md):
//   static bool MSStageWalking::myWarnedInvalidTripinfo;

impl MSStageWalking {
    /// C++: `void MSStageWalking::MSStageWalking(const std::string &, const ConstMSEdgeVector &, MSStoppingPlace *, SUMOTime, double, double, double, double, int, const std::string &)`
    /// constructor
    // MSStageWalking::MSStageWalking(const std::string& personID,
    //                                const ConstMSEdgeVector& route,
    //                                MSStoppingPlace* toStop,
    //                                SUMOTime walkingTime, double speed,
    //                                double departPos, double arrivalPos, double departPosLat, int departLane,
    //                                const std::string& routeID) :
    //     MSStageMoving(MSStageType::WALKING, route, routeID, toStop, speed, departPos, arrivalPos, departPosLat, departLane),
    //     myWalkingTime(walkingTime),
    //     myExitTimes(nullptr),
    //     myInternalDistance(0) {
    //     myDepartPos = SUMOVehicleParameter::interpretEdgePos(departPos, route.front()->getLength(), SUMO_ATTR_DEPARTPOS,
    //                   "person '" + personID + "' walking from edge '" + route.front()->getID() + "'");
    //     myArrivalPos = SUMOVehicleParameter::interpretEdgePos(arrivalPos, route.back()->getLength(), SUMO_ATTR_ARRIVALPOS,
    //                    "person '" + personID + "' walking to edge '" + route.back()->getID() + "'");
    //     if (walkingTime > 0) {
    //         mySpeed = computeAverageSpeed();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(personID: &str, route: &ConstMSEdgeVector, toStop: Ptr<MSStoppingPlace>, walkingTime: SUMOTime, speed: f64, departPos: f64, arrivalPos: f64, departPosLat: f64, departLane: i32, routeID: &str) -> Self {
        todo!("port MSStageWalking::MSStageWalking")
    }

    /// C++: `void MSStageWalking::~MSStageWalking() noexcept` [virtual]
    /// destructor
    // MSStageWalking::~MSStageWalking() {
    //     delete myExitTimes;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSStageWalking::~MSStageWalking")
    }

    /// C++: `MSStage * MSStageWalking::clone() const` [const, virtual]
    // MSStage*
    // MSStageWalking::clone() const {
    //     std::vector<const MSEdge*> route = myRoute;
    //     double departPos = myDepartPos;
    //     double arrivalPos = myArrivalPos;
    //     int departLane = myDepartLane;
    //     if (myRouteID != "" && MSRoute::distDictionary(myRouteID) != nullptr) {
    //         route = MSRoute::dictionary(myRouteID, MSRouteHandler::getParsingRNG())->getEdges();
    //         if (departPos > route[0]->getLength()) {
    //             WRITE_WARNINGF(TL("Adjusting departPos for cloned walk with routeDistribution '%'"), myRouteID);
    //             departPos = route[0]->getLength();
    //         }
    //         if (arrivalPos > route.back()->getLength()) {
    //             WRITE_WARNINGF(TL("Adjusting arrivalPos for cloned walk with routeDistribution '%'"), myRouteID);
    //             arrivalPos = route.back()->getLength();
    //         }
    //         if (departLane >= route[0]->getNumLanes()) {
    //             WRITE_WARNINGF(TL("Adjusting departLane for cloned walk with routeDistribution '%'"), myRouteID);
    //             departLane = route[0]->getNumLanes() - 1;
    //         }
    //     }
    //     MSStage* clon = new MSStageWalking("dummyID", route, myDestinationStop, myWalkingTime, mySpeed, departPos, arrivalPos, myDepartPosLat, departLane, myRouteID);
    //     clon->setParameters(*this);
    //     return clon;
    // }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        todo!("port MSStageWalking::clone")
    }

    /// C++: `bool MSStageWalking::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual]
    /// proceeds to the next step
    // bool
    // MSStageWalking::proceed(MSNet* net, MSTransportable* person, SUMOTime now, MSStage* previous) {
    //     MSTransportableControl& pControl = net->getPersonControl();
    //     myDeparted = now;
    //     myRouteStep = myRoute.begin();
    //     myLastEdgeEntryTime = now;
    //     if (myWalkingTime == 0) {
    //         return person->proceed(net, now);
    //     }
    //     if (previous->getEdgePos(now) >= 0 && previous->getEdge() == *myRouteStep) {
    //         // we need to adapt to the arrival position of the vehicle unless we have an explicit access
    //         myDepartPos = previous->getEdgePos(now);
    //         if (pControl.getMovementModel()->usingDoors()) {
    //             myDepartPosLat = previous->getEdgePosLat(now);
    //         }
    //         if (myWalkingTime > 0) {
    //             mySpeed = computeAverageSpeed();
    //         }
    //     }
    //     myPState = pControl.getMovementModel()->add(person, this, now);
    //     if (myPState == nullptr) {
    //         return false;
    //     }
    //     if (previous->getStageType() != MSStageType::WALKING || previous->getEdge() != getEdge()) {
    //         // we only need new move reminders if we are walking a different edge (else it is probably a rerouting)
    //         activateEntryReminders(person, true);
    //     }
    //     if (OptionsCont::getOptions().getBool("vehroute-output.exit-times")) {
    //         myExitTimes = new std::vector<SUMOTime>();
    //     }
    //     (*myRouteStep)->addTransportable(person);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, person: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        todo!("port MSStageWalking::proceed")
    }

    /// C++: `void MSStageWalking::abort(MSTransportable *)` [virtual]
    /// abort this stage (TraCI)
    // void
    // MSStageWalking::abort(MSTransportable*) {
    //     MSNet::getInstance()->getPersonControl().getMovementModel()->remove(myPState);
    // }
    #[allow(unused_variables)]
    pub fn abort(&mut self, arg0: Ptr<MSTransportable>) {
        todo!("port MSStageWalking::abort")
    }

    /// C++: `void MSStageWalking::setSpeed(double)` [virtual]
    /// sets the walking speed (ignored in other stages)
    // void
    // MSStageWalking::setSpeed(double speed) {
    //     mySpeed = speed;
    // }
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, speed: f64) {
        todo!("port MSStageWalking::setSpeed")
    }

    /// C++: `double MSStageWalking::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double getDistance() const {
    //         return walkDistance();
    //     }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        todo!("port MSStageWalking::getDistance")
    }

    /// C++: `int MSStageWalking::getRoutePosition() const` [const, virtual]
    /// return index of current edge within route
    // int
    // MSStageWalking::getRoutePosition() const {
    //     return (int)(myRouteStep - myRoute.begin());
    // }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        todo!("port MSStageWalking::getRoutePosition")
    }

    /// C++: `std::string MSStageWalking::getStageDescription(const bool) const` [const, virtual]
    // std::string getStageDescription(const bool isPerson) const {
    //         UNUSED_PARAMETER(isPerson);
    //         return "walking";
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        todo!("port MSStageWalking::getStageDescription")
    }

    /// C++: `std::string MSStageWalking::getStageSummary(const bool) const` [const, virtual]
    // std::string
    // MSStageWalking::getStageSummary(const bool /* isPerson */) const {
    //     const std::string dest = (getDestinationStop() == nullptr ?
    //                               " edge '" + getDestination()->getID() + "'" :
    //                               " stop '" + getDestinationStop()->getID() + "'" + (
    //                                   getDestinationStop()->getMyName() != "" ? " (" + getDestinationStop()->getMyName() + ")" : ""));
    //     return "walking to " + dest;
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        todo!("port MSStageWalking::getStageSummary")
    }

    /// C++: `void MSStageWalking::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    /// Saves the current state into the given stream
    // void
    // MSStageWalking::saveState(std::ostringstream& out, MSTransportable* /*transportable*/) {
    //     out << " " << myDeparted << " " << (myRouteStep - myRoute.begin()) << " " << myLastEdgeEntryTime;
    //     if (myExitTimes != nullptr) {
    //         out << " " << myExitTimes->size();
    //         for (SUMOTime t : *myExitTimes) {
    //             out << " " << t;
    //         }
    //     } else {
    //         out << " " << -1;
    //     }
    //     myPState->saveState(out);
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        todo!("port MSStageWalking::saveState")
    }

    /// C++: `void MSStageWalking::loadState(MSTransportable *, std::istringstream &)` [virtual]
    /// Reconstructs the current state
    // void
    // MSStageWalking::loadState(MSTransportable* transportable, std::istringstream& state) {
    //     int stepIdx;
    //     state >> myDeparted >> stepIdx >> myLastEdgeEntryTime;
    //     int exitTimesSize;
    //     state >> exitTimesSize;
    //     if (exitTimesSize >= 0) {
    //         myExitTimes = new std::vector<SUMOTime>();
    //         SUMOTime t;
    //         for (int i = 0; i < exitTimesSize; i++) {
    //             state >> t;
    //             myExitTimes->push_back(t);
    //         }
    //         if (!OptionsCont::getOptions().getBool("vehroute-output.exit-times")) {
    //             delete myExitTimes;
    //             myExitTimes = nullptr;
    //         }
    //     }
    //     myRouteStep = myRoute.begin() + stepIdx;
    //     myPState = MSNet::getInstance()->getPersonControl().getMovementModel()->loadState(transportable, this, state);
    //     if (myPState->getLane() && !myPState->getLane()->isNormal()) {
    //         myCurrentInternalEdge = &myPState->getLane()->getEdge();
    //         myCurrentInternalEdge->addTransportable(transportable);
    //     } else {
    //         (*myRouteStep)->addTransportable(transportable);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut Unmapped /* std::istringstream */) {
        todo!("port MSStageWalking::loadState")
    }

    /// C++: `void MSStageWalking::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSStageWalking::tripInfoOutput(OutputDevice& os, const MSTransportable* const person) const {
    //     if (!myWarnedInvalidTripinfo && MSNet::getInstance()->getPersonControl().getMovementModel()->usingShortcuts()) {
    //         WRITE_WARNING(TL("The pedestrian model uses infrastructure which is not in the network, timeLoss and routeLength may be invalid."));
    //         myWarnedInvalidTripinfo = true;
    //     }
    //     const double distance = walkDistance(true);
    //     const double maxSpeed = getMaxSpeed(person);
    //     const SUMOTime duration = myArrived - myDeparted;
    //     const SUMOTime timeLoss = getTimeLoss(person);
    //     MSDevice_Tripinfo::addPedestrianData(distance, duration, timeLoss);
    //     os.openTag("walk");
    //     os.writeAttr("depart", myDeparted >= 0 ? time2string(myDeparted) : "-1");
    //     os.writeAttr("departPos", myDepartPos);
    //     os.writeAttr("arrival", myArrived >= 0 ? time2string(myArrived) : "-1");
    //     os.writeAttr("arrivalPos", myArrived >= 0 ? toString(myArrivalPos) : "-1");
    //     os.writeAttr("duration", myDeparted < 0 ? "-1" :
    //                  time2string(myArrived >= 0 ? duration : MSNet::getInstance()->getCurrentTimeStep() - myDeparted));
    //     os.writeAttr("routeLength", myArrived >= 0 ? toString(distance) : "-1");
    //     os.writeAttr("timeLoss", time2string(timeLoss));
    //     os.writeAttr("maxSpeed", maxSpeed);
    //     os.writeAttr("waitingTime", time2string(getTotalWaitingTime()));
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        todo!("port MSStageWalking::tripInfoOutput")
    }

    /// C++: `void MSStageWalking::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual]
    /// Called on writing vehroute output
    // void
    // MSStageWalking::routeOutput(const bool /* isPerson */, OutputDevice& os, const bool withRouteLength, const MSStage* const /* previous */, const bool withTiming, const bool saveState) const {
    //     os.openTag("walk").writeAttr(SUMO_ATTR_EDGES, myRoute);
    //     std::string comment = "";
    //     if (myDestinationStop != nullptr) {
    //         os.writeAttr(toString(myDestinationStop->getElement()), myDestinationStop->getID());
    //         if (myDestinationStop->getMyName() != "") {
    //             comment =  " <!-- " + StringUtils::escapeXML(myDestinationStop->getMyName(), true) + " -->";
    //         }
    //     } else if (wasSet(VEHPARS_ARRIVALPOS_SET)) {
    //         os.writeAttr(SUMO_ATTR_ARRIVALPOS, myArrivalPos);
    //     }
    //     if (myWalkingTime > 0) {
    //         os.writeAttr(SUMO_ATTR_DURATION, time2string(myWalkingTime));
    //     } else if (mySpeed > 0) {
    //         os.writeAttr(SUMO_ATTR_SPEED, mySpeed);
    //     }
    //     if (withRouteLength) {
    //         if (myDeparted >= 0) {
    //             os.writeAttr("routeLength", walkDistance(true));
    //         } else {
    //             os.writeAttr("routeLength", "-1");
    //         }
    //     }
    //     if (myExitTimes != nullptr) {
    //         std::vector<std::string> exits;
    //         for (SUMOTime t : *myExitTimes) {
    //             exits.push_back(time2string(t));
    //         }
    //         std::vector<std::string> missing(MAX2(0, (int)myRoute.size() - (int)myExitTimes->size()), "-1");
    //         exits.insert(exits.end(), missing.begin(), missing.end());
    //         os.writeAttr("exitTimes", exits);
    //     }
    //     if (withTiming) {
    //         os.writeAttr(SUMO_ATTR_STARTED, myDeparted >= 0 ? time2string(myDeparted) : "-1");
    //         os.writeAttr(SUMO_ATTR_ENDED, myArrived >= 0 ? time2string(myArrived) : "-1");
    //     }
    //     if (OptionsCont::getOptions().getBool("vehroute-output.cost")) {
    //         os.writeAttr(SUMO_ATTR_COST, getCosts());
    //     }
    //     if (saveState && getTotalWaitingTime() > 0) {
    //         os.writeAttr("waitingTime", time2string(getTotalWaitingTime()));
    //     }
    //     os.closeTag(comment);
    // }
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        todo!("port MSStageWalking::routeOutput")
    }

    /// C++: `bool MSStageWalking::moveToNextEdge(MSTransportable *, SUMOTime, int, MSEdge *, const bool)` [virtual]
    /// move forward and return whether the person arrived
    // bool
    // MSStageWalking::moveToNextEdge(MSTransportable* person, SUMOTime currentTime, int prevDir, MSEdge* nextInternal, const bool isReplay) {
    //     ((MSEdge*)getEdge())->removeTransportable(person);
    //     const MSLane* lane = getSidewalk<MSEdge, MSLane>(getEdge());
    //     const bool arrived = myRouteStep == myRoute.end() - 1;
    //     if (lane != nullptr) {
    //         const double tl = person->getVehicleType().getLength();
    //         const double lastPos = (arrived
    //                                 ? (prevDir == MSPModel::FORWARD
    //                                    ? getArrivalPos() + tl
    //                                    : getArrivalPos() - tl)
    //                                 : person->getPositionOnLane());
    //         activateLeaveReminders(person, lane, lastPos, currentTime, arrived);
    //     }
    //     if (myExitTimes != nullptr && nextInternal == nullptr) {
    //         myExitTimes->push_back(currentTime);
    //     }
    //     myLastEdgeEntryTime = currentTime;
    //     //std::cout << SIMTIME << " moveToNextEdge person=" << person->getID() << "\n";
    //     if (myCurrentInternalEdge != nullptr) {
    //         myInternalDistance += (myPState->getPathLength() == 0 ? myCurrentInternalEdge->getLength() : myPState->getPathLength());
    //     }
    //     if (arrived) {
    //         MSPerson* p = dynamic_cast<MSPerson*>(person);
    //         if (!isReplay && p->hasInfluencer() && p->getInfluencer().isRemoteControlled()) {
    //             myCurrentInternalEdge = nextInternal;
    //             ((MSEdge*) getEdge())->addTransportable(person);
    //             return false;
    //         }
    //         if (myDestinationStop != nullptr) {
    //             myDestinationStop->addTransportable(person);
    //         }
    //         if (isReplay) {
    //             // cannot do this in the replay device because the person might get deleted below
    //             MSNet::getInstance()->getPersonControl().getMovementModel()->remove(myPState);
    //         }
    //         if (!person->proceed(MSNet::getInstance(), currentTime)) {
    //             MSNet::getInstance()->getPersonControl().erase(person);
    //         }
    //         //std::cout << " end walk. myRouteStep=" << (*myRouteStep)->getID() << "\n";
    //         return true;
    //     } else {
    //         if (nextInternal == nullptr) {
    //             ++myRouteStep;
    //         }
    //         myCurrentInternalEdge = nextInternal;
    //         ((MSEdge*) getEdge())->addTransportable(person);
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_to_next_edge(&mut self, person: Ptr<MSTransportable>, currentTime: SUMOTime, prevDir: i32, nextInternal: Ptr<MSEdge>, isReplay: bool) -> bool {
        todo!("port MSStageWalking::moveToNextEdge")
    }

    /// C++: `void MSStageWalking::activateEntryReminders(MSTransportable *, const bool)` [virtual]
    // void
    // MSStageWalking::activateEntryReminders(MSTransportable* person, const bool isDepart) {
    //     const MSLane* const nextLane = getSidewalk<MSEdge, MSLane>(getEdge());
    //     if (nextLane != nullptr) {
    //         for (MSMoveReminder* const rem : nextLane->getMoveReminders()) {
    //             if (rem->notifyEnter(*person, isDepart ? MSMoveReminder::NOTIFICATION_DEPARTED : MSMoveReminder::NOTIFICATION_JUNCTION, nextLane)) {
    //                 myMoveReminders.push_back(rem);
    //             }
    //         }
    //     }
    //     if (hasParameter("rerouter")) {
    //         double minDist = std::numeric_limits<double>::max();
    //         MSTriggeredRerouter* nearest = nullptr;
    //         for (MSMoveReminder* const rem : myMoveReminders) {
    //             MSTriggeredRerouter* rerouter = dynamic_cast<MSTriggeredRerouter*>(rem);
    //             if (rerouter != nullptr) {
    //                 const double dist2 = rerouter->getPosition().distanceSquaredTo2D(person->getPosition());
    //                 if (dist2 < minDist) {
    //                     nearest = rerouter;
    //                     minDist = dist2;
    //                 }
    //             }
    //         }
    //         if (nearest != nullptr) {
    //             nearest->triggerRouting(*person, MSMoveReminder::NOTIFICATION_JUNCTION);
    //         }
    //         // TODO maybe removal of the reminders? Or can we rely on movetonextedge to clean everything up?
    //     }
    // }
    #[allow(unused_variables)]
    pub fn activate_entry_reminders(&mut self, person: Ptr<MSTransportable>, isDepart: bool) {
        todo!("port MSStageWalking::activateEntryReminders")
    }

    /// C++: `void MSStageWalking::activateMoveReminders(MSTransportable *, double, double, double)`
    // void
    // MSStageWalking::activateMoveReminders(MSTransportable* person, double oldPos, double newPos, double newSpeed) {
    //     for (std::vector<MSMoveReminder*>::iterator rem = myMoveReminders.begin(); rem != myMoveReminders.end();) {
    //         if ((*rem)->notifyMove(*person, oldPos, newPos, newSpeed)) {
    //             ++rem;
    //         } else {
    //             rem = myMoveReminders.erase(rem);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn activate_move_reminders(&mut self, person: Ptr<MSTransportable>, oldPos: f64, newPos: f64, newSpeed: f64) {
        todo!("port MSStageWalking::activateMoveReminders")
    }

    /// C++: `void MSStageWalking::activateLeaveReminders(MSTransportable *, const MSLane *, double, SUMOTime, bool)`
    // void
    // MSStageWalking::activateLeaveReminders(MSTransportable* person, const MSLane* lane, double lastPos, SUMOTime t, bool arrived) {
    //     MSMoveReminder::Notification notification = arrived ? MSMoveReminder::NOTIFICATION_ARRIVED : MSMoveReminder::NOTIFICATION_JUNCTION;
    //     for (std::vector<MSMoveReminder*>::iterator rem = myMoveReminders.begin(); rem != myMoveReminders.end();) {
    //         (*rem)->updateDetector(*person, 0.0, lane->getLength(), myLastEdgeEntryTime, t, t, true);
    //         if ((*rem)->notifyLeave(*person, lastPos, notification)) {
    //             ++rem;
    //         } else {
    //             rem = myMoveReminders.erase(rem);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn activate_leave_reminders(&mut self, person: Ptr<MSTransportable>, lane: Ptr<MSLane>, lastPos: f64, t: SUMOTime, arrived: bool) {
        todo!("port MSStageWalking::activateLeaveReminders")
    }

    /// C++: `double MSStageWalking::getMaxSpeed(const MSTransportable *const) const` [const, virtual]
    // double
    // MSStageWalking::getMaxSpeed(const MSTransportable* const person) const {
    //     return mySpeed >= 0 ? mySpeed : (person != nullptr ? person->getMaxSpeed() : DEFAULT_PEDESTRIAN_SPEED);
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self, person: Ptr<MSTransportable>) -> f64 {
        todo!("port MSStageWalking::getMaxSpeed")
    }

    /// C++: `double MSStageWalking::getArrivalPos() const` [const, virtual]
    // inline double getArrivalPos() const {
    //         return myArrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        todo!("port MSStageWalking::getArrivalPos")
    }

    /// C++: `const MSEdge * MSStageWalking::getNextRouteEdge() const` [const, virtual]
    // inline const MSEdge* getNextRouteEdge() const {
    //         return myRouteStep == myRoute.end() - 1 ? nullptr : *(myRouteStep + 1);
    //     }
    #[allow(unused_variables)]
    pub fn get_next_route_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSStageWalking::getNextRouteEdge")
    }

    /// C++: `bool MSStageWalking::isWalk() const` [const, virtual]
    /// Whether the transportable is walking
    // bool isWalk() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_walk(&self) -> bool {
        todo!("port MSStageWalking::isWalk")
    }

    /// C++: `SUMOTime MSStageWalking::getTimeLoss(const MSTransportable *) const` [const, virtual]
    // SUMOTime
    // MSStageWalking::getTimeLoss(const MSTransportable* transportable) const {
    //     SUMOTime timeLoss = myArrived == -1 ? 0 : getDuration() - TIME2STEPS(walkDistance(true) / getMaxSpeed(transportable));
    //     if (timeLoss < 0 && timeLoss > TIME2STEPS(-0.1)) {
    //         // avoid negative timeLoss due to rounding errors
    //         timeLoss = 0;
    //     }
    //     return timeLoss;
    // }
    #[allow(unused_variables)]
    pub fn get_time_loss(&self, transportable: Ptr<MSTransportable>) -> SUMOTime {
        todo!("port MSStageWalking::getTimeLoss")
    }

    /// C++: `bool MSStageWalking::equals(const MSStage &) const` [const, virtual]
    // bool equals(const MSStage& s) const {
    //         if (!MSStageMoving::equals(s)) {
    //             return false;
    //         }
    //         // this is safe because MSStage already checked that the type fits
    //         const MSStageWalking& sw = static_cast<const MSStageWalking&>(s);
    //         return myWalkingTime == sw.myWalkingTime;
    //     }
    #[allow(unused_variables)]
    pub fn equals(&self, s: &MSStage) -> bool {
        todo!("port MSStageWalking::equals")
    }

    /// C++: `void MSStageWalking::setExitTimes(std::vector<SUMOTime> *)`
    // void setExitTimes(std::vector<SUMOTime>* exitTimes) {
    //         myExitTimes = exitTimes;
    //     }
    #[allow(unused_variables)]
    pub fn set_exit_times(&mut self, exitTimes: Ptr<Vec<SUMOTime>>) {
        todo!("port MSStageWalking::setExitTimes")
    }

    /// C++: `double MSStageWalking::walkDistance(bool) const` [const, private]
    /// compute total walking distance
    // double
    // MSStageWalking::walkDistance(bool partial) const {
    //     double length = 0;
    //     auto endIt = partial && myArrived < 0 ? myRouteStep + 1 : myRoute.end();
    //     for (ConstMSEdgeVector::const_iterator i = myRoute.begin(); i != endIt; ++i) {
    //         length += (*i)->getLength();
    //     }
    //     if (myRoute.size() > 1 && MSNet::getInstance()->getPersonControl().getMovementModel()->usingInternalLanes()) {
    //         if (myInternalDistance > 0) {
    //             length += myInternalDistance;
    //         } else {
    //             // use lower bound for distance to pass the intersection
    //             for (ConstMSEdgeVector::const_iterator i = myRoute.begin(); i != endIt - 1; ++i) {
    //                 const MSEdge* fromEdge = *i;
    //                 const MSEdge* toEdge = *(i + 1);
    //                 const MSLane* from = getSidewalk<MSEdge, MSLane>(fromEdge);
    //                 const MSLane* to = getSidewalk<MSEdge, MSLane>(toEdge);
    //                 Position fromPos;
    //                 Position toPos;
    //                 if (from != nullptr && to != nullptr) {
    //                     if (fromEdge->getToJunction() == toEdge->getFromJunction()) {
    //                         fromPos = from->getShape().back();
    //                         toPos = to->getShape().front();
    //                     } else if (fromEdge->getToJunction() == toEdge->getToJunction()) {
    //                         fromPos = from->getShape().back();
    //                         toPos = to->getShape().back();
    //                     } else if (fromEdge->getFromJunction() == toEdge->getFromJunction()) {
    //                         fromPos = from->getShape().front();
    //                         toPos = to->getShape().front();
    //                     } else if (fromEdge->getFromJunction() == toEdge->getToJunction()) {
    //                         fromPos = from->getShape().front();
    //                         toPos = to->getShape().back();
    //                     }
    //                     //std::cout << " from=" << from->getID() << " to=" << to->getID() << " junctionLength=" << fromPos.distanceTo2D(toPos) << "\n";
    //                     length += fromPos.distanceTo2D(toPos);
    //                 }
    //             }
    //         }
    //     }
    //     // determine walking direction for depart and arrival
    //     int dummy = 0;
    //     const int departFwdArrivalDir = MSPModel::canTraverse(MSPModel::FORWARD, myRoute, dummy);
    //     const int departBwdArrivalDir = MSPModel::canTraverse(MSPModel::BACKWARD, myRoute, dummy);
    //     const bool mayStartForward = departFwdArrivalDir != MSPModel::UNDEFINED_DIRECTION;
    //     const bool mayStartBackward = departBwdArrivalDir != MSPModel::UNDEFINED_DIRECTION;
    //     const double arrivalPos = partial && myArrived < 0 ? getEdgePos(SIMSTEP) : myArrivalPos;
    //     const double lengthFwd = (length - myDepartPos - (
    //                                   departFwdArrivalDir == MSPModel::BACKWARD
    //                                   ? arrivalPos
    //                                   : myRoute.back()->getLength() - arrivalPos));
    //     const double lengthBwd = (length - (myRoute.front()->getLength() - myDepartPos) - (
    //                                   departBwdArrivalDir == MSPModel::BACKWARD
    //                                   ? arrivalPos
    //                                   : myRoute.back()->getLength() - arrivalPos));
    //     //std::cout << " length=" << length << " lengthFwd=" << lengthFwd << " lengthBwd=" << lengthBwd << " mayStartForward=" << mayStartForward << " mayStartBackward=" << mayStartBackward << "\n";
    //
    //     if (myRoute.size() == 1) {
    //         if (myDepartPos > myArrivalPos) {
    //             length = lengthBwd;
    //         } else {
    //             length = lengthFwd;
    //         }
    //     } else {
    //         if (mayStartForward && mayStartBackward) {
    //             length = lengthFwd < lengthBwd ? lengthFwd : lengthBwd;
    //         } else if (mayStartForward) {
    //             length = lengthFwd;
    //         } else if (mayStartBackward) {
    //             length = lengthBwd;
    //         } else {
    //             length = lengthFwd;
    //         }
    //     }
    //     //std::cout << SIMTIME << " route=" << toString(myRoute)
    //     //    << " depPos=" << myDepartPos << " arPos=" << myArrivalPos
    //     //    << " dFwdADir=" << departFwdArrivalDir
    //     //    << " dBwdADir=" << departBwdArrivalDir
    //     //    << " lengthFwd=" << lengthFwd
    //     //    << " lengthBwd=" << lengthBwd
    //     //    << "\n";
    //
    //     return MAX2(POSITION_EPS, length);
    // }
    #[allow(unused_variables)]
    pub fn walk_distance(&self, partial: bool) -> f64 {
        todo!("port MSStageWalking::walkDistance")
    }

    /// C++: `double MSStageWalking::computeAverageSpeed() const` [const, private]
    // double
    // MSStageWalking::computeAverageSpeed() const {
    //     return walkDistance() / STEPS2TIME(myWalkingTime + 1); // avoid systematic rounding errors
    // }
    #[allow(unused_variables)]
    pub fn compute_average_speed(&self) -> f64 {
        todo!("port MSStageWalking::computeAverageSpeed")
    }

    /// C++: `void MSStageWalking::MSStageWalking(const MSStageWalking &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageWalking) -> Self {
        todo!("port MSStageWalking::MSStageWalking")
    }

    /// C++: `MSStageWalking & MSStageWalking::operator=(const MSStageWalking &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageWalking) -> &mut MSStageWalking {
        todo!("port MSStageWalking::operator=")
    }

}

