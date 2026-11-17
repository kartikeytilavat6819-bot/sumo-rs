//! Port of utils/common/MapMatcher.h
//!
//! C++ `MapMatcher<E, L, N>` is a template base of the route handlers. Its calls on E, L and N are
//! expressed by the traits `MapEdge`, `MapLane` and `MapNode` (implemented below for the ROEdge/MSEdge
//! families), and its virtual hooks (initLaneTree, retrieveEdge, getRNG) by `MapMatcherHost`.
//! determinism: the closest lane is chosen among lanes in address order (std::set<const Named*>),
//! the same order as the BTreeSet<Ptr<Named>> used here.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, clippy::all)]
use crate::prelude::*;
use crate::utils::named_rtree::{delete_named_rtree, new_named_rtree, NamedRTree};
use std::cell::Cell;
use std::marker::PhantomData;
use utils::common::msg_handler::{write_error, MsgHandler};
use utils::common::named::{Named, NamedStoringVisitor};
use utils::common::ptr::Ptr;
use utils::common::rand_helper::{with_default, SumoRNG};
use utils::common::std_defs::{g_precision, g_precision_geo};
use utils::common::sumo_vehicle_class::{SUMOVehicleClass, SVC_IGNORING};
use utils::geom::boundary::Boundary;
use utils::geom::geo_conv_helper::GeoConvHelper;
use utils::geom::position::Position;
use utils::geom::position_vector::PositionVector;

pub const JUNCTION_TAZ_MISSING_HELP: &str = "\nSet option '--junction-taz' or load a TAZ-file";

/// The edge operations used by MapMatcher (C++ calls on E)
pub trait MapEdge: Sized + 'static {
    type Node: MapNode;
    /// C++ `isInternal()`
    fn mm_is_internal(&self) -> bool;
    /// C++ `getSuccessors().front()`
    fn mm_successor_front(&self) -> Ptr<Self>;
    /// C++ `getID()`
    fn mm_id(&self) -> &str;
    /// C++ `getFromJunction()`
    fn mm_from_junction(&self) -> Ptr<Self::Node>;
    /// C++ `getToJunction()`
    fn mm_to_junction(&self) -> Ptr<Self::Node>;
    /// C++ `isTazConnector()`
    fn mm_is_taz_connector(&self) -> bool;
    /// C++ `getPredecessors()`
    fn mm_predecessors(&self) -> Vec<Ptr<Self>>;
    /// C++ `getSuccessors()` (vClass SVC_IGNORING)
    fn mm_successors(&self) -> Vec<Ptr<Self>>;
    /// C++ `getSuccessors(vClass)`
    fn mm_successors_for(&self, vClass: SUMOVehicleClass) -> Vec<Ptr<Self>>;
}

/// The junction operations used by MapMatcher (C++ calls on N)
pub trait MapNode {
    /// C++ `getPosition()`
    fn mm_position(&self) -> &Position;
    /// C++ `getID()`
    fn mm_id(&self) -> &str;
}

/// The lane operations used by MapMatcher (C++ calls on L)
pub trait MapLane<E>: 'static {
    /// C++ `getEdge()`
    fn mm_edge(&self) -> Ptr<E>;
    /// C++ `allowsVehicleClass(vClass)`
    fn mm_allows(&self, vClass: SUMOVehicleClass) -> bool;
    /// C++ `getShape()`
    fn mm_shape(&self) -> &PositionVector;
}

/// The virtual hooks of the MapMatcher base class, implemented by the derived route handler
pub trait MapMatcherHost<E> {
    /// C++ virtual `initLaneTree(NamedRTree*)`
    fn init_lane_tree(&self, tree: Ptr<NamedRTree>);
    /// C++ virtual `retrieveEdge(const std::string&)`
    fn retrieve_edge(&self, id: &str) -> Ptr<E>;
    /// C++ virtual `getRNG()`: nullptr unless overridden
    fn get_rng(&self) -> Ptr<SumoRNG> {
        Ptr::null()
    }
}

/// C++ `class MapMatcher` (utils/common/MapMatcher.h)
pub struct MapMatcher<E, L, N> {
    /// C++ `bool myMapMatchTAZ`
    pub myMapMatchTAZ: bool,
    /// C++ `NamedRTree* myLaneTree`, created lazily by getLaneTree()
    myLaneTree: Cell<Ptr<NamedRTree>>,
    myMapMatchJunctions: bool,
    myMapMatchingDistance: f64,
    myErrorOutput: Ptr<MsgHandler>,
    _types: PhantomData<(E, L, N)>,
}

