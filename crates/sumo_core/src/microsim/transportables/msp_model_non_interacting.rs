//! Port of microsim/transportables/MSPModel_NonInteracting.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSPModel_NonInteracting::CState` (microsim/transportables/MSPModel_NonInteracting.h)
#[repr(C)]
pub struct MSPModel_NonInteracting_CState {
    /// C++ base class `PState`
    pub base: MSPModel_Striping_PState,
    pub myCurrentBeginPosition: Position,
    pub myCurrentEndPosition: Position,
}

impl std::ops::Deref for MSPModel_NonInteracting_CState {
    type Target = MSPModel_Striping_PState;
    fn deref(&self) -> &MSPModel_Striping_PState {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_NonInteracting_CState {
    fn deref_mut(&mut self) -> &mut MSPModel_Striping_PState {
        &mut self.base
    }
}

// C++ static members of CState (port as module-level statics, see AGENTS.md):
//   static const double CState::LATERAL_OFFSET;

impl MSPModel_NonInteracting_CState {
    /// C++: `void CState::CState(MoveToNextEdge *, std::istringstream *)`
    // MSPModel_NonInteracting::CState::CState(MoveToNextEdge* cmd, std::istringstream* in) : PState(cmd, in) {
    // }
    #[allow(unused_variables)]
    pub fn new(cmd: Ptr<MSPModel_NonInteracting_MoveToNextEdge>, in_: Ptr<Unmapped /* std::istringstream */>) -> Self {
        todo!("port CState::CState")
    }

    /// C++: `Position CState::getPosition(const MSStageMoving &, SUMOTime) const` [const, virtual]
    /// return the network coordinate of the container
    // Position
    // MSPModel_NonInteracting::CState::getPosition(const MSStageMoving& stage, SUMOTime now) const {
    //     const double dist = myCurrentBeginPosition.distanceTo2D(myCurrentEndPosition);    //distance between begin and end position of this tranship stage
    //     double pos = MIN2(STEPS2TIME(now - myLastEntryTime) * stage.getMaxSpeed(), dist);    //the container shall not go beyond its end position
    //     return PositionVector::positionAtOffset2D(myCurrentBeginPosition, myCurrentEndPosition, pos, 0);
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, stage: &MSStageMoving, now: SUMOTime) -> Position {
        todo!("port CState::getPosition")
    }

    /// C++: `double CState::getAngle(const MSStageMoving &, SUMOTime) const` [const, virtual]
    /// return the direction in which the container heading to
    // double
    // MSPModel_NonInteracting::CState::getAngle(const MSStageMoving& stage, SUMOTime now) const {
    //     double angle = stage.getEdgeAngle(stage.getEdge(), getEdgePos(now)) + (myCurrentEndPos < myCurrentBeginPos ? 1.5 * M_PI : 0.5 * M_PI);
    //     if (angle > M_PI) {
    //         angle -= 2 * M_PI;
    //     }
    //     return angle;
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, stage: &MSStageMoving, now: SUMOTime) -> f64 {
        todo!("port CState::getAngle")
    }

    /// C++: `SUMOTime CState::computeDuration(const MSEdge *, const MSStageMoving &, SUMOTime)` [virtual]
    /// compute tranship time on edge and update state members
    // SUMOTime
    // MSPModel_NonInteracting::CState::computeDuration(const MSEdge* /* prev */, const MSStageMoving& stage, SUMOTime currentTime) {
    //     myLastEntryTime = currentTime;
    //
    //     myCurrentBeginPos = stage.getDepartPos();
    //     myCurrentEndPos = stage.getArrivalPos();
    //
    //     const MSLane* fromLane = stage.getFromEdge()->getLanes().front(); //the lane the container starts from during its tranship stage
    //     myCurrentBeginPosition = stage.getLanePosition(fromLane, myCurrentBeginPos, LATERAL_OFFSET);
    //     const MSLane* toLane = stage.getEdges().back()->getLanes().front(); //the lane the container ends during its tranship stage
    //     myCurrentEndPosition = stage.getLanePosition(toLane, myCurrentEndPos, LATERAL_OFFSET);
    //
    //     myCurrentDuration = MAX2((SUMOTime)1, TIME2STEPS(fabs(myCurrentEndPosition.distanceTo(myCurrentBeginPosition)) / stage.getMaxSpeed()));
    //     return myCurrentDuration;
    // }
    #[allow(unused_variables)]
    pub fn compute_duration(&mut self, prev: Ptr<MSEdge>, stage: &MSStageMoving, currentTime: SUMOTime) -> SUMOTime {
        todo!("port CState::computeDuration")
    }

}

