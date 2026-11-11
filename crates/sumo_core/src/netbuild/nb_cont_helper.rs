//! Port of netbuild/NBContHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// NBContHelper Some static helper methods that traverse a sorted list of edges in both directions
/// C++ class `NBContHelper` (netbuild/NBContHelper.h)
pub struct NBContHelper {
}

impl NBContHelper {
    /// C++: `void NBContHelper::nextCW(const EdgeVector &, EdgeVector::const_iterator &)` [static]
    /// Moves the given iterator clockwise within the given container of edges sorted clockwise
    // void
    // NBContHelper::nextCW(const EdgeVector& edges, EdgeVector::const_iterator& from) {
    //     from++;
    //     if (from == edges.end()) {
    //         from = edges.begin();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn next_cw(edges: &EdgeVector, from: &mut Unmapped /* EdgeVector::const_iterator */) {
        todo!("port NBContHelper::nextCW")
    }

    /// C++: `void NBContHelper::nextCCW(const EdgeVector &, EdgeVector::const_iterator &)` [static]
    /// Moves the given iterator counter clockwise within the given container of edges sorted clockwise
    // void
    // NBContHelper::nextCCW(const EdgeVector& edges, EdgeVector::const_iterator& from) {
    //     if (from == edges.begin()) {
    //         from = edges.end() - 1;
    //     } else {
    //         --from;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn next_ccw(edges: &EdgeVector, from: &mut Unmapped /* EdgeVector::const_iterator */) {
        todo!("port NBContHelper::nextCCW")
    }

    /// C++: `double NBContHelper::getMaxSpeed(const EdgeVector &)` [static]
    // double
    // NBContHelper::getMaxSpeed(const EdgeVector& edges) {
    //     if (edges.size() == 0) {
    //         return -1;
    //     }
    //     double ret = (*(edges.begin()))->getSpeed();
    //     for (EdgeVector::const_iterator i = edges.begin() + 1; i != edges.end(); i++) {
    //         if ((*i)->getSpeed() > ret) {
    //             ret = (*i)->getSpeed();
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(edges: &EdgeVector) -> f64 {
        todo!("port NBContHelper::getMaxSpeed")
    }

    /// C++: `double NBContHelper::getMinSpeed(const EdgeVector &)` [static]
    // double
    // NBContHelper::getMinSpeed(const EdgeVector& edges) {
    //     if (edges.size() == 0) {
    //         return -1;
    //     }
    //     double ret = (*(edges.begin()))->getSpeed();
    //     for (EdgeVector::const_iterator i = edges.begin() + 1; i != edges.end(); i++) {
    //         if ((*i)->getSpeed() < ret) {
    //             ret = (*i)->getSpeed();
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_min_speed(edges: &EdgeVector) -> f64 {
        todo!("port NBContHelper::getMinSpeed")
    }

    /// C++: `std::ostream & NBContHelper::out(std::ostream &, const std::vector<bool> &)` [static]
    /// writes the vector of bools to the given stream
    // std::ostream&
    // NBContHelper::out(std::ostream& os, const std::vector<bool>& v) {
    //     for (std::vector<bool>::const_iterator i = v.begin(); i != v.end(); i++) {
    //         os << *i;
    //     }
    //     return os;
    // }
    #[allow(unused_variables)]
    pub fn out(os: &mut Unmapped /* std::ostream */, v: &Vec<bool>) -> &'static mut Unmapped /* std::ostream */ {
        todo!("port NBContHelper::out")
    }

    /// C++: `NBEdge * NBContHelper::findConnectingEdge(const EdgeVector &, NBNode *, NBNode *)` [static]
    /// Tries to return the first edge within the given container which connects both given nodes
    // NBEdge*
    // NBContHelper::findConnectingEdge(const EdgeVector& edges,
    //                                  NBNode* from, NBNode* to) {
    //     for (EdgeVector::const_iterator i = edges.begin(); i != edges.end(); i++) {
    //         if ((*i)->getToNode() == to && (*i)->getFromNode() == from) {
    //             return *i;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn find_connecting_edge(edges: &EdgeVector, from: Ptr<NBNode>, to: Ptr<NBNode>) -> Ptr<NBEdge> {
        todo!("port NBContHelper::findConnectingEdge")
    }

    /// C++: `double NBContHelper::maxSpeed(const EdgeVector &)` [static]
    /// returns the maximum speed allowed on the edges
    // double
    // NBContHelper::maxSpeed(const EdgeVector& ev) {
    //     assert(ev.size() > 0);
    //     double max = (*(ev.begin()))->getSpeed();
    //     for (EdgeVector::const_iterator i = ev.begin() + 1; i != ev.end(); i++) {
    //         max =
    //             max > (*i)->getSpeed()
    //             ? max : (*i)->getSpeed();
    //     }
    //     return max;
    // }
    #[allow(unused_variables)]
    pub fn max_speed(ev: &EdgeVector) -> f64 {
        todo!("port NBContHelper::maxSpeed")
    }

}

