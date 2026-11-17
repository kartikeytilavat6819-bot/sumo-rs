//! Port of netbuild/NBNodeCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<NBNode *, double> NBNodeCont::NodeAndDist`
pub type NBNodeCont_NodeAndDist = (Ptr<NBNode>, f64);

/// C++ typedef `std::vector<NodeSet> NBNodeCont::NodeClusters`
pub type NBNodeCont_NodeClusters = Vec<NodeSet>;

/// C++ typedef `std::map<std::string, NBNode *> NBNodeCont::NodeCont`
pub type NBNodeCont_NodeCont = BTreeMap<String, Ptr<NBNode>>;

use utils::common::to_string::join_to_string;
use utils::common::msg_handler::{write_message, write_warning};
use utils::xml::sumo_xml_definitions::SUMO_PARAM_ORIGID;
use utils::geom::POSITION_EPS;
use super::nb_cont::KeepClear;
use utils::common::std_defs::SUMO_CONST_LANE_WIDTH;
use utils::common::named::{Named, NamedStoringVisitor};
use crate::utils::named_rtree::NamedRTree;

// C++ NBEdge::UNSPECIFIED_SIGNAL_OFFSET (NBEdge.cpp)
const UNSPECIFIED_SIGNAL_OFFSET: f64 = -1.;

/// `std::istringstream(id) >> long long`: optional leading whitespace, optional sign, then digits;
/// None when no number can be read
fn parse_leading_i64(s: &str) -> Option<i64> {
    let t = s.trim_start();
    let bytes = t.as_bytes();
    let mut i = 0;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let digits_start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digits_start {
        return None;
    }
    t[..i].parse().ok()
}

/// C++ `map[key]` on std::map<pair<NBEdge*, NBEdge*>, SVCPermissions>: inserts 0 when the key is missing
fn pc_get_or_insert(map: &mut BTreeMap<(Ptr<NBEdge>, Ptr<NBEdge>), SVCPermissions>, key: (Ptr<NBEdge>, Ptr<NBEdge>)) -> SVCPermissions {
    *map.entry(key).or_insert(0)
}

/// C++ `addLane2LaneConnection(fromLane, dest, toLane, USER, false, mayDefinitelyPass)` with the header defaults filled in
fn add_lane2_lane_connection_user(from: Ptr<NBEdge>, fromLane: i32, dest: Ptr<NBEdge>, toLane: i32, mayDefinitelyPass: bool) -> bool {
    from.m().add_lane2_lane_connection(fromLane, dest, toLane, NBEdge_Lane2LaneInfoType::USER, false, mayDefinitelyPass, KeepClear::KEEPCLEAR_UNSPECIFIED, -1., -1., -1., 1., crate::netbuild::nb_edge::my_default_connection_length(), &PositionVector::EMPTY, false, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false)
}

/// C++ `IDSupplier` (utils/common/IDSupplier.cpp) with an empty prefix, as used by NBNodeCont::remapIDs
struct NodeIDSupplier {
    myCurrent: i64,
}

impl NodeIDSupplier {
    /// C++ `IDSupplier(prefix, knownIDs)`
    fn new(knownIDs: &[String]) -> Self {
        let mut s = NodeIDSupplier { myCurrent: 0 };
        for id in knownIDs {
            if let Some(number) = parse_leading_i64(id) {
                s.myCurrent = s.myCurrent.max(number + 1);
            }
        }
        s
    }

    /// C++ `IDSupplier::getNext()`
    fn get_next(&mut self) -> String {
        let s = self.myCurrent.to_string();
        self.myCurrent += 1;
        s
    }
}

/// C++ `std::set<NBNode*, ComparatorIdLess>` iterates in id order, but NodeSet orders by address:
/// iterate through this wherever the C++ iteration order matters.
// determinism: C++ NodeSet iteration is by node id, so sort by id here
fn nodes_by_id(nodes: &NodeSet) -> Vec<Ptr<NBNode>> {
    let mut v: Vec<Ptr<NBNode>> = nodes.iter().copied().collect();
    v.sort_by(|a, b| a.get_id().cmp(b.get_id()));
    v
}

/// C++ `joinNamedToString(nodes, sep)`: ids in NodeSet (id) order
fn join_node_ids(nodes: &NodeSet, sep: &str) -> String {
    nodes_by_id(nodes).iter().map(|n| n.get_id().to_string()).collect::<Vec<_>>().join(sep)
}

/// C++ `delete node`
fn delete_node(node: Ptr<NBNode>) {
    node.m().drop_();
    drop(unsafe { Box::from_raw(node.as_raw()) });
}

/// C++ class `NBNodeCont::JoinCluster` (netbuild/NBNodeCont.h)
pub struct NBNodeCont_JoinCluster {
    pub cluster: BTreeSet<String>,
    pub node: Ptr<NBNode>,
    pub resetConnections: bool,
}

impl NBNodeCont_JoinCluster {
}

/// Container for nodes during the netbuilding process
/// C++ class `NBNodeCont` (netbuild/NBNodeCont.h)
pub struct NBNodeCont {
    /// The map of names to nodes
    pub myNodes: NBNodeCont_NodeCont,
    /// The extracted nodes which are kept for reference
    pub myExtractedNodes: NBNodeCont_NodeCont,
    /// set of node ids which should not be joined
    pub myJoinExclusions: BTreeSet<String>,
    /// loaded sets of node ids to join (cleared after use)
    pub myClusters2Join: Vec<NBNodeCont_JoinCluster>,
    /// sets of node ids which were joined
    pub myJoinedClusters: Vec<BTreeSet<String>>,
    /// ids found in loaded join clusters used for error checking
    pub myJoined: BTreeSet<String>,
    /// nodes that were created when splitting an edge
    pub mySplit: BTreeSet<Ptr<NBNode>>,
    /// nodes that received a traffic light due to guessing (--tls.guess)
    pub myGuessedTLS: BTreeSet<Ptr<NBNode>>, // C++: std::set<NBNode *, ComparatorIdLess>
    /// nodes that are excluded from tls-guessing
    pub myUnsetTLS: BTreeSet<Ptr<NBNode>>,
    /// node positions for faster lookup
    pub myRTree: NamedRTree,
    /// network components that must be removed if not connected to the road network via stop access
    pub myRailComponents: Vec<Vec<String>>,
}

