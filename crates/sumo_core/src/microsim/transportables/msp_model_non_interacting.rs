//! Port of microsim/transportables/MSPModel_NonInteracting.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_event_control::Command;

const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_BACKWARD: i32 = -1;
const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;
// C++ MSPModel::SIDEWALK_OFFSET
const MSPMODEL_SIDEWALK_OFFSET: f64 = 3.0;

// C++ static members of CState (port as module-level statics, see AGENTS.md):
//   static const double CState::LATERAL_OFFSET;
const CSTATE_LATERAL_OFFSET: f64 = 0.0;

/// C++ getSidewalk<MSEdge, MSLane>(edge) with the default svc = SVC_PEDESTRIAN (utils/router/IntermodalEdge.h)
// shortcut: local copy of the pedestrian-only path, because the generic intermodal_edge.rs getSidewalk is still a stub, upgrade when it is ported
fn get_sidewalk_lane(edge: Ptr<MSEdge>) -> Ptr<MSLane> {
    if edge.is_null() {
        return Ptr::null();
    }
    let lanes = edge.get_lanes();
    // prefer lanes that are exclusive to pedestrians
    for lane in lanes.iter() {
        if lane.get_permissions() == SVC_PEDESTRIAN as SVCPermissions {
            return *lane;
        }
    }
    for lane in lanes.iter() {
        if lane.allows_vehicle_class(SVC_PEDESTRIAN) {
            return *lane;
        }
    }
    Ptr::null()
}

// Virtual dispatch for the PState/CState overrides: the adapter base is the first field of both repr(C) structs.
fn ni_pstate(a: &MSTransportableStateAdapter) -> &MSPModel_NonInteracting_PState {
    unsafe { &*(a as *const MSTransportableStateAdapter as *const MSPModel_NonInteracting_PState) }
}
fn ni_cstate(a: &MSTransportableStateAdapter) -> &MSPModel_NonInteracting_CState {
    unsafe { &*(a as *const MSTransportableStateAdapter as *const MSPModel_NonInteracting_CState) }
}
fn pstate_get_edge_pos(a: &MSTransportableStateAdapter, now: SUMOTime) -> f64 {
    ni_pstate(a).get_edge_pos(now)
}
fn pstate_get_direction(a: &MSTransportableStateAdapter) -> i32 {
    ni_pstate(a).get_direction()
}
fn pstate_get_position(a: &MSTransportableStateAdapter, stage: &MSStageMoving, now: SUMOTime) -> Position {
    ni_pstate(a).get_position(stage, now)
}
fn pstate_get_angle(a: &MSTransportableStateAdapter, stage: &MSStageMoving, now: SUMOTime) -> f64 {
    ni_pstate(a).get_angle(stage, now)
}
fn pstate_get_waiting_time(a: &MSTransportableStateAdapter) -> SUMOTime {
    ni_pstate(a).get_waiting_time()
}
fn pstate_get_speed(a: &MSTransportableStateAdapter, stage: &MSStageMoving) -> f64 {
    ni_pstate(a).get_speed(stage)
}
fn pstate_get_next_edge(a: &MSTransportableStateAdapter, stage: &MSStageMoving) -> Ptr<MSEdge> {
    ni_pstate(a).get_next_edge(stage)
}
fn cstate_get_position(a: &MSTransportableStateAdapter, stage: &MSStageMoving, now: SUMOTime) -> Position {
    ni_cstate(a).get_position(stage, now)
}
fn cstate_get_angle(a: &MSTransportableStateAdapter, stage: &MSStageMoving, now: SUMOTime) -> f64 {
    ni_cstate(a).get_angle(stage, now)
}

fn pstate_save_state(a: &mut MSTransportableStateAdapter, out: &mut String) {
    Ptr::from_mut(a).cast::<MSPModel_NonInteracting_PState>().m().save_state(out)
}

