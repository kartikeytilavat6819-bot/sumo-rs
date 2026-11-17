//! Port of microsim/transportables/MSPModel_Interacting.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `ComparatorNumericalIdLess` key of ActiveLanes: lanes are ordered by numerical ID, not by address
// determinism: C++ ActiveLanes iterates in numerical-ID order; Ptr's own Ord is by address
#[derive(Clone, Copy)]
pub struct MSPModel_Interacting_LaneKey(pub Ptr<MSLane>);
impl PartialEq for MSPModel_Interacting_LaneKey {
    fn eq(&self, o: &Self) -> bool {
        self.0.get_numerical_id() == o.0.get_numerical_id()
    }
}
impl Eq for MSPModel_Interacting_LaneKey {}
impl PartialOrd for MSPModel_Interacting_LaneKey {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for MSPModel_Interacting_LaneKey {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        self.0.get_numerical_id().cmp(&o.0.get_numerical_id())
    }
}

/// C++ typedef `std::map<const MSLane *, Pedestrians, ComparatorNumericalIdLess> MSPModel_Interacting::ActiveLanes`
pub type MSPModel_Interacting_ActiveLanes = BTreeMap<MSPModel_Interacting_LaneKey, MSPModel_Interacting_Pedestrians>;

/// C++ typedef `std::vector<MSPModel_InteractingState *> MSPModel_Interacting::Pedestrians`
pub type MSPModel_Interacting_Pedestrians = Vec<Ptr<MSPModel_InteractingState>>;

const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;

thread_local! {
    /// C++ static `MSPModel_Interacting::noPedestrians` (returned by getPedestrians for unknown lanes)
    static NO_PEDESTRIANS: Ptr<MSPModel_Interacting_Pedestrians> = Ptr::from_mut(Box::leak(Box::new(Vec::new())));
}

/// The abstract superclass for pedestrian models which actually interact with vehicles
/// C++ class `MSPModel_Interacting` (microsim/transportables/MSPModel_Interacting.h)
#[repr(C)]
pub struct MSPModel_Interacting {
    /// C++ base class `MSPModel`
    pub base: MSPModel,
    /// the total number of active pedestrians
    pub myNumActivePedestrians: i32,
    /// store of all lanes which have pedestrians on them
    pub myActiveLanes: MSPModel_Interacting_ActiveLanes,
    /// whether an event for pedestrian processing was added
    pub myAmActive: bool,
    /// all crossings being approached by pedestrians
    pub myApproachedCrossings: BTreeSet<Ptr<MSLink>>,
}

impl std::ops::Deref for MSPModel_Interacting {
    type Target = MSPModel;
    fn deref(&self) -> &MSPModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_Interacting {
    fn deref_mut(&mut self) -> &mut MSPModel {
        &mut self.base
    }
}

// C++ static members of MSPModel_Interacting (port as module-level statics, see AGENTS.md):
//   static Pedestrians MSPModel_Interacting::noPedestrians;

impl MSPModel_Interacting {
    /// C++: `void MSPModel_Interacting::~MSPModel_Interacting() noexcept` [virtual]
    // MSPModel_Interacting::~MSPModel_Interacting() {
    //     clearState();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.clear_state();
    }

