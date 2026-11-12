//! Port of netbuild/NBLoadedTLDef.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<std::string, SignalGroup *> NBLoadedTLDef::SignalGroupCont`
pub type NBLoadedTLDef_SignalGroupCont = BTreeMap<String, Ptr<NBLoadedTLDef_SignalGroup>>;

/// A loaded (complete) traffic light logic
/// C++ class `NBLoadedTLDef` (netbuild/NBLoadedTLDef.h)
#[repr(C)]
pub struct NBLoadedTLDef {
    /// C++ base class `NBTrafficLightDefinition`
    pub base: NBTrafficLightDefinition,
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// Controlled signal groups
    pub mySignalGroups: NBLoadedTLDef_SignalGroupCont,
    /// The duration of a single cycle
    pub myCycleDuration: SUMOTime,
}

impl std::ops::Deref for NBLoadedTLDef {
    type Target = NBTrafficLightDefinition;
    fn deref(&self) -> &NBTrafficLightDefinition {
        &self.base
    }
}
impl std::ops::DerefMut for NBLoadedTLDef {
    fn deref_mut(&mut self) -> &mut NBTrafficLightDefinition {
        &mut self.base
    }
}

impl NBLoadedTLDef {
    /// C++: `void NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont &, const std::string &, const std::vector<NBNode *> &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont& ec, const std::string& id,
    //                              const std::vector<NBNode*>& junctions, SUMOTime offset, TrafficLightType type) :
    //     NBTrafficLightDefinition(id, junctions, DefaultProgramID, offset, type),
    //     myEdgeCont(&ec) {
    // }
    #[allow(unused_variables)]
    pub fn new(ec: &NBEdgeCont, id: &str, junctions: &Vec<Ptr<NBNode>>, offset: SUMOTime, type_: TrafficLightType) -> Self {
        todo!("port NBLoadedTLDef::NBLoadedTLDef")
    }

