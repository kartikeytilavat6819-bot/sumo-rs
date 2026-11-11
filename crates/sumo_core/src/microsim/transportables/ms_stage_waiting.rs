//! Port of microsim/transportables/MSStageWaiting.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A "real" stage performing a waiting over the specified time
/// C++ class `MSStageWaiting` (microsim/transportables/MSStageWaiting.h)
pub struct MSStageWaiting {
    /// C++ base class `MSStage`
    pub base: MSStage,
    /// the time the person is waiting
    pub myWaitingDuration: SUMOTime,
    /// the time until the person is waiting
    pub myWaitingUntil: SUMOTime,
    /// waiting position at stopping place
    pub myStopWaitPos: Position,
    /// The type of activity
    pub myActType: String,
    /// the jump duration if this stop is followed by a jump
    pub myJumpDuration: SUMOTime,
    /// stores the actual end time of the stop (combination of duration and until)
    pub myStopEndTime: SUMOTime,
}

impl std::ops::Deref for MSStageWaiting {
    type Target = MSStage;
    fn deref(&self) -> &MSStage {
        &self.base
    }
}
impl std::ops::DerefMut for MSStageWaiting {
    fn deref_mut(&mut self) -> &mut MSStage {
        &mut self.base
    }
}

impl MSStageWaiting {
    /// C++: `void MSStageWaiting::MSStageWaiting(const MSEdge *, MSStoppingPlace *, SUMOTime, SUMOTime, double, const std::string &, const bool, SUMOTime)`
    /// constructor
    // MSStageWaiting::MSStageWaiting(const MSEdge* destination, MSStoppingPlace* toStop,
    //                                SUMOTime duration, SUMOTime until, double pos, const std::string& actType,
    //                                const bool initial, SUMOTime jumpDuration) :
    //     MSStage(initial ? MSStageType::WAITING_FOR_DEPART : MSStageType::WAITING,
    //             destination,
    //             toStop,
    //             SUMOVehicleParameter::interpretEdgePos(pos, destination->getLength(), SUMO_ATTR_DEPARTPOS, "stopping at " + destination->getID())),
    //     myWaitingDuration(duration),
    //     myWaitingUntil(until),
    //     myStopWaitPos(Position::INVALID),
    //     myActType(actType),
    //     myJumpDuration(jumpDuration),
    //     myStopEndTime(-1) {
    // }
    #[allow(unused_variables)]
    pub fn new(destination: Ptr<MSEdge>, toStop: Ptr<MSStoppingPlace>, duration: SUMOTime, until: SUMOTime, pos: f64, actType: &str, initial: bool, jumpDuration: SUMOTime) -> Self {
        todo!("port MSStageWaiting::MSStageWaiting")
    }

