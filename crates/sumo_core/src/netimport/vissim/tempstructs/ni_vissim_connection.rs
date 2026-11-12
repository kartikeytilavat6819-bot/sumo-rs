//! Port of netimport/vissim/tempstructs/NIVissimConnection.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimConnection *> NIVissimConnection::DictType`
pub type NIVissimConnection_DictType = BTreeMap<i32, Ptr<NIVissimConnection>>;

/// C++ class `NIVissimConnection` (netimport/vissim/tempstructs/NIVissimConnection.h)
#[repr(C)]
pub struct NIVissimConnection {
    /// C++ base class `NIVissimBoundedClusterObject`
    pub base: NIVissimBoundedClusterObject,
    /// C++ base class `NIVissimAbstractEdge`
    pub base2: NIVissimAbstractEdge,
    pub myName: String,
    pub myFromDef: NIVissimExtendedEdgePoint,
    pub myToDef: NIVissimExtendedEdgePoint,
    pub myAssignedVehicles: Vec<i32>,
    pub myClosedLanes: NIVissimClosedLanesVector,
}

impl std::ops::Deref for NIVissimConnection {
    type Target = NIVissimBoundedClusterObject;
    fn deref(&self) -> &NIVissimBoundedClusterObject {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimConnection {
    fn deref_mut(&mut self) -> &mut NIVissimBoundedClusterObject {
        &mut self.base
    }
}

// C++ static members of NIVissimConnection (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimConnection::myDict;
//   static int NIVissimConnection::myMaxID;

impl NIVissimConnection {
    /// C++: `void NIVissimConnection::NIVissimConnection(int, const std::string &, const NIVissimExtendedEdgePoint &, const NIVissimExtendedEdgePoint &, const PositionVector &, const std::vector<int> &, const NIVissimClosedLanesVector &)`
    // NIVissimConnection::NIVissimConnection(int id,
    //                                        const std::string& name, const NIVissimExtendedEdgePoint& from_def,
    //                                        const NIVissimExtendedEdgePoint& to_def,
    //                                        const PositionVector& geom,
    //                                        const std::vector<int>& assignedVehicles, const NIVissimClosedLanesVector& clv)
    //     : NIVissimAbstractEdge(id, geom),
    //       myName(name), myFromDef(from_def), myToDef(to_def),
    //       myAssignedVehicles(assignedVehicles), myClosedLanes(clv) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, from_def: &NIVissimExtendedEdgePoint, to_def: &NIVissimExtendedEdgePoint, geom: &PositionVector, assignedVehicles: &Vec<i32>, clv: &NIVissimClosedLanesVector) -> Self {
        todo!("port NIVissimConnection::NIVissimConnection")
    }

    /// C++: `void NIVissimConnection::~NIVissimConnection() noexcept` [virtual]
    // NIVissimConnection::~NIVissimConnection() {
    //     for (NIVissimClosedLanesVector::iterator i = myClosedLanes.begin(); i != myClosedLanes.end(); i++) {
    //         delete (*i);
    //     }
    //     myClosedLanes.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimConnection::~NIVissimConnection")
    }

    /// C++: `void NIVissimConnection::computeBounding()` [virtual]
    // void
    // NIVissimConnection::computeBounding() {
    //     Boundary* bound = new Boundary();
    //     bound->add(myFromDef.getGeomPosition());
    //     bound->add(myToDef.getGeomPosition());
    //     assert(myBoundary == 0);
    //     myBoundary = bound;
    // }
    #[allow(unused_variables)]
    pub fn compute_bounding(&mut self) {
        todo!("port NIVissimConnection::computeBounding")
    }

    /// C++: `int NIVissimConnection::getFromEdgeID() const` [const]
    // int
    // NIVissimConnection::getFromEdgeID() const {
    //     return myFromDef.getEdgeID();
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge_id(&self) -> i32 {
        todo!("port NIVissimConnection::getFromEdgeID")
    }

