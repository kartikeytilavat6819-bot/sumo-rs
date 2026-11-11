//! Port of dfrouter/RODFEdgeBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Interface for building instances of dfrouter-edges
/// C++ class `RODFEdgeBuilder` (dfrouter/RODFEdgeBuilder.h)
pub struct RODFEdgeBuilder {
    /// C++ base class `ROAbstractEdgeBuilder`
    pub base: ROAbstractEdgeBuilder,
}

impl std::ops::Deref for RODFEdgeBuilder {
    type Target = ROAbstractEdgeBuilder;
    fn deref(&self) -> &ROAbstractEdgeBuilder {
        &self.base
    }
}
impl std::ops::DerefMut for RODFEdgeBuilder {
    fn deref_mut(&mut self) -> &mut ROAbstractEdgeBuilder {
        &mut self.base
    }
}

impl RODFEdgeBuilder {
    /// C++: `void RODFEdgeBuilder::RODFEdgeBuilder()`
    /// Constructor
    // RODFEdgeBuilder::RODFEdgeBuilder() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port RODFEdgeBuilder::RODFEdgeBuilder")
    }

    /// C++: `void RODFEdgeBuilder::~RODFEdgeBuilder() noexcept` [virtual]
    /// Destructor
    // RODFEdgeBuilder::~RODFEdgeBuilder() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFEdgeBuilder::~RODFEdgeBuilder")
    }

    /// C++: `ROEdge * RODFEdgeBuilder::buildEdge(const std::string &, RONode *, RONode *, const int, const std::string &, const std::string &)` [virtual]
    /// Builds an edge with the given name
    // ROEdge*
    // RODFEdgeBuilder::buildEdge(const std::string& name, RONode* from, RONode* to, const int priority, const std::string& type, const std::string& routingType) {
    //     return new RODFEdge(name, from, to, getNextIndex(), priority, type, routingType);
    // }
    #[allow(unused_variables)]
    pub fn build_edge(&mut self, name: &str, from: Ptr<RONode>, to: Ptr<RONode>, priority: i32, type_: &str, routingType: &str) -> Ptr<ROEdge> {
        todo!("port RODFEdgeBuilder::buildEdge")
    }

}