    /// C++: `void MSPModel_Interacting::clearState()` [virtual]
    /// Resets pedestrians when quick-loading state
    // void
    // MSPModel_Interacting::clearState() {
    //     myActiveLanes.clear();
    //     myNumActivePedestrians = 0;
    //     myAmActive = false;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myActiveLanes.clear();
        self.myNumActivePedestrians = 0;
        self.myAmActive = false;
    }

    /// C++: `void MSPModel_Interacting::remove(MSTransportableStateAdapter *)` [virtual]
    /// remove the specified person from the pedestrian simulation
    // void
    // MSPModel_Interacting::remove(MSTransportableStateAdapter* state) {
    //     Pedestrians& pedestrians = myActiveLanes[state->getLane()];
    //     MSPModel_InteractingState* const p = static_cast<MSPModel_InteractingState*>(state);
    //     const auto& it = std::find(pedestrians.begin(), pedestrians.end(), p);
    //     if (it != pedestrians.end()) {
    //         if (p->getNextCrossing() != nullptr) {
    //             unregisterCrossingApproach(*p, p->getNextCrossing());
    //         }
    //         pedestrians.erase(it);
    //         myNumActivePedestrians--;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove(&mut self, state: Ptr<MSTransportableStateAdapter>) {
        let p: Ptr<MSPModel_InteractingState> = state.cast::<MSPModel_InteractingState>();
        let pedestrians = self.myActiveLanes.entry(MSPModel_Interacting_LaneKey(state.get_lane())).or_default();
        if let Some(idx) = pedestrians.iter().position(|x| *x == p) {
            if !p.get_next_crossing().is_null() {
                MSPModel_Interacting::unregister_crossing_approach(&*p, p.get_next_crossing());
            }
            pedestrians.remove(idx);
            self.myNumActivePedestrians -= 1;
        }
    }

    /// C++: `bool MSPModel_Interacting::blockedAtDist(const SUMOTrafficObject *, const MSLane *, double, double, double, std::vector<const MSPerson *> *)` [virtual]
    /// whether a pedestrian is blocking the crossing of lane for the given vehicle bondaries
    // bool
    // MSPModel_Interacting::blockedAtDist(const SUMOTrafficObject* ego, const MSLane* lane, double vehCenter, double vehWidth,
    //                                     double oncomingGap, std::vector<const MSPerson*>* collectBlockers) {
    //     for (const MSPModel_InteractingState* ped : getPedestrians(lane)) {
    //         const double leaderFrontDist = (ped->getDirection() == FORWARD ? vehCenter - ped->getEdgePos(0) : ped->getEdgePos(0) - vehCenter) - vehWidth * 0.5;
    //         const double leaderBackDist = leaderFrontDist + ped->getPerson()->getVehicleType().getLength();
    // #ifdef DEBUG_INTERACTING
    //         if DEBUGCOND(ped) {
    //             std::cout << SIMTIME << " lane=" << lane->getID() << " dir=" << ped->getDirection() << " pX=" << ped->getEdgePos(0) << " pL=" << ped->getPerson()->getVehicleType().getLength()
    //                       << " vehCenter=" << vehCenter
    //                       << " vehWidth=" << vehWidth
    //                       << " lBD=" << leaderBackDist
    //                       << " lFD=" << leaderFrontDist
    //                       << "\n";
    //         }
    // #endif
    //         if (leaderBackDist >= -vehWidth
    //                 && (leaderFrontDist < 0
    //                     // give right of way to (close) approaching pedestrians unless they are standing
    //                     || (leaderFrontDist <= oncomingGap && ped->getWaitingTime() < TIME2STEPS(2.0)))) {
    //             if (MSLink::ignoreFoe(ego, ped->getPerson())) {
    //                 continue;
    //             }
    //             // found one pedestrian that is not completely past the crossing point
    //             //std::cout << SIMTIME << " blocking pedestrian foeLane=" << lane->getID() << " ped=" << ped->getPerson()->getID() << " dir=" << ped->getDirection() << " pX=" << ped->getEdgePos(0) << " pL=" << ped.getLength() << " fDTC=" << distToCrossing << " lBD=" << leaderBackDist << "\n";
    //             if (collectBlockers == nullptr) {
    //                 return true;
    //             }
    //             collectBlockers->push_back(ped->getPerson());
    //         }
    //     }
    //     if (collectBlockers == nullptr) {
    //         return false;
    //     }
    //     return collectBlockers->size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn blocked_at_dist(&mut self, ego: Ptr<SUMOTrafficObject>, lane: Ptr<MSLane>, vehCenter: f64, vehWidth: f64, oncomingGap: f64, collectBlockers: Ptr<Vec<Ptr<MSPerson>>>) -> bool {
        for ped in self.get_pedestrians(lane).iter().copied() {
            let leaderFrontDist = (if ped.get_direction() == MSPMODEL_FORWARD { vehCenter - ped.get_edge_pos(0) } else { ped.get_edge_pos(0) - vehCenter }) - vehWidth * 0.5;
            let leaderBackDist = leaderFrontDist + ped.get_person().get_vehicle_type().get_length();
            // DEBUG_INTERACTING output (compile-time debug logging) is not ported
            if leaderBackDist >= -vehWidth && (leaderFrontDist < 0.0 || (leaderFrontDist <= oncomingGap && ped.get_waiting_time() < utils::common::time::time2steps(2.0))) {
                let person = ped.get_person();
                if MSLink::ignore_foe(ego, Ptr::from_mut(&mut person.m().base.base)) {
                    continue;
                }
                // found one pedestrian that is not completely past the crossing point
                if collectBlockers.is_null() {
                    return true;
                }
                collectBlockers.m().push(person);
            }
        }
        if collectBlockers.is_null() {
            return false;
        }
        collectBlockers.m().len() > 0
    }

    /// C++: `PersonDist MSPModel_Interacting::nextBlocking(const MSLane *, double, double, double, double, bool)` [virtual]
    /// returns the next pedestrian beyond minPos that is laterally between minRight and maxLeft or nullptr
    // PersonDist
    // MSPModel_Interacting::nextBlocking(const MSLane* lane, double minPos, double minRight, double maxLeft, double stopTime, bool bidi) {
    //     PersonDist result((const MSPerson*)nullptr, std::numeric_limits<double>::max());
    //     for (const MSPModel_InteractingState* ped : getPedestrians(lane)) {
    //         // account for distance covered by oncoming pedestrians
    //         double relX2 = ped->getEdgePos(0) - (ped->getDirection() == FORWARD ? 0 : stopTime * ped->getPerson()->getMaxSpeed());
    //         double dist = ((relX2 - minPos) * (bidi ? -1 : 1)
    //                        - (ped->getDirection() == FORWARD ? ped->getPerson()->getVehicleType().getLength() : 0));
    //         const bool aheadOfVehicle = bidi ? ped->getEdgePos(0) < minPos : ped->getEdgePos(0) > minPos;
    //         if (aheadOfVehicle && dist < result.second) {
    //             const double center = ped->getLatOffset() + 0.5 * lane->getWidth();
    //             const double halfWidth = 0.5 * ped->getPerson()->getVehicleType().getWidth();
    //             const bool overlap = (center + halfWidth > minRight && center - halfWidth < maxLeft);
    // #ifdef DEBUG_INTERACTING
    //             if DEBUGCOND(ped) {
    //                 std::cout << "  nextBlocking lane=" << lane->getID() << " bidi=" << bidi
    //                           << " minPos=" << minPos << " minRight=" << minRight << " maxLeft=" << maxLeft
    //                           << " stopTime=" << stopTime
    //                           << " ped=" << ped->getID()
    //                           << " pedX=" << ped->getEdgePos(0)
    //                           << " relX2=" << relX2
    //                           << " center=" << center
    //                           << " pedLeft=" << center + halfWidth
    //                           << " pedRight=" << center - halfWidth
    //                           << " overlap=" << overlap
    //                           << "\n";
    //             }
    // #endif
    //             if (overlap) {
    //                 result.first = ped->getPerson();
    //                 result.second = dist;
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn next_blocking(&mut self, lane: Ptr<MSLane>, minPos: f64, minRight: f64, maxLeft: f64, stopTime: f64, bidi: bool) -> PersonDist {
        let mut result: PersonDist = (Ptr::null(), f64::MAX);
        for ped in self.get_pedestrians(lane).iter().copied() {
            // account for distance covered by oncoming pedestrians
            let relX2 = ped.get_edge_pos(0) - (if ped.get_direction() == MSPMODEL_FORWARD { 0.0 } else { stopTime * ped.get_person().get_max_speed() });
            let dist = ((relX2 - minPos) * (if bidi { -1.0 } else { 1.0 }) - (if ped.get_direction() == MSPMODEL_FORWARD { ped.get_person().get_vehicle_type().get_length() } else { 0.0 }));
            let aheadOfVehicle = if bidi { ped.get_edge_pos(0) < minPos } else { ped.get_edge_pos(0) > minPos };
            if aheadOfVehicle && dist < result.1 {
                let center = ped.get_lat_offset() + 0.5 * lane.get_width();
                let halfWidth = 0.5 * ped.get_person().get_vehicle_type().get_width();
                let overlap = center + halfWidth > minRight && center - halfWidth < maxLeft;
                // DEBUG_INTERACTING output (compile-time debug logging) is not ported
                if overlap {
                    result.0 = ped.get_person();
                    result.1 = dist;
                }
            }
        }
        result
    }

    /// C++: `bool MSPModel_Interacting::hasPedestrians(const MSLane *)` [virtual]
    /// whether the given lane has pedestrians on it
    // bool
    // MSPModel_Interacting::hasPedestrians(const MSLane* lane) {
    //     return getPedestrians(lane).size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn has_pedestrians(&mut self, lane: Ptr<MSLane>) -> bool {
        self.get_pedestrians(lane).len() > 0
    }

    /// C++: `bool MSPModel_Interacting::usingInternalLanes()` [virtual]
    /// whether movements on intersections are modelled /
    // bool
    // MSPModel_Interacting::usingInternalLanes() {
    //     return usingInternalLanesStatic();
    // }
    #[allow(unused_variables)]
    pub fn using_internal_lanes(&mut self) -> bool {
        Self::using_internal_lanes_static()
    }

    /// C++: `bool MSPModel_Interacting::usingInternalLanesStatic()` [static]
    // bool
    // MSPModel_Interacting::usingInternalLanesStatic() {
    //     return MSGlobals::gUsingInternalLanes && MSNet::getInstance()->hasInternalLinks() && MSNet::getInstance()->hasPedestrianNetwork();
    // }
    #[allow(unused_variables)]
    pub fn using_internal_lanes_static() -> bool {
        MSGlobals::using_internal_lanes() && MSNet::get_instance().has_internal_links() && MSNet::get_instance().has_pedestrian_network()
    }

    /// C++: `int MSPModel_Interacting::getActiveNumber()` [virtual]
    /// return the number of active pedestrians
    // int getActiveNumber() {
    //         return myNumActivePedestrians;
    //     }
    #[allow(unused_variables)]
    pub fn get_active_number(&mut self) -> i32 {
        self.myNumActivePedestrians
    }

    /// C++: `void MSPModel_Interacting::registerActive()`
    /// increase the number of active pedestrians
    // void registerActive() {
    //         myNumActivePedestrians++;
    //     }
    #[allow(unused_variables)]
    pub fn register_active(&mut self) {
        self.myNumActivePedestrians += 1;
    }

    /// C++: `void MSPModel_Interacting::unregisterCrossingApproach(const MSPModel_InteractingState &, const MSLane *)` [static]
    /// unregister pedestrian approach with the junction model
    // void
    // MSPModel_Interacting::unregisterCrossingApproach(const MSPModel_InteractingState& ped, const MSLane* crossing) {
    //     // person has entered the crossing
    //     crossing->getIncomingLanes()[0].viaLink->removeApproachingPerson(ped.getPerson());
    // #ifdef DEBUG_CROSSING_APPROACH
    //     if DEBUGCOND(&ped) {
    //         std::cout << SIMTIME << " unregister " << ped.getPerson()->getID() << " at crossing " << crossing->getID() << "\n";
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn unregister_crossing_approach(ped: &MSPModel_InteractingState, crossing: Ptr<MSLane>) {
        crossing.get_incoming_lanes()[0].viaLink.m().remove_approaching_person(ped.get_person());
    }

    /// C++: `Pedestrians & MSPModel_Interacting::getPedestrians(const MSLane *)` [protected]
    /// retrieves the pedestrian vector for the given lane (may be empty)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_pedestrians(&mut self, lane: Ptr<MSLane>) -> &mut MSPModel_Interacting_Pedestrians {
        if let Some(v) = self.myActiveLanes.get_mut(&MSPModel_Interacting_LaneKey(lane)) {
            return v;
        }
        NO_PEDESTRIANS.with(|p| *p).get_mut().unwrap()
    }

}

