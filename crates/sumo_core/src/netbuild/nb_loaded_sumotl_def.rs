//! Port of netbuild/NBLoadedSUMOTLDef.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A loaded (complete) traffic light logic
/// C++ class `NBLoadedSUMOTLDef` (netbuild/NBLoadedSUMOTLDef.h)
#[repr(C)]
pub struct NBLoadedSUMOTLDef {
    /// C++ base class `NBTrafficLightDefinition`
    pub base: NBTrafficLightDefinition,
    /// phases are added directly to myTLLogic which is then returned in myCompute()
    pub myTLLogic: Ptr<NBTrafficLightLogic>,
    /// set of edges with shifted lane indices (to avoid shifting twice)
    pub myShifted: BTreeSet<Ptr<NBEdge>>,
    /// whether the logic must be reconstructed
    pub myReconstructAddedConnections: bool,
    pub myReconstructRemovedConnections: bool,
    pub myPhasesLoaded: bool,
}

impl std::ops::Deref for NBLoadedSUMOTLDef {
    type Target = NBTrafficLightDefinition;
    fn deref(&self) -> &NBTrafficLightDefinition {
        &self.base
    }
}
impl std::ops::DerefMut for NBLoadedSUMOTLDef {
    fn deref_mut(&mut self) -> &mut NBTrafficLightDefinition {
        &mut self.base
    }
}

