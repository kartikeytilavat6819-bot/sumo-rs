//! Port of utils/router/CHBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterVehicle;
use crate::utils::router::router_edge::{RouterEdge, RouterEdgeInfo};

/// C++ typedef `std::pair<const CHConnection *, const CHConnection *> CHBuilder::CHConnectionPair`
pub type CHBuilder_CHConnectionPair<E> = (Ptr<CHBuilder_CHConnection<E>>, Ptr<CHBuilder_CHConnection<E>>);

/// C++ typedef `int CHBuilder::CHConnectionPairs`
pub type CHBuilder_CHConnectionPairs<E> = Vec<CHBuilder_CHConnectionPair<E>>;

/// C++ typedef `int CHBuilder::CHConnections`
pub type CHBuilder_CHConnections<E> = Vec<CHBuilder_CHConnection<E>>;

/// C++ typedef `std::pair<const E *, const E *> CHBuilder::ConstEdgePair`
pub type CHBuilder_ConstEdgePair<E> = (Ptr<E>, Ptr<E>);

/// C++ typedef `int CHBuilder::ShortcutVia`
pub type CHBuilder_ShortcutVia<E> = std::collections::BTreeMap<CHBuilder_ConstEdgePair<E>, Ptr<E>>;

#[allow(unused_variables)]
pub(crate) fn e_permissions<E: RouterEdgeInfo>(e: Ptr<E>) -> SVCPermissions {
    e.edge_permissions()
}

/// C++ `&obj` taken as a pointer (the object is owned by a container that outlives the pointer)
fn ptr_of<T>(r: &T) -> Ptr<T> {
    Ptr::from_raw(r as *const T as *mut T)
}

fn current_millis() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// libstdc++ `__push_heap` (max-heap by `less`), so that tie order matches std::push_heap
fn std_push_heap_hole<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], mut hole: isize, top: isize, value: T, less: &F) {
    while hole > top {
        let parent = (hole - 1) / 2;
        if !less(v[parent as usize], value) {
            break;
        }
        v[hole as usize] = v[parent as usize];
        hole = parent;
    }
    v[hole as usize] = value;
}

/// libstdc++ `__adjust_heap`
fn std_adjust_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], mut hole: isize, len: isize, value: T, less: &F) {
    let top = hole;
    let mut second = hole;
    while second < (len - 1) / 2 {
        second = 2 * (second + 1);
        if less(v[second as usize], v[(second - 1) as usize]) {
            second -= 1;
        }
        v[hole as usize] = v[second as usize];
        hole = second;
    }
    if (len & 1) == 0 && second == (len - 2) / 2 {
        second = 2 * (second + 1);
        v[hole as usize] = v[(second - 1) as usize];
        hole = second - 1;
    }
    std_push_heap_hole(v, hole, top, value, less);
}

/// std::push_heap over the whole slice (the last element is the newly added one)
pub(crate) fn std_push_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], less: &F) {
    let hole = v.len() - 1;
    let value = v[hole];
    std_push_heap_hole(v, hole as isize, 0, value, less);
}

/// std::pop_heap over the whole slice: the front moves to the back
pub(crate) fn std_pop_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], less: &F) {
    let last = v.len() - 1;
    let value = v[last];
    v[last] = v[0];
    std_adjust_heap(v, 0, last as isize, value, less);
}

/// std::make_heap over the whole slice
pub(crate) fn std_make_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], less: &F) {
    let len = v.len() as isize;
    if len < 2 {
        return;
    }
    let mut parent = (len - 2) / 2;
    loop {
        let value = v[parent as usize];
        std_adjust_heap(v, parent, len, value, less);
        if parent == 0 {
            return;
        }
        parent -= 1;
    }
}

