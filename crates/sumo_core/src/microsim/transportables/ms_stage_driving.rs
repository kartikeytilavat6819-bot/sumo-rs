//! Port of microsim/transportables/MSStageDriving.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSStageDriving::BookReservation` (microsim/transportables/MSStageDriving.h)
#[repr(C)]
pub struct MSStageDriving_BookReservation {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    pub myTransportable: Ptr<MSTransportable>,
    pub myEarliestPickupTime: SUMOTime,
    pub myStage: Ptr<MSStageDriving>,
    pub myWaitingPos: f64,
}

impl std::ops::Deref for MSStageDriving_BookReservation {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSStageDriving_BookReservation {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSStageDriving_BookReservation {
    /// C++: `void BookReservation::BookReservation(MSTransportable *, SUMOTime, MSStageDriving *)`
    // BookReservation(MSTransportable* transportable, SUMOTime earliestPickupTime, MSStageDriving* stage) :
    //             myTransportable(transportable), myEarliestPickupTime(earliestPickupTime), myStage(stage), myWaitingPos(stage->myWaitingPos) {}
    #[allow(unused_variables)]
    pub fn new(transportable: Ptr<MSTransportable>, earliestPickupTime: SUMOTime, stage: Ptr<MSStageDriving>) -> Self {
        todo!("port BookReservation::BookReservation")
    }

    /// C++: `SUMOTime BookReservation::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSStageDriving::BookReservation::execute(SUMOTime currentTime) {
    //     MSDevice_Taxi::addReservation(myTransportable, myStage->getLines(), currentTime, currentTime, myEarliestPickupTime,
    //                                   myStage->myOrigin, myStage->myWaitingPos, myStage->myOriginStop, myStage->getDestination(), myStage->getArrivalPos(), myStage->myDestinationStop, myStage->myGroup);
    //     return 0; // do not repeat
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port BookReservation::execute")
    }

}

/// A "real" stage performing the travelling by a transport system The given route will be chosen. The travel time is computed by the simulation
/// C++ class `MSStageDriving` (microsim/transportables/MSStageDriving.h)
#[repr(C)]
pub struct MSStageDriving {
    /// C++ base class `MSStage`
    pub base: MSStage,
    /// the origin edge
    pub myOrigin: Ptr<MSEdge>,
    /// the lines to choose from
    pub myLines: BTreeSet<String>,
    /// The taken vehicle
    pub myVehicle: Ptr<SUMOVehicle>,
    /// cached vehicle data for output after the vehicle has been removed
    pub myVehicleID: String,
    pub myVehicleLine: String,
    pub myVehicleType: String,
    pub myVehicleVClass: SUMOVehicleClass,
    pub myVehicleDistance: f64,
    /// While driving, this is the timeLoss of the vehicle when the ride started, after arrival this is the timeLoss which the vehicle accumulated during the ride
    pub myTimeLoss: SUMOTime,
    pub myWaitingPos: f64,
    /// The time since which this person is waiting for a ride
    pub myWaitingSince: SUMOTime,
    pub myWaitingEdge: Ptr<MSEdge>,
    pub myStopWaitPos: Position,
    /// the stop at which this ride starts (or nullptr)
    pub myOriginStop: Ptr<MSStoppingPlace>,
    pub myIntendedVehicleID: String,
    pub myIntendedDepart: SUMOTime,
    pub myReservationWaitingPos: f64,
}

impl std::ops::Deref for MSStageDriving {
    type Target = MSStage;
    fn deref(&self) -> &MSStage {
        &self.base
    }
}
impl std::ops::DerefMut for MSStageDriving {
    fn deref_mut(&mut self) -> &mut MSStage {
        &mut self.base
    }
}

impl MSStageDriving {
    /// C++: `void MSStageDriving::MSStageDriving(const MSEdge *, const MSEdge *, MSStoppingPlace *, const double, const double, const std::vector<std::string> &, const std::string &, const std::string &, SUMOTime)`
    /// constructor
    // MSStageDriving::MSStageDriving(const MSEdge* origin, const MSEdge* destination,
    //                                MSStoppingPlace* toStop, const double arrivalPos, const double arrivalPosLat,
    //                                const std::vector<std::string>& lines, const std::string& group,
    //                                const std::string& intendedVeh, SUMOTime intendedDepart) :
    //     MSStage(MSStageType::DRIVING, destination, toStop, arrivalPos, arrivalPosLat, group),
    //     myOrigin(origin),
    //     myLines(lines.begin(), lines.end()),
    //     myVehicle(nullptr),
    //     myVehicleVClass(SVC_IGNORING),
    //     myVehicleDistance(-1.),
    //     myTimeLoss(-1),
    //     myWaitingPos(-1),
    //     myWaitingSince(-1),
    //     myWaitingEdge(origin),
    //     myStopWaitPos(Position::INVALID),
    //     myOriginStop(nullptr),
    //     myIntendedVehicleID(intendedVeh),
    //     myIntendedDepart(intendedDepart) {
    // }
    #[allow(unused_variables)]
    pub fn new(origin: Ptr<MSEdge>, destination: Ptr<MSEdge>, toStop: Ptr<MSStoppingPlace>, arrivalPos: f64, arrivalPosLat: f64, lines: &Vec<String>, group: &str, intendedVeh: &str, intendedDepart: SUMOTime) -> Self {
        todo!("port MSStageDriving::MSStageDriving")
    }

    /// C++: `void MSStageDriving::~MSStageDriving() noexcept` [virtual]
    /// destructor
    // MSStageDriving::~MSStageDriving() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSStageDriving::~MSStageDriving")
    }

