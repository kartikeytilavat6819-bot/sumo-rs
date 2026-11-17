//! Port of router/ROAbstractEdgeBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Interface for building instances of router-edges
/// C++ class `ROAbstractEdgeBuilder` (router/ROAbstractEdgeBuilder.h)
pub struct ROAbstractEdgeBuilder {
    /// The next edge's index
    pub myCurrentIndex: i32,
    /// shortcut: virtual dispatch of buildEdge, set by each subclass constructor (C++ vtable), since the base is not a trait object yet
    pub buildEdgeFn: fn(&mut ROAbstractEdgeBuilder, &str, Ptr<RONode>, Ptr<RONode>, i32, &str, &str) -> Ptr<ROEdge>,
}

/// C++ pure virtual buildEdge: a subclass that does not set buildEdgeFn is a bug
fn pure_virtual_build_edge(_eb: &mut ROAbstractEdgeBuilder, _name: &str, _from: Ptr<RONode>, _to: Ptr<RONode>, _priority: i32, _type_: &str, _routingType: &str) -> Ptr<ROEdge> {
    panic!("ROAbstractEdgeBuilder::buildEdge is pure virtual; subclasses must implement it")
}

impl ROAbstractEdgeBuilder {
    /// C++: `void ROAbstractEdgeBuilder::ROAbstractEdgeBuilder()`
    /// Constructor
    // ROAbstractEdgeBuilder() : myCurrentIndex(0) { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        ROAbstractEdgeBuilder { myCurrentIndex: 0, buildEdgeFn: pure_virtual_build_edge }
    }

    /// C++: `void ROAbstractEdgeBuilder::~ROAbstractEdgeBuilder() noexcept` [virtual]
    /// Destructor
    // virtual ~ROAbstractEdgeBuilder() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `ROEdge * ROAbstractEdgeBuilder::buildEdge(const std::string &, RONode *, RONode *, const int, const std::string &, const std::string &)` [virtual, pure virtual]
    /// Builds an edge with the given name
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_edge(&mut self, name: &str, from: Ptr<RONode>, to: Ptr<RONode>, priority: i32, type_: &str, routingType: &str) -> Ptr<ROEdge> {
        // C++ virtual call: dispatched to the subclass through buildEdgeFn
        (self.buildEdgeFn)(self, name, from, to, priority, type_, routingType)
    }

    /// C++: `int ROAbstractEdgeBuilder::getNextIndex()` [protected]
    /// Returns the index of the edge to built
    // int getNextIndex() {
    //         return myCurrentIndex++;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_index(&mut self) -> i32 {
        let idx = self.myCurrentIndex;
        self.myCurrentIndex += 1;
        idx
    }

    /// C++: `void ROAbstractEdgeBuilder::ROAbstractEdgeBuilder(const ROAbstractEdgeBuilder &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROAbstractEdgeBuilder) -> Self {
        // C++ copy constructor is private and never defined: copying is disallowed
        panic!("ROAbstractEdgeBuilder is not copyable (private copy constructor in C++)")
    }

    /// C++: `ROAbstractEdgeBuilder & ROAbstractEdgeBuilder::operator=(const ROAbstractEdgeBuilder &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROAbstractEdgeBuilder) -> &mut ROAbstractEdgeBuilder {
        // C++ assignment operator is private and never defined: assignment is disallowed
        panic!("ROAbstractEdgeBuilder is not assignable (private operator= in C++)")
    }

}

