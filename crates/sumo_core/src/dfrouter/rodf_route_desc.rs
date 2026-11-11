//! Port of dfrouter/RODFRouteDesc.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A route within the DFROUTER
/// C++ class `RODFRouteDesc` (dfrouter/RODFRouteDesc.h)
pub struct RODFRouteDesc {
    /// The edges the route is made of
    pub edges2Pass: ROEdgeVector,
    /// The name of the route
    pub routename: String,
    pub duration_2: f64,
    pub distance: f64,
    pub passedNo: i32,
    pub endDetectorEdge: Ptr<ROEdge>,
    pub lastDetectorEdge: Ptr<ROEdge>,
    pub distance2Last: f64,
    pub duration2Last: SUMOTime,
    pub overallProb: f64,
    pub factor: f64,
}

impl RODFRouteDesc {
}

