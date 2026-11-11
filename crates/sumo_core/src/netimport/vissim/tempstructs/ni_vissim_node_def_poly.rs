//! Port of netimport/vissim/tempstructs/NIVissimNodeDef_Poly.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimNodeDef_Poly` (netimport/vissim/tempstructs/NIVissimNodeDef_Poly.h)
pub struct NIVissimNodeDef_Poly {
    /// C++ base class `NIVissimNodeDef_Edges`
    pub base: NIVissimNodeDef_Edges,
    pub myPoly: PositionVector,
}

impl std::ops::Deref for NIVissimNodeDef_Poly {
    type Target = NIVissimNodeDef_Edges;
    fn deref(&self) -> &NIVissimNodeDef_Edges {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimNodeDef_Poly {
    fn deref_mut(&mut self) -> &mut NIVissimNodeDef_Edges {
        &mut self.base
    }
}

impl NIVissimNodeDef_Poly {
    /// C++: `void NIVissimNodeDef_Poly::NIVissimNodeDef_Poly(int, const std::string &, const PositionVector &)`
    // NIVissimNodeDef_Poly::NIVissimNodeDef_Poly(int id, const std::string& name,
    //         const PositionVector& poly)
    //     : NIVissimNodeDef_Edges(id, name, NIVissimNodeParticipatingEdgeVector()),
    //       myPoly(poly) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, poly: &PositionVector) -> Self {
        Self {
            base: NIVissimNodeDef_Edges::new(id, name, &NIVissimNodeParticipatingEdgeVector::new()),
            myPoly: poly.clone(),
        }
    }

    /// C++: `void NIVissimNodeDef_Poly::~NIVissimNodeDef_Poly() noexcept` [virtual]
    // NIVissimNodeDef_Poly::~NIVissimNodeDef_Poly() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty; the base destructor (NIVissimNodeDef_Edges) runs implicitly.
        self.base.drop_();
    }

    /// C++: `bool NIVissimNodeDef_Poly::dictionary(int, const std::string &, const PositionVector &)` [static]
    // bool
    // NIVissimNodeDef_Poly::dictionary(int id, const std::string& name,
    //                                  const PositionVector& poly) {
    //     NIVissimNodeDef_Poly* o = new NIVissimNodeDef_Poly(id, name, poly);
    //     if (!NIVissimNodeDef::dictionary(id, o)) {
    //         delete o;
    //         assert(false);
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, name: &str, poly: &PositionVector) -> bool {
        let o = Ptr::from_raw(Box::into_raw(Box::new(NIVissimNodeDef_Poly::new(id, name, poly))));
        if !NIVissimNodeDef::dictionary(id, Ptr::from_mut(&mut o.m().base.base)) {
            // delete o;  (runs the base destructors as in C++)
            o.m().drop_();
            drop(unsafe { Box::from_raw(o.as_raw()) });
            debug_assert!(false);
            return false;
        }
        true
    }

}

