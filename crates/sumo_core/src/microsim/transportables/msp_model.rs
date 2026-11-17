//! Port of microsim/transportables/MSPModel.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<const MSPerson *, double> PersonDist`
pub type PersonDist = (Ptr<MSPerson>, f64);

const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_BACKWARD: i32 = -1;
pub const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;

/// The pedestrian (and also sometimes container) movement model
/// C++ class `MSPModel` (microsim/transportables/MSPModel.h)
pub struct MSPModel {
    /// Virtual dispatch table for the pure-virtual methods; each concrete model installs its own
    pub vt: &'static MSPModelVt,
}

/// Virtual dispatch table for the pure-virtual methods of MSPModel (same pattern as SUMOTrafficObjectVt)
pub struct MSPModelVt {
    pub add: fn(&mut MSPModel, Ptr<MSTransportable>, Ptr<MSStageMoving>, SUMOTime) -> Ptr<MSTransportableStateAdapter>,
    pub clear_state: fn(&mut MSPModel),
    pub remove: fn(&mut MSPModel, Ptr<MSTransportableStateAdapter>),
    pub using_internal_lanes: fn(&mut MSPModel) -> bool,
    pub get_active_number: fn(&mut MSPModel) -> i32,
    pub load_state: fn(&mut MSPModel, Ptr<MSTransportable>, Ptr<MSStageMoving>, &mut std::str::SplitWhitespace<'_>) -> Ptr<MSTransportableStateAdapter>,
    pub has_pedestrians: fn(&mut MSPModel, Ptr<MSLane>) -> bool,
    pub blocked_at_dist: fn(&mut MSPModel, Ptr<SUMOTrafficObject>, Ptr<MSLane>, f64, f64, f64, Ptr<Vec<Ptr<MSPerson>>>) -> bool,
    pub next_blocking: fn(&mut MSPModel, Ptr<MSLane>, f64, f64, f64, f64, bool) -> PersonDist,
}

/// C++ MSPModel::loadState base implementation (returns nullptr)
pub fn msp_model_default_load_state(_m: &mut MSPModel, _transportable: Ptr<MSTransportable>, _stage: Ptr<MSStageMoving>, _state: &mut std::str::SplitWhitespace<'_>) -> Ptr<MSTransportableStateAdapter> {
    Ptr::null()
}

/// C++ MSPModel::hasPedestrians base implementation (returns false)
pub fn msp_model_default_has_pedestrians(_m: &mut MSPModel, _lane: Ptr<MSLane>) -> bool {
    false
}

/// C++ MSPModel::blockedAtDist base implementation (returns false)
pub fn msp_model_default_blocked_at_dist(_m: &mut MSPModel, _ego: Ptr<SUMOTrafficObject>, _lane: Ptr<MSLane>, _vehCenter: f64, _vehWidth: f64, _oncomingGap: f64, _collectBlockers: Ptr<Vec<Ptr<MSPerson>>>) -> bool {
    false
}

/// C++ MSPModel::nextBlocking base implementation (returns PersonDist(nullptr, -1))
pub fn msp_model_default_next_blocking(_m: &mut MSPModel, _lane: Ptr<MSLane>, _minPos: f64, _minRight: f64, _maxLeft: f64, _stopTime: f64, _bidi: bool) -> PersonDist {
    (Ptr::null(), -1.0)
}

// C++ static members of MSPModel (port as module-level statics, see AGENTS.md):
//   static const int MSPModel::FORWARD;
//   static const int MSPModel::BACKWARD;
//   static const int MSPModel::UNDEFINED_DIRECTION;
//   static const double MSPModel::SAFETY_GAP;
//   static const double MSPModel::SIDEWALK_OFFSET;
//   static const double MSPModel::UNSPECIFIED_POS_LAT;
//   static const double MSPModel::RANDOM_POS_LAT;

impl MSPModel {
    /// C++: `void MSPModel::~MSPModel() noexcept` [virtual]
    // virtual ~MSPModel() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `MSTransportableStateAdapter * MSPModel::add(MSTransportable *, MSStageMoving *, SUMOTime)` [virtual, pure virtual]
    /// register the given person as a pedestrian
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, now: SUMOTime) -> Ptr<MSTransportableStateAdapter> {
        (self.vt.add)(self, transportable, stage, now)
    }

