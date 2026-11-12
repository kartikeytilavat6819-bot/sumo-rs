//! Port of netbuild/NBOwnTLDef.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

// C++ std::string::operator[] semantics for reads past the end: '\0'
fn byte_at(s: &[u8], i: usize) -> u8 {
    s.get(i).copied().unwrap_or(0)
}
fn link_state_at(s: &[u8], i: i32) -> LinkState {
    if i < 0 { 0 } else { byte_at(s, i as usize) as LinkState }
}
// logic->addStep(duration, state, minDur, maxDur, earliestEnd, latestEnd) (6-argument C++ overload)
fn add_step_6(logic: Ptr<NBTrafficLightLogic>, duration: SUMOTime, state: &str, minDur: SUMOTime, maxDur: SUMOTime, earliestEnd: SUMOTime, latestEnd: SUMOTime) {
    let u = NBTrafficLightDefinition_UNSPECIFIED_DURATION;
    logic.m().add_step_2(duration, state, minDur, maxDur, earliestEnd, latestEnd, u, u, u, "", &Vec::new(), -1);
}

/// A traffic light logics which must be computed (only nodes/edges are given)
/// C++ class `NBOwnTLDef` (netbuild/NBOwnTLDef.h)
pub struct NBOwnTLDef {
    /// C++ base class `NBTrafficLightDefinition`
    pub base: NBTrafficLightDefinition,
    /// Whether left-mover should not have an additional phase
    pub myHaveSinglePhase: bool,
    /// the layout for generated signal plans
    pub myLayout: TrafficLightLayout,
}

impl std::ops::Deref for NBOwnTLDef {
    type Target = NBTrafficLightDefinition;
    fn deref(&self) -> &NBTrafficLightDefinition {
        &self.base
    }
}
impl std::ops::DerefMut for NBOwnTLDef {
    fn deref_mut(&mut self) -> &mut NBTrafficLightDefinition {
        &mut self.base
    }
}

// C++ static members of NBOwnTLDef (port as module-level statics, see AGENTS.md):
//   static const double NBOwnTLDef::MIN_SPEED_CROSSING_TIME;