    /// C++: `void MSStageWaiting::~MSStageWaiting() noexcept` [virtual]
    /// destructor
    // MSStageWaiting::~MSStageWaiting() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSStageWaiting::~MSStageWaiting")
    }

    /// C++: `MSStage * MSStageWaiting::clone() const` [const, virtual]
    // MSStage*
    // MSStageWaiting::clone() const {
    //     MSStage* const clon = new MSStageWaiting(myDestination, myDestinationStop, myWaitingDuration, myWaitingUntil, myArrivalPos,
    //             myActType, myType == MSStageType::WAITING_FOR_DEPART);
    //     clon->setParameters(*this);
    //     return clon;
    // }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        todo!("port MSStageWaiting::clone")
    }

    /// C++: `void MSStageWaiting::abort(MSTransportable *)` [virtual]
    /// abort this stage (TraCI)
    // void
    // MSStageWaiting::abort(MSTransportable* t) {
    //     MSTransportableControl& tc = (t->isPerson() ?
    //                                   MSNet::getInstance()->getPersonControl() :
    //                                   MSNet::getInstance()->getContainerControl());
    //     tc.abortWaiting(t);
    //     if (myType == MSStageType::WAITING_FOR_DEPART) {
    //         tc.forceDeparture();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort(&mut self, arg0: Ptr<MSTransportable>) {
        todo!("port MSStageWaiting::abort")
    }

    /// C++: `SUMOTime MSStageWaiting::getUntil() const` [const]
    // SUMOTime
    // MSStageWaiting::getUntil() const {
    //     return myWaitingUntil;
    // }
    #[allow(unused_variables)]
    pub fn get_until(&self) -> SUMOTime {
        todo!("port MSStageWaiting::getUntil")
    }

    /// C++: `SUMOTime MSStageWaiting::getPlannedDuration() const` [const]
    // SUMOTime
    // MSStageWaiting::getPlannedDuration() const {
    //     return myWaitingDuration;
    // }
    #[allow(unused_variables)]
    pub fn get_planned_duration(&self) -> SUMOTime {
        todo!("port MSStageWaiting::getPlannedDuration")
    }

    /// C++: `SUMOTime MSStageWaiting::getJumpDuration() const` [const, virtual]
    // SUMOTime getJumpDuration() const {
    //         return myJumpDuration;
    //     }
    #[allow(unused_variables)]
    pub fn get_jump_duration(&self) -> SUMOTime {
        todo!("port MSStageWaiting::getJumpDuration")
    }

    /// C++: `SUMOTime MSStageWaiting::getTravelTime() const` [const, virtual]
    // SUMOTime getTravelTime() const {
    //         // not a travelling stage
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self) -> SUMOTime {
        todo!("port MSStageWaiting::getTravelTime")
    }

    /// C++: `SUMOTime MSStageWaiting::getDuration() const` [const, virtual]
    // SUMOTime
    // MSStageWaiting::getDuration() const {
    //     return myType == MSStageType::WAITING_FOR_DEPART ? 0 : MSStage::getDuration();
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        todo!("port MSStageWaiting::getDuration")
    }

    /// C++: `SUMOTime MSStageWaiting::getStopEnd() const` [const]
    // SUMOTime getStopEnd() const {
    //         return myStopEndTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_stop_end(&self) -> SUMOTime {
        todo!("port MSStageWaiting::getStopEnd")
    }

    /// C++: `Position MSStageWaiting::getPosition(SUMOTime) const` [const, virtual]
    // Position
    // MSStageWaiting::getPosition(SUMOTime /* now */) const {
    //     if (myStopWaitPos != Position::INVALID) {
    //         return myStopWaitPos;
    //     }
    //     return getEdgePosition(myDestination, myArrivalPos,
    //                            ROADSIDE_OFFSET * (MSGlobals::gLefthand ? -1 : 1));
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        todo!("port MSStageWaiting::getPosition")
    }

    /// C++: `double MSStageWaiting::getAngle(SUMOTime) const` [const, virtual]
    // double
    // MSStageWaiting::getAngle(SUMOTime /* now */) const {
    //     const double offset = myDestinationStop == nullptr ? M_PI / 2 : myDestinationStop->getAngle();
    //     return getEdgeAngle(myDestination, myArrivalPos) + offset * (MSGlobals::gLefthand ? -1 : 1);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        todo!("port MSStageWaiting::getAngle")
    }

    /// C++: `double MSStageWaiting::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double getDistance() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        todo!("port MSStageWaiting::getDistance")
    }

    /// C++: `std::string MSStageWaiting::getStageDescription(const bool) const` [const, virtual]
    // std::string
    // MSStageWaiting::getStageDescription(const bool isPerson) const {
    //     UNUSED_PARAMETER(isPerson);
    //     if (myActType != "") {
    //         return "waiting (" + myActType + ")";
    //     } else {
    //         return "waiting";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        todo!("port MSStageWaiting::getStageDescription")
    }

    /// C++: `std::string MSStageWaiting::getStageSummary(const bool) const` [const, virtual]
    // std::string
    // MSStageWaiting::getStageSummary(const bool /* isPerson */) const {
    //     std::string timeInfo;
    //     if (myWaitingUntil >= 0) {
    //         timeInfo += " until " + time2string(myWaitingUntil);
    //     }
    //     if (myWaitingDuration >= 0) {
    //         timeInfo += " duration " + time2string(myWaitingDuration);
    //     }
    //     if (getDestinationStop() != nullptr) {
    //         std::string nameMsg = "";
    //         if (getDestinationStop()->getMyName() != "") {
    //             nameMsg = "(" + getDestinationStop()->getMyName() + ") ";
    //         }
    //         return "stopping at stop '" + getDestinationStop()->getID() + "' " + nameMsg + timeInfo + " (" + myActType + ")";
    //     }
    //     return "stopping at edge '" + getDestination()->getID() + "' " + timeInfo + " (" + myActType + ")";
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        todo!("port MSStageWaiting::getStageSummary")
    }

    /// C++: `bool MSStageWaiting::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual]
    /// proceeds to the next step
    // bool
    // MSStageWaiting::proceed(MSNet* net, MSTransportable* transportable, SUMOTime now, MSStage* previous) {
    //     myDeparted = now;
    //     myStopEndTime = MAX3(now, now + myWaitingDuration, myWaitingUntil);
    //     if (unspecifiedArrivalPos()) {
    //         myArrivalPos = previous->getArrivalPos();
    //     }
    //     if (myDestinationStop != nullptr) {
    //         myDestinationStop->addTransportable(transportable);
    //         myStopWaitPos = myDestinationStop->getWaitPosition(transportable);
    //     }
    //
    //     myDestination->addTransportable(transportable);
    //     if (transportable->isPerson()) {
    //         net->getPersonControl().setWaitEnd(myStopEndTime, transportable);
    //     } else {
    //         net->getContainerControl().setWaitEnd(myStopEndTime, transportable);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        todo!("port MSStageWaiting::proceed")
    }

    /// C++: `void MSStageWaiting::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSStageWaiting::tripInfoOutput(OutputDevice& os, const MSTransportable* const) const {
    //     if (myType != MSStageType::WAITING_FOR_DEPART) {
    //         os.openTag(SUMO_TAG_STOP);
    //         os.writeAttr("duration", getDuration() != SUMOTime_MAX ? time2string(getDuration()) : "-1");
    //         os.writeAttr("arrival", time2string(myArrived));
    //         os.writeAttr("arrivalPos", toString(myArrivalPos));
    //         os.writeAttr("actType", myActType == "" ? "waiting" : myActType);
    //         os.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        todo!("port MSStageWaiting::tripInfoOutput")
    }

    /// C++: `void MSStageWaiting::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual]
    /// Called on writing vehroute output
    // void
    // MSStageWaiting::routeOutput(const bool /* isPerson */, OutputDevice& os, const bool, const MSStage* const /* previous */, const bool withTiming, const bool /*saveState*/) const {
    //     if (myType != MSStageType::WAITING_FOR_DEPART) {
    //         os.openTag(SUMO_TAG_STOP);
    //         std::string comment = "";
    //         if (myDestinationStop != nullptr) {
    //             os.writeAttr(toString(myDestinationStop->getElement()), myDestinationStop->getID());
    //             if (myDestinationStop->getMyName() != "") {
    //                 comment =  " <!-- " + StringUtils::escapeXML(myDestinationStop->getMyName(), true) + " -->";
    //             }
    //         } else {
    //             // lane index is arbitrary
    //             os.writeAttr(SUMO_ATTR_LANE, getDestination()->getID() + "_0");
    //             os.writeAttr(SUMO_ATTR_ENDPOS, getArrivalPos());
    //         }
    //         if (myWaitingDuration >= 0) {
    //             os.writeAttr(SUMO_ATTR_DURATION, time2string(myWaitingDuration));
    //         }
    //         if (myWaitingUntil >= 0) {
    //             os.writeAttr(SUMO_ATTR_UNTIL, time2string(myWaitingUntil));
    //         }
    //         if (withTiming) {
    //             os.writeAttr(SUMO_ATTR_STARTED, myDeparted >= 0 ? time2string(myDeparted) : "-1");
    //             os.writeAttr(SUMO_ATTR_ENDED, myArrived >= 0 ? time2string(myArrived) : "-1");
    //         }
    //         if (myJumpDuration >= 0) {
    //             os.writeAttr(SUMO_ATTR_JUMP, time2string(myJumpDuration));
    //         }
    //         if (myActType != "") {
    //             os.writeAttr(SUMO_ATTR_ACTTYPE, myActType);
    //         }
    //         // Write rest of parameters
    //         writeParams(os);
    //         os.closeTag(comment);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        todo!("port MSStageWaiting::routeOutput")
    }

    /// C++: `void MSStageWaiting::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    // void
    // MSStageWaiting::saveState(std::ostringstream& out, MSTransportable* /*transportable*/) {
    //     out << " " << myDeparted;
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        todo!("port MSStageWaiting::saveState")
    }

    /// C++: `void MSStageWaiting::loadState(MSTransportable *, std::istringstream &)` [virtual]
    // void
    // MSStageWaiting::loadState(MSTransportable* transportable, std::istringstream& state) {
    //     state >> myDeparted;
    //     const SUMOTime until = MAX3(myDeparted, myDeparted + myWaitingDuration, myWaitingUntil);
    //     if (myDestinationStop != nullptr) {
    //         myDestinationStop->addTransportable(transportable);
    //         myStopWaitPos = myDestinationStop->getWaitPosition(transportable);
    //     }
    //     if (myDeparted >= 0) {
    //         myDestination->addTransportable(transportable);
    //         MSNet* net = MSNet::getInstance();
    //         if (transportable->isPerson()) {
    //             net->getPersonControl().setWaitEnd(until, transportable);
    //         } else {
    //             net->getContainerControl().setWaitEnd(until, transportable);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut Unmapped /* std::istringstream */) {
        todo!("port MSStageWaiting::loadState")
    }

    /// C++: `void MSStageWaiting::MSStageWaiting(const MSStageWaiting &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageWaiting) -> Self {
        todo!("port MSStageWaiting::MSStageWaiting")
    }

    /// C++: `MSStageWaiting & MSStageWaiting::operator=(const MSStageWaiting &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageWaiting) -> &mut MSStageWaiting {
        todo!("port MSStageWaiting::operator=")
    }

}

