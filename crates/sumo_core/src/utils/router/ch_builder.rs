//! Port of utils/router/CHBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<const CHConnection *, const CHConnection *> CHBuilder::CHConnectionPair`
pub type CHBuilder_CHConnectionPair = (Ptr<CHBuilder_CHConnection>, Ptr<CHBuilder_CHConnection>);

/// C++ typedef `int CHBuilder::CHConnectionPairs`
pub type CHBuilder_CHConnectionPairs = i32;

/// C++ typedef `int CHBuilder::CHConnections`
pub type CHBuilder_CHConnections = i32;

/// C++ typedef `std::pair<const E *, const E *> CHBuilder::ConstEdgePair`
pub type CHBuilder_ConstEdgePair = Unmapped /* std::pair<const E *, const E *> */;

/// C++ typedef `int CHBuilder::ShortcutVia`
pub type CHBuilder_ShortcutVia = i32;

/// Computes the shortest path through a contracted network
/// C++ class `CHBuilder` (utils/router/CHBuilder.h)
pub struct CHBuilder<E, V> {
    /// all edges with numerical ids
    pub myEdges: Ptr<i32>,
    /// the handler for routing errors
    pub myErrorMsgHandler: Ptr<MsgHandler>,
    /// static vector for lookup
    pub myCHInfos: i32,
    /// Comparator for contraction priority
    pub myCmp: CHBuilder_CHInfoComparator,
    /// the shortest path tree to use when searching for shortcuts
    pub mySPTree: Ptr<SPTree<CHBuilder_CHInfo, CHBuilder_CHConnection>>,
    /// the permissions for which the hierarchy was constructed
    pub mySVC: SUMOVehicleClass,
    /// counters for performance logging
    pub myUpdateCount: i32,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> CHBuilder<E, V> {
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
    pub fn new(edges: &i32, unbuildIsWarning: bool, svc: SUMOVehicleClass, validatePermissions: bool) -> Self {
        todo!("port CHBuilder::CHBuilder<E, V>")
    }

    /// C++: `void CHBuilder::~CHBuilder<E, V>()` [virtual]
    /// Destructor
    // virtual ~CHBuilder() {
    //         delete mySPTree;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port CHBuilder::~CHBuilder<E, V>")
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
    //             contractionRank++;
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
    pub fn build_contraction_hierarchy(&mut self, time: SUMOTime, vehicle: Ptr<V>, effortProvider: Ptr<SUMOAbstractRouter<E, V>>) -> Ptr<CHBuilder_Hierarchy> {
        todo!("port CHBuilder::buildContractionHierarchy")
    }

    /// C++: `CHInfo * CHBuilder::getCHInfo(const E *const)` [private]
    // inline CHInfo* getCHInfo(const E* const edge) {
    //         return &(myCHInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn get_ch_info(&mut self, edge: Ptr<E>) -> Ptr<CHBuilder_CHInfo> {
        todo!("port CHBuilder::getCHInfo")
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
    pub fn synchronize(&mut self, info: &mut CHBuilder_CHInfo, time: f64, vehicle: Ptr<V>, effortProvider: Ptr<SUMOAbstractRouter<E, V>>) {
        todo!("port CHBuilder::synchronize")
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
    pub fn disconnect(&mut self, connections: &mut SPTree_CHConnections, other: Ptr<CHBuilder_CHInfo>) {
        todo!("port CHBuilder::disconnect")
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
    pub fn try_update_front(&mut self, queue: &mut i32) -> bool {
        todo!("port CHBuilder::tryUpdateFront")
    }

    /// C++: `CHBuilder<E, V> & CHBuilder::operator=(const CHBuilder<E, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &CHBuilder<E, V>) -> &mut CHBuilder<E, V> {
        todo!("port CHBuilder::operator=")
    }

}

/// Forward/backward connection with associated FORWARD cost
/// C++ class `CHBuilder::CHConnection` (utils/router/CHBuilder.h)
pub struct CHBuilder_CHConnection {
    pub target: Ptr<CHBuilder_CHInfo>,
    pub cost: f64,
    pub permissions: SVCPermissions,
    /// the number of connections underlying this connection
    pub underlying: i32,
}

impl CHBuilder_CHConnection {
    /// C++: `void CHConnection::CHConnection(CHInfo *, double, SVCPermissions, int)`
    // CHConnection(CHInfo* t, double c, SVCPermissions p, int u):
    //             target(t), cost(c), permissions(p), underlying(u) {}
    #[allow(unused_variables)]
    pub fn new(t: Ptr<CHBuilder_CHInfo>, c: f64, p: SVCPermissions, u: i32) -> Self {
        todo!("port CHConnection::CHConnection")
    }

}

/// C++ class `CHBuilder::CHInfo` (utils/router/CHBuilder.h)
pub struct CHBuilder_CHInfo {
    /// The current edge
    pub edge: Ptr<Unmapped /* E */>,
    /// The contraction priority
    pub priority: f64,
    /// The needed shortcuts
    pub shortcuts: i32,
    /// priority subterms
    pub contractedNeighbors: i32,
    pub rank: i32,
    pub level: i32,
    pub underlyingTotal: i32,
    /// connections (only valid after synchronization)
    pub followers: SPTree_CHConnections,
    pub approaching: SPTree_CHConnections,
    /// whether the edge has been visited during shortest path search
    pub visited: bool,
    /// Effort to reach the edge
    pub traveltime: f64,
    /// number of edges from start
    pub depth: i32,
    pub permissions: SVCPermissions,
}

impl CHBuilder_CHInfo {
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
    pub fn new(e: Ptr<Unmapped /* E */>) -> Self {
        todo!("port CHInfo::CHInfo")
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
    pub fn update_priority(&mut self, spTree: Ptr<SPTree<CHBuilder_CHInfo, CHBuilder_CHConnection>>) -> bool {
        todo!("port CHInfo::updatePriority")
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
    pub fn update_shortcuts(&mut self, spTree: Ptr<SPTree<CHBuilder_CHInfo, CHBuilder_CHConnection>>) {
        todo!("port CHInfo::updateShortcuts")
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
        todo!("port CHInfo::updateLevel")
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
        todo!("port CHInfo::resetContractionState")
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
        todo!("port CHInfo::reset")
    }

    /// C++: `void CHInfo::debugNoWitness(const CHConnection &, const CHConnection &)`
    /// debugging methods
    // inline void debugNoWitness(const CHConnection& aInfo, const CHConnection& fInfo) {
    //             std::cout << "adding shortcut between " << aInfo.target->edge->getID() << ", " << fInfo.target->edge->getID() << " via " << edge->getID() << "\n";
    //         }
    #[allow(unused_variables)]
    pub fn debug_no_witness(&mut self, aInfo: &CHBuilder_CHConnection, fInfo: &CHBuilder_CHConnection) {
        todo!("port CHInfo::debugNoWitness")
    }

    /// C++: `void CHInfo::debugWitness(const CHConnection &, const CHConnection &)`
    // inline void debugWitness(const CHConnection& aInfo, const CHConnection& fInfo) {
    //             const double viaCost = aInfo.cost + fInfo.cost;
    //             std::cout << "found witness with length " << fInfo.target->traveltime << " against via " << edge->getID() << " (length " << viaCost << ") for " << aInfo.target->edge->getID() << ", " << fInfo.target->edge->getID() << "\n";
    //         }
    #[allow(unused_variables)]
    pub fn debug_witness(&mut self, aInfo: &CHBuilder_CHConnection, fInfo: &CHBuilder_CHConnection) {
        todo!("port CHInfo::debugWitness")
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
    pub fn call_op(&self, a: Ptr<CHBuilder_CHInfo>, b: Ptr<CHBuilder_CHInfo>) -> bool {
        todo!("port CHInfoComparator::operator()")
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
        todo!("port Connection::Connection")
    }

}

/// C++ class `CHBuilder::Hierarchy` (utils/router/CHBuilder.h)
pub struct CHBuilder_Hierarchy {
    pub shortcuts: CHBuilder_ShortcutVia,
    pub forwardUplinks: i32,
    pub backwardUplinks: i32,
}

impl CHBuilder_Hierarchy {
}

/// C++ class `CHBuilder::Shortcut` (utils/router/CHBuilder.h)
pub struct CHBuilder_Shortcut {
    pub edgePair: CHBuilder_ConstEdgePair,
    pub cost: f64,
    pub underlying: i32,
    pub permissions: SVCPermissions,
}

impl CHBuilder_Shortcut {
    /// C++: `void Shortcut::Shortcut(ConstEdgePair, double, int, SVCPermissions)`
    // Shortcut(ConstEdgePair e, double c, int u, SVCPermissions p):
    //             edgePair(e), cost(c), underlying(u), permissions(p) {}
    #[allow(unused_variables)]
    pub fn new(e: CHBuilder_ConstEdgePair, c: f64, u: i32, p: SVCPermissions) -> Self {
        todo!("port Shortcut::Shortcut")
    }

}