/// edge_by_angle_to_nodeShapeCentroid_sorter Class to sort edges by their angle in relation to the node shape
/// C++ class `NBContHelper::edge_by_angle_to_nodeShapeCentroid_sorter` (netbuild/NBContHelper.h)
pub struct NBContHelper_edge_by_angle_to_nodeShapeCentroid_sorter {
    /// the edge to compute the relative angle of
    pub myNode: Ptr<NBNode>,
}

impl NBContHelper_edge_by_angle_to_nodeShapeCentroid_sorter {
    /// C++: `void edge_by_angle_to_nodeShapeCentroid_sorter::edge_by_angle_to_nodeShapeCentroid_sorter(const NBNode *)`
    /// constructor
    // explicit edge_by_angle_to_nodeShapeCentroid_sorter(const NBNode* n) : myNode(n) {}
    #[allow(unused_variables)]
    pub fn new(n: Ptr<NBNode>) -> Self {
        todo!("port edge_by_angle_to_nodeShapeCentroid_sorter::edge_by_angle_to_nodeShapeCentroid_sorter")
    }

    /// C++: `bool edge_by_angle_to_nodeShapeCentroid_sorter::operator()(const NBEdge *, const NBEdge *) const` [const]
    /// comparing operation
    // bool
    // NBContHelper::edge_by_angle_to_nodeShapeCentroid_sorter::operator()(const NBEdge* e1, const NBEdge* e2) const {
    //     assert(e1->getFromNode() == myNode || e1->getToNode() == myNode);
    //     assert(e2->getFromNode() == myNode || e2->getToNode() == myNode);
    //     const double angle1 = e1->getAngleAtNodeToCenter(myNode);
    //     const double angle2 = e2->getAngleAtNodeToCenter(myNode);
    //     const double absDiff = fabs(angle1 - angle2);
    //
    //     // cannot trust the angle difference hence a heuristic:
    //     if (absDiff < 2 || absDiff > (360 - 2)) {
    //         const bool sameDir = ((e1->getFromNode() == myNode && e2->getFromNode() == myNode)
    //                               || (e1->getToNode() == myNode && e2->getToNode() == myNode));
    //         if (sameDir) {
    //             // put edges that allow pedestrians on the 'outside', but be aware if both allow / disallow
    //             const bool e1Peds = (e1->getPermissions() & SVC_PEDESTRIAN) != 0;
    //             const bool e2Peds = (e2->getPermissions() & SVC_PEDESTRIAN) != 0;
    //             if (e1->getToNode() == myNode) {
    //                 if (e1Peds && !e2Peds) {
    //                     return true;
    //                 } else if (!e1Peds && e2Peds) {
    //                     return false;
    //                 }
    //             } else {
    //                 if (!e1Peds && e2Peds) {
    //                     return true;
    //                 } else if (e1Peds && !e2Peds) {
    //                     return false;
    //                 }
    //             }
    //             // break ties to ensure strictly weak ordering
    //             return e1->getID() < e2->getID();
    //         } else {
    //             // sort incoming before outgoing, no need to break ties here
    //             return e1->getToNode() == myNode;
    //         }
    //     }
    //     return angle1 < angle2;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> bool {
        todo!("port edge_by_angle_to_nodeShapeCentroid_sorter::operator()")
    }

}

/// edge_by_priority_sorter Class to sort edges by their priority
/// C++ class `NBContHelper::edge_by_priority_sorter` (netbuild/NBContHelper.h)
pub struct NBContHelper_edge_by_priority_sorter {
    pub myPermissions: SVCPermissions,
}

