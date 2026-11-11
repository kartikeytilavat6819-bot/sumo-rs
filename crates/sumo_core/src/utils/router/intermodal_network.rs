//! Port of utils/router/IntermodalNetwork.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `ModeChangeOptions`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum ModeChangeOptions {
    PARKING_AREAS = 1,
    PT_STOPS = 2,
    ALL_JUNCTIONS = 4,
    TAXI_DROPOFF_PARKING_AREAS = 8,
    TAXI_DROPOFF_PT = 16,
    TAXI_DROPOFF_ANYWHERE = 32,
    TAXI_PICKUP_PARKING_AREAS = 64,
    TAXI_PICKUP_PT = 128,
    TAXI_PICKUP_ANYWHERE = 256,
}

/// C++ typedef `std::pair<_IntermodalEdge *, _IntermodalEdge *> IntermodalNetwork::EdgePair`
pub type IntermodalNetwork_EdgePair = (Ptr<AccessEdge__IntermodalEdge>, Ptr<AccessEdge__IntermodalEdge>);

/// C++ typedef `AccessEdge<E, L, N, V> IntermodalNetwork::_AccessEdge`
pub type IntermodalNetwork__AccessEdge = Unmapped /* AccessEdge<E, L, N, V> */;

/// C++ typedef `IntermodalEdge<E, L, N, V> IntermodalNetwork::_IntermodalEdge`
pub type IntermodalNetwork__IntermodalEdge = Unmapped /* IntermodalEdge<E, L, N, V> */;

/// C++ typedef `PublicTransportEdge<E, L, N, V> IntermodalNetwork::_PTEdge`
pub type IntermodalNetwork__PTEdge = Unmapped /* PublicTransportEdge<E, L, N, V> */;

/// C++ typedef `PedestrianEdge<E, L, N, V> IntermodalNetwork::_PedestrianEdge`
pub type IntermodalNetwork__PedestrianEdge = Unmapped /* PedestrianEdge<E, L, N, V> */;