    /// C++: `void NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont &, const std::string &, NBNode *, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont& ec, const std::string& id, NBNode* junction, SUMOTime offset, TrafficLightType type) :
    //     NBTrafficLightDefinition(id, junction, DefaultProgramID, offset, type),
    //     myEdgeCont(&ec) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(ec: &NBEdgeCont, id: &str, junction: Ptr<NBNode>, offset: SUMOTime, type_: TrafficLightType) -> Self {
        todo!("port NBLoadedTLDef::NBLoadedTLDef")
    }

    /// C++: `void NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont &, const std::string &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont& ec, const std::string& id, SUMOTime offset, TrafficLightType type) :
    //     NBTrafficLightDefinition(id, DefaultProgramID, offset, type),
    //     myEdgeCont(&ec) {
    // }
    #[allow(unused_variables)]
    pub fn new_3(ec: &NBEdgeCont, id: &str, offset: SUMOTime, type_: TrafficLightType) -> Self {
        todo!("port NBLoadedTLDef::NBLoadedTLDef")
    }

    /// C++: `void NBLoadedTLDef::~NBLoadedTLDef() noexcept` [virtual]
    /// Destructor
    // NBLoadedTLDef::~NBLoadedTLDef() {
    //     for (SignalGroupCont::iterator i = mySignalGroups.begin(); i != mySignalGroups.end(); ++i) {
    //         delete (*i).second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBLoadedTLDef::~NBLoadedTLDef")
    }

    /// C++: `void NBLoadedTLDef::setCycleDuration(SUMOTime)`
    /// Sets the duration of a cycle
    // void
    // NBLoadedTLDef::setCycleDuration(SUMOTime cycleDur) {
    //     myCycleDuration = cycleDur;
    // }
    #[allow(unused_variables)]
    pub fn set_cycle_duration(&mut self, cycleDur: SUMOTime) {
        todo!("port NBLoadedTLDef::setCycleDuration")
    }

    /// C++: `void NBLoadedTLDef::addSignalGroup(const std::string &)`
    /// Adds a signal group
    // void
    // NBLoadedTLDef::addSignalGroup(const std::string& id) {
    //     assert(mySignalGroups.find(id) == mySignalGroups.end());
    //     mySignalGroups[id] = new SignalGroup(id);
    // }
    #[allow(unused_variables)]
    pub fn add_signal_group(&mut self, id: &str) {
        todo!("port NBLoadedTLDef::addSignalGroup")
    }

    /// C++: `bool NBLoadedTLDef::addToSignalGroup(const std::string &, const NBConnection &)`
    /// Adds a connection to a signal group
    // bool
    // NBLoadedTLDef::addToSignalGroup(const std::string& groupid,
    //                                 const NBConnection& connection) {
    //     if (mySignalGroups.find(groupid) == mySignalGroups.end()) {
    //         return false;
    //     }
    //     mySignalGroups[groupid]->addConnection(connection);
    //     NBNode* n1 = connection.getFrom()->getToNode();
    //     if (n1 != nullptr) {
    //         addNode(n1);
    //         n1->addTrafficLight(this);
    //     }
    //     NBNode* n2 = connection.getTo()->getFromNode();
    //     if (n2 != nullptr) {
    //         addNode(n2);
    //         n2->addTrafficLight(this);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_to_signal_group(&mut self, groupid: &str, connection: &NBConnection) -> bool {
        todo!("port NBLoadedTLDef::addToSignalGroup")
    }

    /// C++: `bool NBLoadedTLDef::addToSignalGroup(const std::string &, const NBConnectionVector &)`
    /// Adds a list of connections to a signal group
    // bool
    // NBLoadedTLDef::addToSignalGroup(const std::string& groupid,
    //                                 const NBConnectionVector& connections) {
    //     bool ok = true;
    //     for (NBConnectionVector::const_iterator i = connections.begin(); i != connections.end(); i++) {
    //         ok &= addToSignalGroup(groupid, *i);
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn add_to_signal_group_2(&mut self, groupid: &str, connections: &NBConnectionVector) -> bool {
        todo!("port NBLoadedTLDef::addToSignalGroup")
    }

    /// C++: `void NBLoadedTLDef::addSignalGroupPhaseBegin(const std::string &, SUMOTime, TLColor)`
    /// Sets the information about the begin of a phase
    // void
    // NBLoadedTLDef::addSignalGroupPhaseBegin(const std::string& groupid, SUMOTime time,
    //                                         TLColor color) {
    //     assert(mySignalGroups.find(groupid) != mySignalGroups.end());
    //     mySignalGroups[groupid]->addPhaseBegin(time, color);
    // }
    #[allow(unused_variables)]
    pub fn add_signal_group_phase_begin(&mut self, groupid: &str, time: SUMOTime, color: NBTrafficLightDefinition_TLColor) {
        todo!("port NBLoadedTLDef::addSignalGroupPhaseBegin")
    }

    /// C++: `void NBLoadedTLDef::setSignalYellowTimes(const std::string &, SUMOTime, SUMOTime)`
    /// Sets the times the light is yellow or red/yellow
    // void
    // NBLoadedTLDef::setSignalYellowTimes(const std::string& groupid,
    //                                     SUMOTime myTRedYellow, SUMOTime myTYellow) {
    //     assert(mySignalGroups.find(groupid) != mySignalGroups.end());
    //     mySignalGroups[groupid]->setYellowTimes(myTRedYellow, myTYellow);
    // }
    #[allow(unused_variables)]
    pub fn set_signal_yellow_times(&mut self, groupid: &str, tRedYellow: SUMOTime, tYellow: SUMOTime) {
        todo!("port NBLoadedTLDef::setSignalYellowTimes")
    }

    /// C++: `void NBLoadedTLDef::setTLControllingInformation() const` [const, virtual]
    /// Informs edges about being controlled by a tls
    // void
    // NBLoadedTLDef::setTLControllingInformation() const {
    //     // assign the tl-indices to the edge connections
    //     for (NBConnectionVector::const_iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); it++) {
    //         const NBConnection& c = *it;
    //         if (c.getTLIndex() != NBConnection::InvalidTlIndex) {
    //             c.getFrom()->setControllingTLInformation(c, getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_tl_controlling_information(&self) {
        todo!("port NBLoadedTLDef::setTLControllingInformation")
    }

    /// C++: `void NBLoadedTLDef::remapRemoved(NBEdge *, const EdgeVector &, const EdgeVector &)` [virtual]
    /// Replaces occurrences of the removed edge in incoming/outgoing edges of all definitions
    // void
    // NBLoadedTLDef::remapRemoved(NBEdge* removed,
    //                             const EdgeVector& incoming,
    //                             const EdgeVector& outgoing) {
    //     for (SignalGroupCont::const_iterator i = mySignalGroups.begin(); i != mySignalGroups.end(); i++) {
    //         SignalGroup* group = (*i).second;
    //         if (group->containsIncoming(removed)) {
    //             group->remapIncoming(removed, incoming);
    //         }
    //         if (group->containsOutgoing(removed)) {
    //             group->remapOutgoing(removed, outgoing);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remap_removed(&mut self, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
        todo!("port NBLoadedTLDef::remapRemoved")
    }

    /// C++: `void NBLoadedTLDef::initNeedsContRelation() const` [const, virtual]
    // void
    // NBLoadedTLDef::initNeedsContRelation() const {
    //     if (!myNeedsContRelationReady) {
    //         throw ProcessError(TL("myNeedsContRelation was not properly initialized\n"));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_needs_cont_relation(&self) {
        todo!("port NBLoadedTLDef::initNeedsContRelation")
    }

    /// C++: `int NBLoadedTLDef::getMaxIndex()` [virtual]
    /// Returns the maximum index controlled by this traffic light
    // int
    // NBLoadedTLDef::getMaxIndex() {
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
        todo!("port NBLoadedTLDef::getMaxIndex")
    }

    /// C++: `NBTrafficLightLogic * NBLoadedTLDef::myCompute(int)` [virtual, protected]
    /// Computes the traffic light logic finally in dependence to the type
    // NBTrafficLightLogic*
    // NBLoadedTLDef::myCompute(int brakingTimeSeconds) {
    //     MsgHandler::getWarningInstance()->clear(); // !!!
    //     // compute the switching times and count the signals
    //     std::set<SUMOTime> switchTimes;
    //     int numSignals = 0;
    //     for (const auto& i : mySignalGroups) {
    //         NBLoadedTLDef::SignalGroup* const group = i.second;
    //         // needed later
    //         group->sortPhases();
    //         // patch the yellow time for this group
    //         group->patchTYellow(TIME2STEPS(brakingTimeSeconds), OptionsCont::getOptions().getBool("tls.yellow.patch-small"));
    //         // copy the now valid times into the container
    //         //  both the given red and green phases are added and also the
    //         //  yellow times
    //         const std::vector<SUMOTime> gtimes = group->getTimes(myCycleDuration);
    //         switchTimes.insert(gtimes.begin(), gtimes.end());
    //         numSignals += group->getLinkNo();
    //     }
    //
    //     // build the phases
    //     NBTrafficLightLogic* logic = new NBTrafficLightLogic(getID(), getProgramID(), numSignals, myOffset, myType);
    //     SUMOTime prev = -1;
    //     for (const SUMOTime l : switchTimes) {
    //         if (prev != -1) {
    //             logic->addStep(l - prev, buildPhaseState(prev));
    //         }
    //         prev = l;
    //     }
    //     logic->addStep(myCycleDuration + (*switchTimes.begin()) - prev, buildPhaseState(prev));
    //     // check whether any warnings were printed
    //     if (MsgHandler::getWarningInstance()->wasInformed()) {
    //         WRITE_WARNINGF(TL("During computation of traffic light '%'."), getID());
    //     }
    //     logic->closeBuilding();
    //
    //     // initialize myNeedsContRelation
    //     myNeedsContRelation.clear();
    //     const bool controlledWithin = !OptionsCont::getOptions().getBool("tls.uncontrolled-within");
    //     const std::vector<NBTrafficLightLogic::PhaseDefinition> phases = logic->getPhases();
    //     for (std::vector<NBTrafficLightLogic::PhaseDefinition>::const_iterator it = phases.begin(); it != phases.end(); it++) {
    //         const std::string state = (*it).state;
    //         for (NBConnectionVector::const_iterator it1 = myControlledLinks.begin(); it1 != myControlledLinks.end(); it1++) {
    //             const NBConnection& c1 = *it1;
    //             const int i1 = c1.getTLIndex();
    //             if (i1 == NBConnection::InvalidTlIndex || state[i1] != 'g' || c1.getFrom() == nullptr || c1.getTo() == nullptr) {
    //                 continue;
    //             }
    //             for (NBConnectionVector::const_iterator it2 = myControlledLinks.begin(); it2 != myControlledLinks.end(); it2++) {
    //                 const NBConnection& c2 = *it2;
    //                 const int i2 = c2.getTLIndex();
    //                 if (i2 != NBConnection::InvalidTlIndex
    //                         && i2 != i1
    //                         && (state[i2] == 'G' || state[i2] == 'g')
    //                         && c2.getFrom() != nullptr && c2.getTo() != nullptr) {
    //                     const bool rightTurnConflict = NBNode::rightTurnConflict(
    //                                                        c1.getFrom(), c1.getTo(), c1.getFromLane(), c2.getFrom(), c2.getTo(), c2.getFromLane());
    //                     if (forbids(c2.getFrom(), c2.getTo(), c1.getFrom(), c1.getTo(), true, controlledWithin) || rightTurnConflict) {
    //                         myNeedsContRelation.insert(StreamPair(c1.getFrom(), c1.getTo(), c2.getFrom(), c2.getTo()));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     myNeedsContRelationReady = true;
    //
    //     return logic;
    // }
    #[allow(unused_variables)]
    pub fn my_compute(&mut self, brakingTimeSeconds: i32) -> Ptr<NBTrafficLightLogic> {
        todo!("port NBLoadedTLDef::myCompute")
    }

    /// C++: `void NBLoadedTLDef::setParticipantsInformation()` [virtual, protected]
    /// Builds the list of participating nodes/edges/links
    // void
    // NBLoadedTLDef::setParticipantsInformation() {
    //     // assign participating nodes to the request
    //     collectNodes();
    //     // collect the information about participating edges and links
    //     collectEdges();
    //     collectLinks();
    // }
    #[allow(unused_variables)]
    pub fn set_participants_information(&mut self) {
        todo!("port NBLoadedTLDef::setParticipantsInformation")
    }

    /// C++: `void NBLoadedTLDef::collectNodes()` [protected]
    /// Collects the nodes participating in this traffic light
    // void
    // NBLoadedTLDef::collectNodes() {
    //     myControlledNodes.clear();
    //     SignalGroupCont::const_iterator m;
    //     for (m = mySignalGroups.begin(); m != mySignalGroups.end(); m++) {
    //         SignalGroup* group = (*m).second;
    //         int linkNo = group->getLinkNo();
    //         for (int j = 0; j < linkNo; j++) {
    //             const NBConnection& conn = group->getConnection(j);
    //             NBEdge* edge = conn.getFrom();
    //             NBNode* node = edge->getToNode();
    //             myControlledNodes.push_back(node);
    //         }
    //     }
    //     std::sort(myControlledNodes.begin(), myControlledNodes.end(), NBNode::nodes_by_id_sorter());
    // }
    #[allow(unused_variables)]
    pub fn collect_nodes(&mut self) {
        todo!("port NBLoadedTLDef::collectNodes")
    }

    /// C++: `void NBLoadedTLDef::collectLinks()` [virtual, protected]
    /// Collects the links participating in this traffic light
    // void
    // NBLoadedTLDef::collectLinks() {
    //     myControlledLinks.clear();
    //     // build the list of links which are controlled by the traffic light
    //     for (EdgeVector::iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //         NBEdge* incoming = *i;
    //         int noLanes = incoming->getNumLanes();
    //         for (int j = 0; j < noLanes; j++) {
    //             std::vector<NBEdge::Connection> elv = incoming->getConnectionsFromLane(j);
    //             for (std::vector<NBEdge::Connection>::iterator k = elv.begin(); k != elv.end(); k++) {
    //                 NBEdge::Connection el = *k;
    //                 if (el.toEdge != nullptr) {
    //                     myControlledLinks.push_back(NBConnection(incoming, j, el.toEdge, el.toLane));
    //                 }
    //             }
    //         }
    //     }
    //
    //     // assign tl-indices to myControlledLinks
    //     int pos = 0;
    //     for (SignalGroupCont::const_iterator m = mySignalGroups.begin(); m != mySignalGroups.end(); m++) {
    //         SignalGroup* group = (*m).second;
    //         int linkNo = group->getLinkNo();
    //         for (int j = 0; j < linkNo; j++) {
    //             const NBConnection& conn = group->getConnection(j);
    //             assert(conn.getFromLane() < 0 || (int) conn.getFrom()->getNumLanes() > conn.getFromLane());
    //             NBConnection tst(conn);
    //             tst.setTLIndex(pos);
    //             if (tst.check(*myEdgeCont)) {
    //                 if (tst.getFrom()->mayBeTLSControlled(tst.getFromLane(), tst.getTo(), tst.getToLane())) {
    //                     for (NBConnectionVector::iterator it = myControlledLinks.begin(); it != myControlledLinks.end(); it++) {
    //                         NBConnection& c = *it;
    //                         if (c.getTLIndex() == NBConnection::InvalidTlIndex
    //                                 && tst.getFrom() == c.getFrom() && tst.getTo() == c.getTo()
    //                                 && (tst.getFromLane() < 0 || tst.getFromLane() == c.getFromLane())
    //                                 && (tst.getToLane() < 0 || tst.getToLane() == c.getToLane())) {
    //                             c.setTLIndex(pos);
    //                         }
    //                     }
    //                     //std::cout << getID() << " group=" << (*m).first << " tst=" << tst << "\n";
    //                     pos++;
    //                 }
    //             } else {
    //                 WRITE_WARNINGF(TL("Could not set signal on connection (signal: %, group: %)"), getID(), group->getID());
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_links(&mut self) {
        todo!("port NBLoadedTLDef::collectLinks")
    }

    /// C++: `bool NBLoadedTLDef::mustBrake(const NBConnection &, const std::string &, int) const` [const, protected]
    /// Returns the information whether a connection must brake, given a phase
    // bool
    // NBLoadedTLDef::mustBrake(const NBConnection& possProhibited,
    //                          const std::string& state,
    //                          int strmpos) const {
    //     // check whether the stream has red
    //     if (state[strmpos] != 'g' && state[strmpos] != 'G') {
    //         return true;
    //     }
    //
    //     // check whether another stream which has green is a higher
    //     //  priorised foe to the given
    //     int pos = 0;
    //     for (SignalGroupCont::const_iterator i = mySignalGroups.begin(); i != mySignalGroups.end(); i++) {
    //         SignalGroup* group = (*i).second;
    //         // get other links that have green
    //         int linkNo = group->getLinkNo();
    //         for (int j = 0; j < linkNo; j++) {
    //             // get the current connection (possible foe)
    //             const NBConnection& other = group->getConnection(j);
    //             NBConnection possProhibitor(other);
    //             // if the connection ist still valid ...
    //             if (possProhibitor.check(*myEdgeCont)) {
    //                 // ... do nothing if it starts at the same edge
    //                 if (possProhibited.getFrom() == possProhibitor.getFrom()) {
    //                     pos++;
    //                     continue;
    //                 }
    //                 if (state[pos] == 'g' || state[pos] == 'G') {
    //                     if (NBTrafficLightDefinition::mustBrake(possProhibited, possProhibitor, true)) {
    //                         return true;
    //                     }
    //                 }
    //                 pos++;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn must_brake(&self, possProhibited: &NBConnection, state: &str, strmpos: i32) -> bool {
        todo!("port NBLoadedTLDef::mustBrake")
    }

    /// C++: `void NBLoadedTLDef::replaceRemoved(NBEdge *, int, NBEdge *, int, bool)` [virtual, protected]
    /// Replaces a removed edge/lane
    // void
    // NBLoadedTLDef::replaceRemoved(NBEdge* removed, int removedLane,
    //                               NBEdge* by, int byLane, bool incoming) {
    //     for (SignalGroupCont::const_iterator i = mySignalGroups.begin(); i != mySignalGroups.end(); i++) {
    //         SignalGroup* group = (*i).second;
    //         if ((incoming && group->containsIncoming(removed)) || (!incoming && group->containsOutgoing(removed))) {
    //             group->remap(removed, removedLane, by, byLane);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_removed(&mut self, removed: Ptr<NBEdge>, removedLane: i32, by: Ptr<NBEdge>, byLane: i32, incoming: bool) {
        todo!("port NBLoadedTLDef::replaceRemoved")
    }

    /// C++: `std::string NBLoadedTLDef::buildPhaseState(const SUMOTime) const` [const, private]
    /// Builds the phase for a given time
    // std::string
    // NBLoadedTLDef::buildPhaseState(const SUMOTime time) const {
    //     int pos = 0;
    //     std::string state;
    //     // set the green and yellow information first;
    //     //  the information whether other have to break needs those masks
    //     //  completely filled
    //     for (SignalGroupCont::const_iterator i = mySignalGroups.begin(); i != mySignalGroups.end(); i++) {
    //         SignalGroup* group = (*i).second;
    //         int linkNo = group->getLinkNo();
    //         bool mayDrive = group->mayDrive(time);
    //         bool hasYellow = group->hasYellow(time);
    //         char c = 'r';
    //         if (mayDrive) {
    //             c = 'g';
    //         }
    //         if (hasYellow) {
    //             c = 'y';
    //         }
    //         for (int j = 0; j < linkNo; j++) {
    //             const NBConnection& conn = group->getConnection(j);
    //             NBConnection assConn(conn);
    //             // assert that the connection really exists
    //             if (assConn.check(*myEdgeCont)) {
    //                 state = state + c;
    //                 ++pos;
    //             }
    //         }
    //     }
    //     // set the braking mask
    //     pos = 0;
    //     for (SignalGroupCont::const_iterator i = mySignalGroups.begin(); i != mySignalGroups.end(); i++) {
    //         SignalGroup* group = (*i).second;
    //         int linkNo = group->getLinkNo();
    //         for (int j = 0; j < linkNo; j++) {
    //             const NBConnection& conn = group->getConnection(j);
    //             NBConnection assConn(conn);
    //             if (assConn.check(*myEdgeCont)) {
    //                 if (!mustBrake(assConn, state, pos)) {
    //                     if (state[pos] == 'g') {
    //                         state[pos] = 'G';
    //                     }
    //                     if (state[pos] == 'y') {
    //                         state[pos] = 'Y';
    //                     }
    //                 }
    //                 pos++;
    //             }
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn build_phase_state(&self, time: SUMOTime) -> String {
        todo!("port NBLoadedTLDef::buildPhaseState")
    }

}

