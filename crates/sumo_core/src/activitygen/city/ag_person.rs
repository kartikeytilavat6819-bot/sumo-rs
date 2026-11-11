//! Port of activitygen/city/AGPerson.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Base class of every person in the city (adults and children)
/// C++ class `AGPerson` (activitygen/city/AGPerson.h)
pub struct AGPerson {
    pub age: i32,
}

impl AGPerson {
    /// C++: `int AGPerson::getAge() const` [const, virtual]
    /// Provides the age of the person.
    // int
    // AGPerson::getAge() const {
    //     return age;
    // }
    #[allow(unused_variables)]
    pub fn get_age(&self) -> i32 {
        todo!("port AGPerson::getAge")
    }

    /// C++: `bool AGPerson::decide(double) const` [const, virtual]
    /// Lets the person make a decision.
    // bool
    // AGPerson::decide(double proba) const {
    //     return (RandHelper::rand(1000) < static_cast<int>(1000.0f * proba));
    // }
    #[allow(unused_variables)]
    pub fn decide(&self, probability: f64) -> bool {
        todo!("port AGPerson::decide")
    }

    /// C++: `void AGPerson::print() const` [const, virtual]
    /// Puts out a summary of the class properties.
    // void
    // AGPerson::print() const {
    //     std::cout << "- Person: Age=" << age << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        todo!("port AGPerson::print")
    }

    /// C++: `void AGPerson::AGPerson(int)` [protected]
    /// Initialises the class attributes.
    // AGPerson::AGPerson(int age) : age(age) {}
    #[allow(unused_variables)]
    pub fn new(age: i32) -> Self {
        todo!("port AGPerson::AGPerson")
    }

    /// C++: `void AGPerson::~AGPerson() noexcept` [virtual, protected]
    /// Cleans up everything.
    // AGPerson::~AGPerson() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port AGPerson::~AGPerson")
    }

}

