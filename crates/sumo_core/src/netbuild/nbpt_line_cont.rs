//! Port of netbuild/NBPTLineCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::options::options_cont::{get_bool, get_float};


/// shortcut: NBPTStop is held as Rc but C++ mutates it in place; mutate through the Rc pointer until stop containers hold Ptr<NBPTStop>
#[allow(clippy::mut_from_ref)]
fn stop_mut(stop: &Rc<NBPTStop>) -> &mut NBPTStop {
    unsafe { &mut *(Rc::as_ptr(stop) as *mut NBPTStop) }
}

pub const FWD: i32 = 1;
pub const BWD: i32 = -1;


/// C++ class `NBPTLineCont` (netbuild/NBPTLineCont.h)
pub struct NBPTLineCont {
    /// The map of names to pt lines
    pub myPTLines: BTreeMap<String, Ptr<NBPTLine>>,
    /// The map of edge ids to lines that use this edge in their route
    pub myPTLineLookup: BTreeMap<String, BTreeSet<Ptr<NBPTLine>>>, // C++: std::map<std::string, std::set<NBPTLine *>>
}

// C++ static members of NBPTLineCont (port as module-level statics, see AGENTS.md):
//   static const int NBPTLineCont::FWD;
//   static const int NBPTLineCont::BWD;

