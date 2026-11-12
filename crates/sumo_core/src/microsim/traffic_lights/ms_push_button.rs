//! Port of microsim/traffic_lights/MSPushButton.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::{Cell, RefCell};

/// Pedestrian push button
/// C++ class `MSPedestrianPushButton` (microsim/traffic_lights/MSPushButton.h)
#[repr(C)]
pub struct MSPedestrianPushButton {
    /// C++ base class `MSPushButton`
    pub base: MSPushButton,
}

impl std::ops::Deref for MSPedestrianPushButton {
    type Target = MSPushButton;
    fn deref(&self) -> &MSPushButton {
        &self.base
    }
}
impl std::ops::DerefMut for MSPedestrianPushButton {
    fn deref_mut(&mut self) -> &mut MSPushButton {
        &mut self.base
    }
}

// C++ static members of MSPedestrianPushButton (port as module-level statics, see AGENTS.md):
//   static std::map<std::string, std::vector<std::string>> MSPedestrianPushButton::m_crossingEdgeMap;
//   static bool MSPedestrianPushButton::m_crossingEdgeMapLoaded;
thread_local! {
    static M_CROSSING_EDGE_MAP: RefCell<BTreeMap<String, Vec<String>>> = RefCell::new(BTreeMap::new());
    static M_CROSSING_EDGE_MAP_LOADED: Cell<bool> = Cell::new(false);
}

impl MSPedestrianPushButton {
    /// C++: `void MSPedestrianPushButton::MSPedestrianPushButton(const MSEdge *, const MSEdge *)`
    /// MSPedestrianPushButton constructor
    // MSPedestrianPushButton::MSPedestrianPushButton(const MSEdge* walkingEdge, const MSEdge* crossingEdge)
    //     : MSPushButton(walkingEdge, crossingEdge) {
    //     assert(walkingEdge->isWalkingArea() || ((walkingEdge->getPermissions() & SVC_PEDESTRIAN) != 0));
    //     assert(crossingEdge->isCrossing());
    // }
    #[allow(unused_variables)]
    pub fn new(walkingEdge: Ptr<MSEdge>, crossingEdge: Ptr<MSEdge>) -> Self {
        let base = MSPushButton::new(walkingEdge, crossingEdge);
        debug_assert!(walkingEdge.is_walking_area() || (walkingEdge.get_permissions() & SVC_PEDESTRIAN) != 0);
        debug_assert!(crossingEdge.is_crossing());
        Self { base }
    }