/// Computes the shortest path through a contracted network
/// C++ class `CHBuilder` (utils/router/CHBuilder.h)
pub struct CHBuilder<E, V> {
    /// all edges with numerical ids
    pub myEdges: Ptr<Vec<Ptr<E>>>,
    /// the handler for routing errors
    pub myErrorMsgHandler: Ptr<MsgHandler>,
    /// static vector for lookup
    pub myCHInfos: Vec<CHBuilder_CHInfo<E>>,
    /// Comparator for contraction priority
    pub myCmp: CHBuilder_CHInfoComparator,
    /// the shortest path tree to use when searching for shortcuts
    pub mySPTree: Ptr<SPTree<CHBuilder_CHInfo<E>, CHBuilder_CHConnection<E>>>,
    /// the permissions for which the hierarchy was constructed
    pub mySVC: SUMOVehicleClass,
    /// counters for performance logging
    pub myUpdateCount: i32,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E: RouterEdge<V>, V: RouterVehicle> CHBuilder<E, V> {
    /// C++: `void CHBuilder::CHBuilder<E, V>(const int &, bool, const SUMOVehicleClass, bool)`
    /// Constructor
    // CHBuilder(const std::vector<E*>& edges, bool unbuildIsWarning,
    //               const SUMOVehicleClass svc,
    //               bool validatePermissions):
    //         myEdges(edges),
    //         myErrorMsgHandler(unbuildIsWarning ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //         mySPTree(new SPTree<CHInfo, CHConnection>(4, validatePermissions)),
    //         mySVC(svc),
    //         myUpdateCount(0) {
    //         for (const E* const e : edges) {
    //             myCHInfos.push_back(CHInfo(e));
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, svc: SUMOVehicleClass, validatePermissions: bool) -> Self {
        let myCHInfos = edges.iter().map(|e| CHBuilder_CHInfo::new(*e)).collect();
        Self {
            myEdges: Ptr::from_raw(edges as *const Vec<Ptr<E>> as *mut Vec<Ptr<E>>),
            myErrorMsgHandler: utils::common::msg_handler::with_instance(
                if unbuildIsWarning { utils::common::msg_handler::MsgType::Warning } else { utils::common::msg_handler::MsgType::Error },
                |h| Ptr::from_mut(h),
            ),
            myCHInfos,
            myCmp: CHBuilder_CHInfoComparator {},
            mySPTree: Ptr::from_raw(Box::into_raw(Box::new(SPTree::new(4, validatePermissions)))),
            mySVC: svc,
            myUpdateCount: 0,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void CHBuilder::~CHBuilder<E, V>()` [virtual]
    /// Destructor
    // virtual ~CHBuilder() {
    //         delete mySPTree;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        unsafe { drop(Box::from_raw(self.mySPTree.as_raw())) };
    }

    /// C++: `Hierarchy * CHBuilder::buildContractionHierarchy(SUMOTime, const V *const, const SUMOAbstractRouter<E, V> *)`
    // Hierarchy* buildContractionHierarchy(SUMOTime time, const V* const vehicle, const SUMOAbstractRouter<E, V>* effortProvider) {
    //         Hierarchy* result = new Hierarchy();
    //         const int numEdges = (int)myCHInfos.size();
    //         const std::string vClass = (mySPTree->validatePermissions() ?
    //                                     "all vehicle classes " : "vClass='" + SumoVehicleClassStrings.getString(mySVC) + "' ");
    //         PROGRESS_BEGIN_MESSAGE("Building Contraction Hierarchy for " + vClass
    //                                + "and time=" + time2string(time) + " (" + toString(numEdges) + " edges)\n");
    //         const long startMillis = SysUtils::getCurrentMillis();
    //         // init queue
    //         std::vector<CHInfo*> queue; // max heap: edge to be contracted is front
    //         // reset previous connections etc
    //         for (int i = 0; i < numEdges; i++) {
    //             myCHInfos[i].resetContractionState();
    //             result->forwardUplinks.push_back(std::vector<Connection>());
    //             result->backwardUplinks.push_back(std::vector<Connection>());
    //         }
    //         // copy connections from the original net
    //         const double time_seconds = STEPS2TIME(time); // timelines store seconds!
    //         for (int i = 0; i < numEdges; i++) {
    //             synchronize(myCHInfos[i], time_seconds, vehicle, effortProvider);
    //         }
    //         // synchronization is finished. now we can compute priorities for the first time
    //         for (int i = 0; i < numEdges; i++) {
    //             myCHInfos[i].updatePriority(mySPTree);
    //             queue.push_back(&(myCHInfos[i]));
    //         }
    //         std::make_heap(queue.begin(), queue.end(), myCmp);
    //         int contractionRank = 0;
    //         // contraction loop
    //         while (!queue.empty()) {
    //             while (tryUpdateFront(queue)) {}
    //             CHInfo* max = queue.front();
    //             max->rank = contractionRank;
    // #ifdef CHRouter_DEBUG_CONTRACTION
    //             std::cout << "contracting '" << max->edge->getID() << "' with prio: " << max->priority << " (rank " << contractionRank << ")\n";
    // #endif
    //             const E* const edge = max->edge;
    //             // add outgoing connections to the forward search
    //             const int edgeID = edge->getNumericalID();
    //             for (const CHConnection& con : max->followers) {
    //                 result->forwardUplinks[edgeID].push_back(Connection(con.target->edge->getNumericalID(), con.cost, con.permissions));
    //                 disconnect(con.target->approaching, max);
    //                 con.target->updatePriority(0);
    //             }
    //             // add incoming connections to the backward search
    //             for (const CHConnection& con : max->approaching) {
    //                 result->backwardUplinks[edgeID].push_back(Connection(con.target->edge->getNumericalID(), con.cost, con.permissions));
    //                 disconnect(con.target->followers, max);
    //                 con.target->updatePriority(0);
    //             }
    //             // add shortcuts to the net
    //             for (const Shortcut& s : max->shortcuts) {
    //                 const ConstEdgePair& edgePair = s.edgePair;
    //                 result->shortcuts[edgePair] = edge;
    //                 CHInfo* from = getCHInfo(edgePair.first);
    //                 CHInfo* to = getCHInfo(edgePair.second);
    //                 from->followers.push_back(CHConnection(to, s.cost, s.permissions, s.underlying));
    //                 to->approaching.push_back(CHConnection(from, s.cost, s.permissions, s.underlying));
    //             }
    //             // if you need to debug the chrouter with MSVC uncomment the following line, hierarchy building will get slower and the hierarchy may change though
    //             //std::make_heap(queue.begin(), queue.end(), myCmp);
    //             // remove from queue
    //             std::pop_heap(queue.begin(), queue.end(), myCmp);
    //             queue.pop_back();
    //             /*
    //             if (contractionRank % 10000 == 0) {
    //                 // update all and rebuild queue
    //                 for (typename std::vector<CHInfo*>::iterator it = queue.begin(); it != queue.end(); ++it) {
    //                     (*it)->updatePriority(mySPTree);
    //                 }
    //                 std::make_heap(queue.begin(), queue.end(), myCmp);
    //             }
    //             */
    //             contractionRank += 1;
    //         }
    //         // reporting
    //         const long duration = SysUtils::getCurrentMillis() - startMillis;
    //         WRITE_MESSAGE("Created " + toString(result->shortcuts.size()) + " shortcuts.");
    //         WRITE_MESSAGE("Recomputed priority " + toString(myUpdateCount) + " times.");
    //         MsgHandler::getMessageInstance()->endProcessMsg("done (" + toString(duration) + "ms).");
    //         PROGRESS_DONE_MESSAGE();
    //         myUpdateCount = 0;
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn build_contraction_hierarchy(&mut self, time: SUMOTime, vehicle: Ptr<V>, effortProvider: Ptr<SUMOAbstractRouter<E, V>>) -> Ptr<CHBuilder_Hierarchy<E>> {
        let result = Ptr::from_raw(Box::into_raw(Box::new(CHBuilder_Hierarchy {
            shortcuts: std::collections::BTreeMap::new(),
            forwardUplinks: Vec::new(),
            backwardUplinks: Vec::new(),
        })));
        let numEdges = self.myCHInfos.len() as i32;
        let vClass = if self.mySPTree.m().validate_permissions() {
            "all vehicle classes ".to_string()
        } else {
            format!("vClass='{}' ", utils::common::sumo_vehicle_class::SUMO_VEHICLE_CLASS_STRINGS.get_string(self.mySVC).unwrap())
        };
        utils::common::msg_handler::progress_begin_message(&format!(
            "Building Contraction Hierarchy for {}and time={} ({} edges)\n",
            vClass,
            utils::common::time::time2string(time),
            numEdges
        ));
        let startMillis = current_millis();
        // init queue
        let mut queue: Vec<Ptr<CHBuilder_CHInfo<E>>> = Vec::new(); // max heap: edge to be contracted is front
        // reset previous connections etc
        for i in 0..numEdges as usize {
            self.myCHInfos[i].reset_contraction_state();
            result.m().forwardUplinks.push(Vec::new());
            result.m().backwardUplinks.push(Vec::new());
        }
        // copy connections from the original net
        let time_seconds = utils::common::time::steps2time(time); // timelines store seconds!
        for i in 0..numEdges as usize {
            let info = Ptr::from_mut(&mut self.myCHInfos[i]);
            self.synchronize(info.m(), time_seconds, vehicle, effortProvider);
        }
        // synchronization is finished. now we can compute priorities for the first time
        for i in 0..numEdges as usize {
            self.myCHInfos[i].update_priority(self.mySPTree);
            queue.push(Ptr::from_mut(&mut self.myCHInfos[i]));
        }
        let cmp = CHBuilder_CHInfoComparator {};
        std_make_heap(&mut queue, &|a, b| cmp.call_op::<E, V>(a, b));
        let mut contractionRank = 0;
        // contraction loop
        while !queue.is_empty() {
            while self.try_update_front(&mut queue) {}
            let max = queue[0];
            max.m().rank = contractionRank;
            let edge = max.m().edge;
            // add outgoing connections to the forward search
            let edgeID = edge.numerical_id() as usize;
            for con in max.m().followers.iter() {
                result.m().forwardUplinks[edgeID].push(CHBuilder_Connection::new(con.target.m().edge.numerical_id(), con.cost, con.permissions));
                self.disconnect(&mut con.target.m().approaching, max);
                con.target.m().update_priority(Ptr::null());
            }
            // add incoming connections to the backward search
            for con in max.m().approaching.iter() {
                result.m().backwardUplinks[edgeID].push(CHBuilder_Connection::new(con.target.m().edge.numerical_id(), con.cost, con.permissions));
                self.disconnect(&mut con.target.m().followers, max);
                con.target.m().update_priority(Ptr::null());
            }
            // add shortcuts to the net
            for s in max.m().shortcuts.iter() {
                let edgePair = s.edgePair;
                result.m().shortcuts.insert(edgePair, edge);
                let from = self.get_ch_info(edgePair.0);
                let to = self.get_ch_info(edgePair.1);
                from.m().followers.push(CHBuilder_CHConnection::new(to, s.cost, s.permissions, s.underlying));
                to.m().approaching.push(CHBuilder_CHConnection::new(from, s.cost, s.permissions, s.underlying));
            }
            // if you need to debug the chrouter with MSVC uncomment the following line, hierarchy building will get slower and the hierarchy may change though
            //std::make_heap(queue.begin(), queue.end(), myCmp);
            // remove from queue
            std_pop_heap(&mut queue, &|a, b| cmp.call_op::<E, V>(a, b));
            queue.pop();
            /*
            if (contractionRank % 10000 == 0) {
                // update all and rebuild queue
                for (typename std::vector<CHInfo*>::iterator it = queue.begin(); it != queue.end(); ++it) {
                    (*it)->updatePriority(mySPTree);
                }
                std::make_heap(queue.begin(), queue.end(), myCmp);
            }
            */
            contractionRank += 1;
        }
        // reporting
        let duration = current_millis() - startMillis;
        utils::common::msg_handler::write_message(&format!("Created {} shortcuts.", result.m().shortcuts.len()));
        utils::common::msg_handler::write_message(&format!("Recomputed priority {} times.", self.myUpdateCount));
        utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Message, |h| h.end_process_msg(&format!("done ({}ms).", duration)));
        utils::common::msg_handler::progress_done_message();
        self.myUpdateCount = 0;
        result
    }

    /// C++: `CHInfo * CHBuilder::getCHInfo(const E *const)` [private]
    // inline CHInfo* getCHInfo(const E* const edge) {
    //         return &(myCHInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn get_ch_info(&mut self, edge: Ptr<E>) -> Ptr<CHBuilder_CHInfo<E>> {
        Ptr::from_mut(&mut self.myCHInfos[edge.numerical_id() as usize])
    }

    /// C++: `void CHBuilder::synchronize(CHInfo &, double, const V *const, const SUMOAbstractRouter<E, V> *)` [private]
    /// copy connections from the original net (modified destructively during contraction)
    // void synchronize(CHInfo& info, double time, const V* const vehicle, const SUMOAbstractRouter<E, V>* effortProvider) {
    //         // forward and backward connections are used only in forward search,
    //         // thus approaching costs are those of the approaching edge and not of the edge itself
    //         const bool prune = !mySPTree->validatePermissions();
    //         const E* const edge = info.edge;
    //         if (prune && ((edge->getPermissions() & mySVC) != mySVC)) {
    //             return;
    //         }
    //         const double baseCost = effortProvider->getEffort(edge, vehicle, time);
    //
    //         for (const std::pair<const E*, const E*>& successor : edge->getViaSuccessors(mySVC)) {
    //             const E* fEdge = successor.first;
    //             if (prune && ((fEdge->getPermissions() & mySVC) != mySVC)) {
    //                 continue;
    //             }
    //             CHInfo* const follower = getCHInfo(fEdge);
    //             const SVCPermissions permissions = (edge->getPermissions() & fEdge->getPermissions());
    //             double cost = baseCost;
    //             const E* viaEdge = successor.second;
    //             while (viaEdge != nullptr && viaEdge->isInternal()) {
    //                 cost += effortProvider->getEffort(viaEdge, vehicle, time);
    //                 viaEdge = viaEdge->getViaSuccessors().front().first;
    //             }
    //             info.followers.push_back(CHConnection(follower, cost, permissions, 1));
    //             follower->approaching.push_back(CHConnection(&info, cost, permissions, 1));
    //         }
    // #ifdef CHRouter_DEBUG_WEIGHTS
    //         std::cout << time << ": " << edge->getID() << " baseCost: " << baseCost << "\n";
    // #endif
    //         // @todo: check whether we even need to save approaching in ROEdge;
    //     }
    #[allow(unused_variables)]
    pub fn synchronize(&mut self, info: &mut CHBuilder_CHInfo<E>, time: f64, vehicle: Ptr<V>, effortProvider: Ptr<SUMOAbstractRouter<E, V>>) {
        let prune = !self.mySPTree.m().validate_permissions();
        let edge = info.edge;
        if prune && ((e_permissions(edge) & self.mySVC) != self.mySVC) {
            return;
        }
        let baseCost = effortProvider.m().get_effort(edge, vehicle, time);

        for successor in edge.via_successors(self.mySVC, false) {
            let fEdge = successor.0;
            if prune && ((e_permissions(fEdge) & self.mySVC) != self.mySVC) {
                continue;
            }
            let follower = self.get_ch_info(fEdge);
            let permissions = e_permissions(edge) & e_permissions(fEdge);
            let mut cost = baseCost;
            let mut viaEdge = successor.1;
            while !viaEdge.is_null() && viaEdge.is_internal() {
                cost += effortProvider.m().get_effort(viaEdge, vehicle, time);
                viaEdge = viaEdge.via_successors(utils::common::sumo_vehicle_class::SVC_IGNORING, false)[0].0;
            }
            info.followers.push(CHBuilder_CHConnection::new(follower, cost, permissions, 1));
            follower.m().approaching.push(CHBuilder_CHConnection::new(ptr_of(info), cost, permissions, 1));
        }
        // @todo: check whether we even need to save approaching in ROEdge;
    }

    /// C++: `void CHBuilder::disconnect(CHConnections &, CHInfo *)` [private]
    /// remove all connections to/from the given edge (assume it exists only once)
    // void disconnect(CHConnections& connections, CHInfo* other) {
    //         for (typename CHConnections::iterator it = connections.begin(); it != connections.end(); it++) {
    //             if (it->target == other) {
    //                 connections.erase(it);
    //                 return;
    //             }
    //         }
    //         assert(false);
    //     }
    #[allow(unused_variables)]
    pub fn disconnect(&mut self, connections: &mut Vec<CHBuilder_CHConnection<E>>, other: Ptr<CHBuilder_CHInfo<E>>) {
        if let Some(pos) = connections.iter().position(|c| c.target.as_raw() == other.as_raw()) {
            connections.remove(pos);
            return;
        }
        assert!(false);
    }

    /// C++: `bool CHBuilder::tryUpdateFront(int &)` [private]
    /// tries to update the priority of the first edge
    // bool tryUpdateFront(std::vector<CHInfo*>& queue) {
    //         myUpdateCount++;
    //         CHInfo* max = queue.front();
    // #ifdef CHRouter_DEBUG_CONTRACTION_QUEUE
    //         std::cout << "updating '" << max->edge->getID() << "'\n";
    //         debugPrintQueue(queue);
    // #endif
    //         if (max->updatePriority(mySPTree)) {
    //             std::pop_heap(queue.begin(), queue.end(), myCmp);
    //             std::push_heap(queue.begin(), queue.end(), myCmp);
    //             return true;
    //         } else {
    //             return false;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn try_update_front(&mut self, queue: &mut Vec<Ptr<CHBuilder_CHInfo<E>>>) -> bool {
        self.myUpdateCount += 1;
        let max = queue[0];
        // #ifdef CHRouter_DEBUG_CONTRACTION_QUEUE: "updating '<edge id>'" and debugPrintQueue(queue) (debug only, off)
        if max.m().update_priority(self.mySPTree) {
            let cmp = CHBuilder_CHInfoComparator {};
            std_pop_heap(queue, &|a, b| cmp.call_op::<E, V>(a, b));
            std_push_heap(queue, &|a, b| cmp.call_op::<E, V>(a, b));
            true
        } else {
            false
        }
    }

    /// C++: `CHBuilder<E, V> & CHBuilder::operator=(const CHBuilder<E, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &CHBuilder<E, V>) -> &mut CHBuilder<E, V> {
        // C++ declares operator= private and never defines it, so no caller can reach it
        panic!("CHBuilder::operator= is private and undefined in C++")
    }

}

/// Forward/backward connection with associated FORWARD cost
/// C++ class `CHBuilder::CHConnection` (utils/router/CHBuilder.h)
pub struct CHBuilder_CHConnection<E> {
    pub target: Ptr<CHBuilder_CHInfo<E>>,
    pub cost: f64,
    pub permissions: SVCPermissions,
    /// the number of connections underlying this connection
    pub underlying: i32,
}

impl<E> CHBuilder_CHConnection<E> {
    /// C++: `void CHConnection::CHConnection(CHInfo *, double, SVCPermissions, int)`
    // CHConnection(CHInfo* t, double c, SVCPermissions p, int u):
    //             target(t), cost(c), permissions(p), underlying(u) {}
    #[allow(unused_variables)]
    pub fn new(t: Ptr<CHBuilder_CHInfo<E>>, c: f64, p: SVCPermissions, u: i32) -> Self {
        Self { target: t, cost: c, permissions: p, underlying: u }
    }

}

impl<E> SPTreeConnection<CHBuilder_CHInfo<E>> for CHBuilder_CHConnection<E> {
    fn target(&self) -> Ptr<CHBuilder_CHInfo<E>> {
        self.target
    }
    fn cost(&self) -> f64 {
        self.cost
    }
    fn permissions(&self) -> SVCPermissions {
        self.permissions
    }
}

impl<E: RouterEdgeInfo> SPTreeNode for CHBuilder_CHInfo<E> {
    type Connection = CHBuilder_CHConnection<E>;
    fn traveltime(&self) -> f64 {
        self.traveltime
    }
    fn set_traveltime(&mut self, traveltime: f64) {
        self.traveltime = traveltime;
    }
    fn depth(&self) -> i32 {
        self.depth
    }
    fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }
    fn visited(&self) -> bool {
        self.visited
    }
    fn set_visited(&mut self, visited: bool) {
        self.visited = visited;
    }
    fn permissions(&self) -> SVCPermissions {
        self.permissions
    }
    fn set_permissions(&mut self, permissions: SVCPermissions) {
        self.permissions = permissions;
    }
    fn followers(&self) -> &[CHBuilder_CHConnection<E>] {
        &self.followers
    }
    fn edge_numerical_id(&self) -> i32 {
        self.edge.m().edge_numerical_id()
    }
    fn edge_id(&self) -> String {
        self.edge.m().edge_id()
    }
    fn edge_permissions(&self) -> SVCPermissions {
        self.edge.m().edge_permissions()
    }
    fn reset(&mut self) {
        CHBuilder_CHInfo::<E>::reset(self)
    }
}

/// C++ class `CHBuilder::CHInfo` (utils/router/CHBuilder.h)
pub struct CHBuilder_CHInfo<E> {
    /// The current edge
    pub edge: Ptr<E>,
    /// The contraction priority
    pub priority: f64,
    /// The needed shortcuts
    pub shortcuts: Vec<CHBuilder_Shortcut<E>>,
    /// priority subterms
    pub contractedNeighbors: i32,
    pub rank: i32,
    pub level: i32,
    pub underlyingTotal: i32,
    /// connections (only valid after synchronization)
    pub followers: Vec<CHBuilder_CHConnection<E>>,
    pub approaching: Vec<CHBuilder_CHConnection<E>>,
    /// whether the edge has been visited during shortest path search
    pub visited: bool,
    /// Effort to reach the edge
    pub traveltime: f64,
    /// number of edges from start
    pub depth: i32,
    pub permissions: SVCPermissions,
}

impl<E: RouterEdgeInfo> CHBuilder_CHInfo<E> {
    /// C++: `void CHInfo::CHInfo(const E *const)`
    /// Constructor
    // CHInfo(const E* const e) :
    //             edge(e),
    //             priority(0.),
    //             contractedNeighbors(0),
    //             rank(-1),
    //             level(0),
    //             underlyingTotal(0),
    //             visited(false),
    //             traveltime(std::numeric_limits<double>::max()),
    //             depth(0),
    //             permissions(SVC_IGNORING) {
    //         }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<E>) -> Self {
        Self {
            edge: e,
            priority: 0.,
            shortcuts: Vec::new(),
            contractedNeighbors: 0,
            rank: -1,
            level: 0,
            underlyingTotal: 0,
            followers: Vec::new(),
            approaching: Vec::new(),
            visited: false,
            traveltime: f64::MAX,
            depth: 0,
            permissions: SVC_IGNORING,
        }
    }

