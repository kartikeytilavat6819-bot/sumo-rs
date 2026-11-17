//! Port of microsim/transportables/MSStage.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::transportables::ms_person::MSPerson_MSPersonStage_Access;
use crate::microsim::transportables::ms_stage_driving::MSStageDriving;
use crate::microsim::transportables::ms_stage_tranship::MSStageTranship;
use crate::microsim::transportables::ms_stage_trip::MSStageTrip;
use crate::microsim::transportables::ms_stage_waiting::MSStageWaiting;
use crate::microsim::transportables::ms_stage_walking::MSStageWalking;

/// C++ enum `MSStageType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSStageType {
    WAITING_FOR_DEPART = 0,
    WAITING = 1,
    WALKING = 2,
    DRIVING = 3,
    ACCESS = 4,
    TRIP = 5,
    TRANSHIP = 6,
}

/// The "abstract" class for a single stage of a movement Contains the destination of the current movement step
/// C++ class `MSStage` (microsim/transportables/MSStage.h)
#[repr(C)]
pub struct MSStage {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// the next edge to reach by getting transported
    pub myDestination: Ptr<MSEdge>,
    /// the stop to reach by getting transported (if any)
    pub myDestinationStop: Ptr<MSStoppingPlace>,
    /// the longitudinal position at which we want to arrive
    pub myArrivalPos: f64,
    /// the lateral position at which we want to arrive
    pub myArrivalPosLat: f64,
    /// the time at which this stage started
    pub myDeparted: SUMOTime,
    /// the time at which this stage ended
    pub myArrived: SUMOTime,
    /// The type of this stage
    pub myType: MSStageType,
    /// The id of the group of transportables traveling together
    pub myGroup: String,
    /// The assigned or calculated costs
    pub myCosts: f64,
    /// Information on which parameter were set (mainly for vehroute output)
    pub myParametersSet: i32,
    pub myTrip: Ptr<MSStageTrip>,
}

impl std::ops::Deref for MSStage {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for MSStage {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

// C++ static members of MSStage (port as module-level statics, see AGENTS.md):
//   static const double MSStage::ARRIVALPOS_UNSPECIFIED;
//   static const double MSStage::ROADSIDE_OFFSET;
pub const ARRIVALPOS_UNSPECIFIED: f64 = f64::INFINITY;
pub const ROADSIDE_OFFSET: f64 = 3.;

impl MSStage {
    /// Views this base stage as its concrete subclass. Only called after a `myType` check; every
    /// concrete stage is `#[repr(C)]` with `base` first, so the prefix cast is valid.
    fn as_concrete<T>(&self) -> &T {
        unsafe { &*(self as *const MSStage as *const T) }
    }

    fn as_concrete_mut<T>(&mut self) -> &mut T {
        unsafe { &mut *(self as *mut MSStage as *mut T) }
    }

    /// C++: `void MSStage::MSStage(const MSStageType, const MSEdge *, MSStoppingPlace *, const double, const double, const std::string &)`
    /// constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(type_: MSStageType, destination: Ptr<MSEdge>, toStop: Ptr<MSStoppingPlace>, arrivalPos: f64, arrivalPosLat: f64, group: &str) -> Self {
        MSStage {
            base: Parameterised::new(),
            myDestination: destination,
            myDestinationStop: toStop,
            myArrivalPos: arrivalPos,
            myArrivalPosLat: arrivalPosLat,
            myDeparted: -1,
            myArrived: -1,
            myType: type_,
            myGroup: group.to_string(),
            myCosts: -1.,
            myParametersSet: 0,
            myTrip: Ptr::null(),
        }
    }

    /// C++: `void MSStage::~MSStage() noexcept` [virtual]
    /// destructor
    // MSStage::~MSStage() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
    }

    /// C++: `void MSStage::init(MSTransportable *)` [virtual]
    /// initialization, e.g. for param-related events
    // virtual void init(MSTransportable* /*transportable*/) {}
    #[allow(unused_variables)]
    pub fn init(&mut self, arg0: Ptr<MSTransportable>) {
        match self.myType {
            MSStageType::DRIVING => self.as_concrete_mut::<MSStageDriving>().init(arg0),
            _ => {}
        }
    }