/// C++ PState overrides (getEdgePos, getDirection, getPosition, getAngle, getWaitingTime, getSpeed, getNextEdge, saveState)
static MSPMODEL_NONINTERACTING_PSTATE_VT: MSTransportableStateAdapterVt = MSTransportableStateAdapterVt {
    get_lat_offset: adapter_default_get_lat_offset,
    get_edge_pos: pstate_get_edge_pos,
    get_direction: pstate_get_direction,
    get_position: pstate_get_position,
    get_angle: pstate_get_angle,
    get_waiting_time: pstate_get_waiting_time,
    get_total_waiting_time: adapter_default_get_total_waiting_time,
    get_speed: pstate_get_speed,
    get_next_edge: pstate_get_next_edge,
    move_to: adapter_default_move_to,
    move_to_xy: adapter_default_move_to_xy,
    is_jammed: adapter_default_is_jammed,
    is_finished: adapter_default_is_finished,
    get_lane: adapter_default_get_lane,
    get_path_length: adapter_default_get_path_length,
    save_state: pstate_save_state,
    get_id: adapter_default_get_id,
    get_min_x: adapter_default_get_min_x,
    get_max_x: adapter_default_get_max_x,
    get_width: adapter_default_get_width,
    get_o_type: adapter_default_get_o_type,
};

/// C++ CState overrides (getPosition, getAngle) on top of the PState table
static MSPMODEL_NONINTERACTING_CSTATE_VT: MSTransportableStateAdapterVt = MSTransportableStateAdapterVt {
    get_lat_offset: adapter_default_get_lat_offset,
    get_edge_pos: pstate_get_edge_pos,
    get_direction: pstate_get_direction,
    get_position: cstate_get_position,
    get_angle: cstate_get_angle,
    get_waiting_time: pstate_get_waiting_time,
    get_total_waiting_time: adapter_default_get_total_waiting_time,
    get_speed: pstate_get_speed,
    get_next_edge: pstate_get_next_edge,
    move_to: adapter_default_move_to,
    move_to_xy: adapter_default_move_to_xy,
    is_jammed: adapter_default_is_jammed,
    is_finished: adapter_default_is_finished,
    get_lane: adapter_default_get_lane,
    get_path_length: adapter_default_get_path_length,
    save_state: pstate_save_state,
    get_id: adapter_default_get_id,
    get_min_x: adapter_default_get_min_x,
    get_max_x: adapter_default_get_max_x,
    get_width: adapter_default_get_width,
    get_o_type: adapter_default_get_o_type,
};

/// C++ class `MSPModel_NonInteracting::CState` (microsim/transportables/MSPModel_NonInteracting.h)
#[repr(C)]
pub struct MSPModel_NonInteracting_CState {
    /// C++ base class `PState`
    pub base: MSPModel_NonInteracting_PState,
    pub myCurrentBeginPosition: Position,
    pub myCurrentEndPosition: Position,
}