/// Definition of a single, loaded phase
/// C++ class `SignalGroup::PhaseDef` (netbuild/NBLoadedTLDef.h)
pub struct SignalGroup_PhaseDef {
    /// The begin time of this phase
    pub myTime: SUMOTime,
    /// A signal's color from this time
    pub myColor: NBTrafficLightDefinition_TLColor,
}

impl SignalGroup_PhaseDef {
    /// C++: `void PhaseDef::PhaseDef(SUMOTime, TLColor)`
    /// Constructor
    // PhaseDef(SUMOTime time, TLColor color)
    //                 : myTime(time), myColor(color) { }
    #[allow(unused_variables)]
    pub fn new(time: SUMOTime, color: NBTrafficLightDefinition_TLColor) -> Self {
        todo!("port PhaseDef::PhaseDef")
    }

}

/// A single signal group, may control several connections
/// C++ class `NBLoadedTLDef::SignalGroup` (netbuild/NBLoadedTLDef.h)
#[repr(C)]
pub struct NBLoadedTLDef_SignalGroup {
    /// C++ base class `Named`
    pub base: Named,
    /// Connections controlled by this signal
    pub myConnections: NBConnectionVector,
    /// The phases of this signal
    pub myPhases: Vec<SignalGroup_PhaseDef>,
    /// The times of redyellow and yellow
    pub myTRedYellow: SUMOTime,
    /// The times of redyellow and yellow
    pub myTYellow: SUMOTime,
}

