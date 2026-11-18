//! Trait bounds for the generic router code: the edge and vehicle accessors that the
//! C++ templates call on `E` and `V` (no C++ counterpart file).
use crate::prelude::*;
use crate::utils::router::flipped_edge::FlippedEdge;
use crate::utils::router::rail_edge::RailEdge;
use crate::utils::router::reversed_edge::ReversedEdge;

/// Accessors the generic router code calls on its edge type `E` (`E::getID()`, `E::getSuccessors()`, ...).
/// `Node` and `Lane` are the junction and lane types of the network (C++ `N` and `L` templates).
pub trait RouterEdge<V>: RouterEdgeInfo {
    type Node;
    type Lane;
    fn id(&self) -> &str;
    fn numerical_id(&self) -> i32;
    fn length(&self) -> f64;
    fn length_geometry_factor(&self) -> f64;
    fn distance_to(&self, other: Ptr<Self>, do_boundary_estimate: bool) -> f64;
    fn successors(&self, vclass: SUMOVehicleClass) -> &[Ptr<Self>];
    fn via_successors(&self, vclass: SUMOVehicleClass, ignore_transient: bool) -> &[(Ptr<Self>, Ptr<Self>)];
    // shortcut: IntermodalEdge has no predecessors in C++, upgrade when an edge type needs them
    fn predecessors(&self) -> &[Ptr<Self>] {
        panic!("shortcut: this edge type has no predecessors")
    }
    // shortcut: IntermodalEdge has no lanes in C++, upgrade when an edge type needs them
    fn lanes(&self) -> &[Ptr<Self::Lane>] {
        panic!("shortcut: this edge type has no lanes")
    }
    fn speed_limit(&self) -> f64;
    fn minimum_travel_time(&self, veh: Ptr<V>) -> f64;
    // shortcut: IntermodalEdge has no getPreference in C++, upgrade when an edge type needs routing preferences
    fn preference(&self, pars: &SUMOVTypeParameter) -> f64 {
        panic!("shortcut: this edge type has no routing preference")
    }
    // shortcut: IntermodalEdge has no getFlippedRoutingEdge in C++, upgrade when an edge type needs the arc flag router
    fn flipped_routing_edge(&self) -> Ptr<FlippedEdge<Self, Self::Node, V>>
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no flipped routing edge")
    }
    fn bidi_edge(&self) -> Ptr<Self>;
    // shortcut: IntermodalEdge has no junctions in C++, upgrade when an edge type needs them
    fn to_junction(&self) -> Ptr<Self::Node> {
        panic!("shortcut: this edge type has no to junction")
    }
    fn from_junction(&self) -> Ptr<Self::Node> {
        panic!("shortcut: this edge type has no from junction")
    }
    // shortcut: IntermodalEdge is never a network crossing in C++, upgrade when an edge type needs it
    fn is_crossing(&self) -> bool {
        panic!("shortcut: this edge type has no crossing flag")
    }
    fn is_walking_area(&self) -> bool {
        panic!("shortcut: this edge type has no walking area flag")
    }
    // shortcut: only MSEdge/ROEdge/NBRouterEdge carry a function class, upgrade when IntermodalNetwork runs on another edge type
    fn is_normal(&self) -> bool {
        panic!("shortcut: this edge type has no isNormal()")
    }
    fn function(&self) -> SumoXMLEdgeFunc {
        panic!("shortcut: this edge type has no getFunction()")
    }
    fn other_taz_connector(&self) -> Ptr<Self> {
        panic!("shortcut: this edge type has no getOtherTazConnector()")
    }
    fn stop_position(_stop: &SUMOVehicleParameter_Stop) -> Position
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no static getStopPosition()")
    }
    fn lane_edge(_lane: Ptr<Self::Lane>) -> Ptr<Self>
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no lane-to-edge accessor")
    }
    fn lane_outgoing_via_lanes(_lane: Ptr<Self::Lane>) -> Vec<(Ptr<Self::Lane>, Ptr<Self>)>
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no lane outgoing via lanes")
    }
    // C++ E::getTimePenalty() (traffic light penalty used by pedestrian crossings)
    fn time_penalty(&self) -> f64 {
        panic!("shortcut: this edge type has no getTimePenalty()")
    }
    // C++ E::getPriority() (road class); ROEdge and MSEdge have it, the others never reach it
    fn priority(&self) -> i32 {
        panic!("this edge type has no getPriority()")
    }
    // C++ L::getIncomingLinkState() used by pedestrian crossings
    fn lane_incoming_link_state(_lane: Ptr<Self::Lane>) -> LinkState
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no lane getIncomingLinkState()")
    }
    // C++ L::getPermissions() / L::allowsVehicleClass() as used by getSidewalk
    fn lane_permissions(_lane: Ptr<Self::Lane>) -> SVCPermissions
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no lane permissions")
    }
    fn lane_allows_vehicle_class(_lane: Ptr<Self::Lane>, _svc: SUMOVehicleClass) -> bool
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no lane allowsVehicleClass()")
    }
    // shortcut: only the network edge type provides a railway routing edge, upgrade when IntermodalEdge needs it
    fn railway_routing_edge(&self) -> Ptr<RailEdge<Self, V>>
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no railway routing edge")
    }
    // shortcut: only the network edge type provides E::getAllEdges(), upgrade when IntermodalEdge becomes a network edge
    fn all_edges() -> Vec<Ptr<Self>>
    where
        Self: Sized,
    {
        panic!("shortcut: this edge type has no static all-edges list")
    }
    fn travel_time_static(edge: Ptr<Self>, veh: Ptr<V>, time: f64) -> f64;
    fn travel_time_aggregated(edge: Ptr<Self>, veh: Ptr<V>, time: f64) -> f64;
    /// C++ static `E::getTravelTimeStaticRandomized(const E*, const V*, double)`; only ROEdge has it
    fn travel_time_static_randomized(edge: Ptr<Self>, veh: Ptr<V>, time: f64) -> f64 {
        panic!("this edge type has no static getTravelTimeStaticRandomized")
    }
    fn is_internal(&self) -> bool;
    // shortcut: IntermodalEdge has no isTazConnector in C++, upgrade when an edge type needs it
    fn is_taz_connector(&self) -> bool {
        panic!("shortcut: this edge type has no taz connector flag")
    }
    /// C++ `E::isConnectedTo(const E&, SUMOVehicleClass, bool)`; not a member of IntermodalEdge (C++ would not compile).
    fn is_connected_to(&self, e: &Self, vclass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> bool {
        panic!("this edge type has no isConnectedTo")
    }
    /// C++ `E::getReversedRoutingEdge()`; only the network edge types have it (C++ would not compile for the others).
    fn reversed_routing_edge(&self) -> Ptr<ReversedEdge<Self, V>>
    where
        Self: Sized,
    {
        panic!("this edge type has no reversed routing edge")
    }
    /// C++ member `E::getTravelTime(const V*, double)`; only ROEdge has it (C++ would not compile for the others).
    fn travel_time(&self, veh: Ptr<V>, time: f64) -> f64 {
        panic!("this edge type has no getTravelTime member")
    }
    fn prohibits(&self, veh: Ptr<V>, check_restrictions: bool) -> bool;
    fn restricts(&self, veh: Ptr<V>) -> bool;
}

