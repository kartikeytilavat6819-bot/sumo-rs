//! Port of netbuild/NBTrafficLightDefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `NBTrafficLightDefinition::TLColor`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NBTrafficLightDefinition_TLColor {
    TLCOLOR_RED = 0,
    TLCOLOR_YELLOW = 1,
    TLCOLOR_REDYELLOW = 2,
    TLCOLOR_GREEN = 3,
    TLCOLOR_BLINK = 4,
}

/// C++ typedef `std::set<std::pair<int, int>> NBTrafficLightDefinition::ExtraConflicts`
pub type NBTrafficLightDefinition_ExtraConflicts = BTreeSet<(i32, i32)>;

/// C++ typedef `std::set<StreamPair> NBTrafficLightDefinition::NeedsContRelation`
pub type NBTrafficLightDefinition_NeedsContRelation = BTreeSet<NBTrafficLightDefinition_StreamPair>;

/// Virtual dispatch table for the pure virtual methods of NBTrafficLightDefinition.
/// Each subclass constructor installs its own table in `vt`; the base methods forward through it.
pub struct NBTrafficLightDefinitionVt {
    pub collect_links: fn(&mut NBTrafficLightDefinition),
    pub set_tl_controlling_information: fn(&NBTrafficLightDefinition),
    pub remap_removed: fn(&mut NBTrafficLightDefinition, Ptr<NBEdge>, &EdgeVector, &EdgeVector),
    pub replace_removed: fn(&mut NBTrafficLightDefinition, Ptr<NBEdge>, i32, Ptr<NBEdge>, i32, bool),
    pub my_compute: fn(&mut NBTrafficLightDefinition, i32) -> Ptr<NBTrafficLightLogic>,
    pub get_max_index: fn(&mut NBTrafficLightDefinition) -> i32,
}

fn default_collect_links(_: &mut NBTrafficLightDefinition) {
    panic!("NBTrafficLightDefinition::collectLinks is pure virtual")
}

fn default_set_tl_controlling_information(_: &NBTrafficLightDefinition) {
    panic!("NBTrafficLightDefinition::setTLControllingInformation is pure virtual")
}

fn default_remap_removed(_: &mut NBTrafficLightDefinition, _: Ptr<NBEdge>, _: &EdgeVector, _: &EdgeVector) {
    panic!("NBTrafficLightDefinition::remapRemoved is pure virtual")
}

fn default_replace_removed(_: &mut NBTrafficLightDefinition, _: Ptr<NBEdge>, _: i32, _: Ptr<NBEdge>, _: i32, _: bool) {
    panic!("NBTrafficLightDefinition::replaceRemoved is pure virtual")
}

fn default_my_compute(_: &mut NBTrafficLightDefinition, _: i32) -> Ptr<NBTrafficLightLogic> {
    panic!("NBTrafficLightDefinition::myCompute is pure virtual")
}

fn default_get_max_index(_: &mut NBTrafficLightDefinition) -> i32 {
    panic!("NBTrafficLightDefinition::getMaxIndex is pure virtual")
}

static DEFAULT_NB_TRAFFIC_LIGHT_DEFINITION_VT: NBTrafficLightDefinitionVt = NBTrafficLightDefinitionVt {
    collect_links: default_collect_links,
    set_tl_controlling_information: default_set_tl_controlling_information,
    remap_removed: default_remap_removed,
    replace_removed: default_replace_removed,
    my_compute: default_my_compute,
    get_max_index: default_get_max_index,
};

/// The base class for traffic light logic definitions
/// C++ class `NBTrafficLightDefinition` (netbuild/NBTrafficLightDefinition.h)
#[repr(C)]
pub struct NBTrafficLightDefinition {
    /// C++ base class `Named`
    pub base: Named,
    /// Virtual dispatch table; must stay after `base`
    pub vt: &'static NBTrafficLightDefinitionVt,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// The container with participating nodes
    pub myControlledNodes: Vec<Ptr<NBNode>>,
    /// The list of incoming edges
    pub myIncomingEdges: EdgeVector,
    /// The list of edges within the area controlled by the tls
    pub myEdgesWithin: EdgeVector,
    /// The list of controlled links
    pub myControlledLinks: NBConnectionVector,
    /// Set of inner edges that shall be controlled, though
    pub myControlledInnerEdges: BTreeSet<String>,
    /// The tls program's subid
    pub mySubID: String,
    /// The offset in the program
    pub myOffset: SUMOTime,
    /// The algorithm type for the traffic light
    pub myType: TrafficLightType,
    pub myNeedsContRelation: NBTrafficLightDefinition_NeedsContRelation,
    pub myNeedsContRelationReady: bool,
    pub myExtraConflicts: NBTrafficLightDefinition_ExtraConflicts,
    pub myExtraConflictsReady: bool,
}