impl NBContHelper_edge_by_priority_sorter {
    /// C++: `void edge_by_priority_sorter::edge_by_priority_sorter(SVCPermissions)`
    // edge_by_priority_sorter(SVCPermissions permissions) : myPermissions(permissions) {}
    #[allow(unused_variables)]
    pub fn new(permissions: SVCPermissions) -> Self {
        todo!("port edge_by_priority_sorter::edge_by_priority_sorter")
    }

    /// C++: `int edge_by_priority_sorter::operator()(NBEdge *, NBEdge *) const` [const]
    /// comparing operator
    // int operator()(NBEdge* e1, NBEdge* e2) const {
    //             if (e1->getPriority() != e2->getPriority()) {
    //                 return e1->getPriority() > e2->getPriority();
    //             }
    //             if (e1->getSpeed() != e2->getSpeed()) {
    //                 return e1->getSpeed() > e2->getSpeed();
    //             }
    //             return e1->getNumLanesThatAllow(myPermissions, false) > e2->getNumLanesThatAllow(myPermissions, false);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> i32 {
        todo!("port edge_by_priority_sorter::operator()")
    }

}

/// Class to sort edges by their angle in relation to the given edge
/// C++ class `NBContHelper::edge_opposite_direction_sorter` (netbuild/NBContHelper.h)
pub struct NBContHelper_edge_opposite_direction_sorter {
    /// The related node
    pub myNode: Ptr<NBNode>,
    /// the reference edge
    pub myEdge: Ptr<NBEdge>,
    /// The angle of the related edge at the given node
    pub myAngle: f64,
    /// Whether edge priority may override closer angles
    pub myRegardPriority: bool,
}

impl NBContHelper_edge_opposite_direction_sorter {
    /// C++: `void edge_opposite_direction_sorter::edge_opposite_direction_sorter(const NBEdge *const, const NBNode *const, bool)`
    /// Constructor
    // explicit edge_opposite_direction_sorter(const NBEdge* const e, const NBNode* const n, bool regardPriority) :
    //             myNode(n),
    //             myEdge(e),
    //             myRegardPriority(regardPriority) {
    //             myAngle = getEdgeAngleAt(e, n);
    //         }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>, n: Ptr<NBNode>, regardPriority: bool) -> Self {
        todo!("port edge_opposite_direction_sorter::edge_opposite_direction_sorter")
    }

    /// C++: `int edge_opposite_direction_sorter::operator()(NBEdge *, NBEdge *) const` [const]
    /// Comparing operation
    // int operator()(NBEdge* e1, NBEdge* e2) const {
    //             if (!myRegardPriority || e1->getPriority() == e2->getPriority() || e1 == myEdge || e2 == myEdge) {
    //                 return getDiff(e1) > getDiff(e2);
    //             } else {
    //                 return e1->getPriority() > e2->getPriority();
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> i32 {
        todo!("port edge_opposite_direction_sorter::operator()")
    }

    /// C++: `double edge_opposite_direction_sorter::getDiff(const NBEdge *const) const` [const, protected]
    /// Computes the angle difference between the related and the given edge
    // double getDiff(const NBEdge* const e) const {
    //             return fabs(GeomHelper::angleDiff(getEdgeAngleAt(e, myNode), myAngle));
    //         }
    #[allow(unused_variables)]
    pub fn get_diff(&self, e: Ptr<NBEdge>) -> f64 {
        todo!("port edge_opposite_direction_sorter::getDiff")
    }

    /// C++: `double edge_opposite_direction_sorter::getEdgeAngleAt(const NBEdge *const, const NBNode *const) const` [const, protected]
    /// Returns the given edge's angle at the given node
    // double getEdgeAngleAt(const NBEdge* const e, const NBNode* const n) const {
    //             if (e->getFromNode() == n) {
    //                 return e->getGeometry().angleAt2D(0);
    //             } else {
    //                 return e->getGeometry()[-1].angleTo2D(e->getGeometry()[-2]);
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn get_edge_angle_at(&self, e: Ptr<NBEdge>, n: Ptr<NBNode>) -> f64 {
        todo!("port edge_opposite_direction_sorter::getEdgeAngleAt")
    }

}

/// edge_similar_direction_sorter Class to sort edges by their angle in relation to the given edge The resulting list should have the edge in the most similar direction to the given edge as its first entry
/// C++ class `NBContHelper::edge_similar_direction_sorter` (netbuild/NBContHelper.h)
pub struct NBContHelper_edge_similar_direction_sorter {
    /// the angle to find the edge with the opposite direction
    pub myCompareOutgoing: bool,
    pub myAngle: f64,
}

