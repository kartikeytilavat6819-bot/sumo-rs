//! Port of netbuild/NBPTStopCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<std::string, std::shared_ptr<NBPTStop>> NBPTStopCont::PTStopsCont`
pub type NBPTStopCont_PTStopsCont = BTreeMap<String, Rc<NBPTStop>>;

/// Container for public transport stops during the net building process
/// C++ class `NBPTStopCont` (netbuild/NBPTStopCont.h)
pub struct NBPTStopCont {
    /// The map of names to pt stops
    pub myPTStops: NBPTStopCont_PTStopsCont,
    /// The map of edge ids to stops
    pub myPTStopLookup: BTreeMap<String, Vec<Rc<NBPTStop>>>,
    pub myFloatingStops: Vec<Rc<NBPTStop>>,
}

// C++ static members of NBPTStopCont (port as module-level statics, see AGENTS.md):
//   static std::set<std::string> NBPTStopCont::myIgnoredStops;

impl NBPTStopCont {
    /// C++: `void NBPTStopCont::~NBPTStopCont()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBPTStopCont::~NBPTStopCont")
    }

    /// C++: `bool NBPTStopCont::insert(std::shared_ptr<NBPTStop>, bool)`
    /// Inserts a node into the map
    // bool
    // NBPTStopCont::insert(std::shared_ptr<NBPTStop> ptStop, bool floating) {
    //     std::string id = ptStop->getID();
    //     auto i = myPTStops.find(id);
    //     if (i != myPTStops.end()) {
    //         return false;
    //     }
    //     myPTStops[id] = ptStop;
    //     if (floating) {
    //         myFloatingStops.push_back(ptStop);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert(&mut self, ptStop: Rc<NBPTStop>, floating: bool) -> bool {
        todo!("port NBPTStopCont::insert")
    }

    /// C++: `std::shared_ptr<NBPTStop> NBPTStopCont::get(std::string) const` [const]
    /// Retrieve a previously inserted pt stop
    // std::shared_ptr<NBPTStop>
    // NBPTStopCont::get(std::string id) const {
    //     if (myPTStops.find(id) != myPTStops.end()) {
    //         return myPTStops.find(id)->second;
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get(&self, id: String) -> Rc<NBPTStop> {
        todo!("port NBPTStopCont::get")
    }

    /// C++: `const std::map<std::string, std::shared_ptr<NBPTStop>> & NBPTStopCont::getStops() const` [const]
    /// Returns an unmodifiable reference to the stored pt stops
    // const std::map<std::string, std::shared_ptr<NBPTStop> >& getStops() const {
    //         return myPTStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_stops(&self) -> &BTreeMap<String, Rc<NBPTStop>> {
        todo!("port NBPTStopCont::getStops")
    }

    /// C++: `int NBPTStopCont::cleanupDeleted(NBEdgeCont &, bool)`
    /// remove stops on non existing (removed) edges
    // int
    // NBPTStopCont::cleanupDeleted(NBEdgeCont& cont, bool reAssign) {
    //     int numDeleted = 0;
    //     for (auto i = myPTStops.begin(); i != myPTStops.end();) {
    //         auto stop = i->second;
    //         if (cont.getByID(stop->getEdgeId()) == nullptr) {
    //             NBEdge* reverse = nullptr;
    //             if (reAssign) {
    //                 reverse = cont.getByID("-" + stop->getEdgeId());
    //             }
    //             if (reverse == nullptr) {
    //                 WRITE_WARNINGF(TL("Removing pt stop '%' on non existing edge '%'."), i->first, i->second->getEdgeId());
    //                 i = myPTStops.erase(i);
    //                 numDeleted++;
    //             } else {
    //                 stop->replaceEdge(stop->getEdgeId(), {reverse});
    //                 i++;
    //             }
    //         } else {
    //             i++;
    //         }
    //     }
    //     return numDeleted;
    // }
    #[allow(unused_variables)]
    pub fn cleanup_deleted(&mut self, cont: &mut NBEdgeCont, reAssign: bool) -> i32 {
        todo!("port NBPTStopCont::cleanupDeleted")
    }

    /// C++: `void NBPTStopCont::assignLanes(NBEdgeCont &)`
    // void
    // NBPTStopCont::assignLanes(NBEdgeCont& cont) {
    //     //scnd pass set correct lane
    //     for (auto i = myPTStops.begin(); i != myPTStops.end();) {
    //         std::shared_ptr<NBPTStop> stop = i->second;
    //         if (!stop->findLaneAndComputeBusStopExtent(cont)) {
    //             WRITE_WARNINGF(TL("Could not find corresponding edge or compatible lane for pt stop '%' (%). Thus, it will be removed!"),
    //                            i->first, i->second->getName());
    //             //EdgeVector edgeVector = cont.getGeneratedFrom((*i).second->getOrigEdgeId());
    //             //std::cout << edgeVector.size() << std::endl;
    //             myPTStops.erase(i++);
    //         } else {
    //             i++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn assign_lanes(&mut self, cont: &mut NBEdgeCont) {
        todo!("port NBPTStopCont::assignLanes")
    }

    /// C++: `int NBPTStopCont::generateBidiStops(NBEdgeCont &)`
    /// duplicate stops for superposed rail edges and return the number of generated stops
    // int
    // NBPTStopCont::generateBidiStops(NBEdgeCont& ec) {
    //     int existingBidiStops = 0;
    //     std::vector<std::shared_ptr<NBPTStop> > toAdd;
    //     for (auto i = myPTStops.begin(); i != myPTStops.end(); i++) {
    //         std::shared_ptr<NBPTStop> stop = i->second;
    //         NBEdge* edge = ec.getByID(stop->getEdgeId());
    //         if (edge != nullptr && edge->isBidiRail()) {
    //             NBEdge* bidiEdge = edge->getTurnDestination(true);
    //             assert(bidiEdge != 0);
    //             const std::string id = getReverseID(stop->getID());
    //             if (myPTStops.count(id) > 0) {
    //                 if (myPTStops[id]->getEdgeId() != bidiEdge->getID()) {
    //                     WRITE_WARNINGF(TL("Could not create reverse-direction stop for superposed edge '%' (origStop '%'). Stop id '%' already in use by stop on edge '%'."),
    //                                    bidiEdge->getID(), i->first, id, myPTStops[id]->getEdgeId());
    //                 } else {
    //                     existingBidiStops++;
    //                 }
    //                 continue;
    //             }
    //             std::shared_ptr<NBPTStop> bidiStop = std::make_shared<NBPTStop>(stop->getElement(), id,
    //                                                  stop->getPosition(),
    //                                                  bidiEdge->getID(),
    //                                                  stop->getOrigEdgeId(),
    //                                                  stop->getLength(),
    //                                                  stop->getName(),
    //                                                  stop->getPermissions());
    //             if (bidiStop->findLaneAndComputeBusStopExtent(ec)) {
    //                 toAdd.push_back(bidiStop);
    //                 stop->setBidiStop(bidiStop);
    //                 bidiStop->setBidiStop(stop);
    //             } else {
    //                 // should not happen
    //                 assert(false);
    //             }
    //         } else if (edge != nullptr) {
    //             NBEdge* bidiEdge = edge->getTurnDestination(true);
    //             if (bidiEdge != nullptr) {
    //                 const std::string id = getReverseID(stop->getID());
    //                 if (myPTStops.count(id) > 0) {
    //                     existingBidiStops++;
    //                 }
    //             }
    //         }
    //     }
    //     for (std::shared_ptr<NBPTStop> newStop : toAdd) {
    //         myPTStops[newStop->getID()] = newStop;
    //     }
    //     if (toAdd.size() > 0) {
    //         WRITE_MESSAGEF(TL("Added % stops for superposed rail edges."), toString(toAdd.size()));
    //     }
    //     return (int)toAdd.size() + existingBidiStops;
    // }
    #[allow(unused_variables)]
    pub fn generate_bidi_stops(&mut self, cont: &mut NBEdgeCont) -> i32 {
        todo!("port NBPTStopCont::generateBidiStops")
    }

    /// C++: `int NBPTStopCont::countBidiStops(NBEdgeCont &) const` [const]
    /// count number of stop-pairs for superposed rail-edges
    // int
    // NBPTStopCont::countBidiStops(NBEdgeCont& ec) const {
    //     int existingBidiStops = 0;
    //     for (auto item : myPTStops) {
    //         auto stop = item.second;
    //         NBEdge* edge = ec.getByID(stop->getEdgeId());
    //         if (edge != nullptr && edge->isBidiRail()) {
    //             NBEdge* bidiEdge = edge->getTurnDestination(true);
    //             assert(bidiEdge != 0);
    //             const std::string id = getReverseID(stop->getID());
    //             // @note loaded pairs of bidi-stops might have arbitrary ids and we should rather search through all stops on bidiEdge
    //             auto it = myPTStops.find(id);
    //             if (it != myPTStops.end() && it->second->getEdgeId() == bidiEdge->getID()) {
    //                 existingBidiStops++;
    //             }
    //         }
    //     }
    //     return existingBidiStops;
    // }
    #[allow(unused_variables)]
    pub fn count_bidi_stops(&self, cont: &mut NBEdgeCont) -> i32 {
        todo!("port NBPTStopCont::countBidiStops")
    }

    /// C++: `void NBPTStopCont::localizePTStops(NBEdgeCont &)`
    // void
    // NBPTStopCont::localizePTStops(NBEdgeCont& cont) {
    //     std::vector<std::shared_ptr<NBPTStop> > reverseStops;
    //     //first pass localize pt stop at correct side of the street; create stop for opposite side if needed
    //     for (const auto& ptStopIt : myPTStops) {
    //         std::shared_ptr<NBPTStop> const stop = ptStopIt.second;
    //         bool multipleStopPositions = stop->getIsMultipleStopPositions();
    //         bool platformsDefined = !stop->getPlatformCands().empty();
    //         if (!platformsDefined) {
    //             //create pt stop for reverse edge if edge exists
    //             std::shared_ptr<NBPTStop> reverseStop = getReverseStop(stop, cont);
    //             if (reverseStop != nullptr) {
    //                 reverseStops.push_back(reverseStop);
    //             }
    //         } else if (multipleStopPositions) {
    //             //create pt stop for closest platform at corresponding edge
    //             assignPTStopToEdgeOfClosestPlatform(stop, cont);
    //         } else {
    //             //create pt stop for each side of the street where a platform is defined (create additional pt stop as needed)
    //             std::shared_ptr<NBPTStop> additionalStop = assignAndCreatNewPTStopAsNeeded(stop, cont);
    //             if (additionalStop != nullptr) {
    //                 reverseStops.push_back(additionalStop);
    //             }
    //         }
    //     }
    //     //insert new stops if any
    //     for (std::shared_ptr<NBPTStop>& reverseStop : reverseStops) {
    //         insert(reverseStop);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn localize_pt_stops(&mut self, cont: &mut NBEdgeCont) {
        todo!("port NBPTStopCont::localizePTStops")
    }

    /// C++: `void NBPTStopCont::assignEdgeForFloatingStops(NBEdgeCont &, double)`
    // void
    // NBPTStopCont::assignEdgeForFloatingStops(NBEdgeCont& cont, double maxRadius) {
    //     NamedRTree r;
    //     SVCPermissions publicPermissions = SVC_BUS | SVC_TRAM | SVC_RAIL | SVC_RAIL_URBAN | SVC_TAXI;
    //     for (const auto& item : cont) {
    //         NBEdge* edge = item.second;
    //         if ((edge->getPermissions() & publicPermissions) == 0) {
    //             continue;
    //         }
    //         const Boundary& bound = edge->getGeometry().getBoxBoundary();
    //         float min[2] = { static_cast<float>(bound.xmin()), static_cast<float>(bound.ymin()) };
    //         float max[2] = { static_cast<float>(bound.xmax()), static_cast<float>(bound.ymax()) };
    //         r.Insert(min, max, edge);
    //     }
    //     for (std::shared_ptr<NBPTStop> ptStop : myFloatingStops) {
    //         std::set<const Named*> edges;
    //         Named::StoringVisitor visitor(edges);
    //         const Position& pos = ptStop->getPosition();
    //         float min[2] = {static_cast<float>(pos.x() - maxRadius), static_cast<float>(pos.y() - maxRadius)};
    //         float max[2] = {static_cast<float>(pos.x() + maxRadius), static_cast<float>(pos.y() + maxRadius)};
    //         r.Search(min, max, visitor);
    //         std::vector<NBEdge*> nearby;
    //         for (const Named* namedEdge : edges) {
    //             NBEdge* e = const_cast<NBEdge*>(dynamic_cast<const NBEdge*>(namedEdge));
    //             if ((e->getPermissions() & ptStop->getPermissions()) != 0) {
    //                 nearby.push_back(e);
    //             }
    //         }
    //         std::sort(nearby.begin(), nearby.end(), [pos](NBEdge * a, NBEdge * b) {
    //             return a->getLaneShape(0).distance2D(pos, false) < b->getLaneShape(0).distance2D(pos, false);
    //         });
    //
    //         for (NBEdge* e : nearby) {
    //             ptStop->setEdgeId(e->getID(), cont);
    //             if (ptStop->getLaneId() != "") {
    //                 break;
    //             }
    //         }
    //         if (ptStop->getLaneId() == "") {
    //             WRITE_WARNINGF(TL("Could not find corresponding edge or compatible lane for free-floating pt stop '%' (%). Thus, it will be removed!"),
    //                            ptStop->getID(), ptStop->getName());
    //             myPTStops.erase(ptStop->getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn assign_edge_for_floating_stops(&mut self, cont: &mut NBEdgeCont, maxRadius: f64) {
        todo!("port NBPTStopCont::assignEdgeForFloatingStops")
    }

    /// C++: `void NBPTStopCont::findAccessEdgesForRailStops(NBEdgeCont &, double, int, double)`
    // void
    // NBPTStopCont::findAccessEdgesForRailStops(NBEdgeCont& cont, double maxRadius, int maxCount, double accessFactor) {
    //     NamedRTree r;
    //     for (auto edge : cont) {
    //         const Boundary& bound = edge.second->getGeometry().getBoxBoundary();
    //         float min[2] = { static_cast<float>(bound.xmin()), static_cast<float>(bound.ymin()) };
    //         float max[2] = { static_cast<float>(bound.xmax()), static_cast<float>(bound.ymax()) };
    //         r.Insert(min, max, edge.second);
    //     }
    //     for (auto& ptStop : myPTStops) {
    //         const std::string& stopEdgeID = ptStop.second->getEdgeId();
    //         NBEdge* stopEdge = cont.getByID(stopEdgeID);
    //         //std::cout << "findAccessEdgesForRailStops edge=" << stopEdgeID << " exists=" << (stopEdge != 0) << "\n";
    //         if (stopEdge != nullptr && (stopEdge->getPermissions() & SVC_PEDESTRIAN) == 0) {
    //             //if (stopEdge != 0 && isRailway(stopEdge->getPermissions())) {
    //             std::set<const Named*> edges;
    //             Named::StoringVisitor visitor(edges);
    //             const Position& pos = ptStop.second->getPosition();
    //             float min[2] = {static_cast<float>(pos.x() - maxRadius), static_cast<float>(pos.y() - maxRadius)};
    //             float max[2] = {static_cast<float>(pos.x() + maxRadius), static_cast<float>(pos.y() + maxRadius)};
    //             r.Search(min, max, visitor);
    //             std::vector<NBEdge*> edgCants;
    //             for (const Named* namedEdge : edges) {
    //                 NBEdge* e = const_cast<NBEdge*>(dynamic_cast<const NBEdge*>(namedEdge));
    //                 edgCants.push_back(e);
    //             }
    //             std::sort(edgCants.begin(), edgCants.end(), [pos](NBEdge * a, NBEdge * b) {
    //                 return a->getLaneShape(0).distance2D(pos, false) < b->getLaneShape(0).distance2D(pos, false);
    //             });
    //             int cnt = 0;
    //             for (auto edge : edgCants) {
    //                 int laneIdx = 0;
    //                 for (auto lane : edge->getLanes()) {
    //                     if ((lane.permissions & SVC_PEDESTRIAN) != 0) {
    //                         double offset = lane.shape.nearest_offset_to_point2D(pos, false);
    //                         double finalLength = edge->getFinalLength();
    //                         double laneLength = lane.shape.length();
    //                         double accessLength = pos.distanceTo2D(lane.shape.positionAtOffset2D(offset)) * accessFactor;
    //                         ptStop.second->addAccess(edge->getLaneID(laneIdx), offset * finalLength / laneLength, accessLength);
    //                         cnt++;
    //                         break;
    //                     }
    //                     laneIdx++;
    //                 }
    //                 if (cnt == maxCount) {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_access_edges_for_rail_stops(&mut self, cont: &mut NBEdgeCont, maxRadius: f64, maxCount: i32, accessFactor: f64) {
        todo!("port NBPTStopCont::findAccessEdgesForRailStops")
    }

    /// C++: `void NBPTStopCont::postprocess(std::set<std::string> &)`
    // void
    // NBPTStopCont::postprocess(std::set<std::string>& usedStops) {
    //     for (auto i = myPTStops.begin(); i != myPTStops.end();) {
    //         if (usedStops.find(i->second->getID()) == usedStops.end()) {
    //             myPTStops.erase(i++);
    //         } else {
    //             i++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn postprocess(&mut self, usedStops: &mut BTreeSet<String>) {
        todo!("port NBPTStopCont::postprocess")
    }

    /// C++: `void NBPTStopCont::addEdges2Keep(const OptionsCont &, std::set<std::string> &)`
    /// add edges that must be kept
    // void
    // NBPTStopCont::addEdges2Keep(const OptionsCont& /* oc */, std::set<std::string>& into) {
    //     for (auto stop : myPTStops) {
    //         into.insert(stop.second->getEdgeId());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edges2_keep(&mut self, oc: &OptionsCont, into: &mut BTreeSet<String>) {
        todo!("port NBPTStopCont::addEdges2Keep")
    }

    /// C++: `void NBPTStopCont::replaceEdge(const std::string &, const std::vector<NBEdge *> &)`
    /// replace the edge with the closes edge on the given edge list in all stops
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, edgeID: &str, replacement: &Vec<Ptr<NBEdge>>) {
        todo!("port NBPTStopCont::replaceEdge")
    }

    /// C++: `std::shared_ptr<NBPTStop> NBPTStopCont::findStop(const std::string &, Position, double) const` [const]
    // std::shared_ptr<NBPTStop>
    // NBPTStopCont::findStop(const std::string& origEdgeID, Position pos, double threshold) const {
    //     for (auto& item : myPTStops) {
    //         if (item.second->getOrigEdgeId() == origEdgeID &&
    //                 item.second->getPosition().distanceTo2D(pos) < threshold) {
    //             return item.second;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn find_stop(&self, origEdgeID: &str, pos: Position, threshold: f64) -> Rc<NBPTStop> {
        todo!("port NBPTStopCont::findStop")
    }

    /// C++: `std::shared_ptr<NBPTStop> NBPTStopCont::getReverseStop(std::shared_ptr<NBPTStop>, const NBEdgeCont &)`
    // std::shared_ptr<NBPTStop>
    // NBPTStopCont::getReverseStop(std::shared_ptr<NBPTStop> pStop, const NBEdgeCont& ec) {
    //     std::string edgeId = pStop->getEdgeId();
    //     NBEdge* edge = ec.getByID(edgeId);
    //     NBEdge* reverse = NBPTStopCont::getReverseEdge(edge);
    //     if (reverse != nullptr) {
    //         const std::string reverseID = getReverseID(pStop->getID());
    //         if (myPTStops.count(reverseID) == 0) {
    //             std::shared_ptr<NBPTStop> result = std::make_shared<NBPTStop>(pStop->getElement(), reverseID, pStop->getPosition(), reverse->getID(), reverse->getID(),
    //                                               pStop->getLength(), pStop->getName(), pStop->getPermissions());
    //             for (const std::string& line : pStop->getLines()) {
    //                 result->addLine(line);
    //             }
    //             return result;
    //         } else {
    //             return myPTStops[reverseID];
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_reverse_stop(&mut self, pStop: Rc<NBPTStop>, ec: &NBEdgeCont) -> Rc<NBPTStop> {
        todo!("port NBPTStopCont::getReverseStop")
    }

    /// C++: `void NBPTStopCont::assignPTStopToEdgeOfClosestPlatform(std::shared_ptr<NBPTStop>, NBEdgeCont &)` [private]
    // void
    // NBPTStopCont::assignPTStopToEdgeOfClosestPlatform(std::shared_ptr<NBPTStop> pStop, NBEdgeCont& cont) {
    //     std::string edgeId = pStop->getEdgeId();
    //     NBEdge* edge = cont.getByID(edgeId);
    //     NBEdge* reverse = NBPTStopCont::getReverseEdge(edge);
    //     const NBPTPlatform* closestPlatform = getClosestPlatformToPTStopPosition(pStop);
    //     pStop->setPTStopLength(closestPlatform->getLength());
    //     if (reverse != nullptr) {
    //
    //         //TODO make isLeft in PositionVector static [GL May '17]
    // //        if (PositionVector::isLeft(edge->getFromNode()->getPosition(),edge->getToNode()->getPosition(),closestPlatform)){
    // //
    // //        }
    //         double crossProd = computeCrossProductEdgePosition(edge, closestPlatform->getPos());
    //
    //         //TODO consider driving on the left!!! [GL May '17]
    //         if (crossProd > 0) { //pt stop is on the left of the orig edge
    //             pStop->setEdgeId(reverse->getID(), cont);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn assign_pt_stop_to_edge_of_closest_platform(&mut self, pStop: Rc<NBPTStop>, cont: &mut NBEdgeCont) {
        todo!("port NBPTStopCont::assignPTStopToEdgeOfClosestPlatform")
    }

    /// C++: `const NBPTPlatform * NBPTStopCont::getClosestPlatformToPTStopPosition(std::shared_ptr<NBPTStop>)` [private]
    // const NBPTPlatform*
    // NBPTStopCont::getClosestPlatformToPTStopPosition(std::shared_ptr<NBPTStop> pStop) {
    //     Position stopPosition = pStop->getPosition();
    //     const NBPTPlatform* closest = nullptr;
    //     double minSqrDist = std::numeric_limits<double>::max();
    //     for (const NBPTPlatform& platform : pStop->getPlatformCands()) {
    //         double sqrDist = stopPosition.distanceSquaredTo2D(platform.getPos());
    //         if (sqrDist < minSqrDist) {
    //             minSqrDist = sqrDist;
    //             closest = &platform;
    //         }
    //     }
    //     return closest;
    // }
    #[allow(unused_variables)]
    pub fn get_closest_platform_to_pt_stop_position(&mut self, pStop: Rc<NBPTStop>) -> Ptr<NBPTPlatform> {
        todo!("port NBPTStopCont::getClosestPlatformToPTStopPosition")
    }

    /// C++: `std::shared_ptr<NBPTStop> NBPTStopCont::assignAndCreatNewPTStopAsNeeded(std::shared_ptr<NBPTStop>, NBEdgeCont &)` [private]
    // std::shared_ptr<NBPTStop>
    // NBPTStopCont::assignAndCreatNewPTStopAsNeeded(std::shared_ptr<NBPTStop> pStop, NBEdgeCont& cont) {
    //     std::string edgeId = pStop->getEdgeId();
    //     NBEdge* edge = cont.getByID(edgeId);
    //     if (edge == nullptr) {
    //         return nullptr;
    //     }
    //     bool rightOfEdge = false;
    //     bool leftOfEdge = false;
    //     const NBPTPlatform* left = nullptr;
    //     for (const NBPTPlatform& platform : pStop->getPlatformCands()) {
    //         double crossProd = computeCrossProductEdgePosition(edge, platform.getPos());
    //         //TODO consider driving on the left!!! [GL May '17]
    //         if (crossProd > 0) {
    //             leftOfEdge = true;
    //             left = &platform;
    //         } else {
    //             rightOfEdge = true;
    //             pStop->setPTStopLength(platform.getLength());
    //         }
    //     }
    //
    //     if (leftOfEdge && rightOfEdge) {
    //         std::shared_ptr<NBPTStop> leftStop = getReverseStop(pStop, cont);
    //         if (leftStop) {
    //             leftStop->setPTStopLength(left->getLength());
    //         }
    //         return leftStop;
    //     } else if (leftOfEdge) {
    //         NBEdge* reverse = getReverseEdge(edge);
    //         if (reverse != nullptr) {
    //             pStop->setEdgeId(reverse->getID(), cont);
    //             pStop->setPTStopLength(left->getLength());
    //         }
    //     }
    //
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn assign_and_creat_new_pt_stop_as_needed(&mut self, pStop: Rc<NBPTStop>, cont: &mut NBEdgeCont) -> Rc<NBPTStop> {
        todo!("port NBPTStopCont::assignAndCreatNewPTStopAsNeeded")
    }

    /// C++: `double NBPTStopCont::computeCrossProductEdgePosition(const NBEdge *, const Position &) const` [const, private]
    // double
    // NBPTStopCont::computeCrossProductEdgePosition(const NBEdge* edge, const Position& closestPlatform) const {
    //     PositionVector geom = edge->getGeometry();
    //     int idxTmp = geom.indexOfClosest(closestPlatform);
    //     double offset = geom.nearest_offset_to_point2D(closestPlatform, true);
    //     double offset2 = geom.offsetAtIndex2D(idxTmp);
    //     int idx1, idx2;
    //     if (offset2 < offset) {
    //         idx1 = idxTmp;
    //         idx2 = idx1 + 1;
    //     } else {
    //         idx2 = idxTmp;
    //         idx1 = idxTmp - 1;
    //     }
    //     if (idx1 < 0 || idx1 >= (int) geom.size() || idx2 < 0 || idx2 >= (int) geom.size()) {
    //         WRITE_WARNINGF(TL("Could not determine cross product for edge '%'."), edge->getID());
    //         return 0;
    //     }
    //     Position p1 = geom[idx1];
    //     Position p2 = geom[idx2];
    //
    //     double x0 = p1.x();
    //     double y0 = p1.y();
    //     double x1 = p2.x();
    //     double y1 = p2.y();
    //     double x2 = closestPlatform.x();
    //     double y2 = closestPlatform.y();
    //     double crossProd = (x1 - x0) * (y2 - y0) - (y1 - y0) * (x2 - x0);
    //     return crossProd;
    // }
    #[allow(unused_variables)]
    pub fn compute_cross_product_edge_position(&self, edge: Ptr<NBEdge>, closestPlatform: &Position) -> f64 {
        todo!("port NBPTStopCont::computeCrossProductEdgePosition")
    }

    /// C++: `std::string NBPTStopCont::getReverseID(const std::string &)` [static, private]
    // std::string
    // NBPTStopCont::getReverseID(const std::string& id) {
    //     return id.size() > 0 && id[0] == '-' ? id.substr(1) : "-" + id;
    // }
    #[allow(unused_variables)]
    pub fn get_reverse_id(id: &str) -> String {
        todo!("port NBPTStopCont::getReverseID")
    }

    /// C++: `NBEdge * NBPTStopCont::getReverseEdge(NBEdge *)` [static]
    // NBEdge*
    // NBPTStopCont::getReverseEdge(NBEdge* edge) {
    //     if (edge != nullptr) {
    //         const PositionVector rGeom = edge->getGeometry().reverse();
    //         for (auto it = edge->getToNode()->getOutgoingEdges().begin();
    //                 it != edge->getToNode()->getOutgoingEdges().end();
    //                 it++) {
    //             if ((*it)->getToNode() == edge->getFromNode() && (*it)->getGeometry() == rGeom) {
    //                 return (*it);
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_reverse_edge(edge: Ptr<NBEdge>) -> Ptr<NBEdge> {
        todo!("port NBPTStopCont::getReverseEdge")
    }

    /// C++: `void NBPTStopCont::addIgnored(const std::string &)` [static]
    // static void addIgnored(const std::string& stopID) {
    //         myIgnoredStops.insert(stopID);
    //     }
    #[allow(unused_variables)]
    pub fn add_ignored(stopID: &str) {
        todo!("port NBPTStopCont::addIgnored")
    }

    /// C++: `bool NBPTStopCont::wasIgnored(const std::string &)` [static]
    // static bool wasIgnored(const std::string& stopID) {
    //         return myIgnoredStops.count(stopID) > 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_ignored(stopID: &str) -> bool {
        todo!("port NBPTStopCont::wasIgnored")
    }

    /// C++: `void NBPTStopCont::alignIdSigns()`
    // void
    // NBPTStopCont::alignIdSigns() {
    //     PTStopsCont stops = myPTStops;
    //     for (auto& i : stops) {
    //         std::shared_ptr<NBPTStop> s = i.second;
    //         const std::string& stopId = s->getID();
    //         if (s->getEdgeId() == "" || s->wasLoaded()) {
    //             continue;
    //         }
    //         const char edgeSign = s->getEdgeId().at(0);
    //         const char stopSign = stopId.at(0);
    //         if (edgeSign != stopSign && (edgeSign == '-' || stopSign == '-')) {
    //             const std::string reverseID = getReverseID(stopId);
    //             std::shared_ptr<NBPTStop> rs = get(reverseID);
    //             if (rs != nullptr && rs->wasLoaded()) {
    //                 continue;
    //             }
    //             s->setPTStopId(reverseID);
    //             myPTStops.erase(stopId);
    //             myPTStops[reverseID] = s;
    //             if (rs != nullptr) {
    //                 rs->setPTStopId(stopId);
    //                 myPTStops[stopId] = rs;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn align_id_signs(&mut self) {
        todo!("port NBPTStopCont::alignIdSigns")
    }

}

