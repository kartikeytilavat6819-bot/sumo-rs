//! Port of netbuild/NBCapacity2Lanes.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A helper class which computes the lane number from given capacity
/// C++ class `NBCapacity2Lanes` (netbuild/NBCapacity2Lanes.h)
pub struct NBCapacity2Lanes {
    /// The norming divider
    pub myDivider: f64,
}

impl NBCapacity2Lanes {
    /// C++: `void NBCapacity2Lanes::NBCapacity2Lanes(double)`
    /// Donstructor
    // NBCapacity2Lanes(double divider) : myDivider(divider) { }
    #[allow(unused_variables)]
    pub fn new(divider: f64) -> Self {
        NBCapacity2Lanes { myDivider: divider }
    }

    /// C++: `void NBCapacity2Lanes::~NBCapacity2Lanes() noexcept`
    /// Destructor
    // ~NBCapacity2Lanes() { }
    pub fn drop_(&mut self) {}

    /// C++: `int NBCapacity2Lanes::get(double) const` [const]
    /// Returns the number of lanes computed from the given capacity
    // int get(double capacity) const {
    //         capacity /= myDivider;
    //         if (capacity > (int) capacity) {
    //             capacity += 1;
    //         }
    //         // just assure that the number of lanes is not zero
    //         if (capacity == 0) {
    //             capacity = 1;
    //         }
    //         return (int) capacity;
    //     }
    #[allow(unused_variables)]
    pub fn get(&self, mut capacity: f64) -> i32 {
        capacity /= self.myDivider;
        // C++ (int) cast truncates toward zero, as Rust `as i32` does
        if capacity > (capacity as i32) as f64 {
            capacity += 1.0;
        }
        // just assure that the number of lanes is not zero
        if capacity == 0.0 {
            capacity = 1.0;
        }
        capacity as i32
    }

}

