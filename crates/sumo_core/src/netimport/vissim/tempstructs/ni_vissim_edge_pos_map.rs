//! Port of netimport/vissim/tempstructs/NIVissimEdgePosMap.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, Range> NIVissimEdgePosMap::ContType`
pub type NIVissimEdgePosMap_ContType = BTreeMap<i32, NIVissimEdgePosMap_Range>;

/// C++ typedef `std::pair<double, double> NIVissimEdgePosMap::Range`
pub type NIVissimEdgePosMap_Range = (f64, f64);

/// C++ class `NIVissimEdgePosMap` (netimport/vissim/tempstructs/NIVissimEdgePosMap.h)
pub struct NIVissimEdgePosMap {
    pub myCont: NIVissimBoundedClusterObject_ContType,
}

impl NIVissimEdgePosMap {
    /// C++: `void NIVissimEdgePosMap::NIVissimEdgePosMap()`
    // NIVissimEdgePosMap::NIVissimEdgePosMap() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port NIVissimEdgePosMap::NIVissimEdgePosMap")
    }

    /// C++: `void NIVissimEdgePosMap::~NIVissimEdgePosMap() noexcept`
    // NIVissimEdgePosMap::~NIVissimEdgePosMap() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimEdgePosMap::~NIVissimEdgePosMap")
    }

    /// C++: `void NIVissimEdgePosMap::add(int, double)`
    // void
    // NIVissimEdgePosMap::add(int edgeid, double pos) {
    //     add(edgeid, pos, pos);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, edgeid: i32, pos: f64) {
        todo!("port NIVissimEdgePosMap::add")
    }

    /// C++: `void NIVissimEdgePosMap::add(int, double, double)`
    // void
    // NIVissimEdgePosMap::add(int edgeid, double from, double to) {
    //     if (from > to) {
    //         double tmp = from;
    //         from = to;
    //         to = tmp;
    //     }
    //     ContType::iterator i = myCont.find(edgeid);
    //     if (i == myCont.end()) {
    //         myCont[edgeid] = Range(from, to);
    //     } else {
    //         double pfrom = (*i).second.first;
    //         double pto = (*i).second.second;
    //         if (pfrom < from) {
    //             from = pfrom;
    //         }
    //         if (pto > to) {
    //             to = pto;
    //         }
    //         myCont[edgeid] = Range(from, to);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, edgeid: i32, from: f64, to: f64) {
        todo!("port NIVissimEdgePosMap::add")
    }

    /// C++: `void NIVissimEdgePosMap::join(NIVissimEdgePosMap &)`
    // void
    // NIVissimEdgePosMap::join(NIVissimEdgePosMap& with) {
    //     for (ContType::iterator i = with.myCont.begin(); i != with.myCont.end(); i++) {
    //         add((*i).first, (*i).second.first, (*i).second.second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join(&mut self, with: &mut NIVissimEdgePosMap) {
        todo!("port NIVissimEdgePosMap::join")
    }

}

