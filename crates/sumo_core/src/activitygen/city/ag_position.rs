//! Port of activitygen/city/AGPosition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A location in the 2D plane freely positioned on a street.
/// C++ class `AGPosition` (activitygen/city/AGPosition.h)
pub struct AGPosition {
    pub street: Ptr<AGStreet>,
    pub position: f64,
    pub pos2d: Position,
}

impl AGPosition {
    /// C++: `void AGPosition::AGPosition(const AGStreet &, double)`
    /// Constructs an AGPosition at a certain point on a street.
    // AGPosition::AGPosition(const AGStreet& str, double pos) :
    //     street(&str), position(pos), pos2d(compute2dPosition()) {
    // }
    #[allow(unused_variables)]
    pub fn new(str: &AGStreet, pos: f64) -> Self {
        todo!("port AGPosition::AGPosition")
    }

    /// C++: `void AGPosition::AGPosition(const AGStreet &)`
    /// Constructs an AGPosition at a random point on a street.
    // AGPosition::AGPosition(const AGStreet& str) :
    //     street(&str), position(randomPositionInStreet(str)), pos2d(compute2dPosition()) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(str: &AGStreet) -> Self {
        todo!("port AGPosition::AGPosition")
    }

    /// C++: `const AGStreet & AGPosition::getStreet() const` [const]
    /// Provides the street this AGPosition is located on.
    // const AGStreet&
    // AGPosition::getStreet() const {
    //     return *street;
    // }
    #[allow(unused_variables)]
    pub fn get_street(&self) -> &AGStreet {
        todo!("port AGPosition::getStreet")
    }

    /// C++: `double AGPosition::getPosition() const` [const]
    /// Provides the relative position of this AGPosition on the street.
    // double
    // AGPosition::getPosition() const {
    //     return position;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> f64 {
        todo!("port AGPosition::getPosition")
    }

    /// C++: `bool AGPosition::operator==(const AGPosition &) const` [const]
    /// Tests whether two positions are at the same place.
    // bool
    // AGPosition::operator==(const AGPosition& pos) const {
    //     return pos2d.almostSame(pos.pos2d);
    // }
    #[allow(unused_variables)]
    pub fn eq_op(&self, pos: &AGPosition) -> bool {
        todo!("port AGPosition::operator==")
    }

    /// C++: `double AGPosition::distanceTo(const AGPosition &) const` [const]
    /// Computes the distance between two AGPosition objects.
    // double
    // AGPosition::distanceTo(const AGPosition& otherPos) const {
    //     return pos2d.distanceTo(otherPos.pos2d);
    // }
    #[allow(unused_variables)]
    pub fn distance_to(&self, otherPos: &AGPosition) -> f64 {
        todo!("port AGPosition::distanceTo")
    }

    /// C++: `double AGPosition::minDistanceTo(const std::list<AGPosition> &) const` [const]
    /// Computes the distance to the closest position in a list.
    // double
    // AGPosition::minDistanceTo(const std::list<AGPosition>& positions) const {
    //     double minDist = std::numeric_limits<double>::infinity();
    //     double tempDist;
    //     std::list<AGPosition>::const_iterator itt;
    //
    //     for (itt = positions.begin(); itt != positions.end(); ++itt) {
    //         tempDist = this->distanceTo(*itt);
    //         if (tempDist < minDist) {
    //             minDist = tempDist;
    //         }
    //     }
    //     return minDist;
    // }
    #[allow(unused_variables)]
    pub fn min_distance_to(&self, positions: &VecDeque<AGPosition>) -> f64 {
        todo!("port AGPosition::minDistanceTo")
    }

    /// C++: `double AGPosition::minDistanceTo(const std::map<int, AGPosition> &) const` [const]
    /// Computes the distance to the closest position in a map.
    // double
    // AGPosition::minDistanceTo(const std::map<int, AGPosition>& positions) const {
    //     double minDist = std::numeric_limits<double>::infinity();
    //     double tempDist;
    //     std::map<int, AGPosition>::const_iterator itt;
    //
    //     for (itt = positions.begin(); itt != positions.end(); ++itt) {
    //         tempDist = this->distanceTo(itt->second);
    //         if (tempDist < minDist) {
    //             minDist = tempDist;
    //         }
    //     }
    //     return minDist;
    // }
    #[allow(unused_variables)]
    pub fn min_distance_to_2(&self, positions: &BTreeMap<i32, AGPosition>) -> f64 {
        todo!("port AGPosition::minDistanceTo")
    }

    /// C++: `void AGPosition::print() const` [const]
    /// Prints out a summary of the properties of this class on standard output.
    // void
    // AGPosition::print() const {
    //     std::cout << "- AGPosition: *Street=" << street << " position=" << position << "/" << street->getLength() << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        todo!("port AGPosition::print")
    }

    /// C++: `double AGPosition::randomPositionInStreet(const AGStreet &)` [static, private]
    /// Determines a random relative position on a street.
    // double
    // AGPosition::randomPositionInStreet(const AGStreet& s) {
    //     return RandHelper::rand(0.0, s.getLength());
    // }
    #[allow(unused_variables)]
    pub fn random_position_in_street(street: &AGStreet) -> f64 {
        todo!("port AGPosition::randomPositionInStreet")
    }

    /// C++: `Position AGPosition::compute2dPosition() const` [const, private]
    /// Creates a Position object to the street and position attribute of this class.
    // Position
    // AGPosition::compute2dPosition() const {
    //     // P = From + pos*(To - From) = pos*To + (1-pos)*From
    //     Position From = street->getFromJunction()->getPosition();
    //     Position To = street->getToJunction()->getPosition();
    //     Position position2d(To);
    //
    //     position2d.sub(From);
    //     position2d.mul(position / street->getLength());
    //     position2d.add(From);
    //
    //     return position2d;
    // }
    #[allow(unused_variables)]
    pub fn compute2d_position(&self) -> Position {
        todo!("port AGPosition::compute2dPosition")
    }

}