impl std::ops::Deref for NBTrafficLightDefinition {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NBTrafficLightDefinition {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of NBTrafficLightDefinition (port as module-level statics, see AGENTS.md):
//   static const std::string NBTrafficLightDefinition::DefaultProgramID;
//   static const SUMOTime NBTrafficLightDefinition::UNSPECIFIED_DURATION;
//   static const int NBTrafficLightDefinition::MIN_YELLOW_SECONDS;
//   static const std::string NBTrafficLightDefinition::OSM_DIRECTION;
//   static const std::string NBTrafficLightDefinition::OSM_SIGNAL_DIRECTION;
//   static const std::string NBTrafficLightDefinition::DummyID;
pub const NBTrafficLightDefinition_DefaultProgramID: &str = "0";
pub const NBTrafficLightDefinition_UNSPECIFIED_DURATION: SUMOTime = -1;
pub const NBTrafficLightDefinition_MIN_YELLOW_SECONDS: i32 = 3;
pub const NBTrafficLightDefinition_OSM_DIRECTION: &str = "osm:direction";
pub const NBTrafficLightDefinition_OSM_SIGNAL_DIRECTION: &str = "railway:signal:direction";
pub const NBTrafficLightDefinition_DummyID: &str = "dummy";

impl NBTrafficLightDefinition {
    /// C++: `void NBTrafficLightDefinition::NBTrafficLightDefinition(const std::string &, const std::vector<NBNode *> &, const std::string &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBTrafficLightDefinition::NBTrafficLightDefinition(const std::string& id,
    //         const std::vector<NBNode*>& junctions, const std::string& programID,
    //         SUMOTime offset, TrafficLightType type) :
    //     Named(id),
    //     myControlledNodes(junctions),
    //     mySubID(programID), myOffset(offset),
    //     myType(type),
    //     myNeedsContRelationReady(false),
    //     myExtraConflictsReady(false) {
    //     std::vector<NBNode*>::iterator i = myControlledNodes.begin();
    //     while (i != myControlledNodes.end()) {
    //         for (std::vector<NBNode*>::iterator j = i + 1; j != myControlledNodes.end();) {
    //             if (*i == *j) {
    //                 j = myControlledNodes.erase(j);
    //             } else {
    //                 j++;
    //             }
    //         }
    //         i++;
    //     }
    //     std::sort(myControlledNodes.begin(), myControlledNodes.end(), NBNode::nodes_by_id_sorter());
    //     for (NBNode* const node : junctions) {
    //         node->addTrafficLight(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, junctions: &Vec<Ptr<NBNode>>, programID: &str, offset: SUMOTime, type_: TrafficLightType) -> Self {
        let mut myControlledNodes: Vec<Ptr<NBNode>> = junctions.clone();
        let mut i = 0;
        while i < myControlledNodes.len() {
            let mut j = i + 1;
            while j < myControlledNodes.len() {
                if myControlledNodes[i] == myControlledNodes[j] {
                    myControlledNodes.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
        // determinism: node ids are unique within a net, so this sort order is total
        myControlledNodes.sort_by(|a, b| a.get_id().cmp(b.get_id()));
        // shortcut: C++ calls node->addTrafficLight(this) here, but `this` is not stable in a by-value constructor, so the nodes are not registered (callers must register after boxing), upgrade: construct the definition boxed/in place
        Self {
            base: Named::new(id),
            vt: &DEFAULT_NB_TRAFFIC_LIGHT_DEFINITION_VT,
            base2: Parameterised::new(),
            myControlledNodes,
            myIncomingEdges: Vec::new(),
            myEdgesWithin: Vec::new(),
            myControlledLinks: Vec::new(),
            myControlledInnerEdges: BTreeSet::new(),
            mySubID: programID.to_string(),
            myOffset: offset,
            myType: type_,
            myNeedsContRelation: BTreeSet::new(),
            myNeedsContRelationReady: false,
            myExtraConflicts: BTreeSet::new(),
            myExtraConflictsReady: false,
        }
    }

    /// C++: `void NBTrafficLightDefinition::NBTrafficLightDefinition(const std::string &, NBNode *, const std::string &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBTrafficLightDefinition::NBTrafficLightDefinition(const std::string& id,
    //         NBNode* junction, const std::string& programID, SUMOTime offset, TrafficLightType type) :
    //     Named(id),
    //     mySubID(programID),
    //     myOffset(offset),
    //     myType(type),
    //     myNeedsContRelationReady(false),
    //     myExtraConflictsReady(false) {
    //     addNode(junction);
    // }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, junction: Ptr<NBNode>, programID: &str, offset: SUMOTime, type_: TrafficLightType) -> Self {
        // shortcut: C++ addNode(junction) also calls junction->addTrafficLight(this); `this` is not stable in a by-value constructor, so only the node list is filled, upgrade: construct the definition boxed/in place
        Self {
            base: Named::new(id),
            vt: &DEFAULT_NB_TRAFFIC_LIGHT_DEFINITION_VT,
            base2: Parameterised::new(),
            myControlledNodes: vec![junction],
            myIncomingEdges: Vec::new(),
            myEdgesWithin: Vec::new(),
            myControlledLinks: Vec::new(),
            myControlledInnerEdges: BTreeSet::new(),
            mySubID: programID.to_string(),
            myOffset: offset,
            myType: type_,
            myNeedsContRelation: BTreeSet::new(),
            myNeedsContRelationReady: false,
            myExtraConflicts: BTreeSet::new(),
            myExtraConflictsReady: false,
        }
    }

    /// C++: `void NBTrafficLightDefinition::NBTrafficLightDefinition(const std::string &, const std::string &, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBTrafficLightDefinition::NBTrafficLightDefinition(const std::string& id, const std::string& programID,
    //         SUMOTime offset, TrafficLightType type) :
    //     Named(id),
    //     mySubID(programID),
    //     myOffset(offset),
    //     myType(type),
    //     myNeedsContRelationReady(false),
    //     myExtraConflictsReady(false) {
    // }
    #[allow(unused_variables)]
    pub fn new_3(id: &str, programID: &str, offset: SUMOTime, type_: TrafficLightType) -> Self {
        Self {
            base: Named::new(id),
            vt: &DEFAULT_NB_TRAFFIC_LIGHT_DEFINITION_VT,
            base2: Parameterised::new(),
            myControlledNodes: Vec::new(),
            myIncomingEdges: Vec::new(),
            myEdgesWithin: Vec::new(),
            myControlledLinks: Vec::new(),
            myControlledInnerEdges: BTreeSet::new(),
            mySubID: programID.to_string(),
            myOffset: offset,
            myType: type_,
            myNeedsContRelation: BTreeSet::new(),
            myNeedsContRelationReady: false,
            myExtraConflicts: BTreeSet::new(),
            myExtraConflictsReady: false,
        }
    }

    /// C++: `void NBTrafficLightDefinition::~NBTrafficLightDefinition() noexcept` [virtual]
    /// Destructor
    // NBTrafficLightDefinition::~NBTrafficLightDefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `NBTrafficLightLogic * NBTrafficLightDefinition::compute(const OptionsCont &)`
    /// Computes the traffic light logic
    // NBTrafficLightLogic*
    // NBTrafficLightDefinition::compute(const OptionsCont& oc) {
    //     // it is not really a traffic light if no incoming edge exists
    //     if (amInvalid()) {
    //         // make a copy of myControlledNodes because it will be modified;
    //         std::vector<NBNode*> nodes = myControlledNodes;
    //         for (auto it : nodes) {
    //             it->removeTrafficLight(this);
    //         }
    //         WRITE_WARNINGF(TL("The traffic light '%' does not control any links; it will not be build."), getID());
    //         return nullptr;
    //     }
    //     // compute the time needed to brake
    //     int brakingTime = computeBrakingTime(oc.getFloat("tls.yellow.min-decel"));
    //     // perform the computation depending on whether the traffic light
    //     //  definition was loaded or shall be computed new completely
    //     if (!oc.isDefault("tls.yellow.time")) {
    //         brakingTime = oc.getInt("tls.yellow.time");
    //     }
    //     NBTrafficLightLogic* ret = myCompute(brakingTime);
    //     ret->updateParameters(getParametersMap());
    //     if (ret->getType() == TrafficLightType::ACTUATED) {
    //         for (const std::string param : {
    //                     "jam-threshold", "passing-time", "detector-gap", "detector-length", "inactive-threshold"
    //                 }) {
    //             if (!oc.isDefault("tls.actuated." + param) && !ret->hasParameter(param)) {
    //                 ret->setParameter(param, oc.getValueString("tls.actuated." + param));
    //             }
    //         }
    //         if (!oc.isDefault("tls.actuated.show-detectors") && !ret->hasParameter("show-detectors")) {
    //             ret->setParameter("show-detectors", oc.getValueString("tls.actuated.show-detectors"));
    //         }
    //
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn compute(&mut self, oc: &OptionsCont) -> Ptr<NBTrafficLightLogic> {
        // it is not really a traffic light if no incoming edge exists
        if self.am_invalid() {
            // make a copy of myControlledNodes because it will be modified;
            let nodes = self.myControlledNodes.clone();
            for it in nodes {
                it.m().remove_traffic_light(Ptr::from_mut(self));
            }
            write_warningf!("The traffic light '%' does not control any links; it will not be build.", self.get_id());
            return Ptr::default();
        }
        // compute the time needed to brake
        let mut brakingTime = self.compute_braking_time(oc.get_float("tls.yellow.min-decel").unwrap());
        // perform the computation depending on whether the traffic light
        //  definition was loaded or shall be computed new completely
        if !oc.is_default("tls.yellow.time") {
            brakingTime = oc.get_int("tls.yellow.time").unwrap();
        }
        let ret = self.my_compute(brakingTime);
        ret.m().base2.update_parameters(self.base2.get_parameters_map());
        if ret.get_type() == TrafficLightType::ACTUATED {
            for param in ["jam-threshold", "passing-time", "detector-gap", "detector-length", "inactive-threshold"] {
                let opt = format!("tls.actuated.{}", param);
                if !oc.is_default(&opt) && !ret.base2.has_parameter(param) {
                    ret.m().base2.set_parameter(param, &oc.get_value_string(&opt).unwrap());
                }
            }
            if !oc.is_default("tls.actuated.show-detectors") && !ret.base2.has_parameter("show-detectors") {
                ret.m().base2.set_parameter("show-detectors", &oc.get_value_string("tls.actuated.show-detectors").unwrap());
            }
        }
        ret
    }

    /// C++: `void NBTrafficLightDefinition::addNode(NBNode *)` [virtual]
    /// Adds a node to the traffic light logic
    // void
    // NBTrafficLightDefinition::addNode(NBNode* node) {
    //     if (std::find(myControlledNodes.begin(), myControlledNodes.end(), node) == myControlledNodes.end()) {
    //         myControlledNodes.push_back(node);
    //         std::sort(myControlledNodes.begin(), myControlledNodes.end(), NBNode::nodes_by_id_sorter());
    //     }
    //     node->addTrafficLight(this);
    // }
    #[allow(unused_variables)]
    pub fn add_node(&mut self, node: Ptr<NBNode>) {
        if !self.myControlledNodes.contains(&node) {
            self.myControlledNodes.push(node);
            // determinism: node ids are unique within a net, so this sort order is total
            self.myControlledNodes.sort_by(|a, b| a.get_id().cmp(b.get_id()));
        }
        node.m().add_traffic_light(Ptr::from_mut(self));
    }

    /// C++: `void NBTrafficLightDefinition::removeNode(NBNode *)` [virtual]
    /// Removes the given node from the list of controlled nodes
    // void
    // NBTrafficLightDefinition::removeNode(NBNode* node) {
    //     std::vector<NBNode*>::iterator i = std::find(myControlledNodes.begin(), myControlledNodes.end(), node);
    //     if (i != myControlledNodes.end()) {
    //         myControlledNodes.erase(i);
    //     }
    //     // !!! remove in node?
    // }
    #[allow(unused_variables)]
    pub fn remove_node(&mut self, node: Ptr<NBNode>) {
        if let Some(i) = self.myControlledNodes.iter().position(|n| *n == node) {
            self.myControlledNodes.remove(i);
        }
        // !!! remove in node?
    }

    /// C++: `void NBTrafficLightDefinition::removeConnection(const NBConnection &, bool)` [virtual]
    /// removes the given connection from the traffic light if recontruct=true, reconstructs the logic and informs the edges for immediate use in netedit
    // virtual void removeConnection(const NBConnection& conn, bool reconstruct = true) {
    //         UNUSED_PARAMETER(conn);
    //         UNUSED_PARAMETER(reconstruct);
    //     }
    #[allow(unused_variables)]
    pub fn remove_connection(&mut self, conn: &NBConnection, reconstruct: bool) {
        // C++ body only marks the parameters as unused; the base implementation does nothing
    }

    /// C++: `const std::vector<NBNode *> & NBTrafficLightDefinition::getNodes() const` [const]
    /// Returns the list of controlled nodes
    // const std::vector<NBNode*>& getNodes() const {
    //         return myControlledNodes;
    //     }
    #[allow(unused_variables)]
    pub fn get_nodes(&self) -> &Vec<Ptr<NBNode>> {
        &self.myControlledNodes
    }

    /// C++: `bool NBTrafficLightDefinition::mustBrake(const NBEdge *const, const NBEdge *const) const` [const]
    /// Returns the information whether the described flow must let any other flow pass
    // bool
    // NBTrafficLightDefinition::mustBrake(const NBEdge* const from, const NBEdge* const to) const {
    //     std::vector<NBNode*>::const_iterator i =
    //         find_if(myControlledNodes.begin(), myControlledNodes.end(),
    //                 NBContHelper::node_with_incoming_finder(from));
    //     assert(i != myControlledNodes.end());
    //     NBNode* node = *i;
    //     if (!node->hasOutgoing(to)) {
    //         return true; // !!!
    //     }
    //     // @todo recheck relevance of lane indices
    //     return node->mustBrake(from, to, -1, -1, true);
    // }
    #[allow(unused_variables)]
    pub fn must_brake(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>) -> bool {
        let node = self.myControlledNodes.iter().copied().find(|n| n.has_incoming(from));
        // C++ asserts i != end here
        let node = node.unwrap();
        if !node.has_outgoing(to) {
            return true; // !!!
        }
        // @todo recheck relevance of lane indices
        node.must_brake(from, to, -1, -1, true)
    }

    /// C++: `bool NBTrafficLightDefinition::mustBrake(const NBConnection &, const NBConnection &, bool) const` [const]
    /// Returns the information whether the described flow must let the other flow pass
    // bool
    // NBTrafficLightDefinition::mustBrake(const NBConnection& possProhibited,
    //                                     const NBConnection& possProhibitor,
    //                                     bool regardNonSignalisedLowerPriority) const {
    //     return forbids(possProhibitor.getFrom(), possProhibitor.getTo(),
    //                    possProhibited.getFrom(), possProhibited.getTo(),
    //                    regardNonSignalisedLowerPriority);
    // }
    #[allow(unused_variables)]
    pub fn must_brake_2(&self, possProhibited: &NBConnection, possProhibitor: &NBConnection, regardNonSignalisedLowerPriority: bool) -> bool {
        self.forbids(possProhibitor.get_from(), possProhibitor.get_to(), possProhibited.get_from(), possProhibited.get_to(), regardNonSignalisedLowerPriority, false)
    }

    /// C++: `bool NBTrafficLightDefinition::mustBrake(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const, bool) const` [const]
    /// Returns the information whether the described flow must let any other flow pass
    // bool
    // NBTrafficLightDefinition::mustBrake(const NBEdge* const possProhibitedFrom,
    //                                     const NBEdge* const possProhibitedTo,
    //                                     const NBEdge* const possProhibitorFrom,
    //                                     const NBEdge* const possProhibitorTo,
    //                                     bool regardNonSignalisedLowerPriority) const {
    //     return forbids(possProhibitorFrom, possProhibitorTo,
    //                    possProhibitedFrom, possProhibitedTo,
    //                    regardNonSignalisedLowerPriority);
    // }
    #[allow(unused_variables)]
    pub fn must_brake_3(&self, possProhibitedFrom: Ptr<NBEdge>, possProhibitedTo: Ptr<NBEdge>, possProhibitorFrom: Ptr<NBEdge>, possProhibitorTo: Ptr<NBEdge>, regardNonSignalisedLowerPriority: bool) -> bool {
        self.forbids(possProhibitorFrom, possProhibitorTo, possProhibitedFrom, possProhibitedTo, regardNonSignalisedLowerPriority, false)
    }

    /// C++: `bool NBTrafficLightDefinition::forbids(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const, bool, bool) const` [const]
    /// Returns the information whether "prohibited" flow must let "prohibitor" flow pass
    // bool
    // NBTrafficLightDefinition::forbids(const NBEdge* const possProhibitorFrom,
    //                                   const NBEdge* const possProhibitorTo,
    //                                   const NBEdge* const possProhibitedFrom,
    //                                   const NBEdge* const possProhibitedTo,
    //                                   bool regardNonSignalisedLowerPriority,
    //                                   bool sameNodeOnly) const {
    //     if (possProhibitorFrom == nullptr || possProhibitorTo == nullptr || possProhibitedFrom == nullptr || possProhibitedTo == nullptr) {
    //         return false;
    //     }
    //     // retrieve both nodes
    //     std::vector<NBNode*>::const_iterator incoming =
    //         find_if(myControlledNodes.begin(), myControlledNodes.end(), NBContHelper::node_with_incoming_finder(possProhibitorFrom));
    //     std::vector<NBNode*>::const_iterator outgoing =
    //         find_if(myControlledNodes.begin(), myControlledNodes.end(), NBContHelper::node_with_outgoing_finder(possProhibitedTo));
    //     assert(incoming != myControlledNodes.end());
    //     NBNode* incnode = *incoming;
    //     NBNode* outnode = *outgoing;
    //     EdgeVector::const_iterator i;
    //
    // #ifdef DEBUG_RIGHT_OF_WAY
    //     if (DEBUGCOND) {
    //         std::cout << "forbids tls=" << getID() << " from=" << possProhibitedFrom->getID() << " to=" << possProhibitedTo->getID() << " foeFrom=" << possProhibitorFrom->getID() << " foeTo=" << possProhibitorTo->getID() << " rnslp=" << regardNonSignalisedLowerPriority << " sameNodeOnly=" << sameNodeOnly;
    //     }
    // #endif
    //     if (incnode != outnode) {
    //         if (sameNodeOnly) {
    // #ifdef DEBUG_RIGHT_OF_WAY
    //             if (DEBUGCOND) {
    //                 std::cout << "   differentNodes: allows (no check)\n";
    //             }
    // #endif
    //             return false;
    //         }
    //         // the links are located at different nodes
    //         const EdgeVector& ev1 = possProhibitedTo->getConnectedEdges();
    //         // go through the following edge,
    //         //  check whether one of these connections is prohibited
    //         for (i = ev1.begin(); i != ev1.end(); ++i) {
    //             std::vector<NBNode*>::const_iterator outgoing2 =
    //                 find_if(myControlledNodes.begin(), myControlledNodes.end(), NBContHelper::node_with_outgoing_finder(*i));
    //             if (outgoing2 == myControlledNodes.end()) {
    //                 continue;
    //             }
    //             NBNode* outnode2 = *outgoing2;
    //             if (incnode != outnode2) {
    //                 continue;
    //             }
    //             if (incnode->getDirection(possProhibitedTo, *i) != LinkDirection::STRAIGHT) {
    //                 continue;
    //             }
    //             bool ret1 = incnode->foes(possProhibitorFrom, possProhibitorTo,
    //                                       possProhibitedTo, *i);
    //             bool ret2 = incnode->forbids(possProhibitorFrom, possProhibitorTo,
    //                                          possProhibitedTo, *i,
    //                                          regardNonSignalisedLowerPriority);
    //             bool ret = ret1 || ret2;
    //             if (ret) {
    // #ifdef DEBUG_RIGHT_OF_WAY
    //                 if (DEBUGCOND) {
    //                     std::cout << "   differentNodes: forbids\n";
    //                 }
    // #endif
    //                 return true;
    //             }
    //         }
    //
    //         const EdgeVector& ev2 = possProhibitorTo->getConnectedEdges();
    //         // go through the following edge,
    //         //  check whether one of these connections is prohibited
    //         for (i = ev2.begin(); i != ev2.end(); ++i) {
    //             std::vector<NBNode*>::const_iterator incoming2 =
    //                 find_if(myControlledNodes.begin(), myControlledNodes.end(), NBContHelper::node_with_incoming_finder(possProhibitorTo));
    //             if (incoming2 == myControlledNodes.end()) {
    //                 continue;
    //             }
    //             NBNode* incnode2 = *incoming2;
    //             if (incnode2 != outnode) {
    //                 continue;
    //             }
    //             if (incnode2->getDirection(possProhibitorTo, *i) != LinkDirection::STRAIGHT) {
    //                 continue;
    //             }
    //             bool ret1 = incnode2->foes(possProhibitorTo, *i,
    //                                        possProhibitedFrom, possProhibitedTo);
    //             bool ret2 = incnode2->forbids(possProhibitorTo, *i,
    //                                           possProhibitedFrom, possProhibitedTo,
    //                                           regardNonSignalisedLowerPriority);
    //             bool ret = ret1 || ret2;
    //             if (ret) {
    // #ifdef DEBUG_RIGHT_OF_WAY
    //                 if (DEBUGCOND) {
    //                     std::cout << "   differentNodes: forbids (2)\n";
    //                 }
    // #endif
    //                 return true;
    //             }
    //         }
    // #ifdef DEBUG_RIGHT_OF_WAY
    //         if (DEBUGCOND) {
    //             std::cout << "   differentNodes: allows\n";
    //         }
    // #endif
    //         return false;
    //     }
    //     // both links are located at the same node
    //     //  check using this node's information
    //     const bool result = incnode->forbids(possProhibitorFrom, possProhibitorTo,
    //                                          possProhibitedFrom, possProhibitedTo,
    //                                          regardNonSignalisedLowerPriority);
    // #ifdef DEBUG_RIGHT_OF_WAY
    //     if (DEBUGCOND) {
    //         std::cout << "   sameNodes: " << (result ? "forbids" : "allows") << "\n";
    //     }
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn forbids(&self, possProhibitorFrom: Ptr<NBEdge>, possProhibitorTo: Ptr<NBEdge>, possProhibitedFrom: Ptr<NBEdge>, possProhibitedTo: Ptr<NBEdge>, regardNonSignalisedLowerPriority: bool, sameNodeOnly: bool) -> bool {
        if possProhibitorFrom.is_null() || possProhibitorTo.is_null() || possProhibitedFrom.is_null() || possProhibitedTo.is_null() {
            return false;
        }
        // retrieve both nodes
        let incnode = self.myControlledNodes.iter().copied().find(|n| n.has_incoming(possProhibitorFrom));
        let outnode = self.myControlledNodes.iter().copied().find(|n| n.has_outgoing(possProhibitedTo));
        // C++ asserts incoming != end and dereferences outgoing unchecked; a missing outgoing node is treated as the null node
        let incnode = incnode.unwrap();
        let outnode = outnode.unwrap_or_default();
        if incnode != outnode {
            if sameNodeOnly {
                return false;
            }
            // the links are located at different nodes
            let ev1 = possProhibitedTo.get_connected_edges();
            // go through the following edge,
            //  check whether one of these connections is prohibited
            for &i in ev1.iter() {
                let outnode2 = self.myControlledNodes.iter().copied().find(|n| n.has_outgoing(i));
                let outnode2 = match outnode2 {
                    Some(n) => n,
                    None => continue,
                };
                if incnode != outnode2 {
                    continue;
                }
                if incnode.get_direction(possProhibitedTo, i, false) != LinkDirection::STRAIGHT {
                    continue;
                }
                let ret1 = incnode.foes(possProhibitorFrom, possProhibitorTo, possProhibitedTo, i);
                let ret2 = incnode.forbids(possProhibitorFrom, possProhibitorTo, possProhibitedTo, i, regardNonSignalisedLowerPriority);
                let ret = ret1 || ret2;
                if ret {
                    return true;
                }
            }

            let ev2 = possProhibitorTo.get_connected_edges();
            // go through the following edge,
            //  check whether one of these connections is prohibited
            for &i in ev2.iter() {
                let incnode2 = self.myControlledNodes.iter().copied().find(|n| n.has_incoming(possProhibitorTo));
                let incnode2 = match incnode2 {
                    Some(n) => n,
                    None => continue,
                };
                if incnode2 != outnode {
                    continue;
                }
                if incnode2.get_direction(possProhibitorTo, i, false) != LinkDirection::STRAIGHT {
                    continue;
                }
                let ret1 = incnode2.foes(possProhibitorTo, i, possProhibitedFrom, possProhibitedTo);
                let ret2 = incnode2.forbids(possProhibitorTo, i, possProhibitedFrom, possProhibitedTo, regardNonSignalisedLowerPriority);
                let ret = ret1 || ret2;
                if ret {
                    return true;
                }
            }
            return false;
        }
        // both links are located at the same node
        //  check using this node's information
        incnode.forbids(possProhibitorFrom, possProhibitorTo, possProhibitedFrom, possProhibitedTo, regardNonSignalisedLowerPriority)
    }

    /// C++: `bool NBTrafficLightDefinition::foes(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const) const` [const]
    /// Returns the information whether the given flows cross
    // bool
    // NBTrafficLightDefinition::foes(const NBEdge* const from1, const NBEdge* const to1,
    //                                const NBEdge* const from2, const NBEdge* const to2) const {
    //     if (to1 == nullptr || to2 == nullptr) {
    //         return false;
    //     }
    //     // retrieve both nodes (it is possible that a connection
    //     std::vector<NBNode*>::const_iterator incoming =
    //         find_if(myControlledNodes.begin(), myControlledNodes.end(),
    //                 NBContHelper::node_with_incoming_finder(from1));
    //     std::vector<NBNode*>::const_iterator outgoing =
    //         find_if(myControlledNodes.begin(), myControlledNodes.end(),
    //                 NBContHelper::node_with_outgoing_finder(to1));
    //     assert(incoming != myControlledNodes.end());
    //     NBNode* incnode = *incoming;
    //     NBNode* outnode = *outgoing;
    //     if (incnode != outnode) {
    //         return false;
    //     }
    //     return incnode->foes(from1, to1, from2, to2);
    // }
    #[allow(unused_variables)]
    pub fn foes(&self, from1: Ptr<NBEdge>, to1: Ptr<NBEdge>, from2: Ptr<NBEdge>, to2: Ptr<NBEdge>) -> bool {
        if to1.is_null() || to2.is_null() {
            return false;
        }
        // retrieve both nodes (it is possible that a connection
        let incnode = self.myControlledNodes.iter().copied().find(|n| n.has_incoming(from1));
        let outnode = self.myControlledNodes.iter().copied().find(|n| n.has_outgoing(to1));
        // C++ asserts incoming != end and dereferences outgoing unchecked; a missing outgoing node is treated as the null node
        let incnode = incnode.unwrap();
        let outnode = outnode.unwrap_or_default();
        if incnode != outnode {
            return false;
        }
        incnode.foes(from1, to1, from2, to2)
    }

    /// C++: `void NBTrafficLightDefinition::setTLControllingInformation() const` [const, virtual, pure virtual]
    /// Informs edges about being controlled by a tls
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_tl_controlling_information(&self) {
        (self.vt.set_tl_controlling_information)(self)
    }

    /// C++: `void NBTrafficLightDefinition::setParticipantsInformation()` [virtual]
    /// Builds the list of participating nodes/edges/links
    // void
    // NBTrafficLightDefinition::setParticipantsInformation() {
    //     // collect the information about participating edges and links
    //     collectEdges();
    //     collectLinks();
    // }
    #[allow(unused_variables)]
    pub fn set_participants_information(&mut self) {
        // collect the information about participating edges and links
        self.collect_edges();
        self.collect_links();
    }

    /// C++: `void NBTrafficLightDefinition::addControlledInnerEdges(const std::vector<std::string> &)`
    /// Adds the given ids into the list of inner edges controlled by the tls
    // void
    // NBTrafficLightDefinition::addControlledInnerEdges(const std::vector<std::string>& edges) {
    //     myControlledInnerEdges.insert(edges.begin(), edges.end());
    // }
    #[allow(unused_variables)]
    pub fn add_controlled_inner_edges(&mut self, edges: &Vec<String>) {
        self.myControlledInnerEdges.extend(edges.iter().cloned());
    }

    /// C++: `std::vector<std::string> NBTrafficLightDefinition::getControlledInnerEdges() const` [const]
    /// Retrieve the ids of edges explicitly controlled by the tls
    // std::vector<std::string>
    // NBTrafficLightDefinition::getControlledInnerEdges() const {
    //     return std::vector<std::string>(myControlledInnerEdges.begin(), myControlledInnerEdges.end());
    // }
    #[allow(unused_variables)]
    pub fn get_controlled_inner_edges(&self) -> Vec<String> {
        self.myControlledInnerEdges.iter().cloned().collect()
    }

    /// C++: `void NBTrafficLightDefinition::remapRemoved(NBEdge *, const EdgeVector &, const EdgeVector &)` [virtual, pure virtual]
    /// Replaces occurrences of the removed edge in incoming/outgoing edges of all definitions
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remap_removed(&mut self, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
        (self.vt.remap_removed)(self, removed, incoming, outgoing)
    }

    /// C++: `void NBTrafficLightDefinition::replaceRemoved(NBEdge *, int, NBEdge *, int, bool)` [virtual, pure virtual]
    /// Replaces a removed edge/lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_removed(&mut self, removed: Ptr<NBEdge>, removedLane: i32, by: Ptr<NBEdge>, byLane: i32, incoming: bool) {
        (self.vt.replace_removed)(self, removed, removedLane, by, byLane, incoming)
    }

