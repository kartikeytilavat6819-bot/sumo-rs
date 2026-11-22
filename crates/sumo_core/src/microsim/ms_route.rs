//! Port of microsim/MSRoute.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `ConstMSEdgeVector::const_iterator MSRouteIterator`
/// C++ ConstMSEdgeVector::const_iterator: an index into the route's edge vector (end() == len()).
pub type MSRouteIterator = usize;

/// C++ typedef `std::map<std::string, ConstMSRoutePtr> MSRoute::RouteDict`
pub type MSRoute_RouteDict = BTreeMap<String, ConstMSRoutePtr>;

/// C++ typedef `std::map<std::string, std::pair<RandomDistributor<ConstMSRoutePtr> *, bool>> MSRoute::RouteDistDict`
pub type MSRoute_RouteDistDict = BTreeMap<String, (Ptr<RandomDistributor<ConstMSRoutePtr>>, bool)>;

/// C++ class `MSRoute` (microsim/MSRoute.h)
#[repr(C)]
pub struct MSRoute {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// The list of edges to pass
    pub myEdges: ConstMSEdgeVector,
    /// whether the route may be deleted after the last vehicle abandoned it
    pub myAmPermanent: bool,
    /// The color
    pub myColor: Ptr<RGBColor>,
    /// The period when repeating this route
    pub myPeriod: SUMOTime,
    /// The assigned or calculated costs
    pub myCosts: f64,
    /// The estimated savings when rerouting
    pub mySavings: std::cell::Cell<f64>,
    /// Whether this route is incomplete and requires rerouting
    pub myReroute: bool,
    /// List of the stops on the parsed route
    pub myStops: StopParVector,
    /// The time where this route was replaced with an alternative route (or -1)
    pub myReplacedTime: SUMOTime,
    /// The index where this route was replaced with an alternative route
    pub myReplacedIndex: i32,
}

impl std::ops::Deref for MSRoute {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSRoute {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MSRoute (port as module-level statics, see AGENTS.md):
//   static RouteDict MSRoute::myDict;
//   static RouteDistDict MSRoute::myDistDict;
thread_local! {
    static MY_DICT: std::cell::RefCell<MSRoute_RouteDict> = std::cell::RefCell::new(BTreeMap::new());
    static MY_DIST_DICT: std::cell::RefCell<MSRoute_RouteDistDict> = std::cell::RefCell::new(BTreeMap::new());
    static NULL_ROUTE: std::cell::RefCell<Option<ConstMSRoutePtr>> = const { std::cell::RefCell::new(None) };
}

/// C++ `delete p` for a pointer created with Box::into_raw; deleting nullptr is a no-op
fn delete_ptr<T>(p: Ptr<T>) {
    if !p.is_null() {
        unsafe { drop(Box::from_raw(p.as_raw())) };
    }
}

/// C++ `route != nullptr` on a `ConstMSRoutePtr` returned by `MSRoute::dictionary`
pub fn is_null_route(route: &ConstMSRoutePtr) -> bool {
    NULL_ROUTE.with(|nr| nr.borrow().as_ref().is_some_and(|n| Rc::ptr_eq(n, route)))
}

/// Stand-in for the nullptr MSRoute::dictionary returns when the id is unknown: `ConstMSRoutePtr`
/// is a plain `Rc<MSRoute>` and cannot be null, so a leaked empty route is handed out instead.
// shortcut: C++ nullptr return value cannot be represented through Rc<MSRoute>, callers that must
// distinguish should check MSRoute::has_route first, upgrade when ConstMSRoutePtr becomes Option
fn get_null_route() -> ConstMSRoutePtr {
    NULL_ROUTE.with(|nr| {
        let mut nr = nr.borrow_mut();
        if nr.is_none() {
            *nr = Some(Rc::new(MSRoute::new("", &Vec::new(), true, Ptr::null(), &Vec::new(), -1, -1)));
        }
        nr.clone().unwrap()
    })
}

impl MSRoute {
    /// C++: `void MSRoute::MSRoute(const std::string &, const ConstMSEdgeVector &, const bool, const RGBColor *const, const StopParVector &, SUMOTime, int)`
    /// Constructor
    // MSRoute::MSRoute(const std::string& id,
    //                  const ConstMSEdgeVector& edges,
    //                  const bool isPermanent, const RGBColor* const c,
    //                  const StopParVector& stops,
    //                  SUMOTime replacedTime,
    //                  int replacedIndex) :
    //     Named(id), myEdges(edges), myAmPermanent(isPermanent),
    //     myColor(c),
    //     myPeriod(0),
    //     myCosts(-1),
    //     mySavings(0),
    //     myReroute(false),
    //     myStops(stops),
    //     myReplacedTime(replacedTime),
    //     myReplacedIndex(replacedIndex)
    // {}
    #[allow(unused_variables)]
    pub fn new(id: &str, edges: &ConstMSEdgeVector, isPermanent: bool, c: Ptr<RGBColor>, stops: &StopParVector, replacedTime: SUMOTime, replacedIndex: i32) -> Self {
        MSRoute {
            base: Named::new(id),
            base2: Parameterised::new(),
            myEdges: edges.clone(),
            myAmPermanent: isPermanent,
            myColor: c,
            myPeriod: 0,
            myCosts: -1.0,
            mySavings: std::cell::Cell::new(0.0),
            myReroute: false,
            myStops: stops.clone(),
            myReplacedTime: replacedTime,
            myReplacedIndex: replacedIndex,
        }
    }