    /// C++: `MSTransportableStateAdapter * MSPModel::loadState(MSTransportable *, MSStageMoving *, std::istringstream &)` [virtual]
    /// load the state of the given transportable
    // virtual MSTransportableStateAdapter* loadState(MSTransportable* transportable, MSStageMoving* stage, std::istringstream& state) {
    //         UNUSED_PARAMETER(transportable);
    //         UNUSED_PARAMETER(stage);
    //         UNUSED_PARAMETER(state);
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, state: &mut std::str::SplitWhitespace<'_> /* std::istringstream */) -> Ptr<MSTransportableStateAdapter> {
        (self.vt.load_state)(self, transportable, stage, state)
    }

    /// C++: `void MSPModel::clearState()` [virtual, pure virtual]
    /// Resets pedestrians when quick-loading state
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        (self.vt.clear_state)(self)
    }

    /// C++: `void MSPModel::remove(MSTransportableStateAdapter *)` [virtual, pure virtual]
    /// remove the specified person from the pedestrian simulation
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remove(&mut self, state: Ptr<MSTransportableStateAdapter>) {
        (self.vt.remove)(self, state)
    }

    /// C++: `bool MSPModel::blockedAtDist(const SUMOTrafficObject *, const MSLane *, double, double, double, std::vector<const MSPerson *> *)` [virtual]
    /// whether a pedestrian is blocking the crossing of lane for the given vehicle bondaries
    // virtual bool blockedAtDist(const SUMOTrafficObject* ego, const MSLane* lane, double vehCenter, double vehWidth,
    //                                double oncomingGap, std::vector<const MSPerson*>* collectBlockers) {
    //         UNUSED_PARAMETER(ego);
    //         UNUSED_PARAMETER(lane);
    //         UNUSED_PARAMETER(vehCenter);
    //         UNUSED_PARAMETER(vehWidth);
    //         UNUSED_PARAMETER(oncomingGap);
    //         UNUSED_PARAMETER(collectBlockers);
    //         return false;
    //     }
    pub fn blocked_at_dist(&mut self, ego: Ptr<SUMOTrafficObject>, lane: Ptr<MSLane>, vehCenter: f64, vehWidth: f64, oncomingGap: f64, collectBlockers: Ptr<Vec<Ptr<MSPerson>>>) -> bool {
        (self.vt.blocked_at_dist)(self, ego, lane, vehCenter, vehWidth, oncomingGap, collectBlockers)
    }

    /// C++: `bool MSPModel::hasPedestrians(const MSLane *)` [virtual]
    /// whether the given lane has pedestrians on it
    // virtual bool hasPedestrians(const MSLane* lane) {
    //         UNUSED_PARAMETER(lane);
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn has_pedestrians(&mut self, lane: Ptr<MSLane>) -> bool {
        (self.vt.has_pedestrians)(self, lane)
    }

    /// C++: `PersonDist MSPModel::nextBlocking(const MSLane *, double, double, double, double, bool)` [virtual]
    /// returns the next pedestrian beyond minPos that is laterally between minRight and maxLeft or nullptr
    // virtual PersonDist nextBlocking(const MSLane* lane, double minPos, double minRight, double maxLeft, double stopTime = 0, bool bidi = false) {
    //         UNUSED_PARAMETER(lane);
    //         UNUSED_PARAMETER(minPos);
    //         UNUSED_PARAMETER(minRight);
    //         UNUSED_PARAMETER(maxLeft);
    //         UNUSED_PARAMETER(stopTime);
    //         UNUSED_PARAMETER(bidi);
    //         return PersonDist(nullptr, -1);
    //     }
    #[allow(unused_variables)]
    pub fn next_blocking(&mut self, lane: Ptr<MSLane>, minPos: f64, minRight: f64, maxLeft: f64, stopTime: f64, bidi: bool) -> PersonDist {
        (self.vt.next_blocking)(self, lane, minPos, minRight, maxLeft, stopTime, bidi)
    }

    /// C++: `bool MSPModel::usingInternalLanes()` [virtual, pure virtual]
    /// whether movements on intersections are modelled
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn using_internal_lanes(&mut self) -> bool {
        (self.vt.using_internal_lanes)(self)
    }

    /// C++: `bool MSPModel::usingShortcuts()` [virtual]
    /// whether travel times and distances can reliably be calculated from the network alone
    // virtual bool usingShortcuts() {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn using_shortcuts(&mut self) -> bool {
        false
    }

    /// C++: `bool MSPModel::usingDoors()` [virtual]
    /// whether precise lateral positioning when leaving trains is supported
    // virtual bool usingDoors() {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn using_doors(&mut self) -> bool {
        false
    }

    /// C++: `int MSPModel::getActiveNumber()` [virtual, pure virtual]
    /// return the number of active objects
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_active_number(&mut self) -> i32 {
        (self.vt.get_active_number)(self)
    }

    /// C++: `int MSPModel::canTraverse(int, const ConstMSEdgeVector &, int &)` [static]
    // int
    // MSPModel::canTraverse(int dir, const ConstMSEdgeVector& route, int& passedEdges) {
    //     const MSJunction* junction = nullptr;
    //     for (ConstMSEdgeVector::const_iterator it = route.begin(); it != route.end(); ++it) {
    //         const MSEdge* edge = *it;
    //         if (junction != nullptr) {
    //             //std::cout << " junction=" << junction->getID() << " edge=" << edge->getID() << "\n";
    //             if (junction == edge->getFromJunction()) {
    //                 dir = FORWARD;
    //             } else if (junction == edge->getToJunction()) {
    //                 dir = BACKWARD;
    //             } else {
    //                 return UNDEFINED_DIRECTION;
    //             }
    //         }
    //         junction = dir == FORWARD ? edge->getToJunction() : edge->getFromJunction();
    //         passedEdges++;
    //     }
    //     return dir;
    // }
    #[allow(unused_variables)]
    pub fn can_traverse(mut dir: i32, route: &ConstMSEdgeVector, passedEdges: &mut i32) -> i32 {
        let mut junction: Ptr<MSJunction> = Ptr::null();
        for it in 0..route.len() {
            let edge = route[it];
            if !junction.is_null() {
                //std::cout << " junction=" << junction->getID() << " edge=" << edge->getID() << "\n";
                if junction == edge.get_from_junction() {
                    dir = MSPMODEL_FORWARD;
                } else if junction == edge.get_to_junction() {
                    dir = MSPMODEL_BACKWARD;
                } else {
                    return MSPMODEL_UNDEFINED_DIRECTION;
                }
            }
            junction = if dir == MSPMODEL_FORWARD { edge.get_to_junction() } else { edge.get_from_junction() };
            *passedEdges += 1;
        }
        dir
    }

}

