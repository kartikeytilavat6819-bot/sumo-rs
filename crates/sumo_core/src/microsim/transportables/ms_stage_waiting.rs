//! Port of microsim/transportables/MSStageWaiting.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_globals::lefthand;
use utils::common::std_defs::g_precision;
use utils::common::to_string::f64_to_string_prec;

/// A "real" stage performing a waiting over the specified time
/// C++ class `MSStageWaiting` (microsim/transportables/MSStageWaiting.h)
#[repr(C)]
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
        let arrivalPos = SUMOVehicleParameter::interpret_edge_pos(pos, destination.get_length(), SUMO_ATTR_DEPARTPOS, &format!("stopping at {}", destination.get_id()), false);
        let type_ = if initial { MSStageType::WAITING_FOR_DEPART } else { MSStageType::WAITING };
        MSStageWaiting {
            base: MSStage::new(type_, destination, toStop, arrivalPos, 0., ""),
            myWaitingDuration: duration,
            myWaitingUntil: until,
            myStopWaitPos: Position::INVALID,
            myActType: actType.to_string(),
            myJumpDuration: jumpDuration,
            myStopEndTime: -1,
        }
    }

    /// C++: `void MSStageWaiting::~MSStageWaiting() noexcept` [virtual]
    /// destructor
    // MSStageWaiting::~MSStageWaiting() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
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
        let clon = MSStageWaiting::new(self.myDestination, self.myDestinationStop, self.myWaitingDuration, self.myWaitingUntil, self.myArrivalPos,
            &self.myActType, self.myType == MSStageType::WAITING_FOR_DEPART, -1);
        let clonPtr = Ptr::from_mut(Box::leak(Box::new(clon))).cast::<MSStage>();
        clonPtr.m().set_parameters(self);
        clonPtr
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
        let t = arg0;
        let net = MSNet::get_instance();
        let tc = if t.is_person() { net.m().get_person_control() } else { net.m().get_container_control() };
        tc.abort_waiting(t);
        if self.myType == MSStageType::WAITING_FOR_DEPART {
            tc.force_departure();
        }
    }

    /// C++: `SUMOTime MSStageWaiting::getUntil() const` [const]
    // SUMOTime
    // MSStageWaiting::getUntil() const {
    //     return myWaitingUntil;
    // }
    #[allow(unused_variables)]
    pub fn get_until(&self) -> SUMOTime {
        self.myWaitingUntil
    }

    /// C++: `SUMOTime MSStageWaiting::getPlannedDuration() const` [const]
    // SUMOTime
    // MSStageWaiting::getPlannedDuration() const {
    //     return myWaitingDuration;
    // }
    #[allow(unused_variables)]
    pub fn get_planned_duration(&self) -> SUMOTime {
        self.myWaitingDuration
    }

    /// C++: `SUMOTime MSStageWaiting::getJumpDuration() const` [const, virtual]
    // SUMOTime getJumpDuration() const {
    //         return myJumpDuration;
    //     }
    #[allow(unused_variables)]
    pub fn get_jump_duration(&self) -> SUMOTime {
        self.myJumpDuration
    }

    /// C++: `SUMOTime MSStageWaiting::getTravelTime() const` [const, virtual]
    // SUMOTime getTravelTime() const {
    //         // not a travelling stage
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self) -> SUMOTime {
        // not a travelling stage
        0
    }

    /// C++: `SUMOTime MSStageWaiting::getDuration() const` [const, virtual]
    // SUMOTime
    // MSStageWaiting::getDuration() const {
    //     return myType == MSStageType::WAITING_FOR_DEPART ? 0 : MSStage::getDuration();
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        if self.myType == MSStageType::WAITING_FOR_DEPART { 0 } else { self.base.get_duration() }
    }

    /// C++: `SUMOTime MSStageWaiting::getStopEnd() const` [const]
    // SUMOTime getStopEnd() const {
    //         return myStopEndTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_stop_end(&self) -> SUMOTime {
        self.myStopEndTime
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
        if self.myStopWaitPos != Position::INVALID {
            return self.myStopWaitPos;
        }
        self.get_edge_position(self.myDestination, self.myArrivalPos,
            ROADSIDE_OFFSET * (if lefthand() { -1. } else { 1. }))
    }

    /// C++: `double MSStageWaiting::getAngle(SUMOTime) const` [const, virtual]
    // double
    // MSStageWaiting::getAngle(SUMOTime /* now */) const {
    //     const double offset = myDestinationStop == nullptr ? M_PI / 2 : myDestinationStop->getAngle();
    //     return getEdgeAngle(myDestination, myArrivalPos) + offset * (MSGlobals::gLefthand ? -1 : 1);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        let offset = if self.myDestinationStop.is_null() { std::f64::consts::PI / 2. } else { self.myDestinationStop.get_angle() };
        self.get_edge_angle(self.myDestination, self.myArrivalPos) + offset * (if lefthand() { -1. } else { 1. })
    }

    /// C++: `double MSStageWaiting::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double getDistance() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        0.
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
        if self.myActType != "" {
            format!("waiting ({})", self.myActType)
        } else {
            "waiting".to_string()
        }
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
        let mut timeInfo = String::new();
        if self.myWaitingUntil >= 0 {
            timeInfo += &format!(" until {}", time2string(self.myWaitingUntil));
        }
        if self.myWaitingDuration >= 0 {
            timeInfo += &format!(" duration {}", time2string(self.myWaitingDuration));
        }
        if !self.get_destination_stop().is_null() {
            let stop = self.get_destination_stop();
            let nameMsg = if stop.get_my_name() != "" { format!("({}) ", stop.get_my_name()) } else { String::new() };
            return format!("stopping at stop '{}' {}{} ({})", stop.get_id(), nameMsg, timeInfo, self.myActType);
        }
        format!("stopping at edge '{}' {} ({})", self.get_destination().get_id(), timeInfo, self.myActType)
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
        self.myDeparted = now;
        self.myStopEndTime = now.max(now + self.myWaitingDuration).max(self.myWaitingUntil);
        if self.unspecified_arrival_pos() {
            self.myArrivalPos = previous.get_arrival_pos();
        }
        if !self.myDestinationStop.is_null() {
            self.myDestinationStop.m().add_transportable(transportable, -1);
            self.myStopWaitPos = self.myDestinationStop.get_wait_position(transportable);
        }

        self.myDestination.add_transportable(transportable);
        if transportable.is_person() {
            net.m().get_person_control().set_wait_end(self.myStopEndTime, transportable);
        } else {
            net.m().get_container_control().set_wait_end(self.myStopEndTime, transportable);
        }
        true
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
        if self.myType != MSStageType::WAITING_FOR_DEPART {
            os.open_tag_id(SUMO_TAG_STOP);
            let duration = if self.get_duration() != SUMOTIME_MAX { time2string(self.get_duration()) } else { "-1".to_string() };
            os.write_attr("duration", &duration);
            os.write_attr("arrival", &time2string(self.myArrived));
            os.write_attr("arrivalPos", &f64_to_string_prec(self.myArrivalPos, g_precision()));
            os.write_attr("actType", if self.myActType == "" { "waiting" } else { self.myActType.as_str() });
            os.close_tag("");
        }
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
        if self.myType != MSStageType::WAITING_FOR_DEPART {
            os.open_tag_id(SUMO_TAG_STOP);
            let mut comment = String::new();
            if !self.myDestinationStop.is_null() {
                let stop = self.myDestinationStop;
                os.write_attr(utils::xml::sumo_xml_definitions::Tags.get_string(stop.get_element()), stop.get_id());
                if stop.get_my_name() != "" {
                    comment = format!(" <!-- {} -->", utils::common::string_utils::escape_xml(stop.get_my_name(), true));
                }
            } else {
                // lane index is arbitrary
                os.write_attr(SUMO_ATTR_LANE, &format!("{}_0", self.get_destination().get_id()));
                os.write_attr(SUMO_ATTR_ENDPOS, &self.get_arrival_pos());
            }
            if self.myWaitingDuration >= 0 {
                os.write_attr(SUMO_ATTR_DURATION, &time2string(self.myWaitingDuration));
            }
            if self.myWaitingUntil >= 0 {
                os.write_attr(SUMO_ATTR_UNTIL, &time2string(self.myWaitingUntil));
            }
            if withTiming {
                let started = if self.myDeparted >= 0 { time2string(self.myDeparted) } else { "-1".to_string() };
                os.write_attr(SUMO_ATTR_STARTED, &started);
                let ended = if self.myArrived >= 0 { time2string(self.myArrived) } else { "-1".to_string() };
                os.write_attr(SUMO_ATTR_ENDED, &ended);
            }
            if self.myJumpDuration >= 0 {
                os.write_attr(SUMO_ATTR_JUMP, &time2string(self.myJumpDuration));
            }
            if self.myActType != "" {
                os.write_attr(SUMO_ATTR_ACTTYPE, &self.myActType);
            }
            // Write rest of parameters
            self.write_params(os);
            os.close_tag(&comment);
        }
    }

    /// C++: `void MSStageWaiting::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    // void
    // MSStageWaiting::saveState(std::ostringstream& out, MSTransportable* /*transportable*/) {
    //     out << " " << myDeparted;
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut String /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        out.push_str(&format!(" {}", self.myDeparted));
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
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut std::str::SplitWhitespace<'_> /* std::istringstream */) {
        // C++ `state >> myDeparted` leaves 0 on a failed extraction
        self.myDeparted = state.next().and_then(|t| t.parse::<i64>().ok()).unwrap_or(0);
        let until = self.myDeparted.max(self.myDeparted + self.myWaitingDuration).max(self.myWaitingUntil);
        if !self.myDestinationStop.is_null() {
            self.myDestinationStop.m().add_transportable(transportable, -1);
            self.myStopWaitPos = self.myDestinationStop.get_wait_position(transportable);
        }
        if self.myDeparted >= 0 {
            self.myDestination.add_transportable(transportable);
            let net = MSNet::get_instance();
            if transportable.is_person() {
                net.m().get_person_control().set_wait_end(until, transportable);
            } else {
                net.m().get_container_control().set_wait_end(until, transportable);
            }
        }
    }

    /// C++: `void MSStageWaiting::MSStageWaiting(const MSStageWaiting &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageWaiting) -> Self {
        panic!("MSStageWaiting copy constructor is private and invalidated in C++")
    }

    /// C++: `MSStageWaiting & MSStageWaiting::operator=(const MSStageWaiting &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageWaiting) -> &mut MSStageWaiting {
        panic!("MSStageWaiting assignment operator is private and invalidated in C++")
    }

}

