//! Port of netbuild/NBAlgorithms_Railway.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterEdgeInfo};
use utils::common::msg_handler::{write_message, write_warning};
use utils::common::string_utils::{is_int, to_bool, to_int};
use utils::geom::geom_helper::{angle_diff, deg2rad, rad2deg};
use utils::options::options_cont::{exists, get_bool, get_float, get_string};

// C++ global debug switch, false by default
const gDebugFlag1: bool = false;
// C++ NBTrafficLightDefinition::OSM_DIRECTION
const OSM_DIRECTION: &str = "osm:direction";
// C++ #define SHARP_THRESHOLD_SAMEDIR 100 / SHARP_THRESHOLD 80
const SHARP_THRESHOLD_SAMEDIR: f64 = 100.0;
const SHARP_THRESHOLD: f64 = 80.0;

// C++ MIN2 / MAX2 macros
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

// C++ `new Track(...)`; the caller frees it with Box::from_raw when the C++ code does `delete t`
fn leak_track(t: NBRailwayTopologyAnalyzer_Track) -> Ptr<NBRailwayTopologyAnalyzer_Track> {
    Ptr::from_raw(Box::into_raw(Box::new(t)))
}

/// C++ class `NBRailwayGeometryHelper` (netbuild/NBAlgorithms_Railway.h)
pub struct NBRailwayGeometryHelper {
}

impl NBRailwayGeometryHelper {
    /// C++: `int NBRailwayGeometryHelper::straigthenCorrdidor(NBEdgeCont &, double)` [static]
    // int
    // NBRailwayGeometryHelper::straigthenCorrdidor(NBEdgeCont& ec, double maxAngle) {
    //     int moved = 0;
    //     int numCorridors = 0;
    //     std::set<NBNode*> railNodes = NBRailwayTopologyAnalyzer::getRailNodes(ec);
    //     std::set<NBNode*> railGeomNodes;
    //     for (NBNode* n : railNodes) {
    //         if (n->geometryLike()) {
    //             railGeomNodes.insert(n);
    //         }
    //     }
    //     std::set<NBNode*, ComparatorIdLess> kinkNodes;;
    //     for (NBNode* n : railGeomNodes) {
    //         NBEdge* in = n->getIncomingEdges().front();
    //         NBEdge* out = n->getOutgoingEdges().size() == 1 || n->getOutgoingEdges()[1]->isTurningDirectionAt(in) ? n->getOutgoingEdges().front() : n->getOutgoingEdges().back();
    //         const double relAngle = fabs(RAD2DEG(GeomHelper::angleDiff(DEG2RAD(in->getAngleAtNode(n)), DEG2RAD(out->getAngleAtNode(n)))));
    //         if (maxAngle > 0 && relAngle > maxAngle) {
    //             kinkNodes.insert(n);
    //         }
    //     }
    //     while (!kinkNodes.empty()) {
    //         std::vector<NBNode*> corridor;
    //         std::vector<NBEdge*> corridorEdges;
    //         Boundary corridorBox;
    //         double length = 0;
    //         NBNode* n = *kinkNodes.begin();
    //         kinkNodes.erase(kinkNodes.begin());
    //         // go downstream and upstream, add kinkNodes until a "long" enough
    //         // non-kink stretch is found
    //         NBEdge* in = n->getIncomingEdges().front();
    //         NBEdge* out = n->getOutgoingEdges().size() == 1 || n->getOutgoingEdges()[1]->isTurningDirectionAt(in) ? n->getOutgoingEdges().front() : n->getOutgoingEdges().back();
    //         NBEdge* const centerIn = in;
    //         NBEdge* const centerOut = out;
    //         NBNode* up = in->getFromNode();
    //         NBNode* down = out->getToNode();
    //         corridor.push_back(up);
    //         corridor.push_back(n);
    //         corridor.push_back(down);
    //         corridorBox.add(up->getPosition());
    //         corridorBox.add(down->getPosition());
    //         corridorEdges.push_back(in);
    //         corridorEdges.push_back(out);
    //         length += in->getLoadedLength();
    //         length += out->getLoadedLength();
    //         Position cBeg, cEnd, delta;
    //         while (kinkNodes.count(up) != 0) {
    //             NBEdge* const out2 = in;
    //             NBEdge* const in2 = up->getIncomingEdges().size() == 1 || up->getIncomingEdges()[1]->isTurningDirectionAt(out2) ? up->getIncomingEdges().front() : up->getIncomingEdges().back();
    //             length += in2->getLoadedLength();
    //             up = in2->getFromNode();
    //             corridor.insert(corridor.begin(), up);
    //             corridorEdges.insert(corridorEdges.begin(), in2);
    //             kinkNodes.erase(up);
    //             corridorBox.add(up->getPosition());
    //         }
    //         cBeg = up->getPosition();
    //         cEnd = down->getPosition();
    //         delta = cEnd - cBeg;
    //         while (delta.length2D() <= POSITION_EPS * (double)corridor.size() && railGeomNodes.count(up) != 0) {
    //             NBEdge* const out2 = in;
    //             NBEdge* const in2 = up->getIncomingEdges().size() == 1 || up->getIncomingEdges()[1]->isTurningDirectionAt(out2) ? up->getIncomingEdges().front() : up->getIncomingEdges().back();
    //             length += in2->getLoadedLength();
    //             up = in2->getFromNode();
    //             corridor.insert(corridor.begin(), up);
    //             corridorEdges.insert(corridorEdges.begin(), in2);
    //             kinkNodes.erase(up);
    //             corridorBox.add(up->getPosition());
    //             cBeg = up->getPosition();
    //             cEnd = down->getPosition();
    //             delta = cEnd - cBeg;
    //         }
    //         in = centerIn;
    //         out = centerOut;
    //         while (kinkNodes.count(down) != 0) {
    //             NBEdge* const in2 = out;
    //             NBEdge* const out2 = down->getOutgoingEdges().size() == 1 || down->getOutgoingEdges()[1]->isTurningDirectionAt(in2) ? down->getOutgoingEdges().front() : down->getOutgoingEdges().back();
    //             down = out2->getToNode();
    //             length += out2->getLoadedLength();
    //             corridor.push_back(down);
    //             corridorEdges.push_back(out2);
    //             kinkNodes.erase(down);
    //             corridorBox.add(down->getPosition());
    //         }
    //         cBeg = up->getPosition();
    //         cEnd = down->getPosition();
    //         delta = cEnd - cBeg;
    //         while (delta.length2D() <= POSITION_EPS * (double)corridor.size() && railGeomNodes.count(down) != 0) {
    //             NBEdge* const in2 = out;
    //             NBEdge* const out2 = down->getOutgoingEdges().size() == 1 || down->getOutgoingEdges()[1]->isTurningDirectionAt(in2) ? down->getOutgoingEdges().front() : down->getOutgoingEdges().back();
    //             down = out2->getToNode();
    //             length += out2->getLoadedLength();
    //             corridor.push_back(down);
    //             corridorEdges.push_back(out2);
    //             kinkNodes.erase(down);
    //             corridorBox.add(down->getPosition());
    //             cBeg = up->getPosition();
    //             cEnd = down->getPosition();
    //             delta = cEnd - cBeg;
    //         }
    //         // straighten all edges in corridor (corridorEdges doesn't include bidi)
    //         std::set<NBNode*> corridorNodes(corridor.begin(), corridor.end());
    //         for (NBNode* n2 : corridorNodes) {
    //             for (NBEdge* e : n2->getEdges()) {
    //                 if (corridorNodes.count(e->getFromNode()) != 0
    //                         && corridorNodes.count(e->getToNode()) != 0) {
    //                     PositionVector simpleGeom;
    //                     simpleGeom.push_back(e->getFromNode()->getPosition());
    //                     simpleGeom.push_back(e->getToNode()->getPosition());
    //                     e->setGeometry(simpleGeom);
    //                 }
    //             }
    //         }
    //         if (delta.length2D() > 0) {
    //             double currLength = 0;
    //             for (int i = 1; i < (int)corridor.size() - 1; i++) {
    //                 currLength += corridorEdges[i - 1]->getLoadedLength();
    //                 const Position newPos = cBeg + delta * (currLength / length);
    //                 NBNode* const n2 = corridor[i];
    //                 n2->reinit(newPos, n2->getType());
    //                 for (NBEdge* e : n2->getEdges()) {
    //                     e->resetEndpointAtNode(n2);
    //                 }
    //                 moved += 1;
    //             }
    //             numCorridors += 1;
    //         } else {
    //             WRITE_WARNINGF(TL("Could not straighten corridor %."), toString(corridor));
    //         }
    //     }
    //     //std::cout << " railNodes=" << railNodes.size() << " railGeomNodes=" << railGeomNodes.size() << " kinkNodes=" << kinkNodes.size() << "\n";
    //
    //     WRITE_MESSAGEF(TL("Moved % rail junctions for straightening % corridors."), moved, numCorridors);
    //     return moved;
    // }
    #[allow(unused_variables)]
    pub fn straigthen_corrdidor(ec: &mut NBEdgeCont, maxAngle: f64) -> i32 {
        let mut moved = 0;
        let mut numCorridors = 0;
        let railNodes = NBRailwayTopologyAnalyzer::get_rail_nodes(ec, false);
        let mut railGeomNodes: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        for n in railNodes.iter() {
            if n.geometry_like() {
                railGeomNodes.insert(*n);
            }
        }
        // determinism: std::set<NBNode*, ComparatorIdLess> is ordered by node id, so kinkNodes is keyed by id
        let mut kinkNodes: BTreeMap<String, Ptr<NBNode>> = BTreeMap::new();
        for n in railGeomNodes.iter() {
            let in_ = n.get_incoming_edges()[0];
            let out = if n.get_outgoing_edges().len() == 1 || n.get_outgoing_edges()[1].is_turning_direction_at(in_) {
                n.get_outgoing_edges()[0]
            } else {
                *n.get_outgoing_edges().last().unwrap()
            };
            let relAngle = rad2deg(angle_diff(deg2rad(in_.get_angle_at_node(*n)), deg2rad(out.get_angle_at_node(*n)))).abs();
            if maxAngle > 0.0 && relAngle > maxAngle {
                kinkNodes.insert(n.get_id().to_string(), *n);
            }
        }
        while !kinkNodes.is_empty() {
            let mut corridor: Vec<Ptr<NBNode>> = Vec::new();
            let mut corridorEdges: Vec<Ptr<NBEdge>> = Vec::new();
            let mut corridorBox = Boundary::default();
            let mut length: f64 = 0.0;
            let (_, n) = kinkNodes.pop_first().unwrap();
            // go downstream and upstream, add kinkNodes until a "long" enough
            // non-kink stretch is found
            let mut in_ = n.get_incoming_edges()[0];
            let mut out = if n.get_outgoing_edges().len() == 1 || n.get_outgoing_edges()[1].is_turning_direction_at(in_) {
                n.get_outgoing_edges()[0]
            } else {
                *n.get_outgoing_edges().last().unwrap()
            };
            let centerIn = in_;
            let centerOut = out;
            let mut up = in_.get_from_node();
            let mut down = out.get_to_node();
            corridor.push(up);
            corridor.push(n);
            corridor.push(down);
            corridorBox.add_pos(up.get_position());
            corridorBox.add_pos(down.get_position());
            corridorEdges.push(in_);
            corridorEdges.push(out);
            length += in_.get_loaded_length();
            length += out.get_loaded_length();
            let mut cBeg = Position::default();
            let mut cEnd = Position::default();
            let mut delta = Position::default();
            while kinkNodes.contains_key(up.get_id()) {
                let out2 = in_;
                let in2 = if up.get_incoming_edges().len() == 1 || up.get_incoming_edges()[1].is_turning_direction_at(out2) {
                    up.get_incoming_edges()[0]
                } else {
                    *up.get_incoming_edges().last().unwrap()
                };
                length += in2.get_loaded_length();
                up = in2.get_from_node();
                corridor.insert(0, up);
                corridorEdges.insert(0, in2);
                kinkNodes.remove(up.get_id());
                corridorBox.add_pos(up.get_position());
            }
            cBeg = *up.get_position();
            cEnd = *down.get_position();
            delta = cEnd - cBeg;
            while delta.length2d() <= utils::geom::POSITION_EPS * (corridor.len() as f64) && railGeomNodes.contains(&up) {
                let out2 = in_;
                let in2 = if up.get_incoming_edges().len() == 1 || up.get_incoming_edges()[1].is_turning_direction_at(out2) {
                    up.get_incoming_edges()[0]
                } else {
                    *up.get_incoming_edges().last().unwrap()
                };
                length += in2.get_loaded_length();
                up = in2.get_from_node();
                corridor.insert(0, up);
                corridorEdges.insert(0, in2);
                kinkNodes.remove(up.get_id());
                corridorBox.add_pos(up.get_position());
                cBeg = *up.get_position();
                cEnd = *down.get_position();
                delta = cEnd - cBeg;
            }
            in_ = centerIn;
            out = centerOut;
            while kinkNodes.contains_key(down.get_id()) {
                let in2 = out;
                let out2 = if down.get_outgoing_edges().len() == 1 || down.get_outgoing_edges()[1].is_turning_direction_at(in2) {
                    down.get_outgoing_edges()[0]
                } else {
                    *down.get_outgoing_edges().last().unwrap()
                };
                down = out2.get_to_node();
                length += out2.get_loaded_length();
                corridor.push(down);
                corridorEdges.push(out2);
                kinkNodes.remove(down.get_id());
                corridorBox.add_pos(down.get_position());
            }
            cBeg = *up.get_position();
            cEnd = *down.get_position();
            delta = cEnd - cBeg;
            while delta.length2d() <= utils::geom::POSITION_EPS * (corridor.len() as f64) && railGeomNodes.contains(&down) {
                let in2 = out;
                let out2 = if down.get_outgoing_edges().len() == 1 || down.get_outgoing_edges()[1].is_turning_direction_at(in2) {
                    down.get_outgoing_edges()[0]
                } else {
                    *down.get_outgoing_edges().last().unwrap()
                };
                down = out2.get_to_node();
                length += out2.get_loaded_length();
                corridor.push(down);
                corridorEdges.push(out2);
                kinkNodes.remove(down.get_id());
                corridorBox.add_pos(down.get_position());
                cBeg = *up.get_position();
                cEnd = *down.get_position();
                delta = cEnd - cBeg;
            }
            // straighten all edges in corridor (corridorEdges doesn't include bidi)
            let corridorNodes: BTreeSet<Ptr<NBNode>> = corridor.iter().copied().collect();
            for n2 in corridorNodes.iter() {
                for e in n2.get_edges().iter() {
                    if corridorNodes.contains(&e.get_from_node()) && corridorNodes.contains(&e.get_to_node()) {
                        let simpleGeom = PositionVector::from(vec![*e.get_from_node().get_position(), *e.get_to_node().get_position()]);
                        e.m().set_geometry(&simpleGeom, false);
                    }
                }
            }
            if delta.length2d() > 0.0 {
                let mut currLength = 0.0;
                for i in 1..corridor.len() - 1 {
                    currLength += corridorEdges[i - 1].get_loaded_length();
                    let newPos = cBeg + delta * (currLength / length);
                    let n2 = corridor[i];
                    n2.m().reinit(&newPos, n2.get_type(), false);
                    for e in n2.get_edges().iter() {
                        e.m().reset_endpoint_at_node(n2);
                    }
                    moved += 1;
                }
                numCorridors += 1;
            } else {
                write_warningf!("Could not straighten corridor %.", corridor.iter().map(|n| n.get_id().to_string()).collect::<Vec<_>>().join(" "));
            }
        }
        //std::cout << " railNodes=" << railNodes.size() << " railGeomNodes=" << railGeomNodes.size() << " kinkNodes=" << kinkNodes.size() << "\n";

        write_messagef!("Moved % rail junctions for straightening % corridors.", moved, numCorridors);
        moved
    }

}

/// C++ class `NBRailwaySignalGuesser` (netbuild/NBAlgorithms_Railway.h)
pub struct NBRailwaySignalGuesser {
}

impl NBRailwaySignalGuesser {
    /// C++: `int NBRailwaySignalGuesser::guessRailSignals(NBNodeCont &, NBEdgeCont &, NBPTStopCont &, NBDistrictCont &)` [static]
    // int
    // NBRailwaySignalGuesser::guessRailSignals(NBNodeCont& nc, NBEdgeCont& ec, NBPTStopCont& sc, NBDistrictCont& dc) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     int addedSignals = 0;
    //     if (oc.exists("railway.signal.guess.by-stops")) {
    //         if (oc.getBool("railway.signal.guess.by-stops")) {
    //             NBTurningDirectionsComputer::computeTurnDirections(nc, false);
    //             const double minLength = oc.getFloat("osm.stop-output.length.train");
    //             const bool split = oc.getBool("railway.signal.guess.by-stops.split");
    //             if (split) {
    //                 // needed for finding and splitting bidi-edges
    //                 NBTurningDirectionsComputer::computeTurnDirections(nc, false);
    //             }
    //             addedSignals += guessByStops(nc, ec, sc, dc, minLength, split);
    //         }
    //     }
    //     return addedSignals;
    // }
    #[allow(unused_variables)]
    pub fn guess_rail_signals(nc: &mut NBNodeCont, ec: &mut NBEdgeCont, sc: &mut NBPTStopCont, dc: &mut NBDistrictCont) -> i32 {
        let mut addedSignals = 0;
        if exists("railway.signal.guess.by-stops") {
            if get_bool("railway.signal.guess.by-stops") {
                NBTurningDirectionsComputer::compute_turn_directions(nc, false);
                let minLength = get_float("osm.stop-output.length.train");
                let split = get_bool("railway.signal.guess.by-stops.split");
                if split {
                    // needed for finding and splitting bidi-edges
                    NBTurningDirectionsComputer::compute_turn_directions(nc, false);
                }
                addedSignals += Self::guess_by_stops(nc, ec, sc, dc, minLength, split);
            }
        }
        addedSignals
    }