impl<E, L, N> MapMatcher<E, L, N> {
    /// C++ `MapMatcher(bool matchJunctions, bool matchTAZ, double matchDistance, MsgHandler* errorOutput)`
    pub fn new(matchJunctions: bool, matchTAZ: bool, matchDistance: f64, errorOutput: Ptr<MsgHandler>) -> Self {
        MapMatcher {
            myMapMatchTAZ: matchTAZ,
            myLaneTree: Cell::new(Ptr::null()),
            myMapMatchJunctions: matchJunctions,
            myMapMatchingDistance: matchDistance,
            myErrorOutput: errorOutput,
            _types: PhantomData,
        }
    }
}

/// C++ `virtual ~MapMatcher() { delete myLaneTree; }`
impl<E, L, N> Drop for MapMatcher<E, L, N> {
    fn drop(&mut self) {
        delete_named_rtree(self.myLaneTree.get());
    }
}

impl<E, L, N> MapMatcher<E, L, N>
where
    E: MapEdge<Node = N>,
    L: MapLane<E>,
    N: MapNode,
{
    /// C++ `void MapMatcher::parseGeoEdges(const PositionVector&, bool, SUMOVehicleClass, std::vector<const E*>&, const std::string&, bool, bool&, bool)`
    pub fn parse_geo_edges(&self, host: &dyn MapMatcherHost<E>, positions: &PositionVector, geo: bool, vClass: SUMOVehicleClass, into: &mut Vec<Ptr<E>>, rid: &str, isFrom: bool, ok: &mut bool, forceEdge: bool) {
        if geo && !GeoConvHelper::with_final(|g| g.using_geo_projection()) {
            write_error("Cannot convert geo-positions because the network has no geo-reference");
            *ok = false;
            return;
        }
        for orig in positions.iter() {
            let mut pos = *orig;
            if geo {
                GeoConvHelper::with_final(|g| g.x2cartesian_const(&mut pos));
            }
            let mut dist = f64::min(10.0, self.myMapMatchingDistance);
            let mut best = self.get_closest_lane(host, &pos, vClass, dist);
            while best.is_null() && dist < self.myMapMatchingDistance {
                dist = f64::min(dist * 2.0, self.myMapMatchingDistance);
                best = self.get_closest_lane(host, &pos, vClass, dist);
            }
            if best.is_null() {
                let prec = if geo { g_precision_geo() } else { g_precision() };
                let msg = format!("No edge found near position {} within the route {}.", orig.to_string_prec(prec), rid);
                self.myErrorOutput.m().inform(&msg, true);
                *ok = false;
            } else {
                let mut bestEdge: Ptr<E> = best.m().mm_edge();
                while bestEdge.m().mm_is_internal() {
                    bestEdge = bestEdge.m().mm_successor_front();
                }
                if (self.myMapMatchJunctions || self.myMapMatchTAZ) && !forceEdge {
                    if self.myMapMatchTAZ {
                        bestEdge = self.get_taz(host, &pos, bestEdge, isFrom);
                    } else {
                        bestEdge = self.get_junction_taz(host, &pos, bestEdge, vClass, isFrom);
                    }
                    if !bestEdge.is_null() {
                        into.push(bestEdge);
                    } else {
                        *ok = false;
                    }
                } else {
                    // handle multiple via locations on the same edge without loops
                    if positions.len() == 1 || into.is_empty() || *into.last().unwrap() != bestEdge {
                        into.push(bestEdge);
                    }
                }
            }
        }
    }

    /// C++ `const L* MapMatcher::getClosestLane(const Position&, SUMOVehicleClass, double)`: find closest lane within distance or nullptr
    pub fn get_closest_lane(&self, host: &dyn MapMatcherHost<E>, pos: &Position, vClass: SUMOVehicleClass, distance: f64) -> Ptr<L> {
        let t = self.get_lane_tree(host);
        let mut b = Boundary::default();
        b.add(pos.x(), pos.y(), pos.z());
        b.grow(if distance < 0.0 { self.myMapMatchingDistance } else { distance });
        let cmin = [b.xmin() as f32, b.ymin() as f32];
        let cmax = [b.xmax() as f32, b.ymax() as f32];
        let mut lanes: BTreeSet<Ptr<Named>> = BTreeSet::new();
        t.m().search(&cmin, &cmax, &NamedStoringVisitor::new(&mut lanes), |d, c| d.add_to(c));
        // use closest
        let mut minDist = f64::MAX;
        let mut best: Ptr<L> = Ptr::null();
        for o in lanes.iter() {
            let cand: Ptr<L> = o.cast::<L>();
            if !cand.m().mm_allows(vClass) {
                continue;
            }
            let dist = cand.m().mm_shape().distance2d(pos, false);
            if dist < minDist {
                minDist = dist;
                best = cand;
            }
        }
        best
    }

    /// C++ `const E* MapMatcher::getJunctionTaz(const Position&, const E*, SUMOVehicleClass, bool)`: find closest junction taz given the closest edge
    pub fn get_junction_taz(&self, host: &dyn MapMatcherHost<E>, pos: &Position, closestEdge: Ptr<E>, vClass: SUMOVehicleClass, isFrom: bool) -> Ptr<E> {
        if closestEdge.is_null() {
            return Ptr::null();
        }
        let fromJunction = closestEdge.m().mm_from_junction();
        let toJunction = closestEdge.m().mm_to_junction();
        let fromCloser = fromJunction.m().mm_position().distance_squared_to2d(pos) < toJunction.m().mm_position().distance_squared_to2d(pos);
        let fromSource = host.retrieve_edge(&format!("{}-source", fromJunction.m().mm_id()));
        let fromSink = host.retrieve_edge(&format!("{}-sink", fromJunction.m().mm_id()));
        let toSource = host.retrieve_edge(&format!("{}-source", toJunction.m().mm_id()));
        let toSink = host.retrieve_edge(&format!("{}-sink", toJunction.m().mm_id()));
        if fromSource.is_null() || fromSink.is_null() {
            let msg = format!("Junction-taz '{}' not found when mapping position {}.{}", fromJunction.m().mm_id(), pos.to_string_prec(g_precision()), JUNCTION_TAZ_MISSING_HELP);
            self.myErrorOutput.m().inform(&msg, true);
            return Ptr::null();
        }
        if toSource.is_null() || toSink.is_null() {
            let msg = format!("Junction-taz '{}' not found when mapping position {}.{}", toJunction.m().mm_id(), pos.to_string_prec(g_precision()), JUNCTION_TAZ_MISSING_HELP);
            self.myErrorOutput.m().inform(&msg, true);
            return Ptr::null();
        }
        let fromPossible = if isFrom { fromSource.m().mm_successors_for(vClass).len() > 0 } else { fromSink.m().mm_predecessors().len() > 0 };
        let toPossible = if isFrom { toSource.m().mm_successors_for(vClass).len() > 0 } else { toSink.m().mm_predecessors().len() > 0 };
        if fromCloser && fromPossible {
            // return closest if possible
            if isFrom { fromSource } else { fromSink }
        } else if !fromCloser && toPossible {
            // return closest if possible
            if isFrom { toSource } else { toSink }
        } else {
            // return possible
            if fromPossible {
                if isFrom { fromSource } else { fromSink }
            } else {
                if isFrom { toSource } else { toSink }
            }
        }
    }

    /// C++ `const E* MapMatcher::getTaz(const Position&, const E*, bool)`: find the taz connector of the closest edge
    pub fn get_taz(&self, host: &dyn MapMatcherHost<E>, pos: &Position, closestEdge: Ptr<E>, isFrom: bool) -> Ptr<E> {
        if closestEdge.is_null() {
            return Ptr::null();
        }
        let mut cands: Vec<Ptr<E>> = Vec::new();
        if isFrom {
            for e in closestEdge.m().mm_predecessors() {
                if e.m().mm_is_taz_connector() {
                    cands.push(e);
                }
            }
        } else {
            for e in closestEdge.m().mm_successors() {
                if e.m().mm_is_taz_connector() {
                    cands.push(e);
                }
            }
        }
        if cands.len() == 0 {
            let msg = format!("Taz for edge '{}' not found when mapping position {}.{}", closestEdge.m().mm_id(), pos.to_string_prec(g_precision()), JUNCTION_TAZ_MISSING_HELP);
            self.myErrorOutput.m().inform(&msg, true);
            return Ptr::null();
        }
        if cands.len() > 1 {
            let n = cands.len() as i32;
            let rng = host.get_rng();
            let idx = if rng.is_null() { with_default(|r| r.rand_int(n)) } else { rng.m().rand_int(n) };
            cands[idx as usize]
        } else {
            cands[0]
        }
    }

    /// C++ `NamedRTree* MapMatcher::getLaneTree()`: initialize lane-RTree
    fn get_lane_tree(&self, host: &dyn MapMatcherHost<E>) -> Ptr<NamedRTree> {
        if self.myLaneTree.get().is_null() {
            let tree = new_named_rtree();
            self.myLaneTree.set(tree);
            host.init_lane_tree(tree);
        }
        self.myLaneTree.get()
    }
}