    /// C++: `bool CHInfo::updatePriority(SPTree<CHInfo, CHConnection> *)`
    /// recompute the contraction priority and report whether it changed
    // bool updatePriority(SPTree<CHInfo, CHConnection>* spTree) {
    //             if (spTree != nullptr) {
    //                 updateShortcuts(spTree);
    //                 updateLevel();
    //             } else {
    //                 contractedNeighbors += 1; // called when a connected edge was contracted
    //             }
    //             const double oldPriority = priority;
    //             // priority term as used by abraham []
    //             const int edge_difference = (int)followers.size() + (int)approaching.size() - 2 * (int)shortcuts.size();
    //             priority = (double)(2 * edge_difference - contractedNeighbors - underlyingTotal - 5 * level);
    //             return priority != oldPriority;
    //         }
    #[allow(unused_variables)]
    pub fn update_priority(&mut self, spTree: Ptr<SPTree<CHBuilder_CHInfo<E>, CHBuilder_CHConnection<E>>>) -> bool {
        if !spTree.is_null() {
            self.update_shortcuts(spTree);
            self.update_level();
        } else {
            self.contractedNeighbors += 1; // called when a connected edge was contracted
        }
        let oldPriority = self.priority;
        // priority term as used by abraham []
        let edge_difference = self.followers.len() as i32 + self.approaching.len() as i32 - 2 * self.shortcuts.len() as i32;
        self.priority = (2 * edge_difference - self.contractedNeighbors - self.underlyingTotal - 5 * self.level) as f64;
        self.priority != oldPriority
    }