/// abstract base class for managing callbacks to retrieve various state information from the model
/// C++ class `MSTransportableStateAdapter` (microsim/transportables/MSPModel.h)
pub struct MSTransportableStateAdapter {
    /// Virtual dispatch table of the concrete state (C++ virtual methods); set by each state's constructor
    pub vt: &'static MSTransportableStateAdapterVt,
}

/// Virtual dispatch table for the virtual methods of MSTransportableStateAdapter (same pattern as MSPModelVt)
pub struct MSTransportableStateAdapterVt {
    pub get_edge_pos: fn(&MSTransportableStateAdapter, SUMOTime) -> f64,
    pub get_direction: fn(&MSTransportableStateAdapter) -> i32,
    pub get_position: fn(&MSTransportableStateAdapter, &MSStageMoving, SUMOTime) -> Position,
    pub get_angle: fn(&MSTransportableStateAdapter, &MSStageMoving, SUMOTime) -> f64,
    pub get_waiting_time: fn(&MSTransportableStateAdapter) -> SUMOTime,
    pub get_total_waiting_time: fn(&MSTransportableStateAdapter) -> SUMOTime,
    pub get_speed: fn(&MSTransportableStateAdapter, &MSStageMoving) -> f64,
    pub get_next_edge: fn(&MSTransportableStateAdapter, &MSStageMoving) -> Ptr<MSEdge>,
    pub move_to: fn(&mut MSTransportableStateAdapter, Ptr<MSPerson>, Ptr<MSLane>, f64, f64, SUMOTime),
    pub move_to_xy: fn(&mut MSTransportableStateAdapter, Ptr<MSPerson>, Position, Ptr<MSLane>, f64, f64, f64, i32, &ConstMSEdgeVector, SUMOTime),
    pub is_jammed: fn(&MSTransportableStateAdapter) -> bool,
    pub is_finished: fn(&MSTransportableStateAdapter) -> bool,
    pub get_lane: fn(&MSTransportableStateAdapter) -> Ptr<MSLane>,
    pub get_path_length: fn(&MSTransportableStateAdapter) -> f64,
    pub save_state: fn(&mut MSTransportableStateAdapter, &mut String),
    pub get_id: fn(&MSTransportableStateAdapter) -> &str,
    pub get_min_x: fn(&MSTransportableStateAdapter, bool) -> f64,
    pub get_max_x: fn(&MSTransportableStateAdapter, bool) -> f64,
    pub get_width: fn(&MSTransportableStateAdapter) -> f64,
    pub get_o_type: fn(&MSTransportableStateAdapter) -> super::msp_model_striping::MSPModel_Striping_ObstacleType,
    pub get_lat_offset: fn(&MSTransportableStateAdapter) -> f64,
}

