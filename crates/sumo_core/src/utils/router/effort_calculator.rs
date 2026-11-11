//! Port of utils/router/EffortCalculator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// the effort calculator interface
/// C++ class `EffortCalculator` (utils/router/EffortCalculator.h)
pub struct EffortCalculator {
}

impl EffortCalculator {
    /// C++: `void EffortCalculator::~EffortCalculator() noexcept` [virtual]
    /// empty destructor
    // virtual ~EffortCalculator() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty destructor (C++ `{}`)
    }

    /// C++: `void EffortCalculator::init(const std::vector<std::string> &)` [virtual, pure virtual]
    /// Pass the set of all edges in the routing query to the effortCalculator *
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn init(&mut self, edges: &Vec<String>) {
        panic!("pure virtual EffortCalculator::init called")
    }

    /// C++: `void EffortCalculator::addStop(const int, const Parameterised &)` [virtual, pure virtual]
    /// Add information about stops *
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, stopEdge: i32, params: &Parameterised) {
        panic!("pure virtual EffortCalculator::addStop called")
    }

    /// C++: `double EffortCalculator::getEffort(const int) const` [const, virtual, pure virtual]
    /// Return the effort of a given edge *
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_effort(&self, numericalID: i32) -> f64 {
        panic!("pure virtual EffortCalculator::getEffort called")
    }

    /// C++: `void EffortCalculator::update(const int, const int, const double)` [virtual, pure virtual]
    /// Update the effort of the edge *
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn update(&mut self, edge: i32, prev: i32, length: f64) {
        panic!("pure virtual EffortCalculator::update called")
    }

    /// C++: `void EffortCalculator::setInitialState(const int)` [virtual, pure virtual]
    /// Set the effort of the first edge in the query to zero *
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_initial_state(&mut self, edge: i32) {
        panic!("pure virtual EffortCalculator::setInitialState called")
    }

    /// C++: `std::string EffortCalculator::output(const int) const` [const, virtual, pure virtual]
    /// basic output facility to inform about effort at this edge *
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn output(&self, edge: i32) -> String {
        panic!("pure virtual EffortCalculator::output called")
    }

}

