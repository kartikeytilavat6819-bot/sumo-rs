//! Port of netbuild/NBEdgeCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::{INVALID_DOUBLE, SUMO_CONST_HALF_LANE_WIDTH, SUMO_CONST_LANE_WIDTH};
use utils::geom::POSITION_EPS;
use utils::xml::sumo_xml_definitions::SUMO_PARAM_ORIGID;
use utils::common::sumo_vehicle_class::SVC_UNSPECIFIED;
use super::nb_cont::KeepClear;

// C++ NBEdge::UNSPECIFIED_* defaults (NBEdge.cpp)
const UNSPECIFIED_SPEED: f64 = -1.;
const UNSPECIFIED_FRICTION: f64 = 1.;
const UNSPECIFIED_CONTPOS: f64 = -1.;
const UNSPECIFIED_VISIBILITY_DISTANCE: f64 = -1.;
// shortcut: C++ uses NBEdge::myDefaultConnectionLength (static, settable); the static is not ported yet, so the unspecified default -1 is used
const UNSPECIFIED_LOADED_LENGTH: f64 = -1.;

/// C++ `addLane2LaneConnection(from, dest, toLane, type, mayUseSameDestination)` with the header defaults filled in
fn add_lane2_lane_connection_validated(from: Ptr<NBEdge>, fromLane: i32, dest: Ptr<NBEdge>, toLane: i32, mayUseSameDestination: bool) -> bool {
    from.m().add_lane2_lane_connection(fromLane, dest, toLane, NBEdge_Lane2LaneInfoType::VALIDATED, mayUseSameDestination, false, KeepClear::KEEPCLEAR_UNSPECIFIED, UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_SPEED, UNSPECIFIED_FRICTION, UNSPECIFIED_LOADED_LENGTH, &PositionVector::EMPTY, false, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false)
}

/// C++ `addLane2LaneConnection(from, dest, toLane, COMPUTED)` with the header defaults filled in
fn add_lane2_lane_connection_computed(from: Ptr<NBEdge>, fromLane: i32, dest: Ptr<NBEdge>, toLane: i32) -> bool {
    from.m().add_lane2_lane_connection(fromLane, dest, toLane, NBEdge_Lane2LaneInfoType::COMPUTED, false, false, KeepClear::KEEPCLEAR_UNSPECIFIED, UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_SPEED, UNSPECIFIED_FRICTION, UNSPECIFIED_LOADED_LENGTH, &PositionVector::EMPTY, false, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false)
}

/// C++ typedef `std::map<std::string, NBEdge *> NBEdgeCont::EdgeCont`
pub type NBEdgeCont_EdgeCont = BTreeMap<String, Ptr<NBEdge>>;

/// C++ `delete edge`: runs the destructor body, then frees the object (edges are Box-allocated)
fn delete_edge(edge: Ptr<NBEdge>) {
    edge.m().drop_();
    drop(unsafe { Box::from_raw(edge.as_raw()) });
}

/// C++ class `NBEdgeCont::MinLaneComparatorIdLess` (netbuild/NBEdgeCont.h)
pub struct NBEdgeCont_MinLaneComparatorIdLess {
}

impl NBEdgeCont_MinLaneComparatorIdLess {
    /// C++: `bool MinLaneComparatorIdLess::operator()(const std::pair<NBEdge *, int> &, const std::pair<NBEdge *, int> &) const` [const]
    // bool
    // NBEdgeCont::MinLaneComparatorIdLess::operator()(const std::pair<NBEdge*, int>& a, const std::pair<NBEdge*, int>& b) const {
    //     if (a.first->getID() == b.first->getID()) {
    //         return a.second < b.second;
    //     }
    //     return a.first->getID() < b.first->getID();
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, a: &(Ptr<NBEdge>, i32), b: &(Ptr<NBEdge>, i32)) -> bool {
        if a.0.get_id() == b.0.get_id() {
            return a.1 < b.1;
        }
        a.0.get_id() < b.0.get_id()
    }

}

/// Storage for edges, including some functionality operating on multiple edges
/// C++ class `NBEdgeCont` (netbuild/NBEdgeCont.h)
pub struct NBEdgeCont {
    /// The network builder; used to obtain type information
    pub myTypeCont: Ptr<NBTypeCont>,
    /// The list of connections to recheck
    pub myConnections: BTreeMap<String, Vec<NBEdgeCont_PostProcessConnection>>,
    /// The instance of the dictionary (id->edge)
    pub myEdges: NBEdgeCont_EdgeCont,
    /// The extracted edges which are kept for reference
    pub myExtractedEdges: NBEdgeCont_EdgeCont,
    /// The edges which got extracted twice but may still be referenced somewhere TODO smart_ptr?
    pub myEdgeCemetery: BTreeSet<Ptr<NBEdge>>,
    /// The ids of ignored edges
    pub myIgnoredEdges: BTreeSet<String>,
    /// the number of splits of edges during the building
    pub myEdgesSplit: BTreeMap<Ptr<NBEdge>, (Ptr<NBEdge>, Ptr<NBEdge>)>, // C++: std::map<const NBEdge *, std::pair<NBEdge *, NBEdge *>>
    /// the edges that were created as result of splitting
    pub myWasSplit: BTreeSet<Ptr<NBEdge>>,
    /// The minimum speed an edge may have in order to be kept (default: -1)
    pub myEdgesMinSpeed: f64,
    /// Whether edges shall be joined and patched first, then removed
    pub myRemoveEdgesAfterLoading: bool,
    /// Set of ids of edges which shall explicitly be kept
    pub myEdges2Keep: BTreeSet<String>,
    /// Set of ids of edges which shall explicitly be removed
    pub myEdges2Remove: BTreeSet<String>,
    /// Set of vehicle types which must be allowed on edges in order to keep them
    pub myVehicleClasses2Keep: SVCPermissions,
    /// Set of vehicle types which need not be supported (edges which allow ONLY these are removed)
    pub myVehicleClasses2Remove: SVCPermissions,
    /// Set of edges types which shall be kept
    pub myTypes2Keep: BTreeSet<String>,
    /// Set of edges types which shall be removed
    pub myTypes2Remove: BTreeSet<String>,
    /// Boundary within which an edge must be located in order to be kept
    pub myPruningBoundary: PositionVector,
    /// whether a geo transform has been applied to the pruning boundary
    pub myNeedGeoTransformedPruningBoundary: bool,
    /// Edges marked as belonging to a roundabout by the user (each EdgeVector is a roundabout)
    pub myRoundabouts: BTreeSet<EdgeSet>,
    /// Edges marked as belonging to a roundabout after guessing
    pub myGuessedRoundabouts: BTreeSet<EdgeSet>,
}

impl NBEdgeCont {
    /// C++: `void NBEdgeCont::NBEdgeCont(NBTypeCont &)`
    /// Constructor
    // NBEdgeCont::NBEdgeCont(NBTypeCont& tc) :
    //     myTypeCont(tc),
    //     myVehicleClasses2Keep(0),
    //     myVehicleClasses2Remove(0),
    //     myNeedGeoTransformedPruningBoundary(false) {
    // }
    #[allow(unused_variables)]
    pub fn new(tc: &mut NBTypeCont) -> Self {
        NBEdgeCont {
            myTypeCont: Ptr::from_mut(tc),
            myConnections: BTreeMap::new(),
            myEdges: BTreeMap::new(),
            myExtractedEdges: BTreeMap::new(),
            myEdgeCemetery: BTreeSet::new(),
            myIgnoredEdges: BTreeSet::new(),
            myEdgesSplit: BTreeMap::new(),
            myWasSplit: BTreeSet::new(),
            // C++ leaves these two uninitialised; applyOptions always sets them
            myEdgesMinSpeed: -1.0,
            myRemoveEdgesAfterLoading: false,
            myEdges2Keep: BTreeSet::new(),
            myEdges2Remove: BTreeSet::new(),
            myVehicleClasses2Keep: 0,
            myVehicleClasses2Remove: 0,
            myTypes2Keep: BTreeSet::new(),
            myTypes2Remove: BTreeSet::new(),
            myPruningBoundary: utils::geom::position_vector::PositionVector::default(),
            myNeedGeoTransformedPruningBoundary: false,
            myRoundabouts: BTreeSet::new(),
            myGuessedRoundabouts: BTreeSet::new(),
        }
    }

