//! Port of dfrouter/RODFNet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `RODFNet::DFRouteDescByTimeComperator` (dfrouter/RODFNet.h)
pub struct RODFNet_DFRouteDescByTimeComperator {
}

impl RODFNet_DFRouteDescByTimeComperator {
    /// C++: `void DFRouteDescByTimeComperator::DFRouteDescByTimeComperator()`
    /// Constructor
    // explicit DFRouteDescByTimeComperator() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port DFRouteDescByTimeComperator::DFRouteDescByTimeComperator")
    }

    /// C++: `void DFRouteDescByTimeComperator::~DFRouteDescByTimeComperator() noexcept`
    /// Destructor
    // ~DFRouteDescByTimeComperator() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port DFRouteDescByTimeComperator::~DFRouteDescByTimeComperator")
    }

    /// C++: `bool DFRouteDescByTimeComperator::operator()(const RODFRouteDesc &, const RODFRouteDesc &) const` [const]
    /// Comparing method
    // bool operator()(const RODFRouteDesc& nod1, const RODFRouteDesc& nod2) const {
    //             return nod1.duration_2 > nod2.duration_2;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, nod1: &RODFRouteDesc, nod2: &RODFRouteDesc) -> bool {
        todo!("port DFRouteDescByTimeComperator::operator()")
    }

}

/// C++ class `RODFNet::IterationEdge` (dfrouter/RODFNet.h)
pub struct RODFNet_IterationEdge {
    pub depth: i32,
    pub edge: Ptr<ROEdge>,
}

impl RODFNet_IterationEdge {
}

/// A DFROUTER-network
/// C++ class `RODFNet` (dfrouter/RODFNet.h)
#[repr(C)]
pub struct RODFNet {
    /// C++ base class `RONet`
    pub base: RONet,
    /// Map of edge name->list of names of this edge approaching edges
    pub myApproachingEdges: BTreeMap<Ptr<ROEdge>, ROEdgeVector>,
    /// Map of edge name->list of names of edges approached by this edge
    pub myApproachedEdges: BTreeMap<Ptr<ROEdge>, ROEdgeVector>,
    pub myDetectorsOnEdges: BTreeMap<Ptr<ROEdge>, Vec<String>>, // C++: std::map<ROEdge *, std::vector<std::string>, idComp>
    pub myDetectorEdges: BTreeMap<String, Ptr<ROEdge>>,
    pub myAmInHighwayMode: bool,
    pub mySourceNumber: i32,
    pub mySinkNumber: i32,
    pub myInBetweenNumber: i32,
    pub myInvalidNumber: i32,
    /// List of ids of edges that shall not be used
    pub myDisallowedEdges: Vec<String>,
    pub myAllowedVClass: SUMOVehicleClass,
    pub myKeepTurnarounds: bool,
    /// maximum speed factor in measurements
    pub myMaxSpeedFactorPKW: f64,
    pub myMaxSpeedFactorLKW: f64,
    pub myAvgSpeedFactorPKW: f64,
    pub myAvgSpeedFactorLKW: f64,
}

impl std::ops::Deref for RODFNet {
    type Target = RONet;
    fn deref(&self) -> &RONet {
        &self.base
    }
}
impl std::ops::DerefMut for RODFNet {
    fn deref_mut(&mut self) -> &mut RONet {
        &mut self.base
    }
}

impl RODFNet {
    /// C++: `void RODFNet::RODFNet(bool)`
    /// Constructor
    // RODFNet::RODFNet(bool amInHighwayMode) :
    //     RONet(), myAmInHighwayMode(amInHighwayMode),
    //     mySourceNumber(0), mySinkNumber(0), myInBetweenNumber(0), myInvalidNumber(0),
    //     myMaxSpeedFactorPKW(1),
    //     myMaxSpeedFactorLKW(1),
    //     myAvgSpeedFactorPKW(1),
    //     myAvgSpeedFactorLKW(1) {
    //     myDisallowedEdges = OptionsCont::getOptions().getStringVector("disallowed-edges");
    //     myAllowedVClass = getVehicleClassID(OptionsCont::getOptions().getString("vclass"));
    //     myKeepTurnarounds = OptionsCont::getOptions().getBool("keep-turnarounds");
    // }
    #[allow(unused_variables)]
    pub fn new(amInHighwayMode: bool) -> Self {
        todo!("port RODFNet::RODFNet")
    }

