//! Port of netbuild/NBNode.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::to_string::f64_to_string;
use utils::geom::POSITION_EPS;
use utils::geom::geom_helper::{angle_diff, deg2rad, rad2deg};
use utils::common::sumo_vehicle_class::SVC_PEDESTRIAN;

// C++ static constants of NBNode (NBNode.cpp:86-94)
const NBNODE_FORWARD: i32 = 1;
const NBNODE_BACKWARD: i32 = -1;
const NBNODE_AVOID_WIDE_LEFT_TURN: i32 = 1;
const NBNODE_AVOID_WIDE_RIGHT_TURN: i32 = 2;
const NBNODE_FOUR_CONTROL_POINTS: i32 = 4;
const NBNODE_AVOID_INTERSECTING_LEFT_TURNS: i32 = 8;
const NBNODE_SCURVE_IGNORE: i32 = 16;
const NBNODE_INDIRECT_LEFT: i32 = 32;
// C++ NBNode.cpp:60-63 (degrees / meters)
const NBNODE_EXTEND_CROSSING_ANGLE_THRESHOLD: f64 = 35.0;
const NBNODE_SPLIT_CROSSING_WIDTH_THRESHOLD: f64 = 1.5;
const NBNODE_SPLIT_CROSSING_ANGLE_THRESHOLD: f64 = 5.0;
// C++ NBEdge::UNSPECIFIED_WIDTH
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.;
// C++ utils/common/StdDefs.h SUMO_const_laneWidth
const NBNODE_SUMO_CONST_LANE_WIDTH: f64 = 3.2;

// C++ MAX2 / MIN2 macros (utils/common/StdDefs.h): `(a) > (b) ? (a) : (b)` and `(a) < (b) ? (a) : (b)`
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// C++ `std::sort(v, less)` on an EdgeVector. determinism: std::sort is unstable, sort_by is stable.
fn sort_edges_by(v: &mut EdgeVector, less: impl Fn(Ptr<NBEdge>, Ptr<NBEdge>) -> bool) {
    v.sort_by(|a, b| {
        if less(*a, *b) {
            std::cmp::Ordering::Less
        } else if less(*b, *a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

/// C++ `rotate(begin, std::find(begin, end, first), end)`; a missing or absent element leaves the vector unchanged
fn rotate_edges_to_front(v: &mut EdgeVector, first: Option<Ptr<NBEdge>>) {
    if let Some(f) = first {
        if let Some(pos) = v.iter().position(|e| *e == f) {
            v.rotate_left(pos);
        }
    }
}

/// C++ `std::pair<NBEdge*, NBEdge::Lane>` as used by NBNode::buildWalkingAreas (only the lane
/// fields that function reads are kept; NBEdge_Lane itself is not Clone).
#[derive(Clone)]
struct NBNodeWaLane {
    edge: Ptr<NBEdge>,
    permissions: SVCPermissions,
    shape: PositionVector,
    width: f64,
}

// C++ static constants of NBNode / NBEdge used as defaults below (ported locally until
// nb_edge.rs / nb_node.rs expose them as statics).
// shortcut: NBEdge statics (UNSPECIFIED_*, myDefaultConnectionLength) live here as consts, upgrade when NBEdge statics are ported.
const NBNODE_UNSPECIFIED_RADIUS: f64 = -1.;
const NBEDGE_UNSPECIFIED_SPEED: f64 = -1.;
const NBEDGE_UNSPECIFIED_FRICTION: f64 = 1.;
const NBEDGE_UNSPECIFIED_CONTPOS: f64 = -1.;
const NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE: f64 = -1.;
const NBEDGE_UNSPECIFIED_CONNECTION_UNCONTROLLED: bool = false;
const NBEDGE_UNSPECIFIED_LOADED_LENGTH: f64 = -1.;

/// C++ `NBNode` constructor checks (thrown as ProcessError in C++)
fn nbnode_validate_new(id: &str, position: &Position) {
    if !utils::xml::sumo_xml_definitions::is_valid_net_id(id) {
        panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Invalid node id '{}'.", id)));
    }
    if position.is_nan() {
        panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Invalid position '{}' for node '{}'", position, id)));
    }
}

/// Computes lane-2-lane connections
/// C++ class `NBNode::ApproachingDivider` (netbuild/NBNode.h)
pub struct NBNode_ApproachingDivider {
    /// C++ base class `Bresenham::BresenhamCallBack`
    pub base: Unmapped /* Bresenham::BresenhamCallBack */,
    /// The list of edges that approach the current edge
    pub myApproaching: Ptr<EdgeVector>,
    /// The approached current edge
    pub myCurrentOutgoing: Ptr<NBEdge>,
    /// The available lanes to which connections shall be built
    pub myAvailableLanes: Vec<i32>,
    /// directions from each incoming edge to the outgoing edge
    pub myDirections: Vec<LinkDirection>,
    /// number of straight connections to the outgoing edge
    pub myNumStraight: i32,
    /// whether the outgoing edge is exclusively used by bikes
    pub myIsBikeEdge: bool,
    /// whether the outgoing edge is exclusively used by buses
    pub myIsBusEdge: bool,
}

impl std::ops::Deref for NBNode_ApproachingDivider {
    type Target = Unmapped /* Bresenham::BresenhamCallBack */;
    fn deref(&self) -> &Unmapped /* Bresenham::BresenhamCallBack */ {
        &self.base
    }
}
impl std::ops::DerefMut for NBNode_ApproachingDivider {
    fn deref_mut(&mut self) -> &mut Unmapped /* Bresenham::BresenhamCallBack */ {
        &mut self.base
    }
}

impl NBNode_ApproachingDivider {
    /// C++: `void ApproachingDivider::ApproachingDivider(const EdgeVector &, NBEdge *)`
    /// Constructor
    // NBNode::ApproachingDivider::ApproachingDivider(
    //     const EdgeVector& approaching, NBEdge* currentOutgoing) :
    //     myApproaching(approaching),
    //     myCurrentOutgoing(currentOutgoing),
    //     myNumStraight(0),
    //     myIsBikeEdge(currentOutgoing->getPermissions() == SVC_BICYCLE),
    //     myIsBusEdge((currentOutgoing->getPermissions() & SVC_BUS) != 0 && (currentOutgoing->getPermissions() & ~(SVC_BUS | SVC_VULNERABLE)) == 0)
    // {
    //     // collect lanes which are expliclity targeted
    //     std::set<int> approachedLanes;
    //     bool hasIncomingBusLane = false;
    //     for (const NBEdge* const approachingEdge : myApproaching) {
    //         for (const NBEdge::Connection& con : approachingEdge->getConnections()) {
    //             if (con.toEdge == myCurrentOutgoing) {
    //                 approachedLanes.insert(con.toLane);
    //             }
    //         }
    //         myDirections.push_back(approachingEdge->getToNode()->getDirection(approachingEdge, currentOutgoing));
    //         if (myDirections.back() == LinkDirection::STRAIGHT) {
    //             myNumStraight++;
    //         }
    //         hasIncomingBusLane |= (approachingEdge->getSpecialLane(SVC_BUS) != -1);
    //     }
    //     // compute the indices of lanes that should be targeted (excluding pedestrian
    //     // lanes that will be connected from walkingAreas and forbidden lanes)
    //     // if the lane is targeted by an explicitly set connection we need
    //     // to make it available anyway
    //     for (int i = 0; i < currentOutgoing->getNumLanes(); ++i) {
    //         const SVCPermissions lp = currentOutgoing->getPermissions(i);
    //         if ((lp == SVC_PEDESTRIAN
    //                 // don't consider bicycle lanes as targets unless the target
    //                 // edge is exclusively for bicycles
    //                 || (lp == SVC_BICYCLE && !myIsBikeEdge)
    //                 || (lp == SVC_BUS && hasIncomingBusLane)
    //                 || isForbidden(lp))
    //                 && approachedLanes.count(i) == 0) {
    //             continue;
    //         }
    //         myAvailableLanes.push_back(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(approaching: &EdgeVector, currentOutgoing: Ptr<NBEdge>) -> Self {
        use utils::common::sumo_vehicle_class::{SVC_BICYCLE, SVC_BUS, SVC_VULNERABLE, is_forbidden};
        let myIsBikeEdge = currentOutgoing.get_permissions(-1) == SVC_BICYCLE;
        let myIsBusEdge = (currentOutgoing.get_permissions(-1) & SVC_BUS) != 0
            && (currentOutgoing.get_permissions(-1) & !(SVC_BUS | SVC_VULNERABLE)) == 0;
        let mut me = NBNode_ApproachingDivider {
            base: (),
            myApproaching: Ptr::new(approaching),
            myCurrentOutgoing: currentOutgoing,
            myAvailableLanes: Vec::new(),
            myDirections: Vec::new(),
            myNumStraight: 0,
            myIsBikeEdge,
            myIsBusEdge,
        };
        // collect lanes which are expliclity targeted
        let mut approachedLanes: BTreeSet<i32> = BTreeSet::new();
        let mut hasIncomingBusLane = false;
        for approachingEdge in approaching.iter() {
            let approachingEdge = *approachingEdge;
            for con in approachingEdge.get_connections() {
                if con.toEdge == currentOutgoing {
                    approachedLanes.insert(con.toLane);
                }
            }
            let dir = approachingEdge.get_to_node().get_direction(approachingEdge, currentOutgoing, false);
            me.myDirections.push(dir);
            if dir == LinkDirection::STRAIGHT {
                me.myNumStraight += 1;
            }
            hasIncomingBusLane |= approachingEdge.get_special_lane(SVC_BUS) != -1;
        }
        // compute the indices of lanes that should be targeted (excluding pedestrian
        // lanes that will be connected from walkingAreas and forbidden lanes)
        // if the lane is targeted by an explicitly set connection we need
        // to make it available anyway
        for i in 0..currentOutgoing.get_num_lanes() {
            let lp = currentOutgoing.get_permissions(i);
            if (lp == SVC_PEDESTRIAN
                // don't consider bicycle lanes as targets unless the target
                // edge is exclusively for bicycles
                || (lp == SVC_BICYCLE && !myIsBikeEdge)
                || (lp == SVC_BUS && hasIncomingBusLane)
                || is_forbidden(lp))
                && !approachedLanes.contains(&i)
            {
                continue;
            }
            me.myAvailableLanes.push(i);
        }
        me
    }

    /// C++: `void ApproachingDivider::~ApproachingDivider() noexcept` [virtual]
    /// Destructor
    // NBNode::ApproachingDivider::~ApproachingDivider() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty; the Vec members are dropped automatically
    }

    /// C++: `int ApproachingDivider::numAvailableLanes() const` [const]
    /// @ get number of available lanes
    // int numAvailableLanes() const {
    //             return (int)myAvailableLanes.size();
    //         }
    #[allow(unused_variables)]
    pub fn num_available_lanes(&self) -> i32 {
        self.myAvailableLanes.len() as i32
    }

    /// C++: `void ApproachingDivider::execute(const int, const int)` [virtual]
    /// the bresenham-callback
    // void
    // NBNode::ApproachingDivider::execute(const int src, const int dest) {
    //     assert((int)myApproaching.size() > src);
    //     // get the origin edge
    //     NBEdge* incomingEdge = myApproaching[src];
    //     if (incomingEdge->getStep() == NBEdge::EdgeBuildingStep::LANES2LANES_DONE || incomingEdge->getStep() == NBEdge::EdgeBuildingStep::LANES2LANES_USER) {
    //         return;
    //     }
    //     if (myAvailableLanes.size() == 0) {
    //         return;
    //     }
    //     const bool withBikes = myIsBikeEdge || incomingEdge->getPermissions() == SVC_BICYCLE;
    //     std::vector<int> approachingLanes = incomingEdge->getConnectionLanes(myCurrentOutgoing, withBikes, true);
    //     if (approachingLanes.size() > myAvailableLanes.size() ||
    //             (incomingEdge->getSpecialLane(SVC_BUS) >= 0 && myCurrentOutgoing->getSpecialLane(SVC_BUS) >= 0)) {
    //         const bool withBusLanes = myIsBusEdge || ((incomingEdge->getPermissions() & SVC_BUS) != 0 && (incomingEdge->getPermissions() & ~(SVC_BUS | SVC_VULNERABLE)) == 0);
    //         approachingLanes = incomingEdge->getConnectionLanes(myCurrentOutgoing, withBikes, withBusLanes);
    //     }
    //     if (approachingLanes.size() == 0) {
    //         return;
    //     }
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND2(incomingEdge->getToNode())) {
    //         std::cout << "Bre:ex src=" << src << " dest=" << dest << " in=" << incomingEdge->getID() << " apLanes=" << toString(approachingLanes) << "\n";
    //     }
    //
    // #endif
    //     int numConnections = (int)approachingLanes.size();
    //     double factor = 1;
    //     const bool rightOnRed = incomingEdge->getToNode()->getType() == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED;
    //     if (myNumStraight == 1 && myDirections[src] == LinkDirection::STRAIGHT && (
    //                 // we do not want to destroy ramp-like assignments where the
    //                 // on-connection-per-lane rule avoids conflicts
    //                 // - at a traffic light the phases are seperated so there is no conflict anyway
    //                 (incomingEdge->getToNode()->isTLControlled() && !rightOnRed)
    //                 // - there are no incoming edges to the right
    //                 || src == 0
    //                 // - a minor straight road is likely in conflict anyway
    //                 || (incomingEdge->getJunctionPriority(incomingEdge->getToNode()) == NBEdge::MINOR_ROAD && !rightOnRed))) {
    //         numConnections = (int)myAvailableLanes.size();
    //         factor = (double)approachingLanes.size() / (double)numConnections;
    //         if (factor > 0.5) {
    //             factor = 1;
    //         }
    //     }
    //     std::deque<int>* approachedLanes = spread(numConnections, dest);
    //     assert(approachedLanes->size() <= myAvailableLanes.size());
    //     // set lanes
    //     const int maxFrom = (int)approachingLanes.size() - 1;
    //     for (int i = 0; i < (int)approachedLanes->size(); i++) {
    //         // distribute i evenly on approaching lanes in case we are building more
    //         // connections than there are lanes
    //         int fromLane = approachingLanes[MIN2((int)(i * factor), maxFrom)];
    //         int approached = myAvailableLanes[(*approachedLanes)[i]];
    //         incomingEdge->setConnection(fromLane, myCurrentOutgoing, approached, NBEdge::Lane2LaneInfoType::COMPUTED);
    //     }
    //     delete approachedLanes;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, src: i32, dest: i32) {
        assert!((self.myApproaching.len() as i32) > src);
        // get the origin edge
        let incomingEdge: Ptr<NBEdge> = self.myApproaching[src as usize];
        if incomingEdge.get_step() == NBEdge_EdgeBuildingStep::LANES2LANES_DONE || incomingEdge.get_step() == NBEdge_EdgeBuildingStep::LANES2LANES_USER {
            return;
        }
        if self.myAvailableLanes.is_empty() {
            return;
        }
        let withBikes = self.myIsBikeEdge || incomingEdge.get_permissions(-1) == SVC_BICYCLE;
        let mut approachingLanes = incomingEdge.get_connection_lanes(self.myCurrentOutgoing, withBikes, true);
        if approachingLanes.len() > self.myAvailableLanes.len()
            || (incomingEdge.get_special_lane(SVC_BUS) >= 0 && self.myCurrentOutgoing.get_special_lane(SVC_BUS) >= 0)
        {
            let withBusLanes = self.myIsBusEdge || ((incomingEdge.get_permissions(-1) & SVC_BUS) != 0 && (incomingEdge.get_permissions(-1) & !(SVC_BUS | SVC_VULNERABLE)) == 0);
            approachingLanes = incomingEdge.get_connection_lanes(self.myCurrentOutgoing, withBikes, withBusLanes);
        }
        if approachingLanes.is_empty() {
            return;
        }
        let mut numConnections = approachingLanes.len() as i32;
        let mut factor: f64 = 1.;
        let rightOnRed = incomingEdge.get_to_node().get_type() == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED;
        if self.myNumStraight == 1
            && self.myDirections[src as usize] == LinkDirection::STRAIGHT
            && ((incomingEdge.get_to_node().is_tl_controlled() && !rightOnRed)
                || src == 0
                || (incomingEdge.get_junction_priority(incomingEdge.get_to_node()) == NBEdge_JunctionPriority::MINOR_ROAD as i32 && !rightOnRed))
        {
            numConnections = self.myAvailableLanes.len() as i32;
            factor = approachingLanes.len() as f64 / numConnections as f64;
            if factor > 0.5 {
                factor = 1.;
            }
        }
        let approachedLanes = self.spread(numConnections, dest);
        assert!(approachedLanes.len() <= self.myAvailableLanes.len());
        // set lanes
        let maxFrom = approachingLanes.len() as i32 - 1;
        for i in 0..approachedLanes.len() as i32 {
            // distribute i evenly on approaching lanes in case we are building more
            // connections than there are lanes
            let fromLane = approachingLanes[std::cmp::min((i as f64 * factor) as i32, maxFrom) as usize];
            let approached = self.myAvailableLanes[approachedLanes[i as usize] as usize];
            incomingEdge.m().set_connection(
                fromLane,
                self.myCurrentOutgoing,
                approached,
                NBEdge_Lane2LaneInfoType::COMPUTED,
                false,
                false,
                KeepClear::KEEPCLEAR_UNSPECIFIED,
                NBEDGE_UNSPECIFIED_CONTPOS,
                NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE,
                NBEDGE_UNSPECIFIED_SPEED,
                NBEDGE_UNSPECIFIED_FRICTION,
                NBEDGE_UNSPECIFIED_LOADED_LENGTH, // shortcut: NBEdge::myDefaultConnectionLength is -1 until NBEdge::setDefaultConnectionLength is ported
                &PositionVector::default(),
                NBEDGE_UNSPECIFIED_CONNECTION_UNCONTROLLED,
                SVC_UNSPECIFIED,
                false,
                "",
                SVC_UNSPECIFIED,
                SVC_UNSPECIFIED,
                false,
            );
        }
        // C++ `delete approachedLanes`
        unsafe { drop(Box::from_raw(approachedLanes.as_raw())) };
    }

    /// C++: `std::deque<int> * ApproachingDivider::spread(int, int) const` [const]
    /// the method that spreads the wished number of lanes from the lane given by the bresenham-call to both left and right
    // std::deque<int>*
    // NBNode::ApproachingDivider::spread(int numLanes, int dest) const {
    //     std::deque<int>* ret = new std::deque<int>();
    //     // when only one lane is approached, we check, whether the double-value
    //     //  is assigned more to the left or right lane
    //     if (numLanes == 1) {
    //         ret->push_back(dest);
    //         return ret;
    //     }
    //
    //     const int numOutgoingLanes = (int)myAvailableLanes.size();
    //     //
    //     ret->push_back(dest);
    //     int noSet = 1;
    //     int roffset = 1;
    //     int loffset = 1;
    //     while (noSet < numLanes) {
    //         // It may be possible, that there are not enough lanes the source
    //         //  lanes may be divided on
    //         //  In this case, they remain unset
    //         //  !!! this is only a hack. It is possible, that this yields in
    //         //   uncommon divisions
    //         if (numOutgoingLanes == noSet) {
    //             return ret;
    //         }
    //
    //         // as due to the conversion of double->uint the numbers will be lower
    //         //  than they should be, we try to append to the left side first
    //         //
    //         // check whether the left boundary of the approached street has
    //         //  been overridden; if so, move all lanes to the right
    //         if (dest + loffset >= numOutgoingLanes) {
    //             loffset -= 1;
    //             roffset += 1;
    //             for (int i = 0; i < (int)ret->size(); i++) {
    //                 (*ret)[i] = (*ret)[i] - 1;
    //             }
    //         }
    //         // append the next lane to the left of all edges
    //         //  increase the position (destination edge)
    //         ret->push_back(dest + loffset);
    //         noSet++;
    //         loffset += 1;
    //
    //         // as above
    //         if (numOutgoingLanes == noSet) {
    //             return ret;
    //         }
    //
    //         // now we try to append the next lane to the right side, when needed
    //         if (noSet < numLanes) {
    //             // check whether the right boundary of the approached street has
    //             //  been overridden; if so, move all lanes to the right
    //             if (dest < roffset) {
    //                 loffset += 1;
    //                 roffset -= 1;
    //                 for (int i = 0; i < (int)ret->size(); i++) {
    //                     (*ret)[i] = (*ret)[i] + 1;
    //                 }
    //             }
    //             ret->push_front(dest - roffset);
    //             noSet++;
    //             roffset += 1;
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn spread(&self, numLanes: i32, dest: i32) -> Ptr<VecDeque<i32>> {
        let mut ret: VecDeque<i32> = VecDeque::new();
        // when only one lane is approached, we check, whether the double-value
        //  is assigned more to the left or right lane
        if numLanes == 1 {
            ret.push_back(dest);
            return Ptr::from_raw(Box::into_raw(Box::new(ret)));
        }

        let numOutgoingLanes = self.myAvailableLanes.len() as i32;
        //
        ret.push_back(dest);
        let mut noSet = 1;
        let mut roffset = 1;
        let mut loffset = 1;
        while noSet < numLanes {
            // It may be possible, that there are not enough lanes the source
            //  lanes may be divided on
            //  In this case, they remain unset
            //  !!! this is only a hack. It is possible, that this yields in
            //   uncommon divisions
            if numOutgoingLanes == noSet {
                return Ptr::from_raw(Box::into_raw(Box::new(ret)));
            }

            // as due to the conversion of double->uint the numbers will be lower
            //  than they should be, we try to append to the left side first
            //
            // check whether the left boundary of the approached street has
            //  been overridden; if so, move all lanes to the right
            if dest + loffset >= numOutgoingLanes {
                loffset -= 1;
                roffset += 1;
                for x in ret.iter_mut() {
                    *x = *x - 1;
                }
            }
            // append the next lane to the left of all edges
            //  increase the position (destination edge)
            ret.push_back(dest + loffset);
            noSet += 1;
            loffset += 1;

            // as above
            if numOutgoingLanes == noSet {
                return Ptr::from_raw(Box::into_raw(Box::new(ret)));
            }

            // now we try to append the next lane to the right side, when needed
            if noSet < numLanes {
                // check whether the right boundary of the approached street has
                //  been overridden; if so, move all lanes to the right
                if dest < roffset {
                    loffset += 1;
                    roffset -= 1;
                    for x in ret.iter_mut() {
                        *x = *x + 1;
                    }
                }
                ret.push_front(dest - roffset);
                noSet += 1;
                roffset += 1;
            }
        }
        Ptr::from_raw(Box::into_raw(Box::new(ret)))
    }

    /// C++: `ApproachingDivider & ApproachingDivider::operator=(const ApproachingDivider &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBNode_ApproachingDivider) -> &mut NBNode_ApproachingDivider {
        // C++ declares this operator private and never defines it, so any use is a bug
        panic!("ApproachingDivider::operator= is private and undefined in C++")
    }

}

/// A definition of a pedestrian crossing
/// C++ class `NBNode::Crossing` (netbuild/NBNode.h)
pub struct NBNode_Crossing {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The parent node of this crossing
    pub node: Ptr<NBNode>,
    /// The edges being crossed
    pub edges: EdgeVector,
    /// The crossing's shape
    pub shape: PositionVector,
    /// The outline shape for this crossing
    pub outlineShape: PositionVector,
    /// This crossing's width
    pub customWidth: f64,
    /// This crossing's width
    pub width: f64,
    /// the (edge)-id of this crossing
    pub id: String,
    /// the lane-id of the previous walkingArea
    pub prevWalkingArea: String,
    /// the lane-id of the next walkingArea
    pub nextWalkingArea: String,
    /// whether the pedestrians have priority
    pub priority: bool,
    /// optional customShape for this crossing
    pub customShape: PositionVector,
    /// the traffic light index of this crossing (if controlled)
    pub tlLinkIndex: i32,
    pub tlLinkIndex2: i32,
    /// the custom traffic light index of this crossing (if controlled)
    pub customTLIndex: i32,
    pub customTLIndex2: i32,
    /// The id of the traffic light that controls this connection
    pub tlID: String,
    /// whether this crossing is valid (and can be written to the net.xml). This is needed for netedit because validity can only be checked during junction computation
    pub valid: bool,
}

impl std::ops::Deref for NBNode_Crossing {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NBNode_Crossing {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NBNode_Crossing {
    /// C++: `void Crossing::Crossing(const NBNode *, const EdgeVector &, double, bool, int, int, const PositionVector &)`
    /// constructor
    // NBNode::Crossing::Crossing(const NBNode* _node, const EdgeVector& _edges, double _width, bool _priority, int _customTLIndex, int _customTLIndex2, const PositionVector& _customShape) :
    //     Parameterised(),
    //     node(_node),
    //     edges(_edges),
    //     customWidth(_width),
    //     width(_width),
    //     priority(_priority),
    //     customShape(_customShape),
    //     tlLinkIndex(_customTLIndex),
    //     tlLinkIndex2(_customTLIndex2),
    //     customTLIndex(_customTLIndex),
    //     customTLIndex2(_customTLIndex2),
    //     valid(true) {
    // }
    #[allow(unused_variables)]
    pub fn new(_node: Ptr<NBNode>, _edges: &EdgeVector, _width: f64, _priority: bool, _customTLIndex: i32, _customTLIndex2: i32, _customShape: &PositionVector) -> Self {
        NBNode_Crossing {
            base: Parameterised::new(),
            node: _node,
            edges: _edges.clone(),
            shape: PositionVector::default(),
            outlineShape: PositionVector::default(),
            customWidth: _width,
            width: _width,
            id: String::new(),
            prevWalkingArea: String::new(),
            nextWalkingArea: String::new(),
            priority: _priority,
            customShape: _customShape.clone(),
            tlLinkIndex: _customTLIndex,
            tlLinkIndex2: _customTLIndex2,
            customTLIndex: _customTLIndex,
            customTLIndex2: _customTLIndex2,
            tlID: String::new(),
            valid: true,
        }
    }

}

/// Represents a single node (junction) during network building
/// C++ class `NBNode` (netbuild/NBNode.h)
pub struct NBNode {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// The position the node lies at
    pub myPosition: Position,
    /// Vector of incoming edges
    pub myIncomingEdges: EdgeVector,
    /// Vector of outgoing edges
    pub myOutgoingEdges: EdgeVector,
    /// Vector of incoming and outgoing edges
    pub myAllEdges: EdgeVector,
    /// Vector of crossings
    pub myCrossings: Vec<Box<NBNode_Crossing>>,
    /// Vector of walking areas
    pub myWalkingAreas: Vec<NBNode_WalkingArea>,
    /// Vector of custom walking areas shapes
    pub myWalkingAreaCustomShapes: Vec<NBNode_WalkingAreaCustomShape>,
    /// The type of the junction
    pub myType: SumoXMLNodeType,
    /// The container for connection block dependencies
    pub myBlockedConnections: NBConnectionProhibits,
    /// The district the node is the centre of
    pub myDistrict: Ptr<NBDistrict>,
    /// the (outer) shape of the junction
    pub myPoly: PositionVector,
    /// whether this nodes shape was set by the user
    pub myHaveCustomPoly: bool,
    /// Node requests
    pub myRequest: Ptr<NBRequest>,
    /// traffic lights of node
    pub myTrafficLights: BTreeSet<Ptr<NBTrafficLightDefinition>>,
    /// the turning radius (for all corners) at this node in m.
    pub myRadius: f64,
    /// whether the junction area must be kept clear
    pub myKeepClear: bool,
    /// how to compute right of way for this node
    pub myRightOfWay: RightOfWay,
    /// fringe type of this node
    pub myFringeType: FringeType,
    /// roundabout type of this node
    pub myRoundaboutType: RoundaboutType,
    /// The intersection name (or whatever arbitrary string you wish to attach)
    pub myName: String,
    /// whether to discard all pedestrian crossings
    pub myDiscardAllCrossings: bool,
    /// number of crossings loaded from a sumo net
    pub myCrossingsLoadedFromSumoNet: i32,
    /// geometry error after computation of internal lane shapes
    pub myDisplacementError: f64,
    pub myIsBentPriority: bool,
    /// whether the node type was guessed rather than loaded
    pub myTypeWasGuessed: bool,
}

impl std::ops::Deref for NBNode {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NBNode {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of NBNode (port as module-level statics, see AGENTS.md):
//   static const int NBNode::FORWARD;
//   static const int NBNode::BACKWARD;
//   static const double NBNode::UNSPECIFIED_RADIUS;
//   static const int NBNode::AVOID_WIDE_RIGHT_TURN;
//   static const int NBNode::AVOID_WIDE_LEFT_TURN;
//   static const int NBNode::FOUR_CONTROL_POINTS;
//   static const int NBNode::AVOID_INTERSECTING_LEFT_TURNS;
//   static const int NBNode::SCURVE_IGNORE;
//   static const int NBNode::INDIRECT_LEFT;
//   static SVCPermissions NBNode::myHaveRailSignalClasses;
//   static SVCPermissions NBNode::myPermitUnsignalizedClasses;

impl NBNode {
    /// C++: `void NBNode::NBNode(const std::string &, const Position &, SumoXMLNodeType)`
    /// Constructor
    // NBNode::NBNode(const std::string& id, const Position& position,
    //                SumoXMLNodeType type) :
    //     Named(StringUtils::convertUmlaute(id)),
    //     myPosition(position),
    //     myType(type),
    //     myDistrict(nullptr),
    //     myHaveCustomPoly(false),
    //     myRequest(nullptr),
    //     myRadius(UNSPECIFIED_RADIUS),
    //     myKeepClear(OptionsCont::getOptions().getBool("default.junctions.keep-clear")),
    //     myRightOfWay(SUMOXMLDefinitions::RightOfWayValues.get(OptionsCont::getOptions().getString("default.right-of-way"))),
    //     myFringeType(FringeType::DEFAULT),
    //     myRoundaboutType(RoundaboutType::DEFAULT),
    //     myDiscardAllCrossings(false),
    //     myCrossingsLoadedFromSumoNet(0),
    //     myDisplacementError(0),
    //     myIsBentPriority(false),
    //     myTypeWasGuessed(false) {
    //     if (!SUMOXMLDefinitions::isValidNetID(myID)) {
    //         throw ProcessError(TLF("Invalid node id '%'.", myID));
    //     }
    //     if (myPosition.isNAN()) {
    //         throw ProcessError(TLF("Invalid position '%' for node '%'", myPosition, myID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, position: &Position, type_: SumoXMLNodeType) -> Self {
        let node = NBNode {
            base: Named::new(&utils::common::string_utils::convert_umlaute(id)),
            base2: Parameterised::new(),
            myPosition: *position,
            myIncomingEdges: Vec::new(),
            myOutgoingEdges: Vec::new(),
            myAllEdges: Vec::new(),
            myCrossings: Vec::new(),
            myWalkingAreas: Vec::new(),
            myWalkingAreaCustomShapes: Vec::new(),
            myType: type_,
            myBlockedConnections: BTreeMap::new(),
            myDistrict: Ptr::null(),
            myPoly: PositionVector::default(),
            myHaveCustomPoly: false,
            myRequest: Ptr::null(),
            myTrafficLights: BTreeSet::new(),
            myRadius: NBNODE_UNSPECIFIED_RADIUS,
            myKeepClear: utils::options::options_cont::get_bool("default.junctions.keep-clear"),
            myRightOfWay: *utils::xml::sumo_xml_definitions::RightOfWayValues
                .get(&utils::options::options_cont::get_string("default.right-of-way"))
                .unwrap(),
            myFringeType: FringeType::DEFAULT,
            myRoundaboutType: RoundaboutType::DEFAULT,
            myName: String::new(),
            myDiscardAllCrossings: false,
            myCrossingsLoadedFromSumoNet: 0,
            myDisplacementError: 0.,
            myIsBentPriority: false,
            myTypeWasGuessed: false,
        };
        nbnode_validate_new(node.get_id(), &node.myPosition);
        node
    }

    /// C++: `void NBNode::NBNode(const std::string &, const Position &, NBDistrict *)`
    /// Constructor
    // NBNode::NBNode(const std::string& id, const Position& position, NBDistrict* district) :
    //     Named(StringUtils::convertUmlaute(id)),
    //     myPosition(position),
    //     myType(district == nullptr ? SumoXMLNodeType::UNKNOWN : SumoXMLNodeType::DISTRICT),
    //     myDistrict(district),
    //     myHaveCustomPoly(false),
    //     myRequest(nullptr),
    //     myRadius(UNSPECIFIED_RADIUS),
    //     myKeepClear(OptionsCont::getOptions().getBool("default.junctions.keep-clear")),
    //     myRightOfWay(SUMOXMLDefinitions::RightOfWayValues.get(OptionsCont::getOptions().getString("default.right-of-way"))),
    //     myFringeType(FringeType::DEFAULT),
    //     myRoundaboutType(RoundaboutType::DEFAULT),
    //     myDiscardAllCrossings(false),
    //     myCrossingsLoadedFromSumoNet(0),
    //     myDisplacementError(0),
    //     myIsBentPriority(false),
    //     myTypeWasGuessed(false) {
    //     if (!SUMOXMLDefinitions::isValidNetID(myID)) {
    //         throw ProcessError(TLF("Invalid node id '%'.", myID));
    //     }
    //     if (myPosition.isNAN()) {
    //         throw ProcessError(TLF("Invalid position '%' for node '%'", myPosition, myID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, position: &Position, district: Ptr<NBDistrict>) -> Self {
        // same member initialisation as the (id, position, type) constructor, with the district set
        let mut node = Self::new(id, position, if district.is_null() { SumoXMLNodeType::UNKNOWN } else { SumoXMLNodeType::DISTRICT });
        node.myDistrict = district;
        node
    }

    /// C++: `void NBNode::~NBNode() noexcept` [virtual]
    /// Destructor
    // NBNode::~NBNode() {
    //     delete myRequest;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ `delete myRequest` (a no-op for nullptr)
        if !self.myRequest.is_null() {
            unsafe { drop(Box::from_raw(self.myRequest.as_raw())) };
        }
    }

    /// C++: `void NBNode::reinit(const Position &, SumoXMLNodeType, bool)`
    /// Resets initial values
    // void
    // NBNode::reinit(const Position& position, SumoXMLNodeType type,
    //                bool updateEdgeGeometries) {
    //     myPosition = position;
    //     if (myPosition.isNAN()) {
    //         throw ProcessError(TLF("Invalid position '%' for node '%'", myPosition, myID));
    //     }
    //     // patch type
    //     myType = type;
    //     if (!isTrafficLight(myType)) {
    //         removeTrafficLights();
    //     }
    //     if (updateEdgeGeometries) {
    //         for (EdgeVector::iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //             PositionVector geom = (*i)->getGeometry();
    //             geom[-1] = myPosition;
    //             (*i)->setGeometry(geom);
    //         }
    //         for (EdgeVector::iterator i = myOutgoingEdges.begin(); i != myOutgoingEdges.end(); i++) {
    //             PositionVector geom = (*i)->getGeometry();
    //             geom[0] = myPosition;
    //             (*i)->setGeometry(geom);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reinit(&mut self, position: &Position, type_: SumoXMLNodeType, updateEdgeGeometries: bool) {
        self.myPosition = *position;
        if self.myPosition.is_nan() {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Invalid position '{}' for node '{}'", self.myPosition, self.get_id())));
        }
        // patch type
        self.myType = type_;
        if !Self::is_traffic_light(self.myType) {
            self.remove_traffic_lights(false);
        }
        if updateEdgeGeometries {
            for edge in self.myIncomingEdges.clone() {
                let mut geom = edge.get_geometry().clone();
                *geom.at_mut(-1) = self.myPosition;
                edge.m().set_geometry(&geom, false);
            }
            for edge in self.myOutgoingEdges.clone() {
                let mut geom = edge.get_geometry().clone();
                *geom.at_mut(0) = self.myPosition;
                edge.m().set_geometry(&geom, false);
            }
        }
    }

    /// C++: `const Position & NBNode::getPosition() const` [const]
    /// Returns the position of this node
    // inline const Position& getPosition() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position {
        &self.myPosition
    }

    /// C++: `Position NBNode::getCenter() const` [const]
    /// Returns a position that is guaranteed to lie within the node shape
    // Position
    // NBNode::getCenter() const {
    //     /* Conceptually, the center point would be identical with myPosition.
    //     * However, if the shape is influenced by custom geometry endpoints of the adjoining edges,
    //     * myPosition may fall outside the shape. In this case it is better to use
    //     * the center of the shape
    //     **/
    //     PositionVector tmp = myPoly;
    //     tmp.closePolygon();
    //     //std::cout << getID() << " around=" << tmp.around(myPosition) << " dist=" << tmp.distance2D(myPosition) << "\n";
    //     if (tmp.size() < 3 || tmp.around(myPosition) || tmp.distance2D(myPosition) < POSITION_EPS) {
    //         return myPosition;
    //     }
    //     return myPoly.getPolygonCenter();
    // }
    #[allow(unused_variables)]
    pub fn get_center(&self) -> Position {
        let mut tmp = self.myPoly.clone();
        tmp.close_polygon();
        if tmp.len() < 3 || tmp.around(&self.myPosition, 0.) || tmp.distance2d(&self.myPosition, false) < utils::geom::POSITION_EPS {
            return self.myPosition;
        }
        self.myPoly.get_polygon_center()
    }

    /// C++: `const EdgeVector & NBNode::getIncomingEdges() const` [const]
    /// Returns this node's incoming edges (The edges which yield in this node)
    // inline const EdgeVector& getIncomingEdges() const {
    //         return myIncomingEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_incoming_edges(&self) -> &EdgeVector {
        &self.myIncomingEdges
    }

    /// C++: `const EdgeVector & NBNode::getOutgoingEdges() const` [const]
    /// Returns this node's outgoing edges (The edges which start at this node)
    // inline const EdgeVector& getOutgoingEdges() const {
    //         return myOutgoingEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_outgoing_edges(&self) -> &EdgeVector {
        &self.myOutgoingEdges
    }

    /// C++: `const EdgeVector & NBNode::getEdges() const` [const]
    /// Returns all edges which participate in this node (Edges that start or end at this node)
    // inline const EdgeVector& getEdges() const {
    //         return myAllEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> &EdgeVector {
        &self.myAllEdges
    }

    /// C++: `SumoXMLNodeType NBNode::getType() const` [const]
    /// Returns the type of this node
    // inline SumoXMLNodeType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> SumoXMLNodeType {
        self.myType
    }

    /// C++: `double NBNode::getRadius() const` [const]
    /// Returns the turning radius of this node
    // inline double getRadius() const {
    //         return myRadius;
    //     }
    #[allow(unused_variables)]
    pub fn get_radius(&self) -> f64 {
        self.myRadius
    }

    /// C++: `bool NBNode::getKeepClear() const` [const]
    /// Returns the keepClear flag
    // inline bool getKeepClear() const {
    //         return myKeepClear;
    //     }
    #[allow(unused_variables)]
    pub fn get_keep_clear(&self) -> bool {
        self.myKeepClear
    }

    /// C++: `RightOfWay NBNode::getRightOfWay() const` [const]
    /// Returns hint on how to compute right of way
    // inline RightOfWay getRightOfWay() const {
    //         return myRightOfWay;
    //     }
    #[allow(unused_variables)]
    pub fn get_right_of_way(&self) -> RightOfWay {
        self.myRightOfWay
    }

    /// C++: `FringeType NBNode::getFringeType() const` [const]
    /// Returns fringe type
    // inline FringeType getFringeType() const {
    //         return myFringeType;
    //     }
    #[allow(unused_variables)]
    pub fn get_fringe_type(&self) -> FringeType {
        self.myFringeType
    }

    /// C++: `RoundaboutType NBNode::getRoundaboutType() const` [const]
    /// Returns roundabout type
    // inline RoundaboutType getRoundaboutType() const {
    //         return myRoundaboutType;
    //     }
    #[allow(unused_variables)]
    pub fn get_roundabout_type(&self) -> RoundaboutType {
        self.myRoundaboutType
    }

    /// C++: `const std::string & NBNode::getName() const` [const]
    /// Returns intersection name
    // inline const std::string& getName() const {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        &self.myName
    }

    /// C++: `void NBNode::addTrafficLight(NBTrafficLightDefinition *)`
    /// Adds a traffic light to the list of traffic lights that control this node
    // void
    // NBNode::addTrafficLight(NBTrafficLightDefinition* tlDef) {
    //     myTrafficLights.insert(tlDef);
    //     // rail signals receive a temporary traffic light in order to set connection tl-linkIndex
    //     if (!isTrafficLight(myType) && myType != SumoXMLNodeType::RAIL_SIGNAL && myType != SumoXMLNodeType::RAIL_CROSSING) {
    //         myType = SumoXMLNodeType::TRAFFIC_LIGHT;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_traffic_light(&mut self, tlDef: Ptr<NBTrafficLightDefinition>) {
        self.myTrafficLights.insert(tlDef);
        // rail signals receive a temporary traffic light in order to set connection tl-linkIndex
        if !Self::is_traffic_light(self.myType) && self.myType != SumoXMLNodeType::RAIL_SIGNAL && self.myType != SumoXMLNodeType::RAIL_CROSSING {
            self.myType = SumoXMLNodeType::TRAFFIC_LIGHT;
        }
    }

    /// C++: `void NBNode::removeTrafficLight(NBTrafficLightDefinition *)`
    /// Removes the given traffic light from this node
    // void
    // NBNode::removeTrafficLight(NBTrafficLightDefinition* tlDef) {
    //     tlDef->removeNode(this);
    //     myTrafficLights.erase(tlDef);
    // }
    #[allow(unused_variables)]
    pub fn remove_traffic_light(&mut self, tlDef: Ptr<NBTrafficLightDefinition>) {
        tlDef.m().remove_node(Ptr::from_mut(self));
        self.myTrafficLights.remove(&tlDef);
    }

    /// C++: `void NBNode::removeTrafficLights(bool)`
    /// Removes all references to traffic lights that control this tls
    // void
    // NBNode::removeTrafficLights(bool setAsPriority) {
    //     std::set<NBTrafficLightDefinition*> trafficLights = myTrafficLights; // make a copy because we will modify the original
    //     for (std::set<NBTrafficLightDefinition*>::const_iterator i = trafficLights.begin(); i != trafficLights.end(); ++i) {
    //         removeTrafficLight(*i);
    //     }
    //     if (setAsPriority) {
    //         myType = myRequest != nullptr ? SumoXMLNodeType::PRIORITY : (
    //                      myType == SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION ? SumoXMLNodeType::NOJUNCTION : SumoXMLNodeType::DEAD_END);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_traffic_lights(&mut self, setAsPriority: bool) {
        let trafficLights = self.myTrafficLights.clone(); // make a copy because we will modify the original
        for tl in trafficLights {
            self.remove_traffic_light(tl);
        }
        if setAsPriority {
            self.myType = if !self.myRequest.is_null() {
                SumoXMLNodeType::PRIORITY
            } else if self.myType == SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION {
                SumoXMLNodeType::NOJUNCTION
            } else {
                SumoXMLNodeType::DEAD_END
            };
        }
    }

    /// C++: `bool NBNode::isTLControlled() const` [const]
    /// Returns whether this node is controlled by any tls
    // bool isTLControlled() const {
    //         return myTrafficLights.size() != 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_tl_controlled(&self) -> bool {
        self.myTrafficLights.len() != 0
    }

    /// C++: `bool NBNode::hadSignal() const` [const]
    /// whether this node was marked as having a signal in the (OSM) input
    // bool
    // NBNode::hadSignal() const {
    //     for (NBEdge* e : getIncomingEdges()) {
    //         if (e->getSignalPosition() != Position::INVALID) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn had_signal(&self) -> bool {
        for e in self.get_incoming_edges() {
            if e.get_signal_position() != &Position::INVALID {
                return true;
            }
        }
        false
    }

    /// C++: `const std::set<NBTrafficLightDefinition *> & NBNode::getControllingTLS() const` [const]
    /// Returns the traffic lights that were assigned to this node (The set of tls that control this node)
    // const std::set<NBTrafficLightDefinition*>& getControllingTLS() const {
    //         return myTrafficLights;
    //     }
    #[allow(unused_variables)]
    pub fn get_controlling_tls(&self) -> &BTreeSet<Ptr<NBTrafficLightDefinition>> {
        &self.myTrafficLights
    }

    /// C++: `void NBNode::invalidateTLS(NBTrafficLightLogicCont &, bool, bool)`
    /// causes the traffic light to be computed anew
    // void
    // NBNode::invalidateTLS(NBTrafficLightLogicCont& tlCont, bool addedConnections, bool removedConnections) {
    //     if (isTLControlled()) {
    //         std::set<NBTrafficLightDefinition*> oldDefs(myTrafficLights);
    //         for (std::set<NBTrafficLightDefinition*>::iterator it = oldDefs.begin(); it != oldDefs.end(); ++it) {
    //             NBTrafficLightDefinition* orig = *it;
    //             if (dynamic_cast<NBLoadedSUMOTLDef*>(orig) != nullptr) {
    //                 dynamic_cast<NBLoadedSUMOTLDef*>(orig)->registerModifications(addedConnections, removedConnections);
    //             } else if (dynamic_cast<NBOwnTLDef*>(orig) == nullptr) {
    //                 NBTrafficLightDefinition* newDef = new NBOwnTLDef(orig->getID(), orig->getOffset(), orig->getType());
    //                 const std::vector<NBNode*>& nodes = orig->getNodes();
    //                 while (!nodes.empty()) {
    //                     newDef->addNode(nodes.front());
    //                     nodes.front()->removeTrafficLight(orig);
    //                 }
    //                 tlCont.removeFully(orig->getID());
    //                 tlCont.insert(newDef);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn invalidate_tls(&mut self, tlCont: &mut NBTrafficLightLogicCont, addedConnections: bool, removedConnections: bool) {
        // shortcut: the dynamic_cast checks (NBLoadedSUMOTLDef / NBOwnTLDef) need RTTI, and NBTrafficLightDefinition has no type tag yet; the TL-controlled node is left unchanged, upgrade when a type tag or downcast exists
        let _ = (tlCont, addedConnections, removedConnections);
    }

    /// C++: `void NBNode::shiftTLConnectionLaneIndex(NBEdge *, int, int)`
    /// patches loaded signal plans by modifying lane indices above threshold by the given offset
    // void
    // NBNode::shiftTLConnectionLaneIndex(NBEdge* edge, int offset, int threshold) {
    //     for (std::set<NBTrafficLightDefinition*>::iterator it = myTrafficLights.begin(); it != myTrafficLights.end(); ++it) {
    //         (*it)->shiftTLConnectionLaneIndex(edge, offset, threshold);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn shift_tl_connection_lane_index(&mut self, edge: Ptr<NBEdge>, offset: i32, threshold: i32) {
        for tl in self.myTrafficLights.clone() {
            tl.m().shift_tl_connection_lane_index(edge, offset, threshold);
        }
    }

    /// C++: `int NBNode::removeSelfLoops(NBDistrictCont &, NBEdgeCont &, NBTrafficLightLogicCont &)`
    /// Removes edges which are both incoming and outgoing into this node
    // int
    // NBNode::removeSelfLoops(NBDistrictCont& dc, NBEdgeCont& ec, NBTrafficLightLogicCont& tc) {
    //     int ret = 0;
    //     int pos = 0;
    //     EdgeVector::const_iterator j = myIncomingEdges.begin();
    //     while (j != myIncomingEdges.end()) {
    //         // skip edges which are only incoming and not outgoing
    //         if (find(myOutgoingEdges.begin(), myOutgoingEdges.end(), *j) == myOutgoingEdges.end()) {
    //             ++j;
    //             ++pos;
    //             continue;
    //         }
    //         // an edge with both its origin and destination being the current
    //         //  node should be removed
    //         NBEdge* dummy = *j;
    //         WRITE_WARNINGF(TL(" Removing self-looping edge '%'"), dummy->getID());
    //         // get the list of incoming edges connected to the self-loop
    //         EdgeVector incomingConnected = dummy->getIncomingEdges();
    //         // get the list of outgoing edges connected to the self-loop
    //         EdgeVector outgoingConnected = dummy->getConnectedEdges();
    //         // let the self-loop remap its connections
    //         dummy->remapConnections(incomingConnected);
    //         remapRemoved(tc, dummy, incomingConnected, outgoingConnected);
    //         // delete the self-loop
    //         ec.erase(dc, dummy);
    //         j = myIncomingEdges.begin() + pos;
    //         ++ret;
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn remove_self_loops(&mut self, dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, tc: &mut NBTrafficLightLogicCont) -> i32 {
        let mut ret = 0;
        let mut pos: usize = 0;
        let mut j: usize = 0;
        while j < self.myIncomingEdges.len() {
            // skip edges which are only incoming and not outgoing
            let dummy = self.myIncomingEdges[j];
            if !self.myOutgoingEdges.contains(&dummy) {
                j += 1;
                pos += 1;
                continue;
            }
            // an edge with both its origin and destination being the current
            //  node should be removed
            write_warning(&format!(" Removing self-looping edge '{}'", dummy.get_id()));
            // get the list of incoming edges connected to the self-loop
            let incomingConnected = dummy.get_incoming_edges();
            // get the list of outgoing edges connected to the self-loop
            let outgoingConnected = dummy.get_connected_edges();
            // let the self-loop remap its connections
            dummy.m().remap_connections(&incomingConnected);
            self.remap_removed(tc, dummy, &incomingConnected, &outgoingConnected);
            // delete the self-loop
            ec.erase(dc, dummy);
            j = pos;
            ret += 1;
        }
        ret
    }

    /// C++: `void NBNode::reshiftPosition(double, double)`
    /// Applies an offset to the node
    // void
    // NBNode::reshiftPosition(double xoff, double yoff) {
    //     myPosition.add(xoff, yoff, 0);
    //     myPoly.add(xoff, yoff, 0);
    //     for (auto& wacs : myWalkingAreaCustomShapes) {
    //         wacs.shape.add(xoff, yoff, 0);
    //     }
    //     for (auto& c : myCrossings) {
    //         c->customShape.add(xoff, yoff, 0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reshift_position(&mut self, xoff: f64, yoff: f64) {
        let offset = Position::new3(xoff, yoff, 0.);
        self.myPosition.add(&offset);
        self.myPoly.add(&offset);
        for wacs in self.myWalkingAreaCustomShapes.iter_mut() {
            wacs.shape.add(&offset);
        }
        for c in self.myCrossings.iter_mut() {
            c.customShape.add(&offset);
        }
    }

    /// C++: `void NBNode::roundGeometry()`
    /// ensure consistency between input and output geometries
    // void
    // NBNode::roundGeometry() {
    //     myPosition.round(gPrecision);
    //     if (myHaveCustomPoly) {
    //         myPoly.round(gPrecision);
    //     }
    //     for (auto& wacs : myWalkingAreaCustomShapes) {
    //         wacs.shape.round(gPrecision);
    //     }
    //     for (auto& c : myCrossings) {
    //         c->customShape.round(gPrecision);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn round_geometry(&mut self) {
        self.myPosition.round(utils::common::std_defs::g_precision());
        if self.myHaveCustomPoly {
            self.myPoly.round(utils::common::std_defs::g_precision(), true);
        }
        for wacs in self.myWalkingAreaCustomShapes.iter_mut() {
            wacs.shape.round(utils::common::std_defs::g_precision(), true);
        }
        for c in self.myCrossings.iter_mut() {
            c.customShape.round(utils::common::std_defs::g_precision(), true);
        }
    }

    /// C++: `void NBNode::mirrorX()`
    /// mirror coordinates along the x-axis
    // void
    // NBNode::mirrorX() {
    //     myPosition.mul(1, -1);
    //     myPoly.mirrorX();
    //     // mirror pre-computed geometry of crossings and walkingareas
    //     for (auto& c : myCrossings) {
    //         c->customShape.mirrorX();
    //         c->shape.mirrorX();
    //     }
    //     for (auto& wa : myWalkingAreas) {
    //         wa.shape.mirrorX();
    //     }
    //     for (auto& wacs : myWalkingAreaCustomShapes) {
    //         wacs.shape.mirrorX();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn mirror_x(&mut self) {
        self.myPosition.mul_xy(1., -1.);
        self.myPoly.mirror_x();
        // mirror pre-computed geometry of crossings and walkingareas
        for c in self.myCrossings.iter_mut() {
            c.customShape.mirror_x();
            c.shape.mirror_x();
        }
        for wa in self.myWalkingAreas.iter_mut() {
            wa.shape.mirror_x();
        }
        for wacs in self.myWalkingAreaCustomShapes.iter_mut() {
            wacs.shape.mirror_x();
        }
    }

    /// C++: `void NBNode::addIncomingEdge(NBEdge *)`
    /// adds an incoming edge
    // void
    // NBNode::addIncomingEdge(NBEdge* edge) {
    //     assert(edge != 0);
    //     if (find(myIncomingEdges.begin(), myIncomingEdges.end(), edge) == myIncomingEdges.end()) {
    //         myIncomingEdges.push_back(edge);
    //         myAllEdges.push_back(edge);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_incoming_edge(&mut self, edge: Ptr<NBEdge>) {
        assert!(!edge.is_null());
        if !self.myIncomingEdges.contains(&edge) {
            self.myIncomingEdges.push(edge);
            self.myAllEdges.push(edge);
        }
    }

    /// C++: `void NBNode::addOutgoingEdge(NBEdge *)`
    /// adds an outgoing edge
    // void
    // NBNode::addOutgoingEdge(NBEdge* edge) {
    //     assert(edge != 0);
    //     if (find(myOutgoingEdges.begin(), myOutgoingEdges.end(), edge) == myOutgoingEdges.end()) {
    //         myOutgoingEdges.push_back(edge);
    //         myAllEdges.push_back(edge);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_outgoing_edge(&mut self, edge: Ptr<NBEdge>) {
        assert!(!edge.is_null());
        if !self.myOutgoingEdges.contains(&edge) {
            self.myOutgoingEdges.push(edge);
            self.myAllEdges.push(edge);
        }
    }

    /// C++: `void NBNode::computeLanes2Lanes()`
    /// computes the connections of lanes to edges
    // void
    // NBNode::computeLanes2Lanes() {
    //     // special case a):
    //     //  one in, one out, the outgoing has more lanes
    //     if (myIncomingEdges.size() == 1 && myOutgoingEdges.size() == 1) {
    //         NBEdge* in = myIncomingEdges[0];
    //         NBEdge* out = myOutgoingEdges[0];
    //         // check if it's not the turnaround
    //         if (in->getTurnDestination() == out) {
    //             // will be added later or not...
    //             return;
    //         }
    //         int inOffset, inEnd, outOffset, outEnd, addedLanes;
    //         getReduction(out, in, outOffset, outEnd, inOffset, inEnd, addedLanes);
    //         if (in->getStep() <= NBEdge::EdgeBuildingStep::LANES2EDGES
    //                 && addedLanes > 0
    //                 && in->isConnectedTo(out)) {
    //             const int addedRight = addedLanesRight(out, addedLanes);
    //             const int addedLeft = addedLanes - addedRight;
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "l2l node=" << getID() << " specialCase a. addedRight=" << addedRight << " addedLeft=" << addedLeft << " inOff=" << inOffset << " outOff=" << outOffset << " inEnd=" << inEnd << " outEnd=" << outEnd << "\n";
    //             }
    // #endif
    //             // "straight" connections
    //             for (int i = inOffset; i < inEnd; ++i) {
    //                 in->setConnection(i, out, i - inOffset + outOffset + addedRight, NBEdge::Lane2LaneInfoType::COMPUTED);
    //             }
    //             // connect extra lane on the right
    //             for (int i = 0; i < addedRight; ++i) {
    //                 in->setConnection(inOffset, out, outOffset + i, NBEdge::Lane2LaneInfoType::COMPUTED);
    //             }
    //             // connect extra lane on the left
    //             const int inLeftMost = inEnd - 1;;
    //             const int outOffset2 = outOffset + addedRight + inEnd - inOffset;
    //             for (int i = 0; i < addedLeft; ++i) {
    //                 in->setConnection(inLeftMost, out, outOffset2 + i, NBEdge::Lane2LaneInfoType::COMPUTED);
    //             }
    //             if (out->getSpecialLane(SVC_BICYCLE) >= 0) {
    //                 recheckVClassConnections(out);
    //             }
    //             return;
    //         }
    //     }
    //     // special case b):
    //     //  two in, one out, the outgoing has the same number of lanes as the sum of the incoming
    //     //  --> highway on-ramp
    //     if (myIncomingEdges.size() == 2 && myOutgoingEdges.size() == 1) {
    //         NBEdge* const out = myOutgoingEdges[0];
    //         NBEdge* in1 = myIncomingEdges[0];
    //         NBEdge* in2 = myIncomingEdges[1];
    //         const int outOffset = MAX2(0, out->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //         int in1Offset = MAX2(0, in1->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //         int in2Offset = MAX2(0, in2->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //         if (in1->getNumLanes() + in2->getNumLanes() - in1Offset - in2Offset == out->getNumLanes() - outOffset
    //                 && (in1->getStep() <= NBEdge::EdgeBuildingStep::LANES2EDGES)
    //                 && (in2->getStep() <= NBEdge::EdgeBuildingStep::LANES2EDGES)
    //                 && in1 != out
    //                 && in2 != out
    //                 && in1->isConnectedTo(out)
    //                 && in2->isConnectedTo(out)
    //                 && in1->getSpecialLane(SVC_BICYCLE) == -1
    //                 && in2->getSpecialLane(SVC_BICYCLE) == -1
    //                 && out->getSpecialLane(SVC_BICYCLE) == -1
    //                 && in1->getSpecialLane(SVC_TRAM) == -1
    //                 && in2->getSpecialLane(SVC_TRAM) == -1
    //                 && out->getSpecialLane(SVC_TRAM) == -1
    //                 && isLongEnough(out, MIN_WEAVE_LENGTH)) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "l2l node=" << getID() << " specialCase b\n";
    //             }
    // #endif
    //             // for internal: check which one is the rightmost
    //             double a1 = in1->getAngleAtNode(this);
    //             double a2 = in2->getAngleAtNode(this);
    //             double ccw = GeomHelper::getCCWAngleDiff(a1, a2);
    //             double cw = GeomHelper::getCWAngleDiff(a1, a2);
    //             if (ccw > cw) {
    //                 std::swap(in1, in2);
    //                 std::swap(in1Offset, in2Offset);
    //             }
    //             in1->addLane2LaneConnections(in1Offset, out, outOffset, in1->getNumLanes() - in1Offset, NBEdge::Lane2LaneInfoType::COMPUTED, true);
    //             in2->addLane2LaneConnections(in2Offset, out, in1->getNumLanes() + outOffset - in1Offset, in2->getNumLanes() - in2Offset, NBEdge::Lane2LaneInfoType::COMPUTED, true);
    //             if (out->getSpecialLane(SVC_BICYCLE) >= 0) {
    //                 recheckVClassConnections(out);
    //             }
    //             return;
    //         }
    //     }
    //     // special case c):
    //     //  one in, two out, the incoming has the same number of lanes or only 1 lane less than the sum of the outgoing lanes
    //     //  --> highway off-ramp
    //     if (myIncomingEdges.size() == 1 && myOutgoingEdges.size() == 2) {
    //         NBEdge* in = myIncomingEdges[0];
    //         NBEdge* out1 = myOutgoingEdges[0];
    //         NBEdge* out2 = myOutgoingEdges[1];
    //         const int inOffset = MAX2(0, in->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //         int out1Offset = MAX2(0, out1->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //         int out2Offset = MAX2(0, out2->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //         const int deltaLaneSum = (out2->getNumLanes() + out1->getNumLanes() - out1Offset - out2Offset) - (in->getNumLanes() - inOffset);
    //         if ((deltaLaneSum == 0 || (deltaLaneSum == 1 && in->getPermissionVariants(inOffset, in->getNumLanes()).size() == 1))
    //                 && (in->getStep() <= NBEdge::EdgeBuildingStep::LANES2EDGES)
    //                 && in != out1
    //                 && in != out2
    //                 && in->isConnectedTo(out1)
    //                 && in->isConnectedTo(out2)
    //                 && !in->isTurningDirectionAt(out1)
    //                 && !in->isTurningDirectionAt(out2)
    //            ) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "l2l node=" << getID() << " specialCase c\n";
    //             }
    // #endif
    //             // for internal: check which one is the rightmost
    //             if (NBContHelper::relative_outgoing_edge_sorter(in)(out2, out1)) {
    //                 std::swap(out1, out2);
    //                 std::swap(out1Offset, out2Offset);
    //             }
    //             in->addLane2LaneConnections(inOffset, out1, out1Offset, out1->getNumLanes() - out1Offset, NBEdge::Lane2LaneInfoType::COMPUTED, true);
    //             in->addLane2LaneConnections(out1->getNumLanes() + inOffset - out1Offset - deltaLaneSum, out2, out2Offset, out2->getNumLanes() - out2Offset, NBEdge::Lane2LaneInfoType::COMPUTED, false);
    //             if (in->getSpecialLane(SVC_BICYCLE) >= 0) {
    //                 recheckVClassConnections(out1);
    //                 recheckVClassConnections(out2);
    //             }
    //             return;
    //         }
    //     }
    //     // special case d):
    //     //  one in, one out, the outgoing has one lane less and node has type 'zipper'
    //     if (myIncomingEdges.size() == 1 && myOutgoingEdges.size() == 1 && myType == SumoXMLNodeType::ZIPPER) {
    //         NBEdge* in = myIncomingEdges[0];
    //         NBEdge* out = myOutgoingEdges[0];
    //         // check if it's not the turnaround
    //         if (in->getTurnDestination() == out) {
    //             // will be added later or not...
    //             return;
    //         }
    // #ifdef DEBUG_CONNECTION_GUESSING
    //         if (DEBUGCOND) {
    //             std::cout << "l2l node=" << getID() << " specialCase d\n";
    //         }
    // #endif
    //         const int inOffset = MAX2(0, in->getFirstNonPedestrianLaneIndex(FORWARD, true));
    //         const int outOffset = MAX2(0, out->getFirstNonPedestrianLaneIndex(FORWARD, true));
    //         if (in->getStep() <= NBEdge::EdgeBuildingStep::LANES2EDGES
    //                 && in->getNumLanes() - inOffset == out->getNumLanes() - outOffset + 1
    //                 && in != out
    //                 && in->isConnectedTo(out)) {
    //             for (int i = inOffset; i < in->getNumLanes(); ++i) {
    //                 in->setConnection(i, out, MIN2(outOffset + i, out->getNumLanes() - 1), NBEdge::Lane2LaneInfoType::COMPUTED, true);
    //             }
    //             return;
    //         }
    //     }
    //     // special case f):
    //     //  one in, one out, out has reduced or same number of lanes
    //     if (myIncomingEdges.size() == 1 && myOutgoingEdges.size() == 1) {
    //         NBEdge* in = myIncomingEdges[0];
    //         NBEdge* out = myOutgoingEdges[0];
    //         // check if it's not the turnaround
    //         if (in->getTurnDestination() == out) {
    //             // will be added later or not...
    //             return;
    //         }
    //         int inOffset, inEnd, outOffset, outEnd, reduction;
    //         getReduction(in, out, inOffset, inEnd, outOffset, outEnd, reduction);
    //         if (in->getStep() <= NBEdge::EdgeBuildingStep::LANES2EDGES
    //                 && reduction >= 0
    //                 && in != out
    //                 && in->isConnectedTo(out)) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "l2l node=" << getID() << " specialCase f inOff=" << inOffset << " outOff=" << outOffset << " inEnd=" << inEnd << " outEnd=" << outEnd << " reduction=" << reduction << "\n";
    //             }
    // #endif
    //             // in case of reduced lane number, let the rightmost lanes end
    //             inOffset += reduction;
    //             for (int i = outOffset; i < outEnd; ++i) {
    //                 in->setConnection(i + inOffset - outOffset, out, i, NBEdge::Lane2LaneInfoType::COMPUTED);
    //             }
    //             //std::cout << " special case f at node=" << getID() << " inOffset=" << inOffset << " outOffset=" << outOffset << "\n";
    //             recheckVClassConnections(out);
    //             return;
    //         }
    //     }
    //
    //     // go through this node's outgoing edges
    //     //  for every outgoing edge, compute the distribution of the node's
    //     //  incoming edges on this edge when approaching this edge
    //     // the incoming edges' steps will then also be marked as LANE2LANE_RECHECK...
    //     EdgeVector approaching;
    //     for (NBEdge* currentOutgoing : myOutgoingEdges) {
    //         // get the information about edges that do approach this edge
    //         getEdgesThatApproach(currentOutgoing, approaching);
    //         const int numApproaching = (int)approaching.size();
    //         if (numApproaching != 0) {
    //             ApproachingDivider divider(approaching, currentOutgoing);
    //             Bresenham::compute(&divider, numApproaching, divider.numAvailableLanes());
    //         }
    // #ifdef DEBUG_CONNECTION_GUESSING
    //         if (DEBUGCOND) {
    //             std::cout << "l2l node=" << getID() << " outgoing=" << currentOutgoing->getID() << " bresenham:\n";
    //             for (NBEdge* e : myIncomingEdges) {
    //                 const std::vector<NBEdge::Connection>& elv = e->getConnections();
    //                 for (std::vector<NBEdge::Connection>::const_iterator k = elv.begin(); k != elv.end(); ++k) {
    //                     std::cout << "  " << e->getID() << "_" << (*k).fromLane << " -> " << Named::getIDSecure((*k).toEdge) << "_" << (*k).toLane << "\n";
    //                 }
    //             }
    //         }
    // #endif
    //         recheckVClassConnections(currentOutgoing);
    //
    //         // in case of lane change restrictions on the outgoing edge, ensure that
    //         // all its lanes can be reached from each connected incoming edge
    //         bool targetProhibitsChange = false;
    //         for (int i = 0; i < currentOutgoing->getNumLanes(); i++) {
    //             const NBEdge::Lane& lane = currentOutgoing->getLanes()[i];
    //             if ((lane.changeLeft != SVCAll && lane.changeLeft != SVC_IGNORING && i + 1 < currentOutgoing->getNumLanes())
    //                     || (lane.changeRight != SVCAll && lane.changeRight != SVC_IGNORING && i > 0)) {
    //                 targetProhibitsChange = true;
    //                 break;
    //             }
    //         }
    //         if (targetProhibitsChange) {
    //             //std::cout << " node=" << getID() << " outgoing=" << currentOutgoing->getID() << " targetProhibitsChange\n";
    //             for (NBEdge* incoming : myIncomingEdges) {
    //                 if (incoming->getStep() < NBEdge::EdgeBuildingStep::LANES2LANES_DONE) {
    //                     std::map<int, int> outToIn;
    //                     for (const NBEdge::Connection& c : incoming->getConnections()) {
    //                         if (c.toEdge == currentOutgoing) {
    //                             outToIn[c.toLane] = c.fromLane;
    //                         }
    //                     }
    //                     for (int toLane = 0; toLane < currentOutgoing->getNumLanes(); toLane++) {
    //                         if (outToIn.count(toLane) == 0) {
    //                             bool added = false;
    //                             // find incoming lane for neighboring outgoing
    //                             for (int i = 0; i < toLane; i++) {
    //                                 if (outToIn.count(i) != 0) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                                     if (DEBUGCOND) {
    //                                         std::cout << "l2l node=" << getID() << " from=" << incoming->getID() << " to " << currentOutgoing->getLaneID(toLane) << " (changeProhibited, secondTarget)\n";
    //                                     }
    // #endif
    //                                     incoming->setConnection(outToIn[i], currentOutgoing, toLane, NBEdge::Lane2LaneInfoType::COMPUTED);
    //                                     added = true;
    //                                     break;
    //                                 }
    //                             }
    //                             if (!added) {
    //                                 for (int i = toLane; i < currentOutgoing->getNumLanes(); i++) {
    //                                     if (outToIn.count(i) != 0) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                                         if (DEBUGCOND) {
    //                                             std::cout << "l2l node=" << getID() << " from=" << incoming->getID() << " to " << currentOutgoing->getLaneID(toLane) << " (changeProhibited, newTarget)\n";
    //                                         }
    // #endif
    //                                         incoming->setConnection(outToIn[i], currentOutgoing, toLane, NBEdge::Lane2LaneInfoType::COMPUTED);
    //                                         added = true;
    //                                         break;
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // special case e): rail_crossing
    //     // there should only be straight connections here
    //     if (myType == SumoXMLNodeType::RAIL_CROSSING) {
    //         for (EdgeVector::const_iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //             const std::vector<NBEdge::Connection> cons = (*i)->getConnections();
    //             for (std::vector<NBEdge::Connection>::const_iterator k = cons.begin(); k != cons.end(); ++k) {
    //                 if (getDirection(*i, (*k).toEdge) == LinkDirection::TURN) {
    //                     (*i)->removeFromConnections((*k).toEdge);
    //                 }
    //             }
    //         }
    //     }
    //
    //     // ... but we may have the case that there are no outgoing edges
    //     //  In this case, we have to mark the incoming edges as being in state
    //     //   LANE2LANE( not RECHECK) by hand
    //     if (myOutgoingEdges.size() == 0) {
    //         for (NBEdge* incoming : myIncomingEdges) {
    //             incoming->markAsInLane2LaneState();
    //         }
    //     }
    //
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "final connections at " << getID() << "\n";
    //         for (NBEdge* e : myIncomingEdges) {
    //             const std::vector<NBEdge::Connection>& elv = e->getConnections();
    //             for (std::vector<NBEdge::Connection>::const_iterator k = elv.begin(); k != elv.end(); ++k) {
    //                 std::cout << "  " << e->getID() << "_" << (*k).fromLane << " -> " << Named::getIDSecure((*k).toEdge) << "_" << (*k).toLane << "\n";
    //             }
    //         }
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_lanes2_lanes(&mut self) {
        todo!("port NBNode::computeLanes2Lanes")
    }

    /// C++: `void NBNode::computeLogic(const NBEdgeCont &)`
    /// computes the node's type, logic and traffic light
    // void
    // NBNode::computeLogic(const NBEdgeCont& ec) {
    //     delete myRequest; // possibly recomputation step
    //     myRequest = nullptr;
    //     if (myIncomingEdges.size() == 0 || myOutgoingEdges.size() == 0) {
    //         // no logic if nothing happens here
    //         myType = SumoXMLNodeType::DEAD_END;
    //         removeJoinedTrafficLights();
    //         return;
    //     }
    //     // compute the logic if necessary or split the junction
    //     if (myType != SumoXMLNodeType::NOJUNCTION && myType != SumoXMLNodeType::DISTRICT && myType != SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION) {
    //         // build the request
    //         myRequest = new NBRequest(ec, this, myAllEdges, myIncomingEdges, myOutgoingEdges, myBlockedConnections);
    //         // check whether it is not too large
    //         int numConnections = numNormalConnections();
    //         if (numConnections >= SUMO_MAX_CONNECTIONS) {
    //             // yep -> make it untcontrolled, warn
    //             delete myRequest;
    //             myRequest = nullptr;
    //             if (myType == SumoXMLNodeType::TRAFFIC_LIGHT) {
    //                 myType = SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION;
    //             } else {
    //                 myType = SumoXMLNodeType::NOJUNCTION;
    //             }
    //             WRITE_WARNINGF(TL("Junction '%' is too complicated (% connections, max %); will be set to %."),
    //                            getID(), numConnections, SUMO_MAX_CONNECTIONS, toString(myType));
    //         } else if (numConnections == 0) {
    //             delete myRequest;
    //             myRequest = nullptr;
    //             myType = SumoXMLNodeType::DEAD_END;
    //             removeJoinedTrafficLights();
    //         } else {
    //             myRequest->buildBitfieldLogic();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_logic(&mut self, ec: &NBEdgeCont) {
        // delete myRequest (possibly recomputation step)
        if !self.myRequest.is_null() {
            drop(unsafe { Box::from_raw(self.myRequest.as_raw()) });
        }
        self.myRequest = Ptr::null();
        if self.myIncomingEdges.len() == 0 || self.myOutgoingEdges.len() == 0 {
            // no logic if nothing happens here
            self.myType = SumoXMLNodeType::DEAD_END;
            self.remove_joined_traffic_lights();
            return;
        }
        // compute the logic if necessary or split the junction
        if self.myType != SumoXMLNodeType::NOJUNCTION && self.myType != SumoXMLNodeType::DISTRICT && self.myType != SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION {
            // build the request
            self.myRequest = Ptr::from_raw(Box::into_raw(Box::new(NBRequest::new(ec, Ptr::from_mut(self), &self.myAllEdges, &self.myIncomingEdges, &self.myOutgoingEdges, &self.myBlockedConnections))));
            // check whether it is not too large
            let numConnections = self.num_normal_connections();
            if numConnections >= utils::common::std_defs::SUMO_MAX_CONNECTIONS as i32 {
                // yep -> make it untcontrolled, warn
                if !self.myRequest.is_null() {
                    drop(unsafe { Box::from_raw(self.myRequest.as_raw()) });
                }
                self.myRequest = Ptr::null();
                if self.myType == SumoXMLNodeType::TRAFFIC_LIGHT {
                    self.myType = SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION;
                } else {
                    self.myType = SumoXMLNodeType::NOJUNCTION;
                }
                write_warning(&format!("Junction '{}' is too complicated ({} connections, max {}); will be set to {}.",
                    self.get_id(), numConnections, utils::common::std_defs::SUMO_MAX_CONNECTIONS, NodeTypes.get_string(self.myType)));
            } else if numConnections == 0 {
                if !self.myRequest.is_null() {
                    drop(unsafe { Box::from_raw(self.myRequest.as_raw()) });
                }
                self.myRequest = Ptr::null();
                self.myType = SumoXMLNodeType::DEAD_END;
                self.remove_joined_traffic_lights();
            } else {
                self.myRequest.m().build_bitfield_logic();
            }
        }
    }

    /// C++: `void NBNode::computeLogic2(bool)`
    /// compute right-of-way logic for all lane-to-lane connections
    // void
    // NBNode::computeLogic2(bool checkLaneFoes) {
    //     if (myRequest != nullptr) {
    //         myRequest->computeLogic(checkLaneFoes);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_logic2(&mut self, checkLaneFoes: bool) {
        if !self.myRequest.is_null() {
            self.myRequest.m().compute_logic(checkLaneFoes);
        }
    }

    /// C++: `void NBNode::computeKeepClear()`
    /// compute keepClear status for all connections
    // void
    // NBNode::computeKeepClear() {
    //     if (hasConflict()) {
    //         if (!myKeepClear) {
    //             for (NBEdge* incoming : myIncomingEdges) {
    //                 std::vector<NBEdge::Connection>& connections = incoming->getConnections();
    //                 for (NBEdge::Connection& c : connections) {
    //                     c.keepClear = KEEPCLEAR_FALSE;
    //                 }
    //             }
    //         } else if (geometryLike() && myCrossings.size() == 0 && !isTLControlled()) {
    //             int linkIndex = 0;
    //             for (NBEdge* incoming : myIncomingEdges) {
    //                 std::vector<NBEdge::Connection>& connections = incoming->getConnections();
    //                 for (NBEdge::Connection& c : connections) {
    //                     if (c.keepClear == KEEPCLEAR_UNSPECIFIED && myRequest->hasConflictAtLink(linkIndex)) {
    //                         const LinkState linkState = getLinkState(incoming, c.toEdge, c.fromLane, c.toLane, c.mayDefinitelyPass, c.tlID);
    //                         if (linkState == LINKSTATE_MAJOR) {
    //                             c.keepClear = KEEPCLEAR_FALSE;
    //                         }
    //                     }
    //                 }
    //                 linkIndex++;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_keep_clear(&mut self) {
        todo!("port NBNode::computeKeepClear")
    }

    /// C++: `bool NBNode::writeLogic(OutputDevice &) const` [const]
    /// writes the XML-representation of the logic as a bitset-logic XML representation
    // bool
    // NBNode::writeLogic(OutputDevice& into) const {
    //     if (myRequest) {
    //         myRequest->writeLogic(into);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn write_logic(&self, into: &mut OutputDevice) -> bool {
        if !self.myRequest.is_null() {
            self.myRequest.write_logic(into);
            return true;
        }
        false
    }

    /// C++: `const std::string NBNode::getFoes(int) const` [const]
    /// get the 'foes' string (conflict bit set) of the right-of-way logic
    // const std::string
    // NBNode::getFoes(int linkIndex) const {
    //     if (myRequest == nullptr) {
    //         return "";
    //     } else {
    //         return myRequest->getFoes(linkIndex);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_foes(&self, linkIndex: i32) -> String {
        if self.myRequest.is_null() {
            String::new()
        } else {
            self.myRequest.get_foes(linkIndex).to_string()
        }
    }

    /// C++: `const std::string NBNode::getResponse(int) const` [const]
    /// get the 'response' string (right-of-way bit set) of the right-of-way logic
    // const std::string
    // NBNode::getResponse(int linkIndex) const {
    //     if (myRequest == nullptr) {
    //         return "";
    //     } else {
    //         return myRequest->getResponse(linkIndex);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_response(&self, linkIndex: i32) -> String {
        if self.myRequest.is_null() {
            String::new()
        } else {
            self.myRequest.get_response(linkIndex).to_string()
        }
    }

    /// C++: `bool NBNode::hasConflict() const` [const]
    /// whether there are conflicting streams of traffic at this node
    // bool
    // NBNode::hasConflict() const {
    //     if (myRequest == nullptr) {
    //         return false;
    //     } else {
    //         return myRequest->hasConflict();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn has_conflict(&self) -> bool {
        if self.myRequest.is_null() {
            false
        } else {
            self.myRequest.has_conflict()
        }
    }

    /// C++: `bool NBNode::hasConflict(const NBEdge *) const` [const]
    /// whether the given edge has a conflicting stream of traffic at this node
    // bool
    // NBNode::hasConflict(const NBEdge* e) const {
    //     if (myRequest == nullptr) {
    //         return false;
    //     }
    //     for (const auto& con : e->getConnections()) {
    //         const int index = getConnectionIndex(e, con);
    //         if (myRequest->hasConflictAtLink(index)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_conflict_2(&self, e: Ptr<NBEdge>) -> bool {
        if self.myRequest.is_null() {
            return false;
        }
        for con in e.get_connections().iter() {
            let index = self.get_connection_index(e, con);
            if self.myRequest.has_conflict_at_link(index) {
                return true;
            }
        }
        false
    }

    /// C++: `Position NBNode::getEmptyDir() const` [const]
    /// Returns something like the most unused direction Should only be used to add source or sink nodes
    // Position
    // NBNode::getEmptyDir() const {
    //     Position pos(0, 0);
    //     for (const NBEdge* const in : myIncomingEdges) {
    //         Position toAdd = in->getFromNode()->getPosition();
    //         toAdd.sub(myPosition);
    //         toAdd.norm2D();
    //         pos.add(toAdd);
    //     }
    //     for (const NBEdge* const out : myOutgoingEdges) {
    //         Position toAdd = out->getToNode()->getPosition();
    //         toAdd.sub(myPosition);
    //         toAdd.norm2D();
    //         pos.add(toAdd);
    //     }
    //     pos.mul(-1. / (double)(myIncomingEdges.size() + myOutgoingEdges.size()));
    //     if (pos.x() == 0. && pos.y() == 0.) {
    //         pos = Position(1, 0);
    //     }
    //     pos.norm2D();
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn get_empty_dir(&self) -> Position {
        let mut pos = Position::new(0., 0.);
        for e_in in self.myIncomingEdges.iter() {
            let mut toAdd = e_in.get_from_node().get_position().clone();
            toAdd.sub(&self.myPosition);
            toAdd.norm2d();
            pos.add(&toAdd);
        }
        for e_out in self.myOutgoingEdges.iter() {
            let mut toAdd = e_out.get_to_node().get_position().clone();
            toAdd.sub(&self.myPosition);
            toAdd.norm2d();
            pos.add(&toAdd);
        }
        pos.mul(-1. / ((self.myIncomingEdges.len() + self.myOutgoingEdges.len()) as f64));
        if pos.x() == 0. && pos.y() == 0. {
            pos = Position::new(1., 0.);
        }
        pos.norm2d();
        pos
    }

    /// C++: `bool NBNode::hasIncoming(const NBEdge *const) const` [const]
    /// Returns whether the given edge ends at this node
    // bool
    // NBNode::hasIncoming(const NBEdge* const e) const {
    //     return std::find(myIncomingEdges.begin(), myIncomingEdges.end(), e) != myIncomingEdges.end();
    // }
    #[allow(unused_variables)]
    pub fn has_incoming(&self, e: Ptr<NBEdge>) -> bool {
        self.myIncomingEdges.contains(&e)
    }

    /// C++: `bool NBNode::hasOutgoing(const NBEdge *const) const` [const]
    /// Returns whether the given edge starts at this node
    // bool
    // NBNode::hasOutgoing(const NBEdge* const e) const {
    //     return std::find(myOutgoingEdges.begin(), myOutgoingEdges.end(), e) != myOutgoingEdges.end();
    // }
    #[allow(unused_variables)]
    pub fn has_outgoing(&self, e: Ptr<NBEdge>) -> bool {
        self.myOutgoingEdges.contains(&e)
    }

    /// C++: `NBEdge * NBNode::getOppositeIncoming(NBEdge *) const` [const]
    /// returns the opposite incoming edge of certain edge
    // NBEdge*
    // NBNode::getOppositeIncoming(NBEdge* e) const {
    //     EdgeVector edges = myIncomingEdges;
    //     if (find(edges.begin(), edges.end(), e) != edges.end()) {
    //         edges.erase(find(edges.begin(), edges.end(), e));
    //     }
    //     if (edges.size() == 0) {
    //         return nullptr;
    //     }
    //     if (e->getToNode() == this) {
    //         sort(edges.begin(), edges.end(), NBContHelper::edge_opposite_direction_sorter(e, this, false));
    //     } else {
    //         sort(edges.begin(), edges.end(), NBContHelper::edge_similar_direction_sorter(e));
    //     }
    //     return edges[0];
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_incoming(&self, e: Ptr<NBEdge>) -> Ptr<NBEdge> {
        let me = Ptr::new(self);
        let mut edges: EdgeVector = self.myIncomingEdges.clone();
        if let Some(pos) = edges.iter().position(|x| *x == e) {
            edges.remove(pos);
        }
        if edges.is_empty() {
            return Ptr::null();
        }
        if e.get_to_node() == me {
            let sorter = NBContHelper_edge_opposite_direction_sorter::new(e, me, false);
            sort_edges_by(&mut edges, |a, b| sorter.call_op(a, b) != 0);
        } else {
            let sorter = NBContHelper_edge_similar_direction_sorter::new(e, true);
            sort_edges_by(&mut edges, |a, b| sorter.call_op(a, b) != 0);
        }
        edges[0]
    }

    /// C++: `void NBNode::invalidateIncomingConnections(bool)`
    /// invalidate incoming connections
    // void
    // NBNode::invalidateIncomingConnections(bool reallowSetting) {
    //     for (EdgeVector::const_iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //         (*i)->invalidateConnections(reallowSetting);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn invalidate_incoming_connections(&mut self, reallowSetting: bool) {
        for i in self.myIncomingEdges.iter() {
            i.m().invalidate_connections(reallowSetting);
        }
    }

    /// C++: `void NBNode::invalidateOutgoingConnections(bool)`
    /// invalidate outgoing connections
    // void
    // NBNode::invalidateOutgoingConnections(bool reallowSetting) {
    //     for (EdgeVector::const_iterator i = myOutgoingEdges.begin(); i != myOutgoingEdges.end(); i++) {
    //         (*i)->invalidateConnections(reallowSetting);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn invalidate_outgoing_connections(&mut self, reallowSetting: bool) {
        for i in self.myOutgoingEdges.iter() {
            i.m().invalidate_connections(reallowSetting);
        }
    }

    /// C++: `void NBNode::removeDoubleEdges()`
    /// remove duble edges
    // void
    // NBNode::removeDoubleEdges() {
    //     // check incoming
    //     for (int i = 0; myIncomingEdges.size() > 0 && i < (int)myIncomingEdges.size() - 1; i++) {
    //         int j = i + 1;
    //         while (j < (int)myIncomingEdges.size()) {
    //             if (myIncomingEdges[i] == myIncomingEdges[j]) {
    //                 myIncomingEdges.erase(myIncomingEdges.begin() + j);
    //             } else {
    //                 j++;
    //             }
    //         }
    //     }
    //     // check outgoing
    //     for (int i = 0; myOutgoingEdges.size() > 0 && i < (int)myOutgoingEdges.size() - 1; i++) {
    //         int j = i + 1;
    //         while (j < (int)myOutgoingEdges.size()) {
    //             if (myOutgoingEdges[i] == myOutgoingEdges[j]) {
    //                 myOutgoingEdges.erase(myOutgoingEdges.begin() + j);
    //             } else {
    //                 j++;
    //             }
    //         }
    //     }
    //     // check all
    //     for (int i = 0; myAllEdges.size() > 0 && i < (int)myAllEdges.size() - 1; i++) {
    //         int j = i + 1;
    //         while (j < (int)myAllEdges.size()) {
    //             if (myAllEdges[i] == myAllEdges[j]) {
    //                 myAllEdges.erase(myAllEdges.begin() + j);
    //             } else {
    //                 j++;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_double_edges(&mut self) {
        // C++ loop: for each i, erase every later duplicate of myX[i] (size re-checked each pass)
        fn remove_double_entries(v: &mut EdgeVector) {
            let mut i = 0;
            while !v.is_empty() && i + 1 < v.len() {
                let mut j = i + 1;
                while j < v.len() {
                    if v[i] == v[j] {
                        v.remove(j);
                    } else {
                        j += 1;
                    }
                }
                i += 1;
            }
        }
        // check incoming
        remove_double_entries(&mut self.myIncomingEdges);
        // check outgoing
        remove_double_entries(&mut self.myOutgoingEdges);
        // check all
        remove_double_entries(&mut self.myAllEdges);
    }

    /// C++: `NBEdge * NBNode::getConnectionTo(NBNode *) const` [const]
    /// get connection to certain node
    // NBEdge*
    // NBNode::getConnectionTo(NBNode* n) const {
    //     for (NBEdge* e : myOutgoingEdges) {
    //         if (e->getToNode() == n && e->getPermissions() != 0) {
    //             return e;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_connection_to(&self, n: Ptr<NBNode>) -> Ptr<NBEdge> {
        for e in self.myOutgoingEdges.iter() {
            if e.get_to_node() == n && e.get_permissions(-1) != 0 {
                return *e;
            }
        }
        Ptr::null()
    }

    /// C++: `void NBNode::addSortedLinkFoes(const NBConnection &, const NBConnection &)`
    /// add shorted link FOES
    // void
    // NBNode::addSortedLinkFoes(const NBConnection& mayDrive,
    //                           const NBConnection& mustStop) {
    //     if (mayDrive.getFrom() == nullptr ||
    //             mayDrive.getTo() == nullptr ||
    //             mustStop.getFrom() == nullptr ||
    //             mustStop.getTo() == nullptr) {
    //
    //         WRITE_WARNING(TL("Something went wrong during the building of a connection..."));
    //         return; // !!! mark to recompute connections
    //     }
    //     NBConnectionVector conn = myBlockedConnections[mustStop];
    //     conn.push_back(mayDrive);
    //     myBlockedConnections[mustStop] = conn;
    // }
    #[allow(unused_variables)]
    pub fn add_sorted_link_foes(&mut self, mayDrive: &NBConnection, mustStop: &NBConnection) {
        todo!("port NBNode::addSortedLinkFoes")
    }

    /// C++: `NBEdge * NBNode::getPossiblySplittedIncoming(const std::string &)`
    /// get possibly splitted incoming edge
    // NBEdge*
    // NBNode::getPossiblySplittedIncoming(const std::string& edgeid) {
    //     int size = (int) edgeid.length();
    //     for (EdgeVector::iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //         std::string id = (*i)->getID();
    //         if (id.substr(0, size) == edgeid) {
    //             return *i;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_possibly_splitted_incoming(&mut self, edgeid: &str) -> Ptr<NBEdge> {
        let size = edgeid.len();
        for e in self.myIncomingEdges.iter() {
            // C++ id.substr(0, size) == edgeid (byte-wise, so no char-boundary panic)
            if e.get_id().as_bytes().get(..size) == Some(edgeid.as_bytes()) {
                return *e;
            }
        }
        Ptr::null()
    }

    /// C++: `NBEdge * NBNode::getPossiblySplittedOutgoing(const std::string &)`
    /// get possibly splitted outgoing edge
    // NBEdge*
    // NBNode::getPossiblySplittedOutgoing(const std::string& edgeid) {
    //     int size = (int) edgeid.length();
    //     for (EdgeVector::iterator i = myOutgoingEdges.begin(); i != myOutgoingEdges.end(); i++) {
    //         std::string id = (*i)->getID();
    //         if (id.substr(0, size) == edgeid) {
    //             return *i;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_possibly_splitted_outgoing(&mut self, edgeid: &str) -> Ptr<NBEdge> {
        let size = edgeid.len();
        for e in self.myOutgoingEdges.iter() {
            // C++ id.substr(0, size) == edgeid (byte-wise, so no char-boundary panic)
            if e.get_id().as_bytes().get(..size) == Some(edgeid.as_bytes()) {
                return *e;
            }
        }
        Ptr::null()
    }

    /// C++: `void NBNode::removeEdge(NBEdge *, bool)`
    /// Removes edge from this node and optionally removes connections as well
    // void
    // NBNode::removeEdge(NBEdge* edge, bool removeFromConnections) {
    //     EdgeVector::iterator i = std::find(myAllEdges.begin(), myAllEdges.end(), edge);
    //     if (i != myAllEdges.end()) {
    //         myAllEdges.erase(i);
    //         i = std::find(myOutgoingEdges.begin(), myOutgoingEdges.end(), edge);
    //         if (i != myOutgoingEdges.end()) {
    //             myOutgoingEdges.erase(i);
    //             // potential self-loop
    //             i = std::find(myIncomingEdges.begin(), myIncomingEdges.end(), edge);
    //             if (i != myIncomingEdges.end()) {
    //                 myIncomingEdges.erase(i);
    //             }
    //         } else {
    //             i = std::find(myIncomingEdges.begin(), myIncomingEdges.end(), edge);
    //             if (i != myIncomingEdges.end()) {
    //                 myIncomingEdges.erase(i);
    //             } else {
    //                 // edge must have been either incoming or outgoing
    //                 assert(false);
    //             }
    //         }
    //         if (removeFromConnections) {
    //             for (i = myAllEdges.begin(); i != myAllEdges.end(); ++i) {
    //                 (*i)->removeFromConnections(edge);
    //             }
    //         }
    //         // invalidate controlled connections for loaded traffic light plans
    //         const bool incoming = edge->getToNode() == this;
    //         for (NBTrafficLightDefinition* const tld : myTrafficLights) {
    //             tld->replaceRemoved(edge, -1, nullptr, -1, incoming);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_edge(&mut self, edge: Ptr<NBEdge>, removeFromConnections: bool) {
        if let Some(i) = self.myAllEdges.iter().position(|e| *e == edge) {
            self.myAllEdges.remove(i);
            if let Some(j) = self.myOutgoingEdges.iter().position(|e| *e == edge) {
                self.myOutgoingEdges.remove(j);
                // potential self-loop
                if let Some(k) = self.myIncomingEdges.iter().position(|e| *e == edge) {
                    self.myIncomingEdges.remove(k);
                }
            } else if let Some(k) = self.myIncomingEdges.iter().position(|e| *e == edge) {
                self.myIncomingEdges.remove(k);
            } else {
                // edge must have been either incoming or outgoing
                debug_assert!(false, "edge must have been either incoming or outgoing");
            }
            if removeFromConnections {
                for e in self.myAllEdges.iter() {
                    e.m().remove_from_connections(edge, -1, -1, false, false, false);
                }
            }
            // invalidate controlled connections for loaded traffic light plans
            let incoming = edge.get_to_node() == Ptr::new(self);
            for tld in self.myTrafficLights.iter() {
                tld.m().replace_removed(edge, -1, Ptr::null(), -1, incoming);
            }
        }
    }

    /// C++: `bool NBNode::isLeftMover(const NBEdge *const, const NBEdge *const) const` [const]
    /// Computes whether the given connection is a left mover across the junction
    // bool
    // NBNode::isLeftMover(const NBEdge* const from, const NBEdge* const to) const {
    //     // when the junction has only one incoming edge, there are no
    //     //  problems caused by left blockings
    //     if (myIncomingEdges.size() == 1 || myOutgoingEdges.size() == 1) {
    //         return false;
    //     }
    //     double fromAngle = from->getAngleAtNode(this);
    //     double toAngle = to->getAngleAtNode(this);
    //     double cw = GeomHelper::getCWAngleDiff(fromAngle, toAngle);
    //     double ccw = GeomHelper::getCCWAngleDiff(fromAngle, toAngle);
    //     std::vector<NBEdge*>::const_iterator i = std::find(myAllEdges.begin(), myAllEdges.end(), from);
    //     do {
    //         NBContHelper::nextCW(myAllEdges, i);
    //     } while ((!hasOutgoing(*i) || from->isTurningDirectionAt(*i)) && *i != from);
    //     return cw < ccw && (*i) == to && myOutgoingEdges.size() > 2;
    // }
    #[allow(unused_variables)]
    pub fn is_left_mover(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>) -> bool {
        // when the junction has only one incoming edge, there are no
        //  problems caused by left blockings
        if self.myIncomingEdges.len() == 1 || self.myOutgoingEdges.len() == 1 {
            return false;
        }
        let me = Ptr::new(self);
        let fromAngle = from.get_angle_at_node(me);
        let toAngle = to.get_angle_at_node(me);
        let cw = utils::geom::geom_helper::get_cw_angle_diff(fromAngle, toAngle);
        let ccw = utils::geom::geom_helper::get_ccw_angle_diff(fromAngle, toAngle);
        // C++ std::find; `from` is always an edge of this node when called
        let mut i = self.myAllEdges.iter().position(|e| *e == from).expect("from must be in myAllEdges");
        loop {
            NBContHelper::next_cw(&self.myAllEdges, &mut i);
            let cur = self.myAllEdges[i];
            if !((!self.has_outgoing(cur) || from.is_turning_direction_at(cur)) && cur != from) {
                break;
            }
        }
        cw < ccw && self.myAllEdges[i] == to && self.myOutgoingEdges.len() > 2
    }

    /// C++: `bool NBNode::mustBrake(const NBEdge *const, const NBEdge *const, int, int, bool) const` [const]
    /// Returns the information whether the described flow must let any other flow pass
    // bool
    // NBNode::mustBrake(const NBEdge* const from, const NBEdge* const to, int fromLane, int toLane, bool includePedCrossings) const {
    //     // unregulated->does not need to brake
    //     if (myRequest == nullptr) {
    //         return false;
    //     }
    //     // vehicles which do not have a following lane must always decelerate to the end
    //     if (to == nullptr) {
    //         return true;
    //     }
    //     // maybe we need to brake due to entering a bidi-edge
    //     if (to->isBidiEdge() && !from->isBidiEdge()) {
    //         return true;
    //     }
    //     // check whether any other connection on this node prohibits this connection
    //     return myRequest->mustBrake(from, to, fromLane, toLane, includePedCrossings);
    // }
    #[allow(unused_variables)]
    pub fn must_brake(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>, fromLane: i32, toLane: i32, includePedCrossings: bool) -> bool {
        todo!("port NBNode::mustBrake")
    }

    /// C++: `bool NBNode::mustBrakeForCrossing(const NBEdge *const, const NBEdge *const, const Crossing &) const` [const]
    /// Returns the information whether the described flow must brake for the given crossing
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn must_brake_for_crossing(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>, crossing: &NBNode_Crossing) -> bool {
        todo!("port NBNode::mustBrakeForCrossing")
    }

    /// C++: `bool NBNode::brakeForCrossingOnExit(const NBEdge *, LinkDirection, bool) const` [const]
    /// whether a connection to the given edge must brake for a crossing when leaving the intersection
    // bool
    // NBNode::brakeForCrossingOnExit(const NBEdge* to, LinkDirection dir, bool indirect) const {
    //     // code is called for connections exiting after an internal junction.
    //     // If the connection is turning we do not check for crossing priority anymore.
    //     if (dir == LinkDirection::STRAIGHT && !indirect) {
    //         return false;
    //     }
    //     for (auto& c : myCrossings) {
    //         if (std::find(c->edges.begin(), c->edges.end(), to) != c->edges.end()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn brake_for_crossing_on_exit(&self, to: Ptr<NBEdge>, dir: LinkDirection, indirect: bool) -> bool {
        // code is called for connections exiting after an internal junction.
        // If the connection is turning we do not check for crossing priority anymore.
        if dir == LinkDirection::STRAIGHT && !indirect {
            return false;
        }
        for c in self.myCrossings.iter() {
            if c.edges.contains(&to) {
                return true;
            }
        }
        false
    }

    /// C++: `bool NBNode::rightTurnConflict(const NBEdge *, const NBEdge *, int, const NBEdge *, const NBEdge *, int)` [static]
    /// return whether the given laneToLane connection is a right turn which must yield to a bicycle crossings
    // bool
    // NBNode::rightTurnConflict(const NBEdge* from, const NBEdge* to, int fromLane,
    //                           const NBEdge* prohibitorFrom, const NBEdge* prohibitorTo, int prohibitorFromLane) {
    //     if (from != prohibitorFrom) {
    //         return false;
    //     }
    //     if (from->isTurningDirectionAt(to)
    //             || prohibitorFrom->isTurningDirectionAt(prohibitorTo)) {
    //         // XXX should warn if there are any non-turning connections left of this
    //         return false;
    //     }
    //     // conflict if to is between prohibitorTo and from when going clockwise
    //     if (to->getStartAngle() == prohibitorTo->getStartAngle()) {
    //         // reduce rounding errors
    //         return false;
    //     }
    //     const LinkDirection d1 = from->getToNode()->getDirection(from, to);
    //     // must be a right turn to qualify as rightTurnConflict
    //     if (d1 == LinkDirection::STRAIGHT) {
    //         // no conflict for straight going connections
    //         // XXX actually this should check the main direction (which could also
    //         // be a turn)
    //         return false;
    //     } else {
    //         const LinkDirection d2 = prohibitorFrom->getToNode()->getDirection(prohibitorFrom, prohibitorTo);
    //         /* std::cout
    //             << "from=" << from->getID() << " to=" << to->getID() << " fromLane=" << fromLane
    //             << " pFrom=" << prohibitorFrom->getID() << " pTo=" << prohibitorTo->getID() << " pFromLane=" << prohibitorFromLane
    //             << " d1=" << toString(d1) << " d2=" << toString(d2)
    //             << "\n"; */
    //         bool flip = false;
    //         if (d1 == LinkDirection::LEFT || d1 == LinkDirection::PARTLEFT) {
    //             // check for leftTurnConflicht
    //             flip = !flip;
    //             if (d2 == LinkDirection::RIGHT || d2 == LinkDirection::PARTRIGHT) {
    //                 // assume that the left-turning bicycle goes straight at first
    //                 // and thus gets precedence over a right turning vehicle
    //                 return false;
    //             }
    //         }
    //         if ((!flip && fromLane <= prohibitorFromLane) ||
    //                 (flip && fromLane >= prohibitorFromLane)) {
    //             return false;
    //         }
    //         const double toAngleAtNode = fmod(to->getStartAngle() + 180, (double)360.0);
    //         const double prohibitorToAngleAtNode = fmod(prohibitorTo->getStartAngle() + 180, (double)360.0);
    //         return (flip != (GeomHelper::getCWAngleDiff(from->getEndAngle(), toAngleAtNode) <
    //                          GeomHelper::getCWAngleDiff(from->getEndAngle(), prohibitorToAngleAtNode)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn right_turn_conflict(from: Ptr<NBEdge>, to: Ptr<NBEdge>, fromLane: i32, prohibitorFrom: Ptr<NBEdge>, prohibitorTo: Ptr<NBEdge>, prohibitorFromLane: i32) -> bool {
        todo!("port NBNode::rightTurnConflict")
    }

    /// C++: `bool NBNode::mergeConflictYields(const NBEdge *, int, int, NBEdge *, int) const` [const]
    /// whether one of multple connections from the same edge targeting the same lane must yield
    // bool
    // NBNode::mergeConflictYields(const NBEdge* from, int fromLane, int fromLaneFoe, NBEdge* to, int toLane) const {
    //     if (myRequest == nullptr) {
    //         return false;
    //     }
    //     const NBEdge::Connection& con = from->getConnection(fromLane, to, toLane);
    //     const NBEdge::Connection& prohibitorCon = from->getConnection(fromLaneFoe, to, toLane);
    //     return myRequest->mergeConflict(from, con, from, prohibitorCon, false);
    // }
    #[allow(unused_variables)]
    pub fn merge_conflict_yields(&self, from: Ptr<NBEdge>, fromLane: i32, fromLaneFoe: i32, to: Ptr<NBEdge>, toLane: i32) -> bool {
        todo!("port NBNode::mergeConflictYields")
    }

    /// C++: `bool NBNode::mergeConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &, bool) const` [const]
    /// whether multiple connections from the same edge target the same lane
    // bool
    // NBNode::mergeConflict(const NBEdge* from, const NBEdge::Connection& con,
    //                       const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon, bool foes) const {
    //     if (myRequest == nullptr) {
    //         return false;
    //     }
    //     return myRequest->mergeConflict(from, con, prohibitorFrom, prohibitorCon, foes);
    // }
    #[allow(unused_variables)]
    pub fn merge_conflict(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection, foes: bool) -> bool {
        todo!("port NBNode::mergeConflict")
    }

    /// C++: `bool NBNode::bidiConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &, bool) const` [const]
    /// whether the foe connections is oncoming on the same lane
    // bool
    // NBNode::bidiConflict(const NBEdge* from, const NBEdge::Connection& con,
    //                      const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon, bool foes) const {
    //     if (myRequest == nullptr) {
    //         return false;
    //     }
    //     return myRequest->bidiConflict(from, con, prohibitorFrom, prohibitorCon, foes);
    // }
    #[allow(unused_variables)]
    pub fn bidi_conflict(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection, foes: bool) -> bool {
        todo!("port NBNode::bidiConflict")
    }

    /// C++: `bool NBNode::zipperConflict(const NBEdge *, const NBEdge *, int, int) const` [const]
    // bool
    // NBNode::zipperConflict(const NBEdge* incoming, const NBEdge* outgoing, int fromLane, int toLane) const {
    //     if (mustBrake(incoming, outgoing, fromLane, toLane, false)) {
    //         // there should be another connection with the same target (not just some intersecting trajectories)
    //         for (const NBEdge* in : getIncomingEdges()) {
    //             for (const NBEdge::Connection& c : in->getConnections()) {
    //                 if ((in != incoming || c.fromLane != fromLane) && c.toEdge == outgoing && c.toLane == toLane) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn zipper_conflict(&self, incoming: Ptr<NBEdge>, outgoing: Ptr<NBEdge>, fromLane: i32, toLane: i32) -> bool {
        todo!("port NBNode::zipperConflict")
    }

    /// C++: `bool NBNode::turnFoes(const NBEdge *, const NBEdge *, int, const NBEdge *, const NBEdge *, int, bool) const` [const]
    /// return whether the given laneToLane connection originate from the same edge and are in conflict due to turning across each other
    // bool
    // NBNode::turnFoes(const NBEdge* from, const NBEdge* to, int fromLane,
    //                  const NBEdge* from2, const NBEdge* to2, int fromLane2,
    //                  bool lefthand) const {
    //     UNUSED_PARAMETER(lefthand);
    //     if (from != from2 || to == to2 || fromLane == fromLane2) {
    //         return false;
    //     }
    //     if (from->isTurningDirectionAt(to)
    //             || from2->isTurningDirectionAt(to2)) {
    //         // XXX should warn if there are any non-turning connections left of this
    //         return false;
    //     }
    //     bool result = false;
    //     EdgeVector::const_iterator it = std::find(myAllEdges.begin(), myAllEdges.end(), from);
    //     if (fromLane < fromLane2) {
    //         // conflict if 'to' comes before 'to2' going clockwise starting at 'from'
    //         while (*it != to2) {
    //             if (*it == to) {
    //                 result = true;
    //             }
    //             NBContHelper::nextCW(myAllEdges, it);
    //         }
    //     } else {
    //         // conflict if 'to' comes before 'to2' going counter-clockwise starting at 'from'
    //         while (*it != to2) {
    //             if (*it == to) {
    //                 result = true;
    //             }
    //             NBContHelper::nextCCW(myAllEdges, it);
    //         }
    //     }
    //     /*
    //     if (result) {
    //         std::cout << "turnFoes node=" << getID()
    //         << " from=" << from->getLaneID(fromLane)
    //         << " to=" << to->getID()
    //         << " from2=" << from2->getLaneID(fromLane2)
    //         << " to2=" << to2->getID()
    //         << "\n";
    //     }
    //     */
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn turn_foes(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>, fromLane: i32, from2: Ptr<NBEdge>, to2: Ptr<NBEdge>, fromLane2: i32, lefthand: bool) -> bool {
        todo!("port NBNode::turnFoes")
    }

    /// C++: `bool NBNode::forbids(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const, bool) const` [const]
    /// Returns the information whether "prohibited" flow must let "prohibitor" flow pass
    // bool
    // NBNode::forbids(const NBEdge* const possProhibitorFrom, const NBEdge* const possProhibitorTo,
    //                 const NBEdge* const possProhibitedFrom, const NBEdge* const possProhibitedTo,
    //                 bool regardNonSignalisedLowerPriority) const {
    //     return myRequest != nullptr && myRequest->forbids(possProhibitorFrom, possProhibitorTo,
    //             possProhibitedFrom, possProhibitedTo,
    //             regardNonSignalisedLowerPriority);
    // }
    #[allow(unused_variables)]
    pub fn forbids(&self, possProhibitorFrom: Ptr<NBEdge>, possProhibitorTo: Ptr<NBEdge>, possProhibitedFrom: Ptr<NBEdge>, possProhibitedTo: Ptr<NBEdge>, regardNonSignalisedLowerPriority: bool) -> bool {
        todo!("port NBNode::forbids")
    }

    /// C++: `bool NBNode::foes(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const) const` [const]
    /// Returns the information whether the given flows cross
    // bool
    // NBNode::foes(const NBEdge* const from1, const NBEdge* const to1,
    //              const NBEdge* const from2, const NBEdge* const to2) const {
    //     return myRequest != nullptr && myRequest->foes(from1, to1, from2, to2);
    // }
    #[allow(unused_variables)]
    pub fn foes(&self, from1: Ptr<NBEdge>, to1: Ptr<NBEdge>, from2: Ptr<NBEdge>, to2: Ptr<NBEdge>) -> bool {
        todo!("port NBNode::foes")
    }

    /// C++: `LinkDirection NBNode::getDirection(const NBEdge *const, const NBEdge *const, bool) const` [const]
    /// Returns the representation of the described stream's direction
    // LinkDirection
    // NBNode::getDirection(const NBEdge* const incoming, const NBEdge* const outgoing, bool leftHand) const {
    //     // ok, no connection at all -> dead end
    //     if (outgoing == nullptr) {
    //         return LinkDirection::NODIR;
    //     }
    //     assert(incoming->getToNode() == this);
    //     assert(outgoing->getFromNode() == this);
    //     if (incoming->getJunctionPriority(this) == NBEdge::JunctionPriority::ROUNDABOUT && outgoing->getJunctionPriority(this) == NBEdge::JunctionPriority::ROUNDABOUT) {
    //         return LinkDirection::STRAIGHT;
    //     }
    //     // turning direction
    //     if (incoming->isTurningDirectionAt(outgoing)) {
    //         if (isExplicitRailNoBidi(incoming, outgoing)) {
    //             return LinkDirection::STRAIGHT;
    //         }
    //         return leftHand ? LinkDirection::TURN_LEFTHAND : LinkDirection::TURN;
    //     }
    //     // get the angle between incoming/outgoing at the junction
    //     const double angle = NBHelpers::normRelAngle(incoming->getAngleAtNode(this), outgoing->getAngleAtNode(this));
    //     // ok, should be a straight connection
    //     EdgeVector::const_iterator itOut = std::find(myAllEdges.begin(), myAllEdges.end(), outgoing);
    //     SVCPermissions vehPerm = incoming->getPermissions() & outgoing->getPermissions();
    //     if (vehPerm != SVC_PEDESTRIAN) {
    //         vehPerm &= ~SVC_PEDESTRIAN;
    //     }
    //     const int modeLanes = outgoing->getNumLanesThatAllow(vehPerm);
    //     if (fabs(angle) < 44.) {
    //         if (fabs(angle) > 6.) {
    //             if (isStraighter(incoming, angle, vehPerm, modeLanes, getNextCompatibleOutgoing(incoming, vehPerm, itOut, true))) {
    //                 return angle > 0 ? LinkDirection::PARTRIGHT : LinkDirection::PARTLEFT;
    //             }
    //             if (isStraighter(incoming, angle, vehPerm, modeLanes, getNextCompatibleOutgoing(incoming, vehPerm, itOut, false))) {
    //                 return angle > 0 ? LinkDirection::PARTRIGHT : LinkDirection::PARTLEFT;
    //             }
    //         }
    //         if (angle > 0 && incoming->getJunctionPriority(this) == NBEdge::JunctionPriority::ROUNDABOUT) {
    //             return angle > 15 ? LinkDirection::RIGHT : LinkDirection::PARTRIGHT;
    //         }
    //         return LinkDirection::STRAIGHT;
    //     }
    //
    //     if (angle > 0) {
    //         // check whether any other edge goes further to the right
    //         if (angle > 90 + NUMERICAL_EPS) {
    //             return LinkDirection::RIGHT;
    //         }
    //         NBEdge* outCW = getNextCompatibleOutgoing(incoming, vehPerm, itOut, !leftHand);
    //         if (outCW != nullptr) {
    //             return LinkDirection::PARTRIGHT;
    //         } else {
    //             return LinkDirection::RIGHT;
    //         }
    //     } else {
    //         // check whether any other edge goes further to the left
    //         if (angle < -170 && incoming->getGeometry().reverse() == outgoing->getGeometry()) {
    //             if (isExplicitRailNoBidi(incoming, outgoing)) {
    //                 return LinkDirection::STRAIGHT;
    //             }
    //             return leftHand ? LinkDirection::TURN_LEFTHAND : LinkDirection::TURN;
    //         } else if (angle < -(90 + NUMERICAL_EPS)) {
    //             return LinkDirection::LEFT;
    //         }
    //         NBEdge* outCCW = getNextCompatibleOutgoing(incoming, vehPerm, itOut, leftHand);
    //         if (outCCW != nullptr) {
    //             return LinkDirection::PARTLEFT;
    //         } else {
    //             return LinkDirection::LEFT;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self, incoming: Ptr<NBEdge>, outgoing: Ptr<NBEdge>, leftHand: bool) -> LinkDirection {
        todo!("port NBNode::getDirection")
    }

    /// C++: `LinkState NBNode::getLinkState(const NBEdge *, const NBEdge *, int, int, bool, const std::string &) const` [const]
    /// get link state
    // LinkState
    // NBNode::getLinkState(const NBEdge* incoming, const NBEdge* outgoing, int fromLane, int toLane,
    //                      bool mayDefinitelyPass, const std::string& tlID) const {
    //     if (myType == SumoXMLNodeType::RAIL_CROSSING && isRailway(incoming->getPermissions())) {
    //         return LINKSTATE_MAJOR; // the trains must run on time
    //     }
    //     if (tlID != "") {
    //         if (getRightOfWay() == RightOfWay::ALLWAYSTOP) {
    //             return LINKSTATE_ALLWAY_STOP;
    //         }
    //         return mustBrake(incoming, outgoing, fromLane, toLane, true) ? LINKSTATE_TL_OFF_BLINKING : LINKSTATE_TL_OFF_NOSIGNAL;
    //     }
    //     if (outgoing == nullptr) { // always off
    //         return LINKSTATE_TL_OFF_NOSIGNAL;
    //     }
    //     if ((myType == SumoXMLNodeType::RIGHT_BEFORE_LEFT || myType == SumoXMLNodeType::LEFT_BEFORE_RIGHT)
    //             && mustBrake(incoming, outgoing, fromLane, toLane, true)) {
    //         return LINKSTATE_EQUAL; // all the same
    //     }
    //     if (myType == SumoXMLNodeType::ALLWAY_STOP) {
    //         return LINKSTATE_ALLWAY_STOP; // all drive, first one to arrive may drive first
    //     }
    //     if (myType == SumoXMLNodeType::ZIPPER && zipperConflict(incoming, outgoing, fromLane, toLane)) {
    //         return LINKSTATE_ZIPPER;
    //     }
    //     if (!mayDefinitelyPass
    //             && mustBrake(incoming, outgoing, fromLane, toLane, true)
    //             // legacy mode
    //             && (!incoming->isInsideTLS() || getDirection(incoming, outgoing) != LinkDirection::STRAIGHT)
    //             // avoid linkstate minor at pure railway nodes
    //             && (!NBNodeTypeComputer::isRailwayNode(this) || unsignalizedOperation())) {
    //         return myType == SumoXMLNodeType::PRIORITY_STOP && incoming->getJunctionPriority(this) == NBEdge::JunctionPriority::MINOR_ROAD ? LINKSTATE_STOP : LINKSTATE_MINOR; // minor road
    //     }
    //     // traffic lights are not regarded here
    //     return LINKSTATE_MAJOR;
    // }
    #[allow(unused_variables)]
    pub fn get_link_state(&self, incoming: Ptr<NBEdge>, outgoing: Ptr<NBEdge>, fromLane: i32, toLane: i32, mayDefinitelyPass: bool, tlID: &str) -> LinkState {
        todo!("port NBNode::getLinkState")
    }

    /// C++: `void NBNode::computeNodeShape(double)`
    /// Compute the junction shape for this node
    // void
    // NBNode::computeNodeShape(double mismatchThreshold) {
    //     if (myHaveCustomPoly) {
    //         return;
    //     }
    //     if (myIncomingEdges.size() == 0 && myOutgoingEdges.size() == 0) {
    //         // may be an intermediate step during network editing
    //         myPoly.clear();
    //         myPoly.push_back(myPosition);
    //         return;
    //     }
    //     if (OptionsCont::getOptions().getFloat("default.junctions.radius") < 0) {
    //         // skip shape computation by option
    //         return;
    //     }
    //     try {
    //         NBNodeShapeComputer computer(*this);
    //         myPoly = computer.compute(OptionsCont::getOptions().getBool("junctions.minimal-shape"));
    //         if (myRadius == UNSPECIFIED_RADIUS && !OptionsCont::getOptions().isDefault("default.junctions.radius")) {
    //             myRadius = computer.getRadius();
    //         }
    //         if (myPoly.size() > 0) {
    //             PositionVector tmp = myPoly;
    //             tmp.push_back_noDoublePos(tmp[0]); // need closed shape
    //             if (mismatchThreshold >= 0
    //                     && !tmp.around(myPosition)
    //                     && tmp.distance2D(myPosition) > mismatchThreshold) {
    //                 WRITE_WARNINGF(TL("Shape for junction '%' has distance % to its given position."), myID, tmp.distance2D(myPosition));
    //             }
    //         }
    //     } catch (InvalidArgument&) {
    //         WRITE_WARNINGF(TL("For junction '%': could not compute shape."), myID);
    //         // make sure our shape is not empty because our XML schema forbids empty attributes
    //         myPoly.clear();
    //         myPoly.push_back(myPosition);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_node_shape(&mut self, mismatchThreshold: f64) {
        todo!("port NBNode::computeNodeShape")
    }

    /// C++: `void NBNode::updateSurroundingGeometry()`
    /// update geometry of node and surrounding edges
    // void
    // NBNode::updateSurroundingGeometry() {
    //     NBTurningDirectionsComputer::computeTurnDirectionsForNode(this, false);
    //     sortEdges(false);
    //     computeNodeShape(-1);
    //     for (NBEdge* edge : myAllEdges) {
    //         edge->computeEdgeShape();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_surrounding_geometry(&mut self) {
        todo!("port NBNode::updateSurroundingGeometry")
    }

    /// C++: `const PositionVector & NBNode::getShape() const` [const]
    /// retrieve the junction shape
    // const PositionVector&
    // NBNode::getShape() const {
    //     return myPoly;
    // }
    #[allow(unused_variables)]
    pub fn get_shape(&self) -> &PositionVector {
        &self.myPoly
    }

    /// C++: `void NBNode::setCustomShape(const PositionVector &)`
    /// set the junction shape
    // void
    // NBNode::setCustomShape(const PositionVector& shape) {
    //     myPoly = shape;
    //     myHaveCustomPoly = (myPoly.size() > 1);
    //     if (myHaveCustomPoly) {
    //         for (EdgeVector::iterator i = myAllEdges.begin(); i != myAllEdges.end(); i++) {
    //             (*i)->resetNodeBorder(this);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_custom_shape(&mut self, shape: &PositionVector) {
        self.myPoly = shape.clone();
        self.myHaveCustomPoly = self.myPoly.len() > 1;
        if self.myHaveCustomPoly {
            let me = Ptr::from_mut(self);
            for i in self.myAllEdges.iter() {
                i.m().reset_node_border(me);
            }
        }
    }

    /// C++: `void NBNode::resetShape()`
    /// reset node shape
    // void resetShape() {
    //         myPoly.clear();
    //     }
    #[allow(unused_variables)]
    pub fn reset_shape(&mut self) {
        self.myPoly.clear();
    }

    /// C++: `void NBNode::setRadius(double)`
    /// set the turning radius
    // void setRadius(double radius) {
    //         myRadius = radius;
    //     }
    #[allow(unused_variables)]
    pub fn set_radius(&mut self, radius: f64) {
        self.myRadius = radius;
    }

    /// C++: `void NBNode::setKeepClear(bool)`
    /// set the keepClear flag
    // void setKeepClear(bool keepClear) {
    //         myKeepClear = keepClear;
    //     }
    #[allow(unused_variables)]
    pub fn set_keep_clear(&mut self, keepClear: bool) {
        self.myKeepClear = keepClear;
    }

    /// C++: `void NBNode::setRightOfWay(RightOfWay)`
    /// set method for computing right-of-way
    // void setRightOfWay(RightOfWay rightOfWay) {
    //         myRightOfWay = rightOfWay;
    //     }
    #[allow(unused_variables)]
    pub fn set_right_of_way(&mut self, rightOfWay: RightOfWay) {
        self.myRightOfWay = rightOfWay;
    }

    /// C++: `void NBNode::setFringeType(FringeType)`
    /// set fringe type
    // void setFringeType(FringeType fringeType) {
    //         myFringeType = fringeType;
    //     }
    #[allow(unused_variables)]
    pub fn set_fringe_type(&mut self, fringeType: FringeType) {
        self.myFringeType = fringeType;
    }

    /// C++: `void NBNode::setRoundaboutType(RoundaboutType)`
    /// set roundabout type
    // void setRoundaboutType(RoundaboutType roundaboutType) {
    //         myRoundaboutType = roundaboutType;
    //     }
    #[allow(unused_variables)]
    pub fn set_roundabout_type(&mut self, roundaboutType: RoundaboutType) {
        self.myRoundaboutType = roundaboutType;
    }

    /// C++: `void NBNode::setName(const std::string &)`
    /// set intersection name
    // void setName(const std::string& name) {
    //         myName = name;
    //     }
    #[allow(unused_variables)]
    pub fn set_name(&mut self, name: &str) {
        self.myName = name.to_string();
    }

    /// C++: `bool NBNode::hasCustomShape() const` [const]
    /// return whether the shape was set by the user
    // bool hasCustomShape() const {
    //         return myHaveCustomPoly;
    //     }
    #[allow(unused_variables)]
    pub fn has_custom_shape(&self) -> bool {
        self.myHaveCustomPoly
    }

    /// C++: `bool NBNode::checkIsRemovable() const` [const]
    /// check if node is removable
    // bool
    // NBNode::checkIsRemovable() const {
    //     std::string reason;
    //     return checkIsRemovableReporting(reason);
    // }
    #[allow(unused_variables)]
    pub fn check_is_removable(&self) -> bool {
        let mut reason = String::new();
        self.check_is_removable_reporting(&mut reason)
    }

    /// C++: `bool NBNode::checkIsRemovableReporting(std::string &) const` [const]
    /// check if node is removable and return reason if not
    // bool
    // NBNode::checkIsRemovableReporting(std::string& reason) const {
    //     if (getEdges().empty()) {
    //         return true;
    //     }
    //     // check whether this node is included in a traffic light or crossing
    //     if (myTrafficLights.size() != 0) {
    //         reason = "TLS";
    //         return false;
    //     }
    //     if (myType == SumoXMLNodeType::RAIL_SIGNAL) {
    //         reason = "rail_signal";
    //         return false;
    //     }
    //     if (myCrossings.size() != 0) {
    //         reason = "crossing";
    //         return false;
    //     }
    //     EdgeVector::const_iterator i;
    //     // one in, one out -> just a geometry ...
    //     if (myOutgoingEdges.size() == 1 && myIncomingEdges.size() == 1) {
    //         // ... if types match ...
    //         if (!myIncomingEdges[0]->expandableBy(myOutgoingEdges[0], reason)) {
    //             reason = "edges incompatible: " + reason;
    //             return false;
    //         }
    //         if (myIncomingEdges[0]->getTurnDestination(true) == myOutgoingEdges[0]) {
    //             reason = "turnaround";
    //             return false;
    //         }
    //         return true;
    //     }
    //     // two in, two out -> may be something else
    //     if (myOutgoingEdges.size() == 2 && myIncomingEdges.size() == 2) {
    //         // check whether the origin nodes of the incoming edges differ
    //         std::set<NBNode*> origSet;
    //         for (i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //             origSet.insert((*i)->getFromNode());
    //         }
    //         if (origSet.size() < 2) {
    //             // overlapping case
    //             if (myIncomingEdges[0]->getGeometry() == myIncomingEdges[1]->getGeometry() &&
    //                     myOutgoingEdges[0]->getGeometry() == myOutgoingEdges[1]->getGeometry()) {
    //                 return ((myIncomingEdges[0]->expandableBy(myOutgoingEdges[0], reason) &&
    //                          myIncomingEdges[1]->expandableBy(myOutgoingEdges[1], reason))
    //                         || (myIncomingEdges[0]->expandableBy(myOutgoingEdges[1], reason) &&
    //                             myIncomingEdges[1]->expandableBy(myOutgoingEdges[0], reason)));
    //             }
    //         }
    //         // check whether this node is an intermediate node of
    //         //  a two-directional street
    //         for (i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //             // each of the edges must have an opposite direction edge
    //             NBEdge* opposite = (*i)->getTurnDestination(true);
    //             if (opposite != nullptr) {
    //                 // the other outgoing edges must be the continuation of the current
    //                 NBEdge* continuation = opposite == myOutgoingEdges.front() ? myOutgoingEdges.back() : myOutgoingEdges.front();
    //                 // check whether the types allow joining
    //                 if (!(*i)->expandableBy(continuation, reason)) {
    //                     reason = "edges incompatible: " + reason;
    //                     return false;
    //                 }
    //             } else {
    //                 // ok, at least one outgoing edge is not an opposite
    //                 //  of an incoming one
    //                 reason = "not opposites";
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    //     // ok, a real node
    //     reason = "intersection";
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_is_removable_reporting(&self, reason: &mut String) -> bool {
        if self.get_edges().is_empty() {
            return true;
        }
        // check whether this node is included in a traffic light or crossing
        if !self.myTrafficLights.is_empty() {
            *reason = "TLS".to_string();
            return false;
        }
        if self.myType == SumoXMLNodeType::RAIL_SIGNAL {
            *reason = "rail_signal".to_string();
            return false;
        }
        if !self.myCrossings.is_empty() {
            *reason = "crossing".to_string();
            return false;
        }
        // one in, one out -> just a geometry ...
        if self.myOutgoingEdges.len() == 1 && self.myIncomingEdges.len() == 1 {
            // ... if types match ...
            if !self.myIncomingEdges[0].expandable_by(self.myOutgoingEdges[0], reason) {
                let msg = format!("edges incompatible: {}", reason);
                *reason = msg;
                return false;
            }
            if self.myIncomingEdges[0].get_turn_destination(true) == self.myOutgoingEdges[0] {
                *reason = "turnaround".to_string();
                return false;
            }
            return true;
        }
        // two in, two out -> may be something else
        if self.myOutgoingEdges.len() == 2 && self.myIncomingEdges.len() == 2 {
            // check whether the origin nodes of the incoming edges differ
            let mut origSet: BTreeSet<Ptr<NBNode>> = BTreeSet::new();
            for i in self.myIncomingEdges.iter() {
                origSet.insert(i.get_from_node());
            }
            if origSet.len() < 2 {
                // overlapping case
                if self.myIncomingEdges[0].get_geometry() == self.myIncomingEdges[1].get_geometry()
                    && self.myOutgoingEdges[0].get_geometry() == self.myOutgoingEdges[1].get_geometry()
                {
                    return (self.myIncomingEdges[0].expandable_by(self.myOutgoingEdges[0], reason)
                        && self.myIncomingEdges[1].expandable_by(self.myOutgoingEdges[1], reason))
                        || (self.myIncomingEdges[0].expandable_by(self.myOutgoingEdges[1], reason)
                            && self.myIncomingEdges[1].expandable_by(self.myOutgoingEdges[0], reason));
                }
            }
            // check whether this node is an intermediate node of
            //  a two-directional street
            for i in self.myIncomingEdges.iter() {
                // each of the edges must have an opposite direction edge
                let opposite = i.get_turn_destination(true);
                if !opposite.is_null() {
                    // the other outgoing edges must be the continuation of the current
                    let continuation = if opposite == self.myOutgoingEdges[0] {
                        self.myOutgoingEdges[self.myOutgoingEdges.len() - 1]
                    } else {
                        self.myOutgoingEdges[0]
                    };
                    // check whether the types allow joining
                    if !i.expandable_by(continuation, reason) {
                        let msg = format!("edges incompatible: {}", reason);
                        *reason = msg;
                        return false;
                    }
                } else {
                    // ok, at least one outgoing edge is not an opposite
                    //  of an incoming one
                    *reason = "not opposites".to_string();
                    return false;
                }
            }
            return true;
        }
        // ok, a real node
        *reason = "intersection".to_string();
        false
    }

    /// C++: `std::vector<std::pair<NBEdge *, NBEdge *>> NBNode::getEdgesToJoin() const` [const]
    /// get edges to join
    // std::vector<std::pair<NBEdge*, NBEdge*> >
    // NBNode::getEdgesToJoin() const {
    //     assert(checkIsRemovable());
    //     std::vector<std::pair<NBEdge*, NBEdge*> > ret;
    //     // one in, one out-case
    //     if (myOutgoingEdges.size() == 1 && myIncomingEdges.size() == 1) {
    //         ret.push_back(std::make_pair(myIncomingEdges[0], myOutgoingEdges[0]));
    //         return ret;
    //     }
    //     if (myIncomingEdges.size() == 2 && myOutgoingEdges.size() == 2) {
    //         // two in, two out-case
    //         if (myIncomingEdges[0]->getGeometry() == myIncomingEdges[1]->getGeometry() &&
    //                 myOutgoingEdges[0]->getGeometry() == myOutgoingEdges[1]->getGeometry()) {
    //             // overlapping edges
    //             std::string reason;
    //             if (myIncomingEdges[0]->expandableBy(myOutgoingEdges[0], reason)) {
    //                 ret.push_back(std::make_pair(myIncomingEdges[0], myOutgoingEdges[0]));
    //                 ret.push_back(std::make_pair(myIncomingEdges[1], myOutgoingEdges[1]));
    //             } else {
    //                 ret.push_back(std::make_pair(myIncomingEdges[0], myOutgoingEdges[1]));
    //                 ret.push_back(std::make_pair(myIncomingEdges[1], myOutgoingEdges[0]));
    //             }
    //             return ret;
    //         }
    //     }
    //     for (EdgeVector::const_iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //         // join with the edge that is not a turning direction
    //         NBEdge* opposite = (*i)->getTurnDestination(true);
    //         assert(opposite != 0);
    //         NBEdge* continuation = opposite == myOutgoingEdges.front() ? myOutgoingEdges.back() : myOutgoingEdges.front();
    //         ret.push_back(std::pair<NBEdge*, NBEdge*>(*i, continuation));
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_edges_to_join(&self) -> Vec<(Ptr<NBEdge>, Ptr<NBEdge>)> {
        debug_assert!(self.check_is_removable());
        let mut ret: Vec<(Ptr<NBEdge>, Ptr<NBEdge>)> = Vec::new();
        // one in, one out-case
        if self.myOutgoingEdges.len() == 1 && self.myIncomingEdges.len() == 1 {
            ret.push((self.myIncomingEdges[0], self.myOutgoingEdges[0]));
            return ret;
        }
        if self.myIncomingEdges.len() == 2 && self.myOutgoingEdges.len() == 2 {
            // two in, two out-case
            if self.myIncomingEdges[0].get_geometry() == self.myIncomingEdges[1].get_geometry()
                && self.myOutgoingEdges[0].get_geometry() == self.myOutgoingEdges[1].get_geometry()
            {
                // overlapping edges
                let mut reason = String::new();
                if self.myIncomingEdges[0].expandable_by(self.myOutgoingEdges[0], &mut reason) {
                    ret.push((self.myIncomingEdges[0], self.myOutgoingEdges[0]));
                    ret.push((self.myIncomingEdges[1], self.myOutgoingEdges[1]));
                } else {
                    ret.push((self.myIncomingEdges[0], self.myOutgoingEdges[1]));
                    ret.push((self.myIncomingEdges[1], self.myOutgoingEdges[0]));
                }
                return ret;
            }
        }
        for i in self.myIncomingEdges.iter() {
            // join with the edge that is not a turning direction
            let opposite = i.get_turn_destination(true);
            debug_assert!(!opposite.is_null());
            let continuation = if opposite == self.myOutgoingEdges[0] {
                self.myOutgoingEdges[self.myOutgoingEdges.len() - 1]
            } else {
                self.myOutgoingEdges[0]
            };
            ret.push((*i, continuation));
        }
        ret
    }

    /// C++: `bool NBNode::isNearDistrict() const` [const]
    /// if node is near district
    // bool
    // NBNode::isNearDistrict() const {
    //     if (isDistrict()) {
    //         return false;
    //     }
    //     for (const NBEdge* const t : getEdges()) {
    //         const NBNode* const other = t->getToNode() == this ? t->getFromNode() : t->getToNode();
    //         for (const NBEdge* const k : other->getEdges()) {
    //             if (k->getFromNode()->isDistrict() || k->getToNode()->isDistrict()) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_near_district(&self) -> bool {
        if self.is_district() {
            return false;
        }
        for t in self.get_edges().iter() {
            let other = if t.get_to_node() == Ptr::new(self) { t.get_from_node() } else { t.get_to_node() };
            for k in other.get_edges().iter() {
                if k.get_from_node().is_district() || k.get_to_node().is_district() {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool NBNode::isDistrict() const` [const]
    /// check if node is a district
    // bool
    // NBNode::isDistrict() const {
    //     return myType == SumoXMLNodeType::DISTRICT;
    // }
    #[allow(unused_variables)]
    pub fn is_district(&self) -> bool {
        self.myType == SumoXMLNodeType::DISTRICT
    }

    /// C++: `bool NBNode::needsCont(const NBEdge *, const NBEdge *, const NBEdge::Connection &, const NBEdge::Connection &, bool) const` [const]
    /// whether an internal junction should be built at from and respect other
    // bool
    // NBNode::needsCont(const NBEdge* fromE, const NBEdge* otherFromE,
    //                   const NBEdge::Connection& c, const NBEdge::Connection& otherC, bool checkOnlyTLS) const {
    //     const NBEdge* toE = c.toEdge;
    //     const NBEdge* otherToE = otherC.toEdge;
    //
    //     if (!checkOnlyTLS) {
    //         if (myType == SumoXMLNodeType::RIGHT_BEFORE_LEFT
    //                 || myType == SumoXMLNodeType::LEFT_BEFORE_RIGHT
    //                 || myType == SumoXMLNodeType::ALLWAY_STOP
    //                 || myType == SumoXMLNodeType::ZIPPER) {
    //             return false;
    //         }
    //         LinkDirection d1 = getDirection(fromE, toE);
    //         const bool thisRight = (d1 == LinkDirection::RIGHT || d1 == LinkDirection::PARTRIGHT);
    //         const bool rightTurnConflict = (thisRight &&
    //                                         NBNode::rightTurnConflict(fromE, toE, c.fromLane, otherFromE, otherToE, otherC.fromLane));
    //         if (thisRight && !rightTurnConflict) {
    //             return false;
    //         }
    //         if (myRequest && myRequest->indirectLeftTurnConflict(fromE, c, otherFromE, otherC, false)) {
    //             return true;
    //         }
    //         if (!(foes(otherFromE, otherToE, fromE, toE) || myRequest == nullptr || rightTurnConflict)) {
    //             // if they do not cross, no waiting place is needed
    //             return false;
    //         }
    //         LinkDirection d2 = getDirection(otherFromE, otherToE);
    //         if (d2 == LinkDirection::TURN) {
    //             return false;
    //         }
    //         if (fromE == otherFromE && !thisRight) {
    //             // ignore same edge links except for right-turns
    //             return false;
    //         }
    //         if (thisRight && d2 != LinkDirection::STRAIGHT) {
    //             return false;
    //         }
    //     }
    //     if (c.tlID != "") {
    //         assert(myTrafficLights.size() > 0 || myType == SumoXMLNodeType::RAIL_CROSSING || myType == SumoXMLNodeType::RAIL_SIGNAL);
    //         for (std::set<NBTrafficLightDefinition*>::const_iterator it = myTrafficLights.begin(); it != myTrafficLights.end(); ++it) {
    //             if ((*it)->needsCont(fromE, toE, otherFromE, otherToE)) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    //     if (fromE->getJunctionPriority(this) > 0 && otherFromE->getJunctionPriority(this) > 0) {
    //         return mustBrake(fromE, toE, c.fromLane, c.toLane, false);
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn needs_cont(&self, fromE: Ptr<NBEdge>, otherFromE: Ptr<NBEdge>, c: &NBEdge_Connection, otherC: &NBEdge_Connection, checkOnlyTLS: bool) -> bool {
        let toE = c.toEdge;
        let otherToE = otherC.toEdge;

        if !checkOnlyTLS {
            if self.myType == SumoXMLNodeType::RIGHT_BEFORE_LEFT
                || self.myType == SumoXMLNodeType::LEFT_BEFORE_RIGHT
                || self.myType == SumoXMLNodeType::ALLWAY_STOP
                || self.myType == SumoXMLNodeType::ZIPPER
            {
                return false;
            }
            let d1 = self.get_direction(fromE, toE, false);
            let thisRight = d1 == LinkDirection::RIGHT || d1 == LinkDirection::PARTRIGHT;
            let rightTurnConflict = thisRight
                && NBNode::right_turn_conflict(fromE, toE, c.fromLane, otherFromE, otherToE, otherC.fromLane);
            if thisRight && !rightTurnConflict {
                return false;
            }
            if !self.myRequest.is_null() && self.myRequest.indirect_left_turn_conflict(fromE, c, otherFromE, otherC, false) {
                return true;
            }
            if !(self.foes(otherFromE, otherToE, fromE, toE) || self.myRequest.is_null() || rightTurnConflict) {
                // if they do not cross, no waiting place is needed
                return false;
            }
            let d2 = self.get_direction(otherFromE, otherToE, false);
            if d2 == LinkDirection::TURN {
                return false;
            }
            if fromE == otherFromE && !thisRight {
                // ignore same edge links except for right-turns
                return false;
            }
            if thisRight && d2 != LinkDirection::STRAIGHT {
                return false;
            }
        }
        if !c.tlID.is_empty() {
            debug_assert!(
                !self.myTrafficLights.is_empty()
                    || self.myType == SumoXMLNodeType::RAIL_CROSSING
                    || self.myType == SumoXMLNodeType::RAIL_SIGNAL
            );
            for it in self.myTrafficLights.iter() {
                if it.needs_cont(fromE, toE, otherFromE, otherToE) {
                    return true;
                }
            }
            return false;
        }
        if fromE.get_junction_priority(Ptr::new(self)) > 0 && otherFromE.get_junction_priority(Ptr::new(self)) > 0 {
            return self.must_brake(fromE, toE, c.fromLane, c.toLane, false);
        }
        false
    }

    /// C++: `bool NBNode::tlsStrandedConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &) const` [const]
    /// whether the connection must yield if the foe remains on the intersection after its phase ends
    // bool
    // NBNode::tlsStrandedConflict(const NBEdge* from, const NBEdge::Connection& c,
    //                             const NBEdge* foeFrom, const NBEdge::Connection& foe) const {
    //     return (foe.haveVia && isTLControlled() && c.tlLinkIndex >= 0 && foe.tlLinkIndex >= 0
    //             && !foeFrom->isTurningDirectionAt(foe.toEdge)
    //             && foes(from, c.toEdge, foeFrom, foe.toEdge)
    //             && !needsCont(foeFrom, from, foe, c, true));
    // }
    #[allow(unused_variables)]
    pub fn tls_stranded_conflict(&self, from: Ptr<NBEdge>, c: &NBEdge_Connection, foeFrom: Ptr<NBEdge>, foe: &NBEdge_Connection) -> bool {
        foe.haveVia
            && self.is_tl_controlled()
            && c.tlLinkIndex >= 0
            && foe.tlLinkIndex >= 0
            && !foeFrom.is_turning_direction_at(foe.toEdge)
            && self.foes(from, c.toEdge, foeFrom, foe.toEdge)
            && !self.needs_cont(foeFrom, from, foe, c, true)
    }

    /// C++: `PositionVector NBNode::computeInternalLaneShape(const NBEdge *, const NBEdge::Connection &, int, NBNode *, int) const` [const]
    /// Compute the shape for an internal lane
    // PositionVector
    // NBNode::computeInternalLaneShape(const NBEdge* fromE, const NBEdge::Connection& con, int numPoints, NBNode* recordError, int shapeFlag) const {
    //     if (con.fromLane >= fromE->getNumLanes()) {
    //         throw ProcessError(TLF("Connection '%' starts at a non-existant lane.", con.getDescription(fromE)));
    //     }
    //     if (con.toLane >= con.toEdge->getNumLanes()) {
    //         throw ProcessError(TLF("Connection '%' targets a non-existant lane.", con.getDescription(fromE)));
    //     }
    //     PositionVector fromShape = fromE->getLaneShape(con.fromLane);
    //     PositionVector toShape = con.toEdge->getLaneShape(con.toLane);
    //     PositionVector ret;
    //     bool useCustomShape = con.customShape.size() > 0;
    //     if (useCustomShape) {
    //         // ensure that the shape starts and ends at the intersection boundary
    //         PositionVector startBorder = fromE->getNodeBorder(this);
    //         if (startBorder.size() == 0) {
    //             startBorder = fromShape.getOrthogonal(fromShape.back(), 1, true);
    //         }
    //         PositionVector tmp = NBEdge::startShapeAt(con.customShape, this, startBorder);
    //         if (tmp.size() < 2) {
    //             WRITE_WARNINGF(TL("Could not use custom shape for connection %."), con.getDescription(fromE));
    //             useCustomShape = false;
    //         } else {
    //             if (tmp.length2D() > con.customShape.length2D() + POSITION_EPS) {
    //                 // shape was lengthened at the start, make sure it attaches at the center of the lane
    //                 tmp[0] = fromShape.back();
    //             } else if (recordError != nullptr) {
    //                 const double offset = tmp[0].distanceTo2D(fromShape.back());
    //                 if (offset > fromE->getLaneWidth(con.fromLane) / 2) {
    //                     WRITE_WARNINGF(TL("Custom shape has distance % to incoming lane for connection %."), offset, con.getDescription(fromE));
    //                 }
    //             }
    //             PositionVector endBorder = con.toEdge->getNodeBorder(this);
    //             if (endBorder.size() == 0) {
    //                 endBorder = toShape.getOrthogonal(toShape.front(), 1, false);
    //             }
    //             ret = NBEdge::startShapeAt(tmp.reverse(), this, endBorder).reverse();
    //             if (ret.size() < 2) {
    //                 WRITE_WARNINGF(TL("Could not use custom shape for connection %."), con.getDescription(fromE));
    //                 useCustomShape = false;
    //             } else if (ret.length2D() > tmp.length2D() + POSITION_EPS) {
    //                 // shape was lengthened at the end, make sure it attaches at the center of the lane
    //                 ret[-1] = toShape.front();
    //             } else if (recordError != nullptr) {
    //                 const double offset = ret[-1].distanceTo2D(toShape.front());
    //                 if (offset > con.toEdge->getLaneWidth(con.toLane) / 2) {
    //                     WRITE_WARNINGF(TL("Custom shape has distance % to outgoing lane for connection %."), offset, con.getDescription(fromE));
    //                 }
    //             }
    //         }
    //     }
    //     if (!useCustomShape) {
    //         displaceShapeAtWidthChange(fromE, con, fromShape, toShape);
    //         double extrapolateBeg = 5. * fromE->getNumLanes();
    //         double extrapolateEnd = 5. * con.toEdge->getNumLanes();
    //         LinkDirection dir = getDirection(fromE, con.toEdge);
    //         if (dir == LinkDirection::LEFT || dir == LinkDirection::TURN) {
    //             shapeFlag += AVOID_WIDE_LEFT_TURN;
    //         }
    //         if (con.indirectLeft) {
    //             shapeFlag += INDIRECT_LEFT;
    //         }
    // #ifdef DEBUG_SMOOTH_GEOM
    //         if (DEBUGCOND) {
    //             std::cout << "computeInternalLaneShape node " << getID() << " fromE=" << fromE->getID() << " toE=" << con.toEdge->getID() << "\n";
    //         }
    // #endif
    //         ret = computeSmoothShape(fromShape, toShape,
    //                                  numPoints, fromE->getTurnDestination() == con.toEdge,
    //                                  extrapolateBeg, extrapolateEnd, recordError, shapeFlag);
    //     }
    //     const NBEdge::Lane& lane = fromE->getLaneStruct(con.fromLane);
    //     if (lane.endOffset > 0) {
    //         PositionVector beg = lane.shape.getSubpart(lane.shape.length() - lane.endOffset, lane.shape.length());
    //         beg.append(ret);
    //         ret = beg;
    //     }
    //     if (con.toEdge->isBidiRail() && con.toEdge->getTurnDestination(true)->getEndOffset() > 0) {
    //         PositionVector end = toShape.getSubpart(0, con.toEdge->getTurnDestination(true)->getEndOffset());
    //         ret.append(end);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn compute_internal_lane_shape(&self, fromE: Ptr<NBEdge>, con: &NBEdge_Connection, numPoints: i32, recordError: Ptr<NBNode>, shapeFlag: i32) -> PositionVector {
        let mut shapeFlag = shapeFlag;
        if con.fromLane >= fromE.get_num_lanes() {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Connection '{}' starts at a non-existant lane.", con.get_description(fromE))));
        }
        if con.toLane >= con.toEdge.get_num_lanes() {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Connection '{}' targets a non-existant lane.", con.get_description(fromE))));
        }
        let mut fromShape = fromE.get_lane_shape(con.fromLane).clone();
        let mut toShape = con.toEdge.get_lane_shape(con.toLane).clone();
        let mut ret = PositionVector::default();
        let mut useCustomShape = con.customShape.len() > 0;
        if useCustomShape {
            // ensure that the shape starts and ends at the intersection boundary
            let mut startBorder = fromE.get_node_border(Ptr::new(self)).clone();
            if startBorder.len() == 0 {
                startBorder = fromShape.get_orthogonal(fromShape.back(), 1., true, 1., 90.);
            }
            let mut tmp = NBEdge::start_shape_at(&con.customShape, Ptr::new(self), startBorder);
            if tmp.len() < 2 {
                write_warning(&format!("Could not use custom shape for connection {}.", con.get_description(fromE)));
                useCustomShape = false;
            } else {
                if tmp.length2d() > con.customShape.length2d() + POSITION_EPS {
                    // shape was lengthened at the start, make sure it attaches at the center of the lane
                    *tmp.at_mut(0) = *fromShape.back();
                } else if !recordError.is_null() {
                    let offset = tmp.at(0).distance_to2d(fromShape.back());
                    if offset > fromE.get_lane_width_2(con.fromLane) / 2. {
                        write_warning(&format!("Custom shape has distance {} to incoming lane for connection {}.", f64_to_string(offset), con.get_description(fromE)));
                    }
                }
                let mut endBorder = con.toEdge.get_node_border(Ptr::new(self)).clone();
                if endBorder.len() == 0 {
                    endBorder = toShape.get_orthogonal(toShape.front(), 1., false, 1., 90.);
                }
                let rev = tmp.reverse();
                ret = NBEdge::start_shape_at(&rev, Ptr::new(self), endBorder).reverse();
                if ret.len() < 2 {
                    write_warning(&format!("Could not use custom shape for connection {}.", con.get_description(fromE)));
                    useCustomShape = false;
                } else if ret.length2d() > tmp.length2d() + POSITION_EPS {
                    // shape was lengthened at the end, make sure it attaches at the center of the lane
                    *ret.at_mut(-1) = *toShape.front();
                } else if !recordError.is_null() {
                    let offset = ret.at(-1).distance_to2d(toShape.front());
                    if offset > con.toEdge.get_lane_width_2(con.toLane) / 2. {
                        write_warning(&format!("Custom shape has distance {} to outgoing lane for connection {}.", f64_to_string(offset), con.get_description(fromE)));
                    }
                }
            }
        }
        if !useCustomShape {
            self.displace_shape_at_width_change(fromE, con, &mut fromShape, &mut toShape);
            let extrapolateBeg = 5. * (fromE.get_num_lanes() as f64);
            let extrapolateEnd = 5. * (con.toEdge.get_num_lanes() as f64);
            let dir = self.get_direction(fromE, con.toEdge, false);
            if dir == LinkDirection::LEFT || dir == LinkDirection::TURN {
                shapeFlag += NBNODE_AVOID_WIDE_LEFT_TURN;
            }
            if con.indirectLeft {
                shapeFlag += NBNODE_INDIRECT_LEFT;
            }
            ret = self.compute_smooth_shape(
                &fromShape,
                &toShape,
                numPoints,
                fromE.get_turn_destination(false) == con.toEdge,
                extrapolateBeg,
                extrapolateEnd,
                recordError,
                shapeFlag,
            );
        }
        let lane = fromE.get_lane_struct_2(con.fromLane);
        if lane.endOffset > 0. {
            let mut beg = lane.shape.get_subpart(lane.shape.length() - lane.endOffset, lane.shape.length());
            beg.append(&ret, 2.0);
            ret = beg;
        }
        if con.toEdge.is_bidi_rail(false) && con.toEdge.get_turn_destination(true).get_end_offset() > 0. {
            let end = toShape.get_subpart(0., con.toEdge.get_turn_destination(true).get_end_offset());
            ret.append(&end, 2.0);
        }
        ret
    }

    /// C++: `PositionVector NBNode::computeSmoothShape(const PositionVector &, const PositionVector &, int, bool, double, double, NBNode *, int) const` [const]
    /// Compute a smooth curve between the given geometries
    // PositionVector
    // NBNode::computeSmoothShape(const PositionVector& begShape,
    //                            const PositionVector& endShape,
    //                            int numPoints,
    //                            bool isTurnaround,
    //                            double extrapolateBeg,
    //                            double extrapolateEnd,
    //                            NBNode* recordError,
    //                            int shapeFlag) const {
    //
    //     bool ok = true;
    //     if ((shapeFlag & INDIRECT_LEFT) != 0) {
    //         return indirectLeftShape(begShape, endShape, numPoints);
    //     }
    //     PositionVector init = bezierControlPoints(begShape, endShape, isTurnaround, extrapolateBeg, extrapolateEnd, ok, recordError, DEG2RAD(5), shapeFlag);
    // #ifdef DEBUG_SMOOTH_GEOM
    //     if (DEBUGCOND) {
    //         std::cout << "computeSmoothShape node " << getID() << " begShape=" << begShape << " endShape=" << endShape << " init=" << init << " shapeFlag=" << shapeFlag << "\n";
    //     }
    // #endif
    //     if (init.size() == 0) {
    //         PositionVector ret;
    //         ret.push_back(begShape.back());
    //         ret.push_back(endShape.front());
    //         return ret;
    //     } else {
    //         return init.bezier(numPoints).smoothedZFront();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_smooth_shape(&self, begShape: &PositionVector, endShape: &PositionVector, numPoints: i32, isTurnaround: bool, extrapolateBeg: f64, extrapolateEnd: f64, recordError: Ptr<NBNode>, shapeFlag: i32) -> PositionVector {
        let mut ok = true;
        if (shapeFlag & NBNODE_INDIRECT_LEFT) != 0 {
            return self.indirect_left_shape(begShape, endShape, numPoints);
        }
        // DEG2RAD(5)
        let init = Self::bezier_control_points(
            begShape,
            endShape,
            isTurnaround,
            extrapolateBeg,
            extrapolateEnd,
            &mut ok,
            recordError,
            5. * std::f64::consts::PI / 180.,
            shapeFlag,
        );
        if init.len() == 0 {
            let mut ret = PositionVector::default();
            ret.push(*begShape.back());
            ret.push(*endShape.front());
            return ret;
        } else {
            return init.bezier(numPoints).smoothed_z_front(f64::MAX);
        }
    }

    /// C++: `PositionVector NBNode::bezierControlPoints(const PositionVector &, const PositionVector &, bool, double, double, bool &, NBNode *, double, int)` [static]
    /// get bezier control points
    // PositionVector
    // NBNode::bezierControlPoints(
    //     const PositionVector& begShape,
    //     const PositionVector& endShape,
    //     bool isTurnaround,
    //     double extrapolateBeg,
    //     double extrapolateEnd,
    //     bool& ok,
    //     NBNode* recordError,
    //     double straightThresh,
    //     int shapeFlag) {
    //
    //     const Position beg = begShape.back();
    //     const Position end = endShape.front();
    //     const double dist = beg.distanceTo2D(end);
    //     PositionVector init;
    //     if (dist < POSITION_EPS || beg.distanceTo2D(begShape[-2]) < POSITION_EPS || end.distanceTo2D(endShape[1]) < POSITION_EPS) {
    // #ifdef DEBUG_SMOOTH_GEOM
    //         if (DEBUGCOND2(recordError)) std::cout << "   bezierControlPoints failed beg=" << beg << " end=" << end
    //                                                    << " dist=" << dist
    //                                                    << " distBegLast=" << beg.distanceTo2D(begShape[-2])
    //                                                    << " distEndFirst=" << end.distanceTo2D(endShape[1])
    //                                                    << "\n";
    // #endif
    //         // typically, this node a is a simpleContinuation. see also #2539
    //         return init;
    //     } else {
    //         init.push_back(beg);
    //         if (isTurnaround) {
    //             // turnarounds:
    //             //  - end of incoming lane
    //             //  - position between incoming/outgoing end/begin shifted by the distance orthogonally
    //             //  - begin of outgoing lane
    //             Position center = PositionVector::positionAtOffset2D(beg, end, beg.distanceTo2D(end) / (double) 2.);
    //             center.sub(beg.y() - end.y(), end.x() - beg.x());
    //             init.push_back(center);
    //         } else {
    //             const double EXT = 100;
    //             const double angle = GeomHelper::angleDiff(begShape.angleAt2D(-2), endShape.angleAt2D(0));
    //             PositionVector endShapeBegLine(endShape[0], endShape[1]);
    //             PositionVector begShapeEndLineRev(begShape[-1], begShape[-2]);
    //             endShapeBegLine.extrapolate2D(EXT, true);
    //             begShapeEndLineRev.extrapolate2D(EXT, true);
    // #ifdef DEBUG_SMOOTH_GEOM
    //             if (DEBUGCOND2(recordError)) std::cout
    //                         << "   endShapeBegLine=" << endShapeBegLine
    //                         << " begShapeEndLineRev=" << begShapeEndLineRev
    //                         << " angle=" << RAD2DEG(angle) << "\n";
    // #endif
    //             if (fabs(angle) < M_PI / 4.) {
    //                 // very low angle: could be an s-shape or a straight line
    //                 const double displacementAngle = GeomHelper::angleDiff(begShape.angleAt2D(-2), beg.angleTo2D(end));
    //                 const double bendDeg = RAD2DEG(fabs(displacementAngle - angle));
    //                 const double halfDistance = dist / 2;
    //                 if (fabs(displacementAngle) <= straightThresh && fabs(angle) <= straightThresh) {
    // #ifdef DEBUG_SMOOTH_GEOM
    //                     if (DEBUGCOND2(recordError)) std::cout << "   bezierControlPoints identified straight line beg=" << beg << " end=" << end
    //                                                                << " angle=" << RAD2DEG(angle) << " displacementAngle=" << RAD2DEG(displacementAngle) << "\n";
    // #endif
    //                     return PositionVector();
    //                 } else if (bendDeg > 22.5 && pow(bendDeg / 45, 2) / dist > 0.13) {
    //                     // do not allow s-curves with extreme bends
    //                     // (a linear dependency is to restrictive at low displacementAngles and too permisive at high angles)
    // #ifdef DEBUG_SMOOTH_GEOM
    //                     if (DEBUGCOND2(recordError)) std::cout << "   bezierControlPoints found extreme s-curve, falling back to straight line beg=" << beg << " end=" << end
    //                                                                << " angle=" << RAD2DEG(angle) << " displacementAngle=" << RAD2DEG(displacementAngle)
    //                                                                << " dist=" << dist << " bendDeg=" << bendDeg << " bd2=" << pow(bendDeg / 45, 2)
    //                                                                << " displacementError=" << sin(displacementAngle) * dist
    //                                                                << " begShape=" << begShape << " endShape=" << endShape << "\n";
    // #endif
    //                     ok = false;
    //                     if (recordError != nullptr && (shapeFlag & SCURVE_IGNORE) == 0) {
    //                         recordError->myDisplacementError = MAX2(recordError->myDisplacementError, (double)fabs(sin(displacementAngle) * dist));
    //                     }
    //                     return PositionVector();
    //                 } else {
    //                     const double endLength = begShape[-2].distanceTo2D(begShape[-1]);
    //                     const double off1 = endLength + MIN2(extrapolateBeg, halfDistance);
    //                     init.push_back(PositionVector::positionAtOffset2D(begShapeEndLineRev[1], begShapeEndLineRev[0], off1));
    //                     const double off2 = EXT - MIN2(extrapolateEnd, halfDistance);
    //                     init.push_back(PositionVector::positionAtOffset2D(endShapeBegLine[0], endShapeBegLine[1], off2));
    // #ifdef DEBUG_SMOOTH_GEOM
    //                     if (DEBUGCOND2(recordError)) std::cout << "   bezierControlPoints found s-curve beg=" << beg << " end=" << end
    //                                                                << " angle=" << RAD2DEG(angle) << " displacementAngle=" << RAD2DEG(displacementAngle)
    //                                                                << " halfDistance=" << halfDistance << "\n";
    // #endif
    //                 }
    //             } else {
    //                 // turning
    //                 //  - end of incoming lane
    //                 //  - intersection of the extrapolated lanes
    //                 //  - begin of outgoing lane
    //                 // attention: if there is no intersection, use a straight line
    //                 Position intersect = endShapeBegLine.intersectionPosition2D(begShapeEndLineRev);
    //                 if (intersect == Position::INVALID) {
    // #ifdef DEBUG_SMOOTH_GEOM
    //                     if (DEBUGCOND2(recordError)) {
    //                         std::cout << "   bezierControlPoints failed beg=" << beg << " end=" << end << " intersect=" << intersect
    //                                   << " endShapeBegLine=" << endShapeBegLine
    //                                   << " begShapeEndLineRev=" << begShapeEndLineRev
    //                                   << "\n";
    //                     }
    // #endif
    //                     ok = false;
    //                     if (recordError != nullptr && (shapeFlag & SCURVE_IGNORE) == 0) {
    //                         // it's unclear if this error can be solved via stretching the intersection.
    //                         recordError->myDisplacementError = MAX2(recordError->myDisplacementError, (double)1.0);
    //                     }
    //                     return PositionVector();
    //                 }
    //                 const double begOffset = begShapeEndLineRev.nearest_offset_to_point2D(intersect);
    //                 const double endOffset = endShapeBegLine.nearest_offset_to_point2D(intersect);
    //                 /*
    //                 if ((shapeFlag & FOUR_CONTROL_POINTS) == 0 && (begOffset >= EXT || endOffset >= EXT)) {
    //                     // intersection point lies within begShape / endShape so we cannot use it
    //                     if (dist < 2) {
    //                         return PositionVector();
    //                     }
    //                     shapeFlag |= FOUR_CONTROL_POINTS;
    //                     extrapolateBeg = MIN2(10.0, dist / 2);
    //                     extrapolateEnd = extrapolateBeg;
    //                 }
    //                 */
    //                 const double minControlLength = MIN2((double)1.0, dist / 2);
    //                 const double distBeg = intersect.distanceTo2D(beg);
    //                 const double distEnd = intersect.distanceTo2D(end);
    //                 const bool lengthenBeg = distBeg <= minControlLength;
    //                 const bool lengthenEnd = distEnd <= minControlLength;
    // #ifdef DEBUG_SMOOTH_GEOM
    //                 if (DEBUGCOND2(recordError)) std::cout
    //                             << "   beg=" << beg << " end=" << end << " intersect=" << intersect
    //                             << " distBeg=" << distBeg << " distEnd=" << distEnd
    //                             << " begOffset=" << begOffset << " endOffset=" << endOffset
    //                             << " lEnd=" << lengthenEnd << " lBeg=" << lengthenBeg
    //                             << "\n";
    // #endif
    //                 if (lengthenBeg && lengthenEnd) {
    // #ifdef DEBUG_SMOOTH_GEOM
    //                     if (DEBUGCOND2(recordError)) {
    //                         std::cout << "   bezierControlPoints failed\n";
    //                     }
    // #endif
    //                     if (recordError != nullptr && (shapeFlag & SCURVE_IGNORE) == 0) {
    //                         // This should be fixable with minor stretching
    //                         recordError->myDisplacementError = MAX2(recordError->myDisplacementError, (double)1.0);
    //                     }
    //                     ok = false;
    //                     return PositionVector();
    //                 } else if ((shapeFlag & FOUR_CONTROL_POINTS)) {
    //                     init.push_back(begShapeEndLineRev.positionAtOffset2D(EXT - extrapolateBeg));
    //                     init.push_back(endShapeBegLine.positionAtOffset2D(EXT - extrapolateEnd));
    //                 } else if (lengthenBeg || lengthenEnd) {
    //                     init.push_back(begShapeEndLineRev.positionAtOffset2D(EXT - minControlLength));
    //                     init.push_back(endShapeBegLine.positionAtOffset2D(EXT - minControlLength));
    //                 } else if ((shapeFlag & AVOID_WIDE_LEFT_TURN) != 0
    //                            // there are two reasons for enabling special geometry rules:
    //                            // 1) sharp edge angles which could cause overshoot
    //                            // 2) junction geometries with a large displacement between opposite left turns
    //                            //    which would cause the default geometry to overlap
    //                            && ((shapeFlag & AVOID_INTERSECTING_LEFT_TURNS) != 0
    //                                || (angle > DEG2RAD(95) && (distBeg > 20 || distEnd > 20)))) {
    //                     //std::cout << "   bezierControlPoints intersect=" << intersect << " dist=" << dist << " distBeg=" << distBeg <<  " distEnd=" << distEnd << " angle=" << RAD2DEG(angle) << " flag=" << shapeFlag << "\n";
    //                     const double factor = ((shapeFlag & AVOID_INTERSECTING_LEFT_TURNS) == 0 ? 1
    //                                            : MIN2(0.6, 16 / dist));
    //                     init.push_back(begShapeEndLineRev.positionAtOffset2D(EXT - MIN2(distBeg * factor / 1.2, dist * factor / 1.8)));
    //                     init.push_back(endShapeBegLine.positionAtOffset2D(EXT - MIN2(distEnd * factor / 1.2, dist * factor / 1.8)));
    //                 } else if ((shapeFlag & AVOID_WIDE_RIGHT_TURN) != 0 && angle < DEG2RAD(-95) && (distBeg > 20 || distEnd > 20)) {
    //                     //std::cout << "   bezierControlPoints intersect=" << intersect << " distBeg=" << distBeg <<  " distEnd=" << distEnd << "\n";
    //                     init.push_back(begShapeEndLineRev.positionAtOffset2D(EXT - MIN2(distBeg / 1.4, dist / 2)));
    //                     init.push_back(endShapeBegLine.positionAtOffset2D(EXT - MIN2(distEnd / 1.4, dist / 2)));
    //                 } else {
    //                     double z;
    //                     const double z1 = begShapeEndLineRev.positionAtOffset2D(begOffset).z();
    //                     const double z2 = endShapeBegLine.positionAtOffset2D(endOffset).z();
    //                     const double z3 = 0.5 * (beg.z() + end.z());
    //                     // if z1 and z2 are on the same side in regard to z3 then we
    //                     // can use their avarage. Otherwise, the intersection in 3D
    //                     // is not good and we are better of using z3
    //                     if ((z1 <= z3 && z2 <= z3) || (z1 >= z3 && z2 >= z3)) {
    //                         z = 0.5 * (z1 + z2);
    //                     } else {
    //                         z = z3;
    //                     }
    //                     intersect.set(intersect.x(), intersect.y(), z);
    //                     init.push_back(intersect);
    //                 }
    //             }
    //         }
    //         init.push_back(end);
    //     }
    //     return init;
    // }
    #[allow(unused_variables)]
    pub fn bezier_control_points(begShape: &PositionVector, endShape: &PositionVector, isTurnaround: bool, extrapolateBeg: f64, extrapolateEnd: f64, ok: &mut bool, recordError: Ptr<NBNode>, straightThresh: f64, shapeFlag: i32) -> PositionVector {
        let beg = *begShape.back();
        let end = *endShape.front();
        let dist = beg.distance_to2d(&end);
        let mut init = PositionVector::default();
        if dist < POSITION_EPS || beg.distance_to2d(begShape.at(-2)) < POSITION_EPS || end.distance_to2d(endShape.at(1)) < POSITION_EPS {
            // typically, this node a is a simpleContinuation. see also #2539
            return init;
        }
        init.push(beg);
        if isTurnaround {
            // turnarounds:
            //  - end of incoming lane
            //  - position between incoming/outgoing end/begin shifted by the distance orthogonally
            //  - begin of outgoing lane
            let mut center: Position = PositionVector::position_at_offset2d_seg(&beg, &end, beg.distance_to2d(&end) / 2., 0., false);
            center.sub_xy(beg.y() - end.y(), end.x() - beg.x());
            init.push(center);
        } else {
            let EXT = 100.;
            let angle = angle_diff(begShape.angle_at2d(-2), endShape.angle_at2d(0));
            let mut endShapeBegLine = PositionVector::from_line(*endShape.at(0), *endShape.at(1));
            let mut begShapeEndLineRev = PositionVector::from_line(*begShape.at(-1), *begShape.at(-2));
            endShapeBegLine.extrapolate2d(EXT, true);
            begShapeEndLineRev.extrapolate2d(EXT, true);
            if angle.abs() < std::f64::consts::PI / 4. {
                // very low angle: could be an s-shape or a straight line
                let displacementAngle = angle_diff(begShape.angle_at2d(-2), beg.angle_to2d(&end));
                let bendDeg = rad2deg((displacementAngle - angle).abs());
                let halfDistance = dist / 2.;
                if displacementAngle.abs() <= straightThresh && angle.abs() <= straightThresh {
                    return PositionVector::default();
                } else if bendDeg > 22.5 && (bendDeg / 45.).powf(2.) / dist > 0.13 {
                    // do not allow s-curves with extreme bends
                    // (a linear dependency is to restrictive at low displacementAngles and too permisive at high angles)
                    *ok = false;
                    if !recordError.is_null() && (shapeFlag & NBNODE_SCURVE_IGNORE) == 0 {
                        let v = recordError.myDisplacementError.max((displacementAngle.sin() * dist).abs());
                        recordError.m().myDisplacementError = v;
                    }
                    return PositionVector::default();
                } else {
                    let endLength = begShape.at(-2).distance_to2d(begShape.at(-1));
                    let off1 = endLength + extrapolateBeg.min(halfDistance);
                    init.push(PositionVector::position_at_offset2d_seg(begShapeEndLineRev.at(1), begShapeEndLineRev.at(0), off1, 0., false));
                    let off2 = EXT - extrapolateEnd.min(halfDistance);
                    init.push(PositionVector::position_at_offset2d_seg(endShapeBegLine.at(0), endShapeBegLine.at(1), off2, 0., false));
                }
            } else {
                // turning
                //  - end of incoming lane
                //  - intersection of the extrapolated lanes
                //  - begin of outgoing lane
                // attention: if there is no intersection, use a straight line
                let mut intersect = endShapeBegLine.intersection_position2d_pv(&begShapeEndLineRev);
                if intersect == Position::INVALID {
                    *ok = false;
                    if !recordError.is_null() && (shapeFlag & NBNODE_SCURVE_IGNORE) == 0 {
                        // it's unclear if this error can be solved via stretching the intersection.
                        let v = recordError.myDisplacementError.max(1.0);
                        recordError.m().myDisplacementError = v;
                    }
                    return PositionVector::default();
                }
                let begOffset = begShapeEndLineRev.nearest_offset_to_point2d(&intersect, true);
                let endOffset = endShapeBegLine.nearest_offset_to_point2d(&intersect, true);
                let minControlLength = 1.0_f64.min(dist / 2.);
                let distBeg = intersect.distance_to2d(&beg);
                let distEnd = intersect.distance_to2d(&end);
                let lengthenBeg = distBeg <= minControlLength;
                let lengthenEnd = distEnd <= minControlLength;
                if lengthenBeg && lengthenEnd {
                    // This should be fixable with minor stretching
                    if !recordError.is_null() && (shapeFlag & NBNODE_SCURVE_IGNORE) == 0 {
                        let v = recordError.myDisplacementError.max(1.0);
                        recordError.m().myDisplacementError = v;
                    }
                    *ok = false;
                    return PositionVector::default();
                } else if (shapeFlag & NBNODE_FOUR_CONTROL_POINTS) != 0 {
                    init.push(begShapeEndLineRev.position_at_offset2d(EXT - extrapolateBeg, 0., false));
                    init.push(endShapeBegLine.position_at_offset2d(EXT - extrapolateEnd, 0., false));
                } else if lengthenBeg || lengthenEnd {
                    init.push(begShapeEndLineRev.position_at_offset2d(EXT - minControlLength, 0., false));
                    init.push(endShapeBegLine.position_at_offset2d(EXT - minControlLength, 0., false));
                } else if (shapeFlag & NBNODE_AVOID_WIDE_LEFT_TURN) != 0
                    // there are two reasons for enabling special geometry rules:
                    // 1) sharp edge angles which could cause overshoot
                    // 2) junction geometries with a large displacement between opposite left turns
                    //    which would cause the default geometry to overlap
                    && ((shapeFlag & NBNODE_AVOID_INTERSECTING_LEFT_TURNS) != 0
                        || (angle > deg2rad(95.) && (distBeg > 20. || distEnd > 20.)))
                {
                    let factor = if (shapeFlag & NBNODE_AVOID_INTERSECTING_LEFT_TURNS) == 0 {
                        1.
                    } else {
                        0.6_f64.min(16. / dist)
                    };
                    init.push(begShapeEndLineRev.position_at_offset2d(EXT - (distBeg * factor / 1.2).min(dist * factor / 1.8), 0., false));
                    init.push(endShapeBegLine.position_at_offset2d(EXT - (distEnd * factor / 1.2).min(dist * factor / 1.8), 0., false));
                } else if (shapeFlag & NBNODE_AVOID_WIDE_RIGHT_TURN) != 0 && angle < deg2rad(-95.) && (distBeg > 20. || distEnd > 20.) {
                    init.push(begShapeEndLineRev.position_at_offset2d(EXT - (distBeg / 1.4).min(dist / 2.), 0., false));
                    init.push(endShapeBegLine.position_at_offset2d(EXT - (distEnd / 1.4).min(dist / 2.), 0., false));
                } else {
                    let z1 = begShapeEndLineRev.position_at_offset2d(begOffset, 0., false).z();
                    let z2 = endShapeBegLine.position_at_offset2d(endOffset, 0., false).z();
                    let z3 = 0.5 * (beg.z() + end.z());
                    // if z1 and z2 are on the same side in regard to z3 then we
                    // can use their avarage. Otherwise, the intersection in 3D
                    // is not good and we are better of using z3
                    let z;
                    if (z1 <= z3 && z2 <= z3) || (z1 >= z3 && z2 >= z3) {
                        z = 0.5 * (z1 + z2);
                    } else {
                        z = z3;
                    }
                    intersect.set3(intersect.x(), intersect.y(), z);
                    init.push(intersect);
                }
            }
        }
        init.push(end);
        init
    }

    /// C++: `PositionVector NBNode::indirectLeftShape(const PositionVector &, const PositionVector &, int) const` [const]
    /// compute shape of indirect left turn
    // PositionVector
    // NBNode::indirectLeftShape(const PositionVector& begShape, const PositionVector& endShape, int numPoints) const {
    //     UNUSED_PARAMETER(numPoints);
    //     PositionVector result;
    //     result.push_back(begShape.back());
    //     //const double angle = GeomHelper::angleDiff(begShape.angleAt2D(-2), endShape.angleAt2D(0));
    //     PositionVector endShapeBegLine(endShape[0], endShape[1]);
    //     PositionVector begShapeEndLineRev(begShape[-1], begShape[-2]);
    //     endShapeBegLine.extrapolate2D(100, true);
    //     begShapeEndLineRev.extrapolate2D(100, true);
    //     Position intersect = endShapeBegLine.intersectionPosition2D(begShapeEndLineRev);
    //     if (intersect == Position::INVALID) {
    //         WRITE_WARNINGF(TL("Could not compute indirect left turn shape at node '%'"), getID());
    //     } else {
    //         Position dir = intersect;
    //         dir.sub(endShape[0]);
    //         dir.norm2D();
    //         const double radius = myRadius == NBNode::UNSPECIFIED_RADIUS ? OptionsCont::getOptions().getFloat("default.junctions.radius") : myRadius;
    //         dir.mul(radius);
    //         result.push_back(intersect + dir);
    //     }
    //     result.push_back(endShape.front());
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn indirect_left_shape(&self, begShape: &PositionVector, endShape: &PositionVector, numPoints: i32) -> PositionVector {
        let _ = numPoints; // UNUSED_PARAMETER
        let mut result = PositionVector::default();
        result.push(*begShape.back());
        //const double angle = GeomHelper::angleDiff(begShape.angleAt2D(-2), endShape.angleAt2D(0));
        let mut endShapeBegLine = PositionVector::from_line(*endShape.at(0), *endShape.at(1));
        let mut begShapeEndLineRev = PositionVector::from_line(*begShape.at(-1), *begShape.at(-2));
        endShapeBegLine.extrapolate2d(100., true);
        begShapeEndLineRev.extrapolate2d(100., true);
        let intersect = endShapeBegLine.intersection_position2d_pv(&begShapeEndLineRev);
        if intersect == Position::INVALID {
            write_warning(&format!("Could not compute indirect left turn shape at node '{}'", self.get_id()));
        } else {
            let mut dir = intersect;
            dir.sub(endShape.at(0));
            dir.norm2d();
            let radius = if self.myRadius == NBNODE_UNSPECIFIED_RADIUS {
                utils::options::options_cont::get_float("default.junctions.radius")
            } else {
                self.myRadius
            };
            dir = dir * radius;
            result.push(intersect + dir);
        }
        result.push(*endShape.front());
        result
    }

    /// C++: `double NBNode::getDisplacementError() const` [const]
    /// compute the displacement error during s-curve computation
    // double getDisplacementError() const {
    //         return myDisplacementError;
    //     }
    #[allow(unused_variables)]
    pub fn get_displacement_error(&self) -> f64 {
        self.myDisplacementError
    }

    /// C++: `void NBNode::replaceIncoming(NBEdge *, NBEdge *, int)`
    /// Replaces occurrences of the first edge within the list of incoming by the second Connections are remapped, too
    // void
    // NBNode::replaceIncoming(NBEdge* which, NBEdge* by, int laneOff) {
    //     // replace the edge in the list of incoming nodes
    //     EdgeVector::iterator i = std::find(myIncomingEdges.begin(), myIncomingEdges.end(), which);
    //     if (i != myIncomingEdges.end()) {
    //         (*i) = by;
    //         i = std::find(myAllEdges.begin(), myAllEdges.end(), which);
    //         (*i) = by;
    //     }
    //     // replace within the connetion prohibition dependencies
    //     replaceInConnectionProhibitions(which, by, laneOff, 0);
    // }
    #[allow(unused_variables)]
    pub fn replace_incoming(&mut self, which: Ptr<NBEdge>, by: Ptr<NBEdge>, laneOff: i32) {
        // replace the edge in the list of incoming nodes
        if let Some(i) = self.myIncomingEdges.iter().position(|e| *e == which) {
            self.myIncomingEdges[i] = by;
            if let Some(j) = self.myAllEdges.iter().position(|e| *e == which) {
                self.myAllEdges[j] = by;
            }
        }
        // replace within the connetion prohibition dependencies
        self.replace_in_connection_prohibitions(which, by, laneOff, 0);
    }

    /// C++: `void NBNode::replaceIncoming(const EdgeVector &, NBEdge *)`
    /// Replaces occurrences of every edge from the given list within the list of incoming by the second Connections are remapped, too
    // void
    // NBNode::replaceIncoming(const EdgeVector& which, NBEdge* by) {
    //     // replace edges
    //     int laneOff = 0;
    //     for (EdgeVector::const_iterator i = which.begin(); i != which.end(); i++) {
    //         replaceIncoming(*i, by, laneOff);
    //         laneOff += (*i)->getNumLanes();
    //     }
    //     // removed double occurrences
    //     removeDoubleEdges();
    //     // check whether this node belongs to a district and the edges
    //     //  must here be also remapped
    //     if (myDistrict != nullptr) {
    //         myDistrict->replaceIncoming(which, by);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_incoming_2(&mut self, which: &EdgeVector, by: Ptr<NBEdge>) {
        // replace edges
        let mut laneOff: i32 = 0;
        for i in which.iter() {
            self.replace_incoming(*i, by, laneOff);
            laneOff += i.get_num_lanes();
        }
        // removed double occurrences
        self.remove_double_edges();
        // check whether this node belongs to a district and the edges
        //  must here be also remapped
        if !self.myDistrict.is_null() {
            self.myDistrict.m().replace_incoming(which, by);
        }
    }

    /// C++: `void NBNode::replaceOutgoing(NBEdge *, NBEdge *, int)`
    /// Replaces occurrences of the first edge within the list of outgoing by the second Connections are remapped, too
    // void
    // NBNode::replaceOutgoing(NBEdge* which, NBEdge* by, int laneOff) {
    //     // replace the edge in the list of outgoing nodes
    //     EdgeVector::iterator i = std::find(myOutgoingEdges.begin(), myOutgoingEdges.end(), which);
    //     if (i != myOutgoingEdges.end()) {
    //         (*i) = by;
    //         i = std::find(myAllEdges.begin(), myAllEdges.end(), which);
    //         (*i) = by;
    //     }
    //     // replace the edge in connections of incoming edges
    //     for (i = myIncomingEdges.begin(); i != myIncomingEdges.end(); ++i) {
    //         (*i)->replaceInConnections(which, by, laneOff);
    //     }
    //     // replace within the connetion prohibition dependencies
    //     replaceInConnectionProhibitions(which, by, 0, laneOff);
    // }
    #[allow(unused_variables)]
    pub fn replace_outgoing(&mut self, which: Ptr<NBEdge>, by: Ptr<NBEdge>, laneOff: i32) {
        // replace the edge in the list of outgoing nodes
        if let Some(i) = self.myOutgoingEdges.iter().position(|e| *e == which) {
            self.myOutgoingEdges[i] = by;
            if let Some(j) = self.myAllEdges.iter().position(|e| *e == which) {
                self.myAllEdges[j] = by;
            }
        }
        // replace the edge in connections of incoming edges
        for i in self.myIncomingEdges.iter() {
            i.m().replace_in_connections(which, by, laneOff);
        }
        // replace within the connetion prohibition dependencies
        self.replace_in_connection_prohibitions(which, by, 0, laneOff);
    }

    /// C++: `void NBNode::replaceOutgoing(const EdgeVector &, NBEdge *)`
    /// Replaces occurrences of every edge from the given list within the list of outgoing by the second Connections are remapped, too
    // void
    // NBNode::replaceOutgoing(const EdgeVector& which, NBEdge* by) {
    //     // replace edges
    //     int laneOff = 0;
    //     for (EdgeVector::const_iterator i = which.begin(); i != which.end(); i++) {
    //         replaceOutgoing(*i, by, laneOff);
    //         laneOff += (*i)->getNumLanes();
    //     }
    //     // removed double occurrences
    //     removeDoubleEdges();
    //     // check whether this node belongs to a district and the edges
    //     //  must here be also remapped
    //     if (myDistrict != nullptr) {
    //         myDistrict->replaceOutgoing(which, by);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_outgoing_2(&mut self, which: &EdgeVector, by: Ptr<NBEdge>) {
        // replace edges
        let mut laneOff: i32 = 0;
        for i in which.iter() {
            self.replace_outgoing(*i, by, laneOff);
            laneOff += i.get_num_lanes();
        }
        // removed double occurrences
        self.remove_double_edges();
        // check whether this node belongs to a district and the edges
        //  must here be also remapped
        if !self.myDistrict.is_null() {
            self.myDistrict.m().replace_outgoing(which, by);
        }
    }

    /// C++: `int NBNode::guessCrossings()`
    /// guess pedestrian crossings and return how many were guessed
    // int
    // NBNode::guessCrossings() {
    // #ifdef DEBUG_PED_STRUCTURES
    //     gDebugFlag1 = DEBUGCOND;
    // #endif
    //     int numGuessed = 0;
    //     if (myCrossings.size() > 0 || myDiscardAllCrossings) {
    //         // user supplied crossings, do not guess
    //         return numGuessed;
    //     }
    //     DEBUGCOUT(gDebugFlag1, "guess crossings for " << getID() << "\n")
    //     EdgeVector allEdges = getEdgesSortedByAngleAtNodeCenter();
    //     // check for pedestrial lanes going clockwise around the node
    //     std::vector<std::pair<NBEdge*, bool> > normalizedLanes;
    //     for (EdgeVector::const_iterator it = allEdges.begin(); it != allEdges.end(); ++it) {
    //         NBEdge* edge = *it;
    //         const std::vector<NBEdge::Lane>& lanes = edge->getLanes();
    //         if (edge->getFromNode() == this) {
    //             for (std::vector<NBEdge::Lane>::const_reverse_iterator it_l = lanes.rbegin(); it_l != lanes.rend(); ++it_l) {
    //                 normalizedLanes.push_back(std::make_pair(edge, ((*it_l).permissions & SVC_PEDESTRIAN) != 0));
    //             }
    //         } else {
    //             for (std::vector<NBEdge::Lane>::const_iterator it_l = lanes.begin(); it_l != lanes.end(); ++it_l) {
    //                 normalizedLanes.push_back(std::make_pair(edge, ((*it_l).permissions & SVC_PEDESTRIAN) != 0));
    //             }
    //         }
    //     }
    //     // do we even have a pedestrian lane?
    //     int firstSidewalk = -1;
    //     for (int i = 0; i < (int)normalizedLanes.size(); ++i) {
    //         if (normalizedLanes[i].second) {
    //             firstSidewalk = i;
    //             break;
    //         }
    //     }
    //     int hadCandidates = 0;
    //     std::vector<int> connectedCandidates; // number of crossings that were built for each connected candidate
    //     if (firstSidewalk != -1) {
    //         // rotate lanes to ensure that the first one allows pedestrians
    //         std::vector<std::pair<NBEdge*, bool> > tmp;
    //         copy(normalizedLanes.begin() + firstSidewalk, normalizedLanes.end(), std::back_inserter(tmp));
    //         copy(normalizedLanes.begin(), normalizedLanes.begin() + firstSidewalk, std::back_inserter(tmp));
    //         normalizedLanes = tmp;
    //         // find candidates
    //         EdgeVector candidates;
    //         for (int i = 0; i < (int)normalizedLanes.size(); ++i) {
    //             NBEdge* edge = normalizedLanes[i].first;
    //             const bool allowsPed = normalizedLanes[i].second;
    //             DEBUGCOUT(gDebugFlag1, "  cands=" << toString(candidates) << "  edge=" << edge->getID() << " allowsPed=" << allowsPed << "\n")
    //             if (!allowsPed && (candidates.size() == 0 || candidates.back() != edge)) {
    //                 candidates.push_back(edge);
    //             } else if (allowsPed) {
    //                 if (candidates.size() > 0) {
    //                     if (hadCandidates > 0 || forbidsPedestriansAfter(normalizedLanes, i)) {
    //                         hadCandidates++;
    //                         const int n = checkCrossing(candidates);
    //                         numGuessed += n;
    //                         if (n > 0) {
    //                             connectedCandidates.push_back(n);
    //                         }
    //                     }
    //                     candidates.clear();
    //                 }
    //             }
    //         }
    //         if (hadCandidates > 0 && candidates.size() > 0) {
    //             // avoid wrapping around to the same sidewalk
    //             hadCandidates++;
    //             const int n = checkCrossing(candidates);
    //             numGuessed += n;
    //             if (n > 0) {
    //                 connectedCandidates.push_back(n);
    //             }
    //         }
    //     }
    //     // Avoid duplicate crossing between the same pair of walkingareas
    //     DEBUGCOUT(gDebugFlag1, "  hadCandidates=" << hadCandidates << "  connectedCandidates=" << toString(connectedCandidates) << "\n")
    //     if (hadCandidates == 2 && connectedCandidates.size() == 2) {
    //         // One or both of them might be split: remove the one with less splits
    //         if (connectedCandidates.back() <= connectedCandidates.front()) {
    //             numGuessed -= connectedCandidates.back();
    //             myCrossings.erase(myCrossings.end() - connectedCandidates.back(), myCrossings.end());
    //         } else {
    //             numGuessed -= connectedCandidates.front();
    //             myCrossings.erase(myCrossings.begin(), myCrossings.begin() + connectedCandidates.front());
    //         }
    //     }
    //     std::sort(myCrossings.begin(), myCrossings.end(), NBNodesEdgesSorter::crossing_by_junction_angle_sorter(this, myAllEdges));
    // #ifdef DEBUG_PED_STRUCTURES
    //     if (gDebugFlag1) {
    //         std::cout << "guessedCrossings:\n";
    //         for (auto& crossing : myCrossings) {
    //             std::cout << "  edges=" << toString(crossing->edges) << "\n";
    //         }
    //     }
    // #endif
    //     if (numGuessed > 0 && isSimpleContinuation(true, true)) {
    //         // avoid narrow node shape when there is a crossing
    //         computeNodeShape(-1);
    //         for (NBEdge* e : myAllEdges) {
    //             e->computeEdgeShape();
    //         }
    //     }
    //     return numGuessed;
    // }
    #[allow(unused_variables)]
    pub fn guess_crossings(&mut self) -> i32 {
        let mut numGuessed: i32 = 0;
        if self.myCrossings.len() > 0 || self.myDiscardAllCrossings {
            // user supplied crossings, do not guess
            return numGuessed;
        }
        let me = Ptr::from_mut(self);
        let allEdges = self.get_edges_sorted_by_angle_at_node_center();
        // check for pedestrial lanes going clockwise around the node
        let mut normalizedLanes: Vec<(Ptr<NBEdge>, bool)> = Vec::new();
        for it in allEdges.iter() {
            let edge = *it;
            let lanes = edge.get_lanes();
            if edge.get_from_node() == me {
                for lane in lanes.iter().rev() {
                    normalizedLanes.push((edge, (lane.permissions & SVC_PEDESTRIAN) != 0));
                }
            } else {
                for lane in lanes.iter() {
                    normalizedLanes.push((edge, (lane.permissions & SVC_PEDESTRIAN) != 0));
                }
            }
        }
        // do we even have a pedestrian lane?
        let mut firstSidewalk: i32 = -1;
        for i in 0..normalizedLanes.len() {
            if normalizedLanes[i].1 {
                firstSidewalk = i as i32;
                break;
            }
        }
        let mut hadCandidates: i32 = 0;
        let mut connectedCandidates: Vec<i32> = Vec::new(); // number of crossings that were built for each connected candidate
        if firstSidewalk != -1 {
            // rotate lanes to ensure that the first one allows pedestrians
            let fs = firstSidewalk as usize;
            let mut tmp: Vec<(Ptr<NBEdge>, bool)> = Vec::new();
            tmp.extend_from_slice(&normalizedLanes[fs..]);
            tmp.extend_from_slice(&normalizedLanes[..fs]);
            normalizedLanes = tmp;
            // find candidates
            let mut candidates: EdgeVector = Vec::new();
            for i in 0..normalizedLanes.len() {
                let edge = normalizedLanes[i].0;
                let allowsPed = normalizedLanes[i].1;
                if !allowsPed && (candidates.len() == 0 || candidates.last() != Some(&edge)) {
                    candidates.push(edge);
                } else if allowsPed {
                    if candidates.len() > 0 {
                        if hadCandidates > 0 || self.forbids_pedestrians_after(normalizedLanes.clone(), i as i32) {
                            hadCandidates += 1;
                            let n = self.check_crossing(candidates.clone(), false);
                            numGuessed += n;
                            if n > 0 {
                                connectedCandidates.push(n);
                            }
                        }
                        candidates.clear();
                    }
                }
            }
            if hadCandidates > 0 && candidates.len() > 0 {
                // avoid wrapping around to the same sidewalk
                hadCandidates += 1;
                let n = self.check_crossing(candidates.clone(), false);
                numGuessed += n;
                if n > 0 {
                    connectedCandidates.push(n);
                }
            }
        }
        // Avoid duplicate crossing between the same pair of walkingareas
        if hadCandidates == 2 && connectedCandidates.len() == 2 {
            // One or both of them might be split: remove the one with less splits
            if connectedCandidates[1] <= connectedCandidates[0] {
                numGuessed -= connectedCandidates[1];
                let keep = self.myCrossings.len() - connectedCandidates[1] as usize;
                self.myCrossings.truncate(keep);
            } else {
                numGuessed -= connectedCandidates[0];
                self.myCrossings.drain(0..connectedCandidates[0] as usize);
            }
        }
        // determinism: C++ std::sort is unstable; a stable sort_by is used here
        let sorter = NBNodesEdgesSorter_crossing_by_junction_angle_sorter::new(Ptr::from_mut(self), &self.myAllEdges);
        self.myCrossings.sort_by(|a, b| {
            if sorter.call_op(a, b) != 0 {
                std::cmp::Ordering::Less
            } else if sorter.call_op(b, a) != 0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        if numGuessed > 0 && self.is_simple_continuation(true, true) {
            // avoid narrow node shape when there is a crossing
            self.compute_node_shape(-1.);
            for e in self.myAllEdges.iter() {
                e.m().compute_edge_shape(-1.);
            }
        }
        numGuessed
    }

    /// C++: `int NBNode::checkCrossing(EdgeVector, bool)`
    // int
    // NBNode::checkCrossing(EdgeVector candidates, bool checkOnly) {
    //     DEBUGCOUT(gDebugFlag1, "checkCrossing candidates=" << toString(candidates) << "\n")
    //     if (candidates.size() == 0) {
    //         DEBUGCOUT(gDebugFlag1, "no crossing added (numCandidates=" << candidates.size() << ")\n")
    //         return 0;
    //     } else {
    //         // check whether the edges may be part of a common crossing due to having similar angle
    //         double prevAngle = -100000; // dummy
    //         for (int i = 0; i < (int)candidates.size(); ++i) {
    //             NBEdge* edge = candidates[i];
    //             double angle = edge->getCrossingAngle(this);
    //             // edges should be sorted by angle but this only holds true approximately
    //             if (i > 0 && fabs(NBHelpers::relAngle(angle, prevAngle)) > EXTEND_CROSSING_ANGLE_THRESHOLD) {
    //                 DEBUGCOUT(gDebugFlag1, "no crossing added (found angle difference of " << fabs(NBHelpers::relAngle(angle, prevAngle)) << " at i=" << i << "\n")
    //                 return 0;
    //             }
    //             if (!checkOnly && !isTLControlled() && myType != SumoXMLNodeType::RAIL_CROSSING && edge->getSpeed() > OptionsCont::getOptions().getFloat("crossings.guess.speed-threshold")) {
    //                 DEBUGCOUT(gDebugFlag1, "no crossing added (uncontrolled, edge with speed > " << edge->getSpeed() << ")\n")
    //                 return 0;
    //             }
    //             prevAngle = angle;
    //         }
    //         if (candidates.size() == 1 || getType() == SumoXMLNodeType::RAIL_CROSSING) {
    //             if (!checkOnly) {
    //                 addCrossing(candidates, NBEdge::UNSPECIFIED_WIDTH, isTLControlled()
    //                             || (isRoundabout() && OptionsCont::getOptions().getBool("crossings.guess.roundabout-priority")));
    //                 DEBUGCOUT(gDebugFlag1, "adding crossing: " << toString(candidates) << "\n")
    //             }
    //             return 1;
    //         } else {
    //             // check for intermediate walking areas
    //             prevAngle = -100000; // dummy
    //             for (EdgeVector::iterator it = candidates.begin(); it != candidates.end(); ++it) {
    //                 double angle = (*it)->getCrossingAngle(this);
    //                 if (it != candidates.begin()) {
    //                     NBEdge* prev = *(it - 1);
    //                     NBEdge* curr = *it;
    //                     Position prevPos, currPos;
    //                     int laneI;
    //                     // compute distance between candiate edges
    //                     double intermediateWidth = 0;
    //                     if (prev->getToNode() == this) {
    //                         laneI = prev->getNumLanes() - 1;
    //                         prevPos = prev->getLanes()[laneI].shape[-1];
    //                     } else {
    //                         laneI = 0;
    //                         prevPos = prev->getLanes()[laneI].shape[0];
    //                     }
    //                     intermediateWidth -= 0.5 * prev->getLaneWidth(laneI);
    //                     if (curr->getFromNode() == this) {
    //                         laneI = curr->getNumLanes() - 1;
    //                         currPos = curr->getLanes()[laneI].shape[0];
    //                     } else {
    //                         laneI = 0;
    //                         currPos = curr->getLanes()[laneI].shape[-1];
    //                     }
    //                     intermediateWidth -= 0.5 * curr->getLaneWidth(laneI);
    //                     intermediateWidth += currPos.distanceTo2D(prevPos);
    //                     DEBUGCOUT(gDebugFlag1, " prevAngle=" << prevAngle << " angle=" << angle << " intermediateWidth=" << intermediateWidth << "\n")
    //                     if (fabs(NBHelpers::relAngle(prevAngle, angle)) > SPLIT_CROSSING_ANGLE_THRESHOLD
    //                             || (intermediateWidth > SPLIT_CROSSING_WIDTH_THRESHOLD)) {
    //                         return checkCrossing(EdgeVector(candidates.begin(), it), checkOnly)
    //                                + checkCrossing(EdgeVector(it, candidates.end()), checkOnly);
    //                     }
    //                 }
    //                 prevAngle = angle;
    //             }
    //             if (!checkOnly) {
    //                 addCrossing(candidates, NBEdge::UNSPECIFIED_WIDTH, isTLControlled()
    //                             || (isRoundabout() && OptionsCont::getOptions().getBool("crossings.guess.roundabout-priority")));
    //                 DEBUGCOUT(gDebugFlag1, "adding crossing: " << toString(candidates) << "\n")
    //             }
    //             return 1;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_crossing(&mut self, candidates: EdgeVector, checkOnly: bool) -> i32 {
        if candidates.len() == 0 {
            // no crossing added (numCandidates=0)
            return 0;
        }
        let me = Ptr::from_mut(self);
        // check whether the edges may be part of a common crossing due to having similar angle
        let mut prevAngle: f64 = -100000.; // dummy
        for i in 0..candidates.len() {
            let edge = candidates[i];
            let angle = candidates[i].m().get_crossing_angle(me);
            // edges should be sorted by angle but this only holds true approximately
            if i > 0 && NBHelpers::rel_angle(angle, prevAngle).abs() > NBNODE_EXTEND_CROSSING_ANGLE_THRESHOLD {
                return 0;
            }
            if !checkOnly && !self.is_tl_controlled() && self.get_type() != SumoXMLNodeType::RAIL_CROSSING && edge.get_speed() > utils::options::options_cont::get_float("crossings.guess.speed-threshold") {
                return 0;
            }
            prevAngle = angle;
        }
        if candidates.len() == 1 || self.get_type() == SumoXMLNodeType::RAIL_CROSSING {
            if !checkOnly {
                let priority = self.is_tl_controlled() || (self.is_roundabout() && utils::options::options_cont::get_bool("crossings.guess.roundabout-priority"));
                self.add_crossing(candidates, NBEDGE_UNSPECIFIED_WIDTH, priority, -1, -1, &PositionVector::default(), false, Ptr::default());
            }
            return 1;
        }
        // check for intermediate walking areas
        prevAngle = -100000.; // dummy
        for it in 0..candidates.len() {
            let angle = candidates[it].m().get_crossing_angle(me);
            if it != 0 {
                let prev = candidates[it - 1];
                let curr = candidates[it];
                // compute distance between candiate edges
                let mut intermediateWidth: f64 = 0.;
                let mut laneI: i32;
                let prevPos: Position;
                if prev.get_to_node() == me {
                    laneI = prev.get_num_lanes() - 1;
                    prevPos = *prev.get_lanes()[laneI as usize].shape.at(-1);
                } else {
                    laneI = 0;
                    prevPos = *prev.get_lanes()[laneI as usize].shape.at(0);
                }
                intermediateWidth -= 0.5 * prev.get_lane_width_2(laneI);
                let currPos: Position;
                if curr.get_from_node() == me {
                    laneI = curr.get_num_lanes() - 1;
                    currPos = *curr.get_lanes()[laneI as usize].shape.at(0);
                } else {
                    laneI = 0;
                    currPos = *curr.get_lanes()[laneI as usize].shape.at(-1);
                }
                intermediateWidth -= 0.5 * curr.get_lane_width_2(laneI);
                intermediateWidth += currPos.distance_to2d(&prevPos);
                if NBHelpers::rel_angle(prevAngle, angle).abs() > NBNODE_SPLIT_CROSSING_ANGLE_THRESHOLD || (intermediateWidth > NBNODE_SPLIT_CROSSING_WIDTH_THRESHOLD) {
                    // determinism: C++ leaves the evaluation order of the two operands of '+' unspecified; left-to-right here
                    let first = self.check_crossing(candidates[..it].to_vec(), checkOnly);
                    let second = self.check_crossing(candidates[it..].to_vec(), checkOnly);
                    return first + second;
                }
            }
            prevAngle = angle;
        }
        if !checkOnly {
            let priority = self.is_tl_controlled() || (self.is_roundabout() && utils::options::options_cont::get_bool("crossings.guess.roundabout-priority"));
            self.add_crossing(candidates, NBEDGE_UNSPECIFIED_WIDTH, priority, -1, -1, &PositionVector::default(), false, Ptr::default());
        }
        1
    }

    /// C++: `bool NBNode::checkCrossingDuplicated(EdgeVector)`
    /// return true if there already exist a crossing with the same edges as the input
    // bool
    // NBNode::checkCrossingDuplicated(EdgeVector edges) {
    //     // sort edge vector
    //     std::sort(edges.begin(), edges.end());
    //     // iterate over crossing to find a crossing with the same edges
    //     for (auto& crossing : myCrossings) {
    //         // sort edges of crossing before compare
    //         EdgeVector edgesOfCrossing = crossing->edges;
    //         std::sort(edgesOfCrossing.begin(), edgesOfCrossing.end());
    //         if (edgesOfCrossing == edges) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_crossing_duplicated(&mut self, edges: EdgeVector) -> bool {
        // sort edge vector
        let mut edges = edges;
        edges.sort();
        // iterate over crossing to find a crossing with the same edges
        for crossing in self.myCrossings.iter() {
            // sort edges of crossing before compare
            let mut edgesOfCrossing: EdgeVector = crossing.edges.clone();
            edgesOfCrossing.sort();
            if edgesOfCrossing == edges {
                return true;
            }
        }
        false
    }

    /// C++: `double NBNode::buildInnerEdges()`
    /// build internal lanes, pedestrian crossings and walking areas
    // double
    // NBNode::buildInnerEdges() {
    //     // myDisplacementError is computed during this operation. reset first
    //     myDisplacementError = 0.;
    //     // build inner edges for vehicle movements across the junction
    //     int noInternalNoSplits = 0;
    //     for (const NBEdge* const edge : myIncomingEdges) {
    //         for (const NBEdge::Connection& con : edge->getConnections()) {
    //             if (con.toEdge == nullptr) {
    //                 continue;
    //             }
    //             noInternalNoSplits++;
    //         }
    //     }
    //     int lno = 0;
    //     int splitNo = 0;
    //     double maxCrossingSeconds = 0.;
    //     for (NBEdge* const edge : myIncomingEdges) {
    //         maxCrossingSeconds = MAX2(maxCrossingSeconds, edge->buildInnerEdges(*this, noInternalNoSplits, lno, splitNo));
    //     }
    //     return maxCrossingSeconds;
    // }
    #[allow(unused_variables)]
    pub fn build_inner_edges(&mut self) -> f64 {
        // myDisplacementError is computed during this operation. reset first
        self.myDisplacementError = 0.;
        // build inner edges for vehicle movements across the junction
        let mut noInternalNoSplits: i32 = 0;
        for edge in self.myIncomingEdges.iter() {
            for con in edge.get_connections().iter() {
                if con.toEdge.is_null() {
                    continue;
                }
                noInternalNoSplits += 1;
            }
        }
        let mut lno: i32 = 0;
        let mut splitNo: i32 = 0;
        let mut maxCrossingSeconds: f64 = 0.;
        for edge in self.myIncomingEdges.iter() {
            maxCrossingSeconds = maxCrossingSeconds.max(edge.m().build_inner_edges(&*self, noInternalNoSplits, &mut lno, &mut splitNo));
        }
        maxCrossingSeconds
    }

    /// C++: `int NBNode::buildCrossings()`
    /// build pedestrian crossings
    // int
    // NBNode::buildCrossings() {
    // #ifdef DEBUG_PED_STRUCTURES
    //     gDebugFlag1 = DEBUGCOND;
    // #endif
    //     DEBUGCOUT(gDebugFlag1, "build crossings for " << getID() << ":\n")
    //     if (myDiscardAllCrossings) {
    //         myCrossings.clear();
    //     }
    //     int index = 0;
    //     const double defaultWidth = OptionsCont::getOptions().getFloat("default.crossing-width");
    //     for (auto& c : myCrossings) {
    //         c->valid = true;
    //         c->tlID = ""; // reset for Netedit, set via setCrossingTLIndices()
    //         c->id = ":" + getID() + "_c" + toString(index++);
    //         c->width = (c->customWidth == NBEdge::UNSPECIFIED_WIDTH) ? defaultWidth : c->customWidth;
    //         // reset fields, so repeated computation (Netedit) will successfully perform the checks
    //         // in buildWalkingAreas (split crossings) and buildInnerEdges (sanity check)
    //         c->nextWalkingArea = "";
    //         c->prevWalkingArea = "";
    //         EdgeVector& edges = c->edges;
    //         DEBUGCOUT(gDebugFlag1, "  crossing=" << c->id << " edges=" << toString(edges))
    //         // sorting the edges in the right way is imperative. We want to sort
    //         // them by getAngleAtNodeToCenter() but need to be extra carefull to avoid wrapping around 0 somewhere in between
    //         std::sort(edges.begin(), edges.end(), NBContHelper::edge_by_angle_to_nodeShapeCentroid_sorter(this));
    //         DEBUGCOUT(gDebugFlag1, " sortedEdges=" << toString(edges) << "\n")
    //         // rotate the edges so that the largest relative angle difference comes at the end
    //         std::vector<double> rawAngleDiffs;
    //         double maxAngleDiff = 0;
    //         int maxAngleDiffIndex = 0; // index before maxDist
    //         for (int i = 0; i < (int) edges.size(); i++) {
    //             double diff = NBHelpers::relAngle(edges[i]->getAngleAtNodeToCenter(this),
    //                                               edges[(i + 1) % edges.size()]->getAngleAtNodeToCenter(this));
    //             if (diff < 0) {
    //                 diff += 360;
    //             }
    //             const double rawDiff = NBHelpers::relAngle(
    //                                        edges[i]->getAngleAtNodeNormalized(this),
    //                                        edges[(i + 1) % edges.size()]->getAngleAtNodeNormalized(this));
    //             rawAngleDiffs.push_back(fabs(rawDiff));
    //
    //             DEBUGCOUT(gDebugFlag1, "   i=" << i << " a1=" << edges[i]->getAngleAtNodeToCenter(this) << " a2=" << edges[(i + 1) % edges.size()]->getAngleAtNodeToCenter(this) << " diff=" << diff << "\n")
    //             if (diff > maxAngleDiff) {
    //                 maxAngleDiff = diff;
    //                 maxAngleDiffIndex = i;
    //             }
    //         }
    //         if (maxAngleDiff > 2 && maxAngleDiff < 360 - 2) {
    //             // if the angle differences is too small, we better not rotate
    //             std::rotate(edges.begin(), edges.begin() + (maxAngleDiffIndex + 1) % edges.size(), edges.end());
    //             DEBUGCOUT(gDebugFlag1, " rotatedEdges=" << toString(edges))
    //         }
    //         bool diagonalCrossing = false;
    //         std::sort(rawAngleDiffs.begin(), rawAngleDiffs.end());
    //         if (rawAngleDiffs.size() >= 2 && rawAngleDiffs[rawAngleDiffs.size() - 2] > 30) {
    //             diagonalCrossing = true;
    // #ifdef DEBUG_PED_STRUCTURES
    //             if (gDebugFlag1) {
    //                 std::cout << " detected pedScramble " << c->id << " edges=" << toString(edges) << " rawDiffs=" << toString(rawAngleDiffs) << "\n";
    //                 for (auto e : edges) {
    //                     std::cout << "  e=" << e->getID()
    //                               << " aC=" << e->getAngleAtNodeToCenter(this)
    //                               << " a=" << e->getAngleAtNode(this)
    //                               << " aN=" << e->getAngleAtNodeNormalized(this)
    //                               << "\n";
    //                 }
    //             }
    // #endif
    //         }
    //         // reverse to get them in CCW order (walking direction around the node)
    //         std::reverse(edges.begin(), edges.end());
    //         // compute shape
    //         c->shape.clear();
    //         const int begDir = (edges.front()->getFromNode() == this ? FORWARD : BACKWARD);
    //         const int endDir = (edges.back()->getToNode() == this ? FORWARD : BACKWARD);
    //         int firstNonPedLane = edges.front()->getFirstNonPedestrianLaneIndex(begDir);
    //         int lastNonPedLane = edges.back()->getFirstNonPedestrianLaneIndex(endDir);
    //         DEBUGCOUT(gDebugFlag1, " finalEdges=" << toString(edges) << " firstNonPedLane=" << firstNonPedLane << " lastNonPedLane=" << lastNonPedLane << "\n")
    //         if (firstNonPedLane < 0 || lastNonPedLane < 0) {
    //             // invalid crossing
    //             WRITE_WARNINGF(TL("Discarding invalid crossing '%' at junction '%' with edges [%] (no vehicle lanes to cross)."), c->id, getID(), toString(c->edges));
    //             c->valid = false;
    //             // compute surrogate shape to make it visible in netedit
    //             firstNonPedLane = begDir == FORWARD ? 0 : edges.front()->getNumLanes() - 1;
    //             lastNonPedLane = endDir == FORWARD ? 0 : edges.back()->getNumLanes() - 1;
    //         }
    //         if (c->customShape.size() != 0) {
    //             c->shape = c->customShape;
    //         } else {
    //             NBEdge::Lane crossingBeg = edges.front()->getLanes()[firstNonPedLane];
    //             NBEdge::Lane crossingEnd = edges.back()->getLanes()[lastNonPedLane];
    //             crossingBeg.width = (crossingBeg.width == NBEdge::UNSPECIFIED_WIDTH ? SUMO_const_laneWidth : crossingBeg.width);
    //             crossingEnd.width = (crossingEnd.width == NBEdge::UNSPECIFIED_WIDTH ? SUMO_const_laneWidth : crossingEnd.width);
    //             crossingBeg.shape.move2side(begDir * crossingBeg.width / 2);
    //             crossingEnd.shape.move2side(endDir * crossingEnd.width / 2);
    //             double offset = c->width / 2;
    //             patchOffset_pathAcrossStreet(offset);
    //             crossingBeg.shape.extrapolate(offset);
    //             crossingEnd.shape.extrapolate(offset);
    //             // check if after all changes shape are NAN (in these case, discard)
    //             if (crossingBeg.shape.isNAN() || crossingEnd.shape.isNAN()) {
    //                 WRITE_WARNINGF(TL("Discarding invalid crossing '%' at junction '%' with edges [%] (invalid shape)."), c->id, getID(), toString(c->edges));
    //                 c->valid = false;
    //             } else {
    //                 c->shape.push_back(crossingBeg.shape[begDir == FORWARD ? 0 : -1]);
    //                 c->shape.push_back(crossingEnd.shape[endDir == FORWARD ? -1 : 0]);
    //             }
    //             if (diagonalCrossing) {
    //                 c->shape.move2side(-c->width);
    //             }
    //         }
    //     }
    //     return index;
    // }
    #[allow(unused_variables)]
    pub fn build_crossings(&mut self) -> i32 {
        let me = Ptr::from_mut(self);
        if self.myDiscardAllCrossings {
            self.myCrossings.clear();
        }
        let mut index: i32 = 0;
        let defaultWidth = utils::options::options_cont::get_float("default.crossing-width");
        for ci in 0..self.myCrossings.len() {
            self.myCrossings[ci].valid = true;
            self.myCrossings[ci].tlID = String::new(); // reset for Netedit, set via setCrossingTLIndices()
            self.myCrossings[ci].id = format!(":{}_c{}", self.get_id(), index);
            index += 1;
            self.myCrossings[ci].width = if self.myCrossings[ci].customWidth == NBEDGE_UNSPECIFIED_WIDTH { defaultWidth } else { self.myCrossings[ci].customWidth };
            // reset fields, so repeated computation (Netedit) will successfully perform the checks
            // in buildWalkingAreas (split crossings) and buildInnerEdges (sanity check)
            self.myCrossings[ci].nextWalkingArea = String::new();
            self.myCrossings[ci].prevWalkingArea = String::new();
            let mut edges: EdgeVector = self.myCrossings[ci].edges.clone();
            // sorting the edges in the right way is imperative. We want to sort
            // them by getAngleAtNodeToCenter() but need to be extra carefull to avoid wrapping around 0 somewhere in between
            let sorter = NBContHelper_edge_by_angle_to_nodeShapeCentroid_sorter::new(me);
            edges.sort_by(|a, b| {
                if sorter.call_op(*a, *b) {
                    std::cmp::Ordering::Less
                } else if sorter.call_op(*b, *a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            // rotate the edges so that the largest relative angle difference comes at the end
            let mut rawAngleDiffs: Vec<f64> = Vec::new();
            let mut maxAngleDiff: f64 = 0.;
            let mut maxAngleDiffIndex: usize = 0; // index before maxDist
            let n = edges.len();
            for i in 0..n {
                let mut diff = NBHelpers::rel_angle(edges[i].get_angle_at_node_to_center(me), edges[(i + 1) % n].get_angle_at_node_to_center(me));
                if diff < 0. {
                    diff += 360.;
                }
                let rawDiff = NBHelpers::rel_angle(edges[i].get_angle_at_node_normalized(me), edges[(i + 1) % n].get_angle_at_node_normalized(me));
                rawAngleDiffs.push(rawDiff.abs());
                if diff > maxAngleDiff {
                    maxAngleDiff = diff;
                    maxAngleDiffIndex = i;
                }
            }
            if maxAngleDiff > 2. && maxAngleDiff < 360. - 2. {
                // if the angle differences is too small, we better not rotate
                // std::rotate(begin, begin + (idx+1)%size, end) is a left rotation by that amount
                edges.rotate_left((maxAngleDiffIndex + 1) % n);
            }
            let mut diagonalCrossing = false;
            rawAngleDiffs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            if rawAngleDiffs.len() >= 2 && rawAngleDiffs[rawAngleDiffs.len() - 2] > 30. {
                diagonalCrossing = true;
            }
            // reverse to get them in CCW order (walking direction around the node)
            edges.reverse();
            // edges is a reference to c->edges in C++; keep the stored crossing in sync
            self.myCrossings[ci].edges = edges.clone();
            // compute shape
            self.myCrossings[ci].shape = PositionVector::default();
            let begDir: i32 = if edges[0].get_from_node() == me { NBNODE_FORWARD } else { NBNODE_BACKWARD };
            let endDir: i32 = if edges[edges.len() - 1].get_to_node() == me { NBNODE_FORWARD } else { NBNODE_BACKWARD };
            let mut firstNonPedLane = edges[0].get_first_non_pedestrian_lane_index(begDir, false);
            let mut lastNonPedLane = edges[edges.len() - 1].get_first_non_pedestrian_lane_index(endDir, false);
            if firstNonPedLane < 0 || lastNonPedLane < 0 {
                // invalid crossing
                let edgeIds: Vec<String> = edges.iter().map(|e| e.get_id().to_string()).collect();
                write_warning(&format!(
                    "Discarding invalid crossing '{}' at junction '{}' with edges [{}] (no vehicle lanes to cross).",
                    self.myCrossings[ci].id,
                    self.get_id(),
                    edgeIds.join(" ")
                ));
                self.myCrossings[ci].valid = false;
                // compute surrogate shape to make it visible in netedit
                firstNonPedLane = if begDir == NBNODE_FORWARD { 0 } else { edges[0].get_num_lanes() - 1 };
                lastNonPedLane = if endDir == NBNODE_FORWARD { 0 } else { edges[edges.len() - 1].get_num_lanes() - 1 };
            }
            if self.myCrossings[ci].customShape.len() != 0 {
                self.myCrossings[ci].shape = self.myCrossings[ci].customShape.clone();
            } else {
                let mut crossingBegShape: PositionVector = edges[0].get_lanes()[firstNonPedLane as usize].shape.clone();
                let mut crossingBegWidth: f64 = edges[0].get_lanes()[firstNonPedLane as usize].width;
                let mut crossingEndShape: PositionVector = edges[edges.len() - 1].get_lanes()[lastNonPedLane as usize].shape.clone();
                let mut crossingEndWidth: f64 = edges[edges.len() - 1].get_lanes()[lastNonPedLane as usize].width;
                crossingBegWidth = if crossingBegWidth == NBEDGE_UNSPECIFIED_WIDTH { NBNODE_SUMO_CONST_LANE_WIDTH } else { crossingBegWidth };
                crossingEndWidth = if crossingEndWidth == NBEDGE_UNSPECIFIED_WIDTH { NBNODE_SUMO_CONST_LANE_WIDTH } else { crossingEndWidth };
                crossingBegShape.move2side((begDir as f64) * crossingBegWidth / 2., 100.);
                crossingEndShape.move2side((endDir as f64) * crossingEndWidth / 2., 100.);
                let mut offset = self.myCrossings[ci].width / 2.;
                self.patch_offset_path_across_street(&mut offset);
                crossingBegShape.extrapolate(offset, false, false);
                crossingEndShape.extrapolate(offset, false, false);
                // check if after all changes shape are NAN (in these case, discard)
                if crossingBegShape.is_nan() || crossingEndShape.is_nan() {
                    let edgeIds: Vec<String> = edges.iter().map(|e| e.get_id().to_string()).collect();
                    write_warning(&format!(
                        "Discarding invalid crossing '{}' at junction '{}' with edges [{}] (invalid shape).",
                        self.myCrossings[ci].id,
                        self.get_id(),
                        edgeIds.join(" ")
                    ));
                    self.myCrossings[ci].valid = false;
                } else {
                    let begPt = *crossingBegShape.at(if begDir == NBNODE_FORWARD { 0 } else { -1 });
                    let endPt = *crossingEndShape.at(if endDir == NBNODE_FORWARD { -1 } else { 0 });
                    self.myCrossings[ci].shape.push(begPt);
                    self.myCrossings[ci].shape.push(endPt);
                }
                if diagonalCrossing {
                    let w = -self.myCrossings[ci].width;
                    self.myCrossings[ci].shape.move2side(w, 100.);
                }
            }
        }
        index
    }

    /// C++: `void NBNode::buildWalkingAreas(int, double)`
    /// build pedestrian walking areas and set connections from/to walkingAreas
    // void
    // NBNode::buildWalkingAreas(int cornerDetail, double joinMinDist) {
    // #ifdef DEBUG_PED_STRUCTURES
    //     gDebugFlag1 = DEBUGCOND;
    // #endif
    //     int index = 0;
    //     myWalkingAreas.clear();
    //     DEBUGCOUT(gDebugFlag1, "build walkingAreas for " << getID() << ":\n")
    //     if (myAllEdges.size() == 0) {
    //         return;
    //     }
    //     EdgeVector allEdges = getEdgesSortedByAngleAtNodeCenter();
    //     // shapes are all pointing away from the intersection
    //     std::vector<std::pair<NBEdge*, NBEdge::Lane> > normalizedLanes;
    //     for (EdgeVector::const_iterator it = allEdges.begin(); it != allEdges.end(); ++it) {
    //         NBEdge* edge = *it;
    //         const std::vector<NBEdge::Lane>& lanes = edge->getLanes();
    //         std::vector<NBEdge::Lane> tmp;
    //         bool hadSidewalk = false;
    //         bool hadNonSidewalk = false;
    //         for (int i = 0; i < (int)lanes.size(); i++) {
    //             NBEdge::Lane l = lanes[i];
    //             const bool sidewalk = (l.permissions & SVC_PEDESTRIAN) != 0;
    //             if (sidewalk) {
    //                 if (hadSidewalk && hadNonSidewalk) {
    //                     if (edge->getFromNode() == this) {
    //                         WRITE_WARNINGF(TL("Ignoring additional sidewalk lane % on edge '%' for walkingareas."),
    //                                        i, edge->getID());
    //                     }
    //                     continue;
    //                 }
    //                 hadSidewalk = true;
    //             } else {
    //                 hadNonSidewalk = true;
    //             }
    //             tmp.push_back(l);
    //         }
    //         if (edge->getFromNode() == this) {
    //             std::reverse(tmp.begin(), tmp.end());
    //         } else {
    //             for (NBEdge::Lane& l : tmp) {
    //                 l.shape = l.shape.reverse();
    //             }
    //         }
    //         for (NBEdge::Lane& l : tmp) {
    //             l.shape = l.shape.getSubpartByIndex(0, 2);
    //             l.width = (l.width == NBEdge::UNSPECIFIED_WIDTH ? SUMO_const_laneWidth : l.width);
    //             normalizedLanes.push_back(std::make_pair(edge, l));
    //         }
    //     }
    //     //if (gDebugFlag1) std::cout << "  normalizedLanes=" << normalizedLanes.size() << "\n";
    //     // collect [start,count[ indices in normalizedLanes that belong to a walkingArea
    //     std::vector<std::pair<int, int> > waIndices;
    //     int start = -1;
    //     NBEdge* prevEdge = normalizedLanes.back().first;
    //     for (int i = 0; i < (int)normalizedLanes.size(); ++i) {
    //         NBEdge* edge = normalizedLanes[i].first;
    //         NBEdge::Lane& l = normalizedLanes[i].second;
    //         if (start == -1) {
    //             if ((l.permissions & SVC_PEDESTRIAN) != 0) {
    //                 start = i;
    //             }
    //         } else {
    //             if ((l.permissions & SVC_PEDESTRIAN) == 0
    //                     || crossingBetween(edge, prevEdge)
    //                     || alreadyConnectedPaths(edge, prevEdge, joinMinDist)
    //                     || crossesFringe(edge, prevEdge)
    //                ) {
    //                 waIndices.push_back(std::make_pair(start, i - start));
    //                 if ((l.permissions & SVC_PEDESTRIAN) != 0) {
    //                     start = i;
    //                 } else {
    //                     start = -1;
    //                 }
    //
    //             }
    //         }
    //         DEBUGCOUT(gDebugFlag1, "     i=" << i << " edge=" << edge->getID() << " start=" << start << " ped=" << ((l.permissions & SVC_PEDESTRIAN) != 0)
    //                   << " waI=" << waIndices.size() << " crossingBetween=" << crossingBetween(edge, prevEdge) << "\n")
    //         prevEdge = edge;
    //     }
    //     // deal with wrap-around issues
    //     if (start != - 1) {
    //         const int waNumLanes = (int)normalizedLanes.size() - start;
    //         if (waIndices.size() == 0) {
    //             waIndices.push_back(std::make_pair(start, waNumLanes));
    //             DEBUGCOUT(gDebugFlag1, "  single wa, end at wrap-around\n")
    //         } else {
    //             if (waIndices.front().first == 0) {
    //                 NBEdge* edge = normalizedLanes.front().first;
    //                 if (crossingBetween(edge, normalizedLanes.back().first)
    //                         || crossesFringe(edge, normalizedLanes.back().first)) {
    //                     // do not wrap-around (see above)
    //                     waIndices.push_back(std::make_pair(start, waNumLanes));
    //                     DEBUGCOUT(gDebugFlag1, "  do not wrap around\n")
    //                 } else {
    //                     // first walkingArea wraps around
    //                     waIndices.front().first = start;
    //                     waIndices.front().second = waNumLanes + waIndices.front().second;
    //                     DEBUGCOUT(gDebugFlag1, "  wrapping around\n")
    //                 }
    //             } else {
    //                 // last walkingArea ends at the wrap-around
    //                 waIndices.push_back(std::make_pair(start, waNumLanes));
    //                 DEBUGCOUT(gDebugFlag1, "  end at wrap-around\n")
    //             }
    //         }
    //     }
    // #ifdef DEBUG_PED_STRUCTURES
    //     if (gDebugFlag1) {
    //         std::cout << "  normalizedLanes=" << normalizedLanes.size() << " waIndices:\n";
    //         for (int i = 0; i < (int)waIndices.size(); ++i) {
    //             std::cout << "   " << waIndices[i].first << ", " << waIndices[i].second << "\n";
    //         }
    //     }
    // #endif
    //     // build walking areas connected to a sidewalk
    //     for (int i = 0; i < (int)waIndices.size(); ++i) {
    //         const bool buildExtensions = waIndices[i].second != (int)normalizedLanes.size();
    //         int startIdx = waIndices[i].first;
    //         const int prev = startIdx > 0 ? startIdx - 1 : (int)normalizedLanes.size() - 1;
    //         const int count = waIndices[i].second;
    //         const int end = (startIdx + count) % normalizedLanes.size();
    //         int lastIdx = (startIdx + count - 1) % normalizedLanes.size();
    //
    //         WalkingArea wa(":" + getID() + "_w" + toString(index++), 1);
    //         DEBUGCOUT(gDebugFlag1, "build walkingArea " << wa.id << " start=" << startIdx << " end=" << end << " count=" << count << " prev=" << prev << ":\n")
    //         double endCrossingWidth = 0;
    //         double startCrossingWidth = 0;
    //         PositionVector endCrossingShape;
    //         PositionVector startCrossingShape;
    //         // check for connected crossings
    //         bool connectsCrossing = false;
    //         bool crossingNearSidewalk = false;
    //         int numCrossings = 0;
    //         std::vector<Position> connectedPoints;
    //         for (auto c : getCrossings()) {
    //             DEBUGCOUT(gDebugFlag1, "  crossing=" << c->id << " sortedEdges=" << toString(c->edges) << "\n")
    //             if (c->edges.back() == normalizedLanes[end].first
    //                     && (normalizedLanes[end].second.permissions & SVC_PEDESTRIAN) == 0) {
    //                 // crossing ends
    //                 if (c->nextWalkingArea != "") {
    //                     WRITE_WARNINGF(TL("Invalid pedestrian topology at junction '%'; crossing '%' targets '%' and '%'."),
    //                                    getID(), c->id, c->nextWalkingArea, wa.id);
    //                     c->valid = false;
    //                 }
    //                 c->nextWalkingArea = wa.id;
    //                 wa.prevCrossings.push_back(c->id);
    //                 if ((int)c->edges.size() < wa.minPrevCrossingEdges) {
    //                     // if there are multiple crossings, use the shape of the one that crosses fewer edges
    //                     endCrossingWidth = c->width;
    //                     endCrossingShape = c->shape;
    //                     wa.width = MAX2(wa.width, endCrossingWidth);
    //                     connectsCrossing = true;
    //                     connectedPoints.push_back(c->shape[-1]);
    //                     wa.minPrevCrossingEdges = (int)c->edges.size();
    //                     numCrossings++;
    //                     if (normalizedLanes[lastIdx].second.shape[0].distanceTo2D(connectedPoints.back()) < endCrossingWidth) {
    //                         crossingNearSidewalk = true;
    //                         DEBUGCOUT(gDebugFlag1, "    nearSidewalk\n")
    //                     }
    //                 }
    //                 DEBUGCOUT(gDebugFlag1, "    crossing " << c->id << " ends\n")
    //             }
    //             if (c->edges.front() == normalizedLanes[prev].first
    //                     && (normalizedLanes[prev].second.permissions & SVC_PEDESTRIAN) == 0) {
    //                 // crossing starts
    //                 if (c->prevWalkingArea != "") {
    //                     WRITE_WARNINGF(TL("Invalid pedestrian topology at junction '%'; crossing '%' is targeted by '%' and '%'."),
    //                                    getID(), c->id, c->prevWalkingArea, wa.id);
    //                     c->valid = false;
    //                 }
    //                 if (c->valid && std::find(wa.prevCrossings.begin(), wa.prevCrossings.end(), c->id) != wa.prevCrossings.end()) {
    //                     WRITE_WARNINGF(TL("Invalid pedestrian topology at junction '%'; crossing '%' starts and ends at walkingarea '%'."),
    //                                    getID(), c->id, wa.id);
    //                     c->valid = false;
    //                 }
    //                 c->prevWalkingArea = wa.id;
    //                 wa.nextCrossings.push_back(c->id);
    //                 if ((int)c->edges.size() < wa.minNextCrossingEdges) {
    //                     // if there are multiple crossings, use the shape of the one that crosses fewer edges
    //                     startCrossingWidth = c->width;
    //                     startCrossingShape = c->shape;
    //                     wa.width = MAX2(wa.width, startCrossingWidth);
    //                     connectsCrossing = true;
    //                     connectedPoints.push_back(c->shape[0]);
    //                     wa.minNextCrossingEdges = (int)c->edges.size();
    //                     numCrossings++;
    //                     if (normalizedLanes[startIdx].second.shape[0].distanceTo2D(connectedPoints.back()) < startCrossingWidth) {
    //                         crossingNearSidewalk = true;
    //                         DEBUGCOUT(gDebugFlag1, "    nearSidewalk\n")
    //                     }
    //                 }
    //                 DEBUGCOUT(gDebugFlag1, "    crossing " << c->id << " starts\n")
    //             }
    //             DEBUGCOUT(gDebugFlag1, "  check connections to crossing " << c->id
    //                       << " cFront=" << c->edges.front()->getID() << " cBack=" << c->edges.back()->getID()
    //                       << " wEnd=" << normalizedLanes[end].first->getID() << " wStart=" << normalizedLanes[startIdx].first->getID()
    //                       << " wStartPrev=" << normalizedLanes[prev].first->getID()
    //                       << "\n")
    //         }
    //         if (count < 2 && !connectsCrossing) {
    //             // not relevant for walking
    //             DEBUGCOUT(gDebugFlag1, "    not relevant for walking: count=" << count << " connectsCrossing=" << connectsCrossing << "\n")
    //             continue;
    //         }
    //         // build shape and connections
    //         std::set<const NBEdge*, ComparatorIdLess>& connected = wa.refEdges;
    //         for (int j = 0; j < count; ++j) {
    //             const int nlI = (startIdx + j) % normalizedLanes.size();
    //             NBEdge* edge = normalizedLanes[nlI].first;
    //             NBEdge::Lane l = normalizedLanes[nlI].second;
    //             wa.width = MAX2(wa.width, l.width);
    //             if (connected.count(edge) == 0) {
    //                 if (edge->getFromNode() == this) {
    //                     wa.nextSidewalks.push_back(edge->getSidewalkID());
    //                     connectedPoints.push_back(edge->getLaneShape(0)[0]);
    //                 } else {
    //                     wa.prevSidewalks.push_back(edge->getSidewalkID());
    //                     connectedPoints.push_back(edge->getLaneShape(0)[-1]);
    //                 }
    //                 DEBUGCOUT(gDebugFlag1, "    connectedEdge=" << edge->getID() << " connectedPoint=" << connectedPoints.back() << "\n")
    //                 connected.insert(edge);
    //             }
    //             l.shape.move2side(-l.width / 2);
    //             wa.shape.push_back_noDoublePos(l.shape[0]);
    //             l.shape.move2side(l.width);
    //             wa.shape.push_back(l.shape[0]);
    //         }
    //         if (buildExtensions) {
    //             // extension at starting crossing
    //             if (startCrossingShape.size() > 0) {
    //                 startCrossingShape.move2side(startCrossingWidth / 2);
    //                 wa.shape.push_front_noDoublePos(startCrossingShape[0]); // right corner
    //                 startCrossingShape.move2side(-startCrossingWidth);
    //                 wa.shape.push_front_noDoublePos(startCrossingShape[0]); // left corner goes first
    //                 DEBUGCOUT(gDebugFlag1, "  extension at startCrossingShape=" << endCrossingShape << " waShape=" << wa.shape << "\n")
    //             }
    //             // extension at ending crossing
    //             if (endCrossingShape.size() > 0) {
    //                 endCrossingShape.move2side(endCrossingWidth / 2);
    //                 wa.shape.push_back_noDoublePos(endCrossingShape[-1]);
    //                 endCrossingShape.move2side(-endCrossingWidth);
    //                 wa.shape.push_back_noDoublePos(endCrossingShape[-1]);
    //                 DEBUGCOUT(gDebugFlag1, "  extension at endCrossingShape=" << endCrossingShape << " waShape=" << wa.shape << "\n")
    //             }
    //         }
    //         if (connected.size() == 2 && !connectsCrossing && wa.nextSidewalks.size() == 1 && wa.prevSidewalks.size() == 1
    //                 && normalizedLanes.size() == 2) {
    //             // do not build a walkingArea since a normal connection exists
    //             const NBEdge* e1 = *connected.begin();
    //             const NBEdge* e2 = *(++connected.begin());
    //             if (e1->hasConnectionTo(e2, 0, 0) || e2->hasConnectionTo(e1, 0, 0)) {
    //                 DEBUGCOUT(gDebugFlag1, "    not building a walkingarea since normal connections exist\n")
    //                 continue;
    //             }
    //         }
    //         if (count == (int)normalizedLanes.size()) {
    //             // junction is covered by the whole walkingarea
    //             wa.shape = myPoly;
    //             // increase walking width if the walkingare is wider than a single lane
    //             for (const NBEdge* in : myIncomingEdges) {
    //                 for (const NBEdge* out : myOutgoingEdges) {
    //                     if (in->getFromNode() == out->getToNode() && in->getInnerGeometry().reverse() == out->getInnerGeometry()
    //                             && (in->getPermissions() & SVC_PEDESTRIAN)
    //                             && (out->getPermissions() & SVC_PEDESTRIAN)) {
    //                         // doesn't catch all cases but probably most
    //                         wa.width = MAX2(wa.width, in->getTotalWidth() + out->getTotalWidth());
    //                     }
    //                 }
    //             }
    //         } else if (cornerDetail > 0) {
    //             // build smooth inner curve (optional)
    //             int smoothEnd = end;
    //             int smoothPrev = prev;
    //             // extend to green verge
    //             if (endCrossingWidth > 0 && normalizedLanes[smoothEnd].second.permissions == 0) {
    //                 smoothEnd = (smoothEnd + 1) % normalizedLanes.size();
    //             }
    //             if (startCrossingWidth > 0 && normalizedLanes[smoothPrev].second.permissions == 0) {
    //                 if (smoothPrev == 0) {
    //                     smoothPrev = (int)normalizedLanes.size() - 1;
    //                 } else {
    //                     smoothPrev--;
    //                 }
    //             }
    //             PositionVector begShape = normalizedLanes[smoothEnd].second.shape;
    //             begShape = begShape.reverse();
    //             double shiftBegExtra = 0;
    //             double shiftEndExtra = 0;
    //             if (lastIdx == startIdx) {
    //                 lastIdx = (startIdx + 1) % normalizedLanes.size();
    //                 DEBUGCOUT(gDebugFlag1, "    new lastIdx=" << lastIdx << " startEdge=" << normalizedLanes[startIdx].first->getID() << " lastEdge=" << normalizedLanes[lastIdx].first->getID() << "\n")
    //                 if (normalizedLanes[startIdx].first == normalizedLanes[lastIdx].first) {
    //                     lastIdx = startIdx;
    //                     startIdx--;
    //                     if (startIdx < 0) {
    //                         startIdx = (int)normalizedLanes.size() - 1;
    //                     }
    //                     DEBUGCOUT(gDebugFlag1, "    new startIdx=" << startIdx << " startEdge=" << normalizedLanes[startIdx].first->getID() << " lastEdge=" << normalizedLanes[lastIdx].first->getID() << "\n")
    //                     shiftEndExtra += OptionsCont::getOptions().getFloat("default.sidewalk-width");
    //                 } else {
    //                     shiftBegExtra += OptionsCont::getOptions().getFloat("default.sidewalk-width");
    //                 }
    //             }
    //             PositionVector begShapeOuter = normalizedLanes[lastIdx].second.shape;
    //             begShapeOuter = begShapeOuter.reverse();
    //             //begShape.extrapolate(endCrossingWidth);
    //             begShape.move2side(normalizedLanes[smoothEnd].second.width / 2);
    //             begShapeOuter.move2side(normalizedLanes[lastIdx].second.width / 2 + shiftBegExtra);
    //             PositionVector endShape = normalizedLanes[smoothPrev].second.shape;
    //             PositionVector endShapeOuter = normalizedLanes[startIdx].second.shape;;
    //             endShape.move2side(normalizedLanes[smoothPrev].second.width / 2);
    //             endShapeOuter.move2side(normalizedLanes[startIdx].second.width / 2 + shiftEndExtra);
    //             //endShape.extrapolate(startCrossingWidth);
    //             PositionVector curve;
    //             if (count != (int)normalizedLanes.size() || count == 2) {
    //                 const double angle = GeomHelper::angleDiff(begShape.angleAt2D(-2), endShape.angleAt2D(0));
    //                 if (count == 1 && angle > 0 && crossingNearSidewalk && numCrossings < 2) {
    //                     // do not build smooth shape for an unconnected left turn
    //                     // (the walkingArea would get bigger without a reason to
    //                     // walk there)
    //                 } else if ((normalizedLanes[smoothEnd].first->getPermissions() & normalizedLanes[smoothPrev].first->getPermissions() &
    //                             ~(SVC_PEDESTRIAN | SVC_RAIL_CLASSES)) != 0) {
    //                     DEBUGCOUT(gDebugFlag1, "   traffic curve\n")
    //                     curve = computeSmoothShape(begShape, endShape, cornerDetail + 2, false, 25, 25, gDebugFlag1 ? this : nullptr);
    //                     if (curve.length2D() - begShape.back().distanceTo2D(endShape.front()) > 5) {
    //                         DEBUGCOUT(gDebugFlag1, "   reduceBulge directLength=" << begShape.back().distanceTo2D(endShape.front())
    //                                   << " curveLength=" << curve.length2D()
    //                                   << " delta=" << curve.length2D() - begShape.back().distanceTo2D(endShape.front())
    //                                   << "\n")
    //                         curve = computeSmoothShape(begShape, endShape, cornerDetail + 2, false, 25, 25, nullptr, AVOID_WIDE_LEFT_TURN | AVOID_INTERSECTING_LEFT_TURNS);
    //                     }
    //                 } else {
    //                     DEBUGCOUT(gDebugFlag1, "   nonTraffic curve\n")
    //                     const double extend = MIN2(10.0, begShape.back().distanceTo2D(endShape.front()) / 2);
    //                     curve = computeSmoothShape(begShape, endShape, cornerDetail + 2, false, extend, extend, nullptr, FOUR_CONTROL_POINTS);
    //                 }
    //                 if (curve.size() > 2) {
    //                     curve.erase(curve.begin());
    //                     curve.pop_back();
    //                     if (endCrossingWidth > 0) {
    //                         wa.shape.pop_back();
    //                     }
    //                     if (startCrossingWidth > 0) {
    //                         wa.shape.erase(wa.shape.begin());
    //                     }
    //                     if (count == (int)normalizedLanes.size()) {
    //                         curve = curve.reverse();
    //                     }
    //                     wa.shape.append(curve, 0);
    //                 }
    //                 DEBUGCOUT(gDebugFlag1, " end=" << smoothEnd << " prev=" << smoothPrev
    //                           << " endCrossingWidth=" << endCrossingWidth << " startCrossingWidth=" << startCrossingWidth
    //                           << "  begShape=" << begShape << " endShape=" << endShape << " smooth curve=" << curve
    //                           << "  begShapeOuter=" << begShapeOuter << " endShapeOuter=" << endShapeOuter
    //                           << "  waShape=" << wa.shape
    //                           << "\n")
    //             }
    //             if (curve.size() > 2 && (count == 2 || (count == 1 && numCrossings > 0))) {
    //                 const double innerDist = begShape.back().distanceTo2D(endShape[0]);
    //                 const double outerDist = begShapeOuter.back().distanceTo2D(endShapeOuter[0]);
    //                 DEBUGCOUT(gDebugFlag1, " innerDist=" << innerDist << " outerDist=" << outerDist << "\n")
    //                 if (outerDist > innerDist) {
    //                     // we also need a rounded outer curve (unless we have only a single walkingarea)
    //                     const double extend = MIN2(10.0, begShapeOuter.back().distanceTo2D(endShapeOuter.front()) / 2);
    //                     curve = computeSmoothShape(begShapeOuter, endShapeOuter, cornerDetail + 2, false, extend, extend, nullptr);
    //                     if (curve.length2D() - begShapeOuter.back().distanceTo2D(endShapeOuter.front()) > 5) {
    //                         DEBUGCOUT(gDebugFlag1, "   reduceBulge directLength=" << begShapeOuter.back().distanceTo2D(endShapeOuter.front())
    //                                   << " curveLength=" << curve.length2D()
    //                                   << " delta=" << curve.length2D() - begShapeOuter.back().distanceTo2D(endShapeOuter.front())
    //                                   << "\n")
    //                         curve = computeSmoothShape(begShapeOuter, endShapeOuter, cornerDetail + 2, false, 25, 25, nullptr, AVOID_WIDE_LEFT_TURN | AVOID_INTERSECTING_LEFT_TURNS);
    //                     }
    //                     curve = curve.reverse();
    //                     // keep the points in case of extraShift
    //                     if (shiftBegExtra != 0) {
    //                         curve.push_front_noDoublePos(wa.shape[1]);
    //                         curve.push_back_noDoublePos(wa.shape[2]);
    //                     } else if (shiftEndExtra != 0) {
    //                         curve.push_back_noDoublePos(wa.shape[1]);
    //                         curve.push_back_noDoublePos(wa.shape[2]);
    //                     }
    //                     DEBUGCOUT(gDebugFlag1, " outerCurveRaw=" << curve << " wa1=" << wa.shape[1] << " wa2=" << wa.shape[2] << "\n")
    //                     wa.shape.erase(wa.shape.begin() + 1, wa.shape.begin() + 3);
    //                     wa.shape.insert(wa.shape.begin() + 1, curve.begin(), curve.end());
    //                     DEBUGCOUT(gDebugFlag1, " outerCurve=" << curve << "\n")
    //                 }
    //             }
    //         }
    //         // apply custom shapes
    //         if (myWalkingAreaCustomShapes.size() > 0) {
    //             for (auto wacs : myWalkingAreaCustomShapes) {
    //                 // every edge in wasc.edges must be part of connected
    //                 if ((wacs.shape.size() != 0 || wacs.width != NBEdge::UNSPECIFIED_WIDTH) && includes(connected, wacs.edges)) {
    //                     if (wacs.shape.size() != 0) {
    //                         wa.shape = wacs.shape;
    //                     }
    //                     if (wacs.width != NBEdge::UNSPECIFIED_WIDTH) {
    //                         wa.width = wacs.width;
    //                     }
    //                     wa.hasCustomShape = true;
    //                 }
    //             }
    //         }
    //         // determine length (average of all possible connections)
    //         double lengthSum = 0;
    //         int combinations = 0;
    //         for (std::vector<Position>::const_iterator it1 = connectedPoints.begin(); it1 != connectedPoints.end(); ++it1) {
    //             for (std::vector<Position>::const_iterator it2 = connectedPoints.begin(); it2 != connectedPoints.end(); ++it2) {
    //                 const Position& p1 = *it1;
    //                 const Position& p2 = *it2;
    //                 if (p1 != p2) {
    //                     lengthSum += p1.distanceTo2D(p2);
    //                     combinations += 1;
    //                 }
    //             }
    //         }
    //         DEBUGCOUT(gDebugFlag1, "  combinations=" << combinations << " connectedPoints=" << connectedPoints << "\n")
    //         wa.length = POSITION_EPS;
    //         if (combinations > 0) {
    //             wa.length = MAX2(POSITION_EPS, lengthSum / combinations);
    //         }
    //         myWalkingAreas.push_back(wa);
    //     }
    //     // build walkingAreas between split crossings
    //     std::vector<Crossing*> validCrossings = getCrossings();
    //     for (std::vector<Crossing*>::iterator it = validCrossings.begin(); it != validCrossings.end(); ++it) {
    //         Crossing& prev = **it;
    //         Crossing& next = (it !=  validCrossings.begin() ? **(it - 1) :** (validCrossings.end() - 1));
    //         DEBUGCOUT(gDebugFlag1, "  checkIntermediate: prev=" << prev.id << " next=" << next.id << " prev.nextWA=" << prev.nextWalkingArea << " next.prevWA=" << next.prevWalkingArea << "\n")
    //         if (prev.nextWalkingArea == "") {
    //             if (next.prevWalkingArea != "" || &prev == &next) {
    //                 WRITE_WARNINGF(TL("Invalid pedestrian topology: crossing '%' across [%] has no target."), prev.id, toString(prev.edges));
    //                 prev.valid = false;
    //                 continue;
    //             }
    //             WalkingArea wa(":" + getID() + "_w" + toString(index++), prev.width);
    //             prev.nextWalkingArea = wa.id;
    //             wa.nextCrossings.push_back(next.id);
    //             next.prevWalkingArea = wa.id;
    //             // back of previous crossing
    //             PositionVector tmp = prev.shape;
    //             tmp.move2side(-prev.width / 2);
    //             wa.shape.push_back(tmp[-1]);
    //             tmp.move2side(prev.width);
    //             wa.shape.push_back(tmp[-1]);
    //             // front of next crossing
    //             tmp = next.shape;
    //             tmp.move2side(prev.width / 2);
    //             wa.shape.push_back(tmp[0]);
    //             tmp.move2side(-prev.width);
    //             wa.shape.push_back(tmp[0]);
    //             wa.refEdges.insert(prev.edges.begin(), prev.edges.end());
    //             wa.refEdges.insert(next.edges.begin(), next.edges.end());
    //             // apply custom shapes
    //             if (myWalkingAreaCustomShapes.size() > 0) {
    //                 for (auto wacs : myWalkingAreaCustomShapes) {
    //                     // every edge in wacs.edges must be part of crossed
    //                     if (wacs.shape.size() != 0 && wacs.edges.size() > 1 && includes(wa.refEdges, wacs.edges)) {
    //                         wa.shape = wacs.shape;
    //                         wa.hasCustomShape = true;
    //                     }
    //                 }
    //             }
    //             // length (special case)
    //             wa.length = MAX2(POSITION_EPS, prev.shape.back().distanceTo2D(next.shape.front()));
    //             myWalkingAreas.push_back(wa);
    //             DEBUGCOUT(gDebugFlag1, "     build wa=" << wa.id << "\n")
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_walking_areas(&mut self, cornerDetail: i32, joinMinDist: f64) {
        use utils::common::sumo_vehicle_class::SVC_RAIL_CLASSES;
        let me = Ptr::from_mut(self);
        let mut index: i32 = 0;
        self.myWalkingAreas.clear();
        if self.myAllEdges.is_empty() {
            return;
        }
        let allEdges = self.get_edges_sorted_by_angle_at_node_center();
        // shapes are all pointing away from the intersection
        let mut normalizedLanes: Vec<NBNodeWaLane> = Vec::new();
        for &edge in allEdges.iter() {
            let lanes = edge.get_lanes();
            let mut tmp: Vec<NBNodeWaLane> = Vec::new();
            let mut hadSidewalk = false;
            let mut hadNonSidewalk = false;
            for i in 0..lanes.len() {
                let l = &lanes[i];
                let sidewalk = (l.permissions & SVC_PEDESTRIAN) != 0;
                if sidewalk {
                    if hadSidewalk && hadNonSidewalk {
                        if edge.get_from_node() == me {
                            write_warning(&format!("Ignoring additional sidewalk lane {} on edge '{}' for walkingareas.", i, edge.get_id()));
                        }
                        continue;
                    }
                    hadSidewalk = true;
                } else {
                    hadNonSidewalk = true;
                }
                tmp.push(NBNodeWaLane { edge, permissions: l.permissions, shape: l.shape.clone(), width: l.width });
            }
            if edge.get_from_node() == me {
                tmp.reverse();
            } else {
                for l in tmp.iter_mut() {
                    l.shape = l.shape.reverse();
                }
            }
            for mut l in tmp {
                l.shape = l.shape.get_subpart_by_index(0, 2);
                l.width = if l.width == NBEDGE_UNSPECIFIED_WIDTH { NBNODE_SUMO_CONST_LANE_WIDTH } else { l.width };
                normalizedLanes.push(l);
            }
        }
        if normalizedLanes.is_empty() {
            // shortcut-free: C++ reads normalizedLanes.back() unconditionally (UB when empty); nothing can be built anyway
            return;
        }
        let nlSize = normalizedLanes.len() as i32;
        // collect [start,count[ indices in normalizedLanes that belong to a walkingArea
        let mut waIndices: Vec<(i32, i32)> = Vec::new();
        let mut start: i32 = -1;
        let mut prevEdge = normalizedLanes[normalizedLanes.len() - 1].edge;
        for i in 0..nlSize {
            let edge = normalizedLanes[i as usize].edge;
            let perm = normalizedLanes[i as usize].permissions;
            if start == -1 {
                if (perm & SVC_PEDESTRIAN) != 0 {
                    start = i;
                }
            } else if (perm & SVC_PEDESTRIAN) == 0
                || self.crossing_between(edge, prevEdge)
                || self.already_connected_paths(edge, prevEdge, joinMinDist)
                || self.crosses_fringe(edge, prevEdge)
            {
                waIndices.push((start, i - start));
                if (perm & SVC_PEDESTRIAN) != 0 {
                    start = i;
                } else {
                    start = -1;
                }
            }
            prevEdge = edge;
        }
        // deal with wrap-around issues
        if start != -1 {
            let waNumLanes = nlSize - start;
            if waIndices.is_empty() {
                waIndices.push((start, waNumLanes));
            } else if waIndices[0].0 == 0 {
                let edge = normalizedLanes[0].edge;
                let lastEdge = normalizedLanes[normalizedLanes.len() - 1].edge;
                if self.crossing_between(edge, lastEdge) || self.crosses_fringe(edge, lastEdge) {
                    // do not wrap-around (see above)
                    waIndices.push((start, waNumLanes));
                } else {
                    // first walkingArea wraps around
                    waIndices[0].0 = start;
                    waIndices[0].1 = waNumLanes + waIndices[0].1;
                }
            } else {
                // last walkingArea ends at the wrap-around
                waIndices.push((start, waNumLanes));
            }
        }
        // build walking areas connected to a sidewalk
        for wi in 0..waIndices.len() {
            let buildExtensions = waIndices[wi].1 != nlSize;
            let mut startIdx = waIndices[wi].0;
            let prev = if startIdx > 0 { startIdx - 1 } else { nlSize - 1 };
            let count = waIndices[wi].1;
            let end = (startIdx + count) % nlSize;
            let mut lastIdx = (startIdx + count - 1) % nlSize;

            let mut wa = NBNode_WalkingArea::new(&format!(":{}_w{}", self.get_id(), index), 1.);
            index += 1;
            let mut endCrossingWidth: f64 = 0.;
            let mut startCrossingWidth: f64 = 0.;
            let mut endCrossingShape = PositionVector::default();
            let mut startCrossingShape = PositionVector::default();
            // check for connected crossings
            let mut connectsCrossing = false;
            let mut crossingNearSidewalk = false;
            let mut numCrossings: i32 = 0;
            let mut connectedPoints: Vec<Position> = Vec::new();
            for cp in self.get_crossings() {
                let c = cp.m();
                if c.edges.last().copied() == Some(normalizedLanes[end as usize].edge)
                    && (normalizedLanes[end as usize].permissions & SVC_PEDESTRIAN) == 0
                {
                    // crossing ends
                    if c.nextWalkingArea != "" {
                        write_warning(&format!(
                            "Invalid pedestrian topology at junction '{}'; crossing '{}' targets '{}' and '{}'.",
                            self.get_id(), c.id, c.nextWalkingArea, wa.id
                        ));
                        c.valid = false;
                    }
                    c.nextWalkingArea = wa.id.clone();
                    wa.prevCrossings.push(c.id.clone());
                    if (c.edges.len() as i32) < wa.minPrevCrossingEdges {
                        // if there are multiple crossings, use the shape of the one that crosses fewer edges
                        endCrossingWidth = c.width;
                        endCrossingShape = c.shape.clone();
                        wa.width = max2(wa.width, endCrossingWidth);
                        connectsCrossing = true;
                        connectedPoints.push(*c.shape.back());
                        wa.minPrevCrossingEdges = c.edges.len() as i32;
                        numCrossings += 1;
                        if normalizedLanes[lastIdx as usize].shape.at(0).distance_to2d(connectedPoints.last().unwrap()) < endCrossingWidth {
                            crossingNearSidewalk = true;
                        }
                    }
                }
                if c.edges.first().copied() == Some(normalizedLanes[prev as usize].edge)
                    && (normalizedLanes[prev as usize].permissions & SVC_PEDESTRIAN) == 0
                {
                    // crossing starts
                    if c.prevWalkingArea != "" {
                        write_warning(&format!(
                            "Invalid pedestrian topology at junction '{}'; crossing '{}' is targeted by '{}' and '{}'.",
                            self.get_id(), c.id, c.prevWalkingArea, wa.id
                        ));
                        c.valid = false;
                    }
                    if c.valid && wa.prevCrossings.contains(&c.id) {
                        write_warning(&format!(
                            "Invalid pedestrian topology at junction '{}'; crossing '{}' starts and ends at walkingarea '{}'.",
                            self.get_id(), c.id, wa.id
                        ));
                        c.valid = false;
                    }
                    c.prevWalkingArea = wa.id.clone();
                    wa.nextCrossings.push(c.id.clone());
                    if (c.edges.len() as i32) < wa.minNextCrossingEdges {
                        // if there are multiple crossings, use the shape of the one that crosses fewer edges
                        startCrossingWidth = c.width;
                        startCrossingShape = c.shape.clone();
                        wa.width = max2(wa.width, startCrossingWidth);
                        connectsCrossing = true;
                        connectedPoints.push(*c.shape.at(0));
                        wa.minNextCrossingEdges = c.edges.len() as i32;
                        numCrossings += 1;
                        if normalizedLanes[startIdx as usize].shape.at(0).distance_to2d(connectedPoints.last().unwrap()) < startCrossingWidth {
                            crossingNearSidewalk = true;
                        }
                    }
                }
            }
            if count < 2 && !connectsCrossing {
                // not relevant for walking
                continue;
            }
            // build shape and connections
            // determinism: C++ std::set<const NBEdge*, ComparatorIdLess> orders by edge ID; BTreeSet<Ptr<NBEdge>> orders by address
            for j in 0..count {
                let nlI = ((startIdx + j) % nlSize) as usize;
                let edge = normalizedLanes[nlI].edge;
                let mut l = normalizedLanes[nlI].clone();
                wa.width = max2(wa.width, l.width);
                if !wa.refEdges.contains(&edge) {
                    if edge.get_from_node() == me {
                        wa.nextSidewalks.push(edge.m().get_sidewalk_id());
                        connectedPoints.push(*edge.get_lane_shape(0).at(0));
                    } else {
                        wa.prevSidewalks.push(edge.m().get_sidewalk_id());
                        connectedPoints.push(*edge.get_lane_shape(0).at(-1));
                    }
                    wa.refEdges.insert(edge);
                }
                l.shape.move2side(-l.width / 2., 100.);
                wa.shape.push_back_no_double_pos(l.shape.at(0));
                l.shape.move2side(l.width, 100.);
                wa.shape.push(*l.shape.at(0));
            }
            if buildExtensions {
                // extension at starting crossing
                if startCrossingShape.len() > 0 {
                    startCrossingShape.move2side(startCrossingWidth / 2., 100.);
                    wa.shape.push_front_no_double_pos(startCrossingShape.at(0)); // right corner
                    startCrossingShape.move2side(-startCrossingWidth, 100.);
                    wa.shape.push_front_no_double_pos(startCrossingShape.at(0)); // left corner goes first
                }
                // extension at ending crossing
                if endCrossingShape.len() > 0 {
                    endCrossingShape.move2side(endCrossingWidth / 2., 100.);
                    wa.shape.push_back_no_double_pos(endCrossingShape.at(-1));
                    endCrossingShape.move2side(-endCrossingWidth, 100.);
                    wa.shape.push_back_no_double_pos(endCrossingShape.at(-1));
                }
            }
            if wa.refEdges.len() == 2 && !connectsCrossing && wa.nextSidewalks.len() == 1 && wa.prevSidewalks.len() == 1 && nlSize == 2 {
                // do not build a walkingArea since a normal connection exists
                let mut it = wa.refEdges.iter();
                let e1 = *it.next().unwrap();
                let e2 = *it.next().unwrap();
                if e1.has_connection_to(e2, 0, 0) || e2.has_connection_to(e1, 0, 0) {
                    continue;
                }
            }
            if count == nlSize {
                // junction is covered by the whole walkingarea
                wa.shape = self.myPoly.clone();
                // increase walking width if the walkingare is wider than a single lane
                for inE in self.myIncomingEdges.iter() {
                    for outE in self.myOutgoingEdges.iter() {
                        if inE.get_from_node() == outE.get_to_node()
                            && inE.get_inner_geometry().reverse() == outE.get_inner_geometry()
                            && (inE.get_permissions(-1) & SVC_PEDESTRIAN) != 0
                            && (outE.get_permissions(-1) & SVC_PEDESTRIAN) != 0
                        {
                            // doesn't catch all cases but probably most
                            wa.width = max2(wa.width, inE.get_total_width() + outE.get_total_width());
                        }
                    }
                }
            } else if cornerDetail > 0 {
                // build smooth inner curve (optional)
                let mut smoothEnd = end;
                let mut smoothPrev = prev;
                // extend to green verge
                if endCrossingWidth > 0. && normalizedLanes[smoothEnd as usize].permissions == 0 {
                    smoothEnd = (smoothEnd + 1) % nlSize;
                }
                if startCrossingWidth > 0. && normalizedLanes[smoothPrev as usize].permissions == 0 {
                    if smoothPrev == 0 {
                        smoothPrev = nlSize - 1;
                    } else {
                        smoothPrev -= 1;
                    }
                }
                let mut begShape = normalizedLanes[smoothEnd as usize].shape.clone();
                begShape = begShape.reverse();
                let mut shiftBegExtra: f64 = 0.;
                let mut shiftEndExtra: f64 = 0.;
                if lastIdx == startIdx {
                    lastIdx = (startIdx + 1) % nlSize;
                    if normalizedLanes[startIdx as usize].edge == normalizedLanes[lastIdx as usize].edge {
                        lastIdx = startIdx;
                        startIdx -= 1;
                        if startIdx < 0 {
                            startIdx = nlSize - 1;
                        }
                        shiftEndExtra += utils::options::options_cont::get_float("default.sidewalk-width");
                    } else {
                        shiftBegExtra += utils::options::options_cont::get_float("default.sidewalk-width");
                    }
                }
                let mut begShapeOuter = normalizedLanes[lastIdx as usize].shape.clone();
                begShapeOuter = begShapeOuter.reverse();
                begShape.move2side(normalizedLanes[smoothEnd as usize].width / 2., 100.);
                begShapeOuter.move2side(normalizedLanes[lastIdx as usize].width / 2. + shiftBegExtra, 100.);
                let mut endShape = normalizedLanes[smoothPrev as usize].shape.clone();
                let mut endShapeOuter = normalizedLanes[startIdx as usize].shape.clone();
                endShape.move2side(normalizedLanes[smoothPrev as usize].width / 2., 100.);
                endShapeOuter.move2side(normalizedLanes[startIdx as usize].width / 2. + shiftEndExtra, 100.);
                let mut curve = PositionVector::default();
                if count != nlSize || count == 2 {
                    let angle = angle_diff(begShape.angle_at2d(-2), endShape.angle_at2d(0));
                    if count == 1 && angle > 0. && crossingNearSidewalk && numCrossings < 2 {
                        // do not build smooth shape for an unconnected left turn
                        // (the walkingArea would get bigger without a reason to
                        // walk there)
                    } else if (normalizedLanes[smoothEnd as usize].edge.get_permissions(-1)
                        & normalizedLanes[smoothPrev as usize].edge.get_permissions(-1)
                        & !(SVC_PEDESTRIAN | SVC_RAIL_CLASSES))
                        != 0
                    {
                        // traffic curve (debug recordError is nullptr since gDebugFlag1 is off)
                        curve = self.compute_smooth_shape(&begShape, &endShape, cornerDetail + 2, false, 25., 25., Ptr::null(), 0);
                        if curve.length2d() - begShape.back().distance_to2d(endShape.front()) > 5. {
                            curve = self.compute_smooth_shape(
                                &begShape,
                                &endShape,
                                cornerDetail + 2,
                                false,
                                25.,
                                25.,
                                Ptr::null(),
                                NBNODE_AVOID_WIDE_LEFT_TURN | NBNODE_AVOID_INTERSECTING_LEFT_TURNS,
                            );
                        }
                    } else {
                        // nonTraffic curve
                        let extend = min2(10.0, begShape.back().distance_to2d(endShape.front()) / 2.);
                        curve = self.compute_smooth_shape(&begShape, &endShape, cornerDetail + 2, false, extend, extend, Ptr::null(), NBNODE_FOUR_CONTROL_POINTS);
                    }
                    if curve.len() > 2 {
                        curve.remove(0);
                        curve.pop();
                        if endCrossingWidth > 0. {
                            wa.shape.pop();
                        }
                        if startCrossingWidth > 0. {
                            wa.shape.remove(0);
                        }
                        if count == nlSize {
                            curve = curve.reverse();
                        }
                        wa.shape.append(&curve, 0.);
                    }
                }
                if curve.len() > 2 && (count == 2 || (count == 1 && numCrossings > 0)) {
                    let innerDist = begShape.back().distance_to2d(endShape.at(0));
                    let outerDist = begShapeOuter.back().distance_to2d(endShapeOuter.at(0));
                    if outerDist > innerDist {
                        // we also need a rounded outer curve (unless we have only a single walkingarea)
                        let extend = min2(10.0, begShapeOuter.back().distance_to2d(endShapeOuter.front()) / 2.);
                        curve = self.compute_smooth_shape(&begShapeOuter, &endShapeOuter, cornerDetail + 2, false, extend, extend, Ptr::null(), 0);
                        if curve.length2d() - begShapeOuter.back().distance_to2d(endShapeOuter.front()) > 5. {
                            curve = self.compute_smooth_shape(
                                &begShapeOuter,
                                &endShapeOuter,
                                cornerDetail + 2,
                                false,
                                25.,
                                25.,
                                Ptr::null(),
                                NBNODE_AVOID_WIDE_LEFT_TURN | NBNODE_AVOID_INTERSECTING_LEFT_TURNS,
                            );
                        }
                        curve = curve.reverse();
                        // keep the points in case of extraShift
                        if shiftBegExtra != 0. {
                            curve.push_front_no_double_pos(wa.shape.at(1));
                            curve.push_back_no_double_pos(wa.shape.at(2));
                        } else if shiftEndExtra != 0. {
                            curve.push_back_no_double_pos(wa.shape.at(1));
                            curve.push_back_no_double_pos(wa.shape.at(2));
                        }
                        wa.shape.drain(1..3);
                        wa.shape.splice(1..1, curve.iter().cloned());
                    }
                }
            }
            // apply custom shapes
            for wacs in self.myWalkingAreaCustomShapes.iter() {
                // every edge in wasc.edges must be part of connected
                if (wacs.shape.len() != 0 || wacs.width != NBEDGE_UNSPECIFIED_WIDTH) && NBNode::includes(&wa.refEdges, &wacs.edges) {
                    if wacs.shape.len() != 0 {
                        wa.shape = wacs.shape.clone();
                    }
                    if wacs.width != NBEDGE_UNSPECIFIED_WIDTH {
                        wa.width = wacs.width;
                    }
                    wa.hasCustomShape = true;
                }
            }
            // determine length (average of all possible connections)
            let mut lengthSum: f64 = 0.;
            let mut combinations: i32 = 0;
            for p1 in connectedPoints.iter() {
                for p2 in connectedPoints.iter() {
                    if p1 != p2 {
                        lengthSum += p1.distance_to2d(p2);
                        combinations += 1;
                    }
                }
            }
            wa.length = POSITION_EPS;
            if combinations > 0 {
                wa.length = max2(POSITION_EPS, lengthSum / combinations as f64);
            }
            self.myWalkingAreas.push(wa);
        }
        // build walkingAreas between split crossings
        let validCrossings = self.get_crossings();
        for it in 0..validCrossings.len() {
            let prevC = validCrossings[it].m();
            let nextC = if it != 0 { validCrossings[it - 1].m() } else { validCrossings[validCrossings.len() - 1].m() };
            if prevC.nextWalkingArea == "" {
                if nextC.prevWalkingArea != "" || validCrossings[it] == validCrossings[if it != 0 { it - 1 } else { validCrossings.len() - 1 }] {
                    let prevEdgeIds = prevC.edges.iter().map(|e| e.get_id().to_string()).collect::<Vec<_>>().join(" ");
                    write_warning(&format!("Invalid pedestrian topology: crossing '{}' across [{}] has no target.", prevC.id, prevEdgeIds));
                    prevC.valid = false;
                    continue;
                }
                let mut wa = NBNode_WalkingArea::new(&format!(":{}_w{}", self.get_id(), index), prevC.width);
                index += 1;
                prevC.nextWalkingArea = wa.id.clone();
                wa.nextCrossings.push(nextC.id.clone());
                nextC.prevWalkingArea = wa.id.clone();
                // back of previous crossing
                let mut tmp = prevC.shape.clone();
                tmp.move2side(-prevC.width / 2., 100.);
                wa.shape.push(*tmp.at(-1));
                tmp.move2side(prevC.width, 100.);
                wa.shape.push(*tmp.at(-1));
                // front of next crossing
                tmp = nextC.shape.clone();
                tmp.move2side(prevC.width / 2., 100.);
                wa.shape.push(*tmp.at(0));
                tmp.move2side(-prevC.width, 100.);
                wa.shape.push(*tmp.at(0));
                wa.refEdges.extend(prevC.edges.iter().cloned());
                wa.refEdges.extend(nextC.edges.iter().cloned());
                // apply custom shapes
                for wacs in self.myWalkingAreaCustomShapes.iter() {
                    // every edge in wacs.edges must be part of crossed
                    if wacs.shape.len() != 0 && wacs.edges.len() > 1 && NBNode::includes(&wa.refEdges, &wacs.edges) {
                        wa.shape = wacs.shape.clone();
                        wa.hasCustomShape = true;
                    }
                }
                // length (special case)
                wa.length = max2(POSITION_EPS, prevC.shape.back().distance_to2d(nextC.shape.front()));
                self.myWalkingAreas.push(wa);
            }
        }
    }

    /// C++: `void NBNode::buildCrossingOutlines()`
    /// build crossing outlines after walkingareas are finished
    // void
    // NBNode::buildCrossingOutlines() {
    // #ifdef DEBUG_CROSSING_OUTLINE
    //     if (myCrossings.size() > 0) {
    //         std::cerr << "<add>\n";
    //     }
    // #endif
    //     std::map<std::string, PositionVector> waShapes;
    //     for (auto wa : myWalkingAreas) {
    //         waShapes[wa.id] = wa.shape;
    //     }
    //     for (auto c : getCrossings()) {
    //         PositionVector wa1 = waShapes[c->prevWalkingArea];
    //         PositionVector wa2 = waShapes[c->nextWalkingArea];
    //         if (wa1.empty() || wa2.empty()) {
    //             continue;
    //         }
    //         wa1.closePolygon();
    //         wa2.closePolygon();
    //         PositionVector side1 = c->shape;
    //         PositionVector side2 = c->shape.reverse();
    //         side1.move2side(c->width / 2);
    //         side2.move2side(c->width / 2);
    //         PositionVector side1default = side1;
    //         PositionVector side2default = side2;
    //         side1.extrapolate(POSITION_EPS);
    //         side2.extrapolate(c->width);
    //         side1 = cutAtShapes(side1, wa1, wa2, side1default);
    //         side2 = cutAtShapes(side2, wa1, wa2, side2default);
    //         PositionVector side1ex = side1;
    //         PositionVector side2ex = side2;
    //         side1ex.extrapolate(POSITION_EPS);
    //         side2ex.extrapolate(side2 == side2default ? c->width / 2 : POSITION_EPS);
    //         PositionVector side3 = cutAtShapes(wa2, side1ex, side2ex, PositionVector());
    //         PositionVector side4 = cutAtShapes(wa1, side1ex, side2ex, PositionVector());
    //         c->outlineShape = side1;
    //         c->outlineShape.append(side3, POSITION_EPS);
    //         c->outlineShape.append(side2, POSITION_EPS);
    //         c->outlineShape.append(side4, POSITION_EPS);
    //         c->outlineShape.removeDoublePoints();
    //         if (c->outlineShape.back().almostSame(c->outlineShape.front())) {
    //             c->outlineShape.pop_back();
    //         }
    //         // DEBUG
    // #ifdef DEBUG_CROSSING_OUTLINE
    //         std::cout << "  side1=" << side1 << "\n  side2=" << side2 << "\n  side3=" << side3 << "\n  side4=" << side4 << "\n";
    //         std::cerr << "<poly id=\"" << c->id << "\" shape=\"" << c->outlineShape << "\" color=\"blue\" lineWidth=\"0.2\" layer=\"100\"/>\n";
    // #endif
    //     }
    // #ifdef DEBUG_CROSSING_OUTLINE
    //     if (myCrossings.size() > 0) {
    //         std::cerr << "</add>\n";
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn build_crossing_outlines(&mut self) {
        let mut waShapes: BTreeMap<String, PositionVector> = BTreeMap::new();
        for wa in self.myWalkingAreas.iter() {
            waShapes.insert(wa.id.clone(), wa.shape.clone());
        }
        for c in self.get_crossings() {
            let c = c.m();
            let mut wa1 = waShapes.get(&c.prevWalkingArea).cloned().unwrap_or_default();
            let mut wa2 = waShapes.get(&c.nextWalkingArea).cloned().unwrap_or_default();
            if wa1.is_empty() || wa2.is_empty() {
                continue;
            }
            wa1.close_polygon();
            wa2.close_polygon();
            let mut side1 = c.shape.clone();
            let mut side2 = c.shape.reverse();
            side1.move2side(c.width / 2., 100.);
            side2.move2side(c.width / 2., 100.);
            let side1default = side1.clone();
            let side2default = side2.clone();
            side1.extrapolate(POSITION_EPS, false, false);
            side2.extrapolate(c.width, false, false);
            side1 = self.cut_at_shapes(&side1, &wa1, &wa2, &side1default);
            side2 = self.cut_at_shapes(&side2, &wa1, &wa2, &side2default);
            let mut side1ex = side1.clone();
            let mut side2ex = side2.clone();
            side1ex.extrapolate(POSITION_EPS, false, false);
            side2ex.extrapolate(if side2 == side2default { c.width / 2. } else { POSITION_EPS }, false, false);
            let side3 = self.cut_at_shapes(&wa2, &side1ex, &side2ex, &PositionVector::default());
            let side4 = self.cut_at_shapes(&wa1, &side1ex, &side2ex, &PositionVector::default());
            c.outlineShape = side1;
            c.outlineShape.append(&side3, POSITION_EPS);
            c.outlineShape.append(&side2, POSITION_EPS);
            c.outlineShape.append(&side4, POSITION_EPS);
            c.outlineShape.remove_double_points(POSITION_EPS, false, 0, 0, false);
            if c.outlineShape.back().almost_same(c.outlineShape.front(), POSITION_EPS) {
                c.outlineShape.pop();
            }
        }
    }

    /// C++: `void NBNode::buildCrossingsAndWalkingAreas()`
    /// build crossings, and walkingareas. Also removes invalid loaded crossings if wished
    // void
    // NBNode::buildCrossingsAndWalkingAreas() {
    //     buildCrossings();
    //     buildWalkingAreas(OptionsCont::getOptions().getInt("junctions.corner-detail"),
    //                       OptionsCont::getOptions().getFloat("walkingareas.join-dist"));
    //     buildCrossingOutlines();
    //     // ensure that all crossings are properly connected
    //     bool recheck = myCrossings.size() > 0;
    //     while (recheck) {
    //         recheck = false;
    //         std::set<std::string> waIDs;
    //         int numSidewalks = 0;
    //         for (WalkingArea& wa : myWalkingAreas) {
    //             waIDs.insert(wa.id);
    //             numSidewalks += (int)(wa.prevSidewalks.size() + wa.nextSidewalks.size());
    //         }
    //         if (numSidewalks < 2) {
    //             // all crossings are invalid if there are fewer than 2 sidewalks involved
    //             waIDs.clear();
    //         }
    //         for (auto& crossing : myCrossings) {
    //             if (waIDs.count(crossing->prevWalkingArea) == 0 || waIDs.count(crossing->nextWalkingArea) == 0 || !crossing->valid) {
    //                 if (crossing->valid) {
    //                     WRITE_WARNINGF(TL("Discarding invalid crossing '%' at junction '%' with edges [%] (no walkingarea found)."),
    //                                    crossing->id, getID(), toString(crossing->edges));
    //                     recheck = true;
    //                 }
    //                 for (auto waIt = myWalkingAreas.begin(); waIt != myWalkingAreas.end();) {
    //                     WalkingArea& wa = *waIt;
    //                     std::vector<std::string>::iterator it_nc = std::find(wa.nextCrossings.begin(), wa.nextCrossings.end(), crossing->id);
    //                     if (it_nc != wa.nextCrossings.end()) {
    //                         wa.nextCrossings.erase(it_nc);
    //                     }
    //                     if (wa.prevSidewalks.size() + wa.nextSidewalks.size() + wa.nextCrossings.size() + wa.prevCrossings.size() < 2) {
    //                         waIt = myWalkingAreas.erase(waIt);
    //                         recheck = true;
    //                     } else {
    //                         waIt++;
    //                     }
    //                 }
    //                 crossing->valid = false;
    //                 crossing->prevWalkingArea = "";
    //                 crossing->nextWalkingArea = "";
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_crossings_and_walking_areas(&mut self) {
        self.build_crossings();
        self.build_walking_areas(utils::options::options_cont::get_int("junctions.corner-detail"), utils::options::options_cont::get_float("walkingareas.join-dist"));
        self.build_crossing_outlines();
        // ensure that all crossings are properly connected
        let mut recheck = !self.myCrossings.is_empty();
        while recheck {
            recheck = false;
            let nodeID = self.get_id().to_string();
            let mut waIDs: BTreeSet<String> = BTreeSet::new();
            let mut numSidewalks: i32 = 0;
            for wa in self.myWalkingAreas.iter() {
                waIDs.insert(wa.id.clone());
                numSidewalks += (wa.prevSidewalks.len() + wa.nextSidewalks.len()) as i32;
            }
            if numSidewalks < 2 {
                // all crossings are invalid if there are fewer than 2 sidewalks involved
                waIDs.clear();
            }
            for crossing in self.myCrossings.iter_mut() {
                if !waIDs.contains(&crossing.prevWalkingArea) || !waIDs.contains(&crossing.nextWalkingArea) || !crossing.valid {
                    if crossing.valid {
                        let mut edgesStr = String::new();
                        crate::netbuild::nb_cont_helper::shl_op(&mut edgesStr, &crossing.edges);
                        utils::common::msg_handler::write_warning(&format!("Discarding invalid crossing '{}' at junction '{}' with edges [{}] (no walkingarea found).", crossing.id, nodeID, edgesStr));
                        recheck = true;
                    }
                    let mut i = 0;
                    while i < self.myWalkingAreas.len() {
                        let wa = &mut self.myWalkingAreas[i];
                        if let Some(pos) = wa.nextCrossings.iter().position(|x| *x == crossing.id) {
                            wa.nextCrossings.remove(pos);
                        }
                        if wa.prevSidewalks.len() + wa.nextSidewalks.len() + wa.nextCrossings.len() + wa.prevCrossings.len() < 2 {
                            self.myWalkingAreas.remove(i);
                            recheck = true;
                        } else {
                            i += 1;
                        }
                    }
                    crossing.valid = false;
                    crossing.prevWalkingArea = String::new();
                    crossing.nextWalkingArea = String::new();
                }
            }
        }
    }

    /// C++: `EdgeVector NBNode::edgesBetween(const NBEdge *, const NBEdge *) const` [const]
    /// return all edges that lie clockwise between the given edges
    // EdgeVector
    // NBNode::edgesBetween(const NBEdge* e1, const NBEdge* e2) const {
    //     EdgeVector result;
    //     EdgeVector::const_iterator it = std::find(myAllEdges.begin(), myAllEdges.end(), e1);
    //     assert(it != myAllEdges.end());
    //     NBContHelper::nextCW(myAllEdges, it);
    //     EdgeVector::const_iterator it_end = std::find(myAllEdges.begin(), myAllEdges.end(), e2);
    //     assert(it_end != myAllEdges.end());
    //     while (it != it_end) {
    //         result.push_back(*it);
    //         NBContHelper::nextCW(myAllEdges, it);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn edges_between(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> EdgeVector {
        let mut result: EdgeVector = Vec::new();
        let mut it = self.myAllEdges.iter().position(|e| *e == e1).expect("edge not found in myAllEdges");
        NBContHelper::next_cw(&self.myAllEdges, &mut it);
        let it_end = self.myAllEdges.iter().position(|e| *e == e2).expect("edge not found in myAllEdges");
        while it != it_end {
            result.push(self.myAllEdges[it]);
            NBContHelper::next_cw(&self.myAllEdges, &mut it);
        }
        result
    }

    /// C++: `bool NBNode::crossingBetween(const NBEdge *, const NBEdge *) const` [const]
    /// return true if the given edges are connected by a crossing
    // bool
    // NBNode::crossingBetween(const NBEdge* e1, const NBEdge* e2) const {
    //     if (e1 == e2) {
    //         return false;
    //     }
    //     if (myAllEdges.size() > 3) {
    //         // pedestrian scramble
    //         return false;
    //     }
    //     for (auto c : getCrossings()) {
    //         const EdgeVector& edges = c->edges;
    //         EdgeVector::const_iterator it1 = std::find(edges.begin(), edges.end(), e1);
    //         EdgeVector::const_iterator it2 = std::find(edges.begin(), edges.end(), e2);
    //         if (it1 != edges.end() && it2 != edges.end()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn crossing_between(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> bool {
        if e1 == e2 {
            return false;
        }
        if self.myAllEdges.len() > 3 {
            // pedestrian scramble
            return false;
        }
        for c in self.get_crossings() {
            let edges = &c.edges;
            if edges.contains(&e1) && edges.contains(&e2) {
                return true;
            }
        }
        false
    }

    /// C++: `bool NBNode::alreadyConnectedPaths(const NBEdge *, const NBEdge *, double) const` [const]
    /// return true if the given pedestrian paths are connected at another junction within dist
    // bool
    // NBNode::alreadyConnectedPaths(const NBEdge* e1, const NBEdge* e2, double dist) const {
    //     if (e1 == e2) {
    //         return false;
    //     }
    //     if (e1->getPermissions() != SVC_PEDESTRIAN
    //             || e2->getPermissions() != SVC_PEDESTRIAN) {
    //         // no paths
    //         return false;
    //     }
    //     if (e1->getFinalLength() > dist &&
    //             e2->getFinalLength() > dist) {
    //         // too long
    //         return false;
    //     }
    //     NBNode* other1 = e1->getFromNode() == this ? e1->getToNode() : e1->getFromNode();
    //     NBNode* other2 = e2->getFromNode() == this ? e2->getToNode() : e2->getFromNode();
    //     return other1 == other2;
    // }
    #[allow(unused_variables)]
    pub fn already_connected_paths(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>, dist: f64) -> bool {
        if e1 == e2 {
            return false;
        }
        if e1.get_permissions(-1) != SVC_PEDESTRIAN || e2.get_permissions(-1) != SVC_PEDESTRIAN {
            // no paths
            return false;
        }
        if e1.get_final_length() > dist && e2.get_final_length() > dist {
            // too long
            return false;
        }
        let other1 = if e1.get_from_node() == Ptr::new(self) { e1.get_to_node() } else { e1.get_from_node() };
        let other2 = if e2.get_from_node() == Ptr::new(self) { e2.get_to_node() } else { e2.get_from_node() };
        other1 == other2
    }

    /// C++: `bool NBNode::crossesFringe(const NBEdge *, const NBEdge *) const` [const]
    /// return true if the given sidewalks are separated by a fringe road
    // bool
    // NBNode::crossesFringe(const NBEdge* e1, const NBEdge* e2) const {
    //     return myFringeType != FringeType::DEFAULT
    //            && myIncomingEdges.size() == 1 && myOutgoingEdges.size() == 1
    //            && (e1->isTurningDirectionAt(e2) || e2->isTurningDirectionAt(e1));
    // }
    #[allow(unused_variables)]
    pub fn crosses_fringe(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> bool {
        self.myFringeType != FringeType::DEFAULT
            && self.myIncomingEdges.len() == 1
            && self.myOutgoingEdges.len() == 1
            && (e1.is_turning_direction_at(e2) || e2.is_turning_direction_at(e1))
    }

    /// C++: `const NBConnectionProhibits & NBNode::getProhibitions()`
    /// get prohibitions (BLocked connections)
    // const NBConnectionProhibits& getProhibitions() {
    //         return myBlockedConnections;
    //     }
    #[allow(unused_variables)]
    pub fn get_prohibitions(&mut self) -> &NBConnectionProhibits {
        &self.myBlockedConnections
    }

    /// C++: `bool NBNode::geometryLike() const` [const]
    /// whether this is structurally similar to a geometry node
    // bool
    // NBNode::geometryLike() const {
    //     return geometryLike(myIncomingEdges, myOutgoingEdges);
    // }
    #[allow(unused_variables)]
    pub fn geometry_like(&self) -> bool {
        NBNode::geometry_like_2(&self.myIncomingEdges, &self.myOutgoingEdges)
    }

    /// C++: `bool NBNode::geometryLike(const EdgeVector &, const EdgeVector &)` [static]
    // bool
    // NBNode::geometryLike(const EdgeVector& incoming, const EdgeVector& outgoing) {
    //     if (incoming.size() == 1 && outgoing.size() == 1) {
    //         return incoming.front()->getBidiEdge() != outgoing.front();
    //     }
    //     if (incoming.size() == 2 && outgoing.size() == 2) {
    //         // check whether the incoming and outgoing edges are pairwise (near) parallel and
    //         // thus the only cross-connections could be turn-arounds
    //         NBEdge* in0 = incoming[0];
    //         NBEdge* in1 = incoming[1];
    //         NBEdge* out0 = outgoing[0];
    //         NBEdge* out1 = outgoing[1];
    //         if ((in0->isTurningDirectionAt(out0) || in0->isTurningDirectionAt(out1))
    //                 && (in1->isTurningDirectionAt(out0) || in1->isTurningDirectionAt(out1))) {
    //             return true;
    //         }
    //         if (in0->getGeometry() == in1->getGeometry() && out0->getGeometry() == out1->getGeometry()) {
    //             // overlapping edges
    //             return true;
    //         }
    //         for (EdgeVector::const_iterator it = incoming.begin(); it != incoming.end(); ++it) {
    //             NBEdge* inEdge = *it;
    //             double angle0 = fabs(NBHelpers::relAngle(inEdge->getAngleAtNode(inEdge->getToNode()), out0->getAngleAtNode(out0->getFromNode())));
    //             double angle1 = fabs(NBHelpers::relAngle(inEdge->getAngleAtNode(inEdge->getToNode()), out1->getAngleAtNode(out1->getFromNode())));
    //             if (MAX2(angle0, angle1) <= 160) {
    //                 // neither of the outgoing edges is parallel to inEdge
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn geometry_like_2(incoming: &EdgeVector, outgoing: &EdgeVector) -> bool {
        if incoming.len() == 1 && outgoing.len() == 1 {
            return incoming[0].get_bidi_edge() != outgoing[0];
        }
        if incoming.len() == 2 && outgoing.len() == 2 {
            // check whether the incoming and outgoing edges are pairwise (near) parallel and
            // thus the only cross-connections could be turn-arounds
            let in0 = incoming[0];
            let in1 = incoming[1];
            let out0 = outgoing[0];
            let out1 = outgoing[1];
            if (in0.is_turning_direction_at(out0) || in0.is_turning_direction_at(out1))
                && (in1.is_turning_direction_at(out0) || in1.is_turning_direction_at(out1))
            {
                return true;
            }
            if in0.get_geometry() == in1.get_geometry() && out0.get_geometry() == out1.get_geometry() {
                // overlapping edges
                return true;
            }
            for inEdge in incoming.iter() {
                let angle0 = NBHelpers::rel_angle(inEdge.get_angle_at_node(inEdge.get_to_node()), out0.get_angle_at_node(out0.get_from_node())).abs();
                let angle1 = NBHelpers::rel_angle(inEdge.get_angle_at_node(inEdge.get_to_node()), out1.get_angle_at_node(out1.get_from_node())).abs();
                if angle0.max(angle1) <= 160. {
                    // neither of the outgoing edges is parallel to inEdge
                    return false;
                }
            }
            return true;
        }
        false
    }

    /// C++: `void NBNode::setRoundabout()`
    /// update the type of this node as a roundabout
    // void
    // NBNode::setRoundabout() {
    //     if (myType == SumoXMLNodeType::RIGHT_BEFORE_LEFT || myType == SumoXMLNodeType::LEFT_BEFORE_RIGHT) {
    //         myType = SumoXMLNodeType::PRIORITY;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_roundabout(&mut self) {
        if self.myType == SumoXMLNodeType::RIGHT_BEFORE_LEFT || self.myType == SumoXMLNodeType::LEFT_BEFORE_RIGHT {
            self.myType = SumoXMLNodeType::PRIORITY;
        }
    }

    /// C++: `bool NBNode::isRoundabout() const` [const]
    /// return whether this node is part of a roundabout
    // bool
    // NBNode::isRoundabout() const {
    //     for (NBEdge* out : myOutgoingEdges) {
    //         if (out->getJunctionPriority(this) == NBEdge::JunctionPriority::ROUNDABOUT) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_roundabout(&self) -> bool {
        for out in self.myOutgoingEdges.iter() {
            if out.get_junction_priority(Ptr::new(self)) == NBEdge_JunctionPriority::ROUNDABOUT as i32 {
                return true;
            }
        }
        false
    }

    /// C++: `NBNode::Crossing * NBNode::addCrossing(EdgeVector, double, bool, int, int, const PositionVector &, bool, const Parameterised *)`
    /// add a pedestrian crossing to this node
    // NBNode::Crossing*
    // NBNode::addCrossing(EdgeVector edges, double width, bool priority, int tlIndex, int tlIndex2,
    //                     const PositionVector& customShape, bool fromSumoNet, const Parameterised* params) {
    //     Crossing* c = new Crossing(this, edges, width, priority, tlIndex, tlIndex2, customShape);
    //     if (params != nullptr) {
    //         c->updateParameters(params->getParametersMap());
    //     }
    //     myCrossings.push_back(std::unique_ptr<Crossing>(c));
    //     if (fromSumoNet) {
    //         myCrossingsLoadedFromSumoNet += 1;
    //     }
    //     return c;
    // }
    #[allow(unused_variables)]
    pub fn add_crossing(&mut self, edges: EdgeVector, width: f64, priority: bool, tlIndex: i32, tlIndex2: i32, customShape: &PositionVector, fromSumoNet: bool, params: Ptr<Parameterised>) -> Ptr<NBNode_Crossing> {
        let mut c = Box::new(NBNode_Crossing::new(Ptr::from_mut(self), &edges, width, priority, tlIndex, tlIndex2, customShape));
        if !params.is_null() {
            c.update_parameters(params.get_parameters_map());
        }
        let cp = Ptr::from_box(&c);
        self.myCrossings.push(c);
        if fromSumoNet {
            self.myCrossingsLoadedFromSumoNet += 1;
        }
        cp
    }

    /// C++: `void NBNode::addWalkingAreaShape(EdgeVector, const PositionVector &, double)`
    /// add custom shape for walkingArea
    // void
    // NBNode::addWalkingAreaShape(EdgeVector edges, const PositionVector& shape, double width) {
    //     WalkingAreaCustomShape wacs;
    //     wacs.edges.insert(edges.begin(), edges.end());
    //     wacs.shape = shape;
    //     wacs.width = width;
    //     myWalkingAreaCustomShapes.push_back(wacs);
    // }
    #[allow(unused_variables)]
    pub fn add_walking_area_shape(&mut self, edges: EdgeVector, shape: &PositionVector, width: f64) {
        let mut wacs = NBNode_WalkingAreaCustomShape { edges: BTreeSet::new(), shape: shape.clone(), width };
        wacs.edges.extend(edges.iter().copied());
        self.myWalkingAreaCustomShapes.push(wacs);
    }

    /// C++: `void NBNode::removeCrossing(const EdgeVector &)`
    /// remove a pedestrian crossing from this node (identified by its edges)
    // void
    // NBNode::removeCrossing(const EdgeVector& edges) {
    //     EdgeSet edgeSet(edges.begin(), edges.end());
    //     for (auto it = myCrossings.begin(); it != myCrossings.end();) {
    //         EdgeSet edgeSet2((*it)->edges.begin(), (*it)->edges.end());
    //         if (edgeSet == edgeSet2) {
    //             it = myCrossings.erase(it);
    //         } else {
    //             ++it;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_crossing(&mut self, edges: &EdgeVector) {
        let edgeSet: BTreeSet<Ptr<NBEdge>> = edges.iter().copied().collect();
        self.myCrossings.retain(|c| {
            let edgeSet2: BTreeSet<Ptr<NBEdge>> = c.edges.iter().copied().collect();
            edgeSet != edgeSet2
        });
    }

    /// C++: `void NBNode::discardAllCrossings(bool)`
    /// discard all current (and optionally future) crossings
    // void
    // NBNode::discardAllCrossings(bool rejectAll) {
    //     myCrossings.clear();
    //     // also discard all further crossings
    //     if (rejectAll) {
    //         myDiscardAllCrossings = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn discard_all_crossings(&mut self, rejectAll: bool) {
        self.myCrossings.clear();
        // also discard all further crossings
        if rejectAll {
            self.myDiscardAllCrossings = true;
        }
    }

    /// C++: `void NBNode::discardWalkingareas()`
    /// discard previously built walkingareas (required for repeated computation by netedit)
    // void
    // NBNode::discardWalkingareas() {
    //     myWalkingAreas.clear();
    // }
    #[allow(unused_variables)]
    pub fn discard_walkingareas(&mut self) {
        self.myWalkingAreas.clear();
    }

    /// C++: `int NBNode::numCrossingsFromSumoNet() const` [const]
    /// get num of crossings from sumo net
    // int numCrossingsFromSumoNet() const {
    //         return myCrossingsLoadedFromSumoNet;
    //     }
    #[allow(unused_variables)]
    pub fn num_crossings_from_sumo_net(&self) -> i32 {
        self.myCrossingsLoadedFromSumoNet
    }

    /// C++: `std::vector<Crossing *> NBNode::getCrossings() const` [const]
    /// return this junctions pedestrian crossings
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_crossings(&self) -> Vec<Ptr<NBNode_Crossing>> {
        self.myCrossings.iter().filter(|c| c.valid).map(|c| Ptr::from_box(c)).collect()
    }

    /// C++: `const std::vector<std::unique_ptr<Crossing>> & NBNode::getCrossingsIncludingInvalid() const` [const]
    // inline const std::vector<std::unique_ptr<Crossing> >& getCrossingsIncludingInvalid() const {
    //         return myCrossings;
    //     }
    #[allow(unused_variables)]
    pub fn get_crossings_including_invalid(&self) -> &Vec<Box<NBNode_Crossing>> {
        &self.myCrossings
    }

    /// C++: `const std::vector<WalkingArea> & NBNode::getWalkingAreas() const` [const]
    /// return this junctions pedestrian walking areas
    // inline const std::vector<WalkingArea>& getWalkingAreas() const {
    //         return myWalkingAreas;
    //     }
    #[allow(unused_variables)]
    pub fn get_walking_areas(&self) -> &Vec<NBNode_WalkingArea> {
        &self.myWalkingAreas
    }

    /// C++: `const std::vector<WalkingAreaCustomShape> & NBNode::getWalkingAreaCustomShapes() const` [const]
    // const std::vector<WalkingAreaCustomShape>& getWalkingAreaCustomShapes() const {
    //         return myWalkingAreaCustomShapes;
    //     }
    #[allow(unused_variables)]
    pub fn get_walking_area_custom_shapes(&self) -> &Vec<NBNode_WalkingAreaCustomShape> {
        &self.myWalkingAreaCustomShapes
    }

    /// C++: `Crossing * NBNode::getCrossing(const std::string &) const` [const]
    /// return the crossing with the given id
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_crossing(&self, id: &str) -> Ptr<NBNode_Crossing> {
        for c in self.myCrossings.iter() {
            if c.id == id {
                return Ptr::from_box(c);
            }
        }
        panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Request for unknown crossing '{}'", id)));
    }

    /// C++: `Crossing * NBNode::getCrossing(const EdgeVector &, bool) const` [const]
    /// return the crossing with the given Edges
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_crossing_2(&self, edges: &EdgeVector, hardFail: bool) -> Ptr<NBNode_Crossing> {
        let edgeSet: BTreeSet<Ptr<NBEdge>> = edges.iter().copied().collect();
        for crossing in self.myCrossings.iter() {
            let edgeSet2: BTreeSet<Ptr<NBEdge>> = crossing.edges.iter().copied().collect();
            if edgeSet == edgeSet2 {
                return Ptr::from_box(crossing);
            }
        }
        if !hardFail {
            return Ptr::default();
        }
        panic!("{}", utils::common::util_exceptions::ProcessError::new("Request for unknown crossing for the given Edges"));
    }

    /// C++: `WalkingArea & NBNode::getWalkingArea(const std::string &)`
    /// return the walkingArea with the given ID
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_walking_area(&mut self, id: &str) -> &mut NBNode_WalkingArea {
        if let Some(i) = self.myWalkingAreas.iter().position(|wa| wa.id == id) {
            return &mut self.myWalkingAreas[i];
        }
        // not found, maybe we need to rebuild
        self.update_surrounding_geometry();
        self.sort_edges(true);
        self.build_crossings_and_walking_areas();
        if let Some(i) = self.myWalkingAreas.iter().position(|wa| wa.id == id) {
            return &mut self.myWalkingAreas[i];
        }
        if !self.myWalkingAreas.is_empty() {
            // don't crash
            utils::common::msg_handler::write_warning(&format!("Could not retrieve walkingarea '{}' (edge ordering changed after recompute).", id));
            return &mut self.myWalkingAreas[0];
        }
        panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Request for unknown walkingarea '{}'.", id)));
    }

    /// C++: `bool NBNode::setCrossingTLIndices(const std::string &, int, bool)`
    // bool
    // NBNode::setCrossingTLIndices(const std::string& tlID, int startIndex, bool ignoreCustom) {
    //     bool usedCustom = false;
    //     for (auto c : getCrossings()) {
    //         c->tlLinkIndex = startIndex++;
    //         c->tlID = tlID;
    //         if (c->customTLIndex != -1 && !ignoreCustom) {
    //             usedCustom |= (c->tlLinkIndex != c->customTLIndex);
    //             c->tlLinkIndex = c->customTLIndex;
    //         }
    //         if (c->customTLIndex2 != -1 && !ignoreCustom) {
    //             usedCustom = true;
    //             c->tlLinkIndex2 = c->customTLIndex2;
    //         }
    //     }
    //     return usedCustom;
    // }
    #[allow(unused_variables)]
    pub fn set_crossing_tl_indices(&mut self, tlID: &str, startIndex: i32, ignoreCustom: bool) -> bool {
        let mut usedCustom = false;
        let mut startIndex = startIndex;
        for c in self.get_crossings() {
            let c = c.m();
            c.tlLinkIndex = startIndex;
            startIndex += 1;
            c.tlID = tlID.to_string();
            if c.customTLIndex != -1 && !ignoreCustom {
                usedCustom |= c.tlLinkIndex != c.customTLIndex;
                c.tlLinkIndex = c.customTLIndex;
            }
            if c.customTLIndex2 != -1 && !ignoreCustom {
                usedCustom = true;
                c.tlLinkIndex2 = c.customTLIndex2;
            }
        }
        usedCustom
    }

    /// C++: `int NBNode::numNormalConnections() const` [const]
    /// return the number of lane-to-lane connections at this junction (excluding crossings)
    // int
    // NBNode::numNormalConnections() const {
    //     if (myRequest == nullptr) {
    //         // could be an uncontrolled type
    //         int result = 0;
    //         for (const NBEdge* const edge : myIncomingEdges) {
    //             result += (int)edge->getConnections().size();
    //         }
    //         return result;
    //     } else {
    //         return myRequest->getSizes().second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn num_normal_connections(&self) -> i32 {
        if self.myRequest.is_null() {
            // could be an uncontrolled type
            let mut result = 0;
            for edge in self.myIncomingEdges.iter() {
                result += edge.get_connections().len() as i32;
            }
            result
        } else {
            self.myRequest.get_sizes().1
        }
    }

    /// C++: `void NBNode::avoidOverlap()`
    /// fix overlap
    // void
    // NBNode::avoidOverlap() {
    //     // simple case: edges with LaneSpreadFunction::CENTER and a (possible) turndirection at the same node
    //     bool haveModifications = false;
    //     for (EdgeVector::iterator it = myIncomingEdges.begin(); it != myIncomingEdges.end(); it++) {
    //         NBEdge* edge = *it;
    //         NBEdge* turnDest = edge->getTurnDestination(true);
    //         if (turnDest != nullptr) {
    //             haveModifications |= edge->shiftPositionAtNode(this, turnDest);
    //             haveModifications |= turnDest->shiftPositionAtNode(this, edge);
    //         }
    //     }
    //     if (haveModifications) {
    //         NBTurningDirectionsComputer::computeTurnDirectionsForNode(this, false);
    //     }
    //     // @todo: edges in the same direction with sharp angles starting/ending at the same position
    // }
    #[allow(unused_variables)]
    pub fn avoid_overlap(&mut self) {
        // simple case: edges with LaneSpreadFunction::CENTER and a (possible) turndirection at the same node
        let mut haveModifications = false;
        for edge in self.myIncomingEdges.clone() {
            let turnDest = edge.get_turn_destination(true);
            if !turnDest.is_null() {
                haveModifications |= edge.m().shift_position_at_node(Ptr::from_mut(self), turnDest);
                haveModifications |= turnDest.m().shift_position_at_node(Ptr::from_mut(self), edge);
            }
        }
        if haveModifications {
            crate::netbuild::nb_algorithms::NBTurningDirectionsComputer::compute_turn_directions_for_node(Ptr::from_mut(self), false);
        }
        // @todo: edges in the same direction with sharp angles starting/ending at the same position
    }

    /// C++: `bool NBNode::extraConflict(int, int) const` [const]
    /// whether the given index must yield to the foeIndex while turing right on a red light
    // bool
    // NBNode::extraConflict(int index, int foeIndex) const {
    //     for (NBTrafficLightDefinition* def : myTrafficLights) {
    //         if (def->extraConflict(index, foeIndex)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn extra_conflict(&self, index: i32, foeIndex: i32) -> bool {
        for def in self.myTrafficLights.iter() {
            if def.extra_conflict(index, foeIndex) {
                return true;
            }
        }
        false
    }

    /// C++: `void NBNode::sortEdges(bool)`
    /// sort all edge containers for this node
    // void
    // NBNode::sortEdges(bool useNodeShape) {
    //     if (myAllEdges.size() == 0) {
    //         return;
    //     }
    //     EdgeVector allEdgesOriginal = myAllEdges;
    //     EdgeVector& allEdges = myAllEdges;
    //     EdgeVector& incoming = myIncomingEdges;
    //     EdgeVector& outgoing = myOutgoingEdges;
    //
    //     // sort the edges by angle (this is the canonical sorting)
    //     std::sort(allEdges.begin(), allEdges.end(), NBNodesEdgesSorter::edge_by_junction_angle_sorter(this));
    //     std::sort(incoming.begin(), incoming.end(), NBNodesEdgesSorter::edge_by_junction_angle_sorter(this));
    //     std::sort(outgoing.begin(), outgoing.end(), NBNodesEdgesSorter::edge_by_junction_angle_sorter(this));
    //     std::vector<NBEdge*>::iterator j;
    //     for (j = allEdges.begin(); j != allEdges.end() - 1 && j != allEdges.end(); ++j) {
    //         NBNodesEdgesSorter::swapWhenReversed(this, j, j + 1);
    //     }
    //     if (allEdges.size() > 1 && j != allEdges.end()) {
    //         NBNodesEdgesSorter::swapWhenReversed(this, allEdges.end() - 1, allEdges.begin());
    //     }
    //
    //     // sort again using additional geometry information
    //     NBEdge* firstOfAll = allEdges.front();
    //     NBEdge* firstOfIncoming = incoming.size() > 0 ? incoming.front() : 0;
    //     NBEdge* firstOfOutgoing = outgoing.size() > 0 ? outgoing.front() : 0;
    //     // sort by the angle between the node shape center and the point where the edge meets the node shape
    //     std::sort(allEdges.begin(), allEdges.end(), NBContHelper::edge_by_angle_to_nodeShapeCentroid_sorter(this));
    //     std::sort(incoming.begin(), incoming.end(), NBContHelper::edge_by_angle_to_nodeShapeCentroid_sorter(this));
    //     std::sort(outgoing.begin(), outgoing.end(), NBContHelper::edge_by_angle_to_nodeShapeCentroid_sorter(this));
    //     // let the first edge remain the first
    //     rotate(allEdges.begin(), std::find(allEdges.begin(), allEdges.end(), firstOfAll), allEdges.end());
    //     if (firstOfIncoming != nullptr) {
    //         rotate(incoming.begin(), std::find(incoming.begin(), incoming.end(), firstOfIncoming), incoming.end());
    //     }
    //     if (firstOfOutgoing != nullptr) {
    //         rotate(outgoing.begin(), std::find(outgoing.begin(), outgoing.end(), firstOfOutgoing), outgoing.end());
    //     }
    // #ifdef DEBUG_EDGE_SORTING
    //     if (DEBUGCOND) {
    //         std::cout << "sortedEdges (useNodeShape=" << useNodeShape << "):\n";
    //         for (NBEdge* e : allEdges) {
    //             std::cout << "  " << e->getID()
    //                       << " angleToCenter=" << e->getAngleAtNodeToCenter(this)
    //                       << " junctionAngle=" << e->getAngleAtNode(this) << "\n";
    //         }
    //     }
    // #endif
    //
    //     // fixing some pathological all edges orderings
    //     // if every of the edges a,b,c has a turning edge a',b',c' the all edges ordering should be a,a',b,b',c,c'
    //     if (incoming.size() == outgoing.size() && incoming.front() == allEdges.front()) {
    //         std::vector<NBEdge*>::const_iterator in, out;
    //         std::vector<NBEdge*> allTmp;
    //         for (in = incoming.begin(), out = outgoing.begin(); in != incoming.end(); ++in, ++out) {
    //             if ((*in)->isTurningDirectionAt(*out)) {
    //                 allTmp.push_back(*in);
    //                 allTmp.push_back(*out);
    //             } else {
    //                 break;
    //             }
    //         }
    //         if (allTmp.size() == allEdges.size()) {
    //             allEdges = allTmp;
    //         }
    //     }
    //     // sort the crossings
    //     std::sort(myCrossings.begin(), myCrossings.end(), NBNodesEdgesSorter::crossing_by_junction_angle_sorter(this, allEdges));
    //     //if (crossings.size() > 0) {
    //     //    std::cout << " crossings at " << getID() << "\n";
    //     //    for (std::vector<NBNode::Crossing*>::iterator it = crossings.begin(); it != crossings.end(); ++it) {
    //     //        std::cout << "  " << toString((*it)->edges) << "\n";
    //     //    }
    //     //}
    //
    //     if (useNodeShape && myAllEdges != allEdgesOriginal) {
    //         // sorting order changed after node shape was computed.
    //         computeNodeShape(-1);
    //         for (NBEdge* e : myAllEdges) {
    //             e->computeEdgeShape();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sort_edges(&mut self, useNodeShape: bool) {
        if self.myAllEdges.is_empty() {
            return;
        }
        let me = Ptr::from_mut(self);
        let allEdgesOriginal = self.myAllEdges.clone();

        // sort the edges by angle (this is the canonical sorting)
        let byJunctionAngle = NBNodesEdgesSorter_edge_by_junction_angle_sorter::new(me);
        sort_edges_by(&mut self.myAllEdges, |a, b| byJunctionAngle.call_op(a, b) != 0);
        sort_edges_by(&mut self.myIncomingEdges, |a, b| byJunctionAngle.call_op(a, b) != 0);
        sort_edges_by(&mut self.myOutgoingEdges, |a, b| byJunctionAngle.call_op(a, b) != 0);
        let n = self.myAllEdges.len();
        for j in 0..n.saturating_sub(1) {
            NBNodesEdgesSorter::swap_when_reversed(me, &mut self.myAllEdges, j, j + 1);
        }
        if n > 1 {
            NBNodesEdgesSorter::swap_when_reversed(me, &mut self.myAllEdges, n - 1, 0);
        }

        // sort again using additional geometry information
        let firstOfAll = self.myAllEdges[0];
        let firstOfIncoming: Option<Ptr<NBEdge>> = self.myIncomingEdges.first().copied();
        let firstOfOutgoing: Option<Ptr<NBEdge>> = self.myOutgoingEdges.first().copied();
        // sort by the angle between the node shape center and the point where the edge meets the node shape
        let byCentroid = NBContHelper_edge_by_angle_to_nodeShapeCentroid_sorter::new(me);
        sort_edges_by(&mut self.myAllEdges, |a, b| byCentroid.call_op(a, b));
        sort_edges_by(&mut self.myIncomingEdges, |a, b| byCentroid.call_op(a, b));
        sort_edges_by(&mut self.myOutgoingEdges, |a, b| byCentroid.call_op(a, b));
        // let the first edge remain the first
        rotate_edges_to_front(&mut self.myAllEdges, Some(firstOfAll));
        rotate_edges_to_front(&mut self.myIncomingEdges, firstOfIncoming);
        rotate_edges_to_front(&mut self.myOutgoingEdges, firstOfOutgoing);

        // fixing some pathological all edges orderings
        // if every of the edges a,b,c has a turning edge a',b',c' the all edges ordering should be a,a',b,b',c,c'
        if self.myIncomingEdges.len() == self.myOutgoingEdges.len() && self.myIncomingEdges.first() == self.myAllEdges.first() {
            let mut allTmp: EdgeVector = Vec::new();
            for (inE, outE) in self.myIncomingEdges.iter().zip(self.myOutgoingEdges.iter()) {
                if inE.is_turning_direction_at(*outE) {
                    allTmp.push(*inE);
                    allTmp.push(*outE);
                } else {
                    break;
                }
            }
            if allTmp.len() == self.myAllEdges.len() {
                self.myAllEdges = allTmp;
            }
        }
        // sort the crossings
        let crossingSorter = NBNodesEdgesSorter_crossing_by_junction_angle_sorter::new(me, &self.myAllEdges);
        self.myCrossings.sort_by(|a, b| {
            if crossingSorter.call_op(a, b) != 0 {
                std::cmp::Ordering::Less
            } else if crossingSorter.call_op(b, a) != 0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        if useNodeShape && self.myAllEdges != allEdgesOriginal {
            // sorting order changed after node shape was computed.
            self.compute_node_shape(-1.);
            for e in self.myAllEdges.iter() {
                e.m().compute_edge_shape(-1.);
            }
        }
    }

    /// C++: `int NBNode::getConnectionIndex(const NBEdge *, const NBEdge::Connection &) const` [const]
    /// return the index of the given connection
    // int
    // NBNode::getConnectionIndex(const NBEdge* from, const NBEdge::Connection& con) const {
    //     int result = 0;
    //     for (const NBEdge* const e : myIncomingEdges) {
    //         for (const NBEdge::Connection& cand : e->getConnections()) {
    //             if (e == from && cand.fromLane == con.fromLane && cand.toLane == con.toLane && cand.toEdge == con.toEdge) {
    //                 return result;
    //             }
    //             result++;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_connection_index(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection) -> i32 {
        let mut result: i32 = 0;
        for e in self.myIncomingEdges.iter() {
            let e = *e;
            for cand in e.get_connections() {
                if e == from && cand.fromLane == con.fromLane && cand.toLane == con.toLane && cand.toEdge == con.toEdge {
                    return result;
                }
                result += 1;
            }
        }
        -1
    }

    /// C++: `bool NBNode::isTrafficLight(SumoXMLNodeType)` [static]
    /// return whether the given type is a traffic light
    // bool
    // NBNode::isTrafficLight(SumoXMLNodeType type) {
    //     return type == SumoXMLNodeType::TRAFFIC_LIGHT
    //            || type == SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION
    //            || type == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED;
    // }
    #[allow(unused_variables)]
    pub fn is_traffic_light(type_: SumoXMLNodeType) -> bool {
        type_ == SumoXMLNodeType::TRAFFIC_LIGHT
            || type_ == SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION
            || type_ == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED
    }

    /// C++: `bool NBNode::isTrafficLight() const` [const]
    // inline bool isTrafficLight() const {
    //         return isTrafficLight(myType);
    //     }
    #[allow(unused_variables)]
    pub fn is_traffic_light_2(&self) -> bool {
        NBNode::is_traffic_light(self.myType)
    }

    /// C++: `bool NBNode::isSimpleContinuation(bool, bool) const` [const]
    /// check if node is a simple continuation
    // bool
    // NBNode::isSimpleContinuation(bool checkLaneNumbers, bool checkWidth) const {
    //     // one in, one out->continuation
    //     if (myIncomingEdges.size() == 1 && myOutgoingEdges.size() == 1) {
    //         NBEdge* in = myIncomingEdges.front();
    //         NBEdge* out = myOutgoingEdges.front();
    //         // both must have the same number of lanes
    //         return ((!checkLaneNumbers || in->getNumLanes() == out->getNumLanes())
    //                 && (!checkWidth || in->getTotalWidth() == out->getTotalWidth()));
    //     }
    //     // two in and two out and both in reverse direction
    //     if (myIncomingEdges.size() == 2 && myOutgoingEdges.size() == 2) {
    //         for (EdgeVector::const_iterator i = myIncomingEdges.begin(); i != myIncomingEdges.end(); i++) {
    //             NBEdge* in = *i;
    //             EdgeVector::const_iterator opposite = find_if(myOutgoingEdges.begin(), myOutgoingEdges.end(), NBContHelper::opposite_finder(in));
    //             // must have an opposite edge
    //             if (opposite == myOutgoingEdges.end()) {
    //                 return false;
    //             }
    //             // both must have the same number of lanes
    //             NBContHelper::nextCW(myOutgoingEdges, opposite);
    //             if (checkLaneNumbers && in->getNumLanes() != (*opposite)->getNumLanes()) {
    //                 return false;
    //             }
    //             if (checkWidth && in->getTotalWidth() != (*opposite)->getTotalWidth()) {
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    //     // nope
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_simple_continuation(&self, checkLaneNumbers: bool, checkWidth: bool) -> bool {
        todo!("port NBNode::isSimpleContinuation")
    }

    /// C++: `void NBNode::markBentPriority(bool)`
    /// mark whether a priority road turns at this node
    // void markBentPriority(bool isBent) {
    //         myIsBentPriority = isBent;
    //     }
    #[allow(unused_variables)]
    pub fn mark_bent_priority(&mut self, isBent: bool) {
        todo!("port NBNode::markBentPriority")
    }

    /// C++: `bool NBNode::isBentPriority() const` [const]
    /// return whether a priority road turns at this node
    // bool isBentPriority() const {
    //         return myIsBentPriority;
    //     }
    #[allow(unused_variables)]
    pub fn is_bent_priority(&self) -> bool {
        todo!("port NBNode::isBentPriority")
    }

    /// C++: `bool NBNode::typeWasGuessed() const` [const]
    /// return whether a priority road turns at this node
    // bool typeWasGuessed() const {
    //         return myTypeWasGuessed;
    //     }
    #[allow(unused_variables)]
    pub fn type_was_guessed(&self) -> bool {
        todo!("port NBNode::typeWasGuessed")
    }

    /// C++: `bool NBNode::isConstantWidthTransition() const` [const]
    /// detects whether a given junction splits or merges lanes while keeping constant road width
    // bool
    // NBNode::isConstantWidthTransition() const {
    //     return (myIncomingEdges.size() == 1
    //             && myOutgoingEdges.size() == 1
    //             && myIncomingEdges[0]->getNumLanes() != myOutgoingEdges[0]->getNumLanes()
    //             && myIncomingEdges[0]->getTotalWidth() == myOutgoingEdges[0]->getTotalWidth());
    // }
    #[allow(unused_variables)]
    pub fn is_constant_width_transition(&self) -> bool {
        todo!("port NBNode::isConstantWidthTransition")
    }

    /// C++: `std::vector<std::pair<Position, std::string>> NBNode::getEndPoints() const` [const]
    /// return list of unique endpoint coordinates of all edges at this node
    // std::vector<std::pair<Position, std::string> >
    // NBNode::getEndPoints() const {
    //     // using a set would be nicer but we want to have some slack in position identification
    //     std::vector<std::pair<Position, std::string> >result;
    //     for (NBEdge* e : myAllEdges) {
    //         Position pos = this == e->getFromNode() ? e->getGeometry().front() : e->getGeometry().back();
    //         const std::string origID = e->getParameter(this == e->getFromNode() ? "origFrom" : "origTo");
    //         bool unique = true;
    //         for (const auto& pair : result) {
    //             if (pos.almostSame(pair.first) || (origID != "" && pair.second == origID)) {
    //                 unique = false;
    //                 break;
    //             }
    //         }
    //         if (unique) {
    //             result.push_back(std::make_pair(pos, origID));
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_end_points(&self) -> Vec<(Position, String)> {
        todo!("port NBNode::getEndPoints")
    }

    /// C++: `void NBNode::recheckVClassConnections(NBEdge *)`
    /// ensure connectivity for all vClasses
    // void
    // NBNode::recheckVClassConnections(NBEdge* currentOutgoing) {
    //     // ensure that all modes have a connection if possible
    //     for (NBEdge* incoming : myIncomingEdges) {
    //         if (incoming->getConnectionLanes(currentOutgoing).size() > 0 && incoming->getStep() <= NBEdge::EdgeBuildingStep::LANES2LANES_DONE) {
    //             // no connections are needed for pedestrians during this step
    //             // no satisfaction is possible if the outgoing edge disallows
    //             SVCPermissions unsatisfied = incoming->getPermissions() & currentOutgoing->getPermissions() & ~SVC_PEDESTRIAN;
    //             //std::cout << "initial unsatisfied modes from edge=" << incoming->getID() << " toEdge=" << currentOutgoing->getID() << " deadModes=" << getVehicleClassNames(unsatisfied) << "\n";
    //             const std::vector<NBEdge::Connection>& elv = incoming->getConnections();
    //             for (std::vector<NBEdge::Connection>::const_iterator k = elv.begin(); k != elv.end(); ++k) {
    //                 const NBEdge::Connection& c = *k;
    //                 if (c.toEdge == currentOutgoing && c.toLane >= 0) {
    //                     const SVCPermissions satisfied = (incoming->getPermissions(c.fromLane) & c.toEdge->getPermissions(c.toLane));
    //                     //std::cout << "  from=" << incoming->getID() << "_" << c.fromLane << " to=" << c.toEdge->getID() << "_" << c.toLane << " satisfied=" << getVehicleClassNames(satisfied) << "\n";
    //                     unsatisfied &= ~satisfied;
    //                 }
    //             }
    //             if (unsatisfied != 0) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                 if (DEBUGCOND) {
    //                     std::cout << " unsatisfied modes from edge=" << incoming->getID() << " toEdge=" << currentOutgoing->getID() << " deadModes=" << getVehicleClassNames(unsatisfied) << "\n";
    //                 }
    // #endif
    //                 int fromLane = 0;
    //                 // first attempt: try to use a dedicated fromLane
    //                 while (unsatisfied != 0 && fromLane < incoming->getNumLanes()) {
    //                     if (incoming->getPermissions(fromLane) == unsatisfied) {
    //                         unsatisfied = findToLaneForPermissions(currentOutgoing, fromLane, incoming, unsatisfied);
    //                     }
    //                     fromLane++;
    //                 }
    //                 // second attempt: try to re-use a fromLane that already connects to currentOutgoing
    //                 // (because we don't wont to create extra turn lanes)
    //                 fromLane = 0;
    //                 while (unsatisfied != 0 && fromLane < incoming->getNumLanes()) {
    //                     if ((incoming->getPermissions(fromLane) & unsatisfied) != 0
    //                             && incoming->getConnectionsFromLane(fromLane, currentOutgoing, -1).size() > 0) {
    //                         unsatisfied = findToLaneForPermissions(currentOutgoing, fromLane, incoming, unsatisfied);
    //                     }
    //                     fromLane++;
    //                 }
    //                 // third attempt: use any possible fromLane
    //                 fromLane = 0;
    //                 while (unsatisfied != 0 && fromLane < incoming->getNumLanes()) {
    //                     if ((incoming->getPermissions(fromLane) & unsatisfied) != 0) {
    //                         unsatisfied = findToLaneForPermissions(currentOutgoing, fromLane, incoming, unsatisfied);
    //                     }
    //                     fromLane++;
    //                 }
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                 if (DEBUGCOND) {
    //                     if (unsatisfied != 0) {
    //                         std::cout << "     still unsatisfied modes from edge=" << incoming->getID() << " toEdge=" << currentOutgoing->getID() << " deadModes=" << getVehicleClassNames(unsatisfied) << "\n";
    //                     }
    //                 }
    // #endif
    //             }
    //         }
    //         // prevent dead-end bus and bicycle lanes (they were excluded by the ApproachingDivider)
    //         // and the bus/bicycle class might already be satisfied by other lanes
    //         recheckSpecialConnections(incoming, currentOutgoing, SVC_BUS);
    //         recheckSpecialConnections(incoming, currentOutgoing, SVC_BICYCLE);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_v_class_connections(&mut self, currentOutgoing: Ptr<NBEdge>) {
        todo!("port NBNode::recheckVClassConnections")
    }

    /// C++: `void NBNode::initRailSignalClasses(const NBNodeCont &)` [static]
    /// initialize signalized rail classes
    // void
    // NBNode::initRailSignalClasses(const NBNodeCont& nc) {
    //     myPermitUnsignalizedClasses = parseVehicleClasses(toString(OptionsCont::getOptions().getStringVector("railway.signal.permit-unsignalized")));
    //     myHaveRailSignalClasses = 0;
    //     for (auto it : nc) {
    //         const NBNode* n = it.second;
    //         if (n->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             for (const NBEdge* in : n->getIncomingEdges()) {
    //                 myHaveRailSignalClasses |= in->getPermissions();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_rail_signal_classes(nc: &NBNodeCont) {
        todo!("port NBNode::initRailSignalClasses")
    }

    /// C++: `void NBNode::setPriorityJunctionPriorities()` [private]
    /// sets the priorites in case of a priority junction
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_priority_junction_priorities(&mut self) {
        todo!("port NBNode::setPriorityJunctionPriorities")
    }

    /// C++: `void NBNode::getEdgesThatApproach(NBEdge *, EdgeVector &)` [private]
    /// returns a list of edges which are connected to the given outgoing edge
    // void
    // NBNode::getEdgesThatApproach(NBEdge* currentOutgoing, EdgeVector& approaching) {
    //     // get the position of the node to get the approaching nodes of
    //     EdgeVector::const_iterator i = std::find(myAllEdges.begin(),
    //                                    myAllEdges.end(), currentOutgoing);
    //     // get the first possible approaching edge
    //     NBContHelper::nextCW(myAllEdges, i);
    //     // go through the list of edges clockwise and add the edges
    //     approaching.clear();
    //     for (; *i != currentOutgoing;) {
    //         // check only incoming edges
    //         if ((*i)->getToNode() == this && (*i)->getTurnDestination() != currentOutgoing) {
    //             std::vector<int> connLanes = (*i)->getConnectionLanes(currentOutgoing);
    //             if (connLanes.size() != 0) {
    //                 approaching.push_back(*i);
    //             }
    //         }
    //         NBContHelper::nextCW(myAllEdges, i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_edges_that_approach(&mut self, currentOutgoing: Ptr<NBEdge>, approaching: &mut EdgeVector) {
        todo!("port NBNode::getEdgesThatApproach")
    }

    /// C++: `void NBNode::replaceInConnectionProhibitions(NBEdge *, NBEdge *, int, int)` [private]
    /// replace incoming connections prohibitions
    // void
    // NBNode::replaceInConnectionProhibitions(NBEdge* which, NBEdge* by,
    //                                         int whichLaneOff, int byLaneOff) {
    //     // replace in keys
    //     NBConnectionProhibits::iterator j = myBlockedConnections.begin();
    //     while (j != myBlockedConnections.end()) {
    //         bool changed = false;
    //         NBConnection c = (*j).first;
    //         if (c.replaceFrom(which, whichLaneOff, by, byLaneOff)) {
    //             changed = true;
    //         }
    //         if (c.replaceTo(which, whichLaneOff, by, byLaneOff)) {
    //             changed = true;
    //         }
    //         if (changed) {
    //             myBlockedConnections[c] = (*j).second;
    //             myBlockedConnections.erase(j);
    //             j = myBlockedConnections.begin();
    //         } else {
    //             j++;
    //         }
    //     }
    //     // replace in values
    //     for (j = myBlockedConnections.begin(); j != myBlockedConnections.end(); j++) {
    //         NBConnectionVector& prohibiting = (*j).second;
    //         for (NBConnectionVector::iterator k = prohibiting.begin(); k != prohibiting.end(); k++) {
    //             NBConnection& sprohibiting = *k;
    //             sprohibiting.replaceFrom(which, whichLaneOff, by, byLaneOff);
    //             sprohibiting.replaceTo(which, whichLaneOff, by, byLaneOff);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_in_connection_prohibitions(&mut self, which: Ptr<NBEdge>, by: Ptr<NBEdge>, whichLaneOff: i32, byLaneOff: i32) {
        todo!("port NBNode::replaceInConnectionProhibitions")
    }

    /// C++: `void NBNode::remapRemoved(NBTrafficLightLogicCont &, NBEdge *, const EdgeVector &, const EdgeVector &)` [private]
    /// remap removed
    // void
    // NBNode::remapRemoved(NBTrafficLightLogicCont& tc,
    //                      NBEdge* removed, const EdgeVector& incoming,
    //                      const EdgeVector& outgoing) {
    //     assert(find(incoming.begin(), incoming.end(), removed) == incoming.end());
    //     bool changed = true;
    //     while (changed) {
    //         changed = false;
    //         NBConnectionProhibits blockedConnectionsTmp = myBlockedConnections;
    //         NBConnectionProhibits blockedConnectionsNew;
    //         // remap in connections
    //         for (NBConnectionProhibits::iterator i = blockedConnectionsTmp.begin(); i != blockedConnectionsTmp.end(); i++) {
    //             const NBConnection& blocker = (*i).first;
    //             const NBConnectionVector& blocked = (*i).second;
    //             // check the blocked connections first
    //             // check whether any of the blocked must be changed
    //             bool blockedChanged = false;
    //             NBConnectionVector newBlocked;
    //             NBConnectionVector::const_iterator j;
    //             for (j = blocked.begin(); j != blocked.end(); j++) {
    //                 const NBConnection& sblocked = *j;
    //                 if (sblocked.getFrom() == removed || sblocked.getTo() == removed) {
    //                     blockedChanged = true;
    //                 }
    //             }
    //             // adapt changes if so
    //             for (j = blocked.begin(); blockedChanged && j != blocked.end(); j++) {
    //                 const NBConnection& sblocked = *j;
    //                 if (sblocked.getFrom() == removed && sblocked.getTo() == removed) {
    //                     /*                    for(EdgeVector::const_iterator k=incoming.begin(); k!=incoming.end(); k++) {
    //                     !!!                        newBlocked.push_back(NBConnection(*k, *k));
    //                                         }*/
    //                 } else if (sblocked.getFrom() == removed) {
    //                     assert(sblocked.getTo() != removed);
    //                     for (EdgeVector::const_iterator k = incoming.begin(); k != incoming.end(); k++) {
    //                         newBlocked.push_back(NBConnection(*k, sblocked.getTo()));
    //                     }
    //                 } else if (sblocked.getTo() == removed) {
    //                     assert(sblocked.getFrom() != removed);
    //                     for (EdgeVector::const_iterator k = outgoing.begin(); k != outgoing.end(); k++) {
    //                         newBlocked.push_back(NBConnection(sblocked.getFrom(), *k));
    //                     }
    //                 } else {
    //                     newBlocked.push_back(NBConnection(sblocked.getFrom(), sblocked.getTo()));
    //                 }
    //             }
    //             if (blockedChanged) {
    //                 blockedConnectionsNew[blocker] = newBlocked;
    //                 changed = true;
    //             }
    //             // if the blocked were kept
    //             else {
    //                 if (blocker.getFrom() == removed && blocker.getTo() == removed) {
    //                     changed = true;
    //                     /*                    for(EdgeVector::const_iterator k=incoming.begin(); k!=incoming.end(); k++) {
    //                     !!!                        blockedConnectionsNew[NBConnection(*k, *k)] = blocked;
    //                                         }*/
    //                 } else if (blocker.getFrom() == removed) {
    //                     assert(blocker.getTo() != removed);
    //                     changed = true;
    //                     for (EdgeVector::const_iterator k = incoming.begin(); k != incoming.end(); k++) {
    //                         blockedConnectionsNew[NBConnection(*k, blocker.getTo())] = blocked;
    //                     }
    //                 } else if (blocker.getTo() == removed) {
    //                     assert(blocker.getFrom() != removed);
    //                     changed = true;
    //                     for (EdgeVector::const_iterator k = outgoing.begin(); k != outgoing.end(); k++) {
    //                         blockedConnectionsNew[NBConnection(blocker.getFrom(), *k)] = blocked;
    //                     }
    //                 } else {
    //                     blockedConnectionsNew[blocker] = blocked;
    //                 }
    //             }
    //         }
    //         myBlockedConnections = blockedConnectionsNew;
    //     }
    //     // remap in traffic lights
    //     tc.remapRemoved(removed, incoming, outgoing);
    // }
    #[allow(unused_variables)]
    pub fn remap_removed(&mut self, tc: &mut NBTrafficLightLogicCont, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
        todo!("port NBNode::remapRemoved")
    }

    /// C++: `bool NBNode::forbidsPedestriansAfter(std::vector<std::pair<NBEdge *, bool>>, int)` [private]
    /// return whether there is a non-sidewalk lane after the given index;
    // bool
    // NBNode::forbidsPedestriansAfter(std::vector<std::pair<NBEdge*, bool> > normalizedLanes, int startIndex) {
    //     for (int i = startIndex; i < (int)normalizedLanes.size(); ++i) {
    //         if (!normalizedLanes[i].second) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn forbids_pedestrians_after(&mut self, normalizedLanes: Vec<(Ptr<NBEdge>, bool)>, startIndex: i32) -> bool {
        todo!("port NBNode::forbidsPedestriansAfter")
    }

    /// C++: `EdgeVector NBNode::getEdgesSortedByAngleAtNodeCenter() const` [const, private]
    /// returns the list of all edges sorted clockwise by getAngleAtNodeToCenter
    // EdgeVector
    // NBNode::getEdgesSortedByAngleAtNodeCenter() const {
    //     EdgeVector result = myAllEdges;
    // #ifdef DEBUG_PED_STRUCTURES
    //     if (gDebugFlag1) {
    //         std::cout << "  angles:\n";
    //         for (EdgeVector::const_iterator it = result.begin(); it != result.end(); ++it) {
    //             std::cout << "    edge=" << (*it)->getID() << " edgeAngle=" << (*it)->getAngleAtNode(this) << " angleToShape=" << (*it)->getAngleAtNodeToCenter(this) << "\n";
    //         }
    //         std::cout << "  allEdges before: " << toString(result) << "\n";
    //     }
    // #endif
    //     sort(result.begin(), result.end(), NBContHelper::edge_by_angle_to_nodeShapeCentroid_sorter(this));
    //     // let the first edge in myAllEdges remain the first
    //     DEBUGCOUT(gDebugFlag1, "  allEdges sorted: " << toString(result) << "\n")
    //     rotate(result.begin(), std::find(result.begin(), result.end(), *myAllEdges.begin()), result.end());
    //     DEBUGCOUT(gDebugFlag1, "  allEdges rotated: " << toString(result) << "\n")
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_edges_sorted_by_angle_at_node_center(&self) -> EdgeVector {
        todo!("port NBNode::getEdgesSortedByAngleAtNodeCenter")
    }

    /// C++: `bool NBNode::isLongEnough(NBEdge *, double)` [static, private]
    /// check if is long enough
    // bool
    // NBNode::isLongEnough(NBEdge* out, double minLength) {
    //     double seen = out->getLoadedLength();
    //     while (seen < minLength) {
    //         // advance along trivial continuations
    //         if (out->getToNode()->getOutgoingEdges().size() != 1
    //                 || out->getToNode()->getIncomingEdges().size() != 1) {
    //             return false;
    //         } else {
    //             out = out->getToNode()->getOutgoingEdges()[0];
    //             seen += out->getLoadedLength();
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_long_enough(out: Ptr<NBEdge>, minLength: f64) -> bool {
        todo!("port NBNode::isLongEnough")
    }

    /// C++: `void NBNode::removeJoinedTrafficLights()` [private]
    /// remove all traffic light definitions that are part of a joined tls
    // void
    // NBNode::removeJoinedTrafficLights() {
    //     std::set<NBTrafficLightDefinition*> trafficLights = myTrafficLights; // make a copy because we will modify the original
    //     for (std::set<NBTrafficLightDefinition*>::const_iterator i = trafficLights.begin(); i != trafficLights.end(); ++i) {
    //         // if this is the only controlled node we keep the tlDef as it is to generate a warning later
    //         if ((*i)->getNodes().size() > 1) {
    //             myTrafficLights.erase(*i);
    //             (*i)->removeNode(this);
    //             (*i)->setParticipantsInformation();
    //             (*i)->setTLControllingInformation();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_joined_traffic_lights(&mut self) {
        todo!("port NBNode::removeJoinedTrafficLights")
    }

    /// C++: `void NBNode::displaceShapeAtWidthChange(const NBEdge *, const NBEdge::Connection &, PositionVector &, PositionVector &) const` [const, private]
    /// displace lane shapes to account for change in lane width at this node
    // void
    // NBNode::displaceShapeAtWidthChange(const NBEdge* from, const NBEdge::Connection& con,
    //                                    PositionVector& fromShape, PositionVector& toShape) const {
    //     if (isConstantWidthTransition()) {
    //         // displace shapes
    //         NBEdge* in = myIncomingEdges[0];
    //         NBEdge* out = myOutgoingEdges[0];
    //         double outCenter = out->getLaneWidth(con.toLane) / 2;
    //         for (int i = 0; i < con.toLane; ++i) {
    //             outCenter += out->getLaneWidth(i);
    //         }
    //         double inCenter = in->getLaneWidth(con.fromLane) / 2;
    //         for (int i = 0; i < con.fromLane; ++i) {
    //             inCenter += in->getLaneWidth(i);
    //         }
    //         //std::cout << "displaceShapeAtWidthChange inCenter=" << inCenter << " outCenter=" << outCenter << "\n";
    //         try {
    //             if (in->getNumLanes() > out->getNumLanes()) {
    //                 // shift toShape so the internal lane ends straight at the displaced entry point
    //                 toShape.move2side(outCenter - inCenter);
    //             } else {
    //                 // shift fromShape so the internal lane starts straight at the displaced exit point
    //                 fromShape.move2side(inCenter - outCenter);
    //
    //             }
    //         } catch (InvalidArgument&) { }
    //     } else {
    //         SVCPermissions fromP = from->getPermissions(con.fromLane);
    //         SVCPermissions toP = con.toEdge->getPermissions(con.toLane);
    //         if ((fromP & toP) == SVC_BICYCLE && (fromP | toP) != SVC_BICYCLE) {
    //             double shift = (from->getLaneWidth(con.fromLane) - con.toEdge->getLaneWidth(con.toLane)) / 2;
    //             if (toP == SVC_BICYCLE) {
    //                 // let connection to dedicated bicycle lane start on the right side of a mixed lane for straight an right-going connections
    //                 // (on the left side for left turns)
    //                 // XXX indirect left turns should also start on the right side
    //                 LinkDirection dir = getDirection(from, con.toEdge);
    //                 if ((dir == LinkDirection::LEFT) || (dir == LinkDirection::PARTLEFT) || (dir == LinkDirection::TURN)) {
    //                     fromShape.move2side(-shift);
    //                 } else {
    //                     fromShape.move2side(shift);
    //                 }
    //             } else if (fromP == SVC_BICYCLE) {
    //                 // let connection from dedicated bicycle end on the right side of a mixed lane
    //                 toShape.move2side(-shift);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn displace_shape_at_width_change(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, fromShape: &mut PositionVector, toShape: &mut PositionVector) {
        todo!("port NBNode::displaceShapeAtWidthChange")
    }

    /// C++: `bool NBNode::includes(const std::set<const NBEdge *, ComparatorIdLess> &, const std::set<const NBEdge *, ComparatorIdLess> &)` [static, private]
    /// returns whether sub is a subset of super
    // bool
    // NBNode::includes(const std::set<const NBEdge*, ComparatorIdLess>& super,
    //                  const std::set<const NBEdge*, ComparatorIdLess>& sub) {
    //     // for some reason std::include does not work reliably
    //     for (const NBEdge* e : sub) {
    //         if (super.count(const_cast<NBEdge*>(e)) == 0) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn includes(super_: &BTreeSet<Ptr<NBEdge>>, sub: &BTreeSet<Ptr<NBEdge>>) -> bool {
        todo!("port NBNode::includes")
    }

    /// C++: `NBEdge * NBNode::getNextCompatibleOutgoing(const NBEdge *, SVCPermissions, EdgeVector::const_iterator, bool) const` [const, private]
    // NBEdge*
    // NBNode::getNextCompatibleOutgoing(const NBEdge* incoming, SVCPermissions vehPerm, EdgeVector::const_iterator itOut, bool clockwise) const {
    //     EdgeVector::const_iterator i = itOut;
    //     while (*i != incoming) {
    //         if (clockwise) {
    //             NBContHelper::nextCW(myAllEdges, i);
    //         } else {
    //             NBContHelper::nextCCW(myAllEdges, i);
    //         }
    //         if ((*i)->getFromNode() != this) {
    //             // only look for outgoing edges
    //             // @note we use myAllEdges to stop at the incoming edge
    //             continue;
    //         }
    //         if (incoming->isTurningDirectionAt(*i)) {
    //             return nullptr;
    //         }
    //         if ((vehPerm & (*i)->getPermissions()) != 0 || vehPerm == 0) {
    //             return *i;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_compatible_outgoing(&self, incoming: Ptr<NBEdge>, vehPerm: SVCPermissions, start: Unmapped /* EdgeVector::const_iterator */, clockwise: bool) -> Ptr<NBEdge> {
        todo!("port NBNode::getNextCompatibleOutgoing")
    }

    /// C++: `void NBNode::getReduction(const NBEdge *, const NBEdge *, int &, int &, int &, int &, int &) const` [const, private]
    /// get the reduction in driving lanes at this junction
    // void
    // NBNode::getReduction(const NBEdge* in, const NBEdge* out, int& inOffset, int& inEnd, int& outOffset, int& outEnd, int& reduction) const {
    //     inOffset = MAX2(0, in->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //     outOffset = MAX2(0, out->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //     inEnd = in->getFirstNonPedestrianLaneIndex(BACKWARD, true) + 1;
    //     outEnd = out->getFirstNonPedestrianLaneIndex(BACKWARD, true) + 1;
    //     reduction = (inEnd - inOffset) - (outEnd - outOffset);
    // }
    #[allow(unused_variables)]
    pub fn get_reduction(&self, in_: Ptr<NBEdge>, out: Ptr<NBEdge>, inOffset: &mut i32, inEnd: &mut i32, outOffset: &mut i32, outEnd: &mut i32, reduction: &mut i32) {
        todo!("port NBNode::getReduction")
    }

    /// C++: `SVCPermissions NBNode::findToLaneForPermissions(NBEdge *, int, NBEdge *, SVCPermissions)` [private]
    /// helper function to add connections for unsatisfied modes
    // SVCPermissions
    // NBNode::findToLaneForPermissions(NBEdge* currentOutgoing, int fromLane, NBEdge* incoming, SVCPermissions unsatisfied) {
    //     for (int toLane = 0; toLane < currentOutgoing->getNumLanes(); ++toLane) {
    //         const SVCPermissions satisfied = incoming->getPermissions(fromLane) & currentOutgoing->getPermissions(toLane) & unsatisfied;
    //         if (satisfied != 0 && !incoming->getLaneStruct(fromLane).connectionsDone) {
    //             if (incoming->hasConnectionTo(currentOutgoing, toLane)
    //                     && unsatisfied == SVC_TRAM
    //                     && incoming->getPermissions(fromLane) == currentOutgoing->getPermissions(toLane)) {
    //                 // avoid double tram connection by shifting an existing connection
    //                 for (auto con : incoming->getConnections()) {
    //                     if (con.toEdge == currentOutgoing && con.toLane == toLane) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                         if (DEBUGCOND) {
    //                             std::cout << "  shifting connection from=" << con.fromLane << " to=" << currentOutgoing->getID() << "_" << toLane << ": newFromLane=" << fromLane << " satisfies=" << getVehicleClassNames(satisfied) << "\n";
    //                         }
    // #endif
    //                         incoming->getConnectionRef(con.fromLane, con.toEdge, toLane).fromLane = fromLane;
    //                         unsatisfied &= ~satisfied;
    //                         break;
    //                     }
    //                 }
    //             } else {
    //                 // other modes (i.e. bus) can fix lane permissions NBPTLineCont::fixPermissions but do not wish to create parallel tram tracks here
    //                 bool mayUseSameDestination = unsatisfied == SVC_TRAM || (unsatisfied & SVC_PASSENGER) != 0;
    //                 incoming->setConnection((int)fromLane, currentOutgoing, toLane, NBEdge::Lane2LaneInfoType::COMPUTED, mayUseSameDestination);
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                 if (DEBUGCOND) {
    //                     std::cout << "  new connection from=" << fromLane << " to=" << currentOutgoing->getID() << "_" << toLane << " satisfies=" << getVehicleClassNames(satisfied) << "\n";
    //                 }
    // #endif
    //                 unsatisfied &= ~satisfied;
    //             }
    //         }
    //     }
    //     return unsatisfied;
    // }
    #[allow(unused_variables)]
    pub fn find_to_lane_for_permissions(&mut self, currentOutgoing: Ptr<NBEdge>, fromLane: i32, incoming: Ptr<NBEdge>, unsatisfied: SVCPermissions) -> SVCPermissions {
        todo!("port NBNode::findToLaneForPermissions")
    }

    /// C++: `int NBNode::addedLanesRight(NBEdge *, int) const` [const, private]
    /// check whether this edge has extra lanes on the right side
    // int
    // NBNode::addedLanesRight(NBEdge* out, int addedLanes) const {
    //     if (out->isOffRamp()) {
    //         return addedLanes;
    //     }
    //     NBNode* to = out->getToNode();
    //     // check whether a right lane ends
    //     if (to->getIncomingEdges().size() == 1
    //             && to->getOutgoingEdges().size() == 1) {
    //         int inOffset, inEnd, outOffset, outEnd, reduction;
    //         to->getReduction(out, to->getOutgoingEdges()[0], inOffset, inEnd, outOffset, outEnd, reduction);
    //
    //         if (reduction > 0) {
    //             return reduction;
    //         }
    //     }
    //     // check for the presence of right and left turns at the next intersection
    //     int outLanesRight = 0;
    //     int outLanesLeft = 0;
    //     int outLanesStraight = 0;
    //     for (NBEdge* succ : to->getOutgoingEdges()) {
    //         if (out->isConnectedTo(succ)) {
    //             const int outOffset = MAX2(0, succ->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //             const int usableLanes = succ->getNumLanes() - outOffset;
    //             LinkDirection dir = to->getDirection(out, succ);
    //             if (dir == LinkDirection::STRAIGHT) {
    //                 outLanesStraight += usableLanes;
    //             } else if (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT) {
    //                 outLanesRight += usableLanes;
    //             } else {
    //                 outLanesLeft += usableLanes;
    //             }
    //         }
    //     }
    //     const int outOffset = MAX2(0, out->getFirstNonPedestrianNonBicycleLaneIndex(FORWARD, true));
    //     const int outEnd = out->getFirstNonPedestrianLaneIndex(BACKWARD, true) + 1;
    //     const int usableLanes = outEnd - outOffset;
    //     int addedTurnLanes = MIN3(
    //                              addedLanes,
    //                              MAX2(0, usableLanes - outLanesStraight),
    //                              outLanesRight + outLanesLeft);
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "out=" << out->getID() << " usableLanes=" << usableLanes << " addedTurnLanes=" << addedTurnLanes << " addedLanes=" << addedLanes << " outLanesStraight=" << outLanesStraight << " outLanesLeft=" << outLanesLeft << " outLanesRight=" << outLanesRight << "\n";
    //     }
    // #endif
    //     if (outLanesLeft == 0) {
    //         return addedTurnLanes;
    //     } else {
    //         return MIN2(addedTurnLanes / 2, outLanesRight);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn added_lanes_right(&self, out: Ptr<NBEdge>, addedLanes: i32) -> i32 {
        todo!("port NBNode::addedLanesRight")
    }

    /// C++: `bool NBNode::isStraighter(const NBEdge *const, const double, const SVCPermissions, const int, const NBEdge *const) const` [const, private]
    /// check whether the candidate edge is more likely to be the straight continuation
    // bool
    // NBNode::isStraighter(const NBEdge* const incoming, const double angle, const SVCPermissions vehPerm, const int modeLanes, const NBEdge* const candidate) const {
    //     if (candidate != nullptr) {
    //         const double candAngle = NBHelpers::normRelAngle(incoming->getAngleAtNode(this), candidate->getAngleAtNode(this));
    //         // they are too similar it does not matter
    //         if (fabs(angle - candAngle) < 5.) {
    //             return false;
    //         }
    //         // the other edge is at least 5 degree straighter
    //         if (fabs(candAngle) < fabs(angle) - 5.) {
    //             return true;
    //         }
    //         if (fabs(angle) < fabs(candAngle) - 5.) {
    //             return false;
    //         }
    //         if (fabs(candAngle) < 44.) {
    //             // the lane count for the same modes is larger
    //             const int candModeLanes = candidate->getNumLanesThatAllow(vehPerm);
    //             if (candModeLanes > modeLanes) {
    //                 return true;
    //             }
    //             if (candModeLanes < modeLanes) {
    //                 return false;
    //             }
    //             // we would create a left turn
    //             if (candAngle < 0 && angle > 0) {
    //                 return true;
    //             }
    //             if (angle < 0 && candAngle > 0) {
    //                 return false;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_straighter(&self, incoming: Ptr<NBEdge>, angle: f64, vehPerm: SVCPermissions, modeLanes: i32, candidate: Ptr<NBEdge>) -> bool {
        todo!("port NBNode::isStraighter")
    }

    /// C++: `EdgeVector NBNode::getPassengerEdges(bool) const` [const, private]
    /// return edges that permit passengers (either incoming or outgoing)
    // EdgeVector
    // NBNode::getPassengerEdges(bool incoming) const {
    //     EdgeVector result;
    //     for (NBEdge* e : (incoming ? myIncomingEdges : myOutgoingEdges)) {
    //         if ((e->getPermissions() & SVC_PASSENGER) != 0) {
    //             result.push_back(e);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_passenger_edges(&self, incoming: bool) -> EdgeVector {
        todo!("port NBNode::getPassengerEdges")
    }

    /// C++: `bool NBNode::isExplicitRailNoBidi(const NBEdge *, const NBEdge *)` [static, private]
    /// detect explict rail turns with potential geometry problem
    // bool
    // NBNode::isExplicitRailNoBidi(const NBEdge* incoming, const NBEdge* outgoing) {
    //     // assume explicit connections at sharp turn-arounds are either for reversal or due to a geometry glitch
    //     // (but should not have been guessed)
    //     // @note this function is also called from NBAlgorithms when there aren't any connections ready
    //     return (incoming->getStep() >= NBEdge::EdgeBuildingStep::LANES2LANES_RECHECK
    //             && isRailway(incoming->getPermissions())
    //             && isRailway(outgoing->getPermissions())
    //             && incoming->getBidiEdge() != outgoing);
    // }
    #[allow(unused_variables)]
    pub fn is_explicit_rail_no_bidi(incoming: Ptr<NBEdge>, outgoing: Ptr<NBEdge>) -> bool {
        todo!("port NBNode::isExplicitRailNoBidi")
    }

    /// C++: `PositionVector NBNode::cutAtShapes(const PositionVector &, const PositionVector &, const PositionVector &, const PositionVector &)` [private]
    /// geometry helper that cuts the first shape where bordered by the other two
    // PositionVector
    // NBNode::cutAtShapes(const PositionVector& cut, const PositionVector& border1, const PositionVector& border2, const PositionVector& def) {
    //     std::vector<double> is1 = cut.intersectsAtLengths2D(border1);
    //     std::vector<double> is2 = cut.intersectsAtLengths2D(border2);
    // #ifdef DEBUG_CROSSING_OUTLINE
    //     std::cout << "is1=" << is1 << " is2=" << is2 << " cut=" << cut << " border1=" << border1 << " border2=" << border2 << "\n";
    // #endif
    //     if (is1.size() == 0 && border1.size() == 2) {
    //         const double d1 = cut.distance2D(border1.front());
    //         const double d2 = cut.distance2D(border1.back());
    //         Position closer = d1 < d2 ? border1.front() : border1.back();
    //         double nOp = cut.nearest_offset_to_point2D(closer, false);
    // #ifdef DEBUG_CROSSING_OUTLINE
    //         std::cout << " closer=" << closer << " nOp=" << nOp << "\n";
    // #endif
    //         if (nOp <= 2 * POSITION_EPS && cut.back().distanceTo2D(closer) <= 2 * POSITION_EPS) {
    //             is1.push_back(cut.length2D());
    //         } else {
    //             is1.push_back(nOp);
    //         }
    //     }
    //     if (is2.size() == 0 && border2.size() == 2) {
    //         const double d1 = cut.distance2D(border2.front());
    //         const double d2 = cut.distance2D(border2.back());
    //         Position closer = d1 < d2 ? border2.front() : border2.back();
    //         double nOp = cut.nearest_offset_to_point2D(closer, false);
    //         if (nOp <= 2 * POSITION_EPS && cut.back().distanceTo2D(closer) <= 2 * POSITION_EPS) {
    //             is2.push_back(cut.length2D());
    //         } else {
    //             is2.push_back(nOp);
    //         }
    //     }
    //     if (is1.size() > 0 && is2.size() > 0) {
    //         double of1 = VectorHelper<double>::maxValue(is1);
    //         double of2 = VectorHelper<double>::minValue(is2);
    // #ifdef DEBUG_CROSSING_OUTLINE
    //         std::cout << " of1=" << of1 << " of2=" << of2 << "\n";
    // #endif
    //         if (of1 > of2) {
    //             of1 = VectorHelper<double>::maxValue(is2);
    //             of2 = VectorHelper<double>::minValue(is1);
    // #ifdef DEBUG_CROSSING_OUTLINE
    //             std::cout << " of1=" << of1 << " of2=" << of2 << "\n";
    // #endif
    //         }
    //         if (of1 > of2) {
    //             of2 = VectorHelper<double>::maxValue(is1);
    //             of1 = VectorHelper<double>::minValue(is2);
    // #ifdef DEBUG_CROSSING_OUTLINE
    //             std::cout << " of1=" << of1 << " of2=" << of2 << "\n";
    // #endif
    //         }
    //         assert(of1 <= of2);
    //         return cut.getSubpart(of1, of2);
    //     } else {
    //         return def;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cut_at_shapes(&mut self, cut: &PositionVector, border1: &PositionVector, border2: &PositionVector, def: &PositionVector) -> PositionVector {
        todo!("port NBNode::cutAtShapes")
    }

    /// C++: `void NBNode::patchOffset_pathAcrossStreet(double &)` [private]
    /// compute offset for centering path-across-street crossings
    // void
    // NBNode::patchOffset_pathAcrossStreet(double& offset) {
    //     if (myCrossings.size() == 1 && myAllEdges.size() >= 3) {
    //         EdgeVector nonPedIncoming;
    //         EdgeVector nonPedOutgoing;
    //         EdgeVector pedIncoming;
    //         EdgeVector pedOutgoing;
    //         for (NBEdge* e : getIncomingEdges()) {
    //             if (e->getPermissions() != SVC_PEDESTRIAN) {
    //                 nonPedIncoming.push_back(e);
    //             } else {
    //                 pedIncoming.push_back(e);
    //             }
    //         }
    //         for (NBEdge* e : getOutgoingEdges()) {
    //             if (e->getPermissions() != SVC_PEDESTRIAN) {
    //                 nonPedOutgoing.push_back(e);
    //             } else {
    //                 pedOutgoing.push_back(e);
    //             }
    //         }
    //         if (geometryLike(nonPedIncoming, nonPedOutgoing) && (pedIncoming.size() > 0 || pedOutgoing.size() > 0)) {
    //             double maxAngle = 0;
    //             const NBEdge* in = nonPedIncoming.front();
    //             const NBEdge* out = nonPedOutgoing.front();
    //             if (nonPedIncoming.size() == 1) {
    //                 maxAngle = fabs(NBHelpers::relAngle(in->getAngleAtNode(this), out->getAngleAtNode(this)));
    //             } else {
    //                 for (const NBEdge* const in2 : nonPedIncoming) {
    //                     double minAngle = 180;
    //                     for (const NBEdge* const out2 : nonPedOutgoing) {
    //                         double angle = fabs(NBHelpers::relAngle(in2->getAngleAtNode(this), out2->getAngleAtNode(this)));
    //                         if (angle < minAngle) {
    //                             minAngle = angle;
    //                             in = in2;
    //                             out = out2;
    //                         }
    //                     }
    //                     maxAngle = MAX2(maxAngle, minAngle);
    //                 }
    //             }
    //             // changing the offset only handles the simple case where the road stays straight
    //             if (maxAngle < 15) {
    //                 const int inLane = in->getFirstNonPedestrianLaneIndex(FORWARD);
    //                 const int outLane = out->getFirstNonPedestrianLaneIndex(FORWARD);
    //                 if (inLane >= 0 && outLane >= 0) {
    //                     const Position& p0 = in->getLaneShape(inLane).back();
    //                     const Position& p1 = out->getLaneShape(outLane).front();
    //                     PositionVector road;
    //                     road.push_back(p0);
    //                     road.push_back(p1);
    //                     Position mid = (p0 + p1) / 2;
    //                     double maxPathDist = 0;
    //                     for (NBEdge* e : pedIncoming) {
    //                         const Position roadPos = road.positionAtOffset2D(road.nearest_offset_to_point2D(e->getLaneShape(0).back()));
    //                         maxPathDist = MAX2(maxPathDist, mid.distanceTo2D(roadPos));
    //                     }
    //                     for (NBEdge* e : pedOutgoing) {
    //                         const Position roadPos = road.positionAtOffset2D(road.nearest_offset_to_point2D(e->getLaneShape(0).front()));
    //                         maxPathDist = MAX2(maxPathDist, mid.distanceTo2D(roadPos));
    //                     }
    //                     // if the junction is stretched, the crossing should stay close to the paths
    //                     if (maxPathDist < MAX2(myCrossings.front()->width, 4.0)) {
    //                         offset = p0.distanceTo2D(p1) / 2;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn patch_offset_path_across_street(&mut self, offset: &mut f64) {
        if self.myCrossings.len() == 1 && self.myAllEdges.len() >= 3 {
            let mut nonPedIncoming: EdgeVector = Vec::new();
            let mut nonPedOutgoing: EdgeVector = Vec::new();
            let mut pedIncoming: EdgeVector = Vec::new();
            let mut pedOutgoing: EdgeVector = Vec::new();
            for e in self.get_incoming_edges().iter() {
                if e.get_permissions(-1) != SVC_PEDESTRIAN {
                    nonPedIncoming.push(*e);
                } else {
                    pedIncoming.push(*e);
                }
            }
            for e in self.get_outgoing_edges().iter() {
                if e.get_permissions(-1) != SVC_PEDESTRIAN {
                    nonPedOutgoing.push(*e);
                } else {
                    pedOutgoing.push(*e);
                }
            }
            if NBNode::geometry_like_2(&nonPedIncoming, &nonPedOutgoing) && (pedIncoming.len() > 0 || pedOutgoing.len() > 0) {
                let mut maxAngle: f64 = 0.;
                let mut inE = nonPedIncoming[0];
                let mut outE = nonPedOutgoing[0];
                if nonPedIncoming.len() == 1 {
                    maxAngle = NBHelpers::rel_angle(inE.get_angle_at_node(Ptr::from_mut(self)), outE.get_angle_at_node(Ptr::from_mut(self))).abs();
                } else {
                    for in2 in nonPedIncoming.iter() {
                        let mut minAngle: f64 = 180.;
                        for out2 in nonPedOutgoing.iter() {
                            let angle = NBHelpers::rel_angle(in2.get_angle_at_node(Ptr::from_mut(self)), out2.get_angle_at_node(Ptr::from_mut(self))).abs();
                            if angle < minAngle {
                                minAngle = angle;
                                inE = *in2;
                                outE = *out2;
                            }
                        }
                        maxAngle = maxAngle.max(minAngle);
                    }
                }
                // changing the offset only handles the simple case where the road stays straight
                if maxAngle < 15. {
                    let inLane = inE.get_first_non_pedestrian_lane_index(NBNODE_FORWARD, false);
                    let outLane = outE.get_first_non_pedestrian_lane_index(NBNODE_FORWARD, false);
                    if inLane >= 0 && outLane >= 0 {
                        let p0 = *inE.get_lane_shape(inLane).back();
                        let p1 = *outE.get_lane_shape(outLane).front();
                        let mut road = PositionVector::default();
                        road.push(p0);
                        road.push(p1);
                        let mid = (p0 + p1) / 2.;
                        let mut maxPathDist: f64 = 0.;
                        for e in pedIncoming.iter() {
                            let roadPos = road.position_at_offset2d(road.nearest_offset_to_point2d(e.get_lane_shape(0).back(), true), 0., false);
                            maxPathDist = maxPathDist.max(mid.distance_to2d(&roadPos));
                        }
                        for e in pedOutgoing.iter() {
                            let roadPos = road.position_at_offset2d(road.nearest_offset_to_point2d(e.get_lane_shape(0).front(), true), 0., false);
                            maxPathDist = maxPathDist.max(mid.distance_to2d(&roadPos));
                        }
                        // if the junction is stretched, the crossing should stay close to the paths
                        if maxPathDist < self.myCrossings[0].width.max(4.0) {
                            *offset = p0.distance_to2d(&p1) / 2.;
                        }
                    }
                }
            }
        }
    }

    /// C++: `bool NBNode::unsignalizedOperation() const` [const, private]
    /// whether the given rail connections at this node may run in unsignalized (right-of-way) mode
    // bool
    // NBNode::unsignalizedOperation() const {
    //     SVCPermissions railClasses = 0;
    //     for (NBEdge* e : myIncomingEdges) {
    //         railClasses |= (e->getPermissions() & SVC_RAIL_CLASSES);
    //     }
    //     assert(railClasses != 0);
    //     return ((railClasses & myPermitUnsignalizedClasses) == railClasses
    //             && (railClasses & myHaveRailSignalClasses) == 0);
    // }
    #[allow(unused_variables)]
    pub fn unsignalized_operation(&self) -> bool {
        todo!("port NBNode::unsignalizedOperation")
    }

    /// C++: `void NBNode::recheckSpecialConnections(NBEdge *, NBEdge *, SVCPermissions)` [private]
    /// ensure connectivity for all special vClass
    // void
    // NBNode::recheckSpecialConnections(NBEdge* incoming, NBEdge* currentOutgoing, SVCPermissions svcSpecial) {
    //     // assume that left-turns and turn-arounds are better satisfied from lanes to the left
    //     const int specialTarget = currentOutgoing->getSpecialLane(svcSpecial);
    //     const LinkDirection dir = getDirection(incoming, currentOutgoing);
    //     if (incoming->getStep() <= NBEdge::EdgeBuildingStep::LANES2LANES_DONE
    //             && ((specialTarget >= 0 && dir != LinkDirection::TURN)
    //                 || dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT || dir == LinkDirection::STRAIGHT)) {
    //         bool builtConnection = false;
    //         for (int i = 0; i < (int)incoming->getNumLanes(); i++) {
    //             if (incoming->getPermissions(i) == svcSpecial
    //                     && incoming->getConnectionsFromLane(i, currentOutgoing).size() == 0) {
    //                 // find a dedicated bike lane as target
    //                 if (specialTarget >= 0) {
    //                     incoming->setConnection(i, currentOutgoing, specialTarget, NBEdge::Lane2LaneInfoType::COMPUTED);
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                     if (DEBUGCOND) {
    //                         std::cout << "  extra " << getVehicleClassNames(svcSpecial) << " connection from=" << incoming->getLaneID(i) << " (dedicated) to=" << currentOutgoing->getLaneID(specialTarget)  << "\n";
    //                     }
    // #endif
    //                     builtConnection = true;
    //                 } else {
    //                     // do not create turns that create a conflict with neighboring lanes
    //                     if (avoidConfict(incoming, currentOutgoing, svcSpecial, dir, i)) {
    //                         continue;
    //                     }
    //                     // use any lane that allows the special class
    //                     for (int i2 = 0; i2 < (int)currentOutgoing->getNumLanes(); i2++) {
    //                         if ((currentOutgoing->getPermissions(i2) & svcSpecial) != 0) {
    //                             // possibly a double-connection
    //                             const bool allowDouble = (incoming->getPermissions(i) == svcSpecial
    //                                                       && (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT || dir == LinkDirection::STRAIGHT));
    //                             incoming->setConnection(i, currentOutgoing, i2, NBEdge::Lane2LaneInfoType::COMPUTED, allowDouble);
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                             if (DEBUGCOND) {
    //                                 std::cout << "  extra " << getVehicleClassNames(svcSpecial) << " connection from=" << incoming->getLaneID(i) << " to=" << currentOutgoing->getLaneID(i2)  << "\n";
    //                             }
    // #endif
    //                             builtConnection = true;
    //                             break;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         if (!builtConnection && specialTarget >= 0
    //                 && incoming->getConnectionsFromLane(-1, currentOutgoing, specialTarget).size() == 0) {
    //             // find origin lane that allows bicycles
    //             int start = 0;
    //             int end = incoming->getNumLanes();
    //             int inc = 1;
    //             if (dir == LinkDirection::TURN || dir == LinkDirection::LEFT || dir == LinkDirection::PARTLEFT) {
    //                 std::swap(start, end);
    //                 inc = -1;
    //             }
    //             for (int i = start; i < end; i += inc) {
    //                 if ((incoming->getPermissions(i) & svcSpecial) != 0) {
    //                     incoming->setConnection(i, currentOutgoing, specialTarget, NBEdge::Lane2LaneInfoType::COMPUTED);
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                     if (DEBUGCOND) {
    //                         std::cout << "  extra " << getVehicleClassNames(svcSpecial) << " connection from=" << incoming->getLaneID(i) << " (final) to=" << currentOutgoing->getLaneID(specialTarget)  << "\n";
    //                     }
    // #endif
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_special_connections(&mut self, incoming: Ptr<NBEdge>, currentOutgoing: Ptr<NBEdge>, svcSpecial: SVCPermissions) {
        todo!("port NBNode::recheckSpecialConnections")
    }

    /// C++: `bool NBNode::avoidConfict(NBEdge *, NBEdge *, SVCPermissions, LinkDirection, int)` [private]
    /// helper function for recheckSpecialConnections
    // bool
    // NBNode::avoidConfict(NBEdge* incoming, NBEdge* currentOutgoing, SVCPermissions svcSpecial, LinkDirection dir, int i) {
    //     for (const auto& c : incoming->getConnections()) {
    //         if (incoming->getPermissions(c.fromLane) == svcSpecial && c.toEdge == currentOutgoing) {
    //             return true;
    //         }
    //     }
    //     if (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT) {
    //         for (const auto& c : incoming->getConnections()) {
    //             if (c.fromLane < i && (c.toEdge != currentOutgoing || incoming->getPermissions(c.fromLane) == svcSpecial)) {
    //                 return true;
    //             }
    //         }
    //     } else if (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT) {
    //         for (const auto& c : incoming->getConnections()) {
    //             if (c.fromLane > i && (c.toEdge != currentOutgoing || incoming->getPermissions(c.fromLane) == svcSpecial)) {
    //                 return true;
    //             }
    //         }
    //     } else if (svcSpecial != SVC_BICYCLE && dir == LinkDirection::STRAIGHT) {
    //         for (const auto& c : incoming->getConnections()) {
    //             const LinkDirection dir2 = getDirection(incoming, c.toEdge);
    //             if (c.fromLane < i && (dir2 == LinkDirection::LEFT || dir2 == LinkDirection::PARTLEFT)) {
    //                 return true;
    //             } else if (c.fromLane > i && (dir2 == LinkDirection::RIGHT || dir2 == LinkDirection::PARTRIGHT)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn avoid_confict(&mut self, incoming: Ptr<NBEdge>, currentOutgoing: Ptr<NBEdge>, svcSpecial: SVCPermissions, dir: LinkDirection, i: i32) -> bool {
        todo!("port NBNode::avoidConfict")
    }

    /// C++: `void NBNode::NBNode(const NBNode &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &NBNode) -> Self {
        todo!("port NBNode::NBNode")
    }

    /// C++: `NBNode & NBNode::operator=(const NBNode &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBNode) -> &mut NBNode {
        todo!("port NBNode::operator=")
    }

}

/// A definition of a pedestrian walking area
/// C++ class `NBNode::WalkingArea` (netbuild/NBNode.h)
pub struct NBNode_WalkingArea {
    /// the (edge)-id of this walkingArea
    pub id: String,
    /// This lane's width
    pub width: f64,
    /// This lane's width
    pub length: f64,
    /// The polygonal shape
    pub shape: PositionVector,
    /// the lane-id of the next crossing(s)
    pub nextCrossings: Vec<String>,
    /// the lane-id of the previous crossing(s)
    pub prevCrossings: Vec<String>,
    /// the lane-id of the next sidewalk lane or ""
    pub nextSidewalks: Vec<String>,
    /// the lane-id of the previous sidewalk lane or ""
    pub prevSidewalks: Vec<String>,
    /// whether this walkingArea has a custom shape
    pub hasCustomShape: bool,
    /// minimum number of edges crossed by nextCrossings
    pub minNextCrossingEdges: i32,
    /// minimum number of edges crossed by incoming crossings
    pub minPrevCrossingEdges: i32,
    /// reference edges that uniquely identify this walkingarea
    pub refEdges: BTreeSet<Ptr<NBEdge>>, // C++: std::set<const NBEdge *, ComparatorIdLess>
}

impl NBNode_WalkingArea {
    /// C++: `void WalkingArea::WalkingArea(const std::string &, double)`
    /// constructor
    // WalkingArea(const std::string& _id, double _width) :
    //             id(_id),
    //             width(_width) {
    //         }
    #[allow(unused_variables)]
    pub fn new(_id: &str, _width: f64) -> Self {
        NBNode_WalkingArea {
            id: _id.to_string(),
            width: _width,
            length: utils::common::std_defs::INVALID_DOUBLE,
            shape: PositionVector::default(),
            nextCrossings: Vec::new(),
            prevCrossings: Vec::new(),
            nextSidewalks: Vec::new(),
            prevSidewalks: Vec::new(),
            hasCustomShape: false,
            minNextCrossingEdges: i32::MAX,
            minPrevCrossingEdges: i32::MAX,
            refEdges: BTreeSet::new(),
        }
    }

}

/// C++ class `NBNode::WalkingAreaCustomShape` (netbuild/NBNode.h)
pub struct NBNode_WalkingAreaCustomShape {
    pub edges: BTreeSet<Ptr<NBEdge>>, // C++: std::set<const NBEdge *, ComparatorIdLess>
    pub shape: PositionVector,
    pub width: f64,
}

impl NBNode_WalkingAreaCustomShape {
}

/// Sorts outgoing before incoming edges
/// C++ class `NBNode::edge_by_direction_sorter` (netbuild/NBNode.h)
pub struct NBNode_edge_by_direction_sorter {
    /// The node to compute the relative angle of
    pub myNode: Ptr<NBNode>,
}

impl NBNode_edge_by_direction_sorter {
    /// C++: `void edge_by_direction_sorter::edge_by_direction_sorter(NBNode *)`
    /// constructor
    // explicit edge_by_direction_sorter(NBNode* n) : myNode(n) {}
    #[allow(unused_variables)]
    pub fn new(n: Ptr<NBNode>) -> Self {
        todo!("port edge_by_direction_sorter::edge_by_direction_sorter")
    }

    /// C++: `int edge_by_direction_sorter::operator()(NBEdge *, NBEdge *) const` [const]
    /// operator of selection
    // int operator()(NBEdge* e1, NBEdge* e2) const {
    //             UNUSED_PARAMETER(e2);
    //             return e1->getFromNode() == myNode;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<NBEdge>, e2: Ptr<NBEdge>) -> i32 {
        todo!("port edge_by_direction_sorter::operator()")
    }

}

/// Used for sorting the cells by the begin time they describe
/// C++ class `NBNode::nodes_by_id_sorter` (netbuild/NBNode.h)
pub struct NBNode_nodes_by_id_sorter {
}

impl NBNode_nodes_by_id_sorter {
    /// C++: `void nodes_by_id_sorter::nodes_by_id_sorter()`
    /// Constructor
    // explicit nodes_by_id_sorter() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port nodes_by_id_sorter::nodes_by_id_sorter")
    }

    /// C++: `int nodes_by_id_sorter::operator()(NBNode *, NBNode *) const` [const]
    /// Comparing operator
    // int operator()(NBNode* n1, NBNode* n2) const {
    //             return n1->getID() < n2->getID();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, n1: Ptr<NBNode>, n2: Ptr<NBNode>) -> i32 {
        todo!("port nodes_by_id_sorter::operator()")
    }

}