    /// C++: `void NBEdgeCont::~NBEdgeCont()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ ~NBEdgeCont() { clear(); }
        self.clear();
    }

    /// C++: `void NBEdgeCont::applyOptions(OptionsCont &)`
    /// Initialises the storage by applying given options
    // void
    // NBEdgeCont::applyOptions(OptionsCont& oc) {
    //     // set edges dismiss/accept options
    //     myEdgesMinSpeed = oc.getFloat("keep-edges.min-speed");
    //     myRemoveEdgesAfterLoading = oc.exists("keep-edges.postload") && oc.getBool("keep-edges.postload");
    //     // we possibly have to load the edges to keep/remove
    //     if (oc.isSet("keep-edges.input-file")) {
    //         NBHelpers::loadEdgesFromFile(oc.getString("keep-edges.input-file"), myEdges2Keep);
    //     }
    //     if (oc.isSet("remove-edges.input-file")) {
    //         NBHelpers::loadEdgesFromFile(oc.getString("remove-edges.input-file"), myEdges2Remove);
    //     }
    //     if (oc.isSet("keep-edges.explicit")) {
    //         const std::vector<std::string> edges = oc.getStringVector("keep-edges.explicit");
    //         myEdges2Keep.insert(edges.begin(), edges.end());
    //     }
    //     if (oc.isSet("remove-edges.explicit")) {
    //         const std::vector<std::string> edges = oc.getStringVector("remove-edges.explicit");
    //         myEdges2Remove.insert(edges.begin(), edges.end());
    //     }
    //     if (oc.exists("keep-edges.by-vclass") && oc.isSet("keep-edges.by-vclass")) {
    //         myVehicleClasses2Keep = parseVehicleClasses(oc.getStringVector("keep-edges.by-vclass"));
    //     }
    //     if (oc.exists("remove-edges.by-vclass") && oc.isSet("remove-edges.by-vclass")) {
    //         myVehicleClasses2Remove = parseVehicleClasses(oc.getStringVector("remove-edges.by-vclass"));
    //     }
    //     if (oc.exists("keep-edges.by-type") && oc.isSet("keep-edges.by-type")) {
    //         const std::vector<std::string> types = oc.getStringVector("keep-edges.by-type");
    //         myTypes2Keep.insert(types.begin(), types.end());
    //     }
    //     if (oc.exists("remove-edges.by-type") && oc.isSet("remove-edges.by-type")) {
    //         const std::vector<std::string> types = oc.getStringVector("remove-edges.by-type");
    //         myTypes2Remove.insert(types.begin(), types.end());
    //     }
    //
    //     if (oc.isSet("keep-edges.in-boundary") || oc.isSet("keep-edges.in-geo-boundary")) {
    //
    //         std::string polyPlainString = oc.getValueString(oc.isSet("keep-edges.in-boundary") ?
    //                                       "keep-edges.in-boundary" : "keep-edges.in-geo-boundary");
    //         // try interpreting the boundary like shape attribute with spaces
    //         bool ok = true;
    //         PositionVector boundaryShape = GeomConvHelper::parseShapeReporting(polyPlainString, "pruning-boundary", 0, ok, false, false);
    //         if (ok) {
    //             if (boundaryShape.size() < 2) {
    //                 throw ProcessError(TL("Invalid boundary: need at least 2 coordinates"));
    //             } else if (boundaryShape.size() == 2) {
    //                 // prunning boundary (box)
    //                 myPruningBoundary.push_back(boundaryShape[0]);
    //                 myPruningBoundary.push_back(Position(boundaryShape[1].x(), boundaryShape[0].y()));
    //                 myPruningBoundary.push_back(boundaryShape[1]);
    //                 myPruningBoundary.push_back(Position(boundaryShape[0].x(), boundaryShape[1].y()));
    //             } else {
    //                 myPruningBoundary = boundaryShape;
    //             }
    //         } else {
    //             // maybe positions are separated by ',' instead of ' '
    //             std::vector<std::string> polyS = oc.getStringVector(oc.isSet("keep-edges.in-boundary") ?
    //                                              "keep-edges.in-boundary" : "keep-edges.in-geo-boundary");
    //             std::vector<double> poly;
    //             for (std::vector<std::string>::iterator i = polyS.begin(); i != polyS.end(); ++i) {
    //                 poly.push_back(StringUtils::toDouble((*i))); // !!! may throw something anyhow...
    //             }
    //             if (poly.size() < 4) {
    //                 throw ProcessError(TL("Invalid boundary: need at least 2 coordinates"));
    //             } else if (poly.size() % 2 != 0) {
    //                 throw ProcessError(TL("Invalid boundary: malformed coordinate"));
    //             } else if (poly.size() == 4) {
    //                 // prunning boundary (box)
    //                 myPruningBoundary.push_back(Position(poly[0], poly[1]));
    //                 myPruningBoundary.push_back(Position(poly[2], poly[1]));
    //                 myPruningBoundary.push_back(Position(poly[2], poly[3]));
    //                 myPruningBoundary.push_back(Position(poly[0], poly[3]));
    //             } else {
    //                 for (std::vector<double>::iterator j = poly.begin(); j != poly.end();) {
    //                     double x = *j++;
    //                     double y = *j++;
    //                     myPruningBoundary.push_back(Position(x, y));
    //                 }
    //             }
    //         }
    //         myNeedGeoTransformedPruningBoundary = oc.isSet("keep-edges.in-geo-boundary");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_options(&mut self, oc: &mut OptionsCont) {
        // set edges dismiss/accept options
        self.myEdgesMinSpeed = oc.get_float("keep-edges.min-speed").unwrap();
        self.myRemoveEdgesAfterLoading = oc.exists("keep-edges.postload") && oc.get_bool("keep-edges.postload").unwrap();
        // we possibly have to load the edges to keep/remove
        if oc.is_set("keep-edges.input-file", true).unwrap() {
            super::nb_helpers::NBHelpers::load_edges_from_file(&oc.get_string("keep-edges.input-file").unwrap(), &mut self.myEdges2Keep);
        }
        if oc.is_set("remove-edges.input-file", true).unwrap() {
            super::nb_helpers::NBHelpers::load_edges_from_file(&oc.get_string("remove-edges.input-file").unwrap(), &mut self.myEdges2Remove);
        }
        if oc.is_set("keep-edges.explicit", true).unwrap() {
            let edges = oc.get_string_vector("keep-edges.explicit").unwrap().clone();
            self.myEdges2Keep.extend(edges);
        }
        if oc.is_set("remove-edges.explicit", true).unwrap() {
            let edges = oc.get_string_vector("remove-edges.explicit").unwrap().clone();
            self.myEdges2Remove.extend(edges);
        }
        if oc.exists("keep-edges.by-vclass") && oc.is_set("keep-edges.by-vclass", true).unwrap() {
            self.myVehicleClasses2Keep = utils::common::sumo_vehicle_class::parse_vehicle_classes_vec(oc.get_string_vector("keep-edges.by-vclass").unwrap()).unwrap();
        }
        if oc.exists("remove-edges.by-vclass") && oc.is_set("remove-edges.by-vclass", true).unwrap() {
            self.myVehicleClasses2Remove = utils::common::sumo_vehicle_class::parse_vehicle_classes_vec(oc.get_string_vector("remove-edges.by-vclass").unwrap()).unwrap();
        }
        if oc.exists("keep-edges.by-type") && oc.is_set("keep-edges.by-type", true).unwrap() {
            let types = oc.get_string_vector("keep-edges.by-type").unwrap().clone();
            self.myTypes2Keep.extend(types);
        }
        if oc.exists("remove-edges.by-type") && oc.is_set("remove-edges.by-type", true).unwrap() {
            let types = oc.get_string_vector("remove-edges.by-type").unwrap().clone();
            self.myTypes2Remove.extend(types);
        }

        if oc.is_set("keep-edges.in-boundary", true).unwrap() || oc.is_set("keep-edges.in-geo-boundary", true).unwrap() {
            let use_in_boundary = oc.is_set("keep-edges.in-boundary", true).unwrap();
            let key = if use_in_boundary { "keep-edges.in-boundary" } else { "keep-edges.in-geo-boundary" };
            let poly_plain_string = oc.get_value_string(key).unwrap();
            // try interpreting the boundary like shape attribute with spaces
            let boundary_shape = utils::geom::geom_conv_helper::parse_shape_reporting(&poly_plain_string, "pruning-boundary", None, false, false);
            if let Some(boundary_shape) = boundary_shape {
                if boundary_shape.len() < 2 {
                    panic!("{}", utils::common::util_exceptions::ProcessError::new("Invalid boundary: need at least 2 coordinates"));
                } else if boundary_shape.len() == 2 {
                    // prunning boundary (box)
                    self.myPruningBoundary.push(boundary_shape[0]);
                    self.myPruningBoundary.push(utils::geom::position::Position::new(boundary_shape[1].x(), boundary_shape[0].y()));
                    self.myPruningBoundary.push(boundary_shape[1]);
                    self.myPruningBoundary.push(utils::geom::position::Position::new(boundary_shape[0].x(), boundary_shape[1].y()));
                } else {
                    self.myPruningBoundary = boundary_shape;
                }
            } else {
                // maybe positions are separated by ',' instead of ' '
                let poly_s = oc.get_string_vector(key).unwrap().clone();
                let mut poly: Vec<f64> = Vec::new();
                for i in poly_s.iter() {
                    poly.push(utils::common::string_utils::to_double(i).unwrap()); // !!! may throw something anyhow...
                }
                if poly.len() < 4 {
                    panic!("{}", utils::common::util_exceptions::ProcessError::new("Invalid boundary: need at least 2 coordinates"));
                } else if poly.len() % 2 != 0 {
                    panic!("{}", utils::common::util_exceptions::ProcessError::new("Invalid boundary: malformed coordinate"));
                } else if poly.len() == 4 {
                    // prunning boundary (box)
                    self.myPruningBoundary.push(utils::geom::position::Position::new(poly[0], poly[1]));
                    self.myPruningBoundary.push(utils::geom::position::Position::new(poly[2], poly[1]));
                    self.myPruningBoundary.push(utils::geom::position::Position::new(poly[2], poly[3]));
                    self.myPruningBoundary.push(utils::geom::position::Position::new(poly[0], poly[3]));
                } else {
                    let mut j = 0;
                    while j < poly.len() {
                        let x = poly[j];
                        let y = poly[j + 1];
                        j += 2;
                        self.myPruningBoundary.push(utils::geom::position::Position::new(x, y));
                    }
                }
            }
            self.myNeedGeoTransformedPruningBoundary = oc.is_set("keep-edges.in-geo-boundary", true).unwrap();
        }
    }

    /// C++: `void NBEdgeCont::clear()`
    /// Deletes all edges
    // void
    // NBEdgeCont::clear() {
    //     for (const auto& i : myEdges) {
    //         delete i.second;
    //     }
    //     myEdges.clear();
    //     for (const auto& i : myExtractedEdges) {
    //         delete i.second;
    //     }
    //     myExtractedEdges.clear();
    //     for (NBEdge* const e : myEdgeCemetery) {
    //         delete e;
    //     }
    //     myEdgeCemetery.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        for e in self.myEdges.values() {
            delete_edge(*e);
        }
        self.myEdges.clear();
        for e in self.myExtractedEdges.values() {
            delete_edge(*e);
        }
        self.myExtractedEdges.clear();
        for e in self.myEdgeCemetery.iter() {
            delete_edge(*e);
        }
        self.myEdgeCemetery.clear();
    }

    /// C++: `bool NBEdgeCont::insert(NBEdge *, bool)`
    /// Adds an edge to the dictionary
    // bool
    // NBEdgeCont::insert(NBEdge* edge, bool ignorePrunning) {
    //     if (myEdges.count(edge->getID()) != 0) {
    //         return false;
    //     }
    //     if (!ignorePrunning && ignoreFilterMatch(edge)) {
    //         edge->getFromNode()->removeEdge(edge);
    //         edge->getToNode()->removeEdge(edge);
    //         myIgnoredEdges.insert(edge->getID());
    //         delete edge;
    //     } else {
    //         OptionsCont& oc = OptionsCont::getOptions();
    //         if (oc.exists("dismiss-vclasses") && oc.getBool("dismiss-vclasses")) {
    //             edge->dismissVehicleClassInformation();
    //         }
    //         myEdges[edge->getID()] = edge;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert(&mut self, edge: Ptr<NBEdge>, ignorePrunning: bool) -> bool {
        if self.myEdges.contains_key(edge.get_id()) {
            return false;
        }
        if !ignorePrunning && self.ignore_filter_match(edge) {
            edge.get_from_node().m().remove_edge(edge, true);
            edge.get_to_node().m().remove_edge(edge, true);
            self.myIgnoredEdges.insert(edge.get_id().to_string());
            delete_edge(edge);
        } else {
            if utils::options::options_cont::exists("dismiss-vclasses") && utils::options::options_cont::get_bool("dismiss-vclasses") {
                edge.m().dismiss_vehicle_class_information();
            }
            self.myEdges.insert(edge.get_id().to_string(), edge);
        }
        true
    }

    /// C++: `NBEdge * NBEdgeCont::retrieve(const std::string &, bool) const` [const]
    /// Returns the edge that has the given id
    // NBEdge*
    // NBEdgeCont::retrieve(const std::string& id, bool retrieveExtracted) const {
    //     EdgeCont::const_iterator i = myEdges.find(id);
    //     if (i == myEdges.end()) {
    //         if (retrieveExtracted) {
    //             i = myExtractedEdges.find(id);
    //             if (i == myExtractedEdges.end()) {
    //                 return nullptr;
    //             }
    //         } else {
    //             return nullptr;
    //         }
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn retrieve(&self, id: &str, retrieveExtracted: bool) -> Ptr<NBEdge> {
        if let Some(e) = self.myEdges.get(id) {
            return *e;
        }
        if retrieveExtracted {
            if let Some(e) = self.myExtractedEdges.get(id) {
                return *e;
            }
        }
        Ptr::null()
    }

    /// C++: `NBEdge * NBEdgeCont::retrievePossiblySplit(const std::string &, bool) const` [const]
    /// Tries to retrieve an edge, even if it is splitted
    // NBEdge*
    // NBEdgeCont::retrievePossiblySplit(const std::string& id, bool downstream) const {
    //     NBEdge* edge = retrieve(id);
    //     if (edge != nullptr) {
    //         return edge;
    //     }
    //     // NOTE: (TODO) for multiply split edges (e.g. 15[0][0]) one could try recursion
    //     if ((retrieve(id + "[0]") != nullptr) && (retrieve(id + "[1]") != nullptr)) {
    //         // Edge was split during the netbuilding process
    //         if (downstream) {
    //             return retrieve(id + "[1]");
    //         } else {
    //             return retrieve(id + "[0]");
    //         }
    //     }
    //     return edge;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_possibly_split(&self, id: &str, downstream: bool) -> Ptr<NBEdge> {
        let edge = self.retrieve(id, false);
        if !edge.is_null() {
            return edge;
        }
        // NOTE: (TODO) for multiply split edges (e.g. 15[0][0]) one could try recursion
        if !self.retrieve(&format!("{id}[0]"), false).is_null() && !self.retrieve(&format!("{id}[1]"), false).is_null() {
            // Edge was split during the netbuilding process
            if downstream {
                return self.retrieve(&format!("{id}[1]"), false);
            } else {
                return self.retrieve(&format!("{id}[0]"), false);
            }
        }
        edge
    }

    /// C++: `NBEdge * NBEdgeCont::retrievePossiblySplit(const std::string &, const std::string &, bool) const` [const]
    /// Tries to retrieve an edge, even if it is splitted
    // NBEdge*
    // NBEdgeCont::retrievePossiblySplit(const std::string& id, const std::string& hint, bool incoming) const {
    //     // try to retrieve using the given name (iterative)
    //     NBEdge* edge = retrieve(id);
    //     if (edge != nullptr) {
    //         return edge;
    //     }
    //     // now, we did not find it; we have to look over all possibilities
    //     EdgeVector hints;
    //     // check whether at least the hint was not splitted
    //     NBEdge* hintedge = retrieve(hint);
    //     if (hintedge == nullptr) {
    //         hints = getGeneratedFrom(hint);
    //     } else {
    //         hints.push_back(hintedge);
    //     }
    //     EdgeVector candidates = getGeneratedFrom(id);
    //     for (const NBEdge* const currHint : hints) {
    //         for (NBEdge* const poss_searched : candidates) {
    //             const NBNode* const node = incoming ? poss_searched->myTo : poss_searched->myFrom;
    //             const EdgeVector& cont = incoming ? node->getOutgoingEdges() : node->getIncomingEdges();
    //             if (find(cont.begin(), cont.end(), currHint) != cont.end()) {
    //                 return poss_searched;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_possibly_split_2(&self, id: &str, hint: &str, incoming: bool) -> Ptr<NBEdge> {
        // try to retrieve using the given name (iterative)
        let edge = self.retrieve(id, false);
        if !edge.is_null() {
            return edge;
        }
        // now, we did not find it; we have to look over all possibilities
        // check whether at least the hint was not splitted
        let hintedge = self.retrieve(hint, false);
        let hints: EdgeVector = if hintedge.is_null() {
            self.get_generated_from(hint)
        } else {
            vec![hintedge]
        };
        let candidates = self.get_generated_from(id);
        for curr_hint in hints.iter() {
            for poss_searched in candidates.iter() {
                let node = if incoming { poss_searched.get_to_node() } else { poss_searched.get_from_node() };
                let cont = if incoming { node.get_outgoing_edges() } else { node.get_incoming_edges() };
                if cont.contains(curr_hint) {
                    return *poss_searched;
                }
            }
        }
        Ptr::null()
    }

    /// C++: `NBEdge * NBEdgeCont::retrievePossiblySplit(const std::string &, double) const` [const]
    /// Tries to retrieve an edge, even if it is splitted
    // NBEdge*
    // NBEdgeCont::retrievePossiblySplit(const std::string& id, double pos) const {
    //     // check whether the edge was not split, yet
    //     NBEdge* edge = retrieve(id);
    //     if (edge != nullptr) {
    //         return edge;
    //     }
    //     int maxLength = 0;
    //     std::string tid = id + "[";
    //     for (EdgeCont::const_iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         if ((*i).first.find(tid) == 0) {
    //             maxLength = MAX2(maxLength, (int)(*i).first.length());
    //         }
    //     }
    //     // find the part of the edge which matches the position
    //     double seen = 0;
    //     std::vector<std::string> names;
    //     names.push_back(id + "[1]");
    //     names.push_back(id + "[0]");
    //     while (names.size() > 0) {
    //         // retrieve the first subelement (to follow)
    //         std::string cid = names.back();
    //         names.pop_back();
    //         edge = retrieve(cid);
    //         // The edge was splitted; check its subparts within the
    //         //  next step
    //         if (edge == nullptr) {
    //             if ((int)cid.length() + 3 < maxLength) {
    //                 names.push_back(cid + "[1]");
    //                 names.push_back(cid + "[0]");
    //             }
    //         }
    //         // an edge with the name was found,
    //         //  check whether the position lies within it
    //         else {
    //             seen += edge->getLength();
    //             if (seen >= pos) {
    //                 return edge;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_possibly_split_3(&self, id: &str, pos: f64) -> Ptr<NBEdge> {
        // check whether the edge was not split, yet
        let edge = self.retrieve(id, false);
        if !edge.is_null() {
            return edge;
        }
        let mut max_length: i32 = 0;
        let tid = format!("{id}[");
        for key in self.myEdges.keys() {
            if key.starts_with(&tid) {
                max_length = std::cmp::max(max_length, key.len() as i32);
            }
        }
        // find the part of the edge which matches the position
        let mut seen: f64 = 0.0;
        let mut names: Vec<String> = Vec::new();
        names.push(format!("{id}[1]"));
        names.push(format!("{id}[0]"));
        while let Some(cid) = names.pop() {
            // retrieve the first subelement (to follow)
            let sub = self.retrieve(&cid, false);
            // The edge was splitted; check its subparts within the next step
            if sub.is_null() {
                if (cid.len() as i32) + 3 < max_length {
                    names.push(format!("{cid}[1]"));
                    names.push(format!("{cid}[0]"));
                }
            } else {
                // an edge with the name was found, check whether the position lies within it
                seen += sub.get_length();
                if seen >= pos {
                    return sub;
                }
            }
        }
        Ptr::null()
    }

    /// C++: `void NBEdgeCont::erase(NBDistrictCont &, NBEdge *)`
    /// Removes the given edge from the container (deleting it)
    // void
    // NBEdgeCont::erase(NBDistrictCont& dc, NBEdge* edge) {
    //     extract(dc, edge);
    //     delete edge;
    // }
    #[allow(unused_variables)]
    pub fn erase(&mut self, dc: &mut NBDistrictCont, edge: Ptr<NBEdge>) {
        self.extract(dc, edge, false);
        delete_edge(edge);
    }

    /// C++: `void NBEdgeCont::extract(NBDistrictCont &, NBEdge *, bool)`
    /// Removes the given edge from the container like erase but does not delete it
    // void
    // NBEdgeCont::extract(NBDistrictCont& dc, NBEdge* edge, bool remember) {
    //     if (remember) {
    //         const auto& prevExtracted = myExtractedEdges.find(edge->getID());
    //         if (prevExtracted != myExtractedEdges.end()) {
    //             if (edge != prevExtracted->second) {
    //                 myEdgeCemetery.insert(prevExtracted->second);
    //                 prevExtracted->second = edge;
    //             }
    //         } else {
    //             myExtractedEdges[edge->getID()] = edge;
    //         }
    //     }
    //     myEdges.erase(edge->getID());
    //     edge->myFrom->removeEdge(edge);
    //     edge->myTo->removeEdge(edge);
    //     dc.removeFromSinksAndSources(edge);
    // }
    #[allow(unused_variables)]
    pub fn extract(&mut self, dc: &mut NBDistrictCont, edge: Ptr<NBEdge>, remember: bool) {
        if remember {
            let id = edge.get_id().to_string();
            if let Some(prev_extracted) = self.myExtractedEdges.get(&id).copied() {
                if edge != prev_extracted {
                    self.myEdgeCemetery.insert(prev_extracted);
                    self.myExtractedEdges.insert(id, edge);
                }
            } else {
                self.myExtractedEdges.insert(id, edge);
            }
        }
        self.myEdges.remove(edge.get_id());
        edge.get_from_node().m().remove_edge(edge, true);
        edge.get_to_node().m().remove_edge(edge, true);
        dc.remove_from_sinks_and_sources(edge);
    }

    /// C++: `std::map<std::string, NBEdge *>::const_iterator NBEdgeCont::begin() const` [const]
    /// Returns the pointer to the begin of the stored edges
    // std::map<std::string, NBEdge*>::const_iterator begin() const {
    //         return myEdges.begin();
    //     }
    #[allow(unused_variables)]
    pub fn begin(&self) -> std::collections::btree_map::Iter<'_, String, Ptr<NBEdge>> {
        self.myEdges.iter()
    }

    /// C++: `std::map<std::string, NBEdge *>::const_iterator NBEdgeCont::end() const` [const]
    /// Returns the pointer to the end of the stored edges
    // std::map<std::string, NBEdge*>::const_iterator end() const {
    //         return myEdges.end();
    //     }
    #[allow(unused_variables)]
    pub fn end(&self) -> std::collections::btree_map::Iter<'_, String, Ptr<NBEdge>> {
        // shortcut: C++ iterator pair cannot be compared in Rust, end() is an exhausted iterator; callers iterate begin() instead, upgrade if a caller needs the C++ iterator protocol
        self.myEdges.iter().skip(self.myEdges.len())
    }

    /// C++: `void NBEdgeCont::processSplits(NBEdge *, std::vector<Split>, NBNodeCont &, NBDistrictCont &, NBTrafficLightLogicCont &)`
    /// process splits
    // void
    // NBEdgeCont::processSplits(NBEdge* e, std::vector<Split> splits,
    //                           NBNodeCont& nc, NBDistrictCont& dc, NBTrafficLightLogicCont& tlc) {
    //     if (splits.empty()) {
    //         return;
    //     }
    //     const std::string origID = e->getID();
    //     sort(splits.begin(), splits.end(), split_sorter());
    //     int maxNumLanes = e->getNumLanes();
    //     // compute the node positions and sort the lanes
    //     for (Split& split : splits) {
    //         sort(split.lanes.begin(), split.lanes.end());
    //         maxNumLanes = MAX2(maxNumLanes, (int)split.lanes.size());
    //     }
    //     // split the edge
    //     std::vector<int> currLanes;
    //     for (int l = 0; l < e->getNumLanes(); ++l) {
    //         currLanes.push_back(l);
    //     }
    //     if (e->getNumLanes() != (int)splits.back().lanes.size()) {
    //         // invalidate traffic light definitions loaded from a SUMO network
    //         e->getToNode()->invalidateTLS(tlc, true, true);
    //         // if the number of lanes changes the connections should be
    //         // recomputed
    //         e->invalidateConnections(true);
    //     }
    //
    //     std::string firstID = "";
    //     double seen = 0;
    //     for (const Split& exp : splits) {
    //         assert(exp.lanes.size() != 0);
    //         if (exp.pos > 0 && e->getLoadedLength() + seen > exp.pos && exp.pos > seen) {
    //             nc.insert(exp.node);
    //             nc.markAsSplit(exp.node);
    //             //  split the edge
    //             const std::string idBefore = exp.idBefore == "" ? e->getID() : exp.idBefore;
    //             const std::string idAfter = exp.idAfter == "" ? exp.nameID : exp.idAfter;
    //             if (firstID == "") {
    //                 firstID = idBefore;
    //             }
    //             const bool ok = splitAt(dc, e, exp.pos - seen, exp.node,
    //                                     idBefore, idAfter, e->getNumLanes(), (int) exp.lanes.size(), exp.speed);
    //             if (!ok) {
    //                 WRITE_WARNINGF(TL("Error on parsing a split (edge '%')."), origID);
    //                 return;
    //             }
    //             seen = exp.pos;
    //             std::vector<int> newLanes = exp.lanes;
    //             NBEdge* pe = retrieve(idBefore);
    //             NBEdge* ne = retrieve(idAfter);
    //             // reconnect lanes
    //             pe->invalidateConnections(true);
    //             //  new on right
    //             int rightMostP = currLanes[0];
    //             int rightMostN = newLanes[0];
    //             for (int l = 0; l < (int) rightMostP - (int) rightMostN; ++l) {
    //                 pe->addLane2LaneConnection(0, ne, l, NBEdge::Lane2LaneInfoType::VALIDATED, true);
    //             }
    //             //  new on left
    //             int leftMostP = currLanes.back();
    //             int leftMostN = newLanes.back();
    //             for (int l = 0; l < (int) leftMostN - (int) leftMostP; ++l) {
    //                 pe->addLane2LaneConnection(pe->getNumLanes() - 1, ne, leftMostN - l - rightMostN, NBEdge::Lane2LaneInfoType::VALIDATED, true);
    //             }
    //             //  all other connected
    //             for (int l = 0; l < maxNumLanes; ++l) {
    //                 if (find(currLanes.begin(), currLanes.end(), l) == currLanes.end()) {
    //                     continue;
    //                 }
    //                 if (find(newLanes.begin(), newLanes.end(), l) == newLanes.end()) {
    //                     continue;
    //                 }
    //                 pe->addLane2LaneConnection(l - rightMostP, ne, l - rightMostN, NBEdge::Lane2LaneInfoType::VALIDATED, true);
    //             }
    //             //  if there are edges at this node which are not connected
    //             //  we can assume that this split was attached to an
    //             //  existing node. Reset all connections to let the default
    //             //  algorithm recompute them
    //             if (exp.node->getIncomingEdges().size() > 1 || exp.node->getOutgoingEdges().size() > 1 || exp.node->getType() == SumoXMLNodeType::ZIPPER) {
    //                 for (NBEdge* in : exp.node->getIncomingEdges()) {
    //                     in->invalidateConnections(true);
    //                 }
    //             }
    //             // move to next
    //             e = ne;
    //             currLanes = newLanes;
    //         }  else if (exp.pos == 0) {
    //             const int laneCountDiff = e->getNumLanes() - (int)exp.lanes.size();
    //             if (laneCountDiff < 0) {
    //                 e->incLaneNo(-laneCountDiff);
    //             } else {
    //                 e->decLaneNo(laneCountDiff);
    //             }
    //             currLanes = exp.lanes;
    //             // invalidate traffic light definition loaded from a SUMO network
    //             // XXX it would be preferable to reconstruct the phase definitions heuristically
    //             e->getFromNode()->invalidateTLS(tlc, true, true);
    //             if (exp.speed != -1.) {
    //                 e->setSpeed(-1, exp.speed);
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Split at '%' lies beyond the edge's length (edge '%')."), toString(exp.pos), origID);
    //         }
    //     }
    //     // patch lane offsets
    //     e = retrieve(firstID);
    //     if (e != nullptr) {
    //         if (splits.front().pos != 0) {
    //             // add a dummy split at the beginning to ensure correct offset
    //             Split start;
    //             start.pos = 0;
    //             for (int lane = 0; lane < (int)e->getNumLanes(); ++lane) {
    //                 start.lanes.push_back(lane);
    //             }
    //             start.offset = splits.front().offset;
    //             start.offsetFactor = splits.front().offsetFactor;
    //             splits.insert(splits.begin(), start);
    //         }
    //         for (const Split& split : splits) {
    //             int maxLeft = split.lanes.back();
    //             double offset = split.offset;
    //             if (maxLeft < maxNumLanes) {
    //                 if (e->getLaneSpreadFunction() == LaneSpreadFunction::RIGHT) {
    //                     offset += split.offsetFactor * SUMO_const_laneWidth * (maxNumLanes - 1 - maxLeft);
    //                 } else {
    //                     offset += split.offsetFactor * SUMO_const_halfLaneWidth * (maxNumLanes - 1 - maxLeft);
    //                 }
    //             }
    //             int maxRight = split.lanes.front();
    //             if (maxRight > 0 && e->getLaneSpreadFunction() == LaneSpreadFunction::CENTER) {
    //                 offset -= split.offsetFactor * SUMO_const_halfLaneWidth * maxRight;
    //             }
    //             //std::cout << " processSplits " << origID << " splitOffset=" << (*i).offset << " offset=" << offset << "\n";
    //             if (offset != 0) {
    //                 PositionVector g = e->getGeometry();
    //                 g.move2side(offset);
    //                 e->setGeometry(g);
    //             }
    //             if (e->getToNode()->getOutgoingEdges().size() != 0) {
    //                 e = e->getToNode()->getOutgoingEdges()[0];
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process_splits(&mut self, mut e: Ptr<NBEdge>, mut splits: Vec<NBEdgeCont_Split>, nc: &mut NBNodeCont, dc: &mut NBDistrictCont, tlc: &mut NBTrafficLightLogicCont) {
        if splits.is_empty() {
            return;
        }
        let origID = e.get_id().to_string();
        // determinism: C++ std::sort is unstable (introsort); Rust sort_by is stable, equal pos values may be ordered differently
        splits.sort_by(|a, b| a.pos.partial_cmp(&b.pos).unwrap_or(std::cmp::Ordering::Equal));
        let mut maxNumLanes = e.get_num_lanes();
        // compute the node positions and sort the lanes
        for split in splits.iter_mut() {
            split.lanes.sort();
            maxNumLanes = maxNumLanes.max(split.lanes.len() as i32);
        }
        // split the edge
        let mut currLanes: Vec<i32> = Vec::new();
        for l in 0..e.get_num_lanes() {
            currLanes.push(l);
        }
        if e.get_num_lanes() != splits.last().unwrap().lanes.len() as i32 {
            // invalidate traffic light definitions loaded from a SUMO network
            e.get_to_node().m().invalidate_tls(tlc, true, true);
            // if the number of lanes changes the connections should be
            // recomputed
            e.m().invalidate_connections(true);
        }

        let mut firstID = String::new();
        let mut seen: f64 = 0.0;
        for exp in splits.iter() {
            assert!(!exp.lanes.is_empty());
            if exp.pos > 0.0 && e.get_loaded_length() + seen > exp.pos && exp.pos > seen {
                nc.insert_2(exp.node);
                nc.mark_as_split(exp.node);
                //  split the edge
                let idBefore = if exp.idBefore == "" { e.get_id().to_string() } else { exp.idBefore.clone() };
                let idAfter = if exp.idAfter == "" { exp.nameID.clone() } else { exp.idAfter.clone() };
                if firstID == "" {
                    firstID = idBefore.clone();
                }
                let ok = self.split_at_3(dc, e, exp.pos - seen, exp.node, &idBefore, &idAfter, e.get_num_lanes(), exp.lanes.len() as i32, exp.speed, 1.0, 0);
                if !ok {
                    utils::common::msg_handler::write_warning(&format!("Error on parsing a split (edge '{}').", origID));
                    return;
                }
                seen = exp.pos;
                let newLanes = exp.lanes.clone();
                let pe = self.retrieve(&idBefore, false);
                let ne = self.retrieve(&idAfter, false);
                // reconnect lanes
                pe.m().invalidate_connections(true);
                //  new on right
                let rightMostP = currLanes[0];
                let rightMostN = newLanes[0];
                for l in 0..(rightMostP - rightMostN) {
                    add_lane2_lane_connection_validated(pe, 0, ne, l, true);
                }
                //  new on left
                let leftMostP = *currLanes.last().unwrap();
                let leftMostN = *newLanes.last().unwrap();
                for l in 0..(leftMostN - leftMostP) {
                    add_lane2_lane_connection_validated(pe, pe.get_num_lanes() - 1, ne, leftMostN - l - rightMostN, true);
                }
                //  all other connected
                for l in 0..maxNumLanes {
                    if !currLanes.contains(&l) {
                        continue;
                    }
                    if !newLanes.contains(&l) {
                        continue;
                    }
                    add_lane2_lane_connection_validated(pe, l - rightMostP, ne, l - rightMostN, true);
                }
                //  if there are edges at this node which are not connected
                //  we can assume that this split was attached to an
                //  existing node. Reset all connections to let the default
                //  algorithm recompute them
                if exp.node.get_incoming_edges().len() > 1 || exp.node.get_outgoing_edges().len() > 1 || exp.node.get_type() == SumoXMLNodeType::ZIPPER {
                    for inc in exp.node.get_incoming_edges().clone() {
                        inc.m().invalidate_connections(true);
                    }
                }
                // move to next
                e = ne;
                currLanes = newLanes;
            } else if exp.pos == 0.0 {
                let laneCountDiff = e.get_num_lanes() - exp.lanes.len() as i32;
                if laneCountDiff < 0 {
                    e.m().inc_lane_no(-laneCountDiff);
                } else {
                    e.m().dec_lane_no(laneCountDiff);
                }
                currLanes = exp.lanes.clone();
                // invalidate traffic light definition loaded from a SUMO network
                // XXX it would be preferable to reconstruct the phase definitions heuristically
                e.get_from_node().m().invalidate_tls(tlc, true, true);
                if exp.speed != -1.0 {
                    e.m().set_speed(-1, exp.speed);
                }
            } else {
                utils::common::msg_handler::write_warning(&format!("Split at '{}' lies beyond the edge's length (edge '{}').", utils::common::to_string::to_string(&exp.pos), origID));
            }
        }
        // patch lane offsets
        e = self.retrieve(&firstID, false);
        if !e.is_null() {
            if splits.first().unwrap().pos != 0.0 {
                // add a dummy split at the beginning to ensure correct offset
                let mut start = NBEdgeCont_Split {
                    lanes: Vec::new(),
                    pos: 0.0,
                    speed: INVALID_DOUBLE,
                    friction: INVALID_DOUBLE,
                    node: Ptr::null(),
                    idBefore: String::new(),
                    idAfter: String::new(),
                    nameID: String::new(),
                    offset: splits[0].offset,
                    offsetFactor: splits[0].offsetFactor,
                };
                for lane in 0..e.get_num_lanes() {
                    start.lanes.push(lane);
                }
                splits.insert(0, start);
            }
            for split in splits.iter() {
                let maxLeft = *split.lanes.last().unwrap();
                let mut offset = split.offset;
                if maxLeft < maxNumLanes {
                    if e.get_lane_spread_function() == LaneSpreadFunction::RIGHT {
                        offset += split.offsetFactor as f64 * SUMO_CONST_LANE_WIDTH * (maxNumLanes - 1 - maxLeft) as f64;
                    } else {
                        offset += split.offsetFactor as f64 * SUMO_CONST_HALF_LANE_WIDTH * (maxNumLanes - 1 - maxLeft) as f64;
                    }
                }
                let maxRight = *split.lanes.first().unwrap();
                if maxRight > 0 && e.get_lane_spread_function() == LaneSpreadFunction::CENTER {
                    offset -= split.offsetFactor as f64 * SUMO_CONST_HALF_LANE_WIDTH * maxRight as f64;
                }
                //std::cout << " processSplits " << origID << " splitOffset=" << (*i).offset << " offset=" << offset << "\n";
                if offset != 0.0 {
                    let mut g = e.get_geometry().clone();
                    g.move2side(offset, 100.0);
                    e.m().set_geometry(&g, false);
                }
                if e.get_to_node().get_outgoing_edges().len() != 0 {
                    e = e.get_to_node().get_outgoing_edges()[0];
                }
            }
        }
    }

    /// C++: `bool NBEdgeCont::splitAt(NBDistrictCont &, NBEdge *, NBNode *)`
    /// Splits the edge at the position nearest to the given node
    // bool
    // NBEdgeCont::splitAt(NBDistrictCont& dc, NBEdge* edge, NBNode* node) {
    //     return splitAt(dc, edge, node, edge->getID() + "[0]", edge->getID() + "[1]",
    //                    (int) edge->myLanes.size(), (int) edge->myLanes.size());
    // }
    #[allow(unused_variables)]
    pub fn split_at(&mut self, dc: &mut NBDistrictCont, edge: Ptr<NBEdge>, node: Ptr<NBNode>) -> bool {
        let n = edge.get_num_lanes();
        let first = format!("{}[0]", edge.get_id());
        let second = format!("{}[1]", edge.get_id());
        self.split_at_2(dc, edge, node, &first, &second, n, n, -1.0, UNSPECIFIED_FRICTION, 0)
    }

    /// C++: `bool NBEdgeCont::splitAt(NBDistrictCont &, NBEdge *, NBNode *, const std::string &, const std::string &, int, int, const double, const double, const int)`
    /// Splits the edge at the position nearest to the given node using the given modifications
    // bool
    // NBEdgeCont::splitAt(NBDistrictCont& dc, NBEdge* edge, NBNode* node,
    //                     const std::string& firstEdgeName,
    //                     const std::string& secondEdgeName,
    //                     int noLanesFirstEdge, int noLanesSecondEdge,
    //                     const double speed, const double friction,
    //                     const int changedLeft) {
    //     double pos;
    //     pos = edge->getGeometry().nearest_offset_to_point2D(node->getPosition());
    //     if (pos <= 0) {
    //         pos = GeomHelper::nearest_offset_on_line_to_point2D(
    //                   edge->myFrom->getPosition(), edge->myTo->getPosition(),
    //                   node->getPosition());
    //     }
    //     if (pos <= 0 || pos + POSITION_EPS > edge->getGeometry().length()) {
    //         return false;
    //     }
    //     return splitAt(dc, edge, pos, node, firstEdgeName, secondEdgeName,
    //                    noLanesFirstEdge, noLanesSecondEdge, speed, friction, changedLeft);
    // }
    #[allow(unused_variables)]
    pub fn split_at_2(&mut self, dc: &mut NBDistrictCont, edge: Ptr<NBEdge>, node: Ptr<NBNode>, firstEdgeName: &str, secondEdgeName: &str, noLanesFirstEdge: i32, noLanesSecondEdge: i32, speed: f64, friction: f64, changedLeft: i32) -> bool {
        let mut pos = edge.get_geometry().nearest_offset_to_point2d(node.get_position(), true);
        if pos <= 0.0 {
            pos = utils::geom::geom_helper::nearest_offset_on_line_to_point2d(edge.myFrom.get_position(), edge.myTo.get_position(), node.get_position(), true);
        }
        if pos <= 0.0 || pos + POSITION_EPS > edge.get_geometry().length() {
            return false;
        }
        self.split_at_3(dc, edge, pos, node, firstEdgeName, secondEdgeName, noLanesFirstEdge, noLanesSecondEdge, speed, friction, changedLeft)
    }

    /// C++: `bool NBEdgeCont::splitAt(NBDistrictCont &, NBEdge *, double, NBNode *, const std::string &, const std::string &, int, int, const double, const double, const int)`
    /// Splits the edge at the position nearest to the given node using the given modifications
    // bool
    // NBEdgeCont::splitAt(NBDistrictCont& dc,
    //                     NBEdge* edge, double pos, NBNode* node,
    //                     const std::string& firstEdgeName,
    //                     const std::string& secondEdgeName,
    //                     int noLanesFirstEdge, int noLanesSecondEdge,
    //                     const double speed, const double friction,
    //                     const int changedLeft) {
    //     if (firstEdgeName != edge->getID() && myEdges.count(firstEdgeName) != 0) {
    //         WRITE_ERRORF(TL("Could not insert edge '%' before split of edge '%'."), firstEdgeName, edge->getID());
    //         return false;
    //     }
    //     if (secondEdgeName == firstEdgeName || (secondEdgeName != edge->getID() && myEdges.count(secondEdgeName) != 0)) {
    //         WRITE_ERRORF(TL("Could not insert edge '%' after split of edge '%'."), secondEdgeName, edge->getID());
    //         return false;
    //     }
    //     // there must be at least some overlap between first and second edge
    //     assert(changedLeft > -((int)noLanesFirstEdge));
    //     assert(changedLeft < (int)noLanesSecondEdge);
    //
    //     // build the new edges' geometries
    //     double geomPos = pos;
    //     if (edge->hasLoadedLength()) {
    //         geomPos = edge->getGeometry().nearest_offset_to_point2D(node->getPosition());
    //         if (geomPos <= 0) {
    //             geomPos = pos * edge->getGeometry().length() / edge->getLoadedLength();
    //         }
    //     }
    //     std::pair<PositionVector, PositionVector> geoms = edge->getGeometry().splitAt(geomPos);
    //     // reduce inaccuracies and preserve bidi
    //     if (geoms.first[-1].almostSame(node->getPosition()) || edge->isBidi()) {
    //         geoms.first[-1] = node->getPosition();
    //         geoms.second[0] = node->getPosition();
    //     }
    //     // build and insert the edges
    //     NBEdge* one = new NBEdge(firstEdgeName, edge->myFrom, node, edge, geoms.first, noLanesFirstEdge);
    //     NBEdge* two = new NBEdge(secondEdgeName, node, edge->myTo, edge, geoms.second, noLanesSecondEdge);
    //     if (OptionsCont::getOptions().getBool("output.original-names")) {
    //         const std::string origID = edge->getLaneStruct(0).getParameter(SUMO_PARAM_ORIGID, edge->getID());
    //         if (firstEdgeName != origID) {
    //             one->setOrigID(origID, false);
    //         }
    //         if (secondEdgeName != origID) {
    //             two->setOrigID(origID, false);
    //         }
    //     }
    //     two->copyConnectionsFrom(edge);
    //     if (speed != -1.) {
    //         two->setSpeed(-1, speed);
    //     }
    //     if (friction != -1.) {
    //         two->setFriction(-1, friction);
    //     }
    //     if (edge->getDistance() != 0) {
    //         one->setDistance(edge->getDistance());
    //         two->setDistance(one->getDistance() + pos);
    //     }
    //     if (edge->hasLoadedLength()) {
    //         one->setLoadedLength(pos);
    //         two->setLoadedLength(edge->getLoadedLength() - pos);
    //     }
    //     // replace information about this edge within the nodes
    //     edge->myFrom->replaceOutgoing(edge, one, 0);
    //     edge->myTo->replaceIncoming(edge, two, 0);
    //     // patch tls
    //     for (NBTrafficLightDefinition* const tld : edge->myFrom->getControllingTLS()) {
    //         tld->replaceRemoved(edge, -1, one, -1, false);
    //     }
    //     for (NBTrafficLightDefinition* const tld : edge->myTo->getControllingTLS()) {
    //         tld->replaceRemoved(edge, -1, two, -1, true);
    //     }
    //     // the edge is now occuring twice in both nodes...
    //     //  clean up
    //     edge->myFrom->removeDoubleEdges();
    //     edge->myTo->removeDoubleEdges();
    //     // add connections from the first to the second edge
    //     // there will be as many connections as there are lanes on the second edge
    //     // by default lanes will be added / discontinued on the right side
    //     // (appropriate for highway on-/off-ramps)
    //     const int offset = (int)one->getNumLanes() - (int)two->getNumLanes() + changedLeft;
    //     for (int i2 = 0; i2 < (int)two->getNumLanes(); i2++) {
    //         const int i1 = MIN2(MAX2((int)0, i2 + offset), (int)one->getNumLanes());
    //         if (!one->addLane2LaneConnection(i1, two, i2, NBEdge::Lane2LaneInfoType::COMPUTED)) {
    //             throw ProcessError(TL("Could not set connection!"));
    //         }
    //     }
    //     if (myRemoveEdgesAfterLoading) {
    //         if (myEdges2Keep.count(edge->getID()) != 0) {
    //             myEdges2Keep.insert(one->getID());
    //             myEdges2Keep.insert(two->getID());
    //         }
    //         if (myEdges2Remove.count(edge->getID()) != 0) {
    //             myEdges2Remove.insert(one->getID());
    //             myEdges2Remove.insert(two->getID());
    //         }
    //     }
    //     // erase the splitted edge
    //     patchRoundabouts(edge, one, two, myRoundabouts);
    //     patchRoundabouts(edge, one, two, myGuessedRoundabouts);
    //     const std::string oldID = edge->getID();
    //     extract(dc, edge, true);
    //     insert(one, true);  // duplicate id check happened earlier
    //     insert(two, true);  // duplicate id check happened earlier
    //     myEdgesSplit[edge] = {one, two};
    //     myWasSplit.insert(one);
    //     myWasSplit.insert(two);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn split_at_3(&mut self, dc: &mut NBDistrictCont, edge: Ptr<NBEdge>, edgepos: f64, node: Ptr<NBNode>, firstEdgeName: &str, secondEdgeName: &str, noLanesFirstEdge: i32, noLanesSecondEdge: i32, speed: f64, friction: f64, changedLeft: i32) -> bool {
        if firstEdgeName != edge.get_id() && self.myEdges.contains_key(firstEdgeName) {
            utils::common::msg_handler::write_error(&format!("Could not insert edge '{}' before split of edge '{}'.", firstEdgeName, edge.get_id()));
            return false;
        }
        if secondEdgeName == firstEdgeName || (secondEdgeName != edge.get_id() && self.myEdges.contains_key(secondEdgeName)) {
            utils::common::msg_handler::write_error(&format!("Could not insert edge '{}' after split of edge '{}'.", secondEdgeName, edge.get_id()));
            return false;
        }
        // there must be at least some overlap between first and second edge
        assert!(changedLeft > -noLanesFirstEdge);
        assert!(changedLeft < noLanesSecondEdge);

        // build the new edges' geometries
        let mut geomPos = edgepos;
        if edge.has_loaded_length() {
            geomPos = edge.get_geometry().nearest_offset_to_point2d(node.get_position(), true);
            if geomPos <= 0.0 {
                geomPos = edgepos * edge.get_geometry().length() / edge.get_loaded_length();
            }
        }
        let (mut geoms_first, mut geoms_second) = edge.get_geometry().split_at(geomPos, false).expect("PositionVector::splitAt");
        // reduce inaccuracies and preserve bidi
        if geoms_first.back().almost_same(node.get_position(), POSITION_EPS) || edge.is_bidi() {
            *geoms_first.back_mut() = node.get_position().clone();
            *geoms_second.at_mut(0) = node.get_position().clone();
        }
        // build and insert the edges
        let one = Ptr::from_raw(Box::into_raw(Box::new(NBEdge::new_3(firstEdgeName, edge.myFrom, node, edge, &geoms_first, noLanesFirstEdge))));
        let two = Ptr::from_raw(Box::into_raw(Box::new(NBEdge::new_3(secondEdgeName, node, edge.myTo, edge, &geoms_second, noLanesSecondEdge))));
        if utils::options::options_cont::get_bool("output.original-names") {
            let origID = edge.get_lane_struct_2(0).base.get_parameter(SUMO_PARAM_ORIGID, edge.get_id());
            if firstEdgeName != origID {
                one.m().set_orig_id(origID.clone(), false, -1);
            }
            if secondEdgeName != origID {
                two.m().set_orig_id(origID.clone(), false, -1);
            }
        }
        two.m().copy_connections_from(edge);
        if speed != -1. {
            two.m().set_speed(-1, speed);
        }
        if friction != -1. {
            two.m().set_friction(-1, friction);
        }
        if edge.get_distance() != 0.0 {
            one.m().set_distance(edge.get_distance());
            two.m().set_distance(one.get_distance() + edgepos);
        }
        if edge.has_loaded_length() {
            one.m().set_loaded_length(edgepos);
            two.m().set_loaded_length(edge.get_loaded_length() - edgepos);
        }
        // replace information about this edge within the nodes
        edge.myFrom.m().replace_outgoing(edge, one, 0);
        edge.myTo.m().replace_incoming(edge, two, 0);
        // patch tls
        for tld in edge.myFrom.get_controlling_tls().clone() {
            tld.m().replace_removed(edge, -1, one, -1, false);
        }
        for tld in edge.myTo.get_controlling_tls().clone() {
            tld.m().replace_removed(edge, -1, two, -1, true);
        }
        // the edge is now occuring twice in both nodes...
        //  clean up
        edge.myFrom.m().remove_double_edges();
        edge.myTo.m().remove_double_edges();
        // add connections from the first to the second edge
        // there will be as many connections as there are lanes on the second edge
        // by default lanes will be added / discontinued on the right side
        // (appropriate for highway on-/off-ramps)
        let offset = one.get_num_lanes() - two.get_num_lanes() + changedLeft;
        for i2 in 0..two.get_num_lanes() {
            let i1 = (i2 + offset).max(0).min(one.get_num_lanes());
            if !add_lane2_lane_connection_computed(one, i1, two, i2) {
                panic!("{}", utils::common::util_exceptions::ProcessError::new("Could not set connection!"));
            }
        }
        if self.myRemoveEdgesAfterLoading {
            if self.myEdges2Keep.contains(edge.get_id()) {
                self.myEdges2Keep.insert(one.get_id().to_string());
                self.myEdges2Keep.insert(two.get_id().to_string());
            }
            if self.myEdges2Remove.contains(edge.get_id()) {
                self.myEdges2Remove.insert(one.get_id().to_string());
                self.myEdges2Remove.insert(two.get_id().to_string());
            }
        }
        // erase the splitted edge
        // patchRoundabouts does not use member state, so the sets are taken out to avoid a double &mut borrow of self
        let mut roundabouts = std::mem::take(&mut self.myRoundabouts);
        self.patch_roundabouts(edge, one, two, &mut roundabouts);
        self.myRoundabouts = roundabouts;
        let mut guessedRoundabouts = std::mem::take(&mut self.myGuessedRoundabouts);
        self.patch_roundabouts(edge, one, two, &mut guessedRoundabouts);
        self.myGuessedRoundabouts = guessedRoundabouts;
        let oldID = edge.get_id().to_string();
        self.extract(dc, edge, true);
        self.insert(one, true); // duplicate id check happened earlier
        self.insert(two, true); // duplicate id check happened earlier
        self.myEdgesSplit.insert(edge, (one, two));
        self.myWasSplit.insert(one);
        self.myWasSplit.insert(two);
        true
    }

    /// C++: `std::string NBEdgeCont::createUnusedID(const std::string &, const std::string &)`
    /// if base is an existing edge id, find a unused id of the form base + sep + INT
    // std::string
    // NBEdgeCont::createUnusedID(const std::string& base, const std::string& sep) {
    //     std::string result = base;
    //     int i = 0;
    //     while (myEdges.find(result) != myEdges.end()) {
    //         result = base + sep + toString(i++);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn create_unused_id(&mut self, base: &str, sep: &str) -> String {
        let mut result = base.to_string();
        let mut i: i32 = 0;
        while self.myEdges.contains_key(&result) {
            result = format!("{}{}{}", base, sep, i);
            i += 1;
        }
        result
    }

    /// C++: `int NBEdgeCont::size() const` [const]
    /// Returns the number of edges
    // int size() const {
    //         return (int) myEdges.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myEdges.len() as i32
    }

    /// C++: `std::vector<std::string> NBEdgeCont::getAllNames() const` [const]
    /// Returns all ids of known edges
    // std::vector<std::string>
    // NBEdgeCont::getAllNames() const {
    //     std::vector<std::string> ret;
    //     for (EdgeCont::const_iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         ret.push_back((*i).first);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_all_names(&self) -> Vec<String> {
        self.myEdges.keys().cloned().collect()
    }

    /// C++: `const std::pair<NBEdge *, NBEdge *> * NBEdgeCont::getSplit(const NBEdge *const) const` [const]
    /// Returns the edge split if the edge has been split, nullptr otherwise
    // const std::pair<NBEdge*, NBEdge*>* getSplit(const NBEdge* const origEdge) const {
    //         const auto& split = myEdgesSplit.find(origEdge);
    //         if (split == myEdgesSplit.end()) {
    //             return nullptr;
    //         }
    //         return &split->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_split(&self, origEdge: Ptr<NBEdge>) -> Ptr<(Ptr<NBEdge>, Ptr<NBEdge>)> {
        // pointer into the map value, like C++ `&split->second`
        match self.myEdgesSplit.get(&origEdge) {
            Some(split) => Ptr::new(split),
            None => Ptr::null(),
        }
    }

    /// C++: `NBEdge * NBEdgeCont::getSplitBase(const std::string &) const` [const]
    // NBEdge*
    // NBEdgeCont::getSplitBase(const std::string& edgeID) const {
    //     NBEdge* longest = nullptr;
    //     for (auto item : myEdgesSplit) {
    //         if (item.first->getID() == edgeID) {
    //             if (longest == nullptr || longest->getLoadedLength() < item.first->getLoadedLength()) {
    //                 longest = const_cast<NBEdge*>(item.first);
    //             }
    //         }
    //     }
    //     return longest;
    // }
    #[allow(unused_variables)]
    pub fn get_split_base(&self, edgeID: &str) -> Ptr<NBEdge> {
        let mut longest: Ptr<NBEdge> = Ptr::null();
        for item in self.myEdgesSplit.keys() {
            if item.get_id() == edgeID {
                if longest.is_null() || longest.get_loaded_length() < item.get_loaded_length() {
                    longest = *item;
                }
            }
        }
        longest
    }

    /// C++: `int NBEdgeCont::getNumEdgeSplits() const` [const]
    /// Returns the number of edge splits
    // int getNumEdgeSplits() const {
    //         return (int)myEdgesSplit.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_edge_splits(&self) -> i32 {
        self.myEdgesSplit.len() as i32
    }

    /// C++: `int NBEdgeCont::removeUnwishedEdges(NBDistrictCont &)`
    /// Removes unwished edges (not in keep-edges)
    // int
    // NBEdgeCont::removeUnwishedEdges(NBDistrictCont& dc) {
    //     EdgeVector toRemove;
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         NBEdge* edge = (*i).second;
    //         if (!myEdges2Keep.count(edge->getID())) {
    //             edge->getFromNode()->removeEdge(edge);
    //             edge->getToNode()->removeEdge(edge);
    //             toRemove.push_back(edge);
    //         }
    //     }
    //     for (EdgeVector::iterator j = toRemove.begin(); j != toRemove.end(); ++j) {
    //         erase(dc, *j);
    //     }
    //     return (int)toRemove.size();
    // }
    #[allow(unused_variables)]
    pub fn remove_unwished_edges(&mut self, dc: &mut NBDistrictCont) -> i32 {
        let mut to_remove: EdgeVector = Vec::new();
        for item in self.myEdges.values() {
            let edge = *item;
            if !self.myEdges2Keep.contains(edge.get_id()) {
                edge.get_from_node().m().remove_edge(edge, true);
                edge.get_to_node().m().remove_edge(edge, true);
                to_remove.push(edge);
            }
        }
        for j in to_remove.iter() {
            self.erase(dc, *j);
        }
        to_remove.len() as i32
    }

    /// C++: `void NBEdgeCont::splitGeometry(NBDistrictCont &, NBNodeCont &)`
    /// Splits edges into multiple if they have a complex geometry
    // void
    // NBEdgeCont::splitGeometry(NBDistrictCont& dc, NBNodeCont& nc) {
    //     // make a copy of myEdges because splitting will modify it
    //     EdgeCont edges = myEdges;
    //     for (auto& item : edges) {
    //         NBEdge* edge = item.second;
    //         if (edge->getGeometry().size() < 3) {
    //             continue;
    //         }
    //         PositionVector geom = edge->getGeometry();
    //         const std::string id = edge->getID();
    //         double offset = 0;
    //         for (int i = 1; i < (int)geom.size() - 1; i++) {
    //             offset += geom[i - 1].distanceTo(geom[i]);
    //             std::string nodeID = id + "." + toString((int)offset);
    //             if (!nc.insert(nodeID, geom[i])) {
    //                 WRITE_WARNING("Could not split geometry of edge '" + id + "' at index " + toString(i));
    //                 continue;
    //             }
    //             NBNode* node = nc.retrieve(nodeID);
    //             splitAt(dc, edge, node, edge->getID(), nodeID, edge->getNumLanes(), edge->getNumLanes());
    //             edge = retrieve(nodeID);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn split_geometry(&mut self, dc: &mut NBDistrictCont, nc: &mut NBNodeCont) {
        todo!("port NBEdgeCont::splitGeometry")
    }

    /// C++: `void NBEdgeCont::reduceGeometries(const double)`
    // void
    // NBEdgeCont::reduceGeometries(const double minDist) {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         (*i).second->reduceGeometry(minDist);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reduce_geometries(&mut self, minDist: f64) {
        todo!("port NBEdgeCont::reduceGeometries")
    }

    /// C++: `void NBEdgeCont::checkGeometries(const double, bool, const double, bool, bool, bool)`
    // void
    // NBEdgeCont::checkGeometries(const double maxAngle, bool fixAngle, const double minRadius, bool fix, bool fixRailways, bool silent) {
    //     if (maxAngle > 0 || minRadius > 0) {
    //         for (auto& item : myEdges) {
    //             if ((item.second->getPermissions() & (SVC_PUBLIC_CLASSES | SVC_PASSENGER)) == 0) {
    //                 continue;
    //             }
    //             item.second->checkGeometry(maxAngle, fixAngle, minRadius, fix || (fixRailways && isRailway(item.second->getPermissions())), silent);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_geometries(&mut self, maxAngle: f64, fixAngle: bool, minRadius: f64, fix: bool, fixRailways: bool, silent: bool) {
        todo!("port NBEdgeCont::checkGeometries")
    }

    /// C++: `void NBEdgeCont::sortOutgoingLanesConnections()`
    /// Sorts all lanes of all edges within the container by their direction
    // void
    // NBEdgeCont::sortOutgoingLanesConnections() {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         (*i).second->sortOutgoingConnectionsByAngle();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sort_outgoing_lanes_connections(&mut self) {
        todo!("port NBEdgeCont::sortOutgoingLanesConnections")
    }

    /// C++: `void NBEdgeCont::computeEdge2Edges(bool)`
    /// Computes for each edge the approached edges
    // void
    // NBEdgeCont::computeEdge2Edges(bool noLeftMovers) {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         (*i).second->computeEdge2Edges(noLeftMovers);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_edge2_edges(&mut self, noLeftMovers: bool) {
        todo!("port NBEdgeCont::computeEdge2Edges")
    }

    /// C++: `void NBEdgeCont::computeLanes2Edges()`
    /// Computes for each edge which lanes approach the next edges
    // void
    // NBEdgeCont::computeLanes2Edges() {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         (*i).second->computeLanes2Edges();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_lanes2_edges(&mut self) {
        todo!("port NBEdgeCont::computeLanes2Edges")
    }

    /// C++: `void NBEdgeCont::recheckLanes()`
    /// Rechecks whether all lanes have a successor for each of the stored edges
    // void
    // NBEdgeCont::recheckLanes() {
    //     const bool fixOppositeLengths = OptionsCont::getOptions().getBool("opposites.guess.fix-lengths");
    //     for (const auto& edgeIt : myEdges) {
    //         NBEdge* const edge = edgeIt.second;
    //         edge->recheckLanes();
    //         edge->recheckOpposite(*this, fixOppositeLengths);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_lanes(&mut self) {
        todo!("port NBEdgeCont::recheckLanes")
    }

    /// C++: `void NBEdgeCont::appendTurnarounds(bool, bool, bool, bool, bool)`
    /// Appends turnarounds to all edges stored in the container
    // void
    // NBEdgeCont::appendTurnarounds(bool noTLSControlled, bool noFringe, bool onlyDeadends, bool onlyTurnlane, bool noGeometryLike) {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         (*i).second->appendTurnaround(noTLSControlled, noFringe, onlyDeadends, onlyTurnlane, noGeometryLike, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn append_turnarounds(&mut self, noTLSControlled: bool, noFringe: bool, onlyDeadends: bool, onlyTurnlane: bool, noGeometryLike: bool) {
        todo!("port NBEdgeCont::appendTurnarounds")
    }

    /// C++: `void NBEdgeCont::appendTurnarounds(const std::set<std::string> &, bool)`
    /// Appends turnarounds to all edges stored in the container Calls "NBEdge::appendTurnaround" for edges with the given ids
    // void
    // NBEdgeCont::appendTurnarounds(const std::set<std::string>& ids, bool noTLSControlled) {
    //     for (std::set<std::string>::const_iterator it = ids.begin(); it != ids.end(); it++) {
    //         myEdges[*it]->appendTurnaround(noTLSControlled, false, false, false, false, false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn append_turnarounds_2(&mut self, ids: &BTreeSet<String>, noTLSControlled: bool) {
        todo!("port NBEdgeCont::appendTurnarounds")
    }

    /// C++: `void NBEdgeCont::appendRailwayTurnarounds(const NBPTStopCont &)`
    /// Appends turnarounds to all bidiRail edges with stops
    // void
    // NBEdgeCont::appendRailwayTurnarounds(const NBPTStopCont& sc) {
    //     std::set<std::string> stopEdgeIDs;
    //     for (auto& stopItem : sc.getStops()) {
    //         stopEdgeIDs.insert(stopItem.second->getEdgeId());
    //     }
    //     for (auto& item : myEdges) {
    //         NBEdge* edge = item.second;
    //         if (edge->isBidiRail()
    //                 && (stopEdgeIDs.count(item.first) > 0 ||
    //                     stopEdgeIDs.count(edge->getTurnDestination(true)->getID()) > 0)) {
    //             NBEdge* to = edge->getTurnDestination(true);
    //             assert(to != 0);
    //             edge->setConnection(edge->getNumLanes() - 1,
    //                                 to, to->getNumLanes() - 1, NBEdge::Lane2LaneInfoType::VALIDATED, false, false,
    //                                 KEEPCLEAR_UNSPECIFIED,
    //                                 NBEdge::UNSPECIFIED_CONTPOS, NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE,
    //                                 SUMO_const_haltingSpeed);
    //             // finalize connections to prevent removal during recheck
    //             edge->declareConnectionsAsLoaded();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn append_railway_turnarounds(&mut self, sc: &NBPTStopCont) {
        todo!("port NBEdgeCont::appendRailwayTurnarounds")
    }

    /// C++: `void NBEdgeCont::computeEdgeShapes(double)`
    /// Computes the shapes of all edges stored in the container
    // void
    // NBEdgeCont::computeEdgeShapes(double smoothElevationThreshold) {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         (*i).second->computeEdgeShape(smoothElevationThreshold);
    //     }
    //     // equalize length of opposite edges
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         NBEdge* edge = i->second;
    //         const std::string& oppositeID = edge->getLanes().back().oppositeID;
    //         if (oppositeID != "" && oppositeID != "-") {
    //             NBEdge* oppEdge = retrieve(oppositeID.substr(0, oppositeID.rfind("_")));
    //             if (oppEdge == nullptr || oppEdge->getLaneID(oppEdge->getNumLanes() - 1) != oppositeID) {
    //                 continue;
    //             }
    //             if (fabs(oppEdge->getLength() - edge->getLength()) > NUMERICAL_EPS) {
    //                 double avgLength = (oppEdge->getLength() + edge->getLength()) / 2;
    //                 edge->setAverageLengthWithOpposite(avgLength);
    //                 oppEdge->setAverageLengthWithOpposite(avgLength);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_edge_shapes(&mut self, smoothElevationThreshold: f64) {
        todo!("port NBEdgeCont::computeEdgeShapes")
    }

    /// C++: `void NBEdgeCont::computeLaneShapes()`
    /// Computes the shapes of all lanes of all edges stored in the container
    // void
    // NBEdgeCont::computeLaneShapes() {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         (*i).second->computeLaneShapes();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_lane_shapes(&mut self) {
        todo!("port NBEdgeCont::computeLaneShapes")
    }

    /// C++: `void NBEdgeCont::clearControllingTLInformation() const` [const]
    /// Clears information about controlling traffic lights for all connenections of all edges
    // void
    // NBEdgeCont::clearControllingTLInformation() const {
    //     for (EdgeCont::const_iterator i = myEdges.begin(); i != myEdges.end(); i++) {
    //         (*i).second->clearControllingTLInformation();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_controlling_tl_information(&self) {
        todo!("port NBEdgeCont::clearControllingTLInformation")
    }

    /// C++: `void NBEdgeCont::joinSameNodeConnectingEdges(NBDistrictCont &, NBTrafficLightLogicCont &, EdgeVector)`
    /// Joins the given edges because they connect the same nodes
    // void
    // NBEdgeCont::joinSameNodeConnectingEdges(NBDistrictCont& dc,
    //                                         NBTrafficLightLogicCont& tlc,
    //                                         EdgeVector edges) {
    //     // !!! Attention!
    //     //  No merging of the geometry to come is being done
    //     //  The connections are moved from one edge to another within
    //     //   the replacement where the edge is a node's incoming edge.
    //
    //     // count the number of lanes, the speed and the id
    //     int nolanes = 0;
    //     double speed = 0;
    //     int priority = -1;
    //     bool joinEdges = true;
    //     std::string id;
    //     sort(edges.begin(), edges.end(), NBContHelper::same_connection_edge_sorter());
    //     // retrieve the connected nodes
    //     NBEdge* tpledge = *(edges.begin());
    //     NBNode* from = tpledge->getFromNode();
    //     NBNode* to = tpledge->getToNode();
    //     EdgeVector::const_iterator i;
    //     int myPriority = (*edges.begin())->getPriority();
    //     for (i = edges.begin(); i != edges.end(); i++) {
    //         // some assertions
    //         assert((*i)->getFromNode() == from);
    //         assert((*i)->getToNode() == to);
    //         // ad the number of lanes the current edge has
    //         nolanes += (*i)->getNumLanes();
    //         // build the id
    //         if (i != edges.begin()) {
    //             id += "+";
    //         }
    //         id += (*i)->getID();
    //         // compute the speed
    //         speed += (*i)->getSpeed();
    //         // build the priority
    //         // merged edges should have the same inherited priority
    //         if (myPriority == (*i)->getPriority()) {
    //             priority = myPriority;
    //         } else {
    //             priority = -1;
    //             joinEdges = false;
    //         }
    //     }
    //     if (joinEdges) {
    //         speed /= (double)edges.size();
    //         // build the new edge
    //         NBEdge* newEdge = new NBEdge(id, from, to, "", speed, NBEdge::UNSPECIFIED_FRICTION, nolanes, priority,
    //                                      NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET,
    //                                      tpledge->myLaneSpreadFunction, tpledge->getStreetName());
    //         // copy lane attributes
    //         int laneIndex = 0;
    //         for (i = edges.begin(); i != edges.end(); ++i) {
    //             const std::vector<NBEdge::Lane>& lanes = (*i)->getLanes();
    //             for (int j = 0; j < (int)lanes.size(); ++j) {
    //                 newEdge->setPermissions(lanes[j].permissions, laneIndex);
    //                 newEdge->setLaneWidth(laneIndex, lanes[j].width);
    //                 newEdge->setEndOffset(laneIndex, lanes[j].endOffset);
    //                 laneIndex++;
    //             }
    //         }
    //         insert(newEdge, true);
    //         // replace old edge by current within the nodes
    //         //  and delete the old
    //         from->replaceOutgoing(edges, newEdge);
    //         to->replaceIncoming(edges, newEdge);
    //         // patch connections
    //         //  add edge2edge-information
    //         for (i = edges.begin(); i != edges.end(); i++) {
    //             EdgeVector ev = (*i)->getConnectedEdges();
    //             for (EdgeVector::iterator j = ev.begin(); j != ev.end(); j++) {
    //                 newEdge->addEdge2EdgeConnection(*j);
    //             }
    //         }
    //         //  copy outgoing connections to the new edge
    //         int currLane = 0;
    //         for (i = edges.begin(); i != edges.end(); i++) {
    //             newEdge->moveOutgoingConnectionsFrom(*i, currLane);
    //             currLane += (*i)->getNumLanes();
    //         }
    //         // patch tl-information
    //         currLane = 0;
    //         for (i = edges.begin(); i != edges.end(); i++) {
    //             int noLanes = (*i)->getNumLanes();
    //             for (int j = 0; j < noLanes; j++, currLane++) {
    //                 // replace in traffic lights
    //                 tlc.replaceRemoved(*i, j, newEdge, currLane, true);
    //                 tlc.replaceRemoved(*i, j, newEdge, currLane, false);
    //             }
    //         }
    //         // delete joined edges
    //         for (i = edges.begin(); i != edges.end(); i++) {
    //             extract(dc, *i, true);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn join_same_node_connecting_edges(&mut self, dc: &mut NBDistrictCont, tlc: &mut NBTrafficLightLogicCont, edges: EdgeVector) {
        todo!("port NBEdgeCont::joinSameNodeConnectingEdges")
    }

    /// C++: `void NBEdgeCont::guessOpposites()`
    /// Sets opposite lane information for geometrically close edges
    // void
    // NBEdgeCont::guessOpposites() {
    //     //@todo magic values
    //     const bool fixOppositeLengths = OptionsCont::getOptions().getBool("opposites.guess.fix-lengths");
    //     // ensure consistency of loaded values before starting to guess
    //     for (const auto& edgeIt : myEdges) {
    //         NBEdge* const edge = edgeIt.second;
    //         edge->recheckOpposite(*this, fixOppositeLengths);
    //     }
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         NBEdge* edge = i->second;
    //         edge->guessOpposite();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn guess_opposites(&mut self) {
        todo!("port NBEdgeCont::guessOpposites")
    }

    /// C++: `void NBEdgeCont::recheckLaneSpread()`
    /// Rechecks whether the lane spread is proper
    // void
    // NBEdgeCont::recheckLaneSpread() {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         NBEdge* opposite = getOppositeByID(i->first);
    //         if (opposite != nullptr) {
    //             i->second->setLaneSpreadFunction(LaneSpreadFunction::RIGHT);
    //             opposite->setLaneSpreadFunction(LaneSpreadFunction::RIGHT);
    //         } else {
    //             i->second->setLaneSpreadFunction(LaneSpreadFunction::CENTER);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_lane_spread(&mut self) {
        todo!("port NBEdgeCont::recheckLaneSpread")
    }

    /// C++: `NBEdge * NBEdgeCont::getOppositeByID(const std::string &) const` [const]
    /// Returns the edge with negated id if it exists
    // NBEdge*
    // NBEdgeCont::getOppositeByID(const std::string& edgeID) const {
    //     const std::string oppositeID = edgeID[0] == '-' ? edgeID.substr(1) :  "-" + edgeID;
    //     EdgeCont::const_iterator it = myEdges.find(oppositeID);
    //     return it != myEdges.end() ? it->second : (NBEdge*)nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_by_id(&self, edgeID: &str) -> Ptr<NBEdge> {
        todo!("port NBEdgeCont::getOppositeByID")
    }

    /// C++: `NBEdge * NBEdgeCont::getByID(const std::string &) const` [const]
    /// Returns the edge with id if it exists
    // NBEdge*
    // NBEdgeCont::getByID(const std::string& edgeID) const {
    //     EdgeCont::const_iterator it = myEdges.find(edgeID);
    //     return it != myEdges.end() ? it->second : (NBEdge*)nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_by_id(&self, edgeID: &str) -> Ptr<NBEdge> {
        todo!("port NBEdgeCont::getByID")
    }

    /// C++: `int NBEdgeCont::guessRoundabouts()`
    /// Determines which edges belong to roundabouts and increases their priority
    // int
    // NBEdgeCont::guessRoundabouts() {
    //     myGuessedRoundabouts.clear();
    //     std::set<NBEdge*> loadedRoundaboutEdges;
    //     for (std::set<EdgeSet>::const_iterator it = myRoundabouts.begin(); it != myRoundabouts.end(); ++it) {
    //         loadedRoundaboutEdges.insert(it->begin(), it->end());
    //     }
    //     // step 1: keep only those edges which have no turnarounds and which are not
    //     // part of a loaded roundabout
    //     std::set<NBEdge*> candidates;
    //     SVCPermissions valid = SVCAll & ~SVC_PEDESTRIAN;
    //     for (EdgeCont::const_iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         NBEdge* e = (*i).second;
    //         NBNode* const to = e->getToNode();
    //         if (e->getTurnDestination() == nullptr
    //                 && to->getConnectionTo(e->getFromNode()) == nullptr
    //                 && (e->getPermissions() & valid) != 0) {
    //             candidates.insert(e);
    //         }
    //     }
    //
    //     // step 2:
    //     std::set<NBEdge*> visited;
    //     for (std::set<NBEdge*>::const_iterator i = candidates.begin(); i != candidates.end(); ++i) {
    //         EdgeVector loopEdges;
    //         // start with a random edge (this doesn't have to be a roundabout edge)
    //         // loop over connected edges (using always the leftmost one)
    //         // and keep the list in loopEdges
    //         // continue until we loop back onto a loopEdges and extract the loop
    //         NBEdge* e = (*i);
    //         if (visited.count(e) > 0) {
    //             // already seen
    //             continue;
    //         }
    //         loopEdges.push_back(e);
    //         bool doLoop = true;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //         gDebugFlag1 = e->getID() == DEBUG_EDGE_ID;
    // #endif
    //         do {
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //             if (gDebugFlag1) {
    //                 std::cout << " e=" << e->getID() << " loopEdges=" << toString(loopEdges) << "\n";
    //                 gDebugFlag1 = true;
    //             }
    // #endif
    //             visited.insert(e);
    //             const EdgeVector& edges = e->getToNode()->getEdges();
    //             if ((e->getToNode()->getType() == SumoXMLNodeType::RIGHT_BEFORE_LEFT || e->getToNode()->getType() == SumoXMLNodeType::LEFT_BEFORE_RIGHT)
    //                     && !e->getToNode()->typeWasGuessed()) {
    //                 doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                 if (gDebugFlag1) {
    //                     std::cout << " rbl\n";
    //                 }
    //                 gDebugFlag1 = false;
    // #endif
    //                 break;
    //             }
    //             if (e->getToNode()->getRoundaboutType() == RoundaboutType::NO) {
    //                 doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                 if (gDebugFlag1) {
    //                     std::cout << " disabled\n";
    //                 }
    //                 gDebugFlag1 = false;
    // #endif
    //                 break;
    //             }
    //             if (edges.size() < 2) {
    //                 doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                 if (gDebugFlag1) {
    //                     std::cout << " deadend\n";
    //                 }
    //                 gDebugFlag1 = false;
    // #endif
    //                 break;
    //             }
    //             if (e->getTurnDestination(true) != nullptr || e->getToNode()->getConnectionTo(e->getFromNode()) != nullptr) {
    //                 // do not follow turn-arounds while in a (tentative) loop
    //                 doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                 if (gDebugFlag1) {
    //                     std::cout << " invalid turnAround e=" << e->getID() << " dest=" << Named::getIDSecure(e->getTurnDestination()) << "\n";
    //                 }
    //                 gDebugFlag1 = false;
    // #endif
    //                 break;
    //             }
    //             EdgeVector::const_iterator me = std::find(edges.begin(), edges.end(), e);
    //             NBContHelper::nextCW(edges, me);
    //             NBEdge* left = *me;
    //             while ((left->getPermissions() & valid) == 0 && left != e) {
    //                 NBContHelper::nextCW(edges, me);
    //                 left = *me;
    //             }
    //             if (left == e) {
    //                 // no usable continuation edge found
    //                 doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                 if (gDebugFlag1) {
    //                     std::cout << " noContinuation\n";
    //                 }
    //                 gDebugFlag1 = false;
    // #endif
    //                 break;
    //             }
    //             NBContHelper::nextCW(edges, me);
    //             NBEdge* nextLeft = *me;
    //             double angle = fabs(NBHelpers::relAngle(e->getAngleAtNode(e->getToNode()), left->getAngleAtNode(e->getToNode())));
    //             double nextAngle = nextLeft == e ? 180 : fabs(NBHelpers::relAngle(e->getAngleAtNode(e->getToNode()), nextLeft->getAngleAtNode(e->getToNode())));
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //             if (gDebugFlag1) {
    //                 std::cout << "   e=" << e->getID() << " left=" << left->getID() << " nextLeft=" << nextLeft->getID() << " angle=" << angle << " nextAngle=" << nextAngle << " eLength=" << e->getLength() << " lLength=" << left->getLength() << " dist=" << e->getLaneShape(0).back().distanceTo2D(left->getLaneShape(0).front()) << "\n";
    //             }
    // #endif
    //             // there should be no straigher edge further left
    //             if (angle >= 90 && nextAngle < 45) {
    //                 doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                 if (gDebugFlag1) {
    //                     std::cout << "     failed nextAngle=" << nextAngle << "\n";
    //                 }
    //                 gDebugFlag1 = false;
    // #endif
    //                 break;
    //             }
    //             // roundabouts do not have sharp turns (or they wouldn't be called 'round')
    //             // however, if the roundabout is very small then most of the roundness may be in the junction so the angle may be as high as 180 (for smooth attachments at a joined junction)
    //             if (angle >= 90) {
    //                 double edgeAngle = fabs(NBHelpers::relAngle(e->getStartAngle(), e->getEndAngle()));
    //                 double edgeAngle2 = fabs(NBHelpers::relAngle(left->getStartAngle(), left->getEndAngle()));
    //                 double edgeRadius = e->getGeometry().length2D() / DEG2RAD(edgeAngle);
    //                 double edgeRadius2 = left->getGeometry().length2D() / DEG2RAD(edgeAngle2);
    //                 const double avgRadius = 0.5 * (edgeRadius + edgeRadius2);
    //                 double junctionRadius = e->getLaneShape(0).back().distanceTo2D(left->getLaneShape(0).front()) / DEG2RAD(angle);
    //                 //std::cout << "     junction=" << e->getToNode()->getID() << " e=" << e->getID() << " left=" << left->getID() << " angle=" << angle << " eRadius=" << edgeRadius << " eRadius2=" << edgeRadius2 << " jRadius3=" << junctionRadius << "\n";
    //                 if (junctionRadius < 0.8 * avgRadius) {
    //                     doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                     if (gDebugFlag1) {
    //                         std::cout << "     failed angle=" << angle << " eRadius=" << edgeRadius << " eRadius2=" << edgeRadius2 << " jRadius3=" << junctionRadius << "\n";
    //                     }
    //                     gDebugFlag1 = false;
    // #endif
    //                     break;
    //                 }
    //             }
    //
    //             EdgeVector::const_iterator loopClosed = std::find(loopEdges.begin(), loopEdges.end(), left);
    //             const int loopSize = (int)(loopEdges.end() - loopClosed);
    //             if (loopSize > 0) {
    //                 // loop found
    //                 if (loopSize < 3) {
    //                     doLoop = false; // need at least 3 edges for a roundabout
    //                 } else if (loopSize < (int)loopEdges.size()) {
    //                     // remove initial edges not belonging to the loop
    //                     EdgeVector(loopEdges.begin() + (loopEdges.size() - loopSize), loopEdges.end()).swap(loopEdges);
    //                 }
    //                 // count attachments to the outside. need at least 3 or a roundabout doesn't make much sense
    //                 int attachments = 0;
    //                 for (EdgeVector::const_iterator j = loopEdges.begin(); j != loopEdges.end(); ++j) {
    //                     if ((*j)->getToNode()->getEdges().size() > 2) {
    //                         attachments++;
    //                     }
    //                 }
    //                 if (attachments < 3) {
    //                     doLoop = false;
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                     if (gDebugFlag1) {
    //                         std::cout << " attachments=" << attachments << "\n";
    //                     }
    //                     gDebugFlag1 = false;
    // #endif
    //                 }
    //                 break;
    //             }
    //             if (visited.count(left) > 0) {
    //                 doLoop = false;
    //             } else {
    //                 // keep going
    //                 loopEdges.push_back(left);
    //                 e = left;
    //             }
    //         } while (doLoop);
    //         if (doLoop) {
    //             // check form factor to avoid elongated shapes (circle: 1, square: ~0.79)
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //             if (gDebugFlag1) {
    //                 std::cout << " formFactor=" << formFactor(loopEdges) << "\n";
    //             }
    // #endif
    //             double loopLength = 0;
    //             for (const NBEdge* const le : loopEdges) {
    //                 loopLength += le->getLoadedLength();
    //             }
    //             if (formFactor(loopEdges) > 0.6
    //                     && loopLength < OptionsCont::getOptions().getFloat("roundabouts.guess.max-length")) {
    //                 // collected edges are marked in markRoundabouts
    //                 EdgeSet guessed(loopEdges.begin(), loopEdges.end());
    //                 if (loadedRoundaboutEdges.count(loopEdges.front()) != 0) {
    //                     if (find(myRoundabouts.begin(), myRoundabouts.end(), guessed) == myRoundabouts.end()) {
    //                         for (auto it = myRoundabouts.begin(); it != myRoundabouts.end(); it++) {
    //                             if ((*it).count(loopEdges.front()) != 0) {
    //                                 WRITE_WARNINGF(TL("Replacing loaded roundabout '%' with '%'."), toString(*it), toString(guessed));
    //                                 myRoundabouts.erase(it);
    //                                 break;
    //                             }
    //                         }
    //                         myGuessedRoundabouts.insert(guessed);
    //                     }
    //                 } else {
    //                     myGuessedRoundabouts.insert(guessed);
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //                     if (gDebugFlag1) {
    //                         std::cout << " foundRoundabout=" << toString(loopEdges) << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         }
    // #ifdef DEBUG_GUESS_ROUNDABOUT
    //         gDebugFlag1 = false;
    // #endif
    //     }
    //     return (int)myGuessedRoundabouts.size();
    // }
    #[allow(unused_variables)]
    pub fn guess_roundabouts(&mut self) -> i32 {
        todo!("port NBEdgeCont::guessRoundabouts")
    }

    /// C++: `int NBEdgeCont::extractRoundabouts()`
    /// Determines which edges have been marked as roundabouts and stores them internally
    // int
    // NBEdgeCont::extractRoundabouts() {
    //     std::set<NBEdge*> candidateEdges;
    //     for (const auto& edge : myEdges) {
    //         NBEdge* const e = edge.second;
    //         if (e->getJunctionPriority(e->getToNode()) == NBEdge::JunctionPriority::ROUNDABOUT || e->getJunctionPriority(e->getFromNode()) == NBEdge::JunctionPriority::ROUNDABOUT) {
    //             candidateEdges.insert(e);
    //         }
    //     }
    //     std::set<NBEdge*> visited;
    //     int extracted = 0;
    //     for (const auto& edgeIt : candidateEdges) {
    //         EdgeVector loopEdges;
    //         NBEdge* e = edgeIt;
    //         if (visited.count(e) > 0) {
    //             // already seen
    //             continue;
    //         }
    //         loopEdges.push_back(e);
    //         bool doLoop = true;
    //         //
    //         do {
    //             if (std::find(visited.begin(), visited.end(), e) != visited.end()) {
    //                 if (loopEdges.size() > 1) {
    //                     addRoundabout(EdgeSet(loopEdges.begin(), loopEdges.end()));
    //                     ++extracted;
    //                 }
    //                 doLoop = false;
    //                 break;
    //             }
    //             visited.insert(e);
    //             loopEdges.push_back(e);
    //             const EdgeVector& outgoingEdges = e->getToNode()->getOutgoingEdges();
    //             EdgeVector::const_iterator me = std::find_if(outgoingEdges.begin(), outgoingEdges.end(), [](const NBEdge * outgoingEdge) {
    //                 return outgoingEdge->getJunctionPriority(outgoingEdge->getToNode()) == NBEdge::JunctionPriority::ROUNDABOUT;
    //             });
    //             if (me == outgoingEdges.end()) { // no closed loop
    //                 doLoop = false;
    //             } else {
    //                 e = *me;
    //             }
    //         } while (doLoop);
    //     }
    //     return extracted;
    // }
    #[allow(unused_variables)]
    pub fn extract_roundabouts(&mut self) -> i32 {
        todo!("port NBEdgeCont::extractRoundabouts")
    }

    /// C++: `void NBEdgeCont::cleanupRoundabouts()`
    // void
    // NBEdgeCont::cleanupRoundabouts() {
    //     // only loaded roundabouts are of concern here since guessing comes later
    //     std::set<EdgeSet> validRoundabouts;
    //     std::set<NBEdge*> validEdges;
    //     for (auto item : myEdges) {
    //         validEdges.insert(item.second);
    //     }
    //     for (EdgeSet roundabout : myRoundabouts) {
    //         EdgeSet validRoundabout;
    //         for (NBEdge* cand : roundabout) {
    //             if (validEdges.count(cand) != 0) {
    //                 validRoundabout.insert(cand);
    //             }
    //         }
    //         if (validRoundabout.size() > 0) {
    //             validRoundabouts.insert(validRoundabout);
    //         }
    //     }
    //     myRoundabouts = validRoundabouts;
    // }
    #[allow(unused_variables)]
    pub fn cleanup_roundabouts(&mut self) {
        todo!("port NBEdgeCont::cleanupRoundabouts")
    }

    /// C++: `bool NBEdgeCont::wasIgnored(std::string) const` [const]
    /// Returns whether the edge with the id was ignored during parsing
    // bool wasIgnored(std::string id) const {
    //         return myIgnoredEdges.count(id) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_ignored(&self, id: String) -> bool {
        todo!("port NBEdgeCont::wasIgnored")
    }

    /// C++: `void NBEdgeCont::ignore(std::string)`
    /// mark the given edge id as ignored
    // void ignore(std::string id) {
    //         myIgnoredEdges.insert(id);
    //     }
    #[allow(unused_variables)]
    pub fn ignore(&mut self, id: String) {
        todo!("port NBEdgeCont::ignore")
    }

    /// C++: `bool NBEdgeCont::wasRemoved(std::string) const` [const]
    /// Returns whether the edge with the id was deleted explicitly
    // bool wasRemoved(std::string id) const {
    //         return myExtractedEdges.count(id) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_removed(&self, id: String) -> bool {
        todo!("port NBEdgeCont::wasRemoved")
    }

    /// C++: `void NBEdgeCont::rename(NBEdge *, const std::string &)`
    /// Renames the edge. Throws exception if newID already exists
    // void
    // NBEdgeCont::rename(NBEdge* edge, const std::string& newID) {
    //     if (myEdges.count(newID) != 0) {
    //         throw ProcessError(TLF("Attempt to rename edge using existing id '%'", newID));
    //     }
    //     myEdges.erase(edge->getID());
    //     edge->setID(newID);
    //     myEdges[newID] = edge;
    //     // update oppositeID
    //     if (edge->getLanes().back().oppositeID != "") {
    //         NBEdge* oppo = retrieve(SUMOXMLDefinitions::getEdgeIDFromLane(edge->getLanes().back().oppositeID));
    //         if (oppo != nullptr) {
    //             oppo->getLaneStruct(oppo->getNumLanes() - 1).oppositeID = edge->getLaneID(edge->getNumLanes() - 1);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn rename(&mut self, edge: Ptr<NBEdge>, newID: &str) {
        todo!("port NBEdgeCont::rename")
    }

    /// C++: `void NBEdgeCont::addPostProcessConnection(const std::string &, int, const std::string &, int, bool, KeepClear, double, double, double, double, double, const PositionVector &, bool, bool, SVCPermissions, bool, const std::string &, SVCPermissions, SVCPermissions)`
    /// Adds a connection which could not be set during loading
    // void
    // NBEdgeCont::addPostProcessConnection(const std::string& from, int fromLane, const std::string& to, int toLane, bool mayDefinitelyPass,
    //                                      KeepClear keepClear, double contPos, double visibility, double speed, double friction, double length,
    //                                      const PositionVector& customShape, bool uncontrolled, bool warnOnly,
    //                                      SVCPermissions permissions, bool indirectLeft, const std::string& edgeType, SVCPermissions changeLeft, SVCPermissions changeRight) {
    //     myConnections[from].push_back(PostProcessConnection(from, fromLane, to, toLane, mayDefinitelyPass, keepClear, contPos, visibility,
    //                                   speed, friction, length, customShape, uncontrolled, warnOnly, permissions, indirectLeft, edgeType, changeLeft, changeRight));
    // }
    #[allow(unused_variables)]
    pub fn add_post_process_connection(&mut self, from: &str, fromLane: i32, to: &str, toLane: i32, mayDefinitelyPass: bool, keepClear: KeepClear, contPos: f64, visibility: f64, speed: f64, friction: f64, length: f64, customShape: &PositionVector, uncontrolled: bool, warnOnly: bool, permissions: SVCPermissions, indirectLeft: bool, edgeType: &str, changeLeft: SVCPermissions, changeRight: SVCPermissions) {
        todo!("port NBEdgeCont::addPostProcessConnection")
    }

    /// C++: `bool NBEdgeCont::hasPostProcessConnection(const std::string &, const std::string &)`
    /// add post process connections
    // bool
    // NBEdgeCont::hasPostProcessConnection(const std::string& from, const std::string& to) {
    //     if (myConnections.count(from) == 0) {
    //         return false;
    //     } else {
    //         if (to == "") {
    //             // wildcard
    //             return true;
    //         }
    //         for (const auto& ppc : myConnections[from]) {
    //             if (ppc.to == to) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn has_post_process_connection(&mut self, from: &str, to: &str) -> bool {
        todo!("port NBEdgeCont::hasPostProcessConnection")
    }

    /// C++: `void NBEdgeCont::recheckPostProcessConnections()`
    /// Try to set any stored connections
    // void
    // NBEdgeCont::recheckPostProcessConnections() {
    //     const bool warnOnly = OptionsCont::getOptions().exists("ignore-errors.connections") && OptionsCont::getOptions().getBool("ignore-errors.connections");
    //     for (const auto& item : myConnections) {
    //         for (std::vector<PostProcessConnection>::const_iterator i = item.second.begin(); i != item.second.end(); ++i) {
    //             NBEdge* from = retrievePossiblySplit((*i).from, true);
    //             NBEdge* to = retrievePossiblySplit((*i).to, false);
    //             if (from == nullptr || to == nullptr ||
    //                     !from->addLane2LaneConnection((*i).fromLane, to, (*i).toLane, NBEdge::Lane2LaneInfoType::USER, true, (*i).mayDefinitelyPass,
    //                                                   (*i).keepClear, (*i).contPos, (*i).visibility, (*i).speed, (*i).friction, (*i).customLength, (*i).customShape,
    //                                                   (*i).uncontrolled, (*i).permissions, (*i).indirectLeft, (*i).edgeType, (*i).changeLeft, (*i).changeRight,
    //                                                   true)) {
    //                 const std::string msg = "Could not insert connection between '" + (*i).from + "' and '" + (*i).to + "' after build.";
    //                 if (warnOnly || (*i).warnOnly) {
    //                     WRITE_WARNING(msg);
    //                 } else {
    //                     WRITE_ERROR(msg);
    //                 }
    //             }
    //         }
    //     }
    //     // during loading we also kept some ambiguous connections in hope they might be valid after processing
    //     // we need to make sure that all invalid connections are removed now
    //     for (EdgeCont::iterator it = myEdges.begin(); it != myEdges.end(); ++it) {
    //         NBEdge* edge = it->second;
    //         NBNode* to = edge->getToNode();
    //         // make a copy because we may delete connections
    //         std::vector<NBEdge::Connection> connections = edge->getConnections();
    //         for (std::vector<NBEdge::Connection>::iterator it_con = connections.begin(); it_con != connections.end(); ++it_con) {
    //             NBEdge::Connection& c = *it_con;
    //             if (c.toEdge != nullptr && c.toEdge->getFromNode() != to) {
    //                 WRITE_WARNING("Found and removed invalid connection from edge '" + edge->getID() +
    //                               "' to edge '" + c.toEdge->getID() + "' via junction '" + to->getID() + "'.");
    //                 edge->removeFromConnections(c.toEdge);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_post_process_connections(&mut self) {
        todo!("port NBEdgeCont::recheckPostProcessConnections")
    }

    /// C++: `void NBEdgeCont::generateStreetSigns()`
    /// assigns street signs to edges based on toNode types
    // void
    // NBEdgeCont::generateStreetSigns() {
    //     for (EdgeCont::iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         NBEdge* e = i->second;
    //         const double offset = MAX2(0., e->getLength() - 3);
    //         if (e->getToNode()->isSimpleContinuation(false)) {
    //             // not a "real" junction?
    //             continue;
    //         }
    //         const SumoXMLNodeType nodeType = e->getToNode()->getType();
    //         switch (nodeType) {
    //             case SumoXMLNodeType::PRIORITY:
    //                 // yield or major?
    //                 if (e->getJunctionPriority(e->getToNode()) > 0) {
    //                     e->addSign(NBSign(NBSign::SIGN_TYPE_PRIORITY, offset));
    //                 } else {
    //                     e->addSign(NBSign(NBSign::SIGN_TYPE_YIELD, offset));
    //                 }
    //                 break;
    //             case SumoXMLNodeType::PRIORITY_STOP:
    //                 // yield or major?
    //                 if (e->getJunctionPriority(e->getToNode()) > 0) {
    //                     e->addSign(NBSign(NBSign::SIGN_TYPE_PRIORITY, offset));
    //                 } else {
    //                     e->addSign(NBSign(NBSign::SIGN_TYPE_STOP, offset));
    //                 }
    //                 break;
    //             case SumoXMLNodeType::ALLWAY_STOP:
    //                 e->addSign(NBSign(NBSign::SIGN_TYPE_ALLWAY_STOP, offset));
    //                 break;
    //             case SumoXMLNodeType::RIGHT_BEFORE_LEFT:
    //                 e->addSign(NBSign(NBSign::SIGN_TYPE_RIGHT_BEFORE_LEFT, offset));
    //                 break;
    //             case SumoXMLNodeType::LEFT_BEFORE_RIGHT:
    //                 e->addSign(NBSign(NBSign::SIGN_TYPE_LEFT_BEFORE_RIGHT, offset));
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_street_signs(&mut self) {
        todo!("port NBEdgeCont::generateStreetSigns")
    }

    /// C++: `int NBEdgeCont::guessSpecialLanes(SUMOVehicleClass, double, double, double, bool, const std::string &, NBTrafficLightLogicCont &)`
    /// add sidwalks to edges within the given limits or permissions and return the number of edges affected
    // int
    // NBEdgeCont::guessSpecialLanes(SUMOVehicleClass svc, double width, double minSpeed, double maxSpeed, bool fromPermissions, const std::string& excludeOpt,
    //                               NBTrafficLightLogicCont& tlc) {
    //     int lanesCreated = 0;
    //     std::vector<std::string> edges;
    //     if (excludeOpt != "") {
    //         edges = OptionsCont::getOptions().getStringVector(excludeOpt);
    //     }
    //     std::set<std::string> exclude(edges.begin(), edges.end());
    //     for (EdgeCont::iterator it = myEdges.begin(); it != myEdges.end(); it++) {
    //         NBEdge* edge = it->second;
    //         if (// not excluded
    //             exclude.count(edge->getID()) == 0
    //             // does not yet have a sidewalk/bikelane
    //             && !edge->hasRestrictedLane(svc)
    //             // needs a sidewalk/bikelane
    //             && ((edge->getPermissions() & ~SVC_VULNERABLE) != 0 || (edge->getPermissions() & svc) == 0)
    //             && (
    //                 // guess.from-permissions
    //                 (fromPermissions && (edge->getPermissions() & svc) != 0)
    //                 // guess from speed
    //                 || (!fromPermissions && edge->getSpeed() > minSpeed && edge->getSpeed() <= maxSpeed)
    //             )) {
    //             edge->addRestrictedLane(width, svc);
    //             lanesCreated += 1;
    //             if (svc != SVC_PEDESTRIAN) {
    //                 if (edge->getStep() == NBEdge::EdgeBuildingStep::LANES2LANES_USER) {
    //                     // preserve existing connections and only add new ones
    //                     edge->declareConnectionsAsLoaded(NBEdge::EdgeBuildingStep::LANES2LANES_DONE);
    //                     edge->getFromNode()->recheckVClassConnections(edge);
    //                     for (NBEdge* to : edge->getToNode()->getOutgoingEdges()) {
    //                         edge->getToNode()->recheckVClassConnections(to);
    //                     }
    //                     // patching TLS is not feasible because existing states may
    //                     // change from 'G' to 'g' when bike lanes are added (i.e. right-turns)
    //                 } else {
    //                     edge->invalidateConnections(true);
    //                     edge->getFromNode()->invalidateOutgoingConnections(true);
    //                 }
    //                 edge->getFromNode()->invalidateTLS(tlc, true, false);
    //                 edge->getToNode()->invalidateTLS(tlc, true, false);
    //             }
    //         } else if (!fromPermissions && edge->getSpeed() > maxSpeed && !edge->hasRestrictedLane(svc)) {
    //             edge->disallowVehicleClass(-1, svc);
    //         }
    //     }
    //     return lanesCreated;
    // }
    #[allow(unused_variables)]
    pub fn guess_special_lanes(&mut self, svc: SUMOVehicleClass, width: f64, minSpeed: f64, maxSpeed: f64, fromPermissions: bool, excludeOpt: &str, tlc: &mut NBTrafficLightLogicCont) -> i32 {
        todo!("port NBEdgeCont::guessSpecialLanes")
    }

    /// C++: `const std::set<EdgeSet> NBEdgeCont::getRoundabouts() const` [const]
    /// Returns the determined roundabouts
    // const std::set<EdgeSet>
    // NBEdgeCont::getRoundabouts() const {
    //     std::set<EdgeSet> result = myRoundabouts;
    //     result.insert(myGuessedRoundabouts.begin(), myGuessedRoundabouts.end());
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_roundabouts(&self) -> BTreeSet<EdgeSet> {
        todo!("port NBEdgeCont::getRoundabouts")
    }

    /// C++: `bool NBEdgeCont::hasGuessedRoundabouts() const` [const]
    /// check if there is guessed roundabouts
    // bool hasGuessedRoundabouts() const {
    //         return myGuessedRoundabouts.size() > 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_guessed_roundabouts(&self) -> bool {
        todo!("port NBEdgeCont::hasGuessedRoundabouts")
    }

    /// C++: `void NBEdgeCont::addRoundabout(const EdgeSet &)`
    /// add user specified roundabout
    // void
    // NBEdgeCont::addRoundabout(const EdgeSet& roundabout) {
    //     if (roundabout.size() > 0) {
    //         if (find(myRoundabouts.begin(), myRoundabouts.end(), roundabout) != myRoundabouts.end()) {
    //             WRITE_WARNING("Ignoring duplicate roundabout: " + toString(roundabout));
    //         } else {
    //             myRoundabouts.insert(roundabout);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_roundabout(&mut self, roundabout: &EdgeSet) {
        todo!("port NBEdgeCont::addRoundabout")
    }

    /// C++: `void NBEdgeCont::removeRoundabout(const NBNode *)`
    /// remove roundabout that contains the given node
    // void
    // NBEdgeCont::removeRoundabout(const NBNode* node) {
    //     for (auto it = myRoundabouts.begin(); it != myRoundabouts.end(); ++it) {
    //         for (NBEdge* e : *it) {
    //             if (e->getToNode() == node) {
    //                 myRoundabouts.erase(it);
    //                 return;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_roundabout(&mut self, node: Ptr<NBNode>) {
        todo!("port NBEdgeCont::removeRoundabout")
    }

    /// C++: `void NBEdgeCont::removeRoundaboutEdges(const EdgeSet &)`
    /// remove edges from all stored roundabouts
    // void
    // NBEdgeCont::removeRoundaboutEdges(const EdgeSet& toRemove) {
    //     removeRoundaboutEdges(toRemove, myRoundabouts);
    //     removeRoundaboutEdges(toRemove, myGuessedRoundabouts);
    // }
    #[allow(unused_variables)]
    pub fn remove_roundabout_edges(&mut self, toRemove: &EdgeSet) {
        todo!("port NBEdgeCont::removeRoundaboutEdges")
    }

    /// C++: `void NBEdgeCont::markRoundabouts()`
    /// mark edge priorities and prohibit turn-arounds for all roundabout edges
    // void
    // NBEdgeCont::markRoundabouts() {
    //     for (const EdgeSet& roundaboutSet : getRoundabouts()) {
    //         for (NBEdge* const edge : roundaboutSet) {
    //             // disable turnarounds on incoming edges
    //             NBNode* const node = edge->getToNode();
    //             for (NBEdge* const inEdge : node->getIncomingEdges()) {
    //                 if (roundaboutSet.count(inEdge) > 0) {
    //                     continue;
    //                 }
    //                 if (inEdge->getStep() >= NBEdge::EdgeBuildingStep::LANES2LANES_USER) {
    //                     continue;
    //                 }
    //                 if (inEdge->getTurnDestination() != nullptr) {
    //                     inEdge->removeFromConnections(inEdge->getTurnDestination(), -1);
    //                 } else {
    //                     // also remove connections that are effecively a turnaround but
    //                     // where not correctly detector due to geometrical quirks
    //                     const std::vector<NBEdge::Connection> cons = inEdge->getConnections();
    //                     for (const NBEdge::Connection& con : cons) {
    //                         if (con.toEdge && roundaboutSet.count(con.toEdge) == 0) {
    //                             const double angle = fabs(NBHelpers::normRelAngle(inEdge->getAngleAtNode(node), con.toEdge->getAngleAtNode(node)));
    //                             if (angle > 160) {
    //                                 inEdge->removeFromConnections(con.toEdge, -1);
    //                             }
    //                         }
    //                     }
    //                 }
    //
    //             }
    //             // let the connections to succeeding roundabout edge have a higher priority
    //             edge->setJunctionPriority(node, NBEdge::JunctionPriority::ROUNDABOUT);
    //             edge->setJunctionPriority(edge->getFromNode(), NBEdge::JunctionPriority::ROUNDABOUT);
    //             node->setRoundabout();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn mark_roundabouts(&mut self) {
        todo!("port NBEdgeCont::markRoundabouts")
    }

    /// C++: `void NBEdgeCont::patchRoundabouts(NBEdge *, NBEdge *, NBEdge *, std::set<EdgeSet> &)`
    /// fix roundabout information after splitting an edge
    // void
    // NBEdgeCont::patchRoundabouts(NBEdge* orig, NBEdge* part1, NBEdge* part2, std::set<EdgeSet>& roundabouts) {
    //     std::set<EdgeSet> addLater;
    //     for (std::set<EdgeSet>::iterator it = roundabouts.begin(); it != roundabouts.end(); ++it) {
    //         EdgeSet roundaboutSet = *it;
    //         if (roundaboutSet.count(orig) > 0) {
    //             roundaboutSet.erase(orig);
    //             roundaboutSet.insert(part1);
    //             roundaboutSet.insert(part2);
    //         }
    //         addLater.insert(roundaboutSet);
    //     }
    //     roundabouts.clear();
    //     roundabouts.insert(addLater.begin(), addLater.end());
    // }
    #[allow(unused_variables)]
    pub fn patch_roundabouts(&mut self, orig: Ptr<NBEdge>, part1: Ptr<NBEdge>, part2: Ptr<NBEdge>, roundabouts: &mut BTreeSet<EdgeSet>) {
        todo!("port NBEdgeCont::patchRoundabouts")
    }

    /// C++: `bool NBEdgeCont::ignoreFilterMatch(NBEdge *)`
    /// Returns true if this edge matches one of the removal criteria
    // bool
    // NBEdgeCont::ignoreFilterMatch(NBEdge* edge) {
    //     if (!myRemoveEdgesAfterLoading) {
    //         // check whether the edge is a named edge to keep
    //         if (myEdges2Keep.size() != 0) {
    //             if (myEdges2Keep.count(edge->getID()) == 0) {
    //                 // explicit whitelisting may be combined additively with other filters
    //                 if (myVehicleClasses2Keep == 0 && myVehicleClasses2Remove == 0
    //                         && myTypes2Keep.size() == 0 && myTypes2Remove.size() == 0
    //                         && myPruningBoundary.size() == 0) {
    //                     return true;
    //                 }
    //             } else {
    //                 // explicit whitelisting overrides other filters
    //                 return false;
    //             }
    //         }
    //         // remove edges which allow a speed below a set one (set using "keep-edges.min-speed")
    //         if (edge->getSpeed() < myEdgesMinSpeed) {
    //             return true;
    //         }
    //         // check whether the edge shall be removed because it does not allow any of the wished classes
    //         if (myVehicleClasses2Keep != 0 && (myVehicleClasses2Keep & edge->getPermissions()) == 0) {
    //             return true;
    //         }
    //         // check whether the edge shall be removed due to allowing unwished classes only
    //         if (myVehicleClasses2Remove != 0 && (myVehicleClasses2Remove | edge->getPermissions()) == myVehicleClasses2Remove) {
    //             return true;
    //         }
    //     }
    //     // check whether the edge is a named edge to remove
    //     if (myEdges2Remove.size() != 0) {
    //         if (myEdges2Remove.count(edge->getID()) != 0) {
    //             return true;
    //         }
    //     }
    //     // check whether the edge shall be removed because it does not have one of the requested types
    //     if (myTypes2Keep.size() != 0) {
    //         if (myTypes2Keep.count(edge->getTypeID()) == 0) {
    //             return true;
    //         }
    //     }
    //     // check whether the edge shall be removed because it has one of the forbidden types
    //     if (myTypes2Remove.size() != 0) {
    //         if (myTypes2Remove.count(edge->getTypeID()) > 0) {
    //             return true;
    //         }
    //     }
    //     // check whether the edge is within the pruning boundary
    //     if (myPruningBoundary.size() != 0) {
    //         if (myNeedGeoTransformedPruningBoundary) {
    //             if (GeoConvHelper::getProcessing().usingGeoProjection()) {
    //                 NBNetBuilder::transformCoordinates(myPruningBoundary, false);
    //             } else if (GeoConvHelper::getLoaded().usingGeoProjection()) {
    //                 // XXX what if input file with different projections are loaded?
    //                 for (int i = 0; i < (int) myPruningBoundary.size(); i++) {
    //                     GeoConvHelper::getLoaded().x2cartesian_const(myPruningBoundary[i]);
    //                 }
    //             } else {
    //                 WRITE_ERROR(TL("Cannot prune edges using a geo-boundary because no projection has been loaded"));
    //             }
    //             myNeedGeoTransformedPruningBoundary = false;
    //         }
    //         if (!(edge->getGeometry().getBoxBoundary().grow(POSITION_EPS).overlapsWith(myPruningBoundary))) {
    //             return true;
    //         } else if (!(edge->getGeometry().partialWithin(myPruningBoundary, 2 * POSITION_EPS) || edge->getGeometry().intersects(myPruningBoundary))) {
    //             // a more detailed check is necessary because the bounding box may be much bigger than the edge
    //             // @note: overlapsWith implicitly closes the edge shape but this is not wanted here
    //             return true;
    //         }
    //     }
    //     if (myTypeCont.knows(edge->getTypeID()) && myTypeCont.getEdgeTypeShallBeDiscarded(edge->getTypeID())) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn ignore_filter_match(&mut self, edge: Ptr<NBEdge>) -> bool {
        todo!("port NBEdgeCont::ignoreFilterMatch")
    }

    /// C++: `int NBEdgeCont::remapIDs(bool, bool, bool, const std::string &, NBPTStopCont &)`
    /// remap node IDs accoring to options --numerical-ids and --reserved-ids
    // int
    // NBEdgeCont::remapIDs(bool numericaIDs, bool reservedIDs, bool keptIDs, const std::string& prefix, NBPTStopCont& sc) {
    //     bool startGiven = !OptionsCont::getOptions().isDefault("numerical-ids.edge-start");
    //     if (!numericaIDs && !reservedIDs && prefix == "" && !startGiven) {
    //         return 0;
    //     }
    //     std::vector<std::string> avoid;
    //     if (startGiven) {
    //         avoid.push_back(toString(OptionsCont::getOptions().getInt("numerical-ids.edge-start") - 1));
    //     } else {
    //         avoid = getAllNames();
    //     }
    //     std::set<std::string> reserve;
    //     if (reservedIDs) {
    //         NBHelpers::loadPrefixedIDsFomFile(OptionsCont::getOptions().getString("reserved-ids"), "edge:", reserve);
    //         avoid.insert(avoid.end(), reserve.begin(), reserve.end());
    //     }
    //     IDSupplier idSupplier("", avoid);
    //     std::set<NBEdge*, ComparatorIdLess> toChange;
    //     for (EdgeCont::iterator it = myEdges.begin(); it != myEdges.end(); it++) {
    //         if (startGiven) {
    //             toChange.insert(it->second);
    //             continue;
    //         }
    //         if (numericaIDs) {
    //             try {
    //                 StringUtils::toLong(it->first);
    //             } catch (NumberFormatException&) {
    //                 toChange.insert(it->second);
    //             }
    //         }
    //         if (reservedIDs && reserve.count(it->first) > 0) {
    //             toChange.insert(it->second);
    //         }
    //     }
    //     std::set<std::string> keep;
    //     if (keptIDs) {
    //         NBHelpers::loadPrefixedIDsFomFile(OptionsCont::getOptions().getString("kept-ids"), "edge:", keep);
    //         for (auto it = toChange.begin(); it != toChange.end();) {
    //             if (keep.count((*it)->getID()) != 0) {
    //                 toChange.erase(it++);
    //             } else {
    //                 it++;
    //             }
    //         }
    //     }
    //
    //     std::map<std::string, std::vector<std::shared_ptr<NBPTStop> > > stopsOnEdge;
    //     for (const auto& item : sc.getStops()) {
    //         stopsOnEdge[item.second->getEdgeId()].push_back(item.second);
    //     }
    //
    //     const bool origNames = OptionsCont::getOptions().getBool("output.original-names");
    //     for (NBEdge* edge : toChange) {
    //         myEdges.erase(edge->getID());
    //     }
    //     for (NBEdge* edge : toChange) {
    //         const std::string origID = edge->getID();
    //         if (origNames) {
    //             edge->setOrigID(origID, false);
    //         }
    //         edge->setID(idSupplier.getNext());
    //         myEdges[edge->getID()] = edge;
    //         for (std::shared_ptr<NBPTStop> stop : stopsOnEdge[origID]) {
    //             stop->setEdgeId(prefix + edge->getID(), *this);
    //         }
    //     }
    //     if (prefix.empty()) {
    //         return (int)toChange.size();
    //     } else {
    //         int renamed = 0;
    //         // make a copy because we will modify the map
    //         auto oldEdges = myEdges;
    //         for (auto item : oldEdges) {
    //             if (!StringUtils::startsWith(item.first, prefix) && keep.count(item.first) == 0) {
    //                 rename(item.second, prefix + item.first);
    //                 renamed++;
    //             }
    //         }
    //         return renamed;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remap_i_ds(&mut self, numericaIDs: bool, reservedIDs: bool, keptIDs: bool, prefix: &str, sc: &mut NBPTStopCont) -> i32 {
        todo!("port NBEdgeCont::remapIDs")
    }

    /// C++: `void NBEdgeCont::checkOverlap(double, double) const` [const]
    /// check whether edges overlap
    // void
    // NBEdgeCont::checkOverlap(double threshold, double zThreshold) const {
    //     for (EdgeCont::const_iterator it = myEdges.begin(); it != myEdges.end(); it++) {
    //         const NBEdge* e1 = it->second;
    //         Boundary b1 = e1->getGeometry().getBoxBoundary();
    //         b1.grow(e1->getTotalWidth());
    //         PositionVector outline1 = e1->getCCWBoundaryLine(*e1->getFromNode());
    //         outline1.append(e1->getCCWBoundaryLine(*e1->getToNode()));
    //         // check is symmetric. only check once per pair
    //         for (EdgeCont::const_iterator it2 = it; it2 != myEdges.end(); it2++) {
    //             const NBEdge* e2 = it2->second;
    //             if (e1 == e2) {
    //                 continue;
    //             }
    //             Boundary b2 = e2->getGeometry().getBoxBoundary();
    //             b2.grow(e2->getTotalWidth());
    //             if (b1.overlapsWith(b2)) {
    //                 PositionVector outline2 = e2->getCCWBoundaryLine(*e2->getFromNode());
    //                 outline2.append(e2->getCCWBoundaryLine(*e2->getToNode()));
    //                 const double overlap = outline1.getOverlapWith(outline2, zThreshold);
    //                 if (overlap > threshold) {
    //                     WRITE_WARNINGF(TL("Edge '%' overlaps with edge '%' by %."), e1->getID(), e2->getID(), overlap);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_overlap(&self, threshold: f64, zThreshold: f64) {
        todo!("port NBEdgeCont::checkOverlap")
    }

    /// C++: `void NBEdgeCont::checkGrade(double) const` [const]
    /// check whether edges are to steep
    // void
    // NBEdgeCont::checkGrade(double threshold) const {
    //     for (EdgeCont::const_iterator it = myEdges.begin(); it != myEdges.end(); it++) {
    //         const NBEdge* edge = it->second;
    //         for (int i = 0; i < (int)edge->getNumLanes(); i++) {
    //             double maxJump = 0;
    //             const double grade = edge->getLaneShape(i).getMaxGrade(maxJump);
    //             if (maxJump > 0.01) {
    //                 WRITE_WARNINGF(TL("Edge '%' has a vertical jump of %m."), edge->getID(), maxJump);
    //             } else if (grade > threshold) {
    //                 WRITE_WARNINGF(TL("Edge '%' has a grade of %%."), edge->getID(), grade * 100, "%");
    //                 break;
    //             }
    //         }
    //         const std::vector<NBEdge::Connection>& connections = edge->getConnections();
    //         for (std::vector<NBEdge::Connection>::const_iterator it_con = connections.begin(); it_con != connections.end(); ++it_con) {
    //             const NBEdge::Connection& c = *it_con;
    //             double maxJump = 0;
    //             const double grade = MAX2(c.shape.getMaxGrade(maxJump), c.viaShape.getMaxGrade(maxJump));
    //             if (maxJump > 0.01) {
    //                 WRITE_WARNINGF(TL("Connection '%' has a vertical jump of %m."), c.getDescription(edge), maxJump);
    //             } else if (grade > threshold) {
    //                 WRITE_WARNINGF(TL("Connection '%' has a grade of %%."), c.getDescription(edge), grade * 100, "%");
    //                 break;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_grade(&self, threshold: f64) {
        todo!("port NBEdgeCont::checkGrade")
    }

    /// C++: `EdgeVector NBEdgeCont::getGeneratedFrom(const std::string &) const` [const]
    /// Returns the edges which have been built by splitting the edge of the given id
    // EdgeVector
    // NBEdgeCont::getGeneratedFrom(const std::string& id) const {
    //     int len = (int)id.length();
    //     EdgeVector ret;
    //     for (EdgeCont::const_iterator i = myEdges.begin(); i != myEdges.end(); ++i) {
    //         std::string curr = (*i).first;
    //         // the next check makes it possibly faster - we don not have
    //         //  to compare the names
    //         if ((int)curr.length() <= len) {
    //             continue;
    //         }
    //         // the name must be the same as the given id but something
    //         //  beginning with a '[' must be appended to it
    //         if (curr.substr(0, len) == id && curr[len] == '[') {
    //             ret.push_back((*i).second);
    //             continue;
    //         }
    //         // ok, maybe the edge is a compound made during joining of edges
    //         std::string::size_type pos = curr.find(id);
    //         // surely not
    //         if (pos == std::string::npos) {
    //             continue;
    //         }
    //         // check leading char
    //         if (pos > 0) {
    //             if (curr[pos - 1] != ']' && curr[pos - 1] != '+') {
    //                 // actually, this is another id
    //                 continue;
    //             }
    //         }
    //         if (pos + id.length() < curr.length()) {
    //             if (curr[pos + id.length()] != '[' && curr[pos + id.length()] != '+') {
    //                 // actually, this is another id
    //                 continue;
    //             }
    //         }
    //         ret.push_back((*i).second);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_generated_from(&self, id: &str) -> EdgeVector {
        todo!("port NBEdgeCont::getGeneratedFrom")
    }

    /// C++: `int NBEdgeCont::joinLanes(SVCPermissions)`
    /// join adjacent lanes with the given permissions
    // int
    // NBEdgeCont::joinLanes(SVCPermissions perms) {
    //     int affectedEdges = 0;
    //     for (auto item : myEdges) {
    //         if (item.second->joinLanes(perms)) {
    //             affectedEdges++;
    //         }
    //     }
    //     return affectedEdges;
    // }
    #[allow(unused_variables)]
    pub fn join_lanes(&mut self, perms: SVCPermissions) -> i32 {
        todo!("port NBEdgeCont::joinLanes")
    }

    /// C++: `int NBEdgeCont::joinTramEdges(NBDistrictCont &, NBPTStopCont &, NBPTLineCont &, double)`
    /// join tram edges into adjacent lanes
    // int
    // NBEdgeCont::joinTramEdges(NBDistrictCont& dc, NBPTStopCont& sc, NBPTLineCont& lc, double maxDist) {
    //     // this is different from joinSimilarEdges because there don't need to be
    //     // shared nodes and tram edges may be split
    //     std::vector<NBEdge*> tramEdges;
    //     std::vector<NBEdge*> targetEdges;
    //     for (auto item : myEdges) {
    //         SVCPermissions permissions = item.second->getPermissions();
    //         if (isTram(permissions)) {
    //             if (item.second->getNumLanes() == 1) {
    //                 tramEdges.push_back(item.second);
    //             } else {
    //                 WRITE_WARNINGF(TL("Not joining tram edge '%' with % lanes."), item.second->getID(), item.second->getNumLanes());
    //             }
    //         } else if ((permissions & (SVC_PASSENGER | SVC_BUS)) != 0) {
    //             targetEdges.push_back(item.second);
    //         }
    //     }
    //     if (tramEdges.empty() || targetEdges.empty()) {
    //         return 0;
    //     }
    //     int numJoined = 0;
    //     NamedRTree tramTree;
    //     for (NBEdge* const edge : tramEdges) {
    //         const Boundary& bound = edge->getGeometry().getBoxBoundary();
    //         float min[2] = { static_cast<float>(bound.xmin()), static_cast<float>(bound.ymin()) };
    //         float max[2] = { static_cast<float>(bound.xmax()), static_cast<float>(bound.ymax()) };
    //         tramTree.Insert(min, max, edge);
    //     }
    //     // {targetEdge, laneIndex : tramEdge}
    //     std::map<std::pair<NBEdge*, int>, NBEdge*, MinLaneComparatorIdLess> matches;
    //
    //     for (NBEdge* const edge : targetEdges) {
    //         Boundary bound = edge->getGeometry().getBoxBoundary();
    //         bound.grow(maxDist + edge->getTotalWidth());
    //         float min[2] = { static_cast<float>(bound.xmin()), static_cast<float>(bound.ymin()) };
    //         float max[2] = { static_cast<float>(bound.xmax()), static_cast<float>(bound.ymax()) };
    //         std::set<const Named*> near;
    //         Named::StoringVisitor visitor(near);
    //         tramTree.Search(min, max, visitor);
    //         // the nearby set is actually just re-sorting according to the id to make the tests comparable
    //         std::set<NBEdge*, ComparatorIdLess> nearby;
    //         for (const Named* namedEdge : near) {
    //             nearby.insert(const_cast<NBEdge*>(static_cast<const NBEdge*>(namedEdge)));
    //         }
    //         for (NBEdge* const tramEdge : nearby) {
    //             // find a continous stretch of tramEdge that runs along one of the lanes of the road edge
    //             PositionVector tramShape = tramEdge->getGeometry();
    //             if (tramEdge->getToNode() == edge->getToNode()) {
    //                 tramShape.extrapolate(tramShape.back().distanceTo2D(edge->getGeometry().back()), false, true);
    //             }
    //             double minEdgeDist = maxDist + 1;
    //             int minLane = -1;
    //             // find the lane where the maximum distance from the tram geometry
    //             // is minimal and within maxDist
    //             for (int i = 0; i < edge->getNumLanes(); i++) {
    //                 double maxLaneDist = -1;
    //                 if ((edge->getPermissions(i) & (SVC_PASSENGER | SVC_BUS)) != 0) {
    //                     const PositionVector& laneShape = edge->getLaneShape(i);
    //                     for (Position pos : laneShape) {
    //                         const double dist = tramShape.distance2D(pos, false);
    // #ifdef DEBUG_JOIN_TRAM
    //                         //if (edge->getID() == "106838214#1") {
    //                         //    std::cout << " edge=" << edge->getID() << " tramEdge=" << tramEdge->getID() << " lane=" << i << " pos=" << pos << " dist=" << dist << "\n";
    //                         //}
    // #endif
    //                         if (dist == GeomHelper::INVALID_OFFSET || dist > maxDist) {
    //                             maxLaneDist = -1;
    //                             break;
    //                         }
    //                         maxLaneDist = MAX2(maxLaneDist, dist);
    //                     }
    //                     if (maxLaneDist >= 0 && maxLaneDist < minEdgeDist) {
    //                         minEdgeDist = maxLaneDist;
    //                         minLane = i;
    //                     }
    //                 }
    //             }
    //             if (minLane >= 0) {
    //                 // edge could run in the wrong direction and still fit the threshold we check the angle as well
    //                 const PositionVector& laneShape = edge->getLaneShape(minLane);
    //                 const double offset1 = tramShape.nearest_offset_to_point2D(laneShape.front(), false);
    //                 const double offset2 = tramShape.nearest_offset_to_point2D(laneShape.back(), false);
    //                 Position p1 = tramShape.positionAtOffset2D(offset1);
    //                 Position p2 = tramShape.positionAtOffset2D(offset2);
    //                 double tramAngle = GeomHelper::legacyDegree(p1.angleTo2D(p2), true);
    //                 double laneAngle = GeomHelper::legacyDegree(laneShape[0].angleTo2D(laneShape[-1]), true);
    //                 const double angleMismatch = MIN2(GeomHelper::getMinAngleDiff(tramAngle, edge->getTotalAngle()),
    //                         GeomHelper::getMinAngleDiff(tramAngle, laneAngle));
    //                 bool angleOK = angleMismatch < JOIN_TRAM_MAX_ANGLE;
    //                 if (angleOK && offset2 > offset1) {
    //                     std::pair<NBEdge*, int> key = std::make_pair(edge, minLane);
    //                     if (matches.count(key) == 0) {
    //                         matches[key] = tramEdge;
    //                     } else {
    //                         WRITE_WARNINGF(TL("Ambiguous tram edges '%' and '%' for lane '%'."), matches[key]->getID(), tramEdge->getID(), edge->getLaneID(minLane));
    //                     }
    // #ifdef DEBUG_JOIN_TRAM
    //                     std::cout << edge->getLaneID(minLane) << " is close to tramEdge " << tramEdge->getID() << " maxLaneDist=" << minEdgeDist << " tramLength=" << tramEdge->getLength() << " edgeLength=" << edge->getLength() << " tramAngle=" << tramAngle << " edgeAngle=" << edge->getTotalAngle() << "\n";
    // #endif
    //                 }
    //             }
    //         }
    //     }
    //     if (matches.size() == 0) {
    //         return 0;
    //     }
    //     const bool origNames = OptionsCont::getOptions().getBool("output.original-names");
    //     // find continous runs of matched edges for each tramEdge
    //     for (NBEdge* tramEdge : tramEdges) {
    //         std::vector<std::pair<double, std::pair<NBEdge*, int> > > roads;
    //         for (auto item : matches) {
    //             if (item.second == tramEdge) {
    //                 NBEdge* road = item.first.first;
    //                 int laneIndex = item.first.second;
    //                 const PositionVector& laneShape = road->getLaneShape(laneIndex);
    //                 double tramPos = tramEdge->getGeometry().nearest_offset_to_point2D(laneShape.front(), false);
    //                 //std::cout << " road=" << road->getID() << " tramEdge=" << tramEdge->getID() << " tramShape=" << tramEdge->getGeometry() << " laneFront=" << laneShape.front() << " tramPos=" << tramPos << "\n";
    //                 roads.push_back(std::make_pair(tramPos, item.first));
    //             }
    //         }
    //         if (roads.size() != 0) {
    //
    //             sort(roads.begin(), roads.end());
    // #ifdef DEBUG_JOIN_TRAM
    //             std::cout << " tramEdge=" << tramEdge->getID() << " roads=";
    //             for (auto item : roads) {
    //                 std::cout << item.second.first->getLaneID(item.second.second) << ",";
    //             }
    //             std::cout << " offsets=";
    //             for (auto item : roads) {
    //                 std::cout << item.first << ",";
    //             }
    //             std::cout << "\n";
    // #endif
    //             // merge tramEdge into road lanes
    //             EdgeVector replacement;
    //             double pos = 0;
    //             int tramPart = 0;
    //             std::string tramEdgeID = tramEdge->getID();
    //             NBNode* tramFrom = tramEdge->getFromNode();
    //             PositionVector tramShape = tramEdge->getGeometry();
    //             const double tramLength = tramShape.length();
    //             EdgeVector incoming = tramFrom->getIncomingEdges();
    //             bool erasedLast = false;
    //             for (const auto& item : roads) {
    //                 const double gap = item.first - pos;
    //                 NBEdge* road = item.second.first;
    //                 int laneIndex = item.second.second;
    //                 if (gap >= JOIN_TRAM_MIN_LENGTH && road->getFromNode() != tramEdge->getFromNode()) {
    // #ifdef DEBUG_JOIN_TRAM
    //                     std::cout << "    splitting tramEdge=" << tramEdge->getID() << " at " << item.first << " (gap=" << gap << ")\n";
    // #endif
    //                     const std::string firstPartID = tramEdgeID + "#" + toString(tramPart++);
    //                     splitAt(dc, tramEdge, gap, road->getFromNode(), firstPartID, tramEdgeID, 1, 1);
    //                     tramEdge = retrieve(tramEdgeID); // second part;
    //                     NBEdge* firstPart = retrieve(firstPartID);
    //                     firstPart->invalidateConnections(true);
    //                     incoming.clear();
    //                     incoming.push_back(firstPart);
    //                     replacement.push_back(firstPart);
    //                 }
    //                 pos = item.first + road->getGeometry().length();
    //                 numJoined++;
    //                 replacement.push_back(road);
    //                 // merge section of tramEdge into road lane
    //                 if (road->getToNode() != tramEdge->getToNode() && (tramLength - pos) >= JOIN_TRAM_MIN_LENGTH) {
    //                     tramEdge->reinitNodes(road->getToNode(), tramEdge->getToNode());
    //                     tramEdge->setGeometry(tramShape.getSubpart(pos, tramShape.length()));
    //                     erasedLast = false;
    // #ifdef DEBUG_JOIN_TRAM
    //                     std::cout << "    shorted tramEdge=" << tramEdge->getID() << " (joined with roadEdge=" << road->getID() << "\n";
    // #endif
    //                 } else {
    // #ifdef DEBUG_JOIN_TRAM
    //                     std::cout << "    erased tramEdge=" << tramEdge->getID() << "\n";
    // #endif
    //                     extract(dc, tramEdge, true);
    //                     erasedLast = true;
    //                 }
    //                 road->setPermissions(road->getPermissions(laneIndex) | SVC_TRAM, laneIndex);
    //                 if (origNames) {
    //                     road->setOrigID(tramEdgeID, true, laneIndex);
    //                 }
    //                 for (NBEdge* in : incoming) {
    //                     if (isTram(in->getPermissions()) && !in->isConnectedTo(road)) {
    //                         if (in->getFromNode() != road->getFromNode()) {
    //                             in->reinitNodes(in->getFromNode(), road->getFromNode());
    //                         } else {
    //                             extract(dc, in, true);
    // #ifdef DEBUG_JOIN_TRAM
    //                             std::cout << "    erased incoming tramEdge=" << in->getID() << "\n";
    // #endif
    //                         }
    //                     }
    //                 }
    //                 incoming.clear();
    //             }
    //             NBEdge* lastRoad = roads.back().second.first;
    //             if (erasedLast) {
    //                 // copy to avoid concurrent modification
    //                 auto outEdges = tramEdge->getToNode()->getOutgoingEdges();
    //                 for (NBEdge* out : outEdges) {
    //                     if (isTram(out->getPermissions()) && !lastRoad->isConnectedTo(out)) {
    //                         if (lastRoad->getToNode() != out->getToNode()) {
    //                             out->reinitNodes(lastRoad->getToNode(), out->getToNode());
    //                         } else {
    //                             extract(dc, out, true);
    // #ifdef DEBUG_JOIN_TRAM
    //                             std::cout << "    erased outgoing tramEdge=" << out->getID() << "\n";
    // #endif
    //
    //                         }
    //                     }
    //                 }
    //             } else {
    //                 replacement.push_back(tramEdge);
    //             }
    //             // update ptstops and ptlines
    //             sc.replaceEdge(tramEdgeID, replacement);
    //             lc.replaceEdge(tramEdgeID, replacement);
    //         }
    //     }
    //
    //     return numJoined;
    // }
    #[allow(unused_variables)]
    pub fn join_tram_edges(&mut self, dc: &mut NBDistrictCont, sc: &mut NBPTStopCont, lc: &mut NBPTLineCont, maxDist: f64) -> i32 {
        todo!("port NBEdgeCont::joinTramEdges")
    }

    /// C++: `EdgeVector NBEdgeCont::getAllEdges() const` [const]
    /// return all edges
    // EdgeVector
    // NBEdgeCont::getAllEdges() const {
    //     EdgeVector result;
    //     for (auto item : myEdges) {
    //         item.second->setNumericalID((int)result.size());
    //         result.push_back(item.second);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_all_edges(&self) -> EdgeVector {
        todo!("port NBEdgeCont::getAllEdges")
    }

    /// C++: `RouterEdgeVector NBEdgeCont::getAllRouterEdges() const` [const]
    /// return all router edges
    // RouterEdgeVector
    // NBEdgeCont::getAllRouterEdges() const {
    //     EdgeVector all = getAllEdges();
    //     return RouterEdgeVector(all.begin(), all.end());
    // }
    #[allow(unused_variables)]
    pub fn get_all_router_edges(&self) -> RouterEdgeVector {
        todo!("port NBEdgeCont::getAllRouterEdges")
    }

    /// C++: `bool NBEdgeCont::checkConsistency(const NBNodeCont &)`
    /// ensure that all edges have valid nodes
    // bool
    // NBEdgeCont::checkConsistency(const NBNodeCont& nc) {
    //     bool ok = true;
    //     for (const auto& item : myEdges) {
    //         NBEdge* e = item.second;
    //         if (nc.retrieve(e->getFromNode()->getID()) == nullptr) {
    //             WRITE_ERRORF(TL("Edge's '%' from-node '%' is not known."), e->getID(), e->getFromNode()->getID());
    //             ok = false;
    //         }
    //         if (nc.retrieve(e->getToNode()->getID()) == nullptr) {
    //             WRITE_ERRORF(TL("Edge's '%' to-node '%' is not known."), e->getID(), e->getToNode()->getID());
    //             ok = false;
    //         }
    //
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_consistency(&mut self, nc: &NBNodeCont) -> bool {
        todo!("port NBEdgeCont::checkConsistency")
    }

    /// C++: `void NBEdgeCont::updateAllChangeRestrictions(SVCPermissions)`
    /// modify all restrictions on lane changing for edges and connections
    // void
    // NBEdgeCont::updateAllChangeRestrictions(SVCPermissions ignoring) {
    //     for (auto item : myEdges) {
    //         item.second->updateChangeRestrictions(ignoring);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_all_change_restrictions(&mut self, ignoring: SVCPermissions) {
        todo!("port NBEdgeCont::updateAllChangeRestrictions")
    }

    /// C++: `void NBEdgeCont::addPrefix(const std::string &)`
    /// add prefix to all edges
    // void
    // NBEdgeCont::addPrefix(const std::string& prefix) {
    //     // make a copy of node containers
    //     const auto nodeContainerCopy = myEdges;
    //     myEdges.clear();
    //     for (const auto& node : nodeContainerCopy) {
    //         node.second->setID(prefix + node.second->getID());
    //         myEdges[node.second->getID()] = node.second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_prefix(&mut self, prefix: &str) {
        todo!("port NBEdgeCont::addPrefix")
    }

    /// C++: `void NBEdgeCont::fixSplitCustomLength()`
    /// adapt custom lengths of split edges to account for intersection size
    // void
    // NBEdgeCont::fixSplitCustomLength() {
    //     for (auto item : myEdges) {
    //         NBEdge* e = item.second;
    //         if (e->hasLoadedLength() && myWasSplit.count(e) != 0) {
    //             // subtract half the length of the longest incoming / outgoing connection
    //             double maxLengthOut = 0;
    //             for (const NBEdge::Connection& c : e->getConnections()) {
    //                 maxLengthOut = MAX2(maxLengthOut, c.length + c.viaLength);
    //             }
    //             double maxLengthIn = 0;
    //             for (const NBEdge* in : e->getIncomingEdges()) {
    //                 for (const NBEdge::Connection& c : in->getConnectionsFromLane(-1, e, -1)) {
    //                     maxLengthIn = MAX2(maxLengthIn, c.length + c.viaLength);
    //                 }
    //             }
    //             e->setLoadedLength(MAX2(POSITION_EPS, e->getLoadedLength() - (maxLengthIn + maxLengthOut) / 2));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fix_split_custom_length(&mut self) {
        todo!("port NBEdgeCont::fixSplitCustomLength")
    }

    /// C++: `void NBEdgeCont::computeAngles()`
    /// compute all edge angles
    // void
    // NBEdgeCont::computeAngles() {
    //     for (auto item : myEdges) {
    //         item.second->computeAngle();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_angles(&mut self) {
        todo!("port NBEdgeCont::computeAngles")
    }

    /// C++: `std::set<std::string> NBEdgeCont::getUsedTypes() const` [const]
    /// return all edge types in used
    // std::set<std::string>
    // NBEdgeCont::getUsedTypes() const {
    //     std::set<std::string> result;
    //     for (auto item : myEdges) {
    //         if (item.second->getTypeID() != "") {
    //             result.insert(item.second->getTypeID());
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_used_types(&self) -> BTreeSet<String> {
        todo!("port NBEdgeCont::getUsedTypes")
    }

    /// C++: `int NBEdgeCont::removeEdgesBySpeed(NBDistrictCont &)`
    /// return number of edges removed
    // int
    // NBEdgeCont::removeEdgesBySpeed(NBDistrictCont& dc) {
    //     EdgeSet toRemove;
    //     for (const auto& item : myEdges) {
    //         NBEdge* edge = item.second;
    //         // remove edges which allow a speed below a set one (set using "keep-edges.min-speed")
    //         // explicit whitelist overrides removal
    //         if (edge->getSpeed() < myEdgesMinSpeed && myEdges2Keep.count(edge->getID()) == 0) {
    //             toRemove.insert(edge);
    //         }
    //     }
    //     const int numRemoved = (int)toRemove.size();
    //     for (NBEdge* edge : toRemove) {
    //         erase(dc, edge);
    //     }
    //     return numRemoved;
    // }
    #[allow(unused_variables)]
    pub fn remove_edges_by_speed(&mut self, dc: &mut NBDistrictCont) -> i32 {
        todo!("port NBEdgeCont::removeEdgesBySpeed")
    }

    /// C++: `int NBEdgeCont::removeEdgesByPermissions(NBDistrictCont &)`
    // int
    // NBEdgeCont::removeEdgesByPermissions(NBDistrictCont& dc) {
    //     EdgeSet toRemove;
    //     for (const auto& item : myEdges) {
    //         NBEdge* edge = item.second;
    //         // check whether the edge shall be removed because it does not allow any of the wished classes
    //         // explicit whitelist overrides removal
    //         if (myVehicleClasses2Keep != 0 && (myVehicleClasses2Keep & edge->getPermissions()) == 0 && myEdges2Keep.count(edge->getID()) == 0) {
    //             toRemove.insert(edge);
    //         }
    //         // check whether the edge shall be removed due to allowing unwished classes only
    //         // explicit whitelist overrides removal
    //         if (myVehicleClasses2Remove != 0 && (myVehicleClasses2Remove | edge->getPermissions()) == myVehicleClasses2Remove && myEdges2Keep.count(edge->getID()) == 0) {
    //             toRemove.insert(edge);
    //         }
    //     }
    //     const int numRemoved = (int)toRemove.size();
    //     for (NBEdge* edge : toRemove) {
    //         erase(dc, edge);
    //     }
    //     return numRemoved;
    // }
    #[allow(unused_variables)]
    pub fn remove_edges_by_permissions(&mut self, dc: &mut NBDistrictCont) -> i32 {
        todo!("port NBEdgeCont::removeEdgesByPermissions")
    }

    /// C++: `int NBEdgeCont::removeLanesByWidth(NBDistrictCont &, const double)`
    // int
    // NBEdgeCont::removeLanesByWidth(NBDistrictCont& dc, const double minWidth) {
    //     EdgeSet toRemove;
    //     for (auto item : myEdges) {
    //         NBEdge* const edge = item.second;
    //         std::vector<int> indices;
    //         int idx = 0;
    //         for (const auto& lane : edge->getLanes()) {
    //             if (lane.width != NBEdge::UNSPECIFIED_WIDTH && lane.width < minWidth) {
    //                 indices.push_back(idx);
    //             }
    //             idx++;
    //         }
    //         if ((int)indices.size() == edge->getNumLanes()) {
    //             toRemove.insert(edge);
    //         } else {
    //             std::reverse(indices.begin(), indices.end());
    //             for (const int i : indices) {
    //                 edge->deleteLane(i, false, true);
    //             }
    //         }
    //     }
    //     int numRemoved = 0;
    //     for (NBEdge* edge : toRemove) {
    //         // explicit whitelist overrides removal
    //         if (myEdges2Keep.size() == 0 || myEdges2Keep.count(edge->getID()) == 0) {
    //             extract(dc, edge);
    //             numRemoved++;
    //         }
    //     }
    //     return numRemoved;
    // }
    #[allow(unused_variables)]
    pub fn remove_lanes_by_width(&mut self, dc: &mut NBDistrictCont, minWidth: f64) -> i32 {
        todo!("port NBEdgeCont::removeLanesByWidth")
    }

    /// C++: `int NBEdgeCont::attachRemoved(NBNodeCont &, NBDistrictCont &, const double)`
    /// return number of edges split
    // int
    // NBEdgeCont::attachRemoved(NBNodeCont& nc, NBDistrictCont& dc, const double maxDist) {
    //     int numSplit = 0;
    //     std::map<std::string, std::vector<std::string> > node2edge;
    //     for (auto item : myEdges) {
    //         if (item.second->hasParameter(SUMO_PARAM_REMOVED_NODES)) {
    //             for (std::string& nodeID : StringTokenizer(item.second->getParameter(SUMO_PARAM_REMOVED_NODES)).getVector()) {
    //                 node2edge[nodeID].push_back(item.first);
    //             }
    //         }
    //     }
    //     for (auto item : nc) {
    //         NBNode* n = item.second;
    //         auto itRN = node2edge.find(n->getID());
    //         if (itRN != node2edge.end()) {
    //             bool rebuildConnections = false;
    //             // make a copy because we modify the original
    //             std::vector<std::string> edgeIDs = itRN->second;
    //             for (const std::string& eID : edgeIDs) {
    //                 NBEdge* edge = retrieve(eID);
    //                 assert(edge != nullptr);
    //                 const double dist = edge->getGeometry().distance2D(n->getPosition(), true);
    //                 if (dist != GeomHelper::INVALID_OFFSET && dist <= maxDist) {
    //                     std::string idAfter = edge->getID();
    //                     int index = 1;
    //                     size_t spos = idAfter.find("#");
    //                     if (spos != std::string::npos && spos > 1) {
    //                         idAfter = idAfter.substr(0, spos);
    //                     }
    //                     while (retrieve(idAfter + "#" + toString(index), true) != nullptr) {
    //                         index++;
    //                     }
    //                     idAfter += "#" + toString(index);
    //                     const bool ok = splitAt(dc, edge, n, edge->getID(), idAfter, edge->getNumLanes(), edge->getNumLanes());
    //                     if (ok) {
    //                         rebuildConnections = true;
    //                         numSplit++;
    //                         NBEdge* secondEdge = retrieve(eID); // original was extracted on splitting
    //                         for (std::string& nodeID : StringTokenizer(secondEdge->getParameter(SUMO_PARAM_REMOVED_NODES)).getVector()) {
    //                             node2edge[nodeID].push_back(idAfter);
    //                         }
    //                     }
    //                 }
    //             }
    //             if (rebuildConnections) {
    //                 for (NBEdge* e : n->getIncomingEdges()) {
    //                     e->invalidateConnections(true);
    //                 }
    //             }
    //         }
    //     }
    //     return numSplit;
    // }
    #[allow(unused_variables)]
    pub fn attach_removed(&mut self, nc: &mut NBNodeCont, dc: &mut NBDistrictCont, maxDist: f64) -> i32 {
        todo!("port NBEdgeCont::attachRemoved")
    }

    /// C++: `double NBEdgeCont::formFactor(const EdgeVector &)` [static, private]
    /// compute the form factor for a loop of edges
    // double
    // NBEdgeCont::formFactor(const EdgeVector& loopEdges) {
    //     // A circle (which maximizes area per circumference) has a formfactor of 1, non-circular shapes have a smaller value
    //     PositionVector points;
    //     for (EdgeVector::const_iterator it = loopEdges.begin(); it != loopEdges.end(); ++it) {
    //         points.append((*it)->getGeometry());
    //     }
    //     double circumference = points.length2D();
    //     return 4 * M_PI * points.area() / (circumference * circumference);
    // }
    #[allow(unused_variables)]
    pub fn form_factor(loopEdges: &EdgeVector) -> f64 {
        todo!("port NBEdgeCont::formFactor")
    }

    /// C++: `void NBEdgeCont::removeRoundaboutEdges(const EdgeSet &, std::set<EdgeSet> &)` [private]
    /// remove roundabout edges
    // void
    // NBEdgeCont::removeRoundaboutEdges(const EdgeSet& toRemove, std::set<EdgeSet>& roundabouts) {
    //     // members of a set are constant so we have to do some tricks
    //     std::vector<EdgeSet> rList;
    //     for (const EdgeSet& r : roundabouts) {
    //         EdgeSet r2;
    //         std::set_difference(r.begin(), r.end(), toRemove.begin(), toRemove.end(), std::inserter(r2, r2.end()));
    //         rList.push_back(r2);
    //     }
    //     roundabouts.clear();
    //     roundabouts.insert(rList.begin(), rList.end());
    // }
    #[allow(unused_variables)]
    pub fn remove_roundabout_edges_2(&mut self, toRemove: &EdgeSet, roundabouts: &mut BTreeSet<EdgeSet>) {
        todo!("port NBEdgeCont::removeRoundaboutEdges")
    }

    /// C++: `void NBEdgeCont::NBEdgeCont(const NBEdgeCont &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NBEdgeCont) -> Self {
        todo!("port NBEdgeCont::NBEdgeCont")
    }

    /// C++: `NBEdgeCont & NBEdgeCont::operator=(const NBEdgeCont &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBEdgeCont) -> &mut NBEdgeCont {
        todo!("port NBEdgeCont::operator=")
    }

}