impl NBOwnTLDef {
    /// C++: `void NBOwnTLDef::NBOwnTLDef(const std::string &, const std::vector<NBNode *> &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBOwnTLDef::NBOwnTLDef(const std::string& id,
    //                        const std::vector<NBNode*>& junctions, SUMOTime offset,
    //                        TrafficLightType type) :
    //     NBTrafficLightDefinition(id, junctions, DefaultProgramID, offset, type),
    //     myHaveSinglePhase(false),
    //     myLayout(TrafficLightLayout::DEFAULT) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, junctions: &Vec<Ptr<NBNode>>, offset: SUMOTime, type_: TrafficLightType) -> Self {
        Self {
            base: NBTrafficLightDefinition::new(id, junctions, NBTrafficLightDefinition_DefaultProgramID, offset, type_),
            myHaveSinglePhase: false,
            myLayout: TrafficLightLayout::DEFAULT,
        }
    }

    /// C++: `void NBOwnTLDef::NBOwnTLDef(const std::string &, NBNode *, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBOwnTLDef::NBOwnTLDef(const std::string& id, NBNode* junction, SUMOTime offset,
    //                        TrafficLightType type) :
    //     NBTrafficLightDefinition(id, junction, DefaultProgramID, offset, type),
    //     myHaveSinglePhase(false),
    //     myLayout(TrafficLightLayout::DEFAULT) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, junction: Ptr<NBNode>, offset: SUMOTime, type_: TrafficLightType) -> Self {
        Self {
            base: NBTrafficLightDefinition::new_2(id, junction, NBTrafficLightDefinition_DefaultProgramID, offset, type_),
            myHaveSinglePhase: false,
            myLayout: TrafficLightLayout::DEFAULT,
        }
    }

    /// C++: `void NBOwnTLDef::NBOwnTLDef(const std::string &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBOwnTLDef::NBOwnTLDef(const std::string& id, SUMOTime offset,
    //                        TrafficLightType type) :
    //     NBTrafficLightDefinition(id, DefaultProgramID, offset, type),
    //     myHaveSinglePhase(false),
    //     myLayout(TrafficLightLayout::DEFAULT) {
    // }
    #[allow(unused_variables)]
    pub fn new_3(id: &str, offset: SUMOTime, type_: TrafficLightType) -> Self {
        Self {
            base: NBTrafficLightDefinition::new_3(id, NBTrafficLightDefinition_DefaultProgramID, offset, type_),
            myHaveSinglePhase: false,
            myLayout: TrafficLightLayout::DEFAULT,
        }
    }

    /// C++: `void NBOwnTLDef::~NBOwnTLDef() noexcept` [virtual]
    /// Destructor
    // NBOwnTLDef::~NBOwnTLDef() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; members are released by Rust drop glue
    }

    /// C++: `void NBOwnTLDef::remapRemoved(NBEdge *, const EdgeVector &, const EdgeVector &)` [virtual]
    /// Replaces occurrences of the removed edge in incoming/outgoing edges of all definitions
    // void
    // NBOwnTLDef::remapRemoved(NBEdge* /*removed*/, const EdgeVector& /*incoming*/,
    //                          const EdgeVector& /*outgoing*/) {}
    #[allow(unused_variables)]
    pub fn remap_removed(&mut self, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
        // C++ body is empty (the base definition's behaviour applies, which is also empty for NBOwnTLDef)
    }

    /// C++: `void NBOwnTLDef::setTLControllingInformation() const` [const, virtual]
    /// Informs edges about being controlled by a tls
    // void
    // NBOwnTLDef::setTLControllingInformation() const {
    //     // set the information about the link's positions within the tl into the
    //     //  edges the links are starting at, respectively
    //     for (NBConnectionVector::const_iterator j = myControlledLinks.begin(); j != myControlledLinks.end(); ++j) {
    //         const NBConnection& conn = *j;
    //         NBEdge* edge = conn.getFrom();
    //         edge->setControllingTLInformation(conn, getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_tl_controlling_information(&self) {
        // set the information about the link's positions within the tl into the
        //  edges the links are starting at, respectively
        for conn in self.myControlledLinks.iter() {
            let edge = conn.get_from();
            edge.m().set_controlling_tl_information(conn, self.get_id());
        }
    }

    /// C++: `void NBOwnTLDef::setSinglePhase()`
    /// Forces the definition not to compute an additional phase for left-movers
    // void setSinglePhase() {
    //         myHaveSinglePhase = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_single_phase(&mut self) {
        self.myHaveSinglePhase = true;
    }

    /// C++: `void NBOwnTLDef::addGreenWithin(NBTrafficLightLogic *, const EdgeVector &, EdgeVector &)` [static]
    /// ensure inner edges all get the green light eventually
    // void
    // NBOwnTLDef::addGreenWithin(NBTrafficLightLogic* logic, const EdgeVector& fromEdges, EdgeVector& toProc) {
    //     std::vector<bool> foundGreen(fromEdges.size(), false);
    //     for (const auto& phase : logic->getPhases()) {
    //         const std::string state = phase.state;
    //         for (int j = 0; j < (int)fromEdges.size(); j++) {
    //             LinkState ls = (LinkState)state[j];
    //             if (ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR) {
    //                 foundGreen[j] = true;
    //             }
    //         }
    //     }
    //     for (int j = 0; j < (int)foundGreen.size(); j++) {
    //         if (!foundGreen[j]) {
    //             NBEdge* e = fromEdges[j];
    //             if (std::find(toProc.begin(), toProc.end(), e) == toProc.end()) {
    //                 toProc.push_back(e);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_green_within(logic: Ptr<NBTrafficLightLogic>, fromEdges: &EdgeVector, toProc: &mut EdgeVector) {
        let mut foundGreen: Vec<bool> = vec![false; fromEdges.len()];
        for phase in logic.get_phases().iter() {
            let state = phase.state.as_bytes();
            for j in 0..fromEdges.len() {
                let ls = link_state_at(state, j as i32);
                if ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR {
                    foundGreen[j] = true;
                }
            }
        }
        for j in 0..foundGreen.len() {
            if !foundGreen[j] {
                let e = fromEdges[j];
                if !toProc.contains(&e) {
                    toProc.push(e);
                }
            }
        }
    }

    /// C++: `void NBOwnTLDef::addPedestrianScramble(NBTrafficLightLogic *, int, SUMOTime, SUMOTime, const std::vector<NBNode::Crossing *> &, const EdgeVector &, const EdgeVector &)` [static]
    /// add an additional pedestrian phase if there are crossings that did not get green yet
    // void
    // NBOwnTLDef::addPedestrianScramble(NBTrafficLightLogic* logic, int totalNumLinks, SUMOTime /* greenTime */, SUMOTime brakingTime,
    //                                   const std::vector<NBNode::Crossing*>& crossings, const EdgeVector& fromEdges, const EdgeVector& toEdges) {
    //     // check both indices for each crossing (they may have green in different phases)
    //     std::vector<bool> foundGreen(crossings.size() * 2, false);
    //     const std::vector<NBTrafficLightLogic::PhaseDefinition>& phases = logic->getPhases();
    //     for (int i = 0; i < (int)phases.size(); i++) {
    //         const std::string state = phases[i].state;
    //         int j = 0;
    //         for (auto cross : crossings) {
    //             LinkState ls = (LinkState)state[cross->tlLinkIndex];
    //             LinkState ls2 = cross->tlLinkIndex2 >= 0 ? (LinkState)state[cross->tlLinkIndex2] : ls;
    //             if (ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR) {
    //                 foundGreen[j] = true;
    //             }
    //             if (ls2 == LINKSTATE_TL_GREEN_MAJOR || ls2 == LINKSTATE_TL_GREEN_MINOR) {
    //                 foundGreen[j + crossings.size()] = true;
    //             }
    //             j++;
    //         }
    //     }
    // #ifdef DEBUG_PHASES
    //     if (DEBUGCOND2(logic)) {
    //         std::cout << " foundCrossingGreen=" << toString(foundGreen) << "\n";
    //     }
    // #endif
    //     for (int j = 0; j < (int)foundGreen.size(); j++) {
    //         if (!foundGreen[j]) {
    //             // add a phase where all pedestrians may walk, (preceded by a yellow phase and followed by a clearing phase)
    //             if (phases.size() > 0) {
    //                 bool needYellowPhase = false;
    //                 std::string state = phases.back().state;
    //                 for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //                     if (state[i1] == 'G' || state[i1] == 'g') {
    //                         state[i1] = 'y';
    //                         needYellowPhase = true;
    //                     }
    //                 }
    //                 // add yellow step
    //                 if (needYellowPhase && brakingTime > 0) {
    //                     logic->addStep(brakingTime, state);
    //                 }
    //             }
    //             const SUMOTime pedClearingTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.crossing-clearance.time"));
    //             const SUMOTime scrambleTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.scramble.time"));
    //             addPedestrianPhases(logic, scrambleTime + pedClearingTime, UNSPECIFIED_DURATION,
    //                                 UNSPECIFIED_DURATION, UNSPECIFIED_DURATION, UNSPECIFIED_DURATION, std::string(totalNumLinks, 'r'), crossings, fromEdges, toEdges);
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_pedestrian_scramble(logic: Ptr<NBTrafficLightLogic>, totalNumLinks: i32, greenTime: SUMOTime, yellowTime: SUMOTime, crossings: &Vec<Ptr<NBNode_Crossing>>, fromEdges: &EdgeVector, toEdges: &EdgeVector) {
        let brakingTime = yellowTime;
        // check both indices for each crossing (they may have green in different phases)
        let mut foundGreen: Vec<bool> = vec![false; crossings.len() * 2];
        let phaseStates: Vec<String> = logic.get_phases().iter().map(|p| p.state.clone()).collect();
        for i in 0..phaseStates.len() {
            let state = phaseStates[i].as_bytes();
            let mut j: usize = 0;
            for cross in crossings.iter() {
                let ls = link_state_at(state, cross.tlLinkIndex);
                let ls2 = if cross.tlLinkIndex2 >= 0 { link_state_at(state, cross.tlLinkIndex2) } else { ls };
                if ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR {
                    foundGreen[j] = true;
                }
                if ls2 == LINKSTATE_TL_GREEN_MAJOR || ls2 == LINKSTATE_TL_GREEN_MINOR {
                    foundGreen[j + crossings.len()] = true;
                }
                j += 1;
            }
        }
        for j in 0..foundGreen.len() {
            if !foundGreen[j] {
                // add a phase where all pedestrians may walk, (preceded by a yellow phase and followed by a clearing phase)
                if phaseStates.len() > 0 {
                    let mut needYellowPhase = false;
                    let mut state: Vec<u8> = phaseStates.last().unwrap().clone().into_bytes();
                    for i1 in 0..fromEdges.len() {
                        if i1 < state.len() && (state[i1] == b'G' || state[i1] == b'g') {
                            state[i1] = b'y';
                            needYellowPhase = true;
                        }
                    }
                    let state = String::from_utf8(state).unwrap();
                    // add yellow step
                    if needYellowPhase && brakingTime > 0 {
                        logic.m().add_step(brakingTime, &state, &Vec::new(), "", -1);
                    }
                }
                let pedClearingTime = time2steps(utils::options::options_cont::get_int("tls.crossing-clearance.time") as f64);
                let scrambleTime = time2steps(utils::options::options_cont::get_int("tls.scramble.time") as f64);
                let u = NBTrafficLightDefinition_UNSPECIFIED_DURATION;
                Self::add_pedestrian_phases(logic, scrambleTime + pedClearingTime, u, u, u, u, "r".repeat(totalNumLinks.max(0) as usize), crossings, fromEdges, toEdges);
                break;
            }
        }
    }

    /// C++: `std::string NBOwnTLDef::addPedestrianPhases(NBTrafficLightLogic *, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, std::string, const std::vector<NBNode::Crossing *> &, const EdgeVector &, const EdgeVector &)` [static]
    /// add 1 or 2 phases depending on the presence of pedestrian crossings
    // std::string
    // NBOwnTLDef::addPedestrianPhases(NBTrafficLightLogic* logic, const SUMOTime greenTime, const SUMOTime minDur, const SUMOTime maxDur,
    //                                 const SUMOTime earliestEnd, const SUMOTime latestEnd,
    //                                 std::string state, const std::vector<NBNode::Crossing*>& crossings, const EdgeVector& fromEdges, const EdgeVector& toEdges) {
    //     // compute based on length of the crossing if not set by the user
    //     const SUMOTime pedClearingTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.crossing-clearance.time"));
    //     // compute if not set by user: must be able to reach the middle of the second "Richtungsfahrbahn"
    //     const SUMOTime minPedTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.crossing-min.time"));
    //     const std::string orig = state;
    //     state = patchStateForCrossings(state, crossings, fromEdges, toEdges);
    //     if (orig == state) {
    //         // add step
    //         logic->addStep(greenTime, state, minDur, maxDur, earliestEnd, latestEnd);
    //     } else {
    //         const SUMOTime pedTime = greenTime - pedClearingTime;
    //         if (pedTime >= minPedTime) {
    //             // ensure clearing time for pedestrians
    //             const bool isSimpleActuatedCrossing = logic->getType() == TrafficLightType::ACTUATED
    //                                                   && minDur == UNSPECIFIED_DURATION && logic->getPhases().size() == 2;
    //             if (isSimpleActuatedCrossing) {
    //                 // permit green phase to extend when there are no pedestrians
    //                 logic->setPhaseNext(0, {0, 1});
    //             }
    //             logic->addStep(pedTime, state, minDur, maxDur, earliestEnd, latestEnd);
    // #ifdef DEBUG_PHASES
    //             if (DEBUGCOND2(logic)) {
    //                 std::cout << " intermidate state for addPedestrianPhases " << state << "\n";
    //             }
    // #endif
    //             for (auto cross : crossings) {
    //                 if (cross->tlLinkIndex >= (int)fromEdges.size() || fromEdges[cross->tlLinkIndex] == nullptr) {
    //                     state[cross->tlLinkIndex] = 'r';
    //                 }
    //                 if (cross->tlLinkIndex2 >= 0 && (cross->tlLinkIndex2 >= (int)fromEdges.size() || fromEdges[cross->tlLinkIndex2] == nullptr)) {
    //                     state[cross->tlLinkIndex2] = 'r';
    //                 }
    //             }
    //             logic->addStep(pedClearingTime, state);
    //         } else {
    //             state = orig;
    //             // not safe for pedestrians.
    //             logic->addStep(greenTime, state, minDur, maxDur, earliestEnd, latestEnd);
    //         }
    //     }
    // #ifdef DEBUG_PHASES
    //     if (DEBUGCOND2(logic)) {
    //         std::cout << " state after addPedestrianPhases " << state << "\n";
    //     }
    // #endif
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn add_pedestrian_phases(logic: Ptr<NBTrafficLightLogic>, greenTime: SUMOTime, minDur: SUMOTime, maxDur: SUMOTime, earliestEnd: SUMOTime, latestEnd: SUMOTime, state: String, crossings: &Vec<Ptr<NBNode_Crossing>>, fromEdges: &EdgeVector, toEdges: &EdgeVector) -> String {
        // compute based on length of the crossing if not set by the user
        let pedClearingTime = time2steps(utils::options::options_cont::get_int("tls.crossing-clearance.time") as f64);
        // compute if not set by user: must be able to reach the middle of the second "Richtungsfahrbahn"
        let minPedTime = time2steps(utils::options::options_cont::get_int("tls.crossing-min.time") as f64);
        let orig = state.clone();
        let mut state = Self::patch_state_for_crossings(&state, crossings, fromEdges, toEdges);
        if orig == state {
            // add step
            add_step_6(logic, greenTime, &state, minDur, maxDur, earliestEnd, latestEnd);
        } else {
            let pedTime = greenTime - pedClearingTime;
            if pedTime >= minPedTime {
                // ensure clearing time for pedestrians
                let isSimpleActuatedCrossing = logic.get_type() == TrafficLightType::ACTUATED
                    && minDur == NBTrafficLightDefinition_UNSPECIFIED_DURATION && logic.get_phases().len() == 2;
                if isSimpleActuatedCrossing {
                    // permit green phase to extend when there are no pedestrians
                    logic.m().set_phase_next(0, &vec![0, 1]);
                }
                add_step_6(logic, pedTime, &state, minDur, maxDur, earliestEnd, latestEnd);
                let mut bytes: Vec<u8> = state.into_bytes();
                for cross in crossings.iter() {
                    if cross.tlLinkIndex >= fromEdges.len() as i32 || fromEdges[cross.tlLinkIndex as usize].is_null() {
                        bytes[cross.tlLinkIndex as usize] = b'r';
                    }
                    if cross.tlLinkIndex2 >= 0
                        && (cross.tlLinkIndex2 >= fromEdges.len() as i32 || fromEdges[cross.tlLinkIndex2 as usize].is_null())
                    {
                        bytes[cross.tlLinkIndex2 as usize] = b'r';
                    }
                }
                state = String::from_utf8(bytes).unwrap();
                logic.m().add_step(pedClearingTime, &state, &Vec::new(), "", -1);
            } else {
                state = orig;
                // not safe for pedestrians.
                add_step_6(logic, greenTime, &state, minDur, maxDur, earliestEnd, latestEnd);
            }
        }
        state
    }

    /// C++: `std::string NBOwnTLDef::patchStateForCrossings(const std::string &, const std::vector<NBNode::Crossing *> &, const EdgeVector &, const EdgeVector &)` [static]
    /// compute phase state in regard to pedestrian crossings
    // std::string
    // NBOwnTLDef::patchStateForCrossings(const std::string& state, const std::vector<NBNode::Crossing*>& crossings, const EdgeVector& fromEdges, const EdgeVector& toEdges) {
    //     std::string result = state;
    //     for (const NBNode::Crossing* cross : crossings) {
    //         bool isForbidden = false;
    //         for (int i2 = 0; i2 < (int)fromEdges.size() && !isForbidden; ++i2) {
    //             // only check connections at this crossings node
    //             if (fromEdges[i2] != 0 && toEdges[i2] != 0 && fromEdges[i2]->getToNode() == cross->node) {
    //                 for (EdgeVector::const_iterator it = cross->edges.begin(); it != cross->edges.end(); ++it) {
    //                     const NBEdge* edge = *it;
    //                     const LinkDirection i2dir = cross->node->getDirection(fromEdges[i2], toEdges[i2]);
    //                     if (state[i2] != 'r' && state[i2] != 's' && (edge == fromEdges[i2] ||
    //                             (edge == toEdges[i2] && (i2dir == LinkDirection::STRAIGHT || i2dir == LinkDirection::PARTLEFT || i2dir == LinkDirection::PARTRIGHT)))) {
    //                         isForbidden = true;
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         const int i1 = cross->tlLinkIndex;
    //         assert(i1 >= 0 && i1 < (int)result.size());
    //         const char newState = isForbidden ? 'r' : 'G';
    //         if (i1 < (int)toEdges.size() && toEdges[i1] != nullptr && (result[i1] != newState || !isForbidden)) {
    //             if (cross->tlID != DummyID) {
    //                 WRITE_WARNINGF(TL("Custom crossing linkIndex % conflicts with vehicular connections at tlLogic '%'"), i1, cross->tlID);
    //             }
    //         } else {
    //             result[i1] = newState;
    //         }
    //         if (cross->tlLinkIndex2 >= 0) {
    //             const int i2 = cross->tlLinkIndex2;
    //             if (i2 < (int)toEdges.size() && toEdges[i2] != nullptr && (result[i2] != newState || !isForbidden)) {
    //                 if (cross->tlID != DummyID) {
    //                     WRITE_WARNINGF(TL("Custom crossing linkIndex2 % conflicts with vehicular connections at tlLogic '%'"), i2, cross->tlID);
    //                 }
    //             } else {
    //                 result[i2] = newState;
    //             }
    //         }
    //     }
    //
    //     // correct behaviour for roads that are in conflict with a pedestrian crossing
    //     assert(fromEdges.size() <= result.size());
    //     for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //         if (result[i1] == 'G') {
    //             for (const NBNode::Crossing* cross : crossings) {
    //                 const int i2 = cross->tlLinkIndex;
    //                 const int i3 = cross->tlLinkIndex2;
    //                 if (fromEdges[i1] != 0 && toEdges[i1] != 0 && fromEdges[i1]->getToNode() == cross->node) {
    //                     if ((result[i2] == 'G' || (i3 >= 0 && result[i3] == 'G'))
    //                             && cross->node->mustBrakeForCrossing(fromEdges[i1], toEdges[i1], *cross)) {
    //                         result[i1] = 'g';
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn patch_state_for_crossings(state: &str, crossings: &Vec<Ptr<NBNode_Crossing>>, fromEdges: &EdgeVector, toEdges: &EdgeVector) -> String {
        let sb = state.as_bytes();
        let mut result: Vec<u8> = sb.to_vec();
        for cross in crossings.iter() {
            let mut isForbidden = false;
            let mut i2: usize = 0;
            while i2 < fromEdges.len() && !isForbidden {
                // only check connections at this crossings node
                if !fromEdges[i2].is_null() && !toEdges[i2].is_null() && fromEdges[i2].get_to_node() == cross.node {
                    for edge in cross.edges.iter() {
                        let i2dir = cross.node.get_direction(fromEdges[i2], toEdges[i2], false);
                        if byte_at(sb, i2) != b'r' && byte_at(sb, i2) != b's'
                            && (*edge == fromEdges[i2]
                                || (*edge == toEdges[i2]
                                    && (i2dir == LinkDirection::STRAIGHT || i2dir == LinkDirection::PARTLEFT || i2dir == LinkDirection::PARTRIGHT)))
                        {
                            isForbidden = true;
                            break;
                        }
                    }
                }
                i2 += 1;
            }
            let i1 = cross.tlLinkIndex;
            assert!(i1 >= 0 && (i1 as usize) < result.len());
            let newState: u8 = if isForbidden { b'r' } else { b'G' };
            if (i1 as usize) < toEdges.len() && !toEdges[i1 as usize].is_null() && (result[i1 as usize] != newState || !isForbidden) {
                if cross.tlID != NBTrafficLightDefinition_DummyID {
                    write_warningf!("Custom crossing linkIndex % conflicts with vehicular connections at tlLogic '%'", i1, cross.tlID);
                }
            } else {
                result[i1 as usize] = newState;
            }
            if cross.tlLinkIndex2 >= 0 {
                let i2 = cross.tlLinkIndex2;
                if (i2 as usize) < toEdges.len() && !toEdges[i2 as usize].is_null() && (result[i2 as usize] != newState || !isForbidden) {
                    if cross.tlID != NBTrafficLightDefinition_DummyID {
                        write_warningf!("Custom crossing linkIndex2 % conflicts with vehicular connections at tlLogic '%'", i2, cross.tlID);
                    }
                } else {
                    result[i2 as usize] = newState;
                }
            }
        }

        // correct behaviour for roads that are in conflict with a pedestrian crossing
        assert!(fromEdges.len() <= result.len());
        for i1 in 0..fromEdges.len() {
            if result[i1] == b'G' {
                for cross in crossings.iter() {
                    let i2 = cross.tlLinkIndex;
                    let i3 = cross.tlLinkIndex2;
                    if !fromEdges[i1].is_null() && !toEdges[i1].is_null() && fromEdges[i1].get_to_node() == cross.node {
                        if (byte_at(&result, i2 as usize) == b'G' || (i3 >= 0 && byte_at(&result, i3 as usize) == b'G'))
                            && cross.node.must_brake_for_crossing(fromEdges[i1], toEdges[i1], cross)
                        {
                            result[i1] = b'g';
                            break;
                        }
                    }
                }
            }
        }
        String::from_utf8(result).unwrap()
    }

    /// C++: `std::string NBOwnTLDef::patchNEMAStateForCrossings(const std::string &, const std::vector<NBNode::Crossing *> &, const EdgeVector &, const EdgeVector &, const NBEdge *, NBEdge *)` [static]
    // std::string
    // NBOwnTLDef::patchNEMAStateForCrossings(const std::string& state,
    //                                        const std::vector<NBNode::Crossing*>& crossings,
    //                                        const EdgeVector& fromEdges,
    //                                        const EdgeVector& toEdges,
    //                                        const NBEdge* greenEdge, NBEdge* otherChosen) {
    //     std::string result = state;
    //     const int pos = (int)(state.size() - crossings.size()); // number of controlled vehicle links
    //     const EdgeVector& all = greenEdge->getToNode()->getEdges();
    //     EdgeVector::const_iterator start = std::find(all.begin(), all.end(), greenEdge);
    //
    //     // permit crossings over edges between the current green edge and it's straight continuation
    //     const NBEdge* endEdge = nullptr;
    //     for (int i = 0; i < (int)state.size(); i++) {
    //         if (state[i] == 'G' && fromEdges[i] == greenEdge
    //                 && greenEdge->getToNode()->getDirection(greenEdge, toEdges[i]) == LinkDirection::STRAIGHT) {
    //             // straight edge found
    //             endEdge = toEdges[i];
    //             break;
    //         }
    //     }
    //     if (endEdge == nullptr) {
    //         endEdge = otherChosen;
    //     }
    //     if (endEdge == nullptr) {
    //         // try to find the reverse edge of the green edge
    //         auto itCW = start;
    //         NBContHelper::nextCW(all, itCW);
    //         if ((*itCW)->getFromNode() == greenEdge->getToNode()) {
    //             endEdge = *itCW;
    //         }
    //     }
    //     if (endEdge == nullptr) {
    //         // at least prevent an infinite loop
    //         endEdge = greenEdge;
    //     }
    //     //std::cout << " patchNEMAStateForCrossings green=" << greenEdge->getID() << " other=" << Named::getIDSecure(otherChosen) << " end=" << Named::getIDSecure(end) << " all=" << toString(all) << "\n";
    //
    //     EdgeVector::const_iterator end = std::find(all.begin(), all.end(), endEdge);
    //     if (end == all.end()) {
    //         // at least prevent an infinite loop
    //         end = start;
    //     }
    //     auto it = start;
    //     NBContHelper::nextCCW(all, it);
    //     for (; it != end; NBContHelper::nextCCW(all, it)) {
    //         for (int ic = 0; ic < (int)crossings.size(); ++ic) {
    //             const int i1 = pos + ic;
    //             const NBNode::Crossing& cross = *crossings[ic];
    //             for (const NBEdge* crossed : cross.edges) {
    //                 //std::cout << "   cand=" << (*it)->getID() << " crossed=" << crossed->getID() << "\n";
    //                 if (crossed == *it) {
    //                     result[i1] = 'G';
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     // correct behaviour for roads that are in conflict with a pedestrian crossing
    //     for (int i1 = 0; i1 < pos; ++i1) {
    //         if (result[i1] == 'G') {
    //             for (int ic = 0; ic < (int)crossings.size(); ++ic) {
    //                 const NBNode::Crossing& crossing = *crossings[ic];
    //                 const int i2 = pos + ic;
    //                 if (result[i2] == 'G' && crossing.node->mustBrakeForCrossing(fromEdges[i1], toEdges[i1], crossing)) {
    //                     result[i1] = 'g';
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn patch_nema_state_for_crossings(state: &str, crossings: &Vec<Ptr<NBNode_Crossing>>, fromEdges: &EdgeVector, toEdges: &EdgeVector, greenEdge: Ptr<NBEdge>, otherChosen: Ptr<NBEdge>) -> String {
        let sb = state.as_bytes();
        let mut result: Vec<u8> = sb.to_vec();
        let pos: i32 = state.len() as i32 - crossings.len() as i32; // number of controlled vehicle links
        let toNode = greenEdge.get_to_node();
        // iterators over the node's edges are modelled as indices (len() stands for end())
        let all: EdgeVector = toNode.get_edges().clone();
        let len = all.len();
        let start = all.iter().position(|e| *e == greenEdge).unwrap_or(len);

        // permit crossings over edges between the current green edge and it's straight continuation
        let mut endEdge: Ptr<NBEdge> = Ptr::default();
        for i in 0..state.len() {
            if byte_at(sb, i) == b'G' && i < fromEdges.len() && i < toEdges.len() && fromEdges[i] == greenEdge
                && toNode.get_direction(greenEdge, toEdges[i], false) == LinkDirection::STRAIGHT
            {
                // straight edge found
                endEdge = toEdges[i];
                break;
            }
        }
        if endEdge.is_null() {
            endEdge = otherChosen;
        }
        if endEdge.is_null() {
            // try to find the reverse edge of the green edge
            let mut itCW = start;
            // NBContHelper::nextCW
            itCW += 1;
            if itCW == len {
                itCW = 0;
            }
            if itCW < len && all[itCW].get_from_node() == toNode {
                endEdge = all[itCW];
            }
        }
        if endEdge.is_null() {
            // at least prevent an infinite loop
            endEdge = greenEdge;
        }

        let mut end = all.iter().position(|e| *e == endEdge).unwrap_or(len);
        if end == len {
            // at least prevent an infinite loop
            end = start;
        }
        // NBContHelper::nextCCW(all, it) on index it
        let next_ccw = |it: usize| -> usize { if it == 0 { len.wrapping_sub(1) } else { it - 1 } };
        let mut it = next_ccw(start);
        while it != end {
            for ic in 0..crossings.len() {
                let i1 = (pos + ic as i32) as usize;
                let cross = &crossings[ic];
                for crossed in cross.edges.iter() {
                    if *crossed == all[it] {
                        result[i1] = b'G';
                        break;
                    }
                }
            }
            it = next_ccw(it);
        }
        // correct behaviour for roads that are in conflict with a pedestrian crossing
        for i1 in 0..(pos.max(0) as usize) {
            if result[i1] == b'G' {
                for ic in 0..crossings.len() {
                    let crossing = &crossings[ic];
                    let i2 = (pos + ic as i32) as usize;
                    if byte_at(&result, i2) == b'G' && crossing.node.must_brake_for_crossing(fromEdges[i1], toEdges[i1], crossing) {
                        result[i1] = b'g';
                        break;
                    }
                }
            }
        }
        String::from_utf8(result).unwrap()
    }

    /// C++: `NBTrafficLightLogic * NBOwnTLDef::computeLogicAndConts(int, bool)`
    /// helper function for myCompute
    // NBTrafficLightLogic*
    // NBOwnTLDef::computeLogicAndConts(int brakingTimeSeconds, bool onlyConts) {
    //     if (myControlledNodes.size() == 1) {
    //         // otherwise, use values from previous call to initNeedsContRelation
    //         myNeedsContRelation.clear();
    //     }
    //     myExtraConflicts.clear();
    //     const bool isNEMA = myType == TrafficLightType::NEMA;
    //     const SUMOTime brakingTime = TIME2STEPS(brakingTimeSeconds);
    //     const SUMOTime leftTurnTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.left-green.time"));
    //     const SUMOTime minMinDur = (myType == TrafficLightType::STATIC) ? UNSPECIFIED_DURATION : TIME2STEPS(OptionsCont::getOptions().getInt("tls.min-dur"));
    //     const SUMOTime maxDur = (myType == TrafficLightType::STATIC) ? UNSPECIFIED_DURATION : TIME2STEPS(OptionsCont::getOptions().getInt("tls.max-dur"));
    //     const SUMOTime earliestEnd = UNSPECIFIED_DURATION;
    //     const SUMOTime latestEnd = UNSPECIFIED_DURATION;
    //     const SUMOTime greenTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.green.time"));
    //     SUMOTime allRedTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.allred.time"));
    //     const double minorLeftSpeedThreshold = OptionsCont::getOptions().getFloat("tls.minor-left.max-speed");
    //     const bool noMixed = OptionsCont::getOptions().getBool("tls.no-mixed");
    //     // left-turn phases do not work well for joined tls, so we build incoming instead
    //     if (myLayout == TrafficLightLayout::DEFAULT) {
    //         // @note this prevents updating after loading plain-xml into netedit computing tls and then changing the default layout
    //         myLayout = SUMOXMLDefinitions::TrafficLightLayouts.get(OptionsCont::getOptions().getString("tls.layout"));
    //     }
    //     // corridorLike() resets crossing indices so should be called first
    //     const bool groupOpposites = (myLayout == TrafficLightLayout::OPPOSITES && (myControlledNodes.size() <= 2 || corridorLike()));
    //
    //     // things collect for NEMA phase building
    //     std::vector<std::pair<NBEdge*, NBEdge*> > chosenList;
    //     std::vector<std::string> straightStates;
    //     std::vector<std::string> leftStates;
    //
    //     // build complete lists first
    //     const EdgeVector& incoming = getIncomingEdges();
    //     EdgeVector fromEdges, toEdges;
    //     std::vector<bool> isTurnaround;
    //     std::vector<bool> hasTurnLane;
    //     std::vector<int> fromLanes;
    //     std::vector<int> toLanes;
    //     std::vector<SUMOTime> crossingTime;
    //     int totalNumLinks = 0;
    //     for (NBEdge* const fromEdge : incoming) {
    //         const int numLanes = fromEdge->getNumLanes();
    //         const bool edgeHasStraight = hasStraightConnection(fromEdge);
    //         for (int i2 = 0; i2 < numLanes; i2++) {
    //             bool hasLeft = false;
    //             bool hasPartLeft = false;
    //             bool hasStraight = false;
    //             bool hasRight = false;
    //             bool hasTurnaround = false;
    //             for (const NBEdge::Connection& approached : fromEdge->getConnectionsFromLane(i2)) {
    //                 if (!fromEdge->mayBeTLSControlled(i2, approached.toEdge, approached.toLane)) {
    //                     continue;
    //                 }
    //                 fromEdges.push_back(fromEdge);
    //                 fromLanes.push_back(i2);
    //                 toLanes.push_back(approached.toLane);
    //                 toEdges.push_back(approached.toEdge);
    //                 if (approached.vmax < NUMERICAL_EPS || (fromEdge->getPermissions() & SVC_PASSENGER) == 0
    //                         || (approached.toEdge->getPermissions() & SVC_PASSENGER) == 0) {
    //                     crossingTime.push_back(0);
    //                 } else {
    //                     crossingTime.push_back(TIME2STEPS((approached.length + approached.viaLength) / MAX2(approached.vmax, MIN_SPEED_CROSSING_TIME)));
    //                 }
    //                 // std::cout << fromEdge->getID() << " " << approached.toEdge->getID() << " " << (fromEdge->getPermissions() & SVC_PASSENGER) << " " << approached.length << " " << approached.viaLength << " " << approached.vmax << " " << crossingTime.back() << std::endl;
    //                 if (approached.toEdge != nullptr) {
    //                     isTurnaround.push_back(fromEdge->isTurningDirectionAt(approached.toEdge));
    //                 } else {
    //                     isTurnaround.push_back(true);
    //                 }
    //                 LinkDirection dir = fromEdge->getToNode()->getDirection(fromEdge, approached.toEdge);
    //                 if (dir == LinkDirection::STRAIGHT) {
    //                     hasStraight = true;
    //                 } else if (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT) {
    //                     hasRight = true;
    //                 } else if (dir == LinkDirection::LEFT) {
    //                     hasLeft = true;
    //                 } else if (dir == LinkDirection::PARTLEFT) {
    //                     hasPartLeft = true;
    //                 } else if (dir == LinkDirection::TURN) {
    //                     hasTurnaround = true;
    //                 }
    //                 totalNumLinks++;
    //             }
    //             for (const NBEdge::Connection& approached : fromEdge->getConnectionsFromLane(i2)) {
    //                 if (!fromEdge->mayBeTLSControlled(i2, approached.toEdge, approached.toLane)) {
    //                     continue;
    //                 }
    //                 hasTurnLane.push_back(
    //                     (hasLeft && !hasPartLeft && !hasStraight && !hasRight)
    //                     || (hasPartLeft && !hasLeft && !hasStraight && !hasRight)
    //                     || (hasPartLeft && hasLeft && edgeHasStraight && !hasRight)
    //                     || (!hasLeft && !hasPartLeft && !hasTurnaround && hasRight));
    //             }
    //             //std::cout << " from=" << fromEdge->getID() << "_" << i2 << " hasTurnLane=" << hasTurnLane.back() << " s=" << hasStraight << " l=" << hasLeft << " r=" << hasRight << " t=" << hasTurnaround << "\n";
    //         }
    //     }
    //     // collect crossings
    //     std::vector<NBNode::Crossing*> crossings;
    //     for (NBNode* const node : myControlledNodes) {
    //         const std::vector<NBNode::Crossing*>& c = node->getCrossings();
    //         node->setCrossingTLIndices(getID(), totalNumLinks, onlyConts);
    //         totalNumLinks = MAX2(totalNumLinks, maxCrossingIndex(node) + 1);
    //         copy(c.begin(), c.end(), std::back_inserter(crossings));
    //     }
    //
    //     NBTrafficLightLogic* logic = new NBTrafficLightLogic(getID(), getProgramID(), totalNumLinks, myOffset, myType);
    //     EdgeVector toProc = getConnectedOuterEdges(incoming);
    //
    //     // build all phases
    //     std::vector<int> greenPhases; // indices of green phases
    //     std::vector<bool> hadGreenMajor(totalNumLinks, false);
    //     while (toProc.size() > 0) {
    //         bool groupTram = false;
    //         bool groupOther = false;
    //         std::pair<NBEdge*, NBEdge*> chosen;
    //         std::set<const NBEdge*> chosenSet;
    //         if (groupOpposites) {
    //             if (incoming.size() == 2) {
    //                 // if there are only 2 incoming edges we need to decide whether they are a crossing or a "continuation"
    //                 // @node: this heuristic could be extended to also check the number of outgoing edges
    //                 double angle = fabs(NBHelpers::relAngle(incoming[0]->getAngleAtNode(incoming[0]->getToNode()), incoming[1]->getAngleAtNode(incoming[1]->getToNode())));
    //                 // angle would be 180 for straight opposing incoming edges
    //                 if (angle < 135) {
    //                     chosen = std::pair<NBEdge*, NBEdge*>(toProc[0], static_cast<NBEdge*>(nullptr));
    //                     toProc.erase(toProc.begin());
    //                 } else {
    //                     chosen = getBestPair(toProc);
    //                 }
    //             } else {
    //                 chosen = getBestPair(toProc);
    //                 if (chosen.second == nullptr && chosen.first->getPermissions() == SVC_TRAM) {
    //                     groupTram = true;
    //                     for (auto it = toProc.begin(); it != toProc.end();) {
    //                         if ((*it)->getPermissions() == SVC_TRAM) {
    //                             it = toProc.erase(it);
    //                         } else {
    //                             it++;
    //                         }
    //                     }
    //                 }
    //             }
    //         } else {
    //             NBEdge* chosenEdge = toProc[0];
    //             chosen = std::pair<NBEdge*, NBEdge*>(chosenEdge, static_cast<NBEdge*>(nullptr));
    //             toProc.erase(toProc.begin());
    //             SVCPermissions perms = chosenEdge->getPermissions();
    //             if (perms == SVC_TRAM) {
    //                 groupTram = true;
    //             } else if ((perms & ~(SVC_PEDESTRIAN | SVC_BICYCLE | SVC_DELIVERY)) == 0) {
    //                 if (OptionsCont::getOptions().getBool("tls.ignore-internal-junction-jam")) {
    //                     // otherwise, we can get a mutual conflict for minor green
    //                     // streams which would create deadlock
    //                     groupOther = true;
    //                 }
    //             }
    //             // group all edges with the same permissions into a single phase (later)
    //             if (groupTram || groupOther) {
    //                 for (auto it = toProc.begin(); it != toProc.end();) {
    //                     if ((*it)->getPermissions() == perms) {
    //                         it = toProc.erase(it);
    //                     } else {
    //                         it++;
    //                     }
    //                 }
    //             }
    //         }
    //         int pos = 0;
    //         std::string state(totalNumLinks, 'r');
    // #ifdef DEBUG_PHASES
    //         if (DEBUGCOND) {
    //             std::cout << " computing " << getID() << " prog=" << getProgramID() << " cho1=" << Named::getIDSecure(chosen.first) << " cho2=" << Named::getIDSecure(chosen.second) << " toProc=" << toString(toProc) << " bentPrio=" << chosen.first->getToNode()->isBentPriority() << "\n";
    //         }
    // #endif
    //         chosenList.push_back(chosen);
    //         chosenSet.insert(chosen.first);
    //         if (chosen.second != nullptr) {
    //             chosenSet.insert(chosen.second);
    //         }
    //         // find parallel bike edge for the chosen (passenger) edges
    //         for (const NBEdge* e : chosenSet) {
    //             if ((e->getPermissions() & SVC_PASSENGER) != 0) {
    //                 std::vector<NBEdge*> parallelBikeEdges;
    //                 for (NBEdge* cand : toProc) {
    //                     if ((cand->getPermissions() & ~SVC_PEDESTRIAN) == SVC_BICYCLE) {
    //                         double angle = fabs(NBHelpers::relAngle(e->getAngleAtNode(e->getToNode()), cand->getAngleAtNode(cand->getToNode())));
    //                         if (angle < 30) {
    //                             // roughly parallel
    //                             parallelBikeEdges.push_back(cand);
    //                         }
    //                     }
    //                 }
    //                 for (NBEdge* be : parallelBikeEdges) {
    // #ifdef DEBUG_PHASES
    //                     if (DEBUGCOND) {
    //                         std::cout << " chosen=" << e->getID() << " be=" << be->getID() << "\n";
    //                     }
    // #endif
    //                     chosenSet.insert(be);
    //                     toProc.erase(std::find(toProc.begin(), toProc.end(), be));
    //                 }
    //             }
    //         }
    //         // plain straight movers
    //         double maxSpeed = 0;
    //         bool haveGreen = false;
    //         for (const NBEdge* const fromEdge : incoming) {
    //             const bool inChosen = chosenSet.count(fromEdge) != 0;
    //             const int numLanes = fromEdge->getNumLanes();
    //             for (int i2 = 0; i2 < numLanes; i2++) {
    //                 for (const NBEdge::Connection& approached : fromEdge->getConnectionsFromLane(i2)) {
    //                     if (!fromEdge->mayBeTLSControlled(i2, approached.toEdge, approached.toLane)) {
    //                         continue;
    //                     }
    //                     if (inChosen) {
    //                         state[pos] = 'G';
    //                         haveGreen = true;
    //                         maxSpeed = MAX2(maxSpeed, fromEdge->getSpeed());
    //                     } else {
    //                         state[pos] = 'r';
    //                     }
    //                     ++pos;
    //                 }
    //             }
    //         }
    //         if (!haveGreen) {
    //             continue;
    //         }
    //
    // #ifdef DEBUG_PHASES
    //         if (DEBUGCOND) {
    //             std::cout << " state after plain straight movers " << state << "\n";
    //         }
    // #endif
    //         if (!isNEMA) {
    //             // correct behaviour for those that are not in chosen, but may drive, though
    //             state = allowCompatible(state, fromEdges, toEdges, fromLanes, toLanes);
    // #ifdef DEBUG_PHASES
    //             if (DEBUGCOND) {
    //                 std::cout << " state after allowing compatible " << state << "\n";
    //             }
    // #endif
    //             if (groupTram) {
    //                 state = allowByVClass(state, fromEdges, toEdges, SVC_TRAM);
    //             } else if (groupOther) {
    //                 state = allowByVClass(state, fromEdges, toEdges, SVC_PEDESTRIAN | SVC_BICYCLE | SVC_DELIVERY);
    //             }
    // #ifdef DEBUG_PHASES
    //             if (DEBUGCOND) {
    //                 std::cout << " state after grouping by vClass " << state << " (groupTram=" << groupTram << " groupOther=" << groupOther << ")\n";
    //             }
    // #endif
    //             if (groupOpposites || chosen.first->getToNode()->getType() == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED) {
    //                 state = allowUnrelated(state, fromEdges, toEdges, isTurnaround, crossings);
    //             }
    // #ifdef DEBUG_PHASES
    //             if (DEBUGCOND) {
    //                 std::cout << " state after finding allowUnrelated " << state << "\n";
    //             }
    // #endif
    //         }
    //         // correct behaviour for those that have to wait (mainly left-mover)
    //         bool haveForbiddenLeftMover = false;
    //         std::vector<bool> rightTurnConflicts(pos, false);
    //         std::vector<bool> mergeConflicts(pos, false);
    //         state = correctConflicting(state, fromEdges, toEdges, isTurnaround, fromLanes, toLanes, hadGreenMajor, haveForbiddenLeftMover, rightTurnConflicts, mergeConflicts);
    //         for (int i1 = 0; i1 < pos; ++i1) {
    //             if (state[i1] == 'G') {
    //                 hadGreenMajor[i1] = true;
    //             }
    //         }
    // #ifdef DEBUG_PHASES
    //         if (DEBUGCOND) {
    //             std::cout << " state after correcting left movers=" << state << "\n";
    //         }
    // #endif
    //
    //         std::vector<bool> leftGreen(pos, false);
    //         // check whether at least one left-turn lane exist
    //         bool foundLeftTurnLane = false;
    //         for (int i1 = 0; i1 < pos; ++i1) {
    //             if (state[i1] == 'g' && !rightTurnConflicts[i1] && !mergeConflicts[i1] && hasTurnLane[i1]) {
    //                 foundLeftTurnLane = true;
    //             }
    //         }
    //         const bool buildLeftGreenPhase = (haveForbiddenLeftMover && !myHaveSinglePhase && leftTurnTime > 0 && foundLeftTurnLane
    //                                           && groupOpposites && !groupTram && !groupOther);
    //
    //         // find indices for exclusive left green phase and apply option minor-left.max-speed
    //         for (int i1 = 0; i1 < pos; ++i1) {
    //             if (state[i1] == 'g' && !rightTurnConflicts[i1] && !mergeConflicts[i1]
    //                     // only activate turn-around together with a real left-turn
    //                     && (!isTurnaround[i1] || (i1 > 0 && leftGreen[i1 - 1]))) {
    //                 leftGreen[i1] = true;
    //                 if (fromEdges[i1]->getSpeed() > minorLeftSpeedThreshold) {
    //                     if (buildLeftGreenPhase) {
    //                         state[i1] = 'r';
    //                         //std::cout << " disabling minorLeft " << i1 << " (speed=" << fromEdges[i1]->getSpeed() << " thresh=" << minorLeftSpeedThreshold << ")\n";
    //                     } else if (!isTurnaround[i1]) {
    //                         WRITE_WARNINGF(TL("Minor green from edge '%' to edge '%' exceeds %m/s. Maybe a left-turn lane is missing."),
    //                                        fromEdges[i1]->getID(), toEdges[i1]->getID(), minorLeftSpeedThreshold);
    //                     }
    //                 }
    //             }
    //         }
    //
    // #ifdef DEBUG_PHASES
    //         if (DEBUGCOND) {
    //             std::cout << getID() << " state=" << state << " buildLeft=" << buildLeftGreenPhase << " hFLM=" << haveForbiddenLeftMover << " turnLane=" << foundLeftTurnLane
    //                       << "   \nrtC=" << toString(rightTurnConflicts)
    //                       << "   \nmC=" << toString(mergeConflicts)
    //                       << "   \nhTL=" << toString(hasTurnLane)
    //                       << "   \nlGr=" << toString(leftGreen)
    //                       << "\n";
    //         }
    // #endif
    //         straightStates.push_back(state);
    //
    //         const std::string vehicleState = state; // backup state before pedestrian modifications
    //         greenPhases.push_back((int)logic->getPhases().size());
    //
    //         // 5s at 50km/h, 10s at 80km/h, rounded to full seconds
    //         const double minDurBySpeed = maxSpeed * 3.6 / 6 - 3.3;
    //         SUMOTime minDur = MAX2(minMinDur, TIME2STEPS(floor(minDurBySpeed + 0.5)));
    //         if (chosen.first->getPermissions() == SVC_TRAM && (chosen.second == nullptr || chosen.second->getPermissions() == SVC_TRAM)) {
    //             // shorter minDuration for tram phase (only if the phase is
    //             // exclusively for tram)
    //             bool tramExclusive = true;
    //             for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //                 if (state[i1] == 'G') {
    //                     SVCPermissions linkPerm = (fromEdges[i1]->getPermissions() & toEdges[i1]->getPermissions());
    //                     if (linkPerm != SVC_TRAM) {
    //                         tramExclusive = false;
    //                         break;
    //                     }
    //                 }
    //             }
    //             if (tramExclusive) {
    //                 // one tram per actuated phase
    //                 minDur = TIME2STEPS(1);
    //             }
    //         }
    //
    //         state = addPedestrianPhases(logic, greenTime, minDur, maxDur, earliestEnd, latestEnd, state, crossings, fromEdges, toEdges);
    //         // pedestrians have 'r' from here on
    //         for (int i1 = pos; i1 < (int)state.size(); ++i1) {
    //             state[i1] = 'r';
    //         }
    //         if (brakingTime > 0) {
    //             SUMOTime maxCross = 0;
    //             // build yellow (straight)
    //             for (int i1 = 0; i1 < pos; ++i1) {
    //                 if (state[i1] != 'G' && state[i1] != 'g') {
    //                     continue;
    //                 }
    //                 if ((vehicleState[i1] >= 'a' && vehicleState[i1] <= 'z')
    //                         && buildLeftGreenPhase
    //                         && !rightTurnConflicts[i1]
    //                         && !mergeConflicts[i1]
    //                         && leftGreen[i1]) {
    //                     continue;
    //                 }
    //                 state[i1] = 'y';
    //                 maxCross = MAX2(maxCross, crossingTime[i1]);
    //             }
    //             // add step
    //             logic->addStep(brakingTime, state);
    //             // add optional all-red state
    //             if (!buildLeftGreenPhase) {
    //                 if (myLayout == TrafficLightLayout::ALTERNATE_ONEWAY) {
    //                     allRedTime = computeEscapeTime(state, fromEdges, toEdges);
    //                 }
    //                 buildAllRedState(allRedTime + MAX2(0ll, maxCross - brakingTime - allRedTime), logic, state);
    //             }
    //         }
    //
    //
    //         if (buildLeftGreenPhase) {
    //             // build left green
    //             for (int i1 = 0; i1 < pos; ++i1) {
    //                 if (state[i1] == 'Y' || state[i1] == 'y') {
    //                     state[i1] = 'r';
    //                     continue;
    //                 }
    //                 if (leftGreen[i1]) {
    //                     state[i1] = 'G';
    //                 }
    //             }
    //             leftStates.push_back(state);
    //             state = allowCompatible(state, fromEdges, toEdges, fromLanes, toLanes);
    //             state = correctConflicting(state, fromEdges, toEdges, isTurnaround, fromLanes, toLanes, hadGreenMajor, haveForbiddenLeftMover, rightTurnConflicts, mergeConflicts);
    //             bool buildMixedGreenPhase = false;
    //             std::vector<bool> mixedGreen(pos, false);
    //             const std::string oldState = state;
    //             if (noMixed) {
    //                 state = correctMixed(state, fromEdges, fromLanes, buildMixedGreenPhase, mixedGreen);
    //             }
    //             if (state != oldState) {
    //                 for (int i1 = 0; i1 < pos; ++i1) {
    //                     if (mixedGreen[i1]) {
    //                         // patch previous yellow and allred phase
    //                         int yellowIndex = (int)logic->getPhases().size() - 1;
    //                         if (allRedTime > 0) {
    //                             logic->setPhaseState(yellowIndex--, i1, LINKSTATE_TL_RED);
    //                         }
    //                         if (brakingTime > 0) {
    //                             logic->setPhaseState(yellowIndex, i1, LINKSTATE_TL_YELLOW_MINOR);
    //                         }
    //                     }
    //                 }
    //                 state = allowCompatible(state, fromEdges, toEdges, fromLanes, toLanes);
    //             }
    //
    //             // add step
    //             logic->addStep(leftTurnTime, state, minDur, maxDur, earliestEnd, latestEnd);
    //
    //             // build left yellow
    //             if (brakingTime > 0) {
    //                 SUMOTime maxCross = 0;
    //                 for (int i1 = 0; i1 < pos; ++i1) {
    //                     if (state[i1] != 'G' && state[i1] != 'g') {
    //                         continue;
    //                     }
    //                     state[i1] = 'y';
    //                     maxCross = MAX2(maxCross, crossingTime[i1]);
    //                 }
    //                 // add step
    //                 logic->addStep(brakingTime, state);
    //                 // add optional all-red state
    //                 buildAllRedState(allRedTime + MAX2(0ll, maxCross - brakingTime - allRedTime), logic, state);
    //             }
    //
    //             if (buildMixedGreenPhase) {
    //                 // build mixed green
    //                 // @todo if there is no left green phase we might want to build two
    //                 // mixed-green phases but then we should consider avoid a common
    //                 // opposite phase for this direction
    //
    //                 for (int i1 = 0; i1 < pos; ++i1) {
    //                     if (state[i1] == 'Y' || state[i1] == 'y') {
    //                         state[i1] = 'r';
    //                         continue;
    //                     }
    //                     if (mixedGreen[i1]) {
    //                         state[i1] = 'G';
    //                     }
    //                 }
    //                 state = allowCompatible(state, fromEdges, toEdges, fromLanes, toLanes);
    //                 state = correctConflicting(state, fromEdges, toEdges, isTurnaround, fromLanes, toLanes, hadGreenMajor, haveForbiddenLeftMover, rightTurnConflicts, mergeConflicts);
    //
    //                 // add step
    //                 logic->addStep(leftTurnTime, state, minDur, maxDur, earliestEnd, latestEnd);
    //
    //                 // build mixed yellow
    //                 if (brakingTime > 0) {
    //                     SUMOTime maxCross = 0;
    //                     for (int i1 = 0; i1 < pos; ++i1) {
    //                         if (state[i1] != 'G' && state[i1] != 'g') {
    //                             continue;
    //                         }
    //                         state[i1] = 'y';
    //                         maxCross = MAX2(maxCross, crossingTime[i1]);
    //                     }
    //                     // add step
    //                     logic->addStep(brakingTime, state);
    //                     // add optional all-red state
    //                     buildAllRedState(allRedTime + MAX2(0ll, maxCross - brakingTime - allRedTime), logic, state);
    //                 }
    //             }
    //
    //         } else if (isNEMA) {
    //             std::string& s = straightStates.back();
    //             std::string leftState = s;
    //             for (int ii = 0; ii < pos; ++ii) {
    //                 if (s[ii] != 'r') {
    //                     NBEdge* fromEdge = fromEdges[ii];
    //                     NBEdge* toEdge = toEdges[ii];
    //                     LinkDirection dir = fromEdge->getToNode()->getDirection(fromEdge, toEdge);
    //                     if (hasTurnLane[ii] && (dir == LinkDirection::LEFT || dir == LinkDirection::TURN)) {
    //                         s[ii] = 'r';
    //                         leftState[ii] = 'G';
    //                     } else {
    //                         leftState[ii] = 'r';
    //                     }
    //                 }
    //             }
    //             leftStates.push_back(leftState);
    //         }
    //         // fix edges within joined traffic lights that did not get the green light yet
    //         if (myEdgesWithin.size() > 0 && !isNEMA && toProc.size() == 0 && !onlyConts) {
    //             addGreenWithin(logic, fromEdges, toProc);
    //         }
    //     }
    //     // fix pedestrian crossings that did not get the green light yet
    //     if (crossings.size() > 0) {
    //         addPedestrianScramble(logic, totalNumLinks, TIME2STEPS(10), brakingTime, crossings, fromEdges, toEdges);
    //     }
    //     // add optional red phase if there were no foes
    //     if (logic->getPhases().size() == 2 && brakingTime > 0
    //             && OptionsCont::getOptions().getInt("tls.red.time") > 0) {
    //         const SUMOTime redTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.red.time"));
    //         logic->addStep(redTime, std::string(totalNumLinks, 'r'));
    //     }
    //
    //     if (myLayout == TrafficLightLayout::ALTERNATE_ONEWAY) {
    //         // exiting the oneway section should always be possible
    //         deactivateInsideEdges(logic, fromEdges);
    //     }
    //     if (isNEMA) {
    //         NBTrafficLightLogic* nemaLogic = buildNemaPhases(fromEdges, toEdges, crossings, chosenList, straightStates, leftStates);
    //         if (nemaLogic == nullptr) {
    //             WRITE_WARNINGF(TL("Generating NEMA phases is not supported for traffic light '%' with % incoming edges. Using tlType 'actuated' as fallback"), getID(), incoming.size());
    //             logic->setType(TrafficLightType::ACTUATED);
    //             setType(TrafficLightType::ACTUATED);
    //         } else {
    //             delete logic;
    //             logic = nemaLogic;
    //         }
    //     }
    //
    //     SUMOTime totalDuration = logic->getDuration();
    //
    //     if ((OptionsCont::getOptions().isDefault("tls.green.time") || !OptionsCont::getOptions().isDefault("tls.cycle.time")) && !isNEMA) {
    //         const SUMOTime cycleTime = TIME2STEPS(OptionsCont::getOptions().getInt("tls.cycle.time"));
    //         // adapt to cycle time by changing the duration of the green phases
    //         SUMOTime minGreenDuration = SUMOTime_MAX;
    //         for (std::vector<int>::const_iterator it = greenPhases.begin(); it != greenPhases.end(); ++it) {
    //             const SUMOTime dur = logic->getPhases()[*it].duration;
    //             minGreenDuration = MIN2(minGreenDuration, dur);
    //         }
    //         const int patchSeconds = (int)(STEPS2TIME(cycleTime - totalDuration) / (double)greenPhases.size());
    //         const int patchSecondsRest = (int)(STEPS2TIME(cycleTime - totalDuration)) - patchSeconds * (int)greenPhases.size();
    //         //std::cout << "cT=" << cycleTime << " td=" << totalDuration << " pS=" << patchSeconds << " pSR=" << patchSecondsRest << "\n";
    //         if (STEPS2TIME(minGreenDuration) + patchSeconds < MIN_GREEN_TIME
    //                 || STEPS2TIME(minGreenDuration) + patchSeconds + patchSecondsRest < MIN_GREEN_TIME
    //                 || greenPhases.size() == 0) {
    //             if (getID() != DummyID) {
    //                 WRITE_WARNINGF(TL("The traffic light '%' cannot be adapted to a cycle time of %."), getID(), time2string(cycleTime));
    //             }
    //             // @todo use a multiple of cycleTime ?
    //         } else {
    //             for (std::vector<int>::const_iterator it = greenPhases.begin(); it != greenPhases.end(); ++it) {
    //                 logic->setPhaseDuration(*it, logic->getPhases()[*it].duration + TIME2STEPS(patchSeconds));
    //             }
    //             if (greenPhases.size() > 0) {
    //                 logic->setPhaseDuration(greenPhases.front(), logic->getPhases()[greenPhases.front()].duration + TIME2STEPS(patchSecondsRest));
    //             }
    //             totalDuration = logic->getDuration();
    //         }
    //     }
    //
    //     // check for coherent signal sequence and remove yellow if preceded and followed by green
    //     const std::vector<NBTrafficLightLogic::PhaseDefinition>& allPhases = logic->getPhases();
    //     const int phaseCount = (int)allPhases.size();
    //     const int stateSize = (int)logic->getNumLinks();
    //     for (int i = 0; i < phaseCount; ++i) {
    //         std::string currState = allPhases[i].state;
    //         const int prevIndex = (i == 0) ? phaseCount - 1 : i - 1;
    //         const std::string prevState = allPhases[prevIndex].state;
    //         const std::string nextState = allPhases[(i + 1) % phaseCount].state;
    //         bool updatedState = false;
    //         for (int i1 = 0; i1 < stateSize; ++i1) {
    //             if (currState[i1] == 'y' && (nextState[i1] == prevState[i1] || nextState[i1] == 'G') && (prevState[i1] == 'g' || prevState[i1] == 'G')) {
    //                 logic->setPhaseState(i, i1, (LinkState)prevState[i1]);
    //                 updatedState = true;
    //             }
    //         }
    //         UNUSED_PARAMETER(updatedState);  // disable warning
    // #ifdef DEBUG_PHASES
    //         if (DEBUGCOND) {
    //             if (updatedState) {
    //                 std::cout << getID() << " state of phase index " << i <<  " was patched due to yellow in between green\n";
    //             }
    //
    //         }
    // #endif
    //     }
    //
    //
    //     myExtraConflictsReady = true;
    //     // this computation only makes sense for single nodes
    //     if (myControlledNodes.size() == 1) {
    //         myNeedsContRelationReady = true;
    //     }
    //     if (totalDuration > 0) {
    //         if (totalDuration > 3 * (greenTime + 2 * brakingTime + leftTurnTime) && !isNEMA && getID() != DummyID) {
    //             WRITE_WARNINGF(TL("The traffic light '%' has a high cycle time of %."), getID(), time2string(totalDuration));
    //         }
    //         logic->closeBuilding();
    //         return logic;
    //     } else {
    //         delete logic;
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_logic_and_conts(&mut self, brakingTimeSeconds: i32, onlyConts: bool) -> Ptr<NBTrafficLightLogic> {
        todo!("port NBOwnTLDef::computeLogicAndConts")
    }

    /// C++: `void NBOwnTLDef::initNeedsContRelation() const` [const, virtual]
    // void
    // NBOwnTLDef::initNeedsContRelation() const {
    //     if (!myNeedsContRelationReady) {
    //         if (myControlledNodes.size() > 0) {
    //             // we use a dummy node just to maintain const-correctness
    //             myNeedsContRelation.clear();
    //             for (NBNode* n : myControlledNodes) {
    //                 NBOwnTLDef dummy(DummyID, n, 0, TrafficLightType::STATIC);
    //                 dummy.setParticipantsInformation();
    //                 NBTrafficLightLogic* tllDummy = dummy.computeLogicAndConts(0, true);
    //                 delete tllDummy;
    //                 myNeedsContRelation.insert(dummy.myNeedsContRelation.begin(), dummy.myNeedsContRelation.end());
    //                 n->removeTrafficLight(&dummy);
    //             }
    // #ifdef DEBUG_CONTRELATION
    //             if (DEBUGCOND) {
    //                 std::cout << " contRelations at " << getID() << " prog=" << getProgramID() << ":\n";
    //                 for (const StreamPair& s : myNeedsContRelation) {
    //                     std::cout << "   " << s.from1->getID() << "->" << s.to1->getID() << " foe " << s.from2->getID() << "->" << s.to2->getID() << "\n";
    //                 }
    //             }
    // #endif
    //
    //         }
    //         myNeedsContRelationReady = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_needs_cont_relation(&self) {
        todo!("port NBOwnTLDef::initNeedsContRelation")
    }

    /// C++: `void NBOwnTLDef::buildAllRedState(SUMOTime, NBTrafficLightLogic *, const std::string &)`
    // void
    // NBOwnTLDef::buildAllRedState(SUMOTime allRedTime, NBTrafficLightLogic* logic, const std::string& state) {
    //     if (allRedTime > 0) {
    //         // build all-red phase
    //         std::string allRedState = state;
    //         for (int i = 0; i < (int)state.size(); i++) {
    //             if (allRedState[i] == 'Y' || allRedState[i] == 'y') {
    //                 allRedState[i] = 'r';
    //             }
    //         }
    //         logic->addStep(TIME2STEPS(ceil(STEPS2TIME(allRedTime))), allRedState);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_all_red_state(&mut self, allRedTime: SUMOTime, logic: Ptr<NBTrafficLightLogic>, state: &str) {
        todo!("port NBOwnTLDef::buildAllRedState")
    }

    /// C++: `int NBOwnTLDef::getMaxIndex()` [virtual]
    /// Returns the maximum index controlled by this traffic light
    // int
    // NBOwnTLDef::getMaxIndex() {
    //     setParticipantsInformation();
    //     NBTrafficLightLogic* logic = compute(OptionsCont::getOptions());
    //     if (logic != nullptr) {
    //         return logic->getNumLinks() - 1;
    //     } else {
    //         return -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_max_index(&mut self) -> i32 {
        todo!("port NBOwnTLDef::getMaxIndex")
    }

    /// C++: `void NBOwnTLDef::setLayout(TrafficLightLayout)`
    /// sets the layout for the generated signal plan
    // void setLayout(TrafficLightLayout layout) {
    //         myLayout = layout;
    //     }
    #[allow(unused_variables)]
    pub fn set_layout(&mut self, layout: TrafficLightLayout) {
        todo!("port NBOwnTLDef::setLayout")
    }

    /// C++: `TrafficLightLayout NBOwnTLDef::getLayout() const` [const, virtual]
    // TrafficLightLayout getLayout() const {
    //         return myLayout;
    //     }
    #[allow(unused_variables)]
    pub fn get_layout(&self) -> TrafficLightLayout {
        todo!("port NBOwnTLDef::getLayout")
    }

    /// C++: `NBTrafficLightLogic * NBOwnTLDef::myCompute(int)` [virtual, protected]
    /// Computes the traffic light logic finally in dependence to the type
    // NBTrafficLightLogic*
    // NBOwnTLDef::myCompute(int brakingTimeSeconds) {
    //     if (myControlledNodes.size() > 1) {
    //         // call this first so that the following call to computeLogicAndConts resets linkIndices
    //         initNeedsContRelation();
    //         // reset insideTLS info
    //         collectEdges();
    //     }
    //     return computeLogicAndConts(brakingTimeSeconds);
    // }
    #[allow(unused_variables)]
    pub fn my_compute(&mut self, brakingTimeSeconds: i32) -> Ptr<NBTrafficLightLogic> {
        todo!("port NBOwnTLDef::myCompute")
    }

    /// C++: `void NBOwnTLDef::collectLinks()` [virtual, protected]
    /// Collects the links participating in this traffic light
    // void
    // NBOwnTLDef::collectLinks() {
    //     myControlledLinks.clear();
    //     collectAllLinks(myControlledLinks);
    // }
    #[allow(unused_variables)]
    pub fn collect_links(&mut self) {
        todo!("port NBOwnTLDef::collectLinks")
    }

    /// C++: `void NBOwnTLDef::replaceRemoved(NBEdge *, int, NBEdge *, int, bool)` [virtual, protected]
    /// Replaces a removed edge/lane
    // void
    // NBOwnTLDef::replaceRemoved(NBEdge* /*removed*/, int /*removedLane*/,
    //                            NBEdge* /*by*/, int /*byLane*/, bool /*incoming*/) {}
    #[allow(unused_variables)]
    pub fn replace_removed(&mut self, removed: Ptr<NBEdge>, removedLane: i32, by: Ptr<NBEdge>, byLane: i32, incoming: bool) {
        todo!("port NBOwnTLDef::replaceRemoved")
    }

    /// C++: `bool NBOwnTLDef::corridorLike() const` [const, protected]
    /// test whether a joined tls with layout 'opposites' would be built without dedicated left-turn phase
    // bool
    // NBOwnTLDef::corridorLike() const {
    //     if (getID() == DummyID) {
    //         // avoid infinite recursion
    //         return true;
    //     }
    //     // setParticipantsInformation resets myAmInTLS so we need to make a copy
    //     std::vector<bool> edgeInsideTLS;
    //     for (const NBEdge* e : myIncomingEdges) {
    //         edgeInsideTLS.push_back(e->isInsideTLS());
    //     }
    //     assert(myControlledNodes.size() >= 2);
    //     NBOwnTLDef dummy(DummyID, myControlledNodes, 0, TrafficLightType::STATIC);
    //     dummy.setParticipantsInformation();
    //     NBTrafficLightLogic* tllDummy = dummy.computeLogicAndConts(0, true);
    //     int greenPhases = 0;
    //     for (const auto& phase : tllDummy->getPhases()) {
    //         if (phase.state.find_first_of("gG") != std::string::npos) {
    //             greenPhases++;
    //         }
    //     }
    //     delete tllDummy;
    //     for (const auto& controlledNode : myControlledNodes) {
    //         controlledNode->removeTrafficLight(&dummy);
    //     }
    //     int i = 0;
    //     for (NBEdge* e : myIncomingEdges) {
    //         e->setInsideTLS(edgeInsideTLS[i]);
    //         i++;
    //     }
    //     return greenPhases <= 2;
    // }
    #[allow(unused_variables)]
    pub fn corridor_like(&self) -> bool {
        todo!("port NBOwnTLDef::corridorLike")
    }

    /// C++: `NBTrafficLightLogic * NBOwnTLDef::buildNemaPhases(const EdgeVector &, const EdgeVector &, const std::vector<NBNode::Crossing *> &, const std::vector<std::pair<NBEdge *, NBEdge *>> &, const std::vector<std::string> &, const std::vector<std::string> &)` [protected]
    // NBTrafficLightLogic*
    // NBOwnTLDef::buildNemaPhases(
    //     const EdgeVector& fromEdges,
    //     const EdgeVector& toEdges,
    //     const std::vector<NBNode::Crossing*>& crossings,
    //     const std::vector<std::pair<NBEdge*, NBEdge*> >& chosenList,
    //     const std::vector<std::string>& straightStates,
    //     const std::vector<std::string>& leftStates) {
    //     if (chosenList.size() != 2) {
    //         return nullptr;
    //     }
    //     const SUMOTime dur = TIME2STEPS(OptionsCont::getOptions().getInt("tls.cycle.time"));
    //     const SUMOTime vehExt = TIME2STEPS(OptionsCont::getOptions().getInt("tls.nema.vehExt"));
    //     const SUMOTime yellow = TIME2STEPS(OptionsCont::getOptions().getInt("tls.nema.yellow"));
    //     const SUMOTime red = TIME2STEPS(OptionsCont::getOptions().getInt("tls.nema.red"));
    //     const SUMOTime minMinDur = TIME2STEPS(OptionsCont::getOptions().getInt("tls.min-dur"));
    //     const SUMOTime maxDur = TIME2STEPS(OptionsCont::getOptions().getInt("tls.max-dur"));
    //     const SUMOTime earliestEnd = UNSPECIFIED_DURATION;
    //     const SUMOTime latestEnd = UNSPECIFIED_DURATION;
    //
    //     const int totalNumLinks = (int)straightStates[0].size();
    //     NBTrafficLightLogic* logic = new NBTrafficLightLogic(getID(), getProgramID(), totalNumLinks, myOffset, myType);
    //     std::vector<int> ring1({1, 2, 3, 4});
    //     std::vector<int> ring2({5, 6, 7, 8});
    //     std::vector<int> barrier1({4, 8});
    //     std::vector<int> barrier2({2, 6});
    //     int phaseNameLeft = 1;
    //     for (int i = 0; i < (int)chosenList.size(); i++) {
    //         NBEdge* e1 = chosenList[i].first;
    //         assert(e1 != nullptr);
    //         NBEdge* e2 = chosenList[i].second;
    //         if (i < (int)leftStates.size()) {
    //             std::string left1 = filterState(leftStates[i], fromEdges, e1);
    //             if (left1 != "") {
    //                 logic->addStep(dur, left1, minMinDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, toString(phaseNameLeft));
    //             }
    //         }
    //         if (e2 != nullptr) {
    //             std::string straight2 = filterState(straightStates[i], fromEdges, e2);
    //             straight2 = patchNEMAStateForCrossings(straight2, crossings, fromEdges, toEdges, e2, e1);
    //
    //             logic->addStep(dur, straight2, minMinDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, toString(phaseNameLeft + 1));
    //             if (i < (int)leftStates.size()) {
    //                 std::string left2 = filterState(leftStates[i], fromEdges, e2);
    //                 if (left2 != "") {
    //                     logic->addStep(dur, left2, minMinDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, toString(phaseNameLeft + 4));
    //                 }
    //             }
    //
    //         }
    //         std::string straight1 = filterState(straightStates[i], fromEdges, e1);
    //         if (straight1 == "") {
    //             delete logic;
    //             return nullptr;
    //         }
    //         straight1 = patchNEMAStateForCrossings(straight1, crossings, fromEdges, toEdges, e1, e2);
    //         logic->addStep(dur, straight1, minMinDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, toString(phaseNameLeft + 5));
    //         phaseNameLeft += 2;
    //     }
    //     std::map<int, int> names; // nema phase name -> sumo phase index
    //     for (int i = 0; i < (int)logic->getPhases().size(); i++) {
    //         names[StringUtils::toInt(logic->getPhases()[i].name)] = i;
    //     }
    //
    //     filterMissingNames(ring1, names, false);
    //     filterMissingNames(ring2, names, false);
    //     filterMissingNames(barrier1, names, true, 8);
    //     filterMissingNames(barrier2, names, true, 6);
    //     if (ring1[0] == 0 && ring1[1] == 0) {
    //         ring1[1] = 6;
    //     }
    //     if (ring1[2] == 0 && ring1[3] == 0) {
    //         ring1[3] = 8;
    //     }
    //     fixDurationSum(logic, names, ring1[0], ring1[1], ring2[0], ring2[1]);
    //     fixDurationSum(logic, names, ring1[2], ring1[3], ring2[2], ring2[3]);
    //
    //     logic->setParameter("ring1", joinToString(ring1, ","));
    //     logic->setParameter("ring2", joinToString(ring2, ","));
    //     logic->setParameter("barrierPhases", joinToString(barrier1, ","));
    //     logic->setParameter("barrier2Phases", joinToString(barrier2, ","));
    //     return logic;
    // }
    #[allow(unused_variables)]
    pub fn build_nema_phases(&mut self, fromEdges: &EdgeVector, toEdges: &EdgeVector, crossings: &Vec<Ptr<NBNode_Crossing>>, chosenList: &Vec<(Ptr<NBEdge>, Ptr<NBEdge>)>, straightStates: &Vec<String>, leftStates: &Vec<String>) -> Ptr<NBTrafficLightLogic> {
        todo!("port NBOwnTLDef::buildNemaPhases")
    }

    /// C++: `std::string NBOwnTLDef::filterState(std::string, const EdgeVector &, const NBEdge *)` [protected]
    /// mask out all greens that do not originate at the given edge
    // std::string
    // NBOwnTLDef::filterState(std::string state, const EdgeVector& fromEdges, const NBEdge* e) {
    //     bool haveGreen = false;
    //     for (int j = 0; j < (int)fromEdges.size(); j++) {
    //         if (fromEdges[j] != e) {
    //             state[j] = 'r';
    //         } else if (state[j] != 'r') {
    //             haveGreen = true;
    //         }
    //     }
    //     if (haveGreen) {
    //         return state;
    //     } else {
    //         return "";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn filter_state(&mut self, state: String, fromEdges: &EdgeVector, e: Ptr<NBEdge>) -> String {
        todo!("port NBOwnTLDef::filterState")
    }

    /// C++: `void NBOwnTLDef::filterMissingNames(std::vector<int> &, const std::map<int, int> &, bool, int)` [protected]
    /// keep only valid NEMA phase names (for params)
    // void
    // NBOwnTLDef::filterMissingNames(std::vector<int>& vec, const std::map<int, int>& names, bool isBarrier, int barrierDefault) {
    //     for (int i = 0; i < (int)vec.size(); i++) {
    //         if (names.count(vec[i]) == 0) {
    //             if (isBarrier) {
    //                 if (names.count(vec[i] - 1) > 0) {
    //                     vec[i] = vec[i] - 1;
    //                 } else {
    //                     vec[i] = barrierDefault;
    //                 }
    //             } else {
    //                 vec[i] = 0;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn filter_missing_names(&mut self, vec: &mut Vec<i32>, names: &BTreeMap<i32, i32>, isBarrier: bool, barrierDefault: i32) {
        todo!("port NBOwnTLDef::filterMissingNames")
    }

    /// C++: `void NBOwnTLDef::fixDurationSum(NBTrafficLightLogic *, const std::map<int, int> &, int, int, int, int)` [protected]
    /// ensure that phase max durations before each barrier have the same sum in both rings
    // void
    // NBOwnTLDef::fixDurationSum(NBTrafficLightLogic* logic, const std::map<int, int>& names, int ring1a, int ring1b, int ring2a, int ring2b) {
    //     std::set<int> ring1existing;
    //     std::set<int> ring2existing;
    //     if (names.count(ring1a) != 0) {
    //         ring1existing.insert(ring1a);
    //     }
    //     if (names.count(ring1b) != 0) {
    //         ring1existing.insert(ring1b);
    //     }
    //     if (names.count(ring2a) != 0) {
    //         ring2existing.insert(ring2a);
    //     }
    //     if (names.count(ring2b) != 0) {
    //         ring2existing.insert(ring2b);
    //     }
    //     if (ring1existing.size() > 0 && ring2existing.size() > 0 &&
    //             ring1existing.size() != ring2existing.size()) {
    //         int pI; // sumo phase index
    //         if (ring1existing.size() < ring2existing.size()) {
    //             pI = names.find(*ring1existing.begin())->second;
    //         } else {
    //             pI = names.find(*ring2existing.begin())->second;
    //         }
    //         const auto& p = logic->getPhases()[pI];
    //         SUMOTime newMaxDur = 2 * p.maxDur + p.yellow + p.red;
    //         logic->setPhaseMaxDuration(pI, newMaxDur);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fix_duration_sum(&mut self, logic: Ptr<NBTrafficLightLogic>, names: &BTreeMap<i32, i32>, ring1a: i32, ring1b: i32, ring2a: i32, ring2b: i32) {
        todo!("port NBOwnTLDef::fixDurationSum")
    }

    /// C++: `double NBOwnTLDef::getDirectionalWeight(LinkDirection)` [protected]
    /// Returns the weight of a stream given its direction
    // double
    // NBOwnTLDef::getDirectionalWeight(LinkDirection dir) {
    //     switch (dir) {
    //         case LinkDirection::STRAIGHT:
    //         case LinkDirection::PARTLEFT:
    //         case LinkDirection::PARTRIGHT:
    //             return HEIGH_WEIGHT;
    //         case LinkDirection::LEFT:
    //         case LinkDirection::RIGHT:
    //             return LOW_WEIGHT;
    //         default:
    //             break;
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_directional_weight(&mut self, dir: LinkDirection) -> f64 {
        todo!("port NBOwnTLDef::getDirectionalWeight")
    }

    /// C++: `int NBOwnTLDef::getToPrio(const NBEdge *const)` [protected]
    /// Returns this edge's priority at the node it ends at
    // int
    // NBOwnTLDef::getToPrio(const NBEdge* const e) {
    //     return e->getJunctionPriority(e->getToNode());
    // }
    #[allow(unused_variables)]
    pub fn get_to_prio(&mut self, e: Ptr<NBEdge>) -> i32 {
        todo!("port NBOwnTLDef::getToPrio")
    }

    /// C++: `double NBOwnTLDef::computeUnblockedWeightedStreamNumber(const NBEdge *const, const NBEdge *const)` [protected]
    /// Returns how many streams outgoing from the edges can pass the junction without being blocked
    // double
    // NBOwnTLDef::computeUnblockedWeightedStreamNumber(const NBEdge* const e1, const NBEdge* const e2) {
    //     double val = 0;
    //     for (int e1l = 0; e1l < e1->getNumLanes(); e1l++) {
    //         std::vector<NBEdge::Connection> approached1 = e1->getConnectionsFromLane(e1l);
    //         for (int e2l = 0; e2l < e2->getNumLanes(); e2l++) {
    //             std::vector<NBEdge::Connection> approached2 = e2->getConnectionsFromLane(e2l);
    //             for (std::vector<NBEdge::Connection>::iterator e1c = approached1.begin(); e1c != approached1.end(); ++e1c) {
    //                 if (e1->getTurnDestination() == (*e1c).toEdge) {
    //                     continue;
    //                 }
    //                 for (std::vector<NBEdge::Connection>::iterator e2c = approached2.begin(); e2c != approached2.end(); ++e2c) {
    //                     if (e2->getTurnDestination() == (*e2c).toEdge) {
    //                         continue;
    //                     }
    //                     const double sign = (forbids(e1, (*e1c).toEdge, e2, (*e2c).toEdge, true)
    //                                          || forbids(e2, (*e2c).toEdge, e1, (*e1c).toEdge, true)) ? -1 : 1;
    //                     double w1;
    //                     double w2;
    //                     const int prio1 = e1->getJunctionPriority(e1->getToNode());
    //                     const int prio2 = e2->getJunctionPriority(e2->getToNode());
    //                     if (prio1 == prio2) {
    //                         w1 = getDirectionalWeight(e1->getToNode()->getDirection(e1, (*e1c).toEdge));
    //                         w2 = getDirectionalWeight(e2->getToNode()->getDirection(e2, (*e2c).toEdge));
    //                     } else {
    //                         if (prio1 > prio2) {
    //                             w1 = HEIGH_WEIGHT;
    //                             w2 = LOW_WEIGHT;
    //                         } else {
    //                             w1 = LOW_WEIGHT;
    //                             w2 = HEIGH_WEIGHT;
    //                         }
    //                         if (sign == -1) {
    //                             // extra penalty if edges with different junction priority are in conflict
    //                             w1 *= 2;
    //                             w2 *= 2;
    //                         }
    //                     }
    //                     if (isRailway(e1->getPermissions()) != isRailway(e2->getPermissions())) {
    //                         w1 *= 0.1;
    //                         w2 *= 0.1;
    //                     }
    //                     if ((e1->getPermissions() & SVC_PASSENGER) == 0) {
    //                         w1 *= 0.1;
    //                     }
    //                     if ((e2->getPermissions() & SVC_PASSENGER) == 0) {
    //                         w2 *= 0.1;
    //                     }
    //                     val += sign * w1;
    //                     val += sign * w2;
    // #ifdef DEBUG_STREAM_ORDERING
    //                     if (DEBUGCOND && DEBUGEDGE(e2) && DEBUGEDGE(e1)) {
    //                         std::cout << "      sign=" << sign << " w1=" << w1 << " w2=" << w2 << " val=" << val
    //                                   << " c1=" << (*e1c).getDescription(e1)
    //                                   << " c2=" << (*e2c).getDescription(e2)
    //                                   << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //     }
    // #ifdef DEBUG_STREAM_ORDERING
    //     if (DEBUGCOND && DEBUGEDGE(e2) && DEBUGEDGE(e1)) {
    //         std::cout << "     computeUnblockedWeightedStreamNumber e1=" << e1->getID() << " e2=" << e2->getID() << " val=" << val << "\n";
    //     }
    // #endif
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn compute_unblocked_weighted_stream_number(&mut self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> f64 {
        todo!("port NBOwnTLDef::computeUnblockedWeightedStreamNumber")
    }

    /// C++: `std::pair<NBEdge *, NBEdge *> NBOwnTLDef::getBestCombination(const EdgeVector &)` [protected]
    /// Returns the combination of two edges from the given which has most unblocked streams
    // std::pair<NBEdge*, NBEdge*>
    // NBOwnTLDef::getBestCombination(const EdgeVector& edges) {
    //     std::pair<NBEdge*, NBEdge*> bestPair(static_cast<NBEdge*>(nullptr), static_cast<NBEdge*>(nullptr));
    //     double bestValue = -std::numeric_limits<double>::max();
    //     for (EdgeVector::const_iterator i = edges.begin(); i != edges.end(); ++i) {
    //         for (EdgeVector::const_iterator j = i + 1; j != edges.end(); ++j) {
    //             const double value = computeUnblockedWeightedStreamNumber(*i, *j);
    //             if (value > bestValue) {
    //                 bestValue = value;
    //                 bestPair = std::pair<NBEdge*, NBEdge*>(*i, *j);
    //             } else if (value == bestValue) {
    //                 const double ca = GeomHelper::getMinAngleDiff((*i)->getAngleAtNode((*i)->getToNode()), (*j)->getAngleAtNode((*j)->getToNode()));
    //                 const double oa = GeomHelper::getMinAngleDiff(bestPair.first->getAngleAtNode(bestPair.first->getToNode()), bestPair.second->getAngleAtNode(bestPair.second->getToNode()));
    //                 if (fabs(oa - ca) < NUMERICAL_EPS) { // break ties by id
    //                     if (bestPair.first->getID() < (*i)->getID()) {
    //                         bestPair = std::pair<NBEdge*, NBEdge*>(*i, *j);
    //                     }
    //                 } else if (oa < ca) {
    //                     bestPair = std::pair<NBEdge*, NBEdge*>(*i, *j);
    //                 }
    //             }
    //         }
    //     }
    //     if (bestValue < 0 || (bestValue == 0 && !hasStraightConnection(bestPair.first) && !hasStraightConnection(bestPair.second))) {
    //         // do not group edges
    //         if (bestPair.first->getPriority() < bestPair.second->getPriority()) {
    //             std::swap(bestPair.first, bestPair.second);
    //         }
    //         bestPair.second = nullptr;
    //     }
    // #ifdef DEBUG_STREAM_ORDERING
    //     if (DEBUGCOND) {
    //         std::cout << "   getBestCombination bestValue=" << bestValue << "  best=" << Named::getIDSecure(bestPair.first) << ", " << Named::getIDSecure(bestPair.second) << "\n";
    //     }
    // #endif
    //     return bestPair;
    // }
    #[allow(unused_variables)]
    pub fn get_best_combination(&mut self, edges: &EdgeVector) -> (Ptr<NBEdge>, Ptr<NBEdge>) {
        todo!("port NBOwnTLDef::getBestCombination")
    }

    /// C++: `std::pair<NBEdge *, NBEdge *> NBOwnTLDef::getBestPair(EdgeVector &)` [protected]
    /// Returns the combination of two edges from the given which has most unblocked streams
    // std::pair<NBEdge*, NBEdge*>
    // NBOwnTLDef::getBestPair(EdgeVector& incoming) {
    //     if (incoming.size() == 1) {
    //         // only one there - return the one
    //         std::pair<NBEdge*, NBEdge*> ret(*incoming.begin(), static_cast<NBEdge*>(nullptr));
    //         incoming.clear();
    //         return ret;
    //     }
    //     // determine the best combination
    //     //  by priority, first
    //     EdgeVector used;
    //     std::sort(incoming.begin(), incoming.end(), edge_by_incoming_priority_sorter());
    //     used.push_back(*incoming.begin()); // the first will definitely be used
    //     // get the ones with the same priority
    //     int prio = getToPrio(*used.begin());
    //     for (EdgeVector::iterator i = incoming.begin() + 1; i != incoming.end() && prio == getToPrio(*i); ++i) {
    //         used.push_back(*i);
    //     }
    //     //  if there only lower priorised, use these, too
    //     if (used.size() < 2) {
    //         used = incoming;
    //     }
    //     std::pair<NBEdge*, NBEdge*> ret = getBestCombination(used);
    // #ifdef DEBUG_STREAM_ORDERING
    //     if (DEBUGCOND) {
    //         std::cout << "getBestPair tls=" << getID() << " incoming=" << toString(incoming) << " prio=" << prio << " used=" << toString(used) << " best=" << Named::getIDSecure(ret.first) << ", " << Named::getIDSecure(ret.second) << "\n";
    //     }
    // #endif
    //
    //     incoming.erase(find(incoming.begin(), incoming.end(), ret.first));
    //     if (ret.second != nullptr) {
    //         incoming.erase(find(incoming.begin(), incoming.end(), ret.second));
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_best_pair(&mut self, incoming: &mut EdgeVector) -> (Ptr<NBEdge>, Ptr<NBEdge>) {
        todo!("port NBOwnTLDef::getBestPair")
    }

    /// C++: `bool NBOwnTLDef::hasCrossing(const NBEdge *, const NBEdge *, const std::vector<NBNode::Crossing *> &)` [static, protected]
    /// compute whether the given connection is crossed by pedestrians
    // bool
    // NBOwnTLDef::hasCrossing(const NBEdge* from, const NBEdge* to, const std::vector<NBNode::Crossing*>& crossings) {
    //     assert(to != 0);
    //     for (auto c : crossings) {
    //         const NBNode::Crossing& cross = *c;
    //         // only check connections at this crossings node
    //         if (to->getFromNode() == cross.node) {
    //             for (EdgeVector::const_iterator it_e = cross.edges.begin(); it_e != cross.edges.end(); ++it_e) {
    //                 const NBEdge* edge = *it_e;
    //                 if (edge == from || edge == to) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_crossing(from: Ptr<NBEdge>, to: Ptr<NBEdge>, crossings: &Vec<Ptr<NBNode_Crossing>>) -> bool {
        todo!("port NBOwnTLDef::hasCrossing")
    }

    /// C++: `EdgeVector NBOwnTLDef::getConnectedOuterEdges(const EdgeVector &)` [static, protected]
    /// get edges that have connections
    // EdgeVector
    // NBOwnTLDef::getConnectedOuterEdges(const EdgeVector& incoming) {
    //     EdgeVector result = incoming;
    //     for (EdgeVector::iterator it = result.begin(); it != result.end();) {
    //         if ((*it)->getConnections().size() == 0 || (*it)->isInsideTLS()) {
    //             it = result.erase(it);
    //         } else {
    //             ++it;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_connected_outer_edges(incoming: &EdgeVector) -> EdgeVector {
        todo!("port NBOwnTLDef::getConnectedOuterEdges")
    }

    /// C++: `std::string NBOwnTLDef::allowCompatible(std::string, const EdgeVector &, const EdgeVector &, const std::vector<int> &, const std::vector<int> &)` [protected]
    /// allow connections that are compatible with the chosen edges
    // std::string
    // NBOwnTLDef::allowCompatible(std::string state, const EdgeVector& fromEdges, const EdgeVector& toEdges,
    //                             const std::vector<int>& fromLanes, const std::vector<int>& toLanes) {
    //     state = allowSingleEdge(state, fromEdges);
    // #ifdef DEBUG_PHASES
    //     if (DEBUGCOND) {
    //         std::cout << " state after allowSingle " << state << "\n";
    //     }
    // #endif
    //     if (myControlledNodes.size() > 1) {
    //         state = allowFollowers(state, fromEdges, toEdges);
    // #ifdef DEBUG_PHASES
    //         if (DEBUGCOND) {
    //             std::cout << " state after allowFollowers " << state << "\n";
    //         }
    // #endif
    //         state = allowPredecessors(state, fromEdges, toEdges, fromLanes, toLanes);
    // #ifdef DEBUG_PHASES
    //         if (DEBUGCOND) {
    //             std::cout << " state after allowPredecessors " << state << "\n";
    //         }
    // #endif
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn allow_compatible(&mut self, state: String, fromEdges: &EdgeVector, toEdges: &EdgeVector, fromLanes: &Vec<i32>, toLanes: &Vec<i32>) -> String {
        todo!("port NBOwnTLDef::allowCompatible")
    }

    /// C++: `std::string NBOwnTLDef::allowSingleEdge(std::string, const EdgeVector &)` [protected]
    // std::string
    // NBOwnTLDef::allowSingleEdge(std::string state, const EdgeVector& fromEdges) {
    //     // if only one edge has green, ensure sure that all connections from that edge are green
    //     const int size = (int)fromEdges.size();
    //     NBEdge* greenEdge = nullptr;
    //     for (int i1 = 0; i1 < size; ++i1) {
    //         if (state[i1] == 'G') {
    //             if (greenEdge == nullptr) {
    //                 greenEdge = fromEdges[i1];
    //             } else if (greenEdge != fromEdges[i1]) {
    //                 return state;
    //             }
    //         }
    //     }
    //     if (greenEdge != nullptr) {
    //         for (int i1 = 0; i1 < size; ++i1) {
    //             if (fromEdges[i1] == greenEdge) {
    //                 state[i1] = 'G';
    //             }
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn allow_single_edge(&mut self, state: String, fromEdges: &EdgeVector) -> String {
        todo!("port NBOwnTLDef::allowSingleEdge")
    }

    /// C++: `std::string NBOwnTLDef::allowFollowers(std::string, const EdgeVector &, const EdgeVector &)` [protected]
    // std::string
    // NBOwnTLDef::allowFollowers(std::string state, const EdgeVector& fromEdges, const EdgeVector& toEdges) {
    //     // check continuation within joined traffic lights
    //     bool check = true;
    //     while (check) {
    //         check = false;
    //         for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //             if (state[i1] == 'G') {
    //                 continue;
    //             }
    //             if (forbidden(state, i1, fromEdges, toEdges, true)) {
    //                 continue;
    //             }
    //             bool followsChosen = false;
    //             for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //                 if (state[i2] == 'G' && fromEdges[i1] == toEdges[i2]) {
    //                     followsChosen = true;
    //                     break;
    //                 }
    //             }
    //             if (followsChosen) {
    //                 state[i1] = 'G';
    //                 check = true;
    //             }
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn allow_followers(&mut self, state: String, fromEdges: &EdgeVector, toEdges: &EdgeVector) -> String {
        todo!("port NBOwnTLDef::allowFollowers")
    }

    /// C++: `std::string NBOwnTLDef::allowPredecessors(std::string, const EdgeVector &, const EdgeVector &, const std::vector<int> &, const std::vector<int> &)` [protected]
    // std::string
    // NBOwnTLDef::allowPredecessors(std::string state, const EdgeVector& fromEdges, const EdgeVector& toEdges,
    //                               const std::vector<int>& fromLanes, const std::vector<int>& toLanes) {
    //     // also allow predecessors of chosen edges if the lanes match and there is no conflict
    //     // (must be done after the followers are done because followers are less specific)
    //     bool check = true;
    //     while (check) {
    //         check = false;
    //         for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //             if (state[i1] == 'G') {
    //                 continue;
    //             }
    //             if (forbidden(state, i1, fromEdges, toEdges, false)) {
    //                 continue;
    //             }
    //             bool preceedsChosen = false;
    //             for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //                 if (state[i2] == 'G' && fromEdges[i2] == toEdges[i1]
    //                         && fromLanes[i2] == toLanes[i1]) {
    //                     preceedsChosen = true;
    //                     break;
    //                 }
    //             }
    //             if (preceedsChosen) {
    //                 state[i1] = 'G';
    //                 check = true;
    //             }
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn allow_predecessors(&mut self, state: String, fromEdges: &EdgeVector, toEdges: &EdgeVector, fromLanes: &Vec<i32>, toLanes: &Vec<i32>) -> String {
        todo!("port NBOwnTLDef::allowPredecessors")
    }

    /// C++: `std::string NBOwnTLDef::allowUnrelated(std::string, const EdgeVector &, const EdgeVector &, const std::vector<bool> &, const std::vector<NBNode::Crossing *> &)` [protected]
    // std::string
    // NBOwnTLDef::allowUnrelated(std::string state, const EdgeVector& fromEdges, const EdgeVector& toEdges,
    //                            const std::vector<bool>& isTurnaround,
    //                            const std::vector<NBNode::Crossing*>& crossings) {
    //     for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //         if (state[i1] == 'G') {
    //             continue;
    //         }
    //         bool isForbidden = false;
    //         for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //             if (state[i2] == 'G' && !isTurnaround[i2] &&
    //                     (forbids(fromEdges[i2], toEdges[i2], fromEdges[i1], toEdges[i1], true) || forbids(fromEdges[i1], toEdges[i1], fromEdges[i2], toEdges[i2], true))) {
    //                 isForbidden = true;
    //                 break;
    //             }
    //         }
    //         if (!isForbidden && !hasCrossing(fromEdges[i1], toEdges[i1], crossings)) {
    //             state[i1] = 'G';
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn allow_unrelated(&mut self, state: String, fromEdges: &EdgeVector, toEdges: &EdgeVector, isTurnaround: &Vec<bool>, crossings: &Vec<Ptr<NBNode_Crossing>>) -> String {
        todo!("port NBOwnTLDef::allowUnrelated")
    }

    /// C++: `std::string NBOwnTLDef::allowByVClass(std::string, const EdgeVector &, const EdgeVector &, SVCPermissions)` [protected]
    // std::string
    // NBOwnTLDef::allowByVClass(std::string state, const EdgeVector& fromEdges, const EdgeVector& toEdges, SVCPermissions perm) {
    //     for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //         SVCPermissions linkPerm = (fromEdges[i1]->getPermissions() & toEdges[i1]->getPermissions());
    //         if ((linkPerm & ~perm) == 0) {
    //             state[i1] = 'G';
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn allow_by_v_class(&mut self, state: String, fromEdges: &EdgeVector, toEdges: &EdgeVector, perm: SVCPermissions) -> String {
        todo!("port NBOwnTLDef::allowByVClass")
    }

    /// C++: `bool NBOwnTLDef::forbidden(const std::string &, int, const EdgeVector &, const EdgeVector &, bool)` [protected]
    /// whether the given index is forbidden by a green link in the current state
    // bool
    // NBOwnTLDef::forbidden(const std::string& state, int index, const EdgeVector& fromEdges, const EdgeVector& toEdges, bool allowCont) {
    //     for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //         if (state[i2] == 'G' && foes(fromEdges[i2], toEdges[i2], fromEdges[index], toEdges[index])) {
    //             if (!allowCont || (
    //                         !needsCont(fromEdges[i2], toEdges[i2], fromEdges[index], toEdges[index]) &&
    //                         !needsCont(fromEdges[index], toEdges[index], fromEdges[i2], toEdges[i2]))) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn forbidden(&mut self, state: &str, index: i32, fromEdges: &EdgeVector, toEdges: &EdgeVector, allowCont: bool) -> bool {
        todo!("port NBOwnTLDef::forbidden")
    }

    /// C++: `std::string NBOwnTLDef::correctConflicting(std::string, const EdgeVector &, const EdgeVector &, const std::vector<bool> &, const std::vector<int> &, const std::vector<int> &, const std::vector<bool> &, bool &, std::vector<bool> &, std::vector<bool> &)` [protected]
    /// change 'G' to 'g' for conflicting connections
    // std::string
    // NBOwnTLDef::correctConflicting(std::string state, const EdgeVector& fromEdges, const EdgeVector& toEdges,
    //                                const std::vector<bool>& isTurnaround,
    //                                const std::vector<int>& fromLanes,
    //                                const std::vector<int>& toLanes,
    //                                const std::vector<bool>& hadGreenMajor,
    //                                bool& haveForbiddenLeftMover,
    //                                std::vector<bool>& rightTurnConflicts,
    //                                std::vector<bool>& mergeConflicts) {
    //     const bool controlledWithin = !OptionsCont::getOptions().getBool("tls.uncontrolled-within");
    //     for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //         if (state[i1] == 'G') {
    //             for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //                 if ((state[i2] == 'G' || state[i2] == 'g')) {
    //                     if (NBNode::rightTurnConflict(
    //                                 fromEdges[i1], toEdges[i1], fromLanes[i1], fromEdges[i2], toEdges[i2], fromLanes[i2])) {
    //                         rightTurnConflicts[i1] = true;
    //                     }
    //                     if (forbids(fromEdges[i2], toEdges[i2], fromEdges[i1], toEdges[i1], true, controlledWithin) || rightTurnConflicts[i1]) {
    //                         state[i1] = 'g';
    //                         if (myControlledNodes.size() == 1) {
    //                             myNeedsContRelation.insert(StreamPair(fromEdges[i1], toEdges[i1], fromEdges[i2], toEdges[i2]));
    // #ifdef DEBUG_CONTRELATION
    //                             if (DEBUGCOND) {
    //                                 std::cout << getID() << " p=" << getProgramID() << " contRel: " << fromEdges[i1]->getID() << "->" << toEdges[i1]->getID()
    //                                           << " foe " << fromEdges[i2]->getID() << "->" << toEdges[i2]->getID() << "\n";
    //                             }
    // #endif
    //                         }
    //                         if (!isTurnaround[i1] && !hadGreenMajor[i1] && !rightTurnConflicts[i1]) {
    //                             haveForbiddenLeftMover = true;
    //                         }
    //                     } else if (fromEdges[i1] == fromEdges[i2]
    //                                && fromLanes[i1] != fromLanes[i2]
    //                                && toEdges[i1] == toEdges[i2]
    //                                && toLanes[i1] == toLanes[i2]
    //                                && fromEdges[i1]->getToNode()->mergeConflictYields(fromEdges[i1], fromLanes[i1], fromLanes[i2], toEdges[i1], toLanes[i1])) {
    //                         mergeConflicts[i1] = true;
    //                         state[i1] = 'g';
    //                     }
    //                 }
    //             }
    //         }
    //         if (state[i1] == 'r') {
    //             if (fromEdges[i1]->getToNode()->getType() == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED &&
    //                     fromEdges[i1]->getToNode()->getDirection(fromEdges[i1], toEdges[i1]) == LinkDirection::RIGHT) {
    //                 state[i1] = 's';
    //                 // do not allow right-on-red when in conflict with exclusive left-turn phase
    //                 for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //                     if (state[i2] == 'G' && !isTurnaround[i2] &&
    //                             (forbids(fromEdges[i2], toEdges[i2], fromEdges[i1], toEdges[i1], true) ||
    //                              forbids(fromEdges[i1], toEdges[i1], fromEdges[i2], toEdges[i2], true))) {
    //                         const LinkDirection foeDir = fromEdges[i2]->getToNode()->getDirection(fromEdges[i2], toEdges[i2]);
    //                         if (foeDir == LinkDirection::LEFT || foeDir == LinkDirection::PARTLEFT) {
    //                             state[i1] = 'r';
    //                             break;
    //                         }
    //                     }
    //                 }
    //                 if (state[i1] == 's') {
    //                     // handle right-on-red conflicts
    //                     for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //                         if (state[i2] == 'G' && !isTurnaround[i2] &&
    //                                 (forbids(fromEdges[i2], toEdges[i2], fromEdges[i1], toEdges[i1], true) ||
    //                                  forbids(fromEdges[i1], toEdges[i1], fromEdges[i2], toEdges[i2], true))) {
    //                             myExtraConflicts.insert(std::make_pair(i1, i2));
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn correct_conflicting(&mut self, state: String, fromEdges: &EdgeVector, toEdges: &EdgeVector, isTurnaround: &Vec<bool>, fromLanes: &Vec<i32>, toLanes: &Vec<i32>, hadGreenMajor: &Vec<bool>, haveForbiddenLeftMover: &mut bool, rightTurnConflicts: &mut Vec<bool>, mergeConflicts: &mut Vec<bool>) -> String {
        todo!("port NBOwnTLDef::correctConflicting")
    }

    /// C++: `std::string NBOwnTLDef::correctMixed(std::string, const EdgeVector &, const std::vector<int> &, bool &, std::vector<bool> &)` [protected]
    /// prevent green and red from the same lane
    // std::string
    // NBOwnTLDef::correctMixed(std::string state, const EdgeVector& fromEdges,
    //                          const std::vector<int>& fromLanes,
    //                          bool& buildMixedGreenPhase, std::vector<bool>& mixedGreen) {
    //     for (int i1 = 0; i1 < (int)fromEdges.size(); ++i1) {
    //         if ((state[i1] == 'G' || state[i1] == 'g')) {
    //             for (int i2 = 0; i2 < (int)fromEdges.size(); ++i2) {
    //                 if (i1 != i2 && fromEdges[i1] == fromEdges[i2] && fromLanes[i1] == fromLanes[i2]
    //                         && state[i2] != 'G' && state[i2] != 'g') {
    //                     state[i1] = state[i2];
    //                     //std::cout << " mixedGreen i1=" << i1 << " i2=" << i2 << "\n";
    //                     mixedGreen[i1] = true;
    //                     if (fromEdges[i1]->getNumLanesThatAllow(SVC_PASSENGER) > 1) {
    //                         buildMixedGreenPhase = true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn correct_mixed(&mut self, state: String, fromEdges: &EdgeVector, fromLanes: &Vec<i32>, buildMixedGreenPhase: &mut bool, mixedGreen: &mut Vec<bool>) -> String {
        todo!("port NBOwnTLDef::correctMixed")
    }

    /// C++: `int NBOwnTLDef::maxCrossingIndex(const NBNode *) const` [const, protected]
    /// find maximum crossing index
    // int
    // NBOwnTLDef::maxCrossingIndex(const NBNode* node) const {
    //     int result = 0;
    //     for (auto crossing : node->getCrossings()) {
    //         result = MAX2(result, crossing->tlLinkIndex);
    //         result = MAX2(result, crossing->tlLinkIndex2);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn max_crossing_index(&self, node: Ptr<NBNode>) -> i32 {
        todo!("port NBOwnTLDef::maxCrossingIndex")
    }

    /// C++: `void NBOwnTLDef::fixSuperfluousYellow(NBTrafficLightLogic *) const` [const, protected]
    /// avoid yellow signal between successive green (major) phases
    // void
    // NBOwnTLDef::fixSuperfluousYellow(NBTrafficLightLogic* logic) const {
    //     // assume that yellow states last at most one phase
    //     const int n = logic->getNumLinks();
    //     const int p = (int)logic->getPhases().size();
    //     for (int i1 = 0; i1 < n; ++i1) {
    //         LinkState prev = (LinkState)logic->getPhases().back().state[i1];
    //         for (int i2 = 0; i2 < p; ++i2) {
    //             LinkState cur = (LinkState)logic->getPhases()[i2].state[i1];
    //             LinkState next = (LinkState)logic->getPhases()[(i2 + 1) % p].state[i1];
    //             if (cur == LINKSTATE_TL_YELLOW_MINOR
    //                     && (prev == LINKSTATE_TL_GREEN_MAJOR || prev == LINKSTATE_TL_YELLOW_MINOR)
    //                     && next == LINKSTATE_TL_GREEN_MAJOR) {
    //                 logic->setPhaseState(i2, i1, prev);
    //             }
    //             prev = cur;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fix_superfluous_yellow(&self, logic: Ptr<NBTrafficLightLogic>) {
        todo!("port NBOwnTLDef::fixSuperfluousYellow")
    }

    /// C++: `void NBOwnTLDef::deactivateAlwaysGreen(NBTrafficLightLogic *) const` [const, protected]
    /// switch of signal for links that are always green
    // void
    // NBOwnTLDef::deactivateAlwaysGreen(NBTrafficLightLogic* logic) const {
    //     const int n = logic->getNumLinks();
    //     std::vector<bool> alwaysGreen(n, true);
    //     for (int i1 = 0; i1 < n; ++i1) {
    //         for (const auto& phase : logic->getPhases()) {
    //             if (phase.state[i1] != 'G') {
    //                 alwaysGreen[i1] = false;
    //                 break;
    //             }
    //         }
    //     }
    //     const int p = (int)logic->getPhases().size();
    //     for (int i1 = 0; i1 < n; ++i1) {
    //         if (alwaysGreen[i1]) {
    //             for (int i2 = 0; i2 < p; ++i2) {
    //                 logic->setPhaseState(i2, i1, LINKSTATE_TL_OFF_NOSIGNAL);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deactivate_always_green(&self, logic: Ptr<NBTrafficLightLogic>) {
        todo!("port NBOwnTLDef::deactivateAlwaysGreen")
    }

    /// C++: `void NBOwnTLDef::deactivateInsideEdges(NBTrafficLightLogic *, const EdgeVector &) const` [const, protected]
    /// switch of signal for links that are inside a joined tls
    // void
    // NBOwnTLDef::deactivateInsideEdges(NBTrafficLightLogic* logic, const EdgeVector& fromEdges) const {
    //     const int n = (int)fromEdges.size();
    //     const int p = (int)logic->getPhases().size();
    //     for (int i1 = 0; i1 < n; ++i1) {
    //         if (fromEdges[i1]->isInsideTLS()) {
    //             for (int i2 = 0; i2 < p; ++i2) {
    //                 logic->setPhaseState(i2, i1, LINKSTATE_TL_OFF_NOSIGNAL);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deactivate_inside_edges(&self, logic: Ptr<NBTrafficLightLogic>, fromEdges: &EdgeVector) {
        todo!("port NBOwnTLDef::deactivateInsideEdges")
    }

    /// C++: `SUMOTime NBOwnTLDef::computeEscapeTime(const std::string &, const EdgeVector &, const EdgeVector &) const` [const, protected]
    /// compute time to clear all vehicles from within an alternateOneWay layout
    // SUMOTime
    // NBOwnTLDef::computeEscapeTime(const std::string& state, const EdgeVector& fromEdges, const EdgeVector& toEdges) const {
    //     const int n = (int)fromEdges.size();
    //     double maxTime = 0;
    //     for (int i1 = 0; i1 < n; ++i1) {
    //         if (state[i1] == 'y' && !fromEdges[i1]->isInsideTLS()) {
    //             for (int i2 = 0; i2 < n; ++i2) {
    //                 if (fromEdges[i2]->isInsideTLS()) {
    //                     double gapSpeed = (toEdges[i1]->getSpeed() + fromEdges[i2]->getSpeed()) / 2;
    //                     double time = fromEdges[i1]->getGeometry().back().distanceTo2D(fromEdges[i2]->getGeometry().back()) / gapSpeed;
    //                     maxTime = MAX2(maxTime, time);
    //                 }
    //             }
    //         }
    //     }
    //     // give some slack
    //     return TIME2STEPS(floor(maxTime * 1.2 + 5));
    // }
    #[allow(unused_variables)]
    pub fn compute_escape_time(&self, state: &str, fromEdges: &EdgeVector, toEdges: &EdgeVector) -> SUMOTime {
        todo!("port NBOwnTLDef::computeEscapeTime")
    }

    /// C++: `bool NBOwnTLDef::hasStraightConnection(const NBEdge *)` [protected]
    /// check whether there is a straight connection from this edge
    // bool
    // NBOwnTLDef::hasStraightConnection(const NBEdge* fromEdge) {
    //     for (const NBEdge::Connection& c : fromEdge->getConnections()) {
    //         LinkDirection dir = fromEdge->getToNode()->getDirection(fromEdge, c.toEdge);
    //         if (dir == LinkDirection::STRAIGHT) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_straight_connection(&mut self, fromEdge: Ptr<NBEdge>) -> bool {
        todo!("port NBOwnTLDef::hasStraightConnection")
    }

}

/// Sorts edges by their priority within the node they end at
/// C++ class `NBOwnTLDef::edge_by_incoming_priority_sorter` (netbuild/NBOwnTLDef.h)
pub struct NBOwnTLDef_edge_by_incoming_priority_sorter {
}

impl NBOwnTLDef_edge_by_incoming_priority_sorter {
    /// C++: `int edge_by_incoming_priority_sorter::operator()(const NBEdge *const, const NBEdge *const) const` [const]
    /// comparing operator
    // int operator()(const NBEdge* const e1, const NBEdge* const e2) const {
    //             if (e1->getJunctionPriority(e1->getToNode()) != e2->getJunctionPriority(e2->getToNode())) {
    //                 return e1->getJunctionPriority(e1->getToNode()) > e2->getJunctionPriority(e2->getToNode());
    //             }
    //             return e1->getID() > e2->getID();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> i32 {
        todo!("port edge_by_incoming_priority_sorter::operator()")
    }

}