    /// C++: `void NBTrafficLightDefinition::shiftTLConnectionLaneIndex(NBEdge *, int, int)` [virtual]
    /// patches (loaded) signal plans by modifying lane indices
    // virtual void shiftTLConnectionLaneIndex(NBEdge* edge, int offset, int threshold = -1) {
    //         UNUSED_PARAMETER(edge);
    //         UNUSED_PARAMETER(offset);
    //         UNUSED_PARAMETER(threshold);
    //     }
    #[allow(unused_variables)]
    pub fn shift_tl_connection_lane_index(&mut self, edge: Ptr<NBEdge>, offset: i32, threshold: i32) {
        // C++ base body only marks the parameters as unused; subclasses patch their plans

    }

    /// C++: `const EdgeVector & NBTrafficLightDefinition::getIncomingEdges() const` [const]
    /// Returns the list of incoming edges (must be build first)
    // const EdgeVector&
    // NBTrafficLightDefinition::getIncomingEdges() const {
    //     return myIncomingEdges;
    // }
    #[allow(unused_variables)]
    pub fn get_incoming_edges(&self) -> &EdgeVector {
        &self.myIncomingEdges
    }

    /// C++: `const NBConnectionVector & NBTrafficLightDefinition::getControlledLinks() const` [const]
    /// returns the controlled links (depends on previous call to collectLinks)
    // const NBConnectionVector& getControlledLinks() const {
    //         return myControlledLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_controlled_links(&self) -> &NBConnectionVector {
        &self.myControlledLinks
    }