impl NBContHelper_edge_similar_direction_sorter {
    /// C++: `void edge_similar_direction_sorter::edge_similar_direction_sorter(const NBEdge *const, bool)`
    /// constructor
    // explicit edge_similar_direction_sorter(const NBEdge* const e, bool outgoing = true) :
    //             myCompareOutgoing(outgoing),
    //             myAngle(outgoing ? e->getShapeEndAngle() : e->getShapeStartAngle())
    //         {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>, outgoing: bool) -> Self {
        todo!("port edge_similar_direction_sorter::edge_similar_direction_sorter")
    }

    /// C++: `int edge_similar_direction_sorter::operator()(const NBEdge *, const NBEdge *) const` [const]
    /// comparing operation
    // int operator()(const NBEdge* e1, const NBEdge* e2) const {
    //             const double d1 = angleDiff(myCompareOutgoing ? e1->getShapeStartAngle() : e1->getShapeEndAngle(), myAngle);
    //             const double d2 = angleDiff(myCompareOutgoing ? e2->getShapeStartAngle() : e2->getShapeEndAngle(), myAngle);
    //             if (fabs(fabs(d1) - fabs(d2)) < NUMERICAL_EPS) {
    //                 if (fabs(d1 - d2) > NUMERICAL_EPS) {
    //                     return d1 < d2;
    //                 } else {
    //                     return e1->getNumericalID() < e2->getNumericalID();
    //                 }
    //             }
    //             return fabs(d1) < fabs(d2);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> i32 {
        todo!("port edge_similar_direction_sorter::operator()")
    }

    /// C++: `double edge_similar_direction_sorter::angleDiff(const double, const double) const` [const, private]
    // double angleDiff(const double angle1, const double angle2) const {
    //             double d = angle2 - angle1;
    //             while (d >= 180.) {
    //                 d -= 360.;
    //             }
    //             while (d < -180.) {
    //                 d += 360.;
    //             }
    //             return d;
    //         }
    #[allow(unused_variables)]
    pub fn angle_diff(&self, angle1: f64, angle2: f64) -> f64 {
        todo!("port edge_similar_direction_sorter::angleDiff")
    }

}

/// C++ class `NBContHelper::edge_with_destination_finder` (netbuild/NBContHelper.h)
pub struct NBContHelper_edge_with_destination_finder {
    pub myDestinationNode: Ptr<NBNode>,
}

impl NBContHelper_edge_with_destination_finder {
    /// C++: `void edge_with_destination_finder::edge_with_destination_finder(NBNode *)`
    /// constructor
    // NBContHelper::edge_with_destination_finder::edge_with_destination_finder(NBNode* dest)
    //     : myDestinationNode(dest) {}
    #[allow(unused_variables)]
    pub fn new(dest: Ptr<NBNode>) -> Self {
        todo!("port edge_with_destination_finder::edge_with_destination_finder")
    }

    /// C++: `bool edge_with_destination_finder::operator()(NBEdge *) const` [const]
    // bool
    // NBContHelper::edge_with_destination_finder::operator()(NBEdge* e) const {
    //     return e->getToNode() == myDestinationNode;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, e: Ptr<NBEdge>) -> bool {
        todo!("port edge_with_destination_finder::operator()")
    }

    /// C++: `edge_with_destination_finder & edge_with_destination_finder::operator=(const edge_with_destination_finder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBContHelper_edge_with_destination_finder) -> &mut NBContHelper_edge_with_destination_finder {
        todo!("port edge_with_destination_finder::operator=")
    }

}

/// C++ class `NBContHelper::node_with_incoming_finder` (netbuild/NBContHelper.h)
pub struct NBContHelper_node_with_incoming_finder {
    pub myEdge: Ptr<NBEdge>,
}