/// the intermodal network storing edges, connections and the mappings to the "real" edges
/// C++ class `IntermodalNetwork` (utils/router/IntermodalNetwork.h)
pub struct IntermodalNetwork<E, L, N, V> {
    /// the edge dictionary
    pub myEdges: Vec<Ptr<AccessEdge__IntermodalEdge>>,
    /// retrieve the forward and backward edge for the given input edge E
    pub myBidiLookup: BTreeMap<Ptr<E>, IntermodalNetwork_EdgePair>,
    /// retrieve the depart edges for the given input edge E
    pub myDepartLookup: BTreeMap<Ptr<E>, Vec<Ptr<AccessEdge__IntermodalEdge>>>,
    /// retrieve the arrival edges for the given input edge E
    pub myArrivalLookup: BTreeMap<Ptr<E>, Vec<Ptr<AccessEdge__IntermodalEdge>>>,
    /// the walking connector edge (fake walking area)
    pub myWalkingConnectorLookup: BTreeMap<Ptr<N>, Ptr<AccessEdge__IntermodalEdge>>,
    /// retrieve the car edge for the given input edge E
    pub myCarLookup: BTreeMap<Ptr<E>, Ptr<AccessEdge__IntermodalEdge>>, // C++: std::map<const E *, _IntermodalEdge *, ComparatorNumericalIdLess>
    /// retrieve the public transport edges for the given line
    pub myPTLines: BTreeMap<String, Vec<Ptr<IntermodalNetwork__PTEdge>>>,
    /// retrieve the representing edge for the given stopping place
    pub myStopConnections: BTreeMap<String, Ptr<AccessEdge__IntermodalEdge>>,
    /// retrieve the splitted edges for the given "original"
    pub myAccessSplits: BTreeMap<Ptr<AccessEdge__IntermodalEdge>, Vec<Ptr<AccessEdge__IntermodalEdge>>>,
    /// looped lines need extra checking when building itineraries
    pub myLoopedLines: BTreeSet<String>,
    pub myNumericalID: i32,
    pub myCarWalkTransfer: i32,
    pub myHavePTSchedules: bool,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> IntermodalNetwork<E, L, N, V> {
    /// C++: `void IntermodalNetwork::IntermodalNetwork<E, L, N, V>(const std::vector<E *> &, const bool, const int)`
    // IntermodalNetwork(const std::vector<E*>& edges, const bool pedestrianOnly, const int carWalkTransfer = 0)
    //         : myNumericalID(0), myCarWalkTransfer(carWalkTransfer), myHavePTSchedules(false) {
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //         std::cout << "initIntermodalNetwork\n";
    // #endif
    //         // build the pedestrian edges and the depart / arrival connectors with lookup tables
    //         bool haveSeenWalkingArea = false;
    //         for (const E* const edge : edges) {
    //             if (edge->isTazConnector()) {
    //                 // only a single edge
    //                 _AccessEdge* access = new _AccessEdge(myNumericalID++, edge->getID(), edge);
    //                 addEdge(access);
    //                 myDepartLookup[edge].push_back(access);
    //                 myArrivalLookup[edge].push_back(access);
    //             } else {
    //                 const L* lane = getSidewalk<E, L>(edge);
    //                 if (lane != nullptr) {
    //                     if (edge->isWalkingArea()) {
    //                         // only a single edge
    //                         addEdge(new _PedestrianEdge(myNumericalID++, edge, lane, true));
    //                         myBidiLookup[edge] = std::make_pair(myEdges.back(), myEdges.back());
    //                         myDepartLookup[edge].push_back(myEdges.back());
    //                         myArrivalLookup[edge].push_back(myEdges.back());
    //                         haveSeenWalkingArea = true;
    //                     } else { // regular edge or crossing
    //                         // forward and backward edges
    //                         addEdge(new _PedestrianEdge(myNumericalID++, edge, lane, true));
    //                         addEdge(new _PedestrianEdge(myNumericalID++, edge, lane, false));
    //                         myBidiLookup[edge] = std::make_pair(myEdges[myNumericalID - 2], myEdges.back());
    //                     }
    //                 }
    //                 if (!edge->isWalkingArea()) {
    //                     // depart and arrival edges (the router can decide the initial direction to take and the direction to arrive from)
    //                     _IntermodalEdge* const departConn = new _IntermodalEdge(edge->getID() + "_depart_connector", myNumericalID++, edge, "!connector");
    //                     _IntermodalEdge* const arrivalConn = new _IntermodalEdge(edge->getID() + "_arrival_connector", myNumericalID++, edge, "!connector");
    //                     addConnectors(departConn, arrivalConn, 0);
    //                 }
    //             }
    //         }
    //
    //         // build the walking connectors if there are no walking areas
    //         for (const E* const edge : edges) {
    //             if (edge->isTazConnector() || edge->isInternal()) {
    //                 continue;
    //             }
    //             if (haveSeenWalkingArea) {
    //                 // connectivity needs to be ensured only in the real intermodal case, for simple pedestrian routing we don't have connectors if we have walking areas
    //                 if (!pedestrianOnly && getSidewalk<E, L>(edge) == nullptr) {
    //                     const N* const node = edge->getToJunction();
    //                     if (myWalkingConnectorLookup.count(node) == 0) {
    //                         addEdge(new _IntermodalEdge(node->getID() + "_walking_connector", myNumericalID++, nullptr, "!connector"));
    //                         myWalkingConnectorLookup[node] = myEdges.back();
    //                     }
    //                 }
    //             } else {
    //                 for (const N* const node : {
    //                             edge->getFromJunction(), edge->getToJunction()
    //                         }) {
    //                     if (myWalkingConnectorLookup.count(node) == 0) {
    //                         addEdge(new _IntermodalEdge(node->getID() + "_walking_connector", myNumericalID++, nullptr, "!connector"));
    //                         myWalkingConnectorLookup[node] = myEdges.back();
    //                     }
    //                 }
    //             }
    //         }
    //         // build the connections
    //         for (const E* const edge : edges) {
    //             if (edge->isTazConnector()) {
    //                 // since pedestrians walk in both directions, also allow departing at sinks and arriving at sources
    //                 _IntermodalEdge* const tazDepart = getDepartConnector(edge);
    //                 _IntermodalEdge* const tazArrive = getArrivalConnector(edge);
    //                 const E* other = edge->getOtherTazConnector();
    //                 _IntermodalEdge* const otherTazDepart = other != nullptr ? getDepartConnector(other) : tazDepart;
    //                 _IntermodalEdge* const otherTazArrive = other != nullptr ? getArrivalConnector(other) : tazArrive;
    //                 for (const E* out : edge->getSuccessors()) {
    //                     if (out->isNormal()) {
    //                         tazDepart->addSuccessor(getDepartConnector(out));
    //                         getArrivalConnector(out)->addSuccessor(otherTazArrive);
    //                     }
    //                 }
    //                 for (const E* in : edge->getPredecessors()) {
    //                     if (in->isNormal()) {
    //                         getArrivalConnector(in)->addSuccessor(tazArrive);
    //                         otherTazDepart->addSuccessor(getDepartConnector(in));
    //                     }
    //                 }
    //                 continue;
    //             }
    //             const L* const sidewalk = getSidewalk<E, L>(edge);
    //             if (sidewalk == nullptr) {
    //                 continue;
    //             }
    //             // find all incoming and outgoing lanes for the sidewalk and
    //             // connect the corresponding IntermodalEdges
    //             const EdgePair& pair = getBothDirections(edge);
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //             std::cout << "  building connections from " << sidewalk->getID() << "\n";
    // #endif
    //             if (haveSeenWalkingArea) {
    //                 const std::vector<std::pair<const L*, const E*> > outgoing = sidewalk->getOutgoingViaLanes();
    //                 // if one of the outgoing lanes is a walking area it must be used.
    //                 // All other connections shall be ignored
    //                 // if it has no outgoing walking area, it probably is a walking area itself
    //                 bool hasWalkingArea = false;
    //                 for (const auto& target : outgoing) {
    //                     if (target.first->getEdge().isWalkingArea()) {
    //                         hasWalkingArea = true;
    //                         break;
    //                     }
    //                 }
    //                 for (const auto& target : outgoing) {
    //                     const E* const targetEdge = &(target.first->getEdge());
    //                     const bool used = (target.first == getSidewalk<E, L>(targetEdge)
    //                                        && (!hasWalkingArea || targetEdge->isWalkingArea()));
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //                     const L* potTarget = getSidewalk<E, L>(targetEdge);
    //                     std::cout << "   lane=" << (potTarget == 0 ? "NULL" : potTarget->getID()) << (used ? "(used)" : "") << "\n";
    // #endif
    //                     if (used) {
    //                         const EdgePair& targetPair = getBothDirections(targetEdge);
    //                         pair.first->addSuccessor(targetPair.first);
    //                         targetPair.second->addSuccessor(pair.second);
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //                         std::cout << "     " << pair.first->getID() << " -> " << targetPair.first->getID() << "\n";
    //                         std::cout << "     " << targetPair.second->getID() << " -> " << pair.second->getID() << "\n";
    // #endif
    //                     }
    //                 }
    //             }
    //             // We may have a network without pedestrian structures or a car-only edge.
    //             // In the first case we assume that all sidewalks at a junction are interconnected,
    //             // in the second we connect all car-only edges to all sidewalks.
    //             _IntermodalEdge* const toNodeConn = myWalkingConnectorLookup[edge->getToJunction()];
    //             if (toNodeConn != nullptr) {
    //                 // Check for the outgoing vias and use the shortest one as an approximation
    //                 const std::vector<std::pair<const L*, const E*> > outgoing = sidewalk->getOutgoingViaLanes();
    //                 double minViaLength = std::numeric_limits<double>::max();
    //                 const E* minVia = nullptr;
    //                 for (const auto& target : outgoing) {
    //                     if (target.second != nullptr && target.second->getLength() < minViaLength) {
    //                         minViaLength = target.second->getLength();
    //                         minVia = target.second;
    //                     }
    //                 }
    //                 EdgePair interVia = std::make_pair(nullptr, nullptr);
    //                 if (minVia != nullptr) {
    //                     const auto it = myBidiLookup.find(minVia);
    //                     if (it != myBidiLookup.end()) {
    //                         interVia = it->second;
    //                     }
    //                 }
    //                 if (!haveSeenWalkingArea) {
    //                     // if we have walking areas we should use them and not the connector
    //                     pair.first->addSuccessor(toNodeConn, interVia.first);
    //                 }
    //                 toNodeConn->addSuccessor(pair.second, interVia.second);
    //             }
    //             _IntermodalEdge* const fromNodeConn = myWalkingConnectorLookup[edge->getFromJunction()];
    //             if (fromNodeConn != nullptr) {
    //                 if (!haveSeenWalkingArea) {
    //                     pair.second->addSuccessor(fromNodeConn);
    //                 }
    //                 fromNodeConn->addSuccessor(pair.first);
    //             }
    //             if (!edge->isWalkingArea()) {
    //                 // build connections from depart connector
    //                 _IntermodalEdge* startConnector = getDepartConnector(edge);
    //                 startConnector->addSuccessor(pair.first);
    //                 startConnector->addSuccessor(pair.second);
    //                 // build connections to arrival connector
    //                 _IntermodalEdge* endConnector = getArrivalConnector(edge);
    //                 pair.first->addSuccessor(endConnector);
    //                 pair.second->addSuccessor(endConnector);
    // #ifdef IntermodalRouter_DEBUG_NETWORK
    //                 std::cout << "     " << startConnector->getID() << " -> " << pair.first->getID() << "\n";
    //                 std::cout << "     " << startConnector->getID() << " -> " << pair.second->getID() << "\n";
    //                 std::cout << "     " << pair.first->getID() << " -> " << endConnector->getID() << "\n";
    //                 std::cout << "     " << pair.second->getID() << " -> " << endConnector->getID() << "\n";
    // #endif
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, pedestrianOnly: bool, carWalkTransfer: i32) -> Self {
        todo!("port IntermodalNetwork::IntermodalNetwork<E, L, N, V>")
    }