/// A structure representing a connection between two lanes
/// C++ class `NBEdgeCont::PostProcessConnection` (netbuild/NBEdgeCont.h)
pub struct NBEdgeCont_PostProcessConnection {
    /// The id of the edge the connection starts at
    pub from: String,
    /// The number of the lane the connection starts at
    pub fromLane: i32,
    /// The id of the edge the connection ends at
    pub to: String,
    /// The number of the lane the connection ends at
    pub toLane: i32,
    /// Whether the connection may be passed without braking
    pub mayDefinitelyPass: bool,
    /// Whether the connection may be passed without braking
    pub keepClear: KeepClear,
    /// custom position for internal junction on this connection
    pub contPos: f64,
    /// custom foe visiblity for connection
    pub visibility: f64,
    /// custom speed for connection
    pub speed: f64,
    /// custom friction for connection
    pub friction: f64,
    /// custom length for connection
    pub customLength: f64,
    /// custom shape for connection
    pub customShape: PositionVector,
    /// whether this connection shall not be controlled by a traffic light
    pub uncontrolled: bool,
    /// custom permissions for connection
    pub permissions: SVCPermissions,
    /// whether this connection is an indirect left turn
    pub indirectLeft: bool,
    /// custom edge type
    pub edgeType: String,
    /// custom lane changing permissions for connection
    pub changeLeft: SVCPermissions,
    /// custom lane changing permissions for connection
    pub changeRight: SVCPermissions,
    /// whether a failure to set this connection is a warning or an error
    pub warnOnly: bool,
}

