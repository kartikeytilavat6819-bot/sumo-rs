//! Port of netimport/vissim/tempstructs/NIVissimAbstractEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimAbstractEdge *> NIVissimAbstractEdge::DictType`
pub type NIVissimAbstractEdge_DictType = BTreeMap<i32, Ptr<NIVissimAbstractEdge>>;

/// C++ class `NIVissimAbstractEdge` (netimport/vissim/tempstructs/NIVissimAbstractEdge.h)
pub struct NIVissimAbstractEdge {
    pub myID: i32,
    pub myGeom: PositionVector,
    pub myDisturbances: Vec<i32>,
    pub myNode: i32,
}

// C++ static members of NIVissimAbstractEdge (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimAbstractEdge::myDict;

impl NIVissimAbstractEdge {
    /// C++: `void NIVissimAbstractEdge::NIVissimAbstractEdge(int, const PositionVector &)`
    // NIVissimAbstractEdge::NIVissimAbstractEdge(int id,
    //         const PositionVector& geom)
    //     : myID(id), myNode(-1) {
    //     // convert/publicate geometry
    //     for (PositionVector::const_iterator i = geom.begin(); i != geom.end(); ++i) {
    //         Position p = *i;
    //         if (!NBNetBuilder::transformCoordinate(p)) {
    //             WRITE_WARNINGF(TL("Unable to project coordinates for edge '%'."), toString(id));
    //         }
    //         myGeom.push_back_noDoublePos(p);
    //     }
    //     //
    //     dictionary(id, this);
    // }
    #[allow(unused_variables)]
    pub fn new(id: i32, geom: &PositionVector) -> Self {
        todo!("port NIVissimAbstractEdge::NIVissimAbstractEdge")
    }