    /// C++: `MSStage * MSStageDriving::clone() const` [const, virtual]
    // MSStage*
    // MSStageDriving::clone() const {
    //     MSStage* const clon = new MSStageDriving(myOrigin, myDestination, myDestinationStop, myArrivalPos, myArrivalPosLat,
    //             std::vector<std::string>(myLines.begin(), myLines.end()),
    //             myGroup, myIntendedVehicleID, myIntendedDepart);
    //     clon->setParameters(*this);
    //     return clon;
    // }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        todo!("port MSStageDriving::clone")
    }

    /// C++: `double MSStageDriving::getArrivalPos() const` [const, virtual]
    /// return default value for undefined arrivalPos
    // double
    // MSStageDriving::getArrivalPos() const {
    //     return unspecifiedArrivalPos() ? getDestination()->getLength() : myArrivalPos;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        todo!("port MSStageDriving::getArrivalPos")
    }

    /// C++: `void MSStageDriving::abort(MSTransportable *)` [virtual]
    /// abort this stage (TraCI)
    // void
    // MSStageDriving::abort(MSTransportable* t) {
    //     myDestinationStop = nullptr;
    //     if (myVehicle != nullptr) {
    //         // jumping out of a moving vehicle!
    //         myVehicle->removeTransportable(t);
    //         myDestination = myVehicle->getLane() == nullptr ? myVehicle->getEdge() : &myVehicle->getLane()->getEdge();
    //         myArrivalPos = myVehicle->getPositionOnLane();
    //         // myVehicleDistance and myTimeLoss are updated in subsequent call to setArrived
    //     } else {
    //         MSTransportableControl& tc = (t->isPerson() ?
    //                                       MSNet::getInstance()->getPersonControl() :
    //                                       MSNet::getInstance()->getContainerControl());
    //         tc.abortWaitingForVehicle(t);
    //         MSDevice_Taxi::removeReservation(t, getLines(), myWaitingEdge, myWaitingPos, myDestination, getArrivalPos(), myGroup);
    //         myDestination = myWaitingEdge;
    //         myDestinationStop = myOriginStop;
    //         myArrivalPos = myWaitingPos;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort(&mut self, t: Ptr<MSTransportable>) {
        todo!("port MSStageDriving::abort")
    }

    /// C++: `void MSStageDriving::init(MSTransportable *)` [virtual]
    /// initialization, e.g. for param-related events
    // void
    // MSStageDriving::init(MSTransportable* transportable) {
    //     if (hasParameter("earliestPickupTime")) {
    //         SUMOTime reservationTime = MSNet::getInstance()->getCurrentTimeStep();
    //         if (hasParameter("reservationTime")) {
    //             reservationTime = string2time(getParameter("reservationTime"));
    //         }
    //         SUMOTime earliestPickupTime = string2time(getParameter("earliestPickupTime"));
    //         if (transportable->getNextStage(1) == this) {
    //             // if the ride is the first stage use the departPos (there is a unvisible stop before)
    //             myWaitingPos = transportable->getParameter().departPos;
    //         } else {
    //             // else use the middle of the edge, as also used as default for walk's arrivalPos
    //             myWaitingPos = myOrigin->getLength() / 2;
    //         }
    //         myReservationWaitingPos = myWaitingPos;
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(new BookReservation(transportable, earliestPickupTime, this), reservationTime);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, transportable: Ptr<MSTransportable>) {
        todo!("port MSStageDriving::init")
    }

    /// C++: `const MSEdge * MSStageDriving::getEdge() const` [const, virtual]
    /// Returns the current edge
    // const MSEdge*
    // MSStageDriving::getEdge() const {
    //     if (myVehicle != nullptr) {
    //         if (myVehicle->getLane() != nullptr) {
    //             return &myVehicle->getLane()->getEdge();
    //         }
    //         return myVehicle->getEdge();
    //     } else if (myArrived >= 0) {
    //         return myDestination;
    //     } else {
    //         return myWaitingEdge;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSStageDriving::getEdge")
    }

    /// C++: `const MSEdge * MSStageDriving::getFromEdge() const` [const, virtual]
    // const MSEdge*
    // MSStageDriving::getFromEdge() const {
    //     return myWaitingEdge;
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSStageDriving::getFromEdge")
    }

    /// C++: `double MSStageDriving::getEdgePos(SUMOTime) const` [const, virtual]
    // double
    // MSStageDriving::getEdgePos(SUMOTime /* now */) const {
    //     if (isWaiting4Vehicle()) {
    //         return myWaitingPos;
    //     } else if (myArrived >= 0) {
    //         return myArrivalPos;
    //     } else {
    //         // vehicle may already have passed the lane (check whether this is correct)
    //         return MIN2(myVehicle->getPositionOnLane(), getEdge()->getLength());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        todo!("port MSStageDriving::getEdgePos")
    }

    /// C++: `int MSStageDriving::getDirection() const` [const, virtual]
    /// Return the movement directon on the edge
    // int
    // MSStageDriving::getDirection() const {
    //     if (isWaiting4Vehicle()) {
    //         return MSPModel::UNDEFINED_DIRECTION;
    //     } else if (myArrived >= 0) {
    //         return MSPModel::UNDEFINED_DIRECTION;
    //     } else {
    //         return MSPModel::FORWARD;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        todo!("port MSStageDriving::getDirection")
    }

    /// C++: `const MSLane * MSStageDriving::getLane() const` [const, virtual]
    // const MSLane*
    // MSStageDriving::getLane() const {
    //     return myVehicle != nullptr ? myVehicle->getLane() : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        todo!("port MSStageDriving::getLane")
    }

    /// C++: `MSStoppingPlace * MSStageDriving::getOriginStop() const` [const, virtual]
    // MSStoppingPlace* getOriginStop() const {
    //         return myOriginStop;
    //     }
    #[allow(unused_variables)]
    pub fn get_origin_stop(&self) -> Ptr<MSStoppingPlace> {
        todo!("port MSStageDriving::getOriginStop")
    }

    /// C++: `Position MSStageDriving::getPosition(SUMOTime) const` [const, virtual]
    // Position
    // MSStageDriving::getPosition(SUMOTime /* now */) const {
    //     if (isWaiting4Vehicle()) {
    //         if (myStopWaitPos != Position::INVALID) {
    //             return myStopWaitPos;
    //         }
    //         return getEdgePosition(myWaitingEdge, myWaitingPos,
    //                                ROADSIDE_OFFSET * (MSGlobals::gLefthand ? -1 : 1));
    //     } else if (myArrived >= 0) {
    //         return getEdgePosition(myDestination, myArrivalPos,
    //                                ROADSIDE_OFFSET * (MSGlobals::gLefthand ? -1 : 1));
    //     } else {
    //         return myVehicle->getPosition();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        todo!("port MSStageDriving::getPosition")
    }

    /// C++: `double MSStageDriving::getAngle(SUMOTime) const` [const, virtual]
    // double
    // MSStageDriving::getAngle(SUMOTime /* now */) const {
    //     if (isWaiting4Vehicle()) {
    //         const double offset = myOriginStop == nullptr ? M_PI / 2 : myOriginStop->getAngle();
    //         return getEdgeAngle(myWaitingEdge, myWaitingPos) + offset * (MSGlobals::gLefthand ? -1 : 1);
    //     } else if (myArrived >= 0) {
    //         return getEdgeAngle(myDestination, myArrivalPos) + M_PI / 2. * (MSGlobals::gLefthand ? -1 : 1);
    //     } else {
    //         MSVehicle* veh = dynamic_cast<MSVehicle*>(myVehicle);
    //         if (veh != nullptr) {
    //             return veh->getAngle();
    //         } else {
    //             return 0;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        todo!("port MSStageDriving::getAngle")
    }

    /// C++: `double MSStageDriving::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double
    // MSStageDriving::getDistance() const {
    //     if (myVehicle != nullptr) {
    //         // distance was previously set to driven distance upon embarking
    //         return myVehicle->getOdometer() - myVehicleDistance;
    //     }
    //     return myVehicleDistance;
    // }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        todo!("port MSStageDriving::getDistance")
    }

    /// C++: `std::string MSStageDriving::getStageDescription(const bool) const` [const, virtual]
    /// return (brief) string representation of the current stage
    // std::string
    // MSStageDriving::getStageDescription(const bool isPerson) const {
    //     return isWaiting4Vehicle() ? "waiting for " + joinToString(myLines, ",") : (isPerson ? "driving" : "transport");
    // }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        todo!("port MSStageDriving::getStageDescription")
    }

    /// C++: `std::string MSStageDriving::getStageSummary(const bool) const` [const, virtual]
    /// return string summary of the current stage
    // std::string
    // MSStageDriving::getStageSummary(const bool isPerson) const {
    //     const std::string dest = (getDestinationStop() == nullptr ?
    //                               " edge '" + getDestination()->getID() + "'" :
    //                               " stop '" + getDestinationStop()->getID() + "'" + (
    //                                   getDestinationStop()->getMyName() != "" ? " (" + getDestinationStop()->getMyName() + ")" : ""));
    //     const std::string intended = myIntendedVehicleID != "" ?
    //                                  " (vehicle " + myIntendedVehicleID + " at time=" + time2string(myIntendedDepart) + ")" :
    //                                  "";
    //     const std::string modeName = isPerson ? "driving" : "transported";
    //     return isWaiting4Vehicle() ?
    //            "waiting for " + joinToString(myLines, ",") + intended + " then " + modeName + " to " + dest :
    //            modeName + " to " + dest;
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        todo!("port MSStageDriving::getStageSummary")
    }

    /// C++: `bool MSStageDriving::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual]
    /// proceeds to this stage
    // bool
    // MSStageDriving::proceed(MSNet* net, MSTransportable* transportable, SUMOTime now, MSStage* previous) {
    //     myOriginStop = (previous->getStageType() == MSStageType::TRIP
    //                     ? previous->getOriginStop()
    //                     : previous->getDestinationStop());
    //     myWaitingSince = now;
    //     const bool isPerson = transportable->isPerson();
    //     if (transportable->getParameter().departProcedure == DepartDefinition::TRIGGERED
    //             && transportable->getCurrentStageIndex() == 1) {
    //         // we are the first real stage (stage 0 is WAITING_FOR_DEPART)
    //         const std::string vehID = *myLines.begin();
    //         SUMOVehicle* startVeh = net->getVehicleControl().getVehicle(vehID);
    //         if (startVeh == nullptr && net->hasFlow(vehID)) {
    //             startVeh = net->getInsertionControl().getLastFlowVehicle(vehID);
    //         }
    //         if (startVeh == nullptr) {
    //             throw ProcessError("Vehicle '" + vehID + "' not found for triggered departure of " +
    //                                (isPerson ? "person" : "container") + " '" + transportable->getID() + "'.");
    //         }
    //         if (transportable->isPerson()) {
    //             const int pCap = startVeh->getVehicleType().getParameter().personCapacity;
    //             if (startVeh->getPersonNumber() >= pCap) {
    //                 WRITE_WARNING(TLF("Vehicle '%' exceeds personCapacity % when placing triggered person '%', time=%",
    //                                   startVeh->getID(), pCap, transportable->getID(), time2string(SIMSTEP)));
    //             }
    //         } else {
    //             const int cCap = startVeh->getVehicleType().getParameter().containerCapacity;
    //             if (startVeh->getContainerNumber() >= cCap) {
    //                 WRITE_WARNING(TLF("Vehicle '%' exceeds containerCapacity % when placing triggered container '%', time=%",
    //                                   startVeh->getID(), cCap, transportable->getID(), time2string(SIMSTEP)));
    //             }
    //         }
    //         myDeparted = now;
    //         setVehicle(startVeh);
    //         if (myOriginStop != nullptr) {
    //             myOriginStop->removeTransportable(transportable);
    //         }
    //         myWaitingEdge = previous->getEdge();
    //         myStopWaitPos = Position::INVALID;
    //         myWaitingPos = previous->getEdgePos(now);
    //         myVehicle->addTransportable(transportable);
    //         return true;
    //     }
    //     if (myOriginStop != nullptr) {
    //         // the arrival stop may have an access point
    //         myWaitingEdge = &myOriginStop->getLane().getEdge();
    //         myStopWaitPos = myOriginStop->getWaitPosition(transportable);
    //         myWaitingPos = myOriginStop->getWaitingPositionOnLane(transportable);
    //     } else {
    //         myWaitingEdge = previous->getEdge();
    //         myStopWaitPos = Position::INVALID;
    //         myWaitingPos = previous->getEdgePos(now);
    //     }
    //     if (myOrigin != nullptr && myOrigin != myWaitingEdge
    //             && (myOriginStop == nullptr || myOriginStop->getAccessPos(myOrigin) < 0)) {
    //         // transfer at junction (rather than access)
    //         myWaitingEdge = myOrigin;
    //         myWaitingPos = 0;
    //     }
    //     SUMOVehicle* const availableVehicle = myWaitingEdge->getWaitingVehicle(transportable, myWaitingPos);
    //     const bool triggered = availableVehicle != nullptr &&
    //                            ((isPerson && availableVehicle->getParameter().departProcedure == DepartDefinition::TRIGGERED) ||
    //                             (!isPerson && availableVehicle->getParameter().departProcedure == DepartDefinition::CONTAINER_TRIGGERED));
    //     if (triggered && !availableVehicle->hasDeparted()) {
    //         setVehicle(availableVehicle);
    //         if (myOriginStop != nullptr) {
    //             myOriginStop->removeTransportable(transportable);
    //         }
    //         myVehicle->addTransportable(transportable);
    //         net->getInsertionControl().add(myVehicle);
    //         net->getVehicleControl().handleTriggeredDepart(myVehicle, false);
    //     } else {
    //         registerWaiting(transportable, now);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        todo!("port MSStageDriving::proceed")
    }

    /// C++: `void MSStageDriving::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSStageDriving::tripInfoOutput(OutputDevice& os, const MSTransportable* const transportable) const {
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     const SUMOTime waitingTime = getWaitingTime();
    //     const SUMOTime duration = myArrived - myDeparted;
    //     MSDevice_Tripinfo::addRideTransportData(transportable->isPerson(), myVehicleDistance, duration, myVehicleVClass, myVehicleLine, waitingTime);
    //     os.openTag(transportable->isPerson() ? "ride" : "transport");
    //     os.writeAttr("waitingTime", waitingTime != SUMOTime_MAX ? time2string(waitingTime) : "-1");
    //     os.writeAttr("vehicle", myVehicleID.empty() ? "NULL" : myVehicleID);
    //     os.writeAttr("depart", myDeparted >= 0 ? time2string(myDeparted) : "-1");
    //     os.writeAttr("arrival", myArrived >= 0 ? time2string(myArrived) : "-1");
    //     os.writeAttr("arrivalPos", myArrived >= 0 ? toString(getArrivalPos()) : "-1");
    //     os.writeAttr("duration", myArrived >= 0 ? time2string(duration) :
    //                  (myDeparted >= 0 ? time2string(now - myDeparted) : "-1"));
    //     os.writeAttr(SUMO_ATTR_ROUTELENGTH, myArrived >= 0 || myVehicle != nullptr ? toString(getDistance()) : "-1");
    //     os.writeAttr("timeLoss", myArrived >= 0 ? time2string(getTimeLoss(transportable)) : "-1");
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        todo!("port MSStageDriving::tripInfoOutput")
    }

    /// C++: `void MSStageDriving::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual]
    /// Called on writing vehroute output
    // void
    // MSStageDriving::routeOutput(const bool isPerson, OutputDevice& os, const bool withRouteLength, const MSStage* const previous, const bool withTiming, const bool /*saveState*/) const {
    //     os.openTag(isPerson ? SUMO_TAG_RIDE : SUMO_TAG_TRANSPORT);
    //     if (getFromEdge() != nullptr) {
    //         os.writeAttr(SUMO_ATTR_FROM, getFromEdge()->getID());
    //     } else if (previous != nullptr && previous->getStageType() == MSStageType::WAITING_FOR_DEPART) {
    //         os.writeAttr(SUMO_ATTR_FROM, previous->getEdge()->getID());
    //     }
    //     os.writeAttr(SUMO_ATTR_TO, getDestination()->getID());
    //     std::string comment = "";
    //     if (myDestinationStop != nullptr) {
    //         os.writeAttr(toString(myDestinationStop->getElement()), myDestinationStop->getID());
    //         if (myDestinationStop->getMyName() != "") {
    //             comment = " <!-- " + StringUtils::escapeXML(myDestinationStop->getMyName(), true) + " -->";
    //         }
    //     } else if (!unspecifiedArrivalPos()) {
    //         os.writeAttr(SUMO_ATTR_ARRIVALPOS, myArrivalPos);
    //     }
    //     if (myLines.size() > 1 || *myLines.begin() != LINE_ANY) {
    //         // no need to write the default
    //         os.writeAttr(SUMO_ATTR_LINES, myLines);
    //     }
    //     if (myIntendedVehicleID != "") {
    //         os.writeAttr(SUMO_ATTR_INTENDED, myIntendedVehicleID);
    //     }
    //     if (myIntendedDepart >= 0) {
    //         os.writeAttr(SUMO_ATTR_DEPART, time2string(myIntendedDepart));
    //     }
    //     if (withRouteLength) {
    //         os.writeAttr("routeLength", myVehicleDistance);
    //     }
    //     if (withTiming) {
    //         os.writeAttr("vehicle", myVehicleID.empty() ? "NULL" : myVehicleID);
    //         os.writeAttr(SUMO_ATTR_STARTED, myDeparted >= 0 ? time2string(myDeparted) : "-1");
    //         os.writeAttr(SUMO_ATTR_ENDED, myArrived >= 0 ? time2string(myArrived) : "-1");
    //     }
    //     if (OptionsCont::getOptions().getBool("vehroute-output.cost")) {
    //         os.writeAttr(SUMO_ATTR_COST, getCosts());
    //     }
    //     os.closeTag(comment);
    // }
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        todo!("port MSStageDriving::routeOutput")
    }

    /// C++: `bool MSStageDriving::isWaitingFor(const SUMOVehicle *) const` [const, virtual]
    /// Whether the person waits for the given vehicle
    // bool
    // MSStageDriving::isWaitingFor(const SUMOVehicle* vehicle) const {
    //     assert(myLines.size() > 0);
    //     return (myLines.count(vehicle->getID()) > 0
    //             || ((myLines.count(vehicle->getParameter().line) > 0
    //                  || myLines.count(LINE_ANY) > 0) &&
    //                 // even if the line matches we still have to check for stops (#14526)
    //                 (myDestinationStop == nullptr
    //                  ? vehicle->stopsAtEdge(myDestination)
    //                  : vehicle->stopsAt(myDestinationStop)))
    //             || MSDevice_Taxi::compatibleLine(vehicle->getParameter().line, *myLines.begin()));
    // }
    #[allow(unused_variables)]
    pub fn is_waiting_for(&self, vehicle: Ptr<SUMOVehicle>) -> bool {
        todo!("port MSStageDriving::isWaitingFor")
    }

    /// C++: `bool MSStageDriving::isWaiting4Vehicle() const` [const, virtual]
    /// Whether the person waits for a vehicle
    // bool
    // MSStageDriving::isWaiting4Vehicle() const {
    //     return myVehicle == nullptr && myArrived < 0;
    // }
    #[allow(unused_variables)]
    pub fn is_waiting4_vehicle(&self) -> bool {
        todo!("port MSStageDriving::isWaiting4Vehicle")
    }

    /// C++: `std::string MSStageDriving::getWaitingDescription() const` [const]
    /// Return where the person waits and for what
    // std::string
    // MSStageDriving::getWaitingDescription() const {
    //     return isWaiting4Vehicle() ? ("waiting for " + joinToString(myLines, ",")
    //                                   + " at " + (myOriginStop == nullptr
    //                                           ? ("edge '" + myWaitingEdge->getID() + "'")
    //                                           : (toString(myOriginStop->getElement()) + " '" + myOriginStop->getID() + "'"))
    //                                  ) : "";
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_description(&self) -> String {
        todo!("port MSStageDriving::getWaitingDescription")
    }

    /// C++: `SUMOVehicle * MSStageDriving::getVehicle() const` [const, virtual]
    // SUMOVehicle* getVehicle() const {
    //         return myVehicle;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<SUMOVehicle> {
        todo!("port MSStageDriving::getVehicle")
    }

    /// C++: `SUMOTime MSStageDriving::getWaitingTime(SUMOTime) const` [const]
    /// time spent waiting for a ride
    // SUMOTime
    // MSStageDriving::getWaitingTime(SUMOTime now) const {
    //     return isWaiting4Vehicle() ? now - myWaitingSince : 0;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self, now: SUMOTime) -> SUMOTime {
        todo!("port MSStageDriving::getWaitingTime")
    }

    /// C++: `double MSStageDriving::getSpeed() const` [const, virtual]
    // double
    // MSStageDriving::getSpeed() const {
    //     return myVehicle == nullptr ? 0 : myVehicle->getSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        todo!("port MSStageDriving::getSpeed")
    }

    /// C++: `ConstMSEdgeVector MSStageDriving::getEdges() const` [const, virtual]
    // ConstMSEdgeVector
    // MSStageDriving::getEdges() const {
    //     ConstMSEdgeVector result;
    //     result.push_back(getFromEdge());
    //     result.push_back(getDestination());
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> ConstMSEdgeVector {
        todo!("port MSStageDriving::getEdges")
    }

    /// C++: `void MSStageDriving::setVehicle(SUMOVehicle *)`
    // void
    // MSStageDriving::setVehicle(SUMOVehicle* v) {
    //     myVehicle = v;
    //     if (myVehicle != nullptr) {
    //         myVehicleID = v->getID();
    //         myVehicleLine = v->getParameter().line;
    //         myVehicleType = v->getVehicleType().getID();
    //         myVehicleVClass = v->getVClass();
    //         if (myVehicle->hasDeparted()) {
    //             myVehicleDistance = myVehicle->getOdometer();
    //             myTimeLoss = myVehicle->getTimeLoss();
    //         } else {
    //             // it probably got triggered by the person
    //             myVehicleDistance = 0.;
    //             myTimeLoss = 0;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle(&mut self, v: Ptr<SUMOVehicle>) {
        todo!("port MSStageDriving::setVehicle")
    }

    /// C++: `void MSStageDriving::setVehicleID(const std::string)`
    // void setVehicleID(const std::string vehID) {
    //         myVehicleID = vehID;
    //     }
    #[allow(unused_variables)]
    pub fn set_vehicle_id(&mut self, vehID: String) {
        todo!("port MSStageDriving::setVehicleID")
    }

    /// C++: `const std::string MSStageDriving::setArrived(MSNet *, MSTransportable *, SUMOTime, const bool)` [virtual]
    /// marks arrival time and records driven distance
    // const std::string
    // MSStageDriving::setArrived(MSNet* net, MSTransportable* transportable, SUMOTime now, const bool vehicleArrived) {
    //     MSStage::setArrived(net, transportable, now, vehicleArrived);
    //     if (myVehicle != nullptr) {
    //         // distance was previously set to driven distance upon embarking
    //         myVehicleDistance = myVehicle->getOdometer() - myVehicleDistance;
    //         myTimeLoss = myVehicle->getTimeLoss() - myTimeLoss;
    //         if (vehicleArrived) {
    //             myArrivalPos = myVehicle->getArrivalPos();
    //         } else {
    //             myArrivalPos = myVehicle->getPositionOnLane();
    //         }
    //         const MSStoppingPlace* const stop = getDestinationStop();
    //         if (stop != nullptr) {
    //             MSStoppingPlace::AccessExit exit = MSStoppingPlace::AccessExit::PLATFORM;
    //             for (const auto& access : stop->getAllAccessPos()) {
    //                 if (access.exit != exit) {
    //                     exit = access.exit;
    //                     break;
    //                 }
    //             }
    //             if (exit != MSStoppingPlace::AccessExit::PLATFORM) {
    //                 MSVehicle* train = dynamic_cast<MSVehicle*>(myVehicle);
    //                 if (train != nullptr) {
    //                     MSTrainHelper trainHelper = MSTrainHelper(train);
    //                     const MSLane* const lane = myVehicle->getLane();
    //                     if (!net->getPersonControl().getMovementModel()->usingDoors()) {
    //                         trainHelper.computeDoorPositions();
    //                         const std::vector<MSTrainHelper::Carriage*>& carriages = trainHelper.getCarriages();
    //                         const int randomCarriageIx = RandHelper::rand(trainHelper.getNumCarriages() - trainHelper.getFirstPassengerCarriage()) + trainHelper.getFirstPassengerCarriage();
    //                         const MSTrainHelper::Carriage* randomCarriage = carriages[randomCarriageIx];
    //                         const int randomDoorIx = RandHelper::rand(trainHelper.getCarriageDoors());
    //                         Position randomDoor = randomCarriage->doorPositions[randomDoorIx];
    //                         // Jitter the position before projection because of possible train curvature.
    //                         Position direction = randomCarriage->front - randomCarriage->back;
    //                         direction.norm2D();
    //                         const double doorWidth = train->getVehicleType().getParameter().carriageDoorWidth;
    //                         randomDoor.add(direction * RandHelper::rand(-0.5 * doorWidth, 0.5 * doorWidth));
    //                         // Project onto the lane.
    //                         myArrivalPos = lane->getShape().nearest_offset_to_point2D(randomDoor);
    //                         myArrivalPos = lane->interpolateGeometryPosToLanePos(myArrivalPos);
    //                         myArrivalPos = MIN2(MAX2(0., myArrivalPos), myVehicle->getEdge()->getLength());
    //                     } else {
    //                         std::vector<Position>& unboardingPositions = static_cast<MSDevice_Transportable*>(train->getDevice(typeid(MSDevice_Transportable)))->getUnboardingPositions();
    //                         if (unboardingPositions.empty()) {
    //                             const MSVehicleType* defaultPedestrianType = MSNet::getInstance()->getVehicleControl().getVType(DEFAULT_PEDTYPE_ID, nullptr, true);
    //                             const double defaultPassengerRadius = MAX2(defaultPedestrianType->getLength(), defaultPedestrianType->getWidth()) / 2.;
    //                             trainHelper.computeUnboardingPositions(defaultPassengerRadius, unboardingPositions);
    //                         }
    //                         // Random shuffling of the positions has already been done in the train helper.
    //                         const Position availableUnboardingPosition = unboardingPositions.back();
    //                         unboardingPositions.pop_back();
    //                         const Position arrivalPos = lane->getShape().transformToVectorCoordinates(availableUnboardingPosition);
    //                         myArrivalPos = arrivalPos.x();
    //                         myArrivalPosLat = arrivalPos.y();
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         myVehicleDistance = -1.;
    //         myTimeLoss = -1;
    //     }
    //     myVehicle = nullptr; // avoid dangling pointer after vehicle arrival
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn set_arrived(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, vehicleArrived: bool) -> String {
        todo!("port MSStageDriving::setArrived")
    }

    /// C++: `const std::set<std::string> & MSStageDriving::getLines() const` [const]
    // const std::set<std::string>& getLines() const {
    //         return myLines;
    //     }
    #[allow(unused_variables)]
    pub fn get_lines(&self) -> &BTreeSet<String> {
        todo!("port MSStageDriving::getLines")
    }

    /// C++: `std::string MSStageDriving::getIntendedVehicleID() const` [const]
    // std::string getIntendedVehicleID() const {
    //         return myIntendedVehicleID;
    //     }
    #[allow(unused_variables)]
    pub fn get_intended_vehicle_id(&self) -> String {
        todo!("port MSStageDriving::getIntendedVehicleID")
    }

    /// C++: `SUMOTime MSStageDriving::getIntendedDepart() const` [const]
    // SUMOTime getIntendedDepart() const {
    //         return myIntendedDepart;
    //     }
    #[allow(unused_variables)]
    pub fn get_intended_depart(&self) -> SUMOTime {
        todo!("port MSStageDriving::getIntendedDepart")
    }

    /// C++: `std::string MSStageDriving::getVehicleType() const` [const]
    // std::string getVehicleType() const {
    //         return myVehicleType;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_type(&self) -> String {
        todo!("port MSStageDriving::getVehicleType")
    }

    /// C++: `void MSStageDriving::setOrigin(const MSEdge *, MSStoppingPlace *, double)` [virtual]
    /// change origin for parking area rerouting
    // void setOrigin(const MSEdge* origin, MSStoppingPlace* originStop, double departPos) {
    //         myOrigin = origin;
    //         myOriginStop = originStop;
    //         myWaitingPos = departPos;
    //     }
    #[allow(unused_variables)]
    pub fn set_origin(&mut self, origin: Ptr<MSEdge>, originStop: Ptr<MSStoppingPlace>, departPos: f64) {
        todo!("port MSStageDriving::setOrigin")
    }

    /// C++: `void MSStageDriving::setWaitingSince(SUMOTime)`
    /// restore further state during loading
    // void setWaitingSince(SUMOTime t) {
    //         myWaitingSince = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_waiting_since(&mut self, t: SUMOTime) {
        todo!("port MSStageDriving::setWaitingSince")
    }

    /// C++: `void MSStageDriving::setVehicleDistance(double)`
    /// restore further state during loading
    // void setVehicleDistance(double dist) {
    //         myVehicleDistance = dist;
    //     }
    #[allow(unused_variables)]
    pub fn set_vehicle_distance(&mut self, dist: f64) {
        todo!("port MSStageDriving::setVehicleDistance")
    }

    /// C++: `bool MSStageDriving::canLeaveVehicle(const MSTransportable *, const SUMOVehicle &, const MSStop &)`
    /// checks whether the person may exit at the current vehicle position
    // bool
    // MSStageDriving::canLeaveVehicle(const MSTransportable* t, const SUMOVehicle& veh, const MSStop& stop) {
    //     const MSEdge* stopEdge = stop.getEdge();
    //     bool canLeave = false;
    //     if (t->getDestination() == stopEdge) {
    //         // if this is the last stage, we can use the arrivalPos of the person
    //         const bool unspecifiedAP = unspecifiedArrivalPos() && (
    //                                        t->getNumRemainingStages() > 1 || !t->getParameter().wasSet(VEHPARS_ARRIVALPOS_SET));
    //         const double arrivalPos = (unspecifiedArrivalPos()
    //                                    ? SUMOVehicleParameter::interpretEdgePos(t->getParameter().arrivalPos, stopEdge->getLength(),
    //                                            SUMO_ATTR_ARRIVALPOS, t->getID(), true)
    //                                    : getArrivalPos());
    //         if (unspecifiedAP || stop.isInRange(arrivalPos, veh.getLength() + MSGlobals::gStopTolerance)) {
    //             canLeave = true;
    //         }
    //     }
    //     if (myDestinationStop != nullptr) {
    //         if (!canLeave) {
    //             // check with more tolerance due to busStop size and also check
    //             // access edges
    //             const double accessPos = myDestinationStop->getAccessPos(veh.getEdge());
    //             if (accessPos >= 0) {
    //                 double tolerance = veh.getLength() + MSGlobals::gStopTolerance;
    //                 if (&myDestinationStop->getLane().getEdge() == veh.getEdge()) {
    //                     // accessPos is in the middle of the stop
    //                     tolerance += (myDestinationStop->getEndLanePosition() - myDestinationStop->getBeginLanePosition()) / 2;
    //                 }
    //                 canLeave = stop.isInRange(accessPos, tolerance);
    //             }
    //         }
    //     }
    //     return canLeave;
    // }
    #[allow(unused_variables)]
    pub fn can_leave_vehicle(&mut self, t: Ptr<MSTransportable>, veh: &SUMOVehicle, stop: &MSStop) -> bool {
        todo!("port MSStageDriving::canLeaveVehicle")
    }

    /// C++: `SUMOTime MSStageDriving::getTimeLoss(const MSTransportable *) const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getTimeLoss(const MSTransportable* /*transportable*/) const {
    //     return myArrived >= 0 ? myTimeLoss : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_time_loss(&self, transportable: Ptr<MSTransportable>) -> SUMOTime {
        todo!("port MSStageDriving::getTimeLoss")
    }

    /// C++: `SUMOTime MSStageDriving::getDuration() const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getDuration() const {
    //     return myArrived >= 0 ? myArrived - myWaitingSince : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        todo!("port MSStageDriving::getDuration")
    }

    /// C++: `SUMOTime MSStageDriving::getTravelTime() const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getTravelTime() const {
    //     return myArrived >= 0 ? myArrived - myDeparted : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self) -> SUMOTime {
        todo!("port MSStageDriving::getTravelTime")
    }

    /// C++: `SUMOTime MSStageDriving::getWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getWaitingTime() const {
    //     const SUMOTime departed = myDeparted >= 0 ? myDeparted : SIMSTEP;
    //     return myWaitingSince >= 0 ? departed - myWaitingSince : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time_2(&self) -> SUMOTime {
        todo!("port MSStageDriving::getWaitingTime")
    }

    /// C++: `void MSStageDriving::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    /// Saves the current state into the given stream
    // void
    // MSStageDriving::saveState(std::ostringstream& out, MSTransportable* transportable) {
    //     const bool hasVehicle = myVehicle != nullptr;
    //     out << " " << myWaitingSince << " " << myTimeLoss << " " << myArrived << " " << hasVehicle;
    //     if (hasVehicle) {
    //         out << " " << myDeparted << " " << myVehicle->getID() << " " << myVehicleDistance;
    //     } else if (myOriginStop != nullptr) {
    //         out.setf(std::ios::fixed, std::ios::floatfield);
    //         out << std::setprecision(gPrecision);
    //         out << " " << myOriginStop->checkWaitingSpot(transportable);
    //         out << " " << myWaitingPos << " " << myStopWaitPos.x() << " " << myStopWaitPos.y();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        todo!("port MSStageDriving::saveState")
    }

    /// C++: `void MSStageDriving::loadState(MSTransportable *, std::istringstream &)` [virtual]
    /// Reconstructs the current state
    // void
    // MSStageDriving::loadState(MSTransportable* transportable, std::istringstream& state) {
    //     // there should always be at least one prior WAITING_FOR_DEPART stage
    //     MSStage* previous = transportable->getNextStage(-1);
    //     myOriginStop = (previous->getStageType() == MSStageType::TRIP
    //                     ? previous->getOriginStop()
    //                     : previous->getDestinationStop());
    //     bool hasVehicle;
    //     SUMOTime loadedTimeLoss;
    //     state >> myWaitingSince >> loadedTimeLoss >> myArrived >> hasVehicle;
    //     if (hasVehicle) {
    //         std::string vehID;
    //         state >> myDeparted >> vehID;
    //         SUMOVehicle* startVeh = MSNet::getInstance()->getVehicleControl().getVehicle(vehID);
    //         setVehicle(startVeh);
    //         myTimeLoss = loadedTimeLoss;
    //         myVehicle->addTransportable(transportable);
    //         state >> myVehicleDistance;
    //     } else if (myOriginStop != nullptr) {
    //         int waitingSpot;
    //         state >> waitingSpot;
    //         state >> myWaitingPos;
    //         double x, y;
    //         state >> x;
    //         state >> y;
    //         myStopWaitPos = Position(x, y);
    //         myOriginStop->addTransportable(transportable, waitingSpot);
    //         myWaitingEdge = &myOriginStop->getLane().getEdge();
    //     } else {
    //         myWaitingEdge = previous->getEdge();
    //         myStopWaitPos = Position::INVALID;
    //         myWaitingPos = previous->getArrivalPos();
    //     }
    //     // running reservations will be converted in MSDevice_Taxi::addReservation
    //     registerWaiting(transportable, myWaitingSince);
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut Unmapped /* std::istringstream */) {
        todo!("port MSStageDriving::loadState")
    }

    /// C++: `bool MSStageDriving::equals(const MSStage &) const` [const, virtual]
    // bool equals(const MSStage& s) const {
    //         if (!MSStage::equals(s)) {
    //             return false;
    //         }
    //         // this is safe because MSStage already checked that the type fits
    //         const MSStageDriving& sd = static_cast<const MSStageDriving&>(s);
    //         return myOrigin == sd.myOrigin &&
    //                myLines == sd.myLines &&
    //                myIntendedVehicleID == sd.myIntendedVehicleID;
    //     }
    #[allow(unused_variables)]
    pub fn equals(&self, s: &MSStage) -> bool {
        todo!("port MSStageDriving::equals")
    }

    /// C++: `void MSStageDriving::registerWaiting(MSTransportable *, SUMOTime)` [private]
    /// brief register waiting person (on proceed or loadState)
    // void
    // MSStageDriving::registerWaiting(MSTransportable* transportable, SUMOTime now) {
    //     // check if the ride can be conducted and reserve it
    //     if (MSDevice_Taxi::isReservation(getLines())) {
    //         const MSEdge* to = getDestination();
    //         double toPos = getArrivalPos();
    //         if ((to->getPermissions() & SVC_TAXI) == 0 && getDestinationStop() != nullptr) {
    //             // try to find usable access edge
    //             for (const auto& access : getDestinationStop()->getAllAccessPos()) {
    //                 const MSEdge* accessEdge = &access.lane->getEdge();
    //                 if ((accessEdge->getPermissions() & SVC_TAXI) != 0) {
    //                     to = accessEdge;
    //                     toPos = access.endPos;
    //                     break;
    //                 }
    //             }
    //         }
    //         if ((myWaitingEdge->getPermissions() & SVC_TAXI) == 0 && myOriginStop != nullptr) {
    //             // try to find usable access edge
    //             for (const auto& access : myOriginStop->getAllAccessPos()) {
    //                 const MSEdge* accessEdge = &access.lane->getEdge();
    //                 if ((accessEdge->getPermissions() & SVC_TAXI) != 0) {
    //                     myWaitingEdge = accessEdge;
    //                     myStopWaitPos = Position::INVALID;
    //                     myWaitingPos = access.endPos;
    //                     break;
    //                 }
    //             }
    //         }
    //         // Create reservation only if not already created by previous reservationTime
    //         if (myReservationWaitingPos == INVALID_DOUBLE) {
    //             MSDevice_Taxi::addReservation(transportable, getLines(), now, now, -1, myWaitingEdge, myWaitingPos, myOriginStop, to, toPos, myDestinationStop, myGroup);
    //         } else {
    //             // update "fromPos" with current (new) value of myWaitingPos
    //             MSDevice_Taxi::updateReservationFromPos(transportable, getLines(), myWaitingEdge, myReservationWaitingPos, to, toPos, myGroup, myWaitingPos);
    //         }
    //     }
    //     // check required for state-loading
    //     if (myVehicle == nullptr) {
    //         if (transportable->isPerson()) {
    //             MSNet::getInstance()->getPersonControl().addWaiting(myWaitingEdge, transportable);
    //         } else {
    //             MSNet::getInstance()->getContainerControl().addWaiting(myWaitingEdge, transportable);
    //         }
    //         myWaitingEdge->addTransportable(transportable);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn register_waiting(&mut self, transportable: Ptr<MSTransportable>, now: SUMOTime) {
        todo!("port MSStageDriving::registerWaiting")
    }

    /// C++: `void MSStageDriving::MSStageDriving(const MSStageDriving &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageDriving) -> Self {
        todo!("port MSStageDriving::MSStageDriving")
    }

    /// C++: `MSStageDriving & MSStageDriving::operator=(const MSStageDriving &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageDriving) -> &mut MSStageDriving {
        todo!("port MSStageDriving::operator=")
    }

}

