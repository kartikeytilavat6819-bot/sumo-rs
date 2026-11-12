//! Port of netbuild/NBPTStopCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

thread_local! {
    // C++ static member NBPTStopCont::myIgnoredStops
    static MY_IGNORED_STOPS: std::cell::RefCell<BTreeSet<String>> = std::cell::RefCell::new(BTreeSet::new());
}

/// C++ NamedRTree is RTree<Named*, Named, float, 2, ...>; here data is the edge pointer.
/// shortcut: NamedRTree overrides RectSphericalVolume; that only shapes the tree, and the
/// search result set (collected in an ordered set) is the same, so the generic RTree is used.
type StopEdgeTree = RTree<Ptr<NBEdge>, f32, 2, f32, 8, 4>;

/// C++ std::sort by distance of lane 0 shape to pos
fn sort_by_lane0_distance(edges: &mut Vec<Ptr<NBEdge>>, pos: &Position) {
    // determinism: C++ std::sort is unstable, ties may come out in another order
    edges.sort_by(|a, b| {
        let da = a.get_lane_shape(0).distance2d(pos, false);
        let db = b.get_lane_shape(0).distance2d(pos, false);
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    });
}

/// shortcut: C++ shared_ptr<NBPTStop> is mutated freely; Rc only gives &T, so this goes through
/// the raw pointer like Ptr::m(). Replace with RefCell<NBPTStop> once the owners are ported.
#[allow(clippy::mut_from_ref)]
fn stop_m(stop: &Rc<NBPTStop>) -> &mut NBPTStop {
    unsafe { &mut *(Rc::as_ptr(stop) as *mut NBPTStop) }
}

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
                self.myPTStops.clear();
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
                let id = ptStop.get_id();
        if self.myPTStops.contains_key(&id) {
            return false;
        }
        self.myPTStops.insert(id, ptStop.clone());
        if floating {
            self.myFloatingStops.push(ptStop);
        }
        true
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
    pub fn get(&self, id: String) -> Option<Rc<NBPTStop>> {
                // None where C++ returns nullptr
        self.myPTStops.get(&id).cloned()
    }

    /// C++: `const std::map<std::string, std::shared_ptr<NBPTStop>> & NBPTStopCont::getStops() const` [const]
    /// Returns an unmodifiable reference to the stored pt stops
    // const std::map<std::string, std::shared_ptr<NBPTStop> >& getStops() const {
    //         return myPTStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_stops(&self) -> &BTreeMap<String, Rc<NBPTStop>> {
                &self.myPTStops
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
                let mut numDeleted = 0;
        let keys: Vec<String> = self.myPTStops.keys().cloned().collect();
        for key in keys {
            let stop = self.myPTStops[&key].clone();
            let edgeId = stop.get_edge_id().to_string();
            if cont.get_by_id(&edgeId).is_null() {
                let mut reverse: Ptr<NBEdge> = Ptr::null();
                if reAssign {
                    reverse = cont.get_by_id(&format!("-{}", edgeId));
                }
                if reverse.is_null() {
                    write_warning(&format!("Removing pt stop '{}' on non existing edge '{}'.", key, edgeId));
                    self.myPTStops.remove(&key);
                    numDeleted += 1;
                } else {
                    stop_m(&stop).replace_edge(&edgeId, &vec![reverse]);
                }
            }
        }
        numDeleted
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
                // second pass: set correct lane
        let keys: Vec<String> = self.myPTStops.keys().cloned().collect();
        for key in keys {
            let stop = self.myPTStops[&key].clone();
            if !stop_m(&stop).find_lane_and_compute_bus_stop_extent(cont) {
                write_warning(&format!("Could not find corresponding edge or compatible lane for pt stop '{}' ({}). Thus, it will be removed!", key, stop.get_name()));
                self.myPTStops.remove(&key);
            }
        }
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
                let mut existingBidiStops: i32 = 0;
        let mut toAdd: Vec<Rc<NBPTStop>> = Vec::new();
        let items: Vec<(String, Rc<NBPTStop>)> = self.myPTStops.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        for (key, stop) in items {
            let edge = cont.get_by_id(&stop.get_edge_id().to_string());
            if !edge.is_null() && edge.is_bidi_rail(false) {
                let bidiEdge = edge.get_turn_destination(true);
                debug_assert!(!bidiEdge.is_null());
                let id = Self::get_reverse_id(&stop.get_id());
                if self.myPTStops.contains_key(&id) {
                    let existingEdge = self.myPTStops[&id].get_edge_id().to_string();
                    if existingEdge != bidiEdge.get_id() {
                        write_warning(&format!("Could not create reverse-direction stop for superposed edge '{}' (origStop '{}'). Stop id '{}' already in use by stop on edge '{}'.", bidiEdge.get_id(), key, id, existingEdge));
                    } else {
                        existingBidiStops += 1;
                    }
                    continue;
                }
                let bidiStop = Rc::new(NBPTStop::new(stop.get_element(), id.clone(), stop.get_position().clone(), bidiEdge.get_id().to_string(),
                                                     stop.get_orig_edge_id(), stop.get_length(), stop.get_name(), stop.get_permissions(),
                                                     0.0, RGBColor::new_valid(false), -1.0));
                if stop_m(&bidiStop).find_lane_and_compute_bus_stop_extent(cont) {
                    toAdd.push(bidiStop.clone());
                    stop_m(&stop).set_bidi_stop(bidiStop.clone());
                    stop_m(&bidiStop).set_bidi_stop(stop.clone());
                } else {
                    // should not happen
                    debug_assert!(false);
                }
            } else if !edge.is_null() {
                let bidiEdge = edge.get_turn_destination(true);
                if !bidiEdge.is_null() {
                    let id = Self::get_reverse_id(&stop.get_id());
                    if self.myPTStops.contains_key(&id) {
                        existingBidiStops += 1;
                    }
                }
            }
        }
        let added = toAdd.len();
        for newStop in toAdd {
            let id = newStop.get_id();
            self.myPTStops.insert(id, newStop);
        }
        if added > 0 {
            write_message(&format!("Added {} stops for superposed rail edges.", added));
        }
        added as i32 + existingBidiStops
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
                let mut existingBidiStops: i32 = 0;
        for stop in self.myPTStops.values() {
            let edge = cont.get_by_id(stop.get_edge_id());
            if !edge.is_null() && edge.is_bidi_rail(false) {
                let bidiEdge = edge.get_turn_destination(true);
                debug_assert!(!bidiEdge.is_null());
                let id = Self::get_reverse_id(&stop.get_id());
                if let Some(it) = self.myPTStops.get(&id) {
                    if it.get_edge_id() == bidiEdge.get_id() {
                        existingBidiStops += 1;
                    }
                }
            }
        }
        existingBidiStops
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
                let mut reverseStops: Vec<Rc<NBPTStop>> = Vec::new();
        // first pass: localize pt stop at correct side of the street; create stop for opposite side if needed
        let stops: Vec<Rc<NBPTStop>> = self.myPTStops.values().cloned().collect();
        for stop in stops {
            let multipleStopPositions = stop.get_is_multiple_stop_positions();
            let platformsDefined = !stop_m(&stop).get_platform_cands().is_empty();
            if !platformsDefined {
                // create pt stop for reverse edge if edge exists
                if let Some(reverseStop) = self.get_reverse_stop(stop.clone(), cont) {
                    reverseStops.push(reverseStop);
                }
            } else if multipleStopPositions {
                // create pt stop for closest platform at corresponding edge
                self.assign_pt_stop_to_edge_of_closest_platform(stop.clone(), cont);
            } else {
                // create pt stop for each side of the street where a platform is defined (create additional pt stop as needed)
                if let Some(additionalStop) = self.assign_and_creat_new_pt_stop_as_needed(stop.clone(), cont) {
                    reverseStops.push(additionalStop);
                }
            }
        }
        // insert new stops if any
        for reverseStop in reverseStops {
            self.insert(reverseStop, false);
        }
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
                let publicPermissions: SVCPermissions = SVC_BUS | SVC_TRAM | SVC_RAIL | SVC_RAIL_URBAN | SVC_TAXI;
        let mut r: StopEdgeTree = RTree::default();
        for edgeItem in cont.myEdges.values() {
            let edge = *edgeItem;
            if (edge.get_permissions(-1) & publicPermissions) == 0 {
                continue;
            }
            let bound = edge.get_geometry().get_box_boundary();
            let min = [bound.xmin() as f32, bound.ymin() as f32];
            let max = [bound.xmax() as f32, bound.ymax() as f32];
            r.insert(&min, &max, &edge);
        }
        let floatingStops: Vec<Rc<NBPTStop>> = self.myFloatingStops.clone();
        for ptStop in floatingStops {
            let mut edges: BTreeSet<Ptr<NBEdge>> = BTreeSet::new();
            let pos = ptStop.get_position().clone();
            let min = [(pos.x() - maxRadius) as f32, (pos.y() - maxRadius) as f32];
            let max = [(pos.x() + maxRadius) as f32, (pos.y() + maxRadius) as f32];
            r.search(&min, &max, &(), |d, _| {
                edges.insert(*d);
            });
            let mut nearby: Vec<Ptr<NBEdge>> = Vec::new();
            for e in &edges {
                if (e.get_permissions(-1) & ptStop.get_permissions()) != 0 {
                    nearby.push(*e);
                }
            }
            sort_by_lane0_distance(&mut nearby, &pos);
            for e in nearby {
                stop_m(&ptStop).set_edge_id(e.get_id().to_string(), cont);
                if !ptStop.get_lane_id().is_empty() {
                    break;
                }
            }
            if ptStop.get_lane_id().is_empty() {
                write_warning(&format!("Could not find corresponding edge or compatible lane for free-floating pt stop '{}' ({}). Thus, it will be removed!", ptStop.get_id(), ptStop.get_name()));
                self.myPTStops.remove(&ptStop.get_id());
            }
        }
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
                let mut r: StopEdgeTree = RTree::default();
        for edgeItem in cont.myEdges.values() {
            let edge = *edgeItem;
            let bound = edge.get_geometry().get_box_boundary();
            let min = [bound.xmin() as f32, bound.ymin() as f32];
            let max = [bound.xmax() as f32, bound.ymax() as f32];
            r.insert(&min, &max, &edge);
        }
        let stops: Vec<Rc<NBPTStop>> = self.myPTStops.values().cloned().collect();
        for ptStop in stops {
            let stopEdgeID = ptStop.get_edge_id().to_string();
            let stopEdge = cont.get_by_id(&stopEdgeID);
            if !stopEdge.is_null() && (stopEdge.get_permissions(-1) & SVC_PEDESTRIAN) == 0 {
                let mut edges: BTreeSet<Ptr<NBEdge>> = BTreeSet::new();
                let pos = ptStop.get_position().clone();
                let min = [(pos.x() - maxRadius) as f32, (pos.y() - maxRadius) as f32];
                let max = [(pos.x() + maxRadius) as f32, (pos.y() + maxRadius) as f32];
                r.search(&min, &max, &(), |d, _| {
                    edges.insert(*d);
                });
                let mut edgCants: Vec<Ptr<NBEdge>> = edges.into_iter().collect();
                sort_by_lane0_distance(&mut edgCants, &pos);
                let mut cnt = 0;
                for edge in edgCants {
                    let mut laneIdx: i32 = 0;
                    for lane in edge.get_lanes() {
                        if (lane.permissions & SVC_PEDESTRIAN) != 0 {
                            let offset = lane.shape.nearest_offset_to_point2d(&pos, false);
                            let finalLength = edge.get_final_length();
                            let laneLength = lane.shape.length();
                            let accessLength = pos.distance_to2d(&lane.shape.position_at_offset2d(offset, 0.0, false)) * accessFactor;
                            stop_m(&ptStop).add_access(edge.get_lane_id(laneIdx), offset * finalLength / laneLength, accessLength);
                            cnt += 1;
                            break;
                        }
                        laneIdx += 1;
                    }
                    if cnt == maxCount {
                        break;
                    }
                }
            }
        }
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
                self.myPTStops.retain(|_, stop| usedStops.contains(&stop.get_id()));
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
                for stop in self.myPTStops.values() {
            into.insert(stop.get_edge_id().to_string());
        }
    }

    /// C++: `void NBPTStopCont::replaceEdge(const std::string &, const std::vector<NBEdge *> &)`
    /// replace the edge with the closes edge on the given edge list in all stops
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, edgeID: &str, replacement: &Vec<Ptr<NBEdge>>) {
                if self.myPTStops.len() > 0 && self.myPTStopLookup.len() == 0 {
            // init lookup once
            for item in self.myPTStops.values() {
                self.myPTStopLookup.entry(item.get_edge_id().to_string()).or_default().push(item.clone());
            }
        }
        // make a copy because the vector gets modified
        let stops: Vec<Rc<NBPTStop>> = self.myPTStopLookup.entry(edgeID.to_string()).or_default().clone();
        for stop in stops {
            if !stop_m(&stop).replace_edge(edgeID, replacement) {
                write_warning(&format!("Could not re-assign pt stop '{}' after replacing edge '{}'.", stop.get_id(), edgeID));
            } else {
                let newEdgeId = stop.get_edge_id().to_string();
                self.myPTStopLookup.entry(newEdgeId).or_default().push(stop.clone());
            }
        }
        self.myPTStopLookup.remove(edgeID);
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
    pub fn find_stop(&self, origEdgeID: &str, pos: Position, threshold: f64) -> Option<Rc<NBPTStop>> {
                for item in self.myPTStops.values() {
            if item.get_orig_edge_id() == origEdgeID && item.get_position().distance_to2d(&pos) < threshold {
                return Some(item.clone());
            }
        }
        None
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
    pub fn get_reverse_stop(&mut self, pStop: Rc<NBPTStop>, ec: &NBEdgeCont) -> Option<Rc<NBPTStop>> {
                let edgeId = pStop.get_edge_id().to_string();
        let edge = ec.get_by_id(&edgeId);
        let reverse = Self::get_reverse_edge(edge);
        if !reverse.is_null() {
            let reverseID = Self::get_reverse_id(&pStop.get_id());
            if !self.myPTStops.contains_key(&reverseID) {
                let result = Rc::new(NBPTStop::new(pStop.get_element(), reverseID.clone(), pStop.get_position().clone(), reverse.get_id().to_string(), reverse.get_id().to_string(),
                                                   pStop.get_length(), pStop.get_name(), pStop.get_permissions(),
                                                   0.0, RGBColor::new_valid(false), -1.0));
                for line in pStop.get_lines() {
                    stop_m(&result).add_line(line);
                }
                return Some(result);
            } else {
                return self.myPTStops.get(&reverseID).cloned();
            }
        }
        None
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
                let edgeId = pStop.get_edge_id().to_string();
        let edge = cont.get_by_id(&edgeId);
        let reverse = Self::get_reverse_edge(edge);
        let closestPlatform = self.get_closest_platform_to_pt_stop_position(pStop.clone());
        stop_m(&pStop).set_pt_stop_length(closestPlatform.get_length());
        if !reverse.is_null() {
            // TODO in C++: make isLeft in PositionVector static
            let crossProd = self.compute_cross_product_edge_position(edge, closestPlatform.get_pos());
            // TODO in C++: consider driving on the left
            if crossProd > 0.0 {
                // pt stop is on the left of the orig edge
                stop_m(&pStop).set_edge_id(reverse.get_id().to_string(), cont);
            }
        }
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
                let stopPosition = pStop.get_position().clone();
        let mut closest: Ptr<NBPTPlatform> = Ptr::null();
        let mut minSqrDist = f64::MAX;
        for platform in stop_m(&pStop).get_platform_cands().iter() {
            let sqrDist = stopPosition.distance_squared_to2d(platform.get_pos());
            if sqrDist < minSqrDist {
                minSqrDist = sqrDist;
                closest = Ptr::new(platform);
            }
        }
        closest
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
    pub fn assign_and_creat_new_pt_stop_as_needed(&mut self, pStop: Rc<NBPTStop>, cont: &mut NBEdgeCont) -> Option<Rc<NBPTStop>> {
                let edgeId = pStop.get_edge_id().to_string();
        let edge = cont.get_by_id(&edgeId);
        if edge.is_null() {
            return None;
        }
        let mut rightOfEdge = false;
        let mut leftOfEdge = false;
        let mut left: Ptr<NBPTPlatform> = Ptr::null();
        let platforms: Vec<Ptr<NBPTPlatform>> = stop_m(&pStop).get_platform_cands().iter().map(|p| Ptr::new(p)).collect();
        for platform in platforms {
            let crossProd = self.compute_cross_product_edge_position(edge, platform.get_pos());
            // TODO in C++: consider driving on the left
            if crossProd > 0.0 {
                leftOfEdge = true;
                left = platform;
            } else {
                rightOfEdge = true;
                stop_m(&pStop).set_pt_stop_length(platform.get_length());
            }
        }

        if leftOfEdge && rightOfEdge {
            let leftStop = self.get_reverse_stop(pStop.clone(), cont);
            if let Some(ls) = &leftStop {
                stop_m(ls).set_pt_stop_length(left.get_length());
            }
            return leftStop;
        } else if leftOfEdge {
            let reverse = Self::get_reverse_edge(edge);
            if !reverse.is_null() {
                stop_m(&pStop).set_edge_id(reverse.get_id().to_string(), cont);
                stop_m(&pStop).set_pt_stop_length(left.get_length());
            }
        }
        None
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
                let geom: PositionVector = edge.get_geometry().clone();
        let idxTmp = geom.index_of_closest(closestPlatform, false);
        let offset = geom.nearest_offset_to_point2d(closestPlatform, true);
        let offset2 = geom.offset_at_index2d(idxTmp);
        let idx1: i32;
        let idx2: i32;
        if offset2 < offset {
            idx1 = idxTmp;
            idx2 = idx1 + 1;
        } else {
            idx2 = idxTmp;
            idx1 = idxTmp - 1;
        }
        let size = geom.len() as i32;
        if idx1 < 0 || idx1 >= size || idx2 < 0 || idx2 >= size {
            write_warning(&format!("Could not determine cross product for edge '{}'.", edge.get_id()));
            return 0.0;
        }
        let p1 = geom[idx1 as usize].clone();
        let p2 = geom[idx2 as usize].clone();

        let x0 = p1.x();
        let y0 = p1.y();
        let x1 = p2.x();
        let y1 = p2.y();
        let x2 = closestPlatform.x();
        let y2 = closestPlatform.y();
        let crossProd = (x1 - x0) * (y2 - y0) - (y1 - y0) * (x2 - x0);
        crossProd
    }

    /// C++: `std::string NBPTStopCont::getReverseID(const std::string &)` [static, private]
    // std::string
    // NBPTStopCont::getReverseID(const std::string& id) {
    //     return id.size() > 0 && id[0] == '-' ? id.substr(1) : "-" + id;
    // }
    #[allow(unused_variables)]
    pub fn get_reverse_id(id: &str) -> String {
                if id.starts_with('-') {
            id[1..].to_string()
        } else {
            format!("-{}", id)
        }
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
                if !edge.is_null() {
            let rGeom = edge.get_geometry().reverse();
            for it in edge.get_to_node().get_outgoing_edges() {
                if it.get_to_node() == edge.get_from_node() && it.get_geometry() == &rGeom {
                    return *it;
                }
            }
        }
        Ptr::null()
    }

    /// C++: `void NBPTStopCont::addIgnored(const std::string &)` [static]
    // static void addIgnored(const std::string& stopID) {
    //         myIgnoredStops.insert(stopID);
    //     }
    #[allow(unused_variables)]
    pub fn add_ignored(stopID: &str) {
                MY_IGNORED_STOPS.with(|set| {
            set.borrow_mut().insert(stopID.to_string());
        });
    }

    /// C++: `bool NBPTStopCont::wasIgnored(const std::string &)` [static]
    // static bool wasIgnored(const std::string& stopID) {
    //         return myIgnoredStops.count(stopID) > 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_ignored(stopID: &str) -> bool {
                MY_IGNORED_STOPS.with(|set| set.borrow().contains(stopID))
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
                let stops: BTreeMap<String, Rc<NBPTStop>> = self.myPTStops.clone();
        for (_, s) in stops {
            let stopId = s.get_id();
            if s.get_edge_id().is_empty() || s.was_loaded() {
                continue;
            }
            let edgeSign = s.get_edge_id().as_bytes()[0];
            let stopSign = stopId.as_bytes()[0];
            if edgeSign != stopSign && (edgeSign == b'-' || stopSign == b'-') {
                let reverseID = Self::get_reverse_id(&stopId);
                let rs = self.get(reverseID.clone());
                if let Some(r) = &rs {
                    if r.was_loaded() {
                        continue;
                    }
                }
                stop_m(&s).set_pt_stop_id(reverseID.clone());
                self.myPTStops.remove(&stopId);
                self.myPTStops.insert(reverseID.clone(), s.clone());
                if let Some(r) = rs {
                    stop_m(&r).set_pt_stop_id(stopId.clone());
                    self.myPTStops.insert(stopId.clone(), r);
                }
            }
        }
    }

}

