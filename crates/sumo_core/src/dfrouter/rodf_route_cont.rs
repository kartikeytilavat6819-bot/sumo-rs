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
        Self { myRoutes: Vec::new(), myConnectionOccurrences: BTreeMap::new() }
    }

    /// C++: `void RODFRouteCont::~RODFRouteCont()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for route in &mut self.myRoutes {
            // Drop each route
        }
        
        for (_, count) in &mut self.myConnectionOccurrences {
            // Drop each connection occurrence count
        }
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
        let mut finder = RODFRouteCont_route_finder::new(&*desc);
        match self.myRoutes.iter().position(|r| finder.call_op(r)) {
            None => {
                self.set_id(desc);
                // RODFRouteDesc has no Clone; copy field by field as the C++ push_back copy does
                let copy = RODFRouteDesc {
                    edges2Pass: desc.edges2Pass.clone(),
                    routename: desc.routename.clone(),
                    duration_2: desc.duration_2,
                    distance: desc.distance,
                    passedNo: desc.passedNo,
                    endDetectorEdge: desc.endDetectorEdge,
                    lastDetectorEdge: desc.lastDetectorEdge,
                    distance2Last: desc.distance2Last,
                    duration2Last: desc.duration2Last.clone(),
                    overallProb: desc.overallProb,
                    factor: desc.factor,
                };
                self.myRoutes.push(copy);
            }
            Some(idx) => {
                self.myRoutes[idx].overallProb += desc.overallProb;
            }
        }
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
        let mut finder = RODFRouteCont_route_finder::new(&*desc);
        if !self.myRoutes.iter().any(|r| finder.call_op(r)) {
            return false;
        }
        return true;
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
        let mut haveSavedOneAtLeast = false;
        for desc in self.myRoutes.iter() {
            if saved.contains(&desc.routename) {
                continue;
            }
            saved.push(desc.routename.clone());
            assert!(desc.edges2Pass.len() >= 1);
            out.open_tag_id(SUMO_TAG_ROUTE).write_attr(SUMO_ATTR_ID, format!("{}{}", prependix, desc.routename).as_str());
            out.write(" edges=\"");
            for (idx, k) in desc.edges2Pass.iter().enumerate() {
                if idx != 0 {
                    out.write(" ");
                }
                out.write(k.get_id());
            }
            out.write("\"");
            out.close_tag("");
            haveSavedOneAtLeast = true;
        }
        haveSavedOneAtLeast
    }

    /// C++: `std::vector<RODFRouteDesc> & RODFRouteCont::get()`
    /// Returns the container of stored routes
    // std::vector<RODFRouteDesc>& get() {
    //         return myRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn get(&mut self) -> &mut Vec<RODFRouteDesc> {
        &mut self.myRoutes
    }

    /// C++: `void RODFRouteCont::sortByDistance()`
    /// Sorts routes by their distance (length)
    // void
    // RODFRouteCont::sortByDistance() {
    //     sort(myRoutes.begin(), myRoutes.end(), by_distance_sorter());
    // }
    #[allow(unused_variables)]
    pub fn sort_by_distance(&mut self) {
        let mut sorter = RODFRouteCont_by_distance_sorter::new();
        // determinism: C++ std::sort is unstable and Rust sort_by is stable, so routes with equal distance2Last may end up in a different order
        self.myRoutes.sort_by(|a, b| {
            if sorter.call_op(a, b) != 0 {
                std::cmp::Ordering::Less
            } else if sorter.call_op(b, a) != 0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
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
        let mut i = 0;
        while i < self.myRoutes.len() {
            let desc = &self.myRoutes[i];
            let mut remove = false;
            for j in illegals.iter() {
                if remove {
                    break;
                }
                let mut noFound = 0;
                for k in j.iter() {
                    if remove {
                        break;
                    }
                    if desc.edges2Pass.contains(k) {
                        noFound += 1;
                        if noFound > 1 {
                            remove = true;
                        }
                    }
                }
            }
            if remove {
                self.myRoutes.remove(i);
            } else {
                i += 1;
            }
        }
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
    pub fn set_id(&mut self, desc: &mut RODFRouteDesc) {
        let c = (desc.edges2Pass[0], *desc.edges2Pass.last().unwrap());
        desc.routename = format!("{}_to_{}", c.0.get_id(), c.1.get_id());
        if !self.myConnectionOccurrences.contains_key(&c) {
            self.myConnectionOccurrences.insert(c, 0);
        } else {
            let occurrences = self.myConnectionOccurrences[&c] + 1;
            self.myConnectionOccurrences.insert(c, occurrences);
            desc.routename = format!("{}_{}", desc.routename, occurrences);
        }
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
        {
            Self {}
        }
    }

    /// C++: `int by_distance_sorter::operator()(const RODFRouteDesc &, const RODFRouteDesc &)`
    /// Sorting function; compares RODFRouteDesc::distance2Last
    // int operator()(const RODFRouteDesc& p1, const RODFRouteDesc& p2) {
    //             return p1.distance2Last < p2.distance2Last;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, p1: &RODFRouteDesc, p2: &RODFRouteDesc) -> i32 {
        (p1.distance2Last < p2.distance2Last) as i32
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
        Self { myDesc: Ptr::new(desc) }
    }

    /// C++: `bool route_finder::operator()(const RODFRouteDesc &)`
    /// The comparing function; compares passed edges
    // bool operator()(const RODFRouteDesc& desc) {
    //             return myDesc.edges2Pass == desc.edges2Pass;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, desc: &RODFRouteDesc) -> bool {
        self.myDesc.edges2Pass == desc.edges2Pass
    }

}