/// The pedestrian following model
/// C++ class `MSPModel_NonInteracting` (microsim/transportables/MSPModel_NonInteracting.h)
#[repr(C)]
pub struct MSPModel_NonInteracting {
    /// C++ base class `MSPModel`
    pub base: MSPModel,
    /// the net to which to issue moveToNextEdge commands
    pub myNet: Ptr<MSNet>,
    /// the total number of active pedestrians
    pub myNumActivePedestrians: i32,
}

impl std::ops::Deref for MSPModel_NonInteracting {
    type Target = MSPModel;
    fn deref(&self) -> &MSPModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_NonInteracting {
    fn deref_mut(&mut self) -> &mut MSPModel {
        &mut self.base
    }
}

impl MSPModel_NonInteracting {
    /// C++: `void MSPModel_NonInteracting::MSPModel_NonInteracting(const OptionsCont &, MSNet *)`
    /// Constructor (it should not be necessary to construct more than one instance)
    // MSPModel_NonInteracting::MSPModel_NonInteracting(const OptionsCont& oc, MSNet* net) :
    //     myNet(net),
    //     myNumActivePedestrians(0) {
    //     assert(myNet != 0);
    //     UNUSED_PARAMETER(oc);
    // }
    #[allow(unused_variables)]
    pub fn new(oc: &OptionsCont, net: Ptr<MSNet>) -> Self {
        todo!("port MSPModel_NonInteracting::MSPModel_NonInteracting")
    }

    /// C++: `void MSPModel_NonInteracting::~MSPModel_NonInteracting() noexcept` [virtual]
    // MSPModel_NonInteracting::~MSPModel_NonInteracting() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSPModel_NonInteracting::~MSPModel_NonInteracting")
    }

    /// C++: `MSTransportableStateAdapter * MSPModel_NonInteracting::add(MSTransportable *, MSStageMoving *, SUMOTime)` [virtual]
    /// register the given transportable
    // MSTransportableStateAdapter*
    // MSPModel_NonInteracting::add(MSTransportable* transportable, MSStageMoving* stage, SUMOTime now) {
    //     myNumActivePedestrians++;
    //     MoveToNextEdge* const cmd = new MoveToNextEdge(transportable, *stage, this);
    //     PState* const state = transportable->isPerson() ? new PState(cmd) : new CState(cmd);
    //     myNet->getBeginOfTimestepEvents()->addEvent(cmd, now + state->computeDuration(nullptr, *stage, now));
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, now: SUMOTime) -> Ptr<MSTransportableStateAdapter> {
        todo!("port MSPModel_NonInteracting::add")
    }

    /// C++: `MSTransportableStateAdapter * MSPModel_NonInteracting::loadState(MSTransportable *, MSStageMoving *, std::istringstream &)` [virtual]
    /// load the state of the given transportable
    // MSTransportableStateAdapter*
    // MSPModel_NonInteracting::loadState(MSTransportable* transportable, MSStageMoving* stage, std::istringstream& in) {
    //     myNumActivePedestrians++;
    //     MoveToNextEdge* const cmd = new MoveToNextEdge(transportable, *stage, this);
    //     PState* const state = transportable->isPerson() ? new PState(cmd, &in) : new CState(cmd, &in);
    //     myNet->getBeginOfTimestepEvents()->addEvent(cmd, state->getEventTime());
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, in_: &mut Unmapped /* std::istringstream */) -> Ptr<MSTransportableStateAdapter> {
        todo!("port MSPModel_NonInteracting::loadState")
    }

    /// C++: `void MSPModel_NonInteracting::clearState()` [virtual]
    /// Resets pedestrians when quick-loading state
    // void
    // MSPModel_NonInteracting::clearState() {
    //     myNumActivePedestrians = 0;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        todo!("port MSPModel_NonInteracting::clearState")
    }

    /// C++: `void MSPModel_NonInteracting::remove(MSTransportableStateAdapter *)` [virtual]
    /// remove the specified person from the pedestrian simulation
    // void
    // MSPModel_NonInteracting::remove(MSTransportableStateAdapter* state) {
    //     myNumActivePedestrians--;
    //     dynamic_cast<PState*>(state)->getCommand()->abortWalk();
    // }
    #[allow(unused_variables)]
    pub fn remove(&mut self, state: Ptr<MSTransportableStateAdapter>) {
        todo!("port MSPModel_NonInteracting::remove")
    }

    /// C++: `bool MSPModel_NonInteracting::usingInternalLanes()` [virtual]
    /// whether movements on intersections are modelled
    // bool usingInternalLanes() {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn using_internal_lanes(&mut self) -> bool {
        todo!("port MSPModel_NonInteracting::usingInternalLanes")
    }

    /// C++: `int MSPModel_NonInteracting::getActiveNumber()` [virtual]
    /// return the number of active objects
    // int getActiveNumber() {
    //         return myNumActivePedestrians;
    //     }
    #[allow(unused_variables)]
    pub fn get_active_number(&mut self) -> i32 {
        todo!("port MSPModel_NonInteracting::getActiveNumber")
    }

    /// C++: `void MSPModel_NonInteracting::registerArrived()`
    // void registerArrived() {
    //         myNumActivePedestrians--;
    //     }
    #[allow(unused_variables)]
    pub fn register_arrived(&mut self) {
        todo!("port MSPModel_NonInteracting::registerArrived")
    }

}