impl NBContHelper_node_with_incoming_finder {
    /// C++: `void node_with_incoming_finder::node_with_incoming_finder(const NBEdge *const)`
    /// constructor
    // NBContHelper::node_with_incoming_finder::node_with_incoming_finder(const NBEdge* const e)
    //     : myEdge(e) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>) -> Self {
        todo!("port node_with_incoming_finder::node_with_incoming_finder")
    }

    /// C++: `bool node_with_incoming_finder::operator()(const NBNode *const) const` [const]
    // bool
    // NBContHelper::node_with_incoming_finder::operator()(const NBNode* const n) const {
    //     const EdgeVector& incoming = n->getIncomingEdges();
    //     return std::find(incoming.begin(), incoming.end(), myEdge) != incoming.end();
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, n: Ptr<NBNode>) -> bool {
        todo!("port node_with_incoming_finder::operator()")
    }

}

/// C++ class `NBContHelper::node_with_outgoing_finder` (netbuild/NBContHelper.h)
pub struct NBContHelper_node_with_outgoing_finder {
    pub myEdge: Ptr<NBEdge>,
}

impl NBContHelper_node_with_outgoing_finder {
    /// C++: `void node_with_outgoing_finder::node_with_outgoing_finder(const NBEdge *const)`
    /// constructor
    // NBContHelper::node_with_outgoing_finder::node_with_outgoing_finder(const NBEdge* const e)
    //     : myEdge(e) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>) -> Self {
        todo!("port node_with_outgoing_finder::node_with_outgoing_finder")
    }

    /// C++: `bool node_with_outgoing_finder::operator()(const NBNode *const) const` [const]
    // bool
    // NBContHelper::node_with_outgoing_finder::operator()(const NBNode* const n) const {
    //     const EdgeVector& outgoing = n->getOutgoingEdges();
    //     return std::find(outgoing.begin(), outgoing.end(), myEdge) != outgoing.end();
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, n: Ptr<NBNode>) -> bool {
        todo!("port node_with_outgoing_finder::operator()")
    }

}

/// C++ class `NBContHelper::opposite_finder` (netbuild/NBContHelper.h)
pub struct NBContHelper_opposite_finder {
    pub myReferenceEdge: Ptr<NBEdge>,
}

impl NBContHelper_opposite_finder {
    /// C++: `void opposite_finder::opposite_finder(NBEdge *)`
    /// constructor
    // opposite_finder(NBEdge* edge)
    //             : myReferenceEdge(edge) { }
    #[allow(unused_variables)]
    pub fn new(edge: Ptr<NBEdge>) -> Self {
        todo!("port opposite_finder::opposite_finder")
    }

    /// C++: `bool opposite_finder::operator()(NBEdge *) const` [const]
    // bool operator()(NBEdge* e) const {
    //             return e->isTurningDirectionAt(myReferenceEdge) ||
    //                    myReferenceEdge->isTurningDirectionAt(e);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e: Ptr<NBEdge>) -> bool {
        todo!("port opposite_finder::operator()")
    }

}

/// relative_incoming_edge_sorter Class to sort edges by their angle in relation to an outgoing edge. This is normally done to sort edges incoming at the starting node of this edge by their angle in relation to the using edge's angle (this angle is the reference angle).
/// C++ class `NBContHelper::relative_incoming_edge_sorter` (netbuild/NBContHelper.h)
pub struct NBContHelper_relative_incoming_edge_sorter {
    /// the reference angle to compare edges agains
    pub myAngle: f64,
}

