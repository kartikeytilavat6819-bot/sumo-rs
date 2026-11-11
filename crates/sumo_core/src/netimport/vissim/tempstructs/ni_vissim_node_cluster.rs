//! Port of netimport/vissim/tempstructs/NIVissimNodeCluster.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimNodeCluster *> NIVissimNodeCluster::DictType`
pub type NIVissimNodeCluster_DictType = BTreeMap<i32, Ptr<NIVissimNodeCluster>>;

/// C++ class `NIVissimNodeCluster` (netimport/vissim/tempstructs/NIVissimNodeCluster.h)
pub struct NIVissimNodeCluster {
    pub myID: i32,
    pub myNodeID: i32,
    pub myTLID: i32,
    pub myConnectors: Vec<i32>,
    pub myDisturbances: Vec<i32>,
    pub myPosition: Position,
    pub myNBNode: Ptr<NBNode>,
    pub myAmEdgeSplit: bool,
}

// C++ static members of NIVissimNodeCluster (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimNodeCluster::myDict;
//   static int NIVissimNodeCluster::myCurrentID;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimNodeCluster_DictType> = std::cell::RefCell::new(BTreeMap::new());
    static MY_CURRENT_ID: std::cell::Cell<i32> = const { std::cell::Cell::new(1) };
}