impl NBNodeCont {
    /// C++: `void NBNodeCont::NBNodeCont()`
    /// Constructor
    // NBNodeCont() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        NBNodeCont {
            myNodes: BTreeMap::new(),
            myExtractedNodes: BTreeMap::new(),
            myJoinExclusions: BTreeSet::new(),
            myClusters2Join: Vec::new(),
            myJoinedClusters: Vec::new(),
            myJoined: BTreeSet::new(),
            mySplit: BTreeSet::new(),
            myGuessedTLS: BTreeSet::new(),
            myUnsetTLS: BTreeSet::new(),
            myRTree: NamedRTree::new(),
            myRailComponents: Vec::new(),
        }
    }

    /// C++: `void NBNodeCont::~NBNodeCont()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ ~NBNodeCont() { clear(); }
        self.clear();
    }

    /// C++: `bool NBNodeCont::insert(const std::string &, const Position &, NBDistrict *)`
    /// Inserts a node into the map
    // bool
    // NBNodeCont::insert(const std::string& id, const Position& position,
    //                    NBDistrict* district) {
    //     NodeCont::iterator i = myNodes.find(id);
    //     if (i != myNodes.end()) {
    //         return false;
    //     }
    //     NBNode* node = new NBNode(id, position, district);
    //     myNodes[id] = node;
    //     const float pos[2] = {(float)position.x(), (float)position.y()};
    //     myRTree.Insert(pos, pos, node);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert(&mut self, id: &str, position: &Position, district: Ptr<NBDistrict>) -> bool {
        if self.myNodes.contains_key(id) {
            return false;
        }
        let node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(id, position, district))));
        self.myNodes.insert(id.to_string(), node);
        let pos = [position.x() as f32, position.y() as f32];
        self.myRTree.insert(&pos, &pos, &node.cast::<Named>());
        true
    }

    /// C++: `bool NBNodeCont::insert(NBNode *)`
    /// Inserts a node into the map
    // bool
    // NBNodeCont::insert(NBNode* node) {
    //     std::string id = node->getID();
    //     NodeCont::iterator i = myNodes.find(id);
    //     if (i != myNodes.end()) {
    //         return false;
    //     }
    //     myNodes[id] = node;
    //     const float pos[2] = {(float)node->getPosition().x(), (float)node->getPosition().y()};
    //     myRTree.Insert(pos, pos, node);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert_2(&mut self, node: Ptr<NBNode>) -> bool {
        let id = node.get_id().to_string();
        if self.myNodes.contains_key(&id) {
            return false;
        }
        let p = node.get_position();
        let pos = [p.x() as f32, p.y() as f32];
        self.myNodes.insert(id, node);
        self.myRTree.insert(&pos, &pos, &node.cast::<Named>());
        true
    }

    /// C++: `bool NBNodeCont::erase(NBNode *)`
    /// Removes the given node, deleting it
    // bool
    // NBNodeCont::erase(NBNode* node) {
    //     if (extract(node)) {
    //         delete node;
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn erase(&mut self, node: Ptr<NBNode>) -> bool {
        if self.extract(node, false) {
            delete_node(node);
            true
        } else {
            false
        }
    }

    /// C++: `bool NBNodeCont::extract(NBNode *, bool)`
    /// Removes the given node but does not delete it
    // bool
    // NBNodeCont::extract(NBNode* node, bool remember) {
    //     NodeCont::iterator i = myNodes.find(node->getID());
    //     if (i == myNodes.end()) {
    //         return false;
    //     }
    //     myNodes.erase(i);
    //     const float pos[2] = {(float)node->getPosition().x(), (float)node->getPosition().y()};
    //     myRTree.Remove(pos, pos, node);
    //     node->removeTrafficLights();
    //     if (remember) {
    //         myExtractedNodes[node->getID()] = node;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn extract(&mut self, node: Ptr<NBNode>, remember: bool) -> bool {
        let id = node.get_id().to_string();
        if !self.myNodes.contains_key(&id) {
            return false;
        }
        self.myNodes.remove(&id);
        let p = node.get_position();
        let pos = [p.x() as f32, p.y() as f32];
        self.myRTree.remove(&pos, &pos, &node.cast::<Named>());
        node.m().remove_traffic_lights(false);
        if remember {
            self.myExtractedNodes.insert(node.get_id().to_string(), node);
        }
        true
    }

    /// C++: `NBNode * NBNodeCont::retrieve(const std::string &) const` [const]
    /// Returns the node with the given name
    // NBNode*
    // NBNodeCont::retrieve(const std::string& id) const {
    //     NodeCont::const_iterator i = myNodes.find(id);
    //     if (i == myNodes.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn retrieve(&self, id: &str) -> Ptr<NBNode> {
        self.myNodes.get(id).copied().unwrap_or(Ptr::null())
    }

    /// C++: `std::vector<NBNode *> NBNodeCont::retrieveByPos(const Position &, const double) const` [const]
    /// Returns the node with the given coordinates
    // std::vector<NBNode*>
    // NBNodeCont::retrieveByPos(const Position& position, const double offset) const {
    //     std::vector<NBNode*> result;
    //     const double extOffset = offset + POSITION_EPS;
    //     const float cmin[2] = {(float)(position.x() - extOffset), (float)(position.y() - extOffset)};
    //     const float cmax[2] = {(float)(position.x() + extOffset), (float)(position.y() + extOffset)};
    //     std::set<const Named*> into;
    //     Named::StoringVisitor sv(into);
    //     myRTree.Search(cmin, cmax, sv);
    //     for (const Named* namedNode : into) {
    //         NBNode* node = const_cast<NBNode*>(dynamic_cast<const NBNode*>(namedNode));
    //         if (fabs(node->getPosition().x() - position.x()) <= offset
    //                 &&
    //                 fabs(node->getPosition().y() - position.y()) <= offset) {
    //             result.push_back(node);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_by_pos(&self, position: &Position, offset: f64) -> Vec<Ptr<NBNode>> {
        let extOffset = offset + POSITION_EPS;
        let cmin = [(position.x() - extOffset) as f32, (position.y() - extOffset) as f32];
        let cmax = [(position.x() + extOffset) as f32, (position.y() + extOffset) as f32];
        let mut into: BTreeSet<Ptr<Named>> = BTreeSet::new();
        let sv = NamedStoringVisitor::new(&mut into);
        self.myRTree.search(&cmin, &cmax, &sv, |d, c| d.add_to(c));
        let mut result = Vec::new();
        for namedNode in &into {
            // dynamic_cast<const NBNode*>: only NBNodes are inserted into myRTree
            let node: Ptr<NBNode> = namedNode.cast::<NBNode>();
            let p = node.get_position();
            if (p.x() - position.x()).abs() <= offset && (p.y() - position.y()).abs() <= offset {
                result.push(node);
            }
        }
        result
    }

    /// C++: `std::map<std::string, NBNode *>::const_iterator NBNodeCont::begin() const` [const]
    /// Returns the pointer to the begin of the stored nodes
    // std::map<std::string, NBNode*>::const_iterator begin() const {
    //         return myNodes.begin();
    //     }
    #[allow(unused_variables)]
    pub fn begin(&self) -> std::collections::btree_map::Iter<'_, String, Ptr<NBNode>> {
        self.myNodes.iter()
    }

    /// C++: `std::map<std::string, NBNode *>::const_iterator NBNodeCont::end() const` [const]
    /// Returns the pointer to the end of the stored nodes
    // std::map<std::string, NBNode*>::const_iterator end() const {
    //         return myNodes.end();
    //     }
    #[allow(unused_variables)]
    pub fn end(&self) -> std::iter::Skip<std::collections::btree_map::Iter<'_, String, Ptr<NBNode>>> {
        // shortcut: C++ iterator pair cannot be compared in Rust, end() is an exhausted iterator; callers iterate begin() instead, upgrade if a caller needs the C++ iterator protocol
        self.myNodes.iter().skip(self.myNodes.len())
    }

    /// C++: `void NBNodeCont::addJoinExclusion(const std::vector<std::string> &)`
    // void
    // NBNodeCont::addJoinExclusion(const std::vector<std::string>& ids) {
    //     for (const std::string& nodeID : ids) {
    //         // error handling has to take place here since joinExclusions could be
    //         // loaded from multiple files / command line
    //         if (myJoined.count(nodeID) > 0) {
    //             WRITE_WARNINGF(TL("Ignoring join exclusion for junction '%' since it already occurred in a list of nodes to be joined."), nodeID);
    //         } else {
    //             myJoinExclusions.insert(nodeID);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_join_exclusion(&mut self, ids: &Vec<String>) {
        for nodeID in ids {
            // error handling has to take place here since joinExclusions could be
            // loaded from multiple files / command line
            if self.myJoined.contains(nodeID) {
                write_warningf!("Ignoring join exclusion for junction '%' since it already occurred in a list of nodes to be joined.", nodeID);
            } else {
                self.myJoinExclusions.insert(nodeID.clone());
            }
        }
    }

    /// C++: `std::string NBNodeCont::createUnusedID(const std::string &, const std::string &)`
    /// if base is an existing node id, find a unused id of the form base + sep + INT
    // std::string
    // NBNodeCont::createUnusedID(const std::string& base, const std::string& sep) {
    //     std::string result = base;
    //     int i = 0;
    //     while (myNodes.find(result) != myNodes.end()) {
    //         result = base + sep + toString(i++);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn create_unused_id(&mut self, base: &str, sep: &str) -> String {
        let mut result = base.to_string();
        let mut i = 0;
        while self.myNodes.contains_key(&result) {
            result = format!("{}{}{}", base, sep, i);
            i += 1;
        }
        result
    }

    /// C++: `std::string NBNodeCont::createClusterId(const NodeSet &, const std::string &)`
    /// generate id from cluster node ids
    // std::string createClusterId(const NodeSet& cluster, const std::string& prefix = "cluster_") {
    //         std::set<std::string> clusterIds;
    //         for (NBNode* j : cluster) {
    //             clusterIds.insert(j->getID());
    //         }
    //         return createClusterId(clusterIds, prefix);
    //     }
    #[allow(unused_variables)]
    pub fn create_cluster_id(&mut self, cluster: &NodeSet, prefix: &str) -> String {
        let clusterIds: BTreeSet<String> = cluster.iter().map(|j| j.get_id().to_string()).collect();
        self.create_cluster_id_2(&clusterIds, prefix)
    }

    /// C++: `std::string NBNodeCont::createClusterId(const std::set<std::string> &, const std::string &)`
    /// generate id from cluster node ids
    // std::string
    // NBNodeCont::createClusterId(const std::set<std::string>& cluster, const std::string& prefix) {
    //     int maxIds = OptionsCont::getOptions().getInt("max-join-ids");
    //     if (maxIds <= 0) {
    //         maxIds = (int)cluster.size();
    //     }
    //     if ((int)cluster.size() > maxIds) {
    //         auto clusterIt = cluster.begin();
    //         std::string result = prefix + *clusterIt;
    //         for (int i = 1; i < maxIds; i++) {
    //             ++clusterIt;
    //             result += "_" + *clusterIt;
    //         }
    //         return result + "_#" + toString((int)cluster.size() - maxIds) + "more";
    //     }
    //     return prefix + joinToString(cluster, "_");
    // }
    #[allow(unused_variables)]
    pub fn create_cluster_id_2(&mut self, cluster: &BTreeSet<String>, prefix: &str) -> String {
        let mut maxIds = utils::options::options_cont::get_int("max-join-ids");
        if maxIds <= 0 {
            maxIds = cluster.len() as i32;
        }
        if cluster.len() as i32 > maxIds {
            let mut clusterIt = cluster.iter();
            let mut result = format!("{}{}", prefix, clusterIt.next().unwrap());
            for _i in 1..maxIds {
                result += &format!("_{}", clusterIt.next().unwrap());
            }
            return format!("{}_#{}more", result, cluster.len() as i32 - maxIds);
        }
        format!("{}{}", prefix, join_to_string(cluster, "_"))
    }

    /// C++: `void NBNodeCont::addCluster2Join(const std::set<std::string> &, NBNode *, const bool)`
    /// add ids of nodes which shall be joined into a single node
    // void
    // NBNodeCont::addCluster2Join(const std::set<std::string>& cluster, NBNode* node, const bool resetConnections) {
    //     // error handling has to take place here since joins could be loaded from multiple files
    //     JoinCluster join{{}, node, resetConnections};
    //     for (std::string nodeID : cluster) {
    //         if (myJoinExclusions.count(nodeID) > 0) {
    //             WRITE_WARNINGF(TL("Ignoring join-cluster because junction '%' was already excluded from joining."), nodeID);
    //             return;
    //         } else if (myJoined.count(nodeID) > 0) {
    //             WRITE_WARNINGF(TL("Ignoring join-cluster because junction '%' already occurred in another join-cluster."), nodeID);
    //             return;
    //         } else {
    //             if (retrieve(nodeID) != nullptr) {
    //                 join.cluster.insert(nodeID);
    //             } else {
    //                 WRITE_ERRORF(TL("Unknown junction '%' in join-cluster."), nodeID);
    //             }
    //         }
    //     }
    //     if (join.cluster.size() > 1) {
    //         myJoined.insert(join.cluster.begin(), join.cluster.end());
    //         myClusters2Join.emplace_back(join);
    //     } else {
    //         WRITE_WARNINGF(TL("Ignoring join-cluster '%' because it has size '%'."), node->getID(), join.cluster.size());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_cluster2_join(&mut self, cluster: &BTreeSet<String>, node: Ptr<NBNode>, resetConnections: bool) {
        // error handling has to take place here since joins could be loaded from multiple files
        let mut join = NBNodeCont_JoinCluster { cluster: BTreeSet::new(), node, resetConnections };
        for nodeID in cluster {
            if self.myJoinExclusions.contains(nodeID) {
                write_warningf!("Ignoring join-cluster because junction '%' was already excluded from joining.", nodeID);
                return;
            } else if self.myJoined.contains(nodeID) {
                write_warningf!("Ignoring join-cluster because junction '%' already occurred in another join-cluster.", nodeID);
                return;
            } else if !self.retrieve(nodeID).is_null() {
                join.cluster.insert(nodeID.clone());
            } else {
                write_errorf!("Unknown junction '%' in join-cluster.", nodeID);
            }
        }
        if join.cluster.len() > 1 {
            self.myJoined.extend(join.cluster.iter().cloned());
            self.myClusters2Join.push(join);
        } else {
            write_warningf!("Ignoring join-cluster '%' because it has size '%'.", node.get_id(), join.cluster.len());
        }
    }

    /// C++: `int NBNodeCont::joinLoadedClusters(NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &)`
    /// Joins loaded junction clusters (see NIXMLNodesHandler)
    // int
    // NBNodeCont::joinLoadedClusters(NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tlc) {
    //     int numJoined = 0;
    //     for (JoinCluster& item : myClusters2Join) {
    //         // verify loaded cluster
    //         NodeSet cluster;
    //         for (std::string nodeID : item.cluster) {
    //             NBNode* node = retrieve(nodeID);
    //             if (node == nullptr) {
    //                 WRITE_ERRORF(TL("unknown junction '%' while joining."), nodeID);
    //             } else {
    //                 cluster.insert(node);
    //             }
    //         }
    //         if (cluster.size() > 1) {
    //             joinNodeCluster(cluster, dc, ec, tlc, item.node, item.resetConnections);
    //             numJoined++;
    //             myJoinExclusions.insert(item.node->getID());
    //         }
    //     }
    //     myClusters2Join.clear(); // make save for recompute
    //     return numJoined;
    // }
    #[allow(unused_variables)]
    pub fn join_loaded_clusters(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont) -> i32 {
        let mut numJoined = 0;
        // C++ iterates myClusters2Join by reference while joinNodeCluster runs; index loop avoids aliasing &mut self
        for i in 0..self.myClusters2Join.len() {
            let cluster_ids = self.myClusters2Join[i].cluster.clone();
            let itemNode = self.myClusters2Join[i].node;
            let itemReset = self.myClusters2Join[i].resetConnections;
            // verify loaded cluster
            let mut cluster: NodeSet = NodeSet::new();
            for nodeID in cluster_ids.iter() {
                let node = self.retrieve(nodeID);
                if node.is_null() {
                    write_errorf!("unknown junction '%' while joining.", nodeID);
                } else {
                    cluster.insert(node);
                }
            }
            if cluster.len() > 1 {
                self.join_node_cluster(cluster, dc, ec, tlc, itemNode, itemReset);
                numJoined += 1;
                self.myJoinExclusions.insert(itemNode.get_id().to_string());
            }
        }
        self.myClusters2Join.clear(); // make save for recompute
        numJoined
    }

    /// C++: `int NBNodeCont::joinJunctions(double, NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &, NBPTStopCont &)`
    /// Joins junctions that are very close together
    // int
    // NBNodeCont::joinJunctions(double maxDist, NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tlc, NBPTStopCont& sc) {
    // #ifdef DEBUG_JOINJUNCTIONS
    //     std::cout << "joinJunctions...\n";
    // #endif
    //     NodeClusters cands;
    //     NodeClusters clusters;
    //     std::map<const NBNode*, std::vector<NBNode*> > ptStopEnds;
    //     // check for stop edges within the cluster
    //     for (const auto& stopIt : sc.getStops()) {
    //         NBEdge* edge = ec.retrieve(stopIt.second->getEdgeId());
    //         if (edge != nullptr) {
    //             ptStopEnds[edge->getFromNode()].push_back(edge->getToNode());
    //         }
    //     }
    //     generateNodeClusters(maxDist, cands);
    //     for (NodeSet& cluster : cands) {
    // #ifdef DEBUG_JOINJUNCTIONS
    //         gDebugFlag1 = false;
    //         for (NBNode* n : cluster) {
    //             if (DEBUGCOND(n)) {
    //                 gDebugFlag1 = true;
    //             }
    //         }
    // #endif
    //         // remove join exclusions
    //         for (NodeSet::iterator j = cluster.begin(); j != cluster.end();) {
    //             NodeSet::iterator check = j;
    //             ++j;
    //             if (myJoinExclusions.count((*check)->getID()) > 0) {
    //                 cluster.erase(check);
    //             }
    //         }
    //         std::string origCluster = joinNamedToString(cluster, ',');
    //         // remove nodes that can be eliminated by geometry.remove
    //         pruneClusterFringe(cluster, maxDist);
    //         if (cluster.size() < 2) {
    //             continue;
    //         }
    //         // remove nodes that are part of a bypass lane (typically for turning right without waiting at a traffic light)
    //         pruneSlipLaneNodes(cluster, maxDist);
    //         if (cluster.size() < 2) {
    //             WRITE_WARNINGF(TL("Not joining junctions % (%)."), origCluster, "slip lane");
    //             continue;
    //         }
    //         origCluster = joinNamedToString(cluster, ',');
    //         NBNode* tryRemove = nullptr;
    //         std::string reason;
    //         std::string origReason;
    //         // pruneLongEdges might remove too much, so we check first to have a fallback with the circles
    //         bool feasible = feasibleCluster(cluster, ptStopEnds, maxDist, origReason, tryRemove);
    //         if (feasible && ((int)cluster.size() - pruneLongEdges(cluster, maxDist, true) < 2)) {
    //             origReason = "long edge";
    //             feasible = false;
    //         }
    //         if (!feasible) {
    // #ifdef DEBUG_JOINJUNCTIONS
    //             if (gDebugFlag1) {
    //                 std::cout << "   try to reduce to 4-circle nodes=" << joinNamedToString(cluster, ',') << "\n";
    //             }
    // #endif
    //             if (reduceToCircle(cluster, 4, cluster, maxDist)) {
    //                 feasible = feasibleCluster(cluster, ptStopEnds, maxDist, reason, tryRemove);
    //                 if (feasible) {
    //                     WRITE_WARNINGF(TL("Reducing junction cluster % (%)."), origCluster, origReason);
    //                 }
    //             }
    //         }
    //         if (!feasible) {
    // #ifdef DEBUG_JOINJUNCTIONS
    //             if (gDebugFlag1) {
    //                 std::cout << "   try to reduce to 2-circle nodes=" << joinNamedToString(cluster, ',') << "\n";
    //             }
    // #endif
    //             if (reduceToCircle(cluster, 2, cluster, maxDist)) {
    //                 feasible = feasibleCluster(cluster, ptStopEnds, maxDist, reason, tryRemove);
    //                 if (feasible) {
    //                     WRITE_WARNINGF(TL("Reducing junction cluster % (%)."), origCluster, origReason);
    //                 }
    //             }
    //         }
    //         while (!feasible && tryRemove != nullptr) {
    //             cluster.erase(tryRemove);
    //             pruneClusterFringe(cluster, maxDist);
    //             tryRemove = nullptr;
    //             feasible = feasibleCluster(cluster, ptStopEnds, maxDist, reason, tryRemove);
    //             if (feasible) {
    //                 WRITE_WARNINGF(TL("Reducing junction cluster % (%)."), origCluster, origReason);
    //             }
    //         }
    //         if (cluster.size() < 2) {
    //             WRITE_WARNINGF(TL("Not joining junctions % (%)."), origCluster, "after reduction");
    //             continue;
    //         }
    //         // avoid removal of long edges (must have been added via an alternative path).
    //         const int numPruned = pruneLongEdges(cluster, maxDist);
    //         if (cluster.size() < 2) {
    //             WRITE_WARNINGF(TL("Not joining junctions % (%)."), origCluster, "long edge");
    //             continue;
    //         }
    //         // after pruning long edges we have to recheck
    //         if (numPruned > 0) {
    //             pruneClusterFringe(cluster, maxDist);
    //             if (cluster.size() < 2) {
    //                 WRITE_WARNINGF(TL("Not joining junctions % (%)."), origCluster, "long edge");
    //                 continue;
    //             }
    //             pruneSlipLaneNodes(cluster, maxDist);
    //             if (cluster.size() < 2) {
    //                 WRITE_WARNINGF(TL("Not joining junctions % (%)."), origCluster, "slip lane");
    //                 continue;
    //             }
    //         }
    //         feasible = feasibleCluster(cluster, ptStopEnds, maxDist, origReason, tryRemove);
    //         if (!feasible) {
    //             WRITE_WARNINGF(TL("Not joining junctions % (%)."), origCluster, origReason);
    //             continue;
    //         }
    //         // compute all connected components of this cluster
    //         // (may be more than 1 if intermediate nodes were removed)
    //         NodeClusters components;
    //         for (NBNode* current : cluster) {
    //             // merge all connected components into newComp
    //             NodeSet newComp;
    //             //std::cout << "checking connectivity for " << current->getID() << "\n";
    //             newComp.insert(current);
    //             for (NodeClusters::iterator it_comp = components.begin(); it_comp != components.end();) {
    //                 NodeClusters::iterator check = it_comp;
    //                 //std::cout << "   connected with " << toString(*check) << "?\n";
    //                 bool connected = false;
    //                 for (NBNode* k : *check) {
    //                     if (current->getConnectionTo(k) != nullptr || k->getConnectionTo(current) != nullptr) {
    //                         //std::cout << "joining with connected component " << toString(*check) << "\n";
    //                         newComp.insert((*check).begin(), (*check).end());
    //                         it_comp = components.erase(check);
    //                         connected = true;
    //                         break;
    //                     }
    //                 }
    //                 if (!connected) {
    //                     it_comp++;
    //                 }
    //             }
    //             //std::cout << "adding new component " << toString(newComp) << "\n";
    //             components.push_back(newComp);
    //         }
    //         for (NodeClusters::iterator it_comp = components.begin(); it_comp != components.end(); ++it_comp) {
    //             if ((*it_comp).size() > 1) {
    //                 //std::cout << "adding cluster " << toString(*it_comp) << "\n";
    //                 clusters.push_back(*it_comp);
    //             }
    //         }
    // #ifdef DEBUG_JOINJUNCTIONS
    //         gDebugFlag1 = false;
    // #endif
    //     }
    //     joinNodeClusters(clusters, dc, ec, tlc);
    //     return (int)clusters.size();
    // }
    #[allow(unused_variables)]
    pub fn join_junctions(&mut self, maxDist: f64, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, sc: &mut NBPTStopCont) -> i32 {
        let mut cands: NBNodeCont_NodeClusters = Vec::new();
        let mut clusters: NBNodeCont_NodeClusters = Vec::new();
        let mut ptStopEnds: BTreeMap<Ptr<NBNode>, Vec<Ptr<NBNode>>> = BTreeMap::new();
        // check for stop edges within the cluster
        for stop in sc.get_stops().values() {
            let edge = ec.retrieve(stop.get_edge_id(), false);
            if !edge.is_null() {
                ptStopEnds.entry(edge.get_from_node()).or_default().push(edge.get_to_node());
            }
        }
        self.generate_node_clusters(maxDist, &mut cands);
        for cluster in cands.iter_mut() {
            // remove join exclusions
            cluster.retain(|n| !self.myJoinExclusions.contains(n.get_id()));
            let mut origCluster = join_node_ids(cluster, ",");
            // remove nodes that can be eliminated by geometry.remove
            self.prune_cluster_fringe(cluster, maxDist, false);
            if cluster.len() < 2 {
                continue;
            }
            // remove nodes that are part of a bypass lane (typically for turning right without waiting at a traffic light)
            self.prune_slip_lane_nodes(cluster, maxDist);
            if cluster.len() < 2 {
                write_warningf!("Not joining junctions % (%).", origCluster, "slip lane");
                continue;
            }
            origCluster = join_node_ids(cluster, ",");
            let mut tryRemove: Ptr<NBNode> = Ptr::null();
            let mut reason = String::new();
            let mut origReason = String::new();
            // pruneLongEdges might remove too much, so we check first to have a fallback with the circles
            let mut feasible = self.feasible_cluster(cluster, &ptStopEnds, maxDist, &mut origReason, &mut tryRemove);
            if feasible && ((cluster.len() as i32) - Self::prune_long_edges(cluster, maxDist, true) < 2) {
                origReason = "long edge".to_string();
                feasible = false;
            }
            if !feasible {
                let start = cluster.clone();
                if self.reduce_to_circle(cluster, 4, start, maxDist, Vec::new()) {
                    feasible = self.feasible_cluster(cluster, &ptStopEnds, maxDist, &mut reason, &mut tryRemove);
                    if feasible {
                        write_warningf!("Reducing junction cluster % (%).", origCluster, origReason);
                    }
                }
            }
            if !feasible {
                let start = cluster.clone();
                if self.reduce_to_circle(cluster, 2, start, maxDist, Vec::new()) {
                    feasible = self.feasible_cluster(cluster, &ptStopEnds, maxDist, &mut reason, &mut tryRemove);
                    if feasible {
                        write_warningf!("Reducing junction cluster % (%).", origCluster, origReason);
                    }
                }
            }
            while !feasible && !tryRemove.is_null() {
                cluster.remove(&tryRemove);
                self.prune_cluster_fringe(cluster, maxDist, false);
                tryRemove = Ptr::null();
                feasible = self.feasible_cluster(cluster, &ptStopEnds, maxDist, &mut reason, &mut tryRemove);
                if feasible {
                    write_warningf!("Reducing junction cluster % (%).", origCluster, origReason);
                }
            }
            if cluster.len() < 2 {
                write_warningf!("Not joining junctions % (%).", origCluster, "after reduction");
                continue;
            }
            // avoid removal of long edges (must have been added via an alternative path).
            let numPruned = Self::prune_long_edges(cluster, maxDist, false);
            if cluster.len() < 2 {
                write_warningf!("Not joining junctions % (%).", origCluster, "long edge");
                continue;
            }
            // after pruning long edges we have to recheck
            if numPruned > 0 {
                self.prune_cluster_fringe(cluster, maxDist, false);
                if cluster.len() < 2 {
                    write_warningf!("Not joining junctions % (%).", origCluster, "long edge");
                    continue;
                }
                self.prune_slip_lane_nodes(cluster, maxDist);
                if cluster.len() < 2 {
                    write_warningf!("Not joining junctions % (%).", origCluster, "slip lane");
                    continue;
                }
            }
            feasible = self.feasible_cluster(cluster, &ptStopEnds, maxDist, &mut origReason, &mut tryRemove);
            if !feasible {
                write_warningf!("Not joining junctions % (%).", origCluster, origReason);
                continue;
            }
            // compute all connected components of this cluster
            // (may be more than 1 if intermediate nodes were removed)
            let mut components: NBNodeCont_NodeClusters = Vec::new();
            for current in nodes_by_id(cluster) {
                // merge all connected components into newComp
                let mut newComp: NodeSet = NodeSet::new();
                newComp.insert(current);
                let mut idx = 0;
                while idx < components.len() {
                    let connected = components[idx].iter().any(|k| {
                        !current.get_connection_to(*k).is_null() || !k.get_connection_to(current).is_null()
                    });
                    if connected {
                        let comp = components.remove(idx);
                        newComp.extend(comp);
                    } else {
                        idx += 1;
                    }
                }
                components.push(newComp);
            }
            for comp in components {
                if comp.len() > 1 {
                    clusters.push(comp);
                }
            }
        }
        let numClusters = clusters.len() as i32;
        self.join_node_clusters(clusters, dc, ec, tlc, false);
        numClusters
    }

    /// C++: `int NBNodeCont::joinSameJunctions(NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &, double)`
    /// Joins junctions with similar coordinates regardless of topology
    // int
    // NBNodeCont::joinSameJunctions(NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tlc, double maxDist) {
    // #ifdef DEBUG_JOINJUNCTIONS
    //     std::cout << "joinSameJunctions...\n";
    // #endif
    //     std::set<NBNode*> checked;
    //     NodeClusters clusters;
    //     for (auto& item : myNodes) {
    //         NBNode* n = item.second;
    //         if (myJoinExclusions.count(item.first) > 0) {
    //             continue;
    //         }
    //         std::vector<NBNode*> nearby = retrieveByPos(n->getPosition(), maxDist);
    //         NodeSet cluster;
    //         for (NBNode* n2 : nearby) {
    //             if (myJoinExclusions.count(n2->getID()) == 0 && checked.count(n2) == 0) {
    //                 cluster.insert(n2);
    //             }
    //         }
    //         if (cluster.size() > 1) {
    //             checked.insert(cluster.begin(), cluster.end());
    //             clusters.push_back(cluster);
    //         }
    //     }
    //     joinNodeClusters(clusters, dc, ec, tlc, true);
    //     return (int)clusters.size();
    // }
    #[allow(unused_variables)]
    pub fn join_same_junctions(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, maxDist: f64) -> i32 {
        let mut checked: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let mut clusters: NBNodeCont_NodeClusters = Vec::new();
        for (id, n) in self.myNodes.iter() {
            let n = *n;
            if self.myJoinExclusions.contains(id) {
                continue;
            }
            let nearby = self.retrieve_by_pos(n.get_position(), maxDist);
            let mut cluster: NodeSet = NodeSet::new();
            for n2 in nearby {
                if !self.myJoinExclusions.contains(n2.get_id()) && !checked.contains(&n2) {
                    cluster.insert(n2);
                }
            }
            if cluster.len() > 1 {
                checked.extend(cluster.iter().copied());
                clusters.push(cluster);
            }
        }
        let numClusters = clusters.len() as i32;
        self.join_node_clusters(clusters, dc, ec, tlc, true);
        numClusters
    }

    /// C++: `NodeSet NBNodeCont::getClusterNeighbors(const NBNode *, double, NodeSet &)` [static]
    /// return all cluster neighbors for the given node
    // NodeSet
    // NBNodeCont::getClusterNeighbors(const NBNode* n, double longThreshold, NodeSet& cluster) {
    //     NodeSet result;
    //     for (NBEdge* e : n->getEdges()) {
    //         if (e->getLength() > longThreshold) {
    //             continue;
    //         }
    //         NBNode* neighbor = e->getFromNode() == n ? e->getToNode() : e->getFromNode();
    //         if (cluster.count(neighbor) != 0) {
    //             result.insert(neighbor);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_cluster_neighbors(n: Ptr<NBNode>, longThreshold: f64, cluster: &mut NodeSet) -> NodeSet {
        let mut result: NodeSet = NodeSet::new();
        for e in n.get_edges() {
            if e.get_length() > longThreshold {
                continue;
            }
            let neighbor = if e.get_from_node() == n { e.get_to_node() } else { e.get_from_node() };
            if cluster.contains(&neighbor) {
                result.insert(neighbor);
            }
        }
        result
    }

    /// C++: `bool NBNodeCont::isSlipLaneContinuation(const NBNode *)` [static]
    /// whether the given node may continue a slip lane
    // bool
    // NBNodeCont::isSlipLaneContinuation(const NBNode* cont) {
    //     return cont->getPassengerEdges(true).size() == 1 && cont->getPassengerEdges(false).size() == 1;
    // }
    #[allow(unused_variables)]
    pub fn is_slip_lane_continuation(cont: Ptr<NBNode>) -> bool {
        cont.get_passenger_edges(true).len() == 1 && cont.get_passenger_edges(false).len() == 1
    }

    /// C++: `bool NBNodeCont::maybeSlipLaneStart(const NBNode *, EdgeVector &, double &) const` [const]
    /// check whether the given node maybe the start of a slip lane
    // bool
    // NBNodeCont::maybeSlipLaneStart(const NBNode* n, EdgeVector& outgoing, double& inAngle) const {
    //     EdgeVector inPE = n->getPassengerEdges(true);
    //     EdgeVector outPE = n->getPassengerEdges(false);
    //     if (inPE.size() == 1 && outPE.size() == 2) {
    //         outgoing.insert(outgoing.begin(), outPE.begin(), outPE.end());
    //         inAngle = inPE.front()->getAngleAtNode(n);
    //         return true;
    //     } else if (inPE.size() >= 2 && outPE.size() == 3) {
    //         // check if the incoming edges are going in opposite directions and then
    //         // use the incoming edge that has 2 almost-straight outgoing edges
    //         const double inRelAngle = fabs(NBHelpers::relAngle(inPE.front()->getAngleAtNode(n), inPE.back()->getAngleAtNode(n)));
    //         //std::cout << "n=" << n->getID() << " inRelAngle=" << inRelAngle << "\n";
    //         if (inRelAngle < 135) {
    //             return false; // not opposite incoming
    //         }
    //         for (NBEdge* in : inPE) {
    //             EdgeVector straight;
    //             int numReverse = 0;
    //             for (NBEdge* out : outPE) {
    //                 const double outRelAngle = fabs(NBHelpers::relAngle(in->getAngleAtNode(n), out->getAngleAtNode(n)));
    //                 if (outRelAngle <= 45) {
    //                     straight.push_back(out);
    //                 } else if (outRelAngle >= 135) {
    //                     numReverse++;
    //                 }
    //             }
    //             if (straight.size() == 2 && numReverse == 1) {
    //                 outgoing.insert(outgoing.begin(), straight.begin(), straight.end());
    //                 inAngle = in->getAngleAtNode(n);
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn maybe_slip_lane_start(&self, n: Ptr<NBNode>, outgoing: &mut EdgeVector, inAngle: &mut f64) -> bool {
        let inPE: EdgeVector = n.get_passenger_edges(true);
        let outPE: EdgeVector = n.get_passenger_edges(false);
        if inPE.len() == 1 && outPE.len() == 2 {
            outgoing.splice(0..0, outPE.iter().copied());
            *inAngle = inPE[0].get_angle_at_node(n);
            return true;
        } else if inPE.len() >= 2 && outPE.len() == 3 {
            // check if the incoming edges are going in opposite directions and then
            // use the incoming edge that has 2 almost-straight outgoing edges
            let inRelAngle = NBHelpers::rel_angle(inPE[0].get_angle_at_node(n), inPE[inPE.len() - 1].get_angle_at_node(n)).abs();
            if inRelAngle < 135.0 {
                return false; // not opposite incoming
            }
            for inE in inPE.iter() {
                let mut straight: EdgeVector = Vec::new();
                let mut numReverse = 0;
                for out in outPE.iter() {
                    let outRelAngle = NBHelpers::rel_angle(inE.get_angle_at_node(n), out.get_angle_at_node(n)).abs();
                    if outRelAngle <= 45.0 {
                        straight.push(*out);
                    } else if outRelAngle >= 135.0 {
                        numReverse += 1;
                    }
                }
                if straight.len() == 2 && numReverse == 1 {
                    outgoing.splice(0..0, straight.iter().copied());
                    *inAngle = inE.get_angle_at_node(n);
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool NBNodeCont::maybeSlipLaneEnd(const NBNode *, EdgeVector &, double &) const` [const]
    /// check whether the given node maybe the end of a slip lane
    // bool
    // NBNodeCont::maybeSlipLaneEnd(const NBNode* n, EdgeVector& incoming, double& outAngle) const {
    //     EdgeVector inPE = n->getPassengerEdges(true);
    //     EdgeVector outPE = n->getPassengerEdges(false);
    //     if (inPE.size() == 2 && outPE.size() == 1) {
    //         incoming.insert(incoming.begin(), inPE.begin(), inPE.end());
    //         outAngle = outPE.front()->getAngleAtNode(n);
    //         return true;
    //     } else if (inPE.size() == 3 && outPE.size() >= 2) {
    //         // check if the outgoing edges are going in opposite directions and then
    //         // use the outgoing edge that has 2 almost-straight incoming edges
    //         const double outRelAngle = fabs(NBHelpers::relAngle(outPE.front()->getAngleAtNode(n), outPE.back()->getAngleAtNode(n)));
    //         //std::cout << "n=" << n->getID() << " outRelAngle=" << outRelAngle << "\n";
    //         if (outRelAngle < 135) {
    //             return false; // not opposite outgoing
    //         }
    //         for (NBEdge* out : outPE) {
    //             EdgeVector straight;
    //             int numReverse = 0;
    //             for (NBEdge* in : inPE) {
    //                 const double inRelAngle = fabs(NBHelpers::relAngle(in->getAngleAtNode(n), out->getAngleAtNode(n)));
    //                 if (inRelAngle <= 45) {
    //                     straight.push_back(in);
    //                 } else if (inRelAngle >= 135) {
    //                     numReverse++;
    //                 }
    //             }
    //             if (straight.size() == 2 && numReverse == 1) {
    //                 incoming.insert(incoming.begin(), straight.begin(), straight.end());
    //                 outAngle = out->getAngleAtNode(n);
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn maybe_slip_lane_end(&self, n: Ptr<NBNode>, incoming: &mut EdgeVector, outAngle: &mut f64) -> bool {
        let inPE: EdgeVector = n.get_passenger_edges(true);
        let outPE: EdgeVector = n.get_passenger_edges(false);
        if inPE.len() == 2 && outPE.len() == 1 {
            incoming.splice(0..0, inPE.iter().copied());
            *outAngle = outPE[0].get_angle_at_node(n);
            return true;
        } else if inPE.len() == 3 && outPE.len() >= 2 {
            // check if the outgoing edges are going in opposite directions and then
            // use the outgoing edge that has 2 almost-straight incoming edges
            let outRelAngle = NBHelpers::rel_angle(outPE[0].get_angle_at_node(n), outPE[outPE.len() - 1].get_angle_at_node(n)).abs();
            if outRelAngle < 135.0 {
                return false; // not opposite outgoing
            }
            for out in outPE.iter() {
                let mut straight: EdgeVector = Vec::new();
                let mut numReverse = 0;
                for inE in inPE.iter() {
                    let inRelAngle = NBHelpers::rel_angle(inE.get_angle_at_node(n), out.get_angle_at_node(n)).abs();
                    if inRelAngle <= 45.0 {
                        straight.push(*inE);
                    } else if inRelAngle >= 135.0 {
                        numReverse += 1;
                    }
                }
                if straight.len() == 2 && numReverse == 1 {
                    incoming.splice(0..0, straight.iter().copied());
                    *outAngle = out.get_angle_at_node(n);
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool NBNodeCont::reduceToCircle(NodeSet &, int, NodeSet, double, std::vector<NBNode *>) const` [const]
    /// try to find a joinable subset (recursively)
    // bool
    // NBNodeCont::reduceToCircle(NodeSet& cluster, int circleSize, NodeSet startNodes, double maxDist, std::vector<NBNode*> cands) const {
    // #ifdef DEBUG_REDUCE
    //     std::cout << "reduceToCircle  cs=" << circleSize << " cands=" << toString(cands, ',') << " startNodes=" << joinNamedToString(startNodes, ',') << "\n";
    // #endif
    //     assert(circleSize >= 2);
    //     if ((int)cands.size() == circleSize) {
    //         if (cands.back()->getConnectionTo(cands.front()) != nullptr) {
    //             // cluster found
    //             NodeSet candCluster;
    //             candCluster.insert(cands.begin(), cands.end());
    //             pruneClusterFringe(candCluster, maxDist, true);
    //             bool feasible = (int)candCluster.size() == circleSize;
    //             if (feasible) {
    //                 cluster.clear();
    //                 cluster.insert(cands.begin(), cands.end());
    //             }
    //             return feasible;
    //         } else {
    //             return false;
    //         }
    //     }
    //     if ((int)cluster.size() <= circleSize || startNodes.size() == 0) {
    //         // no reduction possible
    // #ifdef DEBUG_REDUCE
    //         std::cout << "    abort\n";
    // #endif
    //         return false;
    //     }
    //     if (cands.size() == 0) {
    //         // try to find a circle starting from another start node
    //         NBEdge* e = shortestEdge(cluster, startNodes, cands);
    //         if (e != nullptr) {
    //             cands.push_back(e->getFromNode());
    //             startNodes.erase(e->getFromNode());
    //             if (reduceToCircle(cluster, circleSize, startNodes, maxDist, cands)) {
    //                 return true;
    //             } else {
    //                 // try another start node
    //                 return reduceToCircle(cluster, circleSize, startNodes, maxDist);
    //             }
    //         }
    //     } else {
    //         NodeSet singleStart;
    //         singleStart.insert(cands.back());
    //         NBEdge* e = shortestEdge(cluster, singleStart, cands);
    //         if (e != nullptr) {
    //             std::vector<NBNode*> cands2(cands);
    //             cands2.push_back(e->getToNode());
    //             if (reduceToCircle(cluster, circleSize, startNodes, maxDist, cands2)) {
    //                 return true;
    //             }
    //         }
    //     }
    // #ifdef DEBUG_REDUCE
    //     std::cout << "    abort2\n";
    // #endif
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn reduce_to_circle(&self, cluster: &mut NodeSet, circleSize: i32, startNodes: NodeSet, maxDist: f64, cands: Vec<Ptr<NBNode>>) -> bool {
        debug_assert!(circleSize >= 2);
        if cands.len() as i32 == circleSize {
            if !cands[cands.len() - 1].get_connection_to(cands[0]).is_null() {
                // cluster found
                let mut candCluster: NodeSet = NodeSet::new();
                candCluster.extend(cands.iter().copied());
                self.prune_cluster_fringe(&mut candCluster, maxDist, true);
                let feasible = candCluster.len() as i32 == circleSize;
                if feasible {
                    cluster.clear();
                    cluster.extend(cands.iter().copied());
                }
                return feasible;
            } else {
                return false;
            }
        }
        if cluster.len() as i32 <= circleSize || startNodes.is_empty() {
            // no reduction possible
            return false;
        }
        let mut startNodes = startNodes;
        let mut cands = cands;
        if cands.is_empty() {
            // try to find a circle starting from another start node
            let e = self.shortest_edge(cluster, &startNodes, &cands);
            if !e.is_null() {
                cands.push(e.get_from_node());
                startNodes.remove(&e.get_from_node());
                if self.reduce_to_circle(cluster, circleSize, startNodes.clone(), maxDist, cands.clone()) {
                    return true;
                } else {
                    // try another start node
                    return self.reduce_to_circle(cluster, circleSize, startNodes, maxDist, Vec::new());
                }
            }
        } else {
            let mut singleStart: NodeSet = NodeSet::new();
            singleStart.insert(cands[cands.len() - 1]);
            let e = self.shortest_edge(cluster, &singleStart, &cands);
            if !e.is_null() {
                let mut cands2 = cands.clone();
                cands2.push(e.get_to_node());
                if self.reduce_to_circle(cluster, circleSize, startNodes, maxDist, cands2) {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `NBEdge * NBNodeCont::shortestEdge(const NodeSet &, const NodeSet &, const std::vector<NBNode *> &) const` [const]
    /// find closest neighbor for building circle
    // NBEdge*
    // NBNodeCont::shortestEdge(const NodeSet& cluster, const NodeSet& startNodes, const std::vector<NBNode*>& exclude) const {
    //     double minDist = std::numeric_limits<double>::max();
    //     NBEdge* result = nullptr;
    //     for (NBNode* n : startNodes) {
    //         for (NBEdge* e : n->getOutgoingEdges()) {
    //             NBNode* neigh = e->getToNode();
    //             if (cluster.count(neigh) != 0 && std::find(exclude.begin(), exclude.end(), neigh) == exclude.end()) {
    //                 const double dist = n->getPosition().distanceTo2D(neigh->getPosition());
    //                 //std::cout << "    e=" << e->getID() << " dist=" << dist << " minD=" << minDist << "\n";
    //                 if (dist < minDist) {
    //                     minDist = dist;
    //                     result = e;
    //                 }
    //             }
    //         }
    //     }
    //     //std::cout << "closestNeighbor startNodes=" << toString(startNodes) << " result=" << Named::getIDSecure(result) << "\n";
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn shortest_edge(&self, cluster: &NodeSet, startNodes: &NodeSet, exclude: &Vec<Ptr<NBNode>>) -> Ptr<NBEdge> {
        let mut minDist = f64::MAX;
        let mut result: Ptr<NBEdge> = Ptr::null();
        for n in nodes_by_id(startNodes) {
            for e in n.get_outgoing_edges() {
                let neigh = e.get_to_node();
                if cluster.contains(&neigh) && !exclude.contains(&neigh) {
                    let dist = n.get_position().distance_to2d(neigh.get_position());
                    if dist < minDist {
                        minDist = dist;
                        result = *e;
                    }
                }
            }
        }
        result
    }

    /// C++: `int NBNodeCont::removeSelfLoops(NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &)`
    /// Removes self-loop edges (edges where the source and the destination node are the same)
    // int
    // NBNodeCont::removeSelfLoops(NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tc) {
    //     int no = 0;
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         no += (*i).second->removeSelfLoops(dc, ec, tc);
    //     }
    //     if (no != 0) {
    //         WRITE_WARNING(toString(no) + " self-looping edge(s) removed.");
    //     }
    //     return no;
    // }
    #[allow(unused_variables)]
    pub fn remove_self_loops(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tc: &mut NBTrafficLightLogicCont) -> i32 {
        let mut no = 0;
        let nodes: Vec<Ptr<NBNode>> = self.myNodes.values().copied().collect();
        for node in nodes {
            no += node.m().remove_self_loops(dc, ec, tc);
        }
        if no != 0 {
            write_warning(&format!("{} self-looping edge(s) removed.", no));
        }
        no
    }

    /// C++: `void NBNodeCont::joinSimilarEdges(NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &, bool)`
    /// Joins edges connecting the same nodes
    // void
    // NBNodeCont::joinSimilarEdges(NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tlc, bool removeDuplicates) {
    //     // magic values
    //     const double distanceThreshold = 7.; // don't merge edges further apart
    //     const double lengthThreshold = 0.10; // don't merge edges with higher relative length-difference
    //
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         // count the edges to other nodes outgoing from the current node
    //         std::map<NBNode*, EdgeVector> connectionCount;
    //         const EdgeVector& outgoing = (*i).second->getOutgoingEdges();
    //         for (EdgeVector::const_iterator j = outgoing.begin(); j != outgoing.end(); j++) {
    //             connectionCount[(*j)->getToNode()].push_back(*j);
    //         }
    //         // check whether more than a single edge connect another node and join them
    //         std::map<NBNode*, EdgeVector>::iterator k;
    //         for (k = connectionCount.begin(); k != connectionCount.end(); k++) {
    //             // possibly we do not have anything to join...
    //             if ((*k).second.size() < 2) {
    //                 continue;
    //             }
    //             // for the edges that seem to be a single street,
    //             //  check whether the geometry is similar
    //             const EdgeVector& ev = (*k).second;
    //             const NBEdge* const first = ev.front();
    //             EdgeVector::const_iterator jci; // join candidate iterator
    //             for (jci = ev.begin() + 1; jci != ev.end(); ++jci) {
    //                 const double relativeLengthDifference = fabs(first->getLoadedLength() - (*jci)->getLoadedLength()) / first->getLoadedLength();
    //                 if ((!first->isNearEnough2BeJoined2(*jci, distanceThreshold)) ||
    //                         (relativeLengthDifference > lengthThreshold) ||
    //                         (fabs(first->getSpeed() - (*jci)->getSpeed()) >= 0.01) || // output accuracy
    //                         (first->getPermissions() != (*jci)->getPermissions())
    //                    ) {
    //                     break;
    //                 }
    //             }
    //             // @bug If there are 3 edges of which 2 can be joined, no joining will
    //             //   take place with the current implementation
    //             if (jci == ev.end()) {
    //                 if (removeDuplicates) {
    //                     for (int ei = 1; ei < (int)ev.size(); ei++) {
    //                         ec.extract(dc, ev[ei], true);
    //                     }
    //                 } else {
    //                     ec.joinSameNodeConnectingEdges(dc, tlc, ev);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join_similar_edges(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, removeDuplicates: bool) {
        // magic values
        let distanceThreshold = 7.; // don't merge edges further apart
        let lengthThreshold = 0.10; // don't merge edges with higher relative length-difference

        // determinism: C++ map<NBNode*, ...> is ordered by address; Ptr's Ord is used here
        let nodes: Vec<Ptr<NBNode>> = self.myNodes.values().copied().collect();
        for node in nodes {
            // count the edges to other nodes outgoing from the current node
            let mut connectionCount: BTreeMap<Ptr<NBNode>, EdgeVector> = BTreeMap::new();
            let outgoing: EdgeVector = node.get_outgoing_edges().clone();
            for e in outgoing {
                connectionCount.entry(e.get_to_node()).or_default().push(e);
            }
            // check whether more than a single edge connect another node and join them
            for (_, ev) in connectionCount {
                // possibly we do not have anything to join...
                if ev.len() < 2 {
                    continue;
                }
                // for the edges that seem to be a single street,
                //  check whether the geometry is similar
                let first = ev[0];
                let mut jci = 1;
                while jci < ev.len() {
                    let cand = ev[jci];
                    let relativeLengthDifference = (first.get_loaded_length() - cand.get_loaded_length()).abs() / first.get_loaded_length();
                    if (!first.is_near_enough2_be_joined2(cand, distanceThreshold))
                        || (relativeLengthDifference > lengthThreshold)
                        || ((first.get_speed() - cand.get_speed()).abs() >= 0.01) // output accuracy
                        || (first.get_permissions(-1) != cand.get_permissions(-1))
                    {
                        break;
                    }
                    jci += 1;
                }
                // @bug If there are 3 edges of which 2 can be joined, no joining will
                //   take place with the current implementation
                if jci == ev.len() {
                    if removeDuplicates {
                        for ei in 1..ev.len() {
                            ec.extract(dc, ev[ei], true);
                        }
                    } else {
                        ec.join_same_node_connecting_edges(dc, tlc, ev);
                    }
                }
            }
        }
    }

    /// C++: `void NBNodeCont::avoidOverlap()`
    /// fix overlap
    // void
    // NBNodeCont::avoidOverlap() {
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         (*i).second->avoidOverlap();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn avoid_overlap(&mut self) {
        let nodes: Vec<Ptr<NBNode>> = self.myNodes.values().copied().collect();
        for node in nodes {
            node.m().avoid_overlap();
        }
    }

    /// C++: `int NBNodeCont::removeIsolatedRoads(NBDistrictCont &, NBEdgeCont &)`
    /// Removes sequences of edges that are not connected with a junction. Simple roads without junctions sometimes remain when converting from OpenStreetMap, but they make no sense. Remaining empty nodes are also deleted.
    // int
    // NBNodeCont::removeIsolatedRoads(NBDistrictCont& dc, NBEdgeCont& ec) {
    //     int numRemovedEdges = 0;
    //     // Warn of isolated edges, i.e. a single edge with no connection to another edge
    //     const std::vector<std::string>& edgeNames = ec.getAllNames();
    //     for (std::vector<std::string>::const_iterator it = edgeNames.begin(); it != edgeNames.end(); ++it) {
    //         // Test whether this node starts at a dead end, i.e. it has only one adjacent node
    //         // to which an edge exists and from which an edge may come.
    //         NBEdge* e = ec.retrieve(*it);
    //         if (e == nullptr) {
    //             continue;
    //         }
    //         NBNode* from = e->getFromNode();
    //         const EdgeVector& outgoingEdges = from->getOutgoingEdges();
    //         if (outgoingEdges.size() != 1) {
    //             // At this node, several edges or no edge start; so, this node is no dead end.
    //             continue;
    //         }
    //         const EdgeVector& incomingEdges = from->getIncomingEdges();
    //         if (incomingEdges.size() > 1) {
    //             // At this node, several edges end; so, this node is no dead end.
    //             continue;
    //         } else if (incomingEdges.size() == 1) {
    //             NBNode* fromNodeOfIncomingEdge = incomingEdges[0]->getFromNode();
    //             NBNode* toNodeOfOutgoingEdge = outgoingEdges[0]->getToNode();
    //             if (fromNodeOfIncomingEdge != toNodeOfOutgoingEdge) {
    //                 // At this node, an edge ends which is not the inverse direction of
    //                 // the starting node.
    //                 continue;
    //             }
    //         }
    //         // Now we know that the edge e starts a dead end.
    //         // Next we test if the dead end is isolated, i.e. does not lead to a junction
    //         bool hasJunction = false;
    //         EdgeVector road;
    //         NBEdge* eOld = nullptr;
    //         NBNode* to;
    //         NodeSet adjacentNodes;
    //         do {
    //             road.push_back(e);
    //             eOld = e;
    //             from = e->getFromNode();
    //             to = e->getToNode();
    //             const EdgeVector& outgoingEdgesOfToNode = to->getOutgoingEdges();
    //             const EdgeVector& incomingEdgesOfToNode = to->getIncomingEdges();
    //             adjacentNodes.clear();
    //             for (EdgeVector::const_iterator itOfOutgoings = outgoingEdgesOfToNode.begin(); itOfOutgoings != outgoingEdgesOfToNode.end(); ++itOfOutgoings) {
    //                 if ((*itOfOutgoings)->getToNode() != from        // The back path
    //                         && (*itOfOutgoings)->getToNode() != to   // A loop / dummy edge
    //                    ) {
    //                     e = *itOfOutgoings; // Probably the next edge
    //                 }
    //                 adjacentNodes.insert((*itOfOutgoings)->getToNode());
    //             }
    //             for (EdgeVector::const_iterator itOfIncomings = incomingEdgesOfToNode.begin(); itOfIncomings != incomingEdgesOfToNode.end(); ++itOfIncomings) {
    //                 adjacentNodes.insert((*itOfIncomings)->getFromNode());
    //             }
    //             adjacentNodes.erase(to);  // Omit loops
    //             if (adjacentNodes.size() > 2) {
    //                 hasJunction = true;
    //             }
    //         } while (!hasJunction && eOld != e);
    //         if (!hasJunction) {
    //             std::string warningString;
    //             for (EdgeVector::iterator roadIt = road.begin(); roadIt != road.end(); ++roadIt) {
    //                 if (roadIt == road.begin()) {
    //                     warningString += (*roadIt)->getID();
    //                 } else {
    //                     warningString += "," + (*roadIt)->getID();
    //                 }
    //
    //                 NBNode* fromNode = (*roadIt)->getFromNode();
    //                 NBNode* toNode = (*roadIt)->getToNode();
    //                 ec.erase(dc, *roadIt);
    //                 numRemovedEdges++;
    //                 if (fromNode->getIncomingEdges().size() == 0 && fromNode->getOutgoingEdges().size() == 0) {
    //                     // Node is empty; can be removed
    //                     erase(fromNode);
    //                 }
    //                 if (toNode->getIncomingEdges().size() == 0 && toNode->getOutgoingEdges().size() == 0) {
    //                     // Node is empty; can be removed
    //                     erase(toNode);
    //                 }
    //             }
    //             WRITE_WARNINGF(TL("Removed a road without junctions: %."), warningString);
    //         }
    //     }
    //     return numRemovedEdges;
    // }
    #[allow(unused_variables)]
    pub fn remove_isolated_roads(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont) -> i32 {
        let mut numRemovedEdges = 0;
        // Warn of isolated edges, i.e. a single edge with no connection to another edge
        let edgeNames = ec.get_all_names();
        for name in edgeNames.iter() {
            // Test whether this node starts at a dead end, i.e. it has only one adjacent node
            // to which an edge exists and from which an edge may come.
            let mut e = ec.retrieve(name, false);
            if e.is_null() {
                continue;
            }
            let from = e.get_from_node();
            let outgoingEdges: EdgeVector = from.get_outgoing_edges().clone();
            if outgoingEdges.len() != 1 {
                // At this node, several edges or no edge start; so, this node is no dead end.
                continue;
            }
            let incomingEdges: EdgeVector = from.get_incoming_edges().clone();
            if incomingEdges.len() > 1 {
                // At this node, several edges end; so, this node is no dead end.
                continue;
            } else if incomingEdges.len() == 1 {
                let fromNodeOfIncomingEdge = incomingEdges[0].get_from_node();
                let toNodeOfOutgoingEdge = outgoingEdges[0].get_to_node();
                if fromNodeOfIncomingEdge != toNodeOfOutgoingEdge {
                    // At this node, an edge ends which is not the inverse direction of
                    // the starting node.
                    continue;
                }
            }
            // Now we know that the edge e starts a dead end.
            // Next we test if the dead end is isolated, i.e. does not lead to a junction
            let mut hasJunction = false;
            let mut road: EdgeVector = Vec::new();
            loop {
                road.push(e);
                let eOld = e;
                let from = e.get_from_node();
                let to = e.get_to_node();
                let outgoingEdgesOfToNode: EdgeVector = to.get_outgoing_edges().clone();
                let incomingEdgesOfToNode: EdgeVector = to.get_incoming_edges().clone();
                let mut adjacentNodes: NodeSet = NodeSet::new();
                for o in outgoingEdgesOfToNode.iter() {
                    if o.get_to_node() != from        // The back path
                        && o.get_to_node() != to       // A loop / dummy edge
                    {
                        e = *o; // Probably the next edge
                    }
                    adjacentNodes.insert(o.get_to_node());
                }
                for i in incomingEdgesOfToNode.iter() {
                    adjacentNodes.insert(i.get_from_node());
                }
                adjacentNodes.remove(&to); // Omit loops
                if adjacentNodes.len() > 2 {
                    hasJunction = true;
                }
                if hasJunction || eOld == e {
                    break;
                }
            }
            if !hasJunction {
                let mut warningString = String::new();
                for (idx, roadEdge) in road.iter().enumerate() {
                    if idx == 0 {
                        warningString += roadEdge.get_id();
                    } else {
                        warningString += &format!(",{}", roadEdge.get_id());
                    }

                    let fromNode = roadEdge.get_from_node();
                    let toNode = roadEdge.get_to_node();
                    ec.erase(dc, *roadEdge);
                    numRemovedEdges += 1;
                    if fromNode.get_incoming_edges().len() == 0 && fromNode.get_outgoing_edges().len() == 0 {
                        // Node is empty; can be removed
                        self.erase(fromNode);
                    }
                    if toNode.get_incoming_edges().len() == 0 && toNode.get_outgoing_edges().len() == 0 {
                        // Node is empty; can be removed
                        self.erase(toNode);
                    }
                }
                write_warningf!("Removed a road without junctions: %.", warningString);
            }
        }
        numRemovedEdges
    }

    /// C++: `int NBNodeCont::removeComponents(NBDistrictCont &, NBEdgeCont &, const int, bool)`
    /// Checks the network for weak connectivity and removes all but the largest components. The connectivity check is done regardless of edge direction and vclass.
    // int
    // NBNodeCont::removeComponents(NBDistrictCont& dc, NBEdgeCont& ec, const int numKeep, bool hasPTStops) {
    //     myRailComponents.clear();
    //     std::vector<std::set<NBEdge*> > components;
    //     // need to use ids here to have the same ordering on all platforms
    //     std::set<std::string> edgesLeft;
    //     for (std::map<std::string, NBEdge*>::const_iterator edgeIt = ec.begin(); edgeIt != ec.end(); ++edgeIt) {
    //         edgesLeft.insert(edgeIt->first);
    //     }
    //     EdgeVector queue;
    //     std::set<NBEdge*> toRemove;
    //     int foundComponents = 0;
    //     int numRemoved = 0;
    //     while (!edgesLeft.empty()) {
    //         queue.push_back(ec.getByID(*edgesLeft.begin()));
    //         std::set<NBEdge*> component;
    //         while (!queue.empty()) {
    //             NBEdge* const e = queue.back();
    //             queue.pop_back();
    //             component.insert(e);
    //             std::vector<EdgeVector> edgeLists;
    //             edgeLists.push_back(e->getFromNode()->getOutgoingEdges());
    //             edgeLists.push_back(e->getFromNode()->getIncomingEdges());
    //             edgeLists.push_back(e->getToNode()->getOutgoingEdges());
    //             edgeLists.push_back(e->getToNode()->getIncomingEdges());
    //             for (std::vector<EdgeVector>::const_iterator listIt = edgeLists.begin(); listIt != edgeLists.end(); ++listIt) {
    //                 for (EdgeVector::const_iterator edgeIt = listIt->begin(); edgeIt != listIt->end(); ++edgeIt) {
    //                     std::set<std::string>::iterator leftIt = edgesLeft.find((*edgeIt)->getID());
    //                     if (leftIt != edgesLeft.end()) {
    //                         queue.push_back(*edgeIt);
    //                         edgesLeft.erase(leftIt);
    //                     }
    //                 }
    //             }
    //         }
    //         foundComponents++;
    //         std::vector<std::set<NBEdge*> >::iterator cIt;
    //         for (cIt = components.begin(); cIt != components.end(); ++cIt) {
    //             if (cIt->size() < component.size()) {
    //                 break;
    //             }
    //         }
    //         components.insert(cIt, component);
    //         if ((int)components.size() > numKeep) {
    //             bool recheck = false;
    //             if (hasPTStops) {
    //                 for (NBEdge* e : components.back()) {
    //                     SVCPermissions permissions = e->getPermissions();
    //                     if (isRailway(permissions) || isWaterway(permissions)) {
    //                         // recheck for connection to other components via access definitions
    //                         recheck = true;
    //                         break;
    //                     }
    //                 }
    //             }
    //             if (!recheck) {
    //                 toRemove.insert(components.back().begin(), components.back().end());
    //                 numRemoved++;
    //             } else {
    //                 std::vector<std::string> edgeIDs;
    //                 for (NBEdge* e : components.back()) {
    //                     edgeIDs.push_back(e->getID());
    //                 }
    //                 myRailComponents.push_back(edgeIDs);
    //             }
    //             components.pop_back();
    //         }
    //     }
    //     ec.removeRoundaboutEdges(toRemove);
    //     for (NBEdge* e : toRemove) {
    //         NBNode* const fromNode = e->getFromNode();
    //         NBNode* const toNode = e->getToNode();
    //         ec.erase(dc, e);
    //         if (fromNode->getIncomingEdges().size() == 0 && fromNode->getOutgoingEdges().size() == 0) {
    //             erase(fromNode);
    //         }
    //         if (toNode->getIncomingEdges().size() == 0 && toNode->getOutgoingEdges().size() == 0) {
    //             erase(toNode);
    //         }
    //     }
    //     if (foundComponents > 1) {
    //         WRITE_MESSAGEF(TL("Found % components and removed % (% edges)."), toString(foundComponents), toString(numRemoved), toString(toRemove.size()));
    //     }
    //     return (int)toRemove.size();
    // }
    #[allow(unused_variables)]
    pub fn remove_components(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, numKeep: i32, hasPTStops: bool) -> i32 {
        self.myRailComponents.clear();
        let mut components: Vec<EdgeSet> = Vec::new();
        // need to use ids here to have the same ordering on all platforms
        let mut edgesLeft: BTreeSet<String> = ec.get_all_names().into_iter().collect();
        let mut queue: EdgeVector = Vec::new();
        let mut toRemove: EdgeSet = EdgeSet::new();
        let mut foundComponents = 0;
        let mut numRemoved = 0;
        while !edgesLeft.is_empty() {
            let firstLeft = edgesLeft.iter().next().unwrap().clone();
            queue.push(ec.get_by_id(&firstLeft));
            let mut component: EdgeSet = EdgeSet::new();
            while let Some(e) = queue.pop() {
                component.insert(e);
                let edgeLists: Vec<EdgeVector> = vec![
                    e.get_from_node().get_outgoing_edges().clone(),
                    e.get_from_node().get_incoming_edges().clone(),
                    e.get_to_node().get_outgoing_edges().clone(),
                    e.get_to_node().get_incoming_edges().clone(),
                ];
                for list in edgeLists {
                    for edge in list {
                        if edgesLeft.remove(edge.get_id()) {
                            queue.push(edge);
                        }
                    }
                }
            }
            foundComponents += 1;
            // insert before the first component that is strictly smaller
            let pos = components.iter().position(|c| c.len() < component.len()).unwrap_or(components.len());
            components.insert(pos, component);
            if components.len() as i32 > numKeep {
                let mut recheck = false;
                if hasPTStops {
                    for e in components.last().unwrap().iter() {
                        let permissions = e.get_permissions(-1);
                        if is_railway(permissions) || is_waterway(permissions) {
                            // recheck for connection to other components via access definitions
                            recheck = true;
                            break;
                        }
                    }
                }
                if !recheck {
                    toRemove.extend(components.last().unwrap().iter().copied());
                    numRemoved += 1;
                } else {
                    // determinism: C++ set<NBEdge*> is address-ordered; Ptr's Ord is used here
                    let edgeIDs: Vec<String> = components.last().unwrap().iter().map(|e| e.get_id().to_string()).collect();
                    self.myRailComponents.push(edgeIDs);
                }
                components.pop();
            }
        }
        ec.remove_roundabout_edges(&toRemove);
        // determinism: erase order follows Ptr's Ord (C++ set<NBEdge*> is address-ordered)
        let removeList: Vec<Ptr<NBEdge>> = toRemove.iter().copied().collect();
        for e in removeList {
            let fromNode = e.get_from_node();
            let toNode = e.get_to_node();
            ec.erase(dc, e);
            if fromNode.get_incoming_edges().is_empty() && fromNode.get_outgoing_edges().is_empty() {
                self.erase(fromNode);
            }
            if toNode.get_incoming_edges().is_empty() && toNode.get_outgoing_edges().is_empty() {
                self.erase(toNode);
            }
        }
        if foundComponents > 1 {
            write_message(&format!("Found {} components and removed {} ({} edges).", foundComponents, numRemoved, toRemove.len()));
        }
        toRemove.len() as i32
    }

    /// C++: `int NBNodeCont::removeRailComponents(NBDistrictCont &, NBEdgeCont &, NBPTStopCont &)`
    // int
    // NBNodeCont::removeRailComponents(NBDistrictCont& dc, NBEdgeCont& ec, NBPTStopCont& sc) {
    //     std::set<std::string> stopEdges;
    //     for (const auto& item : sc.getStops()) {
    //         stopEdges.insert(item.second->getEdgeId());
    //     }
    //     int numRemoved = 0;
    //     int numRemovedEdges = 0;
    //     for (auto& component : myRailComponents) {
    //         bool keep = false;
    //         for (std::string edgeID : component) {
    //             if (stopEdges.count(edgeID) != 0) {
    //                 keep = true;
    //                 break;
    //             }
    //         }
    //         if (!keep) {
    //             numRemoved++;
    //             numRemovedEdges += (int)component.size();
    //             for (std::string edgeID : component) {
    //                 NBEdge* e = ec.retrieve(edgeID);
    //                 if (e != nullptr) {
    //                     NBNode* const fromNode = e->getFromNode();
    //                     NBNode* const toNode = e->getToNode();
    //                     ec.erase(dc, e);
    //                     if (fromNode->getIncomingEdges().size() == 0 && fromNode->getOutgoingEdges().size() == 0) {
    //                         erase(fromNode);
    //                     }
    //                     if (toNode->getIncomingEdges().size() == 0 && toNode->getOutgoingEdges().size() == 0) {
    //                         erase(toNode);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     if (numRemoved > 0) {
    //         WRITE_MESSAGEF(TL("Removed % railway components (% edges)."), toString(numRemoved), toString(numRemovedEdges));
    //     }
    //     return numRemoved;
    // }
    #[allow(unused_variables)]
    pub fn remove_rail_components(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, sc: &mut NBPTStopCont) -> i32 {
        let mut stopEdges: BTreeSet<String> = BTreeSet::new();
        for item in sc.get_stops().values() {
            stopEdges.insert(item.get_edge_id().to_string());
        }
        let mut numRemoved = 0;
        let mut numRemovedEdges = 0;
        for component in self.myRailComponents.clone() {
            let mut keep = false;
            for edgeID in component.iter() {
                if stopEdges.contains(edgeID) {
                    keep = true;
                    break;
                }
            }
            if !keep {
                numRemoved += 1;
                numRemovedEdges += component.len() as i32;
                for edgeID in component.iter() {
                    let e = ec.retrieve(edgeID, false);
                    if !e.is_null() {
                        let fromNode = e.get_from_node();
                        let toNode = e.get_to_node();
                        ec.erase(dc, e);
                        if fromNode.get_incoming_edges().is_empty() && fromNode.get_outgoing_edges().is_empty() {
                            self.erase(fromNode);
                        }
                        if toNode.get_incoming_edges().is_empty() && toNode.get_outgoing_edges().is_empty() {
                            self.erase(toNode);
                        }
                    }
                }
            }
        }
        if numRemoved > 0 {
            write_message(&format!("Removed {} railway components ({} edges).", numRemoved, numRemovedEdges));
        }
        numRemoved
    }

    /// C++: `int NBNodeCont::removeUnwishedNodes(NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &, NBPTStopCont &, NBPTLineCont &, NBParkingCont &, bool)`
    /// Removes "unwished" nodes
    // int
    // NBNodeCont::removeUnwishedNodes(NBDistrictCont& dc, NBEdgeCont& ec,
    //                                 NBTrafficLightLogicCont& /*tlc*/, NBPTStopCont& sc,
    //                                 NBPTLineCont& lc,
    //                                 NBParkingCont& pc,
    //                                 bool removeGeometryNodes) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     // load edges that shall not be modified
    //     std::set<std::string> edges2keep;
    //     if (removeGeometryNodes) {
    //         if (oc.isSet("geometry.remove.keep-edges.input-file")) {
    //             NBHelpers::loadEdgesFromFile(oc.getString("geometry.remove.keep-edges.input-file"), edges2keep);
    //         }
    //         if (oc.isSet("geometry.remove.keep-edges.explicit")) {
    //             const std::vector<std::string> edges = oc.getStringVector("geometry.remove.keep-edges.explicit");
    //             edges2keep.insert(edges.begin(), edges.end());
    //         }
    //         // no need to keep pt stop edges, they are remapped later
    //         // no need to keep all pt route edges. They are validated again before writing
    //         pc.addEdges2Keep(oc, edges2keep);
    //         if (oc.exists("geometry.remove.keep-ptstops") && oc.getBool("geometry.remove.keep-ptstops")) {
    //             sc.addEdges2Keep(oc, edges2keep);
    //         }
    //     }
    //
    //     std::map<NBEdge*, std::set<NBTrafficLightDefinition*> > tlsLookup;
    //     for (auto it = ec.begin(); it != ec.end(); it++) {
    //         NBEdge* e = it->second;
    //         NBNode* to = e->getToNode();
    //         if (to->isTLControlled()) {
    //             tlsLookup[e] = to->getControllingTLS();
    //         }
    //     }
    //     const bool outputRemoved = oc.getBool("output.removed-nodes");
    //     std::vector<NBNode*> toRemove;
    //     for (const auto& i : myNodes) {
    //         NBNode* const current = i.second;
    //         bool remove = false;
    //         // check for completely empty nodes and check for nodes which are only geometry nodes and ask the node whether to join
    //         if (current->getEdges().empty() || (removeGeometryNodes && mySplit.count(current) == 0 && current->checkIsRemovable())) {
    //             remove = true;
    //             // check whether any of the edges must be kept
    //             for (NBEdge* const it_edge : current->getEdges()) {
    //                 if (edges2keep.find(it_edge->getID()) != edges2keep.end()) {
    //                     remove = false;
    //                     break;
    //                 }
    //             }
    //         }
    //         // remove the node and join the geometries when wished
    //         if (!remove) {
    //             continue;
    //         }
    //         for (const std::pair<NBEdge*, NBEdge*>& j : current->getEdgesToJoin()) {
    //             NBEdge* const begin = j.first;
    //             NBEdge* const continuation = j.second;
    //             begin->append(continuation);
    //             continuation->getToNode()->replaceIncoming(continuation, begin, 0);
    //             auto itTL = tlsLookup.find(continuation);
    //             if (itTL != tlsLookup.end()) {
    //                 for (NBTrafficLightDefinition* tls : itTL->second) {
    //                     tls->replaceRemoved(continuation, -1, begin, -1, true);
    //                 }
    //                 tlsLookup[begin] = itTL->second;
    //             }
    //             sc.replaceEdge(continuation->getID(), { begin });
    //             lc.replaceEdge(continuation->getID(), { begin });
    //             ec.extract(dc, continuation, true);
    //             if (outputRemoved) {
    //                 begin->updateRemovedNodes(current->getID());
    //             }
    //         }
    //         toRemove.push_back(current);
    //     }
    //     // erase all
    //     for (NBNode* n : toRemove) {
    //         extract(n, true);
    //     }
    //     return (int)toRemove.size();
    // }
    #[allow(unused_variables)]
    pub fn remove_unwished_nodes(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, sc: &mut NBPTStopCont, lc: &mut NBPTLineCont, pc: &mut NBParkingCont, removeGeometryNodes: bool) -> i32 {
        // load edges that shall not be modified
        let mut edges2keep: BTreeSet<String> = BTreeSet::new();
        if removeGeometryNodes {
            if utils::options::options_cont::is_set("geometry.remove.keep-edges.input-file") {
                NBHelpers::load_edges_from_file(&utils::options::options_cont::get_string("geometry.remove.keep-edges.input-file"), &mut edges2keep);
            }
            if utils::options::options_cont::is_set("geometry.remove.keep-edges.explicit") {
                let edges = utils::options::options_cont::get_string_vector("geometry.remove.keep-edges.explicit");
                edges2keep.extend(edges);
            }
            // no need to keep pt stop edges, they are remapped later
            // no need to keep all pt route edges. They are validated again before writing
            utils::options::options_cont::with_options(|oc| pc.add_edges2_keep(oc, &mut edges2keep));
            if utils::options::options_cont::exists("geometry.remove.keep-ptstops") && utils::options::options_cont::get_bool("geometry.remove.keep-ptstops") {
                utils::options::options_cont::with_options(|oc| sc.add_edges2_keep(oc, &mut edges2keep));
            }
        }

        let mut tlsLookup: BTreeMap<Ptr<NBEdge>, BTreeSet<Ptr<NBTrafficLightDefinition>>> = BTreeMap::new();
        for (_, e) in ec.begin() {
            let e = *e;
            let to = e.m().get_to_node();
            if to.m().is_tl_controlled() {
                tlsLookup.insert(e, to.m().get_controlling_tls().clone());
            }
        }
        let outputRemoved = utils::options::options_cont::get_bool("output.removed-nodes");
        let mut toRemove: Vec<Ptr<NBNode>> = Vec::new();
        for current in self.myNodes.values().copied() {
            let mut remove = false;
            // check for completely empty nodes and check for nodes which are only geometry nodes and ask the node whether to join
            if current.m().get_edges().is_empty() || (removeGeometryNodes && !self.mySplit.contains(&current) && current.m().check_is_removable()) {
                remove = true;
                // check whether any of the edges must be kept
                for it_edge in current.m().get_edges().iter() {
                    if edges2keep.contains(it_edge.m().get_id()) {
                        remove = false;
                        break;
                    }
                }
            }
            // remove the node and join the geometries when wished
            if !remove {
                continue;
            }
            for j in current.m().get_edges_to_join() {
                let begin = j.0;
                let continuation = j.1;
                begin.m().append(continuation);
                continuation.m().get_to_node().m().replace_incoming(continuation, begin, 0);
                if let Some(itTL) = tlsLookup.get(&continuation).cloned() {
                    for tls in itTL.iter() {
                        tls.m().replace_removed(continuation, -1, begin, -1, true);
                    }
                    tlsLookup.insert(begin, itTL);
                }
                let contID = continuation.m().get_id().to_string();
                sc.replace_edge(&contID, &vec![begin]);
                lc.replace_edge(&contID, &vec![begin]);
                ec.extract(dc, continuation, true);
                if outputRemoved {
                    begin.m().update_removed_nodes(current.m().get_id());
                }
            }
            toRemove.push(current);
        }
        // erase all
        for n in toRemove.iter().copied() {
            self.extract(n, true);
        }
        toRemove.len() as i32
    }

    /// C++: `void NBNodeCont::guessTLs(OptionsCont &, NBTrafficLightLogicCont &)`
    /// Guesses which junctions or junction clusters shall be controlled by tls
    // void
    // NBNodeCont::guessTLs(OptionsCont& oc, NBTrafficLightLogicCont& tlc) {
    //     myGuessedTLS.clear();
    //     // build list of definitely not tls-controlled junctions
    //     const double laneSpeedThreshold = oc.getFloat("tls.guess.threshold");
    //     if (oc.isSet("tls.unset")) {
    //         std::vector<std::string> notTLControlledNodes = oc.getStringVector("tls.unset");
    //         for (std::vector<std::string>::const_iterator i = notTLControlledNodes.begin(); i != notTLControlledNodes.end(); ++i) {
    //             NBNode* n = NBNodeCont::retrieve(*i);
    //             if (n == nullptr) {
    //                 throw ProcessError(TLF(" The junction '%' to set as not-controlled is not known.", *i));
    //             }
    //             std::set<NBTrafficLightDefinition*> tls = n->getControllingTLS();
    //             for (std::set<NBTrafficLightDefinition*>::const_iterator j = tls.begin(); j != tls.end(); ++j) {
    //                 (*j)->removeNode(n);
    //             }
    //             n->removeTrafficLights();
    //             myUnsetTLS.insert(n);
    //         }
    //     }
    //
    //     TrafficLightType type = SUMOXMLDefinitions::TrafficLightTypes.get(OptionsCont::getOptions().getString("tls.default-type"));
    //     // loop#1 checking whether the node shall be tls controlled,
    //     //  because it is assigned to a district
    //     if (oc.exists("tls.taz-nodes") && oc.getBool("tls.taz-nodes")) {
    //         for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //             NBNode* cur = (*i).second;
    //             if (cur->isNearDistrict() && myUnsetTLS.count(cur) == 0) {
    //                 setAsTLControlled(cur, tlc, type);
    //             }
    //         }
    //     }
    //
    //     // figure out which nodes mark the locations of TLS signals
    //     // This assumes nodes are already joined
    //     if (oc.exists("tls.guess-signals") && oc.getBool("tls.guess-signals")) {
    //         // prepare candidate edges
    //         const double signalDist = oc.getFloat("tls.guess-signals.dist");
    //         for (const auto& item : myNodes) {
    //             const NBNode* node = item.second;
    //             if (node->isTLControlled() && (node->getIncomingEdges().size() == 1 || node->geometryLike())) {
    // #ifdef DEBUG_GUESSSIGNALS
    //                 if (DEBUGCOND(node) || true) {
    //                     std::cout << " propagate TLS from " << node->getID() << " downstream\n";
    //                 }
    // #endif
    //                 for (NBEdge* edge : node->getOutgoingEdges()) {
    //                     // do not overwrite closer signals
    //                     if (edge->getSignalOffset() == NBEdge::UNSPECIFIED_SIGNAL_OFFSET) {
    //                         edge->setSignalPosition(node->getPosition(), node);
    //                     }
    //                 }
    //             }
    //         }
    //         std::set<NBEdge*> seen;
    //         std::set<NBEdge*> check;
    //         for (const auto& item : myNodes) {
    //             for (NBEdge* edge : item.second->getOutgoingEdges()) {
    //                 if (edge->getSignalPosition() != Position::INVALID) {
    //                     check.insert(edge);
    //                     seen.insert(edge);
    // #ifdef DEBUG_GUESSSIGNALS
    //                     if (DEBUGCOND(edge->getSignalNode()) || true) {
    //                         std::cout << "   primary signalPosition edge=" << edge->getID() << " pos=" << edge->getSignalPosition() << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //         // propagate signal position until the next real intersection
    //         while (check.size() > 0) {
    //             NBEdge* const edge = *check.begin();
    //             check.erase(check.begin());
    //             seen.insert(edge);
    //             NBNode* const nextNode = edge->getToNode();
    //             if (nextNode->geometryLike() && !nextNode->isTLControlled()) {
    //                 for (NBEdge* const outEdge : nextNode->getOutgoingEdges()) {
    //                     if (seen.count(outEdge) == 0) {
    //                         outEdge->setSignalPosition(edge->getSignalPosition(), edge->getSignalNode());
    // #ifdef DEBUG_GUESSSIGNALS
    //                         if (DEBUGCOND(edge->getSignalNode()) || true) {
    //                             std::cout << "   setSignalPosition edge=" << outEdge->getID() << " pos=" << edge->getSignalPosition() << "\n";
    //                         }
    // #endif
    //                         check.insert(outEdge);
    //                     }
    //                 }
    //             }
    //         }
    //
    //         // check which nodes should be controlled
    //         const int slack = oc.getInt("tls.guess-signals.slack");
    //         for (std::map<std::string, NBNode*>::const_iterator i = myNodes.begin(); i != myNodes.end(); ++i) {
    //             NBNode* node = i->second;
    //             if (myUnsetTLS.count(node) != 0) {
    //                 continue;
    //             }
    //             const EdgeVector& incoming = node->getIncomingEdges();
    //             const EdgeVector& outgoing = node->getOutgoingEdges();
    //             if (!node->isTLControlled() && incoming.size() > 1 && !node->geometryLike()
    //                     && !NBNodeTypeComputer::isRailwayNode(node)
    //                     && node->getType() != SumoXMLNodeType::RAIL_CROSSING) {
    //                 std::vector<const NBNode*> signals;
    //                 int foundSignals = 0;
    //                 int missingSignals = 0;
    //                 // check if there is a signal at every incoming edge
    //                 for (EdgeVector::const_iterator it_i = incoming.begin(); it_i != incoming.end(); ++it_i) {
    //                     const NBEdge* inEdge = *it_i;
    //                     if (inEdge->getSignalOffset() == NBEdge::UNSPECIFIED_SIGNAL_OFFSET) {
    //                         if ((inEdge->getPermissions() & SVC_PASSENGER) != 0) {
    // #ifdef DEBUG_GUESSSIGNALS
    //                             if (DEBUGCOND(node)) {
    //                                 std::cout << " noTLS, edge=" << inEdge->getID() << "\n";
    //                             }
    // #endif
    //                             missingSignals++;
    //                             if (missingSignals > slack) {
    //                                 break;
    //                             }
    //                         }
    //                     } else {
    //                         foundSignals++;
    //                     }
    //                 }
    //                 missingSignals = 0;
    //                 int foundSignalsAtDist = 0;
    //                 if (foundSignals > 1 && missingSignals <= slack && missingSignals < foundSignals) {
    //                     node->updateSurroundingGeometry();
    //                     // check if all signals are within the required distance
    //                     // (requires detailed geometry computation)
    //                     for (EdgeVector::const_iterator it_i = incoming.begin(); it_i != incoming.end(); ++it_i) {
    //                         const NBEdge* inEdge = *it_i;
    //                         if (inEdge->getSignalOffset() == NBEdge::UNSPECIFIED_SIGNAL_OFFSET || inEdge->getSignalOffset() > signalDist) {
    //                             if ((inEdge->getPermissions() & SVC_PASSENGER) != 0) {
    // #ifdef DEBUG_GUESSSIGNALS
    //                                 if (DEBUGCOND(node)) {
    //                                     std::cout << " noTLS, edge=" << inEdge->getID() << " offset=" << inEdge->getSignalOffset() << " tlsPos=" << inEdge->getSignalPosition() << "\n";
    //                                 }
    // #endif
    //                                 missingSignals++;
    //                                 if (missingSignals > slack) {
    //                                     break;
    //                                 }
    //                             }
    //                         } else {
    //                             foundSignalsAtDist++;
    //                         }
    //                         const NBNode* signal = inEdge->getSignalNode();
    //                         if (signal != nullptr) {
    //                             signals.push_back(signal);
    //                         }
    //                     }
    //                     // outgoing edges may be tagged with pedestrian crossings. These
    //                     // should also be merged into the main TLS
    //                     for (const NBEdge* outEdge : outgoing) {
    //                         NBNode* cand = outEdge->getToNode();
    //                         if (cand->isTLControlled() && cand->geometryLike() && outEdge->getLength() <= signalDist) {
    // #ifdef DEBUG_GUESSSIGNALS
    //                             if (DEBUGCOND(node)) {
    //                                 std::cout << " node=" << node->getID() << " outEdge=" << outEdge->getID() << " signalNode=" << cand->getID() << " len=" << outEdge->getLength() << "\n";
    //                             }
    // #endif
    //                             signals.push_back(cand);
    //                         }
    //                     }
    //                 }
    //                 if (foundSignalsAtDist > 1 && missingSignals <= slack && missingSignals < foundSignalsAtDist) {
    //                     for (const NBNode* s : signals) {
    //                         std::set<NBTrafficLightDefinition*> tls = s->getControllingTLS();
    //                         const_cast<NBNode*>(s)->reinit(s->getPosition(), SumoXMLNodeType::PRIORITY);
    //                         for (std::set<NBTrafficLightDefinition*>::iterator k = tls.begin(); k != tls.end(); ++k) {
    //                             tlc.removeFully(s->getID());
    //                         }
    //                     }
    //                     //if (true) std::cout << " node=" << node->getID() << " signals=" << toString(signals) << "\n";
    //                     NBTrafficLightDefinition* tlDef = new NBOwnTLDef("GS_" + node->getID(), node, 0, type);
    //                     // @todo patch endOffset for all incoming lanes according to the signal positions
    //                     if (!tlc.insert(tlDef)) {
    //                         // actually, nothing should fail here
    //                         WRITE_WARNINGF(TL("Could not build joined tls '%'."), node->getID());
    //                         delete tlDef;
    //                         return;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     // guess joined tls first, if wished
    //     if (oc.getBool("tls.guess.joining")) {
    //         // get node clusters
    //         NodeClusters cands;
    //         generateNodeClusters(oc.getFloat("tls.join-dist"), cands);
    //         // check these candidates (clusters) whether they should be controlled by a tls
    //         for (NodeClusters::iterator i = cands.begin(); i != cands.end();) {
    //             NodeSet& c = (*i);
    //             // regard only junctions which are not yet controlled and are not
    //             //  forbidden to be controlled
    //             for (NodeSet::iterator j = c.begin(); j != c.end();) {
    //                 if ((*j)->isTLControlled() || myUnsetTLS.count(*j) != 0) {
    //                     c.erase(j++);
    //                 } else {
    //                     ++j;
    //                 }
    //             }
    //             // check whether the cluster should be controlled
    //             // to avoid gigantic clusters, assume that at most 4 nodes should be needed for a guessed-joined-tls
    //             if (c.size() == 0 || !shouldBeTLSControlled(c, laneSpeedThreshold * (double)c.size() / MIN2((double)c.size(), 4.))) {
    //                 i = cands.erase(i);
    //             } else {
    //                 ++i;
    //             }
    //         }
    //         // cands now only contain sets of junctions that shall be joined into being tls-controlled
    //         for (auto nodeSet : cands) {
    //             std::vector<NBNode*> nodes;
    //             for (NBNode* node : nodeSet) {
    //                 nodes.push_back(node);
    //                 myGuessedTLS.insert(node);
    //             }
    //             const std::string& id = createClusterId(nodeSet, "joinedG_");
    //             NBTrafficLightDefinition* tlDef = new NBOwnTLDef(id, nodes, 0, type);
    //             if (!tlc.insert(tlDef)) {
    //                 // actually, nothing should fail here
    //                 WRITE_WARNING(TL("Could not build guessed, joined tls."));
    //                 delete tlDef;
    //                 return;
    //             }
    //         }
    //     }
    //
    //     // guess single tls
    //     if (oc.getBool("tls.guess")) {
    //         for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //             NBNode* cur = (*i).second;
    //             //  do nothing if already is tl-controlled
    //             if (cur->isTLControlled()) {
    //                 continue;
    //             }
    //             // do nothing if in the list of explicit non-controlled junctions
    //             if (myUnsetTLS.count(cur) != 0) {
    //                 continue;
    //             }
    //             NodeSet c;
    //             c.insert(cur);
    //             if (!shouldBeTLSControlled(c, laneSpeedThreshold) || cur->geometryLike()) {
    //                 continue;
    //             }
    //             setAsTLControlled(cur, tlc, type);
    //             myGuessedTLS.insert(cur);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn guess_t_ls(&mut self, oc: &mut OptionsCont, tlc: &mut NBTrafficLightLogicCont) {
        self.myGuessedTLS.clear();
        // build list of definitely not tls-controlled junctions
        let laneSpeedThreshold = oc.get_float("tls.guess.threshold").unwrap();
        if oc.is_set("tls.unset", true).unwrap() {
            let notTLControlledNodes = oc.get_string_vector("tls.unset").unwrap().clone();
            for i in notTLControlledNodes.iter() {
                let n = self.retrieve(i);
                if n.is_null() {
                    panic!("{}", utils::common::util_exceptions::ProcessError::new(&format!(" The junction '{}' to set as not-controlled is not known.", i)));
                }
                let tls = n.m().get_controlling_tls().clone();
                for j in tls.iter() {
                    j.m().remove_node(n);
                }
                n.m().remove_traffic_lights(false);
                self.myUnsetTLS.insert(n);
            }
        }

        let type_: TrafficLightType = utils::xml::sumo_xml_definitions::TrafficLightTypes
            .get(&oc.get_string("tls.default-type").unwrap())
            .unwrap_or_else(|e| std::panic::panic_any(e));
        // loop#1 checking whether the node shall be tls controlled,
        //  because it is assigned to a district
        if oc.exists("tls.taz-nodes") && oc.get_bool("tls.taz-nodes").unwrap() {
            for cur in self.myNodes.values().copied().collect::<Vec<_>>() {
                if cur.m().is_near_district() && !self.myUnsetTLS.contains(&cur) {
                    self.set_as_tl_controlled(cur, tlc, type_, String::new());
                }
            }
        }

        // figure out which nodes mark the locations of TLS signals
        // This assumes nodes are already joined
        if oc.exists("tls.guess-signals") && oc.get_bool("tls.guess-signals").unwrap() {
            // prepare candidate edges
            let signalDist = oc.get_float("tls.guess-signals.dist").unwrap();
            for node in self.myNodes.values().copied().collect::<Vec<_>>() {
                if node.m().is_tl_controlled() && (node.m().get_incoming_edges().len() == 1 || node.m().geometry_like()) {
                    for edge in node.m().get_outgoing_edges().clone() {
                        // do not overwrite closer signals
                        if edge.m().get_signal_offset() == UNSPECIFIED_SIGNAL_OFFSET {
                            let pos = node.m().get_position().clone();
                            edge.m().set_signal_position(&pos, node);
                        }
                    }
                }
            }
            // determinism: C++ std::set<NBEdge*> is ordered by pointer; BTreeSet<Ptr<NBEdge>> keeps the same order
            let mut seen: BTreeSet<Ptr<NBEdge>> = BTreeSet::new();
            let mut check: BTreeSet<Ptr<NBEdge>> = BTreeSet::new();
            for item in self.myNodes.values().copied().collect::<Vec<_>>() {
                for edge in item.m().get_outgoing_edges().clone() {
                    if *edge.m().get_signal_position() != Position::INVALID {
                        check.insert(edge);
                        seen.insert(edge);
                    }
                }
            }
            // propagate signal position until the next real intersection
            while check.len() > 0 {
                let edge = *check.iter().next().unwrap();
                check.remove(&edge);
                seen.insert(edge);
                let nextNode = edge.m().get_to_node();
                if nextNode.m().geometry_like() && !nextNode.m().is_tl_controlled() {
                    for outEdge in nextNode.m().get_outgoing_edges().clone() {
                        if !seen.contains(&outEdge) {
                            let pos = edge.m().get_signal_position().clone();
                            let sigNode = edge.m().get_signal_node();
                            outEdge.m().set_signal_position(&pos, sigNode);
                            check.insert(outEdge);
                        }
                    }
                }
            }

            // check which nodes should be controlled
            let slack = oc.get_int("tls.guess-signals.slack").unwrap();
            for node in self.myNodes.values().copied().collect::<Vec<_>>() {
                if self.myUnsetTLS.contains(&node) {
                    continue;
                }
                let incoming = node.m().get_incoming_edges().clone();
                let outgoing = node.m().get_outgoing_edges().clone();
                if !node.m().is_tl_controlled() && incoming.len() > 1 && !node.m().geometry_like()
                        && !NBNodeTypeComputer::is_railway_node(node)
                        && node.m().get_type() != SumoXMLNodeType::RAIL_CROSSING {
                    let mut signals: Vec<Ptr<NBNode>> = Vec::new();
                    let mut foundSignals: i32 = 0;
                    let mut missingSignals: i32 = 0;
                    // check if there is a signal at every incoming edge
                    for inEdge in incoming.iter().copied() {
                        if inEdge.m().get_signal_offset() == UNSPECIFIED_SIGNAL_OFFSET {
                            if (inEdge.m().get_permissions(-1) & SVC_PASSENGER) != 0 {
                                missingSignals += 1;
                                if missingSignals > slack {
                                    break;
                                }
                            }
                        } else {
                            foundSignals += 1;
                        }
                    }
                    missingSignals = 0;
                    let mut foundSignalsAtDist: i32 = 0;
                    if foundSignals > 1 && missingSignals <= slack && missingSignals < foundSignals {
                        node.m().update_surrounding_geometry();
                        // check if all signals are within the required distance
                        // (requires detailed geometry computation)
                        for inEdge in incoming.iter().copied() {
                            if inEdge.m().get_signal_offset() == UNSPECIFIED_SIGNAL_OFFSET || inEdge.m().get_signal_offset() > signalDist {
                                if (inEdge.m().get_permissions(-1) & SVC_PASSENGER) != 0 {
                                    missingSignals += 1;
                                    if missingSignals > slack {
                                        break;
                                    }
                                }
                            } else {
                                foundSignalsAtDist += 1;
                            }
                            let signal = inEdge.m().get_signal_node();
                            if !signal.is_null() {
                                signals.push(signal);
                            }
                        }
                        // outgoing edges may be tagged with pedestrian crossings. These
                        // should also be merged into the main TLS
                        for outEdge in outgoing.iter().copied() {
                            let cand = outEdge.m().get_to_node();
                            if cand.m().is_tl_controlled() && cand.m().geometry_like() && outEdge.m().get_length() <= signalDist {
                                signals.push(cand);
                            }
                        }
                    }
                    if foundSignalsAtDist > 1 && missingSignals <= slack && missingSignals < foundSignalsAtDist {
                        for s in signals.iter().copied() {
                            let tls = s.m().get_controlling_tls().clone();
                            s.m().reinit(&s.m().get_position().clone(), SumoXMLNodeType::PRIORITY, false);
                            for _k in tls.iter() {
                                tlc.remove_fully(s.m().get_id().to_string());
                            }
                        }
                        let raw: *mut NBOwnTLDef = Box::into_raw(Box::new(NBOwnTLDef::new_2(&format!("GS_{}", node.m().get_id()), node, 0, type_)));
                        // @todo patch endOffset for all incoming lanes according to the signal positions
                        let tlDef: Ptr<NBTrafficLightDefinition> = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) });
                        node.m().add_traffic_light(tlDef);
                        if !tlc.insert(tlDef, false) {
                            // actually, nothing should fail here
                            write_warningf!("Could not build joined tls '%'.", node.m().get_id());
                            drop(unsafe { Box::from_raw(raw) });
                            return;
                        }
                    }
                }
            }
        }

        // guess joined tls first, if wished
        if oc.get_bool("tls.guess.joining").unwrap() {
            // get node clusters
            let mut cands: NBNodeCont_NodeClusters = Vec::new();
            self.generate_node_clusters(oc.get_float("tls.join-dist").unwrap(), &mut cands);
            // check these candidates (clusters) whether they should be controlled by a tls
            let mut i: usize = 0;
            while i < cands.len() {
                // regard only junctions which are not yet controlled and are not
                //  forbidden to be controlled
                let unset = &self.myUnsetTLS;
                cands[i].retain(|j| !(j.m().is_tl_controlled() || unset.contains(j)));
                // check whether the cluster should be controlled
                // to avoid gigantic clusters, assume that at most 4 nodes should be needed for a guessed-joined-tls
                let sz = cands[i].len() as f64;
                let threshold = laneSpeedThreshold * sz / (if sz < 4. { sz } else { 4. });
                if cands[i].is_empty() || !self.should_be_tls_controlled(&cands[i], threshold, false) {
                    cands.remove(i);
                } else {
                    i += 1;
                }
            }
            // cands now only contain sets of junctions that shall be joined into being tls-controlled
            for nodeSet in cands.iter() {
                let mut nodes: Vec<Ptr<NBNode>> = Vec::new();
                for node in nodeSet.iter().copied() {
                    nodes.push(node);
                    self.myGuessedTLS.insert(node);
                }
                let id = self.create_cluster_id(nodeSet, "joinedG_");
                let raw: *mut NBOwnTLDef = Box::into_raw(Box::new(NBOwnTLDef::new(&id, &nodes, 0, type_)));
                let tlDef: Ptr<NBTrafficLightDefinition> = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) });
                for n in nodes.iter().copied() {
                    n.m().add_traffic_light(tlDef);
                }
                if !tlc.insert(tlDef, false) {
                    write_warning("Could not build guessed, joined tls.");
                    drop(unsafe { Box::from_raw(raw) });
                    return;
                }
            }
        }

        // guess single tls
        if oc.get_bool("tls.guess").unwrap() {
            for cur in self.myNodes.values().copied().collect::<Vec<_>>() {
                //  do nothing if already is tl-controlled
                if cur.m().is_tl_controlled() {
                    continue;
                }
                // do nothing if in the list of explicit non-controlled junctions
                if self.myUnsetTLS.contains(&cur) {
                    continue;
                }
                let mut c: NodeSet = NodeSet::new();
                c.insert(cur);
                if !self.should_be_tls_controlled(&c, laneSpeedThreshold, false) || cur.m().geometry_like() {
                    continue;
                }
                self.set_as_tl_controlled(cur, tlc, type_, String::new());
                self.myGuessedTLS.insert(cur);
            }
        }
    }

    /// C++: `void NBNodeCont::recheckGuessedTLS(NBTrafficLightLogicCont &)`
    /// recheck myGuessedTLS after node logics are computed
    // void
    // NBNodeCont::recheckGuessedTLS(NBTrafficLightLogicCont& tlc) {
    //     std::set<NBTrafficLightDefinition*> recompute;
    //     for (NBNode* node : myGuessedTLS) {
    //         if (!node->hasConflict() || !recheckTLSThreshold(node)) {
    //             const std::set<NBTrafficLightDefinition*>& tlDefs = node->getControllingTLS();
    //             recompute.insert(tlDefs.begin(), tlDefs.end());
    //             node->removeTrafficLights(true);
    //             for (NBEdge* edge : node->getIncomingEdges()) {
    //                 edge->clearControllingTLInformation();
    //             }
    //         }
    //     }
    //     for (NBTrafficLightDefinition* def : recompute) {
    //         if (def->getNodes().size() == 0) {
    //             tlc.removeFully(def->getID());
    //         } else {
    //             def->setParticipantsInformation();
    //             def->setTLControllingInformation();
    //             tlc.computeSingleLogic(OptionsCont::getOptions(), def);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_guessed_tls(&mut self, tlc: &mut NBTrafficLightLogicCont) {
        // determinism: C++ std::set<NBTrafficLightDefinition*> is ordered by pointer; BTreeSet<Ptr<..>> keeps the same order
        let mut recompute: BTreeSet<Ptr<NBTrafficLightDefinition>> = BTreeSet::new();
        for node in self.myGuessedTLS.clone() {
            if !node.m().has_conflict() || !self.recheck_tls_threshold(node) {
                let tlDefs = node.m().get_controlling_tls().clone();
                recompute.extend(tlDefs);
                node.m().remove_traffic_lights(true);
                for edge in node.m().get_incoming_edges().clone() {
                    edge.m().clear_controlling_tl_information();
                }
            }
        }
        for def in recompute {
            if def.m().get_nodes().len() == 0 {
                tlc.remove_fully(def.m().get_id().to_string());
            } else {
                def.m().set_participants_information();
                // shortcut: C++ virtual setTLControllingInformation() is dispatched statically to NBOwnTLDef here, because every
                // definition reachable from myGuessedTLS is created by guessTLs as an NBOwnTLDef; upgrade when virtual dispatch exists
                def.cast::<NBOwnTLDef>().m().set_tl_controlling_information();
                utils::options::options_cont::with_options(|oc| tlc.compute_single_logic(oc, def));
            }
        }
    }

    /// C++: `bool NBNodeCont::recheckTLSThreshold(NBNode *)`
    /// check whether a specific guessed tls should keep its type
    // bool
    // NBNodeCont::recheckTLSThreshold(NBNode* node) {
    //     if (!node->isTLControlled()) {
    //         return false;
    //     }
    //     if ((*node->getControllingTLS().begin())->getNodes().size() != 1) {
    //         // unable to perform check for a joined tls
    //         return true;
    //     }
    //     NodeSet c;
    //     c.insert(node);
    //     const double laneSpeedThreshold = OptionsCont::getOptions().getFloat("tls.guess.threshold");
    //     return shouldBeTLSControlled(c, laneSpeedThreshold, true);
    // }
    #[allow(unused_variables)]
    pub fn recheck_tls_threshold(&mut self, node: Ptr<NBNode>) -> bool {
        if !node.is_tl_controlled() {
            return false;
        }
        let first = *node.get_controlling_tls().iter().next().unwrap();
        if first.get_nodes().len() != 1 {
            // unable to perform check for a joined tls
            return true;
        }
        let mut c: NodeSet = BTreeSet::new();
        c.insert(node);
        let laneSpeedThreshold = utils::options::options_cont::get_float("tls.guess.threshold");
        self.should_be_tls_controlled(&c, laneSpeedThreshold, true)
    }

    /// C++: `void NBNodeCont::computeKeepClear()`
    /// compute keepClear status for all connections
    // void
    // NBNodeCont::computeKeepClear() {
    //     for (const auto& item : myNodes) {
    //         item.second->computeKeepClear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_keep_clear(&mut self) {
        for item in self.myNodes.values() {
            item.m().compute_keep_clear();
        }
    }

    /// C++: `void NBNodeCont::joinTLS(NBTrafficLightLogicCont &, double)`
    /// Builds clusters of tls-controlled junctions and joins the control if possible
    // void
    // NBNodeCont::joinTLS(NBTrafficLightLogicCont& tlc, double maxdist) {
    //     const std::vector<std::string> excludeList = OptionsCont::getOptions().getStringVector("tls.join-exclude");
    //     for (const std::string& tlsID : excludeList) {
    //         if (!tlc.exist(tlsID, false)) {
    //             WRITE_WARNINGF("Unknown tls ID '%' in option tls.join-exclude", tlsID);
    //         }
    //     }
    //     std::set<std::string> exclude(excludeList.begin(), excludeList.end());
    //     NodeClusters cands;
    //     generateNodeClusters(maxdist, cands);
    //     for (NodeSet& c : cands) {
    //         for (NodeSet::iterator j = c.begin(); j != c.end();) {
    //             if (!(*j)->isTLControlled() || exclude.count((*(*j)->getControllingTLS().begin())->getID()) != 0) {
    //                 c.erase(j++);
    //             } else {
    //                 ++j;
    //             }
    //         }
    //         if (c.size() < 2 || onlyCrossings(c) || customTLID(c)) {
    //             continue;
    //         }
    //         // figure out type of the joined TLS
    //         Position dummyPos;
    //         bool dummySetTL = false;
    //         std::string id = "joinedS_"; // prefix (see #3871)
    //         TrafficLightType type;
    //         SumoXMLNodeType nodeType = SumoXMLNodeType::UNKNOWN;
    //         analyzeCluster(c, id, dummyPos, dummySetTL, type, nodeType);
    //         for (NBNode* j : c) {
    //             std::set<NBTrafficLightDefinition*> tls = j->getControllingTLS();
    //             j->removeTrafficLights();
    //             for (NBTrafficLightDefinition* k : tls) {
    //                 tlc.removeFully(k->getID());
    //             }
    //         }
    //         std::vector<NBNode*> nodes;
    //         for (NBNode* j : c) {
    //             nodes.push_back(j);
    //         }
    //         NBTrafficLightDefinition* tlDef = new NBOwnTLDef(id, nodes, 0, type);
    //         if (!tlc.insert(tlDef)) {
    //             // actually, nothing should fail here
    //             WRITE_WARNING(TL("Could not build a joined tls."));
    //             delete tlDef;
    //             return;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join_tls(&mut self, tlc: &mut NBTrafficLightLogicCont, maxdist: f64) {
        let excludeList = utils::options::options_cont::get_string_vector("tls.join-exclude");
        for tlsID in excludeList.iter() {
            if !tlc.exist(tlsID, false) {
                write_warningf!("Unknown tls ID '%' in option tls.join-exclude", tlsID.as_str());
            }
        }
        let exclude: BTreeSet<String> = excludeList.iter().cloned().collect();
        let mut cands: NBNodeCont_NodeClusters = Vec::new();
        self.generate_node_clusters(maxdist, &mut cands);
        for c in cands.iter_mut() {
            c.retain(|j| !(!j.m().is_tl_controlled() || exclude.contains(j.m().get_controlling_tls().iter().next().unwrap().m().get_id())));
            if c.len() < 2 || self.only_crossings(c) || self.custom_tlid(c) {
                continue;
            }
            // figure out type of the joined TLS
            let mut dummyPos = Position::new3(0., 0., 0.);
            let mut dummySetTL = false;
            let mut id = "joinedS_".to_string(); // prefix (see #3871)
            // C++ leaves `type` uninitialized; analyzeCluster assigns it
            let mut type_ = TrafficLightType::STATIC;
            let mut nodeType = SumoXMLNodeType::UNKNOWN;
            self.analyze_cluster(c.clone(), &mut id, &mut dummyPos, &mut dummySetTL, &mut type_, &mut nodeType);
            for j in c.iter().copied() {
                let tls = j.m().get_controlling_tls().clone();
                j.m().remove_traffic_lights(false);
                for k in tls.iter() {
                    tlc.remove_fully(k.m().get_id().to_string());
                }
            }
            let nodes: Vec<Ptr<NBNode>> = c.iter().copied().collect();
            let raw: *mut NBOwnTLDef = Box::into_raw(Box::new(NBOwnTLDef::new(&id, &nodes, 0, type_)));
            let tlDef: Ptr<NBTrafficLightDefinition> = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) });
            for n in nodes.iter().copied() {
                n.m().add_traffic_light(tlDef);
            }
            if !tlc.insert(tlDef, false) {
                // actually, nothing should fail here
                write_warning("Could not build a joined tls.");
                drop(unsafe { Box::from_raw(raw) });
                return;
            }
        }
    }

    /// C++: `void NBNodeCont::setAsTLControlled(NBNode *, NBTrafficLightLogicCont &, TrafficLightType, std::string)`
    /// Sets the given node as being controlled by a tls
    // void
    // NBNodeCont::setAsTLControlled(NBNode* node, NBTrafficLightLogicCont& tlc,
    //                               TrafficLightType type, std::string id) {
    //     if (id == "") {
    //         id = node->getID();
    //     }
    //     NBTrafficLightDefinition* tlDef = new NBOwnTLDef(id, node, 0, type);
    //     if (!tlc.insert(tlDef)) {
    //         // actually, nothing should fail here
    //         WRITE_WARNINGF(TL("Building a tl-logic for junction '%' twice is not possible."), id);
    //         delete tlDef;
    //         return;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_as_tl_controlled(&mut self, node: Ptr<NBNode>, tlc: &mut NBTrafficLightLogicCont, type_: TrafficLightType, id: String) {
        let id = if id == "" { node.get_id().to_string() } else { id };
        let raw: *mut NBOwnTLDef = Box::into_raw(Box::new(NBOwnTLDef::new_2(&id, node, 0, type_)));
        let tlDef: Ptr<NBTrafficLightDefinition> = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) });
        node.m().add_traffic_light(tlDef);
        if !tlc.insert(tlDef, false) {
            // actually, nothing should fail here
            write_warning(&format!("Building a tl-logic for junction '{}' twice is not possible.", id));
            drop(unsafe { Box::from_raw(raw) });
        }
    }

    /// C++: `bool NBNodeCont::wasRemoved(std::string) const` [const]
    /// Returns whether the node with the id was deleted explicitly
    // bool wasRemoved(std::string id) const {
    //         return myExtractedNodes.count(id) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_removed(&self, id: String) -> bool {
        self.myExtractedNodes.contains_key(&id)
    }

    /// C++: `void NBNodeCont::addPrefix(const std::string &)`
    /// add prefix to all nodes
    // void
    // NBNodeCont::addPrefix(const std::string& prefix) {
    //     // make a copy of node containers
    //     const auto nodeContainerCopy = myNodes;
    //     myNodes.clear();
    //     for (const auto& node : nodeContainerCopy) {
    //         node.second->setID(prefix + node.second->getID());
    //         myNodes[node.second->getID()] = node.second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_prefix(&mut self, prefix: &str) {
        // make a copy of node containers
        let nodeContainerCopy = self.myNodes.clone();
        self.myNodes.clear();
        for node in nodeContainerCopy.values() {
            node.m().base.set_id(&format!("{}{}", prefix, node.get_id()));
            self.myNodes.insert(node.get_id().to_string(), *node);
        }
    }

    /// C++: `void NBNodeCont::rename(NBNode *, const std::string &)`
    /// Renames the node. Throws exception if newID already exists
    // void
    // NBNodeCont::rename(NBNode* node, const std::string& newID) {
    //     if (myNodes.count(newID) != 0) {
    //         throw ProcessError(TLF("Attempt to rename node using existing id '%'", newID));
    //     }
    //     myNodes.erase(node->getID());
    //     node->setID(newID);
    //     myNodes[newID] = node;
    // }
    #[allow(unused_variables)]
    pub fn rename(&mut self, node: Ptr<NBNode>, newID: &str) {
        if self.myNodes.contains_key(newID) {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(&format!("Attempt to rename node using existing id '{}'", newID)));
        }
        let oldID = node.get_id().to_string();
        self.myNodes.remove(&oldID);
        node.m().base.set_id(newID);
        self.myNodes.insert(newID.to_string(), node);
    }

    /// C++: `void NBNodeCont::computeLanes2Lanes()`
    /// divides the incoming lanes on outgoing lanes
    // void
    // NBNodeCont::computeLanes2Lanes() {
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         (*i).second->computeLanes2Lanes();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_lanes2_lanes(&mut self) {
        for node in self.myNodes.values() {
            node.m().compute_lanes2_lanes();
        }
    }

    /// C++: `void NBNodeCont::computeLogics(const NBEdgeCont &)`
    /// build the list of outgoing edges and lanes
    // void
    // NBNodeCont::computeLogics(const NBEdgeCont& ec) {
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         (*i).second->computeLogic(ec);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_logics(&mut self, ec: &NBEdgeCont) {
        for node in self.myNodes.values() {
            node.m().compute_logic(ec);
        }
    }

    /// C++: `void NBNodeCont::computeLogics2(const NBEdgeCont &, OptionsCont &)`
    /// compute right-of-way logic for all lane-to-lane connections
    // void
    // NBNodeCont::computeLogics2(const NBEdgeCont& ec, OptionsCont& oc) {
    //     std::set<NBNode*> roundaboutNodes;
    //     const bool checkLaneFoesAll = oc.getBool("check-lane-foes.all");
    //     const bool checkLaneFoesRoundabout = !checkLaneFoesAll && oc.getBool("check-lane-foes.roundabout");
    //     if (checkLaneFoesRoundabout) {
    //         const std::set<EdgeSet>& roundabouts = ec.getRoundabouts();
    //         for (std::set<EdgeSet>::const_iterator i = roundabouts.begin(); i != roundabouts.end(); ++i) {
    //             for (EdgeSet::const_iterator j = (*i).begin(); j != (*i).end(); ++j) {
    //                 roundaboutNodes.insert((*j)->getToNode());
    //             }
    //         }
    //     }
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         const bool checkLaneFoes = checkLaneFoesAll || (checkLaneFoesRoundabout && roundaboutNodes.count((*i).second) > 0);
    //         (*i).second->computeLogic2(checkLaneFoes);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_logics2(&mut self, ec: &NBEdgeCont, oc: &mut OptionsCont) {
        let mut roundaboutNodes: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        let checkLaneFoesAll = oc.get_bool("check-lane-foes.all").unwrap();
        let checkLaneFoesRoundabout = !checkLaneFoesAll && oc.get_bool("check-lane-foes.roundabout").unwrap();
        if checkLaneFoesRoundabout {
            let roundabouts = ec.get_roundabouts();
            for i in roundabouts.iter() {
                for j in i.iter() {
                    roundaboutNodes.insert(j.m().get_to_node());
                }
            }
        }
        for node in self.myNodes.values().copied() {
            let checkLaneFoes = checkLaneFoesAll || (checkLaneFoesRoundabout && roundaboutNodes.contains(&node));
            node.m().compute_logic2(checkLaneFoes);
        }
    }

    /// C++: `int NBNodeCont::size() const` [const]
    /// Returns the number of nodes stored in this container
    // int size() const {
    //         return (int) myNodes.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myNodes.len() as i32
    }

    /// C++: `void NBNodeCont::clear()`
    /// deletes all nodes
    // void
    // NBNodeCont::clear() {
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         delete ((*i).second);
    //     }
    //     myNodes.clear();
    //     for (auto& item : myExtractedNodes) {
    //         delete item.second;
    //     }
    //     myExtractedNodes.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        for item in self.myNodes.values().copied().collect::<Vec<_>>() {
            delete_node(item);
        }
        self.myNodes.clear();
        for item in self.myExtractedNodes.values().copied().collect::<Vec<_>>() {
            delete_node(item);
        }
        self.myExtractedNodes.clear();
    }

    /// C++: `std::string NBNodeCont::getFreeID()`
    /// generates a new node ID
    // std::string
    // NBNodeCont::getFreeID() {
    //     int counter = 0;
    //     std::string freeID = "SUMOGenerated" + toString<int>(counter);
    //     // While there is a node with id equal to freeID
    //     while (retrieve(freeID) != nullptr) {
    //         // update counter and generate a new freeID
    //         counter++;
    //         freeID = "SUMOGenerated" + toString<int>(counter);
    //     }
    //     return freeID;
    // }
    #[allow(unused_variables)]
    pub fn get_free_id(&mut self) -> String {
        let mut counter = 0;
        let mut freeID = format!("SUMOGenerated{}", counter);
        // While there is a node with id equal to freeID
        while !self.retrieve(&freeID).is_null() {
            // update counter and generate a new freeID
            counter += 1;
            freeID = format!("SUMOGenerated{}", counter);
        }
        freeID
    }

    /// C++: `void NBNodeCont::computeNodeShapes(double)`
    /// Compute the junction shape for this node
    // void
    // NBNodeCont::computeNodeShapes(double mismatchThreshold) {
    //     for (NodeCont::iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         (*i).second->computeNodeShape(mismatchThreshold);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_node_shapes(&mut self, mismatchThreshold: f64) {
        for node in self.myNodes.values().copied() {
            node.m().compute_node_shape(mismatchThreshold);
        }
    }

    /// C++: `void NBNodeCont::printBuiltNodesStatistics() const` [const]
    /// Prints statistics about built nodes
    // void
    // NBNodeCont::printBuiltNodesStatistics() const {
    //     WRITE_MESSAGE(TL("-----------------------------------------------------"));
    //     WRITE_MESSAGE(TL("Summary:"));
    //
    //     int numUnregulatedJunctions = 0;
    //     int numDeadEndJunctions = 0;
    //     int numTrafficLightJunctions = 0;
    //     int numPriorityJunctions = 0;
    //     int numRightBeforeLeftJunctions = 0;
    //     int numLeftBeforeRightJunctions = 0;
    //     int numAllWayStopJunctions = 0;
    //     int numZipperJunctions = 0;
    //     int numDistrictJunctions = 0;
    //     int numRailCrossing = 0;
    //     int numRailSignals = 0;
    //     for (NodeCont::const_iterator i = myNodes.begin(); i != myNodes.end(); i++) {
    //         switch ((*i).second->getType()) {
    //             case SumoXMLNodeType::NOJUNCTION:
    //                 ++numUnregulatedJunctions;
    //                 break;
    //             case SumoXMLNodeType::DEAD_END:
    //                 ++numDeadEndJunctions;
    //                 break;
    //             case SumoXMLNodeType::TRAFFIC_LIGHT:
    //             case SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED:
    //             case SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION:
    //                 ++numTrafficLightJunctions;
    //                 break;
    //             case SumoXMLNodeType::PRIORITY:
    //             case SumoXMLNodeType::PRIORITY_STOP:
    //                 ++numPriorityJunctions;
    //                 break;
    //             case SumoXMLNodeType::RIGHT_BEFORE_LEFT:
    //                 ++numRightBeforeLeftJunctions;
    //                 break;
    //             case SumoXMLNodeType::LEFT_BEFORE_RIGHT:
    //                 ++numLeftBeforeRightJunctions;
    //                 break;
    //             case SumoXMLNodeType::ALLWAY_STOP:
    //                 ++numAllWayStopJunctions;
    //                 break;
    //             case SumoXMLNodeType::ZIPPER:
    //                 ++numZipperJunctions;
    //                 break;
    //             case SumoXMLNodeType::DISTRICT:
    //                 ++numDistrictJunctions;
    //                 break;
    //             case SumoXMLNodeType::RAIL_CROSSING:
    //                 ++numRailCrossing;
    //                 break;
    //             case SumoXMLNodeType::RAIL_SIGNAL:
    //                 ++numRailSignals;
    //                 break;
    //             case SumoXMLNodeType::UNKNOWN:
    //                 // should not happen
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    //     WRITE_MESSAGE(TL(" Node type statistics:"));
    //     WRITE_MESSAGE("  Unregulated junctions       : " + toString(numUnregulatedJunctions));
    //     if (numDeadEndJunctions > 0) {
    //         WRITE_MESSAGE("  Dead-end junctions          : " + toString(numDeadEndJunctions));
    //     }
    //     WRITE_MESSAGE("  Priority junctions          : " + toString(numPriorityJunctions));
    //     WRITE_MESSAGE("  Right-before-left junctions : " + toString(numRightBeforeLeftJunctions));
    //     if (numLeftBeforeRightJunctions > 0) {
    //         WRITE_MESSAGE("  Left-before-right junctions      : " + toString(numLeftBeforeRightJunctions));
    //     }
    //     if (numTrafficLightJunctions > 0) {
    //         WRITE_MESSAGE("  Traffic light junctions      : " + toString(numTrafficLightJunctions));
    //     }
    //     if (numAllWayStopJunctions > 0) {
    //         WRITE_MESSAGE("  All-way stop junctions      : " + toString(numAllWayStopJunctions));
    //     }
    //     if (numZipperJunctions > 0) {
    //         WRITE_MESSAGE("  Zipper-merge junctions      : " + toString(numZipperJunctions));
    //     }
    //     if (numRailCrossing > 0) {
    //         WRITE_MESSAGE("  Rail crossing junctions      : " + toString(numRailCrossing));
    //     }
    //     if (numRailSignals > 0) {
    //         WRITE_MESSAGE("  Rail signal junctions      : " + toString(numRailSignals));
    //     }
    //     if (numDistrictJunctions > 0) {
    //         WRITE_MESSAGE("  District junctions      : " + toString(numDistrictJunctions));
    //     }
    //     const GeoConvHelper& geoConvHelper = GeoConvHelper::getProcessing();
    //     WRITE_MESSAGE(TL(" Network boundaries:"));
    //     WRITE_MESSAGE("  Original boundary  : " + toString(geoConvHelper.getOrigBoundary()));
    //     WRITE_MESSAGE("  Applied offset     : " + toString(geoConvHelper.getOffsetBase()));
    //     WRITE_MESSAGE("  Converted boundary : " + toString(geoConvHelper.getConvBoundary()));
    //     WRITE_MESSAGE(TL("-----------------------------------------------------"));
    // }
    #[allow(unused_variables)]
    pub fn print_built_nodes_statistics(&self) {
        write_message("-----------------------------------------------------");
        write_message("Summary:");

        let mut numUnregulatedJunctions: i32 = 0;
        let mut numDeadEndJunctions: i32 = 0;
        let mut numTrafficLightJunctions: i32 = 0;
        let mut numPriorityJunctions: i32 = 0;
        let mut numRightBeforeLeftJunctions: i32 = 0;
        let mut numLeftBeforeRightJunctions: i32 = 0;
        let mut numAllWayStopJunctions: i32 = 0;
        let mut numZipperJunctions: i32 = 0;
        let mut numDistrictJunctions: i32 = 0;
        let mut numRailCrossing: i32 = 0;
        let mut numRailSignals: i32 = 0;
        for node in self.myNodes.values().copied() {
            match node.m().get_type() {
                SumoXMLNodeType::NOJUNCTION => numUnregulatedJunctions += 1,
                SumoXMLNodeType::DEAD_END => numDeadEndJunctions += 1,
                SumoXMLNodeType::TRAFFIC_LIGHT
                | SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED
                | SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION => numTrafficLightJunctions += 1,
                SumoXMLNodeType::PRIORITY | SumoXMLNodeType::PRIORITY_STOP => numPriorityJunctions += 1,
                SumoXMLNodeType::RIGHT_BEFORE_LEFT => numRightBeforeLeftJunctions += 1,
                SumoXMLNodeType::LEFT_BEFORE_RIGHT => numLeftBeforeRightJunctions += 1,
                SumoXMLNodeType::ALLWAY_STOP => numAllWayStopJunctions += 1,
                SumoXMLNodeType::ZIPPER => numZipperJunctions += 1,
                SumoXMLNodeType::DISTRICT => numDistrictJunctions += 1,
                SumoXMLNodeType::RAIL_CROSSING => numRailCrossing += 1,
                SumoXMLNodeType::RAIL_SIGNAL => numRailSignals += 1,
                // UNKNOWN should not happen; default: nothing
                _ => {}
            }
        }
        write_message(" Node type statistics:");
        write_message(&format!("  Unregulated junctions       : {}", numUnregulatedJunctions));
        if numDeadEndJunctions > 0 {
            write_message(&format!("  Dead-end junctions          : {}", numDeadEndJunctions));
        }
        write_message(&format!("  Priority junctions          : {}", numPriorityJunctions));
        write_message(&format!("  Right-before-left junctions : {}", numRightBeforeLeftJunctions));
        if numLeftBeforeRightJunctions > 0 {
            write_message(&format!("  Left-before-right junctions      : {}", numLeftBeforeRightJunctions));
        }
        if numTrafficLightJunctions > 0 {
            write_message(&format!("  Traffic light junctions      : {}", numTrafficLightJunctions));
        }
        if numAllWayStopJunctions > 0 {
            write_message(&format!("  All-way stop junctions      : {}", numAllWayStopJunctions));
        }
        if numZipperJunctions > 0 {
            write_message(&format!("  Zipper-merge junctions      : {}", numZipperJunctions));
        }
        if numRailCrossing > 0 {
            write_message(&format!("  Rail crossing junctions      : {}", numRailCrossing));
        }
        if numRailSignals > 0 {
            write_message(&format!("  Rail signal junctions      : {}", numRailSignals));
        }
        if numDistrictJunctions > 0 {
            write_message(&format!("  District junctions      : {}", numDistrictJunctions));
        }
        let (origBoundary, offsetBase, convBoundary) = GeoConvHelper::with_processing(|g| (g.get_orig_boundary().clone(), g.get_offset_base(), g.get_conv_boundary().clone()));
        write_message(" Network boundaries:");
        write_message(&format!("  Original boundary  : {}", origBoundary));
        write_message(&format!("  Applied offset     : {}", utils::common::to_string::to_string(&offsetBase)));
        write_message(&format!("  Converted boundary : {}", convBoundary));
        write_message("-----------------------------------------------------");
    }

    /// C++: `std::vector<std::string> NBNodeCont::getAllNames() const` [const]
    /// get all node names
    // std::vector<std::string>
    // NBNodeCont::getAllNames() const {
    //     std::vector<std::string> ret;
    //     for (NodeCont::const_iterator i = myNodes.begin(); i != myNodes.end(); ++i) {
    //         ret.push_back((*i).first);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_all_names(&self) -> Vec<String> {
        self.myNodes.keys().cloned().collect()
    }

    /// C++: `void NBNodeCont::analyzeCluster(NodeSet, std::string &, Position &, bool &, TrafficLightType &, SumoXMLNodeType &)`
    // void
    // NBNodeCont::analyzeCluster(NodeSet cluster, std::string& id, Position& pos,
    //                            bool& hasTLS, TrafficLightType& type, SumoXMLNodeType& nodeType) {
    //     id = createClusterId(cluster, id);
    //     bool ambiguousType = false;
    //     for (NBNode* j : cluster) {
    //         pos.add(j->getPosition());
    //         // add a traffic light if any of the cluster members was controlled
    //         if (j->isTLControlled()) {
    //             if (!hasTLS) {
    //                 // init type
    //                 type = (*j->getControllingTLS().begin())->getType();
    //             } else if (type != (*j->getControllingTLS().begin())->getType()) {
    //                 ambiguousType = true;
    //             }
    //             hasTLS = true;
    //         }
    //         SumoXMLNodeType otherType = j->getType();
    //         if (nodeType == SumoXMLNodeType::UNKNOWN) {
    //             nodeType = otherType;
    //         } else if (nodeType != otherType) {
    //             if (hasTLS) {
    //                 nodeType = SumoXMLNodeType::TRAFFIC_LIGHT;
    //             } else if (otherType != SumoXMLNodeType::UNKNOWN) {
    //                 if ((nodeType != SumoXMLNodeType::PRIORITY && (nodeType != SumoXMLNodeType::NOJUNCTION || otherType != SumoXMLNodeType::PRIORITY))
    //                         || (otherType != SumoXMLNodeType::NOJUNCTION && otherType != SumoXMLNodeType::UNKNOWN && otherType != SumoXMLNodeType::PRIORITY)) {
    //                     WRITE_WARNINGF("Ambiguous node type for node cluster '%' (%,%), setting to '" + toString(SumoXMLNodeType::PRIORITY) + "'.", id, toString(nodeType), toString(otherType));
    //                 }
    //                 nodeType = SumoXMLNodeType::PRIORITY;
    //             }
    //         }
    //     }
    //     pos.mul(1. / (double)cluster.size());
    //     if (ambiguousType) {
    //         type = SUMOXMLDefinitions::TrafficLightTypes.get(OptionsCont::getOptions().getString("tls.default-type"));
    //         WRITE_WARNINGF(TL("Ambiguous traffic light type for node cluster '%', setting to '%'."), id, toString(type));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn analyze_cluster(&mut self, cluster: NodeSet, id: &mut String, pos: &mut Position, hasTLS: &mut bool, type_: &mut TrafficLightType, nodeType: &mut SumoXMLNodeType) {
        let prefix = id.clone();
        *id = self.create_cluster_id(&cluster, &prefix);
        let mut ambiguousType = false;
        for j in cluster.iter().copied() {
            pos.add(j.m().get_position());
            // add a traffic light if any of the cluster members was controlled
            if j.m().is_tl_controlled() {
                if !*hasTLS {
                    // init type
                    *type_ = j.m().get_controlling_tls().iter().next().unwrap().m().get_type();
                } else if *type_ != j.m().get_controlling_tls().iter().next().unwrap().m().get_type() {
                    ambiguousType = true;
                }
                *hasTLS = true;
            }
            let otherType = j.m().get_type();
            if *nodeType == SumoXMLNodeType::UNKNOWN {
                *nodeType = otherType;
            } else if *nodeType != otherType {
                if *hasTLS {
                    *nodeType = SumoXMLNodeType::TRAFFIC_LIGHT;
                } else if otherType != SumoXMLNodeType::UNKNOWN {
                    if (*nodeType != SumoXMLNodeType::PRIORITY && (*nodeType != SumoXMLNodeType::NOJUNCTION || otherType != SumoXMLNodeType::PRIORITY))
                            || (otherType != SumoXMLNodeType::NOJUNCTION && otherType != SumoXMLNodeType::UNKNOWN && otherType != SumoXMLNodeType::PRIORITY) {
                        write_warning(&format!("Ambiguous node type for node cluster '{}' ({},{}), setting to '{}'.", id, utils::xml::sumo_xml_definitions::NodeTypes.get_string(*nodeType), utils::xml::sumo_xml_definitions::NodeTypes.get_string(otherType), utils::xml::sumo_xml_definitions::NodeTypes.get_string(SumoXMLNodeType::PRIORITY)));
                    }
                    *nodeType = SumoXMLNodeType::PRIORITY;
                }
            }
        }
        pos.mul(1. / (cluster.len() as f64));
        if ambiguousType {
            *type_ = utils::xml::sumo_xml_definitions::TrafficLightTypes
                .get(&utils::options::options_cont::get_string("tls.default-type"))
                .unwrap_or_else(|e| std::panic::panic_any(e));
            write_warning(&format!("Ambiguous traffic light type for node cluster '{}', setting to '{}'.", id, utils::xml::sumo_xml_definitions::TrafficLightTypes.get_string(*type_)));
        }
    }

    /// C++: `void NBNodeCont::registerJoinedCluster(const NodeSet &)`
    /// gets all joined clusters (see doc for myClusters2Join)
    // void
    // NBNodeCont::registerJoinedCluster(const NodeSet& cluster) {
    //     std::set<std::string> ids;
    //     for (NBNode* n : cluster) {
    //         ids.insert(n->getID());
    //     }
    //     myJoinedClusters.push_back(ids);
    // }
    #[allow(unused_variables)]
    pub fn register_joined_cluster(&mut self, cluster: &NodeSet) {
        let ids: BTreeSet<String> = cluster.iter().map(|n| n.get_id().to_string()).collect();
        self.myJoinedClusters.push(ids);
    }

    /// C++: `void NBNodeCont::registerJoinedCluster(const std::set<std::string> &)`
    // void
    // NBNodeCont::registerJoinedCluster(const std::set<std::string>& cluster) {
    //     myJoinedClusters.push_back(cluster);
    // }
    #[allow(unused_variables)]
    pub fn register_joined_cluster_2(&mut self, cluster: &BTreeSet<String>) {
        self.myJoinedClusters.push(cluster.clone());
    }

    /// C++: `void NBNodeCont::unregisterJoinedCluster(const std::set<std::string> &)`
    /// remove cluster from list (on netedit-undo)
    // void
    // NBNodeCont::unregisterJoinedCluster(const std::set<std::string>& cluster) {
    //     auto it = std::find(myJoinedClusters.begin(), myJoinedClusters.end(), cluster);
    //     if (it != myJoinedClusters.end()) {
    //         myJoinedClusters.erase(it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn unregister_joined_cluster(&mut self, cluster: &BTreeSet<String>) {
        if let Some(pos) = self.myJoinedClusters.iter().position(|c| c == cluster) {
            self.myJoinedClusters.remove(pos);
        }
    }

    /// C++: `const std::vector<std::set<std::string>> & NBNodeCont::getJoinedClusters() const` [const]
    /// gets all joined clusters (see doc for myClusters2Join)
    // const std::vector<std::set<std::string> >& getJoinedClusters() const {
    //         return myJoinedClusters;
    //     }
    #[allow(unused_variables)]
    pub fn get_joined_clusters(&self) -> &Vec<BTreeSet<String>> {
        &self.myJoinedClusters
    }

    /// C++: `void NBNodeCont::discardTrafficLights(NBTrafficLightLogicCont &, bool)`
    // void
    // NBNodeCont::discardTrafficLights(NBTrafficLightLogicCont& tlc, bool geometryLike) {
    //     for (NodeCont::const_iterator i = myNodes.begin(); i != myNodes.end(); ++i) {
    //         NBNode* node = i->second;
    //         if (node->isTLControlled() && (!geometryLike || node->geometryLike())) {
    //             // make a copy of tldefs
    //             const std::set<NBTrafficLightDefinition*> tldefs = node->getControllingTLS();
    //             if (geometryLike && (*tldefs.begin())->getNodes().size() > 1) {
    //                 // do not remove joined tls when only removing geometry-like tls
    //                 continue;
    //             }
    //             if (node->getCrossings().size() > 0) {
    //                 // keep controlled pedestrian crossings
    //                 continue;
    //             }
    //             // record signal location
    //             for (NBEdge* edge : node->getOutgoingEdges()) {
    //                 edge->setSignalPosition(node->getPosition(), nullptr);
    // #ifdef DEBUG_GUESSSIGNALS
    //                 std::cout << "   discard-simple " << node->getID() << "  edge=" << edge->getID() << " pos=" << edge->getSignalPosition() << "\n";
    // #endif
    //             }
    //             for (std::set<NBTrafficLightDefinition*>::const_iterator it = tldefs.begin(); it != tldefs.end(); ++it) {
    //                 NBTrafficLightDefinition* tlDef = *it;
    //                 node->removeTrafficLight(tlDef);
    //                 tlc.extract(tlDef);
    //             }
    //             SumoXMLNodeType newType = NBNodeTypeComputer::isRailwayNode(node) ? SumoXMLNodeType::RAIL_SIGNAL : SumoXMLNodeType::UNKNOWN;
    //             node->reinit(node->getPosition(), newType);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn discard_traffic_lights(&mut self, tlc: &mut NBTrafficLightLogicCont, geometryLike: bool) {
        for node in self.myNodes.values().copied() {
            if node.m().is_tl_controlled() && (!geometryLike || node.m().geometry_like()) {
                // make a copy of tldefs
                let tldefs = node.m().get_controlling_tls().clone();
                if geometryLike && tldefs.iter().next().unwrap().m().get_nodes().len() > 1 {
                    // do not remove joined tls when only removing geometry-like tls
                    continue;
                }
                if node.m().get_crossings().len() > 0 {
                    // keep controlled pedestrian crossings
                    continue;
                }
                // record signal location
                for edge in node.m().get_outgoing_edges().clone() {
                    let pos = node.m().get_position().clone();
                    edge.m().set_signal_position(&pos, Ptr::from_raw(std::ptr::null_mut()));
                }
                for tlDef in tldefs.iter().copied() {
                    node.m().remove_traffic_light(tlDef);
                    tlc.extract(tlDef);
                }
                let newType = if NBNodeTypeComputer::is_railway_node(node) { SumoXMLNodeType::RAIL_SIGNAL } else { SumoXMLNodeType::UNKNOWN };
                let pos = node.m().get_position().clone();
                node.m().reinit(&pos, newType, false);
            }
        }
    }

    /// C++: `void NBNodeCont::discardRailSignals()`
    /// discards rail signals
    // void
    // NBNodeCont::discardRailSignals() {
    //     for (auto& item : myNodes) {
    //         NBNode* node = item.second;
    //         if (node->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             node->reinit(node->getPosition(), SumoXMLNodeType::PRIORITY);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn discard_rail_signals(&mut self) {
        for node in self.myNodes.values().copied().collect::<Vec<_>>() {
            if node.get_type() == SumoXMLNodeType::RAIL_SIGNAL {
                node.m().reinit(node.get_position(), SumoXMLNodeType::PRIORITY, false);
            }
        }
    }

    /// C++: `void NBNodeCont::markAsSplit(const NBNode *)`
    /// mark a node as being created form a split
    // void markAsSplit(const NBNode* node) {
    //         mySplit.insert(node);
    //     }
    #[allow(unused_variables)]
    pub fn mark_as_split(&mut self, node: Ptr<NBNode>) {
        self.mySplit.insert(node);
    }

    /// C++: `void NBNodeCont::markAsNotTLS(const NBNode *)`
    /// mark a node as explicitly not controlled by a TLS
    // void markAsNotTLS(const NBNode* node) {
    //         myUnsetTLS.insert(node);
    //     }
    #[allow(unused_variables)]
    pub fn mark_as_not_tls(&mut self, node: Ptr<NBNode>) {
        self.myUnsetTLS.insert(node);
    }

    /// C++: `int NBNodeCont::remapIDs(bool, bool, bool, const std::string &, NBTrafficLightLogicCont &)`
    /// remap node IDs according to options --numerical-ids and --reserved-ids
    // int
    // NBNodeCont::remapIDs(bool numericaIDs, bool reservedIDs, bool keptIDs, const std::string& prefix, NBTrafficLightLogicCont& tlc) {
    //     bool startGiven = !OptionsCont::getOptions().isDefault("numerical-ids.node-start");
    //     if (!numericaIDs && !reservedIDs && prefix == "" && !startGiven) {
    //         return 0;
    //     }
    //     std::vector<std::string> avoid;
    //     if (startGiven) {
    //         avoid.push_back(toString(OptionsCont::getOptions().getInt("numerical-ids.node-start") - 1));
    //     } else {
    //         avoid = getAllNames();
    //     }
    //     std::set<std::string> reserve;
    //     if (reservedIDs) {
    //         NBHelpers::loadPrefixedIDsFomFile(OptionsCont::getOptions().getString("reserved-ids"), "node:", reserve); // backward compatibility
    //         NBHelpers::loadPrefixedIDsFomFile(OptionsCont::getOptions().getString("reserved-ids"), "junction:", reserve); // selection format
    //         avoid.insert(avoid.end(), reserve.begin(), reserve.end());
    //     }
    //     IDSupplier idSupplier("", avoid);
    //     NodeSet toChange;
    //     for (NodeCont::iterator it = myNodes.begin(); it != myNodes.end(); it++) {
    //         if (startGiven) {
    //             toChange.insert(it->second);
    //             continue;
    //         }
    //         if (numericaIDs) {
    //             try {
    //                 StringUtils::toLong(it->first);
    //             } catch (NumberFormatException&) {
    //                 toChange.insert(it->second);
    //             }
    //         }
    //         if (reservedIDs && reserve.count(it->first) > 0) {
    //             toChange.insert(it->second);
    //         }
    //     }
    //     std::set<std::string> keep;
    //     if (keptIDs) {
    //         NBHelpers::loadPrefixedIDsFomFile(OptionsCont::getOptions().getString("kept-ids"), "node:", keep); // backward compatibility
    //         NBHelpers::loadPrefixedIDsFomFile(OptionsCont::getOptions().getString("kept-ids"), "junction:", keep); // selection format
    //         for (auto it = toChange.begin(); it != toChange.end();) {
    //             if (keep.count((*it)->getID()) != 0) {
    //                 it = toChange.erase(it++);
    //             } else {
    //                 it++;
    //             }
    //         }
    //     }
    //     const bool origNames = OptionsCont::getOptions().getBool("output.original-names");
    //     for (NBNode* node : toChange) {
    //         myNodes.erase(node->getID());
    //     }
    //     for (NBNode* node : toChange) {
    //         if (origNames && node->getParameter(SUMO_PARAM_ORIGID) == "") {
    //             node->setParameter(SUMO_PARAM_ORIGID, node->getID());
    //         }
    //         node->setID(idSupplier.getNext());
    //         for (NBTrafficLightDefinition* tlDef : node->getControllingTLS()) {
    //             tlc.rename(tlDef, node->getID());
    //         }
    //         myNodes[node->getID()] = node;
    //     }
    //     if (prefix.empty()) {
    //         return (int)toChange.size();
    //     } else {
    //         int renamed = 0;
    //         // make a copy because we will modify the map
    //         auto oldNodes = myNodes;
    //         for (auto item : oldNodes) {
    //             if (!StringUtils::startsWith(item.first, prefix) && keep.count(item.first) == 0) {
    //                 rename(item.second, prefix + item.first);
    //                 for (NBTrafficLightDefinition* tlDef : item.second->getControllingTLS()) {
    //                     if (!StringUtils::startsWith(tlDef->getID(), prefix)) {
    //                         tlc.rename(tlDef, prefix + tlDef->getID());
    //                     }
    //                 }
    //                 renamed++;
    //             }
    //         }
    //         return renamed;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remap_i_ds(&mut self, numericaIDs: bool, reservedIDs: bool, keptIDs: bool, prefix: &str, tlc: &mut NBTrafficLightLogicCont) -> i32 {
        let startGiven = !utils::options::options_cont::is_default("numerical-ids.node-start");
        if !numericaIDs && !reservedIDs && prefix == "" && !startGiven {
            return 0;
        }
        let mut avoid: Vec<String> = Vec::new();
        if startGiven {
            avoid.push(format!("{}", utils::options::options_cont::get_int("numerical-ids.node-start") - 1));
        } else {
            avoid = self.get_all_names();
        }
        let mut reserve: BTreeSet<String> = BTreeSet::new();
        if reservedIDs {
            NBHelpers::load_prefixed_i_ds_fom_file(&utils::options::options_cont::get_string("reserved-ids"), "node:".to_string(), &mut reserve); // backward compatibility
            NBHelpers::load_prefixed_i_ds_fom_file(&utils::options::options_cont::get_string("reserved-ids"), "junction:".to_string(), &mut reserve); // selection format
            avoid.extend(reserve.iter().cloned());
        }
        let mut idSupplier = NodeIDSupplier::new(&avoid);
        // C++ std::set<NBNode*, ComparatorIdLess>: ordered by id, first insertion wins
        let mut toChange: BTreeMap<String, Ptr<NBNode>> = BTreeMap::new();
        for (id, node) in self.myNodes.iter() {
            if startGiven {
                toChange.entry(node.m().get_id().to_string()).or_insert(*node);
                continue;
            }
            if numericaIDs {
                if utils::common::string_utils::to_long(id).is_err() {
                    toChange.entry(node.m().get_id().to_string()).or_insert(*node);
                }
            }
            if reservedIDs && reserve.contains(id) {
                toChange.entry(node.m().get_id().to_string()).or_insert(*node);
            }
        }
        let mut keep: BTreeSet<String> = BTreeSet::new();
        if keptIDs {
            NBHelpers::load_prefixed_i_ds_fom_file(&utils::options::options_cont::get_string("kept-ids"), "node:".to_string(), &mut keep); // backward compatibility
            NBHelpers::load_prefixed_i_ds_fom_file(&utils::options::options_cont::get_string("kept-ids"), "junction:".to_string(), &mut keep); // selection format
            toChange.retain(|k, _| !keep.contains(k));
        }
        let origNames = utils::options::options_cont::get_bool("output.original-names");
        for node in toChange.values().copied() {
            let id = node.m().get_id().to_string();
            self.myNodes.remove(&id);
        }
        for node in toChange.values().copied() {
            if origNames && node.m().base2.get_parameter(SUMO_PARAM_ORIGID, "") == "" {
                let id = node.m().get_id().to_string();
                node.m().base2.set_parameter(SUMO_PARAM_ORIGID, &id);
            }
            let newID = idSupplier.get_next();
            node.m().base.set_id(&newID);
            for tlDef in node.m().get_controlling_tls().clone() {
                tlc.rename(tlDef, &newID);
            }
            self.myNodes.insert(newID, node);
        }
        if prefix.is_empty() {
            toChange.len() as i32
        } else {
            let mut renamed: i32 = 0;
            // make a copy because we will modify the map
            let oldNodes = self.myNodes.clone();
            for (key, node) in oldNodes {
                if !utils::common::string_utils::starts_with(&key, prefix) && !keep.contains(&key) {
                    self.rename(node, &format!("{}{}", prefix, key));
                    for tlDef in node.m().get_controlling_tls().clone() {
                        if !utils::common::string_utils::starts_with(tlDef.m().get_id(), prefix) {
                            let newTLID = format!("{}{}", prefix, tlDef.m().get_id());
                            tlc.rename(tlDef, &newTLID);
                        }
                    }
                    renamed += 1;
                }
            }
            renamed
        }
    }

    /// C++: `int NBNodeCont::guessFringe()`
    /// guess and mark fringe nodes
    // int
    // NBNodeCont::guessFringe() {
    //     // guess outer fringe by topology and being on the pareto-boundary
    //     NodeSet topRightFront;
    //     NodeSet topLeftFront;
    //     NodeSet bottomRightFront;
    //     NodeSet bottomLeftFront;
    //     for (const auto& item : myNodes) {
    //         paretoCheck(item.second, topRightFront, 1, 1);
    //         paretoCheck(item.second, topLeftFront, -1, 1);
    //         paretoCheck(item.second, bottomRightFront, 1, -1);
    //         paretoCheck(item.second, bottomLeftFront, -1, -1);
    //     }
    //     NodeSet front;
    //     front.insert(topRightFront.begin(), topRightFront.end());
    //     front.insert(topLeftFront.begin(), topLeftFront.end());
    //     front.insert(bottomRightFront.begin(), bottomRightFront.end());
    //     front.insert(bottomLeftFront.begin(), bottomLeftFront.end());
    //     int numFringe = 0;
    //     for (NBNode* n : front) {
    //         const int in = (int)n->getIncomingEdges().size();
    //         const int out = (int)n->getOutgoingEdges().size();
    //         if ((in <= 1 && out <= 1) &&
    //                 (in == 0 || out == 0
    //                  || n->getIncomingEdges().front()->isTurningDirectionAt(n->getOutgoingEdges().front()))) {
    //             n->setFringeType(FringeType::OUTER);
    //             numFringe++;
    //         }
    //     }
    //     // guess outer fringe by topology and speed
    //     const double speedThreshold = OptionsCont::getOptions().getFloat("fringe.guess.speed-threshold");
    //     for (const auto& item : myNodes) {
    //         NBNode* n = item.second;
    //         if (front.count(n) != 0) {
    //             continue;
    //         }
    //         if (n->getEdges().size() == 1 && n->getEdges().front()->getSpeed() > speedThreshold) {
    //             n->setFringeType(FringeType::OUTER);
    //             numFringe++;
    //         }
    //     }
    //     return numFringe;
    // }
    #[allow(unused_variables)]
    pub fn guess_fringe(&mut self) -> i32 {
        // guess outer fringe by topology and being on the pareto-boundary
        let mut topRightFront: NodeSet = NodeSet::new();
        let mut topLeftFront: NodeSet = NodeSet::new();
        let mut bottomRightFront: NodeSet = NodeSet::new();
        let mut bottomLeftFront: NodeSet = NodeSet::new();
        for item in self.myNodes.values().copied().collect::<Vec<_>>() {
            self.pareto_check(item, &mut topRightFront, 1, 1);
            self.pareto_check(item, &mut topLeftFront, -1, 1);
            self.pareto_check(item, &mut bottomRightFront, 1, -1);
            self.pareto_check(item, &mut bottomLeftFront, -1, -1);
        }
        let mut front: NodeSet = NodeSet::new();
        front.extend(topRightFront.iter().copied());
        front.extend(topLeftFront.iter().copied());
        front.extend(bottomRightFront.iter().copied());
        front.extend(bottomLeftFront.iter().copied());
        let mut numFringe: i32 = 0;
        for n in front.iter().copied() {
            let inE = n.m().get_incoming_edges().clone();
            let outE = n.m().get_outgoing_edges().clone();
            let in_ = inE.len() as i32;
            let out = outE.len() as i32;
            if (in_ <= 1 && out <= 1)
                && (in_ == 0 || out == 0 || inE[0].m().is_turning_direction_at(outE[0])) {
                n.m().set_fringe_type(FringeType::OUTER);
                numFringe += 1;
            }
        }
        // guess outer fringe by topology and speed
        let speedThreshold = utils::options::options_cont::get_float("fringe.guess.speed-threshold");
        for n in self.myNodes.values().copied().collect::<Vec<_>>() {
            if front.contains(&n) {
                continue;
            }
            let edges = n.m().get_edges().clone();
            if edges.len() == 1 && edges[0].m().get_speed() > speedThreshold {
                n.m().set_fringe_type(FringeType::OUTER);
                numFringe += 1;
            }
        }
        numFringe
    }

    /// C++: `void NBNodeCont::applyConditionalDefaults()`
    /// apply default values after loading
    // void
    // NBNodeCont::applyConditionalDefaults() {
    //     for (const auto& item : myNodes) {
    //         NBNode* n = item.second;
    //         if (n->isTLControlled() && n->getRightOfWay() == RightOfWay::DEFAULT) {
    //             bool hasNEMA = false;
    //             for (NBTrafficLightDefinition* tl : n->getControllingTLS()) {
    //                 if (tl->getType() == TrafficLightType::NEMA) {
    //                     hasNEMA = true;
    //                     break;
    //                 }
    //             }
    //             if (hasNEMA) {
    //                 // NEMA controller defaults to allway_stop behavior when switched off
    //                 n->setRightOfWay(RightOfWay::ALLWAYSTOP);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_conditional_defaults(&mut self) {
        for item in self.myNodes.values() {
            let n = *item;
            if n.m().is_tl_controlled() && n.m().get_right_of_way() == RightOfWay::DEFAULT {
                let mut hasNEMA = false;
                for tl in n.m().get_controlling_tls().iter() {
                    if tl.get_type() == TrafficLightType::NEMA {
                        hasNEMA = true;
                        break;
                    }
                }
                if hasNEMA {
                    // NEMA controller defaults to allway_stop behavior when switched off
                    n.m().set_right_of_way(RightOfWay::ALLWAYSTOP);
                }
            }
        }
    }

    /// C++: `bool NBNodeCont::resetNodeShapes()`
    /// reset all node shapes
    // bool
    // NBNodeCont::resetNodeShapes() {
    //     bool hadShapes = false;
    //     for (const auto& item : myNodes) {
    //         if (item.second->getShape().size() > 0 && !item.second->hasCustomShape()) {
    //             hadShapes = true;
    //             item.second->resetShape();
    //         }
    //     }
    //     return hadShapes;
    // }
    #[allow(unused_variables)]
    pub fn reset_node_shapes(&mut self) -> bool {
        let mut hadShapes = false;
        for item in self.myNodes.values() {
            if item.m().get_shape().len() > 0 && !item.m().has_custom_shape() {
                hadShapes = true;
                item.m().reset_shape();
            }
        }
        hadShapes
    }

    /// C++: `void NBNodeCont::generateNodeClusters(double, NodeClusters &) const` [const, private]
    /// Builds node clusters
    // void
    // NBNodeCont::generateNodeClusters(double maxDist, NodeClusters& into) const {
    //     std::set<NBNode*> visited;
    //     for (const auto& i : myNodes) {
    //         if (visited.count(i.second) > 0) {
    //             continue;
    //         }
    //         std::vector<NodeAndDist> toProc;
    //         toProc.emplace_back(i.second, 0.);
    //         NodeSet c;
    //         while (!toProc.empty()) {
    //             NBNode* const n = toProc.back().first;
    //             const double dist = toProc.back().second;
    //             toProc.pop_back();
    //             if (visited.count(n) > 0) {
    //                 continue;
    //             }
    //             visited.insert(n);
    //             bool pureRail = true;
    //             bool railAndPeds = true;
    //             for (NBEdge* e : n->getEdges()) {
    //                 if ((e->getPermissions() & ~(SVC_RAIL_CLASSES | SVC_PEDESTRIAN)) != 0) {
    //                     railAndPeds = false;
    //                     pureRail = false;
    //                     break;
    //                 }
    //                 if ((e->getPermissions() & ~(SVC_RAIL_CLASSES)) != 0) {
    //                     pureRail = false;
    //                 }
    //             }
    //             if (pureRail) {
    //                 // do not join pure rail nodes
    //                 continue;
    //             }
    //             c.insert(n);
    //             for (NBEdge* e : n->getEdges()) {
    //                 NBNode* s = n->hasIncoming(e) ? e->getFromNode() : e->getToNode();
    //                 const double length = e->getLoadedLength();
    // #ifdef DEBUG_JOINJUNCTIONS
    //                 if (DEBUGCOND(s)) {
    //                     std::cout << "generateNodeClusters: consider s=" << s->getID()
    //                               << " clusterNode=" << n->getID() << " edge=" << e->getID() << " dist=" << dist << " length=" << length << " with cluster " << joinNamedToString(c, ' ') << "\n";
    //                 }
    // #endif
    //                 if (railAndPeds && n->getType() != SumoXMLNodeType::RAIL_CROSSING) {
    //                     bool railAndPeds2 = true;
    //                     for (NBEdge* e2 : n->getEdges()) {
    //                         if ((e2->getPermissions() & ~(SVC_RAIL_CLASSES | SVC_PEDESTRIAN)) != 0) {
    //                             railAndPeds2 = false;
    //                             break;
    //                         }
    //                     }
    //                     if (railAndPeds2 && s->getType() != SumoXMLNodeType::RAIL_CROSSING) {
    //                         // do not join rail/ped nodes unless at a rail crossing
    //                         // (neither nodes nor the traffic lights)
    //                         continue;
    //                     }
    //                 }
    //                 const bool bothCrossing = n->getType() == SumoXMLNodeType::RAIL_CROSSING && s->getType() == SumoXMLNodeType::RAIL_CROSSING;
    //                 const bool joinPedCrossings = bothCrossing && e->getPermissions() == SVC_PEDESTRIAN;
    //                 if ( // never join pedestrian stuff (unless at a rail crossing
    //                     !joinPedCrossings && (
    //                         e->getPermissions() == SVC_PEDESTRIAN
    //                         // only join edges for regular passenger traffic or edges that are extremely short
    //                         || (length > 3 * POSITION_EPS
    //                             && (e->getPermissions() & (SVC_PASSENGER | SVC_TRAM)) == 0
    //                             && n->getPosition().distanceTo2D(s->getPosition()) > SUMO_const_laneWidth))) {
    // #ifdef DEBUG_JOINJUNCTIONS
    //                     if (DEBUGCOND(s)) {
    //                         std::cout << " ignored s=" << s->getID() << " pedestrian edge=" << e->getID() << " cd=" << n->getPosition().distanceTo2D(s->getPosition()) << "\n";
    //                     }
    // #endif
    //                     continue;
    //                 }
    //                 // never join rail_crossings with other node types unless the crossing is only for tram
    //                 if ((n->getType() == SumoXMLNodeType::RAIL_CROSSING && s->getType() != SumoXMLNodeType::RAIL_CROSSING)
    //                         || (n->getType() != SumoXMLNodeType::RAIL_CROSSING && s->getType() == SumoXMLNodeType::RAIL_CROSSING)) {
    //                     const SVCPermissions railNoTram = (SVC_RAIL_CLASSES & ~SVC_TRAM);
    //                     bool foundRail = false;
    //                     NBNode* crossingNode = n->getType() == SumoXMLNodeType::RAIL_CROSSING ? n : s;
    //                     for (NBEdge* e2 : crossingNode->getIncomingEdges()) {
    //                         if ((e2->getPermissions() & railNoTram) != 0) {
    //                             foundRail = true;
    //                             break;
    //                         }
    //                     }
    //                     if (foundRail) {
    //                         continue;
    //                     }
    //                 }
    //                 // never join rail_crossings via a rail edge
    //                 if (bothCrossing && (e->getPermissions() & ~SVC_RAIL_CLASSES) == 0) {
    //                     continue;
    //                 }
    //                 if (visited.find(s) != visited.end()) {
    //                     continue;
    //                 }
    //                 if (length + dist < maxDist) {
    //                     // don't add long "boring" appendages but always join the whole rail crossing or tls
    //                     const bool trueGeomLike = s->geometryLike();
    //                     if (trueGeomLike || geometryLikeForClass(s, SVC_VULNERABLE | SVC_DELIVERY)) {
    //                         const bool hasTLS = n->isTrafficLight() || s->isTrafficLight();
    //                         const double fullLength = e->getGeometry().length2D();
    //                         const double length2 = bothCrossing || hasTLS || trueGeomLike ? length : fullLength;
    //                         toProc.emplace_back(s, dist + length2);
    //                     } else {
    //                         toProc.emplace_back(s, 0.);
    //                     }
    //                 }
    //             }
    //         }
    //         if (c.size() < 2) {
    //             continue;
    //         }
    // #ifdef DEBUG_JOINJUNCTIONS
    //         std::cout << " DEBUG: consider cluster " << joinNamedToString(c, ' ') << "\n";
    // #endif
    //         into.push_back(c);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_node_clusters(&self, maxDist: f64, into: &mut NBNodeCont_NodeClusters) {
        let mut visited: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
        for i in self.myNodes.values().copied() {
            if visited.contains(&i) {
                continue;
            }
            // C++ NodeAndDist
            let mut toProc: Vec<(Ptr<NBNode>, f64)> = vec![(i, 0.)];
            let mut c: NodeSet = NodeSet::new();
            while let Some((n, dist)) = toProc.pop() {
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(n);
                let mut pureRail = true;
                let mut railAndPeds = true;
                for e in n.m().get_edges().clone() {
                    if (e.m().get_permissions(-1) & !(SVC_RAIL_CLASSES | SVC_PEDESTRIAN)) != 0 {
                        railAndPeds = false;
                        pureRail = false;
                        break;
                    }
                    if (e.m().get_permissions(-1) & !SVC_RAIL_CLASSES) != 0 {
                        pureRail = false;
                    }
                }
                if pureRail {
                    // do not join pure rail nodes
                    continue;
                }
                c.insert(n);
                for e in n.m().get_edges().clone() {
                    let s = if n.m().has_incoming(e) { e.m().get_from_node() } else { e.m().get_to_node() };
                    let length = e.m().get_loaded_length();
                    if railAndPeds && n.m().get_type() != SumoXMLNodeType::RAIL_CROSSING {
                        let mut railAndPeds2 = true;
                        for e2 in n.m().get_edges().clone() {
                            if (e2.m().get_permissions(-1) & !(SVC_RAIL_CLASSES | SVC_PEDESTRIAN)) != 0 {
                                railAndPeds2 = false;
                                break;
                            }
                        }
                        if railAndPeds2 && s.m().get_type() != SumoXMLNodeType::RAIL_CROSSING {
                            // do not join rail/ped nodes unless at a rail crossing
                            // (neither nodes nor the traffic lights)
                            continue;
                        }
                    }
                    let bothCrossing = n.m().get_type() == SumoXMLNodeType::RAIL_CROSSING && s.m().get_type() == SumoXMLNodeType::RAIL_CROSSING;
                    let joinPedCrossings = bothCrossing && e.m().get_permissions(-1) == SVC_PEDESTRIAN;
                    if !joinPedCrossings && (
                        // never join pedestrian stuff (unless at a rail crossing
                        e.m().get_permissions(-1) == SVC_PEDESTRIAN
                        // only join edges for regular passenger traffic or edges that are extremely short
                        || (length > 3. * POSITION_EPS
                            && (e.m().get_permissions(-1) & (SVC_PASSENGER | SVC_TRAM)) == 0
                            && n.m().get_position().distance_to2d(s.m().get_position()) > SUMO_CONST_LANE_WIDTH)) {
                        continue;
                    }
                    // never join rail_crossings with other node types unless the crossing is only for tram
                    if (n.m().get_type() == SumoXMLNodeType::RAIL_CROSSING && s.m().get_type() != SumoXMLNodeType::RAIL_CROSSING)
                            || (n.m().get_type() != SumoXMLNodeType::RAIL_CROSSING && s.m().get_type() == SumoXMLNodeType::RAIL_CROSSING) {
                        let railNoTram: SVCPermissions = SVC_RAIL_CLASSES & !SVC_TRAM;
                        let mut foundRail = false;
                        let crossingNode = if n.m().get_type() == SumoXMLNodeType::RAIL_CROSSING { n } else { s };
                        for e2 in crossingNode.m().get_incoming_edges().clone() {
                            if (e2.m().get_permissions(-1) & railNoTram) != 0 {
                                foundRail = true;
                                break;
                            }
                        }
                        if foundRail {
                            continue;
                        }
                    }
                    // never join rail_crossings via a rail edge
                    if bothCrossing && (e.m().get_permissions(-1) & !SVC_RAIL_CLASSES) == 0 {
                        continue;
                    }
                    if visited.contains(&s) {
                        continue;
                    }
                    if length + dist < maxDist {
                        // don't add long "boring" appendages but always join the whole rail crossing or tls
                        let trueGeomLike = s.m().geometry_like();
                        if trueGeomLike || NBNodeCont::geometry_like_for_class(s, SVC_VULNERABLE | SVC_DELIVERY) {
                            let hasTLS = n.m().is_traffic_light_2() || s.m().is_traffic_light_2();
                            let fullLength = e.m().get_geometry().length2d();
                            let length2 = if bothCrossing || hasTLS || trueGeomLike { length } else { fullLength };
                            toProc.push((s, dist + length2));
                        } else {
                            toProc.push((s, 0.));
                        }
                    }
                }
            }
            if c.len() < 2 {
                continue;
            }
            into.push(c);
        }
    }

    /// C++: `void NBNodeCont::pruneClusterFringe(NodeSet &, double, bool) const` [const, private]
    /// remove geometry-like fringe nodes from cluster
    // void
    // NBNodeCont::pruneClusterFringe(NodeSet& cluster, double maxDist, bool remove2TLS) const {
    // #ifdef DEBUG_JOINJUNCTIONS
    //     if (gDebugFlag1) {
    //         std::cout << "pruning cluster=" << joinNamedToString(cluster, ' ') << "\n";
    //     }
    // #endif
    //     // iteratively remove the fringe
    //     NodeSet geometryLikeTLS;
    //     bool pruneFringe = true;
    //     bool pruneNoisyFringe = false;
    //     // collect nodes that shall be joined due to distance but are not connected
    //     // to the cluster for passenger traffic
    //     while (pruneFringe) {
    //         pruneFringe = false;
    //         for (NodeSet::iterator j = cluster.begin(); j != cluster.end();) {
    //             NodeSet::iterator check = j;
    //             NBNode* n = *check;
    //             ++j;
    //
    //             // compute clusterDist for node (length of shortest edge which connects this node to the cluster)
    //             double clusterDist = std::numeric_limits<double>::max();
    //             bool touchingCluster = false;
    //             for (EdgeVector::const_iterator it_edge = n->getOutgoingEdges().begin(); it_edge != n->getOutgoingEdges().end(); ++it_edge) {
    //                 NBNode* neighbor = (*it_edge)->getToNode();
    //                 if (cluster.count(neighbor) != 0) {
    //                     clusterDist = MIN2(clusterDist, (*it_edge)->getLoadedLength());
    //                     touchingCluster |= n->getPosition().distanceTo2D(neighbor->getPosition()) <= SUMO_const_laneWidth;
    //                 }
    //             }
    //             for (EdgeVector::const_iterator it_edge = n->getIncomingEdges().begin(); it_edge != n->getIncomingEdges().end(); ++it_edge) {
    //                 NBNode* neighbor = (*it_edge)->getFromNode();
    //                 if (cluster.count(neighbor) != 0) {
    //                     clusterDist = MIN2(clusterDist, (*it_edge)->getLoadedLength());
    //                     touchingCluster |= n->getPosition().distanceTo2D(neighbor->getPosition()) <= SUMO_const_laneWidth;
    //                 }
    //             }
    //             // remove geometry-like nodes at fringe of the cluster
    //             // (they have 1 neighbor in the cluster and at most 1 neighbor outside the cluster)
    //             std::set<NBNode*> outsideNeighbors;
    //             std::set<NBNode*> clusterNeighbors;
    //             const double pedestrianFringeThreshold = 0.3;
    //             for (NBEdge* e : n->getEdges()) {
    //                 NBNode* neighbor = e->getFromNode() == n ? e->getToNode() : e->getFromNode();
    //                 if (cluster.count(neighbor) == 0) {
    //                     if ((e->getPermissions() & SVC_PASSENGER) != 0
    //                             || isRailway(e->getPermissions()) // join railway crossings
    //                             || (clusterDist <= pedestrianFringeThreshold
    //                                 && (!pruneNoisyFringe
    //                                     || isForVulnerableModes(e->getPermissions())
    //                                     // permit joining small opposite merges
    //                                     || getDiameter(cluster) < maxDist
    //                                     || cluster.size() == 2))
    //                             || touchingCluster) {
    //                         outsideNeighbors.insert(neighbor);
    //                     }
    //                 } else {
    //                     clusterNeighbors.insert(neighbor);
    //                 }
    //             }
    // #ifdef DEBUG_JOINJUNCTIONS
    //             if (gDebugFlag1) std::cout << "  check n=" << n->getID()
    //                                            << " clusterDist=" << clusterDist
    //                                            << " cd<th=" << (clusterDist <= pedestrianFringeThreshold)
    //                                            << " touching=" << touchingCluster
    //                                            << " out=" << joinNamedToString(outsideNeighbors, ',')
    //                                            << " in=" << joinNamedToString(clusterNeighbors, ',')
    //                                            << " dia=" << getDiameter(cluster)
    //                                            << "\n";
    // #endif
    //             if (clusterNeighbors.size() == 0
    //                     || (outsideNeighbors.size() <= 1
    //                         && clusterNeighbors.size() == 1
    //                         && !(n->isTLControlled() /*|| n->hadSignal()*/))) {
    //                 cluster.erase(check);
    //                 pruneFringe = true; // other nodes could belong to the fringe now
    // #ifdef DEBUG_JOINJUNCTIONS
    //                 if (gDebugFlag1) {
    //                     std::cout << "  pruned n=" << n->getID() << "\n";
    //                 }
    // #endif
    //             } else if (outsideNeighbors.size() <= 1 && clusterNeighbors.size() == 1) {
    //                 geometryLikeTLS.insert(n);
    //             }
    //         }
    //         if (!pruneFringe && !pruneNoisyFringe) {
    //             // run once more and prune more things (with a look at cluster size)
    //             pruneFringe = true;
    //             pruneNoisyFringe = true;
    //
    //         }
    //     }
    //     if (remove2TLS && geometryLikeTLS.size() == cluster.size()) {
    //         cluster.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn prune_cluster_fringe(&self, cluster: &mut NodeSet, maxDist: f64, remove2TLS: bool) {
        // iteratively remove the fringe
        let mut geometryLikeTLS: NodeSet = NodeSet::new();
        let mut pruneFringe = true;
        let mut pruneNoisyFringe = false;
        // collect nodes that shall be joined due to distance but are not connected
        // to the cluster for passenger traffic
        while pruneFringe {
            pruneFringe = false;
            // C++ iterates the live set and erases only the current element, so a snapshot visits the same nodes
            for n in cluster.iter().copied().collect::<Vec<_>>() {
                // compute clusterDist for node (length of shortest edge which connects this node to the cluster)
                let mut clusterDist = f64::MAX;
                let mut touchingCluster = false;
                for edge in n.m().get_outgoing_edges().clone() {
                    let neighbor = edge.m().get_to_node();
                    if cluster.contains(&neighbor) {
                        let len = edge.m().get_loaded_length();
                        clusterDist = if clusterDist < len { clusterDist } else { len };
                        touchingCluster |= n.m().get_position().distance_to2d(neighbor.m().get_position()) <= SUMO_CONST_LANE_WIDTH;
                    }
                }
                for edge in n.m().get_incoming_edges().clone() {
                    let neighbor = edge.m().get_from_node();
                    if cluster.contains(&neighbor) {
                        let len = edge.m().get_loaded_length();
                        clusterDist = if clusterDist < len { clusterDist } else { len };
                        touchingCluster |= n.m().get_position().distance_to2d(neighbor.m().get_position()) <= SUMO_CONST_LANE_WIDTH;
                    }
                }
                // remove geometry-like nodes at fringe of the cluster
                // (they have 1 neighbor in the cluster and at most 1 neighbor outside the cluster)
                let mut outsideNeighbors: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
                let mut clusterNeighbors: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
                let pedestrianFringeThreshold = 0.3;
                for e in n.m().get_edges().clone() {
                    let neighbor = if e.m().get_from_node() == n { e.m().get_to_node() } else { e.m().get_from_node() };
                    if !cluster.contains(&neighbor) {
                        let perm = e.m().get_permissions(-1);
                        if (perm & SVC_PASSENGER) != 0
                                || utils::common::sumo_vehicle_class::is_railway(perm) // join railway crossings
                                || (clusterDist <= pedestrianFringeThreshold
                                    && (!pruneNoisyFringe
                                        || utils::common::sumo_vehicle_class::is_for_vulnerable_modes(perm)
                                        // permit joining small opposite merges
                                        || NBNodeCont::get_diameter(cluster) < maxDist
                                        || cluster.len() == 2))
                                || touchingCluster {
                            outsideNeighbors.insert(neighbor);
                        }
                    } else {
                        clusterNeighbors.insert(neighbor);
                    }
                }
                if clusterNeighbors.len() == 0
                        || (outsideNeighbors.len() <= 1
                            && clusterNeighbors.len() == 1
                            && !n.m().is_tl_controlled()) {
                    cluster.remove(&n);
                    pruneFringe = true; // other nodes could belong to the fringe now
                } else if outsideNeighbors.len() <= 1 && clusterNeighbors.len() == 1 {
                    geometryLikeTLS.insert(n);
                }
            }
            if !pruneFringe && !pruneNoisyFringe {
                // run once more and prune more things (with a look at cluster size)
                pruneFringe = true;
                pruneNoisyFringe = true;
            }
        }
        if remove2TLS && geometryLikeTLS.len() == cluster.len() {
            cluster.clear();
        }
    }

    /// C++: `int NBNodeCont::pruneLongEdges(NodeSet &, double, const bool)` [static, private]
    /// avoid removal of long edges when joining junction clusters
    // int
    // NBNodeCont::pruneLongEdges(NodeSet& cluster, double maxDist, const bool dryRun) {
    //     std::set<NBNode*> toRemove;
    //     int maxPassengerLanes = 0;
    //     for (NBNode* n : cluster) {
    //         for (NBEdge* edge : n->getEdges()) {
    //             maxPassengerLanes = MAX2(maxPassengerLanes, edge->getNumLanesThatAllow(SVC_PASSENGER));
    //         }
    //     }
    //     for (NBNode* n : cluster) {
    //         for (NBEdge* edge : n->getOutgoingEdges()) {
    //             // we must track the edge length across geometry like nodes
    //             // Also, intersections that are geometry-like
    //             // from the perspective of passenger traffic should be tracked across
    //             std::vector<NBNode*> passed;
    //             double length = 0;
    //             NBEdge* cur = edge;
    //             NBNode* to = edge->getToNode();
    //             while (cluster.count(to) != 0) {
    //                 length += cur->getLoadedLength();
    //                 bool goStraight = (std::find(passed.begin(), passed.end(), to) == passed.end()
    //                                    && (edge->getPermissions() & SVC_PASSENGER) != 0
    //                                    && to->geometryLike(
    //                                        NBEdge::filterByPermissions(to->getIncomingEdges(), SVC_PASSENGER),
    //                                        NBEdge::filterByPermissions(to->getOutgoingEdges(), SVC_PASSENGER)));
    //                 passed.push_back(to);
    //                 if (goStraight) {
    //                     cur = cur->getStraightContinuation(SVC_PASSENGER);
    //                     if (cur != nullptr) {
    //                         to = cur->getToNode();
    //                     } else {
    //                         break;
    //                     }
    //                 } else {
    //                     break;
    //                 }
    //             }
    //             // allow higher threshold at larger junctions
    //             double longThreshold = maxDist + SUMO_const_laneWidth * MAX2(0, maxPassengerLanes - 1);
    // #ifdef DEBUG_JOINJUNCTIONS
    //             if (gDebugFlag1) {
    //                 std::cout << "check edge length " << edge->getID() << " (" << length << ", passed=" << passed.size() << ", max=" << longThreshold << ")\n";
    //             }
    // #endif
    //             if (length > longThreshold) {
    //                 // we found an edge that should not be removed. Maybe we can
    //                 // still keep the start or end in the cluster
    //                 // (keep the start if the end can be removed and vice versa)
    //                 const bool keepStart = getClusterNeighbors(passed.back(), longThreshold, cluster).size() == 1;
    //                 const bool keepEnd = !keepStart && getClusterNeighbors(n, longThreshold, cluster).size() == 1;
    // #ifdef DEBUG_JOINJUNCTIONS
    //                 if (gDebugFlag1) {
    //                     std::cout << "node=" << n->getID() << " long edge " << edge->getID() << " (" << length << ", passed=" << toString(passed) << ", max=" << longThreshold << ") keepStart=" << keepStart << " keepEnd=" << keepEnd << "\n";
    //                 }
    // #endif
    //                 if (!keepStart) {
    //                     toRemove.insert(n);
    //                 }
    //                 toRemove.insert(passed.begin(), passed.end() - 1);
    //                 if (!keepEnd) {
    //                     toRemove.insert(passed.back());
    //                 }
    //
    //             }
    //         }
    //     }
    //     if (!dryRun) {
    //         for (std::set<NBNode*>::iterator j = toRemove.begin(); j != toRemove.end(); ++j) {
    //             cluster.erase(*j);
    //         }
    //     }
    //     return (int)toRemove.size();
    // }
    #[allow(unused_variables)]
    pub fn prune_long_edges(cluster: &mut NodeSet, maxDist: f64, dryRun: bool) -> i32 {
        let mut toRemove: NodeSet = NodeSet::new();
        let mut maxPassengerLanes: i32 = 0;
        for n in cluster.iter().copied().collect::<Vec<_>>() {
            for edge in n.m().get_edges().clone() {
                maxPassengerLanes = if maxPassengerLanes > edge.m().get_num_lanes_that_allow(SVC_PASSENGER, true) { maxPassengerLanes } else { edge.m().get_num_lanes_that_allow(SVC_PASSENGER, true) };
            }
        }
        for n in cluster.iter().copied().collect::<Vec<_>>() {
            for edge in n.m().get_outgoing_edges().clone() {
                // we must track the edge length across geometry like nodes
                // Also, intersections that are geometry-like
                // from the perspective of passenger traffic should be tracked across
                let mut passed: Vec<Ptr<NBNode>> = Vec::new();
                let mut length: f64 = 0.;
                let mut cur = edge;
                let mut to = edge.m().get_to_node();
                while cluster.contains(&to) {
                    length += cur.m().get_loaded_length();
                    let goStraight = !passed.contains(&to)
                        && (edge.m().get_permissions(-1) & SVC_PASSENGER) != 0
                        && NBNode::geometry_like_2(
                            &NBEdge::filter_by_permissions(&to.m().get_incoming_edges().clone(), SVC_PASSENGER),
                            &NBEdge::filter_by_permissions(&to.m().get_outgoing_edges().clone(), SVC_PASSENGER));
                    passed.push(to);
                    if goStraight {
                        cur = cur.m().get_straight_continuation(SVC_PASSENGER);
                        if !cur.is_null() {
                            to = cur.m().get_to_node();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                // allow higher threshold at larger junctions
                let longThreshold = maxDist + SUMO_CONST_LANE_WIDTH * (if 0 > maxPassengerLanes - 1 { 0 } else { maxPassengerLanes - 1 }) as f64;
                if length > longThreshold {
                    // we found an edge that should not be removed. Maybe we can
                    // still keep the start or end in the cluster
                    // (keep the start if the end can be removed and vice versa)
                    let last = *passed.last().unwrap();
                    let keepStart = NBNodeCont::get_cluster_neighbors(last, longThreshold, cluster).len() == 1;
                    let keepEnd = !keepStart && NBNodeCont::get_cluster_neighbors(n, longThreshold, cluster).len() == 1;
                    if !keepStart {
                        toRemove.insert(n);
                    }
                    for p in passed[..passed.len() - 1].iter() {
                        toRemove.insert(*p);
                    }
                    if !keepEnd {
                        toRemove.insert(last);
                    }
                }
            }
        }
        if !dryRun {
            for j in toRemove.iter() {
                cluster.remove(j);
            }
        }
        toRemove.len() as i32
    }

    /// C++: `double NBNodeCont::getDiameter(const NodeSet &)` [static, private]
    /// compute the maximum distance between any two cluster nodes
    // double
    // NBNodeCont::getDiameter(const NodeSet& cluster) {
    //     double result = 0;
    //     for (const NBNode* n1 : cluster) {
    //         for (const NBNode* n2 : cluster) {
    //             result = MAX2(result, n1->getPosition().distanceTo2D(n2->getPosition()));
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_diameter(cluster: &NodeSet) -> f64 {
        let mut result: f64 = 0.;
        for n1 in cluster.iter() {
            for n2 in cluster.iter() {
                // MAX2(result, distance)
                let d = n1.get_position().distance_to2d(n2.get_position());
                result = if result > d { result } else { d };
            }
        }
        result
    }

    /// C++: `bool NBNodeCont::geometryLikeForClass(const NBNode *, SVCPermissions)` [static, private]
    /// check whether the node is geometryLike when only considering edges that support the given permissions
    // bool
    // NBNodeCont::geometryLikeForClass(const NBNode* n, SVCPermissions ignored) {
    //     EdgeVector allowedIn;
    //     EdgeVector allowedOut;
    //     for (NBEdge* e : n->getIncomingEdges()) {
    //         if ((e->getPermissions() & ~ignored) != 0) {
    //             allowedIn.push_back(e);
    //         }
    //     }
    //     for (NBEdge* e : n->getOutgoingEdges()) {
    //         if ((e->getPermissions() & ~ignored) != 0) {
    //             allowedOut.push_back(e);
    //         }
    //     }
    //     if (allowedIn.size() > 0 && allowedOut.size() > 0) {
    //         //std::cout << n->getID() << " geometryLikeForClass=" << n->geometryLike(allowedIn, allowedOut) << " in=" << toString(allowedIn) << " out=" << toString(allowedOut) << "\n";
    //         return n->geometryLike(allowedIn, allowedOut);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn geometry_like_for_class(n: Ptr<NBNode>, permissions: SVCPermissions) -> bool {
        let ignored = permissions;
        let mut allowedIn: EdgeVector = Vec::new();
        let mut allowedOut: EdgeVector = Vec::new();
        for e in n.m().get_incoming_edges().clone() {
            if (e.m().get_permissions(-1) & !ignored) != 0 {
                allowedIn.push(e);
            }
        }
        for e in n.m().get_outgoing_edges().clone() {
            if (e.m().get_permissions(-1) & !ignored) != 0 {
                allowedOut.push(e);
            }
        }
        if allowedIn.len() > 0 && allowedOut.len() > 0 {
            return NBNode::geometry_like_2(&allowedIn, &allowedOut);
        }
        true
    }

    /// C++: `void NBNodeCont::pruneSlipLaneNodes(NodeSet &, double) const` [const, private]
    /// remove nodes that form a slip lane from cluster
    // void
    // NBNodeCont::pruneSlipLaneNodes(NodeSet& cluster, double maxDist) const {
    // #ifdef DEBUG_JOINJUNCTIONS
    //     if (gDebugFlag1) {
    //         std::cout << "pruning slip-lanes at cluster=" << joinNamedToString(cluster, ' ') << "\n";
    //     }
    // #endif
    //     // fringe has already been removed
    //     if (cluster.size() <= 2) {
    //         return;
    //     }
    //     NodeSet toRemove;
    //     for (NBNode* n : cluster) {
    //         EdgeVector outgoing;
    //         double inAngle;
    //         // find slip lanes where the start is part of the cluster
    //         if (maybeSlipLaneStart(n, outgoing, inAngle)) {
    //             // potential slip lane start but we don't know which of the outgoing edges it is
    // #ifdef DEBUG_JOINJUNCTIONS
    //             if (gDebugFlag1) {
    //                 std::cout << "   candidate slip-lane start=" << n->getID() << " outgoing=" << toString(outgoing) << "\n";
    //             }
    // #endif
    //             for (NBEdge* contEdge : outgoing) {
    //                 if ((contEdge->getPermissions() & SVC_PASSENGER) == 0) {
    //                     continue;
    //                 }
    //                 double slipLength = contEdge->getLength();
    //                 NBNode* cont = contEdge->getToNode();
    //                 NodeSet cands;
    //                 cands.insert(n);
    //                 while (isSlipLaneContinuation(cont) && slipLength < MAX_SLIPLANE_LENGTH) {
    //                     if (cands.count(cont) != 0) {
    //                         break; // circle, should not happen
    //                     }
    //                     cands.insert(cont);
    // #ifdef DEBUG_JOINJUNCTIONS
    //                     if (gDebugFlag1) {
    //                         std::cout << "   candidate slip-lane cont=" << cont->getID() << "\n";
    //                     }
    // #endif
    //                     NBEdge* next = cont->getOutgoingEdges().front();
    //                     slipLength += next->getLength();
    //                     cont = next->getToNode();
    //                 }
    // #ifdef DEBUG_JOINJUNCTIONS
    //                 if (gDebugFlag1) {
    //                     std::cout << "   candidate slip-lane end=" << cont->getID() << " slipLength=" << slipLength << "\n";
    //                 }
    // #endif
    //                 if (cont->getIncomingEdges().size() >= 2 && cont->getOutgoingEdges().size() == 1 &&
    //                         // slip lanes are for turning so there needs to be a sufficient angle
    //                         abs(NBHelpers::relAngle(inAngle, cont->getOutgoingEdges().front()->getAngleAtNode(cont))) > 45) {
    //                     // check whether the other continuation at n is also connected to the sliplane end
    //                     const NBEdge* const otherEdge = (contEdge == outgoing.front() ? outgoing.back() : outgoing.front());
    //                     NodeSet visited;
    //                     visited.insert(n);
    //                     std::vector<NodeAndDist> toProc;
    //                     toProc.push_back(std::make_pair(otherEdge->getToNode(), otherEdge->getLength()));
    //                     bool found = false;
    //                     while (!toProc.empty()) {
    //                         NodeAndDist nodeAndDist = toProc.back();
    //                         NBNode* cont2 = nodeAndDist.first;
    //                         double dist = nodeAndDist.second;
    // #ifdef DEBUG_JOINJUNCTIONS
    //                         if (gDebugFlag1) {
    //                             std::cout << "   search alternative cont2=" << cont2->getID() << " dist=" << dist << "\n";
    //                         }
    // #endif
    //                         toProc.pop_back();
    //                         if (visited.find(cont2) != visited.end()) {
    //                             continue;
    //                         }
    //                         visited.insert(cont2);
    //                         if (cont2 == cont) {
    //                             found = true;
    //                             break;
    //                         }
    //                         for (NBEdge* e : cont2->getOutgoingEdges()) {
    //                             const double dist2 = dist + e->getLength();
    //                             if (dist2 < slipLength * 2 && (e->getPermissions() & SVC_PASSENGER) != 0) {
    //                                 toProc.push_back(std::make_pair(e->getToNode(), dist2));
    //                             }
    //                         }
    //                     }
    //                     if (found) {
    //                         // found slip lane
    //                         cands.insert(cont);
    //                         toRemove.insert(cands.begin(), cands.end());
    // #ifdef DEBUG_JOINJUNCTIONS
    //                         if (gDebugFlag1) {
    //                             std::cout << "   found slip-lane with nodes=" << joinNamedToString(cands, ' ') << "\n";
    //                         }
    // #endif
    //                     }
    //                 }
    //             }
    //         }
    //
    //         EdgeVector incoming;
    //         double outAngle;
    //         // find slip lanes where the end is part of the cluster
    //         if (maybeSlipLaneEnd(n, incoming, outAngle)) {
    //             // potential slip lane end but we don't know which of the incoming edges it is
    // #ifdef DEBUG_JOINJUNCTIONS
    //             if (gDebugFlag1) {
    //                 std::cout << "   candidate slip-lane end=" << n->getID() << " incoming=" << toString(incoming) << "\n";
    //             }
    // #endif
    //             for (NBEdge* contEdge : incoming) {
    //                 if ((contEdge->getPermissions() & SVC_PASSENGER) == 0) {
    //                     continue;
    //                 }
    //                 double slipLength = contEdge->getLength();
    //                 NBNode* cont = contEdge->getFromNode();
    //                 NodeSet cands;
    //                 cands.insert(n);
    //                 while (isSlipLaneContinuation(cont) && slipLength < MAX_SLIPLANE_LENGTH) {
    //                     if (cands.count(cont) != 0) {
    //                         break; // circle, should not happen
    //                     }
    //                     cands.insert(cont);
    // #ifdef DEBUG_JOINJUNCTIONS
    //                     if (gDebugFlag1) {
    //                         std::cout << "   candidate slip-lane cont=" << cont->getID() << "\n";
    //                     }
    // #endif
    //                     NBEdge* next = cont->getIncomingEdges().front();
    //                     slipLength += next->getLength();
    //                     cont = next->getFromNode();
    //                 }
    // #ifdef DEBUG_JOINJUNCTIONS
    //                 if (gDebugFlag1) {
    //                     std::cout << "   candidate slip-lane start=" << cont->getID() << " slipLength=" << slipLength << "\n";
    //                 }
    // #endif
    //                 if (cont->getOutgoingEdges().size() >= 2 && cont->getIncomingEdges().size() == 1 &&
    //                         // slip lanes are for turning so there needs to be a sufficient angle
    //                         abs(NBHelpers::relAngle(outAngle, cont->getIncomingEdges().front()->getAngleAtNode(cont))) > 45) {
    //                     // check whether the other continuation at n is also connected to the sliplane end
    //                     const NBEdge* const otherEdge = (contEdge == incoming.front() ? incoming.back() : incoming.front());
    //                     NodeSet visited;
    //                     visited.insert(n);
    //                     std::vector<NodeAndDist> toProc;
    //                     toProc.push_back(std::make_pair(otherEdge->getFromNode(), otherEdge->getLength()));
    //                     bool found = false;
    //                     while (!toProc.empty()) {
    //                         NodeAndDist nodeAndDist = toProc.back();
    //                         NBNode* cont2 = nodeAndDist.first;
    //                         double dist = nodeAndDist.second;
    // #ifdef DEBUG_JOINJUNCTIONS
    //                         if (gDebugFlag1) {
    //                             std::cout << "   search alternative cont2=" << cont2->getID() << " dist=" << dist << "\n";
    //                         }
    // #endif
    //                         toProc.pop_back();
    //                         if (visited.find(cont2) != visited.end()) {
    //                             continue;
    //                         }
    //                         visited.insert(cont2);
    //                         if (cont2 == cont) {
    //                             found = true;
    //                             break;
    //                         }
    //                         for (NBEdge* e : cont2->getIncomingEdges()) {
    //                             const double dist2 = dist + e->getLength();
    //                             if (dist2 < slipLength * 2 && (e->getPermissions() & SVC_PASSENGER) != 0) {
    //                                 toProc.push_back(std::make_pair(e->getFromNode(), dist2));
    //                             }
    //                         }
    //                     }
    //                     if (found) {
    //                         // found slip lane
    //                         cands.insert(cont);
    //                         toRemove.insert(cands.begin(), cands.end());
    // #ifdef DEBUG_JOINJUNCTIONS
    //                         if (gDebugFlag1) {
    //                             std::cout << "   found slip-lane start with nodes=" << joinNamedToString(cands, ' ') << "\n";
    //                         }
    // #endif
    //                     }
    //                 }
    //             }
    //         }
    //
    //
    //
    //     }
    //     int numRemoved = 0;
    //     for (NBNode* n : toRemove) {
    //         numRemoved += (int)cluster.erase(n);
    //     }
    //     if (numRemoved > 0) {
    // #ifdef DEBUG_JOINJUNCTIONS
    //         if (gDebugFlag1) {
    //             std::cout << "   removed " << numRemoved << " nodes from cluster: " << joinNamedToString(toRemove, ' ') << "\n";
    //         }
    // #endif
    //         pruneClusterFringe(cluster, maxDist);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn prune_slip_lane_nodes(&self, cluster: &mut NodeSet, maxDist: f64) {
        const MAX_SLIPLANE_LENGTH: f64 = 1000.; // C++ #define MAX_SLIPLANE_LENGTH 1000
        // fringe has already been removed
        if cluster.len() <= 2 {
            return;
        }
        let mut toRemove: NodeSet = NodeSet::new();
        for n in cluster.iter().copied().collect::<Vec<_>>() {
            let mut outgoing: EdgeVector = Vec::new();
            let mut inAngle: f64 = 0.;
            // find slip lanes where the start is part of the cluster
            if self.maybe_slip_lane_start(n, &mut outgoing, &mut inAngle) {
                // potential slip lane start but we don't know which of the outgoing edges it is
                for contEdge in outgoing.iter().copied() {
                    if (contEdge.m().get_permissions(-1) & SVC_PASSENGER) == 0 {
                        continue;
                    }
                    let mut slipLength = contEdge.m().get_length();
                    let mut cont = contEdge.m().get_to_node();
                    let mut cands: NodeSet = NodeSet::new();
                    cands.insert(n);
                    while NBNodeCont::is_slip_lane_continuation(cont) && slipLength < MAX_SLIPLANE_LENGTH {
                        if cands.contains(&cont) {
                            break; // circle, should not happen
                        }
                        cands.insert(cont);
                        let next = cont.m().get_outgoing_edges()[0];
                        slipLength += next.m().get_length();
                        cont = next.m().get_to_node();
                    }
                    if cont.m().get_incoming_edges().len() >= 2 && cont.m().get_outgoing_edges().len() == 1
                            // slip lanes are for turning so there needs to be a sufficient angle
                            && NBHelpers::rel_angle(inAngle, cont.m().get_outgoing_edges()[0].m().get_angle_at_node(cont)).abs() > 45. {
                        // check whether the other continuation at n is also connected to the sliplane end
                        let otherEdge = if contEdge == outgoing[0] { outgoing[outgoing.len() - 1] } else { outgoing[0] };
                        let mut visited: NodeSet = NodeSet::new();
                        visited.insert(n);
                        let mut toProc: Vec<(Ptr<NBNode>, f64)> = vec![(otherEdge.m().get_to_node(), otherEdge.m().get_length())];
                        let mut found = false;
                        while let Some((cont2, dist)) = toProc.pop() {
                            if visited.contains(&cont2) {
                                continue;
                            }
                            visited.insert(cont2);
                            if cont2 == cont {
                                found = true;
                                break;
                            }
                            for e in cont2.m().get_outgoing_edges().clone() {
                                let dist2 = dist + e.m().get_length();
                                if dist2 < slipLength * 2. && (e.m().get_permissions(-1) & SVC_PASSENGER) != 0 {
                                    toProc.push((e.m().get_to_node(), dist2));
                                }
                            }
                        }
                        if found {
                            // found slip lane
                            cands.insert(cont);
                            toRemove.extend(cands.iter().copied());
                        }
                    }
                }
            }

            let mut incoming: EdgeVector = Vec::new();
            let mut outAngle: f64 = 0.;
            // find slip lanes where the end is part of the cluster
            if self.maybe_slip_lane_end(n, &mut incoming, &mut outAngle) {
                // potential slip lane end but we don't know which of the incoming edges it is
                for contEdge in incoming.iter().copied() {
                    if (contEdge.m().get_permissions(-1) & SVC_PASSENGER) == 0 {
                        continue;
                    }
                    let mut slipLength = contEdge.m().get_length();
                    let mut cont = contEdge.m().get_from_node();
                    let mut cands: NodeSet = NodeSet::new();
                    cands.insert(n);
                    while NBNodeCont::is_slip_lane_continuation(cont) && slipLength < MAX_SLIPLANE_LENGTH {
                        if cands.contains(&cont) {
                            break; // circle, should not happen
                        }
                        cands.insert(cont);
                        let next = cont.m().get_incoming_edges()[0];
                        slipLength += next.m().get_length();
                        cont = next.m().get_from_node();
                    }
                    if cont.m().get_outgoing_edges().len() >= 2 && cont.m().get_incoming_edges().len() == 1
                            // slip lanes are for turning so there needs to be a sufficient angle
                            && NBHelpers::rel_angle(outAngle, cont.m().get_incoming_edges()[0].m().get_angle_at_node(cont)).abs() > 45. {
                        // check whether the other continuation at n is also connected to the sliplane end
                        let otherEdge = if contEdge == incoming[0] { incoming[incoming.len() - 1] } else { incoming[0] };
                        let mut visited: NodeSet = NodeSet::new();
                        visited.insert(n);
                        let mut toProc: Vec<(Ptr<NBNode>, f64)> = vec![(otherEdge.m().get_from_node(), otherEdge.m().get_length())];
                        let mut found = false;
                        while let Some((cont2, dist)) = toProc.pop() {
                            if visited.contains(&cont2) {
                                continue;
                            }
                            visited.insert(cont2);
                            if cont2 == cont {
                                found = true;
                                break;
                            }
                            for e in cont2.m().get_incoming_edges().clone() {
                                let dist2 = dist + e.m().get_length();
                                if dist2 < slipLength * 2. && (e.m().get_permissions(-1) & SVC_PASSENGER) != 0 {
                                    toProc.push((e.m().get_from_node(), dist2));
                                }
                            }
                        }
                        if found {
                            // found slip lane
                            cands.insert(cont);
                            toRemove.extend(cands.iter().copied());
                        }
                    }
                }
            }
        }
        let mut numRemoved: i32 = 0;
        for n in toRemove.iter() {
            if cluster.remove(n) {
                numRemoved += 1;
            }
        }
        if numRemoved > 0 {
            self.prune_cluster_fringe(cluster, maxDist, false);
        }
    }

    /// C++: `bool NBNodeCont::feasibleCluster(const NodeSet &, const std::map<const NBNode *, std::vector<NBNode *>> &, double, std::string &, NBNode *&) const` [const, private]
    /// determine whether the cluster is not too complex for joining
    // bool
    // NBNodeCont::feasibleCluster(const NodeSet& cluster, const std::map<const NBNode*, std::vector<NBNode*> >& ptStopEnds,
    //                             double maxDist, std::string& reason, NBNode*& tryRemove) const {
    //     // check for clusters which are to complex and probably won't work very well
    //     // we count the incoming edges of the final junction
    //     std::map<NBEdge*, double, ComparatorIdLess> finalIncomingAngles;
    //     std::map<NBEdge*, double, ComparatorIdLess> finalOutgoingAngles;
    //     for (NBNode* n : cluster) {
    //         for (EdgeVector::const_iterator it_edge = n->getIncomingEdges().begin(); it_edge != n->getIncomingEdges().end(); ++it_edge) {
    //             NBEdge* edge = *it_edge;
    //             if (cluster.count(edge->getFromNode()) == 0 && (edge->getPermissions() & SVC_PASSENGER) != 0) {
    //                 // incoming edge, does not originate in the cluster
    //                 finalIncomingAngles[edge] = edge->getAngleAtNode(edge->getToNode());
    //             }
    //         }
    //         for (EdgeVector::const_iterator it_edge = n->getOutgoingEdges().begin(); it_edge != n->getOutgoingEdges().end(); ++it_edge) {
    //             NBEdge* edge = *it_edge;
    //             if (cluster.count(edge->getToNode()) == 0 && (edge->getPermissions() & SVC_PASSENGER) != 0) {
    //                 // outgoing edge, does not end in the cluster
    //                 finalOutgoingAngles[edge] = edge->getAngleAtNode(edge->getFromNode());
    //             }
    //         }
    //
    //     }
    // #ifdef DEBUG_JOINJUNCTIONS
    //     for (NBNode* n : cluster) {
    //         if (DEBUGCOND(n)) {
    //             std::cout << "feasibleCluster c=" << joinNamedToString(cluster, ',')
    //                       << "\n inAngles=" << joinNamedToString(finalIncomingAngles, ' ', ':')
    //                       << "\n outAngles=" << joinNamedToString(finalOutgoingAngles, ' ', ':')
    //                       << "\n";
    //         }
    //     }
    // #endif
    //     if (finalIncomingAngles.size() > 5) {
    //         reason = toString(finalIncomingAngles.size()) + " incoming edges";
    //         return false;
    //     }
    //     // check for incoming parallel edges
    //     const double PARALLEL_THRESHOLD_DIFF_NODE = OptionsCont::getOptions().getFloat("junctions.join.parallel-threshold");
    //     const double PARALLEL_THRESHOLD_SAME_NODE = PARALLEL_THRESHOLD_DIFF_NODE / 3;
    //     bool foundParallel = false;
    //     for (auto j = finalIncomingAngles.begin(); j != finalIncomingAngles.end() && !foundParallel; ++j) {
    //         auto k = j;
    //         for (++k; k != finalIncomingAngles.end() && !foundParallel; ++k) {
    //             const double angleDiff = fabs(j->second - k->second);
    //             if (angleDiff < PARALLEL_THRESHOLD_DIFF_NODE) {
    //                 NBEdge* e1 = j->first;
    //                 NBEdge* e2 = k->first;
    //                 // for edge targeting the same node, permit a narrower angle
    //                 const double edgeDist = e1->getLaneShape(0).back().distanceTo2D(e2->getLaneShape(0).back());
    // #ifdef DEBUG_JOINJUNCTIONS
    //                 if (DEBUGCOND(e1->getToNode())) {
    //                     std::cout << " angleDiff=" << angleDiff << " shapeDist=" << edgeDist << "\n";
    //                 }
    // #endif
    //                 if (angleDiff >= PARALLEL_THRESHOLD_SAME_NODE && (
    //                             (e1->getToNode() == e2->getToNode()
    //                              || (edgeDist  < maxDist)))) {
    //                     continue;
    //                 }
    //                 reason = "parallel incoming " + e1->getID() + "," + e2->getID();
    //                 if (e1->getToNode() != e2->getToNode() && (int)cluster.size() > 2) {
    //                     // removing one of the nodes and try again
    //                     if (e1->getPriority() > e2->getPriority()
    //                             || (e1->getPriority() == e2->getPriority() && e1->getNumLanesThatAllow(SVC_PASSENGER) > e2->getNumLanesThatAllow(SVC_PASSENGER))) {
    //                         tryRemove = e2->getToNode();
    //                     } else {
    //                         tryRemove = e1->getToNode();
    //                     }
    //                 }
    //                 return false;
    //             }
    //         }
    //     }
    //     // check for outgoing parallel edges
    //     for (auto j = finalOutgoingAngles.begin(); j != finalOutgoingAngles.end() && !foundParallel; ++j) {
    //         auto k = j;
    //         for (++k; k != finalOutgoingAngles.end() && !foundParallel; ++k) {
    //             const double angleDiff = fabs(j->second - k->second);
    //             if (angleDiff < PARALLEL_THRESHOLD_DIFF_NODE) {
    //                 NBEdge* e1 = j->first;
    //                 NBEdge* e2 = k->first;
    //                 // for edge leaving the same node, permit a narrower angle
    //                 const double edgeDist = e1->getLaneShape(0).front().distanceTo2D(e2->getLaneShape(0).front());
    // #ifdef DEBUG_JOINJUNCTIONS
    //                 if (DEBUGCOND(e1->getFromNode())) {
    //                     std::cout << " angleDiff=" << angleDiff << " shapeDist=" << edgeDist << "\n";
    //                 }
    // #endif
    //                 if (angleDiff >= PARALLEL_THRESHOLD_SAME_NODE && (
    //                             (e1->getFromNode() == e2->getFromNode()
    //                              || (edgeDist < maxDist)))) {
    //                     continue;
    //                 }
    //                 reason = "parallel outgoing " + e1->getID() + "," + e2->getID();
    //                 if (e1->getFromNode() != e2->getFromNode() && (int)cluster.size() > 2) {
    //                     // removing one of the nodes and try again
    //                     if (e1->getPriority() > e2->getPriority()
    //                             || (e1->getPriority() == e2->getPriority() && e1->getNumLanesThatAllow(SVC_PASSENGER) > e2->getNumLanesThatAllow(SVC_PASSENGER))) {
    //                         tryRemove = e2->getFromNode();
    //                     } else {
    //                         tryRemove = e1->getFromNode();
    //                     }
    //                 }
    //                 return false;
    //             }
    //         }
    //     }
    //     // check for stop edges and tls within the cluster
    //     bool hasTLS = false;
    //     for (NBNode* n : cluster) {
    //         if (n->isTLControlled() || n->hadSignal()) {
    //             hasTLS = true;
    //         }
    //         const auto& stopEnds = ptStopEnds.find(n);
    //         if (stopEnds != ptStopEnds.end()) {
    //             for (NBNode* const to : stopEnds->second) {
    //                 if (cluster.count(to) != 0) {
    //                     reason = "it contains a pt stop edge";
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     // prevent removal of long edges unless there is weak circle or a traffic light
    //     if (cluster.size() > 2) {
    //         // find the nodes with the biggests physical distance between them
    //         double maxLength = -1;
    //         NBEdge* maxEdge = nullptr;
    //         for (NBNode* n1 : cluster) {
    //             for (NBNode* n2 : cluster) {
    //                 NBEdge* e1 = n1->getConnectionTo(n2);
    //                 NBEdge* e2 = n2->getConnectionTo(n1);
    //                 if (e1 != nullptr && e1->getLoadedLength() > maxLength) {
    //                     maxLength = e1->getLoadedLength();
    //                     maxEdge = e1;
    //                 }
    //                 if (e2 != nullptr && e2->getLoadedLength() > maxLength) {
    //                     maxLength = e2->getLoadedLength();
    //                     maxEdge = e2;
    //                 }
    //             }
    //         }
    // #ifdef DEBUG_JOINJUNCTIONS
    //         for (NBNode* n : cluster) {
    //             if (DEBUGCOND(n)) {
    //                 std::cout << "feasible hasTLS=" << hasTLS << " maxLength=" << maxLength << " maxEdge=" << maxEdge->getID() << "\n";
    //             }
    //         }
    // #endif
    //         if (!hasTLS && maxLength > 5) {
    //             // find a weak circle within cluster that does not use maxEdge
    //             std::vector<NBNode*> toCheck;
    //             std::set<NBNode*> visited;
    //             toCheck.push_back(maxEdge->getToNode());
    //             bool foundCircle = false;
    //             while (!toCheck.empty()) {
    //                 NBNode* n = toCheck.back();
    //                 if (n == maxEdge->getFromNode()) {
    //                     foundCircle = true;
    //                     break;
    //                 }
    //                 toCheck.pop_back();
    //                 visited.insert(n);
    //                 for (NBEdge* e : n->getEdges()) {
    //                     if (e != maxEdge) {
    //                         NBNode* cand = e->getFromNode() == n ? e->getToNode() : e->getFromNode();
    //                         if (visited.count(cand) == 0 && cluster.count(cand) != 0) {
    //                             toCheck.push_back(cand);
    //                         }
    //                     }
    //                 }
    //             }
    //             if (!foundCircle) {
    //                 reason = "not compact (maxEdge=" + maxEdge->getID() + " length=" + toString(maxLength) + ")";
    //                 return false;
    //             }
    //         }
    //     }
    //     // prevent joining of simple merging/spreading structures
    //     if (cluster.size() >= 2) {
    //         int entryNodes = 0;
    //         int exitNodes = 0;
    //         EdgeVector outsideIncoming;
    //         EdgeVector outsideOutgoing;
    //         int edgesWithin = 0;
    //         for (NBNode* n : cluster) {
    //             bool foundOutsideIncoming = false;
    //             for (NBEdge* e : n->getIncomingEdges()) {
    //                 if (cluster.count(e->getFromNode()) == 0) {
    //                     // edge entering from outside the cluster
    //                     outsideIncoming.push_back(e);
    //                     foundOutsideIncoming = true;
    //                 } else {
    //                     edgesWithin++;
    //                 }
    //             }
    //             if (foundOutsideIncoming) {
    //                 entryNodes++;
    //             }
    //             bool foundOutsideOutgoing = false;
    //             for (NBEdge* e : n->getOutgoingEdges()) {
    //                 if (cluster.count(e->getToNode()) == 0) {
    //                     // edge leaving cluster
    //                     outsideOutgoing.push_back(e);
    //                     foundOutsideOutgoing = true;
    //                 }
    //             }
    //             if (foundOutsideOutgoing) {
    //                 exitNodes++;
    //             }
    //         }
    //         if (!hasTLS) {
    //             if (entryNodes < 2) {
    //                 reason = "only 1 entry node";
    //                 return false;
    //             }
    //             if (exitNodes < 2) {
    //                 reason = "only 1 exit node";
    //                 return false;
    //             }
    //             if (cluster.size() == 2) {
    //                 if (edgesWithin == 1 && outsideIncoming.size() < 3 && outsideOutgoing.size() < 3) {
    //                     reason = "only 1 edge within and no cross-traffic";
    //                     return false;
    //                 }
    //             }
    //         }
    //         /*
    //         if (NBNode::geometryLike(outsideIncoming, outsideOutgoing) && hasTLS && OptionsCont::getOptions().getBool("tls.discard-simple")) {
    //             reason = "geometry-like simple tls";
    //             return false;
    //         }
    //         */
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn feasible_cluster(&self, cluster: &NodeSet, ptStopEnds: &BTreeMap<Ptr<NBNode>, Vec<Ptr<NBNode>>>, maxDist: f64, reason: &mut String, tryRemove: &mut Ptr<NBNode>) -> bool {
        // check for clusters which are to complex and probably won't work very well
        // we count the incoming edges of the final junction
        // (std::map<NBEdge*, double, ComparatorIdLess>: ordered by edge id)
        let mut finalIncomingAngles: BTreeMap<String, (Ptr<NBEdge>, f64)> = BTreeMap::new();
        let mut finalOutgoingAngles: BTreeMap<String, (Ptr<NBEdge>, f64)> = BTreeMap::new();
        for n in cluster.iter().copied() {
            for edge in n.m().get_incoming_edges().clone() {
                if !cluster.contains(&edge.m().get_from_node()) && (edge.m().get_permissions(-1) & SVC_PASSENGER) != 0 {
                    // incoming edge, does not originate in the cluster
                    let a = edge.m().get_angle_at_node(edge.m().get_to_node());
                    finalIncomingAngles.insert(edge.m().get_id().to_string(), (edge, a));
                }
            }
            for edge in n.m().get_outgoing_edges().clone() {
                if !cluster.contains(&edge.m().get_to_node()) && (edge.m().get_permissions(-1) & SVC_PASSENGER) != 0 {
                    // outgoing edge, does not end in the cluster
                    let a = edge.m().get_angle_at_node(edge.m().get_from_node());
                    finalOutgoingAngles.insert(edge.m().get_id().to_string(), (edge, a));
                }
            }
        }
        if finalIncomingAngles.len() > 5 {
            *reason = format!("{} incoming edges", finalIncomingAngles.len());
            return false;
        }
        // check for incoming parallel edges
        let PARALLEL_THRESHOLD_DIFF_NODE = utils::options::options_cont::get_float("junctions.join.parallel-threshold");
        let PARALLEL_THRESHOLD_SAME_NODE = PARALLEL_THRESHOLD_DIFF_NODE / 3.;
        let inVals: Vec<(Ptr<NBEdge>, f64)> = finalIncomingAngles.values().copied().collect();
        for j in 0..inVals.len() {
            for k in (j + 1)..inVals.len() {
                let (e1, a1) = inVals[j];
                let (e2, a2) = inVals[k];
                let angleDiff = (a1 - a2).abs();
                if angleDiff < PARALLEL_THRESHOLD_DIFF_NODE {
                    // for edge targeting the same node, permit a narrower angle
                    let edgeDist = e1.m().get_lane_shape(0).back().distance_to2d(e2.m().get_lane_shape(0).back());
                    if angleDiff >= PARALLEL_THRESHOLD_SAME_NODE && (e1.m().get_to_node() == e2.m().get_to_node() || edgeDist < maxDist) {
                        continue;
                    }
                    *reason = format!("parallel incoming {},{}", e1.m().get_id(), e2.m().get_id());
                    if e1.m().get_to_node() != e2.m().get_to_node() && cluster.len() > 2 {
                        // removing one of the nodes and try again
                        if e1.m().get_priority() > e2.m().get_priority()
                                || (e1.m().get_priority() == e2.m().get_priority() && e1.m().get_num_lanes_that_allow(SVC_PASSENGER, true) > e2.m().get_num_lanes_that_allow(SVC_PASSENGER, true)) {
                            *tryRemove = e2.m().get_to_node();
                        } else {
                            *tryRemove = e1.m().get_to_node();
                        }
                    }
                    return false;
                }
            }
        }
        // check for outgoing parallel edges
        let outVals: Vec<(Ptr<NBEdge>, f64)> = finalOutgoingAngles.values().copied().collect();
        for j in 0..outVals.len() {
            for k in (j + 1)..outVals.len() {
                let (e1, a1) = outVals[j];
                let (e2, a2) = outVals[k];
                let angleDiff = (a1 - a2).abs();
                if angleDiff < PARALLEL_THRESHOLD_DIFF_NODE {
                    // for edge leaving the same node, permit a narrower angle
                    let edgeDist = e1.m().get_lane_shape(0).front().distance_to2d(e2.m().get_lane_shape(0).front());
                    if angleDiff >= PARALLEL_THRESHOLD_SAME_NODE && (e1.m().get_from_node() == e2.m().get_from_node() || edgeDist < maxDist) {
                        continue;
                    }
                    *reason = format!("parallel outgoing {},{}", e1.m().get_id(), e2.m().get_id());
                    if e1.m().get_from_node() != e2.m().get_from_node() && cluster.len() > 2 {
                        // removing one of the nodes and try again
                        if e1.m().get_priority() > e2.m().get_priority()
                                || (e1.m().get_priority() == e2.m().get_priority() && e1.m().get_num_lanes_that_allow(SVC_PASSENGER, true) > e2.m().get_num_lanes_that_allow(SVC_PASSENGER, true)) {
                            *tryRemove = e2.m().get_from_node();
                        } else {
                            *tryRemove = e1.m().get_from_node();
                        }
                    }
                    return false;
                }
            }
        }
        // check for stop edges and tls within the cluster
        let mut hasTLS = false;
        for n in cluster.iter().copied() {
            if n.m().is_tl_controlled() || n.m().had_signal() {
                hasTLS = true;
            }
            if let Some(stopEnds) = ptStopEnds.get(&n) {
                for to in stopEnds.iter() {
                    if cluster.contains(to) {
                        *reason = "it contains a pt stop edge".to_string();
                        return false;
                    }
                }
            }
        }
        // prevent removal of long edges unless there is weak circle or a traffic light
        if cluster.len() > 2 {
            // find the nodes with the biggests physical distance between them
            let mut maxLength: f64 = -1.;
            let mut maxEdge: Ptr<NBEdge> = Ptr::from_raw(std::ptr::null_mut());
            for n1 in cluster.iter().copied() {
                for n2 in cluster.iter().copied() {
                    let e1 = n1.m().get_connection_to(n2);
                    let e2 = n2.m().get_connection_to(n1);
                    if !e1.is_null() && e1.m().get_loaded_length() > maxLength {
                        maxLength = e1.m().get_loaded_length();
                        maxEdge = e1;
                    }
                    if !e2.is_null() && e2.m().get_loaded_length() > maxLength {
                        maxLength = e2.m().get_loaded_length();
                        maxEdge = e2;
                    }
                }
            }
            if !hasTLS && maxLength > 5. {
                // find a weak circle within cluster that does not use maxEdge
                let mut toCheck: Vec<Ptr<NBNode>> = vec![maxEdge.m().get_to_node()];
                let mut visited: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
                let mut foundCircle = false;
                while let Some(&n) = toCheck.last() {
                    if n == maxEdge.m().get_from_node() {
                        foundCircle = true;
                        break;
                    }
                    toCheck.pop();
                    visited.insert(n);
                    for e in n.m().get_edges().clone() {
                        if e != maxEdge {
                            let cand = if e.m().get_from_node() == n { e.m().get_to_node() } else { e.m().get_from_node() };
                            if !visited.contains(&cand) && cluster.contains(&cand) {
                                toCheck.push(cand);
                            }
                        }
                    }
                }
                if !foundCircle {
                    *reason = format!("not compact (maxEdge={} length={})", maxEdge.m().get_id(), utils::common::to_string::to_string(&maxLength));
                    return false;
                }
            }
        }
        // prevent joining of simple merging/spreading structures
        if cluster.len() >= 2 {
            let mut entryNodes: i32 = 0;
            let mut exitNodes: i32 = 0;
            let mut outsideIncoming: EdgeVector = Vec::new();
            let mut outsideOutgoing: EdgeVector = Vec::new();
            let mut edgesWithin: i32 = 0;
            for n in cluster.iter().copied() {
                let mut foundOutsideIncoming = false;
                for e in n.m().get_incoming_edges().clone() {
                    if !cluster.contains(&e.m().get_from_node()) {
                        // edge entering from outside the cluster
                        outsideIncoming.push(e);
                        foundOutsideIncoming = true;
                    } else {
                        edgesWithin += 1;
                    }
                }
                if foundOutsideIncoming {
                    entryNodes += 1;
                }
                let mut foundOutsideOutgoing = false;
                for e in n.m().get_outgoing_edges().clone() {
                    if !cluster.contains(&e.m().get_to_node()) {
                        // edge leaving cluster
                        outsideOutgoing.push(e);
                        foundOutsideOutgoing = true;
                    }
                }
                if foundOutsideOutgoing {
                    exitNodes += 1;
                }
            }
            if !hasTLS {
                if entryNodes < 2 {
                    *reason = "only 1 entry node".to_string();
                    return false;
                }
                if exitNodes < 2 {
                    *reason = "only 1 exit node".to_string();
                    return false;
                }
                if cluster.len() == 2 {
                    if edgesWithin == 1 && outsideIncoming.len() < 3 && outsideOutgoing.len() < 3 {
                        *reason = "only 1 edge within and no cross-traffic".to_string();
                        return false;
                    }
                }
            }
        }
        true
    }

    /// C++: `void NBNodeCont::joinNodeClusters(NodeClusters, NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &, bool)` [private]
    /// joins the given node clusters
    // void
    // NBNodeCont::joinNodeClusters(NodeClusters clusters,
    //                              NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tlc, bool resetConnections) {
    //     for (NodeSet cluster : clusters) {
    //         joinNodeCluster(cluster, dc, ec, tlc, nullptr, resetConnections);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join_node_clusters(&mut self, clusters: NBNodeCont_NodeClusters, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, resetConnections: bool) {
        for cluster in clusters {
            self.join_node_cluster(cluster, dc, ec, tlc, Ptr::from_raw(std::ptr::null_mut()), resetConnections);
        }
    }

    /// C++: `void NBNodeCont::joinNodeCluster(NodeSet, NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &, NBNode *, bool)` [private]
    // void
    // NBNodeCont::joinNodeCluster(NodeSet cluster, NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tlc, NBNode* predefined, bool resetConnections) {
    //     const bool origNames = OptionsCont::getOptions().getBool("output.original-names");
    //     assert(cluster.size() > 1);
    //     std::string id = "cluster_";
    //     Position pos;
    //     bool setTL = false;
    //     SumoXMLNodeType nodeType = SumoXMLNodeType::UNKNOWN;
    //     TrafficLightType type = SUMOXMLDefinitions::TrafficLightTypes.get(OptionsCont::getOptions().getString("tls.default-type"));
    //     // collect edges
    //     std::set<NBEdge*, ComparatorIdLess> allEdges;
    //     for (NBNode* n : cluster) {
    //         const EdgeVector& edges = n->getEdges();
    //         allEdges.insert(edges.begin(), edges.end());
    //     }
    //     // determine edges with are incoming or fully inside
    //     std::set<NBEdge*, ComparatorIdLess> clusterIncoming;
    //     std::set<NBEdge*, ComparatorIdLess> inside;
    //     for (NBEdge* e : allEdges) {
    //         if (cluster.count(e->getToNode()) > 0) {
    //             if (cluster.count(e->getFromNode()) > 0) {
    //                 inside.insert(e);
    //                 if (e->getSignalPosition() != Position::INVALID) {
    //                     setTL = true;
    //                     nodeType = SumoXMLNodeType::TRAFFIC_LIGHT;
    //                 }
    //             } else {
    //                 clusterIncoming.insert(e);
    //             }
    //         }
    //     }
    // #ifdef DEBUG_JOINJUNCTIONS_CONNECTIONS
    //     std::cout << "joining cluster " << joinNamedToString(cluster, ' ')
    //               << "  resetConnections=" << resetConnections << "\n"
    //               << "  incoming=" << joinNamedToString(clusterIncoming, ' ') << "\n"
    //               << "  inside=" << joinNamedToString(inside, ' ') << "\n";
    // #endif
    //     analyzeCluster(cluster, id, pos, setTL, type, nodeType);
    //     NBNode* newNode = nullptr;
    //     if (predefined != nullptr) {
    //         newNode = predefined;
    //     } else {
    //         if (!insert(id, pos)) {
    //             // should not fail
    //             WRITE_WARNINGF(TL("Could not join junctions %."), id);
    //             return;
    //         }
    //         newNode = retrieve(id);
    //     }
    //     std::string tlID = id;
    //     if (predefined != nullptr) {
    //         if (predefined->getType() != SumoXMLNodeType::UNKNOWN) {
    //             nodeType = predefined->getType();
    //             setTL = predefined->isTLControlled();
    //         }
    //         Position ppos = predefined->getPosition();
    //         if (ppos.x() != Position::INVALID.x()) {
    //             pos.setx(ppos.x());
    //         }
    //         if (ppos.y() != Position::INVALID.y()) {
    //             pos.sety(ppos.y());
    //         }
    //         if (ppos.z() != Position::INVALID.z()) {
    //             pos.setz(ppos.z());
    //         }
    //     }
    //     newNode->reinit(pos, nodeType);
    //     if (origNames) {
    //         newNode->setParameter(SUMO_PARAM_ORIGID, joinNamedToString(cluster, ' '));
    //     }
    //     if (setTL && !newNode->isTLControlled()) {
    //         NBTrafficLightDefinition* tlDef = new NBOwnTLDef(tlID, newNode, 0, type);
    //         if (!tlc.insert(tlDef)) {
    //             // actually, nothing should fail here
    //             delete tlDef;
    //             throw ProcessError(TLF("Could not allocate tls '%'.", id));
    //         }
    //     }
    //
    //     // determine possible connectivity from outside edges
    //     std::map<NBEdge*, EdgeSet> reachable;
    //     std::map<std::pair<NBEdge*, NBEdge*>, SVCPermissions> conPermissions;
    //     EdgeSet specialPermissions;
    //     for (NBEdge* const e : clusterIncoming) {
    //         EdgeVector open;
    //         EdgeSet seen;
    //         open.push_back(e);
    //         while (open.size() > 0) {
    //             NBEdge* const cur = open.back();
    //             const SVCPermissions pCur = conPermissions.count({e, cur}) == 0 ? cur->getPermissions() : conPermissions[ {e, cur}];
    // #ifdef DEBUG_JOINJUNCTIONS_CONNECTIONS
    //             std::cout << "e=" << e->getID() << " cur=" << cur->getID() << " open=" << toString(open) << "\n";
    //             std::cout << "e=" << e->getID() << " cur=" << cur->getID() << " open=" << toString(open) << "\n";
    // #endif
    //             seen.insert(cur);
    //             open.pop_back();
    //             if (cluster.count(cur->getToNode()) == 0) {
    //                 //std::cout << "      continue\n";
    //                 continue;
    //             }
    //             const auto& cons = cur->getConnections();
    //             if (cons.size() == 0 || ec.hasPostProcessConnection(cur->getID()) || cur->getStep() == NBEdge::EdgeBuildingStep::INIT) {
    //                 // check permissions to determine reachability
    //                 for (NBEdge* out : cur->getToNode()->getOutgoingEdges()) {
    //                     if (allEdges.count(out) != 0) {
    //                         const SVCPermissions p = pCur & out->getPermissions();
    //                         if (seen.count(out) == 0 || (~conPermissions[ {e, out}] & p) != 0) {
    //                             if ((p & ~SVC_PEDESTRIAN) != 0) {
    //                                 open.push_back(out);
    //                                 conPermissions[ {e, out}] |= p;
    // #ifdef DEBUG_JOINJUNCTIONS_CONNECTIONS
    //                                 std::cout << "  e=" << e->getID() << " out=" << out->getID() << " pOut=" << getVehicleClassNames(out->getPermissions()) << "\n    p=" << getVehicleClassNames(p) << "\n    q=" << getVehicleClassNames(conPermissions[ {e, out}]) << "\n";
    // #endif
    //                             }
    //                         }
    //                     }
    //                 }
    //             } else {
    //                 // check existing connections
    //                 for (const auto& con : cons) {
    //                     if (con.toEdge != nullptr && allEdges.count(con.toEdge) != 0) {
    //                         SVCPermissions p = pCur & con.toEdge->getPermissions();
    //                         if (con.permissions != SVC_UNSPECIFIED) {
    //                             p &= con.permissions;
    //                         }
    //                         if (seen.count(con.toEdge) == 0 || (~conPermissions[ {e, con.toEdge}] & p) != 0) {
    //                             open.push_back(con.toEdge);
    //                             conPermissions[ {e, con.toEdge}] |= p;
    //                             //std::cout << "  e=" << e->getID() << " con.toEdge=" << con.toEdge->getID() << " pSpecial=" << toString(con.permissions) << " pOut=" << getVehicleClassNames(con.toEdge->getPermissions()) << "\n    p=" << getVehicleClassNames(p) << "\n    q=" << getVehicleClassNames(conPermissions[{e, con.toEdge}]) << "\n";
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         seen.erase(e);
    //         for (NBEdge* reached : seen) {
    //             // filter out inside edges from reached
    //             if (inside.count(reached) == 0) {
    //                 if (e->getStep() > NBEdge::EdgeBuildingStep::INIT && reached->getFromNode() == e->getToNode() && !e->isConnectedTo(reached)) {
    //                     // also filter out edges that are outgoing of the to-node of edge but aren't currently connected
    //                     continue;
    //                 }
    //                 reachable[e].insert(reached);
    //                 const SVCPermissions pDefault = e->getPermissions() & reached->getPermissions();
    //                 if (conPermissions[ {e, reached}] != pDefault) {
    //                     specialPermissions.insert(e);
    // #ifdef DEBUG_JOINJUNCTIONS_CONNECTIONS
    //                     std::cout << "e=" << e->getID() << " out=" << reached->getID() << " special=" << getVehicleClassNames(conPermissions[ {e, reached}]) << "\n";
    // #endif
    //                 }
    //             }
    //         }
    // #ifdef DEBUG_JOINJUNCTIONS_CONNECTIONS
    //         std::cout << " reachable e=" << e->getID() << " seen=" << toString(seen) << " reachable=" << toString(reachable[e]) << "\n";
    // #endif
    //     }
    //
    //     // remap and remove edges which are completely within the new intersection
    //     if (origNames) {
    //         newNode->setParameter("origEdgeIds", joinNamedToString(inside, ' '));
    //     }
    //     for (NBEdge* e : inside) {
    //         for (NBEdge* e2 : allEdges) {
    //             if (e != e2) {
    //                 e2->replaceInConnections(e, e->getConnections());
    //             }
    //         }
    //         ec.extract(dc, e, true);
    //         allEdges.erase(e);
    //     }
    //
    //     // remap edges which are incoming / outgoing
    //     for (NBEdge* e : allEdges) {
    //         const bool outgoing = cluster.count(e->getFromNode()) > 0;
    //         NBNode* from = outgoing ? newNode : e->getFromNode();
    //         NBNode* to   = outgoing ? e->getToNode() : newNode;
    //         if (origNames) {
    //             if (outgoing) {
    //                 e->setParameter("origFrom", e->getFromNode()->getID());
    //             } else {
    //                 e->setParameter("origTo", e->getToNode()->getID());
    //             }
    //         }
    //         if (e->getTurnSignTarget() != "") {
    //             for (NBNode* n : cluster) {
    //                 if (e->getTurnSignTarget() == n->getID()) {
    //                     e->setTurnSignTarget(to->getID());
    //                     break;
    //                 }
    //             }
    //         }
    //         e->reinitNodes(from, to);
    //         // re-add connections which previously existed and may still valid.
    //         // connections to removed edges will be ignored
    //         std::vector<NBEdge::Connection> conns = e->getConnections();
    //         for (std::vector<NBEdge::Connection>::iterator k = conns.begin(); k != conns.end(); ++k) {
    //             if ((*k).toEdge == nullptr) {
    //                 // edge explicitly set to have no connections
    //                 continue;
    //             }
    //             e->addLane2LaneConnection((*k).fromLane, (*k).toEdge, (*k).toLane, NBEdge::Lane2LaneInfoType::USER, false, (*k).mayDefinitelyPass);
    //             if ((*k).fromLane >= 0 && (*k).fromLane < e->getNumLanes() && e->getLaneStruct((*k).fromLane).connectionsDone) {
    //                 // @note (see NIImporter_DlrNavteq::ConnectedLanesHandler)
    //                 e->declareConnectionsAsLoaded(NBEdge::EdgeBuildingStep::INIT);
    // #ifdef DEBUG_JOINJUNCTIONS_CONNECTIONS
    //                 std::cout << "   e=" << e->getID() << " declareConnectionsAsLoaded\n";
    // #endif
    //             }
    //         }
    //     }
    //     if (!resetConnections) {
    //         // disable connections that were impossible with the old topology
    //         // if connectivity has special permissions, set edge to edge connections explicitly
    //         for (NBEdge* in : newNode->getIncomingEdges()) {
    //             for (NBEdge* out : newNode->getOutgoingEdges()) {
    //                 if (reachable[in].count(out) == 0) {
    //                     if (!ec.hasPostProcessConnection(in->getID(), out->getID())) {
    //                         //std::cout << " removeUnreachable in=" << in->getID() << " out=" << out->getID() << "\n";
    //                         in->removeFromConnections(out, -1, -1, true, false, true);
    //                     } else {
    //                         //std::cout << " hasPostProcessConnection in=" << in->getID() << " out=" << out->getID() << "\n";
    //                     }
    //                 } else if (specialPermissions.count(in) != 0) {
    //                     SVCPermissions pDefault = in->getPermissions() & out->getPermissions();
    //                     SVCPermissions p = conPermissions[ {in, out}] == 0 ? pDefault : conPermissions[ {in, out}];
    //                     in->addEdge2EdgeConnection(out, true, p == pDefault ? SVC_UNSPECIFIED : p);
    //                     //std::cout << " addEdge2Edge in=" << in->getID() << " out=" << out->getID() << "\n";
    //                 }
    //             }
    //         }
    //     } else {
    //         for (NBEdge* in : newNode->getIncomingEdges()) {
    //             in->invalidateConnections(true);
    //         }
    //     }
    //
    //     // remove original nodes
    //     registerJoinedCluster(cluster);
    //     for (NBNode* n : cluster) {
    //         erase(n);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join_node_cluster(&mut self, clusters: NodeSet, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont, predefined: Ptr<NBNode>, resetConnections: bool) {
        let cluster = clusters;
        let origNames = utils::options::options_cont::get_bool("output.original-names");
        assert!(cluster.len() > 1);
        let mut id = "cluster_".to_string();
        let mut pos = Position::new3(0., 0., 0.);
        let mut setTL = false;
        let mut nodeType = SumoXMLNodeType::UNKNOWN;
        let mut type_: TrafficLightType = utils::xml::sumo_xml_definitions::TrafficLightTypes
            .get(&utils::options::options_cont::get_string("tls.default-type"))
            .unwrap_or_else(|e| std::panic::panic_any(e));
        // collect edges (std::set<NBEdge*, ComparatorIdLess>: keyed by id, first insertion wins)
        let mut allEdges: BTreeMap<String, Ptr<NBEdge>> = BTreeMap::new();
        for n in cluster.iter().copied() {
            for e in n.m().get_edges().clone() {
                allEdges.entry(e.m().get_id().to_string()).or_insert(e);
            }
        }
        // determine edges with are incoming or fully inside
        let mut clusterIncoming: BTreeMap<String, Ptr<NBEdge>> = BTreeMap::new();
        let mut inside: BTreeMap<String, Ptr<NBEdge>> = BTreeMap::new();
        for e in allEdges.values().copied().collect::<Vec<_>>() {
            if cluster.contains(&e.m().get_to_node()) {
                if cluster.contains(&e.m().get_from_node()) {
                    inside.insert(e.m().get_id().to_string(), e);
                    if *e.m().get_signal_position() != Position::INVALID {
                        setTL = true;
                        nodeType = SumoXMLNodeType::TRAFFIC_LIGHT;
                    }
                } else {
                    clusterIncoming.insert(e.m().get_id().to_string(), e);
                }
            }
        }
        self.analyze_cluster(cluster.clone(), &mut id, &mut pos, &mut setTL, &mut type_, &mut nodeType);
        let newNode: Ptr<NBNode>;
        if !predefined.is_null() {
            newNode = predefined;
        } else {
            if !self.insert(&id, &pos, Ptr::from_raw(std::ptr::null_mut())) {
                // should not fail
                write_warningf!("Could not join junctions %.", id.as_str());
                return;
            }
            newNode = self.retrieve(&id);
        }
        let tlID = id.clone();
        if !predefined.is_null() {
            if predefined.m().get_type() != SumoXMLNodeType::UNKNOWN {
                nodeType = predefined.m().get_type();
                setTL = predefined.m().is_tl_controlled();
            }
            let ppos = predefined.m().get_position().clone();
            if ppos.x() != Position::INVALID.x() {
                pos.setx(ppos.x());
            }
            if ppos.y() != Position::INVALID.y() {
                pos.sety(ppos.y());
            }
            if ppos.z() != Position::INVALID.z() {
                pos.setz(ppos.z());
            }
        }
        newNode.m().reinit(&pos, nodeType, false);
        let clusterNames: Vec<String> = cluster.iter().map(|n| n.m().get_id().to_string()).collect();
        if origNames {
            newNode.m().base2.set_parameter(SUMO_PARAM_ORIGID, &clusterNames.join(" "));
        }
        if setTL && !newNode.m().is_tl_controlled() {
            let raw: *mut NBOwnTLDef = Box::into_raw(Box::new(NBOwnTLDef::new_2(&tlID, newNode, 0, type_)));
            let tlDef: Ptr<NBTrafficLightDefinition> = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) });
            newNode.m().add_traffic_light(tlDef);
            if !tlc.insert(tlDef, false) {
                // actually, nothing should fail here
                drop(unsafe { Box::from_raw(raw) });
                panic!("{}", utils::common::util_exceptions::ProcessError::new(&format!("Could not allocate tls '{}'.", id)));
            }
        }

        // determine possible connectivity from outside edges
        // (std::map / std::set over pointers: BTree keyed by pointer keeps the C++ order)
        let mut reachable: BTreeMap<Ptr<NBEdge>, EdgeSet> = BTreeMap::new();
        let mut conPermissions: BTreeMap<(Ptr<NBEdge>, Ptr<NBEdge>), SVCPermissions> = BTreeMap::new();
        let mut specialPermissions: EdgeSet = EdgeSet::new();
        for e in clusterIncoming.values().copied().collect::<Vec<_>>() {
            let mut open: EdgeVector = vec![e];
            let mut seen: EdgeSet = EdgeSet::new();
            while open.len() > 0 {
                let cur = *open.last().unwrap();
                // C++ conPermissions.count(..) == 0 ? ... : conPermissions[..]
                let pCur = match conPermissions.get(&(e, cur)) {
                    None => cur.m().get_permissions(-1),
                    Some(v) => *v,
                };
                seen.insert(cur);
                open.pop();
                if !cluster.contains(&cur.m().get_to_node()) {
                    continue;
                }
                let cons = cur.m().get_connections();
                if cons.len() == 0 || ec.has_post_process_connection(cur.m().get_id(), "") || cur.m().get_step() == NBEdge_EdgeBuildingStep::INIT {
                    // check permissions to determine reachability
                    for out in cur.m().get_to_node().m().get_outgoing_edges().clone() {
                        if allEdges.contains_key(out.m().get_id()) {
                            let p = pCur & out.m().get_permissions(-1);
                            if !seen.contains(&out) || (!pc_get_or_insert(&mut conPermissions, (e, out)) & p) != 0 {
                                if (p & !SVC_PEDESTRIAN) != 0 {
                                    open.push(out);
                                    *conPermissions.entry((e, out)).or_insert(0) |= p;
                                }
                            }
                        }
                    }
                } else {
                    // check existing connections
                    for con in cons.iter() {
                        if !con.toEdge.is_null() && allEdges.contains_key(con.toEdge.m().get_id()) {
                            let mut p = pCur & con.toEdge.m().get_permissions(-1);
                            if con.permissions != SVC_UNSPECIFIED {
                                p &= con.permissions;
                            }
                            if !seen.contains(&con.toEdge) || (!pc_get_or_insert(&mut conPermissions, (e, con.toEdge)) & p) != 0 {
                                open.push(con.toEdge);
                                *conPermissions.entry((e, con.toEdge)).or_insert(0) |= p;
                            }
                        }
                    }
                }
            }
            seen.remove(&e);
            for reached in seen.iter().copied().collect::<Vec<_>>() {
                // filter out inside edges from reached
                if !inside.contains_key(reached.m().get_id()) {
                    if e.m().get_step() > NBEdge_EdgeBuildingStep::INIT && reached.m().get_from_node() == e.m().get_to_node() && !e.m().is_connected_to(reached, false) {
                        // also filter out edges that are outgoing of the to-node of edge but aren't currently connected
                        continue;
                    }
                    reachable.entry(e).or_default().insert(reached);
                    let pDefault = e.m().get_permissions(-1) & reached.m().get_permissions(-1);
                    if pc_get_or_insert(&mut conPermissions, (e, reached)) != pDefault {
                        specialPermissions.insert(e);
                    }
                }
            }
        }

        // remap and remove edges which are completely within the new intersection
        if origNames {
            let insideNames: Vec<String> = inside.values().map(|e| e.m().get_id().to_string()).collect();
            newNode.m().base2.set_parameter("origEdgeIds", &insideNames.join(" "));
        }
        for e in inside.values().copied().collect::<Vec<_>>() {
            for e2 in allEdges.values().copied().collect::<Vec<_>>() {
                if e != e2 {
                    e2.m().replace_in_connections_2(e, e.m().get_connections());
                }
            }
            ec.extract(dc, e, true);
            allEdges.remove(e.m().get_id());
        }

        // remap edges which are incoming / outgoing
        for e in allEdges.values().copied().collect::<Vec<_>>() {
            let outgoing = cluster.contains(&e.m().get_from_node());
            let from = if outgoing { newNode } else { e.m().get_from_node() };
            let to = if outgoing { e.m().get_to_node() } else { newNode };
            if origNames {
                if outgoing {
                    let v = e.m().get_from_node().m().get_id().to_string();
                    e.m().base2.set_parameter("origFrom", &v);
                } else {
                    let v = e.m().get_to_node().m().get_id().to_string();
                    e.m().base2.set_parameter("origTo", &v);
                }
            }
            if e.m().get_turn_sign_target() != "" {
                for n in cluster.iter().copied() {
                    if e.m().get_turn_sign_target() == n.m().get_id() {
                        let toID = to.m().get_id().to_string();
                        e.m().set_turn_sign_target(&toID);
                        break;
                    }
                }
            }
            e.m().reinit_nodes(from, to);
            // re-add connections which previously existed and may still valid.
            // connections to removed edges will be ignored
            // (copy the fields needed; NBEdge_Connection is not Clone and the list is mutated while iterating)
            let conns: Vec<(i32, Ptr<NBEdge>, i32, bool)> = e.m().get_connections().iter().map(|c| (c.fromLane, c.toEdge, c.toLane, c.mayDefinitelyPass)).collect();
            for (fromLane, toEdge, toLane, mayDefinitelyPass) in conns {
                if toEdge.is_null() {
                    // edge explicitly set to have no connections
                    continue;
                }
                add_lane2_lane_connection_user(e, fromLane, toEdge, toLane, mayDefinitelyPass);
                if fromLane >= 0 && fromLane < e.m().get_num_lanes() && e.m().get_lane_struct(fromLane).connectionsDone {
                    // @note (see NIImporter_DlrNavteq::ConnectedLanesHandler)
                    e.m().declare_connections_as_loaded(NBEdge_EdgeBuildingStep::INIT);
                }
            }
        }
        if !resetConnections {
            // disable connections that were impossible with the old topology
            // if connectivity has special permissions, set edge to edge connections explicitly
            for in_ in newNode.m().get_incoming_edges().clone() {
                for out in newNode.m().get_outgoing_edges().clone() {
                    let reachableIn: EdgeSet = reachable.get(&in_).cloned().unwrap_or_default();
                    if !reachableIn.contains(&out) {
                        if !ec.has_post_process_connection(in_.m().get_id(), out.m().get_id()) {
                            in_.m().remove_from_connections(out, -1, -1, true, false, true);
                        }
                    } else if specialPermissions.contains(&in_) {
                        let pDefault = in_.m().get_permissions(-1) & out.m().get_permissions(-1);
                        let stored = pc_get_or_insert(&mut conPermissions, (in_, out));
                        let p = if stored == 0 { pDefault } else { pc_get_or_insert(&mut conPermissions, (in_, out)) };
                        in_.m().add_edge2_edge_connection(out, true, if p == pDefault { SVC_UNSPECIFIED } else { p });
                    }
                }
            }
        } else {
            for in_ in newNode.m().get_incoming_edges().clone() {
                in_.m().invalidate_connections(true);
            }
        }

        // remove original nodes
        self.register_joined_cluster(&cluster);
        for n in cluster.iter().copied() {
            self.erase(n);
        }
    }

    /// C++: `bool NBNodeCont::shouldBeTLSControlled(const NodeSet &, double, bool) const` [const, private]
    /// Returns whether the given node cluster should be controlled by a tls
    // bool
    // NBNodeCont::shouldBeTLSControlled(const NodeSet& c, double laneSpeedThreshold, bool recheck) const {
    //     bool tooFast = false;
    //     double laneSpeedSum = 0;
    //     std::set<NBEdge*> seen;
    //     for (NBNode* j : c) {
    //         for (const NBEdge* e : j->getEdges()) {
    //             if (c.find(e->getFromNode()) != c.end() && c.find(e->getToNode()) != c.end()) {
    //                 // edges fully within the cluster do not count
    //                 continue;
    //             }
    //             if (j->hasIncoming(e)) {
    //                 if (recheck && !j->hasConflict(e)) {
    //                     // edges without conflict do not count
    //                     // we can only check this after connections have been computed
    //                     continue;
    //                 }
    //                 laneSpeedSum += (double)e->getNumLanes() * e->getLaneSpeed(0);
    //             }
    //             if (e->getLaneSpeed(0) * 3.6 > 79) {
    //                 tooFast = true;
    //             }
    //         }
    //     }
    //     //std::cout << " c=" << joinNamedToString(c, ' ') << " size=" << c.size() << " laneSpeedSum=" << laneSpeedSum << " thresh=" << laneSpeedThreshold << " tooFast=" << tooFast << "\n";
    //     return !tooFast && laneSpeedSum >= laneSpeedThreshold && c.size() != 0;
    // }
    #[allow(unused_variables)]
    pub fn should_be_tls_controlled(&self, c: &NodeSet, laneSpeedThreshold: f64, recheck: bool) -> bool {
        let mut tooFast = false;
        let mut laneSpeedSum: f64 = 0.;
        for j in c.iter().copied() {
            for e in j.m().get_edges().clone() {
                if c.contains(&e.m().get_from_node()) && c.contains(&e.m().get_to_node()) {
                    // edges fully within the cluster do not count
                    continue;
                }
                if j.m().has_incoming(e) {
                    if recheck && !j.m().has_conflict_2(e) {
                        // edges without conflict do not count
                        // we can only check this after connections have been computed
                        continue;
                    }
                    laneSpeedSum += (e.m().get_num_lanes() as f64) * e.m().get_lane_speed(0);
                }
                if e.m().get_lane_speed(0) * 3.6 > 79. {
                    tooFast = true;
                }
            }
        }
        !tooFast && laneSpeedSum >= laneSpeedThreshold && c.len() != 0
    }

    /// C++: `bool NBNodeCont::onlyCrossings(const NodeSet &) const` [const, private]
    /// check whether the set of nodes only contains pedestrian crossings
    // bool
    // NBNodeCont::onlyCrossings(const NodeSet& c) const {
    //     // check whether all component nodes are solely pedestrian crossings
    //     // (these work fine without joining)
    //     for (NBNode* node : c) {
    //         EdgeVector nonPedIncoming;
    //         EdgeVector nonPedOutgoing;
    //         for (NBEdge* e : node->getIncomingEdges()) {
    //             if (e->getPermissions() != SVC_PEDESTRIAN) {
    //                 nonPedIncoming.push_back(e);
    //             }
    //         }
    //         for (NBEdge* e : node->getOutgoingEdges()) {
    //             if (e->getPermissions() != SVC_PEDESTRIAN) {
    //                 nonPedOutgoing.push_back(e);
    //             }
    //         }
    //         if (!node->geometryLike(nonPedIncoming, nonPedOutgoing)) {
    //             //for (NBNode* node : c) {
    //             //    if (node->getID() == "2480337678") {
    //             //        std::cout << " node=" << node->getID() << " nonPedIncoming=" << toString(nonPedIncoming) << " nonPedOutgoing=" << toString(nonPedOutgoing) << "\n";
    //             //    }
    //             //}
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn only_crossings(&self, c: &NodeSet) -> bool {
        // check whether all component nodes are solely pedestrian crossings
        // (these work fine without joining)
        for node in c.iter().copied() {
            let mut nonPedIncoming: EdgeVector = Vec::new();
            let mut nonPedOutgoing: EdgeVector = Vec::new();
            for e in node.m().get_incoming_edges().clone() {
                if e.m().get_permissions(-1) != SVC_PEDESTRIAN {
                    nonPedIncoming.push(e);
                }
            }
            for e in node.m().get_outgoing_edges().clone() {
                if e.m().get_permissions(-1) != SVC_PEDESTRIAN {
                    nonPedOutgoing.push(e);
                }
            }
            if !NBNode::geometry_like_2(&nonPedIncoming, &nonPedOutgoing) {
                return false;
            }
        }
        true
    }

    /// C++: `bool NBNodeCont::customTLID(const NodeSet &) const` [const, private]
    /// check whether the set of nodes contains traffic lights with custom id
    // bool
    // NBNodeCont::customTLID(const NodeSet& c) const {
    //     for (NBNode* node : c) {
    //         if (node->isTLControlled()) {
    //             NBTrafficLightDefinition* tl = (*node->getControllingTLS().begin());
    //             if (tl->getNodes().size() > 1) {
    //                 // joined tls also imply a customID
    //                 return true;
    //             }
    //             const std::string tlID = tl->getID();
    //             if (tlID != node->getID()
    //                     && !StringUtils::startsWith(tlID, "joinedS_")
    //                     && !StringUtils::startsWith(tlID, "joinedG_")
    //                     && !StringUtils::startsWith(tlID, "GS")) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn custom_tlid(&self, c: &NodeSet) -> bool {
        for node in c.iter().copied() {
            if node.m().is_tl_controlled() {
                let tl = node.m().get_controlling_tls().iter().next().copied().unwrap();
                if tl.m().get_nodes().len() > 1 {
                    // joined tls also imply a customID
                    return true;
                }
                let tlID = tl.m().get_id();
                if tlID != node.m().get_id()
                        && !tlID.starts_with("joinedS_")
                        && !tlID.starts_with("joinedG_")
                        && !tlID.starts_with("GS") {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `void NBNodeCont::paretoCheck(NBNode *, NodeSet &, int, int)` [private]
    /// update pareto frontier with the given node
    // void
    // NBNodeCont::paretoCheck(NBNode* node, NodeSet& frontier, int xSign, int ySign) {
    //     const double x = node->getPosition().x() * xSign;
    //     const double y = node->getPosition().y() * ySign;
    //     std::vector<NBNode*> dominated;
    //     for (NBNode* fn : frontier) {
    //         const double x2 = fn->getPosition().x() * xSign;
    //         const double y2 = fn->getPosition().y() * ySign;
    //         if (x2 >= x && y2 >= y) {
    //             return;
    //         } else if (x2 <= x && y2 <= y) {
    //             dominated.push_back(fn);
    //         }
    //     }
    //     frontier.insert(node);
    //     for (NBNode* r : dominated) {
    //         frontier.erase(r);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn pareto_check(&mut self, node: Ptr<NBNode>, frontier: &mut NodeSet, xSign: i32, ySign: i32) {
        let x = node.m().get_position().x() * xSign as f64;
        let y = node.m().get_position().y() * ySign as f64;
        let mut dominated: Vec<Ptr<NBNode>> = Vec::new();
        for fnode in frontier.iter().copied().collect::<Vec<_>>() {
            let x2 = fnode.m().get_position().x() * xSign as f64;
            let y2 = fnode.m().get_position().y() * ySign as f64;
            if x2 >= x && y2 >= y {
                return;
            } else if x2 <= x && y2 <= y {
                dominated.push(fnode);
            }
        }
        frontier.insert(node);
        for r in dominated {
            frontier.remove(&r);
        }
    }

    /// C++: `void NBNodeCont::NBNodeCont(const NBNodeCont &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NBNodeCont) -> Self {
        // C++ declares this copy constructor `= delete`: calling it is a compile error there, so it cannot be reached
        panic!("NBNodeCont copy constructor is deleted (NBNodeCont(const NBNodeCont&) = delete)")
    }

    /// C++: `NBNodeCont & NBNodeCont::operator=(const NBNodeCont &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBNodeCont) -> &mut NBNodeCont {
        // C++ declares operator= `= delete`: calling it is a compile error there, so it cannot be reached
        panic!("NBNodeCont assignment operator is deleted (NBNodeCont& operator=(const NBNodeCont&) = delete)")
    }

}