/// Accessors the generic router code calls on its vehicle type `V` (`V::getVClass()`).
pub trait RouterVehicle {
    fn v_class(&self) -> SUMOVehicleClass;
    fn get_id(&self) -> String;
    fn get_max_speed(&self) -> f64;
    fn get_chosen_speed_factor(&self) -> f64;
    fn ignore_transient_permissions(&self) -> bool;
    /// C++ `V::getLength()` — the railway router's max-train-length check.
    fn length(&self) -> f64;
    /// C++ `V::getID()` in warning text (owned string, unlike `get_id`).
    fn id(&self) -> String {
        self.get_id()
    }
    // shortcut: only SUMOVehicle and ROVehicle expose their vtype parameter to the router, upgrade when NBVehicle is routed through IntermodalTrip
    fn v_type_parameter(&self) -> &SUMOVTypeParameter {
        panic!("shortcut: this vehicle type has no getVTypeParameter()")
    }
    /// C++ `V::getParameter().vtypeid`
    fn vtype_id(&self) -> String {
        panic!("shortcut: this vehicle type has no getParameter().vtypeid")
    }
    /// C++ `V::getVehicleType().getLength()`
    fn vehicle_type_length(&self) -> f64 {
        panic!("shortcut: this vehicle type has no getVehicleType().getLength()")
    }
}

/// Accessors the generic router code calls on its node type `N` (`N::getID()`, `N::getPosition()`, `N::getIncoming()`, ...).
/// `Edge` is the edge type of the network (C++ `E` template).
pub trait RouterNode {
    type Edge;
    fn id(&self) -> &str;
    fn position(&self) -> &Position;
    fn incoming(&self) -> &[Ptr<Self::Edge>];
    fn outgoing(&self) -> &[Ptr<Self::Edge>];
}

/// Network-edge accessors that need no vehicle type (C++ `E::getNumericalID()`, `E::getID()`, `E::getPermissions()`),
/// so the CH/SPTree code can read them on a node's edge without `V`.
pub trait RouterEdgeInfo {
    fn edge_numerical_id(&self) -> i32;
    fn edge_id(&self) -> String;
    fn edge_permissions(&self) -> SVCPermissions;
}

impl RouterEdgeInfo for MSEdge {
    fn edge_numerical_id(&self) -> i32 {
        <MSEdge as RouterEdge<SUMOVehicle>>::numerical_id(self)
    }
    fn edge_id(&self) -> String {
        <MSEdge as RouterEdge<SUMOVehicle>>::id(self).to_string()
    }
    fn edge_permissions(&self) -> SVCPermissions {
        self.get_permissions()
    }
}