/// C++ MSPModel_InteractingState::getLatOffset is only reachable on interacting states (the base adapter has none)
pub fn adapter_default_get_lat_offset(_: &MSTransportableStateAdapter) -> f64 {
    panic!("MSPModel_InteractingState::getLatOffset is only available on interacting states")
}

/// C++ MSTransportableStateAdapter base implementations (pure virtuals panic, as the concrete state must provide them)
pub fn adapter_default_get_edge_pos(_: &MSTransportableStateAdapter, _: SUMOTime) -> f64 {
    panic!("MSTransportableStateAdapter::getEdgePos is pure virtual; call it on a concrete state adapter")
}
pub fn adapter_default_get_direction(_: &MSTransportableStateAdapter) -> i32 {
    panic!("MSTransportableStateAdapter::getDirection is pure virtual; call it on a concrete state adapter")
}
pub fn adapter_default_get_position(_: &MSTransportableStateAdapter, _: &MSStageMoving, _: SUMOTime) -> Position {
    panic!("MSTransportableStateAdapter::getPosition is pure virtual; call it on a concrete state adapter")
}
pub fn adapter_default_get_angle(_: &MSTransportableStateAdapter, _: &MSStageMoving, _: SUMOTime) -> f64 {
    panic!("MSTransportableStateAdapter::getAngle is pure virtual; call it on a concrete state adapter")
}
pub fn adapter_default_get_waiting_time(_: &MSTransportableStateAdapter) -> SUMOTime {
    panic!("MSTransportableStateAdapter::getWaitingTime is pure virtual; call it on a concrete state adapter")
}
pub fn adapter_default_get_total_waiting_time(_: &MSTransportableStateAdapter) -> SUMOTime {
    0
}
pub fn adapter_default_get_speed(_: &MSTransportableStateAdapter, _: &MSStageMoving) -> f64 {
    panic!("MSTransportableStateAdapter::getSpeed is pure virtual; call it on a concrete state adapter")
}
pub fn adapter_default_get_next_edge(_: &MSTransportableStateAdapter, _: &MSStageMoving) -> Ptr<MSEdge> {
    panic!("MSTransportableStateAdapter::getNextEdge is pure virtual; call it on a concrete state adapter")
}
pub fn adapter_default_move_to(_: &mut MSTransportableStateAdapter, _: Ptr<MSPerson>, _: Ptr<MSLane>, _: f64, _: f64, _: SUMOTime) {
    utils::common::msg_handler::write_warning("moveTo is ignored by the current movement model");
}
pub fn adapter_default_move_to_xy(_: &mut MSTransportableStateAdapter, _: Ptr<MSPerson>, _: Position, _: Ptr<MSLane>, _: f64, _: f64, _: f64, _: i32, _: &ConstMSEdgeVector, _: SUMOTime) {
    utils::common::msg_handler::write_warning("moveToXY is ignored by the current movement model");
}
pub fn adapter_default_is_jammed(_: &MSTransportableStateAdapter) -> bool {
    false
}
pub fn adapter_default_is_finished(_: &MSTransportableStateAdapter) -> bool {
    true
}
pub fn adapter_default_get_lane(_: &MSTransportableStateAdapter) -> Ptr<MSLane> {
    Ptr::null()
}
pub fn adapter_default_get_path_length(_: &MSTransportableStateAdapter) -> f64 {
    0.0
}
pub fn adapter_default_save_state(_: &mut MSTransportableStateAdapter, _: &mut String) {
}
pub fn adapter_default_get_id(_: &MSTransportableStateAdapter) -> &str {
    panic!("getID is virtual on MSPModel_InteractingState only; call it on an interacting state")
}
pub fn adapter_default_get_min_x(_: &MSTransportableStateAdapter, _: bool) -> f64 {
    panic!("getMinX is virtual on MSPModel_Striping::PState only; call it on a striping state")
}
pub fn adapter_default_get_max_x(_: &MSTransportableStateAdapter, _: bool) -> f64 {
    panic!("getMaxX is virtual on MSPModel_Striping::PState only; call it on a striping state")
}
pub fn adapter_default_get_width(_: &MSTransportableStateAdapter) -> f64 {
    panic!("getWidth is virtual on MSPModel_Striping::PState only; call it on a striping state")
}
pub fn adapter_default_get_o_type(_: &MSTransportableStateAdapter) -> super::msp_model_striping::MSPModel_Striping_ObstacleType {
    panic!("getOType is virtual on MSPModel_Striping::PState only; call it on a striping state")
}

