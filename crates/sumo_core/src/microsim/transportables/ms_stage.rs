//! Port of microsim/transportables/MSStage.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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

impl MSStage {
    /// C++: `void MSStage::MSStage(const MSStageType, const MSEdge *, MSStoppingPlace *, const double, const double, const std::string &)`
    /// constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(type_: MSStageType, destination: Ptr<MSEdge>, toStop: Ptr<MSStoppingPlace>, arrivalPos: f64, arrivalPosLat: f64, group: &str) -> Self {
        todo!("port MSStage::MSStage")
    }

    /// C++: `void MSStage::~MSStage() noexcept` [virtual]
    /// destructor
    // MSStage::~MSStage() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSStage::~MSStage")
    }

    /// C++: `void MSStage::init(MSTransportable *)` [virtual]
    /// initialization, e.g. for param-related events
    // virtual void init(MSTransportable* /*transportable*/) {}
    #[allow(unused_variables)]
    pub fn init(&mut self, arg0: Ptr<MSTransportable>) {
        todo!("port MSStage::init")
    }

    /// C++: `const MSEdge * MSStage::getDestination() const` [const]
    /// returns the destination edge
    // const MSEdge*
    // MSStage::getDestination() const {
    //     return myDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<MSEdge> {
        todo!("port MSStage::getDestination")
    }

    /// C++: `MSStoppingPlace * MSStage::getDestinationStop() const` [const]
    /// returns the destination stop (if any)
    // MSStoppingPlace* getDestinationStop() const {
    //         return myDestinationStop;
    //     }
    #[allow(unused_variables)]
    pub fn get_destination_stop(&self) -> Ptr<MSStoppingPlace> {
        todo!("port MSStage::getDestinationStop")
    }

    /// C++: `MSStoppingPlace * MSStage::getOriginStop() const` [const, virtual]
    /// returns the origin stop (if any). only needed for MSStageTrip
    // virtual MSStoppingPlace* getOriginStop() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_origin_stop(&self) -> Ptr<MSStoppingPlace> {
        todo!("port MSStage::getOriginStop")
    }

    /// C++: `double MSStage::getArrivalPos() const` [const, virtual]
    // virtual double getArrivalPos() const {
    //         return myArrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        todo!("port MSStage::getArrivalPos")
    }

    /// C++: `bool MSStage::unspecifiedArrivalPos() const` [const]
    // bool
    // MSStage::unspecifiedArrivalPos() const {
    //     return myArrivalPos == ARRIVALPOS_UNSPECIFIED;
    // }
    #[allow(unused_variables)]
    pub fn unspecified_arrival_pos(&self) -> bool {
        todo!("port MSStage::unspecifiedArrivalPos")
    }

    /// C++: `double MSStage::getArrivalPosLat() const` [const, virtual]
    // virtual double getArrivalPosLat() const {
    //         return myArrivalPosLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos_lat(&self) -> f64 {
        todo!("port MSStage::getArrivalPosLat")
    }

    /// C++: `void MSStage::setArrivalPos(double)`
    // void setArrivalPos(double arrivalPos) {
    //         myArrivalPos = arrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn set_arrival_pos(&mut self, arrivalPos: f64) {
        todo!("port MSStage::setArrivalPos")
    }

    /// C++: `const MSEdge * MSStage::getEdge() const` [const, virtual]
    /// Returns the current edge
    // const MSEdge*
    // MSStage::getEdge() const {
    //     return myDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSStage::getEdge")
    }

    /// C++: `const MSEdge * MSStage::getFromEdge() const` [const, virtual]
    // const MSEdge*
    // MSStage::getFromEdge() const {
    //     return myDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSStage::getFromEdge")
    }

    /// C++: `double MSStage::getEdgePos(SUMOTime) const` [const, virtual]
    // double
    // MSStage::getEdgePos(SUMOTime /* now */) const {
    //     return myArrivalPos;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        todo!("port MSStage::getEdgePos")
    }

    /// C++: `double MSStage::getEdgePosLat(SUMOTime) const` [const, virtual]
    // double
    // MSStage::getEdgePosLat(SUMOTime /* now */) const {
    //     return myArrivalPosLat;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos_lat(&self, now: SUMOTime) -> f64 {
        todo!("port MSStage::getEdgePosLat")
    }

    /// C++: `int MSStage::getDirection() const` [const, virtual]
    /// Return the movement directon on the edge
    // int
    // MSStage::getDirection() const {
    //     return MSPModel::UNDEFINED_DIRECTION;
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        todo!("port MSStage::getDirection")
    }

    /// C++: `Position MSStage::getPosition(SUMOTime) const` [const, virtual, pure virtual]
    /// returns the position of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        todo!("port MSStage::getPosition")
    }

    /// C++: `double MSStage::getAngle(SUMOTime) const` [const, virtual, pure virtual]
    /// returns the angle of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        todo!("port MSStage::getAngle")
    }

    /// C++: `const MSLane * MSStage::getLane() const` [const, virtual]
    /// Returns the current lane (if applicable)
    // virtual const MSLane* getLane() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        todo!("port MSStage::getLane")
    }

    /// C++: `SUMOTime MSStage::getJumpDuration() const` [const, virtual]
    /// Return the current jump duration (if applicable)
    // virtual SUMOTime getJumpDuration() const {
    //         return -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_jump_duration(&self) -> SUMOTime {
        todo!("port MSStage::getJumpDuration")
    }

    /// C++: `MSStageType MSStage::getStageType() const` [const]
    // MSStageType getStageType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_type(&self) -> MSStageType {
        todo!("port MSStage::getStageType")
    }

    /// C++: `const std::string & MSStage::getGroup() const` [const]
    /// return the id of the group of transportables traveling together
    // const std::string& getGroup() const {
    //         return myGroup;
    //     }
    #[allow(unused_variables)]
    pub fn get_group(&self) -> &str {
        todo!("port MSStage::getGroup")
    }

    /// C++: `std::string MSStage::getStageDescription(const bool) const` [const, virtual, pure virtual]
    /// return (brief) string representation of the current stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        todo!("port MSStage::getStageDescription")
    }

    /// C++: `std::string MSStage::getStageSummary(const bool) const` [const, virtual, pure virtual]
    /// return string summary of the current stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        todo!("port MSStage::getStageSummary")
    }

    /// C++: `bool MSStage::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual, pure virtual]
    /// proceeds to this stage and return True if everything is fine. May return false if the person is unable to proceed in this stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        todo!("port MSStage::proceed")
    }

    /// C++: `void MSStage::abort(MSTransportable *)` [virtual]
    /// abort this stage (TraCI)
    // virtual void abort(MSTransportable*) {}
    #[allow(unused_variables)]
    pub fn abort(&mut self, arg0: Ptr<MSTransportable>) {
        todo!("port MSStage::abort")
    }

    /// C++: `void MSStage::setSpeed(double)` [virtual]
    /// sets the walking speed (ignored in other stages)
    // virtual void setSpeed(double) {}
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, arg0: f64) {
        todo!("port MSStage::setSpeed")
    }

    /// C++: `SUMOTime MSStage::getDeparted() const` [const]
    /// get departure time of stage
    // SUMOTime
    // MSStage::getDeparted() const {
    //     return myDeparted;
    // }
    #[allow(unused_variables)]
    pub fn get_departed(&self) -> SUMOTime {
        todo!("port MSStage::getDeparted")
    }

    /// C++: `SUMOTime MSStage::getArrived() const` [const]
    /// get arrival time of stage
    // SUMOTime
    // MSStage::getArrived() const {
    //     return myArrived;
    // }
    #[allow(unused_variables)]
    pub fn get_arrived(&self) -> SUMOTime {
        todo!("port MSStage::getArrived")
    }

    /// C++: `SUMOTime MSStage::getTimeLoss(const MSTransportable *) const` [const, virtual]
    // SUMOTime
    // MSStage::getTimeLoss(const MSTransportable* /*transportable*/) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_time_loss(&self, transportable: Ptr<MSTransportable>) -> SUMOTime {
        todo!("port MSStage::getTimeLoss")
    }

    /// C++: `SUMOTime MSStage::getDuration() const` [const, virtual]
    // SUMOTime
    // MSStage::getDuration() const {
    //     return myArrived >= 0 ? myArrived - myDeparted : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        todo!("port MSStage::getDuration")
    }

    /// C++: `SUMOTime MSStage::getTravelTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getTravelTime() const {
    //     return getDuration();
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self) -> SUMOTime {
        todo!("port MSStage::getTravelTime")
    }

    /// C++: `SUMOTime MSStage::getWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getWaitingTime() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        todo!("port MSStage::getWaitingTime")
    }

    /// C++: `SUMOTime MSStage::getTotalWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getTotalWaitingTime() const {
    //     return getWaitingTime();
    // }
    #[allow(unused_variables)]
    pub fn get_total_waiting_time(&self) -> SUMOTime {
        todo!("port MSStage::getTotalWaitingTime")
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
        todo!("port MSStage::setDeparted")
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
        todo!("port MSStage::setArrived")
    }

    /// C++: `void MSStage::setEnded(SUMOTime)`
    /// sets end of stage
    // void
    // MSStage::setEnded(SUMOTime t) {
    //     myArrived = t;
    // }
    #[allow(unused_variables)]
    pub fn set_ended(&mut self, t: SUMOTime) {
        todo!("port MSStage::setEnded")
    }

    /// C++: `bool MSStage::isWaitingFor(const SUMOVehicle *) const` [const, virtual]
    /// Whether the transportable waits for the given vehicle
    // bool
    // MSStage::isWaitingFor(const SUMOVehicle* /*vehicle*/) const {
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_waiting_for(&self, vehicle: Ptr<SUMOVehicle>) -> bool {
        todo!("port MSStage::isWaitingFor")
    }

    /// C++: `bool MSStage::isWaiting4Vehicle() const` [const, virtual]
    /// Whether the transportable waits for a vehicle
    // virtual bool isWaiting4Vehicle() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting4_vehicle(&self) -> bool {
        todo!("port MSStage::isWaiting4Vehicle")
    }

    /// C++: `bool MSStage::isWalk() const` [const, virtual]
    /// Whether the transportable is walking
    // virtual bool isWalk() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_walk(&self) -> bool {
        todo!("port MSStage::isWalk")
    }

    /// C++: `SUMOVehicle * MSStage::getVehicle() const` [const, virtual]
    /// Current vehicle in which the transportable is driving (or nullptr)
    // virtual SUMOVehicle* getVehicle() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<SUMOVehicle> {
        todo!("port MSStage::getVehicle")
    }

    /// C++: `double MSStage::getSpeed() const` [const, virtual]
    /// the speed of the transportable
    // double
    // MSStage::getSpeed() const {
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        todo!("port MSStage::getSpeed")
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
        todo!("port MSStage::getEdges")
    }

    /// C++: `int MSStage::getRoutePosition() const` [const, virtual]
    /// return index of current edge within route
    // virtual int getRoutePosition() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        todo!("port MSStage::getRoutePosition")
    }

    /// C++: `Position MSStage::getEdgePosition(const MSEdge *, double, double) const` [const]
    /// get position on edge e at length at with orthogonal offset
    // Position
    // MSStage::getEdgePosition(const MSEdge* e, double at, double offset) const {
    //     return getLanePosition(e->getLanes()[0], at, offset);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_position(&self, e: Ptr<MSEdge>, at: f64, offset: f64) -> Position {
        todo!("port MSStage::getEdgePosition")
    }

    /// C++: `Position MSStage::getLanePosition(const MSLane *, double, double) const` [const]
    /// get position on lane at length at with orthogonal offset
    // Position
    // MSStage::getLanePosition(const MSLane* lane, double at, double offset) const {
    //     return lane->getShape().positionAtOffset(lane->interpolateLanePosToGeometryPos(at), offset);
    // }
    #[allow(unused_variables)]
    pub fn get_lane_position(&self, lane: Ptr<MSLane>, at: f64, offset: f64) -> Position {
        todo!("port MSStage::getLanePosition")
    }

    /// C++: `double MSStage::getEdgeAngle(const MSEdge *, double) const` [const]
    /// get angle of the edge at a certain position
    // double
    // MSStage::getEdgeAngle(const MSEdge* e, double at) const {
    //     return e->getLanes()[0]->getShape().rotationAtOffset(at);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_angle(&self, e: Ptr<MSEdge>, at: f64) -> f64 {
        todo!("port MSStage::getEdgeAngle")
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
        todo!("port MSStage::setDestination")
    }

    /// C++: `void MSStage::setOrigin(const MSEdge *, MSStoppingPlace *, double)` [virtual]
    // virtual void setOrigin(const MSEdge* origin, MSStoppingPlace* originStop, double departPos) {
    //         UNUSED_PARAMETER(origin);
    //         UNUSED_PARAMETER(originStop);
    //         UNUSED_PARAMETER(departPos);
    //     }
    #[allow(unused_variables)]
    pub fn set_origin(&mut self, origin: Ptr<MSEdge>, originStop: Ptr<MSStoppingPlace>, departPos: f64) {
        todo!("port MSStage::setOrigin")
    }

    /// C++: `double MSStage::getDistance() const` [const, virtual, pure virtual]
    /// get travel distance in this stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        todo!("port MSStage::getDistance")
    }

    /// C++: `void MSStage::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual, pure virtual]
    /// Called on writing tripinfo output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        todo!("port MSStage::tripInfoOutput")
    }

    /// C++: `void MSStage::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual, pure virtual]
    /// Called on writing vehroute output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        todo!("port MSStage::routeOutput")
    }

    /// C++: `MSStage * MSStage::clone() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        todo!("port MSStage::clone")
    }

    /// C++: `void MSStage::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    /// Saves the current state into the given stream, standard implementation does nothing
    // virtual void saveState(std::ostringstream& out, MSTransportable* transportable) {
    //         UNUSED_PARAMETER(out);
    //         UNUSED_PARAMETER(transportable);
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        todo!("port MSStage::saveState")
    }

    /// C++: `void MSStage::loadState(MSTransportable *, std::istringstream &)` [virtual]
    /// Reconstructs the current state, standard implementation does nothing
    // virtual void loadState(MSTransportable* transportable, std::istringstream& state) {
    //         UNUSED_PARAMETER(transportable);
    //         UNUSED_PARAMETER(state);
    //     }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut Unmapped /* std::istringstream */) {
        todo!("port MSStage::loadState")
    }

    /// C++: `bool MSStage::wasSet(int) const` [const]
    // bool wasSet(int what) const {
    //         return (myParametersSet & what) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_set(&self, what: i32) -> bool {
        todo!("port MSStage::wasSet")
    }

    /// C++: `void MSStage::markSet(int)`
    // void markSet(int what) {
    //         myParametersSet |= what;
    //     }
    #[allow(unused_variables)]
    pub fn mark_set(&mut self, what: i32) {
        todo!("port MSStage::markSet")
    }

    /// C++: `double MSStage::getCosts() const` [const]
    /// Returns the costs of the stage
    // double getCosts() const {
    //         return myCosts;
    //     }
    #[allow(unused_variables)]
    pub fn get_costs(&self) -> f64 {
        todo!("port MSStage::getCosts")
    }

    /// C++: `void MSStage::setCosts(double)`
    /// Sets the costs of the stage
    // void setCosts(double costs) {
    //         myCosts = costs;
    //     }
    #[allow(unused_variables)]
    pub fn set_costs(&mut self, costs: f64) {
        todo!("port MSStage::setCosts")
    }

    /// C++: `MSStageTrip * MSStage::getTrip() const` [const]
    // MSStageTrip* getTrip() const {
    //         return myTrip;
    //     }
    #[allow(unused_variables)]
    pub fn get_trip(&self) -> Ptr<MSStageTrip> {
        todo!("port MSStage::getTrip")
    }

    /// C++: `void MSStage::setTrip(MSStageTrip *)`
    // void setTrip(MSStageTrip* trip) {
    //         myTrip = trip;
    //     }
    #[allow(unused_variables)]
    pub fn set_trip(&mut self, trip: Ptr<MSStageTrip>) {
        todo!("port MSStage::setTrip")
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
        todo!("port MSStage::equals")
    }

    /// C++: `void MSStage::MSStage(const MSStage &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStage) -> Self {
        todo!("port MSStage::MSStage")
    }

    /// C++: `MSStage & MSStage::operator=(const MSStage &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStage) -> &mut MSStage {
        todo!("port MSStage::operator=")
    }

}