impl NBEdgeCont_PostProcessConnection {
    /// C++: `void PostProcessConnection::PostProcessConnection(const std::string &, int, const std::string &, int, bool, KeepClear, double, double, double, double, double, const PositionVector &, bool, bool, SVCPermissions, bool, const std::string &, SVCPermissions, SVCPermissions)`
    /// Constructor
    // PostProcessConnection(const std::string& from_, int fromLane_, const std::string& to_, int toLane_,
    //                               bool mayDefinitelyPass_, KeepClear keepClear_, double contPos_, double visibility_, double speed_,
    //                               double friction_, double length_,
    //                               const PositionVector& customShape_,
    //                               bool uncontrolled_,
    //                               bool warnOnly_,
    //                               SVCPermissions permissions_,
    //                               bool indirectLeft_,
    //                               const std::string& edgeType_,
    //                               SVCPermissions changeLeft_,
    //                               SVCPermissions changeRight_) :
    //             from(from_), fromLane(fromLane_), to(to_), toLane(toLane_), mayDefinitelyPass(mayDefinitelyPass_), keepClear(keepClear_), contPos(contPos_),
    //             visibility(visibility_),
    //             speed(speed_),
    //             friction(friction_),
    //             customLength(length_),
    //             customShape(customShape_),
    //             uncontrolled(uncontrolled_),
    //             permissions(permissions_),
    //             indirectLeft(indirectLeft_),
    //             edgeType(edgeType_),
    //             changeLeft(changeLeft_),
    //             changeRight(changeRight_),
    //             warnOnly(warnOnly_)
    //         {}
    #[allow(unused_variables)]
    pub fn new(from_: &str, fromLane_: i32, to_: &str, toLane_: i32, mayDefinitelyPass_: bool, keepClear_: KeepClear, contPos_: f64, visibility_: f64, speed_: f64, friction_: f64, length_: f64, customShape_: &PositionVector, uncontrolled_: bool, warnOnly_: bool, permissions_: SVCPermissions, indirectLeft_: bool, edgeType_: &str, changeLeft_: SVCPermissions, changeRight_: SVCPermissions) -> Self {
        todo!("port PostProcessConnection::PostProcessConnection")
    }

}