impl NIVissimNodeCluster {
    /// C++: `void NIVissimNodeCluster::NIVissimNodeCluster(int, int, int, const std::vector<int> &, const std::vector<int> &, bool)`
    // NIVissimNodeCluster::NIVissimNodeCluster(int id, int nodeid, int tlid,
    //         const std::vector<int>& connectors,
    //         const std::vector<int>& disturbances,
    //         bool amEdgeSplitOnly)
    //     : myID(id), myNodeID(nodeid), myTLID(tlid),
    //       myConnectors(connectors), myDisturbances(disturbances),
    //       myNBNode(nullptr), myAmEdgeSplit(amEdgeSplitOnly) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, nodeid: i32, tlid: i32, connectors: &Vec<i32>, disturbances: &Vec<i32>, amEdgeSplitOnly: bool) -> Self {
        Self {
            myID: id,
            myNodeID: nodeid,
            myTLID: tlid,
            myConnectors: connectors.clone(),
            myDisturbances: disturbances.clone(),
            myNBNode: Ptr::null(),
            myAmEdgeSplit: amEdgeSplitOnly,
            myPosition: Position::default(),
        }
    }

    /// C++: `void NIVissimNodeCluster::~NIVissimNodeCluster()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit; nothing to release.
    }

    /// C++: `int NIVissimNodeCluster::getID() const` [const]
    // int getID() const {
    //         return myID;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> i32 {
        self.myID
    }

    /// C++: `void NIVissimNodeCluster::buildNBNode(NBNodeCont &)`
    // void
    // NIVissimNodeCluster::buildNBNode(NBNodeCont& nc) {
    //     if (myConnectors.size() == 0) {
    //         return; // !!! Check, whether this can happen
    //     }
    //
    //     // compute the position
    //     PositionVector crossings;
    //     std::vector<int>::iterator i, j;
    //     // check whether this is a split of an edge only
    //     if (myAmEdgeSplit) {
    // // !!! should be        assert(myTLID==-1);
    //         for (i = myConnectors.begin(); i != myConnectors.end(); i++) {
    //             NIVissimConnection* c1 = NIVissimConnection::dictionary(*i);
    //             crossings.push_back_noDoublePos(c1->getFromGeomPosition());
    //         }
    //     } else {
    //         // compute the places the connections cross
    //         for (i = myConnectors.begin(); i != myConnectors.end(); i++) {
    //             NIVissimAbstractEdge* c1 = NIVissimAbstractEdge::dictionary(*i);
    //             c1->buildGeom();
    //             for (j = i + 1; j != myConnectors.end(); j++) {
    //                 NIVissimAbstractEdge* c2 = NIVissimAbstractEdge::dictionary(*j);
    //                 c2->buildGeom();
    //                 if (c1->crossesEdge(c2)) {
    //                     crossings.push_back_noDoublePos(c1->crossesEdgeAtPoint(c2));
    //                 }
    //             }
    //         }
    //         // alternative way: compute via positions of crossings
    //         if (crossings.size() == 0) {
    //             for (i = myConnectors.begin(); i != myConnectors.end(); i++) {
    //                 NIVissimConnection* c1 = NIVissimConnection::dictionary(*i);
    //                 crossings.push_back_noDoublePos(c1->getFromGeomPosition());
    //                 crossings.push_back_noDoublePos(c1->getToGeomPosition());
    //             }
    //         }
    //     }
    //     // get the position (center)
    //     Position pos = crossings.getPolygonCenter();
    //     // build the node
    //     /*    if(myTLID!=-1) {
    //      !!!        NIVissimTL *tl = NIVissimTL::dictionary(myTLID);
    //             if(tl->getType()=="festzeit") {
    //                 node = new NBNode(getNodeName(), pos.x(), pos.y(),
    //                     "traffic_light");
    //             } else {
    //                 node = new NBNode(getNodeName(), pos.x(), pos.y(),
    //                     "actuated_traffic_light");
    //             }
    //         }*/
    //     NBNode* node = new NBNode(getNodeName(), pos, SumoXMLNodeType::PRIORITY);
    //     if (!nc.insert(node)) {
    //         delete node;
    //         throw 1;
    //     }
    //     myNBNode = node;
    // }
    #[allow(unused_variables)]
    pub fn build_nb_node(&mut self, nc: &mut NBNodeCont) {
        todo!("port NIVissimNodeCluster::buildNBNode")
    }

    /// C++: `bool NIVissimNodeCluster::recheckEdgeChanges()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn recheck_edge_changes(&mut self) -> bool {
        todo!("port NIVissimNodeCluster::recheckEdgeChanges")
    }

    /// C++: `NBNode * NIVissimNodeCluster::getNBNode() const` [const]
    // NBNode*
    // NIVissimNodeCluster::getNBNode() const {
    //     return myNBNode;
    // }
    #[allow(unused_variables)]
    pub fn get_nb_node(&self) -> Ptr<NBNode> {
        self.myNBNode
    }

    /// C++: `Position NIVissimNodeCluster::getPos() const` [const]
    // Position
    // NIVissimNodeCluster::getPos() const {
    //     return myPosition;
    // }
    #[allow(unused_variables)]
    pub fn get_pos(&self) -> Position {
        self.myPosition.clone()
    }

    /// C++: `std::string NIVissimNodeCluster::getNodeName() const` [const]
    // std::string
    // NIVissimNodeCluster::getNodeName() const {
    //     if (myTLID == -1) {
    //         return toString<int>(myID);
    //     } else {
    //         return toString<int>(myID) + "LSA " + toString<int>(myTLID);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_node_name(&self) -> String {
        if self.myTLID == -1 {
            self.myID.to_string()
        } else {
            format!("{}LSA {}", self.myID, self.myTLID)
        }
    }

    /// C++: `bool NIVissimNodeCluster::dictionary(int, NIVissimNodeCluster *)` [static]
    // bool
    // NIVissimNodeCluster::dictionary(int id, NIVissimNodeCluster* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     assert(false);
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, o: Ptr<NIVissimNodeCluster>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if !d.contains_key(&id) {
                d.insert(id, o);
                return true;
            }
            debug_assert!(false);
            false
        })
    }

    /// C++: `int NIVissimNodeCluster::dictionary(int, int, const std::vector<int> &, const std::vector<int> &, bool)` [static]
    // int
    // NIVissimNodeCluster::dictionary(int nodeid, int tlid,
    //                                 const std::vector<int>& connectors,
    //                                 const std::vector<int>& disturbances,
    //                                 bool amEdgeSplitOnly) {
    //     int id = nodeid;
    //     if (nodeid < 0) {
    //         id = myCurrentID++;
    //     }
    //     NIVissimNodeCluster* o = new NIVissimNodeCluster(id,
    //             nodeid, tlid, connectors, disturbances, amEdgeSplitOnly);
    //     dictionary(id, o);
    //     return id;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(nodeid: i32, tlid: i32, connectors: &Vec<i32>, disturbances: &Vec<i32>, amEdgeSplitOnly: bool) -> i32 {
        let mut id = nodeid;
        if nodeid < 0 {
            id = MY_CURRENT_ID.with(|c| {
                let v = c.get();
                c.set(v + 1);
                v
            });
        }
        let o = Ptr::from_raw(Box::into_raw(Box::new(NIVissimNodeCluster::new(
            id, nodeid, tlid, connectors, disturbances, amEdgeSplitOnly,
        ))));
        NIVissimNodeCluster::dictionary(id, o);
        id
    }

    /// C++: `NIVissimNodeCluster * NIVissimNodeCluster::dictionary(int)` [static]
    // NIVissimNodeCluster*
    // NIVissimNodeCluster::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimNodeCluster> {
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `int NIVissimNodeCluster::contSize()` [static]
    // int
    // NIVissimNodeCluster::contSize() {
    //     return (int)myDict.size();
    // }
    #[allow(unused_variables)]
    pub fn cont_size() -> i32 {
        MY_DICT.with(|d| d.borrow().len() as i32)
    }

    /// C++: `void NIVissimNodeCluster::assignToEdges()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_to_edges() {
        // shortcut: C++ declares assignToEdges() but has no definition in SUMO; upgrade only if SUMO adds one.
        panic!("NIVissimNodeCluster::assignToEdges is declared but not defined in SUMO")
    }

    /// C++: `void NIVissimNodeCluster::buildNBNodes(NBNodeCont &)` [static]
    // void
    // NIVissimNodeCluster::buildNBNodes(NBNodeCont& nc) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         (*i).second->buildNBNode(nc);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_nb_nodes(nc: &mut NBNodeCont) {
        todo!("port NIVissimNodeCluster::buildNBNodes")
    }

    /// C++: `void NIVissimNodeCluster::dict_recheckEdgeChanges()` [static]
    // void
    // NIVissimNodeCluster::dict_recheckEdgeChanges() {
    //     return;
    // }
    #[allow(unused_variables)]
    pub fn dict_recheck_edge_changes() {
        return;
    }

    /// C++: `int NIVissimNodeCluster::getFromNode(int)` [static]
    // int
    // NIVissimNodeCluster::getFromNode(int edgeid) {
    //     int ret = -1;
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimNodeCluster* c = (*i).second;
    //         for (std::vector<int>::iterator j = c->myConnectors.begin(); j != c->myConnectors.end(); j++) {
    //             NIVissimConnection* conn = NIVissimConnection::dictionary(*j);
    //             if (conn != nullptr && conn->getToEdgeID() == edgeid) {
    // //                return (*i).first;
    //                 if (ret != -1 && (*i).first != ret) {
    // //                     "NIVissimNodeCluster:DoubleNode:" << ret << endl;
    //                     throw 1; // an edge should not outgo from two different nodes
    // // but actually, a joined cluster may posess a connections more than once
    //                 }
    //                 ret = (*i).first;
    //             }
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_from_node(edgeid: i32) -> i32 {
        todo!("port NIVissimNodeCluster::getFromNode")
    }

    /// C++: `int NIVissimNodeCluster::getToNode(int)` [static]
    // int
    // NIVissimNodeCluster::getToNode(int edgeid) {
    //     int ret = -1;
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimNodeCluster* c = (*i).second;
    //         for (std::vector<int>::iterator j = c->myConnectors.begin(); j != c->myConnectors.end(); j++) {
    //             NIVissimConnection* conn = NIVissimConnection::dictionary(*j);
    //             if (conn != nullptr && conn->getFromEdgeID() == edgeid) {
    // //                return (*i).first;
    //                 if (ret != -1 && ret != (*i).first) {
    // //                  << "NIVissimNodeCluster: multiple to-nodes" << endl;
    //                     throw 1; // an edge should not outgo from two different nodes
    // // but actually, a joined cluster may posess a connections more than once
    //
    //                 }
    //                 ret = (*i).first;
    //             }
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_to_node(edgeid: i32) -> i32 {
        todo!("port NIVissimNodeCluster::getToNode")
    }

    /// C++: `void NIVissimNodeCluster::_debugOut(std::ostream &)` [static]
    // void
    // NIVissimNodeCluster::_debugOut(std::ostream& into) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimNodeCluster* c = (*i).second;
    //         into << std::endl << c->myID << ":";
    //         for (std::vector<int>::iterator j = c->myConnectors.begin(); j != c->myConnectors.end(); j++) {
    //             if (j != c->myConnectors.begin()) {
    //                 into << ", ";
    //             }
    //             into << (*j);
    //         }
    //     }
    //     into << "=======================" << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn debug_out(into: &mut Unmapped /* std::ostream */) {
        todo!("port NIVissimNodeCluster::_debugOut")
    }

    /// C++: `void NIVissimNodeCluster::dict_addDisturbances(NBDistrictCont &, NBNodeCont &, NBEdgeCont &)` [static]
    // void
    // NIVissimNodeCluster::dict_addDisturbances(NBDistrictCont& dc,
    //         NBNodeCont& nc, NBEdgeCont& ec) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         const std::vector<int>& disturbances = (*i).second->myDisturbances;
    //         NBNode* node = nc.retrieve((*i).second->getNodeName());
    //         for (std::vector<int>::const_iterator j = disturbances.begin(); j != disturbances.end(); j++) {
    //             NIVissimDisturbance* disturbance = NIVissimDisturbance::dictionary(*j);
    //             disturbance->addToNode(node, dc, nc, ec);
    //         }
    //     }
    //     NIVissimDisturbance::reportRefused();
    // }
    #[allow(unused_variables)]
    pub fn dict_add_disturbances(dc: &mut NBDistrictCont, nc: &mut NBNodeCont, ec: &mut NBEdgeCont) {
        todo!("port NIVissimNodeCluster::dict_addDisturbances")
    }

    /// C++: `void NIVissimNodeCluster::clearDict()` [static]
    // void
    // NIVissimNodeCluster::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            for (_, o) in d.iter() {
                // delete (*i).second;
                drop(unsafe { Box::from_raw(o.as_raw()) });
            }
            d.clear();
        })
    }

    /// C++: `void NIVissimNodeCluster::setCurrentVirtID(int)` [static]
    // void
    // NIVissimNodeCluster::setCurrentVirtID(int id) {
    //     myCurrentID = id;
    // }
    #[allow(unused_variables)]
    pub fn set_current_virt_id(id: i32) {
        MY_CURRENT_ID.with(|c| c.set(id))
    }

}

