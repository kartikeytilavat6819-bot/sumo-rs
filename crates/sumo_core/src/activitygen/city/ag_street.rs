//! Port of activitygen/city/AGStreet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A model of the street in the city.
/// C++ class `AGStreet` (activitygen/city/AGStreet.h)
pub struct AGStreet {
    /// C++ base class `ROEdge`
    pub base: ROEdge,
    pub myPopulation: f64,
    pub myNumWorkplaces: f64,
}

impl std::ops::Deref for AGStreet {
    type Target = ROEdge;
    fn deref(&self) -> &ROEdge {
        &self.base
    }
}
impl std::ops::DerefMut for AGStreet {
    fn deref_mut(&mut self) -> &mut ROEdge {
        &mut self.base
    }
}

impl AGStreet {
    /// C++: `void AGStreet::AGStreet(const std::string &, RONode *, RONode *, int, const int, const std::string &, const std::string &)`
    // AGStreet::AGStreet(const std::string& id, RONode* from, RONode* to, int index, const int priority, const std::string& type, const std::string& routingType) :
    //     ROEdge(id, from, to, index, priority, type, routingType),
    //     myPopulation(0.), myNumWorkplaces(0.) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, index: i32, priority: i32, type_: &str, routingType: &str) -> Self {
        todo!("port AGStreet::AGStreet")
    }

    /// C++: `double AGStreet::getPopulation() const` [const]
    /// Provides the number of persons living in this street.
    // double
    // AGStreet::getPopulation() const {
    //     return myPopulation;
    // }
    #[allow(unused_variables)]
    pub fn get_population(&self) -> f64 {
        todo!("port AGStreet::getPopulation")
    }

    /// C++: `void AGStreet::setPopulation(const double)`
    /// Modifies the number of persons living in this street.
    // void
    // AGStreet::setPopulation(const double population) {
    //     myPopulation = population;
    // }
    #[allow(unused_variables)]
    pub fn set_population(&mut self, pop: f64) {
        todo!("port AGStreet::setPopulation")
    }

    /// C++: `double AGStreet::getWorkplaceNumber() const` [const]
    /// Provides the number of work places in this street.
    // double
    // AGStreet::getWorkplaceNumber() const {
    //     return myNumWorkplaces;
    // }
    #[allow(unused_variables)]
    pub fn get_workplace_number(&self) -> f64 {
        todo!("port AGStreet::getWorkplaceNumber")
    }

    /// C++: `void AGStreet::setWorkplaceNumber(const double)`
    /// Modifies the number of work places in this street.
    // void
    // AGStreet::setWorkplaceNumber(const double workPositions) {
    //     myNumWorkplaces = workPositions;
    // }
    #[allow(unused_variables)]
    pub fn set_workplace_number(&mut self, work: f64) {
        todo!("port AGStreet::setWorkplaceNumber")
    }

    /// C++: `void AGStreet::print() const` [const]
    /// Prints a summary of the properties of this street to standard output.
    // void
    // AGStreet::print() const {
    //     std::cout << "- AGStreet: Name=" << getID() << " Length=" << getLength() << " pop=" << myPopulation << " work=" << myNumWorkplaces << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        todo!("port AGStreet::print")
    }

    /// C++: `bool AGStreet::allows(const SUMOVehicleClass) const` [const]
    /// Returns whether the given vehicle class is allowed on this street.
    // bool
    // AGStreet::allows(const SUMOVehicleClass vclass) const {
    //     return (getPermissions() & vclass) == vclass;
    // }
    #[allow(unused_variables)]
    pub fn allows(&self, vclass: SUMOVehicleClass) -> bool {
        todo!("port AGStreet::allows")
    }

}

/// C++ class `AGStreet::Builder` (activitygen/city/AGStreet.h)
pub struct AGStreet_Builder {
    /// C++ base class `ROAbstractEdgeBuilder`
    pub base: ROAbstractEdgeBuilder,
}

impl std::ops::Deref for AGStreet_Builder {
    type Target = ROAbstractEdgeBuilder;
    fn deref(&self) -> &ROAbstractEdgeBuilder {
        &self.base
    }
}
impl std::ops::DerefMut for AGStreet_Builder {
    fn deref_mut(&mut self) -> &mut ROAbstractEdgeBuilder {
        &mut self.base
    }
}

impl AGStreet_Builder {
    /// C++: `ROEdge * Builder::buildEdge(const std::string &, RONode *, RONode *, const int, const std::string &, const std::string &)` [virtual]
    /// Builds an edge with the given name
    // ROEdge* buildEdge(const std::string& name, RONode* from, RONode* to, const int priority, const std::string& type, const std::string& routingType) {
    //             return new AGStreet(name, from, to, getNextIndex(), priority, type, routingType);
    //         }
    #[allow(unused_variables)]
    pub fn build_edge(&mut self, name: &str, from: Ptr<RONode>, to: Ptr<RONode>, priority: i32, type_: &str, routingType: &str) -> Ptr<ROEdge> {
        todo!("port Builder::buildEdge")
    }

}