/// C++ class `MSPModel_NonInteracting::MoveToNextEdge` (microsim/transportables/MSPModel_NonInteracting.h)
#[repr(C)]
pub struct MSPModel_NonInteracting_MoveToNextEdge {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    pub myParent: Ptr<MSStageMoving>,
    pub myTransportable: Ptr<MSTransportable>,
    pub myModel: Ptr<MSPModel_NonInteracting>,
}

impl std::ops::Deref for MSPModel_NonInteracting_MoveToNextEdge {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_NonInteracting_MoveToNextEdge {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSPModel_NonInteracting_MoveToNextEdge {
    /// C++: `void MoveToNextEdge::MoveToNextEdge(MSTransportable *, MSStageMoving &, MSPModel_NonInteracting *)`
    // MoveToNextEdge(MSTransportable* transportable, MSStageMoving& walk, MSPModel_NonInteracting* model) :
    //             myParent(walk), myTransportable(transportable), myModel(model) {}
    #[allow(unused_variables)]
    pub fn new(transportable: Ptr<MSTransportable>, walk: &mut MSStageMoving, model: Ptr<MSPModel_NonInteracting>) -> Self {
        todo!("port MoveToNextEdge::MoveToNextEdge")
    }

    /// C++: `SUMOTime MoveToNextEdge::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSPModel_NonInteracting::MoveToNextEdge::execute(SUMOTime currentTime) {
    //     if (myTransportable == nullptr) {
    //         return 0; // descheduled
    //     }
    //     const MSEdge* old = myParent.getEdge();
    //     const bool arrived = myParent.moveToNextEdge(myTransportable, currentTime, myParent.getPState()->getDirection());
    //     if (arrived) {
    //         myModel->registerArrived();
    //         return 0;
    //     }
    //     SUMOTime nextDuration = static_cast<PState*>(myParent.getPState())->computeDuration(old, myParent, currentTime);
    //     // first compute position on the new edge
    //     myParent.activateEntryReminders(myTransportable);
    //     if (myTransportable == nullptr) {
    //         return 0; // descheduled by rerouter on new edge
    //     }
    //     return nextDuration;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MoveToNextEdge::execute")
    }

    /// C++: `void MoveToNextEdge::abortWalk()`
    // void abortWalk() {
    //             myTransportable = nullptr;
    //         }
    #[allow(unused_variables)]
    pub fn abort_walk(&mut self) {
        todo!("port MoveToNextEdge::abortWalk")
    }

    /// C++: `const MSTransportable * MoveToNextEdge::getTransportable() const` [const]
    // const MSTransportable* getTransportable() const {
    //             return myTransportable;
    //         }
    #[allow(unused_variables)]
    pub fn get_transportable(&self) -> Ptr<MSTransportable> {
        todo!("port MoveToNextEdge::getTransportable")
    }

    /// C++: `MoveToNextEdge & MoveToNextEdge::operator=(const MoveToNextEdge &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPModel_NonInteracting_MoveToNextEdge) -> &mut MSPModel_NonInteracting_MoveToNextEdge {
        todo!("port MoveToNextEdge::operator=")
    }

}

/// implementation of callbacks to retrieve various state information from the model
/// C++ class `MSPModel_NonInteracting::PState` (microsim/transportables/MSPModel_NonInteracting.h)
#[repr(C)]
pub struct MSPModel_NonInteracting_PState {
    /// C++ base class `MSTransportableStateAdapter`
    pub base: MSTransportableStateAdapter,
    pub myLastEntryTime: SUMOTime,
    pub myCurrentDuration: SUMOTime,
    pub myCurrentBeginPos: f64,
    pub myCurrentEndPos: f64,
    pub myCommand: Ptr<MSPModel_NonInteracting_MoveToNextEdge>,
}

