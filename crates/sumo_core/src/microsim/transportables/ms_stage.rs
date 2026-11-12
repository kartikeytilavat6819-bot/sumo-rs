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
pub const ARRIVALPOS_UNSPECIFIED: f64 = f64::INFINITY;
pub const ROADSIDE_OFFSET: f64 = 3.;

impl MSStage {
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
        // empty C++ default body
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
        Ptr::null()
    }

    /// C++: `double MSStage::getArrivalPos() const` [const, virtual]
    // virtual double getArrivalPos() const {
    //         return myArrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        self.myArrivalPos
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
        self.myDestination
    }

    /// C++: `const MSEdge * MSStage::getFromEdge() const` [const, virtual]
    // const MSEdge*
    // MSStage::getFromEdge() const {
    //     return myDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        self.myDestination
    }

    /// C++: `double MSStage::getEdgePos(SUMOTime) const` [const, virtual]
    // double
    // MSStage::getEdgePos(SUMOTime /* now */) const {
    //     return myArrivalPos;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        self.myArrivalPos
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
        // shortcut: MSPModel::UNDEFINED_DIRECTION (=0) not ported as a constant yet
        0
    }

    /// C++: `Position MSStage::getPosition(SUMOTime) const` [const, virtual, pure virtual]
    /// returns the position of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::getPosition is pure virtual; a subclass must override it");
    }

    /// C++: `double MSStage::getAngle(SUMOTime) const` [const, virtual, pure virtual]
    /// returns the angle of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::getAngle is pure virtual; a subclass must override it");
    }

    /// C++: `const MSLane * MSStage::getLane() const` [const, virtual]
    /// Returns the current lane (if applicable)
    // virtual const MSLane* getLane() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        Ptr::null()
    }

    /// C++: `SUMOTime MSStage::getJumpDuration() const` [const, virtual]
    /// Return the current jump duration (if applicable)
    // virtual SUMOTime getJumpDuration() const {
    //         return -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_jump_duration(&self) -> SUMOTime {
        -1
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
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::getStageDescription is pure virtual; a subclass must override it");
    }

    /// C++: `std::string MSStage::getStageSummary(const bool) const` [const, virtual, pure virtual]
    /// return string summary of the current stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::getStageSummary is pure virtual; a subclass must override it");
    }

    /// C++: `bool MSStage::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual, pure virtual]
    /// proceeds to this stage and return True if everything is fine. May return false if the person is unable to proceed in this stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::proceed is pure virtual; a subclass must override it");
    }

    /// C++: `void MSStage::abort(MSTransportable *)` [virtual]
    /// abort this stage (TraCI)
    // virtual void abort(MSTransportable*) {}
    #[allow(unused_variables)]
    pub fn abort(&mut self, arg0: Ptr<MSTransportable>) {
        // empty C++ default body
    }

    /// C++: `void MSStage::setSpeed(double)` [virtual]
    /// sets the walking speed (ignored in other stages)
    // virtual void setSpeed(double) {}
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, arg0: f64) {
        // empty C++ default body
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
        0
    }

    /// C++: `SUMOTime MSStage::getDuration() const` [const, virtual]
    // SUMOTime
    // MSStage::getDuration() const {
    //     return myArrived >= 0 ? myArrived - myDeparted : SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
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
        self.get_duration()
    }

    /// C++: `SUMOTime MSStage::getWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getWaitingTime() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        0
    }

    /// C++: `SUMOTime MSStage::getTotalWaitingTime() const` [const, virtual]
    // SUMOTime
    // MSStage::getTotalWaitingTime() const {
    //     return getWaitingTime();
    // }
    #[allow(unused_variables)]
    pub fn get_total_waiting_time(&self) -> SUMOTime {
        self.get_waiting_time()
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
        self.myArrived = now;
        String::new()
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
        false
    }

    /// C++: `bool MSStage::isWaiting4Vehicle() const` [const, virtual]
    /// Whether the transportable waits for a vehicle
    // virtual bool isWaiting4Vehicle() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting4_vehicle(&self) -> bool {
        false
    }

    /// C++: `bool MSStage::isWalk() const` [const, virtual]
    /// Whether the transportable is walking
    // virtual bool isWalk() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_walk(&self) -> bool {
        false
    }

    /// C++: `SUMOVehicle * MSStage::getVehicle() const` [const, virtual]
    /// Current vehicle in which the transportable is driving (or nullptr)
    // virtual SUMOVehicle* getVehicle() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<SUMOVehicle> {
        Ptr::null()
    }

    /// C++: `double MSStage::getSpeed() const` [const, virtual]
    /// the speed of the transportable
    // double
    // MSStage::getSpeed() const {
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        0.
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
        vec![self.get_destination()]
    }

    /// C++: `int MSStage::getRoutePosition() const` [const, virtual]
    /// return index of current edge within route
    // virtual int getRoutePosition() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        0
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
        // empty C++ default body
    }

    /// C++: `double MSStage::getDistance() const` [const, virtual, pure virtual]
    /// get travel distance in this stage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::getDistance is pure virtual; a subclass must override it");
    }

    /// C++: `void MSStage::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual, pure virtual]
    /// Called on writing tripinfo output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::tripInfoOutput is pure virtual; a subclass must override it");
    }

    /// C++: `void MSStage::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual, pure virtual]
    /// Called on writing vehroute output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::routeOutput is pure virtual; a subclass must override it");
    }

    /// C++: `MSStage * MSStage::clone() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        // shortcut: pure virtual in C++, needs vtable dispatch (PORTING.md), upgrade when vtables land
        panic!("MSStage::clone is pure virtual; a subclass must override it");
    }

    /// C++: `void MSStage::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    /// Saves the current state into the given stream, standard implementation does nothing
    // virtual void saveState(std::ostringstream& out, MSTransportable* transportable) {
    //         UNUSED_PARAMETER(out);
    //         UNUSED_PARAMETER(transportable);
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        // empty C++ default body
    }

    /// C++: `void MSStage::loadState(MSTransportable *, std::istringstream &)` [virtual]
    /// Reconstructs the current state, standard implementation does nothing
    // virtual void loadState(MSTransportable* transportable, std::istringstream& state) {
    //         UNUSED_PARAMETER(transportable);
    //         UNUSED_PARAMETER(state);
    //     }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, state: &mut Unmapped /* std::istringstream */) {
        // empty C++ default body
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

