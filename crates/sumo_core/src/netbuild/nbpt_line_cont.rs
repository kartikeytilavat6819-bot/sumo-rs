//! Port of netbuild/NBPTLineCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port NBPTLineCont::~NBPTLineCont")
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
        todo!("port NBPTLineCont::insert")
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
        todo!("port NBPTLineCont::retrieve")
    }

    /// C++: `const std::map<std::string, NBPTLine *> & NBPTLineCont::getLines() const` [const]
    // const std::map<std::string, NBPTLine*>& getLines() const {
    //         return myPTLines;
    //     }
    #[allow(unused_variables)]
    pub fn get_lines(&self) -> &BTreeMap<String, Ptr<NBPTLine>> {
        todo!("port NBPTLineCont::getLines")
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
        todo!("port NBPTLineCont::process")
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
        todo!("port NBPTLineCont::replaceEdge")
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
        todo!("port NBPTLineCont::fixBidiStops")
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
        todo!("port NBPTLineCont::removeInvalidEdges")
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
        todo!("port NBPTLineCont::fixPermissions")
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
        todo!("port NBPTLineCont::getServedPTStops")
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
        todo!("port NBPTLineCont::reviseStops")
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
        todo!("port NBPTLineCont::reviseSingleWayStops")
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
    pub fn find_way(&self, line: Ptr<NBPTLine>, stop: Rc<NBPTStop>, ec: &NBEdgeCont, sc: &mut NBPTStopCont) -> Rc<NBPTStop> {
        todo!("port NBPTLineCont::findWay")
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
        todo!("port NBPTLineCont::constructRoute")
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
        todo!("port NBPTLineCont::getCost")
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
        todo!("port NBPTLineCont::getWayID")
    }

}