impl MapEdge for ROEdge {
    type Node = RONode;
    fn mm_is_internal(&self) -> bool {
        self.is_internal()
    }
    fn mm_successor_front(&self) -> Ptr<ROEdge> {
        self.get_successors(SVC_IGNORING)[0]
    }
    fn mm_id(&self) -> &str {
        self.base.get_id()
    }
    fn mm_from_junction(&self) -> Ptr<RONode> {
        self.get_from_junction()
    }
    fn mm_to_junction(&self) -> Ptr<RONode> {
        self.get_to_junction()
    }
    fn mm_is_taz_connector(&self) -> bool {
        self.is_taz_connector()
    }
    fn mm_predecessors(&self) -> Vec<Ptr<ROEdge>> {
        self.get_predecessors().clone()
    }
    fn mm_successors(&self) -> Vec<Ptr<ROEdge>> {
        self.get_successors(SVC_IGNORING).clone()
    }
    fn mm_successors_for(&self, vClass: SUMOVehicleClass) -> Vec<Ptr<ROEdge>> {
        self.get_successors(vClass).clone()
    }
}

impl MapNode for RONode {
    fn mm_position(&self) -> &Position {
        self.get_position()
    }
    fn mm_id(&self) -> &str {
        self.base.get_id()
    }
}

