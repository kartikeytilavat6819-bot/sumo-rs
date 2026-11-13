//! Port of dfrouter/RODFDetector.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::file_helpers::get_file_path;
use utils::common::rand_helper::with_default;
use utils::common::string_utils;
use utils::geom::NUMERICAL_EPS;
use utils::options::options_cont::{get_bool, get_float, get_string, is_default, is_set};
use utils::xml::sumo_xml_definitions::{get_edge_id_from_lane, get_index_from_lane};

// C++ VTYPEPARS_*_SET bits of SUMOVTypeParameter::parametersSet (private in sumov_type_parameter.rs)
const VTYPEPARS_SPEEDFACTOR_SET: i64 = 1 << 4;
const VTYPEPARS_VEHICLECLASS_SET: i64 = 1 << 7;

/// C++ MAX2(a, b): a > b ? a : b
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// C++ enum `RODFDetectorType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum RODFDetectorType {
    TYPE_NOT_DEFINED = 0,
    DISCARDED_DETECTOR = 1,
    BETWEEN_DETECTOR = 2,
    SOURCE_DETECTOR = 3,
    SINK_DETECTOR = 4,
}

/// Class representing a detector within the DFROUTER
/// C++ class `RODFDetector` (dfrouter/RODFDetector.h)
#[repr(C)]
pub struct RODFDetector {
    /// C++ base class `Named`
    pub base: Named,
    pub myLaneID: String,
    pub myPosition: f64,
    pub myType: RODFDetectorType,
    pub myRoutes: Ptr<RODFRouteCont>,
    pub myPriorDetectors: BTreeSet<Ptr<RODFDetector>>,
    pub myFollowingDetectors: BTreeSet<Ptr<RODFDetector>>,
    pub mySplitProbabilities: Vec<BTreeMap<Ptr<RODFEdge>, f64>>,
    pub myRoute2Edge: BTreeMap<String, Ptr<RODFEdge>>,
}

impl std::ops::Deref for RODFDetector {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for RODFDetector {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl RODFDetector {
    /// C++: `void RODFDetector::RODFDetector(const std::string &, const std::string &, double, const RODFDetectorType)`
    /// Constructor
    // RODFDetector::RODFDetector(const std::string& id, const std::string& laneID,
    //                            double pos, const RODFDetectorType type)
    //     : Named(id), myLaneID(laneID), myPosition(pos), myType(type), myRoutes(nullptr) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, laneID: &str, pos: f64, type_: RODFDetectorType) -> Self {
        RODFDetector {
            base: Named::new(id),
            myLaneID: laneID.to_string(),
            myPosition: pos,
            myType: type_,
            myRoutes: Ptr::null(),
            myPriorDetectors: BTreeSet::new(),
            myFollowingDetectors: BTreeSet::new(),
            mySplitProbabilities: Vec::new(),
            myRoute2Edge: BTreeMap::new(),
        }
    }

    /// C++: `void RODFDetector::RODFDetector(const std::string &, const RODFDetector &)`
    /// Constructor
    // RODFDetector::RODFDetector(const std::string& id, const RODFDetector& f)
    //     : Named(id), myLaneID(f.myLaneID), myPosition(f.myPosition),
    //       myType(f.myType), myRoutes(nullptr) {
    //     if (f.myRoutes != nullptr) {
    //         myRoutes = new RODFRouteCont(*(f.myRoutes));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, f: &RODFDetector) -> Self {
        let mut routes = Ptr::null();
        if !f.myRoutes.is_null() {
            let src = f.myRoutes.m();
            routes = Ptr::from_raw(Box::into_raw(Box::new(RODFRouteCont {
                myRoutes: src
                    .myRoutes
                    .iter()
                    .map(|d| RODFRouteDesc {
                        edges2Pass: d.edges2Pass.clone(),
                        routename: d.routename.clone(),
                        duration_2: d.duration_2,
                        distance: d.distance,
                        passedNo: d.passedNo,
                        endDetectorEdge: d.endDetectorEdge,
                        lastDetectorEdge: d.lastDetectorEdge,
                        distance2Last: d.distance2Last,
                        duration2Last: d.duration2Last,
                        overallProb: d.overallProb,
                        factor: d.factor,
                    })
                    .collect(),
                myConnectionOccurrences: src.myConnectionOccurrences.clone(),
            })));
        }
        RODFDetector {
            base: Named::new(id),
            myLaneID: f.myLaneID.clone(),
            myPosition: f.myPosition,
            myType: f.myType,
            myRoutes: routes,
            myPriorDetectors: BTreeSet::new(),
            myFollowingDetectors: BTreeSet::new(),
            mySplitProbabilities: Vec::new(),
            myRoute2Edge: BTreeMap::new(),
        }
    }

