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
    fn travel_time_static(edge: Ptr<Self>, veh: Ptr<V>, time: f64) -> f64;
    fn travel_time_aggregated(edge: Ptr<Self>, veh: Ptr<V>, time: f64) -> f64;
    fn is_internal(&self) -> bool;
    // shortcut: IntermodalEdge has no isTazConnector in C++, upgrade when an edge type needs it
    fn is_taz_connector(&self) -> bool {
        panic!("shortcut: this edge type has no taz connector flag")
    }
    fn prohibits(&self, veh: Ptr<V>, check_restrictions: bool) -> bool;
    fn restricts(&self, veh: Ptr<V>) -> bool;
    // shortcut: IntermodalEdge has no junctions in C++, upgrade when an edge type needs the k-d tree or arc flags
    fn from_junction(&self) -> Ptr<Self::Node> {
        panic!("shortcut: this edge type has no from junction")
    }
    // shortcut: IntermodalEdge has no junctions in C++, upgrade when an edge type needs the k-d tree or arc flags
    fn to_junction(&self) -> Ptr<Self::Node> {
        panic!("shortcut: this edge type has no to junction")
    }
}

/// Accessors the generic router code calls on its vehicle type `V` (`V::getVClass()`).
pub trait RouterVehicle {
    fn v_class(&self) -> SUMOVehicleClass;
    fn get_id(&self) -> String;
    fn get_max_speed(&self) -> f64;
    fn get_chosen_speed_factor(&self) -> f64;
    fn ignore_transient_permissions(&self) -> bool;
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