    /// C++: `int NBRailwaySignalGuesser::guessByStops(NBNodeCont &, NBEdgeCont &, NBPTStopCont &, NBDistrictCont &, double, bool)` [static, private]
    // int
    // NBRailwaySignalGuesser::guessByStops(NBNodeCont& nc, NBEdgeCont& ec, NBPTStopCont& sc, NBDistrictCont& dc, double minLength, bool split) {
    //     const double splitOffset = 10;
    //     const double splitOffsetSlack = 200;
    //     int addedSignals = 0;
    //     for (const auto& item : sc.getStops()) {
    //         NBEdge* stopEdge = ec.retrieve(item.second->getEdgeId());
    //         if (stopEdge != nullptr && isRailway(stopEdge->getPermissions())) {
    //             NBNode* to = stopEdge->getToNode();
    //             if (canBeSignal(to)) {
    //                 const double stopEnd = item.second->getEndPos();
    //                 if (split && (!to->geometryLike() || stopEdge->getLength() - stopEnd > splitOffsetSlack)) {
    //                     const double splitPos = MIN2(stopEnd + 1, MAX2(stopEdge->getLength() - splitOffset, stopEdge->getLength() / 2));
    //                     const std::string nodeID = nc.createUnusedID(stopEdge->getID() + "_" + item.first, "#");
    //                     NBNode* n = new NBNode(nodeID, stopEdge->getGeometry().positionAtOffset(splitPos), SumoXMLNodeType::RAIL_SIGNAL);
    //                     nc.insert(n);
    //                     const std::string edgeID = ec.createUnusedID(stopEdge->getID() + "." + toString(int(splitPos)), "#");
    //                     const std::string stopEdgeID = stopEdge->getID();
    //                     NBEdge* bidi = const_cast<NBEdge*>(stopEdge->getBidiEdge());
    //                     if (ec.splitAt(dc, stopEdge, n, stopEdgeID, edgeID, stopEdge->getNumLanes(), stopEdge->getNumLanes())) {
    //                         if (bidi != nullptr) {
    //                             const std::string edgeID2 = ec.createUnusedID(bidi->getID() + "." + toString(int(splitPos)), "#");
    //                             ec.splitAt(dc, bidi, n, edgeID2, bidi->getID(), bidi->getNumLanes(), bidi->getNumLanes());
    //                         }
    //                         addedSignals++;
    //                         item.second->findLaneAndComputeBusStopExtent(stopEdge);
    //                         stopEdge = ec.retrieve(stopEdgeID);
    //                         NBTurningDirectionsComputer::computeTurnDirectionsForNode(stopEdge->getFromNode(), false);
    //                         NBTurningDirectionsComputer::computeTurnDirectionsForNode(stopEdge->getToNode(), false);
    //                     }
    //                 } else {
    //                     to->reinit(to->getPosition(), SumoXMLNodeType::RAIL_SIGNAL);
    //                     addedSignals++;
    //                 }
    //             }
    //             NBNode* from = stopEdge->getFromNode();
    //             NBEdge* curr = stopEdge;
    //             double searchDist = minLength - stopEdge->getLoadedLength();
    //             while (searchDist > 0 && from->geometryLike() && from->getType() != SumoXMLNodeType::RAIL_SIGNAL) {
    //                 for (NBEdge* in : from->getIncomingEdges()) {
    //                     if (in->getFromNode() != curr->getToNode()) {
    //                         // found edge that isn't a bidi predecessor
    //                         curr = in;
    //                         break;
    //                     }
    //                 }
    //                 if (curr->getFromNode() == from) {
    //                     // bidi edge without predecessor
    //                     break;
    //                 } else {
    //                     from = curr->getFromNode();
    //                 }
    //                 searchDist -= curr->getLoadedLength();
    //             }
    //             if (canBeSignal(from)) {
    //                 const double minPos = MIN2(splitOffset, curr->getLength() / 2);
    //                 const double splitPos = searchDist >= 0
    //                                         ? minPos
    //                                         : MAX2(minPos, MIN2(item.second->getStartPos(), -searchDist - 10));
    //                 if (split && (!from->geometryLike() || splitPos > splitOffsetSlack)) {
    //                     const std::string nodeID = nc.createUnusedID(curr->getID() + "_" + item.first, "#");
    //                     NBNode* n = new NBNode(nodeID, curr->getGeometry().positionAtOffset(splitPos), SumoXMLNodeType::RAIL_SIGNAL);
    //                     nc.insert(n);
    //                     const std::string edgeID = ec.createUnusedID(curr->getID() + "." + toString(int(splitPos)), "#");
    //                     const std::string currID = curr->getID();
    //                     NBEdge* bidi = const_cast<NBEdge*>(curr->getBidiEdge());
    //                     if (ec.splitAt(dc, curr, n, edgeID, curr->getID(), curr->getNumLanes(), curr->getNumLanes())) {
    //                         if (bidi != nullptr) {
    //                             const std::string edgeID2 = ec.createUnusedID(bidi->getID() + "." + toString(int(splitPos)), "#");
    //                             ec.splitAt(dc, bidi, n, bidi->getID(), edgeID2, bidi->getNumLanes(), bidi->getNumLanes());
    //                         }
    //                         addedSignals++;
    //                         item.second->findLaneAndComputeBusStopExtent(stopEdge);
    //                         curr = ec.retrieve(currID);
    //                         NBTurningDirectionsComputer::computeTurnDirectionsForNode(curr->getFromNode(), false);
    //                         NBTurningDirectionsComputer::computeTurnDirectionsForNode(curr->getToNode(), false);
    //                     }
    //                 } else {
    //                     from->reinit(from->getPosition(), SumoXMLNodeType::RAIL_SIGNAL);
    //                     addedSignals++;
    //                 }
    //             }
    //         }
    //     }
    //     WRITE_MESSAGEF(TL("Added % rail signals at % stops."), addedSignals, sc.getStops().size());
    //     return addedSignals;
    // }
    #[allow(unused_variables)]
    pub fn guess_by_stops(nc: &mut NBNodeCont, ec: &mut NBEdgeCont, sc: &mut NBPTStopCont, dc: &mut NBDistrictCont, minLength: f64, split: bool) -> i32 {
        let splitOffset = 10.0;
        let splitOffsetSlack = 200.0;
        let mut addedSignals = 0;
        for (itemId, item) in sc.get_stops().iter() {
            let mut stopEdge = ec.retrieve(item.get_edge_id(), false);
            if !stopEdge.is_null() && is_railway(stopEdge.get_permissions(-1)) {
                let to = stopEdge.get_to_node();
                if Self::can_be_signal(to) {
                    let stopEnd = item.get_end_pos();
                    if split && (!to.geometry_like() || stopEdge.get_length() - stopEnd > splitOffsetSlack) {
                        let splitPos = min2(stopEnd + 1.0, max2(stopEdge.get_length() - splitOffset, stopEdge.get_length() / 2.0));
                        let nodeID = nc.create_unused_id(&format!("{}_{}", stopEdge.get_id(), itemId), "#");
                        let n = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(&nodeID, &stopEdge.get_geometry().position_at_offset(splitPos, 0.0), SumoXMLNodeType::RAIL_SIGNAL))));
                        nc.insert_2(n);
                        let edgeID = ec.create_unused_id(&format!("{}.{}", stopEdge.get_id(), splitPos as i32), "#");
                        let stopEdgeID = stopEdge.get_id().to_string();
                        let bidi = stopEdge.get_bidi_edge();
                        if ec.split_at_2(dc, stopEdge, n, &stopEdgeID, &edgeID, stopEdge.get_num_lanes(), stopEdge.get_num_lanes(), -1.0, 1.0, 0) {
                            if !bidi.is_null() {
                                let edgeID2 = ec.create_unused_id(&format!("{}.{}", bidi.get_id(), splitPos as i32), "#");
                                ec.split_at_2(dc, bidi, n, bidi.get_id(), &edgeID2, bidi.get_num_lanes(), bidi.get_num_lanes(), -1.0, 1.0, 0);
                            }
                            addedSignals += 1;
                            item.m().find_lane_and_compute_bus_stop_extent_2(stopEdge);
                            stopEdge = ec.retrieve(&stopEdgeID, false);
                            NBTurningDirectionsComputer::compute_turn_directions_for_node(stopEdge.get_from_node(), false);
                            NBTurningDirectionsComputer::compute_turn_directions_for_node(stopEdge.get_to_node(), false);
                        }
                    } else {
                        let p = *to.get_position();
                        to.m().reinit(&p, SumoXMLNodeType::RAIL_SIGNAL, false);
                        addedSignals += 1;
                    }
                }
                let mut from = stopEdge.get_from_node();
                let mut curr = stopEdge;
                let mut searchDist = minLength - stopEdge.get_loaded_length();
                while searchDist > 0.0 && from.geometry_like() && from.get_type() != SumoXMLNodeType::RAIL_SIGNAL {
                    for in_ in from.get_incoming_edges().iter() {
                        if in_.get_from_node() != curr.get_to_node() {
                            // found edge that isn't a bidi predecessor
                            curr = *in_;
                            break;
                        }
                    }
                    if curr.get_from_node() == from {
                        // bidi edge without predecessor
                        break;
                    } else {
                        from = curr.get_from_node();
                    }
                    searchDist -= curr.get_loaded_length();
                }
                if Self::can_be_signal(from) {
                    let minPos = min2(splitOffset, curr.get_length() / 2.0);
                    let splitPos = if searchDist >= 0.0 {
                        minPos
                    } else {
                        max2(minPos, min2(item.get_start_pos(), -searchDist - 10.0))
                    };
                    if split && (!from.geometry_like() || splitPos > splitOffsetSlack) {
                        let nodeID = nc.create_unused_id(&format!("{}_{}", curr.get_id(), itemId), "#");
                        let n = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(&nodeID, &curr.get_geometry().position_at_offset(splitPos, 0.0), SumoXMLNodeType::RAIL_SIGNAL))));
                        nc.insert_2(n);
                        let edgeID = ec.create_unused_id(&format!("{}.{}", curr.get_id(), splitPos as i32), "#");
                        let currID = curr.get_id().to_string();
                        let bidi = curr.get_bidi_edge();
                        if ec.split_at_2(dc, curr, n, &edgeID, curr.get_id(), curr.get_num_lanes(), curr.get_num_lanes(), -1.0, 1.0, 0) {
                            if !bidi.is_null() {
                                let edgeID2 = ec.create_unused_id(&format!("{}.{}", bidi.get_id(), splitPos as i32), "#");
                                ec.split_at_2(dc, bidi, n, bidi.get_id(), &edgeID2, bidi.get_num_lanes(), bidi.get_num_lanes(), -1.0, 1.0, 0);
                            }
                            addedSignals += 1;
                            item.m().find_lane_and_compute_bus_stop_extent_2(stopEdge);
                            curr = ec.retrieve(&currID, false);
                            NBTurningDirectionsComputer::compute_turn_directions_for_node(curr.get_from_node(), false);
                            NBTurningDirectionsComputer::compute_turn_directions_for_node(curr.get_to_node(), false);
                        }
                    } else {
                        let p = *from.get_position();
                        from.m().reinit(&p, SumoXMLNodeType::RAIL_SIGNAL, false);
                        addedSignals += 1;
                    }
                }
            }
        }
        write_messagef!("Added % rail signals at % stops.", addedSignals, sc.get_stops().len());
        addedSignals
    }

    /// C++: `bool NBRailwaySignalGuesser::canBeSignal(const NBNode *)` [static, private]
    // bool
    // NBRailwaySignalGuesser::canBeSignal(const NBNode* node) {
    //     return (node->getType() != SumoXMLNodeType::RAIL_SIGNAL
    //             && node->getIncomingEdges().size() >= 1
    //             && node->getOutgoingEdges().size() >= 1
    //             && (node->getEdges().size() > 2
    //                 || node->getIncomingEdges().front()->getFromNode() != node->getOutgoingEdges().front()->getToNode()));
    // }
    #[allow(unused_variables)]
    pub fn can_be_signal(node: Ptr<NBNode>) -> bool {
        node.get_type() != SumoXMLNodeType::RAIL_SIGNAL
            && node.get_incoming_edges().len() >= 1
            && node.get_outgoing_edges().len() >= 1
            && (node.get_edges().len() > 2
                || node.get_incoming_edges()[0].get_from_node() != node.get_outgoing_edges()[0].get_to_node())
    }

}

/// C++ class `NBRailwayTopologyAnalyzer` (netbuild/NBAlgorithms_Railway.h)
pub struct NBRailwayTopologyAnalyzer {
}

impl NBRailwayTopologyAnalyzer {
    /// C++: `void NBRailwayTopologyAnalyzer::analyzeTopology(NBEdgeCont &)` [static]
    // void
    // NBRailwayTopologyAnalyzer::analyzeTopology(NBEdgeCont& ec) {
    //     getBrokenRailNodes(ec, true);
    // }
    #[allow(unused_variables)]
    pub fn analyze_topology(ec: &mut NBEdgeCont) {
        Self::get_broken_rail_nodes(ec, true);
    }

    /// C++: `int NBRailwayTopologyAnalyzer::repairTopology(NBEdgeCont &, NBPTStopCont &, NBPTLineCont &)` [static]
    // int
    // NBRailwayTopologyAnalyzer::repairTopology(NBEdgeCont& ec, NBPTStopCont& sc, NBPTLineCont& lc) {
    //     const bool minimal = OptionsCont::getOptions().getBool("railway.topology.repair.minimal");
    //     int addedBidi = 0;
    //     if (!minimal) {
    //         addedBidi += extendBidiEdges(ec);
    //         addedBidi += reverseEdges(ec, sc, lc); // technically not bidi but new edges nevertheless
    //         addedBidi += addBidiEdgesForBufferStops(ec);
    //         addedBidi += addBidiEdgesBetweenSwitches(ec);
    //     }
    //     if (lc.getLines().size() > 0) {
    //         addedBidi += addBidiEdgesForStops(ec, lc, sc, minimal);
    //     }
    //     if (OptionsCont::getOptions().getBool("railway.topology.repair.connect-straight")) {
    //         addedBidi += addBidiEdgesForStraightConnectivity(ec, true);
    //         addedBidi += addBidiEdgesForStraightConnectivity(ec, false);
    //         addedBidi += extendBidiEdges(ec);
    //     }
    //     return addedBidi;
    // }
    #[allow(unused_variables)]
    pub fn repair_topology(ec: &mut NBEdgeCont, sc: &mut NBPTStopCont, lc: &mut NBPTLineCont) -> i32 {
        let minimal = get_bool("railway.topology.repair.minimal");
        let mut addedBidi = 0;
        if !minimal {
            addedBidi += Self::extend_bidi_edges(ec);
            addedBidi += Self::reverse_edges(ec, sc, lc); // technically not bidi but new edges nevertheless
            addedBidi += Self::add_bidi_edges_for_buffer_stops(ec);
            addedBidi += Self::add_bidi_edges_between_switches(ec);
        }
        if lc.get_lines().len() > 0 {
            addedBidi += Self::add_bidi_edges_for_stops(ec, lc, sc, minimal);
        }
        if get_bool("railway.topology.repair.connect-straight") {
            addedBidi += Self::add_bidi_edges_for_straight_connectivity(ec, true);
            addedBidi += Self::add_bidi_edges_for_straight_connectivity(ec, false);
            addedBidi += Self::extend_bidi_edges(ec);
        }
        addedBidi
    }

    /// C++: `int NBRailwayTopologyAnalyzer::makeAllBidi(NBEdgeCont &)` [static]
    // int
    // NBRailwayTopologyAnalyzer::makeAllBidi(NBEdgeCont& ec) {
    //     int numNotCenterEdges = 0;
    //     int numAddedBidiEdges = 0;
    //     std::string inputfile = OptionsCont::getOptions().getString("railway.topology.all-bidi.input-file");
    //     std::vector<NBEdge*> edges;
    //     if (inputfile == "") {
    //         for (NBEdge* edge : ec.getAllEdges()) {
    //             edges.push_back(edge);
    //         }
    //     } else {
    //         std::set<std::string> edgeIDs;
    //         NBHelpers::loadEdgesFromFile(inputfile, edgeIDs);
    //         for (const std::string& edgeID : edgeIDs) {
    //             NBEdge* edge = ec.retrieve(edgeID);
    //             if (edge != nullptr) {
    //                 edges.push_back(edge);
    //             }
    //         }
    //     }
    //     for (NBEdge* edge : edges) {
    //         if (hasRailway(edge->getPermissions())) {
    //             // rebuild connections if given from an earlier network
    //             edge->invalidateConnections(true);
    //             if (!edge->isBidiRail()) {
    //                 if (edge->getLaneSpreadFunction() == LaneSpreadFunction::CENTER) {
    //                     NBEdge* e2 = addBidiEdge(ec, edge, false);
    //                     if (e2 != nullptr) {
    //                         numAddedBidiEdges++;
    //                     }
    //                 } else {
    //                     numNotCenterEdges++;
    //                 }
    //             }
    //         }
    //     }
    //     WRITE_MESSAGEF(TL("Added % bidi-edges to ensure that all tracks are usable in both directions."), toString(numAddedBidiEdges));
    //     if (numNotCenterEdges) {
    //         WRITE_WARNINGF(TL("Ignore % edges because they have the wrong spreadType"), toString(numNotCenterEdges));
    //     }
    //     return numAddedBidiEdges;
    // }
    #[allow(unused_variables)]
    pub fn make_all_bidi(ec: &mut NBEdgeCont) -> i32 {
        let mut numNotCenterEdges = 0;
        let mut numAddedBidiEdges = 0;
        let inputfile = get_string("railway.topology.all-bidi.input-file");
        let mut edges: Vec<Ptr<NBEdge>> = Vec::new();
        if inputfile == "" {
            for edge in ec.get_all_edges() {
                edges.push(edge);
            }
        } else {
            let mut edgeIDs: BTreeSet<String> = BTreeSet::new();
            NBHelpers::load_edges_from_file(&inputfile, &mut edgeIDs);
            for edgeID in edgeIDs.iter() {
                let edge = ec.retrieve(edgeID, false);
                if !edge.is_null() {
                    edges.push(edge);
                }
            }
        }
        for edge in edges {
            if Self::has_railway(edge.get_permissions(-1)) {
                // rebuild connections if given from an earlier network
                edge.m().invalidate_connections(true);
                if !edge.is_bidi_rail(false) {
                    if edge.get_lane_spread_function() == LaneSpreadFunction::CENTER {
                        let e2 = Self::add_bidi_edge(ec, edge, false);
                        if !e2.is_null() {
                            numAddedBidiEdges += 1;
                        }
                    } else {
                        numNotCenterEdges += 1;
                    }
                }
            }
        }
        write_messagef!("Added % bidi-edges to ensure that all tracks are usable in both directions.", numAddedBidiEdges);
        if numNotCenterEdges != 0 {
            write_warningf!("Ignore % edges because they have the wrong spreadType", numNotCenterEdges);
        }
        numAddedBidiEdges
    }

    /// C++: `void NBRailwayTopologyAnalyzer::setPTLinePriority(NBEdgeCont &, NBPTLineCont &, SVCPermissions)` [static]
    // void
    // NBRailwayTopologyAnalyzer::setPTLinePriority(NBEdgeCont& ec, NBPTLineCont& lc, SVCPermissions vClasses) {
    //     for (NBEdge* edge : ec.getAllEdges()) {
    //         SVCPermissions p = edge->getPermissions();
    //         if (isRailway(p) && (p & vClasses) != 0) {
    //             if (edge->getRoutingType() == "") {
    //                 edge->setPriority(-1);
    //             } else if (StringUtils::isInt(edge->getRoutingType())) {
    //                 edge->setPriority(StringUtils::toInt(edge->getRoutingType()));
    //             }
    //         }
    //     }
    //     for (const auto& item : lc.getLines()) {
    //         if ((item.second->getVClass() & vClasses) == 0) {
    //             continue;
    //         }
    //         for (NBEdge* edge : item.second->getEdges()) {
    //             if (edge->getRoutingType() == "") {
    //                 edge->setPriority(4);
    //                 edge->setRoutingType("4");
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_pt_line_priority(ec: &mut NBEdgeCont, lc: &mut NBPTLineCont, vClasses: SVCPermissions) {
        for edge in ec.get_all_edges() {
            let p = edge.get_permissions(-1);
            if is_railway(p) && (p & vClasses) != 0 {
                if edge.get_routing_type() == "" {
                    edge.m().set_priority(-1);
                } else if is_int(edge.get_routing_type()) {
                    edge.m().set_priority(to_int(edge.get_routing_type()).unwrap());
                }
            }
        }
        for item in lc.get_lines().values() {
            if (item.get_v_class() & vClasses) == 0 {
                continue;
            }
            for edge in item.get_edges().iter() {
                if edge.get_routing_type() == "" {
                    edge.m().set_priority(4);
                    edge.m().set_routing_type("4");
                }
            }
        }
    }