    /// C++: `NBConnectionVector & NBTrafficLightDefinition::getControlledLinks()`
    /// returns the controlled links (non const version)
    // NBConnectionVector& getControlledLinks() {
    //         return myControlledLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_controlled_links_2(&mut self) -> &mut NBConnectionVector {
        &mut self.myControlledLinks
    }

    /// C++: `const std::string & NBTrafficLightDefinition::getProgramID() const` [const]
    /// Returns the ProgramID
    // const std::string& getProgramID() const {
    //         return mySubID;
    //     }
    #[allow(unused_variables)]
    pub fn get_program_id(&self) -> &str {
        &self.mySubID
    }

    /// C++: `void NBTrafficLightDefinition::setProgramID(const std::string &)` [virtual]
    /// Sets the programID
    // virtual void setProgramID(const std::string& programID) {
    //         mySubID = programID;
    //     }
    #[allow(unused_variables)]
    pub fn set_program_id(&mut self, programID: &str) {
        self.mySubID = programID.to_string();
    }

    /// C++: `TrafficLightLayout NBTrafficLightDefinition::getLayout() const` [const, virtual]
    // virtual TrafficLightLayout getLayout() const {
    //         return TrafficLightLayout::DEFAULT;
    //     }
    #[allow(unused_variables)]
    pub fn get_layout(&self) -> TrafficLightLayout {
        TrafficLightLayout::DEFAULT
    }