    /// C++: `void CHInfo::updateShortcuts(SPTree<CHInfo, CHConnection> *)`
    /// compute needed shortcuts when contracting this edge
    // void updateShortcuts(SPTree<CHInfo, CHConnection>* spTree) {
    //             const bool validatePermissions = spTree->validatePermissions();
    // #ifdef CHRouter_DEBUG_CONTRACTION_DEGREE
    //             const int degree = (int)approaching.size() + (int)followers.size();
    //             std::cout << "computing shortcuts for '" + edge->getID() + "' with degree " + toString(degree) + "\n";
    // #endif
    //             shortcuts.clear();
    //             underlyingTotal = 0;
    //             for (const CHConnection& aInfo : approaching) {
    //                 // build shortest path tree in a fixed neighborhood
    //                 spTree->rebuildFrom(aInfo.target, this);
    //                 for (const CHConnection& fInfo : followers) {
    //                     const double viaCost = aInfo.cost + fInfo.cost;
    //                     const SVCPermissions viaPermissions = (aInfo.permissions & fInfo.permissions);
    //                     if (fInfo.target->traveltime > viaCost) {
    //                         // found no faster path -> we need a shortcut via edge
    // #ifdef CHRouter_DEBUG_CONTRACTION_WITNESSES
    //                         debugNoWitness(aInfo, fInfo);
    // #endif
    //                         const int underlying = aInfo.underlying + fInfo.underlying;
    //                         underlyingTotal += underlying;
    //                         shortcuts.push_back(Shortcut(ConstEdgePair(aInfo.target->edge, fInfo.target->edge),
    //                                                      viaCost, underlying, viaPermissions));
    //
    //                     } else if (validatePermissions) {
    //                         if ((fInfo.target->permissions & viaPermissions) != viaPermissions) {
    //                             // witness has weaker restrictions. try to find another witness
    //                             spTree->registerForValidation(&aInfo, &fInfo);
    //                         } else {
    // #ifdef CHRouter_DEBUG_CONTRACTION_WITNESSES
    //                             debugNoWitness(aInfo, fInfo);
    // #endif
    //                         }
    //                     } else {
    // #ifdef CHRouter_DEBUG_CONTRACTION_WITNESSES
    //                         debugNoWitness(aInfo, fInfo);
    // #endif
    //                     }
    //                 }
    //             }
    //             // insert shortcuts needed due to unmet permissions
    //             if (validatePermissions) {
    //                 for (const CHConnectionPair& chcp : spTree->getNeededShortcuts(this)) {
    //                     const CHConnection* aInfo = chcp.first;
    //                     const CHConnection* fInfo = chcp.second;
    //                     const double viaCost = aInfo->cost + fInfo->cost;
    //                     const SVCPermissions viaPermissions = (aInfo->permissions & fInfo->permissions);
    //                     const int underlying = aInfo->underlying + fInfo->underlying;
    //                     underlyingTotal += underlying;
    //                     shortcuts.push_back(Shortcut(ConstEdgePair(aInfo->target->edge, fInfo->target->edge),
    //                                                  viaCost, underlying, viaPermissions));
    //                 }
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn update_shortcuts(&mut self, spTree: Ptr<SPTree<CHBuilder_CHInfo<E>, CHBuilder_CHConnection<E>>>) {
        let validatePermissions = spTree.m().validate_permissions();
        self.shortcuts.clear();
        self.underlyingTotal = 0;
        let this = Ptr::from_mut(self);
        for aInfo in self.approaching.iter() {
            // build shortest path tree in a fixed neighborhood
            spTree.m().rebuild_from(aInfo.target, this);
            for fInfo in self.followers.iter() {
                let viaCost = aInfo.cost + fInfo.cost;
                let viaPermissions = aInfo.permissions & fInfo.permissions;
                if fInfo.target.m().traveltime > viaCost {
                    // found no faster path -> we need a shortcut via edge
                    let underlying = aInfo.underlying + fInfo.underlying;
                    self.underlyingTotal += underlying;
                    self.shortcuts.push(CHBuilder_Shortcut::new((aInfo.target.m().edge, fInfo.target.m().edge), viaCost, underlying, viaPermissions));
                } else if validatePermissions {
                    if (fInfo.target.m().permissions & viaPermissions) != viaPermissions {
                        // witness has weaker restrictions. try to find another witness
                        spTree.m().register_for_validation(ptr_of(aInfo), ptr_of(fInfo));
                    }
                }
            }
        }
        // insert shortcuts needed due to unmet permissions
        if validatePermissions {
            for &(aInfo, fInfo) in spTree.m().get_needed_shortcuts(this).iter() {
                let viaCost = aInfo.m().cost + fInfo.m().cost;
                let viaPermissions = aInfo.m().permissions & fInfo.m().permissions;
                let underlying = aInfo.m().underlying + fInfo.m().underlying;
                self.underlyingTotal += underlying;
                self.shortcuts.push(CHBuilder_Shortcut::new((aInfo.m().target.m().edge, fInfo.m().target.m().edge), viaCost, underlying, viaPermissions));
            }
        }
    }

