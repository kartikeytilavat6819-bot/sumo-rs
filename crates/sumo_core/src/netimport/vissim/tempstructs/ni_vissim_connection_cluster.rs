//! Port of netimport/vissim/tempstructs/NIVissimConnectionCluster.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::io::Write as _;
use utils::common::msg_handler::write_warning;
use utils::geom::geom_helper;

/// C++ typedef `std::vector<NIVissimConnection *> NodeSubCluster::ConnectionCont`
pub type NodeSubCluster_ConnectionCont = Vec<Ptr<NIVissimConnection>>;

/// C++ typedef `std::vector<NIVissimConnectionCluster *> NIVissimConnectionCluster::ContType`
pub type NIVissimConnectionCluster_ContType = Vec<Ptr<NIVissimConnectionCluster>>;

thread_local! {
    /// C++ static `NIVissimConnectionCluster::myClusters`
    static MY_CLUSTERS: std::cell::RefCell<NIVissimConnectionCluster_ContType> = const { std::cell::RefCell::new(Vec::new()) };
    /// C++ static `NIVissimConnectionCluster::myFirstFreeID`
    static MY_FIRST_FREE_ID: std::cell::Cell<i32> = const { std::cell::Cell::new(100000) };
    /// C++ static `NIVissimConnectionCluster::myStaticBlaID`
    static MY_STATIC_BLA_ID: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

/// C++ `VectorHelper<int>::removeDouble`: keeps the first occurrence of every value, in order
fn remove_double_i32(v: &mut Vec<i32>) {
    let mut seen: Vec<i32> = Vec::new();
    v.retain(|x| {
        if seen.contains(x) {
            false
        } else {
            seen.push(*x);
            true
        }
    });
}

/// C++ `myClusters[idx]`
fn cluster_at(idx: usize) -> Ptr<NIVissimConnectionCluster> {
    MY_CLUSTERS.with(|c| c.borrow()[idx])
}

/// C++ `myClusters.size()`
fn cluster_count() -> usize {
    MY_CLUSTERS.with(|c| c.borrow().len())
}

/// C++ `delete *k;` followed by `myClusters.erase(find(myClusters.begin(), myClusters.end(), *k))`
fn erase_and_delete_cluster(k: Ptr<NIVissimConnectionCluster>) {
    MY_CLUSTERS.with(|c| {
        let mut v = c.borrow_mut();
        let pos = v.iter().position(|x| *x == k);
        if let Some(p) = pos {
            v.remove(p);
        }
    });
    drop(unsafe { Box::from_raw(k.as_raw()) });
}

/// C++ `VectorHelper<int>::subSetExists`: true when some element of v1 also occurs in v2
fn sub_set_exists_i32(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    v1.iter().any(|x| v2.contains(x))
}

/// C++ `std::sort(v.begin(), v.end(), same_direction_sorter(angle))`; callers only use the first element
fn sort_by_same_direction(v: &mut Vec<Ptr<NIVissimConnection>>, angle: f64) {
    // determinism: std::sort is introsort (unstable above 16 elements); up to 16 elements it is an
    // insertion sort, which keeps equal elements in input order like slice::sort_by does
    let sorter = NIVissimConnectionCluster_same_direction_sorter::new(angle);
    v.sort_by(|a, b| {
        if sorter.call_op(*a, *b) != 0 {
            std::cmp::Ordering::Less
        } else if sorter.call_op(*b, *a) != 0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

/// This class holds a list of connections either all outgoing or all incoming within an edge, which do lies close together. This shall be the connections which belong to a single node. It still are not all of the node's connections, as other edges may participate to this node, too.
/// C++ class `NIVissimConnectionCluster` (netimport/vissim/tempstructs/NIVissimConnectionCluster.h)
pub struct NIVissimConnectionCluster {
    /// List of connection-ids which participate within this cluster
    pub myConnections: Vec<i32>,
    /// The boundary of the cluster
    pub myBoundary: Boundary,
    /// The node the cluster is assigned to
    pub myNodeCluster: i32,
    pub myEdges: Vec<i32>,
    pub myNodes: Vec<i32>,
    pub myTLs: Vec<i32>,
    pub myOutgoingEdges: Vec<i32>,
    pub myIncomingEdges: Vec<i32>,
    pub myBlaID: i32,
}

// C++ static members of NIVissimConnectionCluster (port as module-level statics, see AGENTS.md):
//   static ContType NIVissimConnectionCluster::myClusters;
//   static int NIVissimConnectionCluster::myFirstFreeID;
//   static int NIVissimConnectionCluster::myStaticBlaID;

impl NIVissimConnectionCluster {
    /// C++: `void NIVissimConnectionCluster::NIVissimConnectionCluster(const std::vector<int> &, int, int)`
    /// Constructor Build the boundary; The boundary includes both incoming and outgoing nodes
    // NIVissimConnectionCluster::NIVissimConnectionCluster(
    //     const std::vector<int>& connections, int nodeCluster, int edgeid)
    //     : myConnections(connections), myNodeCluster(nodeCluster),
    //       myBlaID(myStaticBlaID++) {
    //     recomputeBoundary();
    //     myClusters.push_back(this);
    //     assert(edgeid > 0);
    //     if (edgeid >= 0) {
    //         myEdges.push_back(edgeid);
    //     }
    //     // add information about incoming and outgoing edges
    //     for (std::vector<int>::const_iterator i = connections.begin(); i != connections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         assert(c != 0);
    //         myOutgoingEdges.push_back(c->getToEdgeID());
    //         myIncomingEdges.push_back(c->getFromEdgeID());
    //         assert(c->getFromEdgeID() == edgeid || c->getToEdgeID() == edgeid);
    //     }
    //     VectorHelper<int>::removeDouble(myIncomingEdges);
    //     VectorHelper<int>::removeDouble(myOutgoingEdges);
    // }
    /// Returns a Box because the constructor registers `this` in myClusters
    #[allow(unused_variables)]
    pub fn new(connections: &Vec<i32>, nodeCluster: i32, edgeid: i32) -> Box<Self> {
        let blaID = MY_STATIC_BLA_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        });
        let mut b = Box::new(Self {
            myConnections: connections.clone(),
            myBoundary: Boundary::default(),
            myNodeCluster: nodeCluster,
            myEdges: Vec::new(),
            myNodes: Vec::new(),
            myTLs: Vec::new(),
            myOutgoingEdges: Vec::new(),
            myIncomingEdges: Vec::new(),
            myBlaID: blaID,
        });
        b.recompute_boundary();
        MY_CLUSTERS.with(|c| c.borrow_mut().push(Ptr::from_box(&b)));
        debug_assert!(edgeid > 0);
        if edgeid >= 0 {
            b.myEdges.push(edgeid);
        }
        // add information about incoming and outgoing edges
        for i in connections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            debug_assert!(!c.is_null());
            b.myOutgoingEdges.push(c.get_to_edge_id());
            b.myIncomingEdges.push(c.get_from_edge_id());
            debug_assert!(c.get_from_edge_id() == edgeid || c.get_to_edge_id() == edgeid);
        }
        remove_double_i32(&mut b.myIncomingEdges);
        remove_double_i32(&mut b.myOutgoingEdges);
        b
    }

    /// C++: `void NIVissimConnectionCluster::NIVissimConnectionCluster(const std::vector<int> &, const Boundary &, int, const std::vector<int> &)`
    // NIVissimConnectionCluster::NIVissimConnectionCluster(
    //     const std::vector<int>& connections, const Boundary& boundary,
    //     int nodeCluster, const std::vector<int>& edges)
    //     : myConnections(connections), myBoundary(boundary),
    //       myNodeCluster(nodeCluster), myEdges(edges) {
    //     myClusters.push_back(this);
    //     recomputeBoundary();
    //     assert(myBoundary.xmax() >= myBoundary.xmin());
    //     // add information about incoming and outgoing edges
    //     for (std::vector<int>::const_iterator i = connections.begin(); i != connections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         assert(c != 0);
    //         myOutgoingEdges.push_back(c->getToEdgeID());
    //         myIncomingEdges.push_back(c->getFromEdgeID());
    //         assert(find(edges.begin(), edges.end(), c->getFromEdgeID()) != edges.end()
    //                ||
    //                std::find(edges.begin(), edges.end(), c->getToEdgeID()) != edges.end());
    //     }
    //     VectorHelper<int>::removeDouble(myIncomingEdges);
    //     VectorHelper<int>::removeDouble(myOutgoingEdges);
    // }
    /// Returns a Box because the constructor registers `this` in myClusters
    #[allow(unused_variables)]
    pub fn new_2(connections: &Vec<i32>, boundary: &Boundary, nodeCluster: i32, edges: &Vec<i32>) -> Box<Self> {
        let blaID = MY_STATIC_BLA_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        });
        let mut b = Box::new(Self {
            myConnections: connections.clone(),
            myBoundary: *boundary,
            myNodeCluster: nodeCluster,
            myEdges: edges.clone(),
            myNodes: Vec::new(),
            myTLs: Vec::new(),
            myOutgoingEdges: Vec::new(),
            myIncomingEdges: Vec::new(),
            myBlaID: blaID,
        });
        MY_CLUSTERS.with(|c| c.borrow_mut().push(Ptr::from_box(&b)));
        b.recompute_boundary();
        debug_assert!(b.myBoundary.xmax() >= b.myBoundary.xmin());
        // add information about incoming and outgoing edges
        for i in connections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            debug_assert!(!c.is_null());
            b.myOutgoingEdges.push(c.get_to_edge_id());
            b.myIncomingEdges.push(c.get_from_edge_id());
            debug_assert!(edges.contains(&c.get_from_edge_id()) || edges.contains(&c.get_to_edge_id()));
        }
        remove_double_i32(&mut b.myIncomingEdges);
        remove_double_i32(&mut b.myOutgoingEdges);
        b
    }

    /// C++: `void NIVissimConnectionCluster::~NIVissimConnectionCluster()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: no members need explicit release
    }

    /// C++: `bool NIVissimConnectionCluster::overlapsWith(NIVissimConnectionCluster *, double) const` [const]
    /// Returns the information whether the given cluster overlaps the current
    // bool
    // NIVissimConnectionCluster::overlapsWith(NIVissimConnectionCluster* c,
    //                                         double offset) const {
    //     assert(myBoundary.xmax() >= myBoundary.xmin());
    //     assert(c->myBoundary.xmax() >= c->myBoundary.xmin());
    //     return c->myBoundary.overlapsWith(myBoundary, offset);
    // }
    #[allow(unused_variables)]
    pub fn overlaps_with(&self, c: Ptr<NIVissimConnectionCluster>, offset: f64) -> bool {
        debug_assert!(self.myBoundary.xmax() >= self.myBoundary.xmin());
        debug_assert!(c.myBoundary.xmax() >= c.myBoundary.xmin());
        c.myBoundary.overlaps_with(&self.myBoundary, offset)
    }

    /// C++: `bool NIVissimConnectionCluster::hasNodeCluster() const` [const]
    // bool
    // NIVissimConnectionCluster::hasNodeCluster() const {
    //     return myNodeCluster != -1;
    // }
    #[allow(unused_variables)]
    pub fn has_node_cluster(&self) -> bool {
        self.myNodeCluster != -1
    }

    /// C++: `NBNode * NIVissimConnectionCluster::getNBNode() const` [const]
    // NBNode*
    // NIVissimConnectionCluster::getNBNode() const {
    //     return NIVissimNodeCluster::dictionary(myNodeCluster)->getNBNode();
    // }
    #[allow(unused_variables)]
    pub fn get_nb_node(&self) -> Ptr<NBNode> {
        NIVissimNodeCluster::dictionary_3(self.myNodeCluster).get_nb_node()
    }

    /// C++: `bool NIVissimConnectionCluster::around(const Position &, double) const` [const]
    // bool
    // NIVissimConnectionCluster::around(const Position& p, double offset) const {
    //     assert(myBoundary.xmax() >= myBoundary.xmin());
    //     return myBoundary.around(p, offset);
    // }
    #[allow(unused_variables)]
    pub fn around(&self, p: &Position, offset: f64) -> bool {
        debug_assert!(self.myBoundary.xmax() >= self.myBoundary.xmin());
        self.myBoundary.around(p, offset)
    }

    /// C++: `double NIVissimConnectionCluster::getPositionForEdge(int) const` [const]
    // double
    // NIVissimConnectionCluster::getPositionForEdge(int edgeid) const {
    //     // return the middle of the connections when there are any
    //     if (myConnections.size() != 0) {
    //         double sum = 0;
    //         int part = 0;
    //         std::vector<int>::const_iterator i;
    //         for (i = myConnections.begin(); i != myConnections.end(); i++) {
    //             NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //             if (c->getFromEdgeID() == edgeid) {
    //                 part++;
    //                 sum += c->getFromPosition();
    //             }
    //             if (c->getToEdgeID() == edgeid) {
    //                 part++;
    //                 sum += c->getToPosition();
    //             }
    //         }
    //         if (part > 0) {
    //             return sum / (double) part;
    //         }
    //     }
    //     // use the position of the node if possible
    //     if (myNodeCluster >= 0) {
    //         // try to find the nearest point on the edge
    //         //  !!! only the main geometry is regarded
    //         NIVissimNodeDef* node =
    //             NIVissimNodeDef::dictionary(myNodeCluster);
    //         if (node != nullptr) {
    //             double pos = node->getEdgePosition(edgeid);
    //             if (pos >= 0) {
    //                 return pos;
    //             }
    //         }
    //         /*
    //                 double try1 = GeomHelper::nearest_offset_on_line_to_point(
    //                     edge->getBegin2D(), edge->getEnd2D(), node->getPos());
    //                 if(try1>=0) {
    //                     return try1;
    //                 }
    //                 // try to use simple distance
    //                 double dist1 =
    //                     GeomHelper::distance(node->getPos(), edge->getBegin2D());
    //                 double dist2 =
    //                     GeomHelper::distance(node->getPos(), edge->getEnd2D());
    //                 return dist1<dist2
    //                     ? 0 : edge->getLength();
    //                     */
    //     }
    //     // what else?
    //     WRITE_WARNING(TL("NIVissimConnectionCluster: how to get an edge's position?"));
    //     // !!!
    //     assert(myBoundary.xmin() <= myBoundary.xmax());
    //     NIVissimEdge* edge = NIVissimEdge::dictionary(edgeid);
    //     std::vector<int>::const_iterator i = std::find(myEdges.begin(), myEdges.end(), edgeid);
    //     if (i == myEdges.end()) {
    //         // edge does not exist!?
    //         throw 1;
    //     }
    //     const PositionVector& edgeGeom = edge->getGeometry();
    //     Position p = GeomHelper::crossPoint(myBoundary, edgeGeom);
    //     return GeomHelper::nearest_offset_on_line_to_point2D(
    //                edgeGeom.front(), edgeGeom.back(), p);
    // }
    #[allow(unused_variables)]
    pub fn get_position_for_edge(&self, edgeid: i32) -> f64 {
        // return the middle of the connections when there are any
        if self.myConnections.len() != 0 {
            let mut sum: f64 = 0.0;
            let mut part: i32 = 0;
            for i in self.myConnections.iter() {
                let c = NIVissimConnection::dictionary_2(*i);
                if c.get_from_edge_id() == edgeid {
                    part += 1;
                    sum += c.get_from_position();
                }
                if c.get_to_edge_id() == edgeid {
                    part += 1;
                    sum += c.get_to_position();
                }
            }
            if part > 0 {
                return sum / (part as f64);
            }
        }
        // use the position of the node if possible
        if self.myNodeCluster >= 0 {
            // try to find the nearest point on the edge
            //  !!! only the main geometry is regarded
            let node = NIVissimNodeDef::dictionary_2(self.myNodeCluster);
            if !node.is_null() {
                let pos = node.get_edge_position(edgeid);
                if pos >= 0.0 {
                    return pos;
                }
            }
        }
        // what else?
        write_warning("NIVissimConnectionCluster: how to get an edge's position?");
        // !!!
        debug_assert!(self.myBoundary.xmin() <= self.myBoundary.xmax());
        let edge = NIVissimEdge::dictionary_3(edgeid);
        if !self.myEdges.contains(&edgeid) {
            // C++: throw 1 (edge does not exist)
            panic!("NIVissimConnectionCluster::getPositionForEdge: edge {} does not exist (C++ throws 1)", edgeid);
        }
        let edge_geom = edge.get_geometry();
        // C++ crossPoint throws 1 when there is no crossing; Option::None maps to that
        let p = geom_helper::cross_point(&self.myBoundary, edge_geom)
            .expect("GeomHelper::crossPoint: no crossing (C++ throws 1)");
        // C++ default for perpendicular is true
        geom_helper::nearest_offset_on_line_to_point2d(edge_geom.front(), edge_geom.back(), &p, true)
    }

    /// C++: `const std::vector<int> & NIVissimConnectionCluster::getConnections() const` [const]
    // const std::vector<int>& getConnections() const {
    //         return myConnections;
    //     }
    #[allow(unused_variables)]
    pub fn get_connections(&self) -> &Vec<i32> {
        &self.myConnections
    }

    /// C++: `PositionVector NIVissimConnectionCluster::getIncomingContinuationGeometry(NIVissimEdge *) const` [const]
    // PositionVector
    // NIVissimConnectionCluster::getIncomingContinuationGeometry(NIVissimEdge* e) const {
    //     // collect connection where this edge is the incoming one
    //     std::vector<NIVissimConnection*> edgeIsIncoming;
    //     for (std::vector<int>::const_iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         if (c->getFromEdgeID() == e->getID()) {
    //             edgeIsIncoming.push_back(c);
    //         }
    //     }
    //     //
    //     if (edgeIsIncoming.size() == 0) {
    //         return PositionVector();
    //     }
    //     // sort connected edges in same direction
    //     sort(edgeIsIncoming.begin(), edgeIsIncoming.end(),
    //          same_direction_sorter(e->getGeometry().beginEndAngle()));
    //     NIVissimConnection* c = *(edgeIsIncoming.begin());
    //     return c->getGeometry();
    // }
    #[allow(unused_variables)]
    pub fn get_incoming_continuation_geometry(&self, e: Ptr<NIVissimEdge>) -> PositionVector {
        // collect connection where this edge is the incoming one
        let mut edge_is_incoming: Vec<Ptr<NIVissimConnection>> = Vec::new();
        for i in self.myConnections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            if c.get_from_edge_id() == e.get_id() {
                edge_is_incoming.push(c);
            }
        }
        //
        if edge_is_incoming.is_empty() {
            return PositionVector::default();
        }
        // sort connected edges in same direction
        sort_by_same_direction(&mut edge_is_incoming, e.get_geometry().begin_end_angle());
        let c = edge_is_incoming[0];
        c.base2.get_geometry().clone()
    }

    /// C++: `PositionVector NIVissimConnectionCluster::getOutgoingContinuationGeometry(NIVissimEdge *) const` [const]
    // PositionVector
    // NIVissimConnectionCluster::getOutgoingContinuationGeometry(NIVissimEdge* e) const {
    //     // collect connection where this edge is the outgoing one
    //     std::vector<NIVissimConnection*> edgeIsOutgoing;
    //     for (std::vector<int>::const_iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         if (c->getToEdgeID() == e->getID()) {
    //             edgeIsOutgoing.push_back(c);
    //         }
    //     }
    //     //
    //     if (edgeIsOutgoing.size() == 0) {
    //         return PositionVector();
    //     }
    //     // sort connected edges in same direction
    //     sort(edgeIsOutgoing.begin(), edgeIsOutgoing.end(),
    //          same_direction_sorter(e->getGeometry().beginEndAngle()));
    //     NIVissimConnection* c = *(edgeIsOutgoing.begin());
    //     return c->getGeometry();
    // }
    #[allow(unused_variables)]
    pub fn get_outgoing_continuation_geometry(&self, e: Ptr<NIVissimEdge>) -> PositionVector {
        // collect connection where this edge is the outgoing one
        let mut edge_is_outgoing: Vec<Ptr<NIVissimConnection>> = Vec::new();
        for i in self.myConnections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            if c.get_to_edge_id() == e.get_id() {
                edge_is_outgoing.push(c);
            }
        }
        //
        if edge_is_outgoing.is_empty() {
            return PositionVector::default();
        }
        // sort connected edges in same direction
        sort_by_same_direction(&mut edge_is_outgoing, e.get_geometry().begin_end_angle());
        let c = edge_is_outgoing[0];
        c.base2.get_geometry().clone()
    }

    /// C++: `NIVissimConnection * NIVissimConnectionCluster::getIncomingContinuation(NIVissimEdge *) const` [const]
    // NIVissimConnection*
    // NIVissimConnectionCluster::getIncomingContinuation(NIVissimEdge* e) const {
    //     // collect connection where this edge is the incoming one
    //     std::vector<NIVissimConnection*> edgeIsIncoming;
    //     for (std::vector<int>::const_iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         if (c->getFromEdgeID() == e->getID()) {
    //             edgeIsIncoming.push_back(c);
    //         }
    //     }
    //     //
    //     if (edgeIsIncoming.size() == 0) {
    //         return nullptr;
    //     }
    //     // sort connected edges in same direction
    //     sort(edgeIsIncoming.begin(), edgeIsIncoming.end(),
    //          same_direction_sorter(e->getGeometry().beginEndAngle()));
    //     return *(edgeIsIncoming.begin());
    // }
    #[allow(unused_variables)]
    pub fn get_incoming_continuation(&self, e: Ptr<NIVissimEdge>) -> Ptr<NIVissimConnection> {
        // collect connection where this edge is the incoming one
        let mut edge_is_incoming: Vec<Ptr<NIVissimConnection>> = Vec::new();
        for i in self.myConnections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            if c.get_from_edge_id() == e.get_id() {
                edge_is_incoming.push(c);
            }
        }
        //
        if edge_is_incoming.is_empty() {
            return Ptr::null();
        }
        // sort connected edges in same direction
        sort_by_same_direction(&mut edge_is_incoming, e.get_geometry().begin_end_angle());
        edge_is_incoming[0]
    }

    /// C++: `NIVissimConnection * NIVissimConnectionCluster::getOutgoingContinuation(NIVissimEdge *) const` [const]
    // NIVissimConnection*
    // NIVissimConnectionCluster::getOutgoingContinuation(NIVissimEdge* e) const {
    //     // collect connection where this edge is the outgoing one
    //     std::vector<NIVissimConnection*> edgeIsOutgoing;
    //     for (std::vector<int>::const_iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         if (c->getToEdgeID() == e->getID()) {
    //             edgeIsOutgoing.push_back(c);
    //         }
    //     }
    //     //
    //     if (edgeIsOutgoing.size() == 0) {
    //         return nullptr;
    //     }
    //     // sort connected edges in same direction
    //     sort(edgeIsOutgoing.begin(), edgeIsOutgoing.end(),
    //          same_direction_sorter(e->getGeometry().beginEndAngle()));
    //     return *(edgeIsOutgoing.begin());
    // }
    #[allow(unused_variables)]
    pub fn get_outgoing_continuation(&self, e: Ptr<NIVissimEdge>) -> Ptr<NIVissimConnection> {
        // collect connection where this edge is the outgoing one
        let mut edge_is_outgoing: Vec<Ptr<NIVissimConnection>> = Vec::new();
        for i in self.myConnections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            if c.get_to_edge_id() == e.get_id() {
                edge_is_outgoing.push(c);
            }
        }
        //
        if edge_is_outgoing.is_empty() {
            return Ptr::null();
        }
        // sort connected edges in same direction
        sort_by_same_direction(&mut edge_is_outgoing, e.get_geometry().begin_end_angle());
        edge_is_outgoing[0]
    }

    /// C++: `void NIVissimConnectionCluster::joinBySameEdges(double)` [static]
    /// Tries to joind clusters participating within a node This is done by joining clusters which overlap
    // void
    // NIVissimConnectionCluster::joinBySameEdges(double offset) {
    //     // !!! ...
    //     // Further, we try to omit joining of overlaping nodes. This is done by holding
    //     //  the lists of incoming and outgoing edges and incrementally building the nodes
    //     //  regarding this information
    //     std::vector<NIVissimConnectionCluster*> joinAble;
    //     int pos = 0;
    //     ContType::iterator i = myClusters.begin();
    //     // step1 - faster but no complete
    //     while (i != myClusters.end()) {
    //         joinAble.clear();
    //         ContType::iterator j = i + 1;
    //
    //         // check whether every combination has been processed
    //         while (j != myClusters.end()) {
    //             // check whether the current clusters overlap
    //             if ((*i)->joinable(*j, offset)) {
    //                 joinAble.push_back(*j);
    //             }
    //             j++;
    //         }
    //         for (std::vector<NIVissimConnectionCluster*>::iterator k = joinAble.begin();
    //                 k != joinAble.end(); k++) {
    //             // add the overlaping cluster
    //             (*i)->add(*k);
    //             // erase the overlaping cluster
    //             delete *k;
    //             myClusters.erase(find(myClusters.begin(), myClusters.end(), *k));
    //         }
    //         //
    //         if (joinAble.size() > 0) {
    //             i = myClusters.begin() + pos;
    //             // clear temporary storages
    //             joinAble.clear();
    //         } else {
    //             i++;
    //             pos++;
    //         }
    //     }
    //     //
    //     pos = 0;
    //     i = myClusters.begin();
    //     while (i != myClusters.end()) {
    //         ContType::iterator j = i + 1;
    //         // check whether every combination has been processed
    //         while (j != myClusters.end()) {
    //             // check whether the current clusters overlap
    //             if ((*i)->joinable(*j, offset)) {
    //                 joinAble.push_back(*j);
    //             }
    //             j++;
    //         }
    //         for (std::vector<NIVissimConnectionCluster*>::iterator k = joinAble.begin();
    //                 k != joinAble.end(); k++) {
    //             // add the overlaping cluster
    //             (*i)->add(*k);
    //             // erase the overlaping cluster
    //             delete *k;
    //             myClusters.erase(find(myClusters.begin(), myClusters.end(), *k));
    //         }
    //         //
    //         if (joinAble.size() > 0) {
    //             i = myClusters.begin();
    //             // clear temporary storages
    //             joinAble.clear();
    //             pos = 0;
    //         } else {
    //             i++;
    //             pos++;
    //         }
    //     }
    //     // check for weak district connections
    //     //  (junctions made up by district connections, where prohibitions are not
    //     //   modelled properly)
    //     pos = 0;
    //     i = myClusters.begin();
    //     while (i != myClusters.end()) {
    //         ContType::iterator j = i + 1;
    //         // check whether every combination has been processed
    //         while (j != myClusters.end()) {
    //             // check whether the current clusters overlap
    //             if ((*i)->isWeakDistrictConnRealisation(*j)) {
    //                 joinAble.push_back(*j);
    //             }
    //             j++;
    //         }
    //         for (std::vector<NIVissimConnectionCluster*>::iterator k = joinAble.begin();
    //                 k != joinAble.end(); k++) {
    //             // add the overlaping cluster
    //             (*i)->add(*k);
    //             // erase the overlaping cluster
    //             delete *k;
    //             myClusters.erase(find(myClusters.begin(), myClusters.end(), *k));
    //         }
    //         //
    //         if (joinAble.size() > 0) {
    //             i = myClusters.begin();
    //             // clear temporary storages
    //             joinAble.clear();
    //             pos = 0;
    //         } else {
    //             i++;
    //             pos++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join_by_same_edges(offset: f64) {
        // determinism: C++ walks myClusters with vector iterators; here the same order is kept by index
        let mut join_able: Vec<Ptr<NIVissimConnectionCluster>> = Vec::new();
        let mut pos: usize = 0;
        let mut i: usize = 0;
        // step1 - faster but no complete
        while i < cluster_count() {
            join_able.clear();
            let mut j = i + 1;
            // check whether every combination has been processed
            while j < cluster_count() {
                // check whether the current clusters overlap
                if cluster_at(i).m().joinable(cluster_at(j), offset) {
                    join_able.push(cluster_at(j));
                }
                j += 1;
            }
            for k in join_able.iter() {
                // add the overlaping cluster
                cluster_at(i).m().add(*k);
                // erase the overlaping cluster
                erase_and_delete_cluster(*k);
            }
            //
            if join_able.len() > 0 {
                i = pos;
                // clear temporary storages
                join_able.clear();
            } else {
                i += 1;
                pos += 1;
            }
        }
        //
        pos = 0;
        i = 0;
        while i < cluster_count() {
            let mut j = i + 1;
            // check whether every combination has been processed
            while j < cluster_count() {
                // check whether the current clusters overlap
                if cluster_at(i).m().joinable(cluster_at(j), offset) {
                    join_able.push(cluster_at(j));
                }
                j += 1;
            }
            for k in join_able.iter() {
                // add the overlaping cluster
                cluster_at(i).m().add(*k);
                // erase the overlaping cluster
                erase_and_delete_cluster(*k);
            }
            //
            if join_able.len() > 0 {
                i = 0;
                // clear temporary storages
                join_able.clear();
                pos = 0;
            } else {
                i += 1;
                pos += 1;
            }
        }
        // check for weak district connections
        //  (junctions made up by district connections, where prohibitions are not
        //   modelled properly)
        pos = 0;
        i = 0;
        while i < cluster_count() {
            let mut j = i + 1;
            // check whether every combination has been processed
            while j < cluster_count() {
                // check whether the current clusters overlap
                if cluster_at(i).m().is_weak_district_conn_realisation(cluster_at(j)) {
                    join_able.push(cluster_at(j));
                }
                j += 1;
            }
            for k in join_able.iter() {
                // add the overlaping cluster
                cluster_at(i).m().add(*k);
                // erase the overlaping cluster
                erase_and_delete_cluster(*k);
            }
            //
            if join_able.len() > 0 {
                i = 0;
                // clear temporary storages
                join_able.clear();
                pos = 0;
            } else {
                i += 1;
                pos += 1;
            }
        }
    }

    /// C++: `void NIVissimConnectionCluster::joinByDisturbances(double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn join_by_disturbances(offset: f64) {
        // C++ declares joinByDisturbances but never defines it; its only call site is commented out
        panic!("NIVissimConnectionCluster::joinByDisturbances is declared but never defined in C++ (link error if called)")
    }

    /// C++: `void NIVissimConnectionCluster::buildNodeClusters()` [static]
    // void
    // NIVissimConnectionCluster::buildNodeClusters() {
    //     for (ContType::iterator i = myClusters.begin(); i != myClusters.end(); i++) {
    //         std::vector<int> disturbances;
    //         std::vector<int> tls;
    //         std::vector<int> nodes;
    //         int tlsid = -1;
    //         int nodeid = -1;
    //         if ((*i)->myConnections.size() > 0) {
    //             (*i)->recomputeBoundary();
    //             disturbances = NIVissimDisturbance::getWithin((*i)->myBoundary);
    //         }
    //         nodes = (*i)->myNodes;//NIVissimTL::getWithin((*i)->myBoundary, 5.0);
    //         if (nodes.size() > 1) {
    //             WRITE_WARNING(TL("NIVissimConnectionCluster: More than a single node"));
    //             //          throw 1; // !!! eigentlich sollte hier nur eine Ampelanlage sein
    //         }
    //         if (nodes.size() > 0) {
    //             nodeid = nodes[0];
    //         }
    //         //
    //         //
    //         int id = NIVissimNodeCluster::dictionary(
    //                      nodeid, tlsid, (*i)->myConnections,
    //                      disturbances, (*i)->myIncomingEdges.size() < 2);
    //         assert((*i)->myNodeCluster == id || (*i)->myNodeCluster < 0);
    //         (*i)->myNodeCluster = id;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_node_clusters() {
        // determinism: iterate over a snapshot in the same order; dictionary_2 below never adds or removes clusters
        let clusters = MY_CLUSTERS.with(|c| c.borrow().clone());
        for i in clusters.iter() {
            let mut disturbances: Vec<i32> = Vec::new();
            let tlsid: i32 = -1;
            let mut nodeid: i32 = -1;
            if i.myConnections.len() > 0 {
                i.m().recompute_boundary();
                disturbances = NIVissimDisturbance::get_within(&i.myBoundary);
            }
            let nodes = i.myNodes.clone(); //NIVissimTL::getWithin((*i)->myBoundary, 5.0);
            if nodes.len() > 1 {
                write_warning("NIVissimConnectionCluster: More than a single node");
                //          throw 1; // !!! eigentlich sollte hier nur eine Ampelanlage sein
            }
            if nodes.len() > 0 {
                nodeid = nodes[0];
            }
            //
            //
            let id = NIVissimNodeCluster::dictionary_2(
                nodeid, tlsid, &i.myConnections,
                &disturbances, i.myIncomingEdges.len() < 2);
            debug_assert!(i.myNodeCluster == id || i.myNodeCluster < 0);
            i.m().myNodeCluster = id;
        }
    }

    /// C++: `void NIVissimConnectionCluster::_debugOut(std::ostream &)` [static]
    // void
    // NIVissimConnectionCluster::_debugOut(std::ostream& into) {
    //     for (ContType::iterator i = myClusters.begin(); i != myClusters.end(); i++) {
    //         std::vector<int> connections = (*i)->myConnections;
    //         for (std::vector<int>::iterator j = connections.begin(); j != connections.end(); j++) {
    //             if (j != connections.begin()) {
    //                 into << ", ";
    //             }
    //             into << *j;
    //         }
    //         into << "(" << (*i)->myBoundary << ")" << std::endl;
    //     }
    //     into << "---------------------------" << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn debug_out(into: &mut dyn std::io::Write) {
        for i in MY_CLUSTERS.with(|c| c.borrow().clone()).iter() {
            let connections = i.myConnections.clone();
            for (idx, j) in connections.iter().enumerate() {
                if idx != 0 {
                    let _ = write!(into, ", ");
                }
                let _ = write!(into, "{}", j);
            }
            let _ = writeln!(into, "({})", i.myBoundary);
        }
        let _ = writeln!(into, "---------------------------");
    }

    /// C++: `int NIVissimConnectionCluster::getNextFreeNodeID()` [static]
    // int
    // NIVissimConnectionCluster::getNextFreeNodeID() {
    //     return myFirstFreeID++;
    // }
    #[allow(unused_variables)]
    pub fn get_next_free_node_id() -> i32 {
        MY_FIRST_FREE_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        })
    }

    /// C++: `void NIVissimConnectionCluster::clearDict()` [static]
    // void
    // NIVissimConnectionCluster::clearDict() {
    //     for (ContType::iterator i = myClusters.begin(); i != myClusters.end(); i++) {
    //         delete (*i);
    //     }
    //     myClusters.clear();
    //     myFirstFreeID = 100000;
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        let clusters = MY_CLUSTERS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        for i in clusters.iter() {
            // delete (*i)
            drop(unsafe { Box::from_raw(i.as_raw()) });
        }
        MY_FIRST_FREE_ID.with(|c| c.set(100000));
    }

    /// C++: `void NIVissimConnectionCluster::add(NIVissimConnectionCluster *)` [private]
    /// Adds the second cluster
    // void
    // NIVissimConnectionCluster::add(NIVissimConnectionCluster* c) {
    //     assert(myBoundary.xmax() >= myBoundary.xmin());
    //     assert(c->myBoundary.xmax() >= c->myBoundary.xmin());
    //     myBoundary.add(c->myBoundary);
    //     for (std::vector<int>::iterator i = c->myConnections.begin(); i != c->myConnections.end(); i++) {
    //         myConnections.push_back(*i);
    //     }
    //     VectorHelper<int>::removeDouble(myConnections);
    //     assert(myNodeCluster == -1 || c->myNodeCluster == -1);
    //     if (myNodeCluster == -1) {
    //         myNodeCluster = c->myNodeCluster;
    //     }
    //     // inform edges about merging
    //     //  !!! merge should be done within one method
    //     for (std::vector<int>::iterator j = c->myEdges.begin(); j != c->myEdges.end(); j++) {
    //         NIVissimEdge::dictionary(*j)->mergedInto(c, this);
    //     }
    //     copy(c->myEdges.begin(), c->myEdges.end(), back_inserter(myEdges));
    //     copy(c->myIncomingEdges.begin(), c->myIncomingEdges.end(),
    //          back_inserter(myIncomingEdges));
    //     copy(c->myOutgoingEdges.begin(), c->myOutgoingEdges.end(),
    //          back_inserter(myOutgoingEdges));
    //     VectorHelper<int>::removeDouble(myEdges);
    //     VectorHelper<int>::removeDouble(myIncomingEdges);
    //     VectorHelper<int>::removeDouble(myOutgoingEdges);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, c: Ptr<NIVissimConnectionCluster>) {
        debug_assert!(self.myBoundary.xmax() >= self.myBoundary.xmin());
        debug_assert!(c.myBoundary.xmax() >= c.myBoundary.xmin());
        self.myBoundary.add_boundary(&c.myBoundary);
        self.myConnections.extend_from_slice(&c.myConnections);
        remove_double_i32(&mut self.myConnections);
        debug_assert!(self.myNodeCluster == -1 || c.myNodeCluster == -1);
        if self.myNodeCluster == -1 {
            self.myNodeCluster = c.myNodeCluster;
        }
        // inform edges about merging
        //  !!! merge should be done within one method
        let this = Ptr::from_mut(&mut *self);
        for j in c.myEdges.iter() {
            NIVissimEdge::dictionary_3(*j).m().merged_into(c, this);
        }
        self.myEdges.extend_from_slice(&c.myEdges);
        self.myIncomingEdges.extend_from_slice(&c.myIncomingEdges);
        self.myOutgoingEdges.extend_from_slice(&c.myOutgoingEdges);
        remove_double_i32(&mut self.myEdges);
        remove_double_i32(&mut self.myIncomingEdges);
        remove_double_i32(&mut self.myOutgoingEdges);
    }

    /// C++: `void NIVissimConnectionCluster::removeConnections(const NodeSubCluster &)` [private]
    // void
    // NIVissimConnectionCluster::removeConnections(const NodeSubCluster& c) {
    //     for (NodeSubCluster::ConnectionCont::const_iterator i = c.myConnections.begin(); i != c.myConnections.end(); i++) {
    //         NIVissimConnection* conn = *i;
    //         int connid = conn->getID();
    //         std::vector<int>::iterator j = std::find(myConnections.begin(), myConnections.end(), connid);
    //         if (j != myConnections.end()) {
    //             myConnections.erase(j);
    //         }
    //     }
    //     recomputeBoundary();
    // }
    #[allow(unused_variables)]
    pub fn remove_connections(&mut self, c: &NIVissimConnectionCluster_NodeSubCluster) {
        for conn in c.myConnections.iter() {
            let connid = conn.base2.get_id();
            let pos = self.myConnections.iter().position(|x| *x == connid);
            if let Some(j) = pos {
                self.myConnections.remove(j);
            }
        }
        self.recompute_boundary();
    }

    /// C++: `void NIVissimConnectionCluster::recomputeBoundary()` [private]
    // void
    // NIVissimConnectionCluster::recomputeBoundary() {
    //     myBoundary = Boundary();
    //     for (std::vector<int>::iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         if (c != nullptr) {
    //             myBoundary.add(c->getFromGeomPosition());
    //             myBoundary.add(c->getToGeomPosition());
    //             if (c->getGeometry().size() != 0) {
    //                 myBoundary.add(c->getGeometry().getBoxBoundary());
    //             }
    //         }
    //     }
    //     assert(myBoundary.xmax() >= myBoundary.xmin());
    // }
    #[allow(unused_variables)]
    pub fn recompute_boundary(&mut self) {
        self.myBoundary = Boundary::default();
        for i in self.myConnections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            if !c.is_null() {
                self.myBoundary.add_pos(&c.get_from_geom_position());
                self.myBoundary.add_pos(&c.get_to_geom_position());
                if c.base2.get_geometry().len() != 0 {
                    self.myBoundary.add_boundary(&c.base2.get_geometry().get_box_boundary());
                }
            }
        }
        debug_assert!(self.myBoundary.xmax() >= self.myBoundary.xmin());
    }

    /// C++: `void NIVissimConnectionCluster::recheckEdges()` [private]
    // void
    // NIVissimConnectionCluster::recheckEdges() {
    //     assert(myConnections.size() != 0);
    //     // remove the cluster from all edges at first
    //     std::vector<int>::iterator i;
    //     for (i = myEdges.begin(); i != myEdges.end(); i++) {
    //         NIVissimEdge* edge = NIVissimEdge::dictionary(*i);
    //         edge->removeFromConnectionCluster(this);
    //     }
    //     // clear edge information
    //     myEdges.clear();
    //     // recheck which edges do still participate and add edges
    //     for (i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         assert(myBoundary.xmax() >= myBoundary.xmin());
    //         if (myBoundary.around(c->getFromGeomPosition(), 5)) {
    //             myEdges.push_back(c->getFromEdgeID());
    //         }
    //         assert(myBoundary.xmax() >= myBoundary.xmin());
    //         if (myBoundary.around(c->getToGeomPosition(), 5)) {
    //             myEdges.push_back(c->getToEdgeID());
    //         }
    //     }
    //     // connect edges
    //     for (i = myEdges.begin(); i != myEdges.end(); i++) {
    //         NIVissimEdge* edge = NIVissimEdge::dictionary(*i);
    //         edge->addToConnectionCluster(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_edges(&mut self) {
        debug_assert!(!self.myConnections.is_empty());
        let this = Ptr::from_mut(&mut *self);
        // remove the cluster from all edges at first
        for i in self.myEdges.iter() {
            let edge = NIVissimEdge::dictionary_3(*i);
            edge.m().remove_from_connection_cluster(this);
        }
        // clear edge information
        self.myEdges.clear();
        // recheck which edges do still participate and add edges
        for i in self.myConnections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            debug_assert!(self.myBoundary.xmax() >= self.myBoundary.xmin());
            if self.myBoundary.around(&c.get_from_geom_position(), 5.0) {
                self.myEdges.push(c.get_from_edge_id());
            }
            debug_assert!(self.myBoundary.xmax() >= self.myBoundary.xmin());
            if self.myBoundary.around(&c.get_to_geom_position(), 5.0) {
                self.myEdges.push(c.get_to_edge_id());
            }
        }
        // connect edges
        for i in self.myEdges.iter() {
            let edge = NIVissimEdge::dictionary_3(*i);
            edge.m().add_to_connection_cluster(this);
        }
    }

    /// C++: `bool NIVissimConnectionCluster::joinable(NIVissimConnectionCluster *, double)` [private]
    // bool
    // NIVissimConnectionCluster::joinable(NIVissimConnectionCluster* c2, double offset) {
    //     // join clusters which have at least one connection in common
    //     if (VectorHelper<int>::subSetExists(myConnections, c2->myConnections)) {
    //         return true;
    //     }
    //
    //     // connections shall overlap otherwise
    //     if (!overlapsWith(c2, offset)) {
    //         return false;
    //     }
    //
    //     // at least one of the clusters shall not be assigned to a node in previous (!!!??)
    //     if (hasNodeCluster() && c2->hasNodeCluster()) {
    //         return false;
    //     }
    //
    //     // join clusters which where connections do disturb each other
    //     if (VectorHelper<int>::subSetExists(c2->getDisturbanceParticipators(), myConnections)
    //             ||
    //             VectorHelper<int>::subSetExists(getDisturbanceParticipators(), c2->myConnections)) {
    //
    //         return true;
    //     }
    //
    //
    //     // join clusters which do share the same incoming or outgoing edges (not mutually)
    //     std::vector<int> extendedOutgoing1;
    //     std::vector<int> extendedIncoming1;
    //     std::vector<int> extendedOutgoing2;
    //     std::vector<int> extendedIncoming2;
    //     if (myIncomingEdges.size() > 1 || c2->myIncomingEdges.size() > 1) {
    //         extendedOutgoing1 =
    //             extendByToTreatAsSame(myOutgoingEdges, myIncomingEdges);
    //         extendedIncoming1 =
    //             extendByToTreatAsSame(myIncomingEdges, myOutgoingEdges);
    //         extendedOutgoing2 =
    //             extendByToTreatAsSame(c2->myOutgoingEdges, c2->myIncomingEdges);
    //         extendedIncoming2 =
    //             extendByToTreatAsSame(c2->myIncomingEdges, c2->myOutgoingEdges);
    //     } else {
    //         extendedOutgoing1 = myIncomingEdges;
    //         extendedIncoming1 = myOutgoingEdges;
    //         extendedOutgoing2 = c2->myIncomingEdges;
    //         extendedIncoming2 = c2->myOutgoingEdges;
    //     }
    //
    //     if (VectorHelper<int>::subSetExists(extendedOutgoing1, extendedOutgoing2)
    //             ||
    //             VectorHelper<int>::subSetExists(extendedIncoming1, extendedIncoming2)
    //        ) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn joinable(&mut self, c2: Ptr<NIVissimConnectionCluster>, offset: f64) -> bool {
        // join clusters which have at least one connection in common
        if sub_set_exists_i32(&self.myConnections, &c2.myConnections) {
            return true;
        }

        // connections shall overlap otherwise
        if !self.overlaps_with(c2, offset) {
            return false;
        }

        // at least one of the clusters shall not be assigned to a node in previous (!!!??)
        if self.has_node_cluster() && c2.has_node_cluster() {
            return false;
        }

        // join clusters which where connections do disturb each other
        if sub_set_exists_i32(&c2.m().get_disturbance_participators(), &self.myConnections)
            || sub_set_exists_i32(&self.get_disturbance_participators(), &c2.myConnections)
        {
            return true;
        }

        // join clusters which do share the same incoming or outgoing edges (not mutually)
        let (extended_outgoing1, extended_incoming1, extended_outgoing2, extended_incoming2) =
            if self.myIncomingEdges.len() > 1 || c2.myIncomingEdges.len() > 1 {
                (
                    self.extend_by_to_treat_as_same(&self.myOutgoingEdges, &self.myIncomingEdges),
                    self.extend_by_to_treat_as_same(&self.myIncomingEdges, &self.myOutgoingEdges),
                    c2.extend_by_to_treat_as_same(&c2.myOutgoingEdges, &c2.myIncomingEdges),
                    c2.extend_by_to_treat_as_same(&c2.myIncomingEdges, &c2.myOutgoingEdges),
                )
            } else {
                (
                    self.myIncomingEdges.clone(),
                    self.myOutgoingEdges.clone(),
                    c2.myIncomingEdges.clone(),
                    c2.myOutgoingEdges.clone(),
                )
            };

        if sub_set_exists_i32(&extended_outgoing1, &extended_outgoing2)
            || sub_set_exists_i32(&extended_incoming1, &extended_incoming2)
        {
            return true;
        }
        false
    }

    /// C++: `std::vector<int> NIVissimConnectionCluster::getDisturbanceParticipators()` [private]
    // std::vector<int>
    // NIVissimConnectionCluster::getDisturbanceParticipators() {
    //     std::vector<int> ret;
    //     for (std::vector<int>::iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         const std::vector<int>& disturbances = c->getDisturbances();
    //         for (std::vector<int>::const_iterator j = disturbances.begin(); j != disturbances.end(); j++) {
    //             NIVissimDisturbance* d = NIVissimDisturbance::dictionary(*j);
    //             ret.push_back(d->getEdgeID());
    //             ret.push_back(d->getDisturbanceID());
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_disturbance_participators(&mut self) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        for i in self.myConnections.iter() {
            let c = NIVissimConnection::dictionary_2(*i);
            let disturbances = c.base2.get_disturbances();
            for j in disturbances.iter() {
                let d = NIVissimDisturbance::dictionary_3(*j);
                ret.push(d.get_edge_id());
                ret.push(d.get_disturbance_id());
            }
        }
        ret
    }

    /// C++: `std::vector<int> NIVissimConnectionCluster::extendByToTreatAsSame(const std::vector<int> &, const std::vector<int> &) const` [const, private]
    // std::vector<int>
    // NIVissimConnectionCluster::extendByToTreatAsSame(const std::vector<int>& iv1,
    //         const std::vector<int>& iv2) const {
    //     std::vector<int> ret(iv1);
    //     for (std::vector<int>::const_iterator i = iv1.begin(); i != iv1.end(); i++) {
    //         NIVissimEdge* e = NIVissimEdge::dictionary(*i);
    //         const std::vector<NIVissimEdge*> treatAsSame = e->getToTreatAsSame();
    //         for (std::vector<NIVissimEdge*>::const_iterator j = treatAsSame.begin(); j != treatAsSame.end(); j++) {
    //             if (find(iv2.begin(), iv2.end(), (*j)->getID()) == iv2.end()) {
    //                 ret.push_back((*j)->getID());
    //             }
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn extend_by_to_treat_as_same(&self, iv1: &Vec<i32>, iv2: &Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = iv1.clone();
        for i in iv1.iter() {
            let e = NIVissimEdge::dictionary_3(*i);
            let treat_as_same = e.get_to_treat_as_same();
            for j in treat_as_same.iter() {
                if !iv2.contains(&j.get_id()) {
                    ret.push(j.get_id());
                }
            }
        }
        ret
    }

    /// C++: `bool NIVissimConnectionCluster::isWeakDistrictConnRealisation(NIVissimConnectionCluster *)` [private]
    // bool
    // NIVissimConnectionCluster::isWeakDistrictConnRealisation(NIVissimConnectionCluster* c2) {
    //     if ((myIncomingEdges.size() == 1 && myOutgoingEdges.size() == 1)) {
    //         return false;
    //     }
    //     if ((c2->myIncomingEdges.size() == 1 && c2->myOutgoingEdges.size() == 1)) {
    //         return false;
    //     }
    //
    //     // ok, may be the other way round
    //     if (myIncomingEdges.size() == 1 && c2->myOutgoingEdges.size() == 1) {
    //         return c2->isWeakDistrictConnRealisation(this);
    //     }
    //     // connections must cross
    //     bool crosses = false;
    //     for (std::vector<int>::const_iterator j1 = myConnections.begin(); j1 != myConnections.end() && !crosses; j1++) {
    //         const PositionVector& g1 = NIVissimConnection::dictionary(*j1)->getGeometry();
    //         for (const int j2 : c2->myConnections) {
    //             const PositionVector& g2 = NIVissimConnection::dictionary(j2)->getGeometry();
    //             if (g1.intersects(g2)) {
    //                 crosses = true;
    //                 break;
    //             }
    //         }
    //     }
    //     if (!crosses) {
    //         return false;
    //     }
    //     // ok, check for connection
    //     if (myOutgoingEdges.size() != 1 || c2->myIncomingEdges.size() != 1) {
    //         return false;
    //     }
    //     // check whether the connection is bidirectional
    //     NIVissimEdge* oe = NIVissimEdge::dictionary(myOutgoingEdges[0]);
    //     NIVissimEdge* ie = NIVissimEdge::dictionary(c2->myIncomingEdges[0]);
    //     if (oe == nullptr || ie == nullptr) {
    //         return false;
    //     }
    //     return fabs(GeomHelper::angleDiff(oe->getGeometry().beginEndAngle(), ie->getGeometry().beginEndAngle())) < DEG2RAD(5);
    // }
    #[allow(unused_variables)]
    pub fn is_weak_district_conn_realisation(&mut self, c2: Ptr<NIVissimConnectionCluster>) -> bool {
        if self.myIncomingEdges.len() == 1 && self.myOutgoingEdges.len() == 1 {
            return false;
        }
        if c2.myIncomingEdges.len() == 1 && c2.myOutgoingEdges.len() == 1 {
            return false;
        }

        // ok, may be the other way round
        if self.myIncomingEdges.len() == 1 && c2.myOutgoingEdges.len() == 1 {
            return c2.m().is_weak_district_conn_realisation(Ptr::from_mut(&mut *self));
        }
        // connections must cross
        let mut crosses = false;
        for j1 in self.myConnections.iter() {
            let c1 = NIVissimConnection::dictionary_2(*j1);
            let g1 = c1.base2.get_geometry();
            for j2 in c2.myConnections.iter() {
                let other = NIVissimConnection::dictionary_2(*j2);
                let g2 = other.base2.get_geometry();
                if g1.intersects_pv(g2) {
                    crosses = true;
                    break;
                }
            }
            if crosses {
                break;
            }
        }
        if !crosses {
            return false;
        }
        // ok, check for connection
        if self.myOutgoingEdges.len() != 1 || c2.myIncomingEdges.len() != 1 {
            return false;
        }
        // check whether the connection is bidirectional
        let oe = NIVissimEdge::dictionary_3(self.myOutgoingEdges[0]);
        let ie = NIVissimEdge::dictionary_3(c2.myIncomingEdges[0]);
        if oe.is_null() || ie.is_null() {
            return false;
        }
        geom_helper::angle_diff(oe.get_geometry().begin_end_angle(), ie.get_geometry().begin_end_angle()).abs()
            < geom_helper::deg2rad(5.0)
    }

    /// C++: `bool NIVissimConnectionCluster::liesOnSameEdgesEnd(NIVissimConnectionCluster *)` [private]
    // bool
    // NIVissimConnectionCluster::liesOnSameEdgesEnd(NIVissimConnectionCluster* cc2) {
    //     //
    //     for (std::vector<int>::iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         NIVissimConnection* c1 = NIVissimConnection::dictionary(*i);
    //         for (std::vector<int>::iterator j = cc2->myConnections.begin(); j != cc2->myConnections.end(); j++) {
    //             NIVissimConnection* c2 = NIVissimConnection::dictionary(*j);
    //             if (c1->getFromEdgeID() == c2->getFromEdgeID()) {
    //                 NIVissimEdge* e = NIVissimEdge::dictionary(c1->getFromEdgeID());
    //                 const PositionVector& g = e->getGeometry();
    //                 double pos1 = GeomHelper::nearest_offset_on_line_to_point2D(
    //                                   g.front(), g.back(), c1->getBoundary().getCenter());
    //                 double pos2 = GeomHelper::nearest_offset_on_line_to_point2D(
    //                                   g.front(), g.back(), c2->getBoundary().getCenter());
    //                 if (pos1 <= 5.0 && pos2 <= 5.0) {
    //                     return true;
    //                 }
    //             }
    //             if (c1->getToEdgeID() == c2->getToEdgeID()) {
    //                 NIVissimEdge* e = NIVissimEdge::dictionary(c1->getFromEdgeID());
    //                 const PositionVector& g = e->getGeometry();
    //                 double pos1 = GeomHelper::nearest_offset_on_line_to_point2D(
    //                                   g.front(), g.back(), c1->getBoundary().getCenter());
    //                 double pos2 = GeomHelper::nearest_offset_on_line_to_point2D(
    //                                   g.front(), g.back(), c2->getBoundary().getCenter());
    //                 if (pos1 >= g.length() - 5.0 && pos2 >= g.length() - 5.0) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn lies_on_same_edges_end(&mut self, cc2: Ptr<NIVissimConnectionCluster>) -> bool {
        //
        for i in self.myConnections.iter() {
            let c1 = NIVissimConnection::dictionary_2(*i);
            for j in cc2.myConnections.iter() {
                let c2 = NIVissimConnection::dictionary_2(*j);
                if c1.get_from_edge_id() == c2.get_from_edge_id() {
                    let e = NIVissimEdge::dictionary_3(c1.get_from_edge_id());
                    let g = e.get_geometry();
                    let pos1 = geom_helper::nearest_offset_on_line_to_point2d(
                        g.front(), g.back(), &c1.get_boundary().get_center(), true);
                    let pos2 = geom_helper::nearest_offset_on_line_to_point2d(
                        g.front(), g.back(), &c2.get_boundary().get_center(), true);
                    if pos1 <= 5.0 && pos2 <= 5.0 {
                        return true;
                    }
                }
                if c1.get_to_edge_id() == c2.get_to_edge_id() {
                    let e = NIVissimEdge::dictionary_3(c1.get_from_edge_id());
                    let g = e.get_geometry();
                    let pos1 = geom_helper::nearest_offset_on_line_to_point2d(
                        g.front(), g.back(), &c1.get_boundary().get_center(), true);
                    let pos2 = geom_helper::nearest_offset_on_line_to_point2d(
                        g.front(), g.back(), &c2.get_boundary().get_center(), true);
                    if pos1 >= g.length() - 5.0 && pos2 >= g.length() - 5.0 {
                        return true;
                    }
                }
            }
        }
        false
    }

}