    /// C++: `void MSPedestrianPushButton::~MSPedestrianPushButton() noexcept` [virtual]
    // virtual ~MSPedestrianPushButton() {
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty
    }

    /// C++: `bool MSPedestrianPushButton::isActivated() const` [const, virtual]
    /// abstract methods inherited from PedestrianState @{
    // bool MSPedestrianPushButton::isActivated() const {
    //     return isActiveForEdge(m_edge, m_crossingEdge);
    // }
    #[allow(unused_variables)]
    pub fn is_activated(&self) -> bool {
        Self::is_active_for_edge(self.m_edge, self.m_crossingEdge)
    }

    /// C++: `bool MSPedestrianPushButton::isActiveForEdge(const MSEdge *, const MSEdge *)` [static]
    /// Checks if the pushbutton has been pressed for a particular crossing from a edge.
    // bool MSPedestrianPushButton::isActiveForEdge(const MSEdge* walkingEdge, const MSEdge* crossing) {
    //     const std::set<MSTransportable*, ComparatorNumericalIdLess> persons = walkingEdge->getPersons();
    //     if (persons.size() > 0) {
    //         for (std::set<MSTransportable*>::const_iterator pIt = persons.begin(); pIt != persons.end(); ++pIt) {
    //             const MSPerson* person = (MSPerson*)*pIt;
    //             const MSEdge* nextEdge = person->getNextEdgePtr();
    //             ///TODO keep using >= 1 or switch to ==1. Should change return value from always active to active only when pressed?
    //             ///TODO If changed the swarm logic must be changed since it relies on this behavior that keeps it active
    //             if (person->getWaitingSeconds() >= 1 && nextEdge && nextEdge->getID() == crossing->getID()) {
    // #ifdef SWARM_DEBUG
    //                 std::ostringstream oss;
    //                 oss << "MSPedestrianPushButton::isActiveForEdge Pushbutton active for edge " << walkingEdge->getID() << " crossing " << crossing->getID()
    //                     << " for " << person->getID() << " wait " << person->getWaitingSeconds();
    //                 WRITE_MESSAGE(oss.str());
    // #endif
    //                 return true;
    //             }
    //         }
    //     } else {
    //         //No person currently on the edge. But there may be some vehicles of class pedestrian
    //         for (std::vector<MSLane*>::const_iterator laneIt = walkingEdge->getLanes().begin();
    //                 laneIt != walkingEdge->getLanes().end(); ++laneIt) {
    //             MSLane* lane = *laneIt;
    //             MSLane::VehCont vehicles = lane->getVehiclesSecure();
    //             for (MSLane::VehCont::const_iterator vehicleIt = vehicles.begin(); vehicleIt != vehicles.end(); ++vehicleIt) {
    //                 MSVehicle* vehicle = *vehicleIt;
    //                 if (vehicle->getVClass() == SVC_PEDESTRIAN) {
    //                     // It's a pedestrian
    //                     const MSEdge* nextEdge = vehicle->succEdge(1);
    //                     if (vehicle->getWaitingSeconds() >= 1 && nextEdge) {
    //                         // Next edge is not internal. Try to find if between the current vehicle edge and the next is the crossing.
    //                         // To do that check if between the successors (or predecessor) of crossing is the next edge and walking  precedes (or ensue) it.
    //                         if ((std::find(crossing->getPredecessors().begin(), crossing->getPredecessors().end(), walkingEdge) != crossing->getPredecessors().end()
    //                                 && std::find(crossing->getSuccessors().begin(), crossing->getSuccessors().end(), nextEdge) != crossing->getSuccessors().end())
    //                                 || (std::find(crossing->getSuccessors().begin(), crossing->getSuccessors().end(), walkingEdge) != crossing->getSuccessors().end()
    //                                     && std::find(crossing->getPredecessors().begin(), crossing->getPredecessors().end(), nextEdge) != crossing->getPredecessors().end())) {
    // #ifdef SWARM_DEBUG
    //                             std::ostringstream oss;
    //                             oss << "MSPedestrianPushButton::isActiveForEdge Pushbutton active for edge " << walkingEdge->getID() << " crossing " << crossing->getID()
    //                                 << " for " << vehicle->getID() << " wait " << vehicle->getWaitingSeconds();
    //                             WRITE_MESSAGE(oss.str());
    // #endif
    //                             // Also release the vehicles here
    //                             lane->releaseVehicles();
    //                             return true;
    //                         }
    //                     }
    //                 }
    //             }
    //             lane->releaseVehicles();
    //         }
    //     }
    // #ifdef SWARM_DEBUG
    //     std::ostringstream oss;
    //     oss << "MSPedestrianPushButton::isActiveForEdge Pushbutton not active for edge " << walkingEdge->getID() << " crossing " << crossing->getID()
    //         << " num Persons " << persons.size();
    //     WRITE_MESSAGE(oss.str());
    // #endif
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_active_for_edge(walkingEdge: Ptr<MSEdge>, crossing: Ptr<MSEdge>) -> bool {
        let persons = walkingEdge.get_persons().clone();
        if !persons.is_empty() {
            // determinism: C++ iterates the std::set in numerical-id order; here it is address order.
            // The loop only ORs a pure predicate, so the boolean result does not depend on the order.
            for &pIt in persons.iter() {
                let person: Ptr<MSPerson> = pIt.cast::<MSPerson>();
                let nextEdge = person.get_next_edge_ptr();
                if person.get_waiting_seconds() >= 1.0 && !nextEdge.is_null() && nextEdge.get_id() == crossing.get_id() {
                    return true;
                }
            }
        } else {
            // No person currently on the edge. But there may be some vehicles of class pedestrian
            for &lane in walkingEdge.get_lanes().iter() {
                let vehicles = lane.get_vehicles_secure().clone();
                for &vehicle in vehicles.iter() {
                    if vehicle.get_v_class() == SVC_PEDESTRIAN {
                        // It's a pedestrian
                        let nextEdge = vehicle.succ_edge(1);
                        if vehicle.get_waiting_seconds() >= 1.0 && !nextEdge.is_null() {
                            // Next edge is not internal. Try to find if between the current vehicle edge and the next is the crossing.
                            // To do that check if between the successors (or predecessor) of crossing is the next edge and walking  precedes (or ensue) it.
                            if (crossing.get_predecessors().contains(&walkingEdge) && crossing.get_successors(SVC_IGNORING).contains(&nextEdge))
                                || (crossing.get_successors(SVC_IGNORING).contains(&walkingEdge) && crossing.get_predecessors().contains(&nextEdge))
                            {
                                // Also release the vehicles here
                                lane.release_vehicles();
                                return true;
                            }
                        }
                    }
                }
                lane.release_vehicles();
            }
        }
        // SWARM_DEBUG-only messages are compiled out, as in a default SUMO build
        false
    }

    /// C++: `bool MSPedestrianPushButton::isActiveOnAnySideOfTheRoad(const MSEdge *)` [static]
    /// Static method to check if the push button is active on both side of the road
    // bool MSPedestrianPushButton::isActiveOnAnySideOfTheRoad(const MSEdge* crossing) {
    //     const std::vector<MSEdge*> walkingList = getWalkingAreas(crossing);
    //     for (std::vector<MSEdge*>::const_iterator wIt = walkingList.begin(); wIt != walkingList.end(); ++wIt) {
    //         MSEdge* walking = *wIt;
    //         if (isActiveForEdge(walking, crossing)) {
    // #ifdef SWARM_DEBUG
    //             WRITE_MESSAGE("MSPedestrianPushButton::isActiveOnAnySideOfTheRoad crossing edge " + crossing->getID() + " walking edge" + walking->getID());
    // #endif
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_active_on_any_side_of_the_road(crossing: Ptr<MSEdge>) -> bool {
        let walkingList = get_walking_areas(crossing);
        for &walking in walkingList.iter() {
            if Self::is_active_for_edge(walking, crossing) {
                return true;
            }
        }
        false
    }

    /// C++: `std::vector<MSPushButton *> MSPedestrianPushButton::loadPushButtons(const MSPhaseDefinition *)` [static]
    /// Loads all the pushbuttons for all the controlled lanes of a stage
    // std::vector<MSPushButton*> MSPedestrianPushButton::loadPushButtons(const MSPhaseDefinition* phase) {
    //     loadCrossingEdgeMap();
    //     std::vector<MSPushButton*> pushButtons;
    //     const std::vector<std::string> lanes = phase->getTargetLaneSet();
    // //    Multiple lane can be of the same edge, so I avoid readding them
    //     std::set<std::string> controlledEdges;
    //     for (std::vector<std::string>::const_iterator lIt = lanes.begin(); lIt != lanes.end(); ++lIt) {
    //         MSLane* lane = MSLane::dictionary(*lIt);
    //         if (lane) {
    //             MSEdge* laneEdge = &lane->getEdge();
    //             if (controlledEdges.count(laneEdge->getID()) != 0) {
    //                 continue;
    //             }
    //             controlledEdges.insert(laneEdge->getID());
    //             if (m_crossingEdgeMap.find(laneEdge->getID()) != m_crossingEdgeMap.end()) {
    //                 //For every crossing edge that crosses this edge
    //                 for (std::vector<std::string>::const_iterator cIt = m_crossingEdgeMap[laneEdge->getID()].begin();
    //                         cIt != m_crossingEdgeMap[laneEdge->getID()].end(); ++cIt) {
    //                     MSEdge* crossing = MSEdge::dictionary(*cIt);
    //                     const std::vector<MSEdge*> walkingList = getWalkingAreas(crossing);
    //                     for (std::vector<MSEdge*>::const_iterator wIt = walkingList.begin(); wIt != walkingList.end(); ++wIt) {
    //                         MSEdge* walking = *wIt;
    // #ifdef SWARM_DEBUG
    //                         WRITE_MESSAGE("MSPedestrianPushButton::loadPushButtons Added pushButton for walking edge " + walking->getID() + " crossing edge "
    //                                       + crossing->getID() + " crossed edge " + laneEdge->getID() + ". Phase state " + phase->getState());
    // #endif
    //                         pushButtons.push_back(new MSPedestrianPushButton(walking, crossing));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return pushButtons;
    // }
    #[allow(unused_variables)]
    pub fn load_push_buttons(arg0: Ptr<MSPhaseDefinition>) -> Vec<Ptr<MSPushButton>> {
        Self::load_crossing_edge_map();
        let mut pushButtons: Vec<Ptr<MSPushButton>> = Vec::new();
        let lanes = arg0.get_target_lane_set();
        // Multiple lane can be of the same edge, so I avoid readding them
        let mut controlledEdges: BTreeSet<String> = BTreeSet::new();
        for lIt in lanes.iter() {
            let lane = MSLane::dictionary_2(lIt);
            if !lane.is_null() {
                let laneEdge = Ptr::from_mut(lane.get_edge());
                if controlledEdges.contains(laneEdge.get_id()) {
                    continue;
                }
                controlledEdges.insert(laneEdge.get_id().to_string());
                if M_CROSSING_EDGE_MAP.with(|m| m.borrow().contains_key(laneEdge.get_id())) {
                    // For every crossing edge that crosses this edge
                    let crossingIds: Vec<String> = M_CROSSING_EDGE_MAP.with(|m| m.borrow()[laneEdge.get_id()].clone());
                    for cIt in crossingIds.iter() {
                        let crossing = edge_by_id(cIt);
                        let walkingList = get_walking_areas(crossing);
                        for &walking in walkingList.iter() {
                            pushButtons.push(Ptr::from_raw(Box::into_raw(Box::new(MSPedestrianPushButton::new(walking, crossing)))).cast::<MSPushButton>());
                        }
                    }
                }
            }
        }
        pushButtons
    }

    /// C++: `void MSPedestrianPushButton::loadCrossingEdgeMap()` [static, private]
    // void MSPedestrianPushButton::loadCrossingEdgeMap() {
    //     if (!m_crossingEdgeMapLoaded) {
    //         m_crossingEdgeMapLoaded = true;
    //         for (MSEdgeVector::const_iterator eIt = MSEdge::getAllEdges().begin(); eIt != MSEdge::getAllEdges().end(); ++eIt) {
    //             const MSEdge* edge = *eIt;
    //             if (edge->isCrossing()) {
    //                 for (std::vector<std::string>::const_iterator cIt = edge->getCrossingEdges().begin();
    //                         cIt != edge->getCrossingEdges().end(); ++cIt) {
    //                     m_crossingEdgeMap[*cIt].push_back(edge->getID());
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_crossing_edge_map() {
        if !M_CROSSING_EDGE_MAP_LOADED.with(|l| l.get()) {
            M_CROSSING_EDGE_MAP_LOADED.with(|l| l.set(true));
            for &edge in MSEdge::get_all_edges().iter() {
                if edge.is_crossing() {
                    for cId in edge.get_crossing_edges().iter() {
                        M_CROSSING_EDGE_MAP.with(|m| m.borrow_mut().entry(cId.clone()).or_default().push(edge.get_id().to_string()));
                    }
                }
            }
        }
    }

}

