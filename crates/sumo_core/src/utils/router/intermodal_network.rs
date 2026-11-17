//! Port of utils/router/IntermodalNetwork.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::intermodal_edge::get_sidewalk;
use crate::utils::router::router_edge::{RouterEdge, RouterEdgeInfo, RouterNode, RouterVehicle};
use utils::geom::POSITION_EPS;
use utils::common::to_string::to_string;

/// C++ `new` for a network edge: owned by `myEdges` (C++ deletes it in the destructor)
fn leak_edge<T>(edge: T) -> Ptr<T> {
    Ptr::from_raw(Box::into_raw(Box::new(edge)))
}

fn e_is_walking_area<E, V>(edge: Ptr<E>) -> bool
where
    E: RouterEdge<V>,
{
    edge.is_walking_area()
}

fn e_is_normal<E, V>(edge: Ptr<E>) -> bool
where
    E: RouterEdge<V>,
{
    edge.is_normal()
}

fn e_has_normal_function<E, V>(edge: Ptr<E>) -> bool
where
    E: RouterEdge<V>,
{
    edge.function() == SumoXMLEdgeFunc::NORMAL
}

fn e_has_internal_function<E, V>(edge: Ptr<E>) -> bool
where
    E: RouterEdge<V>,
{
    edge.function() == SumoXMLEdgeFunc::INTERNAL
}

fn e_permissions<E>(edge: Ptr<E>) -> SVCPermissions
where
    E: RouterEdgeInfo,
{
    edge.edge_permissions()
}

fn e_from_junction<E, V>(edge: Ptr<E>) -> Ptr<E::Node>
where
    E: RouterEdge<V>,
{
    edge.from_junction()
}

fn e_to_junction<E, V>(edge: Ptr<E>) -> Ptr<E::Node>
where
    E: RouterEdge<V>,
{
    edge.to_junction()
}

fn e_other_taz_connector<E, V>(edge: Ptr<E>) -> Ptr<E>
where
    E: RouterEdge<V>,
{
    edge.other_taz_connector()
}

fn e_stop_position<E, V>(stop: &SUMOVehicleParameter_Stop) -> Position
where
    E: RouterEdge<V>,
{
    E::stop_position(stop)
}

fn n_id<N: RouterNode>(node: Ptr<N>) -> String {
    node.id().to_string()
}

fn n_outgoing<E, N: RouterNode<Edge = E>>(node: Ptr<N>) -> Vec<Ptr<E>> {
    node.outgoing().to_vec()
}

fn n_incoming<E, N: RouterNode<Edge = E>>(node: Ptr<N>) -> Vec<Ptr<E>> {
    node.incoming().to_vec()
}

fn l_outgoing_via_lanes<L, E, V>(lane: Ptr<L>) -> Vec<(Ptr<L>, Ptr<E>)>
where
    E: RouterEdge<V, Lane = L>,
{
    E::lane_outgoing_via_lanes(lane)
}

fn l_edge<L, E, V>(lane: Ptr<L>) -> Ptr<E>
where
    E: RouterEdge<V, Lane = L>,
{
    E::lane_edge(lane)
}

// shortcut: C++ dynamic_cast<_AccessEdge*> stand-in: both AccessEdge ctors tag the line "!access" and no other intermodal edge does, upgrade to a type flag when the edge kinds get one
fn as_access_edge<E, L, N, V>(edge: Ptr<IntermodalEdge<E, L, N, V>>) -> Ptr<AccessEdge<E, L, N, V>>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: 'static,
    V: RouterVehicle + 'static,
{
    if edge.get_line() == "!access" { edge.cast() } else { Ptr::null() }
}

fn max2(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// C++ `gTaxiClasses`
fn g_taxi_classes_perm() -> SVCPermissions {
    utils::common::std_defs::g_taxi_classes() as SVCPermissions
}

/// C++ enum `ModeChangeOptions`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum ModeChangeOptions {
    PARKING_AREAS = 1,
    PT_STOPS = 2,
    ALL_JUNCTIONS = 4,
    TAXI_DROPOFF_PARKING_AREAS = 8,
    TAXI_DROPOFF_PT = 16,
    TAXI_DROPOFF_ANYWHERE = 32,
    TAXI_PICKUP_PARKING_AREAS = 64,
    TAXI_PICKUP_PT = 128,
    TAXI_PICKUP_ANYWHERE = 256,
}

/// C++ typedef `std::pair<_IntermodalEdge *, _IntermodalEdge *> IntermodalNetwork::EdgePair`
pub type IntermodalNetwork_EdgePair<E, L, N, V> = (Ptr<AccessEdge__IntermodalEdge<E, L, N, V>>, Ptr<AccessEdge__IntermodalEdge<E, L, N, V>>);

/// The typed forward / backward pair of a network edge (C++ `EdgePair` with `_IntermodalEdge*`)
pub type BidiPair<E, L, N, V> = (Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>);

// shortcut: the edge id is only needed for the error text and RouterEdge is not bounded on the unbounded callers (pedestrian_router), upgrade when callers bound E
fn e_id_unbounded<E, V>(edge: Ptr<E>) -> String
where
    E: RouterEdge<V>,
{
    edge.id().to_string()
}

/// C++ typedef `AccessEdge<E, L, N, V> IntermodalNetwork::_AccessEdge`
pub type IntermodalNetwork__AccessEdge = Unmapped /* AccessEdge<E, L, N, V> */;

/// C++ typedef `IntermodalEdge<E, L, N, V> IntermodalNetwork::_IntermodalEdge`
pub type IntermodalNetwork__IntermodalEdge = Unmapped /* IntermodalEdge<E, L, N, V> */;

/// C++ typedef `PublicTransportEdge<E, L, N, V> IntermodalNetwork::_PTEdge`
pub type IntermodalNetwork__PTEdge = Unmapped /* PublicTransportEdge<E, L, N, V> */;

/// C++ typedef `PedestrianEdge<E, L, N, V> IntermodalNetwork::_PedestrianEdge`
pub type IntermodalNetwork__PedestrianEdge = Unmapped /* PedestrianEdge<E, L, N, V> */;

