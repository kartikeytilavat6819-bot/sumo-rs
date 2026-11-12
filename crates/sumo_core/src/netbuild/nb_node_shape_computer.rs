//! Port of netbuild/NBNodeShapeComputer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::geom::{NUMERICAL_EPS, POSITION_EPS};
use utils::common::std_defs::SUMO_CONST_LANE_WIDTH;
use utils::options::options_cont::{exists, get_bool, get_float, get_int, is_set};

/// C++ typedef `std::map<NBEdge *, PositionVector> NBNodeShapeComputer::GeomsMap`
pub type NBNodeShapeComputer_GeomsMap = BTreeMap<Ptr<NBEdge>, PositionVector>;

/// This class computes shapes of junctions
/// C++ class `NBNodeShapeComputer` (netbuild/NBNodeShapeComputer.h)
pub struct NBNodeShapeComputer {
    /// The node to compute the geometry for
    pub myNode: Ptr<NBNode>,
    /// the computed node radius
    pub myRadius: f64,
    /// the maximum distance to search for a place where neighboring edges intersect and do not overlap
    pub EXT: f64,
}

// C++ static members of NBNodeShapeComputer (port as module-level statics, see AGENTS.md):
//   static const SVCPermissions NBNodeShapeComputer::SVC_LARGE_TURN;

/// C++ `#define EXT2 10.0` (netbuild/NBNodeShapeComputer.cpp)
const EXT2: f64 = 10.0;

/// C++ `NBNode::AVOID_WIDE_LEFT_TURN` (netbuild/NBNode.cpp), used as a shapeFlag for computeSmoothShape
const NBNode_AVOID_WIDE_LEFT_TURN: i32 = 1;

/// C++ `NBNodeShapeComputer::SVC_LARGE_TURN`: foot and bicycle paths, service roads and railways get no large junctions
pub const SVC_LARGE_TURN: SVCPermissions = SVC_ALL & !(SVC_BICYCLE | SVC_PEDESTRIAN | SVC_DELIVERY | SVC_RAIL_CLASSES);

// C++ MAX2/MIN2 (utils/common/StdDefs.h): `a > b ? a : b` / `a < b ? a : b`
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
// C++ MIN3(a, b, c) = MIN2(c, a < b ? a : b)
fn min3(a: f64, b: f64, c: f64) -> f64 {
    min2(c, if a < b { a } else { b })
}

/// C++ `NBNode::UNSPECIFIED_RADIUS` (netbuild/NBNode.cpp)
const NBNODE_UNSPECIFIED_RADIUS: f64 = -1.0;
/// C++ `NBEdge::UNSPECIFIED_WIDTH` (netbuild/NBEdge.cpp)
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;

/// C++ `geoms[e]` (operator[] inserts an empty geometry for a missing edge)
fn geom_get(geoms: &mut NBNodeShapeComputer_GeomsMap, e: Ptr<NBEdge>) -> PositionVector {
    geoms.entry(e).or_default().clone()
}
/// C++ `geoms[e][idx]` (operator[] inserts an empty geometry for a missing edge)
fn geom_pos(geoms: &mut NBNodeShapeComputer_GeomsMap, e: Ptr<NBEdge>, idx: i32) -> Position {
    *geoms.entry(e).or_default().at(idx)
}
/// C++ `distances[e]` read (operator[] yields 0 for a missing edge)
fn dget(distances: &BTreeMap<Ptr<NBEdge>, f64>, e: Ptr<NBEdge>) -> f64 {
    distances.get(&e).copied().unwrap_or(0.0)
}

