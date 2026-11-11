//! Port of dfrouter/RODFRouteCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A container for DFROUTER-routes
/// C++ class `RODFRouteCont` (dfrouter/RODFRouteCont.h)
pub struct RODFRouteCont {
    /// Stored route descriptions
    pub myRoutes: Vec<RODFRouteDesc>,
    /// Counts how many routes connecting the key-edges were already stored
    pub myConnectionOccurrences: BTreeMap<(Ptr<ROEdge>, Ptr<ROEdge>), i32>, // C++: std::map<std::pair<ROEdge *, ROEdge *>, int>
}

impl RODFRouteCont {
    /// C++: `void RODFRouteCont::RODFRouteCont()`
    /// Constructor
    // RODFRouteCont::RODFRouteCont() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port RODFRouteCont::RODFRouteCont")
    }

    /// C++: `void RODFRouteCont::~RODFRouteCont()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFRouteCont::~RODFRouteCont")
    }

    /// C++: `void RODFRouteCont::addRouteDesc(RODFRouteDesc &)`
    /// Adds a route to the container
    // void
    // RODFRouteCont::addRouteDesc(RODFRouteDesc& desc) {
    //     // routes may be duplicate as in-between routes may have different starting points
    //     if (find_if(myRoutes.begin(), myRoutes.end(), route_finder(desc)) == myRoutes.end()) {
    //         // compute route id
    //         setID(desc);
    //         myRoutes.push_back(desc);
    //     } else {
    //         RODFRouteDesc& prev = *find_if(myRoutes.begin(), myRoutes.end(), route_finder(desc));
    //         prev.overallProb += desc.overallProb;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_route_desc(&mut self, desc: &mut RODFRouteDesc) {
        todo!("port RODFRouteCont::addRouteDesc")
    }

    /// C++: `bool RODFRouteCont::removeRouteDesc(RODFRouteDesc &)`
    /// Removes the given route description from the container
    // bool
    // RODFRouteCont::removeRouteDesc(RODFRouteDesc& desc) {
    //     std::vector<RODFRouteDesc>::const_iterator j = find_if(myRoutes.begin(), myRoutes.end(), route_finder(desc));
    //     if (j == myRoutes.end()) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn remove_route_desc(&mut self, desc: &mut RODFRouteDesc) -> bool {
        todo!("port RODFRouteCont::removeRouteDesc")
    }

    /// C++: `bool RODFRouteCont::save(std::vector<std::string> &, const std::string &, OutputDevice &)`
    /// Saves routes
    // bool
    // RODFRouteCont::save(std::vector<std::string>& saved,
    //                     const std::string& prependix, OutputDevice& out) {
    //     bool haveSavedOneAtLeast = false;
    //     for (std::vector<RODFRouteDesc>::const_iterator j = myRoutes.begin(); j != myRoutes.end(); ++j) {
    //         const RODFRouteDesc& desc = (*j);
    //         if (find(saved.begin(), saved.end(), desc.routename) != saved.end()) {
    //             continue;
    //         }
    //         saved.push_back((*j).routename);
    //         assert(desc.edges2Pass.size() >= 1);
    //         out.openTag(SUMO_TAG_ROUTE).writeAttr(SUMO_ATTR_ID, prependix + desc.routename);
    //         out << " edges=\"";
    //         for (ROEdgeVector::const_iterator k = desc.edges2Pass.begin(); k != desc.edges2Pass.end(); k++) {
    //             if (k != desc.edges2Pass.begin()) {
    //                 out << ' ';
    //             }
    //             out << (*k)->getID();
    //         }
    //         out << '"';
    //         out.closeTag();
    //         haveSavedOneAtLeast = true;
    //     }
    //     return haveSavedOneAtLeast;
    // }
    #[allow(unused_variables)]
    pub fn save(&mut self, saved: &mut Vec<String>, prependix: &str, out: &mut OutputDevice) -> bool {
        todo!("port RODFRouteCont::save")
    }

    /// C++: `std::vector<RODFRouteDesc> & RODFRouteCont::get()`
    /// Returns the container of stored routes
    // std::vector<RODFRouteDesc>& get() {
    //         return myRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn get(&mut self) -> &mut Vec<RODFRouteDesc> {
        todo!("port RODFRouteCont::get")
    }

    /// C++: `void RODFRouteCont::sortByDistance()`
    /// Sorts routes by their distance (length)
    // void
    // RODFRouteCont::sortByDistance() {
    //     sort(myRoutes.begin(), myRoutes.end(), by_distance_sorter());
    // }
    #[allow(unused_variables)]
    pub fn sort_by_distance(&mut self) {
        todo!("port RODFRouteCont::sortByDistance")
    }

    /// C++: `void RODFRouteCont::removeIllegal(const std::vector<ROEdgeVector> &)`
    /// Removes "illegal" routes
    // void
    // RODFRouteCont::removeIllegal(const std::vector<ROEdgeVector >& illegals) {
    //     for (std::vector<RODFRouteDesc>::iterator i = myRoutes.begin(); i != myRoutes.end();) {
    //         RODFRouteDesc& desc = *i;
    //         bool remove = false;
    //         for (std::vector<ROEdgeVector >::const_iterator j = illegals.begin(); !remove && j != illegals.end(); ++j) {
    //             int noFound = 0;
    //             for (ROEdgeVector::const_iterator k = (*j).begin(); !remove && k != (*j).end(); ++k) {
    //                 if (find(desc.edges2Pass.begin(), desc.edges2Pass.end(), *k) != desc.edges2Pass.end()) {
    //                     noFound++;
    //                     if (noFound > 1) {
    //                         remove = true;
    //                     }
    //                 }
    //             }
    //         }
    //         if (remove) {
    //             i = myRoutes.erase(i);
    //         } else {
    //             ++i;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_illegal(&mut self, illegals: &Vec<ROEdgeVector>) {
        todo!("port RODFRouteCont::removeIllegal")
    }

    /// C++: `void RODFRouteCont::setID(RODFRouteDesc &) const` [const, protected]
    /// Computes and sets the id of a route
    // void
    // RODFRouteCont::setID(RODFRouteDesc& desc) const {
    //     std::pair<ROEdge*, ROEdge*> c(desc.edges2Pass[0], desc.edges2Pass.back());
    //     desc.routename = c.first->getID() + "_to_" + c.second->getID();
    //     if (myConnectionOccurrences.find(c) == myConnectionOccurrences.end()) {
    //         myConnectionOccurrences[c] = 0;
    //     } else {
    //         myConnectionOccurrences[c] = myConnectionOccurrences[c] + 1;
    //         desc.routename = desc.routename + "_" + toString(myConnectionOccurrences[c]);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_id(&self, desc: &mut RODFRouteDesc) {
        todo!("port RODFRouteCont::setID")
    }

}