    /// C++: `SUMOTime NBTrafficLightDefinition::getOffset()`
    /// Returns the offset
    // SUMOTime getOffset() {
    //         return myOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_offset(&mut self) -> SUMOTime {
        self.myOffset
    }

    /// C++: `TrafficLightType NBTrafficLightDefinition::getType() const` [const]
    /// get the algorithm type (static etc..)
    // TrafficLightType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> TrafficLightType {
        self.myType
    }

    /// C++: `void NBTrafficLightDefinition::setType(TrafficLightType)` [virtual]
    /// set the algorithm type (static etc..)
    // virtual void setType(TrafficLightType type) {
    //         myType = type;
    //     }
    #[allow(unused_variables)]
    pub fn set_type(&mut self, type_: TrafficLightType) {
        self.myType = type_;
    }

    /// C++: `bool NBTrafficLightDefinition::needsCont(const NBEdge *, const NBEdge *, const NBEdge *, const NBEdge *) const` [const]
    // bool
    // NBTrafficLightDefinition::needsCont(const NBEdge* fromE, const NBEdge* toE, const NBEdge* otherFromE, const NBEdge* otherToE) const {
    //     if (!myNeedsContRelationReady) {
    //         initNeedsContRelation();
    //         assert(myNeedsContRelationReady);
    //     }
    //     return std::find(myNeedsContRelation.begin(), myNeedsContRelation.end(),
    //                      StreamPair(fromE, toE, otherFromE, otherToE)) != myNeedsContRelation.end();
    // }
    #[allow(unused_variables)]
    pub fn needs_cont(&self, fromE: Ptr<NBEdge>, toE: Ptr<NBEdge>, otherFromE: Ptr<NBEdge>, otherToE: Ptr<NBEdge>) -> bool {
        if !self.myNeedsContRelationReady {
            self.init_needs_cont_relation();
            // assert(myNeedsContRelationReady)
        }
        self.myNeedsContRelation.contains(&NBTrafficLightDefinition_StreamPair::new(fromE, toE, otherFromE, otherToE))
    }

    /// C++: `bool NBTrafficLightDefinition::extraConflict(int, int) const` [const, virtual]
    /// whether the given index must yield to the foeIndex (i.e. while turning right on a red light)
    // bool
    // NBTrafficLightDefinition::extraConflict(int index, int foeIndex) const {
    //     initExtraConflicts();
    //     return std::find(myExtraConflicts.begin(), myExtraConflicts.end(), std::make_pair(index, foeIndex)) != myExtraConflicts.end();
    // }
    #[allow(unused_variables)]
    pub fn extra_conflict(&self, index: i32, foeIndex: i32) -> bool {
        self.init_extra_conflicts();
        self.myExtraConflicts.contains(&(index, foeIndex))
    }

    /// C++: `void NBTrafficLightDefinition::initNeedsContRelation() const` [const, virtual]
    // void
    // NBTrafficLightDefinition::initNeedsContRelation() const {
    //     if (!amInvalid()) {
    //         NBOwnTLDef dummy(DummyID, myControlledNodes, 0, TrafficLightType::STATIC);
    //         dummy.initNeedsContRelation();
    //         myNeedsContRelation = dummy.myNeedsContRelation;
    //         for (std::vector<NBNode*>::const_iterator i = myControlledNodes.begin(); i != myControlledNodes.end(); i++) {
    //             (*i)->removeTrafficLight(&dummy);
    //         }
    //     }
    //     myNeedsContRelationReady = true;
    // }
    #[allow(unused_variables)]
    pub fn init_needs_cont_relation(&self) {
        if !self.am_invalid() {
            let mut dummy = NBOwnTLDef::new(NBTrafficLightDefinition_DummyID, &self.myControlledNodes, 0, TrafficLightType::STATIC);
            dummy.init_needs_cont_relation();
            // the C++ code assigns through a const method, so the cache is written via Ptr::m
            Ptr::new(self).m().myNeedsContRelation = dummy.myNeedsContRelation.clone();
            for node in self.myControlledNodes.clone() {
                node.m().remove_traffic_light(Ptr::from_mut(&mut dummy.base));
            }
        }
        Ptr::new(self).m().myNeedsContRelationReady = true;
    }

    /// C++: `void NBTrafficLightDefinition::initExtraConflicts() const` [const, virtual]
    // void
    // NBTrafficLightDefinition::initExtraConflicts() const {
    //     if (!myExtraConflictsReady) {
    //         NBOwnTLDef dummy(DummyID, myControlledNodes, 0, TrafficLightType::STATIC);
    //         dummy.setParticipantsInformation();
    //         NBTrafficLightLogic* tllDummy = dummy.computeLogicAndConts(0, true);
    //         delete tllDummy;
    //         myExtraConflicts = dummy.myExtraConflicts;
    //         for (std::vector<NBNode*>::const_iterator i = myControlledNodes.begin(); i != myControlledNodes.end(); i++) {
    //             (*i)->removeTrafficLight(&dummy);
    //         }
    //         myExtraConflictsReady = true;
    //         //std::cout << " extraConflicts tls=" << getID() << " pro=" << getProgramID() << "\n";
    //         //for (auto item : myExtraConflicts) {
    //         //    std::cout << "   " << item.first << ", " << item.second << "\n";
    //         //}
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_extra_conflicts(&self) {
        if !self.myExtraConflictsReady {
            let mut dummy = NBOwnTLDef::new(NBTrafficLightDefinition_DummyID, &self.myControlledNodes, 0, TrafficLightType::STATIC);
            dummy.set_participants_information();
            let tllDummy = dummy.compute_logic_and_conts(0, true);
            // shortcut: C++ deletes tllDummy here; Ptr-owned logics have no ownership convention yet, so it is leaked, upgrade: free it once Ptr ownership is defined
            let _ = tllDummy;
            // the C++ code assigns through a const method, so the cache is written via Ptr::m
            Ptr::new(self).m().myExtraConflicts = dummy.myExtraConflicts.clone();
            for node in self.myControlledNodes.clone() {
                node.m().remove_traffic_light(Ptr::from_mut(&mut dummy.base));
            }
            Ptr::new(self).m().myExtraConflictsReady = true;
        }
    }

    /// C++: `int NBTrafficLightDefinition::getMaxIndex()` [virtual, pure virtual]
    /// Returns the maximum index controlled by this traffic light and assigned to a connection
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_max_index(&mut self) -> i32 {
        (self.vt.get_max_index)(self)
    }

    /// C++: `int NBTrafficLightDefinition::getMaxValidIndex()` [virtual]
    /// Returns the maximum index controlled by this traffic light
    // virtual int getMaxValidIndex() {
    //         return getMaxIndex();
    //     }
    #[allow(unused_variables)]
    pub fn get_max_valid_index(&mut self) -> i32 {
        self.get_max_index()
    }

    /// C++: `int NBTrafficLightDefinition::computeBrakingTime(double) const` [const]
    /// Computes the time vehicles may need to brake
    // int
    // NBTrafficLightDefinition::computeBrakingTime(double minDecel) const {
    //     if (myIncomingEdges.empty()) {
    //         // don't crash
    //         return MIN_YELLOW_SECONDS;
    //     }
    //     const double vmax = NBContHelper::maxSpeed(myIncomingEdges);
    //     if (vmax < 71 / 3.6) {
    //         // up to 50kmh: 3 seconds , 60km/h: 4, 70kmh: 5
    //         // @note: these are German regulations, other countries may differ
    //         return MIN_YELLOW_SECONDS + (int)MAX2(0.0, (floor((vmax - 50 / 3.6) * 0.37)));
    //     } else {
    //         // above 70km/h we use a function that grows according to the "natural"
    //         // formula (vmax / 2 * minDecel) but continues smoothly where the german
    //         // rules leave of
    //         return (int)(1.8 + vmax / 2 / minDecel);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_braking_time(&self, minDecel: f64) -> i32 {
        if self.myIncomingEdges.is_empty() {
            // don't crash
            return NBTrafficLightDefinition_MIN_YELLOW_SECONDS;
        }
        let vmax = NBContHelper::max_speed(&self.myIncomingEdges);
        if vmax < 71.0 / 3.6 {
            // up to 50kmh: 3 seconds , 60km/h: 4, 70kmh: 5
            // @note: these are German regulations, other countries may differ
            return NBTrafficLightDefinition_MIN_YELLOW_SECONDS + (f64::max(0.0, ((vmax - 50.0 / 3.6) * 0.37).floor()) as i32);
        } else {
            // above 70km/h we use a function that grows according to the "natural"
            // formula (vmax / 2 * minDecel) but continues smoothly where the german
            // rules leave of
            return (1.8 + vmax / 2.0 / minDecel) as i32;
        }
    }

    /// C++: `bool NBTrafficLightDefinition::usingSignalGroups() const` [const, virtual]
    /// whether this definition uses signal group (multiple connections with the same link index)
    // virtual bool usingSignalGroups() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn using_signal_groups(&self) -> bool {
        false
    }

    /// C++: `std::string NBTrafficLightDefinition::getDescription() const` [const]
    /// get ID and programID together (for convenient debugging)
    // std::string
    // NBTrafficLightDefinition::getDescription() const {
    //     return getID() + ':' + getProgramID() + '@' + toString(this);
    // }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> String {
        format!("{}:{}@{:p}", self.get_id(), self.get_program_id(), self as *const Self)
    }

    /// C++: `void NBTrafficLightDefinition::finalChecks() const` [const, virtual]
    /// perform optional final checks
    // virtual void finalChecks() const {}
    #[allow(unused_variables)]
    pub fn final_checks(&self) {
        // virtual with an empty base body

    }

    /// C++: `NBTrafficLightLogic * NBTrafficLightDefinition::myCompute(int)` [virtual, pure virtual, protected]
    /// Computes the traffic light logic finally in dependence to the type
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn my_compute(&mut self, brakingTime: i32) -> Ptr<NBTrafficLightLogic> {
        (self.vt.my_compute)(self, brakingTime)
    }

    /// C++: `void NBTrafficLightDefinition::collectLinks()` [virtual, pure virtual, protected]
    /// Collects the links participating in this traffic light
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn collect_links(&mut self) {
        (self.vt.collect_links)(self)
    }

    /// C++: `void NBTrafficLightDefinition::collectEdges()` [virtual, protected]
    /// Build the list of participating edges
    // void
    // NBTrafficLightDefinition::collectEdges() {
    //     myIncomingEdges.clear();
    //     myEdgesWithin.clear();
    //     EdgeVector myOutgoing;
    //     // collect the edges from the participating nodes
    //     for (std::vector<NBNode*>::iterator i = myControlledNodes.begin(); i != myControlledNodes.end(); i++) {
    //         const EdgeVector& incoming = (*i)->getIncomingEdges();
    //         copy(incoming.begin(), incoming.end(), back_inserter(myIncomingEdges));
    //         const EdgeVector& outgoing = (*i)->getOutgoingEdges();
    //         copy(outgoing.begin(), outgoing.end(), back_inserter(myOutgoing));
    //     }
    //     EdgeVector outer;
    //     // check which of the edges are completely within the junction
    //     //  add them to the list of edges lying within the node
    //     for (NBEdge* edge : myIncomingEdges) {
    //         edge->setInsideTLS(false); // reset
    //         // an edge lies within the logic if it is outgoing as well as incoming
    //         EdgeVector::iterator k = std::find(myOutgoing.begin(), myOutgoing.end(), edge);
    //         if (k != myOutgoing.end()) {
    //             myEdgesWithin.push_back(edge);
    //         } else  {
    //             outer.push_back(edge);
    //         }
    //     }
    //     // collect edges that are reachable from the outside via controlled connections
    //     std::set<NBEdge*> reachable = collectReachable(outer, myEdgesWithin, true);
    //     // collect edges that are reachable from the outside regardless of controllability
    //     std::set<NBEdge*> reachable2 = collectReachable(outer, myEdgesWithin, false);
    //
    //     const bool uncontrolledWithin = OptionsCont::getOptions().getBool("tls.uncontrolled-within");
    //     for (EdgeVector::iterator j = myEdgesWithin.begin(); j != myEdgesWithin.end(); ++j) {
    //         NBEdge* edge = *j;
    //         // edges that are marked as 'inner' will not get their own phase when
    //         // computing traffic light logics (unless they cannot be reached from the outside at all)
    //         if (reachable.count(edge) == 1) {
    //             edge->setInsideTLS(true);
    //             // legacy behavior
    //             if (uncontrolledWithin && myControlledInnerEdges.count(edge->getID()) == 0) {
    //                 myIncomingEdges.erase(find(myIncomingEdges.begin(), myIncomingEdges.end(), edge));
    //             }
    //         }
    //         if (reachable2.count(edge) == 0 && edge->getFirstNonPedestrianLaneIndex(NBNode::FORWARD, true) >= 0
    //                 && getID() != DummyID) {
    //             WRITE_WARNINGF(TL("Unreachable edge '%' within tlLogic '%'"), edge->getID(), getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_edges(&mut self) {
        self.myIncomingEdges.clear();
        self.myEdgesWithin.clear();
        let mut myOutgoing: EdgeVector = Vec::new();
        // collect the edges from the participating nodes
        for i in self.myControlledNodes.clone() {
            let incoming = i.get_incoming_edges().clone();
            self.myIncomingEdges.extend(incoming);
            let outgoing = i.get_outgoing_edges().clone();
            myOutgoing.extend(outgoing);
        }
        let mut outer: EdgeVector = Vec::new();
        // check which of the edges are completely within the junction
        //  add them to the list of edges lying within the node
        for edge in self.myIncomingEdges.clone() {
            edge.m().set_inside_tls(false); // reset
            // an edge lies within the logic if it is outgoing as well as incoming
            if myOutgoing.contains(&edge) {
                self.myEdgesWithin.push(edge);
            } else {
                outer.push(edge);
            }
        }
        // collect edges that are reachable from the outside via controlled connections
        let reachable = Self::collect_reachable(outer.clone(), &self.myEdgesWithin, true);
        // collect edges that are reachable from the outside regardless of controllability
        let reachable2 = Self::collect_reachable(outer, &self.myEdgesWithin, false);

        let uncontrolledWithin = utils::options::options_cont::get_bool("tls.uncontrolled-within");
        for edge in self.myEdgesWithin.clone() {
            // edges that are marked as 'inner' will not get their own phase when
            // computing traffic light logics (unless they cannot be reached from the outside at all)
            if reachable.contains(&edge) {
                edge.m().set_inside_tls(true);
                // legacy behavior
                if uncontrolledWithin && !self.myControlledInnerEdges.contains(edge.get_id()) {
                    let pos = self.myIncomingEdges.iter().position(|e| *e == edge).unwrap();
                    self.myIncomingEdges.remove(pos);
                }
            }
            // FORWARD is NBNode::FORWARD (1)
            if !reachable2.contains(&edge) && edge.get_first_non_pedestrian_lane_index(1, true) >= 0
                && self.get_id() != NBTrafficLightDefinition_DummyID {
                write_warningf!("Unreachable edge '%' within tlLogic '%'", edge.get_id(), self.get_id());
            }
        }
    }

    /// C++: `bool NBTrafficLightDefinition::amInvalid() const` [const, virtual, protected]
    // bool
    // NBTrafficLightDefinition::amInvalid() const {
    //     return myControlledLinks.size() == 0;
    // }
    #[allow(unused_variables)]
    pub fn am_invalid(&self) -> bool {
        self.myControlledLinks.len() == 0
    }

    /// C++: `void NBTrafficLightDefinition::collectAllLinks(NBConnectionVector &)` [protected]
    /// helper method for use in NBOwnTLDef and NBLoadedSUMOTLDef
    // void
    // NBTrafficLightDefinition::collectAllLinks(NBConnectionVector& into) {
    //     int tlIndex = 0;
    //     // build the list of links which are controlled by the traffic light
    //     std::vector<int> indirectLeft;
    //     for (EdgeVector::iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //         NBEdge* incoming = *i;
    //         int noLanes = incoming->getNumLanes();
    //         for (int j = 0; j < noLanes; j++) {
    //             std::vector<NBEdge::Connection> connected = incoming->getConnectionsFromLane(j);
    //             for (std::vector<NBEdge::Connection>::iterator k = connected.begin(); k != connected.end(); k++) {
    //                 const NBEdge::Connection& el = *k;
    //                 if (incoming->mayBeTLSControlled(el.fromLane, el.toEdge, el.toLane)) {
    //                     if (el.toEdge != nullptr && el.toLane >= (int) el.toEdge->getNumLanes()) {
    //                         throw ProcessError("Connection '" + incoming->getID() + "_" + toString(j) + "->" + el.toEdge->getID() + "_" + toString(el.toLane) + "' yields in a not existing lane.");
    //                     }
    //                     if (incoming->getToNode()->getType() == SumoXMLNodeType::RAIL_CROSSING
    //                             && isRailway(incoming->getPermissions())) {
    //                         // railways stay uncontrolled at rail crossing but they
    //                         // must be registered in MSRailCrossing
    //                         into.push_back(NBConnection(incoming, el.fromLane, el.toEdge, el.toLane, -1));
    //                     } else if (incoming->getToNode()->getType() == SumoXMLNodeType::RAIL_SIGNAL
    //                                && incoming->getToNode()->getDirection(incoming, el.toEdge) == LinkDirection::TURN
    //                                // assume explicit connections at sharp turn-arounds are either for reversal or due to a geometry glitch
    //                                // (the might also be due to faulty connection
    //                                // input but they would not come from guessing)
    //                                && (incoming->getBidiEdge() == el.toEdge)
    //                               ) {
    //                         // turnarounds stay uncontrolled at rail signal
    //                     } else if (incoming->getToNode()->getType() == SumoXMLNodeType::RAIL_SIGNAL && railSignalUncontrolled(incoming, el.toEdge)) {
    //                         // rail signals may stay uncontrolled in a particular direction
    //                     } else {
    //                         into.push_back(NBConnection(incoming, el.fromLane, el.toEdge, el.toLane, tlIndex++));
    //                         if (el.indirectLeft) {
    //                             indirectLeft.push_back((int)into.size() - 1);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     if (indirectLeft.size() > 0) {
    //         // assign linkIndex2 to indirect left turns
    //         for (int i : indirectLeft) {
    //             NBConnection& c = into[i];
    //             // find straight connection with the same toEdge
    //             for (const NBConnection& c2 : into) {
    //                 if (c2.getTo() == c.getTo() && c2.getFrom() != c.getFrom()) {
    //                     LinkDirection dir =  c.getFrom()->getToNode()->getDirection(c2.getFrom(), c2.getTo());
    //                     if (dir == LinkDirection::STRAIGHT) {
    //                         c.setTLIndex2(c2.getTLIndex());
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     if (into.size() > 0 && tlIndex == 0) {
    //         WRITE_WARNINGF(TL("The rail crossing '%' does not have any roads."), getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_all_links(&mut self, into: &mut NBConnectionVector) {
        let mut tlIndex: i32 = 0;
        // build the list of links which are controlled by the traffic light
        let mut indirectLeft: Vec<i32> = Vec::new();
        for incoming in self.myIncomingEdges.iter() {
            let incoming = *incoming;
            let noLanes = incoming.get_num_lanes();
            for j in 0..noLanes {
                let connected = incoming.get_connections_from_lane(j, Ptr::null(), -1);
                for el in connected.iter() {
                    if incoming.may_be_tls_controlled(el.fromLane, el.toEdge, el.toLane) {
                        if !el.toEdge.is_null() && el.toLane >= el.toEdge.get_num_lanes() {
                            std::panic::panic_any(utils::common::util_exceptions::ProcessError::new(format!(
                                "Connection '{}_{}->{}_{}' yields in a not existing lane.",
                                incoming.get_id(),
                                j,
                                el.toEdge.get_id(),
                                el.toLane
                            )));
                        }
                        if incoming.get_to_node().get_type() == SumoXMLNodeType::RAIL_CROSSING && utils::common::sumo_vehicle_class::is_railway(incoming.get_permissions(-1)) {
                            // railways stay uncontrolled at rail crossing but they
                            // must be registered in MSRailCrossing
                            into.push(NBConnection::new_2(incoming, el.fromLane, el.toEdge, el.toLane, -1, NBCONNECTION_INVALID_TL_INDEX));
                        } else if incoming.get_to_node().get_type() == SumoXMLNodeType::RAIL_SIGNAL
                            && incoming.get_to_node().get_direction(incoming, el.toEdge, false) == LinkDirection::TURN
                            // assume explicit connections at sharp turn-arounds are either for reversal or due to a geometry glitch
                            // (the might also be due to faulty connection
                            // input but they would not come from guessing)
                            && (incoming.get_bidi_edge() == el.toEdge)
                        {
                            // turnarounds stay uncontrolled at rail signal
                        } else if incoming.get_to_node().get_type() == SumoXMLNodeType::RAIL_SIGNAL && Self::rail_signal_uncontrolled(incoming, el.toEdge) {
                            // rail signals may stay uncontrolled in a particular direction
                        } else {
                            into.push(NBConnection::new_2(incoming, el.fromLane, el.toEdge, el.toLane, tlIndex, NBCONNECTION_INVALID_TL_INDEX));
                            tlIndex += 1;
                            if el.indirectLeft {
                                indirectLeft.push(into.len() as i32 - 1);
                            }
                        }
                    }
                }
            }
        }
        if indirectLeft.len() > 0 {
            // assign linkIndex2 to indirect left turns
            // determinism: C++ edits `into` in place while scanning it; the scan only reads getTo/getFrom/getTLIndex, which setTLIndex2 never changes, so a snapshot gives the same result
            let snapshot = into.clone();
            for i in indirectLeft.iter() {
                let i = *i as usize;
                let c = &snapshot[i];
                // find straight connection with the same toEdge
                for c2 in snapshot.iter() {
                    if c2.get_to() == c.get_to() && c2.get_from() != c.get_from() {
                        let dir = c.get_from().get_to_node().get_direction(c2.get_from(), c2.get_to(), false);
                        if dir == LinkDirection::STRAIGHT {
                            into[i].set_tl_index2(c2.get_tl_index());
                            break;
                        }
                    }
                }
            }
        }

        if into.len() > 0 && tlIndex == 0 {
            utils::write_warningf!("The rail crossing '%' does not have any roads.", self.base.get_id());
        }
    }

    /// C++: `std::set<NBEdge *> NBTrafficLightDefinition::collectReachable(EdgeVector, const EdgeVector &, bool)` [static, private]
    // std::set<NBEdge*>
    // NBTrafficLightDefinition::collectReachable(EdgeVector outer, const EdgeVector& within, bool checkControlled) {
    //     std::set<NBEdge*> reachable;
    //     while (outer.size() > 0) {
    //         NBEdge* from = outer.back();
    //         outer.pop_back();
    //         std::vector<NBEdge::Connection>& cons = from->getConnections();
    //         for (std::vector<NBEdge::Connection>::iterator k = cons.begin(); k != cons.end(); k++) {
    //             NBEdge* to = (*k).toEdge;
    //             if (reachable.count(to) == 0 &&
    //                     (find(within.begin(), within.end(), to) != within.end()) &&
    //                     (!checkControlled || from->mayBeTLSControlled((*k).fromLane, to, (*k).toLane))) {
    //                 reachable.insert(to);
    //                 outer.push_back(to);
    //             }
    //         }
    //     }
    //     return reachable;
    // }
    #[allow(unused_variables)]
    pub fn collect_reachable(outer: EdgeVector, within: &EdgeVector, checkControlled: bool) -> BTreeSet<Ptr<NBEdge>> {
        let mut outer = outer;
        // determinism: std::set<NBEdge*> is ordered by address in C++ too, so Ptr's address ordering matches
        let mut reachable: BTreeSet<Ptr<NBEdge>> = BTreeSet::new();
        while outer.len() > 0 {
            let from = outer.pop().unwrap();
            let cons = from.get_connections();
            for k in cons.iter() {
                let to = k.toEdge;
                if !reachable.contains(&to) && within.contains(&to) && (!checkControlled || from.may_be_tls_controlled(k.fromLane, to, k.toLane)) {
                    reachable.insert(to);
                    outer.push(to);
                }
            }
        }
        reachable
    }

    /// C++: `bool NBTrafficLightDefinition::railSignalUncontrolled(const NBEdge *, const NBEdge *)` [static, private]
    // bool
    // NBTrafficLightDefinition::railSignalUncontrolled(const NBEdge* in, const NBEdge* out) {
    //     const NBNode* n = in->getToNode();
    //     if (n->hasParameter(OSM_SIGNAL_DIRECTION) && in->hasParameter(OSM_DIRECTION) && out->hasParameter(OSM_DIRECTION)) {
    //         if (in->getParameter(OSM_DIRECTION) == out->getParameter(OSM_DIRECTION)) {
    //             if (n->getParameter(OSM_SIGNAL_DIRECTION) != in->getParameter(OSM_DIRECTION)) {
    //                 return true;
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Could not interpret rail signal direction at junction '%' due to inconsistent directions of edge '%' (%) and edge '%' (%)"),
    //                            n->getID(),
    //                            in->getID(), in->getParameter(OSM_DIRECTION),
    //                            out->getID(), out->getParameter(OSM_DIRECTION));
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn rail_signal_uncontrolled(in_: Ptr<NBEdge>, out: Ptr<NBEdge>) -> bool {
        let n = in_.get_to_node();
        if n.base2.has_parameter(NBTrafficLightDefinition_OSM_SIGNAL_DIRECTION)
            && in_.base2.has_parameter(NBTrafficLightDefinition_OSM_DIRECTION)
            && out.base2.has_parameter(NBTrafficLightDefinition_OSM_DIRECTION)
        {
            if in_.base2.get_parameter(NBTrafficLightDefinition_OSM_DIRECTION, "") == out.base2.get_parameter(NBTrafficLightDefinition_OSM_DIRECTION, "") {
                if n.base2.get_parameter(NBTrafficLightDefinition_OSM_SIGNAL_DIRECTION, "") != in_.base2.get_parameter(NBTrafficLightDefinition_OSM_DIRECTION, "") {
                    return true;
                }
            } else {
                write_warningf!("Could not interpret rail signal direction at junction '%' due to inconsistent directions of edge '%' (%) and edge '%' (%)",
                    n.get_id(),
                    in_.get_id(), in_.base2.get_parameter(NBTrafficLightDefinition_OSM_DIRECTION, ""),
                    out.get_id(), out.base2.get_parameter(NBTrafficLightDefinition_OSM_DIRECTION, ""));
            }
        }
        false
    }

}

