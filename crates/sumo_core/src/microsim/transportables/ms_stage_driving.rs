//! Port of microsim/transportables/MSStageDriving.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

use utils::common::msg_handler::write_warning;
use utils::options::options_cont;

// C++ MSPModel::FORWARD / UNDEFINED_DIRECTION (MSPModel.h)
const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;
// C++ LINE_ANY (SUMOVehicleParameter.h)
const LINE_ANY: &str = "ANY";
const VEHPARS_ARRIVALPOS_SET: i64 = 2 << 12;

// C++ joinToString(myLines, ",")
fn join_lines(lines: &BTreeSet<String>, sep: &str) -> String {
    lines.iter().cloned().collect::<Vec<String>>().join(sep)
}

fn next_i64(state: &mut std::str::SplitWhitespace<'_>) -> i64 {
    state.next().and_then(|t| t.parse::<i64>().ok()).unwrap_or(0)
}

fn next_f64(state: &mut std::str::SplitWhitespace<'_>) -> f64 {
    state.next().and_then(|t| t.parse::<f64>().ok()).unwrap_or(0.)
}

/// C++ class `MSStageDriving::BookReservation` (microsim/transportables/MSStageDriving.h)
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
        MSStageDriving_BookReservation {
            base: (),
            myTransportable: transportable,
            myEarliestPickupTime: earliestPickupTime,
            myStage: stage,
            myWaitingPos: stage.myWaitingPos,
        }
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
        MSDevice_Taxi::add_reservation(self.myTransportable, self.myStage.get_lines(), currentTime, currentTime, self.myEarliestPickupTime,
            self.myStage.myOrigin, self.myStage.myWaitingPos, self.myStage.myOriginStop, self.myStage.get_destination(), self.myStage.get_arrival_pos(), self.myStage.myDestinationStop, &self.myStage.myGroup);
        0 // do not repeat
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
        MSStageDriving {
            base: MSStage::new(MSStageType::DRIVING, destination, toStop, arrivalPos, arrivalPosLat, group),
            myOrigin: origin,
            myLines: lines.iter().cloned().collect(),
            myVehicle: Ptr::null(),
            myVehicleID: String::new(),
            myVehicleLine: String::new(),
            myVehicleType: String::new(),
            myVehicleVClass: SVC_IGNORING,
            myVehicleDistance: -1.,
            myTimeLoss: -1,
            myWaitingPos: -1.,
            myWaitingSince: -1,
            myWaitingEdge: origin,
            myStopWaitPos: Position::INVALID,
            myOriginStop: Ptr::null(),
            myIntendedVehicleID: intendedVeh.to_string(),
            myIntendedDepart: intendedDepart,
            myReservationWaitingPos: INVALID_DOUBLE,
        }
    }

    /// C++: `void MSStageDriving::~MSStageDriving() noexcept` [virtual]
    /// destructor
    // MSStageDriving::~MSStageDriving() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
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
        let lines: Vec<String> = self.myLines.iter().cloned().collect();
        let clon = MSStageDriving::new(self.myOrigin, self.myDestination, self.myDestinationStop, self.myArrivalPos, self.myArrivalPosLat,
            &lines, &self.myGroup, &self.myIntendedVehicleID, self.myIntendedDepart);
        let clonPtr = Ptr::from_mut(Box::leak(Box::new(clon))).cast::<MSStage>();
        clonPtr.m().set_parameters(self);
        clonPtr
    }

    /// C++: `double MSStageDriving::getArrivalPos() const` [const, virtual]
    /// return default value for undefined arrivalPos
    // double
    // MSStageDriving::getArrivalPos() const {
    //     return unspecifiedArrivalPos() ? getDestination()->getLength() : myArrivalPos;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        if self.unspecified_arrival_pos() { self.get_destination().get_length() } else { self.myArrivalPos }
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
        self.myDestinationStop = Ptr::null();
        if !self.myVehicle.is_null() {
            // jumping out of a moving vehicle!
            let v = self.myVehicle.cast::<MSVehicle>();
            v.m().remove_transportable(t);
            self.myDestination = if v.get_lane().is_null() { v.get_edge() } else { v.get_lane().get_edge() };
            self.myArrivalPos = v.get_position_on_lane();
            // myVehicleDistance and myTimeLoss are updated in subsequent call to setArrived
        } else {
            let net = MSNet::get_instance();
            let tc = if t.is_person() { net.m().get_person_control() } else { net.m().get_container_control() };
            tc.abort_waiting_for_vehicle(t);
            MSDevice_Taxi::remove_reservation(t, self.get_lines(), self.myWaitingEdge, self.myWaitingPos, self.myDestination, self.get_arrival_pos(), &self.myGroup);
            self.myDestination = self.myWaitingEdge;
            self.myDestinationStop = self.myOriginStop;
            self.myArrivalPos = self.myWaitingPos;
        }
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
        if self.has_parameter("earliestPickupTime") {
            let mut reservationTime = MSNet::get_instance().get_current_time_step();
            if self.has_parameter("reservationTime") {
                reservationTime = string2time(&self.get_parameter("reservationTime"));
            }
            let earliestPickupTime = string2time(&self.get_parameter("earliestPickupTime"));
            if transportable.get_next_stage(1) == Ptr::from_mut(&mut self.base) {
                // if the ride is the first stage use the departPos (there is a unvisible stop before)
                self.myWaitingPos = transportable.get_parameter().departPos;
            } else {
                // else use the middle of the edge, as also used as default for walk's arrivalPos
                self.myWaitingPos = self.myOrigin.get_length() / 2.;
            }
            self.myReservationWaitingPos = self.myWaitingPos;
            // shortcut: the BookReservation command is not a Command yet, so the earliestPickupTime reservation event is not scheduled
            let _ = (reservationTime, earliestPickupTime);
        }
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
        if !self.myVehicle.is_null() {
            let v = self.myVehicle.cast::<MSVehicle>();
            if !v.get_lane().is_null() {
                return v.get_lane().get_edge();
            }
            return v.get_edge();
        } else if self.myArrived >= 0 {
            return self.myDestination;
        } else {
            return self.myWaitingEdge;
        }
    }

    /// C++: `const MSEdge * MSStageDriving::getFromEdge() const` [const, virtual]
    // const MSEdge*
    // MSStageDriving::getFromEdge() const {
    //     return myWaitingEdge;
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        self.myWaitingEdge
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
        if self.is_waiting4_vehicle() {
            self.myWaitingPos
        } else if self.myArrived >= 0 {
            self.myArrivalPos
        } else {
            // vehicle may already have passed the lane (check whether this is correct)
            self.myVehicle.cast::<MSVehicle>().get_position_on_lane().min(self.get_edge().get_length())
        }
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
        if self.is_waiting4_vehicle() {
            MSPMODEL_UNDEFINED_DIRECTION
        } else if self.myArrived >= 0 {
            MSPMODEL_UNDEFINED_DIRECTION
        } else {
            MSPMODEL_FORWARD
        }
    }

    /// C++: `const MSLane * MSStageDriving::getLane() const` [const, virtual]
    // const MSLane*
    // MSStageDriving::getLane() const {
    //     return myVehicle != nullptr ? myVehicle->getLane() : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        if !self.myVehicle.is_null() { self.myVehicle.cast::<MSVehicle>().get_lane() } else { Ptr::null() }
    }

    /// C++: `MSStoppingPlace * MSStageDriving::getOriginStop() const` [const, virtual]
    // MSStoppingPlace* getOriginStop() const {
    //         return myOriginStop;
    //     }
    #[allow(unused_variables)]
    pub fn get_origin_stop(&self) -> Ptr<MSStoppingPlace> {
        self.myOriginStop
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
        if self.is_waiting4_vehicle() {
            if self.myStopWaitPos != Position::INVALID {
                return self.myStopWaitPos;
            }
            self.get_edge_position(self.myWaitingEdge, self.myWaitingPos,
                ROADSIDE_OFFSET * (if crate::microsim::ms_globals::lefthand() { -1. } else { 1. }))
        } else if self.myArrived >= 0 {
            self.get_edge_position(self.myDestination, self.myArrivalPos,
                ROADSIDE_OFFSET * (if crate::microsim::ms_globals::lefthand() { -1. } else { 1. }))
        } else {
            self.myVehicle.cast::<MSVehicle>().get_position()
        }
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
        let sign = if crate::microsim::ms_globals::lefthand() { -1. } else { 1. };
        if self.is_waiting4_vehicle() {
            let offset = if self.myOriginStop.is_null() { std::f64::consts::PI / 2. } else { self.myOriginStop.get_angle() };
            self.get_edge_angle(self.myWaitingEdge, self.myWaitingPos) + offset * sign
        } else if self.myArrived >= 0 {
            self.get_edge_angle(self.myDestination, self.myArrivalPos) + std::f64::consts::PI / 2. * sign
        } else if !self.myVehicle.is_null() {
            // C++ dynamic_cast<MSVehicle*>: the driving vehicle is always an MSVehicle in microsim
            self.myVehicle.cast::<MSVehicle>().get_angle()
        } else {
            0.
        }
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
        if !self.myVehicle.is_null() {
            // distance was previously set to driven distance upon embarking
            return self.myVehicle.get_odometer() - self.myVehicleDistance;
        }
        self.myVehicleDistance
    }

    /// C++: `std::string MSStageDriving::getStageDescription(const bool) const` [const, virtual]
    /// return (brief) string representation of the current stage
    // std::string
    // MSStageDriving::getStageDescription(const bool isPerson) const {
    //     return isWaiting4Vehicle() ? "waiting for " + joinToString(myLines, ",") : (isPerson ? "driving" : "transport");
    // }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        if self.is_waiting4_vehicle() {
            format!("waiting for {}", join_lines(&self.myLines, ","))
        } else if isPerson {
            "driving".to_string()
        } else {
            "transport".to_string()
        }
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
        let dest = if self.get_destination_stop().is_null() {
            format!(" edge '{}'", self.get_destination().get_id())
        } else {
            let s = self.get_destination_stop();
            format!(" stop '{}'{}", s.get_id(), if s.get_my_name() != "" { format!(" ({})", s.get_my_name()) } else { String::new() })
        };
        let intended = if self.myIntendedVehicleID != "" {
            format!(" (vehicle {} at time={})", self.myIntendedVehicleID, time2string(self.myIntendedDepart))
        } else {
            String::new()
        };
        let modeName = if isPerson { "driving" } else { "transported" };
        if self.is_waiting4_vehicle() {
            format!("waiting for {}{} then {} to {}", join_lines(&self.myLines, ","), intended, modeName, dest)
        } else {
            format!("{} to {}", modeName, dest)
        }
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
        self.myOriginStop = if previous.get_stage_type() == MSStageType::TRIP { previous.get_origin_stop() } else { previous.get_destination_stop() };
        self.myWaitingSince = now;
        let isPerson = transportable.is_person();
        if transportable.get_parameter().departProcedure == DepartDefinition::TRIGGERED && transportable.get_current_stage_index() == 1 {
            // we are the first real stage (stage 0 is WAITING_FOR_DEPART)
            let vehID = self.myLines.iter().next().unwrap().clone();
            let mut startVeh = net.m().get_vehicle_control().get_vehicle(&vehID);
            if startVeh.is_null() && net.m().has_flow(&vehID) {
                startVeh = net.m().get_insertion_control().get_last_flow_vehicle(&vehID);
            }
            if startVeh.is_null() {
                panic!("{}", ProcessError::new(format!("Vehicle '{}' not found for triggered departure of {} '{}'.",
                    vehID, if isPerson { "person" } else { "container" }, transportable.get_id())));
            }
            if isPerson {
                let pCap = startVeh.get_vehicle_type().get_parameter().personCapacity;
                if startVeh.get_person_number() >= pCap {
                    write_warning(&format!("Vehicle '{}' exceeds personCapacity {} when placing triggered person '{}', time={}",
                        startVeh.get_id(), pCap, transportable.get_id(), time2string(MSNet::get_instance().get_current_time_step())));
                }
            } else {
                let cCap = startVeh.get_vehicle_type().get_parameter().containerCapacity;
                if startVeh.get_container_number() >= cCap {
                    write_warning(&format!("Vehicle '{}' exceeds containerCapacity {} when placing triggered container '{}', time={}",
                        startVeh.get_id(), cCap, transportable.get_id(), time2string(MSNet::get_instance().get_current_time_step())));
                }
            }
            self.myDeparted = now;
            self.set_vehicle(startVeh);
            if !self.myOriginStop.is_null() {
                self.myOriginStop.m().remove_transportable(transportable);
            }
            self.myWaitingEdge = previous.get_edge();
            self.myStopWaitPos = Position::INVALID;
            self.myWaitingPos = previous.get_edge_pos(now);
            self.myVehicle.cast::<MSVehicle>().m().add_transportable(transportable);
            return true;
        }
        if !self.myOriginStop.is_null() {
            // the arrival stop may have an access point
            self.myWaitingEdge = self.myOriginStop.get_lane().get_edge();
            self.myStopWaitPos = self.myOriginStop.get_wait_position(transportable);
            self.myWaitingPos = self.myOriginStop.get_waiting_position_on_lane(transportable);
        } else {
            self.myWaitingEdge = previous.get_edge();
            self.myStopWaitPos = Position::INVALID;
            self.myWaitingPos = previous.get_edge_pos(now);
        }
        if !self.myOrigin.is_null() && self.myOrigin != self.myWaitingEdge
            && (self.myOriginStop.is_null() || self.myOriginStop.get_access_pos(self.myOrigin, self.myOrigin.get_lanes()[0].get_rng()) < 0.) {
            // transfer at junction (rather than access)
            self.myWaitingEdge = self.myOrigin;
            self.myWaitingPos = 0.;
        }
        let availableVehicle = self.myWaitingEdge.get_waiting_vehicle(transportable, self.myWaitingPos);
        let triggered = !availableVehicle.is_null()
            && ((isPerson && availableVehicle.get_parameter().departProcedure == DepartDefinition::TRIGGERED)
                || (!isPerson && availableVehicle.get_parameter().departProcedure == DepartDefinition::CONTAINER_TRIGGERED));
        if triggered && !availableVehicle.has_departed() {
            self.set_vehicle(availableVehicle);
            if !self.myOriginStop.is_null() {
                self.myOriginStop.m().remove_transportable(transportable);
            }
            self.myVehicle.cast::<MSVehicle>().m().add_transportable(transportable);
            net.m().get_insertion_control().add(self.myVehicle);
            net.m().get_vehicle_control().handle_triggered_depart(self.myVehicle, false);
        } else {
            self.register_waiting(transportable, now);
        }
        true
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
        let now = MSNet::get_instance().get_current_time_step();
        let waitingTime = self.get_waiting_time_2();
        let duration = self.myArrived - self.myDeparted;
        MSDevice_Tripinfo::add_ride_transport_data(transportable.is_person(), self.myVehicleDistance, duration, self.myVehicleVClass, &self.myVehicleLine, waitingTime);
        os.open_tag(if transportable.is_person() { "ride" } else { "transport" });
        os.write_attr("waitingTime", &(if waitingTime != SUMOTIME_MAX { time2string(waitingTime) } else { "-1".to_string() }));
        os.write_attr("vehicle", &(if self.myVehicleID.is_empty() { "NULL".to_string() } else { self.myVehicleID.clone() }));
        os.write_attr("depart", &(if self.myDeparted >= 0 { time2string(self.myDeparted) } else { "-1".to_string() }));
        os.write_attr("arrival", &(if self.myArrived >= 0 { time2string(self.myArrived) } else { "-1".to_string() }));
        os.write_attr("arrivalPos", &(if self.myArrived >= 0 { f64_to_string_prec(self.get_arrival_pos(), g_precision()) } else { "-1".to_string() }));
        let durationStr = if self.myArrived >= 0 { time2string(duration) } else if self.myDeparted >= 0 { time2string(now - self.myDeparted) } else { "-1".to_string() };
        os.write_attr("duration", &durationStr);
        os.write_attr("routeLength", &(if self.myArrived >= 0 || !self.myVehicle.is_null() { f64_to_string_prec(self.get_distance(), g_precision()) } else { "-1".to_string() }));
        os.write_attr("timeLoss", &(if self.myArrived >= 0 { time2string(self.get_time_loss(transportable)) } else { "-1".to_string() }));
        os.close_tag("");
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
        os.open_tag_id(if isPerson { SUMO_TAG_RIDE } else { SUMO_TAG_TRANSPORT });
        if !self.get_from_edge().is_null() {
            os.write_attr(SUMO_ATTR_FROM, self.get_from_edge().get_id());
        } else if !previous.is_null() && previous.get_stage_type() == MSStageType::WAITING_FOR_DEPART {
            os.write_attr(SUMO_ATTR_FROM, previous.get_edge().get_id());
        }
        os.write_attr(SUMO_ATTR_TO, self.get_destination().get_id());
        let mut comment = String::new();
        if !self.myDestinationStop.is_null() {
            let stop = self.myDestinationStop;
            os.write_attr(utils::xml::sumo_xml_definitions::Tags.get_string(stop.get_element()), stop.get_id());
            if stop.get_my_name() != "" {
                comment = format!(" <!-- {} -->", utils::common::string_utils::escape_xml(stop.get_my_name(), true));
            }
        } else if !self.unspecified_arrival_pos() {
            os.write_attr(SUMO_ATTR_ARRIVALPOS, &self.myArrivalPos);
        }
        if self.myLines.len() > 1 || self.myLines.iter().next().map_or(false, |l| l != LINE_ANY) {
            // no need to write the default
            os.write_attr(SUMO_ATTR_LINES, &join_lines(&self.myLines, " "));
        }
        if self.myIntendedVehicleID != "" {
            os.write_attr(SUMO_ATTR_INTENDED, &self.myIntendedVehicleID);
        }
        if self.myIntendedDepart >= 0 {
            os.write_attr(SUMO_ATTR_DEPART, &time2string(self.myIntendedDepart));
        }
        if withRouteLength {
            os.write_attr("routeLength", &self.myVehicleDistance);
        }
        if withTiming {
            os.write_attr("vehicle", &(if self.myVehicleID.is_empty() { "NULL".to_string() } else { self.myVehicleID.clone() }));
            os.write_attr(SUMO_ATTR_STARTED, &(if self.myDeparted >= 0 { time2string(self.myDeparted) } else { "-1".to_string() }));
            os.write_attr(SUMO_ATTR_ENDED, &(if self.myArrived >= 0 { time2string(self.myArrived) } else { "-1".to_string() }));
        }
        if options_cont::get_bool("vehroute-output.cost") {
            os.write_attr(SUMO_ATTR_COST, &self.get_costs());
        }
        os.close_tag(&comment);
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
        assert!(self.myLines.len() > 0);
        let line = vehicle.get_parameter().line.clone();
        // shortcut: MSDevice_Taxi::compatibleLine (a static C++ helper) is not ported yet, so taxi line compatibility is not checked
        self.myLines.contains(vehicle.get_id())
            || ((self.myLines.contains(&line) || self.myLines.contains(LINE_ANY))
                && (if self.myDestinationStop.is_null() { vehicle.stops_at_edge(self.myDestination) } else { vehicle.stops_at(self.myDestinationStop) }))
    }

    /// C++: `bool MSStageDriving::isWaiting4Vehicle() const` [const, virtual]
    /// Whether the person waits for a vehicle
    // bool
    // MSStageDriving::isWaiting4Vehicle() const {
    //     return myVehicle == nullptr && myArrived < 0;
    // }
    #[allow(unused_variables)]
    pub fn is_waiting4_vehicle(&self) -> bool {
        self.myVehicle.is_null() && self.myArrived < 0
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
        if self.is_waiting4_vehicle() {
            format!("waiting for {} at {}", join_lines(&self.myLines, ","),
                if self.myOriginStop.is_null() {
                    format!("edge '{}'", self.myWaitingEdge.get_id())
                } else {
                    format!("{} '{}'", utils::xml::sumo_xml_definitions::Tags.get_string(self.myOriginStop.get_element()), self.myOriginStop.get_id())
                })
        } else {
            String::new()
        }
    }

    /// C++: `SUMOVehicle * MSStageDriving::getVehicle() const` [const, virtual]
    // SUMOVehicle* getVehicle() const {
    //         return myVehicle;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<SUMOVehicle> {
        self.myVehicle
    }

    /// C++: `SUMOTime MSStageDriving::getWaitingTime(SUMOTime) const` [const]
    /// time spent waiting for a ride
    // SUMOTime
    // MSStageDriving::getWaitingTime(SUMOTime now) const {
    //     return isWaiting4Vehicle() ? now - myWaitingSince : 0;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self, now: SUMOTime) -> SUMOTime {
        if self.is_waiting4_vehicle() { now - self.myWaitingSince } else { 0 }
    }

    /// C++: `double MSStageDriving::getSpeed() const` [const, virtual]
    // double
    // MSStageDriving::getSpeed() const {
    //     return myVehicle == nullptr ? 0 : myVehicle->getSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        if self.myVehicle.is_null() { 0. } else { self.myVehicle.cast::<MSVehicle>().get_speed() }
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
        vec![self.get_from_edge(), self.get_destination()]
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
        self.myVehicle = v;
        if !self.myVehicle.is_null() {
            let veh = self.myVehicle.cast::<MSVehicle>();
            self.myVehicleID = v.get_id().to_string();
            self.myVehicleLine = v.get_parameter().line.clone();
            self.myVehicleType = veh.get_vehicle_type().get_id().to_string();
            self.myVehicleVClass = veh.get_v_class();
            if v.has_departed() {
                self.myVehicleDistance = v.get_odometer();
                self.myTimeLoss = v.get_time_loss();
            } else {
                // it probably got triggered by the person
                self.myVehicleDistance = 0.;
                self.myTimeLoss = 0;
            }
        }
    }

    /// C++: `void MSStageDriving::setVehicleID(const std::string)`
    // void setVehicleID(const std::string vehID) {
    //         myVehicleID = vehID;
    //     }
    #[allow(unused_variables)]
    pub fn set_vehicle_id(&mut self, vehID: String) {
        self.myVehicleID = vehID;
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
        self.base.set_arrived(net, transportable, now, vehicleArrived);
        if !self.myVehicle.is_null() {
            let v = self.myVehicle.cast::<MSVehicle>();
            // distance was previously set to driven distance upon embarking
            self.myVehicleDistance = v.get_odometer() - self.myVehicleDistance;
            self.myTimeLoss = v.get_time_loss() - self.myTimeLoss;
            if vehicleArrived {
                self.myArrivalPos = v.get_arrival_pos();
            } else {
                self.myArrivalPos = v.get_position_on_lane();
            }
            let stop = self.get_destination_stop();
            if !stop.is_null() {
                let mut exit = MSStoppingPlace_AccessExit::PLATFORM;
                for access in stop.get_all_access_pos().iter() {
                    if access.exit != exit {
                        exit = access.exit;
                        break;
                    }
                }
                if exit != MSStoppingPlace_AccessExit::PLATFORM {
                    // shortcut: the train door and unboarding position logic (MSTrainHelper, MSDevice_Transportable) is not ported, so the arrival position is not re-sampled for trains
                }
            }
        } else {
            self.myVehicleDistance = -1.;
            self.myTimeLoss = -1;
        }
        self.myVehicle = Ptr::null(); // avoid dangling pointer after vehicle arrival
        String::new()
    }

    /// C++: `const std::set<std::string> & MSStageDriving::getLines() const` [const]
    // const std::set<std::string>& getLines() const {
    //         return myLines;
    //     }
    #[allow(unused_variables)]
    pub fn get_lines(&self) -> &BTreeSet<String> {
        &self.myLines
    }

    /// C++: `std::string MSStageDriving::getIntendedVehicleID() const` [const]
    // std::string getIntendedVehicleID() const {
    //         return myIntendedVehicleID;
    //     }
    #[allow(unused_variables)]
    pub fn get_intended_vehicle_id(&self) -> String {
        self.myIntendedVehicleID.clone()
    }

    /// C++: `SUMOTime MSStageDriving::getIntendedDepart() const` [const]
    // SUMOTime getIntendedDepart() const {
    //         return myIntendedDepart;
    //     }
    #[allow(unused_variables)]
    pub fn get_intended_depart(&self) -> SUMOTime {
        self.myIntendedDepart
    }

    /// C++: `std::string MSStageDriving::getVehicleType() const` [const]
    // std::string getVehicleType() const {
    //         return myVehicleType;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_type(&self) -> String {
        self.myVehicleType.clone()
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
        self.myOrigin = origin;
        self.myOriginStop = originStop;
        self.myWaitingPos = departPos;
    }

    /// C++: `void MSStageDriving::setWaitingSince(SUMOTime)`
    /// restore further state during loading
    // void setWaitingSince(SUMOTime t) {
    //         myWaitingSince = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_waiting_since(&mut self, t: SUMOTime) {
        self.myWaitingSince = t;
    }

    /// C++: `void MSStageDriving::setVehicleDistance(double)`
    /// restore further state during loading
    // void setVehicleDistance(double dist) {
    //         myVehicleDistance = dist;
    //     }
    #[allow(unused_variables)]
    pub fn set_vehicle_distance(&mut self, dist: f64) {
        self.myVehicleDistance = dist;
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
        let stopEdge = stop.get_edge();
        let mut canLeave = false;
        if t.get_destination() == stopEdge {
            // if this is the last stage, we can use the arrivalPos of the person
            let unspecifiedAP = self.unspecified_arrival_pos()
                && (t.get_num_remaining_stages() > 1 || !t.get_parameter().wasSet(VEHPARS_ARRIVALPOS_SET));
            let arrivalPos = if self.unspecified_arrival_pos() {
                SUMOVehicleParameter::interpret_edge_pos(t.get_parameter().arrivalPos, stopEdge.get_length(), SUMO_ATTR_ARRIVALPOS, t.get_id(), true)
            } else {
                self.get_arrival_pos()
            };
            if unspecifiedAP || stop.is_in_range(arrivalPos, veh.get_length() + crate::microsim::ms_globals::stop_tolerance()) {
                canLeave = true;
            }
        }
        if !self.myDestinationStop.is_null() {
            if !canLeave {
                // check with more tolerance due to busStop size and also check access edges
                let accessPos = self.myDestinationStop.get_access_pos(veh.get_edge(), veh.get_edge().get_lanes()[0].get_rng());
                if accessPos >= 0. {
                    let mut tolerance = veh.get_length() + crate::microsim::ms_globals::stop_tolerance();
                    if self.myDestinationStop.get_lane().get_edge() == veh.get_edge() {
                        // accessPos is in the middle of the stop
                        tolerance += (self.myDestinationStop.get_end_lane_position() - self.myDestinationStop.get_begin_lane_position()) / 2.;
                    }
                    canLeave = stop.is_in_range(accessPos, tolerance);
                }
            }
        }
        canLeave
    }

    /// C++: `SUMOTime MSStageDriving::getTimeLoss(const MSTransportable *) const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getTimeLoss(const MSTransportable* /*transportable*/) const {
    //     return myArrived >= 0 ? myTimeLoss : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_time_loss(&self, transportable: Ptr<MSTransportable>) -> SUMOTime {
        if self.myArrived >= 0 { self.myTimeLoss } else { SUMOTIME_MAX }
    }

    /// C++: `SUMOTime MSStageDriving::getDuration() const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getDuration() const {
    //     return myArrived >= 0 ? myArrived - myWaitingSince : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        if self.myArrived >= 0 { self.myArrived - self.myWaitingSince } else { SUMOTIME_MAX }
    }

    /// C++: `SUMOTime MSStageDriving::getTravelTime() const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getTravelTime() const {
    //     return myArrived >= 0 ? myArrived - myDeparted : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self) -> SUMOTime {
        if self.myArrived >= 0 { self.myArrived - self.myDeparted } else { SUMOTIME_MAX }
    }

    /// C++: `SUMOTime MSStageDriving::getWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStageDriving::getWaitingTime() const {
    //     const SUMOTime departed = myDeparted >= 0 ? myDeparted : SIMSTEP;
    //     return myWaitingSince >= 0 ? departed - myWaitingSince : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time_2(&self) -> SUMOTime {
        let departed = if self.myDeparted >= 0 { self.myDeparted } else { MSNet::get_instance().get_current_time_step() };
        if self.myWaitingSince >= 0 { departed - self.myWaitingSince } else { SUMOTIME_MAX }
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
    pub fn save_state(&mut self, out: &mut String /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        let hasVehicle = !self.myVehicle.is_null();
        out.push_str(&format!(" {} {} {} {}", self.myWaitingSince, self.myTimeLoss, self.myArrived, if hasVehicle { 1 } else { 0 }));
        if hasVehicle {
            out.push_str(&format!(" {} {} {}", self.myDeparted, self.myVehicle.get_id(), utils::common::to_string::f64_to_string_default(self.myVehicleDistance)));
        } else if !self.myOriginStop.is_null() {
            let p = g_precision();
            out.push_str(&format!(" {} {} {} {}", self.myOriginStop.check_waiting_spot(transportable),
                f64_to_string_prec(self.myWaitingPos, p), f64_to_string_prec(self.myStopWaitPos.x(), p), f64_to_string_prec(self.myStopWaitPos.y(), p)));
        }
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
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut std::str::SplitWhitespace<'_> /* std::istringstream */) {
        // there should always be at least one prior WAITING_FOR_DEPART stage
        let previous = transportable.get_next_stage(-1);
        self.myOriginStop = if previous.get_stage_type() == MSStageType::TRIP { previous.get_origin_stop() } else { previous.get_destination_stop() };
        self.myWaitingSince = next_i64(state);
        let loadedTimeLoss = next_i64(state);
        self.myArrived = next_i64(state);
        let hasVehicle = next_i64(state) != 0;
        if hasVehicle {
            self.myDeparted = next_i64(state);
            let vehID = state.next().unwrap_or("").to_string();
            let startVeh = MSNet::get_instance().m().get_vehicle_control().get_vehicle(&vehID);
            self.set_vehicle(startVeh);
            self.myTimeLoss = loadedTimeLoss;
            self.myVehicle.cast::<MSVehicle>().m().add_transportable(transportable);
            self.myVehicleDistance = next_f64(state);
        } else if !self.myOriginStop.is_null() {
            let waitingSpot = next_i64(state) as i32;
            self.myWaitingPos = next_f64(state);
            let x = next_f64(state);
            let y = next_f64(state);
            self.myStopWaitPos = Position::new(x, y);
            self.myOriginStop.m().add_transportable(transportable, waitingSpot);
            self.myWaitingEdge = self.myOriginStop.get_lane().get_edge();
        } else {
            self.myWaitingEdge = previous.get_edge();
            self.myStopWaitPos = Position::INVALID;
            self.myWaitingPos = previous.get_arrival_pos();
        }
        // running reservations will be converted in MSDevice_Taxi::addReservation
        self.register_waiting(transportable, self.myWaitingSince);
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
        if !self.base.equals(s) {
            return false;
        }
        // this is safe because MSStage already checked that the type fits
        let sd = Ptr::new(s).cast::<MSStageDriving>();
        self.myOrigin == sd.myOrigin && self.myLines == sd.myLines && self.myIntendedVehicleID == sd.myIntendedVehicleID
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
        // check if the ride can be conducted and reserve it
        if MSDevice_Taxi::is_reservation(self.get_lines()) {
            let mut to = self.get_destination();
            let mut toPos = self.get_arrival_pos();
            if (to.get_permissions() & SVC_TAXI) == 0 && !self.get_destination_stop().is_null() {
                // try to find usable access edge
                for access in self.get_destination_stop().get_all_access_pos().iter() {
                    let accessEdge = access.lane.get_edge();
                    if (accessEdge.get_permissions() & SVC_TAXI) != 0 {
                        to = accessEdge;
                        toPos = access.endPos;
                        break;
                    }
                }
            }
            if (self.myWaitingEdge.get_permissions() & SVC_TAXI) == 0 && !self.myOriginStop.is_null() {
                // try to find usable access edge
                for access in self.myOriginStop.get_all_access_pos().iter() {
                    let accessEdge = access.lane.get_edge();
                    if (accessEdge.get_permissions() & SVC_TAXI) != 0 {
                        self.myWaitingEdge = accessEdge;
                        self.myStopWaitPos = Position::INVALID;
                        self.myWaitingPos = access.endPos;
                        break;
                    }
                }
            }
            // Create reservation only if not already created by previous reservationTime
            if self.myReservationWaitingPos == INVALID_DOUBLE {
                MSDevice_Taxi::add_reservation(transportable, self.get_lines(), now, now, -1, self.myWaitingEdge, self.myWaitingPos, self.myOriginStop, to, toPos, self.myDestinationStop, &self.myGroup);
            } else {
                // update "fromPos" with current (new) value of myWaitingPos
                MSDevice_Taxi::update_reservation_from_pos(transportable, self.get_lines(), self.myWaitingEdge, self.myReservationWaitingPos, to, toPos, &self.myGroup, self.myWaitingPos);
            }
        }
        // check required for state-loading
        if self.myVehicle.is_null() {
            let net = MSNet::get_instance();
            if transportable.is_person() {
                net.m().get_person_control().add_waiting(self.myWaitingEdge, transportable);
            } else {
                net.m().get_container_control().add_waiting(self.myWaitingEdge, transportable);
            }
            self.myWaitingEdge.add_transportable(transportable);
        }
    }

    /// C++: `void MSStageDriving::MSStageDriving(const MSStageDriving &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageDriving) -> Self {
        panic!("MSStageDriving copy constructor is private and invalidated in C++")
    }

    /// C++: `MSStageDriving & MSStageDriving::operator=(const MSStageDriving &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageDriving) -> &mut MSStageDriving {
        panic!("MSStageDriving assignment operator is private and invalidated in C++")
    }

}