    /// C++: `void CHInfo::updateLevel()`
    // void updateLevel() {
    //             int maxLower = std::numeric_limits<int>::min();
    //             for (const CHConnection& con : approaching) {
    //                 if (con.target->rank < rank) {
    //                     maxLower = MAX2(rank, maxLower);
    //                 }
    //             }
    //             for (const CHConnection& con : followers) {
    //                 if (con.target->rank < rank) {
    //                     maxLower = MAX2(rank, maxLower);
    //                 }
    //             }
    //             if (maxLower == std::numeric_limits<int>::min()) {
    //                 level = 0;
    //             } else {
    //                 level = maxLower + 1;
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn update_level(&mut self) {
        let mut maxLower = i32::MIN;
        for con in self.approaching.iter() {
            if con.target.m().rank < self.rank {
                maxLower = self.rank.max(maxLower); // faithful to C++ MAX2(rank, maxLower): uses this->rank, not con.target->rank
            }
        }
        for con in self.followers.iter() {
            if con.target.m().rank < self.rank {
                maxLower = self.rank.max(maxLower);
            }
        }
        if maxLower == i32::MIN {
            self.level = 0;
        } else {
            self.level = maxLower + 1;
        }
    }

    /// C++: `void CHInfo::resetContractionState()`
    // void resetContractionState() {
    //             priority = 0.;
    //             shortcuts.clear();
    //             contractedNeighbors = 0;
    //             rank = -1;
    //             level = 0;
    //             underlyingTotal = 0;
    //             followers.clear();
    //             approaching.clear();
    //             reset();  // just to make sure
    //         }
    #[allow(unused_variables)]
    pub fn reset_contraction_state(&mut self) {
        self.priority = 0.;
        self.shortcuts.clear();
        self.contractedNeighbors = 0;
        self.rank = -1;
        self.level = 0;
        self.underlyingTotal = 0;
        self.followers.clear();
        self.approaching.clear();
        self.reset(); // just to make sure
    }

