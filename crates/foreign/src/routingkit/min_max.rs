//! Port of foreign/RoutingKit/include/routingkit/min_max.h (min_to only, the other helpers are unused by SUMO)

/// RoutingKit::min_to: x = min(x, y)
pub fn min_to<T: PartialOrd + Copy>(x: &mut T, y: T) {
    if y < *x {
        *x = y;
    }
}
