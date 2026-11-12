//! Port of dfrouter/RODFDetector.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port RODFDetector::RODFDetector")
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
        todo!("port RODFDetector::RODFDetector")
    }

    /// C++: `void RODFDetector::~RODFDetector() noexcept` [virtual]
    /// Destructor
    // RODFDetector::~RODFDetector() {
    //     delete myRoutes;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFDetector::~RODFDetector")
    }

    /// C++: `const std::string & RODFDetector::getLaneID() const` [const]
    /// Returns the id of the lane this detector is placed on
    // const std::string& getLaneID() const {
    //         return myLaneID;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_id(&self) -> &str {
        todo!("port RODFDetector::getLaneID")
    }

    /// C++: `std::string RODFDetector::getEdgeID() const` [const]
    /// Returns the id of the edge this detector is placed on
    // std::string
    // RODFDetector::getEdgeID() const {
    //     return SUMOXMLDefinitions::getEdgeIDFromLane(myLaneID);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(&self) -> String {
        todo!("port RODFDetector::getEdgeID")
    }

    /// C++: `double RODFDetector::getPos() const` [const]
    /// Returns the position at which the detector lies
    // double getPos() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_pos(&self) -> f64 {
        todo!("port RODFDetector::getPos")
    }

    /// C++: `RODFDetectorType RODFDetector::getType() const` [const]
    /// Returns the type of the detector
    // RODFDetectorType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> RODFDetectorType {
        todo!("port RODFDetector::getType")
    }

    /// C++: `void RODFDetector::setType(RODFDetectorType)`
    /// @}
    // void
    // RODFDetector::setType(RODFDetectorType type) {
    //     myType = type;
    // }
    #[allow(unused_variables)]
    pub fn set_type(&mut self, type_: RODFDetectorType) {
        todo!("port RODFDetector::setType")
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
        todo!("port RODFDetector::addRoute")
    }

    /// C++: `void RODFDetector::addRoutes(RODFRouteCont *)`
    // void
    // RODFDetector::addRoutes(RODFRouteCont* routes) {
    //     delete myRoutes;
    //     myRoutes = routes;
    // }
    #[allow(unused_variables)]
    pub fn add_routes(&mut self, routes: Ptr<RODFRouteCont>) {
        todo!("port RODFDetector::addRoutes")
    }

    /// C++: `bool RODFDetector::hasRoutes() const` [const]
    // bool
    // RODFDetector::hasRoutes() const {
    //     return myRoutes != nullptr && myRoutes->get().size() != 0;
    // }
    #[allow(unused_variables)]
    pub fn has_routes(&self) -> bool {
        todo!("port RODFDetector::hasRoutes")
    }

    /// C++: `const std::vector<RODFRouteDesc> & RODFDetector::getRouteVector() const` [const]
    // const std::vector<RODFRouteDesc>&
    // RODFDetector::getRouteVector() const {
    //     return myRoutes->get();
    // }
    #[allow(unused_variables)]
    pub fn get_route_vector(&self) -> &Vec<RODFRouteDesc> {
        todo!("port RODFDetector::getRouteVector")
    }

    /// C++: `void RODFDetector::addPriorDetector(const RODFDetector *)`
    // void
    // RODFDetector::addPriorDetector(const RODFDetector* det) {
    //     myPriorDetectors.insert(det);
    // }
    #[allow(unused_variables)]
    pub fn add_prior_detector(&mut self, det: Ptr<RODFDetector>) {
        todo!("port RODFDetector::addPriorDetector")
    }

    /// C++: `void RODFDetector::addFollowingDetector(const RODFDetector *)`
    // void
    // RODFDetector::addFollowingDetector(const RODFDetector* det) {
    //     myFollowingDetectors.insert(det);
    // }
    #[allow(unused_variables)]
    pub fn add_following_detector(&mut self, det: Ptr<RODFDetector>) {
        todo!("port RODFDetector::addFollowingDetector")
    }

    /// C++: `const std::set<const RODFDetector *> & RODFDetector::getPriorDetectors() const` [const]
    // const std::set<const RODFDetector*>&
    // RODFDetector::getPriorDetectors() const {
    //     return myPriorDetectors;
    // }
    #[allow(unused_variables)]
    pub fn get_prior_detectors(&self) -> &BTreeSet<Ptr<RODFDetector>> {
        todo!("port RODFDetector::getPriorDetectors")
    }

    /// C++: `const std::set<const RODFDetector *> & RODFDetector::getFollowerDetectors() const` [const]
    // const std::set<const RODFDetector*>&
    // RODFDetector::getFollowerDetectors() const {
    //     return myFollowingDetectors;
    // }
    #[allow(unused_variables)]
    pub fn get_follower_detectors(&self) -> &BTreeSet<Ptr<RODFDetector>> {
        todo!("port RODFDetector::getFollowerDetectors")
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
        todo!("port RODFDetector::writeEmitterDefinition")
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
        todo!("port RODFDetector::writeRoutes")
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
        todo!("port RODFDetector::writeSingleSpeedTrigger")
    }

    /// C++: `void RODFDetector::writeEndRerouterDetectors(const std::string &)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn write_end_rerouter_detectors(&mut self, file: &str) {
        todo!("port RODFDetector::writeEndRerouterDetectors")
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
        todo!("port RODFDetector::buildDestinationDistribution")
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
        todo!("port RODFDetector::computeSplitProbabilities")
    }

    /// C++: `const std::vector<std::map<RODFEdge *, double>> & RODFDetector::getSplitProbabilities() const` [const]
    // const std::vector<std::map<RODFEdge*, double> >& getSplitProbabilities() const {
    //         return mySplitProbabilities;
    //     }
    #[allow(unused_variables)]
    pub fn get_split_probabilities(&self) -> &Vec<BTreeMap<Ptr<RODFEdge>, f64>> {
        todo!("port RODFDetector::getSplitProbabilities")
    }

    /// C++: `int RODFDetector::getFlowFor(const ROEdge *, SUMOTime) const` [const, protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_flow_for(&self, edge: Ptr<ROEdge>, time: SUMOTime) -> i32 {
        todo!("port RODFDetector::getFlowFor")
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
        todo!("port RODFDetector::computeDistanceFactor")
    }

    /// C++: `void RODFDetector::RODFDetector(const RODFDetector &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(src: &RODFDetector) -> Self {
        todo!("port RODFDetector::RODFDetector")
    }

    /// C++: `RODFDetector & RODFDetector::operator=(const RODFDetector &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFDetector) -> &mut RODFDetector {
        todo!("port RODFDetector::operator=")
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
        todo!("port RODFDetectorCon::RODFDetectorCon")
    }

    /// C++: `void RODFDetectorCon::~RODFDetectorCon()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFDetectorCon::~RODFDetectorCon")
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
        todo!("port RODFDetectorCon::addDetector")
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
        todo!("port RODFDetectorCon::removeDetector")
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
        todo!("port RODFDetectorCon::detectorsHaveCompleteTypes")
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
        todo!("port RODFDetectorCon::detectorsHaveRoutes")
    }

    /// C++: `const std::vector<RODFDetector *> & RODFDetectorCon::getDetectors() const` [const]
    // const std::vector< RODFDetector*>&
    // RODFDetectorCon::getDetectors() const {
    //     return myDetectors;
    // }
    #[allow(unused_variables)]
    pub fn get_detectors(&self) -> &Vec<Ptr<RODFDetector>> {
        todo!("port RODFDetectorCon::getDetectors")
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
        todo!("port RODFDetectorCon::save")
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

