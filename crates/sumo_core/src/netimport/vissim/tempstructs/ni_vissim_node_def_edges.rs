//! Port of netimport/vissim/tempstructs/NIVissimNodeDef_Edges.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimNodeDef_Edges` (netimport/vissim/tempstructs/NIVissimNodeDef_Edges.h)
pub struct NIVissimNodeDef_Edges {
    /// C++ base class `NIVissimNodeDef`
    pub base: NIVissimNodeDef,
    pub myEdges: NIVissimNodeParticipatingEdgeVector,
}

impl std::ops::Deref for NIVissimNodeDef_Edges {
    type Target = NIVissimNodeDef;
    fn deref(&self) -> &NIVissimNodeDef {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimNodeDef_Edges {
    fn deref_mut(&mut self) -> &mut NIVissimNodeDef {
        &mut self.base
    }
}

impl NIVissimNodeDef_Edges {
    /// C++: `void NIVissimNodeDef_Edges::NIVissimNodeDef_Edges(int, const std::string &, const NIVissimNodeParticipatingEdgeVector &)`
    // NIVissimNodeDef_Edges::NIVissimNodeDef_Edges(int id,
    //         const std::string& name, const NIVissimNodeParticipatingEdgeVector& edges)
    //     : NIVissimNodeDef(id, name), myEdges(edges) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, edges: &NIVissimNodeParticipatingEdgeVector) -> Self {
        todo!("port NIVissimNodeDef_Edges::NIVissimNodeDef_Edges")
    }

    /// C++: `void NIVissimNodeDef_Edges::~NIVissimNodeDef_Edges() noexcept` [virtual]
    // NIVissimNodeDef_Edges::~NIVissimNodeDef_Edges() {
    //     for (NIVissimNodeParticipatingEdgeVector::iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         delete (*i);
    //     }
    //     myEdges.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimNodeDef_Edges::~NIVissimNodeDef_Edges")
    }

    /// C++: `bool NIVissimNodeDef_Edges::dictionary(int, const std::string &, const NIVissimNodeParticipatingEdgeVector &)` [static]
    // bool
    // NIVissimNodeDef_Edges::dictionary(int id, const std::string& name,
    //                                   const NIVissimNodeParticipatingEdgeVector& edges) {
    //     NIVissimNodeDef_Edges* o = new NIVissimNodeDef_Edges(id, name, edges);
    //     if (!NIVissimNodeDef::dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, name: &str, edges: &NIVissimNodeParticipatingEdgeVector) -> bool {
        todo!("port NIVissimNodeDef_Edges::dictionary")
    }

    /// C++: `double NIVissimNodeDef_Edges::getEdgePosition(int) const` [const, virtual]
    // double
    // NIVissimNodeDef_Edges::getEdgePosition(int edgeid) const {
    //     for (NIVissimNodeParticipatingEdgeVector::const_iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         NIVissimNodeParticipatingEdge* edge = *i;
    //         if (edge->getID() == edgeid) {
    //             return (edge->getFromPos() + edge->getToPos()) / (double) 2.0;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_position(&self, edgeid: i32) -> f64 {
        todo!("port NIVissimNodeDef_Edges::getEdgePosition")
    }

}

/// C++ class `NIVissimNodeDef_Edges::id_matches` (netimport/vissim/tempstructs/NIVissimNodeDef_Edges.h)
pub struct NIVissimNodeDef_Edges_id_matches {
    pub myEdgeID: i32,
}

impl NIVissimNodeDef_Edges_id_matches {
    /// C++: `void id_matches::id_matches(int)`
    // explicit id_matches(int id) : myEdgeID(id) { }
    #[allow(unused_variables)]
    pub fn new(id: i32) -> Self {
        todo!("port id_matches::id_matches")
    }

    /// C++: `bool id_matches::operator()(NIVissimNodeParticipatingEdge *)`
    // bool operator()(NIVissimNodeParticipatingEdge* e) {
    //             return e->getID() == myEdgeID;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, e: Ptr<NIVissimNodeParticipatingEdge>) -> bool {
        todo!("port id_matches::operator()")
    }

}

/// C++ class `NIVissimNodeDef_Edges::lying_within_match` (netimport/vissim/tempstructs/NIVissimNodeDef_Edges.h)
pub struct NIVissimNodeDef_Edges_lying_within_match {
    pub myEdge: Ptr<NIVissimNodeParticipatingEdge>,
}

impl NIVissimNodeDef_Edges_lying_within_match {
    /// C++: `void lying_within_match::lying_within_match(NIVissimNodeParticipatingEdge *)`
    // explicit lying_within_match(NIVissimNodeParticipatingEdge* e) : myEdge(e) { }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NIVissimNodeParticipatingEdge>) -> Self {
        todo!("port lying_within_match::lying_within_match")
    }

    /// C++: `bool lying_within_match::operator()(NIVissimExtendedEdgePoint *)`
    // bool operator()(NIVissimExtendedEdgePoint* e) {
    //             return e->getEdgeID() == myEdge->getID() &&
    //                    myEdge->positionLiesWithin(e->getPosition());
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, e: Ptr<NIVissimExtendedEdgePoint>) -> bool {
        todo!("port lying_within_match::operator()")
    }

}