    /// C++: `void IntermodalNetwork::~IntermodalNetwork<E, L, N, V>()`
    // ~IntermodalNetwork() {
    //         for (typename std::vector<_IntermodalEdge*>::iterator it = myEdges.begin(); it != myEdges.end(); ++it) {
    //             delete *it;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port IntermodalNetwork::~IntermodalNetwork<E, L, N, V>")
    }

    /// C++: `void IntermodalNetwork::addEdge(_IntermodalEdge *)`
    // void addEdge(_IntermodalEdge* edge) {
    //         while ((int)myEdges.size() <= edge->getNumericalID()) {
    //             myEdges.push_back(0);
    //         }
    //         myEdges[edge->getNumericalID()] = edge;
    //     }
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, edge: Ptr<AccessEdge__IntermodalEdge>) {
        todo!("port IntermodalNetwork::addEdge")
    }

    /// C++: `void IntermodalNetwork::addConnectors(_IntermodalEdge *const, _IntermodalEdge *const, const int)`
    // void addConnectors(_IntermodalEdge* const depConn, _IntermodalEdge* const arrConn, const int index) {
    //         addEdge(depConn);
    //         addEdge(arrConn);
    //         myDepartLookup[depConn->getEdge()].insert(myDepartLookup[depConn->getEdge()].begin() + index, depConn);
    //         myArrivalLookup[arrConn->getEdge()].insert(myArrivalLookup[arrConn->getEdge()].begin() + index, arrConn);
    //     }
    #[allow(unused_variables)]
    pub fn add_connectors(&mut self, depConn: Ptr<AccessEdge__IntermodalEdge>, arrConn: Ptr<AccessEdge__IntermodalEdge>, index: i32) {
        todo!("port IntermodalNetwork::addConnectors")
    }

    /// C++: `const std::vector<_IntermodalEdge *> & IntermodalNetwork::getAllEdges()`
    // const std::vector<_IntermodalEdge*>& getAllEdges() {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_all_edges(&mut self) -> &Vec<Ptr<AccessEdge__IntermodalEdge>> {
        todo!("port IntermodalNetwork::getAllEdges")
    }

    /// C++: `const EdgePair & IntermodalNetwork::getBothDirections(const E *) const` [const]
    /// Returns the pair of forward and backward edge
    // const EdgePair& getBothDirections(const E* e) const {
    //         typename std::map<const E*, EdgePair>::const_iterator it = myBidiLookup.find(e);
    //         if (it == myBidiLookup.end()) {
    //             assert(false);
    //             throw ProcessError(TLF("Edge '%' not found in intermodal network.'", e->getID()));
    //         }
    //         return (*it).second;
    //     }
    #[allow(unused_variables)]
    pub fn get_both_directions(&self, e: Ptr<E>) -> &IntermodalNetwork_EdgePair {
        todo!("port IntermodalNetwork::getBothDirections")
    }

    /// C++: `const _IntermodalEdge * IntermodalNetwork::getDepartEdge(const E *, const double) const` [const]
    /// Returns the departing intermodal edge
    // const _IntermodalEdge* getDepartEdge(const E* e, const double pos) const {
    //         typename std::map<const E*, std::vector<_IntermodalEdge*> >::const_iterator it = myDepartLookup.find(e);
    //         if (it == myDepartLookup.end()) {
    //             throw ProcessError(TLF("Depart edge '%' not found in intermodal network.", e->getID()));
    //         }
    //         if ((e->getPermissions() & SVC_PEDESTRIAN) == 0) {
    //             // use most specific split (best trainStop, quay etc)
    //             double bestDist = std::numeric_limits<double>::max();
    //             const _IntermodalEdge* best = nullptr;
    //             for (const _IntermodalEdge* const split : it->second) {
    //                 if (pos >= split->getStartPos() - POSITION_EPS && pos <= split->getEndPos() + POSITION_EPS) {
    //                     const double dist = split->getEndPos() - split->getStartPos();
    //                     if (dist < bestDist) {
    //                         bestDist = dist;
    //                         best = split;
    //                     }
    //                 }
    //             }
    //             assert(best != nullptr);
    //             return best;
    //         } else {
    //             // use next downstream edge
    //             const std::vector<_IntermodalEdge*>& splitList = it->second;
    //             typename std::vector<_IntermodalEdge*>::const_iterator splitIt = splitList.begin();
    //             double totalLength = 0.;
    //             while (splitIt + 1 != splitList.end() && totalLength + (*splitIt)->getLength() < pos) {
    //                 totalLength += (*splitIt)->getLength();
    //                 ++splitIt;
    //             }
    //             return *splitIt;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self, e: Ptr<E>, pos: f64) -> Ptr<AccessEdge__IntermodalEdge> {
        todo!("port IntermodalNetwork::getDepartEdge")
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getDepartConnector(const E *, const int) const` [const]
    /// Returns the departing intermodal connector at the given split offset
    // _IntermodalEdge* getDepartConnector(const E* e, const int splitIndex = 0) const {
    //         typename std::map<const E*, std::vector<_IntermodalEdge*> >::const_iterator it = myDepartLookup.find(e);
    //         if (it == myDepartLookup.end()) {
    //             throw ProcessError(TLF("Depart edge '%' not found in intermodal network.", e->getID()));
    //         }
    //         if (splitIndex >= (int)it->second.size()) {
    //             throw ProcessError("Split index " + toString(splitIndex) + " invalid for depart edge '" + e->getID() + "' .");
    //         }
    //         return it->second[splitIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_connector(&self, e: Ptr<E>, splitIndex: i32) -> Ptr<AccessEdge__IntermodalEdge> {
        todo!("port IntermodalNetwork::getDepartConnector")
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getArrivalEdge(const E *, const double) const` [const]
    /// Returns the arriving intermodal edge
    // _IntermodalEdge* getArrivalEdge(const E* e, const double pos) const {
    //         typename std::map<const E*, std::vector<_IntermodalEdge*> >::const_iterator it = myArrivalLookup.find(e);
    //         if (it == myArrivalLookup.end()) {
    //             throw ProcessError(TLF("Arrival edge '%' not found in intermodal network.", e->getID()));
    //         }
    //         const std::vector<_IntermodalEdge*>& splitList = it->second;
    //         typename std::vector<_IntermodalEdge*>::const_iterator splitIt = splitList.begin();
    //         double totalLength = 0.;
    //         while (splitIt != splitList.end() && totalLength + (*splitIt)->getLength() < pos) {
    //             totalLength += (*splitIt)->getLength();
    //             ++splitIt;
    //         }
    //         if (splitIt != splitList.end()) {
    //             return *splitIt;
    //         } else {
    //             return splitList.back();
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_edge(&self, e: Ptr<E>, pos: f64) -> Ptr<AccessEdge__IntermodalEdge> {
        todo!("port IntermodalNetwork::getArrivalEdge")
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getArrivalConnector(const E *, const int) const` [const]
    /// Returns the arriving intermodal connector at the given split offset
    // _IntermodalEdge* getArrivalConnector(const E* e, const int splitIndex = 0) const {
    //         return myArrivalLookup.find(e)->second[splitIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_connector(&self, e: Ptr<E>, splitIndex: i32) -> Ptr<AccessEdge__IntermodalEdge> {
        todo!("port IntermodalNetwork::getArrivalConnector")
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getWalkingConnector(const E *) const` [const]
    /// Returns the outgoing pedestrian edge, which is either a walking area or a walking connector
    // _IntermodalEdge* getWalkingConnector(const E* e) const {
    //         typename std::map<const N*, _IntermodalEdge*>::const_iterator it = myWalkingConnectorLookup.find(e->getToJunction());
    //         if (it == myWalkingConnectorLookup.end()) {
    //             const L* const sidewalk = getSidewalk<E, L>(e);
    //             if (e->isInternal() || sidewalk == 0) {
    //                 return 0;
    //             }
    //             for (const auto& target : sidewalk->getOutgoingViaLanes()) {
    //                 if (target.first->getEdge().isWalkingArea()) {
    //                     return getBothDirections(&target.first->getEdge()).first;
    //                 }
    //             }
    //             return 0;
    //         }
    //         return it->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_walking_connector(&self, e: Ptr<E>) -> Ptr<AccessEdge__IntermodalEdge> {
        todo!("port IntermodalNetwork::getWalkingConnector")
    }

    /// C++: `void IntermodalNetwork::addCarEdges(const std::vector<E *> &, double)`
    // void addCarEdges(const std::vector<E*>& edges, double taxiWait) {
    //         for (const E* const edge : edges) {
    //             if (edge->getFunction() == SumoXMLEdgeFunc::NORMAL || edge->getFunction() == SumoXMLEdgeFunc::INTERNAL) {
    //                 myCarLookup[edge] = new CarEdge<E, L, N, V>(myNumericalID++, edge);
    //                 addEdge(myCarLookup[edge]);
    //             }
    //         }
    //         for (const auto& edgePair : myCarLookup) {
    //             _IntermodalEdge* const carEdge = edgePair.second;
    //             // connectivity within the car network
    //             for (const auto& suc : edgePair.first->getViaSuccessors()) {
    //                 _IntermodalEdge* const sucCarEdge = getCarEdge(suc.first);
    //                 _IntermodalEdge* const sucViaEdge = getCarEdge(suc.second);
    //                 if (sucCarEdge != nullptr) {
    //                     carEdge->addSuccessor(sucCarEdge, sucViaEdge);
    //                 }
    //             }
    //             // connectivity to the pedestrian network (only for normal edges)
    //             if (edgePair.first->getFunction() != SumoXMLEdgeFunc::NORMAL) {
    //                 continue;
    //             }
    //             if ((myCarWalkTransfer & ALL_JUNCTIONS) != 0) {
    //                 _IntermodalEdge* const walkCon = getWalkingConnector(edgePair.first);
    //                 if (walkCon != 0) {
    //                     carEdge->addSuccessor(walkCon);
    //                 } else {
    //                     // we are on an edge where pedestrians are forbidden and want to continue on an arbitrary pedestrian edge
    //                     for (const E* const out : edgePair.first->getToJunction()->getOutgoing()) {
    //                         if (!out->isInternal() && !out->isTazConnector() && getSidewalk<E, L>(out) != 0) {
    //                             carEdge->addSuccessor(getBothDirections(out).first);
    //                         }
    //                     }
    //                     for (const E* const in : edgePair.first->getToJunction()->getIncoming()) {
    //                         if (!in->isInternal() && !in->isTazConnector() && getSidewalk<E, L>(in) != 0) {
    //                             carEdge->addSuccessor(getBothDirections(in).second);
    //                         }
    //                     }
    //                 }
    //             }
    //             if ((myCarWalkTransfer & ALL_JUNCTIONS) == 0 && (myCarWalkTransfer & TAXI_DROPOFF_ANYWHERE) != 0) {
    //                 // add access edges that allow exiting a taxi
    //                 _IntermodalEdge* const walkCon = getWalkingConnector(edgePair.first);
    //                 if (walkCon != 0) {
    //                     addRestrictedCarExit(carEdge, walkCon, gTaxiClasses);
    //                 } else {
    //                     // we are on an edge where pedestrians are forbidden and want to continue on an arbitrary pedestrian edge
    //                     for (const E* const out : edgePair.first->getToJunction()->getOutgoing()) {
    //                         if (!out->isInternal() && !out->isTazConnector() && getSidewalk<E, L>(out) != 0) {
    //                             addRestrictedCarExit(carEdge, getBothDirections(out).first, gTaxiClasses);
    //                         }
    //                     }
    //                     for (const E* const in : edgePair.first->getToJunction()->getIncoming()) {
    //                         if (!in->isInternal() && !in->isTazConnector() && getSidewalk<E, L>(in) != 0) {
    //                             addRestrictedCarExit(carEdge, getBothDirections(in).second, gTaxiClasses);
    //                         }
    //                     }
    //                 }
    //             }
    //             // use intermediate access edge that prevents taxi departure
    //             _IntermodalEdge* departConn = getDepartConnector(edgePair.first);
    //             _AccessEdge* access = new _AccessEdge(myNumericalID++, departConn, carEdge, 0, (SVCAll & ~SVC_TAXI));
    //             addEdge(access);
    //             departConn->addSuccessor(access);
    //             access->addSuccessor(carEdge);
    //             if ((myCarWalkTransfer & TAXI_PICKUP_ANYWHERE) != 0) {
    //                 // taxi may depart anywhere but there is a time penalty
    //                 _AccessEdge* taxiAccess = new _AccessEdge(myNumericalID++, departConn, carEdge, 0, SVC_TAXI, SVC_IGNORING, taxiWait);
    //                 addEdge(taxiAccess);
    //                 departConn->addSuccessor(taxiAccess);
    //                 taxiAccess->addSuccessor(carEdge);
    //             }
    //             if ((myCarWalkTransfer & TAXI_DROPOFF_ANYWHERE) != 0) {
    //                 // taxi (as all other cars) may arrive anywhere
    //                 carEdge->addSuccessor(getArrivalConnector(edgePair.first));
    //             } else {
    //                 // use intermediate access edge that prevents taxi arrival
    //                 addRestrictedCarExit(carEdge, getArrivalConnector(edgePair.first), (SVCAll & ~gTaxiClasses));
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_car_edges(&mut self, edges: &Vec<Ptr<E>>, taxiWait: f64) {
        todo!("port IntermodalNetwork::addCarEdges")
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getCarEdge(const E *) const` [const]
    /// Returns the associated car edge
    // _IntermodalEdge* getCarEdge(const E* e) const {
    //         if (e == nullptr) {
    //             return nullptr;
    //         }
    //         auto it = myCarLookup.find(e);
    //         if (it == myCarLookup.end()) {
    //             return nullptr;
    //         }
    //         return it->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_edge(&self, e: Ptr<E>) -> Ptr<AccessEdge__IntermodalEdge> {
        todo!("port IntermodalNetwork::getCarEdge")
    }

    /// C++: `_IntermodalEdge * IntermodalNetwork::getStopEdge(const std::string &) const` [const]
    /// Returns the associated stop edge
    // _IntermodalEdge* getStopEdge(const std::string& stopId) const {
    //         auto it = myStopConnections.find(stopId);
    //         if (it == myStopConnections.end()) {
    //             return nullptr;
    //         }
    //         return it->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_stop_edge(&self, stopId: &str) -> Ptr<AccessEdge__IntermodalEdge> {
        todo!("port IntermodalNetwork::getStopEdge")
    }

    /// C++: `void IntermodalNetwork::addAccess(const std::string &, const E *, const double, const double, const double, const SumoXMLTag, bool, double)`
    /// Adds access edges for stopping places to the intermodal network
    // void addAccess(const std::string& stopId, const E* stopEdge, const double startPos, const double endPos, const double length, const SumoXMLTag category, bool isAccess, double taxiWait) {
    //         assert(stopEdge != nullptr);
    //         const bool transferCarWalk = ((category == SUMO_TAG_PARKING_AREA && (myCarWalkTransfer & PARKING_AREAS) != 0) ||
    //                                       (category == SUMO_TAG_BUS_STOP && (myCarWalkTransfer & PT_STOPS) != 0));
    //         const bool transferTaxiWalk = ((category == SUMO_TAG_PARKING_AREA && (myCarWalkTransfer & TAXI_DROPOFF_PARKING_AREAS) != 0) ||
    //                                        (category == SUMO_TAG_BUS_STOP && (myCarWalkTransfer & TAXI_DROPOFF_PT) != 0));
    //         const bool transferWalkTaxi = ((category == SUMO_TAG_PARKING_AREA && (myCarWalkTransfer & TAXI_PICKUP_PARKING_AREAS) != 0) ||
    //                                        (category == SUMO_TAG_BUS_STOP && (myCarWalkTransfer & TAXI_PICKUP_PT) != 0));
    //         const double pos = (startPos + endPos) / 2.;
    // #ifdef IntermodalRouter_DEBUG_ACCESS
    //         std::cout << "addAccess stopId=" << stopId << " stopEdge=" << stopEdge->getID() << " pos=" << pos << " length=" << length << " tag=" << toString(category)
    //                   << " access=" << isAccess << " tWait=" << taxiWait << "\n";
    // #endif
    //         if (myStopConnections.count(stopId) == 0) {
    //             myStopConnections[stopId] = new StopEdge<E, L, N, V>(stopId, myNumericalID++, stopEdge, startPos, endPos);
    //             addEdge(myStopConnections[stopId]);
    //         }
    //         _IntermodalEdge* const stopConn = myStopConnections[stopId];
    //         const L* lane = getSidewalk<E, L>(stopEdge);
    //         if (lane != nullptr) {
    //             const std::pair<_IntermodalEdge*, _IntermodalEdge*>& pair = getBothDirections(stopEdge);
    //             double relPos;
    //             bool needSplit;
    //             const int splitIndex = findSplitIndex(pair.first, pos, relPos, needSplit);
    //             _IntermodalEdge* const fwdSplit = needSplit ? new PedestrianEdge<E, L, N, V>(myNumericalID++, stopEdge, lane, true, pos) : nullptr;
    //             splitEdge(pair.first, splitIndex, fwdSplit, relPos, length, needSplit, stopConn);
    //             _IntermodalEdge* const backSplit = needSplit ? new PedestrianEdge<E, L, N, V>(myNumericalID++, stopEdge, lane, false, pos) : nullptr;
    //             splitEdge(pair.second, splitIndex, backSplit, relPos, length, needSplit, stopConn, false);
    //             _IntermodalEdge* carSplit = nullptr;
    //             if (myCarLookup.count(stopEdge) > 0) {
    //                 if (needSplit) {
    //                     carSplit = new CarEdge<E, L, N, V>(myNumericalID++, stopEdge, pos);
    //                 }
    //                 splitEdge(myCarLookup[stopEdge], splitIndex, carSplit, relPos, length, needSplit, stopConn, true, false, transferCarWalk);
    //             }
    //             if (needSplit) {
    //                 if (carSplit != nullptr && (transferCarWalk || transferTaxiWalk)) {
    //                     // adding access from car to walk
    //                     _IntermodalEdge* const beforeSplit = myAccessSplits[myCarLookup[stopEdge]][splitIndex];
    //                     for (_IntermodalEdge* conn : {
    //                                 fwdSplit, backSplit
    //                             }) {
    //                         if (transferCarWalk) {
    //                             _AccessEdge* access = new _AccessEdge(myNumericalID++, beforeSplit, conn, length);
    //                             addEdge(access);
    //                             beforeSplit->addSuccessor(access);
    //                             access->addSuccessor(conn);
    //                         } else if (transferTaxiWalk) {
    //                             addRestrictedCarExit(beforeSplit, stopConn, gTaxiClasses);
    //                         }
    //                     }
    //                 }
    //                 if (carSplit != nullptr && transferWalkTaxi && !isAccess) {
    //                     _AccessEdge* access = new _AccessEdge(myNumericalID++, stopConn, carSplit, 0, SVC_TAXI, SVC_IGNORING, taxiWait);
    //                     addEdge(access);
    //                     stopConn->addSuccessor(access);
    //                     access->addSuccessor(carSplit);
    //                 }
    //
    //                 // fixing depart connections for the forward pedestrian, the backward pedestrian and the car edge
    //                 _IntermodalEdge* const prevDep = getDepartConnector(stopEdge, splitIndex);
    //                 const std::vector<_IntermodalEdge*>& backSplitList = myAccessSplits[pair.second];
    //                 _IntermodalEdge* const backBeforeSplit = backSplitList[backSplitList.size() - 2 - splitIndex];
    //                 _IntermodalEdge* const depConn = new _IntermodalEdge(stopEdge->getID() + "_depart_connector" + toString(pos), myNumericalID++, stopEdge, "!connector");
    //                 depConn->addSuccessor(fwdSplit);
    //                 depConn->addSuccessor(backBeforeSplit);
    //                 depConn->setLength(fwdSplit->getLength());
    //                 prevDep->removeSuccessor(backBeforeSplit);
    //                 prevDep->addSuccessor(backSplit);
    //                 prevDep->setLength(backSplit->getLength());
    //                 if (carSplit != nullptr) {
    //                     depConn->addSuccessor(carSplit);
    //                 }
    //
    //                 // fixing arrival connections for the forward pedestrian, the backward pedestrian and the car edge
    //                 _IntermodalEdge* const prevArr = getArrivalConnector(stopEdge, splitIndex);
    //                 _IntermodalEdge* const fwdBeforeSplit = myAccessSplits[pair.first][splitIndex];
    //                 _IntermodalEdge* const arrConn = new _IntermodalEdge(stopEdge->getID() + "_arrival_connector" + toString(pos), myNumericalID++, stopEdge, "!connector");
    //                 fwdSplit->addSuccessor(arrConn);
    //                 backBeforeSplit->addSuccessor(arrConn);
    //                 arrConn->setLength(fwdSplit->getLength());
    //                 fwdSplit->removeSuccessor(prevArr);
    //                 fwdBeforeSplit->addSuccessor(prevArr);
    //                 prevArr->setLength(backSplit->getLength());
    //                 if (carSplit != nullptr) {
    //                     if (carSplit->removeSuccessor(prevArr)) {
    //                         carSplit->addSuccessor(arrConn);
    //                         myAccessSplits[myCarLookup[stopEdge]][splitIndex]->addSuccessor(prevArr);
    //                     } else {
    //                         // check for restricted access
    //                         for (_IntermodalEdge* out : carSplit->getSuccessors()) {
    //                             _AccessEdge* aOut = dynamic_cast<_AccessEdge*>(out);
    //                             if (aOut != nullptr && aOut->removeSuccessor(prevArr)) {
    //                                 aOut->addSuccessor(arrConn);
    //                                 addRestrictedCarExit(
    //                                     myAccessSplits[myCarLookup[stopEdge]][splitIndex],
    //                                     prevArr,
    //                                     aOut->getVehicleRetriction());
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                 }
    //                 addConnectors(depConn, arrConn, splitIndex + 1);
    //             }
    //         } else {
    //             // pedestrians cannot walk here:
    //             // add stop edge as depart connector so that pedestrians may start at the stop
    //             std::vector<_IntermodalEdge*>& splitList = myDepartLookup[stopEdge];
    //             assert(splitList.size() > 0);
    //             typename std::vector<_IntermodalEdge*>::iterator splitIt = splitList.begin();
    //             while (splitIt != splitList.end() && startPos > (*splitIt)->getEndPos()) {
    //                 ++splitIt;
    //             }
    //             splitList.insert(splitIt, stopConn);
    //
    //             if (!isAccess && (transferWalkTaxi || transferCarWalk || transferTaxiWalk)) {
    //                 _IntermodalEdge* carEdge =  myCarLookup[stopEdge];
    //                 double relPos;
    //                 bool needSplit;
    //                 const int splitIndex = findSplitIndex(carEdge, pos, relPos, needSplit);
    //                 if (needSplit) {
    //                     _IntermodalEdge* carSplit = new CarEdge<E, L, N, V>(myNumericalID++, stopEdge, pos);
    //                     splitEdge(carEdge, splitIndex, carSplit, relPos, length, needSplit, stopConn, true, false, false);
    //
    //                     if (transferCarWalk || transferTaxiWalk) {
    //                         // adding access from car to walk
    //                         _IntermodalEdge* const beforeSplit = myAccessSplits[myCarLookup[stopEdge]][splitIndex];
    //                         if (transferCarWalk) {
    //                             _AccessEdge* access = new _AccessEdge(myNumericalID++, beforeSplit, stopConn, length);
    //                             addEdge(access);
    //                             beforeSplit->addSuccessor(access);
    //                             access->addSuccessor(stopConn);
    //                         } else if (transferTaxiWalk) {
    //                             addRestrictedCarExit(beforeSplit, stopConn, gTaxiClasses);
    //                         }
    //                     }
    //                     if (transferWalkTaxi) {
    //                         _AccessEdge* access = new _AccessEdge(myNumericalID++, stopConn, carSplit, 0, SVC_TAXI, SVC_IGNORING, taxiWait);
    //                         addEdge(access);
    //                         stopConn->addSuccessor(access);
    //                         access->addSuccessor(carSplit);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_access(&mut self, stopId: &str, stopEdge: Ptr<E>, startPos: f64, endPos: f64, length: f64, category: SumoXMLTag, isAccess: bool, taxiWait: f64) {
        todo!("port IntermodalNetwork::addAccess")
    }

    /// C++: `bool IntermodalNetwork::hasPTSchedules() const` [const]
    // bool hasPTSchedules() const {
    //         return myHavePTSchedules;
    //     }
    #[allow(unused_variables)]
    pub fn has_pt_schedules(&self) -> bool {
        todo!("port IntermodalNetwork::hasPTSchedules")
    }

    /// C++: `void IntermodalNetwork::addSchedule(const SUMOVehicleParameter &, const StopParVector *)`
    // void addSchedule(const SUMOVehicleParameter& pars, const StopParVector* addStops = nullptr) {
    //         SUMOTime lastUntil = 0;
    //         StopParVector validStops;
    //         if (addStops != nullptr) {
    //             // stops are part of a stand-alone route. until times are offsets from vehicle departure
    //             for (const SUMOVehicleParameter::Stop& stop : *addStops) {
    //                 if (myStopConnections.count(stop.busstop) > 0) {
    //                     // compute stop times for the first vehicle
    //                     const SUMOTime newUntil = stop.until + pars.depart;
    //                     if (newUntil >= lastUntil) {
    //                         validStops.push_back(stop);
    //                         validStops.back().until = newUntil;
    //                         lastUntil = newUntil;
    //                     } else {
    //                         WRITE_WARNINGF(TL("Ignoring unordered stop at '%' until % for vehicle '%'."), stop.busstop, time2string(stop.until), pars.id);
    //                     }
    //                 }
    //             }
    //         }
    //         for (const SUMOVehicleParameter::Stop& stop : pars.stops) {
    //             // stops are part of the vehicle until times are absolute times for the first vehicle
    //             if (myStopConnections.count(stop.busstop) > 0 && stop.until >= lastUntil) {
    //                 validStops.push_back(stop);
    //                 lastUntil = stop.until;
    //             } else {
    //                 if (stop.busstop != "" && stop.until >= 0) {
    //                     WRITE_WARNINGF(TL("Ignoring stop at '%' until % for vehicle '%'."), stop.busstop, time2string(stop.until), pars.id);
    //                 }
    //             }
    //         }
    //         if (validStops.size() < 2 && pars.line != "taxi") {
    //             WRITE_WARNINGF(TL("Not using public transport line '%' for routing persons. It has less than two usable stops."), pars.line);
    //             return;
    //         }
    //
    //         typename std::vector<_PTEdge*>& lineEdges = myPTLines[pars.line];
    //         if (lineEdges.empty()) {
    //             _IntermodalEdge* lastStop = nullptr;
    //             Position lastPos;
    //             SUMOTime lastTime = 0;
    //             for (const SUMOVehicleParameter::Stop& s : validStops) {
    //                 _IntermodalEdge* currStop = myStopConnections[s.busstop];
    //                 Position stopPos = E::getStopPosition(s);
    //                 if (lastStop != nullptr) {
    //                     _PTEdge* const newEdge = new _PTEdge(s.busstop, myNumericalID++, lastStop, currStop->getEdge(), pars.line, lastPos.distanceTo(stopPos));
    //                     addEdge(newEdge);
    //                     newEdge->addSchedule(pars.id, lastTime, pars.repetitionNumber, pars.repetitionOffset, s.until - lastTime);
    //                     myHavePTSchedules = true;
    //                     lastStop->addSuccessor(newEdge);
    //                     newEdge->addSuccessor(currStop);
    //                     lineEdges.push_back(newEdge);
    //                 }
    //                 lastTime = s.until;
    //                 lastStop = currStop;
    //                 lastPos = stopPos;
    //             }
    //             if (pars.line != "taxi" && validStops.front().busstop == validStops.back().busstop) {
    //                 myLoopedLines.insert(pars.line);
    //             }
    //         } else {
    //             if (validStops.size() != lineEdges.size() + 1) {
    //                 WRITE_WARNINGF("Number of stops for public transport line '%' does not match earlier definitions, ignoring schedule.", pars.line);
    //                 return;
    //             }
    //             if (lineEdges.front()->getEntryStop() != myStopConnections[validStops.front().busstop]) {
    //                 WRITE_WARNINGF("Different stop for '%' compared to earlier definitions, ignoring schedule.", pars.line);
    //                 return;
    //             }
    //             typename std::vector<_PTEdge*>::const_iterator lineEdge = lineEdges.begin();
    //             typename StopParVector::const_iterator s = validStops.begin() + 1;
    //             for (; s != validStops.end(); ++s, ++lineEdge) {
    //                 if ((*lineEdge)->getSuccessors(SVC_IGNORING)[0] != myStopConnections[s->busstop]) {
    //                     WRITE_WARNINGF("Different stop for '%' compared to earlier definitions, ignoring schedule.", pars.line);
    //                     return;
    //                 }
    //             }
    //             SUMOTime lastTime = validStops.front().until;
    //             if (lineEdges.front()->hasSchedule(lastTime)) {
    //                 WRITE_WARNINGF("Duplicate schedule for '%' at time=%.", pars.line, time2string(lastTime));
    //             }
    //             for (lineEdge = lineEdges.begin(), s = validStops.begin() + 1; lineEdge != lineEdges.end(); ++lineEdge, ++s) {
    //                 (*lineEdge)->addSchedule(pars.id, lastTime, pars.repetitionNumber, pars.repetitionOffset, s->until - lastTime);
    //                 myHavePTSchedules = true;
    //                 lastTime = s->until;
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_schedule(&mut self, pars: &SUMOVehicleParameter, addStops: Ptr<StopParVector>) {
        todo!("port IntermodalNetwork::addSchedule")
    }

    /// C++: `void IntermodalNetwork::addCarAccess(const E *, SUMOVehicleClass, double)`
    /// Adds access edges for transfering from walking to vehicle use
    // void addCarAccess(const E* edge, SUMOVehicleClass svc, double traveltime) {
    //         assert(edge != nullptr);
    //         assert(myCarLookup.count(edge) != 0);
    //         assert(myBidiLookup.count(edge) != 0);
    //         EdgePair pedestrianEdges = myBidiLookup[edge];
    //         _IntermodalEdge* carEdge = myCarLookup[edge];
    //         _AccessEdge* access = new _AccessEdge(myNumericalID++, pedestrianEdges.first, carEdge, 0, svc, SVC_IGNORING, traveltime);
    //         addEdge(access);
    //         pedestrianEdges.first->addSuccessor(access);
    //         pedestrianEdges.second->addSuccessor(access);
    //         access->addSuccessor(carEdge);
    //     }
    #[allow(unused_variables)]
    pub fn add_car_access(&mut self, edge: Ptr<E>, svc: SUMOVehicleClass, traveltime: f64) {
        todo!("port IntermodalNetwork::addCarAccess")
    }

    /// C++: `void IntermodalNetwork::addRestrictedCarExit(_IntermodalEdge *, _IntermodalEdge *, SVCPermissions)`
    /// Adds access edges for transfering from driving to walking that are only usable by a particular vehicle class
    // void addRestrictedCarExit(_IntermodalEdge* from, _IntermodalEdge* to, SVCPermissions vehicleRestriction) {
    //         _AccessEdge* access = new _AccessEdge(myNumericalID++, from, to, 0, SVC_IGNORING, vehicleRestriction);
    //         addEdge(access);
    //         from->addSuccessor(access);
    //         access->addSuccessor(to);
    //     }
    #[allow(unused_variables)]
    pub fn add_restricted_car_exit(&mut self, from: Ptr<AccessEdge__IntermodalEdge>, to: Ptr<AccessEdge__IntermodalEdge>, vehicleRestriction: SVCPermissions) {
        todo!("port IntermodalNetwork::addRestrictedCarExit")
    }

    /// C++: `bool IntermodalNetwork::isLooped(const std::string) const` [const]
    // bool isLooped(const std::string lineID) const {
    //         return myLoopedLines.count(lineID) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_looped(&self, lineID: String) -> bool {
        todo!("port IntermodalNetwork::isLooped")
    }

    /// C++: `int IntermodalNetwork::findSplitIndex(_IntermodalEdge *const, const double, double &, bool &) const` [const, private]
    /// Returns where to insert or use the split edge
    // int findSplitIndex(_IntermodalEdge* const toSplit, const double pos, double& relPos, bool& needSplit) const {
    //         relPos = pos;
    //         needSplit = true;
    //         int splitIndex = 0;
    //         const auto& splitList = myAccessSplits.find(toSplit);
    //         if (splitList != myAccessSplits.end() && !splitList->second.empty()) {
    //             for (const _IntermodalEdge* const split : splitList->second) {
    //                 if (relPos < split->getLength() + POSITION_EPS) {
    //                     break;
    //                 }
    //                 relPos -= split->getLength();
    //                 splitIndex++;
    //             }
    //             assert(splitIndex < (int)splitList->second.size());
    //             if (splitIndex + 1 < (int)splitList->second.size() && fabs(relPos - splitList->second[splitIndex]->getLength()) < POSITION_EPS) {
    //                 needSplit = false;
    //             }
    //         }
    //         return splitIndex;
    //     }
    #[allow(unused_variables)]
    pub fn find_split_index(&self, toSplit: Ptr<AccessEdge__IntermodalEdge>, pos: f64, relPos: &mut f64, needSplit: &mut bool) -> i32 {
        todo!("port IntermodalNetwork::findSplitIndex")
    }

    /// C++: `void IntermodalNetwork::splitEdge(_IntermodalEdge *const, int, _IntermodalEdge *, const double, const double, const bool, _IntermodalEdge *const, const bool, const bool, const bool)` [private]
    /// Splits an edge (if necessary) and connects it to a stopping edge
    // void splitEdge(_IntermodalEdge* const toSplit, int splitIndex,
    //                    _IntermodalEdge* afterSplit, const double relPos, const double length, const bool needSplit,
    //                    _IntermodalEdge* const stopConn, const bool forward = true, const bool addExit = true, const bool addEntry = true) {
    //         std::vector<_IntermodalEdge*>& splitList = myAccessSplits[toSplit];
    //         if (splitList.empty()) {
    //             splitList.push_back(toSplit);
    //         }
    //         if (!forward) {
    //             splitIndex = (int)splitList.size() - 1 - splitIndex;
    //             if (!needSplit) {
    //                 splitIndex--;
    //             }
    //         }
    //         _IntermodalEdge* beforeSplit = splitList[splitIndex];
    //         if (needSplit) {
    //             addEdge(afterSplit);
    //             beforeSplit->transferSuccessors(afterSplit);
    //             beforeSplit->addSuccessor(afterSplit);
    //             if (forward) {
    //                 afterSplit->setLength(MAX2(0.0, beforeSplit->getLength() - relPos));
    //                 beforeSplit->setLength(relPos);
    //             } else {
    //                 afterSplit->setLength(relPos);
    //                 beforeSplit->setLength(MAX2(0.0, beforeSplit->getLength() - relPos));
    //                 // rename backward edges for easier referencing
    //                 const std::string newID = beforeSplit->getID();
    //                 beforeSplit->setID(afterSplit->getID());
    //                 afterSplit->setID(newID);
    //             }
    //             splitList.insert(splitList.begin() + splitIndex + 1, afterSplit);
    //         } else {
    //             // don't split, use the present split edges
    //             afterSplit = splitList[splitIndex + 1];
    //         }
    //         // add access to / from edge
    //         if (addEntry) {
    //             _AccessEdge* access = new _AccessEdge(myNumericalID++, beforeSplit, stopConn, length);
    //             addEdge(access);
    //             beforeSplit->addSuccessor(access);
    //             access->addSuccessor(stopConn);
    //         }
    //         if (addExit) {
    //             // pedestrian case only, exit from public to pedestrian
    //             _AccessEdge* exit = new _AccessEdge(myNumericalID++, stopConn, afterSplit, length);
    //             addEdge(exit);
    //             stopConn->addSuccessor(exit);
    //             exit->addSuccessor(afterSplit);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn split_edge(&mut self, toSplit: Ptr<AccessEdge__IntermodalEdge>, splitIndex: i32, afterSplit: Ptr<AccessEdge__IntermodalEdge>, relPos: f64, length: f64, needSplit: bool, stopConn: Ptr<AccessEdge__IntermodalEdge>, forward: bool, addExit: bool, addEntry: bool) {
        todo!("port IntermodalNetwork::splitEdge")
    }

    /// C++: `IntermodalNetwork<E, L, N, V> & IntermodalNetwork::operator=(const IntermodalNetwork<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &IntermodalNetwork<E, L, N, V>) -> &mut IntermodalNetwork<E, L, N, V> {
        todo!("port IntermodalNetwork::operator=")
    }

}

