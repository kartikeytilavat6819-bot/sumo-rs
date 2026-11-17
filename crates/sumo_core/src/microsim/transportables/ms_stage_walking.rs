//! Port of microsim/transportables/MSStageWalking.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::geom::POSITION_EPS;
use utils::common::to_string::f64_to_string_prec;
use utils::common::std_defs::g_precision;

// C++ constants (MSPModel.h, SUMOVehicleParameter.h): not exported as Rust items yet
const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_BACKWARD: i32 = -1;
const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;
const VEHPARS_ARRIVALPOS_SET: i64 = 2 << 12;

/// A stage performing walking on a sequence of edges. The real movements are part of the pedestrian model.
/// C++ class `MSStageWalking` (microsim/transportables/MSStageWalking.h)
#[repr(C)]
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

// shortcut: MSPModel has no virtual dispatch table yet, so walking calls go to MSPModel_Striping, the model that
// pedestrian.model=striping creates (the only one wired for walking here); upgrade when MSPModel gets a vtable
fn striping_model(model: Ptr<MSPModel>) -> Ptr<MSPModel_Striping> {
    if utils::options::options_cont::get_string("pedestrian.model") != "striping" {
        panic!("{}", ProcessError::new("walking stage needs pedestrian.model=striping, other pedestrian models are not wired yet"));
    }
    model.cast::<MSPModel_Striping>()
}

