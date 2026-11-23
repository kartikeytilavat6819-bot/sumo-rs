//! Port of router/ROHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A function for sorting vehicles by their departure time
/// C++ class `ROVehicleByDepartureComperator` (router/ROHelper.h)
#[repr(C)]
pub struct ROVehicleByDepartureComperator {
    /// C++ base class `std::less<ROVehicle *>`
    pub base: Unmapped /* std::less<ROVehicle *> */,
}

impl std::ops::Deref for ROVehicleByDepartureComperator {
    type Target = Unmapped /* std::less<ROVehicle *> */;
    fn deref(&self) -> &Unmapped /* std::less<ROVehicle *> */ {
        &self.base
    }
}
impl std::ops::DerefMut for ROVehicleByDepartureComperator {
    fn deref_mut(&mut self) -> &mut Unmapped /* std::less<ROVehicle *> */ {
        &mut self.base
    }
}

impl ROVehicleByDepartureComperator {
    /// C++: `void ROVehicleByDepartureComperator::ROVehicleByDepartureComperator()`
    /// Constructor
    // explicit ROVehicleByDepartureComperator() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        ROVehicleByDepartureComperator { base: () }
    }

    /// C++: `void ROVehicleByDepartureComperator::~ROVehicleByDepartureComperator() noexcept`
    /// Destructor
    // ~ROVehicleByDepartureComperator() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool ROVehicleByDepartureComperator::operator()(ROVehicle *, ROVehicle *) const` [const]
    /// Comparing operator
    // bool operator()(ROVehicle* veh1, ROVehicle* veh2) const {
    //         if (veh1->getDepart() == veh2->getDepart()) {
    //             return veh1->getID() > veh2->getID();
    //         }
    //         return veh1->getDepart() > veh2->getDepart();
    //     }
    #[allow(unused_variables)]
    pub fn call_op(&self, veh1: Ptr<ROVehicle>, veh2: Ptr<ROVehicle>) -> bool {
        if veh1.get_depart() == veh2.get_depart() {
            return veh1.get_id() > veh2.get_id();
        }
        veh1.get_depart() > veh2.get_depart()
    }

}

/// C++: `bool (const ConstROEdgeVector &, ConstROEdgeVector::const_iterator, ConstROEdgeVector::const_iterator) ROHelper::noMandatory`
// bool
// noMandatory(const ConstROEdgeVector& mandatory,
//             ConstROEdgeVector::const_iterator start,
//             ConstROEdgeVector::const_iterator end) {
//     for (const ROEdge* m : mandatory) {
//         for (auto it = start; it != end; it++) {
//             if (*it == m) {
//                 return false;
//             }
//         }
//     }
//     return true;
// }
#[allow(unused_variables)]
/// `range` is the C++ iterator pair [start, end) as a slice
pub fn no_mandatory(mandatory: &ConstROEdgeVector, range: &[Ptr<ROEdge>]) -> bool {
    for m in mandatory {
        for it in range {
            if it == m {
                return false;
            }
        }
    }
    true
}

