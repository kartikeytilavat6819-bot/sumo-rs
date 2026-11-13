//! Port of netbuild/NBLoadedTLDef.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;

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
        NBLoadedTLDef {
            base: NBTrafficLightDefinition::new(id, junctions, "0", offset, type_), // DefaultProgramID = "0"
            myEdgeCont: Ptr::new(ec),
            mySignalGroups: BTreeMap::new(),
            myCycleDuration: 0,
        }
    }

    /// C++: `void NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont &, const std::string &, NBNode *, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont& ec, const std::string& id, NBNode* junction, SUMOTime offset, TrafficLightType type) :
    //     NBTrafficLightDefinition(id, junction, DefaultProgramID, offset, type),
    //     myEdgeCont(&ec) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(ec: &NBEdgeCont, id: &str, junction: Ptr<NBNode>, offset: SUMOTime, type_: TrafficLightType) -> Self {
        NBLoadedTLDef {
            base: NBTrafficLightDefinition::new_2(id, junction, "0", offset, type_), // DefaultProgramID = "0"
            myEdgeCont: Ptr::new(ec),
            mySignalGroups: BTreeMap::new(),
            myCycleDuration: 0,
        }
    }

    /// C++: `void NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont &, const std::string &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBLoadedTLDef::NBLoadedTLDef(const NBEdgeCont& ec, const std::string& id, SUMOTime offset, TrafficLightType type) :
    //     NBTrafficLightDefinition(id, DefaultProgramID, offset, type),
    //     myEdgeCont(&ec) {
    // }
    #[allow(unused_variables)]
    pub fn new_3(ec: &NBEdgeCont, id: &str, offset: SUMOTime, type_: TrafficLightType) -> Self {
        NBLoadedTLDef {
            base: NBTrafficLightDefinition::new_3(id, "0", offset, type_), // DefaultProgramID = "0"
            myEdgeCont: Ptr::new(ec),
            mySignalGroups: BTreeMap::new(),
            myCycleDuration: 0,
        }
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
        for g in self.mySignalGroups.values() {
            // the groups were allocated with Box::into_raw in add_signal_group
            unsafe { drop(Box::from_raw(g.as_raw())) };
        }
    }

    /// C++: `void NBLoadedTLDef::setCycleDuration(SUMOTime)`
    /// Sets the duration of a cycle
    // void
    // NBLoadedTLDef::setCycleDuration(SUMOTime cycleDur) {
    //     myCycleDuration = cycleDur;
    // }
    #[allow(unused_variables)]
    pub fn set_cycle_duration(&mut self, cycleDur: SUMOTime) {
        self.myCycleDuration = cycleDur;
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
        debug_assert!(!self.mySignalGroups.contains_key(id));
        let g = Box::into_raw(Box::new(NBLoadedTLDef_SignalGroup::new(id)));
        self.mySignalGroups.insert(id.to_string(), Ptr::from_raw(g));
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
        if !self.mySignalGroups.contains_key(groupid) {
            return false;
        }
        let group = *self.mySignalGroups.get(groupid).unwrap();
        group.m().add_connection(connection);
        let n1 = connection.get_from().get_to_node();
        if !n1.is_null() {
            self.add_node(n1);
            n1.m().add_traffic_light(Ptr::new(&self.base));
        }
        let n2 = connection.get_to().get_from_node();
        if !n2.is_null() {
            self.add_node(n2);
            n2.m().add_traffic_light(Ptr::new(&self.base));
        }
        true
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
        let mut ok = true;
        for c in connections {
            ok &= self.add_to_signal_group(groupid, c);
        }
        ok
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
        debug_assert!(self.mySignalGroups.contains_key(groupid));
        let group = *self.mySignalGroups.get(groupid).unwrap();
        group.m().add_phase_begin(time, color);
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
        debug_assert!(self.mySignalGroups.contains_key(groupid));
        let group = *self.mySignalGroups.get(groupid).unwrap();
        group.m().set_yellow_times(tRedYellow, tYellow);
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
        // assign the tl-indices to the edge connections
        for c in self.myControlledLinks.iter() {
            if c.get_tl_index() != -1 {
                let _ = c.get_from().m().set_controlling_tl_information(c, self.base.base.get_id());
            }
        }
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
        for &group in self.mySignalGroups.values() {
            if group.contains_incoming(removed) {
                group.m().remap_incoming(removed, incoming);
            }
            if group.contains_outgoing(removed) {
                group.m().remap_outgoing(removed, outgoing);
            }
        }
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
        if !self.myNeedsContRelationReady {
            panic!("{}", ProcessError::new("myNeedsContRelation was not properly initialized\n"));
        }
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
        self.set_participants_information();
        let logic = utils::options::options_cont::with_options(|oc| self.compute(oc));
        if !logic.is_null() {
            logic.m().get_num_links() - 1
        } else {
            -1
        }
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
        // MsgHandler::getWarningInstance()->clear()
        utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Warning, |h| h.clear(true));
        // compute the switching times and count the signals
        let mut switchTimes: BTreeSet<SUMOTime> = BTreeSet::new();
        let mut numSignals: i32 = 0;
        let yellowPatch = utils::options::options_cont::get_bool("tls.yellow.patch-small");
        for group in self.mySignalGroups.values() {
            let group = *group;
            // needed later
            group.m().sort_phases();
            // patch the yellow time for this group
            group.m().patch_t_yellow(utils::common::time::time2steps(brakingTimeSeconds as f64), yellowPatch);
            // both the given red and green phases are added and also the yellow times
            let gtimes = group.get_times(self.myCycleDuration);
            switchTimes.extend(gtimes);
            numSignals += group.get_link_no();
        }
        // build the phases
        let logicId = self.base.base.get_id().to_string();
        let logic: Ptr<NBTrafficLightLogic> = Ptr::from_raw(Box::into_raw(Box::new(NBTrafficLightLogic::new(
            &logicId,
            self.get_program_id(),
            numSignals,
            self.myOffset,
            self.myType,
        ))));
        let mut prev: SUMOTime = -1;
        for &l in &switchTimes {
            if prev != -1 {
                let st = self.build_phase_state(prev);
                logic.m().add_step(l - prev, &st, &Vec::<i32>::new(), "", -1);
            }
            prev = l;
        }
        // C++ dereferences begin() unconditionally; an empty switch set is UB there
        let first = *switchTimes.iter().next().unwrap();
        let st = self.build_phase_state(prev);
        logic.m().add_step(self.myCycleDuration + first - prev, &st, &Vec::<i32>::new(), "", -1);
        // check whether any warnings were printed
        if utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Warning, |h| h.was_informed()) {
            write_warningf!("During computation of traffic light '%'.", self.base.base.get_id());
        }
        logic.m().close_building(true);

        // initialize myNeedsContRelation
        self.myNeedsContRelation.clear();
        let controlledWithin = !utils::options::options_cont::get_bool("tls.uncontrolled-within");
        let states: Vec<String> = logic.get_phases().iter().map(|p| p.state.clone()).collect();
        let links = Ptr::new(&self.myControlledLinks);
        for state in &states {
            let sb = state.as_bytes();
            for c1 in links.iter() {
                let i1 = c1.get_tl_index();
                if i1 == -1 || sb[i1 as usize] != b'g' || c1.get_from().is_null() || c1.get_to().is_null() {
                    continue;
                }
                for c2 in links.iter() {
                    let i2 = c2.get_tl_index();
                    if i2 != -1
                        && i2 != i1
                        && (sb[i2 as usize] == b'G' || sb[i2 as usize] == b'g')
                        && !c2.get_from().is_null()
                        && !c2.get_to().is_null()
                    {
                        let rightTurnConflict = NBNode::right_turn_conflict(
                            c1.get_from(),
                            c1.get_to(),
                            c1.get_from_lane(),
                            c2.get_from(),
                            c2.get_to(),
                            c2.get_from_lane(),
                        );
                        if self.forbids(c2.get_from(), c2.get_to(), c1.get_from(), c1.get_to(), true, controlledWithin) || rightTurnConflict {
                            self.myNeedsContRelation.insert(NBTrafficLightDefinition_StreamPair::new(
                                c1.get_from(),
                                c1.get_to(),
                                c2.get_from(),
                                c2.get_to(),
                            ));
                        }
                    }
                }
            }
        }
        self.myNeedsContRelationReady = true;
        logic
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
        // assign participating nodes to the request
        self.collect_nodes();
        // collect the information about participating edges and links
        self.collect_edges();
        self.collect_links();
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
        self.myControlledNodes.clear();
        // local buffer: pushing into self while iterating mySignalGroups through self is a borrow conflict
        let mut controlled = Vec::new();
        for &group in self.mySignalGroups.values() {
            let linkNo = group.get_link_no();
            for j in 0..linkNo {
                let conn = group.get_connection(j);
                let edge = conn.get_from();
                let node = edge.get_to_node();
                controlled.push(node);
            }
        }
        self.myControlledNodes.extend(controlled);
        // determinism: C++ std::sort is unstable; node IDs are unique so ties do not occur
        self.myControlledNodes.sort_by(|a, b| a.get_id().cmp(b.get_id()));
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
        self.myControlledLinks.clear();
        // build the list of links which are controlled by the traffic light
        // local buffer: pushing into self while iterating myIncomingEdges through self is a borrow conflict
        let mut controlled: NBConnectionVector = Vec::new();
        for &incoming in self.myIncomingEdges.iter() {
            let noLanes = incoming.get_num_lanes();
            for j in 0..noLanes {
                let elv = incoming.get_connections_from_lane(j, Ptr::null(), -1);
                for el in elv {
                    if !el.toEdge.is_null() {
                        controlled.push(NBConnection::new_2(incoming, j, el.toEdge, el.toLane, NBCONNECTION_INVALID_TL_INDEX, NBCONNECTION_INVALID_TL_INDEX));
                    }
                }
            }
        }
        self.myControlledLinks.extend(controlled);

        // assign tl-indices to myControlledLinks
        let mut pos: i32 = 0;
        // local copy of the group pointers: the loop mutates myControlledLinks through self
        let groups: Vec<Ptr<NBLoadedTLDef_SignalGroup>> = self.mySignalGroups.values().copied().collect();
        for group in groups {
            let linkNo = group.get_link_no();
            for j in 0..linkNo {
                let conn = group.get_connection(j);
                debug_assert!(conn.get_from_lane() < 0 || conn.get_from().get_num_lanes() > conn.get_from_lane());
                let mut tst = NBConnection::new_4(conn);
                tst.set_tl_index(pos);
                if tst.check(&*self.myEdgeCont) {
                    if tst.get_from().may_be_tls_controlled(tst.get_from_lane(), tst.get_to(), tst.get_to_lane()) {
                        for c in self.myControlledLinks.iter_mut() {
                            if c.get_tl_index() == NBCONNECTION_INVALID_TL_INDEX
                                && tst.get_from() == c.get_from()
                                && tst.get_to() == c.get_to()
                                && (tst.get_from_lane() < 0 || tst.get_from_lane() == c.get_from_lane())
                                && (tst.get_to_lane() < 0 || tst.get_to_lane() == c.get_to_lane())
                            {
                                c.set_tl_index(pos);
                            }
                        }
                        pos += 1;
                    }
                } else {
                    write_warningf!("Could not set signal on connection (signal: %, group: %)", self.base.base.get_id(), group.get_id());
                }
            }
        }
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
        let sb = state.as_bytes();
        // check whether the stream has red
        if sb[strmpos as usize] != b'g' && sb[strmpos as usize] != b'G' {
            return true;
        }
        // check whether another stream which has green is a higher priorised foe to the given
        let mut pos: usize = 0;
        for &group in self.mySignalGroups.values() {
            // get other links that have green
            let linkNo = group.get_link_no();
            for j in 0..linkNo {
                // get the current connection (possible foe)
                let other = group.get_connection(j);
                let mut possProhibitor = NBConnection::new_4(other);
                // if the connection ist still valid ...
                if possProhibitor.check(&*self.myEdgeCont) {
                    // ... do nothing if it starts at the same edge
                    if possProhibited.get_from() == possProhibitor.get_from() {
                        pos += 1;
                        continue;
                    }
                    if sb[pos] == b'g' || sb[pos] == b'G' {
                        if self.must_brake_2(possProhibited, &possProhibitor, true) {
                            return true;
                        }
                    }
                    pos += 1;
                }
            }
        }
        false
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
        for &group in self.mySignalGroups.values() {
            if (incoming && group.contains_incoming(removed)) || (!incoming && group.contains_outgoing(removed)) {
                group.m().remap(removed, removedLane, by, byLane);
            }
        }
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
        let mut state: Vec<u8> = Vec::new();
        // set the green and yellow information first;
        // the information whether other have to break needs those masks completely filled
        for &group in self.mySignalGroups.values() {
            let linkNo = group.get_link_no();
            let mayDrive = group.may_drive(time);
            let hasYellow = group.has_yellow(time);
            let mut c = b'r';
            if mayDrive {
                c = b'g';
            }
            if hasYellow {
                c = b'y';
            }
            for j in 0..linkNo {
                let conn = group.get_connection(j);
                let mut assConn = NBConnection::new_4(conn);
                // assert that the connection really exists
                if assConn.check(&*self.myEdgeCont) {
                    state.push(c);
                }
            }
        }
        // set the braking mask
        let mut pos: usize = 0;
        for &group in self.mySignalGroups.values() {
            let linkNo = group.get_link_no();
            for j in 0..linkNo {
                let conn = group.get_connection(j);
                let mut assConn = NBConnection::new_4(conn);
                if assConn.check(&*self.myEdgeCont) {
                    let stateStr = String::from_utf8(state.clone()).unwrap();
                    if !self.must_brake(&assConn, &stateStr, pos as i32) {
                        if state[pos] == b'g' {
                            state[pos] = b'G';
                        }
                        if state[pos] == b'y' {
                            state[pos] = b'Y';
                        }
                    }
                    pos += 1;
                }
            }
        }
        String::from_utf8(state).unwrap()
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
        SignalGroup_PhaseDef { myTime: time, myColor: color }
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
        NBLoadedTLDef_SignalGroup {
            base: Named::new(id),
            myConnections: Vec::new(),
            myPhases: Vec::new(),
            // C++ leaves these uninitialized; -1 matches the "was not set" check in patchTYellow
            myTRedYellow: -1,
            myTYellow: -1,
        }
    }

    /// C++: `void SignalGroup::~SignalGroup() noexcept` [virtual]
    /// Destructor
    // NBLoadedTLDef::SignalGroup::~SignalGroup() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        self.myConnections.push(NBConnection::new_4(c));
    }

    /// C++: `void SignalGroup::addPhaseBegin(SUMOTime, TLColor)`
    /// Sets the begin of a phase
    // void
    // NBLoadedTLDef::SignalGroup::addPhaseBegin(SUMOTime time, TLColor color) {
    //     myPhases.push_back(PhaseDef(time, color));
    // }
    #[allow(unused_variables)]
    pub fn add_phase_begin(&mut self, time: SUMOTime, color: NBTrafficLightDefinition_TLColor) {
        self.myPhases.push(SignalGroup_PhaseDef::new(time, color));
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
        self.myTRedYellow = tRedYellowe;
        self.myTYellow = tYellow;
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
        // within the phase container, we should have the green and red phases add their times
        let mut ret: Vec<SUMOTime> = Vec::new();
        for p in &self.myPhases {
            ret.push(p.myTime);
        }
        // further, we possibly should set the yellow phases
        if self.myTYellow > 0 {
            for p in &self.myPhases {
                if p.myColor == NBTrafficLightDefinition_TLColor::TLCOLOR_RED {
                    ret.push((p.myTime + self.myTYellow) % cycleDuration);
                }
            }
        }
        ret
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
        // determinism: stable sort; C++ std::sort is unstable on equal begin times
        self.myPhases.sort_by(|p1, p2| p1.myTime.cmp(&p2.myTime));
    }

    /// C++: `int SignalGroup::getLinkNo() const` [const]
    /// Returns the number of links (connection) controlled by this signal
    // int
    // NBLoadedTLDef::SignalGroup::getLinkNo() const {
    //     return (int) myConnections.size();
    // }
    #[allow(unused_variables)]
    pub fn get_link_no(&self) -> i32 {
        self.myConnections.len() as i32
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
        for p in self.myPhases.iter().rev() {
            let nextTime = p.myTime;
            if time >= nextTime {
                return p.myColor == NBTrafficLightDefinition_TLColor::TLCOLOR_GREEN;
            }
        }
        self.myPhases[self.myPhases.len() - 1].myColor == NBTrafficLightDefinition_TLColor::TLCOLOR_GREEN
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
        let has_red_now = !self.may_drive(time);
        let had_green = self.may_drive(time - self.myTYellow);
        has_red_now && had_green
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
        self.myConnections.iter().any(|c| c.get_from() == from)
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
        let mut newConns: Vec<NBConnection> = Vec::new();
        let mut i = 0;
        while i < self.myConnections.len() {
            if self.myConnections[i].get_from() == which {
                let conn = NBConnection::new(self.myConnections[i].get_from(), self.myConnections[i].get_to());
                self.myConnections.remove(i);
                for &j in by {
                    let mut curr = NBConnection::new_4(&conn);
                    if !curr.replace_from(which, j) {
                        panic!("{}", ProcessError::new(format!("Could not replace edge '{}' by '{}'.\nUndefined...", which.get_id(), j.get_id())));
                    }
                    newConns.push(curr);
                }
            } else {
                i += 1;
            }
        }
        self.myConnections.extend(newConns);
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
        self.myConnections.iter().any(|c| c.get_to() == to)
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
        let mut newConns: Vec<NBConnection> = Vec::new();
        let mut i = 0;
        while i < self.myConnections.len() {
            if self.myConnections[i].get_to() == which {
                let conn = NBConnection::new(self.myConnections[i].get_from(), self.myConnections[i].get_to());
                self.myConnections.remove(i);
                for &j in by {
                    let mut curr = NBConnection::new_4(&conn);
                    if !curr.replace_to(which, j) {
                        panic!("{}", ProcessError::new(format!("Could not replace edge '{}' by '{}'.\nUndefined...", which.get_id(), j.get_id())));
                    }
                    newConns.push(curr);
                }
            } else {
                i += 1;
            }
        }
        self.myConnections.extend(newConns);
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
        &self.myConnections[pos as usize]
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
        if self.myTYellow < 0 {
            // was not set before (was not loaded)
            self.myTYellow = tyellow;
        } else if forced && self.myTYellow < tyellow {
            write_warning(&format!(
                "TYellow of signal group '{}' was less than the computed one; patched (was:{}, is:{})",
                self.base.get_id(),
                self.myTYellow,
                utils::common::time::time2string(tyellow)
            ));
            self.myTYellow = tyellow;
        }
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
        for c in self.myConnections.iter_mut() {
            if c.get_to() == removed && (c.get_to_lane() == removedLane || c.get_to_lane() == -1) {
                let _ = c.replace_to_2(removed, removedLane, by, byLane);
            } else if c.get_to() == removed && removedLane == -1 {
                let _ = c.replace_to(removed, by);
            }
            if c.get_from() == removed && (c.get_from_lane() == removedLane || c.get_from_lane() == -1) {
                let _ = c.replace_from_2(removed, removedLane, by, byLane);
            } else if c.get_from() == removed && removedLane == -1 {
                let _ = c.replace_from(removed, by);
            }
        }
    }

}