impl std::ops::Deref for NBLoadedTLDef_SignalGroup {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NBLoadedTLDef_SignalGroup {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl NBLoadedTLDef_SignalGroup {
    /// C++: `void SignalGroup::SignalGroup(const std::string &)`
    /// Constructor
    // NBLoadedTLDef::SignalGroup::SignalGroup(const std::string& id)
    //     : Named(id) {}
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        todo!("port SignalGroup::SignalGroup")
    }

    /// C++: `void SignalGroup::~SignalGroup() noexcept` [virtual]
    /// Destructor
    // NBLoadedTLDef::SignalGroup::~SignalGroup() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SignalGroup::~SignalGroup")
    }

    /// C++: `void SignalGroup::addConnection(const NBConnection &)`
    /// Inserts a controlled connection
    // void
    // NBLoadedTLDef::SignalGroup::addConnection(const NBConnection& c) {
    //     assert(c.getFromLane() < 0 || c.getFrom()->getNumLanes() > c.getFromLane());
    //     myConnections.push_back(c);
    // }
    #[allow(unused_variables)]
    pub fn add_connection(&mut self, c: &NBConnection) {
        todo!("port SignalGroup::addConnection")
    }

    /// C++: `void SignalGroup::addPhaseBegin(SUMOTime, TLColor)`
    /// Sets the begin of a phase
    // void
    // NBLoadedTLDef::SignalGroup::addPhaseBegin(SUMOTime time, TLColor color) {
    //     myPhases.push_back(PhaseDef(time, color));
    // }
    #[allow(unused_variables)]
    pub fn add_phase_begin(&mut self, time: SUMOTime, color: NBTrafficLightDefinition_TLColor) {
        todo!("port SignalGroup::addPhaseBegin")
    }