/// Abstract push button class
/// C++ class `MSPushButton` (microsim/traffic_lights/MSPushButton.h)
#[repr(C)]
pub struct MSPushButton {
    pub m_edge: Ptr<MSEdge>,
    pub m_crossingEdge: Ptr<MSEdge>,
}

impl MSPushButton {
    /// C++: `void MSPushButton::~MSPushButton() noexcept` [virtual]
    // MSPushButton::~MSPushButton() {
    //     /// do not delete the MSEdge here
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty: the MSEdge is not deleted here
    }

    /// C++: `bool MSPushButton::isActivated() const` [const, virtual, pure virtual]
    /// Checks if the pushbutton has been pressed
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_activated(&self) -> bool {
        // pure virtual in C++: only MSPedestrianPushButton implements it
        unreachable!("MSPushButton::isActivated is pure virtual")
    }

    /// C++: `bool MSPushButton::anyActive(const std::vector<MSPushButton *> &)` [static]
    /// Checks if any pushbutton in the vector is active
    // bool MSPushButton::anyActive(const std::vector<MSPushButton*>& pushButtons) {
    //     for (std::vector<MSPushButton*>::const_iterator it = pushButtons.begin(); it != pushButtons.end(); ++it) {
    //         if (it.operator * ()->isActivated()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn any_active(arg0: &Vec<Ptr<MSPushButton>>) -> bool {
        for &it in arg0.iter() {
            // virtual dispatch: the only subclass is MSPedestrianPushButton, whose base is first (repr(C))
            if it.cast::<MSPedestrianPushButton>().is_activated() {
                return true;
            }
        }
        false
    }

    /// C++: `void MSPushButton::MSPushButton(const MSEdge *, const MSEdge *)` [protected]
    /// Protected constructor
    // MSPushButton::MSPushButton(const MSEdge* edge, const MSEdge* crossingEdge) {
    //     m_edge = edge;
    //     m_crossingEdge = crossingEdge;
    // }
    #[allow(unused_variables)]
    pub fn new(edge: Ptr<MSEdge>, crossingEdge: Ptr<MSEdge>) -> Self {
        MSPushButton { m_edge: edge, m_crossingEdge: crossingEdge }
    }

}