    /// C++: `void NBRailwayTopologyAnalyzer::extendDirectionPriority(NBEdgeCont &, bool)` [static]
    // void
    // NBRailwayTopologyAnalyzer::extendDirectionPriority(NBEdgeCont& ec, bool fromUniDir) {
    //     // if fromUniDir=true, assign priority value for each railway edge:
    //     // 4: edge is unidirectional
    //     // 3: edge is in main direction of bidirectional track
    //     // 2: edge is part of bidirectional track, main direction unknown - both edges are extensions of unidirectional edges
    //     // 1: edge is part of bidirectional track, main direction unknown - neither edge is an extension of a unidirectional edge
    //     // 0: edge is part of bidirectional track in reverse of main direction
    //     //
    //     // otherwise:
    //     // assign priority value for each railway edge with priority -1 (undefined):
    //     // x: edges with priority >= 0 keep their priority
    //     // x-1 : edge is in direct (no switch) sequence of an edge with initial priority x
    //     // x-2 : edge and its opposite-direction are in direct (no switch) sequence of an edge with initial priority x
    //     // x-3 : edge is part of bidirectional track, both directions are indirect extensions of x-1 edges
    //     // x-4 : edge is reverse direction of an x-1 edge
    //
    //     std::set<NBEdge*, ComparatorIdLess> bidi;
    //     EdgeSet uni;
    //     for (NBEdge* edge : ec.getAllEdges()) {
    //         if (hasRailway(edge->getPermissions())) {
    //             if (fromUniDir) {
    //                 if (!edge->isBidiRail()) {
    //                     edge->setPriority(4);
    //                     edge->setRoutingType("4");
    //                     uni.insert(edge);
    //                 } else {
    //                     bidi.insert(edge);
    //                 }
    //             } else {
    //                 if (edge->getPriority() >= 0) {
    //                     uni.insert(edge);
    //                 } else {
    //                     bidi.insert(edge);
    //                 }
    //             }
    //         }
    //     }
    //
    //     if (uni.size() == 0) {
    //         if (bidi.size() != 0) {
    //             WRITE_WARNING(TL("Cannot extend track direction priority because there are no track edges with positive priority"));
    //         }
    //         return;
    //     }
    //     EdgeSet seen;
    //     EdgeSet check = uni;
    //     EdgeSet forward;
    //     while (!check.empty()) {
    //         NBEdge* edge = *check.begin();
    //         check.erase(edge);
    //         if (seen.count(edge) != 0) {
    //             continue;
    //         }
    //         seen.insert(edge);
    //         NBEdge* straightOut = edge->getStraightContinuation(edge->getPermissions());
    //         if (straightOut != nullptr && straightOut->getStraightPredecessor(straightOut->getPermissions()) == edge) {
    //             forward.insert(straightOut);
    //             check.insert(straightOut);
    //         }
    //         NBEdge* straightIn = edge->getStraightPredecessor(edge->getPermissions());
    //         if (straightIn != nullptr && straightIn->getStraightContinuation(straightIn->getPermissions()) == edge) {
    //             forward.insert(straightIn);
    //             check.insert(straightIn);
    //         }
    // #ifdef DEBUG_DIRECTION_PRIORITY
    //         std::cout << "edge=" << edge->getID() << " in=" << Named::getIDSecure(straightIn) << " out=" << Named::getIDSecure(straightOut)
    //                   << " outPred=" << (straightOut != nullptr ? Named::getIDSecure(straightOut->getStraightPredecessor(straightOut->getPermissions())) : "")
    //                   << " inSucc=" << (straightIn != nullptr ? Named::getIDSecure(straightIn->getStraightContinuation(straightIn->getPermissions())) : "")
    //                   << "\n";
    // #endif
    //     }
    //
    //     for (NBEdge* edge : bidi) {
    //         NBEdge* bidiEdge = const_cast<NBEdge*>(edge->getBidiEdge());
    //         int prio;
    //         int bidiPrio;
    //         if (forward.count(edge) != 0) {
    //             if (forward.count(bidiEdge) == 0) {
    //                 prio = 3;
    //                 bidiPrio = 0;
    //             } else {
    //                 // both forward
    //                 prio = 2;
    //                 bidiPrio = 2;
    //             }
    //         } else {
    //             if (forward.count(bidiEdge) != 0) {
    //                 prio = 0;
    //                 bidiPrio = 3;
    //             } else {
    //                 // neither forward
    //                 prio = 1;
    //                 bidiPrio = 1;
    //             }
    //         }
    //         if (bidiEdge == nullptr) {
    //             WRITE_WARNINGF(TL("Edge '%' was loaded with undefined priority (%) but has unambiguous main direction (no bidi edge)"), edge->getID(), edge->getPriority());
    //         }
    //         if (edge->getPriority() >= 0) {
    //             bidiPrio = 0;
    //         }
    //         if (bidiEdge != nullptr && bidiEdge->getPriority() >= 0) {
    //             prio = 0;
    //         }
    //         if (edge->getPriority() < 0) {
    //             edge->setPriority(prio);
    //             edge->setRoutingType(toString(prio));
    //         }
    //         if (bidiEdge != nullptr && bidiEdge->getPriority() < 0) {
    //             bidiEdge->setPriority(bidiPrio);
    //             bidiEdge->setRoutingType(toString(bidiPrio));
    //         }
    //     }
    //     std::map<int, int> numPrios;
    //     for (NBEdge* edge : bidi) {
    //         numPrios[edge->getPriority()]++;
    //     }
    //     if (fromUniDir) {
    //         WRITE_MESSAGE("Assigned edge priority based on main direction: " + joinToString(numPrios, " ", ":") + ".")
    //     } else {
    //         WRITE_MESSAGE("Extended edge priority based on main direction: " + joinToString(numPrios, " ", ":") + ".")
    //     }
    // }
    #[allow(unused_variables)]
    pub fn extend_direction_priority(ec: &mut NBEdgeCont, fromUniDir: bool) {
        // determinism: std::set<NBEdge*, ComparatorIdLess> is ordered by edge id, so bidi is keyed by id
        let mut bidi: BTreeMap<String, Ptr<NBEdge>> = BTreeMap::new();
        let mut uni: EdgeSet = EdgeSet::new();
        for edge in ec.get_all_edges() {
            if Self::has_railway(edge.get_permissions(-1)) {
                if fromUniDir {
                    if !edge.is_bidi_rail(false) {
                        edge.m().set_priority(4);
                        edge.m().set_routing_type("4");
                        uni.insert(edge);
                    } else {
                        bidi.insert(edge.get_id().to_string(), edge);
                    }
                } else {
                    if edge.get_priority() >= 0 {
                        uni.insert(edge);
                    } else {
                        bidi.insert(edge.get_id().to_string(), edge);
                    }
                }
            }
        }

        if uni.is_empty() {
            if !bidi.is_empty() {
                write_warning("Cannot extend track direction priority because there are no track edges with positive priority");
            }
            return;
        }
        let mut seen: EdgeSet = EdgeSet::new();
        let mut check = uni.clone();
        let mut forward: EdgeSet = EdgeSet::new();
        while !check.is_empty() {
            let edge = check.pop_first().unwrap();
            if seen.contains(&edge) {
                continue;
            }
            seen.insert(edge);
            let straightOut = edge.get_straight_continuation(edge.get_permissions(-1));
            if !straightOut.is_null() && straightOut.get_straight_predecessor(straightOut.get_permissions(-1)) == edge {
                forward.insert(straightOut);
                check.insert(straightOut);
            }
            let straightIn = edge.get_straight_predecessor(edge.get_permissions(-1));
            if !straightIn.is_null() && straightIn.get_straight_continuation(straightIn.get_permissions(-1)) == edge {
                forward.insert(straightIn);
                check.insert(straightIn);
            }
        }

        for edge in bidi.values() {
            let edge = *edge;
            let bidiEdge = edge.get_bidi_edge();
            let (mut prio, mut bidiPrio) = if forward.contains(&edge) {
                if !forward.contains(&bidiEdge) {
                    (3, 0)
                } else {
                    // both forward
                    (2, 2)
                }
            } else if forward.contains(&bidiEdge) {
                (0, 3)
            } else {
                // neither forward
                (1, 1)
            };
            if bidiEdge.is_null() {
                write_warningf!("Edge '%' was loaded with undefined priority (%) but has unambiguous main direction (no bidi edge)", edge.get_id(), edge.get_priority());
            }
            if edge.get_priority() >= 0 {
                bidiPrio = 0;
            }
            if !bidiEdge.is_null() && bidiEdge.get_priority() >= 0 {
                prio = 0;
            }
            if edge.get_priority() < 0 {
                edge.m().set_priority(prio);
                edge.m().set_routing_type(&prio.to_string());
            }
            if !bidiEdge.is_null() && bidiEdge.get_priority() < 0 {
                bidiEdge.m().set_priority(bidiPrio);
                bidiEdge.m().set_routing_type(&bidiPrio.to_string());
            }
        }
        let mut numPrios: BTreeMap<i32, i32> = BTreeMap::new();
        for edge in bidi.values() {
            *numPrios.entry(edge.get_priority()).or_insert(0) += 1;
        }
        let joined = numPrios.iter().map(|(k, v)| format!("{}:{}", k, v)).collect::<Vec<_>>().join(" ");
        if fromUniDir {
            write_message(&format!("Assigned edge priority based on main direction: {}.", joined));
        } else {
            write_message(&format!("Extended edge priority based on main direction: {}.", joined));
        }
    }

    /// C++: `double NBRailwayTopologyAnalyzer::getTravelTimeStatic(const Track *const, const NBVehicle *const, double)` [static]
    // double
    // NBRailwayTopologyAnalyzer::getTravelTimeStatic(const Track* const track, const NBVehicle* const veh, double time) {
    //     return NBEdge::getTravelTimeStatic(track->edge, veh, time) * track->penalty;
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(track: Ptr<NBRailwayTopologyAnalyzer_Track>, veh: Ptr<NBVehicle>, time: f64) -> f64 {
        NBEdge::get_travel_time_static(track.edge, veh, time) * track.penalty
    }

    /// C++: `std::set<NBNode *> NBRailwayTopologyAnalyzer::getRailNodes(NBEdgeCont &, bool)` [static]
    // std::set<NBNode*>
    // NBRailwayTopologyAnalyzer::getRailNodes(NBEdgeCont& ec, bool verbose) {
    //     std::set<NBNode*> railNodes;
    //     int numRailEdges = 0;
    //     for (auto it = ec.begin(); it != ec.end(); it++) {
    //         if (hasRailway(it->second->getPermissions())) {
    //             numRailEdges++;
    //             railNodes.insert(it->second->getFromNode());
    //             railNodes.insert(it->second->getToNode());
    //         }
    //     }
    //     int numRailSignals = 0;
    //     for (const NBNode* const node : railNodes) {
    //         if (node->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             numRailSignals++;
    //         }
    //     }
    //     if (verbose) {
    //         WRITE_MESSAGEF(TL("Found % railway edges and % railway nodes (% signals)."), toString(numRailEdges), toString(railNodes.size()), toString(numRailSignals));
    //     }
    //     return railNodes;
    // }
    #[allow(unused_variables)]
    pub fn get_rail_nodes(ec: &mut NBEdgeCont, verbose: bool) -> BTreeSet<Ptr<NBNode>> {
        let mut railNodes: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let mut numRailEdges = 0;
        for edge in ec.get_all_edges() {
            if Self::has_railway(edge.get_permissions(-1)) {
                numRailEdges += 1;
                railNodes.insert(edge.get_from_node());
                railNodes.insert(edge.get_to_node());
            }
        }
        let mut numRailSignals = 0;
        for node in railNodes.iter() {
            if node.get_type() == SumoXMLNodeType::RAIL_SIGNAL {
                numRailSignals += 1;
            }
        }
        if verbose {
            write_messagef!("Found % railway edges and % railway nodes (% signals).", numRailEdges, railNodes.len(), numRailSignals);
        }
        railNodes
    }

    /// C++: `std::set<NBNode *> NBRailwayTopologyAnalyzer::getBrokenRailNodes(NBEdgeCont &, bool)` [static, private]
    // std::set<NBNode*>
    // NBRailwayTopologyAnalyzer::getBrokenRailNodes(NBEdgeCont& ec, bool verbose) {
    //     std::set<NBNode*> brokenNodes;
    //     OutputDevice& device = OutputDevice::getDevice(verbose
    //                            ? OptionsCont::getOptions().getString("railway.topology.output")
    //                            : "/dev/null");
    //
    //     device.writeXMLHeader("railwayTopology", "");
    //     std::set<NBNode*> railNodes = getRailNodes(ec, verbose);
    //     std::map<std::pair<int, int>, std::set<NBNode*, ComparatorIdLess> > types;
    //     std::set<NBEdge*, ComparatorIdLess> bidiEdges;
    //     std::set<NBEdge*, ComparatorIdLess> bufferStops;
    //     for (NBNode* node : railNodes) {
    //         EdgeVector inEdges, outEdges;
    //         getRailEdges(node, inEdges, outEdges);
    //         types[std::make_pair((int)inEdges.size(), (int)outEdges.size())].insert(node);
    //         for (NBEdge* e : outEdges) {
    //             if (e->isBidiRail() && bidiEdges.count(e->getTurnDestination(true)) == 0) {
    //                 NBEdge* primary = e;
    //                 NBEdge* secondary = e->getTurnDestination(true);
    //                 if (e->getID()[0] == '-') {
    //                     std::swap(primary, secondary);
    //                 } else if (primary->getID()[0] != '-' && secondary->getID()[0] != '-' && secondary->getID() < primary->getID()) {
    //                     std::swap(primary, secondary);
    //                 }
    //                 if (bidiEdges.count(secondary) == 0) {
    //                     // avoid duplicate when both ids start with '-'
    //                     bidiEdges.insert(primary);
    //                 }
    //             }
    //         }
    //     }
    //
    //     int numBrokenA = 0;
    //     int numBrokenB = 0;
    //     int numBrokenC = 0;
    //     int numBrokenD = 0;
    //     int numBufferStops = 0;
    //     if (verbose && types.size() > 0) {
    //         WRITE_MESSAGE(TL("Railway nodes by number of incoming,outgoing edges:"))
    //     }
    //     device.openTag("legend");
    //     device.openTag("error");
    //     device.writeAttr(SUMO_ATTR_ID, "a");
    //     device.writeAttr("meaning", "edge pair angle supports driving but both are outgoing");
    //     device.closeTag();
    //     device.openTag("error");
    //     device.writeAttr(SUMO_ATTR_ID, "b");
    //     device.writeAttr("meaning", "edge pair angle supports driving but both are incoming");
    //     device.closeTag();
    //     device.openTag("error");
    //     device.writeAttr(SUMO_ATTR_ID, "c");
    //     device.writeAttr("meaning", "an incoming edge has a sharp angle to all outgoing edges");
    //     device.closeTag();
    //     device.openTag("error");
    //     device.writeAttr(SUMO_ATTR_ID, "d");
    //     device.writeAttr("meaning", "an outgoing edge has a sharp angle from all incoming edges");
    //     device.closeTag();
    //     device.closeTag();
    //
    //     for (auto it : types) {
    //         int numBrokenType = 0;
    //         device.openTag("railNodeType");
    //         int in = it.first.first;
    //         int out = it.first.second;
    //         device.writeAttr("in", in);
    //         device.writeAttr("out", out);
    //         for (NBNode* n : it.second) {
    //             device.openTag(SUMO_TAG_NODE);
    //             device.writeAttr(SUMO_ATTR_ID, n->getID());
    //             EdgeVector inRail, outRail;
    //             getRailEdges(n, inRail, outRail);
    //             // check if there is a mismatch between angle and edge direction
    //             // (see above)
    //
    //             std::string broken = "";
    //             if (in < 2 && hasStraightPair(n, outRail, outRail)) {
    //                 broken += "a";
    //                 numBrokenA++;
    //             }
    //             if (out < 2 && hasStraightPair(n, inRail, inRail)) {
    //                 broken += "b";
    //                 numBrokenB++;
    //             }
    //             if (out > 0) {
    //                 for (NBEdge* e : inRail) {
    //                     EdgeVector tmp;
    //                     tmp.push_back(e);
    //                     if (allSharp(n, tmp, outRail)) {
    //                         broken += "c";
    //                         numBrokenC++;
    //                         break;
    //                     }
    //                 }
    //             }
    //             if (in > 0) {
    //                 for (NBEdge* e : outRail) {
    //                     EdgeVector tmp;
    //                     tmp.push_back(e);
    //                     if (allSharp(n, inRail, tmp)) {
    //                         broken += "d";
    //                         numBrokenD++;
    //                         break;
    //                     }
    //                 }
    //             }
    //             // do not mark bidi nodes as broken
    //             if (((in == 1 && out == 1) || (in == 2 && out == 2))
    //                     && allBidi(inRail) && allBidi(outRail)) {
    //                 broken = "";
    //             }
    //
    //             if (broken.size() > 0) {
    //                 device.writeAttr("broken", broken);
    //                 brokenNodes.insert(n);
    //                 numBrokenType++;
    //             }
    //             if (StringUtils::toBool(n->getParameter("buffer_stop", "false"))) {
    //                 device.writeAttr("buffer_stop", "true");
    //                 numBufferStops++;
    //             }
    //             device.closeTag();
    //         }
    //         device.closeTag();
    //         if (verbose) {
    //             WRITE_MESSAGE("   " + toString(it.first.first) + "," + toString(it.first.second)
    //                           + " count: " + toString(it.second.size()) + " broken: " + toString(numBrokenType));
    //         }
    //
    //     }
    //     if (verbose) {
    //         WRITE_MESSAGE("Found " + toString(brokenNodes.size()) + " broken railway nodes "
    //                       + "(A=" + toString(numBrokenA)
    //                       + " B=" + toString(numBrokenB)
    //                       + " C=" + toString(numBrokenC)
    //                       + " D=" + toString(numBrokenD)
    //                       + ")");
    //         WRITE_MESSAGEF(TL("Found % railway nodes marked as buffer_stop"), toString(numBufferStops));
    //     }
    //
    //     for (NBEdge* e : bidiEdges) {
    //         device.openTag("bidiEdge");
    //         device.writeAttr(SUMO_ATTR_ID, e->getID());
    //         device.writeAttr("bidi", e->getTurnDestination(true)->getID());
    //         device.closeTag();
    //     }
    //     if (verbose) {
    //         WRITE_MESSAGEF(TL("Found % bidirectional rail edges"), toString(bidiEdges.size()));
    //     }
    //
    //     device.close();
    //     return brokenNodes;
    // }
    #[allow(unused_variables)]
    pub fn get_broken_rail_nodes(ec: &mut NBEdgeCont, verbose: bool) -> BTreeSet<Ptr<NBNode>> {
        let mut brokenNodes: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let devName = if verbose { get_string("railway.topology.output") } else { "/dev/null".to_string() };
        let deviceRef = OutputDevice::get_device(&devName, true).unwrap();
        let mut device = deviceRef.borrow_mut();

        device.write_xml_header("railwayTopology", "", BTreeMap::new(), true);
        let railNodes = Self::get_rail_nodes(ec, verbose);
        // determinism: std::map<pair<int,int>, std::set<NBNode*, ComparatorIdLess>> and std::set<NBEdge*, ComparatorIdLess>
        // are keyed by id here
        let mut types: BTreeMap<(i32, i32), BTreeMap<String, Ptr<NBNode>>> = BTreeMap::new();
        let mut bidiEdges: BTreeMap<String, Ptr<NBEdge>> = BTreeMap::new();
        for node in railNodes.iter() {
            let mut inEdges: EdgeVector = Vec::new();
            let mut outEdges: EdgeVector = Vec::new();
            Self::get_rail_edges(*node, &mut inEdges, &mut outEdges);
            types.entry((inEdges.len() as i32, outEdges.len() as i32)).or_default().insert(node.get_id().to_string(), *node);
            for e in outEdges.iter() {
                if e.is_bidi_rail(false) && !bidiEdges.contains_key(e.get_turn_destination(true).get_id()) {
                    let mut primary = *e;
                    let mut secondary = e.get_turn_destination(true);
                    if e.get_id().starts_with('-') {
                        std::mem::swap(&mut primary, &mut secondary);
                    } else if !primary.get_id().starts_with('-') && !secondary.get_id().starts_with('-') && secondary.get_id() < primary.get_id() {
                        std::mem::swap(&mut primary, &mut secondary);
                    }
                    if !bidiEdges.contains_key(secondary.get_id()) {
                        // avoid duplicate when both ids start with '-'
                        bidiEdges.insert(primary.get_id().to_string(), primary);
                    }
                }
            }
        }

        let mut numBrokenA = 0;
        let mut numBrokenB = 0;
        let mut numBrokenC = 0;
        let mut numBrokenD = 0;
        let mut numBufferStops = 0;
        if verbose && types.len() > 0 {
            write_message("Railway nodes by number of incoming,outgoing edges:");
        }
        device.open_tag("legend");
        device.open_tag("error");
        device.write_attr(SUMO_ATTR_ID, "a");
        device.write_attr("meaning", "edge pair angle supports driving but both are outgoing");
        device.close_tag("");
        device.open_tag("error");
        device.write_attr(SUMO_ATTR_ID, "b");
        device.write_attr("meaning", "edge pair angle supports driving but both are incoming");
        device.close_tag("");
        device.open_tag("error");
        device.write_attr(SUMO_ATTR_ID, "c");
        device.write_attr("meaning", "an incoming edge has a sharp angle to all outgoing edges");
        device.close_tag("");
        device.open_tag("error");
        device.write_attr(SUMO_ATTR_ID, "d");
        device.write_attr("meaning", "an outgoing edge has a sharp angle from all incoming edges");
        device.close_tag("");
        device.close_tag("");

        for (key, nodes) in types.iter() {
            let mut numBrokenType = 0;
            device.open_tag("railNodeType");
            let (in_, out) = *key;
            device.write_attr("in", &in_);
            device.write_attr("out", &out);
            for n in nodes.values() {
                device.open_tag("node");
                device.write_attr(SUMO_ATTR_ID, n.get_id());
                let mut inRail: EdgeVector = Vec::new();
                let mut outRail: EdgeVector = Vec::new();
                Self::get_rail_edges(*n, &mut inRail, &mut outRail);
                // check if there is a mismatch between angle and edge direction
                // (see above)

                let mut broken = String::new();
                if in_ < 2 && Self::has_straight_pair(*n, &outRail, &outRail) {
                    broken += "a";
                    numBrokenA += 1;
                }
                if out < 2 && Self::has_straight_pair(*n, &inRail, &inRail) {
                    broken += "b";
                    numBrokenB += 1;
                }
                if out > 0 {
                    for e in inRail.iter() {
                        let tmp: EdgeVector = vec![*e];
                        if Self::all_sharp(*n, &tmp, &outRail, false) {
                            broken += "c";
                            numBrokenC += 1;
                            break;
                        }
                    }
                }
                if in_ > 0 {
                    for e in outRail.iter() {
                        let tmp: EdgeVector = vec![*e];
                        if Self::all_sharp(*n, &inRail, &tmp, false) {
                            broken += "d";
                            numBrokenD += 1;
                            break;
                        }
                    }
                }
                // do not mark bidi nodes as broken
                if ((in_ == 1 && out == 1) || (in_ == 2 && out == 2)) && Self::all_bidi(&inRail) && Self::all_bidi(&outRail) {
                    broken = String::new();
                }

                if broken.len() > 0 {
                    device.write_attr("broken", &broken);
                    brokenNodes.insert(*n);
                    numBrokenType += 1;
                }
                if to_bool(&n.base2.get_parameter("buffer_stop", "false")).unwrap() {
                    device.write_attr("buffer_stop", "true");
                    numBufferStops += 1;
                }
                device.close_tag("");
            }
            device.close_tag("");
            if verbose {
                write_message(&format!("   {},{} count: {} broken: {}", in_, out, nodes.len(), numBrokenType));
            }
        }
        if verbose {
            write_message(&format!(
                "Found {} broken railway nodes (A={} B={} C={} D={})",
                brokenNodes.len(),
                numBrokenA,
                numBrokenB,
                numBrokenC,
                numBrokenD
            ));
            write_messagef!("Found % railway nodes marked as buffer_stop", numBufferStops);
        }

        for e in bidiEdges.values() {
            device.open_tag("bidiEdge");
            device.write_attr(SUMO_ATTR_ID, e.get_id());
            device.write_attr("bidi", e.get_turn_destination(true).get_id());
            device.close_tag("");
        }
        if verbose {
            write_messagef!("Found % bidirectional rail edges", bidiEdges.len());
        }

        let _ = device.close();
        brokenNodes
    }