/// A class for sorting route descriptions by their length
/// C++ class `RODFRouteCont::by_distance_sorter` (dfrouter/RODFRouteCont.h)
pub struct RODFRouteCont_by_distance_sorter {
}

impl RODFRouteCont_by_distance_sorter {
    /// C++: `void by_distance_sorter::by_distance_sorter()`
    /// Constructor
    // explicit by_distance_sorter() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port by_distance_sorter::by_distance_sorter")
    }

    /// C++: `int by_distance_sorter::operator()(const RODFRouteDesc &, const RODFRouteDesc &)`
    /// Sorting function; compares RODFRouteDesc::distance2Last
    // int operator()(const RODFRouteDesc& p1, const RODFRouteDesc& p2) {
    //             return p1.distance2Last < p2.distance2Last;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, p1: &RODFRouteDesc, p2: &RODFRouteDesc) -> i32 {
        todo!("port by_distance_sorter::operator()")
    }

}

/// A class for finding a same route (one that passes the same edges)
/// C++ class `RODFRouteCont::route_finder` (dfrouter/RODFRouteCont.h)
pub struct RODFRouteCont_route_finder {
    /// The route description for which a same shall be found
    pub myDesc: Ptr<RODFRouteDesc>,
}

impl RODFRouteCont_route_finder {
    /// C++: `void route_finder::route_finder(const RODFRouteDesc &)`
    /// onstructor
    // explicit route_finder(const RODFRouteDesc& desc) : myDesc(desc) { }
    #[allow(unused_variables)]
    pub fn new(desc: &RODFRouteDesc) -> Self {
        todo!("port route_finder::route_finder")
    }

    /// C++: `bool route_finder::operator()(const RODFRouteDesc &)`
    /// The comparing function; compares passed edges
    // bool operator()(const RODFRouteDesc& desc) {
    //             return myDesc.edges2Pass == desc.edges2Pass;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, desc: &RODFRouteDesc) -> bool {
        todo!("port route_finder::operator()")
    }

}