    /// C++: `void SignalGroup::setYellowTimes(SUMOTime, SUMOTime)`
    /// Sets the times for redyellow and yellow
    // void
    // NBLoadedTLDef::SignalGroup::setYellowTimes(SUMOTime tRedYellow, SUMOTime tYellow) {
    //     myTRedYellow = tRedYellow;
    //     myTYellow = tYellow;
    // }
    #[allow(unused_variables)]
    pub fn set_yellow_times(&mut self, tRedYellowe: SUMOTime, tYellow: SUMOTime) {
        todo!("port SignalGroup::setYellowTimes")
    }

    /// C++: `std::vector<SUMOTime> SignalGroup::getTimes(SUMOTime) const` [const]
    /// Returns the times at which the signal switches
    // std::vector<SUMOTime>
    // NBLoadedTLDef::SignalGroup::getTimes(SUMOTime cycleDuration) const {
    //     // within the phase container, we should have the green and red phases add their times
    //     std::vector<SUMOTime> ret;
    //     for (const PhaseDef& p : myPhases) {
    //         ret.push_back(p.myTime);
    //     }
    //     // further, we possibly should set the yellow phases
    //     if (myTYellow > 0) {
    //         for (const PhaseDef& p : myPhases) {
    //             if (p.myColor == TLCOLOR_RED) {
    //                 ret.push_back((p.myTime + myTYellow) % cycleDuration);
    //             }
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_times(&self, cycleDuration: SUMOTime) -> Vec<SUMOTime> {
        todo!("port SignalGroup::getTimes")
    }

