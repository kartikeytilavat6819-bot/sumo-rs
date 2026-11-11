//! Port of marouter/ROMAAssignments.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// assignment methods
/// C++ class `ROMAAssignments` (marouter/ROMAAssignments.h)
pub struct ROMAAssignments {
    pub myBegin: SUMOTime,
    pub myEnd: SUMOTime,
    pub myAdditiveTraffic: bool,
    pub myAdaptionFactor: f64,
    pub myMaxAlternatives: i32,
    pub myUseDefaultCapacities: bool,
    pub myNet: Ptr<RONet>,
    pub myMatrix: Ptr<ODMatrix>,
    pub myRouter: Ptr<SUMOAbstractRouter<ROEdge, ROVehicle>>,
    pub myDefaultVehicle: Ptr<ROVehicle>,
    pub myNetloadOutput: Ptr<OutputDevice>,
}

// C++ static members of ROMAAssignments (port as module-level statics, see AGENTS.md):
//   static std::map<const ROEdge *const, double> ROMAAssignments::myPenalties;

impl ROMAAssignments {
    /// C++: `void ROMAAssignments::ROMAAssignments(const SUMOTime, const SUMOTime, const bool, const double, const int, const bool, RONet &, ODMatrix &, SUMOAbstractRouter<ROEdge, ROVehicle> &, OutputDevice *)`
    /// Constructor
    // ROMAAssignments::ROMAAssignments(const SUMOTime begin, const SUMOTime end, const bool additiveTraffic,
    //                                  const double adaptionFactor, const int maxAlternatives, const bool defaultCapacities,
    //                                  RONet& net, ODMatrix& matrix, SUMOAbstractRouter<ROEdge, ROVehicle>& router,
    //                                  OutputDevice* netloadOutput)
    //     : myBegin(begin), myEnd(end), myAdditiveTraffic(additiveTraffic), myAdaptionFactor(adaptionFactor),
    //       myMaxAlternatives(maxAlternatives), myUseDefaultCapacities(defaultCapacities),
    //       myNet(net), myMatrix(matrix), myRouter(router), myNetloadOutput(netloadOutput) {
    //     myDefaultVehicle = new ROVehicle(SUMOVehicleParameter(), nullptr, net.getVehicleTypeSecure(DEFAULT_VTYPE_ID), &net);
    // }
    #[allow(unused_variables)]
    pub fn new(begin: SUMOTime, end: SUMOTime, additiveTraffic: bool, adaptionFactor: f64, maxAlternatives: i32, defaultCapacities: bool, net: &mut RONet, matrix: &mut ODMatrix, router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, netloadOutput: Ptr<OutputDevice>) -> Self {
        todo!("port ROMAAssignments::ROMAAssignments")
    }

