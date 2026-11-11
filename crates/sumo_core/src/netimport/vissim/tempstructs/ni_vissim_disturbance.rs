//! Port of netimport/vissim/tempstructs/NIVissimDisturbance.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimDisturbance *> NIVissimDisturbance::DictType`
pub type NIVissimDisturbance_DictType = BTreeMap<i32, Ptr<NIVissimDisturbance>>;

/// C++ class `NIVissimDisturbance` (netimport/vissim/tempstructs/NIVissimDisturbance.h)
pub struct NIVissimDisturbance {
    /// C++ base class `NIVissimBoundedClusterObject`
    pub base: NIVissimBoundedClusterObject,
    pub myID: i32,
    pub myNode: i32,
    pub myName: String,
    pub myEdge: NIVissimExtendedEdgePoint,
    pub myDisturbance: NIVissimExtendedEdgePoint,
}

impl std::ops::Deref for NIVissimDisturbance {
    type Target = NIVissimBoundedClusterObject;
    fn deref(&self) -> &NIVissimBoundedClusterObject {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimDisturbance {
    fn deref_mut(&mut self) -> &mut NIVissimBoundedClusterObject {
        &mut self.base
    }
}

// C++ static members of NIVissimDisturbance (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimDisturbance::myDict;
//   static int NIVissimDisturbance::myRunningID;
//   static int NIVissimDisturbance::refusedProhibits;

impl NIVissimDisturbance {
    /// C++: `void NIVissimDisturbance::NIVissimDisturbance(int, const std::string &, const NIVissimExtendedEdgePoint &, const NIVissimExtendedEdgePoint &)`
    // NIVissimDisturbance::NIVissimDisturbance(int id,
    //         const std::string& name,
    //         const NIVissimExtendedEdgePoint& edge,
    //         const NIVissimExtendedEdgePoint& by)
    //     : myID(id), myNode(-1), myName(name), myEdge(edge), myDisturbance(by) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, edge: &NIVissimExtendedEdgePoint, by: &NIVissimExtendedEdgePoint) -> Self {
        todo!("port NIVissimDisturbance::NIVissimDisturbance")
    }

    /// C++: `void NIVissimDisturbance::~NIVissimDisturbance() noexcept` [virtual]
    // NIVissimDisturbance::~NIVissimDisturbance() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimDisturbance::~NIVissimDisturbance")
    }

    /// C++: `void NIVissimDisturbance::computeBounding()` [virtual]
    // void
    // NIVissimDisturbance::computeBounding() {
    //     assert(myBoundary == 0);
    //     Boundary* bound = new Boundary();
    //     if (NIVissimAbstractEdge::dictionary(myEdge.getEdgeID()) != nullptr) {
    //         bound->add(myEdge.getGeomPosition());
    //     }
    //     if (NIVissimAbstractEdge::dictionary(myDisturbance.getEdgeID()) != nullptr) {
    //         bound->add(myDisturbance.getGeomPosition());
    //     }
    //     myBoundary = bound;
    //     assert(myBoundary != 0 && myBoundary->xmax() >= myBoundary->xmin());
    // }
    #[allow(unused_variables)]
    pub fn compute_bounding(&mut self) {
        todo!("port NIVissimDisturbance::computeBounding")
    }