    /// C++: `void SignalGroup::sortPhases()`
    /// Sorts the phases
    // void
    // NBLoadedTLDef::SignalGroup::sortPhases() {
    //     std::sort(myPhases.begin(), myPhases.end(), [](const PhaseDef & p1, const PhaseDef & p2) {
    //         return p1.myTime < p2.myTime;
    //     });
    // }
    #[allow(unused_variables)]
    pub fn sort_phases(&mut self) {
        todo!("port SignalGroup::sortPhases")
    }

    /// C++: `int SignalGroup::getLinkNo() const` [const]
    /// Returns the number of links (connection) controlled by this signal
    // int
    // NBLoadedTLDef::SignalGroup::getLinkNo() const {
    //     return (int) myConnections.size();
    // }
    #[allow(unused_variables)]
    pub fn get_link_no(&self) -> i32 {
        todo!("port SignalGroup::getLinkNo")
    }

    /// C++: `bool SignalGroup::mayDrive(SUMOTime) const` [const]
    /// Returns whether vehicles on controlled links may drive at the given time
    // bool
    // NBLoadedTLDef::SignalGroup::mayDrive(SUMOTime time) const {
    //     assert(myPhases.size() != 0);
    //     for (std::vector<PhaseDef>::const_reverse_iterator i = myPhases.rbegin(); i != myPhases.rend(); i++) {
    //         SUMOTime nextTime = (*i).myTime;
    //         if (time >= nextTime) {
    //             return (*i).myColor == TLCOLOR_GREEN;
    //         }
    //     }
    //     return (*(myPhases.end() - 1)).myColor == TLCOLOR_GREEN;
    // }
    #[allow(unused_variables)]
    pub fn may_drive(&self, time: SUMOTime) -> bool {
        todo!("port SignalGroup::mayDrive")
    }