/// C++: `void (ConstROEdgeVector &, const ConstROEdgeVector &) ROHelper::recheckForLoops`
// void
// recheckForLoops(ConstROEdgeVector& edges, const ConstROEdgeVector& mandatory) {
//     // for simplicities sake, prevent removal of any mandatory edges
//     // in theory these edges could occur multiple times so it might be possible
//     // to delete some of them anyway.
//     // XXX check for departLane, departPos, departSpeed, ....
//
//     // removal of edge loops within the route (edge occurs twice)
//     // call repeatedly until no more loops have been found
//     bool findLoop = true;
//     while (findLoop) {
//         findLoop = false;
//         std::map<const ROEdge*, int> lastOccurrence; // index of the last occurrence of this edge
//         for (int ii = 0; ii < (int)edges.size(); ++ii) {
//             std::map<const ROEdge*, int>::iterator it_pre = lastOccurrence.find(edges[ii]);
//             if (it_pre != lastOccurrence.end() &&
//                     noMandatory(mandatory, edges.begin() + it_pre->second, edges.begin() + ii)) {
//                 edges.erase(edges.begin() + it_pre->second, edges.begin() + ii);
//                 ii = it_pre->second;
//                 findLoop = true;
//                 break;
//             } else {
//                 lastOccurrence[edges[ii]] = ii;
//             }
//         }
//     }
//
//     // remove loops at the route's begin
//     //  (vehicle makes a turnaround to get into the right direction at an already passed node)
//     const RONode* start = edges[0]->getFromJunction();
//     if (start == nullptr && edges.size() > 1) {
//         // taz edge has no fromJunction
//         start = edges[1]->getFromJunction();
//     }
//     if (start != nullptr) {
//         int lastStart = 0;
//         for (int i = 1; i < (int)edges.size(); i++) {
//             if (edges[i]->getFromJunction() == start) {
//                 lastStart = i;
//             }
//         }
//         if (lastStart > 0 && noMandatory(mandatory, edges.begin(), edges.begin() + lastStart - 1)) {
//             edges.erase(edges.begin(), edges.begin() + lastStart - 1);
//         }
//     }
//     // remove loops at the route's end
//     //  (vehicle makes a turnaround to get into the right direction at an already passed node)
//     const RONode* end = edges.back()->getToJunction();
//     if (end == nullptr && edges.size() > 1) {
//         // taz edge has no toJunction
//         end = edges[edges.size() - 2]->getToJunction();
//     }
//     if (end != nullptr) {
//         for (int i = 0; i < (int)edges.size() - 1; i++) {
//             if (edges[i]->getToJunction() == end && noMandatory(mandatory, edges.begin() + i + 2, edges.end())) {
//                 edges.erase(edges.begin() + i + 2, edges.end());
//                 break;
//             }
//         }
//     }
//
//     // removal of node loops (node occurs twice) is not done because these may occur legitimately
//     /*
//     std::vector<RONode*> nodes;
//     for (ConstROEdgeVector::iterator i = edges.begin(); i != edges.end(); ++i) {
//         nodes.push_back((*i)->getFromJunction());
//     }
//     nodes.push_back(edges.back()->getToJunction());
//     bool changed = false;
//     do {
//         changed = false;
//         for (int b = 0; b < nodes.size() && !changed; ++b) {
//             RONode* bn = nodes[b];
//             for (int e = b + 1; e < nodes.size() && !changed; ++e) {
//                 if (bn == nodes[e]) {
//                     changed = true;
//                     nodes.erase(nodes.begin() + b, nodes.begin() + e);
//                     edges.erase(edges.begin() + b, edges.begin() + e);
//                 }
//             }
//         }
//     } while (changed);
//     */
// }
#[allow(unused_variables)]
pub fn recheck_for_loops(edges: &mut ConstROEdgeVector, mandatory: &ConstROEdgeVector) {
    // for simplicities sake, prevent removal of any mandatory edges
    // in theory these edges could occur multiple times so it might be possible
    // to delete some of them anyway.
    // XXX check for departLane, departPos, departSpeed, ....

    // removal of edge loops within the route (edge occurs twice)
    // call repeatedly until no more loops have been found
    let mut findLoop = true;
    while findLoop {
        findLoop = false;
        let mut lastOccurrence: BTreeMap<Ptr<ROEdge>, usize> = BTreeMap::new(); // index of the last occurrence of this edge
        for ii in 0..edges.len() {
            if let Some(&pre) = lastOccurrence.get(&edges[ii]) {
                if no_mandatory(mandatory, &edges[pre..ii]) {
                    edges.drain(pre..ii);
                    findLoop = true;
                    break;
                }
            }
            lastOccurrence.insert(edges[ii], ii);
        }
    }

    // remove loops at the route's begin
    //  (vehicle makes a turnaround to get into the right direction at an already passed node)
    let mut start = edges[0].get_from_junction();
    if start.is_null() && edges.len() > 1 {
        // taz edge has no fromJunction
        start = edges[1].get_from_junction();
    }
    if !start.is_null() {
        let mut lastStart: usize = 0;
        for i in 1..edges.len() {
            if edges[i].get_from_junction() == start {
                lastStart = i;
            }
        }
        if lastStart > 0 && no_mandatory(mandatory, &edges[0..lastStart - 1]) {
            edges.drain(0..lastStart - 1);
        }
    }
    // remove loops at the route's end
    //  (vehicle makes a turnaround to get into the right direction at an already passed node)
    let mut end = edges[edges.len() - 1].get_to_junction();
    if end.is_null() && edges.len() > 1 {
        // taz edge has no toJunction
        end = edges[edges.len() - 2].get_to_junction();
    }
    if !end.is_null() {
        for i in 0..edges.len() - 1 {
            if edges[i].get_to_junction() == end && no_mandatory(mandatory, &edges[i + 2..]) {
                edges.drain(i + 2..);
                break;
            }
        }
    }

    // removal of node loops (node occurs twice) is not done because these may occur legitimately
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_mandatory_detects_edge_in_range() {
        // C++ ROHelper::noMandatory: false iff any mandatory edge occurs in [start, end).
        let e: Ptr<ROEdge> = Ptr::null();
        let mandatory: ConstROEdgeVector = vec![e];
        assert!(no_mandatory(&mandatory, &[]));
        assert!(!no_mandatory(&mandatory, &[e]));
    }
}

