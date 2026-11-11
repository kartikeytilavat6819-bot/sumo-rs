//! Port of activitygen/city/AGSchool.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGSchool` (activitygen/city/AGSchool.h)
pub struct AGSchool {
    pub beginAge: i32,
    pub endAge: i32,
    pub capacity: i32,
    pub initCapacity: i32,
    pub location: AGPosition,
    pub opening: i32,
    pub closing: i32,
}

impl AGSchool {
    /// C++: `void AGSchool::AGSchool(int, AGPosition, int, int, int, int)`
    // AGSchool(int capacity_, AGPosition pos, int beginAge, int endAge, int open, int close) :
    //         beginAge(beginAge),
    //         endAge(endAge),
    //         capacity(capacity_),
    //         initCapacity(capacity_),
    //         location(pos),
    //         opening(open),
    //         closing(close) {}
    #[allow(unused_variables)]
    pub fn new(capacity_: i32, pos: AGPosition, beginAge: i32, endAge: i32, open: i32, close: i32) -> Self {
        AGSchool {
            beginAge,
            endAge,
            capacity: capacity_,
            initCapacity: capacity_,
            location: pos,
            opening: open,
            closing: close,
        }
    }

    /// C++: `void AGSchool::print() const` [const]
    // void
    // AGSchool::print() const {
    //     std::cout << "- school: " << " placeNbr=" << capacity << " hours=[" << opening << ";" << closing << "] ages=[" << beginAge << ";" << endAge << "]" << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        println!("- school:  placeNbr={} hours=[{};{}] ages=[{};{}]", self.capacity, self.opening, self.closing, self.beginAge, self.endAge);
    }

    /// C++: `int AGSchool::getPlaces()`
    // int
    // AGSchool::getPlaces() {
    //     return capacity;
    // }
    #[allow(unused_variables)]
    pub fn get_places(&mut self) -> i32 {
        self.capacity
    }

    /// C++: `bool AGSchool::addNewChild()`
    // bool
    // AGSchool::addNewChild() {
    //     if (capacity > 0) {
    //         --capacity;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_new_child(&mut self) -> bool {
        if self.capacity > 0 {
            self.capacity -= 1;
            return true;
        }
        false
    }

    /// C++: `bool AGSchool::removeChild()`
    // bool
    // AGSchool::removeChild() {
    //     if (capacity < initCapacity) {
    //         ++capacity;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn remove_child(&mut self) -> bool {
        if self.capacity < self.initCapacity {
            self.capacity += 1;
            return true;
        }
        false
    }

    /// C++: `int AGSchool::getBeginAge()`
    // int
    // AGSchool::getBeginAge() {
    //     return beginAge;
    // }
    #[allow(unused_variables)]
    pub fn get_begin_age(&mut self) -> i32 {
        self.beginAge
    }

    /// C++: `int AGSchool::getEndAge()`
    // int
    // AGSchool::getEndAge() {
    //     return endAge;
    // }
    #[allow(unused_variables)]
    pub fn get_end_age(&mut self) -> i32 {
        self.endAge
    }

    /// C++: `bool AGSchool::acceptThisAge(int)`
    // bool
    // AGSchool::acceptThisAge(int age) {
    //     if (age <= endAge && age >= beginAge) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn accept_this_age(&mut self, age: i32) -> bool {
        if age <= self.endAge && age >= self.beginAge {
            return true;
        }
        false
    }

    /// C++: `AGPosition AGSchool::getPosition()`
    // AGPosition
    // AGSchool::getPosition() {
    //     return location;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&mut self) -> AGPosition {
        // copy the members, as the C++ copy constructor does
        AGPosition { street: self.location.street, position: self.location.position, pos2d: self.location.pos2d }
    }

    /// C++: `int AGSchool::getClosingHour()`
    // int
    // AGSchool::getClosingHour() {
    //     return closing;
    // }
    #[allow(unused_variables)]
    pub fn get_closing_hour(&mut self) -> i32 {
        self.closing
    }

    /// C++: `int AGSchool::getOpeningHour()`
    // int
    // AGSchool::getOpeningHour() {
    //     return opening;
    // }
    #[allow(unused_variables)]
    pub fn get_opening_hour(&mut self) -> i32 {
        self.opening
    }

}