impl NBLoadedSUMOTLDef {
    /// C++: `void NBLoadedSUMOTLDef::NBLoadedSUMOTLDef(const std::string &, const std::string &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBLoadedSUMOTLDef::NBLoadedSUMOTLDef(const std::string& id, const std::string& programID,
    //                                      SUMOTime offset, TrafficLightType type) :
    //     NBTrafficLightDefinition(id, programID, offset, type),
    //     myTLLogic(nullptr),
    //     myReconstructAddedConnections(false),
    //     myReconstructRemovedConnections(false),
    //     myPhasesLoaded(false) {
    //     myTLLogic = new NBTrafficLightLogic(id, programID, 0, offset, type);
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, programID: &str, offset: SUMOTime, type_: TrafficLightType) -> Self {
        todo!("port NBLoadedSUMOTLDef::NBLoadedSUMOTLDef")
    }

    /// C++: `void NBLoadedSUMOTLDef::NBLoadedSUMOTLDef(const NBTrafficLightDefinition &, const NBTrafficLightLogic &)`
    /// Constructor that copies from an existing definition and its computed logic (used by netedit)
    // NBLoadedSUMOTLDef::NBLoadedSUMOTLDef(const NBTrafficLightDefinition& def, const NBTrafficLightLogic& logic) :
    //     // allow for adding a new program for the same def: take the offset and programID from the new logic
    //     NBTrafficLightDefinition(def.getID(), logic.getProgramID(), logic.getOffset(), def.getType()),
    //     myTLLogic(new NBTrafficLightLogic(logic)),
    //     myReconstructAddedConnections(false),
    //     myReconstructRemovedConnections(false),
    //     myPhasesLoaded(false) {
    //     assert(def.getType() == logic.getType());
    //     myControlledLinks = def.getControlledLinks();
    //     myControlledNodes = def.getNodes();
    //     const NBLoadedSUMOTLDef* sumoDef = dynamic_cast<const NBLoadedSUMOTLDef*>(&def);
    //     updateParameters(def.getParametersMap());
    //     if (sumoDef != nullptr) {
    //         myReconstructAddedConnections = sumoDef->myReconstructAddedConnections;
    //         myReconstructRemovedConnections = sumoDef->myReconstructRemovedConnections;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new_2(def: &NBTrafficLightDefinition, logic: &NBTrafficLightLogic) -> Self {
        todo!("port NBLoadedSUMOTLDef::NBLoadedSUMOTLDef")
    }

    /// C++: `void NBLoadedSUMOTLDef::~NBLoadedSUMOTLDef() noexcept` [virtual]
    /// Destructor
    // NBLoadedSUMOTLDef::~NBLoadedSUMOTLDef() {
    //     delete myTLLogic;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBLoadedSUMOTLDef::~NBLoadedSUMOTLDef")
    }

    /// C++: `void NBLoadedSUMOTLDef::setID(const std::string &)` [virtual]
    // void
    // NBLoadedSUMOTLDef::setID(const std::string& newID) {
    //     Named::setID(newID);
    //     myTLLogic->setID(newID);
    // }
    #[allow(unused_variables)]
    pub fn set_id(&mut self, newID: &str) {
        todo!("port NBLoadedSUMOTLDef::setID")
    }

    /// C++: `void NBLoadedSUMOTLDef::setProgramID(const std::string &)` [virtual]
    /// Sets the programID
    // void
    // NBLoadedSUMOTLDef::setProgramID(const std::string& programID) {
    //     NBTrafficLightDefinition::setProgramID(programID);
    //     myTLLogic->setProgramID(programID);
    // }
    #[allow(unused_variables)]
    pub fn set_program_id(&mut self, programID: &str) {
        todo!("port NBLoadedSUMOTLDef::setProgramID")
    }

    /// C++: `void NBLoadedSUMOTLDef::setTLControllingInformation() const` [const, virtual]
    /// Informs edges about being controlled by a tls
    // void
    // NBLoadedSUMOTLDef::setTLControllingInformation() const {
    //     if (myReconstructAddedConnections) {
    //         NBOwnTLDef dummy(DummyID, myControlledNodes, 0, getType());
    //         dummy.setParticipantsInformation();
    //         dummy.setTLControllingInformation();
    //         for (NBNode* const n : myControlledNodes) {
    //             n->removeTrafficLight(&dummy);
    //         }
    //     }
    //     if (myReconstructRemovedConnections) {
    //         return; // will be called again in reconstructLogic()
    //     }
    //     // if nodes have been removed our links may have been invalidated as well
    //     // since no logic will be built anyway there is no reason to inform any edges
    //     if (amInvalid()) {
    //         return;
    //     }
    //     // set the information about the link's positions within the tl into the
    //     //  edges the links are starting at, respectively
    //     for (const NBConnection& c : myControlledLinks) {
    //         if (c.getTLIndex() >= myTLLogic->getNumLinks()) {
    //             throw ProcessError("Invalid linkIndex " + toString(c.getTLIndex()) + " for traffic light '" + getID() +
    //                                "' with " + toString(myTLLogic->getNumLinks()) + " links.");
    //         }
    //         NBEdge* edge = c.getFrom();
    //         if (edge != nullptr && edge->getNumLanes() > c.getFromLane()) {
    //             // logic may have yet to be reconstructed
    //             edge->setControllingTLInformation(c, getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_tl_controlling_information(&self) {
        todo!("port NBLoadedSUMOTLDef::setTLControllingInformation")
    }

    /// C++: `void NBLoadedSUMOTLDef::remapRemoved(NBEdge *, const EdgeVector &, const EdgeVector &)` [virtual]
    /// Replaces occurrences of the removed edge in incoming/outgoing edges of all definitions
    // void
    // NBLoadedSUMOTLDef::remapRemoved(NBEdge*, const EdgeVector&, const EdgeVector&) {}
    #[allow(unused_variables)]
    pub fn remap_removed(&mut self, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
        todo!("port NBLoadedSUMOTLDef::remapRemoved")
    }

    /// C++: `void NBLoadedSUMOTLDef::replaceRemoved(NBEdge *, int, NBEdge *, int, bool)` [virtual]
    /// Replaces a removed edge/lane
    // void
    // NBLoadedSUMOTLDef::replaceRemoved(NBEdge* removed, int removedLane, NBEdge* by, int byLane, bool incoming) {
    //     if (by == nullptr) {
    //         myReconstructRemovedConnections = true;
    //     }
    //     for (NBConnectionVector::iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); ++it) {
    //         if (incoming) {
    //             (*it).replaceFrom(removed, removedLane, by, byLane);
    //         } else {
    //             (*it).replaceTo(removed, removedLane, by, byLane);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_removed(&mut self, removed: Ptr<NBEdge>, removedLane: i32, by: Ptr<NBEdge>, byLane: i32, incoming: bool) {
        todo!("port NBLoadedSUMOTLDef::replaceRemoved")
    }

    /// C++: `void NBLoadedSUMOTLDef::shiftTLConnectionLaneIndex(NBEdge *, int, int)` [virtual]
    /// patches signal plans by modifying lane indices with the given offset, only indices with a value above threshold are modified
    // void
    // NBLoadedSUMOTLDef::shiftTLConnectionLaneIndex(NBEdge* edge, int offset, int threshold) {
    //     // avoid shifting twice if the edge is incoming and outgoing to a joined TLS
    //     if (myShifted.count(edge) == 0) {
    //         /// XXX what if an edge should really be shifted twice?
    //         myShifted.insert(edge);
    //         for (NBConnectionVector::iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); it++) {
    //             (*it).shiftLaneIndex(edge, offset, threshold);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn shift_tl_connection_lane_index(&mut self, edge: Ptr<NBEdge>, offset: i32, threshold: i32) {
        todo!("port NBLoadedSUMOTLDef::shiftTLConnectionLaneIndex")
    }

    /// C++: `void NBLoadedSUMOTLDef::addPhase(const SUMOTime, const std::string &, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const std::vector<int> &, const std::string &)`
    /// Adds a phase to the logic the new phase is inserted at the end of the list of already added phases
    // void
    // NBLoadedSUMOTLDef::addPhase(const SUMOTime duration, const std::string& state, const SUMOTime minDur, const SUMOTime maxDur,
    //                             const SUMOTime earliestEnd, const SUMOTime latestEnd, const SUMOTime vehExt, const SUMOTime yellow,
    //                             const SUMOTime red, const std::vector<int>& next, const std::string& name) {
    //     myTLLogic->addStep(duration, state, minDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, name, next);
    // }
    #[allow(unused_variables)]
    pub fn add_phase(&mut self, duration: SUMOTime, state: &str, minDur: SUMOTime, maxDur: SUMOTime, earliestEnd: SUMOTime, latestEnd: SUMOTime, vehExt: SUMOTime, yellow: SUMOTime, red: SUMOTime, next: &Vec<i32>, name: &str) {
        todo!("port NBLoadedSUMOTLDef::addPhase")
    }

    /// C++: `void NBLoadedSUMOTLDef::phasesLoaded()`
    /// mark phases as load
    // void phasesLoaded() {
    //         myPhasesLoaded = true;
    //     }
    #[allow(unused_variables)]
    pub fn phases_loaded(&mut self) {
        todo!("port NBLoadedSUMOTLDef::phasesLoaded")
    }

    /// C++: `void NBLoadedSUMOTLDef::addConnection(NBEdge *, NBEdge *, int, int, int, int, bool)`
    /// Adds a connection and immediately informs the edges
    // void
    // NBLoadedSUMOTLDef::addConnection(NBEdge* from, NBEdge* to, int fromLane, int toLane, int linkIndex, int linkIndex2, bool reconstruct) {
    //     assert(myTLLogic->getNumLinks() > 0); // logic should be loaded by now
    //     if (linkIndex >= myTLLogic->getNumLinks()) {
    //         throw ProcessError("Invalid linkIndex " + toString(linkIndex) + " in connection from edge '" + from->getID() +
    //                            "' to edge '" + to->getID() + "' for traffic light '" + getID() +
    //                            "' with " + toString(myTLLogic->getNumLinks()) + " links.");
    //     }
    //     if (linkIndex2 >= myTLLogic->getNumLinks()) {
    //         throw ProcessError("Invalid linkIndex2 " + toString(linkIndex2) + " in connection from edge '" + from->getID() +
    //                            "' to edge '" + to->getID() + "' for traffic light '" + getID() +
    //                            "' with " + toString(myTLLogic->getNumLinks()) + " links.");
    //     }
    //     NBConnection conn(from, fromLane, to, toLane, linkIndex, linkIndex2);
    //     // avoid duplicates
    //     auto newEnd = remove_if(myControlledLinks.begin(), myControlledLinks.end(), connection_equal(conn));
    //     // remove_if does not remove, only re-order
    //     myControlledLinks.erase(newEnd, myControlledLinks.end());
    //     myControlledLinks.push_back(conn);
    //     addNode(from->getToNode());
    //     addNode(to->getFromNode());
    //     // added connections are definitely controlled. make sure none are removed because they lie within the tl
    //     // myControlledInnerEdges.insert(from->getID()); // @todo recheck: this appears to be obsolete
    //     // set this information now so that it can be used while loading diffs
    //     from->setControllingTLInformation(conn, getID());
    //     myReconstructAddedConnections |= reconstruct;
    // }
    #[allow(unused_variables)]
    pub fn add_connection(&mut self, from: Ptr<NBEdge>, to: Ptr<NBEdge>, fromLane: i32, toLane: i32, linkIndex: i32, linkIndex2: i32, reconstruct: bool) {
        todo!("port NBLoadedSUMOTLDef::addConnection")
    }

    /// C++: `void NBLoadedSUMOTLDef::removeConnection(const NBConnection &, bool)` [virtual]
    /// removes the given connection from the traffic light if recontruct=true, reconstructs the logic and informs the edges for immediate use in netedit
    // void
    // NBLoadedSUMOTLDef::removeConnection(const NBConnection& conn, bool reconstruct) {
    //     for (auto it = myControlledLinks.begin(); it != myControlledLinks.end();) {
    //         if ((it->getFrom() == conn.getFrom() &&
    //                 it->getTo() == conn.getTo() &&
    //                 it->getFromLane() == conn.getFromLane() &&
    //                 it->getToLane() == conn.getToLane())
    //                 || (it->getTLIndex() == conn.getTLIndex() &&
    //                     conn.getTLIndex() != conn.InvalidTlIndex &&
    //                     (it->getFrom() == nullptr || it->getTo() == nullptr))) {
    //             if (reconstruct) {
    //                 myReconstructRemovedConnections = true;
    //                 it++;
    //             } else {
    //                 it = myControlledLinks.erase(it);
    //             }
    //         } else {
    //             it++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_connection(&mut self, conn: &NBConnection, reconstruct: bool) {
        todo!("port NBLoadedSUMOTLDef::removeConnection")
    }

    /// C++: `void NBLoadedSUMOTLDef::registerModifications(bool, bool)`
    /// register changes that necessitate recomputation
    // void
    // NBLoadedSUMOTLDef::registerModifications(bool addedConnections, bool removedConnections) {
    //     myReconstructAddedConnections |= addedConnections;
    //     myReconstructRemovedConnections |= removedConnections;
    // }
    #[allow(unused_variables)]
    pub fn register_modifications(&mut self, addedConnections: bool, removedConnections: bool) {
        todo!("port NBLoadedSUMOTLDef::registerModifications")
    }

    /// C++: `NBTrafficLightLogic * NBLoadedSUMOTLDef::getLogic()`
    /// Returns the internal logic
    // NBTrafficLightLogic* getLogic() {
    //         return myTLLogic;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic(&mut self) -> Ptr<NBTrafficLightLogic> {
        todo!("port NBLoadedSUMOTLDef::getLogic")
    }

    /// C++: `void NBLoadedSUMOTLDef::setOffset(SUMOTime)`
    /// Sets the offset of this tls
    // void
    // NBLoadedSUMOTLDef::setOffset(SUMOTime offset) {
    //     myOffset = offset;
    //     myTLLogic->setOffset(offset);
    // }
    #[allow(unused_variables)]
    pub fn set_offset(&mut self, offset: SUMOTime) {
        todo!("port NBLoadedSUMOTLDef::setOffset")
    }

    /// C++: `void NBLoadedSUMOTLDef::setType(TrafficLightType)` [virtual]
    /// Sets the algorithm type of this tls
    // void
    // NBLoadedSUMOTLDef::setType(TrafficLightType type) {
    //     myType = type;
    //     myTLLogic->setType(type);
    // }
    #[allow(unused_variables)]
    pub fn set_type(&mut self, type_: TrafficLightType) {
        todo!("port NBLoadedSUMOTLDef::setType")
    }

    /// C++: `bool NBLoadedSUMOTLDef::extraConflict(int, int) const` [const, virtual]
    /// whether the given index must yield to the foeIndex while turing right on a red light
    // bool
    // NBLoadedSUMOTLDef::extraConflict(int index, int foeIndex) const {
    //     if (amInvalid()) {
    //         return false;
    //     }
    //     if (!myExtraConflictsReady) {
    //         initExtraConflicts();
    //         assert(myExtraConflictsReady);
    //     }
    //     return std::find(myExtraConflicts.begin(), myExtraConflicts.end(), std::make_pair(index, foeIndex)) != myExtraConflicts.end();
    // }
    #[allow(unused_variables)]
    pub fn extra_conflict(&self, index: i32, foeIndex: i32) -> bool {
        todo!("port NBLoadedSUMOTLDef::extraConflict")
    }

    /// C++: `bool NBLoadedSUMOTLDef::cleanupStates()`
    // bool
    // NBLoadedSUMOTLDef::cleanupStates() {
    //     const int maxIndex = getMaxValidIndex();
    //     std::vector<int> unusedIndices;
    //     for (int i = 0; i <= maxIndex; i++) {
    //         if (isUsed(i)) {
    //             if (unusedIndices.size() > 0) {
    //                 replaceIndex(i, i - (int)unusedIndices.size());
    //             }
    //         } else {
    //             unusedIndices.push_back(i);
    //         }
    //     }
    //     for (int i = (int)unusedIndices.size() - 1; i >= 0; i--) {
    //         myTLLogic->deleteStateIndex(unusedIndices[i]);
    //     }
    //     if (unusedIndices.size() > 0) {
    //         myTLLogic->setStateLength(maxIndex + 1 - (int)unusedIndices.size());
    //         setTLControllingInformation();
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cleanup_states(&mut self) -> bool {
        todo!("port NBLoadedSUMOTLDef::cleanupStates")
    }

    /// C++: `bool NBLoadedSUMOTLDef::usingSignalGroups() const` [const, virtual]
    /// whether this definition uses signal group (multiple connections with the same link index)
    // bool
    // NBLoadedSUMOTLDef::usingSignalGroups() const {
    //     // count how often each index is used
    //     std::map<int, int> indexUsage;
    //     for (const NBConnection& c : myControlledLinks) {
    //         indexUsage[c.getTLIndex()]++;
    //     }
    //     for (NBNode* n : myControlledNodes) {
    //         for (NBNode::Crossing* c : n->getCrossings()) {
    //             indexUsage[c->tlLinkIndex]++;
    //             indexUsage[c->tlLinkIndex2]++;
    //         }
    //     }
    //     for (auto it : indexUsage) {
    //         if (it.first >= 0 && it.second > 1) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn using_signal_groups(&self) -> bool {
        todo!("port NBLoadedSUMOTLDef::usingSignalGroups")
    }

    /// C++: `void NBLoadedSUMOTLDef::joinLogic(NBTrafficLightDefinition *)`
    /// join nodes and states from the given logic (append red state)
    // void
    // NBLoadedSUMOTLDef::joinLogic(NBTrafficLightDefinition* def) {
    //     def->setParticipantsInformation();
    //     def->compute(OptionsCont::getOptions());
    //     const int maxIndex = MAX2(getMaxIndex(), def->getMaxIndex());
    //     myTLLogic->setStateLength(maxIndex + 1);
    //     myControlledLinks.insert(myControlledLinks.end(), def->getControlledLinks().begin(), def->getControlledLinks().end());
    // }
    #[allow(unused_variables)]
    pub fn join_logic(&mut self, def: Ptr<NBTrafficLightDefinition>) {
        todo!("port NBLoadedSUMOTLDef::joinLogic")
    }

    /// C++: `void NBLoadedSUMOTLDef::guessMinMaxDuration()`
    /// heuristically add minDur and maxDur when switching from tlType fixed to actuated
    // void
    // NBLoadedSUMOTLDef::guessMinMaxDuration() {
    //     bool hasMinMaxDur = false;
    //     for (auto phase : myTLLogic->getPhases()) {
    //         if (phase.maxDur != UNSPECIFIED_DURATION) {
    //             //std::cout << " phase=" << phase.state << " maxDur=" << phase.maxDur << "\n";
    //             hasMinMaxDur = true;
    //         }
    //     }
    //     if (!hasMinMaxDur) {
    //         const SUMOTime minMinDur = TIME2STEPS(OptionsCont::getOptions().getInt("tls.min-dur"));
    //         const SUMOTime maxDur = TIME2STEPS(OptionsCont::getOptions().getInt("tls.max-dur"));
    //         std::set<int> yellowIndices;
    //         for (auto phase : myTLLogic->getPhases()) {
    //             for (int i = 0; i < (int)phase.state.size(); i++) {
    //                 if (phase.state[i] == 'y' || phase.state[i] == 'Y') {
    //                     yellowIndices.insert(i);
    //                 }
    //             }
    //         }
    //         for (int ip = 0; ip < (int)myTLLogic->getPhases().size(); ip++) {
    //             bool needMinMaxDur = false;
    //             auto phase = myTLLogic->getPhases()[ip];
    //             std::set<int> greenIndices;
    //             if (phase.state.find_first_of("yY") != std::string::npos) {
    //                 continue;
    //             }
    //             for (int i = 0; i < (int)phase.state.size(); i++) {
    //                 if (yellowIndices.count(i) != 0 && phase.state[i] == 'G') {
    //                     needMinMaxDur = true;
    //                     greenIndices.insert(i);
    //                 }
    //             }
    //             if (needMinMaxDur) {
    //                 double maxSpeed = 0;
    //                 for (NBConnection& c : myControlledLinks) {
    //                     if (greenIndices.count(c.getTLIndex()) != 0) {
    //                         maxSpeed = MAX2(maxSpeed, c.getFrom()->getLaneSpeed(c.getFromLane()));
    //                     }
    //                 }
    //                 // 5s at 50km/h, 10s at 80km/h, rounded to full seconds
    //                 const double minDurBySpeed = maxSpeed * 3.6 / 6 - 3.3;
    //                 SUMOTime minDur = MAX2(minMinDur, TIME2STEPS(floor(minDurBySpeed + 0.5)));
    //                 myTLLogic->setPhaseMinDuration(ip, minDur);
    //                 myTLLogic->setPhaseMaxDuration(ip, maxDur);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn guess_min_max_duration(&mut self) {
        todo!("port NBLoadedSUMOTLDef::guessMinMaxDuration")
    }

    /// C++: `void NBLoadedSUMOTLDef::groupSignals()`
    /// let connections with the same state use the same link index
    // void
    // NBLoadedSUMOTLDef::groupSignals() {
    //     const int maxIndex = getMaxIndex();
    //     std::vector<int> unusedIndices;
    //     for (int i = 0; i <= maxIndex; i++) {
    //         if (isUsed(i)) {
    //             std::set<const NBEdge*> edges = getEdgesUsingIndex(i);
    //             // compactify
    //             replaceIndex(i, i - (int)unusedIndices.size());
    //             if (edges.size() == 0) {
    //                 // do not group pedestrian crossing signals
    //                 continue;
    //             }
    //             std::string states = getStates(i);
    //             for (int j = i + 1; j <= maxIndex; j++) {
    //                 // only group signals from the same edges as is commonly done by
    //                 // traffic engineers
    //                 if (states == getStates(j) && edges == getEdgesUsingIndex(j)) {
    //                     replaceIndex(j, i - (int)unusedIndices.size());
    //                 }
    //             }
    //         } else {
    //             unusedIndices.push_back(i);
    //         }
    //     }
    //     for (int i = (int)unusedIndices.size() - 1; i >= 0; i--) {
    //         myTLLogic->deleteStateIndex(unusedIndices[i]);
    //     }
    //     cleanupStates();
    //     //std::cout << "oldMaxIndex=" << maxIndex << " newMaxIndex=" << getMaxIndex() << " unused=" << toString(unusedIndices) << "\n";
    //     setTLControllingInformation();
    //     // patch crossing indices
    //     for (NBNode* n : myControlledNodes) {
    //         for (NBNode::Crossing* c : n->getCrossings()) {
    //             for (int i = (int)unusedIndices.size() - 1; i >= 0; i--) {
    //                 if (c->customTLIndex > i) {
    //                     c->customTLIndex--;
    //                 }
    //                 if (c->customTLIndex2 > i) {
    //                     c->customTLIndex2--;
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn group_signals(&mut self) {
        todo!("port NBLoadedSUMOTLDef::groupSignals")
    }

    /// C++: `void NBLoadedSUMOTLDef::ungroupSignals()`
    /// let all connections use a distinct link index
    // void
    // NBLoadedSUMOTLDef::ungroupSignals() {
    //     NBConnectionVector defaultOrdering;
    //     collectAllLinks(defaultOrdering);
    //     std::vector<std::string> states; // organized per link rather than phase
    //     int index = 0;
    //     for (NBConnection& c : defaultOrdering) {
    //         NBConnection& c2 = *find_if(myControlledLinks.begin(), myControlledLinks.end(), connection_equal(c));
    //         states.push_back(getStates(c2.getTLIndex()));
    //         c2.setTLIndex(index++);
    //     }
    //     for (NBNode* n : myControlledNodes) {
    //         for (NBNode::Crossing* c : n->getCrossings()) {
    //             states.push_back(getStates(c->tlLinkIndex));
    //             c->customTLIndex = index++;
    //             if (c->tlLinkIndex2 != NBConnection::InvalidTlIndex) {
    //                 states.push_back(getStates(c->tlLinkIndex2));
    //                 c->customTLIndex2 = index++;
    //             }
    //         }
    //     }
    //     myTLLogic->setStateLength(index);
    //     for (int i = 0; i < (int)states.size(); i++) {
    //         for (int p = 0; p < (int)states[i].size(); p++) {
    //             myTLLogic->setPhaseState(p, i, (LinkState)states[i][p]);
    //         }
    //     }
    //     setTLControllingInformation();
    // }
    #[allow(unused_variables)]
    pub fn ungroup_signals(&mut self) {
        todo!("port NBLoadedSUMOTLDef::ungroupSignals")
    }

    /// C++: `void NBLoadedSUMOTLDef::copyIndices(NBTrafficLightDefinition *)`
    /// copy the assignment of link indices to connections from the given definition and rebuilt the states to match
    // void
    // NBLoadedSUMOTLDef::copyIndices(NBTrafficLightDefinition* def) {
    //     std::map<int, std::string> oldStates; // organized per link index rather than phase
    //     std::map<int, std::string> newStates; // organized per link index rather than phase
    //     for (NBConnection& c : def->getControlledLinks()) {
    //         NBConnection& c2 = *find_if(myControlledLinks.begin(), myControlledLinks.end(), connection_equal(c));
    //         const int oldIndex = c2.getTLIndex();
    //         const int newIndex = c.getTLIndex();
    //         std::string states = getStates(oldIndex);
    //         oldStates[oldIndex] = states;
    //         if (newStates.count(newIndex) != 0 && newStates[newIndex] != states) {
    //             WRITE_WARNING("Signal groups from program '" + def->getProgramID() + "' are incompatible with the states of program '" + getProgramID() + "' at tlLogic '" + getID()
    //                           + "'. Possibly unsafe program.");
    //         } else {
    //             newStates[newIndex] = states;
    //         }
    //         c2.setTLIndex(newIndex);
    //     }
    //     const int maxIndex = getMaxIndex();
    //     myTLLogic->setStateLength(maxIndex + 1);
    //     for (int i = 0; i < (int)newStates.size(); i++) {
    //         for (int p = 0; p < (int)newStates[i].size(); p++) {
    //             myTLLogic->setPhaseState(p, i, (LinkState)newStates[i][p]);
    //         }
    //     }
    //     setTLControllingInformation();
    // }
    #[allow(unused_variables)]
    pub fn copy_indices(&mut self, def: Ptr<NBTrafficLightDefinition>) {
        todo!("port NBLoadedSUMOTLDef::copyIndices")
    }

    /// C++: `void NBLoadedSUMOTLDef::finalChecks() const` [const, virtual]
    /// perform optional final checks (on writing)
    // void
    // NBLoadedSUMOTLDef::finalChecks() const {
    //     for (int i = 0; i < myTLLogic->getNumLinks(); i++) {
    //         if (!isUsed(i)) {
    //             WRITE_WARNINGF(TL("Unused state in tlLogic '%', program '%' at tl-index %"), getID(), getProgramID(), i);
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn final_checks(&self) {
        todo!("port NBLoadedSUMOTLDef::finalChecks")
    }

    /// C++: `void NBLoadedSUMOTLDef::replaceIndex(int, int)`
    /// replace the given link index in all connections
    // void
    // NBLoadedSUMOTLDef::replaceIndex(int oldIndex, int newIndex) {
    //     if (oldIndex == newIndex) {
    //         return;
    //     }
    //     for (NBConnection& c : myControlledLinks) {
    //         if (c.getTLIndex() == oldIndex) {
    //             c.setTLIndex(newIndex);
    //         }
    //         if (c.getTLIndex2() == oldIndex) {
    //             c.setTLIndex2(newIndex);
    //         }
    //     }
    //     for (NBNode* n : myControlledNodes) {
    //         for (NBNode::Crossing* c : n->getCrossings()) {
    //             if (c->tlLinkIndex == oldIndex) {
    //                 c->tlLinkIndex = newIndex;
    //             }
    //             if (c->tlLinkIndex2 == oldIndex) {
    //                 c->tlLinkIndex2 = newIndex;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_index(&mut self, oldIndex: i32, newIndex: i32) {
        todo!("port NBLoadedSUMOTLDef::replaceIndex")
    }

    /// C++: `void NBLoadedSUMOTLDef::collectLinks()` [virtual, protected]
    /// Collects the links participating in this traffic light (only if not previously loaded)
    // void
    // NBLoadedSUMOTLDef::collectLinks() {
    //     if (myControlledLinks.size() == 0) {
    //         // maybe we only loaded a different program for a default traffic light.
    //         // Try to build links now.
    //         collectAllLinks(myControlledLinks);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_links(&mut self) {
        todo!("port NBLoadedSUMOTLDef::collectLinks")
    }

    /// C++: `void NBLoadedSUMOTLDef::collectEdges()` [virtual, protected]
    /// Build the list of participating edges
    // void
    // NBLoadedSUMOTLDef::collectEdges() {
    //     if (myControlledLinks.size() == 0) {
    //         NBTrafficLightDefinition::collectEdges();
    //     }
    //     myIncomingEdges.clear();
    //     EdgeVector myOutgoing;
    //     // collect the edges from the participating nodes
    //     for (std::vector<NBNode*>::iterator i = myControlledNodes.begin(); i != myControlledNodes.end(); i++) {
    //         const EdgeVector& incoming = (*i)->getIncomingEdges();
    //         copy(incoming.begin(), incoming.end(), back_inserter(myIncomingEdges));
    //         const EdgeVector& outgoing = (*i)->getOutgoingEdges();
    //         copy(outgoing.begin(), outgoing.end(), back_inserter(myOutgoing));
    //     }
    //     // check which of the edges are completely within the junction
    //     // and which are uncontrolled as well (we already know myControlledLinks)
    //     for (EdgeVector::iterator j = myIncomingEdges.begin(); j != myIncomingEdges.end();) {
    //         NBEdge* edge = *j;
    //         edge->setInsideTLS(false); // reset
    //         // an edge lies within the logic if it is outgoing as well as incoming
    //         EdgeVector::iterator k = std::find(myOutgoing.begin(), myOutgoing.end(), edge);
    //         if (k != myOutgoing.end()) {
    //             if (myControlledInnerEdges.count(edge->getID()) == 0) {
    //                 bool controlled = false;
    //                 for (NBConnectionVector::iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); it++) {
    //                     if ((*it).getFrom() == edge) {
    //                         controlled = true;
    //                         break;
    //                     }
    //                 }
    //                 if (controlled) {
    //                     myControlledInnerEdges.insert(edge->getID());
    //                 } else {
    //                     myEdgesWithin.push_back(edge);
    //                     edge->setInsideTLS(true);
    //                     ++j; //j = myIncomingEdges.erase(j);
    //                     continue;
    //                 }
    //             }
    //         }
    //         ++j;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_edges(&mut self) {
        todo!("port NBLoadedSUMOTLDef::collectEdges")
    }

    /// C++: `NBTrafficLightLogic * NBLoadedSUMOTLDef::myCompute(int)` [virtual, protected]
    /// Computes the traffic light logic finally in dependence to the type
    // NBTrafficLightLogic*
    // NBLoadedSUMOTLDef::myCompute(int brakingTimeSeconds) {
    //     // @todo what to do with those parameters?
    //     UNUSED_PARAMETER(brakingTimeSeconds);
    //     initExtraConflicts();
    //     reconstructLogic();
    //     myTLLogic->closeBuilding(false);
    //     patchIfCrossingsAdded();
    //     myTLLogic->closeBuilding();
    //     return new NBTrafficLightLogic(myTLLogic);
    // }
    #[allow(unused_variables)]
    pub fn my_compute(&mut self, brakingTimeSeconds: i32) -> Ptr<NBTrafficLightLogic> {
        todo!("port NBLoadedSUMOTLDef::myCompute")
    }

    /// C++: `bool NBLoadedSUMOTLDef::amInvalid() const` [const, virtual, protected]
    // bool
    // NBLoadedSUMOTLDef::amInvalid() const {
    //     if (myControlledLinks.size() == 0) {
    //         return true;
    //     }
    //     if (myIncomingEdges.size() == 0) {
    //         return true;
    //     }
    //     if (myReconstructRemovedConnections) {
    //         // check whether at least one connection is valid
    //         for (const NBConnection& con : myControlledLinks) {
    //             if (isValid(con)) {
    //                 return false;
    //             }
    //         }
    //         // all invalid
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn am_invalid(&self) -> bool {
        todo!("port NBLoadedSUMOTLDef::amInvalid")
    }

    /// C++: `void NBLoadedSUMOTLDef::initNeedsContRelation() const` [const, virtual, protected]
    /// initialize myNeedsContRelation and set myNeedsContRelationReady to true
    // void
    // NBLoadedSUMOTLDef::initNeedsContRelation() const {
    //     if (!amInvalid() && !myNeedsContRelationReady) {
    //         myNeedsContRelation.clear();
    //         myExtraConflicts.clear();
    //         if (myType == TrafficLightType::NEMA) {
    //             NBTrafficLightDefinition::initNeedsContRelation();
    //             NBTrafficLightDefinition::initExtraConflicts();
    //         } else {
    //             const bool controlledWithin = !OptionsCont::getOptions().getBool("tls.uncontrolled-within");
    //             const std::vector<NBTrafficLightLogic::PhaseDefinition> phases = myTLLogic->getPhases();
    //             for (std::vector<NBTrafficLightLogic::PhaseDefinition>::const_iterator it = phases.begin(); it != phases.end(); it++) {
    //                 const std::string state = (*it).state;
    //                 for (NBConnectionVector::const_iterator it1 = myControlledLinks.begin(); it1 != myControlledLinks.end(); it1++) {
    //                     const NBConnection& c1 = *it1;
    //                     const int i1 = c1.getTLIndex();
    //                     if (i1 == NBConnection::InvalidTlIndex || (state[i1] != 'g' && state[i1] != 's') || c1.getFrom() == nullptr || c1.getTo() == nullptr) {
    //                         continue;
    //                     }
    //                     for (NBConnectionVector::const_iterator it2 = myControlledLinks.begin(); it2 != myControlledLinks.end(); it2++) {
    //                         const NBConnection& c2 = *it2;
    //                         const int i2 = c2.getTLIndex();
    //                         if (i2 != NBConnection::InvalidTlIndex
    //                                 && i2 != i1
    //                                 && (state[i2] == 'G' || state[i2] == 'g')
    //                                 && c2.getFrom() != nullptr && c2.getTo() != nullptr) {
    //                             const bool rightTurnConflict = NBNode::rightTurnConflict(
    //                                                                c1.getFrom(), c1.getTo(), c1.getFromLane(), c2.getFrom(), c2.getTo(), c2.getFromLane());
    //                             const bool forbidden = forbids(c2.getFrom(), c2.getTo(), c1.getFrom(), c1.getTo(), true, controlledWithin);
    //                             const bool isFoes = foes(c2.getFrom(), c2.getTo(), c1.getFrom(), c1.getTo()) && !c2.getFrom()->isTurningDirectionAt(c2.getTo());
    //                             const bool hasContRel = (forbidden && state[i1] != 's') || rightTurnConflict;
    //                             if (hasContRel) {
    //                                 myNeedsContRelation.insert(StreamPair(c1.getFrom(), c1.getTo(), c2.getFrom(), c2.getTo()));
    //                             }
    //                             const bool indirectLeft = c1.getFrom()->getConnection(c1.getFromLane(), c1.getTo(), c1.getToLane()).indirectLeft;
    //                             if (isFoes && (state[i1] == 's' || (!hasContRel && state[i2] == 'G' && !indirectLeft))) {
    //                                 myExtraConflicts.insert(std::make_pair(i1, i2));
    //                                 //std::cout << getID() << " prog=" << getProgramID() << " phase=" << (it - phases.begin()) << " extraConflict i1=" << i1 << " i2=" << i2
    //                                 //    << " c1=" << c1 << " c2=" << c2 << "\n";
    //                             }
    //                             //std::cout << getID() << " p=" << (it - phases.begin()) << " i1=" << i1 << " i2=" << i2 << " rightTurnConflict=" << rightTurnConflict << " forbidden=" << forbidden << " isFoes=" << isFoes << "\n";
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     myNeedsContRelationReady = true;
    //     myExtraConflictsReady = true;
    // }
    #[allow(unused_variables)]
    pub fn init_needs_cont_relation(&self) {
        todo!("port NBLoadedSUMOTLDef::initNeedsContRelation")
    }

    /// C++: `int NBLoadedSUMOTLDef::getMaxIndex()` [virtual, protected]
    /// return the highest known tls link index used by any controlled connection or crossing
    // int
    // NBLoadedSUMOTLDef::getMaxIndex() {
    //     int maxIndex = -1;
    //     for (const NBConnection& c : myControlledLinks) {
    //         maxIndex = MAX2(maxIndex, c.getTLIndex());
    //         maxIndex = MAX2(maxIndex, c.getTLIndex2());
    //     }
    //     for (NBNode* n : myControlledNodes) {
    //         for (NBNode::Crossing* c : n->getCrossings()) {
    //             maxIndex = MAX2(maxIndex, c->tlLinkIndex);
    //             maxIndex = MAX2(maxIndex, c->tlLinkIndex2);
    //         }
    //     }
    //     return maxIndex;
    // }
    #[allow(unused_variables)]
    pub fn get_max_index(&mut self) -> i32 {
        todo!("port NBLoadedSUMOTLDef::getMaxIndex")
    }

    /// C++: `int NBLoadedSUMOTLDef::getMaxValidIndex()` [virtual, protected]
    /// Returns the maximum index controlled by this traffic light
    // int
    // NBLoadedSUMOTLDef::getMaxValidIndex() {
    //     return myTLLogic->getNumLinks() - 1;
    // }
    #[allow(unused_variables)]
    pub fn get_max_valid_index(&mut self) -> i32 {
        todo!("port NBLoadedSUMOTLDef::getMaxValidIndex")
    }

    /// C++: `std::string NBLoadedSUMOTLDef::getStates(int)` [protected]
    /// get all states for the given link index
    // std::string
    // NBLoadedSUMOTLDef::getStates(int index) {
    //     assert(index >= 0);
    //     assert(index <= getMaxValidIndex());
    //     std::string result;
    //     for (auto& pd : myTLLogic->getPhases()) {
    //         result += pd.state[index];
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_states(&mut self, index: i32) -> String {
        todo!("port NBLoadedSUMOTLDef::getStates")
    }

    /// C++: `bool NBLoadedSUMOTLDef::isUsed(int) const` [const, protected]
    /// return whether the given link index is used by any connectons
    // bool
    // NBLoadedSUMOTLDef::isUsed(int index) const {
    //     for (const NBConnection& c : myControlledLinks) {
    //         if (c.getTLIndex() == index || c.getTLIndex2() == index) {
    //             return true;
    //         }
    //     }
    //     for (NBNode* n : myControlledNodes) {
    //         for (NBNode::Crossing* c : n->getCrossings()) {
    //             if (c->tlLinkIndex == index || c->tlLinkIndex2 == index) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_used(&self, index: i32) -> bool {
        todo!("port NBLoadedSUMOTLDef::isUsed")
    }

    /// C++: `std::set<const NBEdge *> NBLoadedSUMOTLDef::getEdgesUsingIndex(int) const` [const, protected]
    /// brief retrieve all edges with connections that use the given traffic light index
    // std::set<const NBEdge*>
    // NBLoadedSUMOTLDef::getEdgesUsingIndex(int index) const {
    //     std::set<const NBEdge*> result;
    //     for (const NBConnection& c : myControlledLinks) {
    //         if (c.getTLIndex() == index || c.getTLIndex2() == index) {
    //             result.insert(c.getFrom());
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_edges_using_index(&self, index: i32) -> BTreeSet<Ptr<NBEdge>> {
        todo!("port NBLoadedSUMOTLDef::getEdgesUsingIndex")
    }

    /// C++: `void NBLoadedSUMOTLDef::patchIfCrossingsAdded()` [private]
    /// repair the plan if controlled nodes received pedestrian crossings
    // void
    // NBLoadedSUMOTLDef::patchIfCrossingsAdded() {
    //     const int size = myTLLogic->getNumLinks();
    //     int noLinksAll = 0;
    //     for (NBConnectionVector::const_iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); it++) {
    //         const NBConnection& c = *it;
    //         if (c.getTLIndex() != NBConnection::InvalidTlIndex) {
    //             noLinksAll = MAX2(noLinksAll, (int)c.getTLIndex() + 1);
    //         }
    //     }
    //     const int numNormalLinks = noLinksAll;
    //     int oldCrossings = 0;
    //     // collect crossings
    //     bool customIndex = false;
    //     std::vector<NBNode::Crossing*> crossings;
    //     for (std::vector<NBNode*>::iterator i = myControlledNodes.begin(); i != myControlledNodes.end(); i++) {
    //         const std::vector<NBNode::Crossing*>& c = (*i)->getCrossings();
    //         // set tl indices for crossings
    //         customIndex |= (*i)->setCrossingTLIndices(getID(), noLinksAll);
    //         copy(c.begin(), c.end(), std::back_inserter(crossings));
    //         noLinksAll += (int)c.size();
    //         oldCrossings += (*i)->numCrossingsFromSumoNet();
    //     }
    //     if ((int)crossings.size() != oldCrossings) {
    //         std::vector<NBTrafficLightLogic::PhaseDefinition> phases = myTLLogic->getPhases();
    //         // do not rebuilt crossing states there are custom indices and the state string is long enough
    //         if (phases.size() > 0 && (
    //                     (int)(phases.front().state.size()) < noLinksAll ||
    //                     ((int)(phases.front().state.size()) > noLinksAll && !customIndex))) {
    //             // collect edges
    //             EdgeVector fromEdges(numNormalLinks, (NBEdge*)nullptr);
    //             EdgeVector toEdges(numNormalLinks, (NBEdge*)nullptr);
    //             std::vector<int> fromLanes(size, 0);
    //             collectEdgeVectors(fromEdges, toEdges, fromLanes);
    //             const std::string crossingDefaultState(crossings.size(), 'r');
    //
    //             // rebuild the logic (see NBOwnTLDef.cpp::myCompute)
    //             NBTrafficLightLogic* newLogic = new NBTrafficLightLogic(getID(), getProgramID(), 0, myOffset, myType);
    //             SUMOTime brakingTime = TIME2STEPS(computeBrakingTime(OptionsCont::getOptions().getFloat("tls.yellow.min-decel")));
    //             //std::cout << "patchIfCrossingsAdded for " << getID() << " numPhases=" << phases.size() << "\n";
    //             for (const auto& phase : phases) {
    //                 const std::string state = phase.state.substr(0, numNormalLinks) + crossingDefaultState;
    //                 NBOwnTLDef::addPedestrianPhases(newLogic, phase.duration, phase.minDur, phase.maxDur, phase.earliestEnd, phase.latestEnd,
    //                                                 state, crossings, fromEdges, toEdges);
    //             }
    //             NBOwnTLDef::addPedestrianScramble(newLogic, noLinksAll, TIME2STEPS(10), brakingTime, crossings, fromEdges, toEdges);
    //
    //             delete myTLLogic;
    //             myTLLogic = newLogic;
    //         } else if (phases.size() == 0) {
    //             WRITE_WARNINGF(TL("Could not patch tlLogic '%' for changed crossings"), getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn patch_if_crossings_added(&mut self) {
        todo!("port NBLoadedSUMOTLDef::patchIfCrossingsAdded")
    }

    /// C++: `void NBLoadedSUMOTLDef::collectEdgeVectors(EdgeVector &, EdgeVector &, std::vector<int> &) const` [const, private]
    /// Collects the edges for each tlIndex
    // void
    // NBLoadedSUMOTLDef::collectEdgeVectors(EdgeVector& fromEdges, EdgeVector& toEdges, std::vector<int>& fromLanes) const {
    //     assert(fromEdges.size() > 0);
    //     assert(fromEdges.size() == toEdges.size());
    //     const int size = (int)fromEdges.size();
    //
    //     for (NBConnectionVector::const_iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); it++) {
    //         const NBConnection& c = *it;
    //         if (c.getTLIndex() != NBConnection::InvalidTlIndex) {
    //             if (c.getTLIndex() >= size) {
    //                 throw ProcessError("Invalid linkIndex " + toString(c.getTLIndex()) + " for traffic light '" + getID() +
    //                                    "' with " + toString(size) + " links.");
    //             }
    //             fromEdges[c.getTLIndex()] = c.getFrom();
    //             toEdges[c.getTLIndex()] = c.getTo();
    //             fromLanes[c.getTLIndex()] = c.getFromLane();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_edge_vectors(&self, fromEdges: &mut EdgeVector, toEdges: &mut EdgeVector, fromLanes: &mut Vec<i32>) {
        todo!("port NBLoadedSUMOTLDef::collectEdgeVectors")
    }

    /// C++: `void NBLoadedSUMOTLDef::reconstructLogic()` [private]
    /// adapt to removal or addition of connections
    // void
    // NBLoadedSUMOTLDef::reconstructLogic() {
    //     const bool netedit = NBNetBuilder::runningNetedit();
    // #ifdef DEBUG_RECONSTRUCTION
    //     bool debugPrintModified = myReconstructAddedConnections || myReconstructRemovedConnections;
    //     std::cout << getID() << " reconstructLogic added=" << myReconstructAddedConnections
    //               << " removed=" << myReconstructRemovedConnections
    //               << " valid=" << hasValidIndices()
    //               << " phasesLoaded=" << myPhasesLoaded
    //               << " oldLinks:\n";
    //     for (NBConnectionVector::iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); ++it) {
    //         std::cout << "    " << *it << "\n";
    //     }
    // #endif
    //     if (myReconstructAddedConnections) {
    //         myReconstructAddedConnections = false;
    //         // do not rebuild the logic when running netedit and all links are already covered by the program
    //         if (!myPhasesLoaded && !(netedit && hasValidIndices())) {
    //             // rebuild the logic from scratch
    //             // XXX if a connection with the same from- and to-edge already exisits, its states could be copied instead
    //             NBOwnTLDef dummy(DummyID, myControlledNodes, 0, getType());
    //             dummy.setParticipantsInformation();
    //             dummy.setProgramID(getProgramID());
    //             dummy.setTLControllingInformation();
    //             NBTrafficLightLogic* newLogic = dummy.compute(OptionsCont::getOptions());
    //             myIncomingEdges = dummy.getIncomingEdges();
    //             myControlledLinks = dummy.getControlledLinks();
    //             for (std::vector<NBNode*>::const_iterator i = myControlledNodes.begin(); i != myControlledNodes.end(); i++) {
    //                 (*i)->removeTrafficLight(&dummy);
    //             }
    //             delete myTLLogic;
    //             myTLLogic = newLogic;
    //             if (newLogic != nullptr) {
    //                 newLogic->setID(getID());
    //                 newLogic->setType(getType());
    //                 newLogic->setOffset(getOffset());
    //                 setTLControllingInformation();
    //                 // reset crossing custom indices
    //                 for (NBNode* n : myControlledNodes) {
    //                     for (NBNode::Crossing* c : n->getCrossings()) {
    //                         c->customTLIndex = NBConnection::InvalidTlIndex;
    //                     }
    //                 }
    //
    //             }
    //         } else {
    //             setTLControllingInformation();
    //         }
    //     }
    //     if (myReconstructRemovedConnections) {
    //         myReconstructRemovedConnections = false;
    //         // for each connection, check whether it is still valid
    //         for (NBConnectionVector::iterator it = myControlledLinks.begin(); it != myControlledLinks.end();) {
    //             const NBConnection con = (*it);
    //             if (isValid(con)) {
    //                 it++;
    //             } else {
    //                 // remove connection
    //                 const int removed = con.getTLIndex();
    //                 it = myControlledLinks.erase(it);
    //                 // no automatic modificaions when running netedit
    //                 if (!myPhasesLoaded && !(netedit && hasValidIndices())) {
    //                     // shift index off successive connections and remove entry from all phases if the tlIndex was only used by this connection
    //                     bool exclusive = true;
    //                     for (NBConnection& other : myControlledLinks) {
    //                         if (other != con && other.getTLIndex() == removed) {
    //                             exclusive = false;
    //                             break;
    //                         }
    //                     }
    //                     if (exclusive) {
    //                         // shift indices above the removed index downward
    //                         for (NBConnection& other : myControlledLinks) {
    //                             if (other.getTLIndex() > removed) {
    //                                 other.setTLIndex(other.getTLIndex() - 1);
    //                             }
    //                         }
    //                         // shift crossing custom indices above the removed index downward
    //                         for (NBNode* n : myControlledNodes) {
    //                             for (NBNode::Crossing* c : n->getCrossings()) {
    //                                 if (c->customTLIndex > removed) {
    //                                     c->customTLIndex--;
    //                                 }
    //                             }
    //                         }
    //                         // rebuild the logic
    //                         NBTrafficLightLogic* newLogic = new NBTrafficLightLogic(getID(), getProgramID(), 0, myOffset, myType);
    //                         for (const NBTrafficLightLogic::PhaseDefinition& phase : myTLLogic->getPhases()) {
    //                             std::string newState = phase.state;
    //                             newState.erase(newState.begin() + removed);
    //                             newLogic->addStep(phase.duration, newState);
    //                         }
    //                         delete myTLLogic;
    //                         myTLLogic = newLogic;
    //                     }
    //                 }
    //             }
    //         }
    //         setTLControllingInformation();
    //     }
    // #ifdef DEBUG_RECONSTRUCTION
    //     if (debugPrintModified) {
    //         std::cout << " newLinks:\n";
    //         for (NBConnectionVector::iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); ++it) {
    //             std::cout << "    " << *it << "\n";
    //         }
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn reconstruct_logic(&mut self) {
        todo!("port NBLoadedSUMOTLDef::reconstructLogic")
    }

    /// C++: `bool NBLoadedSUMOTLDef::hasValidIndices() const` [const, private]
    /// return whether all tls link indices are valid
    // bool
    // NBLoadedSUMOTLDef::hasValidIndices() const {
    //     for (const NBConnection& c : myControlledLinks) {
    //         if (c.getTLIndex() == NBConnection::InvalidTlIndex) {
    //             return false;
    //         }
    //     }
    //     for (NBNode* n : myControlledNodes) {
    //         for (NBNode::Crossing* c : n->getCrossings()) {
    //             if (c->tlLinkIndex == NBConnection::InvalidTlIndex) {
    //                 return false;
    //             }
    //         }
    //     }
    //     // method getMaxIndex() is const but cannot be declare as such due to inheritance
    //     return const_cast<NBLoadedSUMOTLDef*>(this)->getMaxIndex() < myTLLogic->getNumLinks();
    // }
    #[allow(unused_variables)]
    pub fn has_valid_indices(&self) -> bool {
        todo!("port NBLoadedSUMOTLDef::hasValidIndices")
    }

    /// C++: `bool NBLoadedSUMOTLDef::isValid(const NBConnection &) const` [const, private]
    /// return whether the given connection is still valid
    // bool
    // NBLoadedSUMOTLDef::isValid(const NBConnection& con) const {
    //     return (// edge still exists
    //                std::find(myIncomingEdges.begin(), myIncomingEdges.end(), con.getFrom()) != myIncomingEdges.end()
    //                // connection still exists
    //                && con.getFrom()->hasConnectionTo(con.getTo(), con.getToLane(), con.getFromLane())
    //                // connection is still set to be controlled
    //                && con.getFrom()->mayBeTLSControlled(con.getFromLane(), con.getTo(), con.getToLane()));
    // }
    #[allow(unused_variables)]
    pub fn is_valid(&self, con: &NBConnection) -> bool {
        todo!("port NBLoadedSUMOTLDef::isValid")
    }

}

/// class for identifying connections
/// C++ class `NBLoadedSUMOTLDef::connection_equal` (netbuild/NBLoadedSUMOTLDef.h)
pub struct NBLoadedSUMOTLDef_connection_equal {
    pub myC: Ptr<NBConnection>,
}

impl NBLoadedSUMOTLDef_connection_equal {
    /// C++: `void connection_equal::connection_equal(const NBConnection &)`
    /// constructor
    // connection_equal(const NBConnection& c) : myC(c) {}
    #[allow(unused_variables)]
    pub fn new(c: &NBConnection) -> Self {
        todo!("port connection_equal::connection_equal")
    }

    /// C++: `bool connection_equal::operator()(const NBConnection &) const` [const]
    // bool operator()(const NBConnection& c) const {
    //             return c.getFrom() == myC.getFrom() && c.getTo() == myC.getTo() &&
    //                    c.getFromLane() == myC.getFromLane() && c.getToLane() == myC.getToLane();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c: &NBConnection) -> bool {
        todo!("port connection_equal::operator()")
    }

}