/// the intermodal network storing edges, connections and the mappings to the "real" edges
/// C++ class `IntermodalNetwork` (utils/router/IntermodalNetwork.h)
pub struct IntermodalNetwork<E, L, N, V> {
    /// the edge dictionary
    pub myEdges: Vec<Ptr<IntermodalEdge<E, L, N, V>>>,
    /// retrieve the forward and backward edge for the given input edge E
    pub myBidiLookup: BTreeMap<Ptr<E>, BidiPair<E, L, N, V>>,
    /// retrieve the depart edges for the given input edge E
    pub myDepartLookup: BTreeMap<Ptr<E>, Vec<Ptr<IntermodalEdge<E, L, N, V>>>>,
    /// retrieve the arrival edges for the given input edge E
    pub myArrivalLookup: BTreeMap<Ptr<E>, Vec<Ptr<IntermodalEdge<E, L, N, V>>>>,
    /// the walking connector edge (fake walking area)
    pub myWalkingConnectorLookup: BTreeMap<Ptr<N>, Ptr<IntermodalEdge<E, L, N, V>>>,
    /// retrieve the car edge for the given input edge E
    pub myCarLookup: BTreeMap<Ptr<E>, Ptr<IntermodalEdge<E, L, N, V>>>, // C++: std::map<const E *, _IntermodalEdge *, ComparatorNumericalIdLess>
    /// retrieve the public transport edges for the given line
    pub myPTLines: BTreeMap<String, Vec<Ptr<PublicTransportEdge<E, L, N, V>>>>,
    /// retrieve the representing edge for the given stopping place
    pub myStopConnections: BTreeMap<String, Ptr<IntermodalEdge<E, L, N, V>>>,
    /// retrieve the splitted edges for the given "original"
    pub myAccessSplits: BTreeMap<Ptr<IntermodalEdge<E, L, N, V>>, Vec<Ptr<IntermodalEdge<E, L, N, V>>>>,
    /// looped lines need extra checking when building itineraries
    pub myLoopedLines: BTreeSet<String>,
    pub myNumericalID: i32,
    pub myCarWalkTransfer: i32,
    pub myHavePTSchedules: bool,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> IntermodalNetwork<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: RouterNode<Edge = E> + 'static,
    V: RouterVehicle + 'static,
{
    /// Returns the next numerical ID (C++ `myNumericalID++`)
    fn next_id(&mut self) -> i32 {
        let id = self.myNumericalID;
        self.myNumericalID += 1;
        id
    }

    /// C++: `void IntermodalNetwork::IntermodalNetwork<E, L, N, V>(const std::vector<E *> &, const bool, const int)`
    // IntermodalNetwork(const std::vector<E*>& edges, const bool pedestrianOnly, const int carWalkTransfer = 0)
    //         : myNumericalID(0), myCarWalkTransfer(carWalkTransfer), myHavePTSchedules(false) {
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //         std::cout << "initIntermodalNetwork\n";
    // #endif
    //         // build the pedestrian edges and the depart / arrival connectors with lookup tables
    //         bool haveSeenWalkingArea = false;
    //         for (const E* const edge : edges) {
    //             if (edge->isTazConnector()) {
    //                 // only a single edge
    //                 _AccessEdge* access = new _AccessEdge(myNumericalID++, edge->getID(), edge);
    //                 addEdge(access);
    //                 myDepartLookup[edge].push_back(access);
    //                 myArrivalLookup[edge].push_back(access);
    //             } else {
    //                 const L* lane = getSidewalk<E, L>(edge);
    //                 if (lane != nullptr) {
    //                     if (edge->isWalkingArea()) {
    //                         // only a single edge
    //                         addEdge(new _PedestrianEdge(myNumericalID++, edge, lane, true));
    //                         myBidiLookup[edge] = std::make_pair(myEdges.back(), myEdges.back());
    //                         myDepartLookup[edge].push_back(myEdges.back());
    //                         myArrivalLookup[edge].push_back(myEdges.back());
    //                         haveSeenWalkingArea = true;
    //                     } else { // regular edge or crossing
    //                         // forward and backward edges
    //                         addEdge(new _PedestrianEdge(myNumericalID++, edge, lane, true));
    //                         addEdge(new _PedestrianEdge(myNumericalID++, edge, lane, false));
    //                         myBidiLookup[edge] = std::make_pair(myEdges[myNumericalID - 2], myEdges.back());
    //                     }
    //                 }
    //                 if (!edge->isWalkingArea()) {
    //                     // depart and arrival edges (the router can decide the initial direction to take and the direction to arrive from)
    //                     _IntermodalEdge* const departConn = new _IntermodalEdge(edge->getID() + "_depart_connector", myNumericalID++, edge, "!connector");
    //                     _IntermodalEdge* const arrivalConn = new _IntermodalEdge(edge->getID() + "_arrival_connector", myNumericalID++, edge, "!connector");
    //                     addConnectors(departConn, arrivalConn, 0);
    //                 }
    //             }
    //         }
    //
    //         // build the walking connectors if there are no walking areas
    //         for (const E* const edge : edges) {
    //             if (edge->isTazConnector() || edge->isInternal()) {
    //                 continue;
    //             }
    //             if (haveSeenWalkingArea) {
    //                 // connectivity needs to be ensured only in the real intermodal case, for simple pedestrian routing we don't have connectors if we have walking areas
    //                 if (!pedestrianOnly && getSidewalk<E, L>(edge) == nullptr) {
    //                     const N* const node = edge->getToJunction();
    //                     if (myWalkingConnectorLookup.count(node) == 0) {
    //                         addEdge(new _IntermodalEdge(node->getID() + "_walking_connector", myNumericalID++, nullptr, "!connector"));
    //                         myWalkingConnectorLookup[node] = myEdges.back();
    //                     }
    //                 }
    //             } else {
    //                 for (const N* const node : {
    //                             edge->getFromJunction(), edge->getToJunction()
    //                         }) {
    //                     if (myWalkingConnectorLookup.count(node) == 0) {
    //                         addEdge(new _IntermodalEdge(node->getID() + "_walking_connector", myNumericalID++, nullptr, "!connector"));
    //                         myWalkingConnectorLookup[node] = myEdges.back();
    //                     }
    //                 }
    //             }
    //         }
    //         // build the connections
    //         for (const E* const edge : edges) {
    //             if (edge->isTazConnector()) {
    //                 // since pedestrians walk in both directions, also allow departing at sinks and arriving at sources
    //                 _IntermodalEdge* const tazDepart = getDepartConnector(edge);
    //                 _IntermodalEdge* const tazArrive = getArrivalConnector(edge);
    //                 const E* other = edge->getOtherTazConnector();
    //                 _IntermodalEdge* const otherTazDepart = other != nullptr ? getDepartConnector(other) : tazDepart;
    //                 _IntermodalEdge* const otherTazArrive = other != nullptr ? getArrivalConnector(other) : tazArrive;
    //                 for (const E* out : edge->getSuccessors()) {
    //                     if (out->isNormal()) {
    //                         tazDepart->addSuccessor(getDepartConnector(out));
    //                         getArrivalConnector(out)->addSuccessor(otherTazArrive);
    //                     }
    //                 }
    //                 for (const E* in : edge->getPredecessors()) {
    //                     if (in->isNormal()) {
    //                         getArrivalConnector(in)->addSuccessor(tazArrive);
    //                         otherTazDepart->addSuccessor(getDepartConnector(in));
    //                     }
    //                 }
    //                 continue;
    //             }
    //             const L* const sidewalk = getSidewalk<E, L>(edge);
    //             if (sidewalk == nullptr) {
    //                 continue;
    //             }
    //             // find all incoming and outgoing lanes for the sidewalk and
    //             // connect the corresponding IntermodalEdges
    //             const EdgePair& pair = getBothDirections(edge);
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //             std::cout << "  building connections from " << sidewalk->getID() << "\n";
    // #endif
    //             if (haveSeenWalkingArea) {
    //                 const std::vector<std::pair<const L*, const E*> > outgoing = sidewalk->getOutgoingViaLanes();
    //                 // if one of the outgoing lanes is a walking area it must be used.
    //                 // All other connections shall be ignored
    //                 // if it has no outgoing walking area, it probably is a walking area itself
    //                 bool hasWalkingArea = false;
    //                 for (const auto& target : outgoing) {
    //                     if (target.first->getEdge().isWalkingArea()) {
    //                         hasWalkingArea = true;
    //                         break;
    //                     }
    //                 }
    //                 for (const auto& target : outgoing) {
    //                     const E* const targetEdge = &(target.first->getEdge());
    //                     const bool used = (target.first == getSidewalk<E, L>(targetEdge)
    //                                        && (!hasWalkingArea || targetEdge->isWalkingArea()));
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //                     const L* potTarget = getSidewalk<E, L>(targetEdge);
    //                     std::cout << "   lane=" << (potTarget == 0 ? "NULL" : potTarget->getID()) << (used ? "(used)" : "") << "\n";
    // #endif
    //                     if (used) {
    //                         const EdgePair& targetPair = getBothDirections(targetEdge);
    //                         pair.first->addSuccessor(targetPair.first);
    //                         targetPair.second->addSuccessor(pair.second);
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //                         std::cout << "     " << pair.first->getID() << " -> " << targetPair.first->getID() << "\n";
    //                         std::cout << "     " << targetPair.second->getID() << " -> " << pair.second->getID() << "\n";
    // #endif
    //                     }
    //                 }
    //             }
    //             // We may have a network without pedestrian structures or a car-only edge.
    //             // In the first case we assume that all sidewalks at a junction are interconnected,
    //             // in the second we connect all car-only edges to all sidewalks.
    //             _IntermodalEdge* const toNodeConn = myWalkingConnectorLookup[edge->getToJunction()];
    //             if (toNodeConn != nullptr) {
    //                 // Check for the outgoing vias and use the shortest one as an approximation
    //                 const std::vector<std::pair<const L*, const E*> > outgoing = sidewalk->getOutgoingViaLanes();
    //                 double minViaLength = std::numeric_limits<double>::max();
    //                 const E* minVia = nullptr;
    //                 for (const auto& target : outgoing) {
    //                     if (target.second != nullptr && target.second->getLength() < minViaLength) {
    //                         minViaLength = target.second->getLength();
    //                         minVia = target.second;
    //                     }
    //                 }
    //                 EdgePair interVia = std::make_pair(nullptr, nullptr);
    //                 if (minVia != nullptr) {
    //                     const auto it = myBidiLookup.find(minVia);
    //                     if (it != myBidiLookup.end()) {
    //                         interVia = it->second;
    //                     }
    //                 }
    //                 if (!haveSeenWalkingArea) {
    //                     // if we have walking areas we should use them and not the connector
    //                     pair.first->addSuccessor(toNodeConn, interVia.first);
    //                 }
    //                 toNodeConn->addSuccessor(pair.second, interVia.second);
    //             }
    //             _IntermodalEdge* const fromNodeConn = myWalkingConnectorLookup[edge->getFromJunction()];
    //             if (fromNodeConn != nullptr) {
    //                 if (!haveSeenWalkingArea) {
    //                     pair.second->addSuccessor(fromNodeConn);
    //                 }
    //                 fromNodeConn->addSuccessor(pair.first);
    //             }
    //             if (!edge->isWalkingArea()) {
    //                 // build connections from depart connector
    //                 _IntermodalEdge* startConnector = getDepartConnector(edge);
    //                 startConnector->addSuccessor(pair.first);
    //                 startConnector->addSuccessor(pair.second);
    //                 // build connections to arrival connector
    //                 _IntermodalEdge* endConnector = getArrivalConnector(edge);
    //                 pair.first->addSuccessor(endConnector);
    //                 pair.second->addSuccessor(endConnector);
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //                 std::cout << "     " << startConnector->getID() << " -> " << pair.first->getID() << "\n";
    //                 std::cout << "     " << startConnector->getID() << " -> " << pair.second->getID() << "\n";
    //                 std::cout << "     " << pair.first->getID() << " -> " << endConnector->getID() << "\n";
    //                 std::cout << "     " << pair.second->getID() << " -> " << endConnector->getID() << "\n";
    // #endif
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, pedestrianOnly: bool, carWalkTransfer: i32) -> Self where E: RouterEdge<V, Node = N> + 'static, L: 'static, N: 'static, V: RouterVehicle + 'static {
        let mut net = Self {
            myEdges: Vec::new(),
            myBidiLookup: BTreeMap::new(),
            myDepartLookup: BTreeMap::new(),
            myArrivalLookup: BTreeMap::new(),
            myWalkingConnectorLookup: BTreeMap::new(),
            myCarLookup: BTreeMap::new(),
            myPTLines: BTreeMap::new(),
            myStopConnections: BTreeMap::new(),
            myAccessSplits: BTreeMap::new(),
            myLoopedLines: BTreeSet::new(),
            myNumericalID: 0,
            myCarWalkTransfer: carWalkTransfer,
            myHavePTSchedules: false,
            _p: std::marker::PhantomData,
        };
        // build the pedestrian edges and the depart / arrival connectors with lookup tables
        let mut haveSeenWalkingArea = false;
        for &edge in edges {
            if edge.is_taz_connector() {
                // only a single edge
                let id = net.next_id();
                let access = leak_edge(AccessEdge::<E, L, N, V>::new_2(id, edge.id(), edge, 0., SVC_IGNORING, SVC_IGNORING)).cast::<IntermodalEdge<E, L, N, V>>();
                net.add_edge(access);
                net.myDepartLookup.entry(edge).or_default().push(access);
                net.myArrivalLookup.entry(edge).or_default().push(access);
            } else {
                let lane = get_sidewalk::<E, L, V>(edge, SVC_PEDESTRIAN);
                if !lane.is_null() {
                    if e_is_walking_area(edge) {
                        // only a single edge
                        let id = net.next_id();
                        net.add_edge(leak_edge(PedestrianEdge::<E, L, N, V>::new(id, edge, lane, true, -1.)).cast());
                        let last = *net.myEdges.last().unwrap();
                        net.myBidiLookup.insert(edge, (last, last));
                        net.myDepartLookup.entry(edge).or_default().push(last);
                        net.myArrivalLookup.entry(edge).or_default().push(last);
                        haveSeenWalkingArea = true;
                    } else { // regular edge or crossing
                        // forward and backward edges
                        let id = net.next_id();
                        net.add_edge(leak_edge(PedestrianEdge::<E, L, N, V>::new(id, edge, lane, true, -1.)).cast());
                        let id = net.next_id();
                        net.add_edge(leak_edge(PedestrianEdge::<E, L, N, V>::new(id, edge, lane, false, -1.)).cast());
                        let fwd = net.myEdges[(net.myNumericalID - 2) as usize];
                        let bwd = *net.myEdges.last().unwrap();
                        net.myBidiLookup.insert(edge, (fwd, bwd));
                    }
                }
                if !e_is_walking_area(edge) {
                    // depart and arrival edges (the router can decide the initial direction to take and the direction to arrive from)
                    let id = net.next_id();
                    let departConn = leak_edge(IntermodalEdge::<E, L, N, V>::new(edge.id().to_string() + "_depart_connector", id, edge, "!connector", -1.));
                    let id = net.next_id();
                    let arrivalConn = leak_edge(IntermodalEdge::<E, L, N, V>::new(edge.id().to_string() + "_arrival_connector", id, edge, "!connector", -1.));
                    net.add_connectors(departConn, arrivalConn, 0);
                }
            }
        }

        // build the walking connectors if there are no walking areas
        for &edge in edges {
            if edge.is_taz_connector() || edge.is_internal() {
                continue;
            }
            if haveSeenWalkingArea {
                // connectivity needs to be ensured only in the real intermodal case, for simple pedestrian routing we don't have connectors if we have walking areas
                if !pedestrianOnly && get_sidewalk::<E, L, V>(edge, SVC_PEDESTRIAN).is_null() {
                    let node: Ptr<N> = e_to_junction(edge);
                    if !net.myWalkingConnectorLookup.contains_key(&node) {
                        let id = net.next_id();
                        let conn = leak_edge(IntermodalEdge::<E, L, N, V>::new(n_id(node) + "_walking_connector", id, Ptr::null(), "!connector", -1.));
                        net.add_edge(conn);
                        net.myWalkingConnectorLookup.insert(node, conn);
                    }
                }
            } else {
                let nodes: [Ptr<N>; 2] = [e_from_junction(edge), e_to_junction(edge)];
                for node in nodes {
                    if !net.myWalkingConnectorLookup.contains_key(&node) {
                        let id = net.next_id();
                        let conn = leak_edge(IntermodalEdge::<E, L, N, V>::new(n_id(node) + "_walking_connector", id, Ptr::null(), "!connector", -1.));
                        net.add_edge(conn);
                        net.myWalkingConnectorLookup.insert(node, conn);
                    }
                }
            }
        }
        // build the connections
        for &edge in edges {
            if edge.is_taz_connector() {
                // since pedestrians walk in both directions, also allow departing at sinks and arriving at sources
                let tazDepart = net.get_depart_connector(edge, 0);
                let tazArrive = net.get_arrival_connector(edge, 0);
                let other = e_other_taz_connector(edge);
                let otherTazDepart = if !other.is_null() { net.get_depart_connector(other, 0) } else { tazDepart };
                let otherTazArrive = if !other.is_null() { net.get_arrival_connector(other, 0) } else { tazArrive };
                for &out in edge.successors(SVC_IGNORING) {
                    if e_is_normal(out) {
                        let d = net.get_depart_connector(out, 0);
                        tazDepart.m().add_successor(d, Ptr::null());
                        net.get_arrival_connector(out, 0).m().add_successor(otherTazArrive, Ptr::null());
                    }
                }
                for &in_ in edge.predecessors() {
                    if e_is_normal(in_) {
                        net.get_arrival_connector(in_, 0).m().add_successor(tazArrive, Ptr::null());
                        let d = net.get_depart_connector(in_, 0);
                        otherTazDepart.m().add_successor(d, Ptr::null());
                    }
                }
                continue;
            }
            let sidewalk = get_sidewalk::<E, L, V>(edge, SVC_PEDESTRIAN);
            if sidewalk.is_null() {
                continue;
            }
            // find all incoming and outgoing lanes for the sidewalk and
            // connect the corresponding IntermodalEdges
            let pair = net.bidi(edge);
            if haveSeenWalkingArea {
                let outgoing = l_outgoing_via_lanes::<L, E, V>(sidewalk);
                // if one of the outgoing lanes is a walking area it must be used.
                // All other connections shall be ignored
                // if it has no outgoing walking area, it probably is a walking area itself
                let mut hasWalkingArea = false;
                for target in &outgoing {
                    if e_is_walking_area(l_edge::<L, E, V>(target.0)) {
                        hasWalkingArea = true;
                        break;
                    }
                }
                for target in &outgoing {
                    let targetEdge = l_edge::<L, E, V>(target.0);
                    let used = target.0 == get_sidewalk::<E, L, V>(targetEdge, SVC_PEDESTRIAN) && (!hasWalkingArea || e_is_walking_area(targetEdge));
                    if used {
                        let targetPair = net.bidi(targetEdge);
                        pair.0.m().add_successor(targetPair.0, Ptr::null());
                        targetPair.1.m().add_successor(pair.1, Ptr::null());
                    }
                }
            }
            // We may have a network without pedestrian structures or a car-only edge.
            // In the first case we assume that all sidewalks at a junction are interconnected,
            // in the second we connect all car-only edges to all sidewalks.
            let toNodeConn = *net.myWalkingConnectorLookup.entry(e_to_junction(edge)).or_insert(Ptr::null());
            if !toNodeConn.is_null() {
                // Check for the outgoing vias and use the shortest one as an approximation
                let outgoing = l_outgoing_via_lanes::<L, E, V>(sidewalk);
                let mut minViaLength = f64::MAX;
                let mut minVia: Ptr<E> = Ptr::null();
                for target in &outgoing {
                    if !target.1.is_null() && target.1.length() < minViaLength {
                        minViaLength = target.1.length();
                        minVia = target.1;
                    }
                }
                let mut interVia: BidiPair<E, L, N, V> = (Ptr::null(), Ptr::null());
                if !minVia.is_null() {
                    if let Some(it) = net.myBidiLookup.get(&minVia) {
                        interVia = *it;
                    }
                }
                if !haveSeenWalkingArea {
                    // if we have walking areas we should use them and not the connector
                    pair.0.m().add_successor(toNodeConn, interVia.0);
                }
                toNodeConn.m().add_successor(pair.1, interVia.1);
            }
            let fromNodeConn = *net.myWalkingConnectorLookup.entry(e_from_junction(edge)).or_insert(Ptr::null());
            if !fromNodeConn.is_null() {
                if !haveSeenWalkingArea {
                    pair.1.m().add_successor(fromNodeConn, Ptr::null());
                }
                fromNodeConn.m().add_successor(pair.0, Ptr::null());
            }
            if !e_is_walking_area(edge) {
                // build connections from depart connector
                let startConnector = net.get_depart_connector(edge, 0);
                startConnector.m().add_successor(pair.0, Ptr::null());
                startConnector.m().add_successor(pair.1, Ptr::null());
                // build connections to arrival connector
                let endConnector = net.get_arrival_connector(edge, 0);
                pair.0.m().add_successor(endConnector, Ptr::null());
                pair.1.m().add_successor(endConnector, Ptr::null());
            }
        }
        net
    }