/// base vtable of MSTransportableStateAdapter, used by states that do not override the virtual methods
pub static MSTRANSPORTABLE_STATE_ADAPTER_VT: MSTransportableStateAdapterVt = MSTransportableStateAdapterVt {
    get_edge_pos: adapter_default_get_edge_pos,
    get_direction: adapter_default_get_direction,
    get_position: adapter_default_get_position,
    get_angle: adapter_default_get_angle,
    get_waiting_time: adapter_default_get_waiting_time,
    get_total_waiting_time: adapter_default_get_total_waiting_time,
    get_speed: adapter_default_get_speed,
    get_next_edge: adapter_default_get_next_edge,
    move_to: adapter_default_move_to,
    move_to_xy: adapter_default_move_to_xy,
    is_jammed: adapter_default_is_jammed,
    is_finished: adapter_default_is_finished,
    get_lane: adapter_default_get_lane,
    get_path_length: adapter_default_get_path_length,
    save_state: adapter_default_save_state,
    get_id: adapter_default_get_id,
    get_min_x: adapter_default_get_min_x,
    get_max_x: adapter_default_get_max_x,
    get_width: adapter_default_get_width,
    get_o_type: adapter_default_get_o_type,
    get_lat_offset: adapter_default_get_lat_offset,
};

impl MSTransportableStateAdapter {
    /// C++: `void MSTransportableStateAdapter::~MSTransportableStateAdapter() noexcept` [virtual]
    // virtual ~MSTransportableStateAdapter() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `double MSTransportableStateAdapter::getEdgePos(SUMOTime) const` [const, virtual, pure virtual]
    /// return the offset from the start of the current edge measured in its natural direction
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        (self.vt.get_edge_pos)(self, now)
    }

    /// C++: `int MSTransportableStateAdapter::getDirection() const` [const, virtual, pure virtual]
    /// return the walking direction (FORWARD, BACKWARD, UNDEFINED_DIRECTION)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        (self.vt.get_direction)(self)
    }

    /// C++: `Position MSTransportableStateAdapter::getPosition(const MSStageMoving &, SUMOTime) const` [const, virtual, pure virtual]
    /// return the network coordinate of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position(&self, stage: &MSStageMoving, now: SUMOTime) -> Position {
        (self.vt.get_position)(self, stage, now)
    }

    /// C++: `double MSTransportableStateAdapter::getAngle(const MSStageMoving &, SUMOTime) const` [const, virtual, pure virtual]
    /// return the direction in which the transportable faces in degrees
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self, stage: &MSStageMoving, now: SUMOTime) -> f64 {
        (self.vt.get_angle)(self, stage, now)
    }

    /// C++: `SUMOTime MSTransportableStateAdapter::getWaitingTime() const` [const, virtual, pure virtual]
    /// return the time the transportable spent standing consecutively
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        (self.vt.get_waiting_time)(self)
    }

    /// C++: `SUMOTime MSTransportableStateAdapter::getTotalWaitingTime() const` [const, virtual]
    // virtual SUMOTime getTotalWaitingTime() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_waiting_time(&self) -> SUMOTime {
        (self.vt.get_total_waiting_time)(self)
    }

    /// C++: `double MSTransportableStateAdapter::getSpeed(const MSStageMoving &) const` [const, virtual, pure virtual]
    /// return the current speed of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_speed(&self, stage: &MSStageMoving) -> f64 {
        (self.vt.get_speed)(self, stage)
    }

    /// C++: `const MSEdge * MSTransportableStateAdapter::getNextEdge(const MSStageMoving &) const` [const, virtual, pure virtual]
    /// return the list of internal edges if the transportable is on an intersection
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        (self.vt.get_next_edge)(self, stage)
    }

    /// C++: `void MSTransportableStateAdapter::moveTo(MSPerson *, MSLane *, double, double, SUMOTime)` [virtual]
    /// try to move transportable to the given position
    // virtual void moveTo(MSPerson* p, MSLane* lane, double lanePos, double lanePosLat, SUMOTime t) {
    //         UNUSED_PARAMETER(p);
    //         UNUSED_PARAMETER(lane);
    //         UNUSED_PARAMETER(lanePos);
    //         UNUSED_PARAMETER(lanePosLat);
    //         UNUSED_PARAMETER(t);
    //         WRITE_WARNING(TL("moveTo is ignored by the current movement model"));
    //     }
    #[allow(unused_variables)]
    pub fn move_to(&mut self, p: Ptr<MSPerson>, lane: Ptr<MSLane>, lanePos: f64, lanePosLat: f64, t: SUMOTime) {
        (self.vt.move_to)(self, p, lane, lanePos, lanePosLat, t)
    }

    /// C++: `void MSTransportableStateAdapter::moveToXY(MSPerson *, Position, MSLane *, double, double, double, int, const ConstMSEdgeVector &, SUMOTime)` [virtual]
    /// try to move transportable to the given position
    // virtual void moveToXY(MSPerson* p, Position pos, MSLane* lane, double lanePos,
    //                           double lanePosLat, double angle, int routeOffset,
    //                           const ConstMSEdgeVector& edges, SUMOTime t) {
    //         UNUSED_PARAMETER(p);
    //         UNUSED_PARAMETER(pos);
    //         UNUSED_PARAMETER(lane);
    //         UNUSED_PARAMETER(lanePos);
    //         UNUSED_PARAMETER(lanePosLat);
    //         UNUSED_PARAMETER(angle);
    //         UNUSED_PARAMETER(routeOffset);
    //         UNUSED_PARAMETER(edges);
    //         UNUSED_PARAMETER(t);
    //         WRITE_WARNING(TL("moveToXY is ignored by the current movement model"));
    //     }
    #[allow(unused_variables)]
    pub fn move_to_xy(&mut self, p: Ptr<MSPerson>, pos: Position, lane: Ptr<MSLane>, lanePos: f64, lanePosLat: f64, angle: f64, routeOffset: i32, edges: &ConstMSEdgeVector, t: SUMOTime) {
        (self.vt.move_to_xy)(self, p, pos, lane, lanePos, lanePosLat, angle, routeOffset, edges, t)
    }

    /// C++: `bool MSTransportableStateAdapter::isJammed() const` [const, virtual]
    /// whether the transportable is jammed
    // virtual bool isJammed() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_jammed(&self) -> bool {
        (self.vt.is_jammed)(self)
    }

    /// C++: `bool MSTransportableStateAdapter::isFinished() const` [const, virtual]
    /// whether the transportable has finished walking
    // virtual bool isFinished() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_finished(&self) -> bool {
        (self.vt.is_finished)(self)
    }

    /// C++: `const MSLane * MSTransportableStateAdapter::getLane() const` [const, virtual]
    /// the current lane of the transportable
    // virtual const MSLane* getLane() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        (self.vt.get_lane)(self)
    }

    /// C++: `double MSTransportableStateAdapter::getPathLength() const` [const, virtual]
    /// return the total length of the current lane (in particular for on a walkingarea)
    // virtual double getPathLength() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_path_length(&self) -> f64 {
        (self.vt.get_path_length)(self)
    }

    /// C++: `void MSTransportableStateAdapter::saveState(std::ostringstream &)` [virtual]
    /// Saves the current state into the given stream
    // virtual void saveState(std::ostringstream& out) {
    //         UNUSED_PARAMETER(out);
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut String /* std::ostringstream */) {
        (self.vt.save_state)(self, out)
    }

}