impl NBPTLineCont {
    /// C++: `void NBPTLineCont::~NBPTLineCont()`
    /// destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: NBPTLine objects are shared as Ptr and their owner is not ported yet, so they are not freed here; upgrade when NBPTLine ownership is decided
        self.myPTLines.clear();
    }

    /// C++: `bool NBPTLineCont::insert(NBPTLine *)`
    /// insert new line
    // bool
    // NBPTLineCont::insert(NBPTLine* ptLine) {
    //     if (myPTLines.count(ptLine->getLineID()) == 0) {
    //         myPTLines[ptLine->getLineID()] = ptLine;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn insert(&mut self, ptLine: Ptr<NBPTLine>) -> bool {
        if !self.myPTLines.contains_key(ptLine.get_line_id()) {
            self.myPTLines.insert(ptLine.get_line_id().to_string(), ptLine);
            return true;
        }
        false
    }

    /// C++: `NBPTLine * NBPTLineCont::retrieve(const std::string &)`
    // NBPTLine*
    // NBPTLineCont::retrieve(const std::string& lineID) {
    //     if (myPTLines.count(lineID) == 0) {
    //         return nullptr;
    //     } else {
    //         return myPTLines[lineID];
    //     }
    // }
    #[allow(unused_variables)]
    pub fn retrieve(&mut self, lineID: &str) -> Ptr<NBPTLine> {
        match self.myPTLines.get(lineID) {
            None => Ptr::null(),
            Some(line) => *line,
        }
    }

    /// C++: `const std::map<std::string, NBPTLine *> & NBPTLineCont::getLines() const` [const]
    // const std::map<std::string, NBPTLine*>& getLines() const {
    //         return myPTLines;
    //     }
    #[allow(unused_variables)]
    pub fn get_lines(&self) -> &BTreeMap<String, Ptr<NBPTLine>> {
        &self.myPTLines
    }

    /// C++: `void NBPTLineCont::process(NBEdgeCont &, NBPTStopCont &, bool)`
    // void
    // NBPTLineCont::process(NBEdgeCont& ec, NBPTStopCont& sc, bool routeOnly) {
    //     for (auto& item : myPTLines) {
    //         NBPTLine* line = item.second;
    //         if (item.second->getWays().size() > 0) {
    //             // loaded from OSM rather than ptline input. We can use extra
    //             // information to reconstruct route and stops
    //             constructRoute(line, ec);
    //             if (!routeOnly) {
    //                 // map stops to ways, using the constructed route for loose stops
    //                 reviseStops(line, ec, sc);
    //             }
    //         }
    //         // fix circular line if necessary
    //         if (line->getStops().size() > 1
    //                 && line->getStops().front() == line->getStops().back()
    //                 && line->getRoute().size() > 1
    //                 && line->getRoute().front() != line->getRoute().back()) {
    //             // we need to duplicate either the first or the last edge depending on the stop locations
    //             const std::string firstStopEdge = line->getStops().front()->getEdgeId();
    //             const std::string lastStopEdge = line->getStops().back()->getEdgeId();
    //             std::vector<NBEdge*> edges = line->getRoute();
    //             if (firstStopEdge == edges.back()->getID()) {
    //                 edges.insert(edges.begin(), edges.back());
    //             } else if (lastStopEdge == edges.front()->getID()) {
    //                 edges.push_back(edges.front());
    //             }
    //             line->setEdges(edges);
    //         }
    //         line->deleteInvalidStops(ec, sc);
    //         //line->deleteDuplicateStops();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process(&mut self, ec: &mut NBEdgeCont, sc: &mut NBPTStopCont, routeOnly: bool) {
        let lines: Vec<Ptr<NBPTLine>> = self.myPTLines.values().copied().collect();
        for line in lines {
            if line.get_ways().len() > 0 {
                // loaded from OSM rather than ptline input. We can use extra
                // information to reconstruct route and stops
                self.construct_route(line, ec);
                if !routeOnly {
                    // map stops to ways, using the constructed route for loose stops
                    self.revise_stops(line, ec, sc);
                }
            }
            // fix circular line if necessary
            let stops: Vec<Rc<NBPTStop>> = line.m().get_stops().clone();
            if stops.len() > 1
                    && Rc::ptr_eq(&stops[0], &stops[stops.len() - 1])
                    && line.get_route().len() > 1
                    && line.get_route()[0] != line.get_route()[line.get_route().len() - 1] {
                // we need to duplicate either the first or the last edge depending on the stop locations
                let firstStopEdge: String = stops[0].get_edge_id().to_string();
                let lastStopEdge: String = stops[stops.len() - 1].get_edge_id().to_string();
                let mut edges: Vec<Ptr<NBEdge>> = line.get_route().clone();
                let lastEdge = edges[edges.len() - 1];
                let firstEdge = edges[0];
                if firstStopEdge == lastEdge.get_id() {
                    edges.insert(0, lastEdge);
                } else if lastStopEdge == firstEdge.get_id() {
                    edges.push(firstEdge);
                }
                line.m().set_edges(&edges);
            }
            line.m().delete_invalid_stops(ec, sc);
            //line->deleteDuplicateStops();
        }
    }

    /// C++: `void NBPTLineCont::replaceEdge(const std::string &, const EdgeVector &)`
    /// replace the edge with the given edge list in all lines
    // void
    // NBPTLineCont::replaceEdge(const std::string& edgeID, const EdgeVector& replacement) {
    //     //std::cout << " replaceEdge " << edgeID << " replacement=" << toString(replacement) << "\n";
    //     if (myPTLines.size() > 0 && myPTLineLookup.size() == 0) {
    //         // init lookup once
    //         for (auto& item : myPTLines) {
    //             for (const NBEdge* e : item.second->getRoute()) {
    //                 myPTLineLookup[e->getID()].insert(item.second);
    //             }
    //         }
    //     }
    //     for (NBPTLine* line : myPTLineLookup[edgeID]) {
    //         line->replaceEdge(edgeID, replacement);
    //         for (const NBEdge* e : replacement) {
    //             myPTLineLookup[e->getID()].insert(line);
    //         }
    //     }
    //     myPTLineLookup.erase(edgeID);
    // }
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, edgeID: &str, replacement: &EdgeVector) {
        //std::cout << " replaceEdge " << edgeID << " replacement=" << toString(replacement) << "\n";
        if self.myPTLines.len() > 0 && self.myPTLineLookup.len() == 0 {
            // init lookup once
            for item in self.myPTLines.values() {
                for e in item.get_route().iter() {
                    self.myPTLineLookup.entry(e.get_id().to_string()).or_default().insert(*item);
                }
            }
        }
        // determinism: std::set<NBPTLine*> orders by pointer, BTreeSet<Ptr> orders by address in the same way
        let lines: Vec<Ptr<NBPTLine>> = self.myPTLineLookup.entry(edgeID.to_string()).or_default().iter().copied().collect();
        for line in lines {
            line.m().replace_edge(edgeID, replacement);
            for e in replacement.iter() {
                self.myPTLineLookup.entry(e.get_id().to_string()).or_default().insert(line);
            }
        }
        self.myPTLineLookup.remove(edgeID);
    }

    /// C++: `void NBPTLineCont::fixBidiStops(const NBEdgeCont &)`
    /// select the correct stop on superposed rail edges
    // void
    // NBPTLineCont::fixBidiStops(const NBEdgeCont& ec) {
    //     std::map<std::string, SUMOVehicleClass> types;
    //     types["bus"] = SVC_BUS;
    //     types["minibus"] = SVC_BUS;
    //     types["trolleybus"] = SVC_BUS;
    //     types["tram"] = SVC_TRAM;
    //     types["train"] = SVC_RAIL;
    //     types["subway"] = SVC_RAIL_URBAN;
    //     types["light_rail"] = SVC_RAIL_URBAN;
    //     types["monorail"] = SVC_RAIL_URBAN;
    //     types["aerialway"] = SVC_RAIL_URBAN;
    //     types["ferry"] = SVC_SHIP;
    //
    //     SUMOAbstractRouter<NBRouterEdge, NBVehicle>* const router = new DijkstraRouter<NBRouterEdge, NBVehicle>(
    //         ec.getAllRouterEdges(), true, &NBRouterEdge::getTravelTimeStatic, nullptr, true);
    //
    //     for (auto& item : myPTLines) {
    //         NBPTLine* line = item.second;
    //         std::vector<std::shared_ptr<NBPTStop> > stops = line->getStops();
    //         if (stops.size() < 2) {
    //             continue;
    //         }
    //         if (types.count(line->getType()) == 0) {
    //             WRITE_WARNINGF(TL("Could not determine vehicle class for public transport line of type '%'."), line->getType());
    //             continue;
    //         }
    //         NBVehicle veh(line->getRef(), types[line->getType()]);
    //         std::vector<std::shared_ptr<NBPTStop> > newStops;
    //         std::shared_ptr<NBPTStop> from = nullptr;
    //         std::vector<NBPTLine::PTStopInfo> stopInfos = line->getStopEdges(ec);
    //         assert(stopInfos.size() == stops.size());
    //         for (auto it = stops.begin(); it != stops.end(); ++it) {
    //             std::shared_ptr<NBPTStop> to = *it;
    //             std::shared_ptr<NBPTStop> used = *it;
    //             bool isRevised = stopInfos[it - stops.begin()].revised;
    //             if (to->getBidiStop() != nullptr && !isRevised) {
    //                 double best = std::numeric_limits<double>::max();
    //                 std::shared_ptr<NBPTStop> to2 = to->getBidiStop();
    //                 if (from == nullptr) {
    //                     if ((it + 1) != stops.end()) {
    //                         from = to;
    //                         std::shared_ptr<NBPTStop> from2 = to2;
    //                         to = *(it + 1);
    //                         const double c1 = getCost(ec, *router, from, to, &veh);
    //                         const double c2 = getCost(ec, *router, from2, to, &veh);
    //                         //std::cout << " from=" << from->getID() << " to=" << to->getID() << " c1=" << MIN2(10000.0, c1) << "\n";
    //                         //std::cout << " from2=" << from2->getID() << " to=" << to->getID() << " c2=" << MIN2(10000.0, c2) << "\n";
    //                         best = c1;
    //                         if (to->getBidiStop() != nullptr) {
    //                             to2 = to->getBidiStop();
    //                             const double c3 = getCost(ec, *router, from, to2, &veh);
    //                             const double c4 = getCost(ec, *router, from2, to2, &veh);
    //                             //std::cout << " from=" << from->getID() << " to2=" << to2->getID() << " c3=" << MIN2(10000.0, c3) << "\n";
    //                             //std::cout << " from2=" << from2->getID() << " to2=" << to2->getID() << " c4=" << MIN2(10000.0, c4) << "\n";
    //                             if (c2 < best) {
    //                                 used = from2;
    //                                 best = c2;
    //                             }
    //                             if (c3 < best) {
    //                                 used = from;
    //                                 best = c3;
    //                             }
    //                             if (c4 < best) {
    //                                 used = from2;
    //                                 best = c4;
    //                             }
    //                         } else {
    //                             if (c2 < c1) {
    //                                 used = from2;
    //                                 best = c2;
    //                             } else {
    //                                 best = c1;
    //                             }
    //                         }
    //                     }
    //                 } else {
    //                     const double c1 = getCost(ec, *router, from, to, &veh);
    //                     const double c2 = getCost(ec, *router, from, to2, &veh);
    //                     //std::cout << " from=" << from->getID() << " to=" << to->getID() << " c1=" << MIN2(10000.0, c1) << "\n";
    //                     //std::cout << " from=" << from->getID() << " t2o=" << to2->getID() << " c2=" << MIN2(10000.0, c2) << "\n";
    //                     if (c2 < c1) {
    //                         used = to2;
    //                         best = c2;
    //                     } else {
    //                         best = c1;
    //                     }
    //
    //                 }
    //                 if (best < std::numeric_limits<double>::max()) {
    //                     from = used;
    //                 } else {
    //                     WRITE_WARNINGF(TL("Could not determine direction for line '%' at stop '%'."), line->getLineID(), used->getID());
    //                 }
    //             }
    //             from = used;
    //             newStops.push_back(used);
    //         }
    //         assert(stops.size() == newStops.size());
    //         line->replaceStops(newStops);
    //     }
    //     delete router;
    // }
    #[allow(unused_variables)]
    pub fn fix_bidi_stops(&mut self, ec: &NBEdgeCont) {
        // shortcut: NBPTStop::myBidiStop is Unmapped (std::weak_ptr) and SUMOAbstractRouter::Operation is Unmapped, so bidi stop direction fixing cannot run yet; upgrade when both are mapped
        panic!("NBPTLineCont::fixBidiStops: shortcut, needs NBPTStop bidi links and router operations");
    }

    /// C++: `void NBPTLineCont::removeInvalidEdges(const NBEdgeCont &)`
    /// filter out edges that were removed due to --geometry.remove
    // void
    // NBPTLineCont::removeInvalidEdges(const NBEdgeCont& ec) {
    //     for (auto& item : myPTLines) {
    //         item.second->removeInvalidEdges(ec);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_invalid_edges(&mut self, ec: &NBEdgeCont) {
        for item in self.myPTLines.values() {
            item.m().remove_invalid_edges(ec);
        }
    }

    /// C++: `void NBPTLineCont::fixPermissions()`
    /// ensure that all turn lanes have sufficient permissions
    // void
    // NBPTLineCont::fixPermissions() {
    //     for (auto& item : myPTLines) {
    //         NBPTLine* line = item.second;
    //         const std::vector<NBEdge*>& route = line->getRoute();
    //         const SUMOVehicleClass svc = line->getVClass();
    //         for (int i = 1; i < (int)route.size(); i++) {
    //             NBEdge* e1 = route[i - 1];
    //             NBEdge* e2 = route[i];
    //             std::vector<NBEdge::Connection> cons = e1->getConnectionsFromLane(-1, e2, -1);
    //             if (cons.size() == 0) {
    //                 //WRITE_WARNINGF(TL("Disconnected ptline '%' between edge '%' and edge '%'"), line->getLineID(), e1->getID(), e2->getID());
    //             } else {
    //                 bool ok = false;
    //                 for (const auto& c : cons) {
    //                     if ((e1->getPermissions(c.fromLane) & svc) == svc) {
    //                         ok = true;
    //                         break;
    //                     }
    //                 }
    //                 if (!ok) {
    //                     int lane = cons[0].fromLane;
    //                     e1->setPermissions(e1->getPermissions(lane) | svc, lane);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fix_permissions(&mut self) {
        for item in self.myPTLines.values() {
            let line = item.m();
            let route: Vec<Ptr<NBEdge>> = line.get_route().clone();
            let svc = line.get_v_class();
            for i in 1..route.len() {
                let e1 = route[i - 1];
                let e2 = route[i];
                let cons = e1.get_connections_from_lane(-1, e2, -1);
                if cons.len() == 0 {
                    //WRITE_WARNINGF(TL("Disconnected ptline '%' between edge '%' and edge '%'"), line->getLineID(), e1->getID(), e2->getID());
                } else {
                    let mut ok = false;
                    for c in &cons {
                        if (e1.get_permissions(c.fromLane) & svc) == svc {
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        let lane = cons[0].fromLane;
                        let perms = e1.get_permissions(lane) | svc;
                        e1.m().set_permissions(perms, lane);
                    }
                }
            }
        }
    }

    /// C++: `std::set<std::string> NBPTLineCont::getServedPTStops()`
    // std::set<std::string>
    // NBPTLineCont::getServedPTStops() {
    //     std::set<std::string> result;
    //     for (auto& item : myPTLines) {
    //         NBPTLine* line = item.second;
    //         for (std::shared_ptr<NBPTStop> stop : line->getStops()) {
    //             result.insert(stop->getID());
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_served_pt_stops(&mut self) -> BTreeSet<String> {
        let mut result: BTreeSet<String> = BTreeSet::new();
        for item in self.myPTLines.values() {
            let line = *item;
            for stop in line.m().get_stops().iter() {
                result.insert(stop.get_id());
            }
        }
        result
    }

    /// C++: `void NBPTLineCont::reviseStops(NBPTLine *, const NBEdgeCont &, NBPTStopCont &)` [private]
    /// find directional edge for all stops of the line
    // void
    // NBPTLineCont::reviseStops(NBPTLine* line, const NBEdgeCont& ec, NBPTStopCont& sc) {
    //     const std::vector<std::string>& waysIds = line->getWays();
    //     if (waysIds.size() == 1 && line->getStops().size() > 1) {
    //         reviseSingleWayStops(line, ec, sc);
    //         return;
    //     }
    //     if (waysIds.size() <= 1) {
    //         WRITE_WARNINGF(TL("Cannot revise pt stop localization for pt line '%', which consist of one way only. Ignoring!"), line->getLineID());
    //         return;
    //     }
    //     if (line->getRoute().size() == 0) {
    //         WRITE_WARNINGF(TL("Cannot revise pt stop localization for pt line '%', which has no route edges. Ignoring!"), line->getLineID());
    //         return;
    //     }
    //     std::vector<std::shared_ptr<NBPTStop> > stops = line->getStops();
    //     std::vector<bool> stopsRevised;
    //     for (std::shared_ptr<NBPTStop> stop : stops) {
    //         stopsRevised.push_back(false);
    //         //get the corresponding and one of the two adjacent ways
    //         stop = findWay(line, stop, ec, sc);
    //         if (stop == nullptr) {
    //             // warning already given
    //             continue;
    //         }
    //         auto waysIdsIt = std::find(waysIds.begin(), waysIds.end(), stop->getOrigEdgeId());
    //         if (waysIdsIt == waysIds.end()) {
    //             // warning already given
    //             continue;
    //         }
    //         // find directional edge (OSM ways are bidirectional)
    //         const std::vector<long long int>* const way = line->getWayNodes(stop->getOrigEdgeId());
    //         if (way == nullptr) {
    //             WRITE_WARNINGF(TL("Cannot assign stop '%' on edge '%' to pt line '%' (wayNodes not found). Ignoring!"),
    //                            stop->getID(), stop->getOrigEdgeId(), line->getLineID());
    //             continue;
    //         }
    //
    //         int dir;
    //         const std::vector<long long int>* wayPrev = nullptr;
    //         if (waysIdsIt != waysIds.begin()) {
    //             wayPrev = line->getWayNodes(*(waysIdsIt - 1));
    //         }
    //         const std::vector<long long int>* wayNext = nullptr;
    //         if (waysIdsIt != (waysIds.end() - 1)) {
    //             wayNext = line->getWayNodes(*(waysIdsIt + 1));
    //         }
    //         if (wayPrev == nullptr && wayNext == nullptr) {
    //             WRITE_WARNINGF(TL("Cannot revise pt stop localization for incomplete pt line '%'. Ignoring!"), line->getLineID());
    //             continue;
    //         }
    //         const long long int wayEnds = way->back();
    //         const long long int wayBegins = way->front();
    //         const long long int wayPrevEnds = wayPrev != nullptr ? wayPrev->back() : 0;
    //         const long long int wayPrevBegins = wayPrev != nullptr ? wayPrev->front() : 0;
    //         const long long int wayNextEnds = wayNext != nullptr ? wayNext->back() : 0;
    //         const long long int wayNextBegins = wayNext != nullptr ? wayNext->front() : 0;
    //         if (wayBegins == wayPrevEnds || wayBegins == wayPrevBegins || wayEnds == wayNextBegins || wayEnds == wayNextEnds) {
    //             dir = FWD;
    //         } else if (wayEnds == wayPrevBegins || wayEnds == wayPrevEnds || wayBegins == wayNextEnds || wayBegins == wayNextBegins) {
    //             dir = BWD;
    //         } else {
    //             WRITE_WARNINGF(TL("Cannot revise pt stop localization for incomplete pt line '%'. Ignoring!"), line->getLineID());
    //             continue;
    //         }
    //
    //         std::string edgeId = stop->getEdgeId();
    //         NBEdge* current = ec.getByID(edgeId);
    //         int assignedDir = edgeId.at(0) == '-' ? BWD : FWD;
    //
    //         if (dir != assignedDir) {
    //             NBEdge* reverse = NBPTStopCont::getReverseEdge(current);
    //             if (reverse == nullptr) {
    //                 const OptionsCont& oc = OptionsCont::getOptions();
    //                 if (!oc.getBool("railway.topology.repair") && oc.getBool("ptstop-output.no-bidi")) {
    //                     WRITE_WARNINGF(TL("Could not re-assign PT stop '%'. May need option --railway.topology.repair"), stop->getID());
    //                 }
    //                 continue;
    //             }
    //             if (stop->getLines().size() > 0) {
    //                 std::shared_ptr<NBPTStop> reverseStop = sc.getReverseStop(stop, ec);
    //                 sc.insert(reverseStop);
    //                 line->replaceStop(stop, reverseStop);
    //                 stop = reverseStop;
    //             } else {
    //                 WRITE_WARNINGF(TL("PT stop '%' has been moved to edge '%'."), stop->getID(), reverse->getID());
    //             }
    //             stop->setEdgeId(reverse->getID(), ec);
    //         }
    //         stop->addLine(line->getRef());
    //         stopsRevised.back() = true;
    //     }
    //     line->setRevised(stopsRevised);
    // }
    #[allow(unused_variables)]
    pub fn revise_stops(&mut self, line: Ptr<NBPTLine>, ec: &NBEdgeCont, sc: &mut NBPTStopCont) {
        let waysIds: Vec<String> = line.get_ways().clone();
        if waysIds.len() == 1 && line.m().get_stops().len() > 1 {
            self.revise_single_way_stops(line, ec, sc);
            return;
        }
        if waysIds.len() <= 1 {
            write_warning(&format!("Cannot revise pt stop localization for pt line '{}', which consist of one way only. Ignoring!", line.get_line_id()));
            return;
        }
        if line.get_route().len() == 0 {
            write_warning(&format!("Cannot revise pt stop localization for pt line '{}', which has no route edges. Ignoring!", line.get_line_id()));
            return;
        }
        let stops: Vec<Rc<NBPTStop>> = line.m().get_stops().clone();
        let mut stopsRevised: Vec<bool> = Vec::new();
        for stop0 in stops {
            stopsRevised.push(false);
            //get the corresponding and one of the two adjacent ways
            let mut stop = match self.find_way(line, stop0, ec, sc) {
                Some(st) => st,
                None => continue, // warning already given
            };
            let waysIdsIt = match waysIds.iter().position(|w| *w == stop.get_orig_edge_id()) {
                Some(idx) => idx,
                None => continue, // warning already given
            };
            // find directional edge (OSM ways are bidirectional)
            let way: &Vec<i64> = match line.m().get_way_nodes(stop.get_orig_edge_id()).get() {
                Some(w) => w,
                None => {
                    write_warning(&format!("Cannot assign stop '{}' on edge '{}' to pt line '{}' (wayNodes not found). Ignoring!",
                        stop.get_id(), stop.get_orig_edge_id(), line.get_line_id()));
                    continue;
                }
            };

            let mut wayPrev: Option<&Vec<i64>> = None;
            if waysIdsIt != 0 {
                wayPrev = line.m().get_way_nodes(waysIds[waysIdsIt - 1].clone()).get();
            }
            let mut wayNext: Option<&Vec<i64>> = None;
            if waysIdsIt != waysIds.len() - 1 {
                wayNext = line.m().get_way_nodes(waysIds[waysIdsIt + 1].clone()).get();
            }
            if wayPrev.is_none() && wayNext.is_none() {
                write_warning(&format!("Cannot revise pt stop localization for incomplete pt line '{}'. Ignoring!", line.get_line_id()));
                continue;
            }
            let wayEnds = *way.last().unwrap();
            let wayBegins = way[0];
            let wayPrevEnds = wayPrev.map_or(0, |w| *w.last().unwrap());
            let wayPrevBegins = wayPrev.map_or(0, |w| w[0]);
            let wayNextEnds = wayNext.map_or(0, |w| *w.last().unwrap());
            let wayNextBegins = wayNext.map_or(0, |w| w[0]);
            let dir: i32;
            if wayBegins == wayPrevEnds || wayBegins == wayPrevBegins || wayEnds == wayNextBegins || wayEnds == wayNextEnds {
                dir = FWD;
            } else if wayEnds == wayPrevBegins || wayEnds == wayPrevEnds || wayBegins == wayNextEnds || wayBegins == wayNextBegins {
                dir = BWD;
            } else {
                write_warning(&format!("Cannot revise pt stop localization for incomplete pt line '{}'. Ignoring!", line.get_line_id()));
                continue;
            }

            let edgeId: String = stop.get_edge_id().to_string();
            let current = ec.get_by_id(&edgeId);
            let assignedDir = if edgeId.starts_with('-') { BWD } else { FWD };

            if dir != assignedDir {
                let reverse = NBPTStopCont::get_reverse_edge(current);
                if reverse.is_null() {
                    if !get_bool("railway.topology.repair") && get_bool("ptstop-output.no-bidi") {
                        write_warning(&format!("Could not re-assign PT stop '{}'. May need option --railway.topology.repair", stop.get_id()));
                    }
                    continue;
                }
                if stop.get_lines().len() > 0 {
                    let reverseStop = sc.get_reverse_stop(stop.clone(), ec).unwrap();
                    sc.insert(reverseStop.clone(), false);
                    line.m().replace_stop(stop.clone(), reverseStop.clone());
                    stop = reverseStop;
                } else {
                    write_warning(&format!("PT stop '{}' has been moved to edge '{}'.", stop.get_id(), reverse.get_id()));
                }
                stop_mut(&stop).set_edge_id(reverse.get_id().to_string(), ec);
            }
            stop_mut(&stop).add_line(line.get_ref());
            let lastIdx = stopsRevised.len() - 1;
            stopsRevised[lastIdx] = true;
        }
        line.m().set_revised(stopsRevised);
    }

    /// C++: `void NBPTLineCont::reviseSingleWayStops(NBPTLine *, const NBEdgeCont &, NBPTStopCont &)` [private]
    // void NBPTLineCont::reviseSingleWayStops(NBPTLine* line, const NBEdgeCont& ec, NBPTStopCont& sc) {
    //     const std::vector<std::string>& waysIds = line->getWays();
    //     for (std::shared_ptr<NBPTStop> stop : line->getStops()) {
    //         //get the corresponding and one of the two adjacent ways
    //         stop = findWay(line, stop, ec, sc);
    //         if (stop == nullptr) {
    //             // warning already given
    //             continue;
    //         }
    //         auto waysIdsIt = std::find(waysIds.begin(), waysIds.end(), stop->getOrigEdgeId());
    //         if (waysIdsIt == waysIds.end()) {
    //             // warning already given
    //             continue;
    //         }
    //         stop->addLine(line->getRef());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn revise_single_way_stops(&mut self, line: Ptr<NBPTLine>, ec: &NBEdgeCont, sc: &mut NBPTStopCont) {
        let waysIds: Vec<String> = line.get_ways().clone();
        let stops: Vec<Rc<NBPTStop>> = line.m().get_stops().clone();
        for stop0 in stops {
            //get the corresponding and one of the two adjacent ways
            let stop = match self.find_way(line, stop0, ec, sc) {
                Some(st) => st,
                None => continue, // warning already given
            };
            if !waysIds.iter().any(|w| *w == stop.get_orig_edge_id()) {
                continue; // warning already given
            }
            stop_mut(&stop).add_line(line.get_ref());
        }
    }

    /// C++: `std::shared_ptr<NBPTStop> NBPTLineCont::findWay(NBPTLine *, std::shared_ptr<NBPTStop>, const NBEdgeCont &, NBPTStopCont &) const` [const, private]
    // std::shared_ptr<NBPTStop>
    // NBPTLineCont::findWay(NBPTLine* line, std::shared_ptr<NBPTStop> stop, const NBEdgeCont& ec, NBPTStopCont& sc) const {
    //     const std::vector<std::string>& waysIds = line->getWays();
    // #ifdef DEBUG_FIND_WAY
    //     if (stop->getID() == DEBUGSTOPID) {
    //         std::cout << " stop=" << stop->getID() << " line=" << line->getLineID() << " edgeID=" << stop->getEdgeId() << " origID=" << stop->getOrigEdgeId() << "\n";
    //     }
    // #endif
    //     if (stop->isLoose()) {
    //         // find closest edge in route
    //         double minDist = std::numeric_limits<double>::max();
    //         NBEdge* best = nullptr;
    //         for (NBEdge* edge : line->getRoute()) {
    //             const double dist = edge->getLaneShape(0).distance2D(stop->getPosition());
    //             if (dist < minDist) {
    //                 best = edge;
    //                 minDist = dist;
    //             }
    //         }
    // #ifdef DEBUG_FIND_WAY
    //         if (stop->getID() == DEBUGSTOPID) {
    //             std::cout << "   best=" << Named::getIDSecure(best) << " minDist=" << minDist << " wayID=" << getWayID(best->getID())
    //                       << " found=" << (std::find(waysIds.begin(), waysIds.end(), getWayID(best->getID())) != waysIds.end())
    //                       << " wayIDs=" << toString(waysIds) << "\n";
    //         }
    // #endif
    //         if (minDist < OptionsCont::getOptions().getFloat("ptline.match-dist")) {
    //             const std::string wayID = getWayID(best->getID());
    //             if (stop->getEdgeId() == "") {
    //                 stop->setEdgeId(best->getID(), ec);
    //                 stop->setOrigEdgeId(wayID);
    //             } else if (stop->getEdgeId() != best->getID()) {
    //                 // stop is used by multiple lines and mapped to different edges.
    //                 // check if an alternative stop already exists
    //                 std::shared_ptr<NBPTStop> newStop = sc.findStop(wayID, stop->getPosition());
    //                 if (newStop == nullptr) {
    //                     newStop = std::make_shared<NBPTStop>(stop->getElement(), stop->getID() + "@" + line->getLineID(), stop->getPosition(), best->getID(), wayID, stop->getLength(), stop->getName(), stop->getPermissions());
    //                     newStop->setEdgeId(best->getID(), ec);  // trigger lane assignment
    //                     sc.insert(newStop);
    //                 }
    //                 line->replaceStop(stop, newStop);
    //                 stop = newStop;
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Could not assign stop '%' to pt line '%' (closest edge '%', distance %). Ignoring!"),
    //                            stop->getID(), line->getLineID(), Named::getIDSecure(best), minDist);
    //             return nullptr;
    //         }
    //     } else {
    //         // if the stop is part of an edge, find that edge among the line edges
    //         auto waysIdsIt = waysIds.begin();
    //         for (; waysIdsIt != waysIds.end(); waysIdsIt++) {
    //             if ((*waysIdsIt) == stop->getOrigEdgeId()) {
    //                 break;
    //             }
    //         }
    //
    //         if (waysIdsIt == waysIds.end()) {
    //             // stop edge not found, try additional edges
    //             for (auto& edgeCand : stop->getAdditionalEdgeCandidates()) {
    //                 bool found = false;
    //                 waysIdsIt =  waysIds.begin();
    //                 for (; waysIdsIt != waysIds.end(); waysIdsIt++) {
    //                     if ((*waysIdsIt) == edgeCand.first) {
    //                         if (stop->setEdgeId(edgeCand.second, ec)) {
    //                             stop->setOrigEdgeId(edgeCand.first);
    //                             found = true;
    //                             break;
    //                         }
    //                     }
    //                 }
    //                 if (found) {
    //                     break;
    //                 }
    //             }
    //             if (waysIdsIt == waysIds.end()) {
    //                 WRITE_WARNINGF(TL("Cannot assign stop % on edge '%' to pt line '%'. Ignoring!"), stop->getID(), stop->getOrigEdgeId(), line->getLineID());
    //             }
    //         }
    //     }
    //     return stop;
    // }
    #[allow(unused_variables)]
    pub fn find_way(&self, line: Ptr<NBPTLine>, stop: Rc<NBPTStop>, ec: &NBEdgeCont, sc: &mut NBPTStopCont) -> Option<Rc<NBPTStop>> {
        let waysIds: Vec<String> = line.get_ways().clone();
        if stop.is_loose() {
            // find closest edge in route
            let mut minDist = f64::MAX;
            let mut best: Ptr<NBEdge> = Ptr::null();
            for edge in line.get_route().iter() {
                let dist = edge.get_lane_shape(0).distance2d(stop.get_position(), false);
                if dist < minDist {
                    best = *edge;
                    minDist = dist;
                }
            }
            if minDist < get_float("ptline.match-dist") {
                let wayID = Self::get_way_id(best.get_id());
                if stop.get_edge_id().is_empty() {
                    stop_mut(&stop).set_edge_id(best.get_id().to_string(), ec);
                    stop_mut(&stop).set_orig_edge_id(&wayID);
                } else if stop.get_edge_id() != best.get_id() {
                    // stop is used by multiple lines and mapped to different edges.
                    // check if an alternative stop already exists
                    // same lookup as NBPTStopCont::findStop(wayID, pos) with threshold 1
                    let mut existing: Option<Rc<NBPTStop>> = None;
                    for item in sc.myPTStops.values() {
                        if item.get_orig_edge_id() == wayID && item.get_position().distance_to2d(stop.get_position()) < 1.0 {
                            existing = Some(item.clone());
                            break;
                        }
                    }
                    let newStop: Rc<NBPTStop> = match existing {
                        Some(ns) => ns,
                        None => {
                            let mut ns = NBPTStop::new(stop.get_element(), format!("{}@{}", stop.get_id(), line.get_line_id()), stop.get_position().clone(),
                                best.get_id().to_string(), wayID.clone(), stop.get_length(), stop.get_name(), stop.get_permissions(),
                                0.0, RGBColor::default(), -1.0);
                            ns.set_edge_id(best.get_id().to_string(), ec); // trigger lane assignment
                            let ns = Rc::new(ns);
                            sc.insert(ns.clone(), false);
                            ns
                        }
                    };
                    line.m().replace_stop(stop.clone(), newStop.clone());
                    return Some(newStop);
                }
            } else {
                write_warning(&format!("Could not assign stop '{}' to pt line '{}' (closest edge '{}', distance {}). Ignoring!",
                    stop.get_id(), line.get_line_id(), if best.is_null() { "" } else { best.get_id() }, minDist));
                return None;
            }
            Some(stop)
        } else {
            // if the stop is part of an edge, find that edge among the line edges
            let waysIdsIt = waysIds.iter().position(|w| *w == stop.get_orig_edge_id());
            if waysIdsIt.is_none() {
                // stop edge not found, try additional edges
                let mut found = false;
                for (edgeCandFirst, edgeCandSecond) in stop.get_additional_edge_candidates().iter() {
                    for w in waysIds.iter() {
                        if w == edgeCandFirst {
                            if stop_mut(&stop).set_edge_id(edgeCandSecond.clone(), ec) {
                                stop_mut(&stop).set_orig_edge_id(edgeCandFirst);
                                found = true;
                                break;
                            }
                        }
                    }
                    if found {
                        break;
                    }
                }
                if !found {
                    write_warning(&format!("Cannot assign stop {} on edge '{}' to pt line '{}'. Ignoring!",
                        stop.get_id(), stop.get_orig_edge_id(), line.get_line_id()));
                }
            }
            Some(stop)
        }
    }

    /// C++: `void NBPTLineCont::constructRoute(NBPTLine *, const NBEdgeCont &)` [private]
    // void NBPTLineCont::constructRoute(NBPTLine* pTLine, const NBEdgeCont& cont) {
    //     std::vector<NBEdge*> edges;
    //
    //     NBNode* first = nullptr;
    //     NBNode* last = nullptr;
    //     std::vector<NBEdge*> prevWayEdges;
    //     std::vector<NBEdge*> prevWayMinusEdges;
    //     std::vector<NBEdge*> currentWayEdges;
    //     std::vector<NBEdge*> currentWayMinusEdges;
    //     for (auto it3 = pTLine->getWays().begin(); it3 != pTLine->getWays().end(); it3++) {
    //
    //         int foundForward = 0;
    //         if (cont.retrieve(*it3, false) != nullptr) {
    //             currentWayEdges.push_back(cont.retrieve(*it3, false));
    //             foundForward++;
    //         } else {
    //             int i = 0;
    //             while (cont.retrieve(*it3 + "#" + std::to_string(i), true) != nullptr) {
    //                 if (cont.retrieve(*it3 + "#" + std::to_string(i), false)) {
    //                     currentWayEdges.push_back(cont.retrieve(*it3 + "#" + std::to_string(i), false));
    //                     foundForward++;
    //                 }
    //                 i++;
    //             }
    //         }
    //
    //         int foundReverse = 0;
    //         if (cont.retrieve("-" + *it3, false) != nullptr) {
    //             currentWayMinusEdges.push_back(cont.retrieve("-" + *it3, false));
    //             foundReverse++;
    //         } else {
    //             int i = 0;
    //             while (cont.retrieve("-" + *it3 + "#" + std::to_string(i), true) != nullptr) {
    //                 if (cont.retrieve("-" + *it3 + "#" + std::to_string(i), false)) {
    //                     currentWayMinusEdges.insert(currentWayMinusEdges.end() - foundReverse,
    //                                                 cont.retrieve("-" + *it3 + "#" + std::to_string(i), false));
    //                     foundReverse++;
    //                 }
    //                 i++;
    //             }
    //         }
    //         bool fakeMinus = false;
    //         if (foundReverse == 0 && foundForward > 0 && isRailway(pTLine->getVClass())) {
    //             // rail tracks may be used in both directions and are often not tagged as such.
    //             // This can be repaired later with option --railway.topology.repair
    //             currentWayMinusEdges.insert(currentWayMinusEdges.begin(), currentWayEdges.rbegin(), currentWayEdges.rbegin() + foundForward);
    //             fakeMinus = true;
    //         }
    // #ifdef DEBUG_CONSTRUCT_ROUTE
    //         if (pTLine->getLineID() == DEBUGLINEID) {
    //             std::cout << " way=" << (*it3)
    //                       << " done=" << toString(edges)
    //                       << " first=" << Named::getIDSecure(first)
    //                       << " last=" << Named::getIDSecure(last)
    //                       << "\n    +=" << toString(currentWayEdges)
    //                       << "\n    -=" << toString(currentWayMinusEdges)
    //                       << "\n   p+=" << toString(prevWayEdges)
    //                       << "\n   p-=" << toString(prevWayMinusEdges)
    //                       << "\n";
    //         }
    // #endif
    //         if (currentWayEdges.empty()) {
    //             continue;
    //         }
    //         if (last == currentWayEdges.front()->getFromNode() && last != nullptr) {
    //             if (!prevWayEdges.empty()) {
    //                 edges.insert(edges.end(), prevWayEdges.begin(), prevWayEdges.end());
    //                 prevWayEdges.clear();
    //                 prevWayMinusEdges.clear();
    //             }
    //             edges.insert(edges.end(), currentWayEdges.begin(), currentWayEdges.end());
    //             last = currentWayEdges.back()->getToNode();
    //         } else if (last == currentWayEdges.back()->getToNode() && last != nullptr) {
    //             if (!prevWayEdges.empty()) {
    //                 edges.insert(edges.end(), prevWayEdges.begin(), prevWayEdges.end());
    //                 prevWayEdges.clear();
    //                 prevWayMinusEdges.clear();
    //             }
    //             if (currentWayMinusEdges.empty()) {
    //                 currentWayEdges.clear();
    //                 last = nullptr;
    //                 continue;
    //             } else {
    //                 edges.insert(edges.end(), currentWayMinusEdges.begin(), currentWayMinusEdges.end());
    //                 if (fakeMinus) {
    //                     last = currentWayMinusEdges.back()->getFromNode();
    //                 } else {
    //                     last = currentWayMinusEdges.back()->getToNode();
    //                 }
    //             }
    //         } else if (first == currentWayEdges.front()->getFromNode() && first != nullptr) {
    //             edges.insert(edges.end(), prevWayMinusEdges.begin(), prevWayMinusEdges.end());
    //             edges.insert(edges.end(), currentWayEdges.begin(), currentWayEdges.end());
    //             last = currentWayEdges.back()->getToNode();
    //             prevWayEdges.clear();
    //             prevWayMinusEdges.clear();
    //         } else if (first == currentWayEdges.back()->getToNode() && first != nullptr) {
    //             edges.insert(edges.end(), prevWayMinusEdges.begin(), prevWayMinusEdges.end());
    //             if (currentWayMinusEdges.empty()) {
    //                 currentWayEdges.clear();
    //                 last = nullptr;
    //                 prevWayEdges.clear();
    //                 prevWayMinusEdges.clear();
    //                 continue;
    //             } else {
    //                 edges.insert(edges.end(), currentWayMinusEdges.begin(), currentWayMinusEdges.end());
    //                 last = currentWayMinusEdges.back()->getToNode();
    //                 prevWayEdges.clear();
    //                 prevWayMinusEdges.clear();
    //             }
    //         } else {
    //             if (it3 != pTLine->getWays().begin()) {
    // #ifdef DEBUG_CONSTRUCT_ROUTE
    //                 if (pTLine->getLineID() == DEBUGLINEID) {
    //                     std::cout << " way " << (*it3)
    //                               << " is not the start of ptline " << pTLine->getLineID()
    //                               << " (" + pTLine->getName() + ")\n";
    //                 }
    // #endif
    //             } else if (pTLine->getWays().size() == 1) {
    //                 if (currentWayEdges.size() > 0) {
    //                     edges.insert(edges.end(), currentWayEdges.begin(), currentWayEdges.end());
    //                 } else {
    //                     edges.insert(edges.end(), currentWayMinusEdges.begin(), currentWayMinusEdges.end());
    //                 }
    //             }
    //             prevWayEdges = currentWayEdges;
    //             prevWayMinusEdges = currentWayMinusEdges;
    //             if (!prevWayEdges.empty()) {
    //                 first = prevWayEdges.front()->getFromNode();
    //                 last = prevWayEdges.back()->getToNode();
    //             } else {
    //                 first = nullptr;
    //                 last = nullptr;
    //             }
    //         }
    //         currentWayEdges.clear();
    //         currentWayMinusEdges.clear();
    //     }
    //     if (edges.empty() && !currentWayMinusEdges.empty()) {
    //         edges = currentWayMinusEdges;
    //     }
    //     pTLine->setEdges(edges);
    // }
    #[allow(unused_variables)]
    pub fn construct_route(&mut self, myPTLine: Ptr<NBPTLine>, cont: &NBEdgeCont) {
        let mut edges: Vec<Ptr<NBEdge>> = Vec::new();

        let mut first = Ptr::null();
        let mut last = Ptr::null();
        let mut prevWayEdges: Vec<Ptr<NBEdge>> = Vec::new();
        let mut prevWayMinusEdges: Vec<Ptr<NBEdge>> = Vec::new();
        let mut currentWayEdges: Vec<Ptr<NBEdge>> = Vec::new();
        let mut currentWayMinusEdges: Vec<Ptr<NBEdge>> = Vec::new();
        let waysIds: Vec<String> = myPTLine.get_ways().clone();
        for (wIdx, way) in waysIds.iter().enumerate() {
            let mut foundForward = 0usize;
            if !cont.retrieve(way, false).is_null() {
                currentWayEdges.push(cont.retrieve(way, false));
                foundForward += 1;
            } else {
                let mut i = 0;
                while !cont.retrieve(&format!("{}#{}", way, i), true).is_null() {
                    if !cont.retrieve(&format!("{}#{}", way, i), false).is_null() {
                        currentWayEdges.push(cont.retrieve(&format!("{}#{}", way, i), false));
                        foundForward += 1;
                    }
                    i += 1;
                }
            }

            let mut foundReverse = 0usize;
            let minusWay = format!("-{}", way);
            if !cont.retrieve(&minusWay, false).is_null() {
                currentWayMinusEdges.push(cont.retrieve(&minusWay, false));
                foundReverse += 1;
            } else {
                let mut i = 0;
                while !cont.retrieve(&format!("-{}#{}", way, i), true).is_null() {
                    if !cont.retrieve(&format!("-{}#{}", way, i), false).is_null() {
                        let pos = currentWayMinusEdges.len() - foundReverse;
                        currentWayMinusEdges.insert(pos, cont.retrieve(&format!("-{}#{}", way, i), false));
                        foundReverse += 1;
                    }
                    i += 1;
                }
            }
            let mut fakeMinus = false;
            if foundReverse == 0 && foundForward > 0 && is_railway(myPTLine.get_v_class()) {
                // rail tracks may be used in both directions and are often not tagged as such.
                // This can be repaired later with option --railway.topology.repair
                let reversed: Vec<Ptr<NBEdge>> = currentWayEdges.iter().rev().take(foundForward).copied().collect();
                currentWayMinusEdges.splice(0..0, reversed);
                fakeMinus = true;
            }

            if currentWayEdges.is_empty() {
                continue;
            }
            let frontFrom = currentWayEdges[0].get_from_node();
            let backTo = currentWayEdges[currentWayEdges.len() - 1].get_to_node();
            if last == frontFrom && !last.is_null() {
                if !prevWayEdges.is_empty() {
                    edges.extend(prevWayEdges.iter().copied());
                    prevWayEdges.clear();
                    prevWayMinusEdges.clear();
                }
                edges.extend(currentWayEdges.iter().copied());
                last = backTo;
            } else if last == backTo && !last.is_null() {
                if !prevWayEdges.is_empty() {
                    edges.extend(prevWayEdges.iter().copied());
                    prevWayEdges.clear();
                    prevWayMinusEdges.clear();
                }
                if currentWayMinusEdges.is_empty() {
                    currentWayEdges.clear();
                    last = Ptr::null();
                    continue;
                } else {
                    edges.extend(currentWayMinusEdges.iter().copied());
                    let minusLast = currentWayMinusEdges[currentWayMinusEdges.len() - 1];
                    if fakeMinus {
                        last = minusLast.get_from_node();
                    } else {
                        last = minusLast.get_to_node();
                    }
                }
            } else if first == frontFrom && !first.is_null() {
                edges.extend(prevWayMinusEdges.iter().copied());
                edges.extend(currentWayEdges.iter().copied());
                last = backTo;
                prevWayEdges.clear();
                prevWayMinusEdges.clear();
            } else if first == backTo && !first.is_null() {
                edges.extend(prevWayMinusEdges.iter().copied());
                if currentWayMinusEdges.is_empty() {
                    currentWayEdges.clear();
                    last = Ptr::null();
                    prevWayEdges.clear();
                    prevWayMinusEdges.clear();
                    continue;
                } else {
                    edges.extend(currentWayMinusEdges.iter().copied());
                    last = currentWayMinusEdges[currentWayMinusEdges.len() - 1].get_to_node();
                    prevWayEdges.clear();
                    prevWayMinusEdges.clear();
                }
            } else {
                if wIdx != 0 {
                    // way is not the start of the ptline
                } else if waysIds.len() == 1 {
                    if currentWayEdges.len() > 0 {
                        edges.extend(currentWayEdges.iter().copied());
                    } else {
                        edges.extend(currentWayMinusEdges.iter().copied());
                    }
                }
                prevWayEdges = currentWayEdges.clone();
                prevWayMinusEdges = currentWayMinusEdges.clone();
                if !prevWayEdges.is_empty() {
                    first = prevWayEdges[0].get_from_node();
                    last = prevWayEdges[prevWayEdges.len() - 1].get_to_node();
                } else {
                    first = Ptr::null();
                    last = Ptr::null();
                }
            }
            currentWayEdges.clear();
            currentWayMinusEdges.clear();
        }
        if edges.is_empty() && !currentWayMinusEdges.is_empty() {
            edges = currentWayMinusEdges.clone();
        }
        myPTLine.m().set_edges(&edges);
    }

    /// C++: `double NBPTLineCont::getCost(const NBEdgeCont &, SUMOAbstractRouter<NBRouterEdge, NBVehicle> &, const std::shared_ptr<NBPTStop>, const std::shared_ptr<NBPTStop>, const NBVehicle *)` [static, private]
    // double
    // NBPTLineCont::getCost(const NBEdgeCont& ec, SUMOAbstractRouter<NBRouterEdge, NBVehicle>& router,
    //                       const std::shared_ptr<NBPTStop> from, const std::shared_ptr<NBPTStop> to, const NBVehicle* veh) {
    //     NBEdge* fromEdge = ec.getByID(from->getEdgeId());
    //     NBEdge* toEdge = ec.getByID(to->getEdgeId());
    //     if (fromEdge == nullptr || toEdge == nullptr) {
    //         return std::numeric_limits<double>::max();
    //     } else if (fromEdge == toEdge) {
    //         if (from->getEndPos() <= to->getEndPos()) {
    //             return to->getEndPos() - from->getEndPos();
    //         } else {
    //             return std::numeric_limits<double>::max();
    //         }
    //     } else if (fromEdge->getBidiEdge() == toEdge) {
    //         return std::numeric_limits<double>::max();
    //     }
    //     std::vector<const NBRouterEdge*> route;
    //     router.compute(fromEdge, toEdge, veh, 0, route);
    //     if (route.size() == 0) {
    //         return std::numeric_limits<double>::max();
    //     } else {
    //         return router.recomputeCosts(route, veh, 0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_cost(ec: &NBEdgeCont, router: &mut SUMOAbstractRouter<NBRouterEdge, NBVehicle>, from: Rc<NBPTStop>, to: Rc<NBPTStop>, veh: Ptr<NBVehicle>) -> f64 {
        // shortcut: SUMOAbstractRouter::Operation is Unmapped and NBRouterEdge is not linked to NBEdge yet, so routing costs cannot be computed; upgrade when the router is ported
        let _ = (ec, router, from, to, veh);
        panic!("NBPTLineCont::getCost: shortcut, router operations are not mapped yet");
    }

    /// C++: `std::string NBPTLineCont::getWayID(const std::string &)` [static, private]
    // std::string
    // NBPTLineCont::getWayID(const std::string& edgeID) {
    //     std::size_t found = edgeID.rfind("#");
    //     std::string result = edgeID;
    //     if (found != std::string::npos) {
    //         result = edgeID.substr(0, found);
    //     }
    //     if (result[0] == '-') {
    //         result = result.substr(1);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_way_id(edgeID: &str) -> String {
        let mut result = edgeID.to_string();
        if let Some(found) = edgeID.rfind('#') {
            result = edgeID[..found].to_string();
        }
        if result.starts_with('-') {
            result = result[1..].to_string();
        }
        result
    }

}