    /// C++: `void IntermodalNetwork::~IntermodalNetwork<E, L, N, V>()`
    // ~IntermodalNetwork() {
    //         for (typename std::vector<_IntermodalEdge*>::iterator it = myEdges.begin(); it != myEdges.end(); ++it) {
    //             delete *it;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: the concrete edge kind is not recorded, so the Box cannot be rebuilt; the edges are leaked, upgrade with a per-kind owner
        self.myEdges.clear();
    }

    /// C++: `void IntermodalNetwork::addEdge(_IntermodalEdge *)`
    // void addEdge(_IntermodalEdge* edge) {
    //         while ((int)myEdges.size() <= edge->getNumericalID()) {
    //             myEdges.push_back(0);
    //         }
    //         myEdges[edge->getNumericalID()] = edge;
    //     }
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, edge: Ptr<IntermodalEdge<E, L, N, V>>) {
        while (self.myEdges.len() as i32) <= edge.get_numerical_id() {
            self.myEdges.push(Ptr::null());
        }
        self.myEdges[edge.get_numerical_id() as usize] = edge;
    }

    /// C++: `void IntermodalNetwork::addConnectors(_IntermodalEdge *const, _IntermodalEdge *const, const int)`
    // void addConnectors(_IntermodalEdge* const depConn, _IntermodalEdge* const arrConn, const int index) {
    //         addEdge(depConn);
    //         addEdge(arrConn);
    //         myDepartLookup[depConn->getEdge()].insert(myDepartLookup[depConn->getEdge()].begin() + index, depConn);
    //         myArrivalLookup[arrConn->getEdge()].insert(myArrivalLookup[arrConn->getEdge()].begin() + index, arrConn);
    //     }
    #[allow(unused_variables)]
    pub fn add_connectors(&mut self, depConn: Ptr<IntermodalEdge<E, L, N, V>>, arrConn: Ptr<IntermodalEdge<E, L, N, V>>, index: i32) {
        self.add_edge(depConn);
        self.add_edge(arrConn);
        let depEdge = depConn.get_edge();
        self.myDepartLookup.entry(depEdge).or_default().insert(index as usize, depConn);
        let arrEdge = arrConn.get_edge();
        self.myArrivalLookup.entry(arrEdge).or_default().insert(index as usize, arrConn);
    }

    /// C++: `const std::vector<_IntermodalEdge *> & IntermodalNetwork::getAllEdges()`
    // const std::vector<_IntermodalEdge*>& getAllEdges() {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_all_edges(&mut self) -> &Vec<Ptr<IntermodalEdge<E, L, N, V>>> {
        &self.myEdges
    }

    /// C++: `const EdgePair & IntermodalNetwork::getBothDirections(const E *) const` [const]
    /// Returns the pair of forward and backward edge
    // const EdgePair& getBothDirections(const E* e) const {
    //         typename std::map<const E*, EdgePair>::const_iterator it = myBidiLookup.find(e);
    //         if (it == myBidiLookup.end()) {
    //             assert(false);
    //             throw ProcessError(TLF("Edge '%' not found in intermodal network.'", e->getID()));
    //         }
    //         return (*it).second;
    //     }
    #[allow(unused_variables)]
    pub fn get_both_directions(&self, e: Ptr<E>) -> IntermodalNetwork_EdgePair<E, L, N, V> {
        let (fwd, bwd) = self.bidi(e);
        (fwd.cast(), bwd.cast())
    }

    /// Typed form of [`get_both_directions`](Self::get_both_directions) for the router-internal callers
    fn bidi(&self, e: Ptr<E>) -> BidiPair<E, L, N, V> {
        match self.myBidiLookup.get(&e) {
            Some(pair) => *pair,
            None => panic!("{}", ProcessError::new(format!("Edge '{}' not found in intermodal network.'", e_id_unbounded(e)))),
        }
    }

    /// C++: `const _IntermodalEdge * IntermodalNetwork::getDepartEdge(const E *, const double) const` [const]
    /// Returns the departing intermodal edge
    // const _IntermodalEdge* getDepartEdge(const E* e, const double pos) const {
    //         typename std::map<const E*, std::vector<_IntermodalEdge*> >::const_iterator it = myDepartLookup.find(e);
    //         if (it == myDepartLookup.end()) {
    //             throw ProcessError(TLF("Depart edge '%' not found in intermodal network.", e->getID()));
    //         }
    //         if ((e->getPermissions() & SVC_PEDESTRIAN) == 0) {
    //             // use most specific split (best trainStop, quay etc)
    //             double bestDist = std::numeric_limits<double>::max();
    //             const _IntermodalEdge* best = nullptr;
    //             for (const _IntermodalEdge* const split : it->second) {
    //                 if (pos >= split->getStartPos() - POSITION_EPS && pos <= split->getEndPos() + POSITION_EPS) {
    //                     const double dist = split->getEndPos() - split->getStartPos();
    //                     if (dist < bestDist) {
    //                         bestDist = dist;
    //                         best = split;
    //                     }
    //                 }
    //             }
    //             assert(best != nullptr);
    //             return best;
    //         } else {
    //             // use next downstream edge
    //             const std::vector<_IntermodalEdge*>& splitList = it->second;
    //             typename std::vector<_IntermodalEdge*>::const_iterator splitIt = splitList.begin();
    //             double totalLength = 0.;
    //             while (splitIt + 1 != splitList.end() && totalLength + (*splitIt)->getLength() < pos) {
    //                 totalLength += (*splitIt)->getLength();
    //                 ++splitIt;
    //             }
    //             return *splitIt;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self, e: Ptr<E>, pos: f64) -> Ptr<IntermodalEdge<E, L, N, V>> where E: RouterEdge<V, Node = N> + 'static, L: 'static, N: 'static, V: RouterVehicle + 'static {
        let splitList = match self.myDepartLookup.get(&e) {
            Some(list) => list,
            None => panic!("{}", ProcessError::new(format!("Depart edge '{}' not found in intermodal network.", e.id()))),
        };
        if (e_permissions(e) & SVC_PEDESTRIAN) == 0 {
            // use most specific split (best trainStop, quay etc)
            let mut bestDist = f64::MAX;
            let mut best: Ptr<IntermodalEdge<E, L, N, V>> = Ptr::null();
            for &split in splitList {
                if pos >= split.get_start_pos() - POSITION_EPS && pos <= split.get_end_pos() + POSITION_EPS {
                    let dist = split.get_end_pos() - split.get_start_pos();
                    if dist < bestDist {
                        bestDist = dist;
                        best = split;
                    }
                }
            }
            assert!(!best.is_null());
            best
        } else {
            // use next downstream edge
            let mut splitIt = 0;
            let mut totalLength = 0.;
            while splitIt + 1 != splitList.len() && totalLength + splitList[splitIt].get_length() < pos {
                totalLength += splitList[splitIt].get_length();
                splitIt += 1;
            }
            splitList[splitIt]
        }
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getDepartConnector(const E *, const int) const` [const]
    /// Returns the departing intermodal connector at the given split offset
    // _IntermodalEdge* getDepartConnector(const E* e, const int splitIndex = 0) const {
    //         typename std::map<const E*, std::vector<_IntermodalEdge*> >::const_iterator it = myDepartLookup.find(e);
    //         if (it == myDepartLookup.end()) {
    //             throw ProcessError(TLF("Depart edge '%' not found in intermodal network.", e->getID()));
    //         }
    //         if (splitIndex >= (int)it->second.size()) {
    //             throw ProcessError("Split index " + toString(splitIndex) + " invalid for depart edge '" + e->getID() + "' .");
    //         }
    //         return it->second[splitIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_connector(&self, e: Ptr<E>, splitIndex: i32) -> Ptr<IntermodalEdge<E, L, N, V>> {
        let splitList = match self.myDepartLookup.get(&e) {
            Some(list) => list,
            None => panic!("{}", ProcessError::new(format!("Depart edge '{}' not found in intermodal network.", e_id_unbounded(e)))),
        };
        if splitIndex >= splitList.len() as i32 {
            panic!("{}", ProcessError::new(format!("Split index {} invalid for depart edge '{}' .", splitIndex, e_id_unbounded(e))));
        }
        splitList[splitIndex as usize]
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getArrivalEdge(const E *, const double) const` [const]
    /// Returns the arriving intermodal edge
    // _IntermodalEdge* getArrivalEdge(const E* e, const double pos) const {
    //         typename std::map<const E*, std::vector<_IntermodalEdge*> >::const_iterator it = myArrivalLookup.find(e);
    //         if (it == myArrivalLookup.end()) {
    //             throw ProcessError(TLF("Arrival edge '%' not found in intermodal network.", e->getID()));
    //         }
    //         const std::vector<_IntermodalEdge*>& splitList = it->second;
    //         typename std::vector<_IntermodalEdge*>::const_iterator splitIt = splitList.begin();
    //         double totalLength = 0.;
    //         while (splitIt != splitList.end() && totalLength + (*splitIt)->getLength() < pos) {
    //             totalLength += (*splitIt)->getLength();
    //             ++splitIt;
    //         }
    //         if (splitIt != splitList.end()) {
    //             return *splitIt;
    //         } else {
    //             return splitList.back();
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_edge(&self, e: Ptr<E>, pos: f64) -> Ptr<IntermodalEdge<E, L, N, V>> where E: RouterEdge<V, Node = N> + 'static, L: 'static, N: 'static, V: RouterVehicle + 'static {
        let splitList = match self.myArrivalLookup.get(&e) {
            Some(list) => list,
            None => panic!("{}", ProcessError::new(format!("Arrival edge '{}' not found in intermodal network.", e.id()))),
        };
        let mut splitIt = 0;
        let mut totalLength = 0.;
        while splitIt < splitList.len() && totalLength + splitList[splitIt].get_length() < pos {
            totalLength += splitList[splitIt].get_length();
            splitIt += 1;
        }
        if splitIt < splitList.len() {
            splitList[splitIt]
        } else {
            *splitList.last().unwrap()
        }
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getArrivalConnector(const E *, const int) const` [const]
    /// Returns the arriving intermodal connector at the given split offset
    // _IntermodalEdge* getArrivalConnector(const E* e, const int splitIndex = 0) const {
    //         return myArrivalLookup.find(e)->second[splitIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_connector(&self, e: Ptr<E>, splitIndex: i32) -> Ptr<IntermodalEdge<E, L, N, V>> {
        self.myArrivalLookup[&e][splitIndex as usize]
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getWalkingConnector(const E *) const` [const]
    /// Returns the outgoing pedestrian edge, which is either a walking area or a walking connector
    // _IntermodalEdge* getWalkingConnector(const E* e) const {
    //         typename std::map<const N*, _IntermodalEdge*>::const_iterator it = myWalkingConnectorLookup.find(e->getToJunction());
    //         if (it == myWalkingConnectorLookup.end()) {
    //             const L* const sidewalk = getSidewalk<E, L>(e);
    //             if (e->isInternal() || sidewalk == 0) {
    //                 return 0;
    //             }
    //             for (const auto& target : sidewalk->getOutgoingViaLanes()) {
    //                 if (target.first->getEdge().isWalkingArea()) {
    //                     return getBothDirections(&target.first->getEdge()).first;
    //                 }
    //             }
    //             return 0;
    //         }
    //         return it->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_walking_connector(&self, e: Ptr<E>) -> Ptr<IntermodalEdge<E, L, N, V>> where E: RouterEdge<V, Node = N> + 'static, L: 'static, N: 'static, V: RouterVehicle + 'static {
        match self.myWalkingConnectorLookup.get(&e_to_junction(e)) {
            Some(conn) => *conn,
            None => {
                let sidewalk = get_sidewalk::<E, L, V>(e, SVC_PEDESTRIAN);
                if e.is_internal() || sidewalk.is_null() {
                    return Ptr::null();
                }
                for target in l_outgoing_via_lanes::<L, E, V>(sidewalk) {
                    if e_is_walking_area(l_edge::<L, E, V>(target.0)) {
                        return self.bidi(l_edge::<L, E, V>(target.0)).0;
                    }
                }
                Ptr::null()
            }
        }
    }

    /// C++: `void IntermodalNetwork::addCarEdges(const std::vector<E *> &, double)`
    // void addCarEdges(const std::vector<E*>& edges, double taxiWait) {
    //         for (const E* const edge : edges) {
    //             if (edge->getFunction() == SumoXMLEdgeFunc::NORMAL || edge->getFunction() == SumoXMLEdgeFunc::INTERNAL) {
    //                 myCarLookup[edge] = new CarEdge<E, L, N, V>(myNumericalID++, edge);
    //                 addEdge(myCarLookup[edge]);
    //             }
    //         }
    //         for (const auto& edgePair : myCarLookup) {
    //             _IntermodalEdge* const carEdge = edgePair.second;
    //             // connectivity within the car network
    //             for (const auto& suc : edgePair.first->getViaSuccessors()) {
    //                 _IntermodalEdge* const sucCarEdge = getCarEdge(suc.first);
    //                 _IntermodalEdge* const sucViaEdge = getCarEdge(suc.second);
    //                 if (sucCarEdge != nullptr) {
    //                     carEdge->addSuccessor(sucCarEdge, sucViaEdge);
    //                 }
    //             }
    //             // connectivity to the pedestrian network (only for normal edges)
    //             if (edgePair.first->getFunction() != SumoXMLEdgeFunc::NORMAL) {
    //                 continue;
    //             }
    //             if ((myCarWalkTransfer & ALL_JUNCTIONS) != 0) {
    //                 _IntermodalEdge* const walkCon = getWalkingConnector(edgePair.first);
    //                 if (walkCon != 0) {
    //                     carEdge->addSuccessor(walkCon);
    //                 } else {
    //                     // we are on an edge where pedestrians are forbidden and want to continue on an arbitrary pedestrian edge
    //                     for (const E* const out : edgePair.first->getToJunction()->getOutgoing()) {
    //                         if (!out->isInternal() && !out->isTazConnector() && getSidewalk<E, L>(out) != 0) {
    //                             carEdge->addSuccessor(getBothDirections(out).first);
    //                         }
    //                     }
    //                     for (const E* const in : edgePair.first->getToJunction()->getIncoming()) {
    //                         if (!in->isInternal() && !in->isTazConnector() && getSidewalk<E, L>(in) != 0) {
    //                             carEdge->addSuccessor(getBothDirections(in).second);
    //                         }
    //                     }
    //                 }
    //             }
    //             if ((myCarWalkTransfer & ALL_JUNCTIONS) == 0 && (myCarWalkTransfer & TAXI_DROPOFF_ANYWHERE) != 0) {
    //                 // add access edges that allow exiting a taxi
    //                 _IntermodalEdge* const walkCon = getWalkingConnector(edgePair.first);
    //                 if (walkCon != 0) {
    //                     addRestrictedCarExit(carEdge, walkCon, gTaxiClasses);
    //                 } else {
    //                     // we are on an edge where pedestrians are forbidden and want to continue on an arbitrary pedestrian edge
    //                     for (const E* const out : edgePair.first->getToJunction()->getOutgoing()) {
    //                         if (!out->isInternal() && !out->isTazConnector() && getSidewalk<E, L>(out) != 0) {
    //                             addRestrictedCarExit(carEdge, getBothDirections(out).first, gTaxiClasses);
    //                         }
    //                     }
    //                     for (const E* const in : edgePair.first->getToJunction()->getIncoming()) {
    //                         if (!in->isInternal() && !in->isTazConnector() && getSidewalk<E, L>(in) != 0) {
    //                             addRestrictedCarExit(carEdge, getBothDirections(in).second, gTaxiClasses);
    //                         }
    //                     }
    //                 }
    //             }
    //             // use intermediate access edge that prevents taxi departure
    //             _IntermodalEdge* departConn = getDepartConnector(edgePair.first);
    //             _AccessEdge* access = new _AccessEdge(myNumericalID++, departConn, carEdge, 0, (SVCAll & ~SVC_TAXI));
    //             addEdge(access);
    //             departConn->addSuccessor(access);
    //             access->addSuccessor(carEdge);
    //             if ((myCarWalkTransfer & TAXI_PICKUP_ANYWHERE) != 0) {
    //                 // taxi may depart anywhere but there is a time penalty
    //                 _AccessEdge* taxiAccess = new _AccessEdge(myNumericalID++, departConn, carEdge, 0, SVC_TAXI, SVC_IGNORING, taxiWait);
    //                 addEdge(taxiAccess);
    //                 departConn->addSuccessor(taxiAccess);
    //                 taxiAccess->addSuccessor(carEdge);
    //             }
    //             if ((myCarWalkTransfer & TAXI_DROPOFF_ANYWHERE) != 0) {
    //                 // taxi (as all other cars) may arrive anywhere
    //                 carEdge->addSuccessor(getArrivalConnector(edgePair.first));
    //             } else {
    //                 // use intermediate access edge that prevents taxi arrival
    //                 addRestrictedCarExit(carEdge, getArrivalConnector(edgePair.first), (SVCAll & ~gTaxiClasses));
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_car_edges(&mut self, edges: &Vec<Ptr<E>>, taxiWait: f64) where E: RouterEdge<V, Node = N> + 'static, L: 'static, N: 'static, V: RouterVehicle + 'static {
        for &edge in edges {
            if e_has_normal_function(edge) || e_has_internal_function(edge) {
                let id = self.next_id();
                let carEdge = leak_edge(CarEdge::<E, L, N, V>::new(id, edge, -1.)).cast::<IntermodalEdge<E, L, N, V>>();
                self.myCarLookup.insert(edge, carEdge);
                self.add_edge(carEdge);
            }
        }
        // determinism: BTreeMap iterates by pointer, like the C++ std::map keyed by E*
        let carPairs: Vec<(Ptr<E>, Ptr<IntermodalEdge<E, L, N, V>>)> = self.myCarLookup.iter().map(|(k, v)| (*k, *v)).collect();
        let taxiClasses = g_taxi_classes_perm();
        for (edge, carEdge) in carPairs {
            // connectivity within the car network
            for &suc in edge.via_successors(SVC_IGNORING, false) {
                let sucCarEdge = self.get_car_edge(suc.0);
                let sucViaEdge = self.get_car_edge(suc.1);
                if !sucCarEdge.is_null() {
                    carEdge.m().add_successor(sucCarEdge, sucViaEdge);
                }
            }
            // connectivity to the pedestrian network (only for normal edges)
            if !e_has_normal_function(edge) {
                continue;
            }
            if (self.myCarWalkTransfer & ModeChangeOptions::ALL_JUNCTIONS as i32) != 0 {
                let walkCon = self.get_walking_connector(edge);
                if !walkCon.is_null() {
                    carEdge.m().add_successor(walkCon, Ptr::null());
                } else {
                    // we are on an edge where pedestrians are forbidden and want to continue on an arbitrary pedestrian edge
                    for out in n_outgoing::<E, N>(e_to_junction(edge)) {
                        if !out.is_internal() && !out.is_taz_connector() && !get_sidewalk::<E, L, V>(out, SVC_PEDESTRIAN).is_null() {
                            let target = self.bidi(out).0;
                            carEdge.m().add_successor(target, Ptr::null());
                        }
                    }
                    for inc in n_incoming::<E, N>(e_to_junction(edge)) {
                        if !inc.is_internal() && !inc.is_taz_connector() && !get_sidewalk::<E, L, V>(inc, SVC_PEDESTRIAN).is_null() {
                            let target = self.bidi(inc).1;
                            carEdge.m().add_successor(target, Ptr::null());
                        }
                    }
                }
            }
            if (self.myCarWalkTransfer & ModeChangeOptions::ALL_JUNCTIONS as i32) == 0 && (self.myCarWalkTransfer & ModeChangeOptions::TAXI_DROPOFF_ANYWHERE as i32) != 0 {
                // add access edges that allow exiting a taxi
                let walkCon = self.get_walking_connector(edge);
                if !walkCon.is_null() {
                    self.add_restricted_car_exit(carEdge, walkCon, taxiClasses);
                } else {
                    // we are on an edge where pedestrians are forbidden and want to continue on an arbitrary pedestrian edge
                    for out in n_outgoing::<E, N>(e_to_junction(edge)) {
                        if !out.is_internal() && !out.is_taz_connector() && !get_sidewalk::<E, L, V>(out, SVC_PEDESTRIAN).is_null() {
                            let target = self.bidi(out).0;
                            self.add_restricted_car_exit(carEdge, target, taxiClasses);
                        }
                    }
                    for inc in n_incoming::<E, N>(e_to_junction(edge)) {
                        if !inc.is_internal() && !inc.is_taz_connector() && !get_sidewalk::<E, L, V>(inc, SVC_PEDESTRIAN).is_null() {
                            let target = self.bidi(inc).1;
                            self.add_restricted_car_exit(carEdge, target, taxiClasses);
                        }
                    }
                }
            }
            // use intermediate access edge that prevents taxi departure
            let departConn = self.get_depart_connector(edge, 0);
            let id = self.next_id();
            let access = leak_edge(AccessEdge::new(id, departConn, carEdge, 0., SVC_ALL & !SVC_TAXI, SVC_IGNORING, -1.)).cast::<IntermodalEdge<E, L, N, V>>();
            self.add_edge(access);
            departConn.m().add_successor(access, Ptr::null());
            access.m().add_successor(carEdge, Ptr::null());
            if (self.myCarWalkTransfer & ModeChangeOptions::TAXI_PICKUP_ANYWHERE as i32) != 0 {
                // taxi may depart anywhere but there is a time penalty
                let id = self.next_id();
                let taxiAccess = leak_edge(AccessEdge::new(id, departConn, carEdge, 0., SVC_TAXI, SVC_IGNORING, taxiWait)).cast::<IntermodalEdge<E, L, N, V>>();
                self.add_edge(taxiAccess);
                departConn.m().add_successor(taxiAccess, Ptr::null());
                taxiAccess.m().add_successor(carEdge, Ptr::null());
            }
            if (self.myCarWalkTransfer & ModeChangeOptions::TAXI_DROPOFF_ANYWHERE as i32) != 0 {
                // taxi (as all other cars) may arrive anywhere
                let arrConn = self.get_arrival_connector(edge, 0);
                carEdge.m().add_successor(arrConn, Ptr::null());
            } else {
                // use intermediate access edge that prevents taxi arrival
                let arrConn = self.get_arrival_connector(edge, 0);
                self.add_restricted_car_exit(carEdge, arrConn, SVC_ALL & !taxiClasses);
            }
        }
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getCarEdge(const E *) const` [const]
    /// Returns the associated car edge
    // _IntermodalEdge* getCarEdge(const E* e) const {
    //         if (e == nullptr) {
    //             return nullptr;
    //         }
    //         auto it = myCarLookup.find(e);
    //         if (it == myCarLookup.end()) {
    //             return nullptr;
    //         }
    //         return it->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_edge(&self, e: Ptr<E>) -> Ptr<IntermodalEdge<E, L, N, V>> {
        if e.is_null() {
            return Ptr::null();
        }
        match self.myCarLookup.get(&e) {
            Some(carEdge) => *carEdge,
            None => Ptr::null(),
        }
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getStopEdge(const std::string &) const` [const]
    /// Returns the associated stop edge
    // _IntermodalEdge* getStopEdge(const std::string& stopId) const {
    //         auto it = myStopConnections.find(stopId);
    //         if (it == myStopConnections.end()) {
    //             return nullptr;
    //         }
    //         return it->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_stop_edge(&self, stopId: &str) -> Ptr<IntermodalEdge<E, L, N, V>> {
        match self.myStopConnections.get(stopId) {
            Some(stopEdge) => *stopEdge,
            None => Ptr::null(),
        }
    }

    /// C++: `void IntermodalNetwork::addAccess(const std::string &, const E *, const double, const double, const double, const SumoXMLTag, bool, double)`
    /// Adds access edges for stopping places to the intermodal network
    // void addAccess(const std::string& stopId, const E* stopEdge, const double startPos, const double endPos, const double length, const SumoXMLTag category, bool isAccess, double taxiWait) {
    //         assert(stopEdge != nullptr);
    //         const bool transferCarWalk = ((category == SUMO_TAG_PARKING_AREA && (myCarWalkTransfer & PARKING_AREAS) != 0) ||
    //                                       (category == SUMO_TAG_BUS_STOP && (myCarWalkTransfer & PT_STOPS) != 0));
    //         const bool transferTaxiWalk = ((category == SUMO_TAG_PARKING_AREA && (myCarWalkTransfer & TAXI_DROPOFF_PARKING_AREAS) != 0) ||
    //                                        (category == SUMO_TAG_BUS_STOP && (myCarWalkTransfer & TAXI_DROPOFF_PT) != 0));
    //         const bool transferWalkTaxi = ((category == SUMO_TAG_PARKING_AREA && (myCarWalkTransfer & TAXI_PICKUP_PARKING_AREAS) != 0) ||
    //                                        (category == SUMO_TAG_BUS_STOP && (myCarWalkTransfer & TAXI_PICKUP_PT) != 0));
    //         const double pos = (startPos + endPos) / 2.;
    // #ifdef IntermodalRouter_DEBUG_ACCESS
    //         std::cout << "addAccess stopId=" << stopId << " stopEdge=" << stopEdge->getID() << " pos=" << pos << " length=" << length << " tag=" << toString(category)
    //                   << " access=" << isAccess << " tWait=" << taxiWait << "\n";
    // #endif
    //         if (myStopConnections.count(stopId) == 0) {
    //             myStopConnections[stopId] = new StopEdge<E, L, N, V>(stopId, myNumericalID++, stopEdge, startPos, endPos);
    //             addEdge(myStopConnections[stopId]);
    //         }
    //         _IntermodalEdge* const stopConn = myStopConnections[stopId];
    //         const L* lane = getSidewalk<E, L>(stopEdge);
    //         if (lane != nullptr) {
    //             const std::pair<_IntermodalEdge*, _IntermodalEdge*>& pair = getBothDirections(stopEdge);
    //             double relPos;
    //             bool needSplit;
    //             const int splitIndex = findSplitIndex(pair.first, pos, relPos, needSplit);
    //             _IntermodalEdge* const fwdSplit = needSplit ? new PedestrianEdge<E, L, N, V>(myNumericalID++, stopEdge, lane, true, pos) : nullptr;
    //             splitEdge(pair.first, splitIndex, fwdSplit, relPos, length, needSplit, stopConn);
    //             _IntermodalEdge* const backSplit = needSplit ? new PedestrianEdge<E, L, N, V>(myNumericalID++, stopEdge, lane, false, pos) : nullptr;
    //             splitEdge(pair.second, splitIndex, backSplit, relPos, length, needSplit, stopConn, false);
    //             _IntermodalEdge* carSplit = nullptr;
    //             if (myCarLookup.count(stopEdge) > 0) {
    //                 if (needSplit) {
    //                     carSplit = new CarEdge<E, L, N, V>(myNumericalID++, stopEdge, pos);
    //                 }
    //                 splitEdge(myCarLookup[stopEdge], splitIndex, carSplit, relPos, length, needSplit, stopConn, true, false, transferCarWalk);
    //             }
    //             if (needSplit) {
    //                 if (carSplit != nullptr && (transferCarWalk || transferTaxiWalk)) {
    //                     // adding access from car to walk
    //                     _IntermodalEdge* const beforeSplit = myAccessSplits[myCarLookup[stopEdge]][splitIndex];
    //                     for (_IntermodalEdge* conn : {
    //                                 fwdSplit, backSplit
    //                             }) {
    //                         if (transferCarWalk) {
    //                             _AccessEdge* access = new _AccessEdge(myNumericalID++, beforeSplit, conn, length);
    //                             addEdge(access);
    //                             beforeSplit->addSuccessor(access);
    //                             access->addSuccessor(conn);
    //                         } else if (transferTaxiWalk) {
    //                             addRestrictedCarExit(beforeSplit, stopConn, gTaxiClasses);
    //                         }
    //                     }
    //                 }
    //                 if (carSplit != nullptr && transferWalkTaxi && !isAccess) {
    //                     _AccessEdge* access = new _AccessEdge(myNumericalID++, stopConn, carSplit, 0, SVC_TAXI, SVC_IGNORING, taxiWait);
    //                     addEdge(access);
    //                     stopConn->addSuccessor(access);
    //                     access->addSuccessor(carSplit);
    //                 }
    //
    //                 // fixing depart connections for the forward pedestrian, the backward pedestrian and the car edge
    //                 _IntermodalEdge* const prevDep = getDepartConnector(stopEdge, splitIndex);
    //                 const std::vector<_IntermodalEdge*>& backSplitList = myAccessSplits[pair.second];
    //                 _IntermodalEdge* const backBeforeSplit = backSplitList[backSplitList.size() - 2 - splitIndex];
    //                 _IntermodalEdge* const depConn = new _IntermodalEdge(stopEdge->getID() + "_depart_connector" + toString(pos), myNumericalID++, stopEdge, "!connector");
    //                 depConn->addSuccessor(fwdSplit);
    //                 depConn->addSuccessor(backBeforeSplit);
    //                 depConn->setLength(fwdSplit->getLength());
    //                 prevDep->removeSuccessor(backBeforeSplit);
    //                 prevDep->addSuccessor(backSplit);
    //                 prevDep->setLength(backSplit->getLength());
    //                 if (carSplit != nullptr) {
    //                     depConn->addSuccessor(carSplit);
    //                 }
    //
    //                 // fixing arrival connections for the forward pedestrian, the backward pedestrian and the car edge
    //                 _IntermodalEdge* const prevArr = getArrivalConnector(stopEdge, splitIndex);
    //                 _IntermodalEdge* const fwdBeforeSplit = myAccessSplits[pair.first][splitIndex];
    //                 _IntermodalEdge* const arrConn = new _IntermodalEdge(stopEdge->getID() + "_arrival_connector" + toString(pos), myNumericalID++, stopEdge, "!connector");
    //                 fwdSplit->addSuccessor(arrConn);
    //                 backBeforeSplit->addSuccessor(arrConn);
    //                 arrConn->setLength(fwdSplit->getLength());
    //                 fwdSplit->removeSuccessor(prevArr);
    //                 fwdBeforeSplit->addSuccessor(prevArr);
    //                 prevArr->setLength(backSplit->getLength());
    //                 if (carSplit != nullptr) {
    //                     if (carSplit->removeSuccessor(prevArr)) {
    //                         carSplit->addSuccessor(arrConn);
    //                         myAccessSplits[myCarLookup[stopEdge]][splitIndex]->addSuccessor(prevArr);
    //                     } else {
    //                         // check for restricted access
    //                         for (_IntermodalEdge* out : carSplit->getSuccessors()) {
    //                             _AccessEdge* aOut = dynamic_cast<_AccessEdge*>(out);
    //                             if (aOut != nullptr && aOut->removeSuccessor(prevArr)) {
    //                                 aOut->addSuccessor(arrConn);
    //                                 addRestrictedCarExit(
    //                                     myAccessSplits[myCarLookup[stopEdge]][splitIndex],
    //                                     prevArr,
    //                                     aOut->getVehicleRetriction());
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                 }
    //                 addConnectors(depConn, arrConn, splitIndex + 1);
    //             }
    //         } else {
    //             // pedestrians cannot walk here:
    //             // add stop edge as depart connector so that pedestrians may start at the stop
    //             std::vector<_IntermodalEdge*>& splitList = myDepartLookup[stopEdge];
    //             assert(splitList.size() > 0);
    //             typename std::vector<_IntermodalEdge*>::iterator splitIt = splitList.begin();
    //             while (splitIt != splitList.end() && startPos > (*splitIt)->getEndPos()) {
    //                 ++splitIt;
    //             }
    //             splitList.insert(splitIt, stopConn);
    //
    //             if (!isAccess && (transferWalkTaxi || transferCarWalk || transferTaxiWalk)) {
    //                 _IntermodalEdge* carEdge =  myCarLookup[stopEdge];
    //                 double relPos;
    //                 bool needSplit;
    //                 const int splitIndex = findSplitIndex(carEdge, pos, relPos, needSplit);
    //                 if (needSplit) {
    //                     _IntermodalEdge* carSplit = new CarEdge<E, L, N, V>(myNumericalID++, stopEdge, pos);
    //                     splitEdge(carEdge, splitIndex, carSplit, relPos, length, needSplit, stopConn, true, false, false);
    //
    //                     if (transferCarWalk || transferTaxiWalk) {
    //                         // adding access from car to walk
    //                         _IntermodalEdge* const beforeSplit = myAccessSplits[myCarLookup[stopEdge]][splitIndex];
    //                         if (transferCarWalk) {
    //                             _AccessEdge* access = new _AccessEdge(myNumericalID++, beforeSplit, stopConn, length);
    //                             addEdge(access);
    //                             beforeSplit->addSuccessor(access);
    //                             access->addSuccessor(stopConn);
    //                         } else if (transferTaxiWalk) {
    //                             addRestrictedCarExit(beforeSplit, stopConn, gTaxiClasses);
    //                         }
    //                     }
    //                     if (transferWalkTaxi) {
    //                         _AccessEdge* access = new _AccessEdge(myNumericalID++, stopConn, carSplit, 0, SVC_TAXI, SVC_IGNORING, taxiWait);
    //                         addEdge(access);
    //                         stopConn->addSuccessor(access);
    //                         access->addSuccessor(carSplit);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_access(&mut self, stopId: &str, stopEdge: Ptr<E>, startPos: f64, endPos: f64, length: f64, category: SumoXMLTag, isAccess: bool, taxiWait: f64) where E: RouterEdge<V, Node = N> + 'static, L: 'static, N: 'static, V: RouterVehicle + 'static {
        assert!(!stopEdge.is_null());
        let transferCarWalk = (category == SUMO_TAG_PARKING_AREA && (self.myCarWalkTransfer & ModeChangeOptions::PARKING_AREAS as i32) != 0)
            || (category == SUMO_TAG_BUS_STOP && (self.myCarWalkTransfer & ModeChangeOptions::PT_STOPS as i32) != 0);
        let transferTaxiWalk = (category == SUMO_TAG_PARKING_AREA && (self.myCarWalkTransfer & ModeChangeOptions::TAXI_DROPOFF_PARKING_AREAS as i32) != 0)
            || (category == SUMO_TAG_BUS_STOP && (self.myCarWalkTransfer & ModeChangeOptions::TAXI_DROPOFF_PT as i32) != 0);
        let transferWalkTaxi = (category == SUMO_TAG_PARKING_AREA && (self.myCarWalkTransfer & ModeChangeOptions::TAXI_PICKUP_PARKING_AREAS as i32) != 0)
            || (category == SUMO_TAG_BUS_STOP && (self.myCarWalkTransfer & ModeChangeOptions::TAXI_PICKUP_PT as i32) != 0);
        let pos = (startPos + endPos) / 2.;

        if !self.myStopConnections.contains_key(stopId) {
            let id = self.next_id();
            let stop = leak_edge(StopEdge::<E, L, N, V>::new(stopId.to_string(), id, stopEdge, startPos, endPos)).cast::<IntermodalEdge<E, L, N, V>>();
            self.myStopConnections.insert(stopId.to_string(), stop);
            self.add_edge(stop);
        }
        let stopConn = self.myStopConnections[stopId];
        let lane = get_sidewalk::<E, L, V>(stopEdge, SVC_PEDESTRIAN);
        if !lane.is_null() {
            let pair = self.bidi(stopEdge);
            let mut relPos = 0.;
            let mut needSplit = false;
            let splitIndex = self.find_split_index(pair.0, pos, &mut relPos, &mut needSplit);
            let fwdSplit: Ptr<IntermodalEdge<E, L, N, V>> = if needSplit {
                let id = self.next_id();
                leak_edge(PedestrianEdge::<E, L, N, V>::new(id, stopEdge, lane, true, pos)).cast()
            } else {
                Ptr::null()
            };
            self.split_edge(pair.0, splitIndex, fwdSplit, relPos, length, needSplit, stopConn, true, true, true);
            let backSplit: Ptr<IntermodalEdge<E, L, N, V>> = if needSplit {
                let id = self.next_id();
                leak_edge(PedestrianEdge::<E, L, N, V>::new(id, stopEdge, lane, false, pos)).cast()
            } else {
                Ptr::null()
            };
            self.split_edge(pair.1, splitIndex, backSplit, relPos, length, needSplit, stopConn, false, true, true);
            let mut carSplit: Ptr<IntermodalEdge<E, L, N, V>> = Ptr::null();
            if self.myCarLookup.contains_key(&stopEdge) {
                if needSplit {
                    let id = self.next_id();
                    carSplit = leak_edge(CarEdge::<E, L, N, V>::new(id, stopEdge, pos)).cast();
                }
                let carEdge = self.myCarLookup[&stopEdge];
                self.split_edge(carEdge, splitIndex, carSplit, relPos, length, needSplit, stopConn, true, false, transferCarWalk);
            }
            if needSplit {
                if !carSplit.is_null() && (transferCarWalk || transferTaxiWalk) {
                    // adding access from car to walk
                    let beforeSplit = self.myAccessSplits[&self.myCarLookup[&stopEdge]][splitIndex as usize];
                    for conn in [fwdSplit, backSplit] {
                        if transferCarWalk {
                            let id = self.next_id();
                            let access = leak_edge(AccessEdge::new(id, beforeSplit, conn, length, SVC_IGNORING, SVC_IGNORING, -1.)).cast::<IntermodalEdge<E, L, N, V>>();
                            self.add_edge(access);
                            beforeSplit.m().add_successor(access, Ptr::null());
                            access.m().add_successor(conn, Ptr::null());
                        } else if transferTaxiWalk {
                            self.add_restricted_car_exit(beforeSplit, stopConn, g_taxi_classes_perm());
                        }
                    }
                }
                if !carSplit.is_null() && transferWalkTaxi && !isAccess {
                    let id = self.next_id();
                    let access = leak_edge(AccessEdge::new(id, stopConn, carSplit, 0., SVC_TAXI, SVC_IGNORING, taxiWait)).cast::<IntermodalEdge<E, L, N, V>>();
                    self.add_edge(access);
                    stopConn.m().add_successor(access, Ptr::null());
                    access.m().add_successor(carSplit, Ptr::null());
                }

                // fixing depart connections for the forward pedestrian, the backward pedestrian and the car edge
                let prevDep = self.get_depart_connector(stopEdge, splitIndex);
                let backSplitList = self.myAccessSplits[&pair.1].clone();
                let backBeforeSplit = backSplitList[(backSplitList.len() as i64 - 2 - splitIndex as i64) as usize];
                let id = self.next_id();
                let depConn = leak_edge(IntermodalEdge::<E, L, N, V>::new(stopEdge.id().to_string() + "_depart_connector" + &to_string(&pos), id, stopEdge, "!connector", -1.));
                depConn.m().add_successor(fwdSplit, Ptr::null());
                depConn.m().add_successor(backBeforeSplit, Ptr::null());
                depConn.m().set_length(fwdSplit.get_length());
                prevDep.m().remove_successor(backBeforeSplit);
                prevDep.m().add_successor(backSplit, Ptr::null());
                prevDep.m().set_length(backSplit.get_length());
                if !carSplit.is_null() {
                    depConn.m().add_successor(carSplit, Ptr::null());
                }

                // fixing arrival connections for the forward pedestrian, the backward pedestrian and the car edge
                let prevArr = self.get_arrival_connector(stopEdge, splitIndex);
                let fwdBeforeSplit = self.myAccessSplits[&pair.0][splitIndex as usize];
                let id = self.next_id();
                let arrConn = leak_edge(IntermodalEdge::<E, L, N, V>::new(stopEdge.id().to_string() + "_arrival_connector" + &to_string(&pos), id, stopEdge, "!connector", -1.));
                fwdSplit.m().add_successor(arrConn, Ptr::null());
                backBeforeSplit.m().add_successor(arrConn, Ptr::null());
                arrConn.m().set_length(fwdSplit.get_length());
                fwdSplit.m().remove_successor(prevArr);
                fwdBeforeSplit.m().add_successor(prevArr, Ptr::null());
                prevArr.m().set_length(backSplit.get_length());
                if !carSplit.is_null() {
                    if carSplit.m().remove_successor(prevArr) {
                        carSplit.m().add_successor(arrConn, Ptr::null());
                        let carSplitBefore = self.myAccessSplits[&self.myCarLookup[&stopEdge]][splitIndex as usize];
                        carSplitBefore.m().add_successor(prevArr, Ptr::null());
                    } else {
                        // check for restricted access
                        for out in carSplit.get_successors(SVC_IGNORING).clone() {
                            let aOut = as_access_edge::<E, L, N, V>(out);
                            if !aOut.is_null() && out.m().remove_successor(prevArr) {
                                out.m().add_successor(arrConn, Ptr::null());
                                let carSplitBefore = self.myAccessSplits[&self.myCarLookup[&stopEdge]][splitIndex as usize];
                                self.add_restricted_car_exit(carSplitBefore, prevArr, aOut.get_vehicle_retriction());
                                break;
                            }
                        }
                    }
                }
                self.add_connectors(depConn, arrConn, splitIndex + 1);
            }
        } else {
            // pedestrians cannot walk here:
            // add stop edge as depart connector so that pedestrians may start at the stop
            let mut splitIt = 0;
            let splitList = self.myDepartLookup.entry(stopEdge).or_default();
            assert!(!splitList.is_empty());
            while splitIt < splitList.len() && startPos > splitList[splitIt].get_end_pos() {
                splitIt += 1;
            }
            splitList.insert(splitIt, stopConn);

            if !isAccess && (transferWalkTaxi || transferCarWalk || transferTaxiWalk) {
                let carEdge = self.myCarLookup[&stopEdge];
                let mut relPos = 0.;
                let mut needSplit = false;
                let splitIndex = self.find_split_index(carEdge, pos, &mut relPos, &mut needSplit);
                if needSplit {
                    let id = self.next_id();
                    let carSplit: Ptr<IntermodalEdge<E, L, N, V>> = leak_edge(CarEdge::<E, L, N, V>::new(id, stopEdge, pos)).cast();
                    self.split_edge(carEdge, splitIndex, carSplit, relPos, length, needSplit, stopConn, true, false, false);

                    if transferCarWalk || transferTaxiWalk {
                        // adding access from car to walk
                        let beforeSplit = self.myAccessSplits[&carEdge][splitIndex as usize];
                        if transferCarWalk {
                            let id = self.next_id();
                            let access = leak_edge(AccessEdge::new(id, beforeSplit, stopConn, length, SVC_IGNORING, SVC_IGNORING, -1.)).cast::<IntermodalEdge<E, L, N, V>>();
                            self.add_edge(access);
                            beforeSplit.m().add_successor(access, Ptr::null());
                            access.m().add_successor(stopConn, Ptr::null());
                        } else if transferTaxiWalk {
                            self.add_restricted_car_exit(beforeSplit, stopConn, g_taxi_classes_perm());
                        }
                    }
                    if transferWalkTaxi {
                        let id = self.next_id();
                        let access = leak_edge(AccessEdge::new(id, stopConn, carSplit, 0., SVC_TAXI, SVC_IGNORING, taxiWait)).cast::<IntermodalEdge<E, L, N, V>>();
                        self.add_edge(access);
                        stopConn.m().add_successor(access, Ptr::null());
                        access.m().add_successor(carSplit, Ptr::null());
                    }
                }
            }
        }
    }

    /// C++: `bool IntermodalNetwork::hasPTSchedules() const` [const]
    // bool hasPTSchedules() const {
    //         return myHavePTSchedules;
    //     }
    #[allow(unused_variables)]
    pub fn has_pt_schedules(&self) -> bool {
        self.myHavePTSchedules
    }

    /// C++: `void IntermodalNetwork::addSchedule(const SUMOVehicleParameter &, const StopParVector *)`
    // void addSchedule(const SUMOVehicleParameter& pars, const StopParVector* addStops = nullptr) {
    //         SUMOTime lastUntil = 0;
    //         StopParVector validStops;
    //         if (addStops != nullptr) {
    //             // stops are part of a stand-alone route. until times are offsets from vehicle departure
    //             for (const SUMOVehicleParameter::Stop& stop : *addStops) {
    //                 if (myStopConnections.count(stop.busstop) > 0) {
    //                     // compute stop times for the first vehicle
    //                     const SUMOTime newUntil = stop.until + pars.depart;
    //                     if (newUntil >= lastUntil) {
    //                         validStops.push_back(stop);
    //                         validStops.back().until = newUntil;
    //                         lastUntil = newUntil;
    //                     } else {
    //                         WRITE_WARNINGF(TL("Ignoring unordered stop at '%' until % for vehicle '%'."), stop.busstop, time2string(stop.until), pars.id);
    //                     }
    //                 }
    //             }
    //         }
    //         for (const SUMOVehicleParameter::Stop& stop : pars.stops) {
    //             // stops are part of the vehicle until times are absolute times for the first vehicle
    //             if (myStopConnections.count(stop.busstop) > 0 && stop.until >= lastUntil) {
    //                 validStops.push_back(stop);
    //                 lastUntil = stop.until;
    //             } else {
    //                 if (stop.busstop != "" && stop.until >= 0) {
    //                     WRITE_WARNINGF(TL("Ignoring stop at '%' until % for vehicle '%'."), stop.busstop, time2string(stop.until), pars.id);
    //                 }
    //             }
    //         }
    //         if (validStops.size() < 2 && pars.line != "taxi") {
    //             WRITE_WARNINGF(TL("Not using public transport line '%' for routing persons. It has less than two usable stops."), pars.line);
    //             return;
    //         }
    //
    //         typename std::vector<_PTEdge*>& lineEdges = myPTLines[pars.line];
    //         if (lineEdges.empty()) {
    //             _IntermodalEdge* lastStop = nullptr;
    //             Position lastPos;
    //             SUMOTime lastTime = 0;
    //             for (const SUMOVehicleParameter::Stop& s : validStops) {
    //                 _IntermodalEdge* currStop = myStopConnections[s.busstop];
    //                 Position stopPos = E::getStopPosition(s);
    //                 if (lastStop != nullptr) {
    //                     _PTEdge* const newEdge = new _PTEdge(s.busstop, myNumericalID++, lastStop, currStop->getEdge(), pars.line, lastPos.distanceTo(stopPos));
    //                     addEdge(newEdge);
    //                     newEdge->addSchedule(pars.id, lastTime, pars.repetitionNumber, pars.repetitionOffset, s.until - lastTime);
    //                     myHavePTSchedules = true;
    //                     lastStop->addSuccessor(newEdge);
    //                     newEdge->addSuccessor(currStop);
    //                     lineEdges.push_back(newEdge);
    //                 }
    //                 lastTime = s.until;
    //                 lastStop = currStop;
    //                 lastPos = stopPos;
    //             }
    //             if (pars.line != "taxi" && validStops.front().busstop == validStops.back().busstop) {
    //                 myLoopedLines.insert(pars.line);
    //             }
    //         } else {
    //             if (validStops.size() != lineEdges.size() + 1) {
    //                 WRITE_WARNINGF("Number of stops for public transport line '%' does not match earlier definitions, ignoring schedule.", pars.line);
    //                 return;
    //             }
    //             if (lineEdges.front()->getEntryStop() != myStopConnections[validStops.front().busstop]) {
    //                 WRITE_WARNINGF("Different stop for '%' compared to earlier definitions, ignoring schedule.", pars.line);
    //                 return;
    //             }
    //             typename std::vector<_PTEdge*>::const_iterator lineEdge = lineEdges.begin();
    //             typename StopParVector::const_iterator s = validStops.begin() + 1;
    //             for (; s != validStops.end(); ++s, ++lineEdge) {
    //                 if ((*lineEdge)->getSuccessors(SVC_IGNORING)[0] != myStopConnections[s->busstop]) {
    //                     WRITE_WARNINGF("Different stop for '%' compared to earlier definitions, ignoring schedule.", pars.line);
    //                     return;
    //                 }
    //             }
    //             SUMOTime lastTime = validStops.front().until;
    //             if (lineEdges.front()->hasSchedule(lastTime)) {
    //                 WRITE_WARNINGF("Duplicate schedule for '%' at time=%.", pars.line, time2string(lastTime));
    //             }
    //             for (lineEdge = lineEdges.begin(), s = validStops.begin() + 1; lineEdge != lineEdges.end(); ++lineEdge, ++s) {
    //                 (*lineEdge)->addSchedule(pars.id, lastTime, pars.repetitionNumber, pars.repetitionOffset, s->until - lastTime);
    //                 myHavePTSchedules = true;
    //                 lastTime = s->until;
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_schedule(&mut self, pars: &SUMOVehicleParameter, addStops: Ptr<StopParVector>) where E: RouterEdge<V>, V: RouterVehicle {
        let mut lastUntil: SUMOTime = 0;
        let mut validStops: StopParVector = Vec::new();
        if !addStops.is_null() {
            // stops are part of a stand-alone route. until times are offsets from vehicle departure
            for stop in addStops.m().iter() {
                if self.myStopConnections.contains_key(&stop.busstop) {
                    // compute stop times for the first vehicle
                    let newUntil = stop.until + pars.depart;
                    if newUntil >= lastUntil {
                        validStops.push(stop.clone());
                        validStops.last_mut().unwrap().until = newUntil;
                        lastUntil = newUntil;
                    } else {
                        write_warningf!("Ignoring unordered stop at '%' until % for vehicle '%'.", stop.busstop, time2string(stop.until), pars.id);
                    }
                }
            }
        }
        for stop in &pars.stops {
            // stops are part of the vehicle until times are absolute times for the first vehicle
            if self.myStopConnections.contains_key(&stop.busstop) && stop.until >= lastUntil {
                validStops.push(stop.clone());
                lastUntil = stop.until;
            } else if stop.busstop != "" && stop.until >= 0 {
                write_warningf!("Ignoring stop at '%' until % for vehicle '%'.", stop.busstop, time2string(stop.until), pars.id);
            }
        }
        if validStops.len() < 2 && pars.line != "taxi" {
            write_warningf!("Not using public transport line '%' for routing persons. It has less than two usable stops.", pars.line);
            return;
        }

        let lineEdgesEmpty = self.myPTLines.entry(pars.line.clone()).or_default().is_empty();
        if lineEdgesEmpty {
            let mut lastStop: Ptr<IntermodalEdge<E, L, N, V>> = Ptr::null();
            let mut lastPos = Position::default();
            let mut lastTime: SUMOTime = 0;
            for s in &validStops {
                let currStop = self.myStopConnections[&s.busstop];
                let stopPos = e_stop_position::<E, V>(s);
                if !lastStop.is_null() {
                    let id = self.next_id();
                    let newEdge = leak_edge(PublicTransportEdge::<E, L, N, V>::new(s.busstop.clone(), id, lastStop, currStop.get_edge(), &pars.line, lastPos.distance_to(&stopPos)));
                    self.add_edge(newEdge.cast());
                    newEdge.m().add_schedule(pars.id.clone(), lastTime, pars.repetitionNumber, pars.repetitionOffset, s.until - lastTime);
                    self.myHavePTSchedules = true;
                    let newBase: Ptr<IntermodalEdge<E, L, N, V>> = newEdge.cast();
                    lastStop.m().add_successor(newBase, Ptr::null());
                    newBase.m().add_successor(currStop, Ptr::null());
                    self.myPTLines.entry(pars.line.clone()).or_default().push(newEdge);
                }
                lastTime = s.until;
                lastStop = currStop;
                lastPos = stopPos;
            }
            if pars.line != "taxi" && validStops.first().unwrap().busstop == validStops.last().unwrap().busstop {
                self.myLoopedLines.insert(pars.line.clone());
            }
        } else {
            let lineEdges = self.myPTLines[&pars.line].clone();
            if validStops.len() != lineEdges.len() + 1 {
                write_warningf!("Number of stops for public transport line '%' does not match earlier definitions, ignoring schedule.", pars.line);
                return;
            }
            if lineEdges[0].get_entry_stop() != self.myStopConnections[&validStops[0].busstop] {
                write_warningf!("Different stop for '%' compared to earlier definitions, ignoring schedule.", pars.line);
                return;
            }
            for (idx, s) in validStops.iter().skip(1).enumerate() {
                if lineEdges[idx].base.get_successors(SVC_IGNORING)[0] != self.myStopConnections[&s.busstop] {
                    write_warningf!("Different stop for '%' compared to earlier definitions, ignoring schedule.", pars.line);
                    return;
                }
            }
            let mut lastTime = validStops[0].until;
            if lineEdges[0].has_schedule(lastTime) {
                write_warningf!("Duplicate schedule for '%' at time=%.", pars.line, time2string(lastTime));
            }
            for (idx, s) in validStops.iter().skip(1).enumerate() {
                lineEdges[idx].m().add_schedule(pars.id.clone(), lastTime, pars.repetitionNumber, pars.repetitionOffset, s.until - lastTime);
                self.myHavePTSchedules = true;
                lastTime = s.until;
            }
        }
    }

    /// C++: `void IntermodalNetwork::addCarAccess(const E *, SUMOVehicleClass, double)`
    /// Adds access edges for transfering from walking to vehicle use
    // void addCarAccess(const E* edge, SUMOVehicleClass svc, double traveltime) {
    //         assert(edge != nullptr);
    //         assert(myCarLookup.count(edge) != 0);
    //         assert(myBidiLookup.count(edge) != 0);
    //         EdgePair pedestrianEdges = myBidiLookup[edge];
    //         _IntermodalEdge* carEdge = myCarLookup[edge];
    //         _AccessEdge* access = new _AccessEdge(myNumericalID++, pedestrianEdges.first, carEdge, 0, svc, SVC_IGNORING, traveltime);
    //         addEdge(access);
    //         pedestrianEdges.first->addSuccessor(access);
    //         pedestrianEdges.second->addSuccessor(access);
    //         access->addSuccessor(carEdge);
    //     }
    #[allow(unused_variables)]
    pub fn add_car_access(&mut self, edge: Ptr<E>, svc: SUMOVehicleClass, traveltime: f64) {
        assert!(!edge.is_null());
        assert!(self.myCarLookup.contains_key(&edge));
        assert!(self.myBidiLookup.contains_key(&edge));
        let pedestrianEdges = self.myBidiLookup[&edge];
        let carEdge = self.myCarLookup[&edge];
        let id = self.next_id();
        let access = leak_edge(AccessEdge::new(id, pedestrianEdges.0, carEdge, 0., svc as SVCPermissions, SVC_IGNORING, traveltime)).cast::<IntermodalEdge<E, L, N, V>>();
        self.add_edge(access);
        pedestrianEdges.0.m().add_successor(access, Ptr::null());
        pedestrianEdges.1.m().add_successor(access, Ptr::null());
        access.m().add_successor(carEdge, Ptr::null());
    }

    /// C++: `void IntermodalNetwork::addRestrictedCarExit(_IntermodalEdge *, _IntermodalEdge *, SVCPermissions)`
    /// Adds access edges for transfering from driving to walking that are only usable by a particular vehicle class
    // void addRestrictedCarExit(_IntermodalEdge* from, _IntermodalEdge* to, SVCPermissions vehicleRestriction) {
    //         _AccessEdge* access = new _AccessEdge(myNumericalID++, from, to, 0, SVC_IGNORING, vehicleRestriction);
    //         addEdge(access);
    //         from->addSuccessor(access);
    //         access->addSuccessor(to);
    //     }
    #[allow(unused_variables)]
    pub fn add_restricted_car_exit(&mut self, from: Ptr<IntermodalEdge<E, L, N, V>>, to: Ptr<IntermodalEdge<E, L, N, V>>, vehicleRestriction: SVCPermissions) {
        let id = self.next_id();
        let access = leak_edge(AccessEdge::new(id, from, to, 0., SVC_IGNORING, vehicleRestriction, -1.)).cast::<IntermodalEdge<E, L, N, V>>();
        self.add_edge(access);
        from.m().add_successor(access, Ptr::null());
        access.m().add_successor(to, Ptr::null());
    }

    /// C++: `bool IntermodalNetwork::isLooped(const std::string) const` [const]
    // bool isLooped(const std::string lineID) const {
    //         return myLoopedLines.count(lineID) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_looped(&self, lineID: String) -> bool {
        self.myLoopedLines.contains(&lineID)
    }

    /// C++: `int IntermodalNetwork::findSplitIndex(_IntermodalEdge *const, const double, double &, bool &) const` [const, private]
    /// Returns where to insert or use the split edge
    // int findSplitIndex(_IntermodalEdge* const toSplit, const double pos, double& relPos, bool& needSplit) const {
    //         relPos = pos;
    //         needSplit = true;
    //         int splitIndex = 0;
    //         const auto& splitList = myAccessSplits.find(toSplit);
    //         if (splitList != myAccessSplits.end() && !splitList->second.empty()) {
    //             for (const _IntermodalEdge* const split : splitList->second) {
    //                 if (relPos < split->getLength() + POSITION_EPS) {
    //                     break;
    //                 }
    //                 relPos -= split->getLength();
    //                 splitIndex++;
    //             }
    //             assert(splitIndex < (int)splitList->second.size());
    //             if (splitIndex + 1 < (int)splitList->second.size() && fabs(relPos - splitList->second[splitIndex]->getLength()) < POSITION_EPS) {
    //                 needSplit = false;
    //             }
    //         }
    //         return splitIndex;
    //     }
    #[allow(unused_variables)]
    pub fn find_split_index(&self, toSplit: Ptr<IntermodalEdge<E, L, N, V>>, pos: f64, relPos: &mut f64, needSplit: &mut bool) -> i32 {
        *relPos = pos;
        *needSplit = true;
        let mut splitIndex: i32 = 0;
        if let Some(splitList) = self.myAccessSplits.get(&toSplit) {
            if !splitList.is_empty() {
                for split in splitList {
                    if *relPos < split.get_length() + POSITION_EPS {
                        break;
                    }
                    *relPos -= split.get_length();
                    splitIndex += 1;
                }
                assert!((splitIndex as usize) < splitList.len());
                if (splitIndex as usize) + 1 < splitList.len() && (*relPos - splitList[splitIndex as usize].get_length()).abs() < POSITION_EPS {
                    *needSplit = false;
                }
            }
        }
        splitIndex
    }

    /// C++: `void IntermodalNetwork::splitEdge(_IntermodalEdge *const, int, _IntermodalEdge *, const double, const double, const bool, _IntermodalEdge *const, const bool, const bool, const bool)` [private]
    /// Splits an edge (if necessary) and connects it to a stopping edge
    // void splitEdge(_IntermodalEdge* const toSplit, int splitIndex,
    //                    _IntermodalEdge* afterSplit, const double relPos, const double length, const bool needSplit,
    //                    _IntermodalEdge* const stopConn, const bool forward = true, const bool addExit = true, const bool addEntry = true) {
    //         std::vector<_IntermodalEdge*>& splitList = myAccessSplits[toSplit];
    //         if (splitList.empty()) {
    //             splitList.push_back(toSplit);
    //         }
    //         if (!forward) {
    //             splitIndex = (int)splitList.size() - 1 - splitIndex;
    //             if (!needSplit) {
    //                 splitIndex--;
    //             }
    //         }
    //         _IntermodalEdge* beforeSplit = splitList[splitIndex];
    //         if (needSplit) {
    //             addEdge(afterSplit);
    //             beforeSplit->transferSuccessors(afterSplit);
    //             beforeSplit->addSuccessor(afterSplit);
    //             if (forward) {
    //                 afterSplit->setLength(MAX2(0.0, beforeSplit->getLength() - relPos));
    //                 beforeSplit->setLength(relPos);
    //             } else {
    //                 afterSplit->setLength(relPos);
    //                 beforeSplit->setLength(MAX2(0.0, beforeSplit->getLength() - relPos));
    //                 // rename backward edges for easier referencing
    //                 const std::string newID = beforeSplit->getID();
    //                 beforeSplit->setID(afterSplit->getID());
    //                 afterSplit->setID(newID);
    //             }
    //             splitList.insert(splitList.begin() + splitIndex + 1, afterSplit);
    //         } else {
    //             // don't split, use the present split edges
    //             afterSplit = splitList[splitIndex + 1];
    //         }
    //         // add access to / from edge
    //         if (addEntry) {
    //             _AccessEdge* access = new _AccessEdge(myNumericalID++, beforeSplit, stopConn, length);
    //             addEdge(access);
    //             beforeSplit->addSuccessor(access);
    //             access->addSuccessor(stopConn);
    //         }
    //         if (addExit) {
    //             // pedestrian case only, exit from public to pedestrian
    //             _AccessEdge* exit = new _AccessEdge(myNumericalID++, stopConn, afterSplit, length);
    //             addEdge(exit);
    //             stopConn->addSuccessor(exit);
    //             exit->addSuccessor(afterSplit);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn split_edge(&mut self, toSplit: Ptr<IntermodalEdge<E, L, N, V>>, mut splitIndex: i32, mut afterSplit: Ptr<IntermodalEdge<E, L, N, V>>, relPos: f64, length: f64, needSplit: bool, stopConn: Ptr<IntermodalEdge<E, L, N, V>>, forward: bool, addExit: bool, addEntry: bool) {
        let splitList = self.myAccessSplits.entry(toSplit).or_default();
        if splitList.is_empty() {
            splitList.push(toSplit);
        }
        if !forward {
            splitIndex = splitList.len() as i32 - 1 - splitIndex;
            if !needSplit {
                splitIndex -= 1;
            }
        }
        let beforeSplit = splitList[splitIndex as usize];
        if needSplit {
            self.add_edge(afterSplit);
            beforeSplit.m().transfer_successors(afterSplit);
            beforeSplit.m().add_successor(afterSplit, Ptr::null());
            if forward {
                afterSplit.m().set_length(max2(0.0, beforeSplit.get_length() - relPos));
                beforeSplit.m().set_length(relPos);
            } else {
                afterSplit.m().set_length(relPos);
                beforeSplit.m().set_length(max2(0.0, beforeSplit.get_length() - relPos));
                // rename backward edges for easier referencing
                let newID = beforeSplit.get_id().to_string();
                beforeSplit.m().set_id(afterSplit.get_id());
                afterSplit.m().set_id(&newID);
            }
            self.myAccessSplits.entry(toSplit).or_default().insert((splitIndex + 1) as usize, afterSplit);
        } else {
            // don't split, use the present split edges
            afterSplit = self.myAccessSplits[&toSplit][(splitIndex + 1) as usize];
        }
        // add access to / from edge
        if addEntry {
            let id = self.next_id();
            let access = leak_edge(AccessEdge::new(id, beforeSplit, stopConn, length, SVC_IGNORING, SVC_IGNORING, -1.)).cast::<IntermodalEdge<E, L, N, V>>();
            self.add_edge(access);
            beforeSplit.m().add_successor(access, Ptr::null());
            access.m().add_successor(stopConn, Ptr::null());
        }
        if addExit {
            // pedestrian case only, exit from public to pedestrian
            let id = self.next_id();
            let exit = leak_edge(AccessEdge::new(id, stopConn, afterSplit, length, SVC_IGNORING, SVC_IGNORING, -1.)).cast::<IntermodalEdge<E, L, N, V>>();
            self.add_edge(exit);
            stopConn.m().add_successor(exit, Ptr::null());
            exit.m().add_successor(afterSplit, Ptr::null());
        }
    }

    /// C++: `IntermodalNetwork<E, L, N, V> & IntermodalNetwork::operator=(const IntermodalNetwork<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &IntermodalNetwork<E, L, N, V>) -> &mut IntermodalNetwork<E, L, N, V> {
        panic!("IntermodalNetwork::operator= is private and never defined in C++, it cannot be called")
    }

}
