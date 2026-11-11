//! Port of netimport/vissim/tempstructs/NIVissimNodeParticipatingEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimNodeParticipatingEdge` (netimport/vissim/tempstructs/NIVissimNodeParticipatingEdge.h)
pub struct NIVissimNodeParticipatingEdge {
    pub myEdgeID: i32,
    pub myFromPos: f64,
    pub myToPos: f64,
}

impl NIVissimNodeParticipatingEdge {
    /// C++: `void NIVissimNodeParticipatingEdge::NIVissimNodeParticipatingEdge(int, double, double)`
    // NIVissimNodeParticipatingEdge::NIVissimNodeParticipatingEdge(
    //     int edgeid, double frompos, double topos)
    //     : myEdgeID(edgeid), myFromPos(frompos), myToPos(topos) {}
    #[allow(unused_variables)]
    pub fn new(edgeid: i32, frompos: f64, topos: f64) -> Self {
        todo!("port NIVissimNodeParticipatingEdge::NIVissimNodeParticipatingEdge")
    }

    /// C++: `void NIVissimNodeParticipatingEdge::~NIVissimNodeParticipatingEdge()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimNodeParticipatingEdge::~NIVissimNodeParticipatingEdge")
    }

    /// C++: `int NIVissimNodeParticipatingEdge::getID() const` [const]
    // int
    // NIVissimNodeParticipatingEdge::getID() const {
    //     return myEdgeID;
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> i32 {
        todo!("port NIVissimNodeParticipatingEdge::getID")
    }

    /// C++: `bool NIVissimNodeParticipatingEdge::positionLiesWithin(double) const` [const]
    // bool
    // NIVissimNodeParticipatingEdge::positionLiesWithin(double pos) const {
    //     return
    //         (myFromPos < myToPos && myFromPos >= pos && myToPos <= pos)
    //         ||
    //         (myFromPos > myToPos && myFromPos <= pos && myToPos >= pos);
    // }
    #[allow(unused_variables)]
    pub fn position_lies_within(&self, pos: f64) -> bool {
        todo!("port NIVissimNodeParticipatingEdge::positionLiesWithin")
    }

    /// C++: `double NIVissimNodeParticipatingEdge::getFromPos() const` [const]
    // double
    // NIVissimNodeParticipatingEdge::getFromPos() const {
    //     return myFromPos;
    // }
    #[allow(unused_variables)]
    pub fn get_from_pos(&self) -> f64 {
        todo!("port NIVissimNodeParticipatingEdge::getFromPos")
    }

    /// C++: `double NIVissimNodeParticipatingEdge::getToPos() const` [const]
    // double
    // NIVissimNodeParticipatingEdge::getToPos() const {
    //     return myToPos;
    // }
    #[allow(unused_variables)]
    pub fn get_to_pos(&self) -> f64 {
        todo!("port NIVissimNodeParticipatingEdge::getToPos")
    }

}