impl NBContHelper_relative_incoming_edge_sorter {
    /// C++: `void relative_incoming_edge_sorter::relative_incoming_edge_sorter(NBEdge *)`
    /// constructor
    // explicit relative_incoming_edge_sorter(NBEdge* e) : myAngle(e->getStartAngle()) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>) -> Self {
        todo!("port relative_incoming_edge_sorter::relative_incoming_edge_sorter")
    }

    /// C++: `void relative_incoming_edge_sorter::relative_incoming_edge_sorter(double)`
    /// constructor
    // explicit relative_incoming_edge_sorter(double angle) : myAngle(angle) {}
    #[allow(unused_variables)]
    pub fn new_2(angle: f64) -> Self {
        todo!("port relative_incoming_edge_sorter::relative_incoming_edge_sorter")
    }

    /// C++: `bool relative_incoming_edge_sorter::operator()(const NBEdge *, const NBEdge *) const` [const]
    /// comparing operation
    // bool
    // NBContHelper::relative_incoming_edge_sorter::operator()(const NBEdge* e1, const NBEdge* e2) const {
    //     assert(e1 != nullptr && e2 != nullptr);
    //     double relAngle1 = NBHelpers::normRelAngle(myAngle, e1->getEndAngle());
    //     double relAngle2 = NBHelpers::normRelAngle(myAngle, e2->getEndAngle());
    //     const double length1 = e1->getGeometry().length2D();
    //     const double length2 = e2->getGeometry().length2D();
    //
    //     double lookAhead = 2 * NBEdge::ANGLE_LOOKAHEAD;
    //     while (fabs(relAngle1 - relAngle2) < 3.0) {
    //         // look at further geometry segments to resolve ambiguity
    //         const double offset1 = MAX2(0.0, MIN2(length1, length1 - lookAhead));
    //         const double offset2 = MAX2(0.0, MIN2(length2, length2 - lookAhead));
    //         const Position referencePos1 = e1->getGeometry().positionAtOffset2D(offset1);
    //         const Position referencePos2 = e2->getGeometry().positionAtOffset2D(offset2);
    //         const double angle1 = GeomHelper::legacyDegree(referencePos1.angleTo2D(e1->getToNode()->getPosition()), true);
    //         const double angle2 = GeomHelper::legacyDegree(referencePos2.angleTo2D(e2->getToNode()->getPosition()), true);
    //         relAngle1 = NBHelpers::normRelAngle(myAngle, angle1);
    //         relAngle2 = NBHelpers::normRelAngle(myAngle, angle2);
    //         if (lookAhead > MAX2(length1, length2)) {
    //             break;
    //         }
    //         lookAhead *= 2;
    //     }
    //     if (fabs(relAngle1 - relAngle2) < NUMERICAL_EPS) {
    //         // need to break ties for windows debug version, numerical id may be -1 for both
    //         return e1->getID() > e2->getID();
    //     }
    //     return relAngle1 > relAngle2;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> bool {
        todo!("port relative_incoming_edge_sorter::operator()")
    }

}

/// relative_outgoing_edge_sorter Class to sort edges by their angle in relation to the node the edge using this class is incoming into. This is normally done to sort edges outgoing from the node the using edge is incoming in by their angle in relation to the using edge's angle (this angle is the reference angle).
/// C++ class `NBContHelper::relative_outgoing_edge_sorter` (netbuild/NBContHelper.h)
pub struct NBContHelper_relative_outgoing_edge_sorter {
    /// the reference angle to compare edges agains
    pub myAngle: f64,
}

impl NBContHelper_relative_outgoing_edge_sorter {
    /// C++: `void relative_outgoing_edge_sorter::relative_outgoing_edge_sorter(NBEdge *)`
    /// constructor
    // explicit relative_outgoing_edge_sorter(NBEdge* e) : myAngle(e->getEndAngle()) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>) -> Self {
        todo!("port relative_outgoing_edge_sorter::relative_outgoing_edge_sorter")
    }

    /// C++: `void relative_outgoing_edge_sorter::relative_outgoing_edge_sorter(double)`
    /// constructor
    // explicit relative_outgoing_edge_sorter(double angle) : myAngle(angle) {}
    #[allow(unused_variables)]
    pub fn new_2(angle: f64) -> Self {
        todo!("port relative_outgoing_edge_sorter::relative_outgoing_edge_sorter")
    }

    /// C++: `bool relative_outgoing_edge_sorter::operator()(const NBEdge *, const NBEdge *) const` [const]
    /// comparing operation
    // bool
    // NBContHelper::relative_outgoing_edge_sorter::operator()(const NBEdge* e1, const NBEdge* e2) const {
    //     assert(e1 != nullptr && e2 != nullptr);
    //     double relAngle1 = NBHelpers::normRelAngle(myAngle, e1->getStartAngle());
    //     double relAngle2 = NBHelpers::normRelAngle(myAngle, e2->getStartAngle());
    //     const double length1 = e1->getGeometry().length2D();
    //     const double length2 = e2->getGeometry().length2D();
    //
    //     double lookAhead = 2 * NBEdge::ANGLE_LOOKAHEAD;
    //     while (fabs(relAngle1 - relAngle2) < 3.0) {
    //         // look at further geometry segments to resolve ambiguity
    //         const double offset1 = MAX2(0.0, MIN2(length1, lookAhead));
    //         const double offset2 = MAX2(0.0, MIN2(length2, lookAhead));
    //         const Position referencePos1 = e1->getGeometry().positionAtOffset2D(offset1);
    //         const Position referencePos2 = e2->getGeometry().positionAtOffset2D(offset2);
    //         const double angle1 = GeomHelper::legacyDegree(e1->getFromNode()->getPosition().angleTo2D(referencePos1), true);
    //         const double angle2 = GeomHelper::legacyDegree(e2->getFromNode()->getPosition().angleTo2D(referencePos2), true);
    //         relAngle1 = NBHelpers::normRelAngle(myAngle, angle1);
    //         relAngle2 = NBHelpers::normRelAngle(myAngle, angle2);
    //         if (lookAhead > MAX2(length1, length2)) {
    //             break;
    //         }
    //         lookAhead *= 2;
    //     }
    //     if (fabs(relAngle1 - relAngle2) < NUMERICAL_EPS) {
    //         // need to break ties for windows debug version, numerical id may be -1 for both
    //         return e1->getID() > e2->getID();
    //     }
    //     return relAngle1 > relAngle2;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> bool {
        todo!("port relative_outgoing_edge_sorter::operator()")
    }

}