/// C++ class `NIVissimConnectionCluster::NodeSubCluster` (netimport/vissim/tempstructs/NIVissimConnectionCluster.h)
pub struct NIVissimConnectionCluster_NodeSubCluster {
    pub myBoundary: Boundary,
    pub myConnections: NodeSubCluster_ConnectionCont,
}

impl NIVissimConnectionCluster_NodeSubCluster {
    /// C++: `void NodeSubCluster::NodeSubCluster(NIVissimConnection *)`
    // NIVissimConnectionCluster::NodeSubCluster::NodeSubCluster(NIVissimConnection* c) {
    //     add(c);
    // }
    #[allow(unused_variables)]
    pub fn new(c: Ptr<NIVissimConnection>) -> Self {
        let mut s = Self {
            myBoundary: Boundary::default(),
            myConnections: Vec::new(),
        };
        s.add(c);
        s
    }

    /// C++: `void NodeSubCluster::~NodeSubCluster()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the Boundary and the vector of raw pointers need no release
    }

    /// C++: `void NodeSubCluster::add(NIVissimConnection *)`
    // void
    // NIVissimConnectionCluster::NodeSubCluster::add(NIVissimConnection* c) {
    //     myBoundary.add(c->getBoundingBox());
    //     myConnections.push_back(c);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, c: Ptr<NIVissimConnection>) {
        self.myBoundary.add_boundary(c.get_bounding_box());
        self.myConnections.push(c);
    }

    /// C++: `void NodeSubCluster::add(const NodeSubCluster &)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_2(&mut self, c: &NIVissimConnectionCluster_NodeSubCluster) {
        for i in c.myConnections.iter() {
            self.add(*i);
        }
    }

    /// C++: `int NodeSubCluster::size() const` [const]
    // int
    // NIVissimConnectionCluster::NodeSubCluster::size() const {
    //     return (int)myConnections.size();
    // }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myConnections.len() as i32
    }

    /// C++: `bool NodeSubCluster::overlapsWith(const NodeSubCluster &, double)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn overlaps_with(&mut self, c: &NIVissimConnectionCluster_NodeSubCluster, offset: f64) -> bool {
        debug_assert!(self.myBoundary.xmax() >= self.myBoundary.xmin());
        debug_assert!(c.myBoundary.xmax() >= c.myBoundary.xmin());
        self.myBoundary.overlaps_with(&c.myBoundary, offset)
    }

    /// C++: `std::vector<int> NodeSubCluster::getConnectionIDs() const` [const]
    // std::vector<int>
    // NIVissimConnectionCluster::NodeSubCluster::getConnectionIDs() const {
    //     std::vector<int> ret;
    //     int id = NIVissimConnectionCluster::getNextFreeNodeID();
    //     for (ConnectionCont::const_iterator i = myConnections.begin(); i != myConnections.end(); i++) {
    //         ret.push_back((*i)->getID());
    //         (*i)->setNodeCluster(id);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_connection_i_ds(&self) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let id = NIVissimConnectionCluster::get_next_free_node_id();
        for c in self.myConnections.iter() {
            ret.push(c.base2.get_id());
            c.m().set_node_cluster(id);
        }
        ret
    }

}