/// C++ `MSEdge::dictionary(id)` lookup
// shortcut: linear scan over MSEdge::get_all_edges() because MSEdge::dictionary_2 takes an Unmapped key in ms_edge.rs, switch to MSEdge::dictionary_2 once that is mapped to &str
fn edge_by_id(id: &str) -> Ptr<MSEdge> {
    for &edge in MSEdge::get_all_edges().iter() {
        if !edge.is_null() && edge.get_id() == id {
            return edge;
        }
    }
    Ptr::null()
}

/// C++: `void (const std::vector<MSEdge *> &, std::vector<MSEdge *> &) getWalking`
// void getWalking(const std::vector<MSEdge*>& edges, std::vector< MSEdge*>& walkingEdges) {
//     for (std::vector<MSEdge*>::const_iterator it = edges.begin(); it != edges.end(); ++it) {
//         MSEdge* edge = *it;
//         if (edge->isWalkingArea() || ((edge->getPermissions() & SVC_PEDESTRIAN) != 0)) {
//             walkingEdges.push_back(edge);
//         }
//     }
// }
#[allow(unused_variables)]
pub fn get_walking(edges: &Vec<Ptr<MSEdge>>, walkingEdges: &mut Vec<Ptr<MSEdge>>) {
    for &edge in edges.iter() {
        if edge.is_walking_area() || (edge.get_permissions() & SVC_PEDESTRIAN) != 0 {
            walkingEdges.push(edge);
        }
    }
}

/// C++: `const std::vector<MSEdge *> (const MSEdge *) getWalkingAreas`
// const std::vector<MSEdge*> getWalkingAreas(const MSEdge* crossing) {
//     std::vector<MSEdge*> walkingEdges;
//     getWalking(crossing->getSuccessors(), walkingEdges);
//     getWalking(crossing->getPredecessors(), walkingEdges);
//     return walkingEdges;
//
// }
#[allow(unused_variables)]
pub fn get_walking_areas(crossing: Ptr<MSEdge>) -> Vec<Ptr<MSEdge>> {
    let mut walkingEdges: Vec<Ptr<MSEdge>> = Vec::new();
    get_walking(crossing.get_successors(SVC_IGNORING), &mut walkingEdges);
    get_walking(crossing.get_predecessors(), &mut walkingEdges);
    walkingEdges
}

