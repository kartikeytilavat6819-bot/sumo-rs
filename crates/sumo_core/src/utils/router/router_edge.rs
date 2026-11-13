//! Trait bounds for the generic router code: the edge and vehicle accessors that the
//! C++ templates call on `E` and `V` (no C++ counterpart file).
use crate::prelude::*;
use crate::utils::router::flipped_edge::FlippedEdge;

/// Accessors the generic router code calls on its edge type `E` (`E::getID()`, `E::getSuccessors()`, ...).
/// `Node` and `Lane` are the junction and lane types of the network (C++ `N` and `L` templates).
pub trait RouterEdge<V> {
    type Node;
    type Lane;
    fn id(&self) -> &str;
    fn numerical_id(&self) -> i32;
    fn length(&self) -> f64;
    fn length_geometry_factor(&self) -> f64;
    fn distance_to(&self, other: Ptr<Self>, do_boundary_estimate: bool) -> f64;
    fn successors(&self, vclass: SUMOVehicleClass) -> &[Ptr<Self>];
    fn via_successors(&self, vclass: SUMOVehicleClass, ignore_transient: bool) -> &[(Ptr<Self>, Ptr<Self>)];
    fn predecessors(&self) -> &[Ptr<Self>];
    fn lanes(&self) -> &[Ptr<Self::Lane>];
    fn speed_limit(&self) -> f64;
    fn preference(&self, pars: &SUMOVTypeParameter) -> f64;
    fn flipped_routing_edge(&self) -> Ptr<FlippedEdge<Self, Self::Node, V>>
    where
        Self: Sized;
    fn travel_time_static(edge: Ptr<Self>, veh: Ptr<V>, time: f64) -> f64;
    fn travel_time_aggregated(edge: Ptr<Self>, veh: Ptr<V>, time: f64) -> f64;
    fn is_internal(&self) -> bool;
    fn is_taz_connector(&self) -> bool;
    fn prohibits(&self, veh: Ptr<V>, check_restrictions: bool) -> bool;
    fn restricts(&self, veh: Ptr<V>) -> bool;
}

/// Accessors the generic router code calls on its vehicle type `V` (`V::getVClass()`).
pub trait RouterVehicle {
    fn v_class(&self) -> SUMOVehicleClass;
}