    /// C++: `void RODFNet::~RODFNet() noexcept` [virtual]
    /// Destructor
    // RODFNet::~RODFNet() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFNet::~RODFNet")
    }

    /// C++: `void RODFNet::buildApproachList()`
    // void
    // RODFNet::buildApproachList() {
    //     for (ROEdge* const ce : ROEdge::getAllEdges()) {
    //         if (!isAllowed(ce)) {
    //             continue;
    //         }
    //         for (ROEdge* const help : ce->getSuccessors()) {
    //             if (!isAllowed(help)) {
    //                 // blocked edges will not be used
    //                 continue;
    //             }
    //             if (!myKeepTurnarounds && help->getToJunction() == ce->getFromJunction()) {
    //                 // do not use turnarounds
    //                 continue;
    //             }
    //             // add the connection help->ce to myApproachingEdges
    //             myApproachingEdges[help].push_back(ce);
    //             // add the connection ce->help to myApproachingEdges
    //             myApproachedEdges[ce].push_back(help);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_approach_list(&mut self) {
        todo!("port RODFNet::buildApproachList")
    }

    /// C++: `void RODFNet::computeTypes(RODFDetectorCon &, bool) const` [const]
    // void
    // RODFNet::computeTypes(RODFDetectorCon& detcont,
    //                       bool sourcesStrict) const {
    //     PROGRESS_BEGIN_MESSAGE(TL("Computing detector types"));
    //     const std::vector< RODFDetector*>& dets = detcont.getDetectors();
    //     // build needed information. first
    //     buildDetectorEdgeDependencies(detcont);
    //     // compute detector types then
    //     for (std::vector< RODFDetector*>::const_iterator i = dets.begin(); i != dets.end(); ++i) {
    //         if (isSource(**i, detcont, sourcesStrict)) {
    //             (*i)->setType(SOURCE_DETECTOR);
    //             mySourceNumber++;
    //         }
    //         if (isDestination(**i, detcont)) {
    //             (*i)->setType(SINK_DETECTOR);
    //             mySinkNumber++;
    //         }
    //         if ((*i)->getType() == TYPE_NOT_DEFINED) {
    //             (*i)->setType(BETWEEN_DETECTOR);
    //             myInBetweenNumber++;
    //         }
    //     }
    //     // recheck sources
    //     for (std::vector< RODFDetector*>::const_iterator i = dets.begin(); i != dets.end(); ++i) {
    //         if ((*i)->getType() == SOURCE_DETECTOR && isFalseSource(**i, detcont)) {
    //             (*i)->setType(DISCARDED_DETECTOR);
    //             myInvalidNumber++;
    //             mySourceNumber--;
    //         }
    //     }
    //     // print results
    //     PROGRESS_DONE_MESSAGE();
    //     WRITE_MESSAGE(TL("Computed detector types:"));
    //     WRITE_MESSAGEF(TL(" % source detectors"), toString(mySourceNumber));
    //     WRITE_MESSAGEF(TL(" % sink detectors"), toString(mySinkNumber));
    //     WRITE_MESSAGEF(TL(" % in-between detectors"), toString(myInBetweenNumber));
    //     WRITE_MESSAGEF(TL(" % invalid detectors"), toString(myInvalidNumber));
    // }
    #[allow(unused_variables)]
    pub fn compute_types(&self, dets: &mut RODFDetectorCon, sourcesStrict: bool) {
        todo!("port RODFNet::computeTypes")
    }

    /// C++: `void RODFNet::buildRoutes(RODFDetectorCon &, bool, bool, bool, int) const` [const]
    // void
    // RODFNet::buildRoutes(RODFDetectorCon& detcont, bool keepUnfoundEnds, bool includeInBetween,
    //                      bool keepShortestOnly, int maxFollowingLength) const {
    //     // build needed information first
    //     buildDetectorEdgeDependencies(detcont);
    //     // then build the routes
    //     std::map<ROEdge*, RODFRouteCont* > doneEdges;
    //     const std::vector< RODFDetector*>& dets = detcont.getDetectors();
    //     for (std::vector< RODFDetector*>::const_iterator i = dets.begin(); i != dets.end(); ++i) {
    //         ROEdge* e = getDetectorEdge(**i);
    //         if (doneEdges.find(e) != doneEdges.end()) {
    //             // use previously build routes
    //             (*i)->addRoutes(new RODFRouteCont(*doneEdges[e]));
    //             continue;
    //         }
    //         ROEdgeVector seen;
    //         RODFRouteCont* routes = new RODFRouteCont();
    //         doneEdges[e] = routes;
    //         RODFRouteDesc rd;
    //         rd.edges2Pass.push_back(e);
    //         rd.duration_2 = (e->getLength() / e->getSpeedLimit()); //!!!;
    //         rd.endDetectorEdge = nullptr;
    //         rd.lastDetectorEdge = nullptr;
    //         rd.distance = e->getLength();
    //         rd.distance2Last = 0;
    //         rd.duration2Last = 0;
    //
    //         rd.overallProb = 0;
    //
    //         ROEdgeVector visited;
    //         visited.push_back(e);
    //         computeRoutesFor(e, rd, 0, keepUnfoundEnds, keepShortestOnly,
    //                          visited, **i, *routes, detcont, maxFollowingLength, seen);
    //         //!!!routes->removeIllegal(illegals);
    //         (*i)->addRoutes(routes);
    //
    //         // add routes to in-between detectors if wished
    //         if (includeInBetween) {
    //             // go through the routes
    //             const std::vector<RODFRouteDesc>& r = routes->get();
    //             for (std::vector<RODFRouteDesc>::const_iterator j = r.begin(); j != r.end(); ++j) {
    //                 const RODFRouteDesc& mrd = *j;
    //                 double duration = mrd.duration_2;
    //                 double distance = mrd.distance;
    //                 // go through each route's edges
    //                 ROEdgeVector::const_iterator routeend = mrd.edges2Pass.end();
    //                 for (ROEdgeVector::const_iterator k = mrd.edges2Pass.begin(); k != routeend; ++k) {
    //                     // check whether any detectors lies on the current edge
    //                     if (myDetectorsOnEdges.find(*k) == myDetectorsOnEdges.end()) {
    //                         duration -= (*k)->getLength() / (*k)->getSpeedLimit();
    //                         distance -= (*k)->getLength();
    //                         continue;
    //                     }
    //                     // go through the detectors
    //                     for (const std::string& l : myDetectorsOnEdges.find(*k)->second) {
    //                         const RODFDetector& m = detcont.getDetector(l);
    //                         if (m.getType() == BETWEEN_DETECTOR) {
    //                             RODFRouteDesc nrd;
    //                             copy(k, routeend, back_inserter(nrd.edges2Pass));
    //                             nrd.duration_2 = duration;//!!!;
    //                             nrd.endDetectorEdge = mrd.endDetectorEdge;
    //                             nrd.lastDetectorEdge = mrd.lastDetectorEdge;
    //                             nrd.distance = distance;
    //                             nrd.distance2Last = mrd.distance2Last;
    //                             nrd.duration2Last = mrd.duration2Last;
    //                             nrd.overallProb = mrd.overallProb;
    //                             nrd.factor = mrd.factor;
    //                             ((RODFDetector&) m).addRoute(nrd);
    //                         }
    //                     }
    //                     duration -= (*k)->getLength() / (*k)->getSpeedLimit();
    //                     distance -= (*k)->getLength();
    //                 }
    //             }
    //         }
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_routes(&self, det: &mut RODFDetectorCon, keepUnfoundEnds: bool, includeInBetween: bool, keepShortestOnly: bool, maxFollowingLength: i32) {
        todo!("port RODFNet::buildRoutes")
    }

    /// C++: `double RODFNet::getAbsPos(const RODFDetector &) const` [const]
    // double
    // RODFNet::getAbsPos(const RODFDetector& det) const {
    //     if (det.getPos() >= 0) {
    //         return det.getPos();
    //     }
    //     return getDetectorEdge(det)->getLength() + det.getPos();
    // }
    #[allow(unused_variables)]
    pub fn get_abs_pos(&self, det: &RODFDetector) -> f64 {
        todo!("port RODFNet::getAbsPos")
    }

    /// C++: `void RODFNet::buildEdgeFlowMap(const RODFDetectorFlows &, const RODFDetectorCon &, SUMOTime, SUMOTime, SUMOTime)`
    // void
    // RODFNet::buildEdgeFlowMap(const RODFDetectorFlows& flows,
    //                           const RODFDetectorCon& detectors,
    //                           SUMOTime startTime, SUMOTime endTime,
    //                           SUMOTime stepOffset) {
    //     std::map<ROEdge*, std::vector<std::string>, idComp>::iterator i;
    //     double speedFactorSumPKW = 0;
    //     double speedFactorSumLKW = 0;
    //     double speedFactorCountPKW = 0;
    //     double speedFactorCountLKW = 0;
    //     for (i = myDetectorsOnEdges.begin(); i != myDetectorsOnEdges.end(); ++i) {
    //         ROEdge* into = (*i).first;
    //         const double maxSpeedPKW = into->getVClassMaxSpeed(SVC_PASSENGER);
    //         const double maxSpeedLKW = into->getVClassMaxSpeed(SVC_TRUCK);
    //
    //         const std::vector<std::string>& dets = (*i).second;
    //         std::map<double, std::vector<std::string> > cliques;
    //         std::vector<std::string>* maxClique = nullptr;
    //         for (std::vector<std::string>::const_iterator j = dets.begin(); j != dets.end(); ++j) {
    //             if (!flows.knows(*j)) {
    //                 continue;
    //             }
    //             const RODFDetector& det = detectors.getDetector(*j);
    //             bool found = false;
    //             for (auto& k : cliques) {
    //                 if (fabs(k.first - det.getPos()) < 1) {
    //                     k.second.push_back(*j);
    //                     if (maxClique == nullptr || k.second.size() > maxClique->size()) {
    //                         maxClique = &k.second;
    //                     }
    //                     found = true;
    //                     break;
    //                 }
    //             }
    //             if (!found) {
    //                 cliques[det.getPos()].push_back(*j);
    //                 maxClique = &cliques[det.getPos()];
    //             }
    //         }
    //         if (maxClique == nullptr) {
    //             continue;
    //         }
    //         std::vector<FlowDef> mflows; // !!! reserve
    //         for (SUMOTime t = startTime; t < endTime; t += stepOffset) {
    //             FlowDef fd;
    //             fd.qPKW = 0;
    //             fd.qLKW = 0;
    //             fd.vLKW = 0;
    //             fd.vPKW = 0;
    //             fd.fLKW = 0;
    //             fd.isLKW = 0;
    //             mflows.push_back(fd);
    //         }
    //         for (std::vector<std::string>::iterator l = maxClique->begin(); l != maxClique->end(); ++l) {
    //             bool didWarn = false;
    //             const std::vector<FlowDef>& dflows = flows.getFlowDefs(*l);
    //             int index = 0;
    //             for (SUMOTime t = startTime; t < endTime; t += stepOffset, index++) {
    //                 const FlowDef& srcFD = dflows[index];
    //                 FlowDef& fd = mflows[index];
    //                 fd.qPKW += srcFD.qPKW;
    //                 fd.qLKW += srcFD.qLKW;
    //                 fd.vLKW += srcFD.vLKW / (double) maxClique->size();
    //                 fd.vPKW += srcFD.vPKW / (double) maxClique->size();
    //                 fd.fLKW += srcFD.fLKW / (double) maxClique->size();
    //                 fd.isLKW += srcFD.isLKW / (double) maxClique->size();
    //                 const double speedFactorPKW = srcFD.vPKW / 3.6 / maxSpeedPKW;
    //                 const double speedFactorLKW = srcFD.vLKW / 3.6 / maxSpeedLKW;
    //                 myMaxSpeedFactorPKW = MAX2(myMaxSpeedFactorPKW, speedFactorPKW);
    //                 myMaxSpeedFactorLKW = MAX2(myMaxSpeedFactorLKW, speedFactorLKW);
    //                 speedFactorCountPKW += srcFD.qPKW;
    //                 speedFactorCountLKW += srcFD.qLKW;
    //                 speedFactorSumPKW += srcFD.qPKW * speedFactorPKW;
    //                 speedFactorSumLKW += srcFD.qLKW * speedFactorLKW;
    //                 if (!didWarn && srcFD.vPKW > 0 && srcFD.vPKW < 255 && srcFD.vPKW / 3.6 > into->getSpeedLimit()) {
    //                     WRITE_MESSAGE("Detected PKW speed (" + toString(srcFD.vPKW / 3.6, 3) + ") higher than allowed speed (" + toString(into->getSpeedLimit(), 3) + ") at '" + (*l) + "' on edge '" + into->getID() + "'.");
    //                     didWarn = true;
    //                 }
    //                 if (!didWarn && srcFD.vLKW > 0 && srcFD.vLKW < 255 && srcFD.vLKW / 3.6 > into->getSpeedLimit()) {
    //                     WRITE_MESSAGE("Detected LKW speed (" + toString(srcFD.vLKW / 3.6, 3) + ") higher than allowed speed (" + toString(into->getSpeedLimit(), 3) + ") at '" + (*l) + "' on edge '" + into->getID() + "'.");
    //                     didWarn = true;
    //                 }
    //             }
    //         }
    //         static_cast<RODFEdge*>(into)->setFlows(mflows);
    //     }
    //     // @note: this assumes that the speedFactors are independent of location and time
    //     if (speedFactorCountPKW > 0) {
    //         myAvgSpeedFactorPKW = speedFactorSumPKW / speedFactorCountPKW;
    //         WRITE_MESSAGEF(TL("Average speedFactor for PKW is % maximum speedFactor is %."), toString(myAvgSpeedFactorPKW), toString(myMaxSpeedFactorPKW));
    //     }
    //     if (speedFactorCountLKW > 0) {
    //         myAvgSpeedFactorLKW = speedFactorSumLKW / speedFactorCountLKW;
    //         WRITE_MESSAGEF(TL("Average speedFactor for LKW is % maximum speedFactor is %."), toString(myAvgSpeedFactorLKW), toString(myMaxSpeedFactorLKW));
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn build_edge_flow_map(&mut self, flows: &RODFDetectorFlows, detectors: &RODFDetectorCon, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime) {
        todo!("port RODFNet::buildEdgeFlowMap")
    }

    /// C++: `void RODFNet::revalidateFlows(const RODFDetectorCon &, RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime)`
    // void
    // RODFNet::revalidateFlows(const RODFDetectorCon& detectors,
    //                          RODFDetectorFlows& flows,
    //                          SUMOTime startTime, SUMOTime endTime,
    //                          SUMOTime stepOffset) {
    //     const std::vector<RODFDetector*>& dets = detectors.getDetectors();
    //     for (std::vector<RODFDetector*>::const_iterator i = dets.begin(); i != dets.end(); ++i) {
    //         // check whether there is at least one entry with a flow larger than zero
    //         revalidateFlows(*i, flows, startTime, endTime, stepOffset);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn revalidate_flows(&mut self, detectors: &RODFDetectorCon, flows: &mut RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime) {
        todo!("port RODFNet::revalidateFlows")
    }

    /// C++: `void RODFNet::removeEmptyDetectors(RODFDetectorCon &, RODFDetectorFlows &)`
    // void
    // RODFNet::removeEmptyDetectors(RODFDetectorCon& detectors,
    //                               RODFDetectorFlows& flows) {
    //     const std::vector<RODFDetector*>& dets = detectors.getDetectors();
    //     for (std::vector<RODFDetector*>::const_iterator i = dets.begin(); i != dets.end();) {
    //         bool remove = true;
    //         // check whether there is at least one entry with a flow larger than zero
    //         if (flows.knows((*i)->getID())) {
    //             remove = false;
    //         }
    //         if (remove) {
    //             WRITE_MESSAGEF(TL("Removed detector '%' because no flows for him exist."), (*i)->getID());
    //             flows.removeFlow((*i)->getID());
    //             detectors.removeDetector((*i)->getID());
    //             i = dets.begin();
    //         } else {
    //             i++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_empty_detectors(&mut self, detectors: &mut RODFDetectorCon, flows: &mut RODFDetectorFlows) {
        todo!("port RODFNet::removeEmptyDetectors")
    }

    /// C++: `void RODFNet::reportEmptyDetectors(RODFDetectorCon &, RODFDetectorFlows &)`
    // void
    // RODFNet::reportEmptyDetectors(RODFDetectorCon& detectors,
    //                               RODFDetectorFlows& flows) {
    //     const std::vector<RODFDetector*>& dets = detectors.getDetectors();
    //     for (std::vector<RODFDetector*>::const_iterator i = dets.begin(); i != dets.end(); ++i) {
    //         bool remove = true;
    //         // check whether there is at least one entry with a flow larger than zero
    //         if (flows.knows((*i)->getID())) {
    //             remove = false;
    //         }
    //         if (remove) {
    //             WRITE_MESSAGEF(TL("Detector '%' has no flow."), (*i)->getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn report_empty_detectors(&mut self, detectors: &mut RODFDetectorCon, flows: &mut RODFDetectorFlows) {
        todo!("port RODFNet::reportEmptyDetectors")
    }

    /// C++: `void RODFNet::buildDetectorDependencies(RODFDetectorCon &)`
    // void
    // RODFNet::buildDetectorDependencies(RODFDetectorCon& detectors) {
    //     // !!! this will not work when several detectors are lying on the same edge on different positions
    //
    //
    //     buildDetectorEdgeDependencies(detectors);
    //     // for each detector, compute the lists of predecessor and following detectors
    //     std::map<std::string, ROEdge*>::const_iterator i;
    //     for (i = myDetectorEdges.begin(); i != myDetectorEdges.end(); ++i) {
    //         const RODFDetector& det = detectors.getDetector((*i).first);
    //         if (!det.hasRoutes()) {
    //             continue;
    //         }
    //         // mark current detectors
    //         std::vector<RODFDetector*> last;
    //         {
    //             const std::vector<std::string>& detNames = myDetectorsOnEdges.find((*i).second)->second;
    //             for (std::vector<std::string>::const_iterator j = detNames.begin(); j != detNames.end(); ++j) {
    //                 last.push_back(&detectors.getModifiableDetector(*j));
    //             }
    //         }
    //         // iterate over the current detector's routes
    //         const std::vector<RODFRouteDesc>& routes = det.getRouteVector();
    //         for (std::vector<RODFRouteDesc>::const_iterator j = routes.begin(); j != routes.end(); ++j) {
    //             const ROEdgeVector& edges2Pass = (*j).edges2Pass;
    //             for (ROEdgeVector::const_iterator k = edges2Pass.begin() + 1; k != edges2Pass.end(); ++k) {
    //                 if (myDetectorsOnEdges.find(*k) != myDetectorsOnEdges.end()) {
    //                     const std::vector<std::string>& detNames = myDetectorsOnEdges.find(*k)->second;
    //                     // ok, consecutive detector found
    //                     for (std::vector<RODFDetector*>::iterator l = last.begin(); l != last.end(); ++l) {
    //                         // mark as follower of current
    //                         for (std::vector<std::string>::const_iterator m = detNames.begin(); m != detNames.end(); ++m) {
    //                             detectors.getModifiableDetector(*m).addPriorDetector(*l);
    //                             (*l)->addFollowingDetector(&detectors.getDetector(*m));
    //                         }
    //                     }
    //                     last.clear();
    //                     for (std::vector<std::string>::const_iterator m = detNames.begin(); m != detNames.end(); ++m) {
    //                         last.push_back(&detectors.getModifiableDetector(*m));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_detector_dependencies(&mut self, detectors: &mut RODFDetectorCon) {
        todo!("port RODFNet::buildDetectorDependencies")
    }

    /// C++: `void RODFNet::mesoJoin(RODFDetectorCon &, RODFDetectorFlows &)`
    // void
    // RODFNet::mesoJoin(RODFDetectorCon& detectors, RODFDetectorFlows& flows) {
    //     buildDetectorEdgeDependencies(detectors);
    //     std::map<ROEdge*, std::vector<std::string>, idComp>::iterator i;
    //     for (i = myDetectorsOnEdges.begin(); i != myDetectorsOnEdges.end(); ++i) {
    //         const std::vector<std::string>& dets = (*i).second;
    //         std::map<double, std::vector<std::string> > cliques;
    //         // compute detector cliques
    //         for (std::vector<std::string>::const_iterator j = dets.begin(); j != dets.end(); ++j) {
    //             const RODFDetector& det = detectors.getDetector(*j);
    //             bool found = false;
    //             for (std::map<double, std::vector<std::string> >::iterator k = cliques.begin(); !found && k != cliques.end(); ++k) {
    //                 if (fabs((*k).first - det.getPos()) < 10.) {
    //                     (*k).second.push_back(*j);
    //                     found = true;
    //                 }
    //             }
    //             if (!found) {
    //                 cliques[det.getPos()] = std::vector<std::string>();
    //                 cliques[det.getPos()].push_back(*j);
    //             }
    //         }
    //         // join detector cliques
    //         for (std::map<double, std::vector<std::string> >::iterator m = cliques.begin(); m != cliques.end(); ++m) {
    //             std::vector<std::string> clique = (*m).second;
    //             // do not join if only one
    //             if (clique.size() == 1) {
    //                 continue;
    //             }
    //             std::string nid;
    //             for (std::vector<std::string>::iterator n = clique.begin(); n != clique.end(); ++n) {
    //                 std::cout << *n << " ";
    //                 if (n != clique.begin()) {
    //                     nid = nid + "_";
    //                 }
    //                 nid = nid + *n;
    //             }
    //             std::cout << ":" << nid << std::endl;
    //             flows.mesoJoin(nid, (*m).second);
    //             detectors.mesoJoin(nid, (*m).second);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn meso_join(&mut self, detectors: &mut RODFDetectorCon, flows: &mut RODFDetectorFlows) {
        todo!("port RODFNet::mesoJoin")
    }

    /// C++: `bool RODFNet::hasDetector(ROEdge *) const` [const]
    // bool
    // RODFNet::hasDetector(ROEdge* edge) const {
    //     return myDetectorsOnEdges.find(edge) != myDetectorsOnEdges.end()
    //            && myDetectorsOnEdges.find(edge)->second.size() != 0;
    // }
    #[allow(unused_variables)]
    pub fn has_detector(&self, edge: Ptr<ROEdge>) -> bool {
        todo!("port RODFNet::hasDetector")
    }

    /// C++: `const std::vector<std::string> & RODFNet::getDetectorList(ROEdge *) const` [const]
    // const std::vector<std::string>&
    // RODFNet::getDetectorList(ROEdge* edge) const {
    //     return myDetectorsOnEdges.find(edge)->second;
    // }
    #[allow(unused_variables)]
    pub fn get_detector_list(&self, edge: Ptr<ROEdge>) -> &Vec<String> {
        todo!("port RODFNet::getDetectorList")
    }

    /// C++: `double RODFNet::getMaxSpeedFactorPKW() const` [const]
    // double getMaxSpeedFactorPKW() const {
    //         return myMaxSpeedFactorPKW;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed_factor_pkw(&self) -> f64 {
        todo!("port RODFNet::getMaxSpeedFactorPKW")
    }

    /// C++: `double RODFNet::getMaxSpeedFactorLKW() const` [const]
    // double getMaxSpeedFactorLKW() const {
    //         return myMaxSpeedFactorLKW;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed_factor_lkw(&self) -> f64 {
        todo!("port RODFNet::getMaxSpeedFactorLKW")
    }

    /// C++: `double RODFNet::getAvgSpeedFactorPKW() const` [const]
    // double getAvgSpeedFactorPKW() const {
    //         return myAvgSpeedFactorPKW;
    //     }
    #[allow(unused_variables)]
    pub fn get_avg_speed_factor_pkw(&self) -> f64 {
        todo!("port RODFNet::getAvgSpeedFactorPKW")
    }

    /// C++: `double RODFNet::getAvgSpeedFactorLKW() const` [const]
    // double getAvgSpeedFactorLKW() const {
    //         return myAvgSpeedFactorLKW;
    //     }
    #[allow(unused_variables)]
    pub fn get_avg_speed_factor_lkw(&self) -> f64 {
        todo!("port RODFNet::getAvgSpeedFactorLKW")
    }

    /// C++: `void RODFNet::revalidateFlows(const RODFDetector *, RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime)` [protected]
    // void
    // RODFNet::revalidateFlows(const RODFDetector* detector,
    //                          RODFDetectorFlows& flows,
    //                          SUMOTime startTime, SUMOTime endTime,
    //                          SUMOTime stepOffset) {
    //     {
    //         if (flows.knows(detector->getID())) {
    //             const std::vector<FlowDef>& detFlows = flows.getFlowDefs(detector->getID());
    //             for (std::vector<FlowDef>::const_iterator j = detFlows.begin(); j != detFlows.end(); ++j) {
    //                 if ((*j).qPKW > 0 || (*j).qLKW > 0) {
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    //     // ok, there is no information for the whole time;
    //     //  lets find preceding detectors and rebuild the flows if possible
    //     WRITE_WARNINGF(TL("Detector '%' has no flows.\n Trying to rebuild."), detector->getID());
    //     // go back and collect flows
    //     ROEdgeVector previous;
    //     {
    //         std::vector<IterationEdge> missing;
    //         IterationEdge ie;
    //         ie.depth = 0;
    //         ie.edge = getDetectorEdge(*detector);
    //         missing.push_back(ie);
    //         bool maxDepthReached = false;
    //         while (!missing.empty() && !maxDepthReached) {
    //             IterationEdge last = missing.back();
    //             missing.pop_back();
    //             ROEdgeVector approaching = myApproachingEdges[last.edge];
    //             for (ROEdgeVector::const_iterator j = approaching.begin(); j != approaching.end(); ++j) {
    //                 if (hasDetector(*j)) {
    //                     previous.push_back(*j);
    //                 } else {
    //                     ie.depth = last.depth + 1;
    //                     ie.edge = *j;
    //                     missing.push_back(ie);
    //                     if (ie.depth > 5) {
    //                         maxDepthReached = true;
    //                     }
    //                 }
    //             }
    //         }
    //         if (maxDepthReached) {
    //             WRITE_WARNING(TL(" Could not build list of previous flows."));
    //         }
    //     }
    //     // Edges with previous detectors are now in "previous";
    //     //  compute following
    //     ROEdgeVector latter;
    //     {
    //         std::vector<IterationEdge> missing;
    //         for (ROEdgeVector::const_iterator k = previous.begin(); k != previous.end(); ++k) {
    //             IterationEdge ie;
    //             ie.depth = 0;
    //             ie.edge = *k;
    //             missing.push_back(ie);
    //         }
    //         bool maxDepthReached = false;
    //         while (!missing.empty() && !maxDepthReached) {
    //             IterationEdge last = missing.back();
    //             missing.pop_back();
    //             ROEdgeVector approached = myApproachedEdges[last.edge];
    //             for (ROEdgeVector::const_iterator j = approached.begin(); j != approached.end(); ++j) {
    //                 if (*j == getDetectorEdge(*detector)) {
    //                     continue;
    //                 }
    //                 if (hasDetector(*j)) {
    //                     latter.push_back(*j);
    //                 } else {
    //                     IterationEdge ie;
    //                     ie.depth = last.depth + 1;
    //                     ie.edge = *j;
    //                     missing.push_back(ie);
    //                     if (ie.depth > 5) {
    //                         maxDepthReached = true;
    //                     }
    //                 }
    //             }
    //         }
    //         if (maxDepthReached) {
    //             WRITE_WARNING(TL(" Could not build list of latter flows."));
    //             return;
    //         }
    //     }
    //     // Edges with latter detectors are now in "latter";
    //
    //     // lets not validate them by now - surely this should be done
    //     // for each time step: collect incoming flows; collect outgoing;
    //     std::vector<FlowDef> mflows;
    //     int index = 0;
    //     for (SUMOTime t = startTime; t < endTime; t += stepOffset, index++) {
    //         // collect incoming
    //         FlowDef inFlow;
    //         inFlow.qLKW = 0;
    //         inFlow.qPKW = 0;
    //         inFlow.vLKW = 0;
    //         inFlow.vPKW = 0;
    //         // !! time difference is missing
    //         for (const ROEdge* const e : previous) {
    //             const std::vector<FlowDef>& eflows = static_cast<const RODFEdge*>(e)->getFlows();
    //             if (eflows.size() != 0) {
    //                 const FlowDef& srcFD = eflows[index];
    //                 inFlow.qLKW += srcFD.qLKW;
    //                 inFlow.qPKW += srcFD.qPKW;
    //                 inFlow.vLKW += srcFD.vLKW;
    //                 inFlow.vPKW += srcFD.vPKW;
    //             }
    //         }
    //         inFlow.vLKW /= (double) previous.size();
    //         inFlow.vPKW /= (double) previous.size();
    //         // collect outgoing
    //         FlowDef outFlow;
    //         outFlow.qLKW = 0;
    //         outFlow.qPKW = 0;
    //         outFlow.vLKW = 0;
    //         outFlow.vPKW = 0;
    //         // !! time difference is missing
    //         for (const ROEdge* const e : latter) {
    //             const std::vector<FlowDef>& eflows = static_cast<const RODFEdge*>(e)->getFlows();
    //             if (eflows.size() != 0) {
    //                 const FlowDef& srcFD = eflows[index];
    //                 outFlow.qLKW += srcFD.qLKW;
    //                 outFlow.qPKW += srcFD.qPKW;
    //                 outFlow.vLKW += srcFD.vLKW;
    //                 outFlow.vPKW += srcFD.vPKW;
    //             }
    //         }
    //         outFlow.vLKW /= (double) latter.size();
    //         outFlow.vPKW /= (double) latter.size();
    //         //
    //         FlowDef mFlow;
    //         mFlow.qLKW = inFlow.qLKW - outFlow.qLKW;
    //         mFlow.qPKW = inFlow.qPKW - outFlow.qPKW;
    //         mFlow.vLKW = (inFlow.vLKW + outFlow.vLKW) / (double) 2.;
    //         mFlow.vPKW = (inFlow.vPKW + outFlow.vPKW) / (double) 2.;
    //         mflows.push_back(mFlow);
    //     }
    //     static_cast<RODFEdge*>(getDetectorEdge(*detector))->setFlows(mflows);
    //     flows.setFlows(detector->getID(), mflows);
    // }
    #[allow(unused_variables)]
    pub fn revalidate_flows_2(&mut self, detector: Ptr<RODFDetector>, flows: &mut RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime) {
        todo!("port RODFNet::revalidateFlows")
    }

    /// C++: `bool RODFNet::isSource(const RODFDetector &, const RODFDetectorCon &, bool) const` [const, protected]
    // bool
    // RODFNet::isSource(const RODFDetector& det, const RODFDetectorCon& detectors,
    //                   bool strict) const {
    //     ROEdgeVector seen;
    //     return isSource(det, getDetectorEdge(det), seen, detectors, strict);
    // }
    #[allow(unused_variables)]
    pub fn is_source(&self, det: &RODFDetector, detectors: &RODFDetectorCon, strict: bool) -> bool {
        todo!("port RODFNet::isSource")
    }

    /// C++: `bool RODFNet::isFalseSource(const RODFDetector &, const RODFDetectorCon &) const` [const, protected]
    // bool
    // RODFNet::isFalseSource(const RODFDetector& det, const RODFDetectorCon& detectors) const {
    //     ROEdgeVector seen;
    //     return isFalseSource(det, getDetectorEdge(det), seen, detectors);
    // }
    #[allow(unused_variables)]
    pub fn is_false_source(&self, det: &RODFDetector, detectors: &RODFDetectorCon) -> bool {
        todo!("port RODFNet::isFalseSource")
    }

    /// C++: `bool RODFNet::isDestination(const RODFDetector &, const RODFDetectorCon &) const` [const, protected]
    // bool
    // RODFNet::isDestination(const RODFDetector& det, const RODFDetectorCon& detectors) const {
    //     ROEdgeVector seen;
    //     return isDestination(det, getDetectorEdge(det), seen, detectors);
    // }
    #[allow(unused_variables)]
    pub fn is_destination(&self, det: &RODFDetector, detectors: &RODFDetectorCon) -> bool {
        todo!("port RODFNet::isDestination")
    }

    /// C++: `ROEdge * RODFNet::getDetectorEdge(const RODFDetector &) const` [const, protected]
    // ROEdge*
    // RODFNet::getDetectorEdge(const RODFDetector& det) const {
    //     const std::string edgeName = SUMOXMLDefinitions::getEdgeIDFromLane(det.getLaneID());
    //     ROEdge* ret = getEdge(edgeName);
    //     if (ret == nullptr) {
    //         throw ProcessError("Edge '" + edgeName + "' used by detector '" + det.getID() + "' is not known.");
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_detector_edge(&self, det: &RODFDetector) -> Ptr<ROEdge> {
        todo!("port RODFNet::getDetectorEdge")
    }

    /// C++: `bool RODFNet::isSource(const RODFDetector &, ROEdge *, ROEdgeVector &, const RODFDetectorCon &, bool) const` [const, protected]
    // bool
    // RODFNet::isSource(const RODFDetector& det, ROEdge* edge,
    //                   ROEdgeVector& seen,
    //                   const RODFDetectorCon& detectors,
    //                   bool strict) const {
    //     if (seen.size() == 1000) { // !!!
    //         WRITE_WARNINGF(TL("Quitting checking for being a source for detector '%' due to seen edge limit."), det.getID());
    //         return false;
    //     }
    //     if (edge == getDetectorEdge(det)) {
    //         // maybe there is another detector at the same edge
    //         //  get the list of this/these detector(s)
    //         const std::vector<std::string>& detsOnEdge = myDetectorsOnEdges.find(edge)->second;
    //         for (std::vector<std::string>::const_iterator i = detsOnEdge.begin(); i != detsOnEdge.end(); ++i) {
    //             if ((*i) == det.getID()) {
    //                 continue;
    //             }
    //             const RODFDetector& sec = detectors.getDetector(*i);
    //             if (getAbsPos(sec) < getAbsPos(det)) {
    //                 // ok, there is another detector on the same edge and it is
    //                 //  before this one -> no source
    //                 return false;
    //             }
    //         }
    //     }
    //     // it's a source if no edges are approaching the edge
    //     if (!hasApproaching(edge)) {
    //         if (edge != getDetectorEdge(det)) {
    //             if (hasDetector(edge)) {
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    //     if (edge != getDetectorEdge(det)) {
    //         // ok, we are at one of the edges in front
    //         if (myAmInHighwayMode) {
    //             if (edge->getSpeedLimit() >= 19.4) {
    //                 if (hasDetector(edge)) {
    //                     // we are still on the highway and there is another detector
    //                     return false;
    //                 }
    //                 // the next is a hack for the A100 scenario...
    //                 //  We have to look into further edges herein edges
    //                 const ROEdgeVector& appr = myApproachingEdges.find(edge)->second;
    //                 int noFalse = 0;
    //                 int noSkipped = 0;
    //                 for (int i = 0; i < (int)appr.size(); i++) {
    //                     if (hasDetector(appr[i])) {
    //                         noFalse++;
    //                     }
    //                 }
    //                 if (noFalse + noSkipped == (int)appr.size()) {
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //
    //     if (myAmInHighwayMode) {
    //         if (edge->getSpeedLimit() < 19.4 && edge != getDetectorEdge(det)) {
    //             // we have left the highway already
    //             //  -> the detector will be a highway source
    //             if (!hasDetector(edge)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     if (myDetectorsOnEdges.find(edge) != myDetectorsOnEdges.end()
    //             &&
    //             myDetectorEdges.find(det.getID())->second != edge) {
    //         return false;
    //     }
    //
    //     // let's check the edges in front
    //     const ROEdgeVector& appr = myApproachingEdges.find(edge)->second;
    //     int numOk = 0;
    //     int numFalse = 0;
    //     int numSkipped = 0;
    //     seen.push_back(edge);
    //     for (int i = 0; i < (int)appr.size(); i++) {
    //         bool had = std::find(seen.begin(), seen.end(), appr[i]) != seen.end();
    //         if (!had) {
    //             if (isSource(det, appr[i], seen, detectors, strict)) {
    //                 numOk++;
    //             } else {
    //                 numFalse++;
    //             }
    //         } else {
    //             numSkipped++;
    //         }
    //     }
    //     if (strict) {
    //         return numOk + numSkipped == (int)appr.size();
    //     }
    //     return numFalse + numSkipped != (int)appr.size();
    // }
    #[allow(unused_variables)]
    pub fn is_source_2(&self, det: &RODFDetector, edge: Ptr<ROEdge>, seen: &mut ROEdgeVector, detectors: &RODFDetectorCon, strict: bool) -> bool {
        todo!("port RODFNet::isSource")
    }

    /// C++: `bool RODFNet::isFalseSource(const RODFDetector &, ROEdge *, ROEdgeVector &, const RODFDetectorCon &) const` [const, protected]
    // bool
    // RODFNet::isFalseSource(const RODFDetector& det, ROEdge* edge, ROEdgeVector& seen,
    //                        const RODFDetectorCon& detectors) const {
    //     if (seen.size() == 1000) { // !!!
    //         WRITE_WARNINGF(TL("Quitting checking for being a false source for detector '%' due to seen edge limit."), det.getID());
    //         return false;
    //     }
    //     seen.push_back(edge);
    //     if (edge != getDetectorEdge(det)) {
    //         // ok, we are at one of the edges coming behind
    //         if (hasDetector(edge)) {
    //             const std::vector<std::string>& dets = myDetectorsOnEdges.find(edge)->second;
    //             for (std::vector<std::string>::const_iterator i = dets.begin(); i != dets.end(); ++i) {
    //                 if (detectors.getDetector(*i).getType() == SINK_DETECTOR) {
    //                     return false;
    //                 }
    //                 if (detectors.getDetector(*i).getType() == BETWEEN_DETECTOR) {
    //                     return false;
    //                 }
    //                 if (detectors.getDetector(*i).getType() == SOURCE_DETECTOR) {
    //                     return true;
    //                 }
    //             }
    //         } else {
    //             if (myAmInHighwayMode && edge->getSpeedLimit() < 19.) {
    //                 return false;
    //             }
    //         }
    //     }
    //
    //     if (myApproachedEdges.find(edge) == myApproachedEdges.end()) {
    //         return false;
    //     }
    //
    //     const ROEdgeVector& appr  = myApproachedEdges.find(edge)->second;
    //     bool isall = false;
    //     for (int i = 0; i < (int)appr.size() && !isall; i++) {
    //         //printf("checking %s->\n", appr[i].c_str());
    //         bool had = std::find(seen.begin(), seen.end(), appr[i]) != seen.end();
    //         if (!had) {
    //             if (isFalseSource(det, appr[i], seen, detectors)) {
    //                 isall = true;
    //             }
    //         }
    //     }
    //     return isall;
    // }
    #[allow(unused_variables)]
    pub fn is_false_source_2(&self, det: &RODFDetector, edge: Ptr<ROEdge>, seen: &mut ROEdgeVector, detectors: &RODFDetectorCon) -> bool {
        todo!("port RODFNet::isFalseSource")
    }

    /// C++: `bool RODFNet::isDestination(const RODFDetector &, ROEdge *, ROEdgeVector &, const RODFDetectorCon &) const` [const, protected]
    // bool
    // RODFNet::isDestination(const RODFDetector& det, ROEdge* edge, ROEdgeVector& seen,
    //                        const RODFDetectorCon& detectors) const {
    //     if (seen.size() == 1000) { // !!!
    //         WRITE_WARNINGF(TL("Quitting checking for being a destination for detector '%' due to seen edge limit."), det.getID());
    //         return false;
    //     }
    //     if (edge == getDetectorEdge(det)) {
    //         // maybe there is another detector at the same edge
    //         //  get the list of this/these detector(s)
    //         const std::vector<std::string>& detsOnEdge = myDetectorsOnEdges.find(edge)->second;
    //         for (std::vector<std::string>::const_iterator i = detsOnEdge.begin(); i != detsOnEdge.end(); ++i) {
    //             if ((*i) == det.getID()) {
    //                 continue;
    //             }
    //             const RODFDetector& sec = detectors.getDetector(*i);
    //             if (getAbsPos(sec) > getAbsPos(det)) {
    //                 // ok, there is another detector on the same edge and it is
    //                 //  after this one -> no destination
    //                 return false;
    //             }
    //         }
    //     }
    //     if (!hasApproached(edge)) {
    //         if (edge != getDetectorEdge(det)) {
    //             if (hasDetector(edge)) {
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    //     if (edge != getDetectorEdge(det)) {
    //         // ok, we are at one of the edges coming behind
    //         if (myAmInHighwayMode) {
    //             if (edge->getSpeedLimit() >= 19.4) {
    //                 if (hasDetector(edge)) {
    //                     // we are still on the highway and there is another detector
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //
    //     if (myAmInHighwayMode) {
    //         if (edge->getSpeedLimit() < 19.4 && edge != getDetectorEdge(det)) {
    //             if (hasDetector(edge)) {
    //                 return true;
    //             }
    //             if (myApproachedEdges.find(edge)->second.size() > 1) {
    //                 return true;
    //             }
    //
    //         }
    //     }
    //
    //     if (myDetectorsOnEdges.find(edge) != myDetectorsOnEdges.end()
    //             &&
    //             myDetectorEdges.find(det.getID())->second != edge) {
    //         return false;
    //     }
    //     const ROEdgeVector& appr  = myApproachedEdges.find(edge)->second;
    //     bool isall = true;
    //     seen.push_back(edge);
    //     for (int i = 0; i < (int)appr.size() && isall; i++) {
    //         bool had = std::find(seen.begin(), seen.end(), appr[i]) != seen.end();
    //         if (!had) {
    //             if (!isDestination(det, appr[i], seen, detectors)) {
    //                 isall = false;
    //             }
    //         }
    //     }
    //     return isall;
    // }
    #[allow(unused_variables)]
    pub fn is_destination_2(&self, det: &RODFDetector, edge: Ptr<ROEdge>, seen: &mut ROEdgeVector, detectors: &RODFDetectorCon) -> bool {
        todo!("port RODFNet::isDestination")
    }

    /// C++: `void RODFNet::computeRoutesFor(ROEdge *, RODFRouteDesc &, int, bool, bool, ROEdgeVector &, const RODFDetector &, RODFRouteCont &, const RODFDetectorCon &, int, ROEdgeVector &) const` [const, protected]
    // void
    // RODFNet::computeRoutesFor(ROEdge* edge, RODFRouteDesc& base, int /*no*/,
    //                           bool keepUnfoundEnds,
    //                           bool keepShortestOnly,
    //                           ROEdgeVector& /*visited*/,
    //                           const RODFDetector& det, RODFRouteCont& into,
    //                           const RODFDetectorCon& detectors,
    //                           int maxFollowingLength,
    //                           ROEdgeVector& seen) const {
    //     std::vector<RODFRouteDesc> unfoundEnds;
    //     std::priority_queue<RODFRouteDesc, std::vector<RODFRouteDesc>, DFRouteDescByTimeComperator> toSolve;
    //     std::map<ROEdge*, ROEdgeVector > dets2Follow;
    //     dets2Follow[edge] = ROEdgeVector();
    //     base.passedNo = 0;
    //     double minDist = OptionsCont::getOptions().getFloat("min-route-length");
    //     toSolve.push(base);
    //     while (!toSolve.empty()) {
    //         RODFRouteDesc current = toSolve.top();
    //         toSolve.pop();
    //         ROEdge* last = *(current.edges2Pass.end() - 1);
    //         if (hasDetector(last)) {
    //             if (dets2Follow.find(last) == dets2Follow.end()) {
    //                 dets2Follow[last] = ROEdgeVector();
    //             }
    //             for (ROEdgeVector::reverse_iterator i = current.edges2Pass.rbegin() + 1; i != current.edges2Pass.rend(); ++i) {
    //                 if (hasDetector(*i)) {
    //                     dets2Follow[*i].push_back(last);
    //                     break;
    //                 }
    //             }
    //         }
    //
    //         // do not process an edge twice
    //         if (find(seen.begin(), seen.end(), last) != seen.end() && keepShortestOnly) {
    //             continue;
    //         }
    //         seen.push_back(last);
    //         // end if the edge has no further connections
    //         if (!hasApproached(last)) {
    //             // ok, no further connections to follow
    //             current.factor = 1.;
    //             double cdist = current.edges2Pass[0]->getFromJunction()->getPosition().distanceTo(current.edges2Pass.back()->getToJunction()->getPosition());
    //             if (minDist < cdist) {
    //                 into.addRouteDesc(current);
    //             }
    //             continue;
    //         }
    //         // check for passing detectors:
    //         //  if the current last edge is not the one the detector is placed on ...
    //         bool addNextNoFurther = false;
    //         if (last != getDetectorEdge(det)) {
    //             // ... if there is a detector ...
    //             if (hasDetector(last)) {
    //                 if (!hasInBetweenDetectorsOnly(last, detectors)) {
    //                     // ... and it's not an in-between-detector
    //                     // -> let's add this edge and the following, but not any further
    //                     addNextNoFurther = true;
    //                     current.lastDetectorEdge = last;
    //                     current.duration2Last = (SUMOTime) current.duration_2;
    //                     current.distance2Last = current.distance;
    //                     current.endDetectorEdge = last;
    //                     if (hasSourceDetector(last, detectors)) {
    // ///!!!                        //toDiscard.push_back(current);
    //                     }
    //                     current.factor = 1.;
    //                     double cdist = current.edges2Pass[0]->getFromJunction()->getPosition().distanceTo(current.edges2Pass.back()->getToJunction()->getPosition());
    //                     if (minDist < cdist) {
    //                         into.addRouteDesc(current);
    //                     }
    //                     continue;
    //                 } else {
    //                     // ... if it's an in-between-detector
    //                     // -> mark the current route as to be continued
    //                     current.passedNo = 0;
    //                     current.duration2Last = (SUMOTime) current.duration_2;
    //                     current.distance2Last = current.distance;
    //                     current.lastDetectorEdge = last;
    //                 }
    //             }
    //         }
    //         // check for highway off-ramps
    //         if (myAmInHighwayMode) {
    //             // if it's beside the highway...
    //             if (last->getSpeedLimit() < 19.4 && last != getDetectorEdge(det)) {
    //                 // ... and has more than one following edge
    //                 if (myApproachedEdges.find(last)->second.size() > 1) {
    //                     // -> let's add this edge and the following, but not any further
    //                     addNextNoFurther = true;
    //                 }
    //
    //             }
    //         }
    //         // check for missing end connections
    //         if (!addNextNoFurther) {
    //             // ... if this one would be processed, but already too many edge
    //             //  without a detector occurred
    //             if (current.passedNo > maxFollowingLength) {
    //                 // mark not to process any further
    //                 WRITE_WARNINGF(TL("Could not close route for '%'"), det.getID());
    //                 unfoundEnds.push_back(current);
    //                 current.factor = 1.;
    //                 double cdist = current.edges2Pass[0]->getFromJunction()->getPosition().distanceTo(current.edges2Pass.back()->getToJunction()->getPosition());
    //                 if (minDist < cdist) {
    //                     into.addRouteDesc(current);
    //                 }
    //                 continue;
    //             }
    //         }
    //         // ... else: loop over the next edges
    //         const ROEdgeVector& appr  = myApproachedEdges.find(last)->second;
    //         bool hadOne = false;
    //         for (int i = 0; i < (int)appr.size(); i++) {
    //             if (find(current.edges2Pass.begin(), current.edges2Pass.end(), appr[i]) != current.edges2Pass.end()) {
    //                 // do not append an edge twice (do not build loops)
    //                 continue;
    //             }
    //             RODFRouteDesc t(current);
    //             t.duration_2 += (appr[i]->getLength() / appr[i]->getSpeedLimit()); //!!!
    //             t.distance += appr[i]->getLength();
    //             t.edges2Pass.push_back(appr[i]);
    //             if (!addNextNoFurther) {
    //                 t.passedNo++;
    //                 toSolve.push(t);
    //             } else {
    //                 if (!hadOne) {
    //                     t.factor = (double) 1. / (double) appr.size();
    //                     double cdist = current.edges2Pass[0]->getFromJunction()->getPosition().distanceTo(current.edges2Pass.back()->getToJunction()->getPosition());
    //                     if (minDist < cdist) {
    //                         into.addRouteDesc(t);
    //                     }
    //                     hadOne = true;
    //                 }
    //             }
    //         }
    //     }
    //     //
    //     if (!keepUnfoundEnds) {
    //         std::vector<RODFRouteDesc>::iterator i;
    //         ConstROEdgeVector lastDetEdges;
    //         for (i = unfoundEnds.begin(); i != unfoundEnds.end(); ++i) {
    //             if (find(lastDetEdges.begin(), lastDetEdges.end(), (*i).lastDetectorEdge) == lastDetEdges.end()) {
    //                 lastDetEdges.push_back((*i).lastDetectorEdge);
    //             } else {
    //                 bool ok = into.removeRouteDesc(*i);
    //                 assert(ok);
    //                 UNUSED_PARAMETER(ok); // only used for assertion
    //             }
    //         }
    //     } else {
    //         // !!! patch the factors
    //     }
    //     while (!toSolve.empty()) {
    // //        RODFRouteDesc d = toSolve.top();
    //         toSolve.pop();
    // //        delete d;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_routes_for(&self, edge: Ptr<ROEdge>, base: &mut RODFRouteDesc, no: i32, keepUnfoundEnds: bool, keepShortestOnly: bool, visited: &mut ROEdgeVector, det: &RODFDetector, into: &mut RODFRouteCont, detectors: &RODFDetectorCon, maxFollowingLength: i32, seen: &mut ROEdgeVector) {
        todo!("port RODFNet::computeRoutesFor")
    }

    /// C++: `void RODFNet::buildDetectorEdgeDependencies(RODFDetectorCon &) const` [const, protected]
    // void
    // RODFNet::buildDetectorEdgeDependencies(RODFDetectorCon& detcont) const {
    //     myDetectorsOnEdges.clear();
    //     myDetectorEdges.clear();
    //     const std::vector<RODFDetector*>& dets = detcont.getDetectors();
    //     for (std::vector<RODFDetector*>::const_iterator i = dets.begin(); i != dets.end(); ++i) {
    //         ROEdge* e = getDetectorEdge(**i);
    //         myDetectorsOnEdges[e].push_back((*i)->getID());
    //         myDetectorEdges[(*i)->getID()] = e;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_detector_edge_dependencies(&self, dets: &mut RODFDetectorCon) {
        todo!("port RODFNet::buildDetectorEdgeDependencies")
    }

    /// C++: `bool RODFNet::hasApproaching(ROEdge *) const` [const, protected]
    // bool
    // RODFNet::hasApproaching(ROEdge* edge) const {
    //     return myApproachingEdges.find(edge) != myApproachingEdges.end()
    //            && myApproachingEdges.find(edge)->second.size() != 0;
    // }
    #[allow(unused_variables)]
    pub fn has_approaching(&self, edge: Ptr<ROEdge>) -> bool {
        todo!("port RODFNet::hasApproaching")
    }

    /// C++: `bool RODFNet::hasApproached(ROEdge *) const` [const, protected]
    // bool
    // RODFNet::hasApproached(ROEdge* edge) const {
    //     return myApproachedEdges.find(edge) != myApproachedEdges.end()
    //            && myApproachedEdges.find(edge)->second.size() != 0;
    // }
    #[allow(unused_variables)]
    pub fn has_approached(&self, edge: Ptr<ROEdge>) -> bool {
        todo!("port RODFNet::hasApproached")
    }

    /// C++: `bool RODFNet::hasInBetweenDetectorsOnly(ROEdge *, const RODFDetectorCon &) const` [const, protected]
    // bool
    // RODFNet::hasInBetweenDetectorsOnly(ROEdge* edge,
    //                                    const RODFDetectorCon& detectors) const {
    //     assert(myDetectorsOnEdges.find(edge) != myDetectorsOnEdges.end());
    //     const std::vector<std::string>& detIDs = myDetectorsOnEdges.find(edge)->second;
    //     std::vector<std::string>::const_iterator i;
    //     for (i = detIDs.begin(); i != detIDs.end(); ++i) {
    //         const RODFDetector& det = detectors.getDetector(*i);
    //         if (det.getType() != BETWEEN_DETECTOR) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn has_in_between_detectors_only(&self, edge: Ptr<ROEdge>, detectors: &RODFDetectorCon) -> bool {
        todo!("port RODFNet::hasInBetweenDetectorsOnly")
    }

    /// C++: `bool RODFNet::hasSourceDetector(ROEdge *, const RODFDetectorCon &) const` [const, protected]
    // bool
    // RODFNet::hasSourceDetector(ROEdge* edge,
    //                            const RODFDetectorCon& detectors) const {
    //     assert(myDetectorsOnEdges.find(edge) != myDetectorsOnEdges.end());
    //     const std::vector<std::string>& detIDs = myDetectorsOnEdges.find(edge)->second;
    //     std::vector<std::string>::const_iterator i;
    //     for (i = detIDs.begin(); i != detIDs.end(); ++i) {
    //         const RODFDetector& det = detectors.getDetector(*i);
    //         if (det.getType() == SOURCE_DETECTOR) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_source_detector(&self, edge: Ptr<ROEdge>, detectors: &RODFDetectorCon) -> bool {
        todo!("port RODFNet::hasSourceDetector")
    }

    /// C++: `bool RODFNet::isAllowed(const ROEdge *const) const` [const, protected]
    // bool
    // RODFNet::isAllowed(const ROEdge* const edge) const {
    //     return (!edge->isInternal() && !edge->isWalkingArea() && !edge->isCrossing() &&
    //             (edge->getPermissions() & myAllowedVClass) == myAllowedVClass &&
    //             find(myDisallowedEdges.begin(), myDisallowedEdges.end(), edge->getID()) == myDisallowedEdges.end());
    //
    // }
    #[allow(unused_variables)]
    pub fn is_allowed(&self, edge: Ptr<ROEdge>) -> bool {
        todo!("port RODFNet::isAllowed")
    }

}

/// comparator for maps using edges as key, used only in myDetectorsOnEdges to make tests comparable
/// C++ class `RODFNet::idComp` (dfrouter/RODFNet.h)
pub struct RODFNet_idComp {
}

impl RODFNet_idComp {
    /// C++: `bool idComp::operator()(ROEdge *const, ROEdge *const) const` [const]
    // bool operator()(ROEdge* const lhs, ROEdge* const rhs) const {
    //             return lhs->getID() < rhs->getID();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, lhs: Ptr<ROEdge>, rhs: Ptr<ROEdge>) -> bool {
        todo!("port idComp::operator()")
    }

}