impl std::ops::Deref for MSPModel_NonInteracting_PState {
    type Target = MSTransportableStateAdapter;
    fn deref(&self) -> &MSTransportableStateAdapter {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_NonInteracting_PState {
    fn deref_mut(&mut self) -> &mut MSTransportableStateAdapter {
        &mut self.base
    }
}

impl MSPModel_NonInteracting_PState {
    /// C++: `void PState::PState(MoveToNextEdge *, std::istringstream *)`
    // MSPModel_NonInteracting::PState::PState(MoveToNextEdge* cmd, std::istringstream* in) : myCommand(cmd) {
    //     if (in != nullptr) {
    //         (*in) >> myLastEntryTime >> myCurrentDuration;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(cmd: Ptr<MSPModel_NonInteracting_MoveToNextEdge>, in_: Ptr<Unmapped /* std::istringstream */>) -> Self {
        todo!("port PState::PState")
    }

    /// C++: `double PState::getEdgePos(SUMOTime) const` [const, virtual]
    /// return the offset from the start of the current edge measured in its natural direction
    // double
    // MSPModel_NonInteracting::PState::getEdgePos(SUMOTime now) const {
    //     //std::cout << SIMTIME << " lastEntryTime=" << myLastEntryTime << " pos=" << (myCurrentBeginPos + (myCurrentEndPos - myCurrentBeginPos) / myCurrentDuration * (now - myLastEntryTime)) << "\n";
    //     return myCurrentBeginPos + (myCurrentEndPos - myCurrentBeginPos) / (double)myCurrentDuration * (double)(now - myLastEntryTime);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        todo!("port PState::getEdgePos")
    }