    /// C++: `void RODFDetector::~RODFDetector() noexcept` [virtual]
    /// Destructor
    // RODFDetector::~RODFDetector() {
    //     delete myRoutes;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myRoutes.is_null() {
            drop(unsafe { Box::from_raw(self.myRoutes.as_raw()) });
        }
    }

    /// C++: `const std::string & RODFDetector::getLaneID() const` [const]
    /// Returns the id of the lane this detector is placed on
    // const std::string& getLaneID() const {
    //         return myLaneID;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_id(&self) -> &str {
        &self.myLaneID
    }

    /// C++: `std::string RODFDetector::getEdgeID() const` [const]
    /// Returns the id of the edge this detector is placed on
    // std::string
    // RODFDetector::getEdgeID() const {
    //     return SUMOXMLDefinitions::getEdgeIDFromLane(myLaneID);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(&self) -> String {
        get_edge_id_from_lane(&self.myLaneID)
    }

    /// C++: `double RODFDetector::getPos() const` [const]
    /// Returns the position at which the detector lies
    // double getPos() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_pos(&self) -> f64 {
        self.myPosition
    }

    /// C++: `RODFDetectorType RODFDetector::getType() const` [const]
    /// Returns the type of the detector
    // RODFDetectorType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> RODFDetectorType {
        self.myType
    }

    /// C++: `void RODFDetector::setType(RODFDetectorType)`
    /// @}
    // void
    // RODFDetector::setType(RODFDetectorType type) {
    //     myType = type;
    // }
    #[allow(unused_variables)]
    pub fn set_type(&mut self, type_: RODFDetectorType) {
        self.myType = type_;
    }

    /// C++: `void RODFDetector::addRoute(RODFRouteDesc &)`
    // void
    // RODFDetector::addRoute(RODFRouteDesc& nrd) {
    //     if (myRoutes == nullptr) {
    //         myRoutes = new RODFRouteCont();
    //     }
    //     myRoutes->addRouteDesc(nrd);
    // }
    #[allow(unused_variables)]
    pub fn add_route(&mut self, nrd: &mut RODFRouteDesc) {
        if self.myRoutes.is_null() {
            self.myRoutes = Ptr::from_raw(Box::into_raw(Box::new(RODFRouteCont::new())));
        }
        self.myRoutes.m().add_route_desc(nrd);
    }

    /// C++: `void RODFDetector::addRoutes(RODFRouteCont *)`
    // void
    // RODFDetector::addRoutes(RODFRouteCont* routes) {
    //     delete myRoutes;
    //     myRoutes = routes;
    // }
    #[allow(unused_variables)]
    pub fn add_routes(&mut self, routes: Ptr<RODFRouteCont>) {
        if !self.myRoutes.is_null() {
            drop(unsafe { Box::from_raw(self.myRoutes.as_raw()) });
        }
        self.myRoutes = routes;
    }

    /// C++: `bool RODFDetector::hasRoutes() const` [const]
    // bool
    // RODFDetector::hasRoutes() const {
    //     return myRoutes != nullptr && myRoutes->get().size() != 0;
    // }
    #[allow(unused_variables)]
    pub fn has_routes(&self) -> bool {
        !self.myRoutes.is_null() && self.myRoutes.m().get().len() != 0
    }

    /// C++: `const std::vector<RODFRouteDesc> & RODFDetector::getRouteVector() const` [const]
    // const std::vector<RODFRouteDesc>&
    // RODFDetector::getRouteVector() const {
    //     return myRoutes->get();
    // }
    #[allow(unused_variables)]
    pub fn get_route_vector(&self) -> &Vec<RODFRouteDesc> {
        self.myRoutes.m().get()
    }

    /// C++: `void RODFDetector::addPriorDetector(const RODFDetector *)`
    // void
    // RODFDetector::addPriorDetector(const RODFDetector* det) {
    //     myPriorDetectors.insert(det);
    // }
    #[allow(unused_variables)]
    pub fn add_prior_detector(&mut self, det: Ptr<RODFDetector>) {
        self.myPriorDetectors.insert(det);
    }

    /// C++: `void RODFDetector::addFollowingDetector(const RODFDetector *)`
    // void
    // RODFDetector::addFollowingDetector(const RODFDetector* det) {
    //     myFollowingDetectors.insert(det);
    // }
    #[allow(unused_variables)]
    pub fn add_following_detector(&mut self, det: Ptr<RODFDetector>) {
        self.myFollowingDetectors.insert(det);
    }

    /// C++: `const std::set<const RODFDetector *> & RODFDetector::getPriorDetectors() const` [const]
    // const std::set<const RODFDetector*>&
    // RODFDetector::getPriorDetectors() const {
    //     return myPriorDetectors;
    // }
    #[allow(unused_variables)]
    pub fn get_prior_detectors(&self) -> &BTreeSet<Ptr<RODFDetector>> {
        &self.myPriorDetectors
    }

    /// C++: `const std::set<const RODFDetector *> & RODFDetector::getFollowerDetectors() const` [const]
    // const std::set<const RODFDetector*>&
    // RODFDetector::getFollowerDetectors() const {
    //     return myFollowingDetectors;
    // }
    #[allow(unused_variables)]
    pub fn get_follower_detectors(&self) -> &BTreeSet<Ptr<RODFDetector>> {
        &self.myFollowingDetectors
    }

    /// C++: `bool RODFDetector::writeEmitterDefinition(const std::string &, const std::map<SUMOTime, RandomDistributor<int> *> &, const RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime, bool, double, bool, double) const` [const]
    /// @{
    // bool
    // RODFDetector::writeEmitterDefinition(const std::string& file,
    //                                      const std::map<SUMOTime, RandomDistributor<int>* >& dists,
    //                                      const RODFDetectorFlows& flows,
    //                                      SUMOTime startTime, SUMOTime endTime,
    //                                      SUMOTime stepOffset,
    //                                      bool includeUnusedRoutes,
    //                                      double scale,
    //                                      bool insertionsOnly,
    //                                      double defaultSpeed) const {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (getType() != SOURCE_DETECTOR) {
    //         out.writeXMLHeader("additional", "additional_file.xsd");
    //     }
    //     // routes
    //     if (myRoutes != nullptr && myRoutes->get().size() != 0) {
    //         const std::vector<RODFRouteDesc>& routes = myRoutes->get();
    //         out.openTag(SUMO_TAG_ROUTE_DISTRIBUTION).writeAttr(SUMO_ATTR_ID, myID);
    //         bool isEmptyDist = true;
    //         for (std::vector<RODFRouteDesc>::const_iterator i = routes.begin(); i != routes.end(); ++i) {
    //             if ((*i).overallProb > 0) {
    //                 isEmptyDist = false;
    //             }
    //         }
    //         for (std::vector<RODFRouteDesc>::const_iterator i = routes.begin(); i != routes.end(); ++i) {
    //             if (isEmptyDist) {
    //                 out.openTag(SUMO_TAG_ROUTE).writeAttr(SUMO_ATTR_REFID, (*i).routename).writeAttr(SUMO_ATTR_PROB, 1.0).closeTag();
    //             } else if ((*i).overallProb > 0 || includeUnusedRoutes) {
    //                 out.openTag(SUMO_TAG_ROUTE).writeAttr(SUMO_ATTR_REFID, (*i).routename).writeAttr(SUMO_ATTR_PROB, (*i).overallProb).closeTag();
    //             }
    //         }
    //         out.closeTag(); // routeDistribution
    //     } else {
    //         WRITE_ERRORF(TL("Detector '%' has no routes!?"), getID());
    //         return false;
    //     }
    //     // insertions
    //     int vehicleIndex = 0;
    //     if (insertionsOnly || flows.knows(myID)) {
    //         // get the flows for this detector
    //         const std::vector<FlowDef>& mflows = flows.getFlowDefs(myID);
    //         // go through the simulation seconds
    //         int index = 0;
    //         for (SUMOTime time = startTime; time < endTime; time += stepOffset, index++) {
    //             // get own (departure flow)
    //             assert(index < (int)mflows.size());
    //             const FlowDef& srcFD = mflows[index];  // !!! check stepOffset
    //             // get flows at end
    //             RandomDistributor<int>* destDist = dists.find(time) != dists.end() ? dists.find(time)->second : 0;
    //             // go through the cars
    //             const int numCars = (int)((srcFD.qPKW + srcFD.qLKW) * scale);
    //
    //
    //             std::vector<SUMOTime> departures;
    //             if (oc.getBool("randomize-flows")) {
    //                 for (int i = 0; i < numCars; ++i) {
    //                     departures.push_back(time + RandHelper::rand(stepOffset));
    //                 }
    //                 std::sort(departures.begin(), departures.end());
    //             } else {
    //                 for (int i = 0; i < numCars; ++i) {
    //                     departures.push_back(time + (stepOffset * i / numCars));
    //                 }
    //             }
    //
    //             for (int car = 0; car < numCars; ++car) {
    //                 // get the vehicle parameter
    //                 double v = -1;
    //                 std::string vtype;
    //                 int destIndex = -1;
    //                 if (destDist != nullptr) {
    //                     if (destDist->getOverallProb() > 0) {
    //                         destIndex = destDist->get();
    //                     } else if (myRoutes->get().size() > 0) {
    //                         // equal probabilities. see writeEmitterDefinition()
    //                         destIndex = RandHelper::rand((int)myRoutes->get().size());
    //                     }
    //                 }
    //                 if (srcFD.isLKW >= 1) {
    //                     srcFD.isLKW = srcFD.isLKW - 1.;
    //                     v = srcFD.vLKW;
    //                     vtype = "LKW";
    //                 } else {
    //                     v = srcFD.vPKW;
    //                     vtype = "PKW";
    //                 }
    //                 // compute insertion speed
    //                 if (v <= 0 || v > 250) {
    //                     v = defaultSpeed;
    //                 } else {
    //                     v /= 3.6;
    //                 }
    //                 // compute the departure time
    //                 const SUMOTime ctime = departures[car];
    //
    //                 // write
    //                 out.openTag(SUMO_TAG_VEHICLE);
    //                 out.writeAttr(SUMO_ATTR_ID, myID + "." + toString(vehicleIndex));
    //                 if (oc.getBool("vtype")) {
    //                     out.writeAttr(SUMO_ATTR_TYPE, vtype);
    //                 }
    //                 out.writeAttr(SUMO_ATTR_DEPART, time2string(ctime));
    //                 if (oc.isSet("departlane")) {
    //                     out.writeNonEmptyAttr(SUMO_ATTR_DEPARTLANE, oc.getString("departlane"));
    //                 } else {
    //                     out.writeAttr(SUMO_ATTR_DEPARTLANE, SUMOXMLDefinitions::getIndexFromLane(myLaneID));
    //                 }
    //                 if (oc.isSet("departpos")) {
    //                     std::string posDesc = oc.getString("departpos");
    //                     if (posDesc.substr(0, 8) == "detector") {
    //                         double position = myPosition;
    //                         if (posDesc.length() > 8) {
    //                             if (posDesc[8] == '+') {
    //                                 position += StringUtils::toDouble(posDesc.substr(9));
    //                             } else if (posDesc[8] == '-') {
    //                                 position -= StringUtils::toDouble(posDesc.substr(9));
    //                             } else {
    //                                 throw NumberFormatException("");
    //                             }
    //                         }
    //                         out.writeAttr(SUMO_ATTR_DEPARTPOS, position);
    //                     } else {
    //                         out.writeNonEmptyAttr(SUMO_ATTR_DEPARTPOS, posDesc);
    //                     }
    //                 } else {
    //                     out.writeAttr(SUMO_ATTR_DEPARTPOS, myPosition);
    //                 }
    //                 if (oc.isSet("departspeed")) {
    //                     out.writeNonEmptyAttr(SUMO_ATTR_DEPARTSPEED, oc.getString("departspeed"));
    //                 } else {
    //                     out.writeAttr(SUMO_ATTR_DEPARTSPEED, v);
    //                 }
    //                 if (oc.isSet("arrivallane")) {
    //                     out.writeNonEmptyAttr(SUMO_ATTR_ARRIVALLANE, oc.getString("arrivallane"));
    //                 }
    //                 if (oc.isSet("arrivalpos")) {
    //                     out.writeNonEmptyAttr(SUMO_ATTR_ARRIVALPOS, oc.getString("arrivalpos"));
    //                 }
    //                 if (oc.isSet("arrivalspeed")) {
    //                     out.writeNonEmptyAttr(SUMO_ATTR_ARRIVALSPEED, oc.getString("arrivalspeed"));
    //                 }
    //                 if (destIndex >= 0) {
    //                     out.writeAttr(SUMO_ATTR_ROUTE, myRoutes->get()[destIndex].routename);
    //                 } else {
    //                     out.writeAttr(SUMO_ATTR_ROUTE, myID);
    //                 }
    //                 out.closeTag();
    //                 srcFD.isLKW += srcFD.fLKW;
    //                 vehicleIndex++;
    //             }
    //         }
    //     }
    //     if (getType() != SOURCE_DETECTOR) {
    //         out.close();
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn write_emitter_definition(&self, file: &str, dists: &BTreeMap<SUMOTime, Ptr<RandomDistributor<i32>>>, flows: &RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime, includeUnusedRoutes: bool, scale: f64, insertionsOnly: bool, defaultSpeed: f64) -> bool {
        let out = Ptr::from_raw(OutputDevice::get_device(file, true).unwrap_or_else(|e| panic!("{e}")).as_ptr());
        if self.get_type() != RODFDetectorType::SOURCE_DETECTOR {
            out.m().write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        }
        // routes
        if !self.myRoutes.is_null() && self.myRoutes.m().get().len() != 0 {
            let routes: &Vec<RODFRouteDesc> = self.myRoutes.m().get();
            out.m().open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION).write_attr(SUMO_ATTR_ID, self.get_id());
            let mut isEmptyDist = true;
            for i in routes.iter() {
                if i.overallProb > 0. {
                    isEmptyDist = false;
                }
            }
            for i in routes.iter() {
                if isEmptyDist {
                    out.m().open_tag_id(SUMO_TAG_ROUTE).write_attr(SUMO_ATTR_REFID, &i.routename).write_attr(SUMO_ATTR_PROB, &1.0_f64).close_tag("");
                } else if i.overallProb > 0. || includeUnusedRoutes {
                    out.m().open_tag_id(SUMO_TAG_ROUTE).write_attr(SUMO_ATTR_REFID, &i.routename).write_attr(SUMO_ATTR_PROB, &i.overallProb).close_tag("");
                }
            }
            out.m().close_tag(""); // routeDistribution
        } else {
            write_errorf!("Detector '%' has no routes!?", self.get_id());
            return false;
        }
        // insertions
        let mut vehicleIndex: i32 = 0;
        if insertionsOnly || flows.knows(self.get_id()) {
            // get the flows for this detector
            let mflows: &Vec<FlowDef> = flows.get_flow_defs(self.get_id());
            // go through the simulation seconds
            let mut index: usize = 0;
            let mut time = startTime;
            while time < endTime {
                // get own (departure flow)
                debug_assert!(index < mflows.len());
                let srcFD = Ptr::new(&mflows[index]); // !!! check stepOffset
                // get flows at end
                let destDist: Option<Ptr<RandomDistributor<i32>>> = dists.get(&time).copied();
                // go through the cars
                let numCars = ((srcFD.qPKW + srcFD.qLKW) * scale) as i32;

                let mut departures: Vec<SUMOTime> = Vec::new();
                if get_bool("randomize-flows") {
                    for _i in 0..numCars {
                        departures.push(time + with_default(|r| r.rand_i64(stepOffset)));
                    }
                    departures.sort();
                } else {
                    for i in 0..numCars {
                        departures.push(time + (stepOffset * i as SUMOTime / numCars as SUMOTime));
                    }
                }

                for car in 0..numCars {
                    // get the vehicle parameter
                    let mut v: f64;
                    let vtype: &str;
                    let mut destIndex: i32 = -1;
                    if let Some(destDist) = destDist {
                        if destDist.get_overall_prob() > 0. {
                            destIndex = (*destDist).get(None).unwrap_or_else(|e| panic!("{e}"));
                        } else if self.myRoutes.m().get().len() > 0 {
                            // equal probabilities. see writeEmitterDefinition()
                            destIndex = with_default(|r| r.rand_int(self.myRoutes.m().get().len() as i32));
                        }
                    }
                    if srcFD.isLKW >= 1. {
                        srcFD.m().isLKW = srcFD.isLKW - 1.;
                        v = srcFD.vLKW;
                        vtype = "LKW";
                    } else {
                        v = srcFD.vPKW;
                        vtype = "PKW";
                    }
                    // compute insertion speed
                    if v <= 0. || v > 250. {
                        v = defaultSpeed;
                    } else {
                        v /= 3.6;
                    }
                    // compute the departure time
                    let ctime: SUMOTime = departures[car as usize];

                    // write
                    out.m().open_tag_id(SUMO_TAG_VEHICLE);
                    out.m().write_attr(SUMO_ATTR_ID, &format!("{}.{}", self.get_id(), vehicleIndex));
                    if get_bool("vtype") {
                        out.m().write_attr(SUMO_ATTR_TYPE, vtype);
                    }
                    out.m().write_attr(SUMO_ATTR_DEPART, &time2string(ctime));
                    if is_set("departlane") {
                        out.m().write_non_empty_attr(SUMO_ATTR_DEPARTLANE, &get_string("departlane"));
                    } else {
                        out.m().write_attr(SUMO_ATTR_DEPARTLANE, &get_index_from_lane(&self.myLaneID).unwrap_or_else(|e| panic!("{e}")));
                    }
                    if is_set("departpos") {
                        let posDesc = get_string("departpos");
                        if posDesc.get(0..8) == Some("detector") {
                            let mut position = self.myPosition;
                            if posDesc.len() > 8 {
                                if posDesc.as_bytes()[8] == b'+' {
                                    position += string_utils::to_double(&posDesc[9..]).unwrap_or_else(|e| panic!("{e}"));
                                } else if posDesc.as_bytes()[8] == b'-' {
                                    position -= string_utils::to_double(&posDesc[9..]).unwrap_or_else(|e| panic!("{e}"));
                                } else {
                                    panic!("{}", ProcessError::NumberFormat(String::new()));
                                }
                            }
                            out.m().write_attr(SUMO_ATTR_DEPARTPOS, &position);
                        } else {
                            out.m().write_non_empty_attr(SUMO_ATTR_DEPARTPOS, &posDesc);
                        }
                    } else {
                        out.m().write_attr(SUMO_ATTR_DEPARTPOS, &self.myPosition);
                    }
                    if is_set("departspeed") {
                        out.m().write_non_empty_attr(SUMO_ATTR_DEPARTSPEED, &get_string("departspeed"));
                    } else {
                        out.m().write_attr(SUMO_ATTR_DEPARTSPEED, &v);
                    }
                    if is_set("arrivallane") {
                        out.m().write_non_empty_attr(SUMO_ATTR_ARRIVALLANE, &get_string("arrivallane"));
                    }
                    if is_set("arrivalpos") {
                        out.m().write_non_empty_attr(SUMO_ATTR_ARRIVALPOS, &get_string("arrivalpos"));
                    }
                    if is_set("arrivalspeed") {
                        out.m().write_non_empty_attr(SUMO_ATTR_ARRIVALSPEED, &get_string("arrivalspeed"));
                    }
                    if destIndex >= 0 {
                        out.m().write_attr(SUMO_ATTR_ROUTE, &self.myRoutes.m().get()[destIndex as usize].routename);
                    } else {
                        out.m().write_attr(SUMO_ATTR_ROUTE, self.get_id());
                    }
                    out.m().close_tag("");
                    srcFD.m().isLKW += srcFD.fLKW;
                    vehicleIndex += 1;
                }
                index += 1;
                time += stepOffset;
            }
        }
        if self.get_type() != RODFDetectorType::SOURCE_DETECTOR {
            out.m().close();
        }
        true
    }

    /// C++: `bool RODFDetector::writeRoutes(std::vector<std::string> &, OutputDevice &)`
    // bool
    // RODFDetector::writeRoutes(std::vector<std::string>& saved,
    //                           OutputDevice& out) {
    //     if (myRoutes != nullptr) {
    //         return myRoutes->save(saved, "", out);
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn write_routes(&mut self, saved: &mut Vec<String>, out: &mut OutputDevice) -> bool {
        if !self.myRoutes.is_null() {
            return self.myRoutes.m().save(saved, "", out);
        }
        false
    }

    /// C++: `void RODFDetector::writeSingleSpeedTrigger(const std::string &, const RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime, double)`
    // void
    // RODFDetector::writeSingleSpeedTrigger(const std::string& file,
    //                                       const RODFDetectorFlows& flows,
    //                                       SUMOTime startTime, SUMOTime endTime,
    //                                       SUMOTime stepOffset, double defaultSpeed) {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     const std::vector<FlowDef>& mflows = flows.getFlowDefs(myID);
    //     int index = 0;
    //     for (SUMOTime t = startTime; t < endTime; t += stepOffset, index++) {
    //         assert(index < (int)mflows.size());
    //         const FlowDef& srcFD = mflows[index];
    //         double speed = MAX2(srcFD.vLKW, srcFD.vPKW);
    //         if (speed <= 0 || speed > 250) {
    //             speed = defaultSpeed;
    //         } else {
    //             speed = (double)(speed / 3.6);
    //         }
    //         out.openTag(SUMO_TAG_STEP).writeAttr(SUMO_ATTR_TIME, time2string(t)).writeAttr(SUMO_ATTR_SPEED, speed).closeTag();
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn write_single_speed_trigger(&mut self, file: &str, flows: &RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime, defaultSpeed: f64) {
        let out = Ptr::from_raw(OutputDevice::get_device(file, true).unwrap_or_else(|e| panic!("{e}")).as_ptr());
        out.m().write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        let mflows: &Vec<FlowDef> = flows.get_flow_defs(self.get_id());
        let mut index: usize = 0;
        let mut t = startTime;
        while t < endTime {
            debug_assert!(index < mflows.len());
            let srcFD = &mflows[index];
            let mut speed = if srcFD.vLKW > srcFD.vPKW { srcFD.vLKW } else { srcFD.vPKW };
            if speed <= 0. || speed > 250. {
                speed = defaultSpeed;
            } else {
                speed = speed / 3.6;
            }
            out.m().open_tag_id(SUMO_TAG_STEP).write_attr(SUMO_ATTR_TIME, &time2string(t)).write_attr(SUMO_ATTR_SPEED, &speed).close_tag("");
            index += 1;
            t += stepOffset;
        }
        out.m().close();
    }

    /// C++: `void RODFDetector::writeEndRerouterDetectors(const std::string &)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn write_end_rerouter_detectors(&mut self, file: &str) {
        // shortcut: C++ declares RODFDetector::writeEndRerouterDetectors but never defines it; the only caller is RODFDetectorCon::writeEndRerouterDetectors. Keep panicking if anything reaches it.
        panic!("RODFDetector::writeEndRerouterDetectors is declared but not defined in SUMO");
    }

    /// C++: `void RODFDetector::buildDestinationDistribution(const RODFDetectorCon &, SUMOTime, SUMOTime, SUMOTime, const RODFNet &, std::map<SUMOTime, RandomDistributor<int> *> &) const` [const]
    /// @}
    // void
    // RODFDetector::buildDestinationDistribution(const RODFDetectorCon& detectors,
    //         SUMOTime startTime, SUMOTime endTime, SUMOTime stepOffset,
    //         const RODFNet& net,
    //         std::map<SUMOTime, RandomDistributor<int>* >& into) const {
    //     if (myRoutes == nullptr) {
    //         if (myType != DISCARDED_DETECTOR && myType != BETWEEN_DETECTOR) {
    //             WRITE_ERRORF(TL("Missing routes for detector '%'."), myID);
    //         }
    //         return;
    //     }
    //     std::vector<RODFRouteDesc>& descs = myRoutes->get();
    //     // iterate through time (in output interval steps)
    //     for (SUMOTime time = startTime; time < endTime; time += stepOffset) {
    //         into[time] = new RandomDistributor<int>();
    //         std::map<ROEdge*, double> flowMap;
    //         // iterate through the routes
    //         int index = 0;
    //         for (std::vector<RODFRouteDesc>::iterator ri = descs.begin(); ri != descs.end(); ++ri, index++) {
    //             double prob = 1.;
    //             for (ROEdgeVector::iterator j = (*ri).edges2Pass.begin(); j != (*ri).edges2Pass.end() && prob > 0;) {
    //                 if (!net.hasDetector(*j)) {
    //                     ++j;
    //                     continue;
    //                 }
    //                 const RODFDetector& det = detectors.getAnyDetectorForEdge(static_cast<RODFEdge*>(*j));
    //                 const std::vector<std::map<RODFEdge*, double> >& probs = det.getSplitProbabilities();
    //                 if (probs.size() == 0) {
    //                     prob = 0;
    //                     ++j;
    //                     continue;
    //                 }
    //                 const std::map<RODFEdge*, double>& tprobs = probs[(int)((time - startTime) / stepOffset)];
    //                 RODFEdge* splitEdge = nullptr;
    //                 for (std::map<RODFEdge*, double>::const_iterator k = tprobs.begin(); k != tprobs.end(); ++k) {
    //                     if (find(j, (*ri).edges2Pass.end(), (*k).first) != (*ri).edges2Pass.end()) {
    //                         prob *= (*k).second;
    //                         splitEdge = (*k).first;
    //                         break;
    //                     }
    //                 }
    //                 if (splitEdge != nullptr) {
    //                     j = std::find(j, (*ri).edges2Pass.end(), splitEdge);
    //                 } else {
    //                     ++j;
    //                 }
    //             }
    //             into[time]->add(index, prob);
    //             (*ri).overallProb = prob;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_destination_distribution(&self, detectors: &RODFDetectorCon, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime, net: &RODFNet, into: &mut BTreeMap<SUMOTime, Ptr<RandomDistributor<i32>>>) {
        if self.myRoutes.is_null() {
            if self.myType != RODFDetectorType::DISCARDED_DETECTOR && self.myType != RODFDetectorType::BETWEEN_DETECTOR {
                write_errorf!("Missing routes for detector '%'.", self.get_id());
            }
            return;
        }
        let descs: &mut Vec<RODFRouteDesc> = self.myRoutes.m().get();
        // iterate through time (in output interval steps)
        let mut time = startTime;
        while time < endTime {
            let dist = Ptr::from_raw(Box::into_raw(Box::new(RandomDistributor::<i32>::default())));
            into.insert(time, dist);
            // iterate through the routes
            for (index, ri) in descs.iter_mut().enumerate() {
                let mut prob = 1.;
                let mut j: usize = 0;
                while j < ri.edges2Pass.len() && prob > 0. {
                    if !net.has_detector(ri.edges2Pass[j]) {
                        j += 1;
                        continue;
                    }
                    let det: &RODFDetector = detectors.get_any_detector_for_edge(ri.edges2Pass[j].cast::<RODFEdge>());
                    let probs = det.get_split_probabilities();
                    if probs.len() == 0 {
                        prob = 0.;
                        j += 1;
                        continue;
                    }
                    let tprobs = &probs[((time - startTime) / stepOffset) as usize];
                    // determinism: std::map<RODFEdge*, double> is ordered by address; BTreeMap<Ptr<_>> keeps that order
                    let mut splitEdge: Option<Ptr<RODFEdge>> = None;
                    for (k_edge, k_prob) in tprobs.iter() {
                        if ri.edges2Pass[j..].iter().any(|e| e.cast::<RODFEdge>() == *k_edge) {
                            prob *= *k_prob;
                            splitEdge = Some(*k_edge);
                            break;
                        }
                    }
                    match splitEdge {
                        Some(se) => {
                            j += ri.edges2Pass[j..].iter().position(|e| e.cast::<RODFEdge>() == se).unwrap();
                        }
                        None => {
                            j += 1;
                        }
                    }
                }
                dist.m().add(index as i32, prob, true);
                ri.overallProb = prob;
            }
            time += stepOffset;
        }
    }

    /// C++: `void RODFDetector::computeSplitProbabilities(const RODFNet *, const RODFDetectorCon &, const RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime)`
    // void
    // RODFDetector::computeSplitProbabilities(const RODFNet* net, const RODFDetectorCon& detectors,
    //                                         const RODFDetectorFlows& flows,
    //                                         SUMOTime startTime, SUMOTime endTime, SUMOTime stepOffset) {
    //     if (myRoutes == nullptr) {
    //         return;
    //     }
    //     // compute edges to determine split probabilities
    //     const std::vector<RODFRouteDesc>& routes = myRoutes->get();
    //     std::vector<RODFEdge*> nextDetEdges;
    //     std::set<ROEdge*> preSplitEdges;
    //     for (std::vector<RODFRouteDesc>::const_iterator i = routes.begin(); i != routes.end(); ++i) {
    //         const RODFRouteDesc& rd = *i;
    //         bool hadSplit = false;
    //         for (ROEdgeVector::const_iterator j = rd.edges2Pass.begin(); j != rd.edges2Pass.end(); ++j) {
    //             if (hadSplit && net->hasDetector(*j)) {
    //                 if (find(nextDetEdges.begin(), nextDetEdges.end(), *j) == nextDetEdges.end()) {
    //                     nextDetEdges.push_back(static_cast<RODFEdge*>(*j));
    //                 }
    //                 myRoute2Edge[rd.routename] = static_cast<RODFEdge*>(*j);
    //                 break;
    //             }
    //             if (!hadSplit) {
    //                 preSplitEdges.insert(*j);
    //             }
    //             if ((*j)->getNumSuccessors() > 1) {
    //                 hadSplit = true;
    //             }
    //         }
    //     }
    //     std::map<ROEdge*, double> inFlows;
    //     if (OptionsCont::getOptions().getBool("respect-concurrent-inflows")) {
    //         for (std::vector<RODFEdge*>::const_iterator i = nextDetEdges.begin(); i != nextDetEdges.end(); ++i) {
    //             std::set<ROEdge*> seen(preSplitEdges);
    //             ROEdgeVector pending;
    //             pending.push_back(*i);
    //             seen.insert(*i);
    //             while (!pending.empty()) {
    //                 ROEdge* e = pending.back();
    //                 pending.pop_back();
    //                 for (ROEdgeVector::const_iterator it = e->getPredecessors().begin(); it != e->getPredecessors().end(); it++) {
    //                     ROEdge* e2 = *it;
    //                     if (e2->getNumSuccessors() == 1 && seen.count(e2) == 0) {
    //                         if (net->hasDetector(e2)) {
    //                             inFlows[*i] += detectors.getAggFlowFor(e2, 0, 0, flows);
    //                         } else {
    //                             pending.push_back(e2);
    //                         }
    //                         seen.insert(e2);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // compute the probabilities to use a certain direction
    //     int index = 0;
    //     for (SUMOTime time = startTime; time < endTime; time += stepOffset, ++index) {
    //         mySplitProbabilities.push_back(std::map<RODFEdge*, double>());
    //         double overallProb = 0;
    //         // retrieve the probabilities
    //         for (std::vector<RODFEdge*>::const_iterator i = nextDetEdges.begin(); i != nextDetEdges.end(); ++i) {
    //             double flow = detectors.getAggFlowFor(*i, time, 60, flows) - inFlows[*i];
    //             overallProb += flow;
    //             mySplitProbabilities[index][*i] = flow;
    //         }
    //         // norm probabilities
    //         if (overallProb > 0) {
    //             for (std::vector<RODFEdge*>::const_iterator i = nextDetEdges.begin(); i != nextDetEdges.end(); ++i) {
    //                 mySplitProbabilities[index][*i] = mySplitProbabilities[index][*i] / overallProb;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_split_probabilities(&mut self, net: Ptr<RODFNet>, detectors: &RODFDetectorCon, flows: &RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime) {
        if self.myRoutes.is_null() {
            return;
        }
        // compute edges to determine split probabilities
        let routes: &Vec<RODFRouteDesc> = self.myRoutes.m().get();
        let mut nextDetEdges: Vec<Ptr<RODFEdge>> = Vec::new();
        let mut preSplitEdges: BTreeSet<Ptr<ROEdge>> = BTreeSet::new();
        for rd in routes.iter() {
            let mut hadSplit = false;
            for j in rd.edges2Pass.iter() {
                if hadSplit && net.has_detector(*j) {
                    if !nextDetEdges.contains(&j.cast::<RODFEdge>()) {
                        nextDetEdges.push(j.cast::<RODFEdge>());
                    }
                    self.myRoute2Edge.insert(rd.routename.clone(), j.cast::<RODFEdge>());
                    break;
                }
                if !hadSplit {
                    preSplitEdges.insert(*j);
                }
                if j.get_num_successors() > 1 {
                    hadSplit = true;
                }
            }
        }
        let mut inFlows: BTreeMap<Ptr<ROEdge>, f64> = BTreeMap::new();
        if get_bool("respect-concurrent-inflows") {
            for i in nextDetEdges.iter() {
                let mut seen: BTreeSet<Ptr<ROEdge>> = preSplitEdges.clone();
                let mut pending: ROEdgeVector = Vec::new();
                pending.push(i.cast::<ROEdge>());
                seen.insert(i.cast::<ROEdge>());
                while let Some(e) = pending.pop() {
                    for it in e.get_predecessors().iter() {
                        let e2 = *it;
                        if e2.get_num_successors() == 1 && !seen.contains(&e2) {
                            if net.has_detector(e2) {
                                *inFlows.entry(i.cast::<ROEdge>()).or_insert(0.0) += detectors.get_agg_flow_for(e2, 0, 0, flows) as f64;
                            } else {
                                pending.push(e2);
                            }
                            seen.insert(e2);
                        }
                    }
                }
            }
        }
        // compute the probabilities to use a certain direction
        let mut index: usize = 0;
        let mut time = startTime;
        while time < endTime {
            self.mySplitProbabilities.push(BTreeMap::new());
            let mut overallProb = 0.0;
            // retrieve the probabilities
            for i in nextDetEdges.iter() {
                let flow = detectors.get_agg_flow_for(i.cast::<ROEdge>(), time, 60, flows) as f64 - *inFlows.entry(i.cast::<ROEdge>()).or_insert(0.0);
                overallProb += flow;
                self.mySplitProbabilities[index].insert(*i, flow);
            }
            // norm probabilities
            if overallProb > 0. {
                for i in nextDetEdges.iter() {
                    let v = self.mySplitProbabilities[index][i] / overallProb;
                    self.mySplitProbabilities[index].insert(*i, v);
                }
            }
            index += 1;
            time += stepOffset;
        }
    }

    /// C++: `const std::vector<std::map<RODFEdge *, double>> & RODFDetector::getSplitProbabilities() const` [const]
    // const std::vector<std::map<RODFEdge*, double> >& getSplitProbabilities() const {
    //         return mySplitProbabilities;
    //     }
    #[allow(unused_variables)]
    pub fn get_split_probabilities(&self) -> &Vec<BTreeMap<Ptr<RODFEdge>, f64>> {
        &self.mySplitProbabilities
    }

    /// C++: `int RODFDetector::getFlowFor(const ROEdge *, SUMOTime) const` [const, protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_flow_for(&self, edge: Ptr<ROEdge>, time: SUMOTime) -> i32 {
        // shortcut: C++ declares RODFDetector::getFlowFor but has no definition and no caller; reaching it is a link error in C++
        panic!("RODFDetector::getFlowFor is declared but not defined in SUMO");
    }

    /// C++: `double RODFDetector::computeDistanceFactor(const RODFRouteDesc &) const` [const, protected]
    // double
    // RODFDetector::computeDistanceFactor(const RODFRouteDesc& rd) const {
    //     double distance = rd.edges2Pass[0]->getFromJunction()->getPosition().distanceTo(rd.edges2Pass.back()->getToJunction()->getPosition());
    //     double length = 0;
    //     for (ROEdgeVector::const_iterator i = rd.edges2Pass.begin(); i != rd.edges2Pass.end(); ++i) {
    //         length += (*i)->getLength();
    //     }
    //     return (distance / length);
    // }
    #[allow(unused_variables)]
    pub fn compute_distance_factor(&self, rd: &RODFRouteDesc) -> f64 {
        let distance = rd.edges2Pass[0].get_from_junction().get_position().distance_to(rd.edges2Pass.last().unwrap().get_to_junction().get_position());
        let mut length = 0.;
        for i in rd.edges2Pass.iter() {
            length += i.get_length();
        }
        distance / length
    }

    /// C++: `void RODFDetector::RODFDetector(const RODFDetector &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(src: &RODFDetector) -> Self {
        // shortcut: C++ declares this copy constructor private and invalidated with no definition; it cannot be called
        panic!("RODFDetector copy constructor is invalidated in SUMO");
    }

    /// C++: `RODFDetector & RODFDetector::operator=(const RODFDetector &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFDetector) -> &mut RODFDetector {
        // shortcut: C++ declares operator= private and invalidated with no definition; it cannot be called
        panic!("RODFDetector assignment operator is invalidated in SUMO");
    }

}

