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
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimAbstractEdge_DictType> = std::cell::RefCell::new(BTreeMap::new());
}

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
    /// Geometry part of the C++ constructor, without the final `dictionary(id, this)` registration.
    /// Subclasses embed the base by value, so they register it once their own Box is allocated.
    #[allow(unused_variables)]
    pub fn new_unregistered(id: i32, geom: &PositionVector) -> Self {
        let mut s = Self { myID: id, myGeom: PositionVector::default(), myDisturbances: Vec::new(), myNode: -1 };
        // convert/publicate geometry
        for i in geom.iter() {
            let mut p = *i;
            if !NBNetBuilder::transform_coordinate(&mut p, true, Ptr::null()) {
                write_warningf!("Unable to project coordinates for edge '%'.", id);
            }
            s.myGeom.push_back_no_double_pos(&p);
        }
        s
    }

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
    // the constructor registers `this` in myDict, so the object must stay at a fixed address: return the Box
    pub fn new(id: i32, geom: &PositionVector) -> Box<Self> {
        let b = Box::new(Self::new_unregistered(id, geom));
        Self::dictionary(id, Ptr::from_box(&b));
        b
    }

    /// C++: `void NIVissimAbstractEdge::~NIVissimAbstractEdge() noexcept` [virtual]
    // NIVissimAbstractEdge::~NIVissimAbstractEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
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
        if self.myGeom.length() > pos {
            self.myGeom.position_at_offset(pos, 0.)
        } else if self.myGeom.length() == pos {
            *self.myGeom.at(-1)
        } else {
            let mut g = self.myGeom.clone();
            let amount = pos - self.myGeom.length();
            g.extrapolate(amount * 2., false, false);
            g.position_at_offset(pos + amount * 2., 0.)
        }
    }

    /// C++: `void NIVissimAbstractEdge::splitAssigning()`
    // void
    // NIVissimAbstractEdge::splitAssigning() {}
    #[allow(unused_variables)]
    pub fn split_assigning(&mut self) {
    }

    /// C++: `bool NIVissimAbstractEdge::crossesEdge(NIVissimAbstractEdge *) const` [const]
    // bool
    // NIVissimAbstractEdge::crossesEdge(NIVissimAbstractEdge* c) const {
    //     return myGeom.intersects(c->myGeom);
    // }
    #[allow(unused_variables)]
    pub fn crosses_edge(&self, c: Ptr<NIVissimAbstractEdge>) -> bool {
        self.myGeom.intersects_pv(&c.myGeom)
    }

    /// C++: `Position NIVissimAbstractEdge::crossesEdgeAtPoint(NIVissimAbstractEdge *) const` [const]
    // Position
    // NIVissimAbstractEdge::crossesEdgeAtPoint(NIVissimAbstractEdge* c) const {
    //     return myGeom.intersectionPosition2D(c->myGeom);
    // }
    #[allow(unused_variables)]
    pub fn crosses_edge_at_point(&self, c: Ptr<NIVissimAbstractEdge>) -> Position {
        self.myGeom.intersection_position2d_pv(&c.myGeom)
    }

    /// C++: `bool NIVissimAbstractEdge::overlapsWith(const AbstractPoly &, double) const` [const]
    // bool
    // NIVissimAbstractEdge::overlapsWith(const AbstractPoly& p, double offset) const {
    //     return myGeom.overlapsWith(p, offset);
    // }
    #[allow(unused_variables)]
    pub fn overlaps_with(&self, p: &dyn AbstractPoly, offset: f64) -> bool {
        self.myGeom.overlaps_with(p, offset)
    }

    /// C++: `void NIVissimAbstractEdge::setNodeCluster(int)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_node_cluster(&mut self, nodeid: i32) {
        panic!("NIVissimAbstractEdge::setNodeCluster is pure virtual; subclasses implement it")
    }

    /// C++: `bool NIVissimAbstractEdge::hasNodeCluster() const` [const]
    // bool
    // NIVissimAbstractEdge::hasNodeCluster() const {
    //     return myNode != -1;
    // }
    #[allow(unused_variables)]
    pub fn has_node_cluster(&self) -> bool {
        self.myNode != -1
    }

    /// C++: `void NIVissimAbstractEdge::buildGeom()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_geom(&mut self) {
        panic!("NIVissimAbstractEdge::buildGeom is pure virtual; subclasses implement it")
    }

    /// C++: `int NIVissimAbstractEdge::getID() const` [const]
    // int
    // NIVissimAbstractEdge::getID() const {
    //     return myID;
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> i32 {
        self.myID
    }

    /// C++: `const PositionVector & NIVissimAbstractEdge::getGeometry() const` [const]
    // const PositionVector&
    // NIVissimAbstractEdge::getGeometry() const {
    //     return myGeom;
    // }
    #[allow(unused_variables)]
    pub fn get_geometry(&self) -> &PositionVector {
        &self.myGeom
    }

    /// C++: `void NIVissimAbstractEdge::addDisturbance(int)`
    // void
    // NIVissimAbstractEdge::addDisturbance(int disturbance) {
    //     myDisturbances.push_back(disturbance);
    // }
    #[allow(unused_variables)]
    pub fn add_disturbance(&mut self, disturbance: i32) {
        self.myDisturbances.push(disturbance);
    }

    /// C++: `const std::vector<int> & NIVissimAbstractEdge::getDisturbances() const` [const]
    // const std::vector<int>&
    // NIVissimAbstractEdge::getDisturbances() const {
    //     return myDisturbances;
    // }
    #[allow(unused_variables)]
    pub fn get_disturbances(&self) -> &Vec<i32> {
        &self.myDisturbances
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
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if let std::collections::btree_map::Entry::Vacant(v) = d.entry(id) {
                v.insert(e);
                true
            } else {
                false
            }
        })
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
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or(Ptr::null()))
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
        let edges: Vec<Ptr<NIVissimAbstractEdge>> = MY_DICT.with(|d| d.borrow().values().copied().collect());
        for e in edges {
            e.m().split_assigning();
        }
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
        let edges: Vec<Ptr<NIVissimAbstractEdge>> = MY_DICT.with(|d| d.borrow().values().copied().collect());
        let mut ret = Vec::new();
        for e in edges {
            if e.overlaps_with(p, offset) {
                ret.push(e.myID);
            }
        }
        ret
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
        // shortcut: C++ deletes each entry; entries are only forgotten (leaked) since the concrete subclass type is not known here, upgrade with an owner that knows the concrete type
        MY_DICT.with(|d| d.borrow_mut().clear());
    }

}