    /// C++: `int PState::getDirection() const` [const, virtual]
    // int
    // MSPModel_NonInteracting::PState::getDirection() const {
    //     if (myCurrentBeginPos == myCurrentEndPos) {
    //         return UNDEFINED_DIRECTION;
    //     } else {
    //         return myCurrentBeginPos < myCurrentEndPos ? FORWARD : BACKWARD;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        todo!("port PState::getDirection")
    }

    /// C++: `Position PState::getPosition(const MSStageMoving &, SUMOTime) const` [const, virtual]
    // Position
    // MSPModel_NonInteracting::PState::getPosition(const MSStageMoving& stage, SUMOTime now) const {
    //     const MSLane* lane = getSidewalk<MSEdge, MSLane>(stage.getEdge());
    //     if (lane == nullptr) {
    //         //std::string error = "Pedestrian '" + myCommand->myPerson->getID() + "' could not find sidewalk on edge '" + state.getEdge()->getID() + "', time="
    //         //    + time2string(MSNet::getInstance()->getCurrentTimeStep()) + ".";
    //         //if (!OptionsCont::getOptions().getBool("ignore-route-errors")) {
    //         //    throw ProcessError(error);
    //         //}
    //         lane = stage.getEdge()->getLanes().front();
    //     }
    //     const double lateral_offset = (lane->allowsVehicleClass(SVC_PEDESTRIAN) ? 0 : SIDEWALK_OFFSET
    //                                    * (MSGlobals::gLefthand ? -1 : 1));
    //     return stage.getLanePosition(lane, getEdgePos(now), lateral_offset);
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, stage: &MSStageMoving, now: SUMOTime) -> Position {
        todo!("port PState::getPosition")
    }

    /// C++: `double PState::getAngle(const MSStageMoving &, SUMOTime) const` [const, virtual]
    // double
    // MSPModel_NonInteracting::PState::getAngle(const MSStageMoving& stage, SUMOTime now) const {
    //     //std::cout << SIMTIME << " rawAngle=" << stage.getEdgeAngle(stage.getEdge(), getEdgePos(stage, now)) << " angle=" << stage.getEdgeAngle(stage.getEdge(), getEdgePos(stage, now)) + (myCurrentEndPos < myCurrentBeginPos ? 180 : 0) << "\n";
    //     double angle = stage.getEdgeAngle(stage.getEdge(), getEdgePos(now)) + (myCurrentEndPos < myCurrentBeginPos ? M_PI : 0);
    //     if (angle > M_PI) {
    //         angle -= 2 * M_PI;
    //     }
    //     return angle;
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, stage: &MSStageMoving, now: SUMOTime) -> f64 {
        todo!("port PState::getAngle")
    }

    /// C++: `SUMOTime PState::getWaitingTime() const` [const, virtual]
    // SUMOTime getWaitingTime() const {
    //             return 0;
    //         }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        todo!("port PState::getWaitingTime")
    }

    /// C++: `double PState::getSpeed(const MSStageMoving &) const` [const, virtual]
    // double
    // MSPModel_NonInteracting::PState::getSpeed(const MSStageMoving& stage) const {
    //     return stage.getMaxSpeed(myCommand->getTransportable());
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self, stage: &MSStageMoving) -> f64 {
        todo!("port PState::getSpeed")
    }

    /// C++: `const MSEdge * PState::getNextEdge(const MSStageMoving &) const` [const, virtual]
    // const MSEdge*
    // MSPModel_NonInteracting::PState::getNextEdge(const MSStageMoving& stage) const {
    //     return stage.getNextRouteEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        todo!("port PState::getNextEdge")
    }

    /// C++: `SUMOTime PState::computeDuration(const MSEdge *, const MSStageMoving &, SUMOTime)` [virtual]
    /// compute walking time on edge and update state members
    // SUMOTime
    // MSPModel_NonInteracting::PState::computeDuration(const MSEdge* prev, const MSStageMoving& stage, SUMOTime currentTime) {
    //     myLastEntryTime = currentTime;
    //     const MSEdge* edge = stage.getEdge();
    //     const MSEdge* next = stage.getNextRouteEdge();
    //     int dir = UNDEFINED_DIRECTION;
    //     if (prev == nullptr) {
    //         myCurrentBeginPos = stage.getDepartPos();
    //     } else {
    //         // default to FORWARD if not connected
    //         dir = (edge->getToJunction() == prev->getToJunction() || edge->getToJunction() == prev->getFromJunction()) ? BACKWARD : FORWARD;
    //         myCurrentBeginPos = dir == FORWARD ? 0 : edge->getLength();
    //     }
    //     if (next == nullptr) {
    //         myCurrentEndPos = stage.getArrivalPos();
    //     } else {
    //         if (dir == UNDEFINED_DIRECTION) {
    //             // default to FORWARD if not connected
    //             dir = (edge->getFromJunction() == next->getFromJunction() || edge->getFromJunction() == next->getToJunction()) ? BACKWARD : FORWARD;
    //         }
    //         myCurrentEndPos = dir == FORWARD ? edge->getLength() : 0;
    //     }
    //     // ensure that a result > 0 is returned even if the walk ends immediately
    //     // adding 0.5ms is done to ensure proper rounding
    //     myCurrentDuration = MAX2((SUMOTime)1, TIME2STEPS(fabs(myCurrentEndPos - myCurrentBeginPos) / stage.getMaxSpeed(myCommand->getTransportable())));
    //     //std::cout << std::setprecision(8) << SIMTIME << " curBeg=" << myCurrentBeginPos << " curEnd=" << myCurrentEndPos << " speed=" << stage.getMaxSpeed(myCommand->getTransportable()) << " dur=" << myCurrentDuration << "\n";
    //     // round to the next timestep to avoid systematic higher walking speed
    //     if ((myCurrentDuration % DELTA_T) > 0) {
    //         myCurrentDuration += DELTA_T;
    //     }
    //     return myCurrentDuration;
    // }
    #[allow(unused_variables)]
    pub fn compute_duration(&mut self, prev: Ptr<MSEdge>, stage: &MSStageMoving, currentTime: SUMOTime) -> SUMOTime {
        todo!("port PState::computeDuration")
    }

    /// C++: `MoveToNextEdge * PState::getCommand() const` [const]
    // MoveToNextEdge* getCommand() const {
    //             return myCommand;
    //         }
    #[allow(unused_variables)]
    pub fn get_command(&self) -> Ptr<MSPModel_NonInteracting_MoveToNextEdge> {
        todo!("port PState::getCommand")
    }

    /// C++: `SUMOTime PState::getEventTime() const` [const]
    // SUMOTime getEventTime() const {
    //             return myLastEntryTime + myCurrentDuration;
    //         }
    #[allow(unused_variables)]
    pub fn get_event_time(&self) -> SUMOTime {
        todo!("port PState::getEventTime")
    }

    /// C++: `void PState::saveState(std::ostringstream &)` [virtual]
    /// Saves the current state into the given stream
    // void
    // MSPModel_NonInteracting::PState::saveState(std::ostringstream& out) {
    //     out << " " << myLastEntryTime << " " << myCurrentDuration;
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */) {
        todo!("port PState::saveState")
    }

}