    /// C++: `void CHInfo::reset()`
    // inline void reset() {
    //             traveltime = std::numeric_limits<double>::max();
    //             visited = false;
    //             depth = 0;
    //             permissions = SVC_IGNORING;
    //         }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.traveltime = f64::MAX;
        self.visited = false;
        self.depth = 0;
        self.permissions = SVC_IGNORING;
    }

    /// C++: `void CHInfo::debugNoWitness(const CHConnection &, const CHConnection &)`
    /// debugging methods
    // inline void debugNoWitness(const CHConnection& aInfo, const CHConnection& fInfo) {
    //             std::cout << "adding shortcut between " << aInfo.target->edge->getID() << ", " << fInfo.target->edge->getID() << " via " << edge->getID() << "\n";
    //         }
    #[allow(unused_variables)]
    pub fn debug_no_witness<V>(&mut self, aInfo: &CHBuilder_CHConnection<E>, fInfo: &CHBuilder_CHConnection<E>) where E: RouterEdge<V> {
        print!("adding shortcut between {}, {} via {}\n", aInfo.target.m().edge.id(), fInfo.target.m().edge.id(), self.edge.id());
    }

    /// C++: `void CHInfo::debugWitness(const CHConnection &, const CHConnection &)`
    // inline void debugWitness(const CHConnection& aInfo, const CHConnection& fInfo) {
    //             const double viaCost = aInfo.cost + fInfo.cost;
    //             std::cout << "found witness with length " << fInfo.target->traveltime << " against via " << edge->getID() << " (length " << viaCost << ") for " << aInfo.target->edge->getID() << ", " << fInfo.target->edge->getID() << "\n";
    //         }
    #[allow(unused_variables)]
    pub fn debug_witness<V>(&mut self, aInfo: &CHBuilder_CHConnection<E>, fInfo: &CHBuilder_CHConnection<E>) where E: RouterEdge<V> {
        let viaCost = aInfo.cost + fInfo.cost;
        print!(
            "found witness with length {} against via {} (length {}) for {}, {}\n",
            utils::common::to_string::f64_to_string_prec(fInfo.target.m().traveltime, 6),
            self.edge.id(),
            utils::common::to_string::f64_to_string_prec(viaCost, 6),
            aInfo.target.m().edge.id(),
            fInfo.target.m().edge.id()
        );
    }

}

