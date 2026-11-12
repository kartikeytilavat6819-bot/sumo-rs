//! Port of duarouter/RODUAEdgeBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Interface for building instances of duarouter-edges
/// C++ class `RODUAEdgeBuilder` (duarouter/RODUAEdgeBuilder.h)
#[repr(C)]
pub struct RODUAEdgeBuilder {
    /// C++ base class `ROAbstractEdgeBuilder`
    pub base: ROAbstractEdgeBuilder,
}

impl std::ops::Deref for RODUAEdgeBuilder {
    type Target = ROAbstractEdgeBuilder;
    fn deref(&self) -> &ROAbstractEdgeBuilder {
        &self.base
    }
}
impl std::ops::DerefMut for RODUAEdgeBuilder {
    fn deref_mut(&mut self) -> &mut ROAbstractEdgeBuilder {
        &mut self.base
    }
}

impl RODUAEdgeBuilder {
    /// C++: `void RODUAEdgeBuilder::RODUAEdgeBuilder()`
    /// Constructor
    // RODUAEdgeBuilder::RODUAEdgeBuilder() {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        RODUAEdgeBuilder { base: ROAbstractEdgeBuilder::new() }
    }

    /// C++: `void RODUAEdgeBuilder::~RODUAEdgeBuilder() noexcept` [virtual]
    /// Destructor
    // RODUAEdgeBuilder::~RODUAEdgeBuilder() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `ROEdge * RODUAEdgeBuilder::buildEdge(const std::string &, RONode *, RONode *, const int, const std::string &, const std::string &)` [virtual]
    /// Builds an edge with the given name
    // ROEdge*
    // RODUAEdgeBuilder::buildEdge(const std::string& name, RONode* from, RONode* to, const int priority, const std::string& type, const std::string& routingType) {
    //     return new ROEdge(name, from, to, getNextIndex(), priority, type, routingType);
    // }
    #[allow(unused_variables)]
    pub fn build_edge(&mut self, name: &str, from: Ptr<RONode>, to: Ptr<RONode>, priority: i32, type_: &str, routingType: &str) -> Ptr<ROEdge> {
        Ptr::from_raw(Box::into_raw(Box::new(ROEdge::new(name, from, to, self.get_next_index(), priority, type_, routingType))))
    }

}

