//! Port of activitygen/city/AGBus.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGBus` (activitygen/city/AGBus.h)
pub struct AGBus {
    pub myName: String,
    pub myDepartureTime: i32,
}

impl AGBus {
    /// C++: `void AGBus::AGBus(std::string, int)`
    // AGBus(std::string name, int depTime) :
    //         myName(name),
    //         myDepartureTime(depTime) {}
    #[allow(unused_variables)]
    pub fn new(name: String, depTime: i32) -> Self {
        AGBus { myName: name, myDepartureTime: depTime }
    }

    /// C++: `void AGBus::AGBus(int)`
    // AGBus(int depTime) :
    //         myDepartureTime(depTime) {}
    #[allow(unused_variables)]
    pub fn new_2(depTime: i32) -> Self {
        AGBus { myName: String::new(), myDepartureTime: depTime }
    }

    /// C++: `void AGBus::setName(std::string)`
    // void AGBus::setName(std::string name) {
    //     myName = name;
    // }
    #[allow(unused_variables)]
    pub fn set_name(&mut self, name: String) {
        self.myName = name;
    }

    /// C++: `int AGBus::getDeparture()`
    // int
    // AGBus::getDeparture() {
    //     return myDepartureTime;
    // }
    #[allow(unused_variables)]
    pub fn get_departure(&mut self) -> i32 {
        self.myDepartureTime
    }

    /// C++: `std::string AGBus::getName()`
    // std::string
    // AGBus::getName() {
    //     return myName;
    // }
    #[allow(unused_variables)]
    pub fn get_name(&mut self) -> String {
        self.myName.clone()
    }

    /// C++: `void AGBus::print() const` [const]
    // void
    // AGBus::print() const {
    //     std::cout << "- Bus:" << " name=" << myName << " depTime=" << myDepartureTime << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        println!("- Bus: name={} depTime={}", self.myName, self.myDepartureTime);
    }

}

