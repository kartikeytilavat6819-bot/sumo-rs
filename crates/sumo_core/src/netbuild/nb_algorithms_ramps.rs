//! Port of netbuild/NBAlgorithms_Ramps.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::netbuild::nb_helpers::NBHelpers;
use utils::common::std_defs::SUMO_CONST_LANE_WIDTH;

// C++ static NBEdge::UNSPECIFIED_WIDTH (netbuild/NBEdge.cpp:69)
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;

// C++ static NBRampsComputer::ADDED_ON_RAMP_EDGE (netbuild/NBAlgorithms_Ramps.cpp:45)
const ADDED_ON_RAMP_EDGE: &str = "-AddedOnRampEdge";
// C++ #define OFFRAMP_LOOKBACK / MIN_SPLIT_LENGTH (netbuild/NBAlgorithms_Ramps.cpp:34-35)
const OFFRAMP_LOOKBACK: f64 = 500.0;
const MIN_SPLIT_LENGTH: f64 = utils::geom::POSITION_EPS;

// C++ `throw ProcessError(...)` and throwing option getters: unwind with the ProcessError as the panic payload
fn ramps_ok<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| std::panic::panic_any(e))
}

fn ramps_connection_error() -> ! {
    std::panic::panic_any(ProcessError::new("Could not set connection!"))
}

/// C++ class `NBRampsComputer` (netbuild/NBAlgorithms_Ramps.h)
pub struct NBRampsComputer {
    pub myShiftedEdges: BTreeMap<Ptr<NBEdge>, f64>,
}

// C++ static members of NBRampsComputer (port as module-level statics, see AGENTS.md):
//   static const std::string NBRampsComputer::ADDED_ON_RAMP_EDGE;

