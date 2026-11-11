//! Port of netbuild/NBCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `KeepClear`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum KeepClear {
    KEEPCLEAR_FALSE = 0,
    KEEPCLEAR_TRUE = 1,
    KEEPCLEAR_UNSPECIFIED = -1,
}

/// C++ typedef `std::vector<std::pair<const NBRouterEdge *, const NBRouterEdge *>> ConstRouterEdgePairVector`
pub type ConstRouterEdgePairVector = Vec<(Ptr<NBRouterEdge>, Ptr<NBRouterEdge>)>;

/// C++ typedef `std::vector<const NBRouterEdge *> ConstRouterEdgeVector`
pub type ConstRouterEdgeVector = Vec<Ptr<NBRouterEdge>>;

/// C++ typedef `std::set<NBEdge *> EdgeSet`
pub type EdgeSet = BTreeSet<Ptr<NBEdge>>;

/// C++ typedef `std::vector<NBEdge *> EdgeVector`
pub type EdgeVector = Vec<Ptr<NBEdge>>;

/// C++ typedef `std::vector<int> LaneVector`
pub type LaneVector = Vec<i32>;

/// C++ typedef `std::vector<std::pair<const NBRouterEdge *, const NBRouterEdge *>> NBViaSuccessor`
pub type NBViaSuccessor = Vec<(Ptr<NBRouterEdge>, Ptr<NBRouterEdge>)>;

/// C++ typedef `std::set<NBNode *, ComparatorIdLess> NodeSet`
pub type NodeSet = BTreeSet<Ptr<NBNode>>;

/// C++ typedef `std::vector<NBRouterEdge *> RouterEdgeVector`
pub type RouterEdgeVector = Vec<Ptr<NBRouterEdge>>;

