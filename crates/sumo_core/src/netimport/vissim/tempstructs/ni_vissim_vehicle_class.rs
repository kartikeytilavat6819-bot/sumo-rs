//! Port of netimport/vissim/tempstructs/NIVissimVehicleClass.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimVehicleClass` (netimport/vissim/tempstructs/NIVissimVehicleClass.h)
pub struct NIVissimVehicleClass {
    pub myVWish: i32,
}

impl NIVissimVehicleClass {
    /// C++: `void NIVissimVehicleClass::NIVissimVehicleClass(int)`
    // NIVissimVehicleClass::NIVissimVehicleClass(int vwish)
    //     : myVWish(vwish) {}
    #[allow(unused_variables)]
    pub fn new(vwish: i32) -> Self {
        todo!("port NIVissimVehicleClass::NIVissimVehicleClass")
    }

    /// C++: `void NIVissimVehicleClass::~NIVissimVehicleClass()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimVehicleClass::~NIVissimVehicleClass")
    }

    /// C++: `int NIVissimVehicleClass::getSpeed() const` [const]
    // int
    // NIVissimVehicleClass::getSpeed() const {
    //     return myVWish;
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> i32 {
        todo!("port NIVissimVehicleClass::getSpeed")
    }

}