/// A structure which describes changes of lane number or speed along the road
/// C++ class `NBEdgeCont::Split` (netbuild/NBEdgeCont.h)
pub struct NBEdgeCont_Split {
    /// The lanes after this change
    pub lanes: Vec<i32>,
    /// The position of this change
    pub pos: f64,
    /// The speed after this change
    pub speed: f64,
    /// The friction after this change
    pub friction: f64,
    /// The new node that is created for this split
    pub node: Ptr<NBNode>,
    /// The id for the edge before the split
    pub idBefore: String,
    /// The id for the edge after the split
    pub idAfter: String,
    /// the default node id
    pub nameID: String,
    /// lateral offset to edge geometry
    pub offset: f64,
    /// direction in which to apply the offset (used by netgenerate for lefthand networks)
    pub offsetFactor: i32,
}

impl NBEdgeCont_Split {
}

/// Sorts splits by their position (increasing)
/// C++ class `NBEdgeCont::split_sorter` (netbuild/NBEdgeCont.h)
pub struct NBEdgeCont_split_sorter {
}

impl NBEdgeCont_split_sorter {
    /// C++: `void split_sorter::split_sorter()`
    /// Constructor
    // explicit split_sorter() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {}
    }

    /// C++: `int split_sorter::operator()(const Split &, const Split &) const` [const]
    /// Comparing operator
    // int operator()(const Split& e1, const Split& e2) const {
    //             return e1.pos < e2.pos;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: &NBEdgeCont_Split, e2: &NBEdgeCont_Split) -> i32 {
        (e1.pos < e2.pos) as i32
    }

}

