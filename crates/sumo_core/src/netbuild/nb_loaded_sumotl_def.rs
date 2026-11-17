//! Port of netbuild/NBLoadedSUMOTLDef.{h,cpp} â€” GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

const NBCONNECTION_INVALID_TL_INDEX: i32 = -1;

/// C++ `delete myTLLogic`: frees a logic that was allocated with `Box::into_raw`.
fn delete_tl_logic(logic: Ptr<NBTrafficLightLogic>) {
    if !logic.is_null() {
        // SAFETY: every logic owned by a definition is created with Box::into_raw (see `new`)
        drop(unsafe { Box::from_raw(logic.as_raw()) });
    }
}

// C++ virtual overrides of NBTrafficLightDefinition, dispatched through NBTrafficLightDefinition::vt
fn sumo_tl_def_of(d: &NBTrafficLightDefinition) -> &NBLoadedSUMOTLDef {
    Ptr::new(d).cast::<NBLoadedSUMOTLDef>().get().unwrap()
}
fn sumo_tl_def_of_mut(d: &mut NBTrafficLightDefinition) -> &mut NBLoadedSUMOTLDef {
    Ptr::from_mut(d).cast::<NBLoadedSUMOTLDef>().get_mut().unwrap()
}
fn sumo_tl_def_collect_links(d: &mut NBTrafficLightDefinition) {
    sumo_tl_def_of_mut(d).collect_links()
}
fn sumo_tl_def_set_tl_controlling_information(d: &NBTrafficLightDefinition) {
    sumo_tl_def_of(d).set_tl_controlling_information()
}
fn sumo_tl_def_remap_removed(d: &mut NBTrafficLightDefinition, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
    sumo_tl_def_of_mut(d).remap_removed(removed, incoming, outgoing)
}
fn sumo_tl_def_replace_removed(d: &mut NBTrafficLightDefinition, removed: Ptr<NBEdge>, removedLane: i32, by: Ptr<NBEdge>, byLane: i32, incoming: bool) {
    sumo_tl_def_of_mut(d).replace_removed(removed, removedLane, by, byLane, incoming)
}
fn sumo_tl_def_my_compute(d: &mut NBTrafficLightDefinition, brakingTime: i32) -> Ptr<NBTrafficLightLogic> {
    sumo_tl_def_of_mut(d).my_compute(brakingTime)
}
fn sumo_tl_def_get_max_index(d: &mut NBTrafficLightDefinition) -> i32 {
    sumo_tl_def_of_mut(d).get_max_index()
}
fn sumo_tl_def_set_program_id(d: &mut NBTrafficLightDefinition, programID: &str) {
    sumo_tl_def_of_mut(d).set_program_id(programID)
}
fn sumo_tl_def_shift_tl_connection_lane_index(d: &mut NBTrafficLightDefinition, edge: Ptr<NBEdge>, offset: i32, threshold: i32) {
    sumo_tl_def_of_mut(d).shift_tl_connection_lane_index(edge, offset, threshold)
}
fn sumo_tl_def_remove_connection(d: &mut NBTrafficLightDefinition, conn: &NBConnection, reconstruct: bool) {
    sumo_tl_def_of_mut(d).remove_connection(conn, reconstruct)
}
fn sumo_tl_def_set_type(d: &mut NBTrafficLightDefinition, type_: TrafficLightType) {
    sumo_tl_def_of_mut(d).set_type(type_)
}
fn sumo_tl_def_extra_conflict(d: &NBTrafficLightDefinition, index: i32, foeIndex: i32) -> bool {
    sumo_tl_def_of(d).extra_conflict(index, foeIndex)
}
fn sumo_tl_def_using_signal_groups(d: &NBTrafficLightDefinition) -> bool {
    sumo_tl_def_of(d).using_signal_groups()
}
fn sumo_tl_def_final_checks(d: &NBTrafficLightDefinition) {
    sumo_tl_def_of(d).final_checks()
}
fn sumo_tl_def_collect_edges(d: &mut NBTrafficLightDefinition) {
    sumo_tl_def_of_mut(d).collect_edges()
}
fn sumo_tl_def_am_invalid(d: &NBTrafficLightDefinition) -> bool {
    sumo_tl_def_of(d).am_invalid()
}
fn sumo_tl_def_init_needs_cont_relation(d: &NBTrafficLightDefinition) {
    sumo_tl_def_of(d).init_needs_cont_relation()
}
fn sumo_tl_def_get_max_valid_index(d: &mut NBTrafficLightDefinition) -> i32 {
    sumo_tl_def_of_mut(d).get_max_valid_index()
}
pub static NB_LOADED_SUMOTL_DEF_VT: NBTrafficLightDefinitionVt = NBTrafficLightDefinitionVt {
    collect_links: sumo_tl_def_collect_links,
    set_tl_controlling_information: sumo_tl_def_set_tl_controlling_information,
    remap_removed: sumo_tl_def_remap_removed,
    replace_removed: sumo_tl_def_replace_removed,
    my_compute: sumo_tl_def_my_compute,
    get_max_index: sumo_tl_def_get_max_index,
    set_program_id: sumo_tl_def_set_program_id,
    shift_tl_connection_lane_index: sumo_tl_def_shift_tl_connection_lane_index,
    remove_connection: sumo_tl_def_remove_connection,
    set_type: sumo_tl_def_set_type,
    extra_conflict: sumo_tl_def_extra_conflict,
    using_signal_groups: sumo_tl_def_using_signal_groups,
    final_checks: sumo_tl_def_final_checks,
    collect_edges: sumo_tl_def_collect_edges,
    am_invalid: sumo_tl_def_am_invalid,
    init_needs_cont_relation: sumo_tl_def_init_needs_cont_relation,
    get_max_valid_index: sumo_tl_def_get_max_valid_index,
    ..DEFAULT_NB_TRAFFIC_LIGHT_DEFINITION_VT
};

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
        let myTLLogic = Ptr::from_raw(Box::into_raw(Box::new(NBTrafficLightLogic::new(id, programID, 0, offset, type_))));
        let mut d = NBLoadedSUMOTLDef {
            base: NBTrafficLightDefinition::new_3(id, programID, offset, type_),
            myTLLogic,
            myShifted: BTreeSet::new(),
            myReconstructAddedConnections: false,
            myReconstructRemovedConnections: false,
            myPhasesLoaded: false,
        };
        d.base.vt = &NB_LOADED_SUMOTL_DEF_VT;
        d
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
        assert!(def.get_type() == logic.get_type());
        let mut ret = NBLoadedSUMOTLDef {
            base: NBTrafficLightDefinition::new_3(def.get_id(), logic.get_program_id(), logic.get_offset(), def.get_type()),
            myTLLogic: Ptr::from_raw(Box::into_raw(Box::new(NBTrafficLightLogic::new_2(Ptr::new(logic))))),
            myShifted: BTreeSet::new(),
            myReconstructAddedConnections: false,
            myReconstructRemovedConnections: false,
            myPhasesLoaded: false,
        };
        ret.base.vt = &NB_LOADED_SUMOTL_DEF_VT;
        ret.base.myControlledLinks = def.get_controlled_links().iter().map(NBConnection::new_4).collect();
        ret.base.myControlledNodes = def.get_nodes().clone();
        ret.base.base2.update_parameters(def.base2.get_parameters_map());
        // shortcut: C++ dynamic_cast<const NBLoadedSUMOTLDef*>(&def) has no runtime type check in this port, so the reconstruct flags of a source NBLoadedSUMOTLDef are not copied, upgrade: add a kind tag to NBTrafficLightDefinition
        ret
    }

    /// C++: `void NBLoadedSUMOTLDef::~NBLoadedSUMOTLDef() noexcept` [virtual]
    /// Destructor
    // NBLoadedSUMOTLDef::~NBLoadedSUMOTLDef() {
    //     delete myTLLogic;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        delete_tl_logic(self.myTLLogic);
        self.myTLLogic = Ptr::null();
        self.base.drop_();
    }

    /// C++: `void NBLoadedSUMOTLDef::setID(const std::string &)` [virtual]
    // void
    // NBLoadedSUMOTLDef::setID(const std::string& newID) {
    //     Named::setID(newID);
    //     myTLLogic->setID(newID);
    // }
    #[allow(unused_variables)]
    pub fn set_id(&mut self, newID: &str) {
        self.base.base.set_id(newID);
        self.myTLLogic.m().set_id(newID);
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
        self.base.set_program_id_base(programID);
        self.myTLLogic.m().set_program_id(programID);
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
        if self.myReconstructAddedConnections {
            let mut dummy = NBOwnTLDef::new(NBTrafficLightDefinition_DummyID, &self.base.myControlledNodes, 0, self.base.get_type());
            dummy.set_participants_information();
            dummy.set_tl_controlling_information();
            for n in self.base.myControlledNodes.iter() {
                n.m().remove_traffic_light(Ptr::from_mut(&mut dummy.base));
            }
        }
        if self.myReconstructRemovedConnections {
            return; // will be called again in reconstructLogic()
        }
        // if nodes have been removed our links may have been invalidated as well
        // since no logic will be built anyway there is no reason to inform any edges
        if self.am_invalid() {
            return;
        }
        // set the information about the link's positions within the tl into the
        //  edges the links are starting at, respectively
        for c in self.base.myControlledLinks.iter() {
            if c.get_tl_index() >= self.myTLLogic.m().get_num_links() {
                std::panic::panic_any(ProcessError::new(format!(
                    "Invalid linkIndex {} for traffic light '{}' with {} links.",
                    c.get_tl_index(),
                    self.get_id(),
                    self.myTLLogic.m().get_num_links()
                )));
            }
            let edge = c.get_from();
            if !edge.is_null() && edge.get_num_lanes() > c.get_from_lane() {
                // logic may have yet to be reconstructed
                edge.m().set_controlling_tl_information(c, self.get_id());
            }
        }
    }

    /// C++: `void NBLoadedSUMOTLDef::remapRemoved(NBEdge *, const EdgeVector &, const EdgeVector &)` [virtual]
    /// Replaces occurrences of the removed edge in incoming/outgoing edges of all definitions
    // void
    // NBLoadedSUMOTLDef::remapRemoved(NBEdge*, const EdgeVector&, const EdgeVector&) {}
    #[allow(unused_variables)]
    pub fn remap_removed(&mut self, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
        // C++ body is empty
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
        if by.is_null() {
            self.myReconstructRemovedConnections = true;
        }
        for it in self.base.myControlledLinks.iter_mut() {
            if incoming {
                it.replace_from_2(removed, removedLane, by, byLane);
            } else {
                it.replace_to_2(removed, removedLane, by, byLane);
            }
        }
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
        // avoid shifting twice if the edge is incoming and outgoing to a joined TLS
        if !self.myShifted.contains(&edge) {
            // XXX what if an edge should really be shifted twice?
            self.myShifted.insert(edge);
            for it in self.base.myControlledLinks.iter_mut() {
                it.shift_lane_index(edge, offset, threshold);
            }
        }
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
        self.myTLLogic.m().add_step_2(duration, state, minDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, name, next, -1);
    }

    /// C++: `void NBLoadedSUMOTLDef::phasesLoaded()`
    /// mark phases as load
    // void phasesLoaded() {
    //         myPhasesLoaded = true;
    //     }
    #[allow(unused_variables)]
    pub fn phases_loaded(&mut self) {
        self.myPhasesLoaded = true;
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
        assert!(self.myTLLogic.m().get_num_links() > 0); // logic should be loaded by now
        if linkIndex >= self.myTLLogic.m().get_num_links() {
            std::panic::panic_any(ProcessError::new(format!(
                "Invalid linkIndex {} in connection from edge '{}' to edge '{}' for traffic light '{}' with {} links.",
                linkIndex,
                from.get_id(),
                to.get_id(),
                self.get_id(),
                self.myTLLogic.m().get_num_links()
            )));
        }
        if linkIndex2 >= self.myTLLogic.m().get_num_links() {
            std::panic::panic_any(ProcessError::new(format!(
                "Invalid linkIndex2 {} in connection from edge '{}' to edge '{}' for traffic light '{}' with {} links.",
                linkIndex2,
                from.get_id(),
                to.get_id(),
                self.get_id(),
                self.myTLLogic.m().get_num_links()
            )));
        }
        let conn = NBConnection::new_2(from, fromLane, to, toLane, linkIndex, linkIndex2);
        // avoid duplicates
        let eq = NBLoadedSUMOTLDef_connection_equal::new(&conn);
        // remove_if does not remove, only re-order
        self.base.myControlledLinks.retain(|c| !eq.call_op(c));
        self.base.myControlledLinks.push(NBConnection::new_4(&conn));
        self.base.add_node(from.get_to_node());
        self.base.add_node(to.get_from_node());
        // added connections are definitely controlled. make sure none are removed because they lie within the tl
        // myControlledInnerEdges.insert(from->getID()); // @todo recheck: this appears to be obsolete
        // set this information now so that it can be used while loading diffs
        from.m().set_controlling_tl_information(&conn, self.get_id());
        self.myReconstructAddedConnections |= reconstruct;
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
        let mut i: usize = 0;
        while i < self.base.myControlledLinks.len() {
            let it = &self.base.myControlledLinks[i];
            let matches = (it.get_from() == conn.get_from()
                && it.get_to() == conn.get_to()
                && it.get_from_lane() == conn.get_from_lane()
                && it.get_to_lane() == conn.get_to_lane())
                || (it.get_tl_index() == conn.get_tl_index()
                    && conn.get_tl_index() != NBCONNECTION_INVALID_TL_INDEX
                    && (it.get_from().is_null() || it.get_to().is_null()));
            if matches {
                if reconstruct {
                    self.myReconstructRemovedConnections = true;
                    i += 1;
                } else {
                    self.base.myControlledLinks.remove(i);
                }
            } else {
                i += 1;
            }
        }
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
        self.myReconstructAddedConnections |= addedConnections;
        self.myReconstructRemovedConnections |= removedConnections;
    }

    /// C++: `NBTrafficLightLogic * NBLoadedSUMOTLDef::getLogic()`
    /// Returns the internal logic
    // NBTrafficLightLogic* getLogic() {
    //         return myTLLogic;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic(&mut self) -> Ptr<NBTrafficLightLogic> {
        self.myTLLogic
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
        self.base.myOffset = offset;
        self.myTLLogic.m().set_offset(offset);
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
        self.base.myType = type_;
        self.myTLLogic.m().set_type(type_);
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
        if self.am_invalid() {
            return false;
        }
        if !self.base.myExtraConflictsReady.get() {
            self.base.init_extra_conflicts();
            assert!(self.base.myExtraConflictsReady.get());
        }
        self.base.myExtraConflicts.borrow().contains(&(index, foeIndex))
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
        let maxIndex = self.get_max_valid_index();
        let mut unusedIndices: Vec<i32> = Vec::new();
        for i in 0..=maxIndex {
            if self.is_used(i) {
                if unusedIndices.len() > 0 {
                    self.replace_index(i, i - unusedIndices.len() as i32);
                }
            } else {
                unusedIndices.push(i);
            }
        }
        for i in (0..unusedIndices.len()).rev() {
            self.myTLLogic.m().delete_state_index(unusedIndices[i]);
        }
        if unusedIndices.len() > 0 {
            self.myTLLogic.m().set_state_length(maxIndex + 1 - unusedIndices.len() as i32, LINKSTATE_TL_RED);
            self.set_tl_controlling_information();
            true
        } else {
            false
        }
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
        // count how often each index is used
        let mut indexUsage: BTreeMap<i32, i32> = BTreeMap::new();
        for c in self.base.myControlledLinks.iter() {
            *indexUsage.entry(c.get_tl_index()).or_insert(0) += 1;
        }
        for n in self.base.myControlledNodes.iter() {
            for c in n.get_crossings() {
                *indexUsage.entry(c.tlLinkIndex).or_insert(0) += 1;
                *indexUsage.entry(c.tlLinkIndex2).or_insert(0) += 1;
            }
        }
        indexUsage.iter().any(|(k, v)| *k >= 0 && *v > 1)
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
        def.m().set_participants_information();
        // C++ def->compute(OptionsCont::getOptions())
        let _ = utils::options::options_cont::with_options(|oc| def.m().compute(oc));
        let maxIndex = std::cmp::max(self.get_max_index(), def.m().get_max_index());
        self.myTLLogic.m().set_state_length(maxIndex + 1, LINKSTATE_TL_RED);
        for c in def.get_controlled_links().iter() {
            self.base.myControlledLinks.push(NBConnection::new_4(c));
        }
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
        use utils::common::time::time2steps;
        let mut hasMinMaxDur = false;
        for phase in self.myTLLogic.m().get_phases().iter() {
            if phase.maxDur != NBTrafficLightDefinition_UNSPECIFIED_DURATION {
                hasMinMaxDur = true;
            }
        }
        if !hasMinMaxDur {
            let minMinDur: SUMOTime = time2steps(utils::options::options_cont::get_int("tls.min-dur") as f64);
            let maxDur: SUMOTime = time2steps(utils::options::options_cont::get_int("tls.max-dur") as f64);
            let mut yellowIndices: BTreeSet<usize> = BTreeSet::new();
            for phase in self.myTLLogic.m().get_phases().iter() {
                for (i, &ch) in phase.state.as_bytes().iter().enumerate() {
                    if ch == b'y' || ch == b'Y' {
                        yellowIndices.insert(i);
                    }
                }
            }
            let numPhases = self.myTLLogic.m().get_phases().len();
            for ip in 0..numPhases {
                let mut needMinMaxDur = false;
                let phaseState: String = self.myTLLogic.m().get_phases()[ip].state.clone();
                let mut greenIndices: BTreeSet<i32> = BTreeSet::new();
                if phaseState.bytes().any(|b| b == b'y' || b == b'Y') {
                    continue;
                }
                for (i, &ch) in phaseState.as_bytes().iter().enumerate() {
                    if yellowIndices.contains(&i) && ch == b'G' {
                        needMinMaxDur = true;
                        greenIndices.insert(i as i32);
                    }
                }
                if needMinMaxDur {
                    let mut maxSpeed: f64 = 0.0;
                    for c in self.base.myControlledLinks.iter() {
                        if greenIndices.contains(&c.get_tl_index()) {
                            let speed = c.get_from().get_lane_speed(c.get_from_lane());
                            maxSpeed = if maxSpeed > speed { maxSpeed } else { speed };
                        }
                    }
                    // 5s at 50km/h, 10s at 80km/h, rounded to full seconds
                    let minDurBySpeed: f64 = maxSpeed * 3.6 / 6.0 - 3.3;
                    let minDur: SUMOTime = std::cmp::max(minMinDur, time2steps((minDurBySpeed + 0.5).floor()));
                    self.myTLLogic.m().set_phase_min_duration(ip as i32, minDur);
                    self.myTLLogic.m().set_phase_max_duration(ip as i32, maxDur);
                }
            }
        }
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
        let maxIndex = self.get_max_index();
        let mut unusedIndices: Vec<i32> = Vec::new();
        for i in 0..=maxIndex {
            if self.is_used(i) {
                let edges = self.get_edges_using_index(i);
                // compactify
                self.replace_index(i, i - unusedIndices.len() as i32);
                if edges.len() == 0 {
                    // do not group pedestrian crossing signals
                    continue;
                }
                let states = self.get_states(i);
                for j in (i + 1)..=maxIndex {
                    // only group signals from the same edges as is commonly done by
                    // traffic engineers
                    if states == self.get_states(j) && edges == self.get_edges_using_index(j) {
                        self.replace_index(j, i - unusedIndices.len() as i32);
                    }
                }
            } else {
                unusedIndices.push(i);
            }
        }
        for i in (0..unusedIndices.len()).rev() {
            self.myTLLogic.m().delete_state_index(unusedIndices[i]);
        }
        self.cleanup_states();
        self.set_tl_controlling_information();
        // patch crossing indices
        for n in self.base.myControlledNodes.clone() {
            for c in n.get_crossings() {
                for i in (0..unusedIndices.len()).rev() {
                    if c.m().customTLIndex > i as i32 {
                        c.m().customTLIndex -= 1;
                    }
                    if c.m().customTLIndex2 > i as i32 {
                        c.m().customTLIndex2 -= 1;
                    }
                }
            }
        }
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
        let mut defaultOrdering: NBConnectionVector = Vec::new();
        self.base.collect_all_links(&mut defaultOrdering);
        let mut states: Vec<String> = Vec::new(); // organized per link rather than phase
        let mut index: i32 = 0;
        for c in defaultOrdering.iter() {
            let eq = NBLoadedSUMOTLDef_connection_equal::new(c);
            let pos = self
                .base
                .myControlledLinks
                .iter()
                .position(|c2| eq.call_op(c2))
                .expect("connection_equal: connection not found");
            let oldIndex = self.base.myControlledLinks[pos].get_tl_index();
            let s = self.get_states(oldIndex);
            states.push(s);
            self.base.myControlledLinks[pos].set_tl_index(index);
            index += 1;
        }
        for n in self.base.myControlledNodes.clone() {
            for c in n.get_crossings() {
                let s = self.get_states(c.tlLinkIndex);
                states.push(s);
                c.m().customTLIndex = index;
                index += 1;
                if c.tlLinkIndex2 != NBCONNECTION_INVALID_TL_INDEX {
                    let s2 = self.get_states(c.tlLinkIndex2);
                    states.push(s2);
                    c.m().customTLIndex2 = index;
                    index += 1;
                }
            }
        }
        self.myTLLogic.m().set_state_length(index, LINKSTATE_TL_RED);
        for i in 0..states.len() {
            for (p, &ch) in states[i].as_bytes().iter().enumerate() {
                self.myTLLogic.m().set_phase_state(p as i32, i as i32, ch as LinkState);
            }
        }
        self.set_tl_controlling_information();
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
        let mut oldStates: BTreeMap<i32, String> = BTreeMap::new(); // organized per link index rather than phase
        let mut newStates: BTreeMap<i32, String> = BTreeMap::new(); // organized per link index rather than phase
        for c in def.get_controlled_links().iter() {
            let eq = NBLoadedSUMOTLDef_connection_equal::new(c);
            let pos = self
                .base
                .myControlledLinks
                .iter()
                .position(|c2| eq.call_op(c2))
                .expect("connection_equal: connection not found");
            let oldIndex = self.base.myControlledLinks[pos].get_tl_index();
            let newIndex = c.get_tl_index();
            let states = self.get_states(oldIndex);
            oldStates.insert(oldIndex, states.clone());
            if newStates.contains_key(&newIndex) && newStates[&newIndex] != states {
                write_warningf!("Signal groups from program '%' are incompatible with the states of program '%' at tlLogic '%'. Possibly unsafe program.", def.get_program_id(), self.get_program_id(), self.get_id());
            } else {
                newStates.insert(newIndex, states);
            }
            self.base.myControlledLinks[pos].set_tl_index(newIndex);
        }
        let maxIndex = self.get_max_index();
        self.myTLLogic.m().set_state_length(maxIndex + 1, LINKSTATE_TL_RED);
        // C++ re-evaluates newStates.size() and operator[] (which may insert) in every iteration
        let mut i: i32 = 0;
        while (i as usize) < newStates.len() {
            let states: String = newStates.entry(i).or_default().clone();
            for (p, &ch) in states.as_bytes().iter().enumerate() {
                self.myTLLogic.m().set_phase_state(p as i32, i, ch as LinkState);
            }
            i += 1;
        }
        self.set_tl_controlling_information();
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
        let mut i: i32 = 0;
        while i < self.myTLLogic.m().get_num_links() {
            if !self.is_used(i) {
                write_warningf!("Unused state in tlLogic '%', program '%' at tl-index %", self.get_id(), self.get_program_id(), i);
                break;
            }
            i += 1;
        }
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
        if oldIndex == newIndex {
            return;
        }
        for c in self.base.myControlledLinks.iter_mut() {
            if c.get_tl_index() == oldIndex {
                c.set_tl_index(newIndex);
            }
            if c.get_tl_index2() == oldIndex {
                c.set_tl_index2(newIndex);
            }
        }
        for n in self.base.myControlledNodes.iter() {
            for c in n.get_crossings() {
                if c.tlLinkIndex == oldIndex {
                    c.m().tlLinkIndex = newIndex;
                }
                if c.tlLinkIndex2 == oldIndex {
                    c.m().tlLinkIndex2 = newIndex;
                }
            }
        }
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
        if self.base.myControlledLinks.len() == 0 {
            // maybe we only loaded a different program for a default traffic light.
            // Try to build links now.
            let mut links = std::mem::take(&mut self.base.myControlledLinks);
            self.base.collect_all_links(&mut links);
            self.base.myControlledLinks = links;
        }
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
        if self.base.myControlledLinks.len() == 0 {
            self.base.collect_edges_base();
        }
        self.base.myIncomingEdges.clear();
        let mut myOutgoing: EdgeVector = Vec::new();
        // collect the edges from the participating nodes
        for node in self.base.myControlledNodes.iter() {
            self.base.myIncomingEdges.extend(node.get_incoming_edges().iter().copied());
            myOutgoing.extend(node.get_outgoing_edges().iter().copied());
        }
        // check which of the edges are completely within the junction
        // and which are uncontrolled as well (we already know myControlledLinks)
        let mut j: usize = 0;
        while j < self.base.myIncomingEdges.len() {
            let edge = self.base.myIncomingEdges[j];
            edge.m().set_inside_tls(false); // reset
            // an edge lies within the logic if it is outgoing as well as incoming
            if myOutgoing.contains(&edge) {
                if !self.base.myControlledInnerEdges.contains(edge.get_id()) {
                    let mut controlled = false;
                    for it in self.base.myControlledLinks.iter() {
                        if it.get_from() == edge {
                            controlled = true;
                            break;
                        }
                    }
                    if controlled {
                        self.base.myControlledInnerEdges.insert(edge.get_id().to_string());
                    } else {
                        self.base.myEdgesWithin.push(edge);
                        edge.m().set_inside_tls(true);
                        j += 1; //j = myIncomingEdges.erase(j);
                        continue;
                    }
                }
            }
            j += 1;
        }
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
        // @todo what to do with those parameters?
        let _ = brakingTimeSeconds;
        self.base.init_extra_conflicts();
        self.reconstruct_logic();
        self.myTLLogic.m().close_building(false);
        self.patch_if_crossings_added();
        self.myTLLogic.m().close_building(true);
        Ptr::from_raw(Box::into_raw(Box::new(NBTrafficLightLogic::new_2(self.myTLLogic))))
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
        if self.base.myControlledLinks.len() == 0 {
            return true;
        }
        if self.base.myIncomingEdges.len() == 0 {
            return true;
        }
        if self.myReconstructRemovedConnections {
            // check whether at least one connection is valid
            for con in self.base.myControlledLinks.iter() {
                if self.is_valid(con) {
                    return false;
                }
            }
            // all invalid
            return true;
        }
        false
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
        let me = Ptr::new(self);
        if !self.am_invalid() && !self.base.myNeedsContRelationReady.get() {
            self.base.myNeedsContRelation.borrow_mut().clear();
            self.base.myExtraConflicts.borrow_mut().clear();
            if self.base.myType == TrafficLightType::NEMA {
                me.m().base.init_needs_cont_relation_base();
                me.m().base.init_extra_conflicts();
            } else {
                let controlledWithin = !utils::options::options_cont::get_bool("tls.uncontrolled-within");
                let phases: Vec<String> = self.myTLLogic.m().get_phases().iter().map(|p| p.state.clone()).collect();
                for state in phases.iter() {
                    // C++ std::string operator[] at size() yields '\0'
                    let at = |i: i32| -> u8 { state.as_bytes().get(i as usize).copied().unwrap_or(0) };
                    for c1 in self.base.myControlledLinks.iter() {
                        let i1 = c1.get_tl_index();
                        if i1 == NBCONNECTION_INVALID_TL_INDEX
                            || (at(i1) != b'g' && at(i1) != b's')
                            || c1.get_from().is_null()
                            || c1.get_to().is_null()
                        {
                            continue;
                        }
                        for c2 in self.base.myControlledLinks.iter() {
                            let i2 = c2.get_tl_index();
                            if i2 != NBCONNECTION_INVALID_TL_INDEX
                                && i2 != i1
                                && (at(i2) == b'G' || at(i2) == b'g')
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
                                let forbidden = self.base.forbids(c2.get_from(), c2.get_to(), c1.get_from(), c1.get_to(), true, controlledWithin);
                                let isFoes = self.base.foes(c2.get_from(), c2.get_to(), c1.get_from(), c1.get_to())
                                    && !c2.get_from().is_turning_direction_at(c2.get_to());
                                let hasContRel = (forbidden && at(i1) != b's') || rightTurnConflict;
                                if hasContRel {
                                    self.base.myNeedsContRelation.borrow_mut().insert(NBTrafficLightDefinition_StreamPair::new(
                                        c1.get_from(),
                                        c1.get_to(),
                                        c2.get_from(),
                                        c2.get_to(),
                                    ));
                                }
                                // shortcut: NBEdge::get_connection returns CHBuilder_Connection, which has no indirectLeft field in this port, so indirectLeft is taken as false, upgrade: port NBEdge::getConnection returning NBEdge_Connection
                                let indirectLeft = false;
                                if isFoes && (at(i1) == b's' || (!hasContRel && at(i2) == b'G' && !indirectLeft)) {
                                    self.base.myExtraConflicts.borrow_mut().insert((i1, i2));
                                }
                            }
                        }
                    }
                }
            }
        }
        me.base.myNeedsContRelationReady.set(true);
        me.base.myExtraConflictsReady.set(true);
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
        let mut maxIndex: i32 = -1;
        for c in self.base.myControlledLinks.iter() {
            maxIndex = std::cmp::max(maxIndex, c.get_tl_index());
            maxIndex = std::cmp::max(maxIndex, c.get_tl_index2());
        }
        for n in self.base.myControlledNodes.iter() {
            for c in n.get_crossings() {
                maxIndex = std::cmp::max(maxIndex, c.tlLinkIndex);
                maxIndex = std::cmp::max(maxIndex, c.tlLinkIndex2);
            }
        }
        maxIndex
    }

    /// C++: `int NBLoadedSUMOTLDef::getMaxValidIndex()` [virtual, protected]
    /// Returns the maximum index controlled by this traffic light
    // int
    // NBLoadedSUMOTLDef::getMaxValidIndex() {
    //     return myTLLogic->getNumLinks() - 1;
    // }
    #[allow(unused_variables)]
    pub fn get_max_valid_index(&mut self) -> i32 {
        self.myTLLogic.m().get_num_links() - 1
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
        assert!(index >= 0);
        assert!(index <= self.get_max_valid_index());
        let mut result = String::new();
        for pd in self.myTLLogic.m().get_phases().iter() {
            result.push(pd.state.as_bytes()[index as usize] as char);
        }
        result
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
        for c in self.base.myControlledLinks.iter() {
            if c.get_tl_index() == index || c.get_tl_index2() == index {
                return true;
            }
        }
        for n in self.base.myControlledNodes.iter() {
            for c in n.get_crossings() {
                if c.tlLinkIndex == index || c.tlLinkIndex2 == index {
                    return true;
                }
            }
        }
        false
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
        let mut result: BTreeSet<Ptr<NBEdge>> = BTreeSet::new();
        for c in self.base.myControlledLinks.iter() {
            if c.get_tl_index() == index || c.get_tl_index2() == index {
                result.insert(c.get_from());
            }
        }
        result
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
        let size = self.myTLLogic.m().get_num_links();
        let mut noLinksAll: i32 = 0;
        for c in self.base.myControlledLinks.iter() {
            if c.get_tl_index() != NBCONNECTION_INVALID_TL_INDEX {
                noLinksAll = std::cmp::max(noLinksAll, c.get_tl_index() + 1);
            }
        }
        let numNormalLinks = noLinksAll;
        let mut oldCrossings: i32 = 0;
        // collect crossings
        let mut customIndex = false;
        let mut crossings: Vec<Ptr<NBNode_Crossing>> = Vec::new();
        for n in self.base.myControlledNodes.iter() {
            let c: Vec<Ptr<NBNode_Crossing>> = n.get_crossings();
            // set tl indices for crossings
            customIndex |= n.m().set_crossing_tl_indices(self.get_id(), noLinksAll, false);
            crossings.extend(c.iter().copied());
            noLinksAll += c.len() as i32;
            oldCrossings += n.num_crossings_from_sumo_net();
        }
        if crossings.len() as i32 != oldCrossings {
            let phases: Vec<NBTrafficLightLogic_PhaseDefinition> = self.myTLLogic.m().get_phases().clone();
            // do not rebuilt crossing states there are custom indices and the state string is long enough
            if phases.len() > 0
                && ((phases[0].state.len() as i32) < noLinksAll || ((phases[0].state.len() as i32) > noLinksAll && !customIndex))
            {
                // collect edges
                let mut fromEdges: EdgeVector = vec![Ptr::null(); numNormalLinks as usize];
                let mut toEdges: EdgeVector = vec![Ptr::null(); numNormalLinks as usize];
                let mut fromLanes: Vec<i32> = vec![0; size as usize];
                self.collect_edge_vectors(&mut fromEdges, &mut toEdges, &mut fromLanes);
                let crossingDefaultState: String = "r".repeat(crossings.len());

                // rebuild the logic (see NBOwnTLDef.cpp::myCompute)
                let newLogicObj = NBTrafficLightLogic::new(self.get_id(), self.get_program_id(), 0, self.base.myOffset, self.base.myType);
                let newLogic: Ptr<NBTrafficLightLogic> = Ptr::from_raw(Box::into_raw(Box::new(newLogicObj)));
                let brakingTime: SUMOTime = utils::common::time::time2steps(
                    self.base.compute_braking_time(utils::options::options_cont::get_float("tls.yellow.min-decel")) as f64,
                );
                for phase in phases.iter() {
                    let len = std::cmp::min(numNormalLinks as usize, phase.state.len());
                    let state = format!("{}{}", &phase.state[..len], crossingDefaultState);
                    let _ = NBOwnTLDef::add_pedestrian_phases(
                        newLogic,
                        phase.duration,
                        phase.minDur,
                        phase.maxDur,
                        phase.earliestEnd,
                        phase.latestEnd,
                        state,
                        &crossings,
                        &fromEdges,
                        &toEdges,
                    );
                }
                NBOwnTLDef::add_pedestrian_scramble(
                    newLogic,
                    noLinksAll,
                    utils::common::time::time2steps(10.0),
                    brakingTime,
                    &crossings,
                    &fromEdges,
                    &toEdges,
                );

                // delete myTLLogic
                delete_tl_logic(self.myTLLogic);
                self.myTLLogic = newLogic;
            } else if phases.len() == 0 {
                write_warningf!("Could not patch tlLogic '%' for changed crossings", self.get_id());
            }
        }
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
        assert!(fromEdges.len() > 0);
        assert!(fromEdges.len() == toEdges.len());
        let size = fromEdges.len() as i32;
        for c in self.base.myControlledLinks.iter() {
            if c.get_tl_index() != NBCONNECTION_INVALID_TL_INDEX {
                if c.get_tl_index() >= size {
                    std::panic::panic_any(ProcessError::new(format!(
                        "Invalid linkIndex {} for traffic light '{}' with {} links.",
                        c.get_tl_index(),
                        self.get_id(),
                        size
                    )));
                }
                let idx = c.get_tl_index() as usize;
                fromEdges[idx] = c.get_from();
                toEdges[idx] = c.get_to();
                fromLanes[idx] = c.get_from_lane();
            }
        }
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
        let netedit = NBNetBuilder::running_netedit();
        if self.myReconstructAddedConnections {
            self.myReconstructAddedConnections = false;
            // do not rebuild the logic when running netedit and all links are already covered by the program
            if !self.myPhasesLoaded && !(netedit && self.has_valid_indices()) {
                // rebuild the logic from scratch
                // XXX if a connection with the same from- and to-edge already exisits, its states could be copied instead
                let mut dummy = NBOwnTLDef::new(NBTrafficLightDefinition_DummyID, &self.base.myControlledNodes, 0, self.base.get_type());
                dummy.set_participants_information();
                let programID: String = self.base.get_program_id().to_string();
                dummy.set_program_id(&programID);
                dummy.set_tl_controlling_information();
                let newLogic: Ptr<NBTrafficLightLogic> = utils::options::options_cont::with_options(|oc| dummy.compute(oc));
                self.base.myIncomingEdges = dummy.get_incoming_edges().clone();
                self.base.myControlledLinks = dummy.get_controlled_links().iter().map(NBConnection::new_4).collect();
                for n in self.base.myControlledNodes.iter() {
                    n.m().remove_traffic_light(Ptr::from_mut(&mut dummy.base));
                }
                delete_tl_logic(self.myTLLogic);
                self.myTLLogic = newLogic;
                if !newLogic.is_null() {
                    newLogic.m().set_id(self.get_id());
                    newLogic.m().set_type(self.base.get_type());
                    let offset = self.base.get_offset();
                    newLogic.m().set_offset(offset);
                    self.set_tl_controlling_information();
                    // reset crossing custom indices
                    for n in self.base.myControlledNodes.iter() {
                        for c in n.get_crossings() {
                            c.m().customTLIndex = NBCONNECTION_INVALID_TL_INDEX;
                        }
                    }
                }
            } else {
                self.set_tl_controlling_information();
            }
        }
        if self.myReconstructRemovedConnections {
            self.myReconstructRemovedConnections = false;
            // for each connection, check whether it is still valid
            let mut idx: usize = 0;
            while idx < self.base.myControlledLinks.len() {
                let con = NBConnection::new_4(&self.base.myControlledLinks[idx]);
                if self.is_valid(&con) {
                    idx += 1;
                } else {
                    // remove connection
                    let removed = con.get_tl_index();
                    self.base.myControlledLinks.remove(idx);
                    // no automatic modificaions when running netedit
                    if !self.myPhasesLoaded && !(netedit && self.has_valid_indices()) {
                        // shift index off successive connections and remove entry from all phases if the tlIndex was only used by this connection
                        let mut exclusive = true;
                        for other in self.base.myControlledLinks.iter() {
                            if other.ne_op(&con) && other.get_tl_index() == removed {
                                exclusive = false;
                                break;
                            }
                        }
                        if exclusive {
                            // shift indices above the removed index downward
                            for other in self.base.myControlledLinks.iter_mut() {
                                if other.get_tl_index() > removed {
                                    other.set_tl_index(other.get_tl_index() - 1);
                                }
                            }
                            // shift crossing custom indices above the removed index downward
                            for n in self.base.myControlledNodes.iter() {
                                for c in n.get_crossings() {
                                    if c.customTLIndex > removed {
                                        c.m().customTLIndex -= 1;
                                    }
                                }
                            }
                            // rebuild the logic
                            let newLogic: Ptr<NBTrafficLightLogic> = Ptr::from_raw(Box::into_raw(Box::new(NBTrafficLightLogic::new(
                                self.get_id(),
                                self.get_program_id(),
                                0,
                                self.base.myOffset,
                                self.base.myType,
                            ))));
                            for phase in self.myTLLogic.m().get_phases().iter() {
                                let mut newState: String = phase.state.clone();
                                newState.remove(removed as usize);
                                newLogic.m().add_step(phase.duration, &newState, &Vec::new(), "", -1);
                            }
                            delete_tl_logic(self.myTLLogic);
                            self.myTLLogic = newLogic;
                        }
                    }
                }
            }
            self.set_tl_controlling_information();
        }
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
        for c in self.base.myControlledLinks.iter() {
            if c.get_tl_index() == NBCONNECTION_INVALID_TL_INDEX {
                return false;
            }
        }
        for n in self.base.myControlledNodes.iter() {
            for c in n.get_crossings() {
                if c.tlLinkIndex == NBCONNECTION_INVALID_TL_INDEX {
                    return false;
                }
            }
        }
        // method getMaxIndex() is const but cannot be declare as such due to inheritance
        Ptr::new(self).m().get_max_index() < self.myTLLogic.m().get_num_links()
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
        self.base.myIncomingEdges.contains(&con.get_from())
            // connection still exists
            && con.get_from().has_connection_to(con.get_to(), con.get_to_lane(), con.get_from_lane())
            // connection is still set to be controlled
            && con.get_from().may_be_tls_controlled(con.get_from_lane(), con.get_to(), con.get_to_lane())
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
        Self { myC: Ptr::new(c) }
    }

    /// C++: `bool connection_equal::operator()(const NBConnection &) const` [const]
    // bool operator()(const NBConnection& c) const {
    //             return c.getFrom() == myC.getFrom() && c.getTo() == myC.getTo() &&
    //                    c.getFromLane() == myC.getFromLane() && c.getToLane() == myC.getToLane();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c: &NBConnection) -> bool {
        c.get_from() == self.myC.get_from()
            && c.get_to() == self.myC.get_to()
            && c.get_from_lane() == self.myC.get_from_lane()
            && c.get_to_lane() == self.myC.get_to_lane()
    }

}