/// Container for pedestrian state and individual position update function
/// C++ class `MSPModel_InteractingState` (microsim/transportables/MSPModel_Interacting.h)
#[repr(C)]
pub struct MSPModel_InteractingState {
    /// C++ base class `MSTransportableStateAdapter`
    pub base: MSTransportableStateAdapter,
    /// the person who is being represented
    pub myPerson: Ptr<MSPerson>,
    /// the current stage of this pedestrian
    pub myStage: Ptr<MSStageMoving>,
    /// the current lane of this pedestrian
    pub myLane: Ptr<MSLane>,
    /// the advancement along the current lane
    pub myEdgePos: f64,
    /// the orthogonal shift on the current lane
    pub myPosLat: f64,
    /// the walking direction on the current lane (1 forward, -1 backward)
    pub myDir: i32,
    /// the current walking speed
    pub mySpeed: f64,
    /// the current lateral walking speed
    pub mySpeedLat: f64,
    /// whether the pedestrian is waiting to start its walk
    pub myWaitingToEnter: bool,
    /// the consecutive time spent at speed 0
    pub myWaitingTime: SUMOTime,
    /// the total time spent at speed 0
    pub myTotalWaitingTime: SUMOTime,
    /// whether the person is jammed
    pub myAmJammed: bool,
    /// remote-controlled position
    pub myRemoteXYPos: Position,
    /// cached angle
    pub myAngle: std::cell::Cell<f64>,
}