    /// C++: `const MSEdge * MSStage::getDestination() const` [const]
    /// returns the destination edge
    // const MSEdge*
    // MSStage::getDestination() const {
    //     return myDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<MSEdge> {
        self.myDestination
    }

    /// C++: `MSStoppingPlace * MSStage::getDestinationStop() const` [const]
    /// returns the destination stop (if any)
    // MSStoppingPlace* getDestinationStop() const {
    //         return myDestinationStop;
    //     }
    #[allow(unused_variables)]
    pub fn get_destination_stop(&self) -> Ptr<MSStoppingPlace> {
        self.myDestinationStop
    }

    /// C++: `MSStoppingPlace * MSStage::getOriginStop() const` [const, virtual]
    /// returns the origin stop (if any). only needed for MSStageTrip
    // virtual MSStoppingPlace* getOriginStop() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_origin_stop(&self) -> Ptr<MSStoppingPlace> {
        match self.myType {
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_origin_stop(),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_origin_stop(),
            _ => Ptr::null(),
        }
    }

    /// C++: `double MSStage::getArrivalPos() const` [const, virtual]
    // virtual double getArrivalPos() const {
    //         return myArrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_arrival_pos(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_arrival_pos(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_arrival_pos(),
            _ => self.myArrivalPos,
        }
    }

    /// C++: `bool MSStage::unspecifiedArrivalPos() const` [const]
    // bool
    // MSStage::unspecifiedArrivalPos() const {
    //     return myArrivalPos == ARRIVALPOS_UNSPECIFIED;
    // }
    #[allow(unused_variables)]
    pub fn unspecified_arrival_pos(&self) -> bool {
        self.myArrivalPos == ARRIVALPOS_UNSPECIFIED
    }

    /// C++: `double MSStage::getArrivalPosLat() const` [const, virtual]
    // virtual double getArrivalPosLat() const {
    //         return myArrivalPosLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos_lat(&self) -> f64 {
        self.myArrivalPosLat
    }

    /// C++: `void MSStage::setArrivalPos(double)`
    // void setArrivalPos(double arrivalPos) {
    //         myArrivalPos = arrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn set_arrival_pos(&mut self, arrivalPos: f64) {
        self.myArrivalPos = arrivalPos;
    }

    /// C++: `const MSEdge * MSStage::getEdge() const` [const, virtual]
    /// Returns the current edge
    // const MSEdge*
    // MSStage::getEdge() const {
    //     return myDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_edge(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_edge(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_edge(),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_edge(),
            _ => self.myDestination,
        }
    }

    /// C++: `const MSEdge * MSStage::getFromEdge() const` [const, virtual]
    // const MSEdge*
    // MSStage::getFromEdge() const {
    //     return myDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_from_edge(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_from_edge(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_from_edge(),
            _ => self.myDestination,
        }
    }

    /// C++: `double MSStage::getEdgePos(SUMOTime) const` [const, virtual]
    // double
    // MSStage::getEdgePos(SUMOTime /* now */) const {
    //     return myArrivalPos;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_edge_pos(now),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_edge_pos(now),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_edge_pos(now),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_edge_pos(now),
            _ => self.myArrivalPos,
        }
    }

    /// C++: `double MSStage::getEdgePosLat(SUMOTime) const` [const, virtual]
    // double
    // MSStage::getEdgePosLat(SUMOTime /* now */) const {
    //     return myArrivalPosLat;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos_lat(&self, now: SUMOTime) -> f64 {
        self.myArrivalPosLat
    }

    /// C++: `int MSStage::getDirection() const` [const, virtual]
    /// Return the movement directon on the edge
    // int
    // MSStage::getDirection() const {
    //     return MSPModel::UNDEFINED_DIRECTION;
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_direction(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_direction(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_direction(),
            // shortcut: MSPModel::UNDEFINED_DIRECTION (=0) not ported as a constant yet
            _ => 0,
        }
    }

    /// C++: `Position MSStage::getPosition(SUMOTime) const` [const, virtual, pure virtual]
    /// returns the position of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_position(now),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_position(now),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_position(now),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_position(now),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().get_position(now),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_position(now),
        }
    }

    /// C++: `double MSStage::getAngle(SUMOTime) const` [const, virtual, pure virtual]
    /// returns the angle of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_angle(now),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_angle(now),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_angle(now),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_angle(now),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().get_angle(now),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_angle(now),
        }
    }

    /// C++: `const MSLane * MSStage::getLane() const` [const, virtual]
    /// Returns the current lane (if applicable)
    // virtual const MSLane* getLane() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_lane(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_lane(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_lane(),
            _ => Ptr::null(),
        }
    }

    /// C++: `SUMOTime MSStage::getJumpDuration() const` [const, virtual]
    /// Return the current jump duration (if applicable)
    // virtual SUMOTime getJumpDuration() const {
    //         return -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_jump_duration(&self) -> SUMOTime {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_jump_duration(),
            _ => -1,
        }
    }

    /// C++: `MSStageType MSStage::getStageType() const` [const]
    // MSStageType getStageType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_type(&self) -> MSStageType {
        self.myType
    }

    /// C++: `const std::string & MSStage::getGroup() const` [const]
    /// return the id of the group of transportables traveling together
    // const std::string& getGroup() const {
    //         return myGroup;
    //     }
    #[allow(unused_variables)]
    pub fn get_group(&self) -> &str {
        &self.myGroup
    }

    /// C++: `std::string MSStage::getStageDescription(const bool) const` [const, virtual, pure virtual]
    /// return (brief) string representation of the current stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_stage_description(isPerson),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_stage_description(isPerson),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_stage_description(isPerson),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_stage_description(isPerson),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().get_stage_description(isPerson),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_stage_description(isPerson),
        }
    }

    /// C++: `std::string MSStage::getStageSummary(const bool) const` [const, virtual, pure virtual]
    /// return string summary of the current stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_stage_summary(isPerson),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_stage_summary(isPerson),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_stage_summary(isPerson),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_stage_summary(isPerson),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().get_stage_summary(isPerson),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_stage_summary(isPerson),
        }
    }

    /// C++: `bool MSStage::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual, pure virtual]
    /// proceeds to this stage and return True if everything is fine. May return false if the person is unable to proceed in this stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete_mut::<MSStageWaiting>().proceed(net, transportable, now, previous),
            MSStageType::WALKING => self.as_concrete_mut::<MSStageWalking>().proceed(net, transportable, now, previous),
            MSStageType::TRANSHIP => self.as_concrete_mut::<MSStageTranship>().proceed(net, transportable, now, previous),
            MSStageType::DRIVING => self.as_concrete_mut::<MSStageDriving>().proceed(net, transportable, now, previous),
            MSStageType::ACCESS => self.as_concrete_mut::<MSPerson_MSPersonStage_Access>().proceed(net, transportable, now, previous),
            MSStageType::TRIP => self.as_concrete_mut::<MSStageTrip>().proceed(net, transportable, now, previous),
        }
    }

    /// C++: `void MSStage::abort(MSTransportable *)` [virtual]
    /// abort this stage (TraCI)
    // virtual void abort(MSTransportable*) {}
    #[allow(unused_variables)]
    pub fn abort(&mut self, arg0: Ptr<MSTransportable>) {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete_mut::<MSStageWaiting>().abort(arg0),
            MSStageType::WALKING => self.as_concrete_mut::<MSStageWalking>().abort(arg0),
            MSStageType::DRIVING => self.as_concrete_mut::<MSStageDriving>().abort(arg0),
            _ => {}
        }
    }

    /// C++: `void MSStage::setSpeed(double)` [virtual]
    /// sets the walking speed (ignored in other stages)
    // virtual void setSpeed(double) {}
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, arg0: f64) {
        match self.myType {
            MSStageType::WALKING => self.as_concrete_mut::<MSStageWalking>().set_speed(arg0),
            _ => {}
        }
    }

    /// C++: `SUMOTime MSStage::getDeparted() const` [const]
    /// get departure time of stage
    // SUMOTime
    // MSStage::getDeparted() const {
    //     return myDeparted;
    // }
    #[allow(unused_variables)]
    pub fn get_departed(&self) -> SUMOTime {
        self.myDeparted
    }

    /// C++: `SUMOTime MSStage::getArrived() const` [const]
    /// get arrival time of stage
    // SUMOTime
    // MSStage::getArrived() const {
    //     return myArrived;
    // }
    #[allow(unused_variables)]
    pub fn get_arrived(&self) -> SUMOTime {
        self.myArrived
    }

    /// C++: `SUMOTime MSStage::getTimeLoss(const MSTransportable *) const` [const, virtual]
    // SUMOTime
    // MSStage::getTimeLoss(const MSTransportable* /*transportable*/) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_time_loss(&self, transportable: Ptr<MSTransportable>) -> SUMOTime {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_time_loss(transportable),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_time_loss(transportable),
            _ => 0,
        }
    }

    /// C++: `SUMOTime MSStage::getDuration() const` [const, virtual]
    // SUMOTime
    // MSStage::getDuration() const {
    //     return myArrived >= 0 ? myArrived - myDeparted : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_duration(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_duration(),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_duration(),
            _ => self.get_duration_base(),
        }
    }

    /// The C++ body of `MSStage::getDuration` without virtual dispatch (for subclasses calling it explicitly).
    pub fn get_duration_base(&self) -> SUMOTime {
        if self.myArrived >= 0 {
            self.myArrived - self.myDeparted
        } else {
            SUMOTIME_MAX
        }
    }

    /// C++: `SUMOTime MSStage::getTravelTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getTravelTime() const {
    //     return getDuration();
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self) -> SUMOTime {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_travel_time(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_travel_time(),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_travel_time(),
            _ => self.get_duration(),
        }
    }

    /// C++: `SUMOTime MSStage::getWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getWaitingTime() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_waiting_time(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_waiting_time(),
            _ => 0,
        }
    }

    /// C++: `SUMOTime MSStage::getTotalWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getTotalWaitingTime() const {
    //     return getWaitingTime();
    // }
    #[allow(unused_variables)]
    pub fn get_total_waiting_time(&self) -> SUMOTime {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_total_waiting_time(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_total_waiting_time(),
            _ => self.get_waiting_time(),
        }
    }

    /// C++: `void MSStage::setDeparted(SUMOTime)`
    /// logs beginning of stage
    // void
    // MSStage::setDeparted(SUMOTime now) {
    //     if (myDeparted < 0) {
    //         myDeparted = now;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_departed(&mut self, now: SUMOTime) {
        if self.myDeparted < 0 {
            self.myDeparted = now;
        }
    }

    /// C++: `const std::string MSStage::setArrived(MSNet *, MSTransportable *, SUMOTime, const bool)` [virtual]
    /// logs end of the step
    // const std::string
    // MSStage::setArrived(MSNet* /* net */, MSTransportable* /* transportable */, SUMOTime now, const bool /* vehicleArrived */) {
    //     myArrived = now;
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn set_arrived(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, vehicleArrived: bool) -> String {
        match self.myType {
            MSStageType::DRIVING => self.as_concrete_mut::<MSStageDriving>().set_arrived(net, transportable, now, vehicleArrived),
            MSStageType::TRIP => self.as_concrete_mut::<MSStageTrip>().set_arrived(net, transportable, now, vehicleArrived),
            _ => {
                self.myArrived = now;
                String::new()
            }
        }
    }

    /// C++: `void MSStage::setEnded(SUMOTime)`
    /// sets end of stage
    // void
    // MSStage::setEnded(SUMOTime t) {
    //     myArrived = t;
    // }
    #[allow(unused_variables)]
    pub fn set_ended(&mut self, t: SUMOTime) {
        self.myArrived = t;
    }

    /// C++: `bool MSStage::isWaitingFor(const SUMOVehicle *) const` [const, virtual]
    /// Whether the transportable waits for the given vehicle
    // bool
    // MSStage::isWaitingFor(const SUMOVehicle* /*vehicle*/) const {
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_waiting_for(&self, vehicle: Ptr<SUMOVehicle>) -> bool {
        match self.myType {
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().is_waiting_for(vehicle),
            _ => false,
        }
    }

    /// C++: `bool MSStage::isWaiting4Vehicle() const` [const, virtual]
    /// Whether the transportable waits for a vehicle
    // virtual bool isWaiting4Vehicle() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting4_vehicle(&self) -> bool {
        match self.myType {
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().is_waiting4_vehicle(),
            _ => false,
        }
    }

    /// C++: `bool MSStage::isWalk() const` [const, virtual]
    /// Whether the transportable is walking
    // virtual bool isWalk() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_walk(&self) -> bool {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().is_walk(),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().is_walk(),
            _ => false,
        }
    }

    /// C++: `SUMOVehicle * MSStage::getVehicle() const` [const, virtual]
    /// Current vehicle in which the transportable is driving (or nullptr)
    // virtual SUMOVehicle* getVehicle() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<SUMOVehicle> {
        match self.myType {
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_vehicle(),
            _ => Ptr::null(),
        }
    }

    /// C++: `double MSStage::getSpeed() const` [const, virtual]
    /// the speed of the transportable
    // double
    // MSStage::getSpeed() const {
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_speed(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_speed(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_speed(),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().get_speed(),
            _ => 0.,
        }
    }

    /// C++: `ConstMSEdgeVector MSStage::getEdges() const` [const, virtual]
    /// the edges of the current stage
    // ConstMSEdgeVector
    // MSStage::getEdges() const {
    //     ConstMSEdgeVector result;
    //     result.push_back(getDestination());
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> ConstMSEdgeVector {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_edges(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_edges(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_edges(),
            _ => vec![self.get_destination()],
        }
    }

    /// C++: `int MSStage::getRoutePosition() const` [const, virtual]
    /// return index of current edge within route
    // virtual int getRoutePosition() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_route_position(),
            _ => 0,
        }
    }

    /// C++: `Position MSStage::getEdgePosition(const MSEdge *, double, double) const` [const]
    /// get position on edge e at length at with orthogonal offset
    // Position
    // MSStage::getEdgePosition(const MSEdge* e, double at, double offset) const {
    //     return getLanePosition(e->getLanes()[0], at, offset);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_position(&self, e: Ptr<MSEdge>, at: f64, offset: f64) -> Position {
        let lane = e.get_lanes()[0];
        self.get_lane_position(lane, at, offset)
    }

    /// C++: `Position MSStage::getLanePosition(const MSLane *, double, double) const` [const]
    /// get position on lane at length at with orthogonal offset
    // Position
    // MSStage::getLanePosition(const MSLane* lane, double at, double offset) const {
    //     return lane->getShape().positionAtOffset(lane->interpolateLanePosToGeometryPos(at), offset);
    // }
    #[allow(unused_variables)]
    pub fn get_lane_position(&self, lane: Ptr<MSLane>, at: f64, offset: f64) -> Position {
        lane.get_shape_2().position_at_offset(lane.interpolate_lane_pos_to_geometry_pos(at), offset)
    }

    /// C++: `double MSStage::getEdgeAngle(const MSEdge *, double) const` [const]
    /// get angle of the edge at a certain position
    // double
    // MSStage::getEdgeAngle(const MSEdge* e, double at) const {
    //     return e->getLanes()[0]->getShape().rotationAtOffset(at);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_angle(&self, e: Ptr<MSEdge>, at: f64) -> f64 {
        e.get_lanes()[0].get_shape_2().rotation_at_offset(at)
    }

    /// C++: `void MSStage::setDestination(const MSEdge *, MSStoppingPlace *)`
    // void
    // MSStage::setDestination(const MSEdge* newDestination, MSStoppingPlace* newDestStop) {
    //     myDestination = newDestination;
    //     myDestinationStop = newDestStop;
    //     if (newDestStop != nullptr) {
    //         myArrivalPos = (newDestStop->getBeginLanePosition() + newDestStop->getEndLanePosition()) / 2;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_destination(&mut self, newDestination: Ptr<MSEdge>, newDestStop: Ptr<MSStoppingPlace>) {
        self.myDestination = newDestination;
        self.myDestinationStop = newDestStop;
        if !newDestStop.is_null() {
            self.myArrivalPos = (newDestStop.get_begin_lane_position() + newDestStop.get_end_lane_position()) / 2.;
        }
    }

    /// C++: `void MSStage::setOrigin(const MSEdge *, MSStoppingPlace *, double)` [virtual]
    // virtual void setOrigin(const MSEdge* origin, MSStoppingPlace* originStop, double departPos) {
    //         UNUSED_PARAMETER(origin);
    //         UNUSED_PARAMETER(originStop);
    //         UNUSED_PARAMETER(departPos);
    //     }
    #[allow(unused_variables)]
    pub fn set_origin(&mut self, origin: Ptr<MSEdge>, originStop: Ptr<MSStoppingPlace>, departPos: f64) {
        match self.myType {
            MSStageType::DRIVING => self.as_concrete_mut::<MSStageDriving>().set_origin(origin, originStop, departPos),
            MSStageType::TRIP => self.as_concrete_mut::<MSStageTrip>().set_origin(origin, originStop, departPos),
            _ => {}
        }
    }

    /// C++: `double MSStage::getDistance() const` [const, virtual, pure virtual]
    /// get travel distance in this stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().get_distance(),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().get_distance(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().get_distance(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().get_distance(),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().get_distance(),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().get_distance(),
        }
    }

    /// C++: `void MSStage::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual, pure virtual]
    /// Called on writing tripinfo output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().trip_info_output(os, transportable),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().trip_info_output(os, transportable),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().trip_info_output(os, transportable),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().trip_info_output(os, transportable),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().trip_info_output(os, transportable),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().trip_info_output(os, transportable),
        }
    }

    /// C++: `void MSStage::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual, pure virtual]
    /// Called on writing vehroute output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().route_output(isPerson, os, withRouteLength, previous, withTiming, saveState),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().route_output(isPerson, os, withRouteLength, previous, withTiming, saveState),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().route_output(isPerson, os, withRouteLength, previous, withTiming, saveState),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().route_output(isPerson, os, withRouteLength, previous, withTiming, saveState),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().route_output(isPerson, os, withRouteLength, previous, withTiming, saveState),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().route_output(isPerson, os, withRouteLength, previous, withTiming, saveState),
        }
    }

    /// C++: `MSStage * MSStage::clone() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete::<MSStageWaiting>().clone(),
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().clone(),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().clone(),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().clone(),
            MSStageType::ACCESS => self.as_concrete::<MSPerson_MSPersonStage_Access>().clone(),
            MSStageType::TRIP => self.as_concrete::<MSStageTrip>().clone(),
        }
    }

    /// C++: `void MSStage::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    /// Saves the current state into the given stream, standard implementation does nothing
    // virtual void saveState(std::ostringstream& out, MSTransportable* transportable) {
    //         UNUSED_PARAMETER(out);
    //         UNUSED_PARAMETER(transportable);
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut String /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete_mut::<MSStageWaiting>().save_state(out, transportable),
            MSStageType::WALKING => self.as_concrete_mut::<MSStageWalking>().save_state(out, transportable),
            MSStageType::DRIVING => self.as_concrete_mut::<MSStageDriving>().save_state(out, transportable),
            MSStageType::ACCESS => self.as_concrete_mut::<MSPerson_MSPersonStage_Access>().save_state(out, transportable),
            _ => {}
        }
    }

    /// C++: `void MSStage::loadState(MSTransportable *, std::istringstream &)` [virtual]
    /// Reconstructs the current state, standard implementation does nothing
    // virtual void loadState(MSTransportable* transportable, std::istringstream& state) {
    //         UNUSED_PARAMETER(transportable);
    //         UNUSED_PARAMETER(state);
    //     }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut std::str::SplitWhitespace<'_> /* std::istringstream */) {
        match self.myType {
            MSStageType::WAITING_FOR_DEPART | MSStageType::WAITING => self.as_concrete_mut::<MSStageWaiting>().load_state(transportable, state),
            MSStageType::WALKING => self.as_concrete_mut::<MSStageWalking>().load_state(transportable, state),
            MSStageType::DRIVING => self.as_concrete_mut::<MSStageDriving>().load_state(transportable, state),
            MSStageType::ACCESS => self.as_concrete_mut::<MSPerson_MSPersonStage_Access>().load_state(transportable, state),
            _ => {}
        }
    }

    /// C++: `bool MSStage::wasSet(int) const` [const]
    // bool wasSet(int what) const {
    //         return (myParametersSet & what) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_set(&self, what: i32) -> bool {
        (self.myParametersSet & what) != 0
    }

    /// C++: `void MSStage::markSet(int)`
    // void markSet(int what) {
    //         myParametersSet |= what;
    //     }
    #[allow(unused_variables)]
    pub fn mark_set(&mut self, what: i32) {
        self.myParametersSet |= what;
    }

    /// C++: `double MSStage::getCosts() const` [const]
    /// Returns the costs of the stage
    // double getCosts() const {
    //         return myCosts;
    //     }
    #[allow(unused_variables)]
    pub fn get_costs(&self) -> f64 {
        self.myCosts
    }

    /// C++: `void MSStage::setCosts(double)`
    /// Sets the costs of the stage
    // void setCosts(double costs) {
    //         myCosts = costs;
    //     }
    #[allow(unused_variables)]
    pub fn set_costs(&mut self, costs: f64) {
        self.myCosts = costs;
    }

    /// C++: `MSStageTrip * MSStage::getTrip() const` [const]
    // MSStageTrip* getTrip() const {
    //         return myTrip;
    //     }
    #[allow(unused_variables)]
    pub fn get_trip(&self) -> Ptr<MSStageTrip> {
        self.myTrip
    }

    /// C++: `void MSStage::setTrip(MSStageTrip *)`
    // void setTrip(MSStageTrip* trip) {
    //         myTrip = trip;
    //     }
    #[allow(unused_variables)]
    pub fn set_trip(&mut self, trip: Ptr<MSStageTrip>) {
        self.myTrip = trip;
    }

    /// C++: `bool MSStage::equals(const MSStage &) const` [const, virtual]
    // virtual bool equals(const MSStage& s) const {
    //         return myDestination == s.myDestination &&
    //                myDestinationStop == s.myDestinationStop &&
    //                myArrivalPos == s.myArrivalPos &&
    //                myArrivalPosLat == s.myArrivalPosLat &&
    //                myType == s.myType &&
    //                myGroup == s.myGroup;
    //     }
    #[allow(unused_variables)]
    pub fn equals(&self, s: &MSStage) -> bool {
        match self.myType {
            MSStageType::WALKING => self.as_concrete::<MSStageWalking>().equals(s),
            MSStageType::TRANSHIP => self.as_concrete::<MSStageTranship>().equals(s),
            MSStageType::DRIVING => self.as_concrete::<MSStageDriving>().equals(s),
            _ => self.equals_base(s),
        }
    }

    /// The C++ body of `MSStage::equals` without virtual dispatch (for subclasses calling `MSStage::equals` explicitly).
    pub fn equals_base(&self, s: &MSStage) -> bool {
        self.myDestination == s.myDestination
            && self.myDestinationStop == s.myDestinationStop
            && self.myArrivalPos == s.myArrivalPos
            && self.myArrivalPosLat == s.myArrivalPosLat
            && self.myType == s.myType
            && self.myGroup == s.myGroup
    }

    /// C++: `void MSStage::MSStage(const MSStage &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStage) -> Self {
        panic!("MSStage copy constructor is invalidated in C++ (private)");
    }

    /// C++: `MSStage & MSStage::operator=(const MSStage &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStage) -> &mut MSStage {
        panic!("MSStage assignment operator is invalidated in C++ (private)");
    }

}