impl NBRampsComputer {
    /// C++: `void NBRampsComputer::NBRampsComputer()`
    /// Computes highway on-/off-ramps (if wished)
    // NBRampsComputer::NBRampsComputer() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { myShiftedEdges: BTreeMap::new() }
    }

    /// C++: `void NBRampsComputer::computeRamps(NBNetBuilder &, OptionsCont &, bool)`
    // void
    // NBRampsComputer::computeRamps(NBNetBuilder& nb, OptionsCont& oc, bool mayAddOrRemove) {
    //     const bool guessAndAdd = oc.getBool("ramps.guess") && mayAddOrRemove;
    //     const double minHighwaySpeed = oc.getFloat("ramps.min-highway-speed");
    //     const double maxRampSpeed = oc.getFloat("ramps.max-ramp-speed");
    //     const double rampLength = oc.getFloat("ramps.ramp-length");
    //     const double minWeaveLength = oc.getFloat("ramps.min-weave-length");
    //     const bool dontSplit = oc.getBool("ramps.no-split");
    //     NBNodeCont& nc = nb.getNodeCont();
    //     NBEdgeCont& ec = nb.getEdgeCont();
    //     NBDistrictCont& dc = nb.getDistrictCont();
    //     std::set<NBEdge*> incremented;
    //     // collect join exclusions
    //     std::set<std::string> noramps;
    //     if (oc.isSet("ramps.unset")) {
    //         std::vector<std::string> edges = oc.getStringVector("ramps.unset");
    //         noramps.insert(edges.begin(), edges.end());
    //     }
    //     // exclude roundabouts
    //     for (const EdgeSet& round : ec.getRoundabouts()) {
    //         for (NBEdge* const edge : round) {
    //             noramps.insert(edge->getID());
    //         }
    //     }
    //     // exclude public transport edges
    //     nb.getPTStopCont().addEdges2Keep(oc, noramps);
    //     nb.getParkingCont().addEdges2Keep(oc, noramps);
    //
    //     // check whether on-off ramps shall be guessed
    //     if (guessAndAdd || oc.getBool("ramps.guess-acceleration-lanes")) {
    //         for (const auto& it : ec) {
    //             it.second->markOffRamp(false);
    //         }
    //
    //         // if an edge is part of two ramps, ordering is important
    //         std::set<NBNode*, ComparatorIdLess> potOnRamps;
    //         std::set<NBNode*, ComparatorIdLess> potOffRamps;
    //         for (const auto& i : nc) {
    //             NBNode* cur = i.second;
    // #ifdef DEBUG_RAMPS
    //             if (DEBUGCOND(cur)) {
    //                 std::cout << "check ramps cur=" << cur->getID() << "\n";
    //             }
    // #endif
    //             if (mayNeedOnRamp(cur, minHighwaySpeed, maxRampSpeed, noramps, minWeaveLength)) {
    //                 potOnRamps.insert(cur);
    //             }
    //             if (mayNeedOffRamp(cur, minHighwaySpeed, maxRampSpeed, noramps)) {
    //                 potOffRamps.insert(cur);
    //             }
    //         }
    //         for (std::set<NBNode*, ComparatorIdLess>::const_iterator i = potOnRamps.begin(); i != potOnRamps.end(); ++i) {
    //             buildOnRamp(*i, nc, ec, dc, rampLength, dontSplit || !guessAndAdd, guessAndAdd);
    //         }
    //         for (std::set<NBNode*, ComparatorIdLess>::const_iterator i = potOffRamps.begin(); i != potOffRamps.end(); ++i) {
    //             buildOffRamp(*i, nc, ec, dc, rampLength, dontSplit || !guessAndAdd, guessAndAdd, potOnRamps);
    //         }
    //     }
    //     // check whether on-off ramps are specified
    //     if (oc.isSet("ramps.set") && mayAddOrRemove) {
    //         std::vector<std::string> edges = oc.getStringVector("ramps.set");
    //         std::set<NBNode*, ComparatorIdLess> potOnRamps;
    //         for (const std::string& i : edges) {
    //             NBEdge* e = ec.retrieve(i);
    //             if (noramps.count(i) != 0) {
    //                 WRITE_WARNINGF(TL("Can not build ramp on edge '%' - the edge is unsuitable."), i);
    //                 continue;
    //             }
    //             if (e == nullptr) {
    //                 WRITE_WARNINGF(TL("Can not build on ramp on edge '%' - the edge is not known."), i);
    //                 continue;
    //             }
    //             NBNode* from = e->getFromNode();
    //             if (from->getIncomingEdges().size() == 2 && from->getOutgoingEdges().size() == 1) {
    //                 buildOnRamp(from, nc, ec, dc, rampLength, dontSplit, true);
    //                 potOnRamps.insert(from);
    //             }
    //             // load edge again to check offramps
    //             e = ec.retrieve(i);
    //             if (e == nullptr) {
    //                 WRITE_WARNINGF(TL("Can not build off ramp on edge '%' - the edge is not known."), i);
    //                 continue;
    //             }
    //             NBNode* to = e->getToNode();
    //             if (to->getIncomingEdges().size() == 1 && to->getOutgoingEdges().size() == 2) {
    //                 buildOffRamp(to, nc, ec, dc, rampLength, dontSplit, true, potOnRamps);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_ramps(&mut self, nb: &mut NBNetBuilder, oc: &mut OptionsCont, mayAddOrRemove: bool) {
        let guessAndAdd = ramps_ok(oc.get_bool("ramps.guess")) && mayAddOrRemove;
        let minHighwaySpeed = ramps_ok(oc.get_float("ramps.min-highway-speed"));
        let maxRampSpeed = ramps_ok(oc.get_float("ramps.max-ramp-speed"));
        let rampLength = ramps_ok(oc.get_float("ramps.ramp-length"));
        let minWeaveLength = ramps_ok(oc.get_float("ramps.min-weave-length"));
        let dontSplit = ramps_ok(oc.get_bool("ramps.no-split"));
        // the containers live inside nb; take raw-pointer handles so they can be used together
        let nc_p = Ptr::from_mut(nb.get_node_cont());
        let nc = nc_p.m();
        let ec_p = Ptr::from_mut(nb.get_edge_cont());
        let ec = ec_p.m();
        let dc_p = Ptr::from_mut(nb.get_district_cont());
        let dc = dc_p.m();
        // collect join exclusions
        let mut noramps: BTreeSet<String> = BTreeSet::new();
        if ramps_ok(oc.is_set("ramps.unset", true)) {
            let edges = ramps_ok(oc.get_string_vector("ramps.unset")).clone();
            for s in edges.iter() {
                noramps.insert(s.clone());
            }
        }
        // exclude roundabouts
        for round in ec.get_roundabouts().iter() {
            for edge in round.iter() {
                noramps.insert(edge.get_id().to_string());
            }
        }
        // exclude public transport edges
        nb.get_pt_stop_cont().add_edges2_keep(oc, &mut noramps);
        nb.get_parking_cont().add_edges2_keep(oc, &mut noramps);

        // check whether on-off ramps shall be guessed
        if guessAndAdd || ramps_ok(oc.get_bool("ramps.guess-acceleration-lanes")) {
            for it in ec.myEdges.values() {
                it.m().mark_off_ramp(false);
            }

            // if an edge is part of two ramps, ordering is important
            // determinism: C++ std::set with ComparatorIdLess iterates by node id, so the candidates are keyed by id
            let mut potOnRamps: BTreeMap<String, Ptr<NBNode>> = BTreeMap::new();
            let mut potOffRamps: BTreeMap<String, Ptr<NBNode>> = BTreeMap::new();
            for cur in nc.myNodes.values().copied() {
                if Self::may_need_on_ramp(cur, minHighwaySpeed, maxRampSpeed, &noramps, minWeaveLength) {
                    potOnRamps.insert(cur.get_id().to_string(), cur);
                }
                if Self::may_need_off_ramp(cur, minHighwaySpeed, maxRampSpeed, &noramps) {
                    potOffRamps.insert(cur.get_id().to_string(), cur);
                }
            }
            let potOnRampSet: BTreeSet<Ptr<NBNode>> = potOnRamps.values().copied().collect();
            for i in potOnRamps.values().copied() {
                self.build_on_ramp(i, nc, ec, dc, rampLength, dontSplit || !guessAndAdd, guessAndAdd);
            }
            for i in potOffRamps.values().copied() {
                self.build_off_ramp(i, nc, ec, dc, rampLength, dontSplit || !guessAndAdd, guessAndAdd, &potOnRampSet);
            }
        }

        // check whether on-off ramps are specified
        if ramps_ok(oc.is_set("ramps.set", true)) && mayAddOrRemove {
            let edges = ramps_ok(oc.get_string_vector("ramps.set")).clone();
            let mut potOnRamps: BTreeMap<String, Ptr<NBNode>> = BTreeMap::new();
            for i in edges.iter() {
                let mut e: Ptr<NBEdge> = ec.retrieve(i, false);
                if noramps.contains(i) {
                    write_warningf!("Can not build ramp on edge '%' - the edge is unsuitable.", i);
                    continue;
                }
                if e.is_null() {
                    write_warningf!("Can not build on ramp on edge '%' - the edge is not known.", i);
                    continue;
                }
                let from = e.get_from_node();
                if from.get_incoming_edges().len() == 2 && from.get_outgoing_edges().len() == 1 {
                    self.build_on_ramp(from, nc, ec, dc, rampLength, dontSplit, true);
                    potOnRamps.insert(from.get_id().to_string(), from);
                }
                // load edge again to check offramps
                e = ec.retrieve(i, false);
                if e.is_null() {
                    write_warningf!("Can not build off ramp on edge '%' - the edge is not known.", i);
                    continue;
                }
                let to = e.get_to_node();
                if to.get_incoming_edges().len() == 1 && to.get_outgoing_edges().len() == 2 {
                    let potOnRampSet: BTreeSet<Ptr<NBNode>> = potOnRamps.values().copied().collect();
                    self.build_off_ramp(to, nc, ec, dc, rampLength, dontSplit, true, &potOnRampSet);
                }
            }
        }
    }

    /// C++: `bool NBRampsComputer::mayNeedOnRamp(NBNode *, double, double, const std::set<std::string> &, double)` [static, private]
    /// Determines whether the given node may be an on-ramp begin
    // bool
    // NBRampsComputer::mayNeedOnRamp(NBNode* cur, double minHighwaySpeed, double maxRampSpeed, const std::set<std::string>& noramps, double minWeaveLength) {
    //     if (cur->getOutgoingEdges().size() != 1 || cur->getIncomingEdges().size() != 2) {
    //         return false;
    //     }
    //     NBEdge* potHighway, *potRamp, *cont;
    //     getOnRampEdges(cur, &potHighway, &potRamp, &cont);
    //     // may be an on-ramp
    // #ifdef DEBUG_RAMPS
    //     if (DEBUGCOND(cur)) {
    //         std::cout << "check on ramp hw=" << potHighway->getID() << " ramp=" << potRamp->getID() << " cont=" << cont->getID() << std::endl;
    //     }
    // #endif
    //     if (fulfillsRampConstraints(potHighway, potRamp, cont, minHighwaySpeed, maxRampSpeed, noramps)) {
    //         // prevent short weaving section
    //         double seen = cont->getLength();
    //         while (seen < minWeaveLength) {
    //             if (cont->getToNode()->getOutgoingEdges().size() > 1) {
    //                 return false;
    //             } else if (cont->getToNode()->getOutgoingEdges().size() == 0) {
    //                 return true;
    //             }
    //             cont = cont->getToNode()->getOutgoingEdges().front();
    //             seen += cont->getLength();
    //         }
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn may_need_on_ramp(cur: Ptr<NBNode>, minHighwaySpeed: f64, maxRampSpeed: f64, noramps: &BTreeSet<String>, minWeaveLength: f64) -> bool {
        if cur.get_outgoing_edges().len() != 1 || cur.get_incoming_edges().len() != 2 {
            return false;
        }
        let mut potHighway: Ptr<NBEdge> = Ptr::null();
        let mut potRamp: Ptr<NBEdge> = Ptr::null();
        let mut cont: Ptr<NBEdge> = Ptr::null();
        Self::get_on_ramp_edges(cur, Ptr::from_mut(&mut potHighway), Ptr::from_mut(&mut potRamp), Ptr::from_mut(&mut cont));
        // may be an on-ramp
        if Self::fulfills_ramp_constraints(potHighway, potRamp, cont, minHighwaySpeed, maxRampSpeed, noramps) {
            // prevent short weaving section
            let mut seen = cont.get_length();
            while seen < minWeaveLength {
                if cont.get_to_node().get_outgoing_edges().len() > 1 {
                    return false;
                } else if cont.get_to_node().get_outgoing_edges().len() == 0 {
                    return true;
                }
                cont = cont.get_to_node().get_outgoing_edges()[0];
                seen += cont.get_length();
            }
            true
        } else {
            false
        }
    }

    /// C++: `bool NBRampsComputer::mayNeedOffRamp(NBNode *, double, double, const std::set<std::string> &)` [static, private]
    /// Determines whether the given node may be an off-ramp end
    // bool
    // NBRampsComputer::mayNeedOffRamp(NBNode* cur, double minHighwaySpeed, double maxRampSpeed, const std::set<std::string>& noramps) {
    //     if (cur->getIncomingEdges().size() != 1 || cur->getOutgoingEdges().size() != 2) {
    //         return false;
    //     }
    //     // may be an off-ramp
    //     NBEdge* potHighway, *potRamp, *prev;
    //     getOffRampEdges(cur, &potHighway, &potRamp, &prev);
    // #ifdef DEBUG_RAMPS
    //     if (DEBUGCOND(cur)) {
    //         std::cout << "check off ramp hw=" << potHighway->getID() << " ramp=" << potRamp->getID() << " prev=" << prev->getID() << std::endl;
    //     }
    // #endif
    //     return fulfillsRampConstraints(potHighway, potRamp, prev, minHighwaySpeed, maxRampSpeed, noramps);
    // }
    #[allow(unused_variables)]
    pub fn may_need_off_ramp(cur: Ptr<NBNode>, minHighwaySpeed: f64, maxRampSpeed: f64, noramps: &BTreeSet<String>) -> bool {
        if cur.get_incoming_edges().len() != 1 || cur.get_outgoing_edges().len() != 2 {
            return false;
        }
        // may be an off-ramp
        let mut potHighway: Ptr<NBEdge> = Ptr::null();
        let mut potRamp: Ptr<NBEdge> = Ptr::null();
        let mut prev: Ptr<NBEdge> = Ptr::null();
        Self::get_off_ramp_edges(cur, Ptr::from_mut(&mut potHighway), Ptr::from_mut(&mut potRamp), Ptr::from_mut(&mut prev));
        Self::fulfills_ramp_constraints(potHighway, potRamp, prev, minHighwaySpeed, maxRampSpeed, noramps)
    }

    /// C++: `void NBRampsComputer::buildOnRamp(NBNode *, NBNodeCont &, NBEdgeCont &, NBDistrictCont &, double, bool, bool)` [private]
    /// Builds an on-ramp starting at the given node
    // void
    // NBRampsComputer::buildOnRamp(NBNode* cur, NBNodeCont& nc, NBEdgeCont& ec, NBDistrictCont& dc, double rampLength, bool dontSplit, bool addLanes) {
    //     NBEdge* potHighway, *potRamp, *cont;
    //     getOnRampEdges(cur, &potHighway, &potRamp, &cont);
    // #ifdef DEBUG_RAMPS
    //     if (DEBUGCOND(cur)) {
    //         std::cout << "buildOnRamp cur=" << cur->getID() << " hw=" << potHighway->getID() << " ramp=" << potRamp->getID() << " cont=" << cont->getID() << "\n";
    //     }
    // #endif
    //     // compute the number of lanes to append
    //     const int firstLaneNumber = cont->getNumLanes();
    //     const int toAdd = (potRamp->getNumLanes() + potHighway->getNumLanes()) - firstLaneNumber;
    //     NBEdge* first = cont;
    //     NBEdge* last = cont;
    //     NBEdge* curr = cont;
    //     std::set<NBEdge*> incremented;
    //     if (addLanes && toAdd > 0 && std::find(incremented.begin(), incremented.end(), cont) == incremented.end()) {
    //         double currLength = 0;
    //         while (curr != nullptr && currLength + curr->getGeometry().length() - POSITION_EPS < rampLength) {
    //             if (find(incremented.begin(), incremented.end(), curr) == incremented.end()) {
    //                 curr->incLaneNo(toAdd);
    //                 // we need to distinguish between loading a .net.xml (connections are defined and applicable)
    //                 // and manual connection patches (should be post-prcess because the lane wasn't added yet)
    //                 if (curr->getStep() < NBEdge::EdgeBuildingStep::LANES2LANES_USER || !ec.hasPostProcessConnection(curr->getID())) {
    //                     curr->invalidateConnections(true);
    //                 }
    //                 incremented.insert(curr);
    //                 moveRampRight(curr, toAdd);
    //                 currLength += curr->getGeometry().length(); // !!! loaded length?
    //                 last = curr;
    //                 // mark acceleration lanes
    //                 for (int i = 0; i < curr->getNumLanes() - potHighway->getNumLanes(); ++i) {
    //                     curr->setAcceleration(i, true);
    //                 }
    //             }
    //             NBNode* nextN = curr->getToNode();
    //             if (nextN->getOutgoingEdges().size() == 1 && nextN->getIncomingEdges().size() == 1) {
    //                 curr = nextN->getOutgoingEdges()[0];
    //                 if (curr->getNumLanes() != firstLaneNumber) {
    //                     // the number of lanes changes along the computation; we'll stop...
    //                     curr = nullptr;
    //                 } else if (curr->isTurningDirectionAt(last)) {
    //                     // turnarounds certainly should not be included in a ramp
    //                     curr = nullptr;
    //                 } else if (curr == potHighway || curr == potRamp) {
    //                     // circular connectivity. do not split!
    //                     curr = nullptr;
    //                 }
    //             } else {
    //                 // ambiguous; and, in fact, what should it be? ...stop
    //                 curr = nullptr;
    //             }
    //         }
    //         // check whether a further split is necessary
    //         if (curr != nullptr && !dontSplit && currLength + MIN_SPLIT_LENGTH < rampLength && curr->getNumLanes() == firstLaneNumber && std::find(incremented.begin(), incremented.end(), curr) == incremented.end()) {
    //             // there is enough place to build a ramp; do it
    //             bool wasFirst = first == curr;
    //             std::string newNodeID = getUnusedID(curr->getID() + "-AddedOnRampNode", nc);
    //             std::string newEdgeID = getUnusedID(curr->getID() + ADDED_ON_RAMP_EDGE, ec);
    //             NBNode* rn = new NBNode(newNodeID, curr->getGeometry().positionAtOffset(rampLength - currLength));
    //             nc.insert(rn);
    //             std::string name = curr->getID();
    //             const double currShift = myShiftedEdges[curr];
    //             if (!ec.splitAt(dc, curr, rn, newEdgeID, curr->getID(), curr->getNumLanes() + toAdd, curr->getNumLanes())) {
    //                 WRITE_WARNING("Could not build on-ramp for edge '"  + curr->getID() + "' for unknown reason");
    //                 return;
    //             }
    //             //ec.retrieve(name)->invalidateConnections();
    //             curr = ec.retrieve(newEdgeID);
    //             // copy shift over
    //             myShiftedEdges[curr] = currShift;
    //             incremented.insert(curr);
    //             last = curr;
    //             moveRampRight(curr, toAdd);
    //             if (wasFirst) {
    //                 first = curr;
    //             }
    //             // mark acceleration lanes
    //             for (int i = 0; i < curr->getNumLanes() - potHighway->getNumLanes(); ++i) {
    //                 curr->setAcceleration(i, true);
    //             }
    //         }
    //         if (curr == cont && dontSplit && addLanes) {
    //             WRITE_WARNING("Could not build on-ramp for edge '"  + curr->getID() + "' due to option '--ramps.no-split'");
    //             return;
    //         }
    //     } else {
    //         // mark acceleration lanes
    //         for (int i = 0; i < firstLaneNumber - potHighway->getNumLanes(); ++i) {
    //             cont->setAcceleration(i, true);
    //         }
    //     }
    //     // set connections from ramp/highway to added ramp
    //     if (addLanes) {
    //         if (potHighway->getStep() < NBEdge::EdgeBuildingStep::LANES2LANES_USER) {
    //             if (!potHighway->addLane2LaneConnections(0, first, potRamp->getNumLanes(), MIN2(first->getNumLanes() - potRamp->getNumLanes(), potHighway->getNumLanes()), NBEdge::Lane2LaneInfoType::VALIDATED, true)) {
    //                 throw ProcessError(TL("Could not set connection!"));
    //             }
    //         }
    //         if (potRamp->getStep() < NBEdge::EdgeBuildingStep::LANES2LANES_USER) {
    //             if (!potRamp->addLane2LaneConnections(0, first, 0, potRamp->getNumLanes(), NBEdge::Lane2LaneInfoType::VALIDATED, true)) {
    //                 throw ProcessError(TL("Could not set connection!"));
    //             }
    //         }
    //         patchRampGeometry(potRamp, first, potHighway, false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_on_ramp(&mut self, cur: Ptr<NBNode>, nc: &mut NBNodeCont, ec: &mut NBEdgeCont, dc: &mut NBDistrictCont, rampLength: f64, dontSplit: bool, addLanes: bool) {
        let mut potHighway: Ptr<NBEdge> = Ptr::null();
        let mut potRamp: Ptr<NBEdge> = Ptr::null();
        let mut cont: Ptr<NBEdge> = Ptr::null();
        Self::get_on_ramp_edges(cur, Ptr::from_mut(&mut potHighway), Ptr::from_mut(&mut potRamp), Ptr::from_mut(&mut cont));
        // compute the number of lanes to append
        let firstLaneNumber = cont.get_num_lanes();
        let toAdd = (potRamp.get_num_lanes() + potHighway.get_num_lanes()) - firstLaneNumber;
        let mut first = cont;
        let mut last = cont;
        let mut curr = cont;
        let mut incremented: HashSet<Ptr<NBEdge>> = HashSet::new();
        if addLanes && toAdd > 0 && !incremented.contains(&cont) {
            let mut currLength: f64 = 0.0;
            while !curr.is_null() && currLength + curr.get_geometry().length() - utils::geom::POSITION_EPS < rampLength {
                if !incremented.contains(&curr) {
                    curr.m().inc_lane_no(toAdd);
                    // we need to distinguish between loading a .net.xml (connections are defined and applicable)
                    // and manual connection patches (should be post-prcess because the lane wasn't added yet)
                    if curr.get_step() < NBEdge_EdgeBuildingStep::LANES2LANES_USER || !ec.has_post_process_connection(curr.get_id(), "") {
                        curr.m().invalidate_connections(true);
                    }
                    incremented.insert(curr);
                    self.move_ramp_right(curr, toAdd);
                    currLength += curr.get_geometry().length(); // !!! loaded length?
                    last = curr;
                    // mark acceleration lanes
                    for i in 0..(curr.get_num_lanes() - potHighway.get_num_lanes()) {
                        curr.m().set_acceleration(i, true);
                    }
                }
                let nextN = curr.get_to_node();
                if nextN.get_outgoing_edges().len() == 1 && nextN.get_incoming_edges().len() == 1 {
                    curr = nextN.get_outgoing_edges()[0];
                    if curr.get_num_lanes() != firstLaneNumber {
                        // the number of lanes changes along the computation; we'll stop...
                        curr = Ptr::null();
                    } else if curr.is_turning_direction_at(last) {
                        // turnarounds certainly should not be included in a ramp
                        curr = Ptr::null();
                    } else if curr == potHighway || curr == potRamp {
                        // circular connectivity. do not split!
                        curr = Ptr::null();
                    }
                } else {
                    // ambiguous; and, in fact, what should it be? ...stop
                    curr = Ptr::null();
                }
            }
            // check whether a further split is necessary
            if !curr.is_null() && !dontSplit && currLength + MIN_SPLIT_LENGTH < rampLength && curr.get_num_lanes() == firstLaneNumber && !incremented.contains(&curr) {
                // there is enough place to build a ramp; do it
                let wasFirst = first == curr;
                let newNodeID = NBRampsComputer::get_unused_id(&(curr.get_id().to_string() + "-AddedOnRampNode"), |id| !nc.retrieve(id).is_null());
                let newEdgeID = NBRampsComputer::get_unused_id(&(curr.get_id().to_string() + ADDED_ON_RAMP_EDGE), |id| !ec.retrieve(id, false).is_null());
                let pos = curr.get_geometry().position_at_offset(rampLength - currLength, 0.0);
                let rn: Ptr<NBNode> = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(&newNodeID, &pos, Ptr::null()))));
                nc.insert_2(rn);
                let name = curr.get_id().to_string();
                let currShift = *self.myShiftedEdges.entry(curr).or_insert(0.0);
                if !ec.split_at_2(dc, curr, rn, &newEdgeID, &name, curr.get_num_lanes() + toAdd, curr.get_num_lanes(), -1.0, 1.0, 0) {
                    write_warningf!("Could not build on-ramp for edge '%' for unknown reason", curr.get_id());
                    return;
                }
                curr = ec.retrieve(&newEdgeID, false);
                // copy shift over
                self.myShiftedEdges.insert(curr, currShift);
                incremented.insert(curr);
                last = curr;
                self.move_ramp_right(curr, toAdd);
                if wasFirst {
                    first = curr;
                }
                // mark acceleration lanes
                for i in 0..(curr.get_num_lanes() - potHighway.get_num_lanes()) {
                    curr.m().set_acceleration(i, true);
                }
            }
            if curr == cont && dontSplit && addLanes {
                write_warningf!("Could not build on-ramp for edge '%' due to option '--ramps.no-split'", curr.get_id());
                return;
            }
        } else {
            // mark acceleration lanes
            for i in 0..(firstLaneNumber - potHighway.get_num_lanes()) {
                cont.m().set_acceleration(i, true);
            }
        }
        // set connections from ramp/highway to added ramp
        if addLanes {
            if potHighway.get_step() < NBEdge_EdgeBuildingStep::LANES2LANES_USER {
                let toLane = potRamp.get_num_lanes();
                let no = (first.get_num_lanes() - potRamp.get_num_lanes()).min(potHighway.get_num_lanes());
                if !potHighway.m().add_lane2_lane_connections(0, first, toLane, no, NBEdge_Lane2LaneInfoType::VALIDATED, true, false) {
                    ramps_connection_error();
                }
            }
            if potRamp.get_step() < NBEdge_EdgeBuildingStep::LANES2LANES_USER {
                let no = potRamp.get_num_lanes();
                if !potRamp.m().add_lane2_lane_connections(0, first, 0, no, NBEdge_Lane2LaneInfoType::VALIDATED, true, false) {
                    ramps_connection_error();
                }
            }
            Self::patch_ramp_geometry(potRamp, first, potHighway, false);
        }
    }

    /// C++: `void NBRampsComputer::buildOffRamp(NBNode *, NBNodeCont &, NBEdgeCont &, NBDistrictCont &, double, bool, bool, const std::set<NBNode *, ComparatorIdLess> &)` [private]
    /// Builds an off-ramp ending at the given node
    // void
    // NBRampsComputer::buildOffRamp(NBNode* cur, NBNodeCont& nc, NBEdgeCont& ec, NBDistrictCont& dc, double rampLength, bool dontSplit, bool addLanes,
    //                               const std::set<NBNode*, ComparatorIdLess>& potOnRamps) {
    //     NBEdge* potHighway, *potRamp, *prev;
    //     getOffRampEdges(cur, &potHighway, &potRamp, &prev);
    // #ifdef DEBUG_RAMPS
    //     if (DEBUGCOND(cur)) {
    //         std::cout << "buildOffRamp cur=" << cur->getID() << " hw=" << potHighway->getID() << " ramp=" << potRamp->getID() << " prev=" << prev->getID() << "\n";
    //     }
    // #endif
    //     // compute the number of lanes to append
    //     const int firstLaneNumber = prev->getNumLanes();
    //     const int toAdd = (potRamp->getNumLanes() + potHighway->getNumLanes()) - firstLaneNumber;
    //     NBEdge* first = prev;
    //     NBEdge* last = prev;
    //     NBEdge* curr = prev;
    //     std::set<NBEdge*> incremented;
    //     if (addLanes && toAdd > 0 && std::find(incremented.begin(), incremented.end(), prev) == incremented.end()) {
    //         double currLength = 0;
    //         while (curr != nullptr && currLength + curr->getGeometry().length() - POSITION_EPS < rampLength) {
    //             if (find(incremented.begin(), incremented.end(), curr) == incremented.end()) {
    //                 curr->incLaneNo(toAdd);
    //                 // we need to distinguish between loading a .net.xml (connections are defined and applicable)
    //                 // and manual connection patches (should be post-prcess because the lane wasn't added yet)
    //                 if (curr->getStep() < NBEdge::EdgeBuildingStep::LANES2LANES_USER || !ec.hasPostProcessConnection(curr->getID())) {
    //                     curr->invalidateConnections(true);
    //                 }
    //                 incremented.insert(curr);
    //                 moveRampRight(curr, toAdd);
    //                 currLength += curr->getGeometry().length(); // !!! loaded length?
    //                 last = curr;
    //             }
    //             NBNode* prevN = curr->getFromNode();
    //             if (prevN->getIncomingEdges().size() == 1 && prevN->getOutgoingEdges().size() == 1) {
    //                 curr = prevN->getIncomingEdges()[0];
    //                 if (curr->getStep() < NBEdge::EdgeBuildingStep::LANES2LANES_USER || !ec.hasPostProcessConnection(curr->getID())) {
    //                     // curr might be an onRamp. In this case connections need to be rebuilt
    //                     curr->invalidateConnections();
    //                 }
    //                 if (curr->getNumLanes() != firstLaneNumber) {
    //                     // the number of lanes changes along the computation; we'll stop...
    //                     curr = nullptr;
    //                 } else if (last->isTurningDirectionAt(curr)) {
    //                     // turnarounds certainly should not be included in a ramp
    //                     curr = nullptr;
    //                 } else if (curr == potHighway || curr == potRamp) {
    //                     // circular connectivity. do not split!
    //                     curr = nullptr;
    //                 }
    //             } else {
    //                 // ambiguous; and, in fact, what should it be? ...stop
    //                 curr = nullptr;
    //             }
    //         }
    //         // check whether a further split is necessary
    //         if (curr != nullptr && !dontSplit && currLength + MIN_SPLIT_LENGTH < rampLength && curr->getNumLanes() == firstLaneNumber && std::find(incremented.begin(), incremented.end(), curr) == incremented.end()) {
    //             // there is enough place to build a ramp; do it
    //             bool wasFirst = first == curr;
    //             Position pos = curr->getGeometry().positionAtOffset(curr->getGeometry().length() - (rampLength  - currLength));
    //             std::string newNodeID = getUnusedID(curr->getID() + "-AddedOffRampNode", nc);
    //             std::string newEdgeID = getUnusedID(curr->getID() + "-AddedOffRampEdge", ec);
    //             NBNode* rn = new NBNode(newNodeID, pos);
    //             nc.insert(rn);
    //             std::string name = curr->getID();
    //             const double currShift = myShiftedEdges[curr];
    //             if (!ec.splitAt(dc, curr, rn, curr->getID(), newEdgeID, curr->getNumLanes(), curr->getNumLanes() + toAdd)) {
    //                 WRITE_WARNING("Could not build off-ramp for edge '"  + curr->getID() + "' for unknown reason");
    //                 return;
    //             }
    //             curr = ec.retrieve(newEdgeID);
    //             // copy shift over
    //             myShiftedEdges[curr] = currShift;
    //             incremented.insert(curr);
    //             last = curr;
    //             moveRampRight(curr, toAdd);
    //             if (wasFirst) {
    //                 first = curr;
    //             }
    //         }
    //         if (curr == prev && dontSplit && addLanes) {
    //             WRITE_WARNING("Could not build off-ramp for edge '"  + curr->getID() + "' due to option '--ramps.no-split'");
    //             return;
    //         }
    //     }
    //     NBEdge* toMark = first;
    //     toMark->markOffRamp(true);
    //     double markedLength = toMark->getLoadedLength();
    //     while (markedLength < OFFRAMP_LOOKBACK) {
    //         if (toMark != first && toMark->getToNode()->getOutgoingEdges().size() != 1) {
    //             break;
    //         }
    //         NBNode* from = toMark->getFromNode();
    //         if (from->getIncomingEdges().size() == 1) {
    //             toMark = from->getIncomingEdges()[0];
    //         } else if (potOnRamps.count(from) == 1) {
    //             NBEdge* potOnRamp, *cont;
    //             getOnRampEdges(from, &toMark, &potOnRamp, &cont);
    //         } else {
    //             break;
    //         }
    //         toMark->markOffRamp(true);
    //         markedLength += toMark->getLoadedLength();
    //     }
    //     // set connections from added ramp to ramp/highway
    //     if (addLanes) {
    //         if (first->getStep() < NBEdge::EdgeBuildingStep::LANES2LANES_USER) {
    //             if (!first->addLane2LaneConnections(potRamp->getNumLanes(), potHighway, 0, MIN2(first->getNumLanes() - 1, potHighway->getNumLanes()), NBEdge::Lane2LaneInfoType::VALIDATED, true)) {
    //                 throw ProcessError(TL("Could not set connection!"));
    //             }
    //             if (!first->addLane2LaneConnections(0, potRamp, 0, potRamp->getNumLanes(), NBEdge::Lane2LaneInfoType::VALIDATED, false)) {
    //                 throw ProcessError(TL("Could not set connection!"));
    //             }
    //         }
    //         patchRampGeometry(potRamp, first, potHighway, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_off_ramp(&mut self, cur: Ptr<NBNode>, nc: &mut NBNodeCont, ec: &mut NBEdgeCont, dc: &mut NBDistrictCont, rampLength: f64, dontSplit: bool, addLanes: bool, potOnRamps: &BTreeSet<Ptr<NBNode>>) {
        let mut potHighway: Ptr<NBEdge> = Ptr::null();
        let mut potRamp: Ptr<NBEdge> = Ptr::null();
        let mut prev: Ptr<NBEdge> = Ptr::null();
        Self::get_off_ramp_edges(cur, Ptr::from_mut(&mut potHighway), Ptr::from_mut(&mut potRamp), Ptr::from_mut(&mut prev));
        // compute the number of lanes to append
        let firstLaneNumber = prev.get_num_lanes();
        let toAdd = (potRamp.get_num_lanes() + potHighway.get_num_lanes()) - firstLaneNumber;
        let mut first = prev;
        let mut last = prev;
        let mut curr = prev;
        let mut incremented: HashSet<Ptr<NBEdge>> = HashSet::new();
        if addLanes && toAdd > 0 && !incremented.contains(&prev) {
            let mut currLength: f64 = 0.0;
            while !curr.is_null() && currLength + curr.get_geometry().length() - utils::geom::POSITION_EPS < rampLength {
                if !incremented.contains(&curr) {
                    curr.m().inc_lane_no(toAdd);
                    // we need to distinguish between loading a .net.xml (connections are defined and applicable)
                    // and manual connection patches (should be post-prcess because the lane wasn't added yet)
                    if curr.get_step() < NBEdge_EdgeBuildingStep::LANES2LANES_USER || !ec.has_post_process_connection(curr.get_id(), "") {
                        curr.m().invalidate_connections(true);
                    }
                    incremented.insert(curr);
                    self.move_ramp_right(curr, toAdd);
                    currLength += curr.get_geometry().length(); // !!! loaded length?
                    last = curr;
                }
                let prevN = curr.get_from_node();
                if prevN.get_incoming_edges().len() == 1 && prevN.get_outgoing_edges().len() == 1 {
                    curr = prevN.get_incoming_edges()[0];
                    if curr.get_step() < NBEdge_EdgeBuildingStep::LANES2LANES_USER || !ec.has_post_process_connection(curr.get_id(), "") {
                        // curr might be an onRamp. In this case connections need to be rebuilt
                        curr.m().invalidate_connections(false);
                    }
                    if curr.get_num_lanes() != firstLaneNumber {
                        // the number of lanes changes along the computation; we'll stop...
                        curr = Ptr::null();
                    } else if last.is_turning_direction_at(curr) {
                        // turnarounds certainly should not be included in a ramp
                        curr = Ptr::null();
                    } else if curr == potHighway || curr == potRamp {
                        // circular connectivity. do not split!
                        curr = Ptr::null();
                    }
                } else {
                    // ambiguous; and, in fact, what should it be? ...stop
                    curr = Ptr::null();
                }
            }
            // check whether a further split is necessary
            if !curr.is_null() && !dontSplit && currLength + MIN_SPLIT_LENGTH < rampLength && curr.get_num_lanes() == firstLaneNumber && !incremented.contains(&curr) {
                // there is enough place to build a ramp; do it
                let wasFirst = first == curr;
                let pos = curr.get_geometry().position_at_offset(curr.get_geometry().length() - (rampLength - currLength), 0.0);
                let newNodeID = NBRampsComputer::get_unused_id(&(curr.get_id().to_string() + "-AddedOffRampNode"), |id| !nc.retrieve(id).is_null());
                let newEdgeID = NBRampsComputer::get_unused_id(&(curr.get_id().to_string() + "-AddedOffRampEdge"), |id| !ec.retrieve(id, false).is_null());
                let rn: Ptr<NBNode> = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(&newNodeID, &pos, Ptr::null()))));
                nc.insert_2(rn);
                let name = curr.get_id().to_string();
                let currShift = *self.myShiftedEdges.entry(curr).or_insert(0.0);
                if !ec.split_at_2(dc, curr, rn, &name, &newEdgeID, curr.get_num_lanes(), curr.get_num_lanes() + toAdd, -1.0, 1.0, 0) {
                    write_warningf!("Could not build off-ramp for edge '%' for unknown reason", curr.get_id());
                    return;
                }
                curr = ec.retrieve(&newEdgeID, false);
                // copy shift over
                self.myShiftedEdges.insert(curr, currShift);
                incremented.insert(curr);
                last = curr;
                self.move_ramp_right(curr, toAdd);
                if wasFirst {
                    first = curr;
                }
            }
            if curr == prev && dontSplit && addLanes {
                write_warningf!("Could not build off-ramp for edge '%' due to option '--ramps.no-split'", curr.get_id());
                return;
            }
        }
        let mut toMark: Ptr<NBEdge> = first;
        toMark.m().mark_off_ramp(true);
        let mut markedLength = toMark.get_loaded_length();
        while markedLength < OFFRAMP_LOOKBACK {
            if toMark != first && toMark.get_to_node().get_outgoing_edges().len() != 1 {
                break;
            }
            let from = toMark.get_from_node();
            if from.get_incoming_edges().len() == 1 {
                toMark = from.get_incoming_edges()[0];
            } else if potOnRamps.contains(&from) {
                let mut potOnRamp: Ptr<NBEdge> = Ptr::null();
                let mut cont: Ptr<NBEdge> = Ptr::null();
                Self::get_on_ramp_edges(from, Ptr::from_mut(&mut toMark), Ptr::from_mut(&mut potOnRamp), Ptr::from_mut(&mut cont));
            } else {
                break;
            }
            toMark.m().mark_off_ramp(true);
            markedLength += toMark.get_loaded_length();
        }
        // set connections from added ramp to ramp/highway
        if addLanes {
            if first.get_step() < NBEdge_EdgeBuildingStep::LANES2LANES_USER {
                let fromLane = potRamp.get_num_lanes();
                let no = (first.get_num_lanes() - 1).min(potHighway.get_num_lanes());
                if !first.m().add_lane2_lane_connections(fromLane, potHighway, 0, no, NBEdge_Lane2LaneInfoType::VALIDATED, true, false) {
                    ramps_connection_error();
                }
                let no = potRamp.get_num_lanes();
                if !first.m().add_lane2_lane_connections(0, potRamp, 0, no, NBEdge_Lane2LaneInfoType::VALIDATED, false, false) {
                    ramps_connection_error();
                }
            }
            Self::patch_ramp_geometry(potRamp, first, potHighway, true);
        }
    }

    /// C++: `void NBRampsComputer::getOnRampEdges(NBNode *, NBEdge **, NBEdge **, NBEdge **)` [static, private]
    // void
    // NBRampsComputer::getOnRampEdges(NBNode* n, NBEdge** potHighway, NBEdge** potRamp, NBEdge** other) {
    //     *other = n->getOutgoingEdges()[0];
    //     const std::vector<NBEdge*>& edges = n->getIncomingEdges();
    //     assert(edges.size() == 2);
    //     *potHighway = edges[0];
    //     *potRamp = edges[1];
    //     /*
    //     // heuristic: highway is faster than ramp
    //     if(determinedBySpeed(potHighway, potRamp)) {
    //         return;
    //     }
    //     // heuristic: highway has more lanes than ramp
    //     if(determinedByLaneNumber(potHighway, potRamp)) {
    //         return;
    //     }
    //     */
    //     // heuristic: ramp comes from right
    //     if (NBContHelper::relative_incoming_edge_sorter(*other)(*potRamp, *potHighway)) {
    //         std::swap(*potHighway, *potRamp);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_on_ramp_edges(n: Ptr<NBNode>, potHighway: Ptr<Ptr<NBEdge>>, potRamp: Ptr<Ptr<NBEdge>>, other: Ptr<Ptr<NBEdge>>) {
        *other.m() = n.get_outgoing_edges()[0];
        let edges: EdgeVector = n.get_incoming_edges().clone();
        assert!(edges.len() == 2);
        *potHighway.m() = edges[0];
        *potRamp.m() = edges[1];
        // heuristic: ramp comes from right
        if NBContHelper_relative_incoming_edge_sorter::new(*other).call_op(*potRamp, *potHighway) {
            let t = *potHighway;
            *potHighway.m() = *potRamp;
            *potRamp.m() = t;
        }
    }

    /// C++: `void NBRampsComputer::getOffRampEdges(NBNode *, NBEdge **, NBEdge **, NBEdge **)` [static, private]
    // void
    // NBRampsComputer::getOffRampEdges(NBNode* n, NBEdge** potHighway, NBEdge** potRamp, NBEdge** other) {
    //     *other = n->getIncomingEdges()[0];
    //     const std::vector<NBEdge*>& edges = n->getOutgoingEdges();
    //     *potHighway = edges[0];
    //     *potRamp = edges[1];
    //     assert(edges.size() == 2);
    //     /*
    //     // heuristic: highway is faster than ramp
    //     if(determinedBySpeed(potHighway, potRamp)) {
    //         return;
    //     }
    //     // heuristic: highway has more lanes than ramp
    //     if(determinedByLaneNumber(potHighway, potRamp)) {
    //         return;
    //     }
    //     */
    //     // heuristic: ramp goes to right
    //     const std::vector<NBEdge*>& edges2 = n->getEdges();
    // #ifdef DEBUG_RAMPS
    //     if (DEBUGCOND(n)) {
    //         std::cout << "  edges=" << toString(edges) << " edges2=" << toString(edges2) << "\n";
    //     }
    // #endif
    //     std::vector<NBEdge*>::const_iterator i = std::find(edges2.begin(), edges2.end(), *other);
    //     NBContHelper::nextCW(edges2, i);
    //     if ((*i) == *potRamp) {
    //         std::swap(*potHighway, *potRamp);
    //     }
    //     // the following would be better but runs afoul of misleading angles when both edges
    //     // have the same geometry start point but different references lanes are
    //     // chosen for NBEdge::computeAngle()
    //     //if (NBContHelper::relative_outgoing_edge_sorter(*other)(*potHighway, *potRamp)) {
    //     //    std::swap(*potHighway, *potRamp);
    //     //}
    // }
    #[allow(unused_variables)]
    pub fn get_off_ramp_edges(n: Ptr<NBNode>, potHighway: Ptr<Ptr<NBEdge>>, potRamp: Ptr<Ptr<NBEdge>>, other: Ptr<Ptr<NBEdge>>) {
        *other.m() = n.get_incoming_edges()[0];
        let edges: EdgeVector = n.get_outgoing_edges().clone();
        *potHighway.m() = edges[0];
        *potRamp.m() = edges[1];
        assert!(edges.len() == 2);
        // heuristic: ramp goes to right
        let edges2: EdgeVector = n.get_edges().clone();
        let mut i = edges2.iter().position(|x| *x == *other).expect("other must be an edge of the node");
        NBContHelper::next_cw(&edges2, &mut i);
        if edges2[i] == *potRamp {
            let t = *potHighway;
            *potHighway.m() = *potRamp;
            *potRamp.m() = t;
        }
    }

    /// C++: `bool NBRampsComputer::determinedBySpeed(NBEdge **, NBEdge **)` [static, private]
    // bool
    // NBRampsComputer::determinedBySpeed(NBEdge** potHighway, NBEdge** potRamp) {
    //     if (fabs((*potHighway)->getSpeed() - (*potRamp)->getSpeed()) < .1) {
    //         return false;
    //     }
    //     if ((*potHighway)->getSpeed() < (*potRamp)->getSpeed()) {
    //         std::swap(*potHighway, *potRamp);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn determined_by_speed(potHighway: Ptr<Ptr<NBEdge>>, potRamp: Ptr<Ptr<NBEdge>>) -> bool {
        if (potHighway.get_speed() - potRamp.get_speed()).abs() < 0.1 {
            return false;
        }
        if potHighway.get_speed() < potRamp.get_speed() {
            let t = *potHighway;
            *potHighway.m() = *potRamp;
            *potRamp.m() = t;
        }
        true
    }

    /// C++: `bool NBRampsComputer::determinedByLaneNumber(NBEdge **, NBEdge **)` [static, private]
    // bool
    // NBRampsComputer::determinedByLaneNumber(NBEdge** potHighway, NBEdge** potRamp) {
    //     if ((*potHighway)->getNumLanes() == (*potRamp)->getNumLanes()) {
    //         return false;
    //     }
    //     if ((*potHighway)->getNumLanes() < (*potRamp)->getNumLanes()) {
    //         std::swap(*potHighway, *potRamp);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn determined_by_lane_number(potHighway: Ptr<Ptr<NBEdge>>, potRamp: Ptr<Ptr<NBEdge>>) -> bool {
        if potHighway.get_num_lanes() == potRamp.get_num_lanes() {
            return false;
        }
        if potHighway.get_num_lanes() < potRamp.get_num_lanes() {
            let t = *potHighway;
            *potHighway.m() = *potRamp;
            *potRamp.m() = t;
        }
        true
    }

    /// C++: `bool NBRampsComputer::fulfillsRampConstraints(NBEdge *, NBEdge *, NBEdge *, double, double, const std::set<std::string> &)` [static, private]
    /// Checks whether an on-/off-ramp can be bult here
    // bool
    // NBRampsComputer::fulfillsRampConstraints(
    //     NBEdge* potHighway, NBEdge* potRamp, NBEdge* other, double minHighwaySpeed, double maxRampSpeed,
    //     const std::set<std::string>& noramps) {
    //     // check modes that are not appropriate for rampsdo not build ramps on rail edges
    //     if (hasWrongMode(potHighway) || hasWrongMode(potRamp) || hasWrongMode(other)) {
    //         return false;
    //     }
    //     // do not build ramps at traffic lights
    //     if (NBNode::isTrafficLight(potRamp->getToNode()->getType())) {
    //         return false;
    //     }
    //     // do not build ramps on connectors
    //     if (potHighway->isMacroscopicConnector() || potRamp->isMacroscopicConnector() || other->isMacroscopicConnector()) {
    //         return false;
    //     }
    //     // check whether a lane is missing
    //     if (potHighway->getNumLanes() + potRamp->getNumLanes() < other->getNumLanes()) {
    //         return false;
    //     }
    //     // is it really a highway?
    //     double maxSpeed = MAX3(potHighway->getSpeed(), other->getSpeed(), potRamp->getSpeed());
    //     if (maxSpeed < minHighwaySpeed) {
    //         return false;
    //     }
    //     // is any of the connections a turnaround?
    //     if (other->getToNode() == potHighway->getFromNode()) {
    //         // off ramp
    //         if (other->isTurningDirectionAt(potHighway) ||
    //                 other->isTurningDirectionAt(potRamp)) {
    //             return false;
    //         }
    //     } else {
    //         // on ramp
    //         if (other->isTurningDirectionAt(potHighway) ||
    //                 other->isTurningDirectionAt(potRamp)) {
    //             return false;
    //         }
    //     }
    //     // are the angles between highway and other / ramp and other more or less straight?
    //     const NBNode* node = ((potHighway->getToNode() == potRamp->getToNode() && potHighway->getToNode() == other->getFromNode())
    //                           ? potHighway->getToNode() : potHighway->getFromNode());
    //     double angle = fabs(NBHelpers::relAngle(potHighway->getAngleAtNode(node), other->getAngleAtNode(node)));
    //     if (angle >= 60) {
    //         return false;
    //     }
    //     angle = fabs(NBHelpers::relAngle(potRamp->getAngleAtNode(node), other->getAngleAtNode(node)));
    //     if (angle >= 60) {
    //         return false;
    //     }
    //     /*
    //     if (potHighway->getSpeed() < minHighwaySpeed || other->getSpeed() < minHighwaySpeed) {
    //         return false;
    //     }
    //     */
    //     // is it really a ramp?
    //     if (maxRampSpeed > 0 && maxRampSpeed < potRamp->getSpeed()) {
    //         return false;
    //     }
    //     if (noramps.find(other->getID()) != noramps.end()) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn fulfills_ramp_constraints(potHighway: Ptr<NBEdge>, potRamp: Ptr<NBEdge>, other: Ptr<NBEdge>, minHighwaySpeed: f64, maxRampSpeed: f64, noramps: &BTreeSet<String>) -> bool {
        // check modes that are not appropriate for ramps: do not build ramps on rail edges
        if Self::has_wrong_mode(potHighway) || Self::has_wrong_mode(potRamp) || Self::has_wrong_mode(other) {
            return false;
        }
        // do not build ramps at traffic lights
        if NBNode::is_traffic_light(potRamp.get_to_node().get_type()) {
            return false;
        }
        // do not build ramps on connectors
        if potHighway.is_macroscopic_connector() || potRamp.is_macroscopic_connector() || other.is_macroscopic_connector() {
            return false;
        }
        // check whether a lane is missing
        if potHighway.get_num_lanes() + potRamp.get_num_lanes() < other.get_num_lanes() {
            return false;
        }
        // is it really a highway?
        let maxSpeed = f64::max(f64::max(potHighway.get_speed(), other.get_speed()), potRamp.get_speed());
        if maxSpeed < minHighwaySpeed {
            return false;
        }
        // is any of the connections a turnaround?
        if other.get_to_node() == potHighway.get_from_node() {
            // off ramp
            if other.is_turning_direction_at(potHighway) || other.is_turning_direction_at(potRamp) {
                return false;
            }
        } else {
            // on ramp
            if other.is_turning_direction_at(potHighway) || other.is_turning_direction_at(potRamp) {
                return false;
            }
        }
        // are the angles between highway and other / ramp and other more or less straight?
        let node: Ptr<NBNode> = if potHighway.get_to_node() == potRamp.get_to_node() && potHighway.get_to_node() == other.get_from_node() {
            potHighway.get_to_node()
        } else {
            potHighway.get_from_node()
        };
        let mut angle = NBHelpers::rel_angle(potHighway.get_angle_at_node(node), other.get_angle_at_node(node)).abs();
        if angle >= 60.0 {
            return false;
        }
        angle = NBHelpers::rel_angle(potRamp.get_angle_at_node(node), other.get_angle_at_node(node)).abs();
        if angle >= 60.0 {
            return false;
        }
        // is it really a ramp?
        if maxRampSpeed > 0.0 && maxRampSpeed < potRamp.get_speed() {
            return false;
        }
        if noramps.contains(other.get_id()) {
            return false;
        }
        true
    }

    /// C++: `void NBRampsComputer::moveRampRight(NBEdge *, int)` [private]
    /// Moves the ramp to the right, as new lanes were added
    // void
    // NBRampsComputer::moveRampRight(NBEdge* ramp, int addedLanes) {
    //     if (ramp->getLaneSpreadFunction() != LaneSpreadFunction::CENTER) {
    //         return;
    //     }
    //     try {
    //         PositionVector g = ramp->getGeometry();
    //         double offset = (0.5 * addedLanes *
    //                          (ramp->getLaneWidth() == NBEdge::UNSPECIFIED_WIDTH ? SUMO_const_laneWidth : ramp->getLaneWidth()));
    //         if (myShiftedEdges.count(ramp) != 0) {
    //             offset -= myShiftedEdges[ramp];
    //         }
    //         g.move2side(offset);
    //         ramp->setGeometry(g);
    //         myShiftedEdges[ramp] = offset;
    //     } catch (InvalidArgument&) {
    //         WRITE_WARNINGF(TL("For edge '%': could not compute shape."), ramp->getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_ramp_right(&mut self, ramp: Ptr<NBEdge>, addedLanes: i32) {
        if ramp.get_lane_spread_function() != LaneSpreadFunction::CENTER {
            return;
        }
        let mut g: PositionVector = ramp.get_geometry().clone();
        let mut offset = 0.5 * (addedLanes as f64) * (if ramp.get_lane_width() == NBEDGE_UNSPECIFIED_WIDTH { SUMO_CONST_LANE_WIDTH } else { ramp.get_lane_width() });
        if self.myShiftedEdges.contains_key(&ramp) {
            offset -= self.myShiftedEdges[&ramp];
        }
        // shortcut: Rust move2side cannot signal InvalidArgument, so the C++ catch warning is not emitted
        g.move2side(offset, 100.0);
        ramp.m().set_geometry(&g, false);
        self.myShiftedEdges.insert(ramp, offset);
    }

    /// C++: `bool NBRampsComputer::hasWrongMode(NBEdge *)` [static, private]
    /// whether the edge has a mode that does not indicate a ramp edge
    // bool
    // NBRampsComputer::hasWrongMode(NBEdge* edge) {
    //     // must allow passenger vehicles
    //     if ((edge->getPermissions() & SVC_PASSENGER) == 0) {
    //         return true;
    //     }
    //     // must not have a green verge or a lane that is only for soft modes
    //     for (int i = 0; i < (int)edge->getNumLanes(); ++i) {
    //         if ((edge->getPermissions(i) & ~(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_wrong_mode(edge: Ptr<NBEdge>) -> bool {
        // must allow passenger vehicles
        if (edge.get_permissions(-1) & SVC_PASSENGER) == 0 {
            return true;
        }
        // must not have a green verge or a lane that is only for soft modes
        for i in 0..edge.get_num_lanes() {
            if (edge.get_permissions(i) & !(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0 {
                return true;
            }
        }
        false
    }

    /// C++: `void NBRampsComputer::patchRampGeometry(NBEdge *, NBEdge *, NBEdge *, bool)` [static, private]
    /// shift ramp geometry to merge smoothly with the motorway
    // void
    // NBRampsComputer::patchRampGeometry(NBEdge* potRamp, NBEdge* first, NBEdge* potHighway, bool onRamp) {
    //     // geometry of first and highway should allign on the left side
    //     if (first->getLaneSpreadFunction() == LaneSpreadFunction::CENTER && first->hasDefaultGeometryEndpoints()) {
    //         const NBNode* n = onRamp ? potHighway->getToNode() : potHighway->getFromNode();
    //         if (potHighway->hasDefaultGeometryEndpointAtNode(n)) {
    //             PositionVector p2 = first->getGeometry();
    //             try {
    //                 p2.move2side((first->getNumLanes() - potHighway->getNumLanes()) * first->getLaneWidth(0) * 0.5);
    //                 first->setGeometry(p2);
    //             } catch (InvalidArgument&) {}
    //         }
    //     }
    //
    //     // ramp should merge smoothly with first
    //     PositionVector p = potRamp->getGeometry();
    //     double offset = 0;
    //     int firstIndex = MAX2(0, MIN2(potRamp->getNumLanes(), first->getNumLanes()) - 1);
    //     if (potRamp->getLaneSpreadFunction() == LaneSpreadFunction::RIGHT) {
    //         offset = -first->getLaneWidth(firstIndex) / 2;
    //     } else {
    //         if (firstIndex % 2 == 1) {
    //             // even number of lanes
    //             offset = -first->getLaneWidth(firstIndex / 2) / 2;
    //         }
    //         firstIndex /= 2; // integer division
    //     }
    //     // reset lane shape (might be affected by earlier junctions.join step. see #947)
    //     first->resetLaneShapes();
    //     PositionVector l = first->getLaneShape(firstIndex);
    //     try {
    //         l.move2side(offset);
    //     } catch (InvalidArgument&) {}
    //     //std::cout << " ramp=" << potRamp->getID() << " firstIndex=" << firstIndex << " offset=" << offset << " l=" << l << "\n";
    //
    //     if (onRamp) {
    //         p[0] = l[-1];
    //     } else {
    //         p.pop_back();
    //         p.push_back(l[0]);
    //     }
    //     potRamp->setGeometry(p);
    //
    // }
    #[allow(unused_variables)]
    pub fn patch_ramp_geometry(potRamp: Ptr<NBEdge>, first: Ptr<NBEdge>, potHighway: Ptr<NBEdge>, onRamp: bool) {
        // geometry of first and highway should allign on the left side
        if first.get_lane_spread_function() == LaneSpreadFunction::CENTER && first.has_default_geometry_endpoints() {
            let n = if onRamp { potHighway.get_to_node() } else { potHighway.get_from_node() };
            if potHighway.has_default_geometry_endpoint_at_node(n) {
                let mut p2: PositionVector = first.get_geometry().clone();
                // shortcut: Rust move2side cannot signal InvalidArgument, so the C++ empty catch is not needed here
                p2.move2side((first.get_num_lanes() - potHighway.get_num_lanes()) as f64 * first.get_lane_width_2(0) * 0.5, 100.0);
                first.m().set_geometry(&p2, false);
            }
        }

        // ramp should merge smoothly with first
        let mut p: PositionVector = potRamp.get_geometry().clone();
        let mut offset = 0.0;
        let mut firstIndex: i32 = std::cmp::max(0, std::cmp::min(potRamp.get_num_lanes(), first.get_num_lanes()) - 1);
        if potRamp.get_lane_spread_function() == LaneSpreadFunction::RIGHT {
            offset = -first.get_lane_width_2(firstIndex) / 2.0;
        } else {
            if firstIndex % 2 == 1 {
                // even number of lanes
                offset = -first.get_lane_width_2(firstIndex / 2) / 2.0;
            }
            firstIndex /= 2; // integer division
        }
        // reset lane shape (might be affected by earlier junctions.join step. see #947)
        first.m().reset_lane_shapes();
        let mut l: PositionVector = first.get_lane_shape(firstIndex).clone();
        l.move2side(offset, 100.0);

        if onRamp {
            *p.at_mut(0) = *l.at(-1);
        } else {
            p.0.pop();
            p.0.push(*l.at(0));
        }
        potRamp.m().set_geometry(&p, false);
    }

    /// C++: `std::string NBRampsComputer::getUnusedID(const std::string &, const T &)` [static, private]
    // template <class T>
    //     static std::string getUnusedID(const std::string& prefix, const T& objectCont) {
    //         if (objectCont.retrieve(prefix) == nullptr) {
    //             return prefix;
    //         }
    //         int i = 0;
    //         std::string result = prefix + "#" + toString(i);
    //         while (objectCont.retrieve(result) != nullptr) {
    //             i++;
    //             result = prefix + "#" + toString(i);
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn get_unused_id(prefix: &str, exists: impl Fn(&str) -> bool) -> String {
        if !exists(prefix) {
            return prefix.to_string();
        }
        let mut i = 0;
        let mut result = format!("{}#{}", prefix, i);
        while exists(&result) {
            i += 1;
            result = format!("{}#{}", prefix, i);
        }
        result
    }

}