impl std::ops::Deref for MSPModel_InteractingState {
    type Target = MSTransportableStateAdapter;
    fn deref(&self) -> &MSTransportableStateAdapter {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_InteractingState {
    fn deref_mut(&mut self) -> &mut MSTransportableStateAdapter {
        &mut self.base
    }
}

/// virtual overrides of MSPModel_InteractingState (the others keep the MSTransportableStateAdapter base implementation)
pub static MSPMODEL_INTERACTING_STATE_VT: MSTransportableStateAdapterVt = MSTransportableStateAdapterVt {
    get_edge_pos: |a, now| Ptr::new(a).cast::<MSPModel_InteractingState>().get_edge_pos(now),
    get_direction: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_direction(),
    get_position: adapter_default_get_position,
    get_angle: adapter_default_get_angle,
    get_waiting_time: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_waiting_time(),
    get_total_waiting_time: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_total_waiting_time(),
    get_speed: |a, _| Ptr::new(a).cast::<MSPModel_InteractingState>().get_speed(),
    get_next_edge: adapter_default_get_next_edge,
    move_to: adapter_default_move_to,
    move_to_xy: adapter_default_move_to_xy,
    is_jammed: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().is_jammed(),
    is_finished: adapter_default_is_finished,
    get_lane: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_lane(),
    get_path_length: adapter_default_get_path_length,
    save_state: adapter_default_save_state,
    get_id: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get().unwrap().myPerson.get_id(),
    get_min_x: adapter_default_get_min_x,
    get_max_x: adapter_default_get_max_x,
    get_width: adapter_default_get_width,
    get_o_type: adapter_default_get_o_type,
};

impl MSPModel_InteractingState {
    /// C++: `void MSPModel_InteractingState::MSPModel_InteractingState(MSPerson *, MSStageMoving *, const MSLane *)`
    /// constructor
    // MSPModel_InteractingState(MSPerson* person, MSStageMoving* stage, const MSLane* lane) :
    //         myPerson(person),
    //         myStage(stage),
    //         myLane(lane),
    //         myEdgePos(stage != nullptr ? stage->getDepartPos() : 0.),
    //         myPosLat(stage != nullptr ? stage->getDepartPosLat() : 0.),
    //         myDir(MSPModel::UNDEFINED_DIRECTION),
    //         myWaitingToEnter(person != nullptr) {}
    #[allow(unused_variables)]
    pub fn new(person: Ptr<MSPerson>, stage: Ptr<MSStageMoving>, lane: Ptr<MSLane>) -> Self {
        Self {
            base: MSTransportableStateAdapter { vt: &MSPMODEL_INTERACTING_STATE_VT },
            myPerson: person,
            myStage: stage,
            myLane: lane,
            myEdgePos: if !stage.is_null() { stage.get_depart_pos() } else { 0. },
            myPosLat: if !stage.is_null() { stage.get_depart_pos_lat() } else { 0. },
            myDir: MSPMODEL_UNDEFINED_DIRECTION,
            mySpeed: 0.,
            mySpeedLat: 0.,
            myWaitingToEnter: !person.is_null(),
            myWaitingTime: 0,
            myTotalWaitingTime: 0,
            myAmJammed: false,
            myRemoteXYPos: Position::INVALID,
            myAngle: std::cell::Cell::new(f64::MAX),
        }
    }

    /// C++: `void MSPModel_InteractingState::~MSPModel_InteractingState() noexcept` [virtual]
    // ~MSPModel_InteractingState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `double MSPModel_InteractingState::getEdgePos(SUMOTime) const` [const, virtual]
    /// return the offset from the start of the current edge measured in its natural direction
    // inline double getEdgePos(SUMOTime /* now */) const {
    //         return myEdgePos;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, arg0: SUMOTime) -> f64 {
        self.myEdgePos
    }

    /// C++: `int MSPModel_InteractingState::getDirection() const` [const, virtual]
    /// return the walking direction (FORWARD, BACKWARD, UNDEFINED_DIRECTION)
    // inline int getDirection() const {
    //         return myDir;
    //     }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        self.myDir
    }