    /// C++: `void MSRoute::~MSRoute() noexcept` [virtual]
    /// Destructor
    // MSRoute::~MSRoute() {
    //     delete myColor;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // delete myColor (null-safe, like delete nullptr)
        if !self.myColor.is_null() {
            drop(unsafe { Box::from_raw(self.myColor.as_raw()) });
        }
    }

    /// C++: `MSRouteIterator MSRoute::begin() const` [const]
    /// Returns the begin of the list of edges to pass
    // MSRouteIterator
    // MSRoute::begin() const {
    //     return myEdges.begin();
    // }
    #[allow(unused_variables)]
    pub fn begin(&self) -> MSRouteIterator {
        0
    }

    /// C++: `MSRouteIterator MSRoute::end() const` [const]
    /// Returns the end of the list of edges to pass
    // MSRouteIterator
    // MSRoute::end() const {
    //     return myEdges.end();
    // }
    #[allow(unused_variables)]
    pub fn end(&self) -> MSRouteIterator {
        self.myEdges.len()
    }

    /// C++: `int MSRoute::size() const` [const]
    /// Returns the number of edges to pass
    // int
    // MSRoute::size() const {
    //     return (int)myEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myEdges.len() as i32
    }

    /// C++: `const MSEdge * MSRoute::getLastEdge() const` [const]
    /// returns the destination edge
    // const MSEdge*
    // MSRoute::getLastEdge() const {
    //     assert(myEdges.size() > 0);
    //     return myEdges.back();
    // }
    #[allow(unused_variables)]
    pub fn get_last_edge(&self) -> Ptr<MSEdge> {
        debug_assert!(!self.myEdges.is_empty());
        *self.myEdges.last().unwrap()
    }

    /// C++: `const MSEdge * MSRoute::getFirstEdge() const` [const]
    /// returns the origin edge
    // const MSEdge*
    // MSRoute::getFirstEdge() const {
    //     assert(myEdges.size() > 0);
    //     return myEdges.front();
    // }
    #[allow(unused_variables)]
    pub fn get_first_edge(&self) -> Ptr<MSEdge> {
        debug_assert!(!self.myEdges.is_empty());
        *self.myEdges.first().unwrap()
    }