impl NBNodeShapeComputer {
    /// C++: `void NBNodeShapeComputer::NBNodeShapeComputer(const NBNode &)`
    /// Constructor
    // NBNodeShapeComputer::NBNodeShapeComputer(const NBNode& node) :
    //     myNode(node),
    //     myRadius(node.getRadius()) {
    //     if (node.getEdges().size() > 4 && !NBNodeTypeComputer::isRailwayNode(&node)) {
    //         EXT = 50;
    //     } else {
    //         EXT = 100;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(node: &NBNode) -> Self {
        let ext = if node.get_edges().len() > 4 && !NBNodeTypeComputer::is_railway_node(Ptr::new(node)) { 50.0 } else { 100.0 };
        NBNodeShapeComputer { myNode: Ptr::new(node), myRadius: node.get_radius(), EXT: ext }
    }

    /// C++: `void NBNodeShapeComputer::~NBNodeShapeComputer() noexcept`
    /// Destructor
    // NBNodeShapeComputer::~NBNodeShapeComputer() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `const PositionVector NBNodeShapeComputer::compute(bool)`
    /// Computes the shape of the assigned junction
    // const PositionVector
    // NBNodeShapeComputer::compute(bool forceSmall) {
    // #ifdef DEBUG_NODE_SHAPE
    //     if (DEBUGCOND) {
    //         // annotate edges edges to make their ordering visible
    //         int i = 0;
    //         for (NBEdge* e : myNode.getEdges()) {
    //             e->setStreetName(toString(i));
    //             i++;
    //         }
    //     }
    // #endif
    //     // check whether the node is a dead end node or a node where only turning is possible
    //     //  in this case, we will use "computeNodeShapeSmall"
    //     if (myNode.getEdges().size() == 1 || forceSmall) {
    //         return computeNodeShapeSmall();
    //     }
    //     if (myNode.getEdges().size() == 2 && myNode.getIncomingEdges().size() == 1) {
    //         if (myNode.getIncomingEdges()[0]->isTurningDirectionAt(myNode.getOutgoingEdges()[0])) {
    //             return computeNodeShapeSmall();
    //         }
    //     }
    //     const bool geometryLike = myNode.isSimpleContinuation(true, true);
    //     const PositionVector& ret = computeNodeShapeDefault(geometryLike);
    //     // fail fall-back: use "computeNodeShapeSmall"
    //     if (ret.size() < 3) {
    //         return computeNodeShapeSmall();
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn compute(&mut self, forceSmall: bool) -> PositionVector {
        // DEBUG_NODE_SHAPE edge-annotation block is a debug-only #ifdef and is not ported
        // check whether the node is a dead end node or a node where only turning is possible
        //  in this case, we will use "computeNodeShapeSmall"
        if self.myNode.get_edges().len() == 1 || forceSmall {
            return self.compute_node_shape_small();
        }
        if self.myNode.get_edges().len() == 2 && self.myNode.get_incoming_edges().len() == 1 {
            if self.myNode.get_incoming_edges()[0].is_turning_direction_at(self.myNode.get_outgoing_edges()[0]) {
                return self.compute_node_shape_small();
            }
        }
        let geometry_like = self.myNode.is_simple_continuation(true, true);
        let ret = self.compute_node_shape_default(geometry_like);
        // fail fall-back: use "computeNodeShapeSmall"
        if ret.len() < 3 {
            return self.compute_node_shape_small();
        }
        ret
    }

    /// C++: `double NBNodeShapeComputer::getRadius() const` [const]
    /// get computed radius for node
    // double getRadius() const {
    //         return myRadius;
    //     }
    #[allow(unused_variables)]
    pub fn get_radius(&self) -> f64 {
        self.myRadius
    }

    /// C++: `const PositionVector NBNodeShapeComputer::computeNodeShapeDefault(bool)` [private]
    /// Computes the node geometry Edges with the same direction are grouped. Then the node geometry is built from intersection between the borders of adjacent edge groups
    // const PositionVector
    // NBNodeShapeComputer::computeNodeShapeDefault(bool simpleContinuation) {
    //     // if we have less than two edges, we can not compute the node's shape this way
    //     if (myNode.getEdges().size() < 2) {
    //         return PositionVector();
    //     }
    //     // magic values
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const double defaultRadius = getDefaultRadius(oc);
    //     const bool useDefaultRadius = myNode.getRadius() == NBNode::UNSPECIFIED_RADIUS || myNode.getRadius() == defaultRadius;
    //     myRadius = (useDefaultRadius ? defaultRadius : myNode.getRadius());
    //     double smallRadius = useDefaultRadius ? oc.getFloat("junctions.small-radius") : myRadius;
    //     const int cornerDetail = oc.getInt("junctions.corner-detail");
    //     const double sCurveStretch = oc.getFloat("junctions.scurve-stretch");
    //     const bool useEndpoints = oc.getBool("junctions.endpoint-shape");
    //     const bool rectangularCut = oc.getBool("rectangular-lane-cut");
    //     const bool openDriveOutput = oc.isSet("opendrive-output");
    //
    //     // Extend geometries to move the stop line forward.
    //     // In OpenDrive the junction starts whenever the geometry changes. Stop
    //     // line information is not given or ambiguous (sign positions at most)
    //     // In SUMO, stop lines are where the junction starts. This is computed
    //     // heuristically from intersecting the junctions roads geometries.
    //     const double advanceStopLine = oc.exists("opendrive-files") && oc.isSet("opendrive-files") ? oc.getFloat("opendrive.advance-stopline") : 0;
    //
    //
    // #ifdef DEBUG_NODE_SHAPE
    //     if (DEBUGCOND) {
    //         std::cout << "\ncomputeNodeShapeDefault node " << myNode.getID() << " simple=" << simpleContinuation << " useDefaultRadius=" << useDefaultRadius << " radius=" << myRadius << "\n";
    //     }
    // #endif
    //
    //     // initialise
    //     EdgeVector::const_iterator i;
    //     // edges located in the value-vector have the same direction as the key edge
    //     std::map<NBEdge*, std::set<NBEdge*, ComparatorIdLess> > same;
    //     // the counter-clockwise boundary of the edge regarding possible same-direction edges
    //     GeomsMap geomsCCW;
    //     // the clockwise boundary of the edge regarding possible same-direction edges
    //     GeomsMap geomsCW;
    //     EdgeVector usedEdges = myNode.getEdges();
    //     computeEdgeBoundaries(usedEdges, geomsCCW, geomsCW);
    //
    //     // check which edges are parallel
    //     joinSameDirectionEdges(usedEdges, same, useEndpoints);
    //     // compute unique direction list
    //     EdgeVector newAll = computeUniqueDirectionList(usedEdges, same, geomsCCW, geomsCW);
    //     // if we have only two "directions", let's not compute the geometry using this method
    //     if (newAll.size() < 2) {
    //         return PositionVector();
    //     }
    //
    //     // All geoms are outgoing from myNode.
    //     // for every direction in newAll we compute the offset at which the
    //     // intersection ends and the edge starts. This value is saved in 'distances'
    //     // If the geometries need to be extended to get an intersection, this is
    //     // recorded in 'myExtended'
    //     std::map<NBEdge*, double> distances;
    //     std::map<NBEdge*, double> distances2;
    //     std::map<NBEdge*, bool> myExtended;
    //
    //     for (i = newAll.begin(); i != newAll.end(); ++i) {
    //         EdgeVector::const_iterator cwi = i;
    //         EdgeVector::const_iterator ccwi = i;
    //         double ccad;
    //         double cad;
    //         initNeighbors(newAll, i, geomsCW, geomsCCW, cwi, ccwi, cad, ccad);
    //         assert(geomsCCW.find(*i) != geomsCCW.end());
    //         assert(geomsCW.find(*ccwi) != geomsCW.end());
    //         assert(geomsCW.find(*cwi) != geomsCW.end());
    //
    //         // there are only 2 directions and they are almost parallel
    //         if (*cwi == *ccwi &&
    //                 (
    //                     // no change in lane numbers, even low angles still give a good intersection
    //                     (simpleContinuation && fabs(ccad - cad) < (double) 0.1)
    //                     // lane numbers change, a direct intersection could be far away from the node position
    //                     // so we use a larger threshold
    //                     || (!simpleContinuation && fabs(ccad - cad) < DEG2RAD(22.5)))
    //            ) {
    //             // compute the mean position between both edges ends ...
    //             Position p;
    //             if (myExtended.find(*ccwi) != myExtended.end()) {
    //                 p = geomsCCW[*ccwi][0];
    //                 p.add(geomsCW[*ccwi][0]);
    //                 p.mul(0.5);
    // #ifdef DEBUG_NODE_SHAPE
    //                 if (DEBUGCOND) {
    //                     std::cout << " extended: p=" << p << " angle=" << (ccad - cad) << "\n";
    //                 }
    // #endif
    //             } else {
    //                 p = geomsCCW[*ccwi][0];
    //                 p.add(geomsCW[*ccwi][0]);
    //                 p.add(geomsCCW[*i][0]);
    //                 p.add(geomsCW[*i][0]);
    //                 p.mul(0.25);
    // #ifdef DEBUG_NODE_SHAPE
    //                 if (DEBUGCOND) {
    //                     std::cout << " unextended: p=" << p << " angle=" << (ccad - cad) << "\n";
    //                 }
    // #endif
    //             }
    //             // ... compute the distance to this point ...
    //             double dist = MAX2(
    //                               geomsCCW[*i].nearest_offset_to_point2D(p),
    //                               geomsCW[*i].nearest_offset_to_point2D(p));
    //             if (dist < 0) {
    //                 if (isRailway((*i)->getPermissions())) {
    //                     // better not mess up bidi geometries
    //                     return PositionVector();
    //                 }
    //                 // ok, we have the problem that even the extrapolated geometry
    //                 //  does not reach the point
    //                 // in this case, the geometry has to be extenden... too bad ...
    //                 // ... let's append the mean position to the geometry
    //                 PositionVector g = (*i)->getGeometry();
    //                 if (myNode.hasIncoming(*i)) {
    //                     g.push_back_noDoublePos(p);
    //                 } else {
    //                     g.push_front_noDoublePos(p);
    //                 }
    //                 (*i)->setGeometry(g);
    //                 // and rebuild previous information
    //                 geomsCCW[*i] = (*i)->getCCWBoundaryLine(myNode);
    //                 geomsCCW[*i].extrapolate(EXT);
    //                 geomsCW[*i] = (*i)->getCWBoundaryLine(myNode);
    //                 geomsCW[*i].extrapolate(EXT);
    //                 // the distance is now = zero (the point we have appended)
    //                 distances[*i] = EXT;
    //                 myExtended[*i] = true;
    // #ifdef DEBUG_NODE_SHAPE
    //                 if (DEBUGCOND) {
    //                     std::cout << " extending (dist=" << dist << ")\n";
    //                 }
    // #endif
    //             } else {
    //                 if (!simpleContinuation) {
    //                     dist += myRadius;
    //                 } else {
    //                     // if the angles change, junction should have some size to avoid degenerate shape
    //                     double radius2 = fabs(ccad - cad) * (*i)->getNumLanes();
    //                     if (radius2 > NUMERICAL_EPS || openDriveOutput) {
    //                         radius2 = MAX2(0.15, radius2);
    //                     }
    //                     if (myNode.getCrossings().size() > 0) {
    //                         double width = myNode.getCrossings()[0]->customWidth;
    //                         if (width == NBEdge::UNSPECIFIED_WIDTH) {
    //                             width = OptionsCont::getOptions().getFloat("default.crossing-width");
    //                         }
    //                         radius2 = MAX2(radius2, width / 2);
    //                     }
    //                     if (!useDefaultRadius) {
    //                         radius2 = MAX2(radius2, myRadius);
    //                     }
    //                     dist += radius2;
    // #ifdef DEBUG_NODE_SHAPE
    //                     if (DEBUGCOND) {
    //                         std::cout << " using radius=" << radius2 << " ccad=" << ccad << " cad=" << cad << "\n";
    //                     }
    // #endif
    //                 }
    //                 distances[*i] = dist;
    //             }
    //
    //         } else {
    //             // the angles are different enough to compute the intersection of
    //             // the outer boundaries directly (or there are more than 2 directions). The "nearer" neighbor causes the furthest distance
    //             const bool ccwCloser = ccad < cad;
    //             const bool cwLargeTurn = needsLargeTurn(*i, *cwi, same);
    //             const bool ccwLargeTurn = needsLargeTurn(*i, *ccwi, same);
    //             const bool neighLargeTurn = ccwCloser ? ccwLargeTurn : cwLargeTurn;
    //             const bool neigh2LargeTurn =  ccwCloser ? cwLargeTurn : ccwLargeTurn;
    //             // the border facing the closer neighbor
    //             const PositionVector& currGeom = ccwCloser ? geomsCCW[*i] : geomsCW[*i];
    //             // the border facing the far neighbor
    //             const PositionVector& currGeom2 = ccwCloser ? geomsCW[*i] : geomsCCW[*i];
    //             // the border of the closer neighbor
    //             const PositionVector& neighGeom = ccwCloser ? geomsCW[*ccwi] : geomsCCW[*cwi];
    //             // the border of the far neighbor
    //             const PositionVector& neighGeom2 = ccwCloser ? geomsCCW[*cwi] : geomsCW[*ccwi];
    //             // whether the current edge/direction spans a divided road
    //             const bool keepBothDistances = isDivided(*i, same[*i], geomsCCW[*i], geomsCW[*i]);
    // #ifdef DEBUG_NODE_SHAPE
    //             if (DEBUGCOND) {
    //                 std::cout << " i=" << (*i)->getID() << " neigh=" << (*ccwi)->getID() << " neigh2=" << (*cwi)->getID() << "\n";
    //                 std::cout << "    ccwCloser=" << ccwCloser << " divided=" << keepBothDistances
    //                           << "\n      currGeom=" << currGeom << " neighGeom=" << neighGeom
    //                           << "\n      currGeom2=" << currGeom2 << " neighGeom2=" << neighGeom2
    //                           << "\n";
    //             }
    // #endif
    //             if (!simpleContinuation) {
    //                 if (useEndpoints && !(*i)->hasDefaultGeometryEndpointAtNode(&myNode)) {
    //                     distances[*i] = EXT;
    //                 } else if (currGeom.intersects(neighGeom)) {
    //                     distances[*i] = (neighLargeTurn ? myRadius : smallRadius) + closestIntersection(currGeom, neighGeom, EXT);
    // #ifdef DEBUG_NODE_SHAPE
    //                     if (DEBUGCOND) {
    //                         std::cout << "   neigh intersects dist=" << distances[*i] << " currGeom=" << currGeom << " neighGeom=" << neighGeom << "\n";
    //                     }
    // #endif
    //                     if (*cwi != *ccwi && currGeom2.intersects(neighGeom2)) {
    //                         // also use the second intersection point
    //                         // but prevent very large node shapes
    //                         const double farAngleDist = ccwCloser ? cad : ccad;
    //                         double a1 = distances[*i];
    //                         double a2 = (neigh2LargeTurn ? myRadius : smallRadius) + closestIntersection(currGeom2, neighGeom2, EXT);
    // #ifdef DEBUG_NODE_SHAPE
    //                         if (DEBUGCOND) {
    //                             std::cout << "      neigh2 also intersects a1=" << a1 << " a2=" << a2 << " ccad=" << RAD2DEG(ccad) << " cad=" << RAD2DEG(cad) << " dist[cwi]=" << distances[*cwi] << " dist[ccwi]=" << distances[*ccwi] << " farAngleDist=" << RAD2DEG(farAngleDist) << " currGeom2=" << currGeom2 << " neighGeom2=" << neighGeom2 << "\n";
    //                         }
    // #endif
    //                         //if (RAD2DEG(farAngleDist) < 175) {
    //                         //    distances[*i] = MAX2(a1, MIN2(a2, a1 + 180 - RAD2DEG(farAngleDist)));
    //                         //}
    //                         if (a2 <= EXT) {
    //                             if (keepBothDistances) {
    //                                 if (ccwCloser) {
    //                                     distances2[*i] = a2;
    //                                 } else {
    //                                     distances[*i] = a2;
    //                                     distances2[*i] = a1;
    //                                 }
    //                             } else {
    //                                 distances[*i] = MAX2(a1, a2);
    //                             }
    //                         } else if (ccad > DEG2RAD(90. + 45.) && cad > DEG2RAD(90. + 45.)
    //                                    && (fabs(ccad - cad) > DEG2RAD(10)
    //                                        || MAX2(ccad, cad) > DEG2RAD(160)
    //                                        || (a2 - a1) > 7
    //                                        // keep roundabouts nodes small
    //                                        || myNode.isRoundabout())) {
    // #ifdef DEBUG_NODE_SHAPE
    //                             if (DEBUGCOND) {
    //                                 std::cout << "     ignore a2\n";
    //                             }
    // #endif
    //                             // do nothing.
    //                         } else if (farAngleDist < DEG2RAD(135) || (fabs(RAD2DEG(farAngleDist) - 180) > 1 && fabs(a2 - a1) < 10)) {
    //                             if (keepBothDistances) {
    //                                 if (ccwCloser) {
    //                                     distances2[*i] = a2;
    //                                 } else {
    //                                     distances[*i] = a2;
    //                                     distances2[*i] = a1;
    //                                 }
    //                             } else {
    //                                 distances[*i] = MAX2(a1, a2);
    //                             }
    //                         }
    // #ifdef DEBUG_NODE_SHAPE
    //                         if (DEBUGCOND) {
    //                             std::cout << "   a1=" << a1 << " a2=" << a2 << " keepBoth=" << keepBothDistances << " dist=" << distances[*i] << "\n";
    //                         }
    // #endif
    //                     }
    //                 } else {
    //                     if (*cwi != *ccwi && currGeom2.intersects(neighGeom2)) {
    //                         distances[*i] = (neigh2LargeTurn ? myRadius : smallRadius) + currGeom2.intersectsAtLengths2D(neighGeom2)[0];
    // #ifdef DEBUG_NODE_SHAPE
    //                         if (DEBUGCOND) {
    //                             std::cout << "   neigh2 intersects dist=" << distances[*i] << " currGeom2=" << currGeom2 << " neighGeom2=" << neighGeom2 << "\n";
    //                         }
    // #endif
    //                     } else {
    //                         distances[*i] = EXT + myRadius;
    // #ifdef DEBUG_NODE_SHAPE
    //                         if (DEBUGCOND) {
    //                             std::cout << "   no intersects dist=" << distances[*i]  << " currGeom=" << currGeom << " neighGeom=" << neighGeom << " currGeom2=" << currGeom2 << " neighGeom2=" << neighGeom2 << "\n";
    //                         }
    // #endif
    //                     }
    //                 }
    //             } else {
    //                 if (currGeom.intersects(neighGeom)) {
    //                     distances[*i] = currGeom.intersectsAtLengths2D(neighGeom)[0];
    //                 } else {
    //                     distances[*i] = (double) EXT;
    //                 }
    //             }
    //         }
    //         if (useDefaultRadius && sCurveStretch > 0) {
    //             double sCurveWidth = myNode.getDisplacementError();
    //             if (sCurveWidth > 0) {
    //                 const double sCurveRadius = myRadius + sCurveWidth / SUMO_const_laneWidth * sCurveStretch * pow((*i)->getSpeed(), 2 + sCurveStretch) / 1000;
    //                 const double stretch = EXT + sCurveRadius - distances[*i];
    //                 if (stretch > 0) {
    //                     distances[*i] += stretch;
    //                     // fixate extended geometry for repeated computation
    //                     const double shorten = distances[*i] - EXT;
    //                     (*i)->shortenGeometryAtNode(&myNode, shorten);
    //                     for (std::set<NBEdge*>::iterator k = same[*i].begin(); k != same[*i].end(); ++k) {
    //                         (*k)->shortenGeometryAtNode(&myNode, shorten);
    //                     }
    // #ifdef DEBUG_NODE_SHAPE
    //                     if (DEBUGCOND) {
    //                         std::cout << "   stretching junction: sCurveWidth=" << sCurveWidth << " sCurveRadius=" << sCurveRadius << " stretch=" << stretch << " dist=" << distances[*i]  << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //     }
    //
    //     for (NBEdge* const edge : newAll) {
    //         if (distances.find(edge) == distances.end()) {
    //             assert(false);
    //             distances[edge] = EXT;
    //         }
    //     }
    //     // because of lane spread right the crossing point may be identical to the junction center and thus the distance is exactly EXT
    //     const double off = EXT - NUMERICAL_EPS;
    //     // prevent inverted node shapes
    //     // (may happen with near-parallel edges)
    //     const double minDistSum = 2 * (EXT + myRadius);
    //     for (NBEdge* const edge : newAll) {
    //         if (distances[edge] < off && edge->hasDefaultGeometryEndpointAtNode(&myNode)) {
    //             for (EdgeVector::const_iterator j = newAll.begin(); j != newAll.end(); ++j) {
    //                 if (distances[*j] > off && (*j)->hasDefaultGeometryEndpointAtNode(&myNode) && distances[edge] + distances[*j] < minDistSum) {
    //                     const double angleDiff = fabs(NBHelpers::relAngle(edge->getAngleAtNode(&myNode), (*j)->getAngleAtNode(&myNode)));
    //                     if (angleDiff > 160 || angleDiff < 20) {
    // #ifdef DEBUG_NODE_SHAPE
    //                         if (DEBUGCOND) {
    //                             std::cout << "   increasing dist for i=" << edge->getID() << " because of j=" << (*j)->getID() << " jDist=" << distances[*j]
    //                                       << "  oldI=" << distances[edge] << " newI=" << minDistSum - distances[*j]
    //                                       << " angleDiff=" << angleDiff
    //                                       << " geomI=" << edge->getGeometry() << " geomJ=" << (*j)->getGeometry() << "\n";
    //                         }
    // #endif
    //                         distances[edge] = minDistSum - distances[*j];
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //
    //     // build
    //     PositionVector ret;
    //     for (i = newAll.begin(); i != newAll.end(); ++i) {
    //         const PositionVector& ccwBound = geomsCCW[*i];
    //         const PositionVector& cwBound = geomsCW[*i];
    //         //double offset = MIN3(distances[*i], cwBound.length2D() - POSITION_EPS, ccwBound.length2D() - POSITION_EPS);
    //         double offset = distances[*i];
    //         double offset2 = distances2.count(*i) != 0 ? distances2[*i] : offset;
    //         if (offset != offset2) {
    //             // keep rectangular cuts if the difference is small or the roads aren't
    //             // really divided by much (unless the angle is very different)
    //             const double dWidth = divisionWidth(*i, same[*i],
    //                                                 ccwBound.positionAtOffset2D(offset),
    //                                                 cwBound.positionAtOffset2D(offset2));
    //             const double angle = RAD2DEG(GeomHelper::angleDiff(ccwBound.angleAt2D(0), cwBound.angleAt2D(0)));
    //             const double oDelta = fabs(offset - offset2);
    //             //std::cout << " i=" << (*i)->getID() << " offset=" << offset << " offset2=" << offset2 << " dWidth=" << dWidth << " angle=" << angle << " same=" << joinNamedToStringSorting(same[*i], ",") << "\n";
    //             if ((((oDelta < 5 || dWidth < 10) && fabs(angle) < 30)) || (fabs(angle) < 5 && myNode.getType() != SumoXMLNodeType::RAIL_CROSSING)) {
    // #ifdef DEBUG_NODE_SHAPE
    //                 std::cout << " i=" << (*i)->getID() << " offset=" << offset << " offset2=" << offset2 << " dWidth=" << dWidth << " angle=" << angle << " same=" << joinNamedToStringSorting(same[*i], ",") << "\n";
    // #endif
    //                 offset = MAX2(offset, offset2);
    //                 offset2 = offset;
    //             }
    //         }
    //         if (!(*i)->hasDefaultGeometryEndpointAtNode(&myNode)) {
    //             // for non geometry-endpoints, only shorten but never extend the geometry
    //             if (advanceStopLine > 0 && offset < EXT) {
    // #ifdef DEBUG_NODE_SHAPE
    //                 std::cout << " i=" << (*i)->getID() << " offset=" << offset << " advanceStopLine=" << advanceStopLine << "\n";
    // #endif
    //                 // fixate extended geometry for repeated computation
    //                 (*i)->extendGeometryAtNode(&myNode, advanceStopLine);
    //                 for (std::set<NBEdge*>::iterator k = same[*i].begin(); k != same[*i].end(); ++k) {
    //                     (*k)->extendGeometryAtNode(&myNode, advanceStopLine);
    //                 }
    //             }
    //             offset = MAX2(EXT - advanceStopLine, offset);
    //             offset2 = MAX2(EXT - advanceStopLine, offset2);
    //         }
    //         if (offset == -1) {
    //             WRITE_WARNINGF(TL("Fixing offset for edge '%' at node '%."), (*i)->getID(), myNode.getID());
    //             offset = -.1;
    //             offset2 = -.1;
    //         }
    //         Position p = ccwBound.positionAtOffset2D(offset);
    //         p.setz(myNode.getPosition().z());
    //         if (i != newAll.begin()) {
    //             ret.append(getSmoothCorner(geomsCW[*(i - 1)], ccwBound, ret[-1], p, cornerDetail));
    //         }
    //         Position p2 = cwBound.positionAtOffset2D(offset2);
    //         p2.setz(myNode.getPosition().z());
    //         //ret.append(getEdgeCuts(*i, geomsCCW, geomsCW, offset, offset2, same));
    //         ret.push_back_noDoublePos(p);
    //         ret.push_back_noDoublePos(p2);
    // #ifdef DEBUG_NODE_SHAPE
    //         if (DEBUGCOND) {
    //             std::cout << "   build stopLine for i=" << (*i)->getID() << " offset=" << offset << " offset2=" << offset2 << " dist=" << distances[*i] << " cwLength=" << cwBound.length2D() << " ccwLength=" << ccwBound.length2D() << " p=" << p << " p2=" << p2 << " ccwBound=" <<  ccwBound << " cwBound=" << cwBound << "\n";
    //         }
    // #endif
    //         (*i)->setNodeBorder(&myNode, p, p2, rectangularCut);
    //         for (std::set<NBEdge*>::iterator k = same[*i].begin(); k != same[*i].end(); ++k) {
    //             (*k)->setNodeBorder(&myNode, p, p2, rectangularCut);
    //         }
    //     }
    //     // final curve segment
    //     ret.append(getSmoothCorner(geomsCW[*(newAll.end() - 1)], geomsCCW[*newAll.begin()], ret[-1], ret[0], cornerDetail));
    // #ifdef DEBUG_NODE_SHAPE
    //     if (DEBUGCOND) {
    //         std::cout << " final shape=" << ret << "\n";
    //     }
    // #endif
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn compute_node_shape_default(&mut self, simpleContinuation: bool) -> PositionVector {
        use utils::geom::geom_helper::{deg2rad, rad2deg};
        let node = self.myNode;
        // if we have less than two edges, we can not compute the node's shape this way
        if node.get_edges().len() < 2 {
            return PositionVector::new();
        }
        // magic values
        let defaultRadius = utils::options::options_cont::with_options(|oc| self.get_default_radius(oc));
        let useDefaultRadius = node.get_radius() == NBNODE_UNSPECIFIED_RADIUS || node.get_radius() == defaultRadius;
        self.myRadius = if useDefaultRadius { defaultRadius } else { node.get_radius() };
        let smallRadius = if useDefaultRadius { get_float("junctions.small-radius") } else { self.myRadius };
        let cornerDetail = get_int("junctions.corner-detail");
        let sCurveStretch = get_float("junctions.scurve-stretch");
        let useEndpoints = get_bool("junctions.endpoint-shape");
        let rectangularCut = get_bool("rectangular-lane-cut");
        let openDriveOutput = is_set("opendrive-output");
        // Extend geometries to move the stop line forward (OpenDRIVE junction start heuristic)
        let advanceStopLine = if exists("opendrive-files") && is_set("opendrive-files") { get_float("opendrive.advance-stopline") } else { 0.0 };
        let ext = self.EXT;

        // initialise
        // edges located in the value-vector have the same direction as the key edge
        let mut same: BTreeMap<Ptr<NBEdge>, BTreeSet<Ptr<NBEdge>>> = BTreeMap::new();
        // the counter-clockwise boundary of the edge regarding possible same-direction edges
        let mut geomsCCW: NBNodeShapeComputer_GeomsMap = BTreeMap::new();
        // the clockwise boundary of the edge regarding possible same-direction edges
        let mut geomsCW: NBNodeShapeComputer_GeomsMap = BTreeMap::new();
        let usedEdges: EdgeVector = node.get_edges().clone();
        self.compute_edge_boundaries(&usedEdges, &mut geomsCCW, &mut geomsCW);

        // check which edges are parallel
        self.join_same_direction_edges(&usedEdges, &mut same, useEndpoints);
        // compute unique direction list
        let newAll = self.compute_unique_direction_list(&usedEdges, &mut same, &mut geomsCCW, &mut geomsCW);
        // if we have only two "directions", let's not compute the geometry using this method
        if newAll.len() < 2 {
            return PositionVector::new();
        }

        // All geoms are outgoing from myNode.
        // distances: offset at which the intersection ends and the edge starts
        // distances2: second offset for divided roads
        // myExtended: edges whose geometry had to be extended to get an intersection
        let mut distances: BTreeMap<Ptr<NBEdge>, f64> = BTreeMap::new();
        let mut distances2: BTreeMap<Ptr<NBEdge>, f64> = BTreeMap::new();
        let mut myExtended: BTreeSet<Ptr<NBEdge>> = BTreeSet::new();
        let n = newAll.len();

        for i in 0..n {
            let ei = newAll[i];
            let mut cwi: usize = 0;
            let mut ccwi: usize = 0;
            let mut cad: f64 = 0.0;
            let mut ccad: f64 = 0.0;
            Self::init_neighbors(&newAll, i, &mut geomsCW, &mut geomsCCW, &mut cwi, &mut ccwi, &mut cad, &mut ccad);
            let eCwi = newAll[cwi];
            let eCcwi = newAll[ccwi];

            // there are only 2 directions and they are almost parallel
            if eCwi == eCcwi
                && ((simpleContinuation && (ccad - cad).abs() < 0.1) || (!simpleContinuation && (ccad - cad).abs() < deg2rad(22.5)))
            {
                // compute the mean position between both edges ends ...
                let mut p: Position;
                if myExtended.contains(&eCcwi) {
                    p = geom_pos(&mut geomsCCW, eCcwi, 0);
                    p.add(&geom_pos(&mut geomsCW, eCcwi, 0));
                    p.mul(0.5);
                } else {
                    p = geom_pos(&mut geomsCCW, eCcwi, 0);
                    p.add(&geom_pos(&mut geomsCW, eCcwi, 0));
                    p.add(&geom_pos(&mut geomsCCW, ei, 0));
                    p.add(&geom_pos(&mut geomsCW, ei, 0));
                    p.mul(0.25);
                }
                // ... compute the distance to this point ...
                let dCcw = geom_get(&mut geomsCCW, ei).nearest_offset_to_point2d(&p, true);
                let dCw = geom_get(&mut geomsCW, ei).nearest_offset_to_point2d(&p, true);
                let mut dist = max2(dCcw, dCw);
                if dist < 0.0 {
                    if utils::common::sumo_vehicle_class::is_railway(ei.get_permissions(-1)) {
                        // better not mess up bidi geometries
                        return PositionVector::new();
                    }
                    // ok, the extrapolated geometry does not reach the point:
                    // append the mean position to the geometry
                    let mut g = ei.get_geometry().clone();
                    if node.has_incoming(ei) {
                        g.push_back_no_double_pos(&p);
                    } else {
                        g.push_front_no_double_pos(&p);
                    }
                    ei.m().set_geometry(&g, false);
                    // and rebuild previous information
                    let mut ccwB = ei.get_ccw_boundary_line(&*node);
                    ccwB.extrapolate(ext, false, false);
                    geomsCCW.insert(ei, ccwB);
                    let mut cwB = ei.get_cw_boundary_line(&*node);
                    cwB.extrapolate(ext, false, false);
                    geomsCW.insert(ei, cwB);
                    // the distance is now = zero (the point we have appended)
                    distances.insert(ei, ext);
                    myExtended.insert(ei);
                } else {
                    if !simpleContinuation {
                        dist += self.myRadius;
                    } else {
                        // if the angles change, junction should have some size to avoid degenerate shape
                        let mut radius2 = (ccad - cad).abs() * ei.get_num_lanes() as f64;
                        if radius2 > NUMERICAL_EPS || openDriveOutput {
                            radius2 = max2(0.15, radius2);
                        }
                        if node.get_crossings().len() > 0 {
                            let mut width = node.get_crossings()[0].customWidth;
                            if width == NBEDGE_UNSPECIFIED_WIDTH {
                                width = get_float("default.crossing-width");
                            }
                            radius2 = max2(radius2, width / 2.0);
                        }
                        if !useDefaultRadius {
                            radius2 = max2(radius2, self.myRadius);
                        }
                        dist += radius2;
                    }
                    distances.insert(ei, dist);
                }
            } else {
                // the angles are different enough to compute the intersection of
                // the outer boundaries directly (or there are more than 2 directions). The "nearer" neighbor causes the furthest distance
                let ccwCloser = ccad < cad;
                let cwLargeTurn = self.needs_large_turn(ei, eCwi, &mut same);
                let ccwLargeTurn = self.needs_large_turn(ei, eCcwi, &mut same);
                let neighLargeTurn = if ccwCloser { ccwLargeTurn } else { cwLargeTurn };
                let neigh2LargeTurn = if ccwCloser { cwLargeTurn } else { ccwLargeTurn };
                // the border facing the closer neighbor
                let currGeom = if ccwCloser { geom_get(&mut geomsCCW, ei) } else { geom_get(&mut geomsCW, ei) };
                // the border facing the far neighbor
                let currGeom2 = if ccwCloser { geom_get(&mut geomsCW, ei) } else { geom_get(&mut geomsCCW, ei) };
                // the border of the closer neighbor
                let neighGeom = if ccwCloser { geom_get(&mut geomsCW, eCcwi) } else { geom_get(&mut geomsCCW, eCwi) };
                // the border of the far neighbor
                let neighGeom2 = if ccwCloser { geom_get(&mut geomsCCW, eCwi) } else { geom_get(&mut geomsCW, eCcwi) };
                // whether the current edge/direction spans a divided road
                let sameI: BTreeSet<Ptr<NBEdge>> = same.entry(ei).or_default().clone();
                let keepBothDistances = self.is_divided(ei, sameI, &geom_get(&mut geomsCCW, ei), &geom_get(&mut geomsCW, ei));
                if !simpleContinuation {
                    if useEndpoints && !ei.has_default_geometry_endpoint_at_node(node) {
                        distances.insert(ei, ext);
                    } else if currGeom.intersects_pv(&neighGeom) {
                        let base = if neighLargeTurn { self.myRadius } else { smallRadius };
                        let ci = self.closest_intersection(&currGeom, &neighGeom, ext);
                        distances.insert(ei, base + ci);
                        if eCwi != eCcwi && currGeom2.intersects_pv(&neighGeom2) {
                            // also use the second intersection point
                            // but prevent very large node shapes
                            let farAngleDist = if ccwCloser { cad } else { ccad };
                            let a1 = dget(&distances, ei);
                            let base2 = if neigh2LargeTurn { self.myRadius } else { smallRadius };
                            let ci2 = self.closest_intersection(&currGeom2, &neighGeom2, ext);
                            let a2 = base2 + ci2;
                            if a2 <= ext {
                                if keepBothDistances {
                                    if ccwCloser {
                                        distances2.insert(ei, a2);
                                    } else {
                                        distances.insert(ei, a2);
                                        distances2.insert(ei, a1);
                                    }
                                } else {
                                    distances.insert(ei, max2(a1, a2));
                                }
                            } else if ccad > deg2rad(90.0 + 45.0)
                                && cad > deg2rad(90.0 + 45.0)
                                && ((ccad - cad).abs() > deg2rad(10.0)
                                    || max2(ccad, cad) > deg2rad(160.0)
                                    || (a2 - a1) > 7.0
                                    // keep roundabouts nodes small
                                    || node.is_roundabout())
                            {
                                // ignore a2: do nothing.
                            } else if farAngleDist < deg2rad(135.0) || ((rad2deg(farAngleDist) - 180.0).abs() > 1.0 && (a2 - a1).abs() < 10.0) {
                                if keepBothDistances {
                                    if ccwCloser {
                                        distances2.insert(ei, a2);
                                    } else {
                                        distances.insert(ei, a2);
                                        distances2.insert(ei, a1);
                                    }
                                } else {
                                    distances.insert(ei, max2(a1, a2));
                                }
                            }
                        }
                    } else if eCwi != eCcwi && currGeom2.intersects_pv(&neighGeom2) {
                        let base = if neigh2LargeTurn { self.myRadius } else { smallRadius };
                        distances.insert(ei, base + currGeom2.intersects_at_lengths2d_pv(&neighGeom2)[0]);
                    } else {
                        distances.insert(ei, ext + self.myRadius);
                    }
                } else if currGeom.intersects_pv(&neighGeom) {
                    distances.insert(ei, currGeom.intersects_at_lengths2d_pv(&neighGeom)[0]);
                } else {
                    distances.insert(ei, ext);
                }
            }
            if useDefaultRadius && sCurveStretch > 0.0 {
                let sCurveWidth = node.get_displacement_error();
                if sCurveWidth > 0.0 {
                    let sCurveRadius = self.myRadius + sCurveWidth / SUMO_CONST_LANE_WIDTH * sCurveStretch * ei.get_speed().powf(2.0 + sCurveStretch) / 1000.0;
                    let stretch = ext + sCurveRadius - dget(&distances, ei);
                    if stretch > 0.0 {
                        // fixate extended geometry for repeated computation
                        let nd = dget(&distances, ei) + stretch;
                        distances.insert(ei, nd);
                        let shorten = nd - ext;
                        ei.m().shorten_geometry_at_node(node, shorten);
                        let sameV: Vec<Ptr<NBEdge>> = same.get(&ei).map(|s| s.iter().copied().collect()).unwrap_or_default();
                        for k in sameV {
                            k.m().shorten_geometry_at_node(node, shorten);
                        }
                    }
                }
            }
        }

        for &edge in &newAll {
            if !distances.contains_key(&edge) {
                distances.insert(edge, ext);
            }
        }
        // because of lane spread right the crossing point may be identical to the junction center and thus the distance is exactly EXT
        let off = ext - NUMERICAL_EPS;
        // prevent inverted node shapes (may happen with near-parallel edges)
        let minDistSum = 2.0 * (ext + self.myRadius);
        for &edge in &newAll {
            if dget(&distances, edge) < off && edge.has_default_geometry_endpoint_at_node(node) {
                for &j in &newAll {
                    if dget(&distances, j) > off && j.has_default_geometry_endpoint_at_node(node) && dget(&distances, edge) + dget(&distances, j) < minDistSum {
                        let angleDiff = NBHelpers::rel_angle(edge.get_angle_at_node(node), j.get_angle_at_node(node)).abs();
                        if angleDiff > 160.0 || angleDiff < 20.0 {
                            let nd = minDistSum - dget(&distances, j);
                            distances.insert(edge, nd);
                        }
                    }
                }
            }
        }

        // build
        let mut ret = PositionVector::new();
        for i in 0..n {
            let edge = newAll[i];
            let ccwBound = geom_get(&mut geomsCCW, edge);
            let cwBound = geom_get(&mut geomsCW, edge);
            let mut offset = dget(&distances, edge);
            let mut offset2 = distances2.get(&edge).copied().unwrap_or(offset);
            if offset != offset2 {
                // keep rectangular cuts if the difference is small or the roads aren't
                // really divided by much (unless the angle is very different)
                let sameEdge: BTreeSet<Ptr<NBEdge>> = same.get(&edge).cloned().unwrap_or_default();
                let dWidth = Self::division_width(edge, sameEdge, &ccwBound.position_at_offset2d(offset, 0.0, false), &cwBound.position_at_offset2d(offset2, 0.0, false));
                let angle = rad2deg(utils::geom::geom_helper::angle_diff(ccwBound.angle_at2d(0), cwBound.angle_at2d(0)));
                let oDelta = (offset - offset2).abs();
                if ((oDelta < 5.0 || dWidth < 10.0) && angle.abs() < 30.0) || (angle.abs() < 5.0 && node.get_type() != SumoXMLNodeType::RAIL_CROSSING) {
                    offset = max2(offset, offset2);
                    offset2 = offset;
                }
            }
            if !edge.has_default_geometry_endpoint_at_node(node) {
                // for non geometry-endpoints, only shorten but never extend the geometry
                if advanceStopLine > 0.0 && offset < ext {
                    // fixate extended geometry for repeated computation
                    edge.m().extend_geometry_at_node(node, advanceStopLine);
                    let sameV: Vec<Ptr<NBEdge>> = same.get(&edge).map(|s| s.iter().copied().collect()).unwrap_or_default();
                    for k in sameV {
                        k.m().extend_geometry_at_node(node, advanceStopLine);
                    }
                }
                offset = max2(ext - advanceStopLine, offset);
                offset2 = max2(ext - advanceStopLine, offset2);
            }
            if offset == -1.0 {
                write_warningf!("Fixing offset for edge '%' at node '%.", edge.get_id(), node.get_id());
                offset = -0.1;
                offset2 = -0.1;
            }
            let mut p = ccwBound.position_at_offset2d(offset, 0.0, false);
            p.setz(node.get_position().z());
            if i != 0 {
                let prev = geom_get(&mut geomsCW, newAll[i - 1]);
                let corner = self.get_smooth_corner(prev, ccwBound.clone(), ret.at(-1), &p, cornerDetail);
                ret.append(&corner, 2.0);
            }
            let mut p2 = cwBound.position_at_offset2d(offset2, 0.0, false);
            p2.setz(node.get_position().z());
            //ret.append(getEdgeCuts(*i, geomsCCW, geomsCW, offset, offset2, same));
            ret.push_back_no_double_pos(&p);
            ret.push_back_no_double_pos(&p2);
            edge.m().set_node_border(node, &p, &p2, rectangularCut);
            let sameV: Vec<Ptr<NBEdge>> = same.get(&edge).map(|s| s.iter().copied().collect()).unwrap_or_default();
            for k in sameV {
                k.m().set_node_border(node, &p, &p2, rectangularCut);
            }
        }
        // final curve segment
        let corner = self.get_smooth_corner(geom_get(&mut geomsCW, newAll[n - 1]), geom_get(&mut geomsCCW, newAll[0]), ret.at(-1), ret.at(0), cornerDetail);
        ret.append(&corner, 2.0);
        ret
    }

    /// C++: `const PositionVector NBNodeShapeComputer::computeNodeShapeSmall()` [private]
    /// Computes the node geometry using normals
    // const PositionVector
    // NBNodeShapeComputer::computeNodeShapeSmall() {
    // #ifdef DEBUG_NODE_SHAPE
    //     if (DEBUGCOND) {
    //         std::cout << "computeNodeShapeSmall node=" << myNode.getID() << "\n";
    //     }
    // #endif
    //     PositionVector ret;
    //     for (NBEdge* e : myNode.getEdges()) {
    //         // compute crossing with normal
    //         PositionVector edgebound1 = e->getCCWBoundaryLine(myNode).getSubpartByIndex(0, 2);
    //         PositionVector edgebound2 = e->getCWBoundaryLine(myNode).getSubpartByIndex(0, 2);
    //         Position delta = edgebound1[1] - edgebound1[0];
    //         delta.set(-delta.y(), delta.x()); // rotate 90 degrees
    //         PositionVector cross(myNode.getPosition(), myNode.getPosition() + delta);
    //         cross.extrapolate2D(500);
    //         edgebound1.extrapolate2D(500);
    //         edgebound2.extrapolate2D(500);
    //         if (cross.intersects(edgebound1)) {
    //             Position np = cross.intersectionPosition2D(edgebound1);
    //             np.set(np.x(), np.y(), myNode.getPosition().z());
    //             ret.push_back_noDoublePos(np);
    //         }
    //         if (cross.intersects(edgebound2)) {
    //             Position np = cross.intersectionPosition2D(edgebound2);
    //             np.set(np.x(), np.y(), myNode.getPosition().z());
    //             ret.push_back_noDoublePos(np);
    //         }
    //         e->resetNodeBorder(&myNode);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn compute_node_shape_small(&mut self) -> PositionVector {
        let node = self.myNode;
        let mut ret = PositionVector::new();
        for &e in node.get_edges() {
            // compute crossing with normal
            let mut edgebound1 = e.get_ccw_boundary_line(&*node).get_subpart_by_index(0, 2);
            let mut edgebound2 = e.get_cw_boundary_line(&*node).get_subpart_by_index(0, 2);
            let mut delta = edgebound1[1] - edgebound1[0];
            delta.set(-delta.y(), delta.x()); // rotate 90 degrees
            let mut cross = PositionVector::from_line(*node.get_position(), *node.get_position() + delta);
            cross.extrapolate2d(500.0, false);
            edgebound1.extrapolate2d(500.0, false);
            edgebound2.extrapolate2d(500.0, false);
            if cross.intersects_pv(&edgebound1) {
                let mut np = cross.intersection_position2d_pv(&edgebound1);
                np.set3(np.x(), np.y(), node.get_position().z());
                ret.push_back_no_double_pos(&np);
            }
            if cross.intersects_pv(&edgebound2) {
                let mut np = cross.intersection_position2d_pv(&edgebound2);
                np.set3(np.x(), np.y(), node.get_position().z());
                ret.push_back_no_double_pos(&np);
            }
            e.m().reset_node_border(node);
        }
        ret
    }

    /// C++: `void NBNodeShapeComputer::computeEdgeBoundaries(const EdgeVector &, GeomsMap &, GeomsMap &)` [private]
    /// compute clockwise/counter-clockwise edge boundaries
    // void
    // NBNodeShapeComputer::computeEdgeBoundaries(const EdgeVector& edges,
    //         GeomsMap& geomsCCW,
    //         GeomsMap& geomsCW) {
    //     // compute boundary lines and extend it by EXT m
    //     for (NBEdge* const edge : edges) {
    //         // store current edge's boundary as current ccw/cw boundary
    //         try {
    //             geomsCCW[edge] = edge->getCCWBoundaryLine(myNode);
    //         } catch (InvalidArgument& e) {
    //             WRITE_WARNING("While computing intersection geometry at junction '" + myNode.getID() + "': " + std::string(e.what()));
    //             geomsCCW[edge] = edge->getGeometry();
    //         }
    //         try {
    //             geomsCW[edge] = edge->getCWBoundaryLine(myNode);
    //         } catch (InvalidArgument& e) {
    //             WRITE_WARNING("While computing intersection geometry at junction '" + myNode.getID() + "': " + std::string(e.what()));
    //             geomsCW[edge] = edge->getGeometry();
    //         }
    //         // ensure the boundary is valid
    //         if (geomsCCW[edge].length2D() < NUMERICAL_EPS) {
    //             geomsCCW[edge] = edge->getGeometry();
    //         }
    //         if (geomsCW[edge].length2D() < NUMERICAL_EPS) {
    //             geomsCW[edge] = edge->getGeometry();
    //         }
    //         // cut off all parts beyond EXT to avoid issues with curved-back roads
    //         geomsCCW[edge] = geomsCCW[edge].getSubpart2D(0, MAX2(EXT, edge->getTotalWidth()));
    //         geomsCW[edge] = geomsCW[edge].getSubpart2D(0, MAX2(EXT, edge->getTotalWidth()));
    //         // extend the boundary by extrapolating it by EXT m towards the junction
    //         geomsCCW[edge].extrapolate2D(EXT, true);
    //         geomsCW[edge].extrapolate2D(EXT, true);
    //         // ensure minimum length by extending it away from the junction
    //         geomsCCW[edge].extrapolate(EXT2, false, true);
    //         geomsCW[edge].extrapolate(EXT2, false, true);
    //         if (geomsCCW[edge].isNAN() || geomsCW[edge].isNAN()) {
    //             WRITE_WARNING("While computing intersection geometry at junction '" + myNode.getID() + "': found invalid boundary line for edge '" + edge->getID() + "'.")
    //         }
    //         if (geomsCCW[edge].isNAN()) {
    //             geomsCCW[edge] = edge->getGeometry();
    //         }
    //         if (geomsCW[edge].isNAN()) {
    //             geomsCW[edge] = edge->getGeometry();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_edge_boundaries(&mut self, edges: &EdgeVector, geomsCCW: &mut NBNodeShapeComputer_GeomsMap, geomsCW: &mut NBNodeShapeComputer_GeomsMap) {
        let node = self.myNode;
        let ext = self.EXT;
        // compute boundary lines and extend it by EXT m
        for &edge in edges {
            // store current edge's boundary as current ccw/cw boundary
            // shortcut: C++ try/catch(InvalidArgument) around getCCW/CWBoundaryLine is not ported, NBEdge boundary getters return no Result; upgrade when they can fail
            let mut ccw = edge.get_ccw_boundary_line(&*node);
            let mut cw = edge.get_cw_boundary_line(&*node);
            // ensure the boundary is valid
            if ccw.length2d() < NUMERICAL_EPS {
                ccw = edge.get_geometry().clone();
            }
            if cw.length2d() < NUMERICAL_EPS {
                cw = edge.get_geometry().clone();
            }
            // cut off all parts beyond EXT to avoid issues with curved-back roads
            ccw = ccw.get_subpart2d(0.0, max2(ext, edge.get_total_width()));
            cw = cw.get_subpart2d(0.0, max2(ext, edge.get_total_width()));
            // extend the boundary by extrapolating it by EXT m towards the junction
            ccw.extrapolate2d(ext, true);
            cw.extrapolate2d(ext, true);
            // ensure minimum length by extending it away from the junction
            ccw.extrapolate(EXT2, false, true);
            cw.extrapolate(EXT2, false, true);
            if ccw.is_nan() || cw.is_nan() {
                write_warning(&format!(
                    "While computing intersection geometry at junction '{}': found invalid boundary line for edge '{}'.",
                    node.get_id(),
                    edge.get_id()
                ));
            }
            if ccw.is_nan() {
                ccw = edge.get_geometry().clone();
            }
            if cw.is_nan() {
                cw = edge.get_geometry().clone();
            }
            geomsCCW.insert(edge, ccw);
            geomsCW.insert(edge, cw);
        }
    }

    /// C++: `void NBNodeShapeComputer::joinSameDirectionEdges(const EdgeVector &, std::map<NBEdge *, std::set<NBEdge *, ComparatorIdLess>> &, bool)` [private]
    /// Joins edges and computes ccw/cw boundaries
    // void
    // NBNodeShapeComputer::joinSameDirectionEdges(const EdgeVector& edges, std::map<NBEdge*, std::set<NBEdge*, ComparatorIdLess> >& same, bool useEndpoints) {
    //     // compute same (edges where an intersection doesn't work well
    //     // (always check an edge and its cw neighbor)
    //     const double angleChangeLookahead = 35; // distance to look ahead for a misleading angle
    //     const bool isXodr = OptionsCont::getOptions().exists("opendrive-files") && OptionsCont::getOptions().isSet("opendrive-files");
    //     EdgeSet foundOpposite;
    //     for (EdgeVector::const_iterator i = edges.begin(); i != edges.end(); i++) {
    //         EdgeVector::const_iterator j;
    //         if (i == edges.end() - 1) {
    //             j = edges.begin();
    //         } else {
    //             j = i + 1;
    //         }
    //         if (useEndpoints
    //                 && !(*i)->hasDefaultGeometryEndpointAtNode(&myNode)
    //                 && !(*j)->hasDefaultGeometryEndpointAtNode(&myNode)) {
    //             continue;
    //         }
    //         const bool incoming = (*i)->getToNode() == &myNode;
    //         const bool incoming2 = (*j)->getToNode() == &myNode;
    //         const bool differentDirs = (incoming != incoming2);
    //         const bool sameGeom = (*i)->getGeometry() == (differentDirs ? (*j)->getGeometry().reverse() : (*j)->getGeometry());
    //         const PositionVector g1 = incoming ? (*i)->getCCWBoundaryLine(myNode) : (*i)->getCWBoundaryLine(myNode);
    //         const PositionVector g2 = incoming ? (*j)->getCCWBoundaryLine(myNode) : (*j)->getCWBoundaryLine(myNode);
    //         const double angle1further = (g1.size() > 2 && g1[0].distanceTo2D(g1[1]) < angleChangeLookahead ?
    //                                       g1.angleAt2D(1) : g1.angleAt2D(0));
    //         const double angle2further = (g2.size() > 2 && g2[0].distanceTo2D(g2[1]) < angleChangeLookahead ?
    //                                       g2.angleAt2D(1) : g2.angleAt2D(0));
    //         const double angleDiff = GeomHelper::angleDiff(g1.angleAt2D(0), g2.angleAt2D(0));
    //         const double angleDiffFurther = GeomHelper::angleDiff(angle1further, angle2further);
    //         const bool ambiguousGeometry = ((angleDiff > 0 && angleDiffFurther < 0) || (angleDiff < 0 && angleDiffFurther > 0));
    //         //if (ambiguousGeometry) {
    //         //    @todo: this warning would be helpful in many cases. However, if angle and angleFurther jump between 179 and -179 it is misleading
    //         //    WRITE_WARNINGF(TL("Ambiguous angles at junction '%' for edges '%' and '%'."), myNode.getID(), (*i)->getID(), (*j)->getID());
    //         //}
    // #ifdef DEBUG_NODE_SHAPE
    //         if (DEBUGCOND) {
    //             std::cout << "   checkSameDirection " << (*i)->getID() << " " << (*j)->getID()
    //                       << " sameGeom=" << sameGeom
    //                       << " diffDirs=" << differentDirs
    //                       << " isOpposite=" << (differentDirs && foundOpposite.count(*i) == 0)
    //                       << " angleDiff=" << angleDiff
    //                       << " ambiguousGeometry=" << ambiguousGeometry
    //                       << " badInsersection=" << badIntersection(*i, *j, EXT)
    //                       << "\n";
    //
    //         }
    // #endif
    //         if (sameGeom || fabs(angleDiff) < DEG2RAD(20)) {
    //             const bool isOpposite = differentDirs && foundOpposite.count(*i) == 0;
    //             if (isOpposite) {
    //                 foundOpposite.insert(*i);
    //                 foundOpposite.insert(*j);
    //             }
    //             if (isOpposite || ambiguousGeometry || (!isXodr && badIntersection(*i, *j, EXT))) {
    //                 // maintain equivalence relation for all members of the equivalence class
    //                 for (std::set<NBEdge*>::iterator k = same[*i].begin(); k != same[*i].end(); ++k) {
    //                     if (*j != *k) {
    //                         same[*k].insert(*j);
    //                         same[*j].insert(*k);
    //                     }
    //                 }
    //                 for (std::set<NBEdge*>::iterator k = same[*j].begin(); k != same[*j].end(); ++k) {
    //                     if (*i != *k) {
    //                         same[*k].insert(*i);
    //                         same[*i].insert(*k);
    //                     }
    //                 }
    //                 same[*i].insert(*j);
    //                 same[*j].insert(*i);
    // #ifdef DEBUG_NODE_SHAPE
    //                 if (DEBUGCOND) {
    //                     std::cout << "   joinedSameDirectionEdges " << (*i)->getID() << "   " << (*j)->getID() << " isOpposite=" << isOpposite << " ambiguousGeometry=" << ambiguousGeometry << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join_same_direction_edges(&mut self, edges: &EdgeVector, same: &mut BTreeMap<Ptr<NBEdge>, BTreeSet<Ptr<NBEdge>>>, useEndpoints: bool) {
        let node = self.myNode;
        let ext = self.EXT;
        // compute same (edges where an intersection doesn't work well
        // (always check an edge and its cw neighbor)
        let angleChangeLookahead: f64 = 35.0; // distance to look ahead for a misleading angle
        let isXodr = utils::options::options_cont::exists("opendrive-files") && utils::options::options_cont::is_set("opendrive-files");
        let mut foundOpposite: EdgeSet = EdgeSet::new();
        for i in 0..edges.len() {
            let j = if i == edges.len() - 1 { 0 } else { i + 1 };
            let ei = edges[i];
            let ej = edges[j];
            if useEndpoints && !ei.has_default_geometry_endpoint_at_node(node) && !ej.has_default_geometry_endpoint_at_node(node) {
                continue;
            }
            let incoming = ei.get_to_node() == node;
            let incoming2 = ej.get_to_node() == node;
            let differentDirs = incoming != incoming2;
            let sameGeom = if differentDirs {
                ei.get_geometry() == &ej.get_geometry().reverse()
            } else {
                ei.get_geometry() == ej.get_geometry()
            };
            let g1 = if incoming { ei.get_ccw_boundary_line(&*node) } else { ei.get_cw_boundary_line(&*node) };
            let g2 = if incoming { ej.get_ccw_boundary_line(&*node) } else { ej.get_cw_boundary_line(&*node) };
            let angle1further = if g1.len() > 2 && g1[0].distance_to2d(&g1[1]) < angleChangeLookahead { g1.angle_at2d(1) } else { g1.angle_at2d(0) };
            let angle2further = if g2.len() > 2 && g2[0].distance_to2d(&g2[1]) < angleChangeLookahead { g2.angle_at2d(1) } else { g2.angle_at2d(0) };
            let angleDiff = utils::geom::geom_helper::angle_diff(g1.angle_at2d(0), g2.angle_at2d(0));
            let angleDiffFurther = utils::geom::geom_helper::angle_diff(angle1further, angle2further);
            let ambiguousGeometry = (angleDiff > 0.0 && angleDiffFurther < 0.0) || (angleDiff < 0.0 && angleDiffFurther > 0.0);
            //if (ambiguousGeometry) {
            //    @todo: this warning would be helpful in many cases. However, if angle and angleFurther jump between 179 and -179 it is misleading
            //    WRITE_WARNINGF(TL("Ambiguous angles at junction '%' for edges '%' and '%'."), myNode.getID(), (*i)->getID(), (*j)->getID());
            //}
            if sameGeom || angleDiff.abs() < utils::geom::geom_helper::deg2rad(20.0) {
                let isOpposite = differentDirs && !foundOpposite.contains(&ei);
                if isOpposite {
                    foundOpposite.insert(ei);
                    foundOpposite.insert(ej);
                }
                if isOpposite || ambiguousGeometry || (!isXodr && self.bad_intersection(ei, ej, ext)) {
                    // determinism: C++ iterates the ComparatorIdLess sets in ID order; here the snapshot is in address order.
                    // Only inserts happen in these loops, so the resulting sets do not depend on the iteration order.
                    // maintain equivalence relation for all members of the equivalence class
                    let si: Vec<Ptr<NBEdge>> = same.get(&ei).map(|s| s.iter().copied().collect()).unwrap_or_default();
                    for k in si {
                        if ej != k {
                            same.entry(k).or_default().insert(ej);
                            same.entry(ej).or_default().insert(k);
                        }
                    }
                    let sj: Vec<Ptr<NBEdge>> = same.get(&ej).map(|s| s.iter().copied().collect()).unwrap_or_default();
                    for k in sj {
                        if ei != k {
                            same.entry(k).or_default().insert(ei);
                            same.entry(ei).or_default().insert(k);
                        }
                    }
                    same.entry(ei).or_default().insert(ej);
                    same.entry(ej).or_default().insert(ei);
                }
            }
        }
    }

    /// C++: `EdgeVector NBNodeShapeComputer::computeUniqueDirectionList(const EdgeVector &, std::map<NBEdge *, std::set<NBEdge *, ComparatorIdLess>> &, GeomsMap &, GeomsMap &)` [private]
    /// Joins edges
    // EdgeVector
    // NBNodeShapeComputer::computeUniqueDirectionList(
    //     const EdgeVector& all,
    //     std::map<NBEdge*, std::set<NBEdge*, ComparatorIdLess> >& same,
    //     GeomsMap& geomsCCW,
    //     GeomsMap& geomsCW) {
    //     // store relationships
    //     EdgeVector newAll = all;
    //     for (NBEdge* e1 : all) {
    //         // determine which of the edges marks the outer boundary
    //         auto e2NewAll = std::find(newAll.begin(), newAll.end(), e1);
    // #ifdef DEBUG_NODE_SHAPE
    //         if (DEBUGCOND) std::cout << "computeUniqueDirectionList e1=" << e1->getID()
    //                                      << " deleted=" << (e2NewAll == newAll.end())
    //                                      << " same=" << joinNamedToStringSorting(same[e1], ',') << "\n";
    // #endif
    //         if (e2NewAll == newAll.end()) {
    //             continue;
    //         }
    //         auto e1It = std::find(all.begin(), all.end(), e1);
    //         auto bestCCW = e1It;
    //         auto bestCW = e1It;
    //         bool changed = true;
    //         while (changed) {
    //             changed = false;
    //             for (NBEdge* e2 : same[e1]) {
    // #ifdef DEBUG_NODE_SHAPE
    //                 if (DEBUGCOND) {
    //                     std::cout << "  e2=" << e2->getID() << "\n";
    //                 }
    // #endif
    //                 auto e2It = std::find(all.begin(), all.end(), e2);
    //                 if (e2It + 1 == bestCCW || (e2It == (all.end() - 1) && bestCCW == all.begin())) {
    //                     bestCCW = e2It;
    //                     changed = true;
    // #ifdef DEBUG_NODE_SHAPE
    //                     if (DEBUGCOND) {
    //                         std::cout << "    bestCCW=" << e2->getID() << "\n";
    //                     }
    // #endif
    //                 } else if (bestCW + 1 == e2It || (bestCW == (all.end() - 1) && e2It == all.begin())) {
    //                     bestCW = e2It;
    //                     changed = true;
    // #ifdef DEBUG_NODE_SHAPE
    //                     if (DEBUGCOND) {
    //                         std::cout << "    bestCW=" << e2->getID() << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    //         if (bestCW != e1It) {
    //             geomsCW[e1] = geomsCW[*bestCW];
    //             computeSameEnd(geomsCW[e1], geomsCCW[e1]);
    //         }
    //         if (bestCCW != e1It) {
    //             geomsCCW[e1] = geomsCCW[*bestCCW];
    //             computeSameEnd(geomsCW[e1], geomsCCW[e1]);
    //         }
    //         // clean up
    //         for (NBEdge* e2 : same[e1]) {
    //             auto e2NewAllIt = std::find(newAll.begin(), newAll.end(), e2);
    //             if (e2NewAllIt != newAll.end()) {
    //                 newAll.erase(e2NewAllIt);
    //             }
    //         }
    //     }
    // #ifdef DEBUG_NODE_SHAPE
    //     if (DEBUGCOND) {
    //         std::cout << "  newAll:\n";
    //         for (NBEdge* e : newAll) {
    //             std::cout << "    " << e->getID() << " geomCCW=" << geomsCCW[e] << " geomsCW=" << geomsCW[e] << "\n";
    //         }
    //     }
    // #endif
    //     return newAll;
    // }
    #[allow(unused_variables)]
    pub fn compute_unique_direction_list(&mut self, all: &EdgeVector, same: &mut BTreeMap<Ptr<NBEdge>, BTreeSet<Ptr<NBEdge>>>, geomsCCW: &mut NBNodeShapeComputer_GeomsMap, geomsCW: &mut NBNodeShapeComputer_GeomsMap) -> EdgeVector {
        // store relationships
        let mut newAll: EdgeVector = all.clone();
        let n = all.len();
        for &e1 in all {
            // determine which of the edges marks the outer boundary
            let e2NewAll = newAll.iter().position(|&x| x == e1);
            if e2NewAll.is_none() {
                continue;
            }
            // iterators into `all` are modelled as indices; `n` is `all.end()`
            let e1It = all.iter().position(|&x| x == e1).unwrap_or(n);
            let mut bestCCW = e1It;
            let mut bestCW = e1It;
            let mut changed = true;
            // determinism: C++ visits same[e1] in ComparatorIdLess (ID) order, so the snapshot is sorted by ID
            let mut sameE1: Vec<Ptr<NBEdge>> = same.get(&e1).map(|s| s.iter().copied().collect()).unwrap_or_default();
            sameE1.sort_by(|a, b| a.get_id().cmp(b.get_id()));
            while changed {
                changed = false;
                for &e2 in &sameE1 {
                    let e2It = all.iter().position(|&x| x == e2).unwrap_or(n);
                    if e2It + 1 == bestCCW || (e2It == n - 1 && bestCCW == 0) {
                        bestCCW = e2It;
                        changed = true;
                    } else if bestCW + 1 == e2It || (bestCW == n - 1 && e2It == 0) {
                        bestCW = e2It;
                        changed = true;
                    }
                }
            }
            if bestCW != e1It {
                let v = geomsCW.entry(all[bestCW]).or_default().clone();
                geomsCW.insert(e1, v);
                let mut cw = geomsCW.entry(e1).or_default().clone();
                let mut ccw = geomsCCW.entry(e1).or_default().clone();
                self.compute_same_end(&mut cw, &mut ccw);
                geomsCW.insert(e1, cw);
                geomsCCW.insert(e1, ccw);
            }
            if bestCCW != e1It {
                let v = geomsCCW.entry(all[bestCCW]).or_default().clone();
                geomsCCW.insert(e1, v);
                let mut cw = geomsCW.entry(e1).or_default().clone();
                let mut ccw = geomsCCW.entry(e1).or_default().clone();
                self.compute_same_end(&mut cw, &mut ccw);
                geomsCW.insert(e1, cw);
                geomsCCW.insert(e1, ccw);
            }
            // clean up
            for e2 in sameE1 {
                if let Some(pos) = newAll.iter().position(|&x| x == e2) {
                    newAll.remove(pos);
                }
            }
        }
        newAll
    }

    /// C++: `PositionVector NBNodeShapeComputer::getSmoothCorner(PositionVector, PositionVector, const Position &, const Position &, int)` [private]
    /// Compute smoothed corner shape
    // PositionVector
    // NBNodeShapeComputer::getSmoothCorner(PositionVector begShape, PositionVector endShape,
    //                                      const Position& begPoint, const Position& endPoint, int cornerDetail) {
    //     PositionVector ret;
    //     if (cornerDetail > 0) {
    //         PositionVector begShape2 = begShape.reverse().getSubpart2D(EXT2, begShape.length());
    //         const double begSplit = begShape2.nearest_offset_to_point2D(begPoint, false);
    // #ifdef DEBUG_SMOOTH_CORNERS
    //         if (DEBUGCOND) {
    //             std::cout << " begLength=" << begShape2.length2D() << " begSplit=" << begSplit << "\n";
    //         }
    // #endif
    //         if (begSplit > POSITION_EPS && begSplit < begShape2.length2D() - POSITION_EPS) {
    //             begShape2 = begShape2.splitAt(begSplit, true).first;
    //         } else {
    //             return ret;
    //         }
    //         PositionVector endShape2 = endShape.getSubpart(0, endShape.length() - EXT2);
    //         const double endSplit = endShape2.nearest_offset_to_point2D(endPoint, false);
    // #ifdef DEBUG_SMOOTH_CORNERS
    //         if (DEBUGCOND) {
    //             std::cout << " endLength=" << endShape2.length2D() << " endSplit=" << endSplit << "\n";
    //         }
    // #endif
    //         if (endSplit > POSITION_EPS && endSplit < endShape2.length2D() - POSITION_EPS) {
    //             endShape2 = endShape2.splitAt(endSplit, true).second;
    //         } else {
    //             return ret;
    //         }
    //         // flatten z to junction z level
    //         begShape2 = begShape2.interpolateZ(myNode.getPosition().z(), myNode.getPosition().z());
    //         endShape2 = endShape2.interpolateZ(myNode.getPosition().z(), myNode.getPosition().z());
    // #ifdef DEBUG_SMOOTH_CORNERS
    //         if (DEBUGCOND) {
    //             std::cout << "getSmoothCorner begPoint=" << begPoint << " endPoint=" << endPoint
    //                       << " begShape=" << begShape << " endShape=" << endShape
    //                       << " begShape2=" << begShape2 << " endShape2=" << endShape2
    //                       << "\n";
    //         }
    // #endif
    //         if (begShape2.size() < 2 || endShape2.size() < 2) {
    //             return ret;
    //         }
    //         const double angle = GeomHelper::angleDiff(begShape2.angleAt2D(-2), endShape2.angleAt2D(0));
    //         NBNode* recordError = nullptr;
    // #ifdef DEBUG_SMOOTH_CORNERS
    //         if (DEBUGCOND) {
    //             std::cout << "   angle=" << RAD2DEG(angle) << "\n";
    //         }
    //         recordError = const_cast<NBNode*>(&myNode);
    // #endif
    //         // fill highly acute corners
    //         //if (fabs(angle) > DEG2RAD(135)) {
    //         //    return ret;
    //         //}
    //         PositionVector curve = myNode.computeSmoothShape(begShape2, endShape2, cornerDetail + 2, false, 25, 25, recordError, NBNode::AVOID_WIDE_LEFT_TURN);
    //         //PositionVector curve = myNode.computeSmoothShape(begShape2, endShape2, cornerDetail + 2, false, 25, 25, recordError, 0);
    //         const double curvature = curve.length2D() / MAX2(NUMERICAL_EPS, begPoint.distanceTo2D(endPoint));
    // #ifdef DEBUG_SMOOTH_CORNERS
    //         if (DEBUGCOND) {
    //             std::cout << "   curve=" << curve << " curveLength=" << curve.length2D() << " dist=" << begPoint.distanceTo2D(endPoint) << " curvature=" << curvature << "\n";
    //         }
    // #endif
    //         if (curvature > 2 && angle > DEG2RAD(85)) {
    //             // simplify dubious inside corner shape
    //             return ret;
    //         }
    //         if (curve.size() > 2) {
    //             curve.erase(curve.begin());
    //             curve.pop_back();
    //             ret = curve;
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_smooth_corner(&mut self, begShape: PositionVector, endShape: PositionVector, begPoint: &Position, endPoint: &Position, cornerDetail: i32) -> PositionVector {
        let node = self.myNode;
        let mut ret = PositionVector::new();
        if cornerDetail > 0 {
            let mut begShape2 = begShape.reverse().get_subpart2d(EXT2, begShape.length());
            let begSplit = begShape2.nearest_offset_to_point2d(begPoint, false);
            if begSplit > POSITION_EPS && begSplit < begShape2.length2d() - POSITION_EPS {
                begShape2 = begShape2.split_at(begSplit, true).unwrap().0;
            } else {
                return ret;
            }
            let mut endShape2 = endShape.get_subpart(0.0, endShape.length() - EXT2);
            let endSplit = endShape2.nearest_offset_to_point2d(endPoint, false);
            if endSplit > POSITION_EPS && endSplit < endShape2.length2d() - POSITION_EPS {
                endShape2 = endShape2.split_at(endSplit, true).unwrap().1;
            } else {
                return ret;
            }
            // flatten z to junction z level
            begShape2 = begShape2.interpolate_z(node.get_position().z(), node.get_position().z());
            endShape2 = endShape2.interpolate_z(node.get_position().z(), node.get_position().z());
            if begShape2.len() < 2 || endShape2.len() < 2 {
                return ret;
            }
            let angle = utils::geom::geom_helper::angle_diff(begShape2.angle_at2d(-2), endShape2.angle_at2d(0));
            // DEBUG_SMOOTH_CORNERS only sets recordError; the release path passes nullptr
            let recordError: Ptr<NBNode> = Ptr::null();
            // fill highly acute corners
            //if (fabs(angle) > DEG2RAD(135)) {
            //    return ret;
            //}
            let mut curve = node.compute_smooth_shape(&begShape2, &endShape2, cornerDetail + 2, false, 25.0, 25.0, recordError, NBNode_AVOID_WIDE_LEFT_TURN);
            //PositionVector curve = myNode.computeSmoothShape(begShape2, endShape2, cornerDetail + 2, false, 25, 25, recordError, 0);
            let curvature = curve.length2d() / max2(NUMERICAL_EPS, begPoint.distance_to2d(endPoint));
            if curvature > 2.0 && angle > utils::geom::geom_helper::deg2rad(85.0) {
                // simplify dubious inside corner shape
                return ret;
            }
            if curve.len() > 2 {
                curve.remove(0);
                curve.pop();
                ret = curve;
            }
        }
        ret
    }

    /// C++: `void NBNodeShapeComputer::initNeighbors(const EdgeVector &, const EdgeVector::const_iterator &, GeomsMap &, GeomsMap &, EdgeVector::const_iterator &, EdgeVector::const_iterator &, double &, double &)` [static, private]
    /// Initialize neighbors and angles
    // void
    // NBNodeShapeComputer::initNeighbors(const EdgeVector& edges, const EdgeVector::const_iterator& current,
    //                                    GeomsMap& geomsCW,
    //                                    GeomsMap& geomsCCW,
    //                                    EdgeVector::const_iterator& cwi,
    //                                    EdgeVector::const_iterator& ccwi,
    //                                    double& cad,
    //                                    double& ccad) {
    //     const double twoPI = (double)(2 * M_PI);
    //     cwi = current;
    //     cwi++;
    //     if (cwi == edges.end()) {
    //         std::advance(cwi, -((int)edges.size())); // set to edges.begin();
    //     }
    //     ccwi = current;
    //     if (ccwi == edges.begin()) {
    //         std::advance(ccwi, edges.size() - 1); // set to edges.end() - 1;
    //     } else {
    //         ccwi--;
    //     }
    //
    //     const double angleCurCCW = geomsCCW[*current].angleAt2D(0);
    //     const double angleCurCW = geomsCW[*current].angleAt2D(0);
    //     const double angleCCW = geomsCW[*ccwi].angleAt2D(0);
    //     const double angleCW = geomsCCW[*cwi].angleAt2D(0);
    //     ccad = angleCCW - angleCurCCW;
    //     while (ccad < 0.) {
    //         ccad += twoPI;
    //     }
    //     cad = angleCurCW - angleCW;
    //     while (cad < 0.) {
    //         cad += twoPI;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_neighbors(edges: &EdgeVector, current: usize, geomsCW: &mut NBNodeShapeComputer_GeomsMap, geomsCCW: &mut NBNodeShapeComputer_GeomsMap, cwi: &mut usize, ccwi: &mut usize, cad: &mut f64, ccad: &mut f64) {
        // iterators into `edges` are modelled as indices (the C++ iterator arithmetic becomes index arithmetic)
        let twoPI: f64 = 2.0 * std::f64::consts::PI;
        *cwi = current;
        *cwi += 1;
        if *cwi == edges.len() {
            *cwi = 0; // set to edges.begin();
        }
        *ccwi = current;
        if *ccwi == 0 {
            *ccwi = edges.len() - 1; // set to edges.end() - 1;
        } else {
            *ccwi -= 1;
        }
        let angleCurCCW = geomsCCW.entry(edges[current]).or_default().angle_at2d(0);
        let angleCurCW = geomsCW.entry(edges[current]).or_default().angle_at2d(0);
        let angleCCW = geomsCW.entry(edges[*ccwi]).or_default().angle_at2d(0);
        let angleCW = geomsCCW.entry(edges[*cwi]).or_default().angle_at2d(0);
        *ccad = angleCCW - angleCurCCW;
        while *ccad < 0.0 {
            *ccad += twoPI;
        }
        *cad = angleCurCW - angleCW;
        while *cad < 0.0 {
            *cad += twoPI;
        }
    }

    /// C++: `bool NBNodeShapeComputer::badIntersection(const NBEdge *, const NBEdge *, double)` [private]
    /// Returns whether trying to intersect these edges would probably fail
    // bool
    // NBNodeShapeComputer::badIntersection(const NBEdge* e1, const NBEdge* e2, double distance) {
    //     // check whether the two edges are on top of each other. In that case they should be joined
    //     // also, if they never touch along their common length
    //     const double commonLength = MIN3(distance, e1->getGeometry().length(), e2->getGeometry().length());
    //     PositionVector geom1 = e1->getGeometry();
    //     PositionVector geom2 = e2->getGeometry();
    //     // shift to make geom the centerline of the edge regardless of spreadtype
    //     if (e1->getLaneSpreadFunction() == LaneSpreadFunction::RIGHT) {
    //         geom1.move2side(e1->getTotalWidth() / 2);
    //     }
    //     if (e2->getLaneSpreadFunction() == LaneSpreadFunction::RIGHT) {
    //         geom2.move2side(e2->getTotalWidth() / 2);
    //     }
    //     // always let geometry start at myNode
    //     if (e1->getToNode() == &myNode) {
    //         geom1 = geom1.reverse();
    //     }
    //     if (e2->getToNode() == &myNode) {
    //         geom2 = geom2.reverse();
    //     }
    //     geom1 = geom1.getSubpart2D(0, commonLength);
    //     geom2 = geom2.getSubpart2D(0, commonLength);
    //     double endAngleDiff = 0;
    //     if (geom1.size() >= 2 && geom2.size() >= 2) {
    //         endAngleDiff = fabs(RAD2DEG(GeomHelper::angleDiff(
    //                                         geom1.angleAt2D((int)geom1.size() - 2),
    //                                         geom2.angleAt2D((int)geom2.size() - 2))));
    //     }
    //     const double minDistanceThreshold = (e1->getTotalWidth() + e2->getTotalWidth()) / 2 + POSITION_EPS;
    //     std::vector<double> distances = geom1.distances(geom2, true);
    //     std::vector<double> distances2 = geom1.distances(geom2);
    //     const double minDist = VectorHelper<double>::minValue(distances2);
    //     const double maxDist = VectorHelper<double>::maxValue(distances);
    //     const bool curvingTowards = geom1[0].distanceTo2D(geom2[0]) > minDistanceThreshold && minDist < minDistanceThreshold;
    //     const bool onTop = (maxDist - POSITION_EPS < minDistanceThreshold) && endAngleDiff < 30;
    //     const bool bothDefault = e1->hasDefaultGeometryEndpointAtNode(&myNode) && e2->hasDefaultGeometryEndpointAtNode(&myNode);
    //     const bool neverTouch = minDist > minDistanceThreshold * 2 && !bothDefault;
    //     geom1.extrapolate2D(EXT);
    //     geom2.extrapolate2D(EXT);
    //     Position intersect = geom1.intersectionPosition2D(geom2);
    //     const bool intersects = intersect != Position::INVALID && geom1.distance2D(intersect) < POSITION_EPS;
    // #ifdef DEBUG_NODE_SHAPE
    //     if (DEBUGCOND) {
    //         std::cout << "    badIntersect: onTop=" << onTop << " curveTo=" << curvingTowards << " intersects=" << intersects
    //                   << " endAngleDiff=" << endAngleDiff
    //                   << " geom1=" << geom1 << " geom2=" << geom2
    //                   << " distances=" << toString(distances) << " minDist=" << minDist << " maxDist=" << maxDist << " thresh=" << minDistanceThreshold
    //                   << " neverTouch=" << neverTouch
    //                   << " intersectPos=" << intersect
    //                   << "\n";
    //     }
    // #endif
    //     return onTop || curvingTowards || !intersects || neverTouch;
    // }
    #[allow(unused_variables)]
    pub fn bad_intersection(&mut self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>, distance: f64) -> bool {
        let node = self.myNode;
        let ext = self.EXT;
        // check whether the two edges are on top of each other. In that case they should be joined
        // also, if they never touch along their common length
        let commonLength = min3(distance, e1.get_geometry().length(), e2.get_geometry().length());
        let mut geom1 = e1.get_geometry().clone();
        let mut geom2 = e2.get_geometry().clone();
        // shift to make geom the centerline of the edge regardless of spreadtype
        if e1.get_lane_spread_function() == LaneSpreadFunction::RIGHT {
            geom1.move2side(e1.get_total_width() / 2.0, 100.0);
        }
        if e2.get_lane_spread_function() == LaneSpreadFunction::RIGHT {
            geom2.move2side(e2.get_total_width() / 2.0, 100.0);
        }
        // always let geometry start at myNode
        if e1.get_to_node() == node {
            geom1 = geom1.reverse();
        }
        if e2.get_to_node() == node {
            geom2 = geom2.reverse();
        }
        geom1 = geom1.get_subpart2d(0.0, commonLength);
        geom2 = geom2.get_subpart2d(0.0, commonLength);
        let mut endAngleDiff = 0.0;
        if geom1.len() >= 2 && geom2.len() >= 2 {
            endAngleDiff = utils::geom::geom_helper::rad2deg(utils::geom::geom_helper::angle_diff(
                geom1.angle_at2d(geom1.len() as i32 - 2),
                geom2.angle_at2d(geom2.len() as i32 - 2),
            ))
            .abs();
        }
        let minDistanceThreshold = (e1.get_total_width() + e2.get_total_width()) / 2.0 + POSITION_EPS;
        let distances = geom1.distances(&geom2, true);
        let distances2 = geom1.distances(&geom2, false);
        // VectorHelper<double>::minValue / maxValue
        let minDist = distances2.iter().fold(f64::MAX, |m, &x| if x < m { x } else { m });
        let maxDist = distances.iter().fold(-f64::MAX, |m, &x| if x > m { x } else { m });
        let curvingTowards = geom1[0].distance_to2d(&geom2[0]) > minDistanceThreshold && minDist < minDistanceThreshold;
        let onTop = (maxDist - POSITION_EPS < minDistanceThreshold) && endAngleDiff < 30.0;
        let bothDefault = e1.has_default_geometry_endpoint_at_node(node) && e2.has_default_geometry_endpoint_at_node(node);
        let neverTouch = minDist > minDistanceThreshold * 2.0 && !bothDefault;
        geom1.extrapolate2d(ext, false);
        geom2.extrapolate2d(ext, false);
        let intersect = geom1.intersection_position2d_pv(&geom2);
        let intersects = intersect != Position::INVALID && geom1.distance2d(&intersect, false) < POSITION_EPS;
        onTop || curvingTowards || !intersects || neverTouch
    }

    /// C++: `double NBNodeShapeComputer::closestIntersection(const PositionVector &, const PositionVector &, double)` [private]
    /// return the intersection point closest to the given offset
    // double
    // NBNodeShapeComputer::closestIntersection(const PositionVector& geom1, const PositionVector& geom2, double offset) {
    //     std::vector<double> intersections = geom1.intersectsAtLengths2D(geom2);
    //     double result = intersections[0];
    //     for (std::vector<double>::iterator it = intersections.begin() + 1; it != intersections.end(); ++it) {
    //         if (fabs(*it - offset) < fabs(result - offset)) {
    //             result = *it;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn closest_intersection(&mut self, geom1: &PositionVector, geom2: &PositionVector, offset: f64) -> f64 {
        let intersections = geom1.intersects_at_lengths2d_pv(geom2);
        let mut result = intersections[0];
        for &it in &intersections[1..] {
            if (it - offset).abs() < (result - offset).abs() {
                result = it;
            }
        }
        result
    }

    /// C++: `bool NBNodeShapeComputer::needsLargeTurn(NBEdge *, NBEdge *, std::map<NBEdge *, std::set<NBEdge *, ComparatorIdLess>> &) const` [const, private]
    /// whether the given edges (along with those in the same direction) requires a large turning radius
    // bool
    // NBNodeShapeComputer::needsLargeTurn(NBEdge* e1, NBEdge* e2,
    //                                     std::map<NBEdge*, std::set<NBEdge*, ComparatorIdLess> >& same) const {
    //     const SVCPermissions p1 = e1->getPermissions();
    //     const SVCPermissions p2 = e2->getPermissions();
    //     if ((p1 & p2 & SVC_LARGE_TURN) != 0) {
    //         // note: would could also check whether there is actually a connection
    //         // between those edges
    //         return true;
    //     }
    //     // maybe edges in the same direction need a large turn
    //     for (NBEdge* e2s : same[e2]) {
    //         if ((p1 & e2s->getPermissions() & SVC_LARGE_TURN) != 0
    //                 && (e1->getToNode() == e2s->getFromNode() || e2s->getToNode() == e1->getFromNode())) {
    //             return true;
    //         }
    //         for (NBEdge* e1s : same[e1]) {
    //             if ((e2s->getPermissions() & e1s->getPermissions() & SVC_LARGE_TURN) != 0
    //                     && (e2s->getToNode() == e1s->getFromNode() || e1s->getToNode() == e2s->getFromNode())) {
    //                 return true;
    //             }
    //         }
    //     }
    //     for (NBEdge* e1s : same[e1]) {
    //         if ((p2 & e1s->getPermissions() & SVC_LARGE_TURN) != 0
    //                 && (e2->getToNode() == e1s->getFromNode() || e1s->getToNode() == e2->getFromNode())) {
    //             return true;
    //         }
    //     }
    //     //std::cout << " e1=" << e1->getID() << " e2=" << e2->getID() << " sameE1=" << toString(same[e1]) << " sameE2=" << toString(same[e2]) << "\n";
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn needs_large_turn(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>, same: &mut BTreeMap<Ptr<NBEdge>, BTreeSet<Ptr<NBEdge>>>) -> bool {
        let p1 = e1.get_permissions(-1);
        let p2 = e2.get_permissions(-1);
        if (p1 & p2 & SVC_LARGE_TURN) != 0 {
            // note: would could also check whether there is actually a connection
            // between those edges
            return true;
        }
        // maybe edges in the same direction need a large turn
        // (only a boolean "any" is computed here, so the iteration order does not matter)
        let same_e2: Vec<Ptr<NBEdge>> = same.get(&e2).map(|s| s.iter().copied().collect()).unwrap_or_default();
        for e2s in same_e2 {
            if (p1 & e2s.get_permissions(-1) & SVC_LARGE_TURN) != 0 && (e1.get_to_node() == e2s.get_from_node() || e2s.get_to_node() == e1.get_from_node()) {
                return true;
            }
            let same_e1: Vec<Ptr<NBEdge>> = same.get(&e1).map(|s| s.iter().copied().collect()).unwrap_or_default();
            for e1s in same_e1 {
                if (e2s.get_permissions(-1) & e1s.get_permissions(-1) & SVC_LARGE_TURN) != 0
                    && (e2s.get_to_node() == e1s.get_from_node() || e1s.get_to_node() == e2s.get_from_node())
                {
                    return true;
                }
            }
        }
        let same_e1: Vec<Ptr<NBEdge>> = same.get(&e1).map(|s| s.iter().copied().collect()).unwrap_or_default();
        for e1s in same_e1 {
            if (p2 & e1s.get_permissions(-1) & SVC_LARGE_TURN) != 0 && (e2.get_to_node() == e1s.get_from_node() || e1s.get_to_node() == e2.get_from_node()) {
                return true;
            }
        }
        false
    }

    /// C++: `double NBNodeShapeComputer::getDefaultRadius(const OptionsCont &)` [private]
    /// determine the default radius appropriate for the current junction
    // double
    // NBNodeShapeComputer::getDefaultRadius(const OptionsCont& oc) {
    //     // look for incoming/outgoing edge pairs that do not go straight and allow wide vehicles
    //     // (connection information is not available yet)
    //     // @TODO compute the radius for each pair of neighboring edge intersections in computeNodeShapeDefault rather than use the maximum
    //     const double radius = oc.getFloat("default.junctions.radius");
    //     const double smallRadius = oc.getFloat("junctions.small-radius");
    //     double maxRightAngle = 0; // rad
    //     double extraWidthRight = 0; // m
    //     double maxLeftAngle = 0; // rad
    //     double extraWidthLeft = 0; // m
    //     int laneDelta = 0;
    //     int totalWideLanesIn = 0;
    //     for (NBEdge* in : myNode.getIncomingEdges()) {
    //         int wideLanesIn = 0;
    //         for (int i = 0; i < in->getNumLanes(); i++) {
    //             if ((in->getPermissions(i) & SVC_LARGE_TURN) != 0) {
    //                 wideLanesIn++;
    //             }
    //         }
    //         totalWideLanesIn += wideLanesIn;
    //         for (NBEdge* out : myNode.getOutgoingEdges()) {
    //             if ((in->getPermissions() & out->getPermissions() & SVC_LARGE_TURN) != 0) {
    //                 if (myNode.getDirection(in, out) == LinkDirection::TURN) {
    //                     continue;
    //                 };
    //                 const double angle = GeomHelper::angleDiff(
    //                                          in->getGeometry().angleAt2D(-2),
    //                                          out->getGeometry().angleAt2D(0));
    //                 if (angle < 0) {
    //                     if (maxRightAngle < -angle) {
    //                         maxRightAngle = -angle;
    //                         extraWidthRight = MAX2(getExtraWidth(in, SVC_LARGE_TURN), getExtraWidth(out, SVC_LARGE_TURN));
    //                     }
    //                 } else {
    //                     if (maxLeftAngle < angle) {
    //                         maxLeftAngle = angle;
    //                         // all edges clockwise between in and out count as extra width
    //                         extraWidthLeft = 0;
    //                         EdgeVector::const_iterator pIn = std::find(myNode.getEdges().begin(), myNode.getEdges().end(), in);
    //                         NBContHelper::nextCW(myNode.getEdges(), pIn);
    //                         while (*pIn != out) {
    //                             extraWidthLeft += (*pIn)->getTotalWidth();
    // #ifdef DEBUG_RADIUS
    //                             if (DEBUGCOND) {
    //                                 std::cout << "   in=" << in->getID() << " out=" << out->getID() << " extra=" << (*pIn)->getID() << " extraWidthLeft=" << extraWidthLeft << "\n";
    //                             }
    // #endif
    //                             NBContHelper::nextCW(myNode.getEdges(), pIn);
    //                         }
    //                     }
    //                 }
    //                 int wideLanesOut = 0;
    //                 for (int i = 0; i < out->getNumLanes(); i++) {
    //                     if ((out->getPermissions(i) & SVC_LARGE_TURN) != 0) {
    //                         wideLanesOut++;
    //                     }
    //                 }
    // #ifdef DEBUG_RADIUS
    //                 if (DEBUGCOND) {
    //                     std::cout << "   in=" << in->getID() << " out=" << out->getID() << " wideLanesIn=" << wideLanesIn << " wideLanesOut=" << wideLanesOut << "\n";
    //                 }
    // #endif
    //                 laneDelta = MAX2(laneDelta, abs(wideLanesOut - wideLanesIn));
    //             }
    //         }
    //     }
    //     // special case: on/off-ramp
    //     if (myNode.getOutgoingEdges().size() == 1 || myNode.getIncomingEdges().size() == 1) {
    //         int totalWideLanesOut = 0;
    //         for (NBEdge* out : myNode.getOutgoingEdges()) {
    //             for (int i = 0; i < out->getNumLanes(); i++) {
    //                 if ((out->getPermissions(i) & SVC_LARGE_TURN) != 0) {
    //                     totalWideLanesOut++;
    //                 }
    //             }
    //         }
    //         if (totalWideLanesIn == totalWideLanesOut) {
    //             // use total laneDelta instead of individual edge lane delta
    //             laneDelta = 0;
    //         }
    //     }
    //     // changing the number of wide-vehicle lanes on a straight segment requires a larger junction to allow for smooth driving
    //     // otherwise we can reduce the radius according to the angle
    //     double result = radius;
    //     // left turns are assumed to cross additional edges and thus du not determine the required radius in most cases
    //     double maxTurnAngle = maxRightAngle;
    //     double extraWidth = extraWidthRight;
    //     if (maxRightAngle < DEG2RAD(5)) {
    //         maxTurnAngle = maxLeftAngle;
    //         extraWidth = extraWidthLeft;
    //     }
    //     const double minRadius = maxTurnAngle >= DEG2RAD(30) ? MIN2(smallRadius, radius) : smallRadius;
    //     if (laneDelta == 0 || maxTurnAngle >= DEG2RAD(30) || myNode.isConstantWidthTransition()) {
    //         // subtract radius gained from extra lanes
    //         // do not increase radius for turns that are sharper than a right angle
    //         result = radius * tan(0.5 * MIN2(0.5 * M_PI, maxTurnAngle)) - extraWidth;
    //     }
    //     result = MAX2(minRadius, result);
    // #ifdef DEBUG_RADIUS
    //     if (DEBUGCOND) {
    //         std::cout << "getDefaultRadius n=" << myNode.getID()
    //                   << " r=" << radius << " sr=" << smallRadius
    //                   << " mr=" << minRadius
    //                   << " laneDelta=" << laneDelta
    //                   << " rightA=" << RAD2DEG(maxRightAngle)
    //                   << " leftA=" << RAD2DEG(maxLeftAngle)
    //                   << " maxA=" << RAD2DEG(maxTurnAngle)
    //                   << " extraWidth=" << extraWidth
    //                   << " result=" << result << "\n";
    //     }
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_default_radius(&mut self, oc: &OptionsCont) -> f64 {
        let node = self.myNode;
        // look for incoming/outgoing edge pairs that do not go straight and allow wide vehicles
        // (connection information is not available yet)
        // @TODO compute the radius for each pair of neighboring edge intersections in computeNodeShapeDefault rather than use the maximum
        let radius = oc.get_float("default.junctions.radius").unwrap();
        let smallRadius = oc.get_float("junctions.small-radius").unwrap();
        let mut maxRightAngle = 0.0; // rad
        let mut extraWidthRight = 0.0; // m
        let mut maxLeftAngle = 0.0; // rad
        let mut extraWidthLeft = 0.0; // m
        let mut laneDelta: i32 = 0;
        let mut totalWideLanesIn: i32 = 0;
        for &in_ in node.get_incoming_edges() {
            let mut wideLanesIn: i32 = 0;
            for i in 0..in_.get_num_lanes() {
                if (in_.get_permissions(i) & SVC_LARGE_TURN) != 0 {
                    wideLanesIn += 1;
                }
            }
            totalWideLanesIn += wideLanesIn;
            for &out in node.get_outgoing_edges() {
                if (in_.get_permissions(-1) & out.get_permissions(-1) & SVC_LARGE_TURN) != 0 {
                    if node.get_direction(in_, out, false) == LinkDirection::TURN {
                        continue;
                    }
                    let angle = utils::geom::geom_helper::angle_diff(in_.get_geometry().angle_at2d(-2), out.get_geometry().angle_at2d(0));
                    if angle < 0.0 {
                        if maxRightAngle < -angle {
                            maxRightAngle = -angle;
                            extraWidthRight = max2(Self::get_extra_width(in_, SVC_LARGE_TURN), Self::get_extra_width(out, SVC_LARGE_TURN));
                        }
                    } else if maxLeftAngle < angle {
                        maxLeftAngle = angle;
                        // all edges clockwise between in and out count as extra width
                        extraWidthLeft = 0.0;
                        let edges = node.get_edges();
                        let mut pIn = edges.iter().position(|&x| x == in_).unwrap_or(edges.len());
                        // NBContHelper::nextCW inlined: its iterator signature is not ported yet
                        pIn = if pIn + 1 == edges.len() { 0 } else { pIn + 1 };
                        while edges[pIn] != out {
                            extraWidthLeft += edges[pIn].get_total_width();
                            pIn = if pIn + 1 == edges.len() { 0 } else { pIn + 1 };
                        }
                    }
                    let mut wideLanesOut: i32 = 0;
                    for i in 0..out.get_num_lanes() {
                        if (out.get_permissions(i) & SVC_LARGE_TURN) != 0 {
                            wideLanesOut += 1;
                        }
                    }
                    laneDelta = std::cmp::max(laneDelta, (wideLanesOut - wideLanesIn).abs());
                }
            }
        }
        // special case: on/off-ramp
        if node.get_outgoing_edges().len() == 1 || node.get_incoming_edges().len() == 1 {
            let mut totalWideLanesOut: i32 = 0;
            for &out in node.get_outgoing_edges() {
                for i in 0..out.get_num_lanes() {
                    if (out.get_permissions(i) & SVC_LARGE_TURN) != 0 {
                        totalWideLanesOut += 1;
                    }
                }
            }
            if totalWideLanesIn == totalWideLanesOut {
                // use total laneDelta instead of individual edge lane delta
                laneDelta = 0;
            }
        }
        // changing the number of wide-vehicle lanes on a straight segment requires a larger junction to allow for smooth driving
        // otherwise we can reduce the radius according to the angle
        let mut result = radius;
        // left turns are assumed to cross additional edges and thus du not determine the required radius in most cases
        let mut maxTurnAngle = maxRightAngle;
        let mut extraWidth = extraWidthRight;
        if maxRightAngle < utils::geom::geom_helper::deg2rad(5.0) {
            maxTurnAngle = maxLeftAngle;
            extraWidth = extraWidthLeft;
        }
        let minRadius = if maxTurnAngle >= utils::geom::geom_helper::deg2rad(30.0) { min2(smallRadius, radius) } else { smallRadius };
        if laneDelta == 0 || maxTurnAngle >= utils::geom::geom_helper::deg2rad(30.0) || node.is_constant_width_transition() {
            // subtract radius gained from extra lanes
            // do not increase radius for turns that are sharper than a right angle
            result = radius * (0.5 * min2(0.5 * std::f64::consts::PI, maxTurnAngle)).tan() - extraWidth;
        }
        result = max2(minRadius, result);
        result
    }

    /// C++: `void NBNodeShapeComputer::computeSameEnd(PositionVector &, PositionVector &)` [private]
    // void
    // NBNodeShapeComputer::computeSameEnd(PositionVector& l1, PositionVector& l2) {
    //     assert(l1[0].distanceTo2D(l1[1]) >= EXT);
    //     assert(l2[0].distanceTo2D(l2[1]) >= EXT);
    //     PositionVector tmp;
    //     tmp.push_back(PositionVector::positionAtOffset2D(l1[0], l1[1], EXT));
    //     tmp.push_back(l1[1]);
    //     tmp[1].sub(tmp[0]);
    //     tmp[1].set(-tmp[1].y(), tmp[1].x());
    //     tmp[1].add(tmp[0]);
    //     tmp.extrapolate2D(EXT);
    //     if (l2.intersects(tmp[0], tmp[1])) {
    //         const double offset = l2.intersectsAtLengths2D(tmp)[0];
    //         if (l2.length2D() - offset > POSITION_EPS) {
    //             PositionVector tl2 = l2.getSubpart2D(offset, l2.length2D());
    //             tl2.extrapolate2D(EXT);
    //             l2.erase(l2.begin(), l2.begin() + (l2.size() - tl2.size()));
    //             l2[0] = tl2[0];
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_same_end(&mut self, l1: &mut PositionVector, l2: &mut PositionVector) {
        let ext = self.EXT;
        // assert(l1[0].distanceTo2D(l1[1]) >= EXT) and the matching assert on l2 are debug-only checks
        let mut tmp = PositionVector::new();
        tmp.push(PositionVector::position_at_offset2d_seg(&l1[0], &l1[1], ext, 0.0, false));
        tmp.push(l1[1]);
        let t0 = tmp[0];
        tmp[1].sub(&t0);
        let (x1, y1) = (tmp[1].x(), tmp[1].y());
        tmp[1].set(-y1, x1); // rotate 90 degrees
        tmp[1].add(&t0);
        tmp.extrapolate2d(ext, false);
        if l2.intersects(&tmp[0], &tmp[1]) {
            let offset = l2.intersects_at_lengths2d_pv(&tmp)[0];
            if l2.length2d() - offset > POSITION_EPS {
                let mut tl2 = l2.get_subpart2d(offset, l2.length2d());
                tl2.extrapolate2d(ext, false);
                let k = l2.len() - tl2.len();
                l2.drain(..k);
                l2[0] = tl2[0];
            }
        }
    }

    /// C++: `bool NBNodeShapeComputer::isDivided(const NBEdge *, std::set<NBEdge *, ComparatorIdLess>, const PositionVector &, const PositionVector &) const` [const, private]
    // bool
    // NBNodeShapeComputer::isDivided(const NBEdge* e, std::set<NBEdge*, ComparatorIdLess> same, const PositionVector& ccw, const PositionVector& cw) const {
    //     if (same.size() < 2) {
    //         return false;
    //     }
    //     std::set<Position> endPoints;
    //     endPoints.insert(e->getEndpointAtNode(&myNode));
    //     for (NBEdge* s : same) {
    //         endPoints.insert(s->getEndpointAtNode(&myNode));
    //     }
    //     if (endPoints.size() > 1) {
    //         std::vector<double> distances = ccw.distances(cw, true);
    //         double width = e->getTotalWidth();
    //         for (const NBEdge* e2 : same) {
    //             width += e2->getTotalWidth();
    //         }
    //         const double maxDist = VectorHelper<double>::maxValue(distances);
    //         const double maxDivider = maxDist - width;
    //         return maxDivider >= 5;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_divided(&self, e: Ptr<NBEdge>, same: BTreeSet<Ptr<NBEdge>>, ccw: &PositionVector, cw: &PositionVector) -> bool {
        let node = self.myNode;
        if same.len() < 2 {
            return false;
        }
        // C++ std::set<Position>; Position has no Ord, so distinct end points are counted by equality
        let mut endPoints: Vec<Position> = Vec::new();
        let p = e.get_endpoint_at_node(node);
        if !endPoints.contains(&p) {
            endPoints.push(p);
        }
        for &s in &same {
            let p = s.get_endpoint_at_node(node);
            if !endPoints.contains(&p) {
                endPoints.push(p);
            }
        }
        if endPoints.len() > 1 {
            let distances = ccw.distances(cw, true);
            let mut width = e.get_total_width();
            for &e2 in &same {
                width += e2.get_total_width();
            }
            let maxDist = distances.iter().fold(-f64::MAX, |m, &x| if x > m { x } else { m }); // VectorHelper::maxValue
            let maxDivider = maxDist - width;
            return maxDivider >= 5.0;
        }
        false
    }

    /// C++: `double NBNodeShapeComputer::getExtraWidth(const NBEdge *, SVCPermissions)` [static, private]
    /// compute with of rightmost lanes that exlude the given permissions
    // double
    // NBNodeShapeComputer::getExtraWidth(const NBEdge* e, SVCPermissions exclude) {
    //     double result = 0;
    //     int lane = 0;
    //     while (lane < e->getNumLanes() && e->getPermissions(lane) == 0) {
    //         // ignore forbidden lanes out the outside
    //         lane++;
    //     }
    //     while (lane < e->getNumLanes() && (e->getPermissions(lane) & exclude) == 0) {
    //         result += e->getLaneWidth(lane);
    //         lane++;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_extra_width(e: Ptr<NBEdge>, exclude: SVCPermissions) -> f64 {
        let mut result = 0.0;
        let mut lane = 0;
        while lane < e.get_num_lanes() && e.get_permissions(lane) == 0 {
            // ignore forbidden lanes out the outside
            lane += 1;
        }
        while lane < e.get_num_lanes() && (e.get_permissions(lane) & exclude) == 0 {
            result += e.get_lane_width_2(lane);
            lane += 1;
        }
        result
    }

    /// C++: `double NBNodeShapeComputer::divisionWidth(const NBEdge *, std::set<NBEdge *, ComparatorIdLess>, const Position &, const Position &)` [static, private]
    /// compute the width of the divider space for divided roads
    // double
    // NBNodeShapeComputer::divisionWidth(const NBEdge* e, std::set<NBEdge*, ComparatorIdLess> same, const Position& p, const Position& p2) {
    //     double result = p.distanceTo2D(p2);
    //     result -= e->getTotalWidth();
    //     for (NBEdge* e2 : same) {
    //         result -= e2->getTotalWidth();
    //     }
    //     return MAX2(0.0, result);
    // }
    #[allow(unused_variables)]
    pub fn division_width(e: Ptr<NBEdge>, same: BTreeSet<Ptr<NBEdge>>, p: &Position, p2: &Position) -> f64 {
        let mut result = p.distance_to2d(p2);
        result -= e.get_total_width();
        for &e2 in &same {
            result -= e2.get_total_width();
        }
        max2(0.0, result)
    }

    /// C++: `NBNodeShapeComputer & NBNodeShapeComputer::operator=(const NBNodeShapeComputer &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBNodeShapeComputer) -> &mut NBNodeShapeComputer {
        // C++ declares operator= private and never defines it (calling it is a link error);
        // copying the fields keeps the Rust API total
        self.myNode = s.myNode;
        self.myRadius = s.myRadius;
        self.EXT = s.EXT;
        self
    }

}