/// A container for RODFDetectors
/// C++ class `RODFDetectorCon` (dfrouter/RODFDetector.h)
pub struct RODFDetectorCon {
    pub myDetectors: Vec<Ptr<RODFDetector>>,
    pub myDetectorMap: BTreeMap<String, Ptr<RODFDetector>>,
    pub myDetectorEdgeMap: BTreeMap<String, Vec<Ptr<RODFDetector>>>,
}

impl RODFDetectorCon {
    /// C++: `void RODFDetectorCon::RODFDetectorCon()`
    // RODFDetectorCon::RODFDetectorCon() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        RODFDetectorCon {
            myDetectors: Vec::new(),
            myDetectorMap: BTreeMap::new(),
            myDetectorEdgeMap: BTreeMap::new(),
        }
    }

    /// C++: `void RODFDetectorCon::~RODFDetectorCon()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: the containers hold non-owning pointers, so there is nothing to free
    }

    /// C++: `bool RODFDetectorCon::addDetector(RODFDetector *)`
    // bool
    // RODFDetectorCon::addDetector(RODFDetector* dfd) {
    //     if (myDetectorMap.find(dfd->getID()) != myDetectorMap.end()) {
    //         return false;
    //     }
    //     myDetectorMap[dfd->getID()] = dfd;
    //     myDetectors.push_back(dfd);
    //     const std::string edgeid = SUMOXMLDefinitions::getEdgeIDFromLane(dfd->getLaneID());
    //     if (myDetectorEdgeMap.find(edgeid) == myDetectorEdgeMap.end()) {
    //         myDetectorEdgeMap[edgeid] = std::vector<RODFDetector*>();
    //     }
    //     myDetectorEdgeMap[edgeid].push_back(dfd);
    //     return true; // !!!
    // }
    #[allow(unused_variables)]
    pub fn add_detector(&mut self, dfd: Ptr<RODFDetector>) -> bool {
        if self.myDetectorMap.contains_key(dfd.get_id()) {
            return false;
        }
        self.myDetectorMap.insert(dfd.get_id().to_string(), dfd);
        self.myDetectors.push(dfd);
        let edgeid = get_edge_id_from_lane(dfd.get_lane_id());
        self.myDetectorEdgeMap.entry(edgeid).or_insert_with(Vec::new).push(dfd);
        true // !!!
    }

    /// C++: `void RODFDetectorCon::removeDetector(const std::string &)`
    // void
    // RODFDetectorCon::removeDetector(const std::string& id) {
    //     //
    //     std::map<std::string, RODFDetector*>::iterator ri1 = myDetectorMap.find(id);
    //     RODFDetector* oldDet = (*ri1).second;
    //     myDetectorMap.erase(ri1);
    //     //
    //     std::vector<RODFDetector*>::iterator ri2 =
    //         std::find(myDetectors.begin(), myDetectors.end(), oldDet);
    //     myDetectors.erase(ri2);
    //     //
    //     bool found = false;
    //     for (std::map<std::string, std::vector<RODFDetector*> >::iterator rr3 = myDetectorEdgeMap.begin(); !found && rr3 != myDetectorEdgeMap.end(); ++rr3) {
    //         std::vector<RODFDetector*>& dets = (*rr3).second;
    //         for (std::vector<RODFDetector*>::iterator ri3 = dets.begin(); !found && ri3 != dets.end();) {
    //             if (*ri3 == oldDet) {
    //                 found = true;
    //                 ri3 = dets.erase(ri3);
    //             } else {
    //                 ++ri3;
    //             }
    //         }
    //     }
    //     delete oldDet;
    // }
    #[allow(unused_variables)]
    pub fn remove_detector(&mut self, id: &str) {
        //
        let oldDet = self.myDetectorMap.remove(id).unwrap();
        //
        let ri2 = self.myDetectors.iter().position(|d| *d == oldDet).unwrap();
        self.myDetectors.remove(ri2);
        //
        let mut found = false;
        for dets in self.myDetectorEdgeMap.values_mut() {
            if found {
                break;
            }
            let mut ri3 = 0;
            while !found && ri3 < dets.len() {
                if dets[ri3] == oldDet {
                    found = true;
                    dets.remove(ri3);
                } else {
                    ri3 += 1;
                }
            }
        }
        // delete oldDet: detectors are Box-allocated (see RODFDetectorHandler)
        drop(unsafe { Box::from_raw(oldDet.as_raw()) });
    }

    /// C++: `bool RODFDetectorCon::detectorsHaveCompleteTypes() const` [const]
    // bool
    // RODFDetectorCon::detectorsHaveCompleteTypes() const {
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         if ((*i)->getType() == TYPE_NOT_DEFINED) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn detectors_have_complete_types(&self) -> bool {
        for i in self.myDetectors.iter() {
            if i.get_type() == RODFDetectorType::TYPE_NOT_DEFINED {
                return false;
            }
        }
        true
    }

    /// C++: `bool RODFDetectorCon::detectorsHaveRoutes() const` [const]
    // bool
    // RODFDetectorCon::detectorsHaveRoutes() const {
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         if ((*i)->hasRoutes()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn detectors_have_routes(&self) -> bool {
        for i in self.myDetectors.iter() {
            if i.has_routes() {
                return true;
            }
        }
        false
    }

    /// C++: `const std::vector<RODFDetector *> & RODFDetectorCon::getDetectors() const` [const]
    // const std::vector< RODFDetector*>&
    // RODFDetectorCon::getDetectors() const {
    //     return myDetectors;
    // }
    #[allow(unused_variables)]
    pub fn get_detectors(&self) -> &Vec<Ptr<RODFDetector>> {
        &self.myDetectors
    }

    /// C++: `void RODFDetectorCon::save(const std::string &) const` [const]
    // void
    // RODFDetectorCon::save(const std::string& file) const {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("detectors", "detectors_file.xsd");
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         out.openTag(SUMO_TAG_DETECTOR_DEFINITION).writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML((*i)->getID())).writeAttr(SUMO_ATTR_LANE, (*i)->getLaneID()).writeAttr(SUMO_ATTR_POSITION, (*i)->getPos());
    //         switch ((*i)->getType()) {
    //             case BETWEEN_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "between");
    //                 break;
    //             case SOURCE_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "source");
    //                 break;
    //             case SINK_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "sink");
    //                 break;
    //             case DISCARDED_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "discarded");
    //                 break;
    //             default:
    //                 throw 1;
    //         }
    //         out.closeTag();
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn save(&self, file: &str) {
        let out = Ptr::from_raw(OutputDevice::get_device(file, true).unwrap_or_else(|e| panic!("{e}")).as_ptr());
        out.m().write_xml_header("detectors", "detectors_file.xsd", BTreeMap::new(), true);
        for i in self.myDetectors.iter() {
            out.m().open_tag_id(SUMO_TAG_DETECTOR_DEFINITION).write_attr(SUMO_ATTR_ID, &string_utils::escape_xml(i.get_id(), false)).write_attr(SUMO_ATTR_LANE, i.get_lane_id()).write_attr(SUMO_ATTR_POSITION, &i.get_pos());
            match i.get_type() {
                RODFDetectorType::BETWEEN_DETECTOR => {
                    out.m().write_attr(SUMO_ATTR_TYPE, "between");
                }
                RODFDetectorType::SOURCE_DETECTOR => {
                    out.m().write_attr(SUMO_ATTR_TYPE, "source");
                }
                RODFDetectorType::SINK_DETECTOR => {
                    out.m().write_attr(SUMO_ATTR_TYPE, "sink");
                }
                RODFDetectorType::DISCARDED_DETECTOR => {
                    out.m().write_attr(SUMO_ATTR_TYPE, "discarded");
                }
                _ => std::panic::panic_any(1),
            }
            out.m().close_tag("");
        }
        out.m().close();
    }

    /// C++: `void RODFDetectorCon::saveAsPOIs(const std::string &) const` [const]
    // void
    // RODFDetectorCon::saveAsPOIs(const std::string& file) const {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         out.openTag(SUMO_TAG_POI).writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML((*i)->getID()));
    //         switch ((*i)->getType()) {
    //             case BETWEEN_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "between_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor::BLUE);
    //                 break;
    //             case SOURCE_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "source_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor::GREEN);
    //                 break;
    //             case SINK_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "sink_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor::RED);
    //                 break;
    //             case DISCARDED_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "discarded_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor(51, 51, 51, 255));
    //                 break;
    //             default:
    //                 throw 1;
    //         }
    //         out.writeAttr(SUMO_ATTR_LANE, (*i)->getLaneID()).writeAttr(SUMO_ATTR_POSITION, (*i)->getPos()).closeTag();
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn save_as_po_is(&self, file: &str) {
        todo!("port RODFDetectorCon::saveAsPOIs")
    }

    /// C++: `void RODFDetectorCon::saveRoutes(const std::string &) const` [const]
    // void
    // RODFDetectorCon::saveRoutes(const std::string& file) const {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("routes", "routes_file.xsd");
    //     std::vector<std::string> saved;
    //     // write for source detectors
    //     bool lastWasSaved = true;
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         if ((*i)->getType() != SOURCE_DETECTOR) {
    //             // do not build routes for other than sources
    //             continue;
    //         }
    //         if (lastWasSaved) {
    //             out << "\n";
    //         }
    //         lastWasSaved = (*i)->writeRoutes(saved, out);
    //     }
    //     out << "\n";
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn save_routes(&self, file: &str) {
        todo!("port RODFDetectorCon::saveRoutes")
    }

    /// C++: `const RODFDetector & RODFDetectorCon::getDetector(const std::string &) const` [const]
    // const RODFDetector&
    // RODFDetectorCon::getDetector(const std::string& id) const {
    //     return *(myDetectorMap.find(id)->second);
    // }
    #[allow(unused_variables)]
    pub fn get_detector(&self, id: &str) -> &RODFDetector {
        todo!("port RODFDetectorCon::getDetector")
    }

    /// C++: `RODFDetector & RODFDetectorCon::getModifiableDetector(const std::string &) const` [const]
    // RODFDetector&
    // RODFDetectorCon::getModifiableDetector(const std::string& id) const {
    //     return *(myDetectorMap.find(id)->second);
    // }
    #[allow(unused_variables)]
    pub fn get_modifiable_detector(&self, id: &str) -> &mut RODFDetector {
        todo!("port RODFDetectorCon::getModifiableDetector")
    }

    /// C++: `const RODFDetector & RODFDetectorCon::getAnyDetectorForEdge(const RODFEdge *const) const` [const]
    // const RODFDetector&
    // RODFDetectorCon::getAnyDetectorForEdge(const RODFEdge* const edge) const {
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         if ((*i)->getEdgeID() == edge->getID()) {
    //             return **i;
    //         }
    //     }
    //     throw 1;
    // }
    #[allow(unused_variables)]
    pub fn get_any_detector_for_edge(&self, edge: Ptr<RODFEdge>) -> &RODFDetector {
        todo!("port RODFDetectorCon::getAnyDetectorForEdge")
    }

    /// C++: `bool RODFDetectorCon::knows(const std::string &) const` [const]
    // bool
    // RODFDetectorCon::knows(const std::string& id) const {
    //     return myDetectorMap.find(id) != myDetectorMap.end();
    // }
    #[allow(unused_variables)]
    pub fn knows(&self, id: &str) -> bool {
        todo!("port RODFDetectorCon::knows")
    }

    /// C++: `void RODFDetectorCon::writeEmitters(const std::string &, const RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime, const RODFNet &, bool, bool, double, bool)`
    // void
    // RODFDetectorCon::writeEmitters(const std::string& file,
    //                                const RODFDetectorFlows& flows,
    //                                SUMOTime startTime, SUMOTime endTime,
    //                                SUMOTime stepOffset, const RODFNet& net,
    //                                bool writeCalibrators,
    //                                bool includeUnusedRoutes,
    //                                double scale,
    //                                bool insertionsOnly) {
    //     // compute turn probabilities at detector
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         (*i)->computeSplitProbabilities(&net, *this, flows, startTime, endTime, stepOffset);
    //     }
    //     //
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     // write vType(s)
    //     const bool separateVTypeOutput = OptionsCont::getOptions().getString("vtype-output") != "";
    //     OutputDevice& vTypeOut = separateVTypeOutput ? OutputDevice::getDevice(OptionsCont::getOptions().getString("vtype-output")) : out;
    //     if (separateVTypeOutput) {
    //         vTypeOut.writeXMLHeader("additional", "additional_file.xsd");
    //     }
    //     const bool forceDev = !OptionsCont::getOptions().isDefault("speeddev");
    //     const double speedDev = OptionsCont::getOptions().getFloat("speeddev");
    //     if (OptionsCont::getOptions().getBool("vtype")) {
    //         // write separate types
    //         SUMOVTypeParameter pkwType = SUMOVTypeParameter("PKW", SVC_PASSENGER);
    //         setSpeedFactorAndDev(pkwType, net.getMaxSpeedFactorPKW(), net.getAvgSpeedFactorPKW(), speedDev, forceDev);
    //         pkwType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //         pkwType.write(vTypeOut);
    //         SUMOVTypeParameter lkwType = SUMOVTypeParameter("LKW", SVC_TRUCK);
    //         setSpeedFactorAndDev(lkwType, net.getMaxSpeedFactorLKW(), net.getAvgSpeedFactorLKW(), speedDev, forceDev);
    //         lkwType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //         lkwType.write(vTypeOut);
    //     } else {
    //         // patch default type
    //         SUMOVTypeParameter type = SUMOVTypeParameter(DEFAULT_VTYPE_ID, SVC_PASSENGER);
    //         setSpeedFactorAndDev(type, MAX2(net.getMaxSpeedFactorPKW(), net.getMaxSpeedFactorLKW()), net.getAvgSpeedFactorPKW(), speedDev, forceDev);
    //         if (type.parametersSet != 0) {
    //             type.write(vTypeOut);
    //         }
    //     }
    //
    //
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         RODFDetector* det = *i;
    //         // get file name for values (emitter/calibrator definition)
    //         std::string escapedID = StringUtils::escapeXML(det->getID());
    //         std::string defFileName;
    //         if (det->getType() == SOURCE_DETECTOR) {
    //             defFileName = file;
    //         } else if (writeCalibrators && det->getType() == BETWEEN_DETECTOR) {
    //             defFileName = FileHelpers::getFilePath(file) + "calibrator_" + escapedID + ".def.xml";
    //         } else {
    //             defFileName = FileHelpers::getFilePath(file) + "other_" + escapedID + ".def.xml";
    //             continue;
    //         }
    //         // try to write the definition
    //         double defaultSpeed = net.getEdge(det->getEdgeID())->getSpeedLimit();
    //         //  ... compute routes' distribution over time
    //         std::map<SUMOTime, RandomDistributor<int>* > dists;
    //         if (!insertionsOnly && flows.knows(det->getID())) {
    //             det->buildDestinationDistribution(*this, startTime, endTime, stepOffset, net, dists);
    //         }
    //         //  ... write the definition
    //         if (!det->writeEmitterDefinition(defFileName, dists, flows, startTime, endTime, stepOffset, includeUnusedRoutes, scale, insertionsOnly, defaultSpeed)) {
    //             // skip if something failed... (!!!)
    //             continue;
    //         }
    //         //  ... clear temporary values
    //         clearDists(dists);
    //         // write the declaration into the file
    //         if (writeCalibrators && det->getType() == BETWEEN_DETECTOR) {
    //             out.openTag(SUMO_TAG_CALIBRATOR).writeAttr(SUMO_ATTR_ID, "calibrator_" + escapedID).writeAttr(SUMO_ATTR_POSITION, det->getPos());
    //             out.writeAttr(SUMO_ATTR_LANE, det->getLaneID()).writeAttr(SUMO_ATTR_FRIENDLY_POS, true).writeAttr(SUMO_ATTR_FILE, defFileName).closeTag();
    //         }
    //     }
    //     out.close();
    //     if (separateVTypeOutput) {
    //         vTypeOut.close();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_emitters(&mut self, file: &str, flows: &RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime, net: &RODFNet, writeCalibrators: bool, includeUnusedRoutes: bool, scale: f64, insertionsOnly: bool) {
        todo!("port RODFDetectorCon::writeEmitters")
    }

    /// C++: `void RODFDetectorCon::writeEmitterPOIs(const std::string &, const RODFDetectorFlows &)`
    // void
    // RODFDetectorCon::writeEmitterPOIs(const std::string& file,
    //                                   const RODFDetectorFlows& flows) {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         RODFDetector* det = *i;
    //         double flow = flows.getFlowSumSecure(det->getID());
    //         const unsigned char col = static_cast<unsigned char>(128 * flow / flows.getMaxDetectorFlow() + 128);
    //         out.openTag(SUMO_TAG_POI).writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML((*i)->getID()) + ":" + toString(flow));
    //         switch ((*i)->getType()) {
    //             case BETWEEN_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "between_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor(0, 0, col, 255));
    //                 break;
    //             case SOURCE_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "source_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor(0, col, 0, 255));
    //                 break;
    //             case SINK_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "sink_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor(col, 0, 0, 255));
    //                 break;
    //             case DISCARDED_DETECTOR:
    //                 out.writeAttr(SUMO_ATTR_TYPE, "discarded_detector_position").writeAttr(SUMO_ATTR_COLOR, RGBColor(51, 51, 51, 255));
    //                 break;
    //             default:
    //                 throw 1;
    //         }
    //         out.writeAttr(SUMO_ATTR_LANE, (*i)->getLaneID()).writeAttr(SUMO_ATTR_POSITION, (*i)->getPos()).closeTag();
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn write_emitter_po_is(&mut self, file: &str, flows: &RODFDetectorFlows) {
        todo!("port RODFDetectorCon::writeEmitterPOIs")
    }

    /// C++: `void RODFDetectorCon::writeSpeedTrigger(const RODFNet *const, const std::string &, const RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime)`
    // void
    // RODFDetectorCon::writeSpeedTrigger(const RODFNet* const net,
    //                                    const std::string& file,
    //                                    const RODFDetectorFlows& flows,
    //                                    SUMOTime startTime, SUMOTime endTime,
    //                                    SUMOTime stepOffset) {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         RODFDetector* det = *i;
    //         // write the declaration into the file
    //         if (det->getType() == SINK_DETECTOR && flows.knows(det->getID())) {
    //             std::string filename = FileHelpers::getFilePath(file) + "vss_" + det->getID() + ".def.xml";
    //             out.openTag(SUMO_TAG_VSS).writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML(det->getID())).writeAttr(SUMO_ATTR_LANES, det->getLaneID()).writeAttr(SUMO_ATTR_FILE, filename).closeTag();
    //             double defaultSpeed = net != nullptr ? net->getEdge(det->getEdgeID())->getSpeedLimit() : (double) 200.;
    //             det->writeSingleSpeedTrigger(filename, flows, startTime, endTime, stepOffset, defaultSpeed);
    //         }
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn write_speed_trigger(&mut self, net: Ptr<RODFNet>, file: &str, flows: &RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime) {
        todo!("port RODFDetectorCon::writeSpeedTrigger")
    }

    /// C++: `void RODFDetectorCon::writeValidationDetectors(const std::string &, bool, bool, bool)`
    // void
    // RODFDetectorCon::writeValidationDetectors(const std::string& file,
    //         bool includeSources,
    //         bool singleFile, bool friendly) {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         RODFDetector* det = *i;
    //         // write the declaration into the file
    //         if (det->getType() != SOURCE_DETECTOR || includeSources) {
    //             double pos = det->getPos();
    //             if (det->getType() == SOURCE_DETECTOR) {
    //                 pos += 1;
    //             }
    //             out.openTag(SUMO_TAG_E1DETECTOR).writeAttr(SUMO_ATTR_ID, "validation_" + StringUtils::escapeXML(det->getID())).writeAttr(SUMO_ATTR_LANE, det->getLaneID());
    //             out.writeAttr(SUMO_ATTR_POSITION, pos).writeAttr(SUMO_ATTR_PERIOD, 60);
    //             if (friendly) {
    //                 out.writeAttr(SUMO_ATTR_FRIENDLY_POS, true);
    //             }
    //             if (!singleFile) {
    //                 out.writeAttr(SUMO_ATTR_FILE, "validation_det_" + StringUtils::escapeXML(det->getID()) + ".xml");
    //             } else {
    //                 out.writeAttr(SUMO_ATTR_FILE, "validation_dets.xml");
    //             }
    //             out.closeTag();
    //         }
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn write_validation_detectors(&mut self, file: &str, includeSources: bool, singleFile: bool, friendly: bool) {
        todo!("port RODFDetectorCon::writeValidationDetectors")
    }

    /// C++: `void RODFDetectorCon::writeEndRerouterDetectors(const std::string &)`
    // void
    // RODFDetectorCon::writeEndRerouterDetectors(const std::string& file) {
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         RODFDetector* det = *i;
    //         // write the declaration into the file
    //         if (det->getType() == SINK_DETECTOR) {
    //             out.openTag(SUMO_TAG_REROUTER).writeAttr(SUMO_ATTR_ID, "endrerouter_" + StringUtils::escapeXML(det->getID())).writeAttr(SUMO_ATTR_EDGES, det->getLaneID());
    //             out.writeAttr(SUMO_ATTR_POSITION, 0.).writeAttr(SUMO_ATTR_FILE, "endrerouter_" + det->getID() + ".def.xml").closeTag();
    //         }
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn write_end_rerouter_detectors(&mut self, file: &str) {
        todo!("port RODFDetectorCon::writeEndRerouterDetectors")
    }

    /// C++: `int RODFDetectorCon::getAggFlowFor(const ROEdge *, SUMOTime, SUMOTime, const RODFDetectorFlows &) const` [const]
    // int
    // RODFDetectorCon::getAggFlowFor(const ROEdge* edge, SUMOTime time, SUMOTime period,
    //                                const RODFDetectorFlows&) const {
    //     UNUSED_PARAMETER(period);
    //     UNUSED_PARAMETER(time);
    //     if (edge == nullptr) {
    //         return 0;
    //     }
    // //    double stepOffset = 60; // !!!
    // //    double startTime = 0; // !!!
    // //    cout << edge->getID() << endl;
    //     assert(myDetectorEdgeMap.find(edge->getID()) != myDetectorEdgeMap.end());
    //     const std::vector<FlowDef>& flows = static_cast<const RODFEdge*>(edge)->getFlows();
    //     double agg = 0;
    //     for (std::vector<FlowDef>::const_iterator i = flows.begin(); i != flows.end(); ++i) {
    //         const FlowDef& srcFD = *i;
    //         if (srcFD.qLKW >= 0) {
    //             agg += srcFD.qLKW;
    //         }
    //         if (srcFD.qPKW >= 0) {
    //             agg += srcFD.qPKW;
    //         }
    //     }
    //     return (int) agg;
    //     /* !!! make this time variable
    //     if (flows.size()!=0) {
    //         double agg = 0;
    //         int beginIndex = (int)((time/stepOffset) - startTime);  // !!! falsch!!!
    //         for (SUMOTime t=0; t<period&&beginIndex<flows.size(); t+=(SUMOTime) stepOffset) {
    //             const FlowDef &srcFD = flows[beginIndex++];
    //             if (srcFD.qLKW>=0) {
    //                 agg += srcFD.qLKW;
    //             }
    //             if (srcFD.qPKW>=0) {
    //                 agg += srcFD.qPKW;
    //             }
    //         }
    //         return (int) agg;
    //     }
    //     */
    // //    return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_agg_flow_for(&self, edge: Ptr<ROEdge>, time: SUMOTime, period: SUMOTime, flows: &RODFDetectorFlows) -> i32 {
        todo!("port RODFDetectorCon::getAggFlowFor")
    }

    /// C++: `void RODFDetectorCon::guessEmptyFlows(RODFDetectorFlows &)`
    // void
    // RODFDetectorCon::guessEmptyFlows(RODFDetectorFlows& flows) {
    //     // routes must be built (we have ensured this in main)
    //     // detector followers/prior must be build (we have ensured this in main)
    //     //
    //     for (std::vector<RODFDetector*>::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         RODFDetector* det = *i;
    //         const std::set<const RODFDetector*>& prior = det->getPriorDetectors();
    //         const std::set<const RODFDetector*>& follower = det->getFollowerDetectors();
    //         int noFollowerWithRoutes = 0;
    //         int noPriorWithRoutes = 0;
    //         // count occurrences of detectors with/without routes
    //         std::set<const RODFDetector*>::const_iterator j;
    //         for (j = prior.begin(); j != prior.end(); ++j) {
    //             if (flows.knows((*j)->getID())) {
    //                 ++noPriorWithRoutes;
    //             }
    //         }
    //         for (j = follower.begin(); j != follower.end(); ++j) {
    //             if (flows.knows((*j)->getID())) {
    //                 ++noFollowerWithRoutes;
    //             }
    //         }
    //
    //         // do not process detectors which have no routes
    //         if (!flows.knows(det->getID())) {
    //             continue;
    //         }
    //
    //         // plain case: all of the prior detectors have routes
    //         if (noPriorWithRoutes == (int)prior.size()) {
    //             // the number of vehicles is the sum of all vehicles on prior
    //             continue;
    //         }
    //
    //         // plain case: all of the follower detectors have routes
    //         if (noFollowerWithRoutes == (int)follower.size()) {
    //             // the number of vehicles is the sum of all vehicles on follower
    //             continue;
    //         }
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn guess_empty_flows(&mut self, flows: &mut RODFDetectorFlows) {
        todo!("port RODFDetectorCon::guessEmptyFlows")
    }

    /// C++: `void RODFDetectorCon::mesoJoin(const std::string &, const std::vector<std::string> &)`
    // void
    // RODFDetectorCon::mesoJoin(const std::string& nid,
    //                           const std::vector<std::string>& oldids) {
    //     // build the new detector
    //     const RODFDetector& first = getDetector(*(oldids.begin()));
    //     RODFDetector* newDet = new RODFDetector(nid, first);
    //     addDetector(newDet);
    //     // delete previous
    //     for (std::vector<std::string>::const_iterator i = oldids.begin(); i != oldids.end(); ++i) {
    //         removeDetector(*i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn meso_join(&mut self, nid: &str, oldids: &Vec<String>) {
        todo!("port RODFDetectorCon::mesoJoin")
    }

    /// C++: `void RODFDetectorCon::setSpeedFactorAndDev(SUMOVTypeParameter &, double, double, double, bool)`
    // void
    // RODFDetectorCon::setSpeedFactorAndDev(SUMOVTypeParameter& type, double maxFactor, double avgFactor, double dev, bool forceDev) {
    //     if (avgFactor > 1) {
    //         // systematically low speeds can easily be caused by traffic
    //         // conditions. Whereas elevated speeds probably reflect speeding
    //         type.speedFactor.setParameter(0, avgFactor);
    //         type.parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    //     }
    //     if (forceDev || (maxFactor > 1 && maxFactor > type.speedFactor.getParameter(0) + NUMERICAL_EPS)) {
    //         // setting a non-zero speed deviation causes the simulation to recompute
    //         // individual speedFactors to match departSpeed (MSEdge::insertVehicle())
    //         type.speedFactor.setParameter(1, dev);
    //         type.parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    //     } else {
    //         type.speedFactor.setParameter(1, -1.); // do not write speedDev, only simple speedFactor
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_speed_factor_and_dev(&mut self, type_: &mut SUMOVTypeParameter, maxFactor: f64, avgFactor: f64, dev: f64, forceDev: bool) {
        todo!("port RODFDetectorCon::setSpeedFactorAndDev")
    }

    /// C++: `void RODFDetectorCon::clearDists(std::map<SUMOTime, RandomDistributor<int> *> &) const` [const, protected]
    /// Clears the given distributions map, deleting the timed distributions
    // void
    // RODFDetectorCon::clearDists(std::map<SUMOTime, RandomDistributor<int>* >& dists) const {
    //     for (std::map<SUMOTime, RandomDistributor<int>* >::iterator i = dists.begin(); i != dists.end(); ++i) {
    //         delete (*i).second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_dists(&self, dists: &mut BTreeMap<SUMOTime, Ptr<RandomDistributor<i32>>>) {
        todo!("port RODFDetectorCon::clearDists")
    }

    /// C++: `void RODFDetectorCon::RODFDetectorCon(const RODFDetectorCon &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RODFDetectorCon) -> Self {
        todo!("port RODFDetectorCon::RODFDetectorCon")
    }

    /// C++: `RODFDetectorCon & RODFDetectorCon::operator=(const RODFDetectorCon &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFDetectorCon) -> &mut RODFDetectorCon {
        todo!("port RODFDetectorCon::operator=")
    }

}