    /// C++: `void NBRailwayTopologyAnalyzer::getRailEdges(const NBNode *, EdgeVector &, EdgeVector &)` [static, private]
    /// filter out rail edges among all edges of a the given node
    // void
    // NBRailwayTopologyAnalyzer::getRailEdges(const NBNode* node,
    //                                         EdgeVector& inEdges, EdgeVector& outEdges) {
    //     for (NBEdge* e : node->getIncomingEdges()) {
    //         if ((e->getPermissions() & SVC_RAIL_CLASSES) != 0) {
    //             inEdges.push_back(e);
    //         }
    //     }
    //     for (NBEdge* e : node->getOutgoingEdges()) {
    //         if ((e->getPermissions() & SVC_RAIL_CLASSES) != 0) {
    //             outEdges.push_back(e);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_rail_edges(node: Ptr<NBNode>, inEdges: &mut EdgeVector, outEdges: &mut EdgeVector) {
        for e in node.get_incoming_edges().iter() {
            if (e.get_permissions(-1) & SVC_RAIL_CLASSES) != 0 {
                inEdges.push(*e);
            }
        }
        for e in node.get_outgoing_edges().iter() {
            if (e.get_permissions(-1) & SVC_RAIL_CLASSES) != 0 {
                outEdges.push(*e);
            }
        }
    }

    /// C++: `bool NBRailwayTopologyAnalyzer::hasRailway(SVCPermissions)` [static, private]
    /// filter for rail edges but do not return (legacy) all purpose edges
    // static bool hasRailway(SVCPermissions permissions) {
    //         return (permissions & SVC_RAIL_CLASSES) > 0 && permissions != SVCAll;
    //     }
    #[allow(unused_variables)]
    pub fn has_railway(permissions: SVCPermissions) -> bool {
        (permissions & SVC_RAIL_CLASSES) > 0 && permissions != SVC_ALL
    }

    /// C++: `bool NBRailwayTopologyAnalyzer::isStraight(const NBNode *, const NBEdge *, const NBEdge *)` [static, private]
    // bool
    // NBRailwayTopologyAnalyzer::isStraight(const NBNode* node, const NBEdge* e1, const NBEdge* e2) {
    //     const double relAngle = NBHelpers::normRelAngle(e1->getAngleAtNode(node), e2->getAngleAtNode(node));
    //     /*
    //     std::cout << "  isStraight n=" << node->getID()
    //         << " e1=" << e1->getID()
    //         << " e2=" << e2->getID()
    //         << " a1=" << e1->getAngleAtNode(node)
    //         << " a2=" << e2->getAngleAtNode(node)
    //         << " rel=" << relAngle
    //         << "\n";
    //         */
    //     if ((e1->getToNode() == node && e2->getFromNode() == node)
    //             || (e1->getFromNode() == node && e2->getToNode() == node)) {
    //         // edges go in the same direction
    //         return fabs(relAngle) < SHARP_THRESHOLD;
    //     } else {
    //         // edges go in the opposite direction (both incoming or outgoing)
    //         return fabs(relAngle) > SHARP_THRESHOLD_SAMEDIR;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn is_straight(node: Ptr<NBNode>, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> bool {
        let relAngle = NBHelpers::norm_rel_angle(e1.get_angle_at_node(node), e2.get_angle_at_node(node));
        /*
        std::cout << "  isStraight n=" << node->getID()
            << " e1=" << e1->getID()
            << " e2=" << e2->getID()
            << " a1=" << e1->getAngleAtNode(node)
            << " a2=" << e2->getAngleAtNode(node)
            << " rel=" << relAngle
            << "\n";
            */
        if (e1.get_to_node() == node && e2.get_from_node() == node) || (e1.get_from_node() == node && e2.get_to_node() == node) {
            // edges go in the same direction
            relAngle.abs() < SHARP_THRESHOLD
        } else {
            // edges go in the opposite direction (both incoming or outgoing)
            relAngle.abs() > SHARP_THRESHOLD_SAMEDIR
        }
    }

    /// C++: `bool NBRailwayTopologyAnalyzer::hasStraightPair(const NBNode *, const EdgeVector &, const EdgeVector &)` [static, private]
    // bool
    // NBRailwayTopologyAnalyzer::hasStraightPair(const NBNode* node, const EdgeVector& edges,
    //         const EdgeVector& edges2) {
    // #ifdef DEBUG_SEQSTOREVERSE
    //     //if (node->getID() == DEBUGNODEID2) {
    //     //    std::cout << " edges=" << toString(edges) << " edges2=" << toString(edges2) << "\n";
    //     //}
    // #endif
    //     for (NBEdge* e1 : edges) {
    //         for (NBEdge* e2 : edges2) {
    //             //if (e1->getID() == "195411601#2" && e2->getID() == "93584120#3") {
    //             //    std::cout
    //             //        << " DEBUG normRelA=" << NBHelpers::normRelAngle(
    //             //                    e1->getAngleAtNode(node),
    //             //                    e2->getAngleAtNode(node))
    //             //        << "\n";
    //             //}
    //             if (e1 != e2 && isStraight(node, e1, e2)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_straight_pair(node: Ptr<NBNode>, edges: &EdgeVector, edges2: &EdgeVector) -> bool {
        for e1 in edges.iter() {
            for e2 in edges2.iter() {
                //if (e1->getID() == "195411601#2" && e2->getID() == "93584120#3") {
                //    std::cout
                //        << " DEBUG normRelA=" << NBHelpers::normRelAngle(
                //                    e1->getAngleAtNode(node),
                //                    e2->getAngleAtNode(node))
                //        << "\n";
                //}
                if e1 != e2 && Self::is_straight(node, *e1, *e2) {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool NBRailwayTopologyAnalyzer::allBroken(const NBNode *, NBEdge *, const EdgeVector &, const EdgeVector &)` [static, private]
    // bool
    // NBRailwayTopologyAnalyzer::allBroken(const NBNode* node, NBEdge* candOut, const EdgeVector& in, const EdgeVector& out) {
    //     for (NBEdge* e : in) {
    //         if (e != candOut && isStraight(node, e, candOut)) {
    //             if (gDebugFlag1) {
    //                 std::cout << " isStraight e=" << e->getID() << " candOut=" << candOut->getID() << "\n";
    //             }
    //             return false;
    //         }
    //     }
    //     for (NBEdge* e : out) {
    //         if (e != candOut && !isStraight(node, e, candOut)) {
    //             if (gDebugFlag1) {
    //                 std::cout << " isSharp e=" << e->getID() << " candOut=" << candOut->getID() << "\n";
    //             }
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn all_broken(node: Ptr<NBNode>, candOut: Ptr<NBEdge>, in_: &EdgeVector, out: &EdgeVector) -> bool {
        for e in in_.iter() {
            if *e != candOut && Self::is_straight(node, *e, candOut) {
                if gDebugFlag1 {
                    println!(" isStraight e={} candOut={}", e.get_id(), candOut.get_id());
                }
                return false;
            }
        }
        for e in out.iter() {
            if *e != candOut && !Self::is_straight(node, *e, candOut) {
                if gDebugFlag1 {
                    println!(" isSharp e={} candOut={}", e.get_id(), candOut.get_id());
                }
                return false;
            }
        }
        true
    }

    /// C++: `bool NBRailwayTopologyAnalyzer::allSharp(const NBNode *, const EdgeVector &, const EdgeVector &, bool)` [static, private]
    // bool
    // NBRailwayTopologyAnalyzer::allSharp(const NBNode* node, const EdgeVector& in, const EdgeVector& out, bool countBidiAsSharp) {
    //     bool allBidi = true;
    //     for (NBEdge* e1 : in) {
    //         for (NBEdge* e2 : out) {
    //             if (e1 != e2 && isStraight(node, e1, e2)) {
    //                 return false;
    //             }
    //             if (!e1->isBidiRail(true)) {
    //                 //std::cout << " allSharp node=" << node->getID() << " e1=" << e1->getID() << " is not bidi\n";
    //                 allBidi = false;
    //             }
    //         }
    //     }
    //     return !allBidi || countBidiAsSharp;
    // }
    #[allow(unused_variables)]
    pub fn all_sharp(node: Ptr<NBNode>, in_: &EdgeVector, out: &EdgeVector, countBidiAsSharp: bool) -> bool {
        let mut allBidi = true;
        for e1 in in_.iter() {
            for e2 in out.iter() {
                if e1 != e2 && Self::is_straight(node, *e1, *e2) {
                    return false;
                }
                if !e1.is_bidi_rail(true) {
                    //std::cout << " allSharp node=" << node->getID() << " e1=" << e1->getID() << " is not bidi\n";
                    allBidi = false;
                }
            }
        }
        !allBidi || countBidiAsSharp
    }

    /// C++: `bool NBRailwayTopologyAnalyzer::allBidi(const EdgeVector &)` [static, private]
    // bool
    // NBRailwayTopologyAnalyzer::allBidi(const EdgeVector& edges) {
    //     for (NBEdge* e : edges) {
    //         if (!e->isBidiRail()) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn all_bidi(edges: &EdgeVector) -> bool {
        for e in edges.iter() {
            if !e.is_bidi_rail(false) {
                return false;
            }
        }
        true
    }

    /// C++: `NBEdge * NBRailwayTopologyAnalyzer::isBidiSwitch(const NBNode *)` [static, private]
    // NBEdge*
    // NBRailwayTopologyAnalyzer::isBidiSwitch(const NBNode* n) {
    //     EdgeVector inRail, outRail;
    //     getRailEdges(n, inRail, outRail);
    //     if (inRail.size() == 2 && outRail.size() == 1 && isStraight(n, inRail.front(), inRail.back())) {
    //         if (isStraight(n, inRail.front(), outRail.front())) {
    //             return inRail.front();
    //         } else if (isStraight(n, inRail.back(), outRail.front())) {
    //             return inRail.back();
    //         }
    //     }
    //     if (inRail.size() == 1 && outRail.size() == 2 && isStraight(n, outRail.front(), outRail.back())) {
    //         if (isStraight(n, outRail.front(), inRail.front())) {
    //             return outRail.front();
    //         } else if (isStraight(n, outRail.back(), inRail.front())) {
    //             return outRail.back();
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn is_bidi_switch(n: Ptr<NBNode>) -> Ptr<NBEdge> {
        let mut inRail = EdgeVector::new();
        let mut outRail = EdgeVector::new();
        Self::get_rail_edges(n, &mut inRail, &mut outRail);
        if inRail.len() == 2 && outRail.len() == 1 && Self::is_straight(n, inRail[0], inRail[inRail.len() - 1]) {
            if Self::is_straight(n, inRail[0], outRail[0]) {
                return inRail[0];
            } else if Self::is_straight(n, inRail[inRail.len() - 1], outRail[0]) {
                return inRail[inRail.len() - 1];
            }
        }
        if inRail.len() == 1 && outRail.len() == 2 && Self::is_straight(n, outRail[0], outRail[outRail.len() - 1]) {
            if Self::is_straight(n, outRail[0], inRail[0]) {
                return outRail[0];
            } else if Self::is_straight(n, outRail[outRail.len() - 1], inRail[0]) {
                return outRail[outRail.len() - 1];
            }
        }
        Ptr::null()
    }

    /// C++: `NBEdge * NBRailwayTopologyAnalyzer::addBidiEdge(NBEdgeCont &, NBEdge *, bool)` [static, private]
    /// add bidi-edge for the given edge
    // NBEdge*
    // NBRailwayTopologyAnalyzer::addBidiEdge(NBEdgeCont& ec, NBEdge* edge, bool update) {
    //     assert(edge->getLaneSpreadFunction() == LaneSpreadFunction::CENTER);
    //     assert(!edge->isBidiRail());
    //     const std::string id2 = (edge->getID()[0] == '-'
    //                              ? edge->getID().substr(1)
    //                              : "-" + edge->getID());
    //     if (ec.wasIgnored(id2)) {
    //         // we had it before so the warning is already there
    //         return nullptr;
    //     }
    //     if (ec.retrieve(id2) == nullptr) {
    //         NBEdge* e2 = new NBEdge(id2, edge->getToNode(), edge->getFromNode(),
    //                                 edge, edge->getGeometry().reverse());
    //         if (edge->getParameter(NBTrafficLightDefinition::OSM_DIRECTION) == "forward") {
    //             e2->setParameter(NBTrafficLightDefinition::OSM_DIRECTION, "backward");
    //         } else if (edge->getParameter(NBTrafficLightDefinition::OSM_DIRECTION) == "backward") {
    //             e2->setParameter(NBTrafficLightDefinition::OSM_DIRECTION, "forward");
    //         }
    //         if (edge->getDistance() != 0) {
    //             e2->setDistance(-edge->getDistance() - edge->getLoadedLength());
    //         }
    //         ec.insert(e2);
    //         if (ec.retrieve(id2) == nullptr) {
    //             WRITE_WARNINGF(TL("Bidi-edge '%' prevented by filtering rules."), id2);
    //             return nullptr;
    //         }
    //         if (update) {
    //             updateTurns(edge);
    //             // reconnected added edges
    //             for (NBEdge* incoming : e2->getFromNode()->getIncomingEdges()) {
    //                 if (hasRailway(incoming->getPermissions())) {
    //                     incoming->invalidateConnections(true);
    //                 }
    //             }
    //         }
    //         return e2;
    //     } else {
    //         WRITE_WARNINGF(TL("Could not add bidi-edge '%'."), id2);
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_bidi_edge(ec: &mut NBEdgeCont, edge: Ptr<NBEdge>, update: bool) -> Ptr<NBEdge> {
        debug_assert!(edge.get_lane_spread_function() == LaneSpreadFunction::CENTER);
        debug_assert!(!edge.is_bidi_rail(false));
        let id2 = if edge.get_id().starts_with('-') {
            edge.get_id()[1..].to_string()
        } else {
            format!("-{}", edge.get_id())
        };
        if ec.was_ignored(id2.clone()) {
            // we had it before so the warning is already there
            return Ptr::null();
        }
        if ec.retrieve(&id2, false).is_null() {
            // C++ NBEdge(id, from, to, tpl, geom) constructor: new_2 with the template's values, then the copies below
            let geom = edge.get_geometry().reverse();
            let tryIgnoreNodePositions = !geom.0.is_empty();
            let mut e2box = NBEdge::new_2(
                &id2,
                edge.get_to_node(),
                edge.get_from_node(),
                edge.get_type_id().to_string(),
                edge.get_speed(),
                edge.get_friction(),
                edge.get_num_lanes(),
                edge.get_priority(),
                edge.get_lane_width(),
                edge.get_end_offset(),
                geom,
                edge.get_lane_spread_function(),
                edge.get_street_name(),
                "",
                tryIgnoreNodePositions,
            );
            let e2 = Ptr::from_raw(Box::into_raw(e2box));
            for i in 0..e2.get_num_lanes() {
                let tplIndex = std::cmp::min(i, edge.get_num_lanes() - 1);
                e2.m().set_speed(i, edge.get_lane_speed(tplIndex));
                e2.m().set_friction(i, edge.get_lane_friction(tplIndex));
                e2.m().set_permissions(edge.get_permissions(tplIndex), i);
                let lane = &edge.get_lanes()[tplIndex as usize];
                e2.m().set_lane_width(i, lane.width);
                e2.m().set_lane_type(i, &lane.type_);
                e2.m().myLanes[i as usize].update_parameters(lane.get_parameters_map());
                if e2.get_to_node() == edge.get_to_node() {
                    e2.m().set_end_offset(i, lane.endOffset);
                    e2.m().set_edge_stop_offset(i, &lane.laneStopOffset, false);
                }
            }
            if e2.get_to_node() == edge.get_to_node() {
                e2.m().mySignalPosition = edge.mySignalPosition;
                e2.m().mySignalNode = edge.mySignalNode;
            }
            e2.m().myIsBidi = edge.myIsBidi;
            // geom is edge.getGeometry().reverse() by construction, so the C++ loaded-length geometry check holds
            if edge.myLoadedLength > 0.0 {
                e2.m().myLoadedLength = edge.myLoadedLength;
            }
            e2.m().base2.update_parameters(edge.base2.get_parameters_map());
            if edge.base2.get_parameter(OSM_DIRECTION, "") == "forward" {
                e2.m().base2.set_parameter(OSM_DIRECTION, "backward");
            } else if edge.base2.get_parameter(OSM_DIRECTION, "") == "backward" {
                e2.m().base2.set_parameter(OSM_DIRECTION, "forward");
            }
            if edge.get_distance() != 0.0 {
                e2.m().set_distance(-edge.get_distance() - edge.get_loaded_length());
            }
            ec.insert(e2, false);
            if ec.retrieve(&id2, false).is_null() {
                write_warningf!("Bidi-edge '%' prevented by filtering rules.", id2.clone());
                return Ptr::null();
            }
            if update {
                Self::update_turns(edge);
                // reconnected added edges
                for incoming in e2.get_from_node().get_incoming_edges().iter() {
                    if Self::has_railway(incoming.get_permissions(-1)) {
                        incoming.m().invalidate_connections(true);
                    }
                }
            }
            e2
        } else {
            write_warningf!("Could not add bidi-edge '%'.", id2.clone());
            Ptr::null()
        }
    }

    /// C++: `int NBRailwayTopologyAnalyzer::extendBidiEdges(NBEdgeCont &)` [static, private]
    /// add further bidi-edges near existing bidi-edges
    // int
    // NBRailwayTopologyAnalyzer::extendBidiEdges(NBEdgeCont& ec) {
    //     int added = 0;
    //     for (auto it = ec.begin(); it != ec.end(); it++) {
    //         NBEdge* e = it->second;
    //         if (e->isBidiRail()) {
    //             added += extendBidiEdges(ec, e->getFromNode(), e->getTurnDestination(true));
    //             added += extendBidiEdges(ec, e->getToNode(), e);
    //         }
    //     }
    //     if (added > 0) {
    //         WRITE_MESSAGEF(TL("Added % bidi-edges as extension of existing bidi edges."), toString(added));
    //     }
    //     return added;
    // }
    #[allow(unused_variables)]
    pub fn extend_bidi_edges(ec: &mut NBEdgeCont) -> i32 {
        // C++ iterates the live map while addBidiEdge inserts into it: each step re-reads the map after the last key
        let mut added = 0;
        let mut cursor: Option<String> = None;
        loop {
            let next = match &cursor {
                None => ec.myEdges.iter().next(),
                Some(k) => ec.myEdges.range::<String, _>((std::ops::Bound::Excluded(k), std::ops::Bound::Unbounded)).next(),
            };
            let Some((key, e)) = next.map(|(k, e)| (k.clone(), *e)) else { break };
            cursor = Some(key);
            if e.is_bidi_rail(false) {
                added += Self::extend_bidi_edges_2(ec, e.get_from_node(), e.get_turn_destination(true));
                added += Self::extend_bidi_edges_2(ec, e.get_to_node(), e);
            }
        }
        if added > 0 {
            write_messagef!("Added % bidi-edges as extension of existing bidi edges.", added);
        }
        added
    }
    // int
    // NBRailwayTopologyAnalyzer::extendBidiEdges(NBEdgeCont& ec, NBNode* node, NBEdge* bidiIn) {
    //     assert(bidiIn->getToNode() == node);
    //     NBEdge* bidiOut = bidiIn->getTurnDestination(true);
    //     if (bidiOut == nullptr) {
    //         WRITE_WARNINGF(TL("Could not find bidi-edge for edge '%'"), bidiIn->getID());
    //         return 0;
    //     }
    //     EdgeVector tmpBidiOut;
    //     tmpBidiOut.push_back(bidiOut);
    //     EdgeVector tmpBidiIn;
    //     tmpBidiIn.push_back(bidiIn);
    //     int added = 0;
    //     EdgeVector inRail, outRail;
    //     getRailEdges(node, inRail, outRail);
    //     for (NBEdge* cand : outRail) {
    //         //std::cout << " extendBidiEdges n=" << node->getID() << " bidiIn=" << bidiIn->getID() << " cand=" << cand->getID() << " isStraight=" << isStraight(node, bidiIn, cand) <<  " allSharp=" << allSharp(node, inRail, tmpBidiOut, true) << "\n";
    //         if (!cand->isBidiRail() && isStraight(node, bidiIn, cand)
    //                 && cand->getLaneSpreadFunction() == LaneSpreadFunction::CENTER
    //                 && allSharp(node, inRail, tmpBidiOut, true)) {
    //             NBEdge* e2 = addBidiEdge(ec, cand);
    //             if (e2 != nullptr) {
    //                 added += 1 + extendBidiEdges(ec, cand->getToNode(), cand);
    //             }
    //         }
    //     }
    //     for (NBEdge* cand : inRail) {
    //         //std::cout << " extendBidiEdges n=" << node->getID() << " bidiOut=" << bidiOut->getID() << " cand=" << cand->getID() << " isStraight=" << isStraight(node, cand, bidiOut) << " allSharp=" << allSharp(node, outRail, tmpBidiIn, true) << "\n";
    //         if (!cand->isBidiRail() && isStraight(node, cand, bidiOut)
    //                 && cand->getLaneSpreadFunction() == LaneSpreadFunction::CENTER
    //                 && allSharp(node, outRail, tmpBidiIn, true)) {
    //             NBEdge* e2 = addBidiEdge(ec, cand);
    //             if (e2 != nullptr) {
    //                 added += 1 + extendBidiEdges(ec, cand->getFromNode(), e2);
    //             }
    //         }
    //     }
    //     return added;
    // }
    #[allow(unused_variables)]
    pub fn extend_bidi_edges_2(ec: &mut NBEdgeCont, node: Ptr<NBNode>, bidiIn: Ptr<NBEdge>) -> i32 {
        assert!(bidiIn.get_to_node() == node);
        let bidiOut = bidiIn.get_turn_destination(true);
        if bidiOut.is_null() {
            write_warningf!("Could not find bidi-edge for edge '%'", bidiIn.get_id());
            return 0;
        }
        let tmpBidiOut: EdgeVector = vec![bidiOut];
        let tmpBidiIn: EdgeVector = vec![bidiIn];
        let mut added = 0;
        let mut inRail = EdgeVector::new();
        let mut outRail = EdgeVector::new();
        Self::get_rail_edges(node, &mut inRail, &mut outRail);
        for cand in outRail.clone() {
            //std::cout << " extendBidiEdges n=" << node->getID() << " bidiIn=" << bidiIn->getID() << " cand=" << cand->getID() << " isStraight=" << isStraight(node, bidiIn, cand) <<  " allSharp=" << allSharp(node, inRail, tmpBidiOut, true) << "\n";
            if !cand.is_bidi_rail(false) && Self::is_straight(node, bidiIn, cand) && cand.get_lane_spread_function() == LaneSpreadFunction::CENTER && Self::all_sharp(node, &inRail, &tmpBidiOut, true) {
                let e2 = Self::add_bidi_edge(ec, cand, true);
                if !e2.is_null() {
                    added += 1 + Self::extend_bidi_edges_2(ec, cand.get_to_node(), cand);
                }
            }
        }
        for cand in inRail.clone() {
            //std::cout << " extendBidiEdges n=" << node->getID() << " bidiOut=" << bidiOut->getID() << " cand=" << cand->getID() << " isStraight=" << isStraight(node, cand, bidiOut) << " allSharp=" << allSharp(node, outRail, tmpBidiIn, true) << "\n";
            if !cand.is_bidi_rail(false) && Self::is_straight(node, cand, bidiOut) && cand.get_lane_spread_function() == LaneSpreadFunction::CENTER && Self::all_sharp(node, &outRail, &tmpBidiIn, true) {
                let e2 = Self::add_bidi_edge(ec, cand, true);
                if !e2.is_null() {
                    added += 1 + Self::extend_bidi_edges_2(ec, cand.get_from_node(), e2);
                }
            }
        }
        added
    }

    /// C++: `int NBRailwayTopologyAnalyzer::reverseEdges(NBEdgeCont &, NBPTStopCont &, NBPTLineCont &)` [static, private]
    /// reverse edges sequences that are to broken nodes on both sides
    // int
    // NBRailwayTopologyAnalyzer::reverseEdges(NBEdgeCont& ec, NBPTStopCont& sc, NBPTLineCont& lc) {
    //     std::set<NBNode*> brokenNodes = getBrokenRailNodes(ec);
    //     // find reversible edge sequences between broken nodes
    //     std::vector<EdgeVector> seqsToReverse;
    //     EdgeSet lineEdges;
    //     for (auto item : lc.getLines()) {
    //         const EdgeVector& route = item.second->getEdges();
    //         lineEdges.insert(route.begin(), route.end());
    //     }
    //     for (NBNode* n : brokenNodes) {
    //         EdgeVector inRail, outRail;
    //         getRailEdges(n, inRail, outRail);
    //         for (NBEdge* start : outRail) {
    //             // only reverse edges where the node would be unbroken afterwards
    //             if (!allBroken(n, start, inRail, outRail)
    //                     || (inRail.size() == 1 && outRail.size() == 1)) {
    // #ifdef DEBUG_SEQSTOREVERSE
    //                 if (n->getID() == DEBUGNODEID) {
    //                     std::cout << " abort at start n=" << n->getID() << " (not all broken)\n";
    //                 }
    // #endif
    //                 continue;
    //             }
    //             //std::cout << " get sequences from " << start->getID() << "\n";
    //             bool forward = true;
    //             EdgeVector seq;
    //             while (forward) {
    //                 if (lineEdges.count(start)) {
    //                     // ptline edges should never be reversed. They are fixed by a different algorithm which requires their original ids and orientation
    //                     break;
    //                 } else if (start->hasParameter(NBTrafficLightDefinition::OSM_DIRECTION)) {
    //                     // do not reverse edges where direction was explicitly set in OSM (including via signal direction)
    // #ifdef DEBUG_SEQSTOREVERSE
    //                     if (n->getID() == DEBUGNODEID) {
    //                         std::cout << " abort at edge=" << start->getID() << " (has OSM_DIRECTION " << start->getParameter(NBTrafficLightDefinition::OSM_DIRECTION) << ")\n";
    //                     }
    // #endif
    //                     break;
    //                 }
    //                 seq.push_back(start);
    //                 //std::cout << " seq=" << toString(seq) << "\n";
    //                 NBNode* n2 = start->getToNode();
    //                 EdgeVector inRail2, outRail2;
    //                 getRailEdges(n2, inRail2, outRail2);
    //                 if (brokenNodes.count(n2) != 0) {
    //                     EdgeVector tmp2;
    //                     tmp2.push_back(start);
    //                     if (allBroken(n2, start, outRail2, inRail2)) {
    //                         seqsToReverse.push_back(seq);
    //                     } else {
    // #ifdef DEBUG_SEQSTOREVERSE
    //                         if (n->getID() == DEBUGNODEID) {
    //                             std::cout << " abort at n2=" << n2->getID() << " (not all broken)\n";
    //                         }
    // #endif
    //                     }
    //                     forward = false;
    //                 } else {
    //                     if (outRail2.size() == 0) {
    //                         // stop at network border
    //                         forward = false;
    // #ifdef DEBUG_SEQSTOREVERSE
    //                         if (n->getID() == DEBUGNODEID) {
    //                             std::cout << " abort at n2=" << n2->getID() << " (border)\n";
    //                         }
    // #endif
    //                     } else if (outRail2.size() > 1 || inRail2.size() > 1) {
    //                         // stop at switch
    //                         forward = false;
    // #ifdef DEBUG_SEQSTOREVERSE
    //                         if (n->getID() == DEBUGNODEID) {
    //                             std::cout << " abort at n2=" << n2->getID() << " (switch)\n";
    //                         }
    // #endif
    //                     } else {
    //                         start = outRail2.front();
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // sort by sequence length
    //     if (seqsToReverse.size() > 0) {
    //         WRITE_MESSAGEF(TL("Found % reversible edge sequences between broken rail nodes"), toString(seqsToReverse.size()));
    //     }
    //     std::sort(seqsToReverse.begin(), seqsToReverse.end(),
    //     [](const EdgeVector & a, const EdgeVector & b) {
    //         return a.size() < b.size();
    //     });
    //     int numReversed = 0;
    //     std::set<NBNode*> affectedEndpoints;
    //     std::set<std::string> reversedIDs;
    //     std::map<int, int> seqLengths;
    //     for (EdgeVector& seq : seqsToReverse) {
    //         NBNode* seqStart = seq.front()->getFromNode();
    //         NBNode* seqEnd = seq.back()->getToNode();
    //         // avoid reversing sequences on both sides of a broken node
    //         if (affectedEndpoints.count(seqStart) == 0
    //                 && affectedEndpoints.count(seqEnd) == 0) {
    //             affectedEndpoints.insert(seqStart);
    //             affectedEndpoints.insert(seqEnd);
    //             //WRITE_MESSAGE("  reversed seq=" + toString(seq));
    //             for (NBEdge* e : seq) {
    //                 reverseEdge(e);
    //                 reversedIDs.insert(e->getID());
    //             }
    //             seqLengths[(int)seq.size()]++;
    //             numReversed++;
    //         }
    //     }
    //     if (numReversed > 0) {
    //         WRITE_MESSAGEF(TL("Reversed % sequences (count by length: %)"), toString(numReversed), joinToString(seqLengths, " ", ":"));
    //         for (auto& item : sc.getStops()) {
    //             if (reversedIDs.count(item.second->getEdgeId())) {
    //                 item.second->findLaneAndComputeBusStopExtent(ec);
    //             }
    //         }
    //     }
    //     return numReversed;
    // }
    #[allow(unused_variables)]
    pub fn reverse_edges(ec: &mut NBEdgeCont, sc: &mut NBPTStopCont, lc: &mut NBPTLineCont) -> i32 {
        // determinism: C++ iterates std::set<NBNode*> (pointer order) for brokenNodes
        let brokenNodes = Self::get_broken_rail_nodes(ec, false);
        // find reversible edge sequences between broken nodes
        let mut seqsToReverse: Vec<EdgeVector> = Vec::new();
        let mut lineEdges: EdgeSet = EdgeSet::new();
        for item in lc.get_lines().values() {
            let route = item.get_edges();
            lineEdges.extend(route.iter().copied());
        }
        for n in brokenNodes.iter() {
            let mut inRail = EdgeVector::new();
            let mut outRail = EdgeVector::new();
            Self::get_rail_edges(*n, &mut inRail, &mut outRail);
            for mut start in outRail.clone() {
                // only reverse edges where the node would be unbroken afterwards
                if !Self::all_broken(*n, start, &inRail, &outRail) || (inRail.len() == 1 && outRail.len() == 1) {
                    continue;
                }
                let mut forward = true;
                let mut seq: EdgeVector = EdgeVector::new();
                while forward {
                    if lineEdges.contains(&start) {
                        // ptline edges should never be reversed. They are fixed by a different algorithm which requires their original ids and orientation
                        break;
                    }
                    seq.push(start);
                    //std::cout << " seq=" << toString(seq) << "\n";
                    let n2 = start.get_to_node();
                    let mut inRail2 = EdgeVector::new();
                    let mut outRail2 = EdgeVector::new();
                    Self::get_rail_edges(n2, &mut inRail2, &mut outRail2);
                    if brokenNodes.contains(&n2) {
                        if Self::all_broken(n2, start, &outRail2, &inRail2) {
                            seqsToReverse.push(seq.clone());
                        }
                        forward = false;
                    } else {
                        if outRail2.len() == 0 {
                            // stop at network border
                            forward = false;
                        } else if outRail2.len() > 1 || inRail2.len() > 1 {
                            // stop at switch
                            forward = false;
                        } else {
                            start = outRail2[0];
                        }
                    }
                }
            }
        }
        // sort by sequence length
        if seqsToReverse.len() > 0 {
            write_messagef!("Found % reversible edge sequences between broken rail nodes", seqsToReverse.len());
        }
        // determinism: C++ std::sort is unstable and the comparator only orders by length; std_sort reproduces libstdc++ tie order
        utils::common::std_sort::std_sort_vec(&mut seqsToReverse, |a, b| a.len() < b.len());
        let mut numReversed = 0;
        let mut affectedEndpoints: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let mut reversedIDs: BTreeSet<String> = BTreeSet::new();
        let mut seqLengths: BTreeMap<i32, i32> = BTreeMap::new();
        for seq in seqsToReverse.iter() {
            let seqStart = seq[0].get_from_node();
            let seqEnd = seq[seq.len() - 1].get_to_node();
            // avoid reversing sequences on both sides of a broken node
            if !affectedEndpoints.contains(&seqStart) && !affectedEndpoints.contains(&seqEnd) {
                affectedEndpoints.insert(seqStart);
                affectedEndpoints.insert(seqEnd);
                for e in seq.iter() {
                    Self::reverse_edge(*e);
                    reversedIDs.insert(e.get_id().to_string());
                }
                *seqLengths.entry(seq.len() as i32).or_insert(0) += 1;
                numReversed += 1;
            }
        }
        if numReversed > 0 {
            let joined = seqLengths.iter().map(|(k, v)| format!("{}:{}", k, v)).collect::<Vec<_>>().join(" ");
            write_messagef!("Reversed % sequences (count by length: %)", numReversed, joined);
            for item in sc.get_stops().values() {
                if reversedIDs.contains(item.get_edge_id()) {
                    item.m().find_lane_and_compute_bus_stop_extent(ec);
                }
            }
        }
        numReversed
    }

    /// C++: `void NBRailwayTopologyAnalyzer::reverseEdge(NBEdge *)` [static, private]
    /// reverse a single edge
    // void
    // NBRailwayTopologyAnalyzer::reverseEdge(NBEdge* e) {
    //     e->reinitNodes(e->getToNode(), e->getFromNode());
    //     e->setGeometry(e->getGeometry().reverse());
    //     if (e->getParameter(NBTrafficLightDefinition::OSM_DIRECTION) == "forward") {
    //         e->setParameter(NBTrafficLightDefinition::OSM_DIRECTION, "backward");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reverse_edge(e: Ptr<NBEdge>) {
        e.m().reinit_nodes(e.get_to_node(), e.get_from_node());
        let reversed = e.get_geometry().reverse();
        e.m().set_geometry(&reversed, false);
        if e.base2.get_parameter(OSM_DIRECTION, "") == "forward" {
            e.m().base2.set_parameter(OSM_DIRECTION, "backward");
        }
    }

    /// C++: `int NBRailwayTopologyAnalyzer::addBidiEdgesForBufferStops(NBEdgeCont &)` [static, private]
    /// add bidi-edges to connect buffers stops in both directions
    // int
    // NBRailwayTopologyAnalyzer::addBidiEdgesForBufferStops(NBEdgeCont& ec) {
    //     std::set<NBNode*> brokenNodes = getBrokenRailNodes(ec);
    //     std::set<NBNode*> railNodes = getRailNodes(ec);
    //     // find buffer stops and ensure that they are connect to the network in both directions
    //     int numBufferStops = 0;
    //     int numAddedBidiTotal = 0;
    //     for (NBNode* node : railNodes) {
    //         if (StringUtils::toBool(node->getParameter("buffer_stop", "false"))) {
    //             if (node->getEdges().size() != 1) {
    //                 WRITE_WARNINGF(TL("Ignoring buffer stop junction '%' with % edges."), node->getID(), node->getEdges().size());
    //                 continue;
    //             }
    //             // int numAddedBidi = 0;
    //             numBufferStops++;
    //             NBEdge* prev = nullptr;
    //             NBEdge* prev2 = nullptr;
    //             EdgeVector inRail, outRail;
    //             getRailEdges(node, inRail, outRail);
    //             bool addAway = true; // add new edges away from buffer stop
    //             while (prev == nullptr || (inRail.size() + outRail.size()) == 3) {
    //                 NBEdge* e = nullptr;
    //                 if (prev == nullptr) {
    //                     assert(node->getEdges().size() == 1);
    //                     e = node->getEdges().front();
    //                     addAway = node == e->getToNode();
    //                 } else {
    //                     if (addAway) {
    //                         // XXX if node is broken we need to switch direction
    //                         assert(inRail.size() == 2);
    //                         e = inRail.front() == prev2 ? inRail.back() : inRail.front();
    //                     } else {
    //                         // XXX if node is broken we need to switch direction
    //                         assert(outRail.size() == 2);
    //                         e = outRail.front() == prev2 ? outRail.back() : outRail.front();
    //                     }
    //                 }
    //                 e->setLaneSpreadFunction(LaneSpreadFunction::CENTER);
    //                 NBNode* e2From = nullptr;
    //                 NBNode* e2To = nullptr;
    //                 if (addAway) {
    //                     e2From = node;
    //                     e2To = e->getFromNode();
    //                     node = e2To;
    //                 } else {
    //                     e2From = e->getToNode();
    //                     e2To = node;
    //                     node = e2From;
    //                 }
    //                 NBEdge* e2 = addBidiEdge(ec, e);
    //                 if (e2 == nullptr) {
    //                     break;
    //                 }
    //                 prev = e;
    //                 prev2 = e2;
    //                 // numAddedBidi++;
    //                 numAddedBidiTotal++;
    //                 inRail.clear();
    //                 outRail.clear();
    //                 getRailEdges(node, inRail, outRail);
    //             }
    //             //if (numAddedBidi > 0) {
    //             //    WRITE_MESSAGEF(TL(" added % edges between buffer stop junction '%' and junction '%'"), toString(numAddedBidi), bufferStop->getID(), node->getID());
    //             //}
    //         }
    //     }
    //     if (numAddedBidiTotal > 0) {
    //         WRITE_MESSAGEF(TL("Added % edges to connect % buffer stops in both directions."), toString(numAddedBidiTotal), toString(numBufferStops));
    //     }
    //     return numAddedBidiTotal;
    // }
    #[allow(unused_variables)]
    pub fn add_bidi_edges_for_buffer_stops(ec: &mut NBEdgeCont) -> i32 {
        let brokenNodes = Self::get_broken_rail_nodes(ec, false);
        let railNodes = Self::get_rail_nodes(ec, false);
        // find buffer stops and ensure that they are connect to the network in both directions
        let mut numBufferStops = 0;
        let mut numAddedBidiTotal = 0;
        for n in railNodes.iter() {
            let mut node = *n;
            if to_bool(&node.base2.get_parameter("buffer_stop", "false")).unwrap() {
                if node.get_edges().len() != 1 {
                    write_warningf!("Ignoring buffer stop junction '%' with % edges.", node.get_id(), node.get_edges().len());
                    continue;
                }
                // int numAddedBidi = 0;
                numBufferStops += 1;
                let mut prev: Ptr<NBEdge> = Ptr::null();
                let mut prev2: Ptr<NBEdge> = Ptr::null();
                let mut inRail = EdgeVector::new();
                let mut outRail = EdgeVector::new();
                Self::get_rail_edges(node, &mut inRail, &mut outRail);
                let mut addAway = true; // add new edges away from buffer stop
                while prev.is_null() || (inRail.len() + outRail.len()) == 3 {
                    let e: Ptr<NBEdge>;
                    if prev.is_null() {
                        debug_assert!(node.get_edges().len() == 1);
                        e = node.get_edges()[0];
                        addAway = node == e.get_to_node();
                    } else {
                        // XXX if node is broken we need to switch direction
                        if addAway {
                            debug_assert!(inRail.len() == 2);
                            e = if inRail[0] == prev2 { inRail[inRail.len() - 1] } else { inRail[0] };
                        } else {
                            // XXX if node is broken we need to switch direction
                            debug_assert!(outRail.len() == 2);
                            e = if outRail[0] == prev2 { outRail[outRail.len() - 1] } else { outRail[0] };
                        }
                    }
                    e.m().set_lane_spread_function(LaneSpreadFunction::CENTER);
                    if addAway {
                        node = e.get_from_node();
                    } else {
                        node = e.get_to_node();
                    }
                    let e2 = Self::add_bidi_edge(ec, e, true);
                    if e2.is_null() {
                        break;
                    }
                    prev = e;
                    prev2 = e2;
                    // numAddedBidi++;
                    numAddedBidiTotal += 1;
                    inRail.clear();
                    outRail.clear();
                    Self::get_rail_edges(node, &mut inRail, &mut outRail);
                }
                //if (numAddedBidi > 0) {
                //    WRITE_MESSAGEF(TL(" added % edges between buffer stop junction '%' and junction '%'"), toString(numAddedBidi), bufferStop->getID(), node->getID());
                //}
            }
        }
        if numAddedBidiTotal > 0 {
            write_messagef!("Added % edges to connect % buffer stops in both directions.", numAddedBidiTotal, numBufferStops);
        }
        numAddedBidiTotal
    }

    /// C++: `int NBRailwayTopologyAnalyzer::addBidiEdgesBetweenSwitches(NBEdgeCont &)` [static, private]
    /// add bidi-edges to connect switches that are approached in both directions
    // int
    // NBRailwayTopologyAnalyzer::addBidiEdgesBetweenSwitches(NBEdgeCont& ec) {
    //     std::set<NBNode*> brokenNodes = getBrokenRailNodes(ec);
    //     std::map<int, int> seqLengths;
    //     int numAdded = 0;
    //     int numSeqs = 0;
    //     for (NBNode* n : brokenNodes) {
    //         NBEdge* edge = isBidiSwitch(n);
    //         if (edge != nullptr && edge->getLaneSpreadFunction() == LaneSpreadFunction::CENTER) {
    //             std::vector<NBNode*> nodeSeq;
    //             EdgeVector edgeSeq;
    //             NBNode* prev = n;
    //             nodeSeq.push_back(prev);
    //             edgeSeq.push_back(edge);
    //             bool forward = true;
    //             //std::cout << "Looking for potential bidi-edge sequence starting at junction '" << n->getID() << "' with edge '" + edge->getID() << "'\n";
    //             // find a suitable end point for adding bidi edges
    //             while (forward) {
    //                 NBNode* next = edge->getFromNode() == prev ? edge->getToNode() : edge->getFromNode();
    //                 EdgeVector allRail;
    //                 getRailEdges(next, allRail, allRail);
    //                 if (allRail.size() == 2 && isStraight(next, allRail.front(), allRail.back())) {
    //                     prev = next;
    //                     edge = allRail.front() == edge ? allRail.back() : allRail.front();
    //                     nodeSeq.push_back(prev);
    //                     edgeSeq.push_back(edge);
    //                 } else {
    //                     forward = false;
    //                     EdgeVector inRail2, outRail2;
    //                     getRailEdges(next, inRail2, outRail2);
    //                     if (isBidiSwitch(next) == edge) {
    //                         // suitable switch found as endpoint, add reverse edges
    //                         //WRITE_MESSAGE("Adding " + toString(edgeSeq.size())
    //                         //        + " bidi-edges between switches junction '" + n->getID() + "' and junction '" + next->getID() + "'");
    //                         for (NBEdge* e : edgeSeq) {
    //                             addBidiEdge(ec, e);
    //                         }
    //                         seqLengths[(int)edgeSeq.size()]++;
    //                         numSeqs++;
    //                         numAdded += (int)edgeSeq.size();
    //                     } else {
    //                         //std::cout << " sequence ended at junction " << next->getID()
    //                         //    << " in=" << inRail2.size()
    //                         //    << " out=" << outRail2.size()
    //                         //    << " bidiSwitch=" << Named::getIDSecure(isBidiSwitch(next))
    //                         //    << "\n";
    //                     }
    //
    //                 }
    //             }
    //
    //         }
    //     }
    //     if (seqLengths.size() > 0) {
    //         WRITE_MESSAGEF(TL("Added % bidi-edges between % pairs of railway switches (count by length: %)"), toString(numAdded), toString(numSeqs), joinToString(seqLengths, " ", ":"));
    //     }
    //     return numAdded;
    // }
    #[allow(unused_variables)]
    pub fn add_bidi_edges_between_switches(ec: &mut NBEdgeCont) -> i32 {
        let brokenNodes = Self::get_broken_rail_nodes(ec, false);
        let mut seqLengths: BTreeMap<i32, i32> = BTreeMap::new();
        let mut numAdded = 0;
        let mut numSeqs = 0;
        for n in brokenNodes.iter() {
            let mut edge = Self::is_bidi_switch(*n);
            if !edge.is_null() && edge.get_lane_spread_function() == LaneSpreadFunction::CENTER {
                let mut prev = *n;
                let mut edgeSeq: EdgeVector = vec![edge];
                let mut forward = true;
                //std::cout << "Looking for potential bidi-edge sequence starting at junction '" << n->getID() << "' with edge '" + edge->getID() << "'\n";
                // find a suitable end point for adding bidi edges
                while forward {
                    let next = if edge.get_from_node() == prev { edge.get_to_node() } else { edge.get_from_node() };
                    let mut allRail = EdgeVector::new();
                    let mut allRailOut = EdgeVector::new();
                    Self::get_rail_edges(next, &mut allRail, &mut allRailOut);
                    allRail.extend(allRailOut);
                    if allRail.len() == 2 && Self::is_straight(next, allRail[0], allRail[allRail.len() - 1]) {
                        prev = next;
                        edge = if allRail[0] == edge { allRail[allRail.len() - 1] } else { allRail[0] };
                        edgeSeq.push(edge);
                    } else {
                        forward = false;
                        if Self::is_bidi_switch(next) == edge {
                            // suitable switch found as endpoint, add reverse edges
                            for e in edgeSeq.iter() {
                                Self::add_bidi_edge(ec, *e, true);
                            }
                            *seqLengths.entry(edgeSeq.len() as i32).or_insert(0) += 1;
                            numSeqs += 1;
                            numAdded += edgeSeq.len() as i32;
                        } else {
                            //std::cout << " sequence ended at junction " << next->getID() ...
                        }
                    }
                }
            }
        }
        if seqLengths.len() > 0 {
            let joined = seqLengths.iter().map(|(k, v)| format!("{}:{}", k, v)).collect::<Vec<_>>().join(" ");
            write_messagef!("Added % bidi-edges between % pairs of railway switches (count by length: %)", numAdded, numSeqs, joined);
        }
        numAdded
    }

    /// C++: `int NBRailwayTopologyAnalyzer::addBidiEdgesForStops(NBEdgeCont &, NBPTLineCont &, NBPTStopCont &, bool)` [static, private]
    /// add bidi-edges to connect successive public transport stops
    // int
    // NBRailwayTopologyAnalyzer::addBidiEdgesForStops(NBEdgeCont& ec, NBPTLineCont& lc, NBPTStopCont& sc, bool minimal) {
    //     const double penalty = OptionsCont::getOptions().getFloat("railway.topology.repair.bidi-penalty");
    //     // generate bidirectional routing graph
    //     std::vector<Track*> tracks;
    //     for (NBEdge* edge : ec.getAllEdges()) {
    //         tracks.push_back(new Track(edge));
    //     }
    //     const int numEdges = (int)tracks.size();
    //     for (NBEdge* edge : ec.getAllEdges()) {
    //         tracks.push_back(new Track(edge, (int)tracks.size(), edge->getID() + "_reverse", penalty));
    //     }
    //     // add special tracks for starting end ending in both directions
    //     std::map<NBEdge*, std::pair<Track*, Track*> > stopTracks;
    //     for (NBEdge* edge : ec.getAllEdges()) {
    //         if ((edge->getPermissions() & SVC_RAIL_CLASSES) != 0) {
    //             Track* start = new Track(edge, (int)tracks.size(), edge->getID() + "_start");
    //             tracks.push_back(start);
    //             Track* end = new Track(edge, (int)tracks.size(), edge->getID() + "_end");
    //             tracks.push_back(end);
    //             stopTracks[edge] = {start, end};
    //         }
    //     }
    //     // set successors based on angle (connections are not yet built)
    //     for (NBNode* node : getRailNodes(ec)) {
    //         EdgeVector railEdges;
    //         getRailEdges(node, railEdges, railEdges);
    //         for (NBEdge* e1 : railEdges) {
    //             for (NBEdge* e2 : railEdges) {
    //                 if (e1 != e2 && isStraight(node, e1, e2)) {
    //                     int i = e1->getNumericalID();
    //                     int i2 = e2->getNumericalID();
    //                     if (e1->getToNode() == node) {
    //                         if (e2->getFromNode() == node) {
    //                             // case 1) plain forward connection
    //                             tracks[i]->addSuccessor(tracks[i2]);
    //                             // reverse edge (numerical id incremented by numEdges)
    //                             tracks[i2 + numEdges]->addSuccessor(tracks[i + numEdges]);
    //                         } else {
    //                             // case 2) both edges pointing towards each other
    //                             tracks[i]->addSuccessor(tracks[i2 + numEdges]);
    //                             tracks[i2]->addSuccessor(tracks[i + numEdges]);
    //                         }
    //                     } else {
    //                         if (e2->getFromNode() == node) {
    //                             // case 3) both edges pointing away from each other
    //                             tracks[i + numEdges]->addSuccessor(tracks[i2]);
    //                             tracks[i2 + numEdges]->addSuccessor(tracks[i]);
    //                         } else {
    //                             // already handled via case 1)
    //                         }
    //                     }
    //
    //                 }
    //             }
    //         }
    //     }
    //     // define start and end successors
    //     for (auto& item : stopTracks) {
    //         const int index = item.first->getNumericalID();
    //         // start
    //         item.second.first->addSuccessor(tracks[index]);
    //         item.second.first->addSuccessor(tracks[index + numEdges]);
    //         // end
    //         tracks[index]->addSuccessor(item.second.second);
    //         tracks[index + numEdges]->addSuccessor(item.second.second);
    //     }
    //     // DEBUG
    //     /*
    //     for (Track* t : tracks) {
    //         std::cout << "track " << t->getID() << " e=" << t->edge->getID() << " i=" << t->getNumericalID() << " succ:\n";
    //         for (Track* s : t->getSuccessors(SVC_IGNORING)) {
    //             std::cout << "   succ=" << s->getID() << "\n";
    //         }
    //     }
    //     */
    //
    //     SUMOAbstractRouter<Track, NBVehicle>* const router = new DijkstraRouter<Track, NBVehicle>(
    //         tracks, true, &NBRailwayTopologyAnalyzer::getTravelTimeStatic, nullptr, true);
    //
    //     int added = 0;
    //     int numDisconnected = 0;
    //     std::set<std::shared_ptr<NBPTStop>> addBidiStops;
    //     std::set<NBEdge*, ComparatorIdLess> addBidiStopEdges;
    //     std::set<NBEdge*, ComparatorIdLess> addBidiEdges;
    //     std::set<std::pair<std::string, std::string> > visited;
    //
    //     // the isConsistent heuristic may fail in some cases. If we observe that a
    //     // specific sequence of stop ids in encoded in both directions, we take this
    //     // as a reason to overrule the heuristic
    //     std::set<NBPTLine*> requireBidi = findBidiCandidates(lc);
    //
    //     for (const auto& item : lc.getLines()) {
    //         NBPTLine* line = item.second;
    //         std::vector<NBPTLine::PTStopInfo> stops = line->getStopEdges(ec);
    //         std::vector<NBEdge*> stopEdges;
    //         for (auto it : stops) {
    //             stopEdges.push_back(it.edge);
    //         }
    //         NBEdge* routeStart = line->getRouteStart(ec);
    //         NBEdge* routeEnd = line->getRouteEnd(ec);
    //         if (routeStart != nullptr && (stopEdges.empty() || routeStart != stopEdges.front())) {
    //             stops.insert(stops.begin(), NBPTLine::PTStopInfo(routeStart, routeStart->getID(), 0, false));
    //         }
    //         if (routeEnd != nullptr && (stopEdges.empty() || routeEnd != stopEdges.back())) {
    //             stops.push_back(NBPTLine::PTStopInfo(routeEnd, routeEnd->getID(), routeEnd->getLength(), false));
    //         }
    //         if (stops.size() < 2) {
    //             continue;
    //         }
    //         if (!line->isConsistent(stopEdges) && requireBidi.count(line) == 0) {
    //             WRITE_WARNINGF(TL("Edge sequence is not consistent with stop sequence in line '%', not adding bidi edges."), item.first);
    //             continue;
    //         }
    //         for (auto it = stops.begin(); it + 1 != stops.end(); ++it) {
    //             NBEdge* fromEdge = it->edge;
    //             NBEdge* toEdge = (it + 1)->edge;
    //             const std::string fromStop = it->stopID;
    //             const std::string toStop = (it + 1)->stopID;
    //             const double fromPos = it->pos;
    //             const double toPos = (it + 1)->pos;
    //             bool fromRevised = it->revised;
    //             bool toRevised = (it + 1)->revised;
    //             std::pair<std::string, std::string> trip(fromStop, toStop);
    //             std::pair<std::string, std::string> reverseTrip(toStop, fromStop);
    //             //if (line->getLineID() == "147471") {
    //             //    std::cout << " line=" << line->getLineID() << " trip=" << Named::getIDSecure(fromEdge) << "->" << Named::getIDSecure(toEdge) << " visited=" << (visited.count(trip) != 0) << " fromStop=" << fromStop << " toStop=" << toStop << " fromRevised=" << fromRevised << " toRevised=" << toRevised << "\n";
    //             //}
    //             if (visited.count(trip) != 0) {
    //                 continue;
    //             } else {
    //                 visited.insert(trip);
    //             }
    //             if (stopTracks.count(fromEdge) == 0
    //                     || stopTracks.count(toEdge) == 0) {
    //                 continue;
    //             }
    //             bool needBidi = visited.count(reverseTrip) != 0;
    //             NBVehicle veh(line->getRef(), (SUMOVehicleClass)(fromEdge->getPermissions() & SVC_RAIL_CLASSES));
    //             std::vector<const Track*> route;
    //             Track* from = fromRevised ? tracks[fromEdge->getNumericalID()] : stopTracks[fromEdge].first;
    //             Track* to = toRevised ? tracks[toEdge->getNumericalID()] : stopTracks[toEdge].second;
    //             int iStart = fromRevised ? 0 : 1;
    //             int iDeltaEnd = toRevised ? 0 : 1;
    //             if (fromEdge == toEdge && fromPos > toPos) {
    //                 // must use reverse edge
    //                 route.push_back(tracks[fromEdge->getNumericalID() + numEdges]);
    //                 iStart = 0;
    //                 iDeltaEnd = 0;
    //                 needBidi = true;
    //             } else {
    //                 router->compute(from, to, &veh, 0, route);
    //             }
    //             //if (line->getLineID() == "147471") {
    //             //    std::cout << "DEBUG: route=" << toString(route) << "\n";
    //             //}
    //             if (route.size() > 0) {
    //                 assert((int)route.size() > iStart + iDeltaEnd);
    //                 for (int i = iStart; i < (int)route.size() - iDeltaEnd; ++i) {
    //                     const bool isBidi = route[i]->getNumericalID() >= numEdges;
    //                     bool isStop = i == iStart || i == (int)route.size() - 1 - iDeltaEnd;
    //                     if (isBidi || needBidi) {
    //                         NBEdge* edge = route[i]->edge;
    //                         if (addBidiEdges.count(edge) == 0) {
    //                             if (!edge->isBidiRail(true)) {
    //                                 if (edge->getLaneSpreadFunction() == LaneSpreadFunction::CENTER) {
    //                                     addBidiEdges.insert(edge);
    //                                     if (isStop) {
    //                                         addBidiStopEdges.insert(edge);
    //                                     }
    //                                 } else {
    //                                     if (isStop) {
    //                                         WRITE_WARNINGF(TL("Stop on edge '%' can only be reached in reverse but edge has the wrong spreadType."), fromEdge->getID());
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     if (isStop && isBidi) {
    //                         std::shared_ptr<NBPTStop> fs = sc.get(fromStop);
    //                         if (fs) {
    //                             addBidiStops.insert(fs);
    //                         }
    //                         std::shared_ptr<NBPTStop> ts = sc.get(toStop);
    //                         if (ts) {
    //                             addBidiStops.insert(ts);
    //                         }
    //                     }
    //                 }
    //             } else {
    //                 WRITE_WARNINGF(TL("No connection found between stops on edge '%' and edge '%'."), fromEdge->getID(), toEdge->getID());
    //                 numDisconnected++;
    //             }
    //         }
    //     }
    //     for (NBEdge* edge : addBidiEdges) {
    //         if (!edge->isBidiRail()) {
    //             if (edge->getFromNode()->getIncomingEdges().size() == 0
    //                     && edge->getToNode()->getOutgoingEdges().size() == 0) {
    //                 // flip a single broken edge instead of adding the reverse
    //                 reverseEdge(edge);
    //             } else {
    //                 NBEdge* e2 = addBidiEdge(ec, edge);
    //                 //std::cout << " add bidiEdge for stop at edge " << edge->getID() << "\n";
    //                 if (e2 != nullptr) {
    //                     added++;
    //                     if (!minimal) {
    //                         added += extendBidiEdges(ec, edge->getToNode(), edge);
    //                         added += extendBidiEdges(ec, edge->getFromNode(), e2);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     for (std::shared_ptr<NBPTStop> stop : addBidiStops) {
    //         std::shared_ptr<NBPTStop> fromReverse = sc.getReverseStop(stop, ec);
    //         if (fromReverse) {
    //             sc.insert(fromReverse);
    //             stop->setBidiStop(fromReverse);
    //         }
    //     }
    //     if (addBidiEdges.size() > 0 || numDisconnected > 0) {
    //         WRITE_MESSAGE("Added " + toString(addBidiStopEdges.size()) + " bidi-edges for public transport stops and a total of "
    //                       + toString(added) + " bidi-edges to ensure connectivity of stops ("
    //                       + toString(numDisconnected) + " stops remain disconnected)");
    //     }
    //
    //     // clean up
    //     for (Track* t : tracks) {
    //         delete t;
    //     }
    //     delete router;
    //     return (int)addBidiEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn add_bidi_edges_for_stops(ec: &mut NBEdgeCont, lc: &mut NBPTLineCont, sc: &mut NBPTStopCont, minimal: bool) -> i32 {
        type Track = NBRailwayTopologyAnalyzer_Track;
        let penalty = get_float("railway.topology.repair.bidi-penalty");
        // generate bidirectional routing graph
        let mut tracks: Vec<Ptr<Track>> = Vec::new();
        for edge in ec.get_all_edges() {
            tracks.push(leak_track(Track::new(edge, -1, "", 1.0)));
        }
        let numEdges = tracks.len() as i32;
        for edge in ec.get_all_edges() {
            let index = tracks.len() as i32;
            tracks.push(leak_track(Track::new(edge, index, &format!("{}_reverse", edge.get_id()), penalty)));
        }
        // add special tracks for starting end ending in both directions
        let mut stopTracks: BTreeMap<Ptr<NBEdge>, (Ptr<Track>, Ptr<Track>)> = BTreeMap::new();
        for edge in ec.get_all_edges() {
            if (edge.get_permissions(-1) & SVC_RAIL_CLASSES) != 0 {
                let startIndex = tracks.len() as i32;
                let start = leak_track(Track::new(edge, startIndex, &format!("{}_start", edge.get_id()), 1.0));
                tracks.push(start);
                let endIndex = tracks.len() as i32;
                let end = leak_track(Track::new(edge, endIndex, &format!("{}_end", edge.get_id()), 1.0));
                tracks.push(end);
                stopTracks.insert(edge, (start, end));
            }
        }
        // set successors based on angle (connections are not yet built)
        for node in Self::get_rail_nodes(ec, false).iter() {
            // C++ getRailEdges(node, railEdges, railEdges) appends the incoming and then the outgoing rail edges
            let mut inRail = EdgeVector::new();
            let mut outRail = EdgeVector::new();
            Self::get_rail_edges(*node, &mut inRail, &mut outRail);
            let mut railEdges = inRail;
            railEdges.extend(outRail);
            for e1 in railEdges.iter() {
                for e2 in railEdges.iter() {
                    if e1 != e2 && Self::is_straight(*node, *e1, *e2) {
                        let i = e1.get_numerical_id();
                        let i2 = e2.get_numerical_id();
                        if e1.get_to_node() == *node {
                            if e2.get_from_node() == *node {
                                // case 1) plain forward connection
                                tracks[i as usize].m().add_successor(tracks[i2 as usize]);
                                // reverse edge (numerical id incremented by numEdges)
                                tracks[(i2 + numEdges) as usize].m().add_successor(tracks[(i + numEdges) as usize]);
                            } else {
                                // case 2) both edges pointing towards each other
                                tracks[i as usize].m().add_successor(tracks[(i2 + numEdges) as usize]);
                                tracks[i2 as usize].m().add_successor(tracks[(i + numEdges) as usize]);
                            }
                        } else {
                            if e2.get_from_node() == *node {
                                // case 3) both edges pointing away from each other
                                tracks[(i + numEdges) as usize].m().add_successor(tracks[i2 as usize]);
                                tracks[(i2 + numEdges) as usize].m().add_successor(tracks[i as usize]);
                            } else {
                                // already handled via case 1)
                            }
                        }
                    }
                }
            }
        }
        // define start and end successors
        for (edge, (start, end)) in stopTracks.iter() {
            let index = edge.get_numerical_id();
            // start
            start.m().add_successor(tracks[index as usize]);
            start.m().add_successor(tracks[(index + numEdges) as usize]);
            // end
            tracks[index as usize].m().add_successor(*end);
            tracks[(index + numEdges) as usize].m().add_successor(*end);
        }
        // DEBUG
        /*
        for (Track* t : tracks) {
            std::cout << "track " << t->getID() << " e=" << t->edge->getID() << " i=" << t->getNumericalID() << " succ:\n";
            for (Track* s : t->getSuccessors(SVC_IGNORING)) {
                std::cout << "   succ=" << s->getID() << "\n";
            }
        }
        */

        let mut router = DijkstraRouter::new(&tracks, true, Some(NBRailwayTopologyAnalyzer::get_travel_time_static), None, true, Ptr::null(), false, false);
        let mut added = 0;
        let mut numDisconnected = 0;
        // determinism: std::set<std::shared_ptr<NBPTStop>> and the edge sets are ordered by id here
        let mut addBidiStops: BTreeMap<String, Ptr<NBPTStop>> = BTreeMap::new();
        let mut addBidiStopEdges: BTreeMap<String, Ptr<NBEdge>> = BTreeMap::new();
        let mut addBidiEdges: BTreeMap<String, Ptr<NBEdge>> = BTreeMap::new();
        let mut visited: BTreeSet<(String, String)> = BTreeSet::new();

        // the isConsistent heuristic may fail in some cases. If we observe that a
        // specific sequence of stop ids in encoded in both directions, we take this
        // as a reason to overrule the heuristic
        let requireBidi = Self::find_bidi_candidates(lc);

        for (lineId, item) in lc.get_lines().iter() {
            let line = *item;
            let mut stops = line.get_stop_edges(ec);
            let stopEdges: EdgeVector = stops.iter().map(|it| it.edge).collect();
            let routeStart = line.get_route_start(ec);
            let routeEnd = line.get_route_end(ec);
            if !routeStart.is_null() && (stopEdges.is_empty() || routeStart != stopEdges[0]) {
                stops.insert(0, NBPTLine_PTStopInfo::new(routeStart, routeStart.get_id(), 0.0, false));
            }
            if !routeEnd.is_null() && (stopEdges.is_empty() || routeEnd != stopEdges[stopEdges.len() - 1]) {
                stops.push(NBPTLine_PTStopInfo::new(routeEnd, routeEnd.get_id(), routeEnd.get_length(), false));
            }
            if stops.len() < 2 {
                continue;
            }
            if !line.is_consistent(stopEdges.clone()) && !requireBidi.contains(&line) {
                write_warningf!("Edge sequence is not consistent with stop sequence in line '%', not adding bidi edges.", lineId.as_str());
                continue;
            }
            for i in 0..stops.len() - 1 {
                let fromEdge = stops[i].edge;
                let toEdge = stops[i + 1].edge;
                let fromStop = stops[i].stopID.clone();
                let toStop = stops[i + 1].stopID.clone();
                let fromPos = stops[i].pos;
                let toPos = stops[i + 1].pos;
                let fromRevised = stops[i].revised;
                let toRevised = stops[i + 1].revised;
                let trip = (fromStop.clone(), toStop.clone());
                let reverseTrip = (toStop.clone(), fromStop.clone());
                if visited.contains(&trip) {
                    continue;
                } else {
                    visited.insert(trip);
                }
                if !stopTracks.contains_key(&fromEdge) || !stopTracks.contains_key(&toEdge) {
                    continue;
                }
                let mut needBidi = visited.contains(&reverseTrip);
                let mut veh = NBVehicle::new(line.get_ref(), fromEdge.get_permissions(-1) & SVC_RAIL_CLASSES);
                let mut route: Vec<Ptr<Track>> = Vec::new();
                let from = if fromRevised { tracks[fromEdge.get_numerical_id() as usize] } else { stopTracks[&fromEdge].0 };
                let to = if toRevised { tracks[toEdge.get_numerical_id() as usize] } else { stopTracks[&toEdge].1 };
                let mut iStart = if fromRevised { 0 } else { 1 };
                let mut iDeltaEnd = if toRevised { 0 } else { 1 };
                if fromEdge == toEdge && fromPos > toPos {
                    // must use reverse edge
                    route.push(tracks[(fromEdge.get_numerical_id() + numEdges) as usize]);
                    iStart = 0;
                    iDeltaEnd = 0;
                    needBidi = true;
                } else {
                    router.compute(from, to, Ptr::from_mut(&mut veh), 0, &mut route, false);
                }
                if route.len() > 0 {
                    debug_assert!(route.len() as i32 > iStart + iDeltaEnd);
                    let mut i2 = iStart;
                    while i2 < route.len() as i32 - iDeltaEnd {
                        let isBidi = route[i2 as usize].get_numerical_id() >= numEdges;
                        let isStop = i2 == iStart || i2 == route.len() as i32 - 1 - iDeltaEnd;
                        if isBidi || needBidi {
                            let edge = route[i2 as usize].edge;
                            if !addBidiEdges.contains_key(edge.get_id()) {
                                if !edge.is_bidi_rail(true) {
                                    if edge.get_lane_spread_function() == LaneSpreadFunction::CENTER {
                                        addBidiEdges.insert(edge.get_id().to_string(), edge);
                                        if isStop {
                                            addBidiStopEdges.insert(edge.get_id().to_string(), edge);
                                        }
                                    } else if isStop {
                                        write_warningf!("Stop on edge '%' can only be reached in reverse but edge has the wrong spreadType.", fromEdge.get_id());
                                    }
                                }
                            }
                        }
                        if isStop && isBidi {
                            if let Some(fs) = sc.get(fromStop.clone()) {
                                addBidiStops.insert(fs.get_id(), fs);
                            }
                            if let Some(ts) = sc.get(toStop.clone()) {
                                addBidiStops.insert(ts.get_id(), ts);
                            }
                        }
                        i2 += 1;
                    }
                } else {
                    write_warningf!("No connection found between stops on edge '%' and edge '%'.", fromEdge.get_id(), toEdge.get_id());
                    numDisconnected += 1;
                }
            }
        }
        for edge in addBidiEdges.values() {
            let edge = *edge;
            if !edge.is_bidi_rail(false) {
                if edge.get_from_node().get_incoming_edges().len() == 0 && edge.get_to_node().get_outgoing_edges().len() == 0 {
                    // flip a single broken edge instead of adding the reverse
                    Self::reverse_edge(edge);
                } else {
                    let e2 = Self::add_bidi_edge(ec, edge, true);
                    //std::cout << " add bidiEdge for stop at edge " << edge->getID() << "\n";
                    if !e2.is_null() {
                        added += 1;
                        if !minimal {
                            added += Self::extend_bidi_edges_2(ec, edge.get_to_node(), edge);
                            added += Self::extend_bidi_edges_2(ec, edge.get_from_node(), e2);
                        }
                    }
                }
            }
        }
        for stop in addBidiStops.values() {
            if let Some(fromReverse) = sc.get_reverse_stop(stop.clone(), ec) {
                sc.insert(fromReverse.clone(), false);
                stop.m().set_bidi_stop(fromReverse);
            }
        }
        if addBidiEdges.len() > 0 || numDisconnected > 0 {
            write_message(&format!(
                "Added {} bidi-edges for public transport stops and a total of {} bidi-edges to ensure connectivity of stops ({} stops remain disconnected)",
                addBidiStopEdges.len(),
                added,
                numDisconnected
            ));
        }

        // clean up
        for t in tracks {
            // SAFETY: every track was created by leak_track() and is not referenced after this point
            unsafe { drop(Box::from_raw(t.as_raw())) };
        }
        addBidiEdges.len() as i32
    }

    /// C++: `int NBRailwayTopologyAnalyzer::addBidiEdgesForStraightConnectivity(NBEdgeCont &, bool)` [static, private]
    /// add bidi-edges to connect straight tracks
    // int
    // NBRailwayTopologyAnalyzer::addBidiEdgesForStraightConnectivity(NBEdgeCont& ec, bool geometryLike) {
    //     int added = 0;
    //     std::set<NBNode*> brokenNodes = getBrokenRailNodes(ec);
    //     for (const auto& e : ec) {
    //         if (!hasRailway(e.second->getPermissions())) {
    //             continue;
    //         }
    //         NBNode* const from = e.second->getFromNode();
    //         NBNode* const to = e.second->getToNode();
    //         if (brokenNodes.count(from) == 0 && brokenNodes.count(to) == 0) {
    //             continue;
    //         }
    //         if (e.second->isBidiRail()) {
    //             continue;
    //         }
    //         EdgeVector inRailFrom, outRailFrom, inRailTo, outRailTo;
    //         getRailEdges(from, inRailFrom, outRailFrom);
    //         getRailEdges(to, inRailTo, outRailTo);
    //         // check whether there is a straight edge pointing away from this one at the from-node
    //         // and there is no straight incoming edge at the from-node
    //         bool haveStraight = false;
    //         bool haveStraightReverse = false;
    //         if (!geometryLike || outRailFrom.size() + inRailFrom.size() == 2) {
    //             for (const NBEdge* fromStraightCand : outRailFrom) {
    //                 if (fromStraightCand != e.second && isStraight(from, fromStraightCand, e.second)) {
    //                     haveStraightReverse = true;
    //                     //std::cout << " haveStraightReverse outRailFrom=" << fromStraightCand->getID() << "\n";
    //                     break;
    //                 }
    //             }
    //             if (haveStraightReverse) {
    //                 for (const NBEdge* fromStraightCand : inRailFrom) {
    //                     if (fromStraightCand != e.second && isStraight(from, fromStraightCand, e.second)) {
    //                         haveStraight = true;
    //                         //std::cout << " haveStraight inRailFrom=" << fromStraightCand->getID() << "\n";
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         if ((!haveStraightReverse || haveStraight) && (!geometryLike || outRailTo.size() + inRailTo.size() == 2)) {
    //             // check whether there is a straight edge pointing towards this one at the to-node
    //             // and there is no straight outgoing edge at the to-node
    //             haveStraight = false;
    //             haveStraightReverse = false;
    //             for (const NBEdge* toStraightCand : inRailTo) {
    //                 if (toStraightCand != e.second && isStraight(to, toStraightCand, e.second)) {
    //                     haveStraightReverse = true;
    //                     //std::cout << " haveStraightReverse inRailTo=" << toStraightCand->getID() << "\n";
    //                     break;
    //                 }
    //             }
    //             if (haveStraightReverse) {
    //                 for (const NBEdge* toStraightCand : outRailTo) {
    //                     if (toStraightCand != e.second && isStraight(to, toStraightCand, e.second)) {
    //                         haveStraight = true;
    //                         //std::cout << " haveStraightReverse outRailTo=" << toStraightCand->getID() << "\n";
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         //std::cout << "edge=" << e.second->getID() << " haveStraight=" << haveStraight << " haveStraightReverse=" << haveStraightReverse << "\n";
    //         if (haveStraightReverse && !haveStraight) {
    //             NBEdge* e2 = addBidiEdge(ec, e.second);
    //             //std::cout << " add bidiEdge for straight connectivity at edge " << e.second->getID() << " fromBroken=" << brokenNodes.count(from) << " toBroken=" << brokenNodes.count(to) << "\n";
    //             if (e2 != nullptr) {
    //                 added++;
    //                 added += extendBidiEdges(ec, to, e.second);
    //                 added += extendBidiEdges(ec, from, e2);
    //             }
    //         }
    //     }
    //     if (added > 0) {
    //         if (geometryLike) {
    //             WRITE_MESSAGEF(TL("Added % bidi-edges to ensure connectivity of straight tracks at geometry-like nodes."), toString(added));
    //         } else {
    //             WRITE_MESSAGEF(TL("Added % bidi-edges to ensure connectivity of straight tracks at switches."), toString(added));
    //         }
    //     }
    //     return added;
    // }
    #[allow(unused_variables)]
    pub fn add_bidi_edges_for_straight_connectivity(ec: &mut NBEdgeCont, geometryLike: bool) -> i32 {
        let mut added = 0;
        let brokenNodes = Self::get_broken_rail_nodes(ec, false);
        for e in ec.get_all_edges() {
            if !Self::has_railway(e.get_permissions(-1)) {
                continue;
            }
            let from = e.get_from_node();
            let to = e.get_to_node();
            if !brokenNodes.contains(&from) && !brokenNodes.contains(&to) {
                continue;
            }
            if e.is_bidi_rail(false) {
                continue;
            }
            let mut inRailFrom = EdgeVector::new();
            let mut outRailFrom = EdgeVector::new();
            let mut inRailTo = EdgeVector::new();
            let mut outRailTo = EdgeVector::new();
            Self::get_rail_edges(from, &mut inRailFrom, &mut outRailFrom);
            Self::get_rail_edges(to, &mut inRailTo, &mut outRailTo);
            // check whether there is a straight edge pointing away from this one at the from-node
            // and there is no straight incoming edge at the from-node
            let mut haveStraight = false;
            let mut haveStraightReverse = false;
            if !geometryLike || outRailFrom.len() + inRailFrom.len() == 2 {
                for fromStraightCand in outRailFrom.iter() {
                    if *fromStraightCand != e && Self::is_straight(from, *fromStraightCand, e) {
                        haveStraightReverse = true;
                        //std::cout << " haveStraightReverse outRailFrom=" << fromStraightCand->getID() << "\n";
                        break;
                    }
                }
                if haveStraightReverse {
                    for fromStraightCand in inRailFrom.iter() {
                        if *fromStraightCand != e && Self::is_straight(from, *fromStraightCand, e) {
                            haveStraight = true;
                            //std::cout << " haveStraight inRailFrom=" << fromStraightCand->getID() << "\n";
                            break;
                        }
                    }
                }
            }
            if (!haveStraightReverse || haveStraight) && (!geometryLike || outRailTo.len() + inRailTo.len() == 2) {
                // check whether there is a straight edge pointing towards this one at the to-node
                // and there is no straight outgoing edge at the to-node
                haveStraight = false;
                haveStraightReverse = false;
                for toStraightCand in inRailTo.iter() {
                    if *toStraightCand != e && Self::is_straight(to, *toStraightCand, e) {
                        haveStraightReverse = true;
                        //std::cout << " haveStraightReverse inRailTo=" << toStraightCand->getID() << "\n";
                        break;
                    }
                }
                if haveStraightReverse {
                    for toStraightCand in outRailTo.iter() {
                        if *toStraightCand != e && Self::is_straight(to, *toStraightCand, e) {
                            haveStraight = true;
                            //std::cout << " haveStraightReverse outRailTo=" << toStraightCand->getID() << "\n";
                            break;
                        }
                    }
                }
            }
            //std::cout << "edge=" << e.second->getID() << " haveStraight=" << haveStraight << " haveStraightReverse=" << haveStraightReverse << "\n";
            if haveStraightReverse && !haveStraight {
                let e2 = Self::add_bidi_edge(ec, e, true);
                //std::cout << " add bidiEdge for straight connectivity at edge " << e.second->getID() << " fromBroken=" << brokenNodes.count(from) << " toBroken=" << brokenNodes.count(to) << "\n";
                if !e2.is_null() {
                    added += 1;
                    added += Self::extend_bidi_edges_2(ec, to, e);
                    added += Self::extend_bidi_edges_2(ec, from, e2);
                }
            }
        }
        if added > 0 {
            if geometryLike {
                write_messagef!("Added % bidi-edges to ensure connectivity of straight tracks at geometry-like nodes.", added);
            } else {
                write_messagef!("Added % bidi-edges to ensure connectivity of straight tracks at switches.", added);
            }
        }
        added
    }

    /// C++: `void NBRailwayTopologyAnalyzer::updateTurns(NBEdge *)` [static, private]
    /// recompute turning directions for both nodes of the given edge
    // void
    // NBRailwayTopologyAnalyzer::updateTurns(NBEdge* edge) {
    //     NBTurningDirectionsComputer::computeTurnDirectionsForNode(edge->getFromNode(), false);
    //     NBTurningDirectionsComputer::computeTurnDirectionsForNode(edge->getToNode(), false);
    // }
    #[allow(unused_variables)]
    pub fn update_turns(edge: Ptr<NBEdge>) {
        NBTurningDirectionsComputer::compute_turn_directions_for_node(edge.get_from_node(), false);
        NBTurningDirectionsComputer::compute_turn_directions_for_node(edge.get_to_node(), false);
    }

    /// C++: `std::set<NBPTLine *> NBRailwayTopologyAnalyzer::findBidiCandidates(NBPTLineCont &)` [static, private]
    /// identify lines that are likely to require bidirectional tracks
    // std::set<NBPTLine*>
    // NBRailwayTopologyAnalyzer::findBidiCandidates(NBPTLineCont& lc) {
    //     std::set<NBPTLine*>  result;
    //     std::set<std::pair<std::shared_ptr<NBPTStop>, std::shared_ptr<NBPTStop> > > visited;
    //     for (const auto& item : lc.getLines()) {
    //         const std::vector<std::shared_ptr<NBPTStop> >& stops = item.second->getStops();
    //         if (stops.size() > 1) {
    //             for (auto it = stops.begin(); it + 1 != stops.end(); ++it) {
    //                 std::shared_ptr<NBPTStop> fromStop = *it;
    //                 std::shared_ptr<NBPTStop> toStop = *(it + 1);
    //                 visited.insert({fromStop, toStop});
    //             }
    //         }
    //     }
    //     for (const auto& item : lc.getLines()) {
    //         const std::vector<std::shared_ptr<NBPTStop> >& stops = item.second->getStops();
    //         if (stops.size() > 1) {
    //             for (auto it = stops.begin(); it + 1 != stops.end(); ++it) {
    //                 std::shared_ptr<NBPTStop> fromStop = *it;
    //                 std::shared_ptr<NBPTStop> toStop = *(it + 1);
    //                 std::pair<std::shared_ptr<NBPTStop>, std::shared_ptr<NBPTStop> > reverseTrip({toStop, fromStop});
    //                 if (visited.count(reverseTrip)) {
    //                     result.insert(item.second);
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn find_bidi_candidates(lc: &mut NBPTLineCont) -> BTreeSet<Ptr<NBPTLine>> {
        let mut result: BTreeSet<Ptr<NBPTLine>> = BTreeSet::new();
        // std::set<std::pair<shared_ptr<NBPTStop>, shared_ptr<NBPTStop>>> is ordered by pointer, so the stops are keyed by address
        let mut visited: BTreeSet<(usize, usize)> = BTreeSet::new();
        for item in lc.get_lines().values() {
            let stops = item.m().get_stops();
            if stops.len() > 1 {
                for w in stops.windows(2) {
                    visited.insert((w[0].as_raw() as usize, w[1].as_raw() as usize));
                }
            }
        }
        for item in lc.get_lines().values() {
            let stops = item.m().get_stops();
            if stops.len() > 1 {
                for w in stops.windows(2) {
                    let reverseTrip = (w[1].as_raw() as usize, w[0].as_raw() as usize);
                    if visited.contains(&reverseTrip) {
                        result.insert(*item);
                        break;
                    }
                }
            }
        }
        result
    }

}

/// routing edge
/// C++ class `NBRailwayTopologyAnalyzer::Track` (netbuild/NBAlgorithms_Railway.h)
pub struct NBRailwayTopologyAnalyzer_Track {
    pub edge: Ptr<NBEdge>,
    pub penalty: f64,
    pub index: i32,
    pub id: String,
    pub successors: Vec<Ptr<NBRailwayTopologyAnalyzer_Track>>,
    pub viaSuccessors: Vec<(Ptr<NBRailwayTopologyAnalyzer_Track>, Ptr<NBRailwayTopologyAnalyzer_Track>)>,
    pub minPermissions: SVCPermissions,
    // boxed so the cached vectors keep their address when the map grows (the router borrows them)
    pub svcSuccessors: std::cell::RefCell<BTreeMap<SUMOVehicleClass, Box<Vec<Ptr<NBRailwayTopologyAnalyzer_Track>>>>>,
    pub svcViaSuccessors: std::cell::RefCell<BTreeMap<SUMOVehicleClass, Box<Vec<(Ptr<NBRailwayTopologyAnalyzer_Track>, Ptr<NBRailwayTopologyAnalyzer_Track>)>>>>, // C++: std::map<SUMOVehicleClass, std::vector<std::pair<const Track *, const Track *>>>
}

impl NBRailwayTopologyAnalyzer_Track {
    /// C++: `void Track::Track(NBEdge *, int, const std::string &, double)`
    // Track(NBEdge* e, int i = -1, const std::string& _id = "", double _penalty = 1) :
    //             edge(e),
    //             penalty(_penalty),
    //             index(i < 0 ? edge->getNumericalID() : i),
    //             id(_id == "" ? edge->getID() : _id),
    //             minPermissions(edge->getPermissions()) {
    //         }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>, i: i32, _id: &str, _penalty: f64) -> Self {
        NBRailwayTopologyAnalyzer_Track {
            edge: e,
            penalty: _penalty,
            index: if i < 0 { e.get_numerical_id() } else { i },
            id: if _id == "" { e.get_id().to_string() } else { _id.to_string() },
            successors: Vec::new(),
            viaSuccessors: Vec::new(),
            minPermissions: e.get_permissions(-1),
            svcSuccessors: std::cell::RefCell::new(BTreeMap::new()),
            svcViaSuccessors: std::cell::RefCell::new(BTreeMap::new()),
        }
    }

    /// C++: `void Track::addSuccessor(Track *)`
    // void
    // NBRailwayTopologyAnalyzer::Track::addSuccessor(Track* track) {
    //     successors.push_back(track);
    //     viaSuccessors.push_back(std::make_pair(track, nullptr));
    //     minPermissions &= track->edge->getPermissions();
    // }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, track: Ptr<NBRailwayTopologyAnalyzer_Track>) {
        self.successors.push(track);
        self.viaSuccessors.push((track, Ptr::null()));
        self.minPermissions &= track.edge.get_permissions(-1);
    }

    /// C++: `const std::vector<Track *> & Track::getSuccessors(SUMOVehicleClass) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_successors(&self, svc: SUMOVehicleClass) -> Vec<Ptr<NBRailwayTopologyAnalyzer_Track>> {
        if (self.minPermissions & svc) != 0 {
            return self.successors.clone();
        }
        // C++ `mutable` cache, filled on first use. Returns a copy: a RefCell cannot lend a reference out of the cache
        if !self.svcSuccessors.borrow().contains_key(&svc) {
            let mut succ: Vec<Ptr<NBRailwayTopologyAnalyzer_Track>> = Vec::new();
            for t in self.successors.iter() {
                if (t.edge.get_permissions(-1) & svc) != 0 {
                    succ.push(*t);
                }
            }
            self.svcSuccessors.borrow_mut().insert(svc, Box::new(succ));
        }
        self.svcSuccessors.borrow()[&svc].to_vec()
    }

    /// C++: `const std::vector<std::pair<const Track *, const Track *>> & Track::getViaSuccessors(SUMOVehicleClass, bool) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, svc: SUMOVehicleClass, ignoreTransientPermissions: bool) -> Vec<(Ptr<NBRailwayTopologyAnalyzer_Track>, Ptr<NBRailwayTopologyAnalyzer_Track>)> {
        if (self.minPermissions & svc) != 0 {
            return self.viaSuccessors.clone();
        }
        // C++ `mutable` cache, filled on first use. Returns a copy: a RefCell cannot lend a reference out of the cache
        if !self.svcViaSuccessors.borrow().contains_key(&svc) {
            let mut succ: Vec<(Ptr<NBRailwayTopologyAnalyzer_Track>, Ptr<NBRailwayTopologyAnalyzer_Track>)> = Vec::new();
            for t in self.successors.iter() {
                if (t.edge.get_permissions(-1) & svc) != 0 {
                    succ.push((*t, Ptr::null()));
                }
            }
            self.svcViaSuccessors.borrow_mut().insert(svc, Box::new(succ));
        }
        self.svcViaSuccessors.borrow()[&svc].to_vec()
    }

    /// C++: `const std::string & Track::getID() const` [const]
    // const std::string& getID() const {
    //             return id;
    //         }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// C++: `int Track::getNumericalID() const` [const]
    // int getNumericalID() const {
    //             return index;
    //         }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.index
    }

    /// C++: `double Track::getLength() const` [const]
    // double getLength() const {
    //             return 0;
    //         }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        0.0
    }

    /// C++: `const Track * Track::getBidiEdge() const` [const]
    // const Track* getBidiEdge() const {
    //             return this;
    //         }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<NBRailwayTopologyAnalyzer_Track> {
        Ptr::new(self)
    }

    /// C++: `bool Track::isInternal() const` [const]
    // bool isInternal() const {
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        false
    }

    /// C++: `bool Track::prohibits(const NBVehicle *const) const` [const]
    // inline bool prohibits(const NBVehicle* const /*veh*/) const {
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn prohibits(&self, arg0: Ptr<NBVehicle>) -> bool {
        false
    }

    /// C++: `bool Track::restricts(const NBVehicle *const) const` [const]
    // inline bool restricts(const NBVehicle* const /*veh*/) const {
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn restricts(&self, arg0: Ptr<NBVehicle>) -> bool {
        false
    }

    /// C++: `Track & Track::operator=(const Track &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBRailwayTopologyAnalyzer_Track) -> &mut NBRailwayTopologyAnalyzer_Track {
        // C++ declares Track::operator=(const Track&) = delete (private), so no assignment can happen
        panic!("NBRailwayTopologyAnalyzer::Track::operator= is deleted in C++")
    }

}

