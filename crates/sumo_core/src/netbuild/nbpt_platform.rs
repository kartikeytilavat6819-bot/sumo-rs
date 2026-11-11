//! Port of netbuild/NBPTPlatform.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NBPTPlatform` (netbuild/NBPTPlatform.h)
pub struct NBPTPlatform {
    pub myPos: Position,
    pub myLength: f64,
}

impl NBPTPlatform {
    /// C++: `void NBPTPlatform::NBPTPlatform(Position, double)`
    // NBPTPlatform::NBPTPlatform(Position position, double d): myPos(position), myLength(d) {
    // }
    #[allow(unused_variables)]
    pub fn new(position: Position, d: f64) -> Self {
        todo!("port NBPTPlatform::NBPTPlatform")
    }

    /// C++: `const Position & NBPTPlatform::getPos() const` [const]
    // const Position&
    // NBPTPlatform::getPos() const {
    //     return myPos;
    // }
    #[allow(unused_variables)]
    pub fn get_pos(&self) -> &Position {
        todo!("port NBPTPlatform::getPos")
    }

    /// C++: `void NBPTPlatform::reshiftPosition(const double, const double)`
    // void
    // NBPTPlatform::reshiftPosition(const double offsetX, const double offsetY) {
    //     myPos.add(offsetX, offsetY, 0);
    // }
    #[allow(unused_variables)]
    pub fn reshift_position(&mut self, offsetX: f64, offsetY: f64) {
        todo!("port NBPTPlatform::reshiftPosition")
    }

    /// C++: `double NBPTPlatform::getLength() const` [const]
    // double
    // NBPTPlatform::getLength() const {
    //     return myLength;
    // }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port NBPTPlatform::getLength")
    }

}