impl std::ops::Deref for MSPModel_NonInteracting_CState {
    type Target = MSPModel_NonInteracting_PState;
    fn deref(&self) -> &MSPModel_NonInteracting_PState {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_NonInteracting_CState {
    fn deref_mut(&mut self) -> &mut MSPModel_NonInteracting_PState {
        &mut self.base
    }
}

impl MSPModel_NonInteracting_CState {
    /// C++: `void CState::CState(MoveToNextEdge *, std::istringstream *)`
    // MSPModel_NonInteracting::CState::CState(MoveToNextEdge* cmd, std::istringstream* in) : PState(cmd, in) {
    // }
    #[allow(unused_variables)]
    pub fn new(cmd: Ptr<MSPModel_NonInteracting_MoveToNextEdge>, in_: Ptr<Unmapped /* std::istringstream */>) -> Self {
        let mut pstate = MSPModel_NonInteracting_PState::new(cmd, in_);
        pstate.base.vt = &MSPMODEL_NONINTERACTING_CSTATE_VT;
        Self {
            base: pstate,
            myCurrentBeginPosition: Position::default(),
            myCurrentEndPosition: Position::default(),
        }
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
        let dist = self.myCurrentBeginPosition.distance_to2d(&self.myCurrentEndPosition); //distance between begin and end position of this tranship stage
        let speedDist = steps2time(now - self.myLastEntryTime) * stage.get_max_speed(Ptr::null()); //the container shall not go beyond its end position
        let pos = if speedDist < dist { speedDist } else { dist };
        PositionVector::position_at_offset2d_seg(&self.myCurrentBeginPosition, &self.myCurrentEndPosition, pos, 0.0, false)
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
        let mut angle = stage.get_edge_angle(stage.get_edge(), self.get_edge_pos(now)) + (if self.myCurrentEndPos < self.myCurrentBeginPos { 1.5 * std::f64::consts::PI } else { 0.5 * std::f64::consts::PI });
        if angle > std::f64::consts::PI {
            angle -= 2.0 * std::f64::consts::PI;
        }
        angle
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
        self.myLastEntryTime = currentTime;

        self.myCurrentBeginPos = stage.get_depart_pos();
        self.myCurrentEndPos = stage.get_arrival_pos();

        let fromLane = stage.get_from_edge().get_lanes()[0]; //the lane the container starts from during its tranship stage
        self.myCurrentBeginPosition = stage.get_lane_position(fromLane, self.myCurrentBeginPos, CSTATE_LATERAL_OFFSET);
        let edges = stage.get_edges();
        let toLane = edges.last().unwrap().get_lanes()[0]; //the lane the container ends during its tranship stage
        self.myCurrentEndPosition = stage.get_lane_position(toLane, self.myCurrentEndPos, CSTATE_LATERAL_OFFSET);

        self.myCurrentDuration = std::cmp::max(1, time2steps((self.myCurrentEndPosition.distance_to(&self.myCurrentBeginPosition)).abs() / stage.get_max_speed(Ptr::null())));
        self.myCurrentDuration
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

// C++ virtual overrides of MSPModel, dispatched through MSPModel::vt
static MSPMODEL_NONINTERACTING_VT: MSPModelVt = MSPModelVt {
    add: noninteracting_vt_add,
    clear_state: noninteracting_vt_clear_state,
    remove: noninteracting_vt_remove,
    using_internal_lanes: noninteracting_vt_using_internal_lanes,
    get_active_number: noninteracting_vt_get_active_number,
    load_state: msp_model_default_load_state,
    has_pedestrians: msp_model_default_has_pedestrians,
    next_blocking: msp_model_default_next_blocking,
};
fn noninteracting_vt_add(m: &mut MSPModel, t: Ptr<MSTransportable>, s: Ptr<MSStageMoving>, now: SUMOTime) -> Ptr<MSTransportableStateAdapter> {
    Ptr::from_mut(m).cast::<MSPModel_NonInteracting>().m().add(t, s, now)
}
fn noninteracting_vt_clear_state(m: &mut MSPModel) {
    Ptr::from_mut(m).cast::<MSPModel_NonInteracting>().m().clear_state()
}
fn noninteracting_vt_remove(m: &mut MSPModel, state: Ptr<MSTransportableStateAdapter>) {
    Ptr::from_mut(m).cast::<MSPModel_NonInteracting>().m().remove(state)
}
fn noninteracting_vt_using_internal_lanes(m: &mut MSPModel) -> bool {
    Ptr::from_mut(m).cast::<MSPModel_NonInteracting>().m().using_internal_lanes()
}
fn noninteracting_vt_get_active_number(m: &mut MSPModel) -> i32 {
    Ptr::from_mut(m).cast::<MSPModel_NonInteracting>().m().get_active_number()
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
        Self {
            base: MSPModel { vt: &MSPMODEL_NONINTERACTING_VT },
            myNet: net,
            myNumActivePedestrians: 0,
        }
    }

    /// C++: `void MSPModel_NonInteracting::~MSPModel_NonInteracting() noexcept` [virtual]
    // MSPModel_NonInteracting::~MSPModel_NonInteracting() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
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
        self.myNumActivePedestrians += 1;
        // shortcut: MoveToNextEdge and the states are leaked, C++ hands ownership to the event queue, upgrade when the event control owns Commands
        let cmd: Ptr<MSPModel_NonInteracting_MoveToNextEdge> = Ptr::from_mut(Box::leak(Box::new(MSPModel_NonInteracting_MoveToNextEdge::new(transportable, stage.m(), Ptr::from_mut(self)))));
        let (state, duration) = if transportable.is_person() {
            let s: Ptr<MSPModel_NonInteracting_PState> = Ptr::from_mut(Box::leak(Box::new(MSPModel_NonInteracting_PState::new(cmd, Ptr::null()))));
            let d = s.m().compute_duration(Ptr::null(), &*stage, now);
            (Ptr::from_mut(&mut s.m().base), d)
        } else {
            let s: Ptr<MSPModel_NonInteracting_CState> = Ptr::from_mut(Box::leak(Box::new(MSPModel_NonInteracting_CState::new(cmd, Ptr::null()))));
            let d = s.m().compute_duration(Ptr::null(), &*stage, now);
            (Ptr::from_mut(&mut s.m().base.base), d)
        };
        self.myNet.m().get_begin_of_timestep_events().m().add_event(Ptr::from_mut(cmd.m() as &mut dyn Command), now + duration);
        state
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
        self.myNumActivePedestrians += 1;
        // shortcut: MoveToNextEdge and the states are leaked, C++ hands ownership to the event queue, upgrade when the event control owns Commands
        let cmd: Ptr<MSPModel_NonInteracting_MoveToNextEdge> = Ptr::from_mut(Box::leak(Box::new(MSPModel_NonInteracting_MoveToNextEdge::new(transportable, stage.m(), Ptr::from_mut(self)))));
        let (state, eventTime) = if transportable.is_person() {
            let s: Ptr<MSPModel_NonInteracting_PState> = Ptr::from_mut(Box::leak(Box::new(MSPModel_NonInteracting_PState::new(cmd, Ptr::from_mut(in_)))));
            (Ptr::from_mut(&mut s.m().base), s.get_event_time())
        } else {
            let s: Ptr<MSPModel_NonInteracting_CState> = Ptr::from_mut(Box::leak(Box::new(MSPModel_NonInteracting_CState::new(cmd, Ptr::from_mut(in_)))));
            (Ptr::from_mut(&mut s.m().base.base), s.get_event_time())
        };
        self.myNet.m().get_begin_of_timestep_events().m().add_event(Ptr::from_mut(cmd.m() as &mut dyn Command), eventTime);
        state
    }

    /// C++: `void MSPModel_NonInteracting::clearState()` [virtual]
    /// Resets pedestrians when quick-loading state
    // void
    // MSPModel_NonInteracting::clearState() {
    //     myNumActivePedestrians = 0;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myNumActivePedestrians = 0;
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
        self.myNumActivePedestrians -= 1;
        // static_cast-equivalent: every state handed out by this model is a PState (CState derives from it)
        state.cast::<MSPModel_NonInteracting_PState>().get_command().m().abort_walk();
    }

    /// C++: `bool MSPModel_NonInteracting::usingInternalLanes()` [virtual]
    /// whether movements on intersections are modelled
    // bool usingInternalLanes() {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn using_internal_lanes(&mut self) -> bool {
        false
    }