    /// C++: `void MSRoute::checkRemoval(bool) const` [const]
    /// removes the route from the internal dict if it is not marked as permanent
    // void
    // MSRoute::checkRemoval(bool force) const {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     if (!myAmPermanent || force) {
    //         myDict.erase(getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_removal(&self, force: bool) {
        // the dict owns its routes; erasing may drop the last reference, as in C++
        if !self.myAmPermanent || force {
            MY_DICT.with(|d| d.borrow_mut().remove(self.get_id()));
        }
    }

    /// C++: `int MSRoute::writeEdgeIDs(OutputDevice &, int, int, bool, SUMOVehicleClass) const` [const]
    /// Output the edge ids up to but not including the id of the given edge
    // int
    // MSRoute::writeEdgeIDs(OutputDevice& os, int firstIndex, int lastIndex, bool withInternal, SUMOVehicleClass svc) const {
    //     //std::cout << SIMTIME << " writeEdgeIDs " << getID() << " first=" << firstIndex << " lastIndex=" << lastIndex << " edges=" << toString(myEdges) << "\n";
    //     if (lastIndex < 0) {
    //         lastIndex = (int)myEdges.size();
    //     }
    //     int internal = 0;
    //     // collect the ids first, writing them to the stream one by one is slow
    //     std::string ids;
    //     for (int i = firstIndex; i < lastIndex; i++) {
    //         ids += myEdges[i]->getID();
    //         ids += ' ';
    //         if (withInternal && i + 1 < lastIndex) {
    //             const MSEdge* next = myEdges[i + 1];
    //             const MSEdge* edge = myEdges[i]->getInternalFollowingEdge(next, svc);
    //             // Take into account non-internal lengths until next non-internal edge
    //             while (edge != nullptr && edge->isInternal()) {
    //                 ids += edge->getID();
    //                 ids += ' ';
    //                 internal++;
    //                 edge = edge->getInternalFollowingEdge(next, svc);
    //             }
    //         }
    //     }
    //     os << ids;
    //     return internal + lastIndex - firstIndex;
    // }
    #[allow(unused_variables)]
    pub fn write_edge_i_ds(&self, os: &mut OutputDevice, firstIndex: i32, lastIndex: i32, withInternal: bool, svc: SUMOVehicleClass) -> i32 {
        let mut lastIndex = lastIndex;
        if lastIndex < 0 {
            lastIndex = self.myEdges.len() as i32;
        }
        let mut internal = 0;
        // collect the ids first, writing them to the stream one by one is slow
        let mut ids = String::new();
        let mut i = firstIndex;
        while i < lastIndex {
            ids.push_str(self.myEdges[i as usize].get_id());
            ids.push(' ');
            if withInternal && i + 1 < lastIndex {
                let next = self.myEdges[(i + 1) as usize];
                let mut edge = self.myEdges[i as usize].get_internal_following_edge(next, svc);
                // Take into account non-internal lengths until next non-internal edge
                while !edge.is_null() && edge.is_internal() {
                    ids.push_str(edge.get_id());
                    ids.push(' ');
                    internal += 1;
                    edge = edge.get_internal_following_edge(next, svc);
                }
            }
            i += 1;
        }
        os.write(&ids);
        internal + lastIndex - firstIndex
    }

    /// C++: `bool MSRoute::contains(const MSEdge *const) const` [const]
    // bool contains(const MSEdge* const edge) const {
    //         return std::find(myEdges.begin(), myEdges.end(), edge) != myEdges.end();
    //     }
    #[allow(unused_variables)]
    pub fn contains(&self, edge: Ptr<MSEdge>) -> bool {
        self.myEdges.contains(&edge)
    }

    /// C++: `bool MSRoute::containsAnyOf(const MSEdgeVector &) const` [const]
    // bool
    // MSRoute::containsAnyOf(const MSEdgeVector& edgelist) const {
    //     MSEdgeVector::const_iterator i = edgelist.begin();
    //     for (; i != edgelist.end(); ++i) {
    //         if (contains(*i)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn contains_any_of(&self, edgelist: &MSEdgeVector) -> bool {
        edgelist.iter().any(|e| self.contains(*e))
    }

    /// C++: `const MSEdge * MSRoute::operator[](int) const` [const]
    // const MSEdge*
    // MSRoute::operator[](int index) const {
    //     return myEdges[index];
    // }
    #[allow(unused_variables)]
    pub fn index_op(&self, index: i32) -> Ptr<MSEdge> {
        self.myEdges[index as usize]
    }

    /// C++: `void MSRoute::dict_saveState(OutputDevice &)` [static]
    /// Saves all known routes into the given stream
    // void
    // MSRoute::dict_saveState(OutputDevice& out) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     for (RouteDict::iterator it = myDict.begin(); it != myDict.end(); ++it) {
    //         ConstMSRoutePtr r = (*it).second;
    //         out.openTag(SUMO_TAG_ROUTE);
    //         out.writeAttr(SUMO_ATTR_ID, r->getID());
    //         out.writeAttr(SUMO_ATTR_STATE, r->myAmPermanent);
    //         out.writeAttr(SUMO_ATTR_EDGES, r->myEdges);
    //         if (r->myColor != nullptr) {
    //             out.writeAttr(SUMO_ATTR_COLOR, *r->myColor);
    //         }
    //         for (auto stop : r->getStops()) {
    //             stop.write(out);
    //         }
    //         out.closeTag();
    //     }
    //     for (const auto& item : myDistDict) {
    //         if (item.second.first->getVals().size() > 0) {
    //             out.openTag(SUMO_TAG_ROUTE_DISTRIBUTION).writeAttr(SUMO_ATTR_ID, item.first);
    //             out.writeAttr(SUMO_ATTR_STATE, item.second.second);
    //             std::ostringstream oss;
    //             bool space = false;
    //             for (const auto& route : item.second.first->getVals()) {
    //                 if (space) {
    //                     oss << " ";
    //                 }
    //                 oss << route->getID();
    //                 space = true;
    //             }
    //             out.writeAttr(SUMO_ATTR_ROUTES, oss.str());
    //             out.writeAttr(SUMO_ATTR_PROBS, item.second.first->getProbs());
    //             out.closeTag();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_save_state(out: &mut OutputDevice) {
        MY_DICT.with(|d| {
            let d = d.borrow();
            for (_id, r) in d.iter() {
                out.open_tag_id(SUMO_TAG_ROUTE);
                out.write_attr(SUMO_ATTR_ID, r.get_id());
                out.write_attr(SUMO_ATTR_STATE, &r.myAmPermanent);
                let edges: Vec<&str> = r.myEdges.iter().map(|e| e.get_id()).collect();
                out.write_attr(SUMO_ATTR_EDGES, &edges.join(" "));
                if !r.myColor.is_null() {
                    out.write_attr(SUMO_ATTR_COLOR, &(*r.myColor).to_string());
                }
                for stop in r.get_stops().iter() {
                    stop.write(out, true, true);
                }
                out.close_tag("");
            }
        });
        MY_DIST_DICT.with(|d| {
            let d = d.borrow();
            for (id, item) in d.iter() {
                let (dist, permanent) = item;
                if !dist.get_vals().is_empty() {
                    out.open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION).write_attr(SUMO_ATTR_ID, id);
                    out.write_attr(SUMO_ATTR_STATE, permanent);
                    let routesStr = dist.get_vals().iter().map(|route| route.get_id()).collect::<Vec<_>>().join(" ");
                    out.write_attr(SUMO_ATTR_ROUTES, &routesStr);
                    out.write_attr(SUMO_ATTR_PROBS, &utils::common::to_string::to_string(dist.get_probs()));
                    out.close_tag("");
                }
            }
        });
    }

    /// C++: `void MSRoute::dict_clearState()` [static]
    /// Decrement all route references before quick-loading state
    // void
    // MSRoute::dict_clearState() {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     myDistDict.clear();
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn dict_clear_state() {
        MY_DIST_DICT.with(|d| d.borrow_mut().clear());
        MY_DICT.with(|d| d.borrow_mut().clear());
    }

    /// C++: `const ConstMSEdgeVector & MSRoute::getEdges() const` [const]
    /// @}
    // const ConstMSEdgeVector& getEdges() const {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> &ConstMSEdgeVector {
        &self.myEdges
    }

    /// C++: `double MSRoute::getDistanceBetween(double, double, const MSLane *, const MSLane *, int) const` [const]
    /// Compute the distance between 2 given edges on this route, optionally including the length of internal lanes. Note, that for routes which contain loops: - the first occurance of fromEdge after routePosition will be used - the first occurance of toEdge after the above occurance of fromEdge will be used If fromEdge or toEdge are not in the route or the toPos on toEdge is not downstream of fromPos on fromEdge std::numeric_limits<double>::max() will be returned.
    // double
    // MSRoute::getDistanceBetween(double fromPos, double toPos,
    //                             const MSLane* fromLane, const MSLane* toLane, int routePosition) const {
    //     // std::cout << SIMTIME << " getDistanceBetween from=" << fromEdge->getID() << " to=" << toEdge->getID() << " fromPos=" << fromPos << " toPos=" << toPos << "\n";
    //     assert(fromPos >= 0. && fromPos <= fromLane->getLength());
    //     assert(toPos >= 0. && toPos <= toLane->getLength());
    //     assert(routePosition >= 0 && routePosition < (int)myEdges.size());
    //     assert(routePosition == 0 || !myEdges.front()->isInternal());
    //     const MSEdge* fromEdge = &fromLane->getEdge();
    //     const MSEdge* toEdge = &toLane->getEdge();
    //     if (fromEdge == toEdge && fromPos <= toPos) {
    //         return toPos - fromPos;
    //     }
    //     // TODO If fromEdge and toEdge are identical or both are internal and directly connected,
    //     // the code does not check whether they are in any relation to the route.
    //     if (fromEdge->isInternal()) {
    //         double minDist = std::numeric_limits<double>::max();
    //         for (const auto& via : fromEdge->getViaSuccessors()) {
    //             const MSEdge* const succ = via.second == nullptr ? via.first : via.second;
    //             assert(succ != nullptr);
    //             // std::cout << "  recurse fromSucc=" << succ->getID() << "\n";
    //             const double d = getDistanceBetween(0., toPos, succ->getLanes()[0], toLane, routePosition);
    //             if (d != std::numeric_limits<double>::max() && fromLane->getLength() - fromPos + d  < minDist) {
    //                 minDist = fromLane->getLength() - fromPos + d;
    //             }
    //         }
    //         return minDist;
    //     }
    //     if (toEdge->isInternal()) {
    //         const MSEdge* const pred = toEdge->getPredecessors().front();
    //         assert(pred != nullptr);
    //         // std::cout << "  recurse toPred=" << pred->getID() << "\n";
    //         const double d = getDistanceBetween(fromPos, pred->getLength(), fromLane, pred->getLanes()[0], routePosition);
    //         return d == std::numeric_limits<double>::max() ? d : toPos + d;
    //     }
    //     ConstMSEdgeVector::const_iterator fromIt = std::find(myEdges.begin() + routePosition, myEdges.end(), fromEdge);
    //     if (fromIt == myEdges.end()) {
    //         // start not contained in route
    //         return std::numeric_limits<double>::max();
    //     }
    //     ConstMSEdgeVector::const_iterator toIt = std::find(fromIt + 1, myEdges.end(), toEdge);
    //     if (toIt == myEdges.end()) {
    //         // destination not contained in route
    //         return std::numeric_limits<double>::max();
    //     }
    //     return getDistanceBetween(fromPos, toPos, fromIt, toIt, true);
    // }
    #[allow(unused_variables)]
    pub fn get_distance_between(&self, fromPos: f64, toPos: f64, fromLane: Ptr<MSLane>, toLane: Ptr<MSLane>, routePosition: i32) -> f64 {
        debug_assert!(fromPos >= 0. && fromPos <= fromLane.get_length());
        debug_assert!(toPos >= 0. && toPos <= toLane.get_length());
        debug_assert!(routePosition >= 0 && routePosition < self.myEdges.len() as i32);
        debug_assert!(routePosition == 0 || !self.myEdges.first().unwrap().is_internal());
        let fromEdge = Ptr::new(fromLane.get_edge());
        let toEdge = Ptr::new(toLane.get_edge());
        if fromEdge == toEdge && fromPos <= toPos {
            return toPos - fromPos;
        }
        // TODO If fromEdge and toEdge are identical or both are internal and directly connected,
        // the code does not check whether they are in any relation to the route.
        if fromEdge.is_internal() {
            let mut minDist = f64::MAX;
            for via in fromEdge.get_via_successors(SVC_IGNORING, false) {
                let succ = if via.1.is_null() { via.0 } else { via.1 };
                debug_assert!(!succ.is_null());
                // std::cout << "  recurse fromSucc=" << succ->getID() << "\n";
                let d = self.get_distance_between(0., toPos, succ.get_lanes()[0], toLane, routePosition);
                if d != f64::MAX && fromLane.get_length() - fromPos + d < minDist {
                    minDist = fromLane.get_length() - fromPos + d;
                }
            }
            return minDist;
        }
        if toEdge.is_internal() {
            let pred = toEdge.get_predecessors()[0];
            debug_assert!(!pred.is_null());
            // std::cout << "  recurse toPred=" << pred->getID() << "\n";
            let d = self.get_distance_between(fromPos, pred.get_length(), fromLane, pred.get_lanes()[0], routePosition);
            return if d == f64::MAX { d } else { toPos + d };
        }
        let fromIt = self.myEdges.iter().skip(routePosition as usize).position(|e| *e == fromEdge);
        let Some(fromIt) = fromIt else {
            // start not contained in route
            return f64::MAX;
        };
        let fromIt = fromIt + routePosition as usize;
        let toIt = self.myEdges.iter().skip(fromIt + 1).position(|e| *e == toEdge);
        let Some(toIt) = toIt else {
            // destination not contained in route
            return f64::MAX;
        };
        let toIt = toIt + fromIt + 1;
        self.get_distance_between_2(fromPos, toPos, &fromIt, &toIt, true)
    }

    /// C++: `double MSRoute::getDistanceBetween(double, double, const MSRouteIterator &, const MSRouteIterator &, bool) const` [const]
    /// Compute the distance between 2 given edges on this route, optionally including the length of internal lanes. This has the same semantics as above but uses iterators instead of edge points so looping routes are not an issue.
    // double
    // MSRoute::getDistanceBetween(double fromPos, double toPos,
    //                             const MSRouteIterator& fromEdge, const MSRouteIterator& toEdge, bool includeInternal) const {
    //     bool isFirstIteration = true;
    //     double distance = -fromPos;
    //     MSRouteIterator it = fromEdge;
    //     if (fromEdge == toEdge) {
    //         // destination position is on start edge
    //         if (fromPos <= toPos) {
    //             return toPos - fromPos;
    //         } else {
    //             // we cannot go backwards. Something is wrong here
    //             return std::numeric_limits<double>::max();
    //         }
    //     } else if (fromEdge > toEdge) {
    //         // we don't visit the edge again
    //         return std::numeric_limits<double>::max();
    //     }
    //     for (; it != end(); ++it) {
    //         if (it == toEdge && !isFirstIteration) {
    //             distance += toPos;
    //             break;
    //         } else {
    //             distance += (*it)->getLength();
    //             if (includeInternal && (it + 1) != end()) {
    //                 // XXX the length may be wrong if there are parallel internal edges for different vClasses
    //                 distance += (*it)->getInternalFollowingLengthTo(*(it + 1), SVC_IGNORING);
    //             }
    //         }
    //         isFirstIteration = false;
    //     }
    //     return distance;
    // }
    #[allow(unused_variables)]
    pub fn get_distance_between_2(&self, fromPos: f64, toPos: f64, fromEdge: &MSRouteIterator, toEdge: &MSRouteIterator, includeInternal: bool) -> f64 {
        let mut isFirstIteration = true;
        let mut distance = -fromPos;
        let mut it = *fromEdge;
        if fromEdge == toEdge {
            // destination position is on start edge
            if fromPos <= toPos {
                return toPos - fromPos;
            } else {
                // we cannot go backwards. Something is wrong here
                return f64::MAX;
            }
        } else if *fromEdge > *toEdge {
            // we don't visit the edge again
            return f64::MAX;
        }
        while it != self.end() {
            if it == *toEdge && !isFirstIteration {
                distance += toPos;
                break;
            } else {
                distance += self.myEdges[it].get_length();
                if includeInternal && it + 1 != self.end() {
                    // XXX the length may be wrong if there are parallel internal edges for different vClasses
                    distance += self.myEdges[it].get_internal_following_length_to(self.myEdges[it + 1], SVC_IGNORING);
                }
            }
            isFirstIteration = false;
            it += 1;
        }
        distance
    }

    /// C++: `const RGBColor & MSRoute::getColor() const` [const]
    /// Returns the color
    // const RGBColor&
    // MSRoute::getColor() const {
    //     if (myColor == nullptr) {
    //         return RGBColor::DEFAULT_COLOR;
    //     }
    //     return *myColor;
    // }
    #[allow(unused_variables)]
    pub fn get_color(&self) -> &RGBColor {
        if self.myColor.is_null() {
            return &RGBColor::DEFAULT_COLOR;
        }
        self.myColor.get().unwrap()
    }

    /// C++: `SUMOTime MSRoute::getPeriod() const` [const]
    /// returns the period
    // SUMOTime getPeriod() const {
    //         return myPeriod;
    //     }
    #[allow(unused_variables)]
    pub fn get_period(&self) -> SUMOTime {
        self.myPeriod
    }

    /// C++: `double MSRoute::getCosts() const` [const]
    /// Returns the costs of the route
    // double getCosts() const {
    //         return myCosts;
    //     }
    #[allow(unused_variables)]
    pub fn get_costs(&self) -> f64 {
        self.myCosts
    }

    /// C++: `double MSRoute::getSavings() const` [const]
    /// Returns the estimated savings due to using this route (compare to the route before rerouting)
    // double getSavings() const {
    //         return mySavings;
    //     }
    #[allow(unused_variables)]
    pub fn get_savings(&self) -> f64 {
        self.mySavings.get()
    }

    /// C++: `SUMOTime MSRoute::getReplacedTime() const` [const]
    /// Returns the time at which this route was replaced (or -1)
    // SUMOTime getReplacedTime() const {
    //         return myReplacedTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_replaced_time(&self) -> SUMOTime {
        self.myReplacedTime
    }

    /// C++: `int MSRoute::getReplacedIndex() const` [const]
    /// Returns the index at which this route was replaced
    // int getReplacedIndex() const {
    //         return myReplacedIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_replaced_index(&self) -> i32 {
        self.myReplacedIndex
    }

    /// C++: `void MSRoute::setPeriod(SUMOTime)`
    /// sets the period
    // void setPeriod(SUMOTime period) {
    //         myPeriod = period;
    //     }
    #[allow(unused_variables)]
    pub fn set_period(&mut self, period: SUMOTime) {
        self.myPeriod = period;
    }

    /// C++: `void MSRoute::setCosts(double)`
    /// Sets the costs of the route
    // void setCosts(double costs) {
    //         myCosts = costs;
    //     }
    #[allow(unused_variables)]
    pub fn set_costs(&mut self, costs: f64) {
        self.myCosts = costs;
    }

    /// C++: `void MSRoute::setSavings(double)`
    /// Sets the savings of the route
    // void setSavings(double savings) {
    //         mySavings = savings;
    //     }
    #[allow(unused_variables)]
    pub fn set_savings(&self, savings: f64) {
        self.mySavings.set(savings);
    }

    /// C++: `bool MSRoute::mustReroute() const` [const]
    // bool mustReroute() const {
    //         return myReroute;
    //     }
    #[allow(unused_variables)]
    pub fn must_reroute(&self) -> bool {
        self.myReroute
    }

    /// C++: `void MSRoute::setReroute(bool)`
    // void setReroute(bool reroute = true) {
    //         myReroute = reroute;
    //     }
    #[allow(unused_variables)]
    pub fn set_reroute(&mut self, reroute: bool) {
        self.myReroute = reroute;
    }

    /// C++: `const StopParVector & MSRoute::getStops() const` [const]
    /// Returns the stops
    // const StopParVector&
    // MSRoute::getStops() const {
    //     return myStops;
    // }
    #[allow(unused_variables)]
    pub fn get_stops(&self) -> &StopParVector {
        &self.myStops
    }

    /// C++: `bool MSRoute::dictionary(const std::string &, ConstMSRoutePtr)` [static]
    /// Adds a route to the dictionary.
    // bool
    // MSRoute::dictionary(const std::string& id, ConstMSRoutePtr route) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     if (myDict.find(id) == myDict.end() && myDistDict.find(id) == myDistDict.end()) {
    //         myDict[id] = route;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: &str, route: ConstMSRoutePtr) -> bool {
        if !MY_DICT.with(|d| d.borrow().contains_key(id)) && !MY_DIST_DICT.with(|d| d.borrow().contains_key(id)) {
            MY_DICT.with(|d| d.borrow_mut().insert(id.to_string(), route));
            return true;
        }
        false
    }

    /// C++: `bool MSRoute::dictionary(const std::string &, RandomDistributor<ConstMSRoutePtr> *const, const bool)` [static]
    /// Adds a route distribution to the dictionary.
    // bool
    // MSRoute::dictionary(const std::string& id, RandomDistributor<ConstMSRoutePtr>* const routeDist, const bool permanent) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     if (myDict.find(id) == myDict.end() && myDistDict.find(id) == myDistDict.end()) {
    //         myDistDict[id] = std::make_pair(routeDist, permanent);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: &str, routeDist: Ptr<RandomDistributor<ConstMSRoutePtr>>, permanent: bool) -> bool {
        if !MY_DICT.with(|d| d.borrow().contains_key(id)) && !MY_DIST_DICT.with(|d| d.borrow().contains_key(id)) {
            MY_DIST_DICT.with(|d| d.borrow_mut().insert(id.to_string(), (routeDist, permanent)));
            return true;
        }
        false
    }

    /// C++: `ConstMSRoutePtr MSRoute::dictionary(const std::string &, SumoRNG *)` [static]
    /// Returns the named route or a sample from the named distribution.
    // ConstMSRoutePtr
    // MSRoute::dictionary(const std::string& id, SumoRNG* rng) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     RouteDict::iterator it = myDict.find(id);
    //     if (it == myDict.end()) {
    //         RouteDistDict::iterator it2 = myDistDict.find(id);
    //         if (it2 == myDistDict.end() || it2->second.first->getOverallProb() == 0) {
    //             return nullptr;
    //         }
    //         return it2->second.first->get(rng);
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: &str, rng: Ptr<SumoRNG>) -> ConstMSRoutePtr {
        let route = MY_DICT.with(|d| d.borrow().get(id).cloned());
        if let Some(r) = route {
            return r;
        }
        let result = MY_DIST_DICT.with(|d| {
            d.borrow().get(id).and_then(|(dist, _)| {
                if (*dist).get_overall_prob() == 0.0 {
                    None
                } else {
                    let which = if rng.is_null() { None } else { Some(unsafe { &mut *rng.as_raw() }) };
                    match (**dist).get(which) {
                        Ok(r) => Some(r),
                        // C++ rethrows the ProcessError raised by RandomDistributor::get
                        Err(e) => std::panic::panic_any(e),
                    }
                }
            })
        });
        match result {
            Some(r) => r,
            None => get_null_route(),
        }
    }

    /// C++: `bool MSRoute::hasRoute(const std::string &)` [static]
    /// returns whether a route with the given id exists
    // bool
    // MSRoute::hasRoute(const std::string& id) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     return myDict.find(id) != myDict.end();
    // }
    #[allow(unused_variables)]
    pub fn has_route(id: &str) -> bool {
        MY_DICT.with(|d| d.borrow().contains_key(id))
    }

    /// C++: `RandomDistributor<ConstMSRoutePtr> * MSRoute::distDictionary(const std::string &)` [static]
    /// Returns the named route distribution.
    // RandomDistributor<ConstMSRoutePtr>*
    // MSRoute::distDictionary(const std::string& id) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     RouteDistDict::iterator it2 = myDistDict.find(id);
    //     if (it2 == myDistDict.end()) {
    //         return nullptr;
    //     }
    //     return it2->second.first;
    // }
    #[allow(unused_variables)]
    pub fn dist_dictionary(id: &str) -> Ptr<RandomDistributor<ConstMSRoutePtr>> {
        let dist = MY_DIST_DICT.with(|d| d.borrow().get(id).map(|(p, _)| *p));
        match dist {
            Some(d) => d,
            None => Ptr::null(),
        }
    }

    /// C++: `void MSRoute::clear()` [static]
    /// Clears the dictionary (delete all known routes, too)
    // void
    // MSRoute::clear() {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     for (RouteDistDict::iterator i = myDistDict.begin(); i != myDistDict.end(); ++i) {
    //         delete i->second.first;
    //     }
    //     myDistDict.clear();
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear() {
        let dists: Vec<Ptr<RandomDistributor<ConstMSRoutePtr>>> = MY_DIST_DICT.with(|d| {
            let mut d = d.borrow_mut();
            let v: Vec<Ptr<RandomDistributor<ConstMSRoutePtr>>> = d.values().map(|(p, _)| *p).collect();
            d.clear();
            v
        });
        for dist in dists {
            delete_ptr(dist);
        }
        MY_DICT.with(|d| d.borrow_mut().clear());
    }

    /// C++: `void MSRoute::checkDist(const std::string &)` [static]
    /// Checks the distribution whether it is permanent and deletes it if not
    // void
    // MSRoute::checkDist(const std::string& id) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     RouteDistDict::iterator it = myDistDict.find(id);
    //     if (it != myDistDict.end() && !it->second.second) {
    //         for (ConstMSRoutePtr rp : it->second.first->getVals()) {
    //             const MSRoute& r = *rp;
    //             r.checkRemoval();
    //         }
    //         delete it->second.first;
    //         myDistDict.erase(it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_dist(id: &str) {
        let found = MY_DIST_DICT.with(|d| {
            let d = d.borrow();
            match d.get(id) {
                Some((dist, permanent)) if !*permanent => {
                    let rps: Vec<ConstMSRoutePtr> = dist.get_vals().to_vec();
                    // `it` stays alive until the end of the block, so erase after the loop below
                    Some((*dist, rps))
                }
                _ => None,
            }
        });
        if let Some((dist, rps)) = found {
            for rp in rps {
                let r: &MSRoute = &rp;
                r.check_removal(false);
            }
            delete_ptr(dist);
            MY_DIST_DICT.with(|d| d.borrow_mut().remove(id));
        }
    }

    /// C++: `void MSRoute::insertIDs(std::vector<std::string> &)` [static]
    // void
    // MSRoute::insertIDs(std::vector<std::string>& into) {
    // #ifdef HAVE_FOX
    //     FXMutexLock f(myDictMutex);
    // #endif
    //     into.reserve(myDict.size() + myDistDict.size() + into.size());
    //     for (RouteDict::const_iterator i = myDict.begin(); i != myDict.end(); ++i) {
    //         into.push_back((*i).first);
    //     }
    //     for (RouteDistDict::const_iterator i = myDistDict.begin(); i != myDistDict.end(); ++i) {
    //         into.push_back((*i).first);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insert_i_ds(into: &mut Vec<String>) {
        into.reserve(MY_DICT.with(|d| d.borrow().len()) + MY_DIST_DICT.with(|d| d.borrow().len()) + into.len());
        MY_DICT.with(|d| {
            for i in d.borrow().keys() {
                into.push(i.clone());
            }
        });
        MY_DIST_DICT.with(|d| {
            for i in d.borrow().keys() {
                into.push(i.clone());
            }
        });
    }

    /// C++: `MSRoute & MSRoute::operator=(const MSRoute &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSRoute) -> &mut MSRoute {
        // C++ declares the assignment operator private and never defines it: unreachable in C++
        panic!("MSRoute assignment operator is invalidated")
    }

}