impl MapLane<ROEdge> for ROLane {
    fn mm_edge(&self) -> Ptr<ROEdge> {
        Ptr::from_mut(self.get_edge())
    }
    fn mm_allows(&self, vClass: SUMOVehicleClass) -> bool {
        self.allows_vehicle_class(vClass)
    }
    fn mm_shape(&self) -> &PositionVector {
        self.get_shape()
    }
}

impl MapEdge for MSEdge {
    type Node = MSJunction;
    fn mm_is_internal(&self) -> bool {
        self.is_internal()
    }
    fn mm_successor_front(&self) -> Ptr<MSEdge> {
        self.get_successors(SVC_IGNORING)[0]
    }
    fn mm_id(&self) -> &str {
        self.base.get_id()
    }
    fn mm_from_junction(&self) -> Ptr<MSJunction> {
        self.get_from_junction()
    }
    fn mm_to_junction(&self) -> Ptr<MSJunction> {
        self.get_to_junction()
    }
    fn mm_is_taz_connector(&self) -> bool {
        self.is_taz_connector()
    }
    fn mm_predecessors(&self) -> Vec<Ptr<MSEdge>> {
        self.get_predecessors().clone()
    }
    fn mm_successors(&self) -> Vec<Ptr<MSEdge>> {
        self.get_successors(SVC_IGNORING).clone()
    }
    fn mm_successors_for(&self, vClass: SUMOVehicleClass) -> Vec<Ptr<MSEdge>> {
        self.get_successors(vClass).clone()
    }
}

impl MapNode for MSJunction {
    fn mm_position(&self) -> &Position {
        self.get_position(false)
    }
    fn mm_id(&self) -> &str {
        self.base.get_id()
    }
}

impl MapLane<MSEdge> for MSLane {
    fn mm_edge(&self) -> Ptr<MSEdge> {
        Ptr::from_mut(self.get_edge())
    }
    fn mm_allows(&self, vClass: SUMOVehicleClass) -> bool {
        self.allows_vehicle_class(vClass)
    }
    fn mm_shape(&self) -> &PositionVector {
        self.get_shape(false)
    }
}