    /// C++: `SUMOTime MSPModel_InteractingState::getWaitingTime() const` [const, virtual]
    /// return the time the transportable spent standing
    // inline SUMOTime getWaitingTime() const {
    //         return myWaitingTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        self.myWaitingTime
    }

    /// C++: `SUMOTime MSPModel_InteractingState::getTotalWaitingTime() const` [const, virtual]
    // virtual SUMOTime getTotalWaitingTime() const {
    //         return myTotalWaitingTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_waiting_time(&self) -> SUMOTime {
        self.myTotalWaitingTime
    }

    /// C++: `double MSPModel_InteractingState::getSpeed() const` [const]
    /// return the current speed of the transportable, we need to be able to do this for vehicles as well, so we cannot refer to the stage
    // inline double getSpeed() const {
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        self.mySpeed
    }

    /// C++: `double MSPModel_InteractingState::getSpeed(const MSStageMoving &) const` [const, virtual]
    /// return the current speed of the transportable
    // inline double getSpeed(const MSStageMoving& /* stage */) const {
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_2(&self, arg0: &MSStageMoving) -> f64 {
        self.mySpeed
    }

    /// C++: `bool MSPModel_InteractingState::isJammed() const` [const, virtual]
    /// whether the transportable is jammed
    // inline bool isJammed() const {
    //         return myAmJammed;
    //     }
    #[allow(unused_variables)]
    pub fn is_jammed(&self) -> bool {
        self.myAmJammed
    }

    /// C++: `const MSLane * MSPModel_InteractingState::getLane() const` [const, virtual]
    /// the current lane of the transportable
    // inline const MSLane* getLane() const {
    //         return myLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        self.myLane
    }

    /// C++: `const MSLane * MSPModel_InteractingState::getNextCrossing() const` [const, virtual]
    /// placeholder function for the accessing the next crossing
    // virtual const MSLane* getNextCrossing() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_crossing(&self) -> Ptr<MSLane> {
        Ptr::null()
    }

    /// C++: `double MSPModel_InteractingState::getLatOffset() const` [const, virtual]
    /// return the lateral offset to the lane center
    // virtual double getLatOffset() const {
    //         return myPosLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_lat_offset(&self) -> f64 {
        self.myPosLat
    }

    /// C++: `MSPerson * MSPModel_InteractingState::getPerson() const` [const]
    /// return the represented person
    // inline MSPerson* getPerson() const {
    //         return myPerson;
    //     }
    #[allow(unused_variables)]
    pub fn get_person(&self) -> Ptr<MSPerson> {
        self.myPerson
    }

    /// C++: `MSStageMoving * MSPModel_InteractingState::getStage() const` [const]
    /// return the current stage
    // inline MSStageMoving* getStage() const {
    //         return myStage;
    //     }
    #[allow(unused_variables)]
    pub fn get_stage(&self) -> Ptr<MSStageMoving> {
        self.myStage
    }

    /// C++: `bool MSPModel_InteractingState::isWaitingToEnter() const` [const]
    /// whether the person still waits to entere the network
    // inline bool isWaitingToEnter() const {
    //         return myWaitingToEnter;
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting_to_enter(&self) -> bool {
        self.myWaitingToEnter
    }

    /// C++: `const Position & MSPModel_InteractingState::getRemotePosition() const` [const]
    /// return the remote position if being controlled by TraCI or JuPedSim
    // inline const Position& getRemotePosition() const {
    //         return myRemoteXYPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_remote_position(&self) -> &Position {
        &self.myRemoteXYPos
    }

    /// C++: `const std::string & MSPModel_InteractingState::getID() const` [const, virtual]
    /// return ID of the person (or sometimes vehicle) being represented
    // virtual const std::string& getID() const {
    //         return myPerson->getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        (self.base.vt.get_id)(&self.base)
    }

    /// C++: `double MSPModel_InteractingState::getAngle(const MSStageMoving &, SUMOTime) const` [const, virtual]
    /// return the current orientation in degrees
    // virtual double getAngle(const MSStageMoving&, SUMOTime) const {
    //         return myAngle;
    //     }
    #[allow(unused_variables)]
    pub fn get_angle(&self, arg0: &MSStageMoving, arg1: SUMOTime) -> f64 {
        self.myAngle.get()
    }

    /// C++: `MSPModel_InteractingState & MSPModel_InteractingState::operator=(const MSPModel_InteractingState &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPModel_InteractingState) -> &mut MSPModel_InteractingState {
        panic!("MSPModel_InteractingState is not assignable (operator= is deleted in C++)")
    }

}