    /// C++: `bool NIVissimDisturbance::addToNode(NBNode *, NBDistrictCont &, NBNodeCont &, NBEdgeCont &)`
    // bool
    // NIVissimDisturbance::addToNode(NBNode* node, NBDistrictCont& dc,
    //                                NBNodeCont& nc, NBEdgeCont& ec) {
    //     myNode = 0;
    //     NIVissimConnection* pc =
    //         NIVissimConnection::dictionary(myEdge.getEdgeID());
    //     NIVissimConnection* bc =
    //         NIVissimConnection::dictionary(myDisturbance.getEdgeID());
    //     if (pc == nullptr && bc == nullptr) {
    //         // This has not been tested completely, yet
    //         // Both competing abstract edges are normal edges
    //         // We have to find a crossing point, build a node here,
    //         //  split both edges and add the connections
    //         NIVissimEdge* e1 = NIVissimEdge::dictionary(myEdge.getEdgeID());
    //         NIVissimEdge* e2 = NIVissimEdge::dictionary(myDisturbance.getEdgeID());
    //         WRITE_WARNINGF(TL("Ugly split to prohibit '%' by '%'."), toString<int>(e1->getID()), toString<int>(e2->getID()));
    //         //Position pos = e1->crossesEdgeAtPoint(e2);
    //         std::string id1 = toString<int>(e1->getID()) + "x" + toString<int>(e2->getID());
    //         std::string id2 = toString<int>(e2->getID()) + "x" + toString<int>(e1->getID());
    //         NBNode* node1 = nc.retrieve(id1);
    //         NBNode* node2 = nc.retrieve(id2);
    //         NBNode* splitNode = node1 == nullptr ? node2 : node1;
    //         assert(node1 == nullptr || node2 == nullptr);
    //         if (splitNode == nullptr) {
    //             refusedProhibits++;
    //             return false;
    //             /*            node = new NBNode(id1, pos.x(), pos.y(), "priority");
    //                         if(!myNodeCont.insert(node)) {
    //                              "nope, NIVissimDisturbance" << endl;
    //                             throw 1;
    //                         }*/
    //         }
    //         ec.splitAt(dc, ec.retrievePossiblySplit(toString<int>(e1->getID()), myEdge.getPosition()), splitNode);
    //         ec.splitAt(dc, ec.retrievePossiblySplit(toString<int>(e2->getID()), myDisturbance.getPosition()), splitNode);
    //         // !!! in some cases, one of the edges is not being build because it's too short
    //         // !!! what to do in these cases?
    //         NBEdge* mayDriveFrom = ec.retrieve(toString<int>(e1->getID()) + "[0]");
    //         NBEdge* mayDriveTo = ec.retrieve(toString<int>(e1->getID()) + "[1]");
    //         NBEdge* mustStopFrom = ec.retrieve(toString<int>(e2->getID()) + "[0]");
    //         NBEdge* mustStopTo = ec.retrieve(toString<int>(e2->getID()) + "[1]");
    //         if (mayDriveFrom != nullptr && mayDriveTo != nullptr && mustStopFrom != nullptr && mustStopTo != nullptr) {
    //             node->addSortedLinkFoes(
    //                 NBConnection(mayDriveFrom, mayDriveTo),
    //                 NBConnection(mayDriveFrom, mayDriveTo));
    //         } else {
    //             refusedProhibits++;
    //             return false;
    //             // !!! warning
    //         }
    // //        }
    //     } else if (pc != nullptr && bc == nullptr) {
    //         // The prohibited abstract edge is a connection, the other
    //         //  is not;
    //         // The connection will be prohibitesd by all connections
    //         //  outgoing from the "real" edge
    //
    //         NBEdge* e = ec.retrievePossiblySplit(toString<int>(myDisturbance.getEdgeID()), myDisturbance.getPosition());
    //         if (e == nullptr) {
    //             WRITE_WARNINGF(TL("Could not prohibit '%' by '%'. Have not found disturbance."), toString<int>(myEdge.getEdgeID()), toString<int>(myDisturbance.getEdgeID()));
    //             refusedProhibits++;
    //             return false;
    //         }
    //         if (e->getFromNode() == e->getToNode()) {
    //             WRITE_WARNINGF(TL("Could not prohibit '%' by '%'. Disturbance connects same node."), toString<int>(myEdge.getEdgeID()), toString<int>(myDisturbance.getEdgeID()));
    //             refusedProhibits++;
    //             // What to do with self-looping edges?
    //             return false;
    //         }
    //         // get the begin of the prohibited connection
    //         std::string id_pcoe = toString<int>(pc->getFromEdgeID());
    //         std::string id_pcie = toString<int>(pc->getToEdgeID());
    //         NBEdge* const pcoe = ec.retrievePossiblySplit(id_pcoe, id_pcie, true);
    //         NBEdge* const pcie = ec.retrievePossiblySplit(id_pcie, id_pcoe, false);
    //         // check whether its ending node is the node the prohibited
    //         //  edge end at
    //         if (pcoe != nullptr && pcie != nullptr && pcoe->getToNode() == e->getToNode()) {
    //             // if so, simply prohibit the connections
    //             NBNode* const toNode = e->getToNode();
    //             for (NBEdge* const edge : e->getConnectedEdges()) {
    //                 toNode->addSortedLinkFoes(NBConnection(e, edge), NBConnection(pcoe, pcie));
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Would have to split edge '%' to build a prohibition"), e->getID());
    //             refusedProhibits++;
    //             // quite ugly - why was it not build?
    //             return false;
    //             /*
    //             std::string nid1 = e->getID() + "[0]";
    //             std::string nid2 = e->getID() + "[1]";
    //
    //             if(ec.splitAt(e, node)) {
    //                 node->addSortedLinkFoes(
    //                         NBConnection(
    //                             ec.retrieve(nid1),
    //                             ec.retrieve(nid2)
    //                         ),
    //                         getConnection(node, myEdge.getEdgeID())
    //                     );
    //             }
    //             */
    //         }
    //     } else if (bc != nullptr && pc == nullptr) {
    //         // The prohibiting abstract edge is a connection, the other
    //         //  is not;
    //         // We have to split the other one and add the prohibition
    //         //  description
    //
    //         NBEdge* e = ec.retrievePossiblySplit(toString<int>(myEdge.getEdgeID()), myEdge.getPosition());
    //         if (e == nullptr) {
    //             WRITE_WARNINGF(TL("Could not prohibit '%' - it was not built."), toString<int>(myEdge.getEdgeID()));
    //             return false;
    //         }
    //         std::string nid1 = e->getID() + "[0]";
    //         std::string nid2 = e->getID() + "[1]";
    //         if (e->getFromNode() == e->getToNode()) {
    //             WRITE_WARNINGF(TL("Could not prohibit '%' by '%'."), toString<int>(myEdge.getEdgeID()), toString<int>(myDisturbance.getEdgeID()));
    //             refusedProhibits++;
    //             // What to do with self-looping edges?
    //             return false;
    //         }
    //         // get the begin of the prohibiting connection
    //         std::string id_bcoe = toString<int>(bc->getFromEdgeID());
    //         std::string id_bcie = toString<int>(bc->getToEdgeID());
    //         NBEdge* const bcoe = ec.retrievePossiblySplit(id_bcoe, id_bcie, true);
    //         NBEdge* const bcie = ec.retrievePossiblySplit(id_bcie, id_bcoe, false);
    //         // check whether its ending node is the node the prohibited
    //         //  edge end at
    //         if (bcoe != nullptr && bcie != nullptr && bcoe->getToNode() == e->getToNode()) {
    //             // if so, simply prohibit the connections
    //             NBNode* const toNode = e->getToNode();
    //             for (NBEdge* const edge : e->getConnectedEdges()) {
    //                 toNode->addSortedLinkFoes(NBConnection(bcoe, bcie), NBConnection(e, edge));
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Would have to split edge '%' to build a prohibition"), e->getID());
    //             refusedProhibits++;
    //             return false;
    //             /*
    //             // quite ugly - why was it not build?
    //             if(ec.splitAt(e, node)) {
    //                 node->addSortedLinkFoes(
    //                         getConnection(node, myDisturbance.getEdgeID()),
    //                         NBConnection(
    //                             ec.retrieve(nid1),
    //                             ec.retrieve(nid2)
    //                         )
    //                     );
    //             }
    //             */
    //         }
    //     } else {
    //         // both the prohibiting and the prohibited abstract edges
    //         //  are connections
    //         // We can retrieve the conected edges and add the desription
    //         NBConnection conn1 = getConnection(node, myDisturbance.getEdgeID());
    //         NBConnection conn2 = getConnection(node, myEdge.getEdgeID());
    //         if (!conn1.check(ec) || !conn2.check(ec)) {
    //             refusedProhibits++;
    //             return false;
    //         }
    //         node->addSortedLinkFoes(conn1, conn2);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_to_node(&mut self, node: Ptr<NBNode>, dc: &mut NBDistrictCont, nc: &mut NBNodeCont, ec: &mut NBEdgeCont) -> bool {
        todo!("port NIVissimDisturbance::addToNode")
    }

    /// C++: `int NIVissimDisturbance::getEdgeID() const` [const]
    // int getEdgeID() const {
    //         return myEdge.getEdgeID();
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_id(&self) -> i32 {
        todo!("port NIVissimDisturbance::getEdgeID")
    }

    /// C++: `int NIVissimDisturbance::getDisturbanceID() const` [const]
    // int getDisturbanceID() const {
    //         return myDisturbance.getEdgeID();
    //     }
    #[allow(unused_variables)]
    pub fn get_disturbance_id(&self) -> i32 {
        todo!("port NIVissimDisturbance::getDisturbanceID")
    }

    /// C++: `NBConnection NIVissimDisturbance::getConnection(NBNode *, int)`
    // NBConnection
    // NIVissimDisturbance::getConnection(NBNode* node, int aedgeid) {
    //     if (NIVissimEdge::dictionary(myEdge.getEdgeID()) == nullptr) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(aedgeid);
    //         NBEdge* from =
    //             node->getPossiblySplittedIncoming(toString<int>(c->getFromEdgeID()));
    //         NBEdge* to =
    //             node->getPossiblySplittedOutgoing(toString<int>(c->getToEdgeID()));
    //
    //         // source is a connection
    //         return NBConnection(toString<int>(c->getFromEdgeID()), from,
    //                             toString<int>(c->getToEdgeID()), to);
    //     } else {
    //         WRITE_WARNING(TL("NIVissimDisturbance: no connection"));
    //         return NBConnection::InvalidConnection;
    // //        throw 1; // !!! what to do?
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn get_connection(&mut self, node: Ptr<NBNode>, aedgeid: i32) -> NBConnection {
        todo!("port NIVissimDisturbance::getConnection")
    }

    /// C++: `bool NIVissimDisturbance::dictionary(const std::string &, const NIVissimExtendedEdgePoint &, const NIVissimExtendedEdgePoint &)` [static]
    // bool
    // NIVissimDisturbance::dictionary(const std::string& name,
    //                                 const NIVissimExtendedEdgePoint& edge,
    //                                 const NIVissimExtendedEdgePoint& by) {
    //     int nid = myRunningID++;
    //     NIVissimDisturbance* o =
    //         new NIVissimDisturbance(nid, name, edge, by);
    //     if (!dictionary(nid, o)) {
    //         delete o;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(name: &str, edge: &NIVissimExtendedEdgePoint, by: &NIVissimExtendedEdgePoint) -> bool {
        todo!("port NIVissimDisturbance::dictionary")
    }

    /// C++: `bool NIVissimDisturbance::dictionary(int, NIVissimDisturbance *)` [static]
    // bool
    // NIVissimDisturbance::dictionary(int id, NIVissimDisturbance* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, o: Ptr<NIVissimDisturbance>) -> bool {
        todo!("port NIVissimDisturbance::dictionary")
    }

    /// C++: `NIVissimDisturbance * NIVissimDisturbance::dictionary(int)` [static]
    // NIVissimDisturbance*
    // NIVissimDisturbance::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimDisturbance> {
        todo!("port NIVissimDisturbance::dictionary")
    }

    /// C++: `std::vector<int> NIVissimDisturbance::getWithin(const AbstractPoly &)` [static]
    // std::vector<int>
    // NIVissimDisturbance::getWithin(const AbstractPoly& poly) {
    //     std::vector<int> ret;
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         if ((*i).second->crosses(poly)) {
    //             ret.push_back((*i).second->myID);
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_within(poly: &dyn AbstractPoly) -> Vec<i32> {
        todo!("port NIVissimDisturbance::getWithin")
    }

    /// C++: `void NIVissimDisturbance::clearDict()` [static]
    // void
    // NIVissimDisturbance::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        todo!("port NIVissimDisturbance::clearDict")
    }

    /// C++: `void NIVissimDisturbance::dict_SetDisturbances()` [static]
    // void
    // NIVissimDisturbance::dict_SetDisturbances() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimDisturbance* d = (*i).second;
    //         NIVissimAbstractEdge::dictionary(d->myEdge.getEdgeID())->addDisturbance((*i).first);
    //         NIVissimAbstractEdge::dictionary(d->myDisturbance.getEdgeID())->addDisturbance((*i).first);
    //     }
    //     /*    for(DictType::iterator i=myDict.begin(); i!=myDict.end(); i++) {
    //             delete (*i).second;
    //         }
    //         */
    // }
    #[allow(unused_variables)]
    pub fn dict_set_disturbances() {
        todo!("port NIVissimDisturbance::dict_SetDisturbances")
    }

    /// C++: `void NIVissimDisturbance::reportRefused()` [static]
    // void
    // NIVissimDisturbance::reportRefused() {
    //     if (refusedProhibits > 0) {
    //         WRITE_WARNINGF(TL("Could not build % of % disturbances."), toString<int>(refusedProhibits), toString<int>((int)myDict.size()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn report_refused() {
        todo!("port NIVissimDisturbance::reportRefused")
    }

}