/// same_connection_edge_sorter This class is used to sort edges which connect the same nodes. The edges are sorted in dependence to edges connecting them. The rightmost will be the first in the list; the leftmost the last one.
/// C++ class `NBContHelper::same_connection_edge_sorter` (netbuild/NBContHelper.h)
pub struct NBContHelper_same_connection_edge_sorter {
}

impl NBContHelper_same_connection_edge_sorter {
    /// C++: `void same_connection_edge_sorter::same_connection_edge_sorter()`
    /// constructor
    // explicit same_connection_edge_sorter() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port same_connection_edge_sorter::same_connection_edge_sorter")
    }

    /// C++: `int same_connection_edge_sorter::operator()(NBEdge *, NBEdge *) const` [const]
    /// comparing operation
    // int operator()(NBEdge* e1, NBEdge* e2) const {
    //             std::pair<double, double> mm1 = getMinMaxRelAngles(e1);
    //             std::pair<double, double> mm2 = getMinMaxRelAngles(e2);
    //             if (mm1.first == mm2.first && mm1.second == mm2.second) {
    //                 // ok, let's simply sort them arbitrarily
    //                 return e1->getID() < e2->getID();
    //             }
    //
    //             assert(
    //                 (mm1.first <= mm2.first && mm1.second <= mm2.second)
    //                 ||
    //                 (mm1.first >= mm2.first && mm1.second >= mm2.second));
    //             return (mm1.first >= mm2.first && mm1.second >= mm2.second);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> i32 {
        todo!("port same_connection_edge_sorter::operator()")
    }

    /// C++: `std::pair<double, double> same_connection_edge_sorter::getMinMaxRelAngles(NBEdge *) const` [const]
    // std::pair<double, double> getMinMaxRelAngles(NBEdge* e) const {
    //             double min = 360;
    //             double max = 360;
    //             const EdgeVector& ev = e->getConnectedEdges();
    //             for (EdgeVector::const_iterator i = ev.begin(); i != ev.end(); ++i) {
    //                 double angle = NBHelpers::normRelAngle(
    //                                    e->getTotalAngle(), (*i)->getTotalAngle());
    //                 if (min == 360 || min > angle) {
    //                     min = angle;
    //                 }
    //                 if (max == 360 || max < angle) {
    //                     max = angle;
    //                 }
    //             }
    //             return std::pair<double, double>(min, max);
    //         }
    #[allow(unused_variables)]
    pub fn get_min_max_rel_angles(&self, e: Ptr<NBEdge>) -> (f64, f64) {
        todo!("port same_connection_edge_sorter::getMinMaxRelAngles")
    }

}

/// C++: `std::ostream &(std::ostream &, const EdgeVector &) operator<<`
// std::ostream&
// operator<<(std::ostream& os, const EdgeVector& ev) {
//     for (EdgeVector::const_iterator i = ev.begin(); i != ev.end(); i++) {
//         if (i != ev.begin()) {
//             os << ", ";
//         }
//         os << (*i)->getID();
//     }
//     return os;
// }
#[allow(unused_variables)]
pub fn shl_op(os: &mut Unmapped /* std::ostream */, ev: &EdgeVector) -> &'static mut Unmapped /* std::ostream */ {
    todo!("port operator<<")
}