/// C++ class `NIVissimConnectionCluster::same_direction_sorter` (netimport/vissim/tempstructs/NIVissimConnectionCluster.h)
pub struct NIVissimConnectionCluster_same_direction_sorter {
    pub myAngle: f64,
}

impl NIVissimConnectionCluster_same_direction_sorter {
    /// C++: `void same_direction_sorter::same_direction_sorter(double)`
    /// constructor
    // explicit same_direction_sorter(double angle)
    //             : myAngle(angle) { }
    #[allow(unused_variables)]
    pub fn new(angle: f64) -> Self {
        Self { myAngle: angle }
    }

    /// C++: `int same_direction_sorter::operator()(NIVissimConnection *, NIVissimConnection *) const` [const]
    /// comparing operation
    // int operator()(NIVissimConnection* c1, NIVissimConnection* c2) const {
    //             return fabs(GeomHelper::angleDiff(c1->getGeometry().beginEndAngle(), myAngle))
    //                    <
    //                    fabs(GeomHelper::angleDiff(c2->getGeometry().beginEndAngle(), myAngle));
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c1: Ptr<NIVissimConnection>, c2: Ptr<NIVissimConnection>) -> i32 {
        if geom_helper::angle_diff(c1.base2.get_geometry().begin_end_angle(), self.myAngle).abs()
            < geom_helper::angle_diff(c2.base2.get_geometry().begin_end_angle(), self.myAngle).abs()
        {
            1
        } else {
            0
        }
    }

}