    /// C++: `int NIVissimConnection::getToEdgeID() const` [const]
    // int
    // NIVissimConnection::getToEdgeID() const {
    //     return myToDef.getEdgeID();
    // }
    #[allow(unused_variables)]
    pub fn get_to_edge_id(&self) -> i32 {
        todo!("port NIVissimConnection::getToEdgeID")
    }

    /// C++: `double NIVissimConnection::getFromPosition() const` [const]
    // double
    // NIVissimConnection::getFromPosition() const {
    //     return myFromDef.getPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_from_position(&self) -> f64 {
        todo!("port NIVissimConnection::getFromPosition")
    }

    /// C++: `double NIVissimConnection::getToPosition() const` [const]
    // double
    // NIVissimConnection::getToPosition() const {
    //     return myToDef.getPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_to_position(&self) -> f64 {
        todo!("port NIVissimConnection::getToPosition")
    }

    /// C++: `Position NIVissimConnection::getFromGeomPosition() const` [const]
    // Position
    // NIVissimConnection::getFromGeomPosition() const {
    //     return myFromDef.getGeomPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_from_geom_position(&self) -> Position {
        todo!("port NIVissimConnection::getFromGeomPosition")
    }

    /// C++: `Position NIVissimConnection::getToGeomPosition() const` [const]
    // Position
    // NIVissimConnection::getToGeomPosition() const {
    //     return myToDef.getGeomPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_to_geom_position(&self) -> Position {
        todo!("port NIVissimConnection::getToGeomPosition")
    }

    /// C++: `void NIVissimConnection::setNodeCluster(int)` [virtual]
    // void
    // NIVissimConnection::setNodeCluster(int nodeid) {
    //     assert(myNode == -1);
    //     myNode = nodeid;
    // }
    #[allow(unused_variables)]
    pub fn set_node_cluster(&mut self, nodeid: i32) {
        todo!("port NIVissimConnection::setNodeCluster")
    }

    /// C++: `const Boundary & NIVissimConnection::getBoundingBox() const` [const]
    // const Boundary&
    // NIVissimConnection::getBoundingBox() const {
    //     assert(myBoundary != 0 && myBoundary->xmax() >= myBoundary->xmin());
    //     return *myBoundary;
    // }
    #[allow(unused_variables)]
    pub fn get_bounding_box(&self) -> &Boundary {
        todo!("port NIVissimConnection::getBoundingBox")
    }

    /// C++: `int NIVissimConnection::buildEdgeConnections(NBEdgeCont &)`
    // int
    // NIVissimConnection::buildEdgeConnections(NBEdgeCont& ec) {
    //     int unsetConnections = 0;
    //     // try to determine the connected edges
    //     NBEdge* fromEdge = nullptr;
    //     NBEdge* toEdge = nullptr;
    //     NIVissimEdge* vissimFrom = NIVissimEdge::dictionary(getFromEdgeID());
    //     if (vissimFrom->wasWithinAJunction()) {
    //         // this edge was not built, try to get one that approaches it
    //         vissimFrom = vissimFrom->getBestIncoming();
    //         if (vissimFrom != nullptr) {
    //             fromEdge = ec.retrievePossiblySplit(toString(vissimFrom->getID()), toString(getFromEdgeID()), true);
    //         }
    //     } else {
    //         // this edge was built, try to get the proper part
    //         fromEdge = ec.retrievePossiblySplit(toString(getFromEdgeID()), toString(getToEdgeID()), true);
    //     }
    //     NIVissimEdge* vissimTo = NIVissimEdge::dictionary(getToEdgeID());
    //     if (vissimTo->wasWithinAJunction()) {
    //         vissimTo = vissimTo->getBestOutgoing();
    //         if (vissimTo != nullptr) {
    //             toEdge = ec.retrievePossiblySplit(toString(vissimTo->getID()), toString(getToEdgeID()), true);
    //         }
    //     } else {
    //         toEdge = ec.retrievePossiblySplit(toString(getToEdgeID()), toString(getFromEdgeID()), false);
    //     }
    //
    //     // try to get the edges the current connection connects
    //     /*
    //     NBEdge *fromEdge = ec.retrievePossiblySplit(toString(getFromEdgeID()), toString(getToEdgeID()), true);
    //     NBEdge *toEdge = ec.retrievePossiblySplit(toString(getToEdgeID()), toString(getFromEdgeID()), false);
    //     */
    //     if (fromEdge == nullptr || toEdge == nullptr) {
    //         WRITE_WARNINGF(TL("Could not build connection between '%' and '%'."), toString(getFromEdgeID()), toString(getToEdgeID()));
    //         return 1; // !!! actually not 1
    //     }
    //     recheckLanes(fromEdge, toEdge);
    //     const std::vector<int>& fromLanes = getFromLanes();
    //     const std::vector<int>& toLanes = getToLanes();
    //     if (fromLanes.size() != toLanes.size()) {
    //         WRITE_WARNINGF(TL("Lane sizes differ for connection '%'."), toString(getID()));
    //     } else {
    //         for (int index = 0; index < (int)fromLanes.size(); ++index) {
    //             if (fromEdge->getNumLanes() <= fromLanes[index]) {
    //                 WRITE_WARNING("Could not set connection between '" + fromEdge->getID() + "_" + toString(fromLanes[index]) + "' and '" + toEdge->getID() + "_" + toString(toLanes[index]) + "'.");
    //                 ++unsetConnections;
    //             } else if (!fromEdge->addLane2LaneConnection(fromLanes[index], toEdge, toLanes[index], NBEdge::Lane2LaneInfoType::VALIDATED, true)) {
    //                 WRITE_WARNING("Could not set connection between '" + fromEdge->getID() + "_" + toString(fromLanes[index]) + "' and '" + toEdge->getID() + "_" + toString(toLanes[index]) + "'.");
    //                 ++unsetConnections;
    //             }
    //         }
    //     }
    //     return unsetConnections;
    // }
    #[allow(unused_variables)]
    pub fn build_edge_connections(&mut self, ec: &mut NBEdgeCont) -> i32 {
        todo!("port NIVissimConnection::buildEdgeConnections")
    }

    /// C++: `void NIVissimConnection::buildGeom()` [virtual]
    // void
    // NIVissimConnection::buildGeom() {
    //     if (myGeom.size() > 0) {
    //         return;
    //     }
    //     myGeom.push_back(myFromDef.getGeomPosition());
    //     myGeom.push_back(myToDef.getGeomPosition());
    // }
    #[allow(unused_variables)]
    pub fn build_geom(&mut self) {
        todo!("port NIVissimConnection::buildGeom")
    }

    /// C++: `void NIVissimConnection::recheckLanes(const NBEdge *const, const NBEdge *const)`
    /// Resets lane numbers if all lanes shall be used
    // void
    // NIVissimConnection::recheckLanes(const NBEdge* const fromEdge, const NBEdge* const toEdge) {
    //     myFromDef.recheckLanes(fromEdge);
    //     myToDef.recheckLanes(toEdge);
    // }
    #[allow(unused_variables)]
    pub fn recheck_lanes(&mut self, fromEdge: Ptr<NBEdge>, toEdge: Ptr<NBEdge>) {
        todo!("port NIVissimConnection::recheckLanes")
    }

    /// C++: `const std::vector<int> & NIVissimConnection::getFromLanes() const` [const]
    // const std::vector<int>&
    // NIVissimConnection::getFromLanes() const {
    //     return myFromDef.getLanes();
    // }
    #[allow(unused_variables)]
    pub fn get_from_lanes(&self) -> &Vec<i32> {
        todo!("port NIVissimConnection::getFromLanes")
    }

    /// C++: `const std::vector<int> & NIVissimConnection::getToLanes() const` [const]
    // const std::vector<int>&
    // NIVissimConnection::getToLanes() const {
    //     return myToDef.getLanes();
    // }
    #[allow(unused_variables)]
    pub fn get_to_lanes(&self) -> &Vec<i32> {
        todo!("port NIVissimConnection::getToLanes")
    }

    /// C++: `bool NIVissimConnection::dictionary(int, NIVissimConnection *)` [static]
    // bool
    // NIVissimConnection::dictionary(int id, NIVissimConnection* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, o: Ptr<NIVissimConnection>) -> bool {
        todo!("port NIVissimConnection::dictionary")
    }

    /// C++: `NIVissimConnection * NIVissimConnection::dictionary(int)` [static]
    // NIVissimConnection*
    // NIVissimConnection::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32) -> Ptr<NIVissimConnection> {
        todo!("port NIVissimConnection::dictionary")
    }

    /// C++: `std::vector<int> NIVissimConnection::getWithin(const AbstractPoly &)` [static]
    // std::vector<int>
    // NIVissimConnection::getWithin(const AbstractPoly& poly) {
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
        todo!("port NIVissimConnection::getWithin")
    }

    /// C++: `void NIVissimConnection::buildNodeClusters()` [static]
    // void
    // NIVissimConnection::buildNodeClusters() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimConnection* e = (*i).second;
    //         if (!e->clustered()) {
    //             assert(e->myBoundary != 0 && e->myBoundary->xmax() > e->myBoundary->xmin());
    //             std::vector<int> connections =
    //                 NIVissimConnection::getWithin(*(e->myBoundary));
    //             NIVissimNodeCluster::dictionary(-1, -1, connections,
    //                                             std::vector<int>(), true); // 19.5.!!! should be on a single edge
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_node_clusters() {
        todo!("port NIVissimConnection::buildNodeClusters")
    }

    /// C++: `std::vector<int> NIVissimConnection::getForEdge(int, bool)` [static]
    // std::vector<int>
    // NIVissimConnection::getForEdge(int edgeid, bool /*omitNodeAssigned*/) {
    //     std::vector<int> ret;
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         int connID = (*i).first;
    //         if ((*i).second->myFromDef.getEdgeID() == edgeid
    //                 ||
    //                 (*i).second->myToDef.getEdgeID() == edgeid) {
    //             if (!(*i).second->hasNodeCluster()) {
    //                 ret.push_back(connID);
    //             }
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_for_edge(edgeid: i32, omitNodeAssigned: bool) -> Vec<i32> {
        todo!("port NIVissimConnection::getForEdge")
    }

    /// C++: `void NIVissimConnection::dict_buildNBEdgeConnections(NBEdgeCont &)` [static]
    // void
    // NIVissimConnection::dict_buildNBEdgeConnections(NBEdgeCont& ec) {
    //     int unsetConnections = 0;
    //     // go through connections
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         unsetConnections += (*i).second->buildEdgeConnections(ec);
    //     }
    //     if (unsetConnections != 0) {
    //         WRITE_WARNING(toString<int>(unsetConnections) + " of " + toString<int>((int)myDict.size()) + " connections could not be assigned.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_build_nb_edge_connections(ec: &mut NBEdgeCont) {
        todo!("port NIVissimConnection::dict_buildNBEdgeConnections")
    }

    /// C++: `void NIVissimConnection::dict_assignToEdges()` [static]
    // void
    // NIVissimConnection::dict_assignToEdges() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimConnection* c = (*i).second;
    //         NIVissimEdge::dictionary(c->getFromEdgeID())->addOutgoingConnection((*i).first);
    //         NIVissimEdge::dictionary(c->getToEdgeID())->addIncomingConnection((*i).first);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_assign_to_edges() {
        todo!("port NIVissimConnection::dict_assignToEdges")
    }

    /// C++: `int NIVissimConnection::getMaxID()` [static]
    // int
    // NIVissimConnection::getMaxID() {
    //     return myMaxID;
    // }
    #[allow(unused_variables)]
    pub fn get_max_id() -> i32 {
        todo!("port NIVissimConnection::getMaxID")
    }

}

