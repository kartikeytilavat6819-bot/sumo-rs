//! Port of marouter/ROMAEdgeBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `ROMAEdgeBuilder::buildEdge` body, the target of the virtual dispatch in ROAbstractEdgeBuilder
fn roma_build_edge(eb: &mut ROAbstractEdgeBuilder, name: &str, from: Ptr<RONode>, to: Ptr<RONode>, priority: i32, type_: &str, routingType: &str) -> Ptr<ROEdge> {
    // C++ implicit derived-to-base conversion: point at the embedded ROEdge sub-object
    let edge = Box::into_raw(Box::new(ROMAEdge::new(name, from, to, eb.get_next_index(), priority, type_, routingType)));
    let base = Ptr::from_raw(unsafe { &raw mut (*edge).base });
    ROEdge::register_in_all_edges(base);
    base
}

/// Interface for building instances of duarouter-edges
/// C++ class `ROMAEdgeBuilder` (marouter/ROMAEdgeBuilder.h)
#[repr(C)]
pub struct ROMAEdgeBuilder {
    /// C++ base class `ROAbstractEdgeBuilder`
    pub base: ROAbstractEdgeBuilder,
}

impl std::ops::Deref for ROMAEdgeBuilder {
    type Target = ROAbstractEdgeBuilder;
    fn deref(&self) -> &ROAbstractEdgeBuilder {
        &self.base
    }
}
impl std::ops::DerefMut for ROMAEdgeBuilder {
    fn deref_mut(&mut self) -> &mut ROAbstractEdgeBuilder {
        &mut self.base
    }
}

impl ROMAEdgeBuilder {
    /// C++: `void ROMAEdgeBuilder::ROMAEdgeBuilder()`
    /// Constructor
    // ROMAEdgeBuilder::ROMAEdgeBuilder() {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        let mut base = ROAbstractEdgeBuilder::new();
        base.buildEdgeFn = roma_build_edge;
        ROMAEdgeBuilder { base }
    }

    /// C++: `void ROMAEdgeBuilder::~ROMAEdgeBuilder() noexcept` [virtual]
    /// Destructor
    // ROMAEdgeBuilder::~ROMAEdgeBuilder() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `ROEdge * ROMAEdgeBuilder::buildEdge(const std::string &, RONode *, RONode *, const int, const std::string &, const std::string &)` [virtual]
    /// Builds an edge with the given name
    // ROEdge*
    // ROMAEdgeBuilder::buildEdge(const std::string& name, RONode* from, RONode* to, const int priority, const std::string& type, const std::string& routingType) {
    //     return new ROMAEdge(name, from, to, getNextIndex(), priority, type, routingType);
    // }
    #[allow(unused_variables)]
    pub fn build_edge(&mut self, name: &str, from: Ptr<RONode>, to: Ptr<RONode>, priority: i32, type_: &str, routingType: &str) -> Ptr<ROEdge> {
        roma_build_edge(&mut self.base, name, from, to, priority, type_, routingType)
    }

}