    /// C++: `int MSPModel_NonInteracting::getActiveNumber()` [virtual]
    /// return the number of active objects
    // int getActiveNumber() {
    //         return myNumActivePedestrians;
    //     }
    #[allow(unused_variables)]
    pub fn get_active_number(&mut self) -> i32 {
        self.myNumActivePedestrians
    }

    /// C++: `void MSPModel_NonInteracting::registerArrived()`
    // void registerArrived() {
    //         myNumActivePedestrians--;
    //     }
    #[allow(unused_variables)]
    pub fn register_arrived(&mut self) {
        self.myNumActivePedestrians -= 1;
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
        Self {
            base: (),
            myParent: Ptr::from_mut(walk),
            myTransportable: transportable,
            myModel: model,
        }
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
        if self.myTransportable.is_null() {
            return 0; // descheduled
        }
        // the state's concrete type (PState or CState) is fixed by the transportable kind, used for the virtual computeDuration below
        let is_person = self.myTransportable.is_person();
        let old = self.myParent.get_edge();
        let dir = self.myParent.get_p_state().cast::<MSPModel_NonInteracting_PState>().get_direction();
        // shortcut: MSStageMoving::move_to_next_edge is a panic stub (virtual dispatch to the walking/tranship stage is not ported), upgrade when stage subclasses dispatch
        let arrived = self.myParent.m().move_to_next_edge(self.myTransportable, currentTime, dir, Ptr::null(), false);
        if arrived {
            self.myModel.m().register_arrived();
            return 0;
        }
        let nextDuration = if is_person {
            self.myParent.get_p_state().cast::<MSPModel_NonInteracting_PState>().m().compute_duration(old, &*self.myParent, currentTime)
        } else {
            self.myParent.get_p_state().cast::<MSPModel_NonInteracting_CState>().m().compute_duration(old, &*self.myParent, currentTime)
        };
        // first compute position on the new edge
        self.myParent.m().activate_entry_reminders(self.myTransportable, false);
        if self.myTransportable.is_null() {
            return 0; // descheduled by rerouter on new edge
        }
        nextDuration
    }

