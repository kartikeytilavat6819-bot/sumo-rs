//! Port of netimport/vissim/tempstructs/NIVissimExtendedEdgePoint.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimExtendedEdgePoint` (netimport/vissim/tempstructs/NIVissimExtendedEdgePoint.h)
pub struct NIVissimExtendedEdgePoint {
    pub myEdgeID: i32,
    pub myLanes: Vec<i32>,
    pub myPosition: f64,
    pub myAssignedVehicles: Vec<i32>,
}

impl NIVissimExtendedEdgePoint {
    /// C++: `void NIVissimExtendedEdgePoint::NIVissimExtendedEdgePoint(int, const std::vector<int> &, double, const std::vector<int> &)`
    /// Constructor
    // NIVissimExtendedEdgePoint::NIVissimExtendedEdgePoint(
    //     int edgeid, const std::vector<int>& lanes, double position,
    //     const std::vector<int>& assignedVehicles)
    //     : myEdgeID(edgeid), myLanes(lanes), myPosition(position),
    //       myAssignedVehicles(assignedVehicles) {}
    #[allow(unused_variables)]
    pub fn new(edgeid: i32, lanes: &Vec<i32>, position: f64, assignedVehicles: &Vec<i32>) -> Self {
        todo!("port NIVissimExtendedEdgePoint::NIVissimExtendedEdgePoint")
    }

    /// C++: `void NIVissimExtendedEdgePoint::~NIVissimExtendedEdgePoint() noexcept`
    // NIVissimExtendedEdgePoint::~NIVissimExtendedEdgePoint() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimExtendedEdgePoint::~NIVissimExtendedEdgePoint")
    }

    /// C++: `int NIVissimExtendedEdgePoint::getEdgeID() const` [const]
    // int
    // NIVissimExtendedEdgePoint::getEdgeID() const {
    //     return myEdgeID;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(&self) -> i32 {
        todo!("port NIVissimExtendedEdgePoint::getEdgeID")
    }

    /// C++: `double NIVissimExtendedEdgePoint::getPosition() const` [const]
    // double
    // NIVissimExtendedEdgePoint::getPosition() const {
    //     return myPosition;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> f64 {
        todo!("port NIVissimExtendedEdgePoint::getPosition")
    }

    /// C++: `Position NIVissimExtendedEdgePoint::getGeomPosition() const` [const]
    // Position
    // NIVissimExtendedEdgePoint::getGeomPosition() const {
    //     return
    //         NIVissimAbstractEdge::dictionary(myEdgeID)->getGeomPosition(myPosition);
    // }
    #[allow(unused_variables)]
    pub fn get_geom_position(&self) -> Position {
        todo!("port NIVissimExtendedEdgePoint::getGeomPosition")
    }

    /// C++: `const std::vector<int> & NIVissimExtendedEdgePoint::getLanes() const` [const]
    // const std::vector<int>&
    // NIVissimExtendedEdgePoint::getLanes() const {
    //     return myLanes;
    // }
    #[allow(unused_variables)]
    pub fn get_lanes(&self) -> &Vec<i32> {
        todo!("port NIVissimExtendedEdgePoint::getLanes")
    }

    /// C++: `void NIVissimExtendedEdgePoint::recheckLanes(const NBEdge *const)`
    /// Resets lane numbers if all lanes shall be used
    // void
    // NIVissimExtendedEdgePoint::recheckLanes(const NBEdge* const edge) {
    //     // check whether an "all" indicator is there
    //     bool hadAll = false;
    //     for (std::vector<int>::const_iterator i = myLanes.begin(); !hadAll && i != myLanes.end(); ++i) {
    //         if ((*i) == -1) {
    //             hadAll = true;
    //         }
    //     }
    //     // no -> return
    //     if (!hadAll) {
    //         return;
    //     }
    //     // patch lane indices
    //     myLanes.clear();
    //     for (int i = 0; i < (int) edge->getNumLanes(); ++i) {
    //         myLanes.push_back(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_lanes(&mut self, edge: Ptr<NBEdge>) {
        todo!("port NIVissimExtendedEdgePoint::recheckLanes")
    }

}