    /// C++: `void NIVissimAbstractEdge::~NIVissimAbstractEdge() noexcept` [virtual]
    // NIVissimAbstractEdge::~NIVissimAbstractEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimAbstractEdge::~NIVissimAbstractEdge")
    }

    /// C++: `Position NIVissimAbstractEdge::getGeomPosition(double) const` [const]
    // Position
    // NIVissimAbstractEdge::getGeomPosition(double pos) const {
    //     if (myGeom.length() > pos) {
    //         return myGeom.positionAtOffset(pos);
    //     } else if (myGeom.length() == pos) {
    //         return myGeom[-1];
    //     } else {
    //         PositionVector g(myGeom);
    //         const double amount = pos - myGeom.length();
    //         g.extrapolate(amount * 2);
    //         return g.positionAtOffset(pos + amount * 2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_geom_position(&self, pos: f64) -> Position {
        todo!("port NIVissimAbstractEdge::getGeomPosition")
    }

    /// C++: `void NIVissimAbstractEdge::splitAssigning()`
    // void
    // NIVissimAbstractEdge::splitAssigning() {}
    #[allow(unused_variables)]
    pub fn split_assigning(&mut self) {
        todo!("port NIVissimAbstractEdge::splitAssigning")
    }

    /// C++: `bool NIVissimAbstractEdge::crossesEdge(NIVissimAbstractEdge *) const` [const]
    // bool
    // NIVissimAbstractEdge::crossesEdge(NIVissimAbstractEdge* c) const {
    //     return myGeom.intersects(c->myGeom);
    // }
    #[allow(unused_variables)]
    pub fn crosses_edge(&self, c: Ptr<NIVissimAbstractEdge>) -> bool {
        todo!("port NIVissimAbstractEdge::crossesEdge")
    }

    /// C++: `Position NIVissimAbstractEdge::crossesEdgeAtPoint(NIVissimAbstractEdge *) const` [const]
    // Position
    // NIVissimAbstractEdge::crossesEdgeAtPoint(NIVissimAbstractEdge* c) const {
    //     return myGeom.intersectionPosition2D(c->myGeom);
    // }
    #[allow(unused_variables)]
    pub fn crosses_edge_at_point(&self, c: Ptr<NIVissimAbstractEdge>) -> Position {
        todo!("port NIVissimAbstractEdge::crossesEdgeAtPoint")
    }

    /// C++: `bool NIVissimAbstractEdge::overlapsWith(const AbstractPoly &, double) const` [const]
    // bool
    // NIVissimAbstractEdge::overlapsWith(const AbstractPoly& p, double offset) const {
    //     return myGeom.overlapsWith(p, offset);
    // }
    #[allow(unused_variables)]
    pub fn overlaps_with(&self, p: &dyn AbstractPoly, offset: f64) -> bool {
        todo!("port NIVissimAbstractEdge::overlapsWith")
    }

    /// C++: `void NIVissimAbstractEdge::setNodeCluster(int)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_node_cluster(&mut self, nodeid: i32) {
        todo!("port NIVissimAbstractEdge::setNodeCluster")
    }

    /// C++: `bool NIVissimAbstractEdge::hasNodeCluster() const` [const]
    // bool
    // NIVissimAbstractEdge::hasNodeCluster() const {
    //     return myNode != -1;
    // }
    #[allow(unused_variables)]
    pub fn has_node_cluster(&self) -> bool {
        todo!("port NIVissimAbstractEdge::hasNodeCluster")
    }

    /// C++: `void NIVissimAbstractEdge::buildGeom()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_geom(&mut self) {
        todo!("port NIVissimAbstractEdge::buildGeom")
    }

    /// C++: `int NIVissimAbstractEdge::getID() const` [const]
    // int
    // NIVissimAbstractEdge::getID() const {
    //     return myID;
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> i32 {
        todo!("port NIVissimAbstractEdge::getID")
    }

    /// C++: `const PositionVector & NIVissimAbstractEdge::getGeometry() const` [const]
    // const PositionVector&
    // NIVissimAbstractEdge::getGeometry() const {
    //     return myGeom;
    // }
    #[allow(unused_variables)]
    pub fn get_geometry(&self) -> &PositionVector {
        todo!("port NIVissimAbstractEdge::getGeometry")
    }

    /// C++: `void NIVissimAbstractEdge::addDisturbance(int)`
    // void
    // NIVissimAbstractEdge::addDisturbance(int disturbance) {
    //     myDisturbances.push_back(disturbance);
    // }
    #[allow(unused_variables)]
    pub fn add_disturbance(&mut self, disturbance: i32) {
        todo!("port NIVissimAbstractEdge::addDisturbance")
    }

    /// C++: `const std::vector<int> & NIVissimAbstractEdge::getDisturbances() const` [const]
    // const std::vector<int>&
    // NIVissimAbstractEdge::getDisturbances() const {
    //     return myDisturbances;
    // }
    #[allow(unused_variables)]
    pub fn get_disturbances(&self) -> &Vec<i32> {
        todo!("port NIVissimAbstractEdge::getDisturbances")
    }

    /// C++: `bool NIVissimAbstractEdge::dictionary(int, NIVissimAbstractEdge *)` [static]
    // bool
    // NIVissimAbstractEdge::dictionary(int id, NIVissimAbstractEdge* e) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = e;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, e: Ptr<NIVissimAbstractEdge>) -> bool {
        todo!("port NIVissimAbstractEdge::dictionary")
    }

    /// C++: `NIVissimAbstractEdge * NIVissimAbstractEdge::dictionary(int)` [static]
    // NIVissimAbstractEdge*
    // NIVissimAbstractEdge::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32) -> Ptr<NIVissimAbstractEdge> {
        todo!("port NIVissimAbstractEdge::dictionary")
    }

    /// C++: `void NIVissimAbstractEdge::splitAndAssignToNodes()` [static]
    // void
    // NIVissimAbstractEdge::splitAndAssignToNodes() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimAbstractEdge* e = (*i).second;
    //         e->splitAssigning();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn split_and_assign_to_nodes() {
        todo!("port NIVissimAbstractEdge::splitAndAssignToNodes")
    }

    /// C++: `std::vector<int> NIVissimAbstractEdge::getWithin(const AbstractPoly &, double)` [static]
    // std::vector<int>
    // NIVissimAbstractEdge::getWithin(const AbstractPoly& p, double offset) {
    //     std::vector<int> ret;
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimAbstractEdge* e = (*i).second;
    //         if (e->overlapsWith(p, offset)) {
    //             ret.push_back(e->myID);
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_within(p: &dyn AbstractPoly, offset: f64) -> Vec<i32> {
        todo!("port NIVissimAbstractEdge::getWithin")
    }

    /// C++: `void NIVissimAbstractEdge::clearDict()` [static]
    // void
    // NIVissimAbstractEdge::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        todo!("port NIVissimAbstractEdge::clearDict")
    }

}