/// Class to compare (and so sort) nodes by their contraction priority
/// C++ class `CHBuilder::CHInfoComparator` (utils/router/CHBuilder.h)
pub struct CHBuilder_CHInfoComparator {
}

impl CHBuilder_CHInfoComparator {
    /// C++: `bool CHInfoComparator::operator()(const CHInfo *, const CHInfo *) const` [const]
    /// Comparing method
    // bool operator()(const CHInfo* a, const CHInfo* b) const {
    //             if (a->priority == b->priority) {
    //                 return a->edge->getNumericalID() > b->edge->getNumericalID();
    //             } else {
    //                 return a->priority < b->priority;
    //             };
    //         }
    #[allow(unused_variables)]
    pub fn call_op<E: RouterEdge<V>, V>(&self, a: Ptr<CHBuilder_CHInfo<E>>, b: Ptr<CHBuilder_CHInfo<E>>) -> bool {
        if a.m().priority == b.m().priority {
            a.m().edge.numerical_id() > b.m().edge.numerical_id()
        } else {
            a.m().priority < b.m().priority
        }
    }

}

/// Forward/backward connection with associated forward/backward cost
/// C++ class `CHBuilder::Connection` (utils/router/CHBuilder.h)
pub struct CHBuilder_Connection {
    pub target: i32,
    pub cost: f64,
    pub permissions: SVCPermissions,
}