impl RouterEdgeInfo for NBRailwayTopologyAnalyzer_Track {
    fn edge_numerical_id(&self) -> i32 {
        self.get_numerical_id()
    }
    fn edge_id(&self) -> String {
        self.get_id().to_string()
    }
    // C++ Track has no getPermissions(); only the ch/cch/sptree routers read it, and C++ never runs them on a Track
    fn edge_permissions(&self) -> SVCPermissions {
        panic!("NBRailwayTopologyAnalyzer::Track has no getPermissions in C++")
    }
}

impl RouterEdge<NBVehicle> for NBRailwayTopologyAnalyzer_Track {
    type Node = NBNode;
    type Lane = NBEdge_Lane;
    fn id(&self) -> &str {
        self.get_id()
    }
    fn numerical_id(&self) -> i32 {
        self.get_numerical_id()
    }
    fn length(&self) -> f64 {
        self.get_length()
    }
    // C++ Track defines none of the following; DijkstraRouter<Track> never calls them (only the AF/A* routers do, and C++ never instantiates those with a Track)
    fn length_geometry_factor(&self) -> f64 {
        panic!("NBRailwayTopologyAnalyzer::Track has no getLengthGeometryFactor in C++")
    }
    fn distance_to(&self, _other: Ptr<Self>, _do_boundary_estimate: bool) -> f64 {
        panic!("NBRailwayTopologyAnalyzer::Track has no getDistanceTo in C++")
    }
    fn speed_limit(&self) -> f64 {
        panic!("NBRailwayTopologyAnalyzer::Track has no getSpeedLimit in C++")
    }
    fn minimum_travel_time(&self, _veh: Ptr<NBVehicle>) -> f64 {
        panic!("NBRailwayTopologyAnalyzer::Track has no getMinimumTravelTime in C++")
    }
    fn travel_time_aggregated(_edge: Ptr<Self>, _veh: Ptr<NBVehicle>, _time: f64) -> f64 {
        panic!("NBRailwayTopologyAnalyzer::Track has no getTravelTimeAggregated in C++")
    }
    fn successors(&self, vclass: SUMOVehicleClass) -> &[Ptr<Self>] {
        if (self.minPermissions & vclass) != 0 {
            return &self.successors;
        }
        self.get_successors(vclass);
        // SAFETY: the cache entry is a Box that is never removed, and the RefCell is not borrowed here
        unsafe { (*self.svcSuccessors.as_ptr()).get(&vclass).unwrap().as_slice() }
    }
    fn via_successors(&self, vclass: SUMOVehicleClass, ignore_transient: bool) -> &[(Ptr<Self>, Ptr<Self>)] {
        if (self.minPermissions & vclass) != 0 {
            return &self.viaSuccessors;
        }
        self.get_via_successors(vclass, ignore_transient);
        // SAFETY: as in successors above
        unsafe { (*self.svcViaSuccessors.as_ptr()).get(&vclass).unwrap().as_slice() }
    }
    fn bidi_edge(&self) -> Ptr<Self> {
        self.get_bidi_edge()
    }
    fn is_internal(&self) -> bool {
        self.is_internal()
    }
    fn prohibits(&self, veh: Ptr<NBVehicle>, _check_restrictions: bool) -> bool {
        self.prohibits(veh)
    }
    fn restricts(&self, veh: Ptr<NBVehicle>) -> bool {
        self.restricts(veh)
    }
    fn travel_time_static(edge: Ptr<Self>, veh: Ptr<NBVehicle>, time: f64) -> f64 {
        NBRailwayTopologyAnalyzer::get_travel_time_static(edge, veh, time)
    }
}

