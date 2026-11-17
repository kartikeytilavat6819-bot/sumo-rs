//! Port of microsim/transportables/MSPModel.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<const MSPerson *, double> PersonDist`
pub type PersonDist = (Ptr<MSPerson>, f64);

const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_BACKWARD: i32 = -1;
const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;

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
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, state: &mut Unmapped /* std::istringstream */) -> Ptr<MSTransportableStateAdapter> {
        Ptr::null()
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
    #[allow(unused_variables)]
    pub fn blocked_at_dist(&mut self, ego: Ptr<SUMOTrafficObject>, lane: Ptr<MSLane>, vehCenter: f64, vehWidth: f64, oncomingGap: f64, collectBlockers: Ptr<Vec<Ptr<MSPerson>>>) -> bool {
        false
    }

    /// C++: `bool MSPModel::hasPedestrians(const MSLane *)` [virtual]
    /// whether the given lane has pedestrians on it
    // virtual bool hasPedestrians(const MSLane* lane) {
    //         UNUSED_PARAMETER(lane);
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn has_pedestrians(&mut self, lane: Ptr<MSLane>) -> bool {
        false
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
        (Ptr::null(), -1.0)
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
}

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
        // (pure virtual in C++: no body; subclasses implement it)
        panic!("MSTransportableStateAdapter::getEdgePos is pure virtual; call it on a concrete state adapter")
    }

    /// C++: `int MSTransportableStateAdapter::getDirection() const` [const, virtual, pure virtual]
    /// return the walking direction (FORWARD, BACKWARD, UNDEFINED_DIRECTION)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        // (pure virtual in C++: no body; subclasses implement it)
        panic!("MSTransportableStateAdapter::getDirection is pure virtual; call it on a concrete state adapter")
    }

    /// C++: `Position MSTransportableStateAdapter::getPosition(const MSStageMoving &, SUMOTime) const` [const, virtual, pure virtual]
    /// return the network coordinate of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position(&self, stage: &MSStageMoving, now: SUMOTime) -> Position {
        // (pure virtual in C++: no body; subclasses implement it)
        panic!("MSTransportableStateAdapter::getPosition is pure virtual; call it on a concrete state adapter")
    }

    /// C++: `double MSTransportableStateAdapter::getAngle(const MSStageMoving &, SUMOTime) const` [const, virtual, pure virtual]
    /// return the direction in which the transportable faces in degrees
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self, stage: &MSStageMoving, now: SUMOTime) -> f64 {
        // (pure virtual in C++: no body; subclasses implement it)
        panic!("MSTransportableStateAdapter::getAngle is pure virtual; call it on a concrete state adapter")
    }

    /// C++: `SUMOTime MSTransportableStateAdapter::getWaitingTime() const` [const, virtual, pure virtual]
    /// return the time the transportable spent standing consecutively
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        // (pure virtual in C++: no body; subclasses implement it)
        panic!("MSTransportableStateAdapter::getWaitingTime is pure virtual; call it on a concrete state adapter")
    }

    /// C++: `SUMOTime MSTransportableStateAdapter::getTotalWaitingTime() const` [const, virtual]
    // virtual SUMOTime getTotalWaitingTime() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_waiting_time(&self) -> SUMOTime {
        0
    }

    /// C++: `double MSTransportableStateAdapter::getSpeed(const MSStageMoving &) const` [const, virtual, pure virtual]
    /// return the current speed of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_speed(&self, stage: &MSStageMoving) -> f64 {
        // (pure virtual in C++: no body; subclasses implement it)
        panic!("MSTransportableStateAdapter::getSpeed is pure virtual; call it on a concrete state adapter")
    }

    /// C++: `const MSEdge * MSTransportableStateAdapter::getNextEdge(const MSStageMoving &) const` [const, virtual, pure virtual]
    /// return the list of internal edges if the transportable is on an intersection
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        // (pure virtual in C++: no body; subclasses implement it)
        panic!("MSTransportableStateAdapter::getNextEdge is pure virtual; call it on a concrete state adapter")
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
        utils::common::msg_handler::write_warning("moveTo is ignored by the current movement model");
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
        utils::common::msg_handler::write_warning("moveToXY is ignored by the current movement model");
    }

    /// C++: `bool MSTransportableStateAdapter::isJammed() const` [const, virtual]
    /// whether the transportable is jammed
    // virtual bool isJammed() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_jammed(&self) -> bool {
        false
    }

    /// C++: `bool MSTransportableStateAdapter::isFinished() const` [const, virtual]
    /// whether the transportable has finished walking
    // virtual bool isFinished() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_finished(&self) -> bool {
        true
    }

    /// C++: `const MSLane * MSTransportableStateAdapter::getLane() const` [const, virtual]
    /// the current lane of the transportable
    // virtual const MSLane* getLane() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        Ptr::null()
    }

    /// C++: `double MSTransportableStateAdapter::getPathLength() const` [const, virtual]
    /// return the total length of the current lane (in particular for on a walkingarea)
    // virtual double getPathLength() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_path_length(&self) -> f64 {
        0.0
    }

    /// C++: `void MSTransportableStateAdapter::saveState(std::ostringstream &)` [virtual]
    /// Saves the current state into the given stream
    // virtual void saveState(std::ostringstream& out) {
    //         UNUSED_PARAMETER(out);
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */) {
    }

}