impl CHBuilder_Connection {
    /// C++: `void Connection::Connection(int, double, SVCPermissions)`
    // Connection(int t, double c, SVCPermissions p): target(t), cost(c), permissions(p) {}
    #[allow(unused_variables)]
    pub fn new(t: i32, c: f64, p: SVCPermissions) -> Self {
        Self { target: t, cost: c, permissions: p }
    }

}

/// C++ class `CHBuilder::Hierarchy` (utils/router/CHBuilder.h)
pub struct CHBuilder_Hierarchy<E> {
    pub shortcuts: CHBuilder_ShortcutVia<E>,
    pub forwardUplinks: Vec<Vec<CHBuilder_Connection>>,
    pub backwardUplinks: Vec<Vec<CHBuilder_Connection>>,
}

impl<E> CHBuilder_Hierarchy<E> {
}

/// C++ class `CHBuilder::Shortcut` (utils/router/CHBuilder.h)
pub struct CHBuilder_Shortcut<E> {
    pub edgePair: CHBuilder_ConstEdgePair<E>,
    pub cost: f64,
    pub underlying: i32,
    pub permissions: SVCPermissions,
}

impl<E> CHBuilder_Shortcut<E> {
    /// C++: `void Shortcut::Shortcut(ConstEdgePair, double, int, SVCPermissions)`
    // Shortcut(ConstEdgePair e, double c, int u, SVCPermissions p):
    //             edgePair(e), cost(c), underlying(u), permissions(p) {}
    #[allow(unused_variables)]
    pub fn new(e: CHBuilder_ConstEdgePair<E>, c: f64, u: i32, p: SVCPermissions) -> Self {
        Self { edgePair: e, cost: c, underlying: u, permissions: p }
    }

}