    /// C++: `bool SignalGroup::hasYellow(SUMOTime) const` [const]
    /// Returns whether controlled links have yellow at the given time
    // bool
    // NBLoadedTLDef::SignalGroup::hasYellow(SUMOTime time) const {
    //     bool has_red_now = !mayDrive(time);
    //     bool had_green = mayDrive(time - myTYellow);
    //     return has_red_now && had_green;
    // }
    #[allow(unused_variables)]
    pub fn has_yellow(&self, time: SUMOTime) -> bool {
        todo!("port SignalGroup::hasYellow")
    }

    /// C++: `bool SignalGroup::containsIncoming(NBEdge *) const` [const]
    /// Returns whether this signal controls the given edge
    // bool
    // NBLoadedTLDef::SignalGroup::containsIncoming(NBEdge* from) const {
    //     for (NBConnectionVector::const_iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         if ((*i).getFrom() == from) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn contains_incoming(&self, from: Ptr<NBEdge>) -> bool {
        todo!("port SignalGroup::containsIncoming")
    }

    /// C++: `void SignalGroup::remapIncoming(NBEdge *, const EdgeVector &)`
    /// Replaces the given incoming edge by the others given
    // void
    // NBLoadedTLDef::SignalGroup::remapIncoming(NBEdge* which, const EdgeVector& by) {
    //     NBConnectionVector newConns;
    //     for (NBConnectionVector::iterator i = myConnections.begin(); i != myConnections.end();) {
    //         if ((*i).getFrom() == which) {
    //             NBConnection conn((*i).getFrom(), (*i).getTo());
    //             i = myConnections.erase(i);
    //             for (EdgeVector::const_iterator j = by.begin(); j != by.end(); j++) {
    //                 NBConnection curr(conn);
    //                 if (!curr.replaceFrom(which, *j)) {
    //                     throw ProcessError("Could not replace edge '" + which->getID() + "' by '" + (*j)->getID() + "'.\nUndefined...");
    //                 }
    //                 newConns.push_back(curr);
    //             }
    //         } else {
    //             i++;
    //         }
    //     }
    //     copy(newConns.begin(), newConns.end(),
    //          back_inserter(myConnections));
    // }
    #[allow(unused_variables)]
    pub fn remap_incoming(&mut self, which: Ptr<NBEdge>, by: &EdgeVector) {
        todo!("port SignalGroup::remapIncoming")
    }

    /// C++: `bool SignalGroup::containsOutgoing(NBEdge *) const` [const]
    /// Returns whether this signal controls a connection where the given edge is the destination
    // bool
    // NBLoadedTLDef::SignalGroup::containsOutgoing(NBEdge* to) const {
    //     for (NBConnectionVector::const_iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         if ((*i).getTo() == to) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn contains_outgoing(&self, to: Ptr<NBEdge>) -> bool {
        todo!("port SignalGroup::containsOutgoing")
    }

    /// C++: `void SignalGroup::remapOutgoing(NBEdge *, const EdgeVector &)`
    /// Replaces the given outgoing edge by the others given
    // void
    // NBLoadedTLDef::SignalGroup::remapOutgoing(NBEdge* which, const EdgeVector& by) {
    //     NBConnectionVector newConns;
    //     for (NBConnectionVector::iterator i = myConnections.begin(); i != myConnections.end();) {
    //         if ((*i).getTo() == which) {
    //             NBConnection conn((*i).getFrom(), (*i).getTo());
    //             i = myConnections.erase(i);
    //             for (EdgeVector::const_iterator j = by.begin(); j != by.end(); j++) {
    //                 NBConnection curr(conn);
    //                 if (!curr.replaceTo(which, *j)) {
    //                     throw ProcessError("Could not replace edge '" + which->getID() + "' by '" + (*j)->getID() + "'.\nUndefined...");
    //                 }
    //                 newConns.push_back(curr);
    //             }
    //         } else {
    //             i++;
    //         }
    //     }
    //     copy(newConns.begin(), newConns.end(),
    //          back_inserter(myConnections));
    // }
    #[allow(unused_variables)]
    pub fn remap_outgoing(&mut self, which: Ptr<NBEdge>, by: &EdgeVector) {
        todo!("port SignalGroup::remapOutgoing")
    }

    /// C++: `const NBConnection & SignalGroup::getConnection(int) const` [const]
    /// Returns the connection at the given index
    // const NBConnection&
    // NBLoadedTLDef::SignalGroup::getConnection(int pos) const {
    //     assert(pos < (int)myConnections.size());
    //     return myConnections[pos];
    // }
    #[allow(unused_variables)]
    pub fn get_connection(&self, pos: i32) -> &NBConnection {
        todo!("port SignalGroup::getConnection")
    }

    /// C++: `void SignalGroup::patchTYellow(SUMOTime, bool)`
    /// Sets the yellow time
    // void
    // NBLoadedTLDef::SignalGroup::patchTYellow(SUMOTime tyellow, bool forced) {
    //     if (myTYellow < 0) {
    //         // was not set before (was not loaded)
    //         myTYellow = tyellow;
    //     } else if (forced && myTYellow < tyellow) {
    //         WRITE_WARNINGF(TL("TYellow of signal group '%' was less than the computed one; patched (was:%, is:%)"), getID(), toString(myTYellow), time2string(tyellow));
    //         myTYellow = tyellow;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn patch_t_yellow(&mut self, tyellow: SUMOTime, forced: bool) {
        todo!("port SignalGroup::patchTYellow")
    }

    /// C++: `void SignalGroup::remap(NBEdge *, int, NBEdge *, int)`
    /// Replaces a removed edge/lane
    // void
    // NBLoadedTLDef::SignalGroup::remap(NBEdge* removed, int removedLane,
    //                                   NBEdge* by, int byLane) {
    //     for (NBConnectionVector::iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         if ((*i).getTo() == removed
    //                 &&
    //                 ((*i).getToLane() == removedLane
    //                  ||
    //                  (*i).getToLane() == -1)) {
    //             (*i).replaceTo(removed, removedLane, by, byLane);
    //
    //         } else if ((*i).getTo() == removed && removedLane == -1) {
    //             (*i).replaceTo(removed, by);
    //         }
    //
    //         if ((*i).getFrom() == removed
    //                 &&
    //                 ((*i).getFromLane() == removedLane
    //                  ||
    //                  (*i).getFromLane() == -1)) {
    //             (*i).replaceFrom(removed, removedLane, by, byLane);
    //
    //         } else if ((*i).getFrom() == removed && removedLane == -1) {
    //             (*i).replaceFrom(removed, by);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remap(&mut self, removed: Ptr<NBEdge>, removedLane: i32, by: Ptr<NBEdge>, byLane: i32) {
        todo!("port SignalGroup::remap")
    }

}