/// C++ compares `const MSRoute*` by pointer in RandomDistributor, so equality is identity
impl PartialEq for MSRoute {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edge(id: &str) -> Ptr<MSEdge> {
        Ptr::from_raw(Box::into_raw(Box::new(MSEdge::new(id, 0, SumoXMLEdgeFunc::NORMAL, "", "", "", 0, 0.0))))
    }

    #[test]
    fn dictionary_roundtrip() {
        let r: ConstMSRoutePtr = Rc::new(MSRoute::new("t_dict_r", &vec![edge("x")], true, Ptr::null(), &Vec::new(), -1, -1));
        assert!(!MSRoute::has_route("t_dict_r"));
        assert!(MSRoute::dictionary("t_dict_r", r.clone()));
        assert!(!MSRoute::dictionary("t_dict_r", r.clone()));
        assert!(MSRoute::has_route("t_dict_r"));
        assert_eq!(MSRoute::dictionary_3("t_dict_r", Ptr::null()).size(), 1);
        // unknown id: C++ returns nullptr; the Rust stand-in is a leaked empty route
        assert_eq!(MSRoute::dictionary_3("t_dict_none", Ptr::null()).size(), 0);
        // C++ MSRoute::dictionary(id, nullptr) falls back to myDistDict; the
        // calibrator relies on this when its flow references a <routeDistribution>.
        let rd: ConstMSRoutePtr = Rc::new(MSRoute::new("t_dict_dist_r", &vec![edge("x")], true, Ptr::null(), &Vec::new(), -1, -1));
        let mut dist = RandomDistributor::<ConstMSRoutePtr>::default();
        assert!(dist.add(rd, 1.0, false));
        assert!(MSRoute::dictionary_2("t_dict_dist", Ptr::from_raw(Box::into_raw(Box::new(dist))), true));
        assert!(!MSRoute::has_route("t_dict_dist"));
        assert_eq!(MSRoute::dictionary_3("t_dict_dist", Ptr::null()).size(), 1);
        MSRoute::clear();
        assert!(!MSRoute::has_route("t_dict_r"));
        assert!(MSRoute::dist_dictionary("t_dict_r").is_null());
    }

    #[test]
    fn write_edge_ids_and_distance() {
        let r = Box::new(MSRoute::new("t_wrt", &vec![edge("a"), edge("b"), edge("c")], true, Ptr::null(), &Vec::new(), -1, -1));
        let mut os = OutputDevice::new_string(0);
        assert_eq!(r.write_edge_i_ds(&mut os, 0, -1, false, SVC_IGNORING), 3);
        assert_eq!(os.get_string(), "a b c ");
        // zero-length edges: distance from pos 5 on the first edge to pos 7 on the last = -5 + 7
        assert_eq!(r.get_distance_between_2(5.0, 7.0, &r.begin(), &2, false), 2.0);
        // destination position is on the start edge, backwards: cannot go backwards
        assert_eq!(r.get_distance_between_2(7.0, 5.0, &r.begin(), &0, false), f64::MAX);
        // we don't visit the edge again
        assert_eq!(r.get_distance_between_2(0.0, 1.0, &2, &0, false), f64::MAX);
        // first to last, including the destination position
        assert_eq!(r.get_distance_between_2(0.0, 1.0, &r.begin(), &2, false), 1.0);
    }
}