    /// C++: `void ROMAAssignments::~ROMAAssignments() noexcept`
    /// Destructor
    // ROMAAssignments::~ROMAAssignments() {
    //     delete myDefaultVehicle;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROMAAssignments::~ROMAAssignments")
    }

    /// C++: `ROVehicle * ROMAAssignments::getDefaultVehicle() const` [const]
    // ROVehicle* getDefaultVehicle() const {
    //         return myDefaultVehicle;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_vehicle(&self) -> Ptr<ROVehicle> {
        todo!("port ROMAAssignments::getDefaultVehicle")
    }

    /// C++: `double ROMAAssignments::getCapacity(const ROEdge *) const` [const]
    // double
    // ROMAAssignments::getCapacity(const ROEdge* edge) const {
    //     if (edge->isTazConnector()) {
    //         return 0;
    //     }
    //     const int roadClass = myUseDefaultCapacities ? -1 : -edge->getPriority();
    //     // TODO: differ road class 1 from the unknown road class 1!!!
    //     if (edge->getNumLanes() == 0) {
    //         // TAZ have no cost
    //         return 0;
    //     } else if (roadClass == 0 || roadClass == 1)  {
    //         return edge->getNumLanes() * 2000.; //CR13 in table.py
    //     } else if (roadClass == 2 && edge->getSpeedLimit() <= 11.) {
    //         return edge->getNumLanes() * 1333.33; //CR5 in table.py
    //     } else if (roadClass == 2 && edge->getSpeedLimit() > 11. && edge->getSpeedLimit() <= 16.) {
    //         return edge->getNumLanes() * 1500.; //CR3 in table.py
    //     } else if (roadClass == 2 && edge->getSpeedLimit() > 16.) {
    //         return edge->getNumLanes() * 2000.; //CR13 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() <= 11.) {
    //         return edge->getNumLanes() * 800.; //CR5 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() > 11. && edge->getSpeedLimit() <= 13.) {
    //         return edge->getNumLanes() * 875.; //CR5 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() > 13. && edge->getSpeedLimit() <= 16.) {
    //         return edge->getNumLanes() * 1500.; //CR4 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() > 16.) {
    //         return edge->getNumLanes() * 1800.; //CR13 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() <= 5.) {
    //         return edge->getNumLanes() * 200.; //CR7 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 5. && edge->getSpeedLimit() <= 7.) {
    //         return edge->getNumLanes() * 412.5; //CR7 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 7. && edge->getSpeedLimit() <= 9.) {
    //         return edge->getNumLanes() * 600.; //CR6 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 9. && edge->getSpeedLimit() <= 11.) {
    //         return edge->getNumLanes() * 800.; //CR5 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 11. && edge->getSpeedLimit() <= 13.) {
    //         return edge->getNumLanes() * 1125.; //CR5 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 13. && edge->getSpeedLimit() <= 16.) {
    //         return edge->getNumLanes() * 1583.; //CR4 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 16. && edge->getSpeedLimit() <= 18.) {
    //         return edge->getNumLanes() * 1100.; //CR3 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 18. && edge->getSpeedLimit() <= 22.) {
    //         return edge->getNumLanes() * 1200.; //CR3 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 22. && edge->getSpeedLimit() <= 26.) {
    //         return edge->getNumLanes() * 1300.; //CR3 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 26.) {
    //         return edge->getNumLanes() * 1400.; //CR3 in table.py
    //     }
    //     return edge->getNumLanes() * 800.; //CR5 in table.py
    // }
    #[allow(unused_variables)]
    pub fn get_capacity(&self, edge: Ptr<ROEdge>) -> f64 {
        todo!("port ROMAAssignments::getCapacity")
    }

    /// C++: `double ROMAAssignments::capacityConstraintFunction(const ROEdge *, const double) const` [const]
    // double
    // ROMAAssignments::capacityConstraintFunction(const ROEdge* edge, const double flow) const {
    //     if (edge->isTazConnector()) {
    //         return 0;
    //     }
    //     const int roadClass = myUseDefaultCapacities ? -1 : -edge->getPriority();
    //     const double capacity = getCapacity(edge);
    //     // TODO: differ road class 1 from the unknown road class 1!!!
    //     if (edge->getNumLanes() == 0) {
    //         // TAZ have no cost
    //         return 0;
    //     } else if (roadClass == 0 || roadClass == 1)  {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.3)) * 2.); //CR13 in table.py
    //     } else if (roadClass == 2 && edge->getSpeedLimit() <= 11.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.9)) * 3.); //CR5 in table.py
    //     } else if (roadClass == 2 && edge->getSpeedLimit() > 11. && edge->getSpeedLimit() <= 16.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.)) * 2.); //CR3 in table.py
    //     } else if (roadClass == 2 && edge->getSpeedLimit() > 16.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.3)) * 2.); //CR13 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() <= 11.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.9)) * 3.); //CR5 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() > 11. && edge->getSpeedLimit() <= 13.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.9)) * 3.); //CR5 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() > 13. && edge->getSpeedLimit() <= 16.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.7 * (flow / (capacity * 1.)) * 2.); //CR4 in table.py
    //     } else if (roadClass == 3 && edge->getSpeedLimit() > 16.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.3)) * 2.); //CR13 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() <= 5.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.5)) * 3.); //CR7 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 5. && edge->getSpeedLimit() <= 7.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.5)) * 3.); //CR7 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 7. && edge->getSpeedLimit() <= 9.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.8)) * 3.); //CR6 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 9. && edge->getSpeedLimit() <= 11.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.9)) * 3.); //CR5 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 11. && edge->getSpeedLimit() <= 13.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.9)) * 3.); //CR5 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 13. && edge->getSpeedLimit() <= 16.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.7 * (flow / (capacity * 1.)) * 2.); //CR4 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 16. && edge->getSpeedLimit() <= 18.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.)) * 2.); //CR3 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 18. && edge->getSpeedLimit() <= 22.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.)) * 2.); //CR3 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 22. && edge->getSpeedLimit() <= 26.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.)) * 2.); //CR3 in table.py
    //     } else if ((roadClass >= 4 || roadClass == -1) && edge->getSpeedLimit() > 26.) {
    //         return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 1.)) * 2.); //CR3 in table.py
    //     }
    //     return edge->getLength() / edge->getSpeedLimit() * (1. + 1.*(flow / (capacity * 0.9)) * 3.); //CR5 in table.py
    // }
    #[allow(unused_variables)]
    pub fn capacity_constraint_function(&self, edge: Ptr<ROEdge>, flow: f64) -> f64 {
        todo!("port ROMAAssignments::capacityConstraintFunction")
    }

    /// C++: `void ROMAAssignments::resetFlows()`
    // void
    // ROMAAssignments::resetFlows() {
    //     const double begin = STEPS2TIME(MIN2(myBegin, myMatrix.getCells().front()->begin));
    //     for (ROEdge* e : ROEdge::getAllEdges()) {
    //         ROMAEdge* const edge = static_cast<ROMAEdge*>(e);
    //         edge->setFlow(begin, STEPS2TIME(myEnd), 0.);
    //         edge->setHelpFlow(begin, STEPS2TIME(myEnd), 0.);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_flows(&mut self) {
        todo!("port ROMAAssignments::resetFlows")
    }

    /// C++: `void ROMAAssignments::writeInterval(const SUMOTime, const SUMOTime)`
    // void
    // ROMAAssignments::writeInterval(const SUMOTime begin, const SUMOTime end) {
    //     if (myNetloadOutput != nullptr) {
    //         myNetloadOutput->openTag(SUMO_TAG_INTERVAL).writeAttr(SUMO_ATTR_BEGIN, time2string(begin)).writeAttr(SUMO_ATTR_END, time2string(end));
    //         for (ROEdge* e : ROEdge::getAllEdges()) {
    //             const ROMAEdge* const edge = static_cast<ROMAEdge*>(e);
    //             if (edge->getFunction() == SumoXMLEdgeFunc::NORMAL) {
    //                 myNetloadOutput->openTag(SUMO_TAG_EDGE).writeAttr(SUMO_ATTR_ID, edge->getID());
    //                 const double traveltime = edge->getTravelTime(getDefaultVehicle(), STEPS2TIME(begin));
    //                 const double speed = edge->getLength() / traveltime;
    //                 const double flow = edge->getFlow(STEPS2TIME(begin));
    //                 const double timeGap = STEPS2TIME(end - begin) / flow;
    //                 const double spaceGap = timeGap * speed;
    //                 const double density = 1000.0 / spaceGap;
    //                 const double laneDensity = density / edge->getNumLanes();
    //                 myNetloadOutput->writeAttr(SUMO_ATTR_TRAVELTIME, traveltime);
    //                 myNetloadOutput->writeAttr(SUMO_ATTR_SPEED, speed);
    //                 myNetloadOutput->writeAttr(SUMO_ATTR_SPEEDREL, speed / edge->getSpeedLimit());
    //                 myNetloadOutput->writeAttr(SUMO_ATTR_ENTERED, flow);
    //                 myNetloadOutput->writeAttr(SUMO_ATTR_DENSITY, density);
    //                 myNetloadOutput->writeAttr(SUMO_ATTR_LANEDENSITY, laneDensity);
    //                 myNetloadOutput->writeAttr("flowCapacityRatio", 100. * flow / getCapacity(edge));
    //                 myNetloadOutput->closeTag();
    //             }
    //         }
    //         myNetloadOutput->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_interval(&mut self, begin: SUMOTime, end: SUMOTime) {
        todo!("port ROMAAssignments::writeInterval")
    }

    /// C++: `void ROMAAssignments::incremental(const int, const bool)`
    // void
    // ROMAAssignments::incremental(const int numIter, const bool verbose) {
    //     SUMOTime lastBegin = -1;
    //     std::vector<int> intervals;
    //     int count = 0;
    //     for (const ODCell* const c : myMatrix.getCells()) {
    //         if (c->begin != lastBegin) {
    //             intervals.push_back(count);
    //             lastBegin = c->begin;
    //         }
    //         count++;
    //     }
    //     lastBegin = -1;
    //     for (std::vector<int>::const_iterator offset = intervals.begin(); offset != intervals.end(); offset++) {
    //         std::vector<ODCell*>::const_iterator firstCell = myMatrix.getCells().begin() + (*offset);
    //         std::vector<ODCell*>::const_iterator lastCell = myMatrix.getCells().end();
    //         if (offset != intervals.end() - 1) {
    //             lastCell = myMatrix.getCells().begin() + (*(offset + 1));
    //         }
    //         const SUMOTime intervalStart = (*firstCell)->begin;
    //         if (verbose) {
    //             WRITE_MESSAGE(" starting interval " + time2string(intervalStart));
    //         }
    //         std::map<const ROMAEdge*, double> loadedTravelTimes;
    //         for (ROEdge* e : ROEdge::getAllEdges()) {
    //             const ROMAEdge* const edge = static_cast<ROMAEdge*>(e);
    //             if (edge->hasLoadedTravelTime(STEPS2TIME(intervalStart))) {
    //                 loadedTravelTimes[edge] = edge->getTravelTime(myDefaultVehicle, STEPS2TIME(intervalStart));
    //             }
    //         }
    //         if (loadedTravelTimes.empty()) {
    //             // we don't have loaded travel times for this interval but we may have set ourselves some for the interval before but no need to warn
    //             ROEdge::disableTimelineWarning();
    //         }
    //         for (int t = 0; t < numIter; t++) {
    //             if (verbose) {
    //                 WRITE_MESSAGE("  starting iteration " + toString(t));
    //             }
    //             std::string lastOrigin = "";
    //             int workerIndex = 0;
    //             for (std::vector<ODCell*>::const_iterator i = firstCell; i != lastCell; i++) {
    //                 ODCell* const c = *i;
    //                 const double linkFlow = c->vehicleNumber / numIter;
    //                 const SUMOTime begin = myAdditiveTraffic ? myBegin : c->begin;
    // #ifdef HAVE_FOX
    //                 if (myNet.getThreadPool().size() > 0) {
    //                     if (lastOrigin != c->origin) {
    //                         workerIndex++;
    //                         if (workerIndex == myNet.getThreadPool().size()) {
    //                             workerIndex = 0;
    //                         }
    //                         myNet.getThreadPool().add(new RONet::BulkmodeTask(false), workerIndex);
    //                         lastOrigin = c->origin;
    //                         myNet.getThreadPool().add(new RoutingTask(*this, c, begin, linkFlow), workerIndex);
    //                     } else {
    //                         myNet.getThreadPool().add(new RoutingTask(*this, c, begin, linkFlow), workerIndex);
    //                     }
    //                     continue;
    //                 }
    // #else
    //                 UNUSED_PARAMETER(workerIndex);
    // #endif
    //                 if (lastOrigin != c->origin) {
    //                     myRouter.setBulkMode(false);
    //                     lastOrigin = c->origin;
    //                 }
    //                 computePath(c, begin, linkFlow, &myRouter, true);
    //             }
    // #ifdef HAVE_FOX
    //             if (myNet.getThreadPool().size() > 0) {
    //                 myNet.getThreadPool().waitAll();
    //             }
    // #endif
    //             for (std::vector<ODCell*>::const_iterator i = firstCell; i != lastCell; i++) {
    //                 ODCell* const c = *i;
    //                 const double linkFlow = c->vehicleNumber / numIter;
    //                 const SUMOTime begin = myAdditiveTraffic ? myBegin : c->begin;
    //                 const SUMOTime end = myAdditiveTraffic ? myEnd : c->end;
    //                 const double intervalLengthInHours = STEPS2TIME(end - begin) / 3600.;
    //                 const ConstROEdgeVector& edges = c->pathsVector.back()->getEdgeVector();
    //                 for (ConstROEdgeVector::const_iterator e = edges.begin(); e != edges.end(); e++) {
    //                     ROMAEdge* edge = static_cast<ROMAEdge*>(myNet.getEdge((*e)->getID()));
    //                     const double newFlow = edge->getFlow(STEPS2TIME(begin)) + linkFlow;
    //                     edge->setFlow(STEPS2TIME(begin), STEPS2TIME(end), newFlow);
    //                     double travelTime = capacityConstraintFunction(edge, newFlow / intervalLengthInHours);
    //                     if (lastBegin >= 0 && myAdaptionFactor > 0.) {
    //                         if (loadedTravelTimes.count(edge) != 0) {
    //                             travelTime = loadedTravelTimes[edge] * myAdaptionFactor + (1. - myAdaptionFactor) * travelTime;
    //                         } else {
    //                             travelTime = edge->getTravelTime(myDefaultVehicle, STEPS2TIME(lastBegin)) * myAdaptionFactor + (1. - myAdaptionFactor) * travelTime;
    //                         }
    //                     }
    //                     edge->addTravelTime(travelTime, STEPS2TIME(begin), STEPS2TIME(end));
    //                 }
    //             }
    //             myRouter.reset(myDefaultVehicle);
    //         }
    //         writeInterval(intervalStart, (*(lastCell - 1))->end);
    //         lastBegin = intervalStart;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn incremental(&mut self, numIter: i32, verbose: bool) {
        todo!("port ROMAAssignments::incremental")
    }

    /// C++: `void ROMAAssignments::ue()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn ue(&mut self) {
        todo!("port ROMAAssignments::ue")
    }

    /// C++: `void ROMAAssignments::sue(const int, const int, const int, const double, const double, const std::string)`
    // void
    // ROMAAssignments::sue(const int maxOuterIteration, const int maxInnerIteration, const int kPaths, const double penalty, const double tolerance, const std::string /* routeChoiceMethod */) {
    //     getKPaths(kPaths, penalty);
    //     std::map<const SUMOTime, SUMOTime> intervals;
    //     if (myAdditiveTraffic) {
    //         intervals[myBegin] = myEnd;
    //     } else {
    //         for (const ODCell* const c : myMatrix.getCells()) {
    //             intervals[c->begin] = c->end;
    //         }
    //     }
    //     for (int outer = 0; outer < maxOuterIteration; outer++) {
    //         for (int inner = 0; inner < maxInnerIteration; inner++) {
    //             for (const ODCell* const c : myMatrix.getCells()) {
    //                 const SUMOTime begin = myAdditiveTraffic ? myBegin : c->begin;
    //                 const SUMOTime end = myAdditiveTraffic ? myEnd : c->end;
    //                 // update path cost
    //                 for (const std::shared_ptr<RORoute>& r : c->pathsVector) {
    //                     r->setCosts(myRouter.recomputeCosts(r->getEdgeVector(), myDefaultVehicle, 0));
    //                     //                    std::cout << std::setprecision(20) << r->getID() << ":" << r->getCosts() << std::endl;
    //                 }
    //                 // calculate route utilities and probabilities
    //                 RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().calculateProbabilities(c->pathsVector, myDefaultVehicle, 0);
    //                 // calculate route flows
    //                 for (const std::shared_ptr<RORoute>& r : c->pathsVector) {
    //                     const double pathFlow = r->getProbability() * c->vehicleNumber;
    //                     // assign edge flow deltas
    //                     for (ConstROEdgeVector::const_iterator e = r->getEdgeVector().begin(); e != r->getEdgeVector().end(); e++) {
    //                         ROMAEdge* edge = static_cast<ROMAEdge*>(myNet.getEdge((*e)->getID()));
    //                         edge->setHelpFlow(STEPS2TIME(begin), STEPS2TIME(end), edge->getHelpFlow(STEPS2TIME(begin)) + pathFlow);
    //                     }
    //                 }
    //             }
    //             // calculate new edge flows and check for stability
    //             int unstableEdges = 0;
    //             for (const auto& it : intervals) {
    //                 const double intervalLengthInHours = STEPS2TIME(it.second - it.first) / 3600.;
    //                 const double intBegin = STEPS2TIME(it.first);
    //                 const double intEnd = STEPS2TIME(it.second);
    //                 for (ROEdge* e : ROEdge::getAllEdges()) {
    //                     ROMAEdge* const edge = static_cast<ROMAEdge*>(e);
    //                     const double oldFlow = edge->getFlow(intBegin);
    //                     double newFlow = oldFlow;
    //                     if (inner == 0 && outer == 0) {
    //                         newFlow += edge->getHelpFlow(intBegin);
    //                     } else {
    //                         newFlow += (edge->getHelpFlow(intBegin) - oldFlow) / (inner + 1);
    //                     }
    //                     //                if not lohse:
    //                     if (newFlow > 0.) {
    //                         if (fabs(newFlow - oldFlow) / newFlow > tolerance) {
    //                             unstableEdges++;
    //                         }
    //                     } else if (newFlow == 0.) {
    //                         if (oldFlow != 0. && (fabs(newFlow - oldFlow) / oldFlow > tolerance)) {
    //                             unstableEdges++;
    //                         }
    //                     } else { // newFlow < 0.
    //                         unstableEdges++;
    //                         newFlow = 0.;
    //                     }
    //                     edge->setFlow(intBegin, intEnd, newFlow);
    //                     const double travelTime = capacityConstraintFunction(edge, newFlow / intervalLengthInHours);
    //                     edge->addTravelTime(travelTime, intBegin, intEnd);
    //                     edge->setHelpFlow(intBegin, intEnd, 0.);
    //                 }
    //             }
    //             // if stable break
    //             if (unstableEdges == 0) {
    //                 break;
    //             }
    //             // additional stability check from python script: if notstable < math.ceil(net.geteffEdgeCounts()*0.005) or notstable < 3: stable = True
    //         }
    //         // travel times changed since the metric was last built (or since the
    //         // previous outer iteration): flag it stale so the *first* subsequent
    //         // router query re-customizes (CCH) / rebuilds the hierarchy (CH)
    //         // from current edge weights. recomputeCosts() above never consults
    //         // the cached metric (it reads the live effort function directly), so
    //         // a single reset() here -- right before the only call that does,
    //         // computePath()'s router->compute() below -- is exactly as correct
    //         // as resetting after every inner iteration, but for an
    //         // expensive-to-rebuild algorithm like CH this cuts the number of
    //         // rebuilds from O(maxInnerIteration) to one per outer iteration.
    //         myRouter.reset(myDefaultVehicle);
    //         // check for a new route, if none available, break
    //         // several modifications about when a route is new and when to break are in the original script
    //         bool newRoute = false;
    //         for (ODCell* const c : myMatrix.getCells()) {
    //             newRoute |= !computePath(c).empty();
    //         }
    //         if (!newRoute) {
    //             break;
    //         }
    //     }
    //     // final round of assignment
    //     for (const ODCell* const c : myMatrix.getCells()) {
    //         // update path cost
    //         for (const std::shared_ptr<RORoute>& r : c->pathsVector) {
    //             r->setCosts(myRouter.recomputeCosts(r->getEdgeVector(), myDefaultVehicle, 0));
    //         }
    //         // calculate route utilities and probabilities
    //         RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().calculateProbabilities(c->pathsVector, myDefaultVehicle, 0);
    //         // calculate route flows
    //         for (const std::shared_ptr<RORoute>& r : c->pathsVector) {
    //             r->setProbability(r->getProbability() * c->vehicleNumber);
    //         }
    //     }
    //     for (const auto& it : intervals) {
    //         writeInterval(it.first, it.second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sue(&mut self, maxOuterIteration: i32, maxInnerIteration: i32, kPaths: i32, penalty: f64, tolerance: f64, routeChoiceMethod: String) {
        todo!("port ROMAAssignments::sue")
    }

    /// C++: `double ROMAAssignments::getPenalizedEffort(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Returns the effort to pass an edge including penalties
    // double
    // ROMAAssignments::getPenalizedEffort(const ROEdge* const e, const ROVehicle* const v, double t) {
    //     const std::map<const ROEdge* const, double>::const_iterator i = myPenalties.find(e);
    //     return i == myPenalties.end() ? e->getEffort(v, t) : e->getEffort(v, t) + i->second;
    // }
    #[allow(unused_variables)]
    pub fn get_penalized_effort(e: Ptr<ROEdge>, v: Ptr<ROVehicle>, t: f64) -> f64 {
        todo!("port ROMAAssignments::getPenalizedEffort")
    }

    /// C++: `double ROMAAssignments::getPenalizedTT(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Returns the traveltime on an edge including penalties
    // double
    // ROMAAssignments::getPenalizedTT(const ROEdge* const e, const ROVehicle* const v, double t) {
    //     const std::map<const ROEdge* const, double>::const_iterator i = myPenalties.find(e);
    //     return i == myPenalties.end() ? e->getTravelTime(v, t) : e->getTravelTime(v, t) + i->second;
    // }
    #[allow(unused_variables)]
    pub fn get_penalized_tt(e: Ptr<ROEdge>, v: Ptr<ROVehicle>, t: f64) -> f64 {
        todo!("port ROMAAssignments::getPenalizedTT")
    }

    /// C++: `double ROMAAssignments::getTravelTime(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Returns the traveltime on an edge without penalties
    // double
    // ROMAAssignments::getTravelTime(const ROEdge* const e, const ROVehicle* const v, double t) {
    //     return e->getTravelTime(v, t);
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(e: Ptr<ROEdge>, v: Ptr<ROVehicle>, t: f64) -> f64 {
        todo!("port ROMAAssignments::getTravelTime")
    }

    /// C++: `bool ROMAAssignments::addRoute(const ConstROEdgeVector &, std::vector<std::shared_ptr<RORoute>> &, std::string, double)` [private]
    /// add a route and check for duplicates
    // bool
    // ROMAAssignments::addRoute(const ConstROEdgeVector& edges, std::vector<std::shared_ptr<RORoute> >& paths, std::string routeId, double prob) {
    //     std::vector<std::shared_ptr<RORoute> >::iterator p;
    //     for (p = paths.begin(); p != paths.end(); p++) {
    //         if (edges == (*p)->getEdgeVector()) {
    //             break;
    //         }
    //     }
    //     if (p == paths.end()) {
    //         paths.push_back(std::make_shared<RORoute>(routeId, 0., prob, edges, nullptr, StopParVector()));
    //         return true;
    //     }
    //     (*p)->addProbability(prob);
    //     std::iter_swap(paths.end() - 1, p);
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_route(&mut self, edges: &ConstROEdgeVector, paths: &mut Vec<Rc<RORoute>>, routeId: String, prob: f64) -> bool {
        todo!("port ROMAAssignments::addRoute")
    }

    /// C++: `const ConstROEdgeVector ROMAAssignments::computePath(ODCell *, const SUMOTime, const double, SUMOAbstractRouter<ROEdge, ROVehicle> *, bool)` [private]
    // const ConstROEdgeVector
    // ROMAAssignments::computePath(ODCell* cell, const SUMOTime time, const double probability, SUMOAbstractRouter<ROEdge, ROVehicle>* router, bool setBulkMode) {
    //     const ROEdge* const from = myNet.getEdge(cell->origin + (cell->originIsEdge ? "" : "-source"));
    //     if (from == nullptr) {
    //         throw ProcessError(TLF("Unknown origin '%'.", cell->origin));
    //     }
    //     const ROEdge* const to = myNet.getEdge(cell->destination + (cell->destinationIsEdge ? "" : "-sink"));
    //     if (to == nullptr) {
    //         throw ProcessError(TLF("Unknown destination '%'.", cell->destination));
    //     }
    //     ConstROEdgeVector edges;
    //     if (router == nullptr) {
    //         router = &myRouter;
    //     }
    //     if (myMaxAlternatives > 0 && (int)cell->pathsVector.size() < myMaxAlternatives) {
    //         router->compute(from, to, myDefaultVehicle, time, edges);
    //         if (setBulkMode) {
    //             router->setBulkMode(true);
    //         }
    //         if (addRoute(edges, cell->pathsVector, cell->origin + cell->destination + toString(cell->pathsVector.size()), probability)) {
    //             return edges;
    //         }
    //     } else {
    //         double minCost = std::numeric_limits<double>::max();
    //         std::shared_ptr<RORoute> minRoute = nullptr;
    //         for (const std::shared_ptr<RORoute>& p : cell->pathsVector) {
    //             const double cost = router->recomputeCosts(edges, myDefaultVehicle, time);
    //             if (cost < minCost) {
    //                 minCost = cost;
    //                 minRoute = p;
    //             }
    //         }
    //         minRoute->addProbability(probability);
    //     }
    //     return ConstROEdgeVector();
    // }
    #[allow(unused_variables)]
    pub fn compute_path(&mut self, cell: Ptr<ODCell>, time: SUMOTime, probability: f64, router: Ptr<SUMOAbstractRouter<ROEdge, ROVehicle>>, setBulkMode: bool) -> ConstROEdgeVector {
        todo!("port ROMAAssignments::computePath")
    }

    /// C++: `void ROMAAssignments::getKPaths(const int, const double)` [private]
    /// get the k shortest paths
    // void
    // ROMAAssignments::getKPaths(const int kPaths, const double penalty) {
    //     for (ODCell* const c : myMatrix.getCells()) {
    //         myPenalties.clear();
    //         for (int k = 0; k < kPaths; k++) {
    //             for (const ROEdge* const e : computePath(c)) {
    //                 myPenalties[e] += penalty;
    //             }
    //         }
    //     }
    //     myPenalties.clear();
    // }
    #[allow(unused_variables)]
    pub fn get_k_paths(&mut self, kPaths: i32, penalty: f64) {
        todo!("port ROMAAssignments::getKPaths")
    }

    /// C++: `ROMAAssignments & ROMAAssignments::operator=(const ROMAAssignments &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROMAAssignments) -> &mut ROMAAssignments {
        todo!("port ROMAAssignments::operator=")
    }

}

