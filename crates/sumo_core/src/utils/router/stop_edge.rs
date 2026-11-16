//! Port of utils/router/StopEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};

/// the stop edge type representing bus and train stops
/// C++ class `StopEdge` (utils/router/StopEdge.h)
#[repr(C)]
pub struct StopEdge<E, L, N, V> {
    /// C++ base class `IntermodalEdge<E, L, N, V>`
    pub base: IntermodalEdge<E, L, N, V>,
    /// start position
    pub myStartPos: f64,
    /// end position
    pub myEndPos: f64,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for StopEdge<E, L, N, V> {
    type Target = IntermodalEdge<E, L, N, V>;
    fn deref(&self) -> &IntermodalEdge<E, L, N, V> {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for StopEdge<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut IntermodalEdge<E, L, N, V> {
        &mut self.base
    }
}

impl<E, L, N, V> StopEdge<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: 'static,
    V: RouterVehicle + 'static,
{
    /// C++: `void StopEdge::StopEdge<E, L, N, V>(const std::string, int, const E *, const double, const double)`
    // StopEdge(const std::string id, int numericalID, const E* edge, const double startPos, const double endPos) :
    //         IntermodalEdge<E, L, N, V>(id, numericalID, edge, "!stop", 0), myStartPos(startPos), myEndPos(endPos) { }
    #[allow(unused_variables)]
    pub fn new(id: String, numericalID: i32, edge: Ptr<E>, startPos: f64, endPos: f64) -> Self {
        StopEdge {
            base: IntermodalEdge::new(id, numericalID, edge, "!stop", 0.0),
            myStartPos: startPos,
            myEndPos: endPos,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `bool StopEdge::includeInRoute(bool) const` [const]
    // bool includeInRoute(bool /* allEdges */) const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, arg0: bool) -> bool {
        true
    }

    /// C++: `double StopEdge::getStartPos() const` [const]
    // double getStartPos() const {
    //         return myStartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        self.myStartPos
    }

    /// C++: `double StopEdge::getEndPos() const` [const]
    // double getEndPos() const {
    //         return myEndPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        self.myEndPos
    }

}

