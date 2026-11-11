//! Port of router/ROAbstractEdgeBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Interface for building instances of router-edges
/// C++ class `ROAbstractEdgeBuilder` (router/ROAbstractEdgeBuilder.h)
pub struct ROAbstractEdgeBuilder {
    /// The next edge's index
    pub myCurrentIndex: i32,
}

impl ROAbstractEdgeBuilder {
    /// C++: `void ROAbstractEdgeBuilder::ROAbstractEdgeBuilder()`
    /// Constructor
    // ROAbstractEdgeBuilder() : myCurrentIndex(0) { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port ROAbstractEdgeBuilder::ROAbstractEdgeBuilder")
    }

    /// C++: `void ROAbstractEdgeBuilder::~ROAbstractEdgeBuilder() noexcept` [virtual]
    /// Destructor
    // virtual ~ROAbstractEdgeBuilder() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROAbstractEdgeBuilder::~ROAbstractEdgeBuilder")
    }

    /// C++: `ROEdge * ROAbstractEdgeBuilder::buildEdge(const std::string &, RONode *, RONode *, const int, const std::string &, const std::string &)` [virtual, pure virtual]
    /// Builds an edge with the given name
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge(&mut self, name: &str, from: Ptr<RONode>, to: Ptr<RONode>, priority: i32, type_: &str, routingType: &str) -> Ptr<ROEdge> {
        todo!("port ROAbstractEdgeBuilder::buildEdge")
    }

    /// C++: `int ROAbstractEdgeBuilder::getNextIndex()` [protected]
    /// Returns the index of the edge to built
    // int getNextIndex() {
    //         return myCurrentIndex++;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_index(&mut self) -> i32 {
        todo!("port ROAbstractEdgeBuilder::getNextIndex")
    }

    /// C++: `void ROAbstractEdgeBuilder::ROAbstractEdgeBuilder(const ROAbstractEdgeBuilder &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROAbstractEdgeBuilder) -> Self {
        todo!("port ROAbstractEdgeBuilder::ROAbstractEdgeBuilder")
    }

    /// C++: `ROAbstractEdgeBuilder & ROAbstractEdgeBuilder::operator=(const ROAbstractEdgeBuilder &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROAbstractEdgeBuilder) -> &mut ROAbstractEdgeBuilder {
        todo!("port ROAbstractEdgeBuilder::operator=")
    }

}