/// data structure for caching needsCont information
/// C++ class `NBTrafficLightDefinition::StreamPair` (netbuild/NBTrafficLightDefinition.h)
#[derive(Clone, Copy)]
pub struct NBTrafficLightDefinition_StreamPair {
    pub from1: Ptr<NBEdge>,
    pub to1: Ptr<NBEdge>,
    pub from2: Ptr<NBEdge>,
    pub to2: Ptr<NBEdge>,
}

impl NBTrafficLightDefinition_StreamPair {
    /// C++: `void StreamPair::StreamPair(const NBEdge *, const NBEdge *, const NBEdge *, const NBEdge *)`
    // StreamPair(const NBEdge* _from1, const NBEdge* _to1, const NBEdge* _from2, const NBEdge* _to2):
    //             from1(_from1),
    //             to1(_to1),
    //             from2(_from2),
    //             to2(_to2) {}
    #[allow(unused_variables)]
    pub fn new(_from1: Ptr<NBEdge>, _to1: Ptr<NBEdge>, _from2: Ptr<NBEdge>, _to2: Ptr<NBEdge>) -> Self {
        Self { from1: _from1, to1: _to1, from2: _from2, to2: _to2 }
    }

    /// C++: `bool StreamPair::operator==(const StreamPair &) const` [const]
    // bool operator==(const StreamPair& o) const {
    //             return (from1 == o.from1 && to1 == o.to1
    //                     && from2 == o.from2 && to2 == o.to2);
    //         }
    #[allow(unused_variables)]
    pub fn eq_op(&self, o: &NBTrafficLightDefinition_StreamPair) -> bool {
        self.from1 == o.from1 && self.to1 == o.to1 && self.from2 == o.from2 && self.to2 == o.to2
    }