// shortcut: MSTransportableStateAdapter::getPathLength is virtual without dispatch; striping states are MSPModel_Striping_PState, upgrade with the vtable above
fn pstate_path_length(state: Ptr<MSTransportableStateAdapter>) -> f64 {
    state.cast::<MSPModel_Striping_PState>().get_path_length()
}

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
        let mut walk = MSStageWalking {
            base: MSStageMoving::new(MSStageType::WALKING, route, routeID, toStop, speed, departPos, arrivalPos, departPosLat, departLane),
            myWalkingTime: walkingTime,
            myLastEdgeEntryTime: 0,
            myMoveReminders: Vec::new(),
            myExitTimes: Ptr::null(),
            myInternalDistance: 0.,
        };
        let firstEdge = route[0];
        let lastEdge = route[route.len() - 1];
        walk.myDepartPos = SUMOVehicleParameter::interpret_edge_pos(departPos, firstEdge.get_length(), SUMO_ATTR_DEPARTPOS, &format!("person '{}' walking from edge '{}'", personID, firstEdge.get_id()), false);
        walk.myArrivalPos = SUMOVehicleParameter::interpret_edge_pos(arrivalPos, lastEdge.get_length(), SUMO_ATTR_ARRIVALPOS, &format!("person '{}' walking to edge '{}'", personID, lastEdge.get_id()), false);
        if walkingTime > 0 {
            walk.mySpeed = walk.compute_average_speed();
        }
        walk
    }

    /// C++: `void MSStageWalking::~MSStageWalking() noexcept` [virtual]
    /// destructor
    // MSStageWalking::~MSStageWalking() {
    //     delete myExitTimes;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: C++ deletes myExitTimes here; ownership of the exit-times vector is not modelled yet
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
        // shortcut: the routeDistribution lookup (MSRoute::distDictionary/dictionary) is not ported, so the clone keeps the current route
        let route = self.myRoute.clone();
        let departPos = self.myDepartPos;
        let arrivalPos = self.myArrivalPos;
        let departLane = self.myDepartLane;
        let routeID = self.myRouteID.clone();
        let clon = MSStageWalking::new("dummyID", &route, self.myDestinationStop, self.myWalkingTime, self.mySpeed, departPos, arrivalPos, self.myDepartPosLat, departLane, &routeID);
        let clonPtr = Ptr::from_mut(Box::leak(Box::new(clon))).cast::<MSStage>();
        clonPtr.m().set_parameters(self);
        clonPtr
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
        self.myDeparted = now;
        self.myRouteStep = 0;
        self.myLastEdgeEntryTime = now;
        if self.myWalkingTime == 0 {
            return person.m().proceed(net, now, false);
        }
        if previous.get_edge_pos(now) >= 0. && previous.get_edge() == self.get_edge() {
            // we need to adapt to the arrival position of the vehicle unless we have an explicit access
            self.myDepartPos = previous.get_edge_pos(now);
            if net.m().get_person_control().get_movement_model().m().using_doors() {
                self.myDepartPosLat = previous.get_edge_pos_lat(now);
            }
            if self.myWalkingTime > 0 {
                self.mySpeed = self.compute_average_speed();
            }
        }
        self.myPState = striping_model(net.m().get_person_control().get_movement_model()).m().add(person, Ptr::from_mut(&mut self.base), now);
        if self.myPState.is_null() {
            return false;
        }
        if previous.get_stage_type() != MSStageType::WALKING || previous.get_edge() != self.get_edge() {
            // we only need new move reminders if we are walking a different edge (else it is probably a rerouting)
            self.activate_entry_reminders(person, true);
        }
        if utils::options::options_cont::get_bool("vehroute-output.exit-times") {
            self.myExitTimes = Ptr::from_mut(Box::leak(Box::new(Vec::new())));
        }
        self.get_edge().m().add_transportable(person);
        true
    }

    /// C++: `void MSStageWalking::abort(MSTransportable *)` [virtual]
    /// abort this stage (TraCI)
    // void
    // MSStageWalking::abort(MSTransportable*) {
    //     MSNet::getInstance()->getPersonControl().getMovementModel()->remove(myPState);
    // }
    #[allow(unused_variables)]
    pub fn abort(&mut self, arg0: Ptr<MSTransportable>) {
        panic!("MSStageWalking::abort needs the pedestrian movement model (MSPModel::remove), which is not ported yet");
    }

    /// C++: `void MSStageWalking::setSpeed(double)` [virtual]
    /// sets the walking speed (ignored in other stages)
    // void
    // MSStageWalking::setSpeed(double speed) {
    //     mySpeed = speed;
    // }
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, speed: f64) {
        self.mySpeed = speed;
    }

    /// C++: `double MSStageWalking::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double getDistance() const {
    //         return walkDistance();
    //     }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        self.walk_distance(false)
    }

    /// C++: `int MSStageWalking::getRoutePosition() const` [const, virtual]
    /// return index of current edge within route
    // int
    // MSStageWalking::getRoutePosition() const {
    //     return (int)(myRouteStep - myRoute.begin());
    // }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        panic!("MSStageWalking::getRoutePosition needs MSStageMoving::myRouteStep, which is Unmapped (std::vector iterator); not ported yet");
    }

    /// C++: `std::string MSStageWalking::getStageDescription(const bool) const` [const, virtual]
    // std::string getStageDescription(const bool isPerson) const {
    //         UNUSED_PARAMETER(isPerson);
    //         return "walking";
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        "walking".to_string()
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
        let dest = if self.get_destination_stop().is_null() {
            format!(" edge '{}'", self.get_destination().get_id())
        } else {
            let stop = self.get_destination_stop();
            let name = if stop.get_my_name() != "" { format!(" ({})", stop.get_my_name()) } else { String::new() };
            format!(" stop '{}'{}", stop.get_id(), name)
        };
        format!("walking to {}", dest)
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
    pub fn save_state(&mut self, out: &mut String /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        panic!("MSStageWalking::saveState needs MSStageMoving::myRouteStep (Unmapped iterator) and the movement model state (saveState); not ported yet");
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
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut std::str::SplitWhitespace<'_> /* std::istringstream */) {
        panic!("MSStageWalking::loadState needs MSStageMoving::myRouteStep (Unmapped iterator) and the movement model loadState; not ported yet");
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
        // shortcut: MSPModel::usingShortcuts() is assumed false (movement model not ported), so the invalid-tripinfo warning is never written
        let distance = self.walk_distance(true);
        let maxSpeed = self.get_max_speed(transportable);
        let duration = self.myArrived - self.myDeparted;
        let timeLoss = self.get_time_loss(transportable);
        // shortcut: MSDevice_Tripinfo::addPedestrianData is not ported yet, so the pedestrian totals are not updated
        os.open_tag("walk");
        let depart = if self.myDeparted >= 0 { time2string(self.myDeparted) } else { "-1".to_string() };
        os.write_attr("depart", &depart);
        os.write_attr("departPos", &self.myDepartPos);
        let arrival = if self.myArrived >= 0 { time2string(self.myArrived) } else { "-1".to_string() };
        os.write_attr("arrival", &arrival);
        let arrivalPos = if self.myArrived >= 0 { f64_to_string_prec(self.myArrivalPos, g_precision()) } else { "-1".to_string() };
        os.write_attr("arrivalPos", &arrivalPos);
        let durationStr = if self.myDeparted < 0 { "-1".to_string() } else { time2string(if self.myArrived >= 0 { duration } else { MSNet::get_instance().get_current_time_step() - self.myDeparted }) };
        os.write_attr("duration", &durationStr);
        let routeLength = if self.myArrived >= 0 { f64_to_string_prec(distance, g_precision()) } else { "-1".to_string() };
        os.write_attr("routeLength", &routeLength);
        os.write_attr("timeLoss", &time2string(timeLoss));
        os.write_attr("maxSpeed", &maxSpeed);
        os.write_attr("waitingTime", &time2string(self.get_total_waiting_time()));
        os.close_tag("");
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
        os.open_tag("walk");
        let edges: Vec<String> = self.myRoute.iter().map(|e| e.get_id().to_string()).collect();
        os.write_attr(SUMO_ATTR_EDGES, &edges.join(" "));
        let mut comment = String::new();
        if !self.get_destination_stop().is_null() {
            let stop = self.get_destination_stop();
            os.write_attr(utils::xml::sumo_xml_definitions::Tags.get_string(stop.get_element()), stop.get_id());
            if stop.get_my_name() != "" {
                comment = format!(" <!-- {} -->", utils::common::string_utils::escape_xml(stop.get_my_name(), true));
            }
        } else if self.was_set(VEHPARS_ARRIVALPOS_SET as i32) {
            os.write_attr(SUMO_ATTR_ARRIVALPOS, &self.myArrivalPos);
        }
        if self.myWalkingTime > 0 {
            os.write_attr(SUMO_ATTR_DURATION, &time2string(self.myWalkingTime));
        } else if self.mySpeed > 0. {
            os.write_attr(SUMO_ATTR_SPEED, &self.mySpeed);
        }
        if withRouteLength {
            if self.myDeparted >= 0 {
                os.write_attr("routeLength", &self.walk_distance(true));
            } else {
                os.write_attr("routeLength", "-1");
            }
        }
        if !self.myExitTimes.is_null() {
            let mut exits: Vec<String> = self.myExitTimes.iter().map(|t| time2string(*t)).collect();
            let missing = if self.myRoute.len() as i64 - self.myExitTimes.len() as i64 > 0 { self.myRoute.len() - self.myExitTimes.len() } else { 0 };
            for _ in 0..missing {
                exits.push("-1".to_string());
            }
            os.write_attr("exitTimes", &exits.join(" "));
        }
        if withTiming {
            let started = if self.myDeparted >= 0 { time2string(self.myDeparted) } else { "-1".to_string() };
            os.write_attr(SUMO_ATTR_STARTED, &started);
            let ended = if self.myArrived >= 0 { time2string(self.myArrived) } else { "-1".to_string() };
            os.write_attr(SUMO_ATTR_ENDED, &ended);
        }
        if utils::options::options_cont::get_bool("vehroute-output.cost") {
            os.write_attr(SUMO_ATTR_COST, &self.get_costs());
        }
        if saveState && self.get_total_waiting_time() > 0 {
            os.write_attr("waitingTime", &time2string(self.get_total_waiting_time()));
        }
        os.close_tag(&comment);
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
        self.get_edge().m().remove_transportable(person);
        let lane = get_sidewalk_lane(self.get_edge(), SVC_PEDESTRIAN);
        let arrived = self.myRouteStep == self.myRoute.len() - 1;
        if !lane.is_null() {
            let tl = person.get_vehicle_type().get_length();
            let lastPos = if arrived {
                if prevDir == MSPMODEL_FORWARD { self.get_arrival_pos() + tl } else { self.get_arrival_pos() - tl }
            } else {
                person.get_position_on_lane()
            };
            self.activate_leave_reminders(person, lane, lastPos, currentTime, arrived);
        }
        if !self.myExitTimes.is_null() && nextInternal.is_null() {
            self.myExitTimes.m().push(currentTime);
        }
        self.myLastEdgeEntryTime = currentTime;
        if !self.myCurrentInternalEdge.is_null() {
            self.myInternalDistance += if pstate_path_length(self.myPState) == 0. { self.myCurrentInternalEdge.get_length() } else { pstate_path_length(self.myPState) };
        }
        if arrived {
            // shortcut: dynamic_cast<MSPerson*> is unchecked; only persons walk, so the cast is safe
            let p: Ptr<MSPerson> = person.cast::<MSPerson>();
            if !isReplay && p.has_influencer() && p.m().get_influencer().is_remote_controlled() {
                self.myCurrentInternalEdge = nextInternal;
                self.get_edge().m().add_transportable(person);
                return false;
            }
            if !self.myDestinationStop.is_null() {
                self.myDestinationStop.m().add_transportable(person, -1);
            }
            if isReplay {
                // cannot do this in the replay device because the person might get deleted below
                striping_model(MSNet::get_instance().m().get_person_control().get_movement_model()).m().remove(self.myPState);
            }
            if !person.m().proceed(MSNet::get_instance(), currentTime, false) {
                MSNet::get_instance().m().get_person_control().erase(person);
            }
            true
        } else {
            if nextInternal.is_null() {
                self.myRouteStep += 1;
            }
            self.myCurrentInternalEdge = nextInternal;
            self.get_edge().m().add_transportable(person);
            false
        }
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
        let nextLane = get_sidewalk_lane(self.get_edge(), SVC_PEDESTRIAN);
        if !nextLane.is_null() {
            for rem in nextLane.get_move_reminders().clone() {
                let reason = if isDepart { MSMoveReminder_Notification::NOTIFICATION_DEPARTED } else { MSMoveReminder_Notification::NOTIFICATION_JUNCTION };
                if rem.m().notify_enter(person.m(), reason, nextLane) {
                    self.myMoveReminders.push(rem);
                }
            }
        }
        // shortcut: the hasParameter("rerouter") branch (MSTriggeredRerouter::triggerRouting for the nearest rerouter) is not wired, upgrade when persons carry rerouter params
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
        panic!("MSStageWalking::activateMoveReminders needs the MSMoveReminder notifyMove API, which is not ported yet");
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
        let notification = if arrived { MSMoveReminder_Notification::NOTIFICATION_ARRIVED } else { MSMoveReminder_Notification::NOTIFICATION_JUNCTION };
        let mut i = 0;
        while i < self.myMoveReminders.len() {
            let rem = self.myMoveReminders[i];
            rem.m().update_detector(person.m(), 0.0, lane.get_length(), self.myLastEdgeEntryTime, t, t, true);
            if rem.m().notify_leave(person.m(), lastPos, notification, lane) {
                i += 1;
            } else {
                self.myMoveReminders.remove(i);
            }
        }
    }

    /// C++: `double MSStageWalking::getMaxSpeed(const MSTransportable *const) const` [const, virtual]
    // double
    // MSStageWalking::getMaxSpeed(const MSTransportable* const person) const {
    //     return mySpeed >= 0 ? mySpeed : (person != nullptr ? person->getMaxSpeed() : DEFAULT_PEDESTRIAN_SPEED);
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self, person: Ptr<MSTransportable>) -> f64 {
        if self.mySpeed >= 0. {
            self.mySpeed
        } else if !person.is_null() {
            person.get_max_speed()
        } else {
            DEFAULT_PEDESTRIAN_SPEED
        }
    }

    /// C++: `double MSStageWalking::getArrivalPos() const` [const, virtual]
    // inline double getArrivalPos() const {
    //         return myArrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        self.myArrivalPos
    }

    /// C++: `const MSEdge * MSStageWalking::getNextRouteEdge() const` [const, virtual]
    // inline const MSEdge* getNextRouteEdge() const {
    //         return myRouteStep == myRoute.end() - 1 ? nullptr : *(myRouteStep + 1);
    //     }
    #[allow(unused_variables)]
    pub fn get_next_route_edge(&self) -> Ptr<MSEdge> {
        panic!("MSStageWalking::getNextRouteEdge needs MSStageMoving::myRouteStep (Unmapped iterator); not ported yet");
    }

    /// C++: `bool MSStageWalking::isWalk() const` [const, virtual]
    /// Whether the transportable is walking
    // bool isWalk() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_walk(&self) -> bool {
        true
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
        let timeLoss = if self.myArrived == -1 { 0 } else { self.get_duration() - time2steps(self.walk_distance(true) / self.get_max_speed(transportable)) };
        if timeLoss < 0 && timeLoss > time2steps(-0.1) {
            // avoid negative timeLoss due to rounding errors
            return 0;
        }
        timeLoss
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
        if !self.base.equals_base(s) {
            return false;
        }
        // this is safe because MSStage already checked that the type fits
        let sw = Ptr::new(s).cast::<MSStageWalking>();
        self.myWalkingTime == sw.myWalkingTime
    }

    /// C++: `void MSStageWalking::setExitTimes(std::vector<SUMOTime> *)`
    // void setExitTimes(std::vector<SUMOTime>* exitTimes) {
    //         myExitTimes = exitTimes;
    //     }
    #[allow(unused_variables)]
    pub fn set_exit_times(&mut self, exitTimes: Ptr<Vec<SUMOTime>>) {
        self.myExitTimes = exitTimes;
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
        // shortcut: the partial (current position) distance needs MSStageMoving::myRouteStep (Unmapped iterator), upgrade when it is ported
        if partial && self.myArrived < 0 {
            panic!("MSStageWalking::walkDistance(partial) needs MSStageMoving::myRouteStep (Unmapped iterator); not ported yet");
        }
        let mut length: f64 = 0.;
        for e in self.myRoute.iter() {
            length += e.get_length();
        }
        // shortcut: MSPModel::usingInternalLanes() is assumed false (movement model not ported), so walkingarea distances are not added
        // determine walking direction for depart and arrival
        let mut dummy: i32 = 0;
        let departFwdArrivalDir = MSPModel::can_traverse(MSPMODEL_FORWARD, &self.myRoute, &mut dummy);
        let departBwdArrivalDir = MSPModel::can_traverse(MSPMODEL_BACKWARD, &self.myRoute, &mut dummy);
        let mayStartForward = departFwdArrivalDir != MSPMODEL_UNDEFINED_DIRECTION;
        let mayStartBackward = departBwdArrivalDir != MSPMODEL_UNDEFINED_DIRECTION;
        let arrivalPos = self.myArrivalPos;
        let lastLength = self.myRoute[self.myRoute.len() - 1].get_length();
        let firstLength = self.myRoute[0].get_length();
        let lengthFwd = length - self.myDepartPos - (if departFwdArrivalDir == MSPMODEL_BACKWARD { arrivalPos } else { lastLength - arrivalPos });
        let lengthBwd = length - (firstLength - self.myDepartPos) - (if departBwdArrivalDir == MSPMODEL_BACKWARD { arrivalPos } else { lastLength - arrivalPos });
        if self.myRoute.len() == 1 {
            if self.myDepartPos > self.myArrivalPos {
                length = lengthBwd;
            } else {
                length = lengthFwd;
            }
        } else if mayStartForward && mayStartBackward {
            length = if lengthFwd < lengthBwd { lengthFwd } else { lengthBwd };
        } else if mayStartForward {
            length = lengthFwd;
        } else if mayStartBackward {
            length = lengthBwd;
        } else {
            length = lengthFwd;
        }
        if POSITION_EPS > length { POSITION_EPS } else { length }
    }

    /// C++: `double MSStageWalking::computeAverageSpeed() const` [const, private]
    // double
    // MSStageWalking::computeAverageSpeed() const {
    //     return walkDistance() / STEPS2TIME(myWalkingTime + 1); // avoid systematic rounding errors
    // }
    #[allow(unused_variables)]
    pub fn compute_average_speed(&self) -> f64 {
        self.walk_distance(false) / steps2time(self.myWalkingTime + 1)
    }

    /// C++: `void MSStageWalking::MSStageWalking(const MSStageWalking &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageWalking) -> Self {
        panic!("MSStageWalking copy constructor is invalidated in C++ (private)");
    }

    /// C++: `MSStageWalking & MSStageWalking::operator=(const MSStageWalking &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageWalking) -> &mut MSStageWalking {
        panic!("MSStageWalking assignment operator is invalidated in C++ (private)");
    }

}

