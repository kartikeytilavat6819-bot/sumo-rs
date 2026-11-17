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
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimConnection_DictType> = std::cell::RefCell::new(BTreeMap::new());
    // C++ never assigns myMaxID, so it stays 0
    static MY_MAX_ID: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

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
    pub fn new(id: i32, name: &str, from_def: &NIVissimExtendedEdgePoint, to_def: &NIVissimExtendedEdgePoint, geom: &PositionVector, assignedVehicles: &Vec<i32>, clv: &NIVissimClosedLanesVector) -> Box<Self> {
        // the base constructors register `this`, so the object must stay at a fixed address: return the Box
        let mut b = Box::new(Self {
            base: NIVissimBoundedClusterObject { myBoundary: Ptr::null(), myClusterID: -1 },
            base2: NIVissimAbstractEdge::new_unregistered(id, geom),
            myName: name.to_string(),
            myFromDef: NIVissimExtendedEdgePoint::new(from_def.get_edge_id(), from_def.get_lanes(), from_def.get_position(), &from_def.myAssignedVehicles),
            myToDef: NIVissimExtendedEdgePoint::new(to_def.get_edge_id(), to_def.get_lanes(), to_def.get_position(), &to_def.myAssignedVehicles),
            myAssignedVehicles: assignedVehicles.clone(),
            myClosedLanes: clv.clone(),
        });
        b.base2.myKind = NIVissimAbstractEdgeKind::Connection;
        NIVissimAbstractEdge::dictionary(id, Ptr::from_mut(&mut b.base2));
        NIVissimBoundedClusterObject::register(Ptr::from_mut(&mut b.base), |o| Ptr::from_mut(o).cast::<NIVissimConnection>().m().compute_bounding());
        b
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
        // C++ deletes the closed-lane defs it shares with its creator too; kept as is
        for c in self.myClosedLanes.drain(..) {
            drop(unsafe { Box::from_raw(c.as_raw()) });
        }
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
        let mut bound = Boundary::new();
        let from = self.myFromDef.get_geom_position();
        bound.add(from.x(), from.y(), from.z());
        let to = self.myToDef.get_geom_position();
        bound.add(to.x(), to.y(), to.z());
        debug_assert!(self.base.myBoundary.is_null());
        self.base.myBoundary = Ptr::from_raw(Box::into_raw(Box::new(bound)));
    }

    /// C++: `int NIVissimConnection::getFromEdgeID() const` [const]
    // int
    // NIVissimConnection::getFromEdgeID() const {
    //     return myFromDef.getEdgeID();
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge_id(&self) -> i32 {
        self.myFromDef.get_edge_id()
    }

    /// C++: `int NIVissimConnection::getToEdgeID() const` [const]
    // int
    // NIVissimConnection::getToEdgeID() const {
    //     return myToDef.getEdgeID();
    // }
    #[allow(unused_variables)]
    pub fn get_to_edge_id(&self) -> i32 {
        self.myToDef.get_edge_id()
    }

    /// C++: `double NIVissimConnection::getFromPosition() const` [const]
    // double
    // NIVissimConnection::getFromPosition() const {
    //     return myFromDef.getPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_from_position(&self) -> f64 {
        self.myFromDef.get_position()
    }

    /// C++: `double NIVissimConnection::getToPosition() const` [const]
    // double
    // NIVissimConnection::getToPosition() const {
    //     return myToDef.getPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_to_position(&self) -> f64 {
        self.myToDef.get_position()
    }

    /// C++: `Position NIVissimConnection::getFromGeomPosition() const` [const]
    // Position
    // NIVissimConnection::getFromGeomPosition() const {
    //     return myFromDef.getGeomPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_from_geom_position(&self) -> Position {
        self.myFromDef.get_geom_position()
    }

    /// C++: `Position NIVissimConnection::getToGeomPosition() const` [const]
    // Position
    // NIVissimConnection::getToGeomPosition() const {
    //     return myToDef.getGeomPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_to_geom_position(&self) -> Position {
        self.myToDef.get_geom_position()
    }

    /// C++: `void NIVissimConnection::setNodeCluster(int)` [virtual]
    // void
    // NIVissimConnection::setNodeCluster(int nodeid) {
    //     assert(myNode == -1);
    //     myNode = nodeid;
    // }
    #[allow(unused_variables)]
    pub fn set_node_cluster(&mut self, nodeid: i32) {
        debug_assert!(self.base2.myNode == -1);
        self.base2.myNode = nodeid;
    }

    /// C++: `const Boundary & NIVissimConnection::getBoundingBox() const` [const]
    // const Boundary&
    // NIVissimConnection::getBoundingBox() const {
    //     assert(myBoundary != 0 && myBoundary->xmax() >= myBoundary->xmin());
    //     return *myBoundary;
    // }
    #[allow(unused_variables)]
    pub fn get_bounding_box(&self) -> &Boundary {
        debug_assert!(self.base.myBoundary.get().is_some_and(|b| b.xmax() >= b.xmin()));
        self.get_boundary()
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
        let mut unsetConnections: i32 = 0;
        // try to determine the connected edges
        let mut fromEdge: Ptr<NBEdge> = Ptr::null();
        let mut toEdge: Ptr<NBEdge> = Ptr::null();
        let mut vissimFrom: Ptr<NIVissimEdge> = NIVissimEdge::dictionary_3(self.get_from_edge_id());
        if vissimFrom.m().was_within_a_junction() {
            // this edge was not built, try to get one that approaches it
            vissimFrom = vissimFrom.m().get_best_incoming();
            if !vissimFrom.is_null() {
                fromEdge = ec.retrieve_possibly_split_2(&vissimFrom.m().get_id().to_string(), &self.get_from_edge_id().to_string(), true);
            }
        } else {
            // this edge was built, try to get the proper part
            fromEdge = ec.retrieve_possibly_split_2(&self.get_from_edge_id().to_string(), &self.get_to_edge_id().to_string(), true);
        }
        let mut vissimTo: Ptr<NIVissimEdge> = NIVissimEdge::dictionary_3(self.get_to_edge_id());
        if vissimTo.m().was_within_a_junction() {
            vissimTo = vissimTo.m().get_best_outgoing();
            if !vissimTo.is_null() {
                toEdge = ec.retrieve_possibly_split_2(&vissimTo.m().get_id().to_string(), &self.get_to_edge_id().to_string(), true);
            }
        } else {
            toEdge = ec.retrieve_possibly_split_2(&self.get_to_edge_id().to_string(), &self.get_from_edge_id().to_string(), false);
        }

        if fromEdge.is_null() || toEdge.is_null() {
            write_warningf!("Could not build connection between '%' and '%'.", self.get_from_edge_id(), self.get_to_edge_id());
            return 1; // !!! actually not 1
        }
        self.recheck_lanes(fromEdge, toEdge);
        let fromLanes = self.get_from_lanes();
        let toLanes = self.get_to_lanes();
        if fromLanes.len() != toLanes.len() {
            write_warningf!("Lane sizes differ for connection '%'.", self.base2.get_id());
        } else {
            for index in 0..fromLanes.len() {
                if fromEdge.m().get_num_lanes() <= fromLanes[index] {
                    utils::common::msg_handler::write_warning(&format!("Could not set connection between '{}_{}' and '{}_{}'.", fromEdge.m().get_id(), fromLanes[index], toEdge.m().get_id(), toLanes[index]));
                    unsetConnections += 1;
                } else if !fromEdge.m().add_lane2_lane_connection(fromLanes[index], toEdge, toLanes[index], NBEdge_Lane2LaneInfoType::VALIDATED, true, false, KeepClear::KEEPCLEAR_UNSPECIFIED, UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_SPEED, UNSPECIFIED_FRICTION, UNSPECIFIED_LOADED_LENGTH, &PositionVector::EMPTY, UNSPECIFIED_CONNECTION_UNCONTROLLED, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false) {
                    utils::common::msg_handler::write_warning(&format!("Could not set connection between '{}_{}' and '{}_{}'.", fromEdge.m().get_id(), fromLanes[index], toEdge.m().get_id(), toLanes[index]));
                    unsetConnections += 1;
                }
            }
        }
        unsetConnections
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
        if !self.base2.myGeom.0.is_empty() {
            return;
        }
        let from = self.myFromDef.get_geom_position();
        self.base2.myGeom.0.push(from);
        let to = self.myToDef.get_geom_position();
        self.base2.myGeom.0.push(to);
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
        self.myFromDef.recheck_lanes(fromEdge);
        self.myToDef.recheck_lanes(toEdge);
    }

    /// C++: `const std::vector<int> & NIVissimConnection::getFromLanes() const` [const]
    // const std::vector<int>&
    // NIVissimConnection::getFromLanes() const {
    //     return myFromDef.getLanes();
    // }
    #[allow(unused_variables)]
    pub fn get_from_lanes(&self) -> &Vec<i32> {
        self.myFromDef.get_lanes()
    }

    /// C++: `const std::vector<int> & NIVissimConnection::getToLanes() const` [const]
    // const std::vector<int>&
    // NIVissimConnection::getToLanes() const {
    //     return myToDef.getLanes();
    // }
    #[allow(unused_variables)]
    pub fn get_to_lanes(&self) -> &Vec<i32> {
        self.myToDef.get_lanes()
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
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if let std::collections::btree_map::Entry::Vacant(v) = d.entry(id) {
                v.insert(o);
                true
            } else {
                false
            }
        })
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
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or(Ptr::null()))
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
        let mut ret = Vec::new();
        MY_DICT.with(|d| {
            for (_, c) in d.borrow().iter() {
                if c.crosses(poly, 0.0) {
                    ret.push(c.base2.myID);
                }
            }
        });
        ret
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
        // snapshot the dictionary so the RefCell borrow is not held while clusters are built
        let conns: Vec<Ptr<NIVissimConnection>> = MY_DICT.with(|d| d.borrow().values().copied().collect());
        for e in conns {
            if !e.clustered() {
                debug_assert!(e.base.myBoundary.get().is_some_and(|b| b.xmax() > b.xmin()));
                let connections = NIVissimConnection::get_within(e.get_boundary());
                NIVissimNodeCluster::dictionary_2(-1, -1, &connections, &Vec::new(), true); // 19.5.!!! should be on a single edge
            }
        }
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
        let mut ret = Vec::new();
        MY_DICT.with(|d| {
            for (connID, c) in d.borrow().iter() {
                if c.myFromDef.get_edge_id() == edgeid || c.myToDef.get_edge_id() == edgeid {
                    if !c.base2.has_node_cluster() {
                        ret.push(*connID);
                    }
                }
            }
        });
        ret
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
        let mut unsetConnections: i32 = 0;
        // go through connections
        let conns: Vec<Ptr<NIVissimConnection>> = MY_DICT.with(|d| d.borrow().values().copied().collect());
        for c in conns {
            unsetConnections += c.m().build_edge_connections(ec);
        }
        if unsetConnections != 0 {
            let size = MY_DICT.with(|d| d.borrow().len());
            utils::common::msg_handler::write_warning(&format!("{} of {} connections could not be assigned.", unsetConnections, size));
        }
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
        let conns: Vec<(i32, Ptr<NIVissimConnection>)> = MY_DICT.with(|d| d.borrow().iter().map(|(k, v)| (*k, *v)).collect());
        for (id, c) in conns {
            NIVissimEdge::dictionary_3(c.get_from_edge_id()).m().add_outgoing_connection(id);
            NIVissimEdge::dictionary_3(c.get_to_edge_id()).m().add_incoming_connection(id);
        }
    }

    /// C++: `int NIVissimConnection::getMaxID()` [static]
    // int
    // NIVissimConnection::getMaxID() {
    //     return myMaxID;
    // }
    #[allow(unused_variables)]
    pub fn get_max_id() -> i32 {
        MY_MAX_ID.with(|m| m.get())
    }

}