    /// C++: `bool StreamPair::operator<(const StreamPair &) const` [const]
    // bool operator<(const StreamPair& o) const {
    //             if (from1 != o.from1) {
    //                 return from1 < o.from1;
    //             }
    //             if (to1 != o.to1) {
    //                 return to1 < o.to1;
    //             }
    //             if (from2 != o.from2) {
    //                 return from2 < o.from2;
    //             }
    //             return to2 < o.to2;
    //         }
    #[allow(unused_variables)]
    pub fn lt_op(&self, o: &NBTrafficLightDefinition_StreamPair) -> bool {
        if self.from1 != o.from1 {
            return self.from1 < o.from1;
        }
        if self.to1 != o.to1 {
            return self.to1 < o.to1;
        }
        if self.from2 != o.from2 {
            return self.from2 < o.from2;
        }
        self.to2 < o.to2
    }

}

// std::set<StreamPair> needs total ordering; both are defined via the C++ operators above
impl PartialEq for NBTrafficLightDefinition_StreamPair {
    fn eq(&self, o: &Self) -> bool { self.eq_op(o) }
}
impl Eq for NBTrafficLightDefinition_StreamPair {}
impl PartialOrd for NBTrafficLightDefinition_StreamPair {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(o)) }
}
impl Ord for NBTrafficLightDefinition_StreamPair {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        if self.lt_op(o) {
            std::cmp::Ordering::Less
        } else if o.lt_op(self) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