    /// C++: `void MoveToNextEdge::abortWalk()`
    // void abortWalk() {
    //             myTransportable = nullptr;
    //         }
    #[allow(unused_variables)]
    pub fn abort_walk(&mut self) {
        self.myTransportable = Ptr::null();
    }

    /// C++: `const MSTransportable * MoveToNextEdge::getTransportable() const` [const]
    // const MSTransportable* getTransportable() const {
    //             return myTransportable;
    //         }
    #[allow(unused_variables)]
    pub fn get_transportable(&self) -> Ptr<MSTransportable> {
        self.myTransportable
    }

    /// C++: `MoveToNextEdge & MoveToNextEdge::operator=(const MoveToNextEdge &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPModel_NonInteracting_MoveToNextEdge) -> &mut MSPModel_NonInteracting_MoveToNextEdge {
        panic!("MoveToNextEdge cannot be assigned: the assignment operator is invalidated in C++")
    }

}

impl Command for MSPModel_NonInteracting_MoveToNextEdge {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSPModel_NonInteracting_MoveToNextEdge::execute(self, currentTime)
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
        // shortcut: `(*in) >> myLastEntryTime >> myCurrentDuration` is not ported (no istringstream type in the Rust port), upgrade when the loadState stream type is decided
        Self {
            base: MSTransportableStateAdapter { vt: &MSPMODEL_NONINTERACTING_PSTATE_VT },
            myLastEntryTime: 0,
            myCurrentDuration: 0,
            myCurrentBeginPos: 0.0,
            myCurrentEndPos: 0.0,
            myCommand: cmd,
        }
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
        self.myCurrentBeginPos + (self.myCurrentEndPos - self.myCurrentBeginPos) / (self.myCurrentDuration as f64) * ((now - self.myLastEntryTime) as f64)
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
        if self.myCurrentBeginPos == self.myCurrentEndPos {
            MSPMODEL_UNDEFINED_DIRECTION
        } else if self.myCurrentBeginPos < self.myCurrentEndPos {
            MSPMODEL_FORWARD
        } else {
            MSPMODEL_BACKWARD
        }
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
        let mut lane = get_sidewalk_lane(stage.get_edge());
        if lane.is_null() {
            lane = stage.get_edge().get_lanes()[0];
        }
        let lateral_offset = if lane.allows_vehicle_class(SVC_PEDESTRIAN) { 0.0 } else { MSPMODEL_SIDEWALK_OFFSET * (if MSGlobals::lefthand() { -1.0 } else { 1.0 }) };
        stage.get_lane_position(lane, self.get_edge_pos(now), lateral_offset)
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
        let mut angle = stage.get_edge_angle(stage.get_edge(), self.get_edge_pos(now)) + (if self.myCurrentEndPos < self.myCurrentBeginPos { std::f64::consts::PI } else { 0.0 });
        if angle > std::f64::consts::PI {
            angle -= 2.0 * std::f64::consts::PI;
        }
        angle
    }

    /// C++: `SUMOTime PState::getWaitingTime() const` [const, virtual]
    // SUMOTime getWaitingTime() const {
    //             return 0;
    //         }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        0
    }

    /// C++: `double PState::getSpeed(const MSStageMoving &) const` [const, virtual]
    // double
    // MSPModel_NonInteracting::PState::getSpeed(const MSStageMoving& stage) const {
    //     return stage.getMaxSpeed(myCommand->getTransportable());
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self, stage: &MSStageMoving) -> f64 {
        stage.get_max_speed(self.myCommand.get_transportable())
    }

    /// C++: `const MSEdge * PState::getNextEdge(const MSStageMoving &) const` [const, virtual]
    // const MSEdge*
    // MSPModel_NonInteracting::PState::getNextEdge(const MSStageMoving& stage) const {
    //     return stage.getNextRouteEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        stage.get_next_route_edge()
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
        self.myLastEntryTime = currentTime;
        let edge = stage.get_edge();
        let next = stage.get_next_route_edge();
        let mut dir = MSPMODEL_UNDEFINED_DIRECTION;
        if prev.is_null() {
            self.myCurrentBeginPos = stage.get_depart_pos();
        } else {
            // default to FORWARD if not connected
            dir = if edge.get_to_junction() == prev.get_to_junction() || edge.get_to_junction() == prev.get_from_junction() { MSPMODEL_BACKWARD } else { MSPMODEL_FORWARD };
            self.myCurrentBeginPos = if dir == MSPMODEL_FORWARD { 0.0 } else { edge.get_length() };
        }
        if next.is_null() {
            self.myCurrentEndPos = stage.get_arrival_pos();
        } else {
            if dir == MSPMODEL_UNDEFINED_DIRECTION {
                // default to FORWARD if not connected
                dir = if edge.get_from_junction() == next.get_from_junction() || edge.get_from_junction() == next.get_to_junction() { MSPMODEL_BACKWARD } else { MSPMODEL_FORWARD };
            }
            self.myCurrentEndPos = if dir == MSPMODEL_FORWARD { edge.get_length() } else { 0.0 };
        }
        // ensure that a result > 0 is returned even if the walk ends immediately
        // adding 0.5ms is done to ensure proper rounding
        self.myCurrentDuration = std::cmp::max(1, time2steps((self.myCurrentEndPos - self.myCurrentBeginPos).abs() / stage.get_max_speed(self.myCommand.get_transportable())));
        // round to the next timestep to avoid systematic higher walking speed
        if self.myCurrentDuration % delta_t() > 0 {
            self.myCurrentDuration += delta_t();
        }
        self.myCurrentDuration
    }

    /// C++: `MoveToNextEdge * PState::getCommand() const` [const]
    // MoveToNextEdge* getCommand() const {
    //             return myCommand;
    //         }
    #[allow(unused_variables)]
    pub fn get_command(&self) -> Ptr<MSPModel_NonInteracting_MoveToNextEdge> {
        self.myCommand
    }

    /// C++: `SUMOTime PState::getEventTime() const` [const]
    // SUMOTime getEventTime() const {
    //             return myLastEntryTime + myCurrentDuration;
    //         }
    #[allow(unused_variables)]
    pub fn get_event_time(&self) -> SUMOTime {
        self.myLastEntryTime + self.myCurrentDuration
    }

    /// C++: `void PState::saveState(std::ostringstream &)` [virtual]
    /// Saves the current state into the given stream
    // void
    // MSPModel_NonInteracting::PState::saveState(std::ostringstream& out) {
    //     out << " " << myLastEntryTime << " " << myCurrentDuration;
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut String) {
        out.push_str(&format!(" {} {}", self.myLastEntryTime, self.myCurrentDuration));
    }

}

