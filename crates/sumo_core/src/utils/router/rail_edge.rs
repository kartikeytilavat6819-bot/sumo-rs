//! Port of utils/router/RailEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<std::pair<const _RailEdge *, const _RailEdge *>> RailEdge::ConstEdgePairVector`
pub type RailEdge_ConstEdgePairVector = Vec<(Ptr<RailEdge__RailEdge>, Ptr<RailEdge__RailEdge>)>;

/// C++ typedef `RailEdge<E, V> RailEdge::_RailEdge`
pub type RailEdge__RailEdge = Unmapped /* RailEdge<E, V> */;

/// the edge type representing backward edges
/// C++ class `RailEdge` (utils/router/RailEdge.h)
pub struct RailEdge<E, V> {
    pub myNumericalID: i32,
    pub myID: String,
    pub myOriginal: Ptr<E>,
    pub myTurnaround: Ptr<RailEdge__RailEdge>,
    pub myIsVirtual: bool,
    /// actual edges to return when passing this (turnaround) edge - only forward
    pub myReplacementEdges: Vec<Ptr<E>>,
    /// maximum train length for passing this (turnaround) edge
    pub myMaxLength: f64,
    /// length of the edge where this turn starts
    pub myStartLength: f64,
    /// The successors available for a given vClass
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, ReversedEdge_ConstEdgePairVector>,
    pub myViaSuccessors: ReversedEdge_ConstEdgePairVector,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> RailEdge<E, V> {
    /// C++: `void RailEdge::RailEdge<E, V>(const E *)`
    // RailEdge(const E* orig) :
    //         myNumericalID(orig->getNumericalID()),
    //         myOriginal(orig),
    //         myTurnaround(nullptr),
    //         myIsVirtual(true)
    //     { }
    #[allow(unused_variables)]
    pub fn new(orig: Ptr<E>) -> Self {
        todo!("port RailEdge::RailEdge<E, V>")
    }

    /// C++: `void RailEdge::RailEdge<E, V>(const E *, const E *, int)`
    // RailEdge(const E* turnStart, const E* turnEnd, int numericalID) :
    //         myNumericalID(numericalID),
    //         myID("TrainReversal!" + turnStart->getID() + "->" + turnEnd->getID()),
    //         myOriginal(nullptr),
    //         myTurnaround(nullptr),
    //         myIsVirtual(true),
    //         myMaxLength(turnStart->getLength() - REVERSAL_SLACK),
    //         myStartLength(turnStart->getLength() - REVERSAL_SLACK) {
    //         myViaSuccessors.push_back(std::make_pair(turnEnd->getRailwayRoutingEdge(), nullptr));
    //     }
    #[allow(unused_variables)]
    pub fn new_2(turnStart: Ptr<E>, turnEnd: Ptr<E>, numericalID: i32) -> Self {
        todo!("port RailEdge::RailEdge<E, V>")
    }

    /// C++: `void RailEdge::~RailEdge<E, V>()` [virtual]
    /// Destructor.
    // virtual ~RailEdge() {
    //         delete myTurnaround;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RailEdge::~RailEdge<E, V>")
    }

    /// C++: `void RailEdge::update(double, const std::vector<const E *> &)`
    // void update(double maxTrainLength, const std::vector<const E*>& replacementEdges) {
    //         if (maxTrainLength > myMaxLength) {
    //             myMaxLength = maxTrainLength;
    //             myReplacementEdges = replacementEdges;
    // #ifdef RailEdge_DEBUG_INIT
    //             std::cout << "    update RailEdge " << getID() << " myMaxLength=" << myMaxLength << " repl=" << toString(myReplacementEdges) << "\n";
    // #endif
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn update(&mut self, maxTrainLength: f64, replacementEdges: &Vec<Ptr<E>>) {
        todo!("port RailEdge::update")
    }

    /// C++: `void RailEdge::addVirtualTurns(const E *, const E *, std::vector<_RailEdge *> &, int &, double, double, const std::vector<const E *> &)`
    // void addVirtualTurns(const E* forward, const E* backward,
    //                          std::vector<_RailEdge*>& railEdges, int& numericalID, double dist,
    //                          double maxTrainLength, const std::vector<const E*>& replacementEdges) {
    //         // search backwards until dist and add virtual turnaround edges with
    //         // replacement edges up to the real turnaround
    // #ifdef RailEdge_DEBUG_INIT
    //         std::cout << "addVirtualTurns forward=" << forward->getID() << " backward=" << backward->getID() << " dist=" << dist
    //                   << " maxLength=" << maxTrainLength << " repl=" << toString(replacementEdges) << "\n";
    // #endif
    //         if (dist <= 0) {
    //             return;
    //         }
    //         for (const E* prev : forward->getPredecessors()) {
    //             if (prev == backward) {
    //                 continue;
    //             }
    //             const E* bidi = prev->getBidiEdge();
    //             if (bidi != nullptr && backward->isConnectedTo(*bidi, SVC_IGNORING)) {
    //                 _RailEdge* prevRailEdge = prev->getRailwayRoutingEdge();
    //                 if (prevRailEdge->myTurnaround == nullptr) {
    //                     prevRailEdge->myTurnaround = new _RailEdge(prev, bidi, numericalID++);
    //                     prevRailEdge->myViaSuccessors.push_back(std::make_pair(prevRailEdge->myTurnaround, nullptr));
    //                     railEdges.push_back(prevRailEdge->myTurnaround);
    // #ifdef RailEdge_DEBUG_INIT
    //                     std::cout << "  RailEdge " << prevRailEdge->getID() << " virtual turnaround " << prevRailEdge->myTurnaround->getID() << "\n";
    // #endif
    //                 }
    //                 /*
    //                 // doesn't compile though I don't know why
    //                 auto itFound = std::find(replacementEdges.begin(), replacementEdges.end(), prev);
    //                 bool notFound = itFound == replacementEdges.end();
    //                 */
    //                 bool notFound = true;
    //                 for (const E* r : replacementEdges) {
    //                     if (r == prev) {
    //                         notFound = false;
    //                         break;
    //                     }
    //                 }
    //
    //                 if (notFound) {
    //                     // prevent loops in replacementEdges
    //                     prevRailEdge->myTurnaround->update(prev->getLength() + maxTrainLength - REVERSAL_SLACK, replacementEdges);
    //                     std::vector<const E*> replacementEdges2;
    //                     replacementEdges2.push_back(prev);
    //                     replacementEdges2.insert(replacementEdges2.end(), replacementEdges.begin(), replacementEdges.end());
    //                     addVirtualTurns(prev, bidi, railEdges, numericalID, dist - prev->getLength(),
    //                                     maxTrainLength + prev->getLength(), replacementEdges2);
    //                 }
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_virtual_turns(&mut self, forward: Ptr<E>, backward: Ptr<E>, railEdges: &mut Vec<Ptr<RailEdge__RailEdge>>, numericalID: &mut i32, dist: f64, maxTrainLength: f64, replacementEdges: &Vec<Ptr<E>>) {
        todo!("port RailEdge::addVirtualTurns")
    }

    /// C++: `void RailEdge::init(std::vector<_RailEdge *> &, int &, double, bool)`
    // void init(std::vector<_RailEdge*>& railEdges, int& numericalID, double maxTrainLength, bool permitReversal) {
    //         // replace turnaround-via with an explicit RailEdge that checks length
    //         for (const auto& viaPair : myOriginal->getViaSuccessors()) {
    //             if (viaPair.first == myOriginal->getBidiEdge()) {
    //                 if (!permitReversal) {
    //                     continue;
    //                 }
    //                 // direction reversal
    //                 if (myTurnaround == nullptr) {
    //                     myTurnaround = new _RailEdge(myOriginal, viaPair.first, numericalID++);
    //                     myViaSuccessors.push_back(std::make_pair(myTurnaround, nullptr));
    //                     railEdges.push_back(myTurnaround);
    // #ifdef RailEdge_DEBUG_INIT
    //                     std::cout << "   added new turnaround " << myTurnaround->getID() << "\n";
    // #endif
    //                 }
    // #ifdef RailEdge_DEBUG_INIT
    //                 std::cout << "RailEdge " << getID() << " actual turnaround " << myTurnaround->getID() << "\n";
    // #endif
    //                 myTurnaround->myIsVirtual = false;
    //                 // ensure at least one virtual turnaround (at the start of the
    //                 // edge) to avoid driving up to the end of long edges
    //                 const double initialDist = MAX2(maxTrainLength - getLength(), POSITION_EPS);
    //                 addVirtualTurns(myOriginal, viaPair.first, railEdges, numericalID,
    //                                 initialDist, getLength(), std::vector<const E*> {myOriginal});
    //             } else {
    //                 myViaSuccessors.push_back(std::make_pair(viaPair.first->getRailwayRoutingEdge(),
    //                                           viaPair.second == nullptr ? nullptr : viaPair.second->getRailwayRoutingEdge()));
    //             }
    //         }
    // #ifdef RailEdge_DEBUG_SUCCESSORS
    //         std::cout << "RailEdge " << getID() << " successors=" << myViaSuccessors.size() << " orig=" << myOriginal->getViaSuccessors().size() << "\n";
    //         for (const auto& viaPair : myViaSuccessors) {
    //             std::cout << "    " << viaPair.first->getID() << "\n";
    //         }
    // #endif
    //     }
    #[allow(unused_variables)]
    pub fn init(&mut self, railEdges: &mut Vec<Ptr<RailEdge__RailEdge>>, numericalID: &mut i32, maxTrainLength: f64, permitReversal: bool) {
        todo!("port RailEdge::init")
    }

    /// C++: `int RailEdge::getNumericalID() const` [const]
    /// Returns the index (numeric id) of the edge
    // inline int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        todo!("port RailEdge::getNumericalID")
    }

    /// C++: `const E * RailEdge::getOriginal() const` [const]
    /// Returns the original edge
    // const E* getOriginal() const {
    //         return myOriginal;
    //     }
    #[allow(unused_variables)]
    pub fn get_original(&self) -> Ptr<E> {
        todo!("port RailEdge::getOriginal")
    }

    /// C++: `const std::string & RailEdge::getID() const` [const]
    /// Returns the id of the edge
    // const std::string& getID() const {
    //         return myOriginal != nullptr ? myOriginal->getID() : myID;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        todo!("port RailEdge::getID")
    }

    /// C++: `void RailEdge::insertOriginalEdges(double, std::vector<const E *> &) const` [const]
    // void insertOriginalEdges(double length, std::vector<const E*>& into) const {
    //         if (myOriginal != nullptr) {
    //             into.push_back(myOriginal);
    //         } else {
    //             double seen = myStartLength;
    //             int nPushed = 0;
    //             //std::cout << "insertOriginalEdges e=" << getID() << " length=" << length << " seen=" << seen << " into=" << toString(into) << "\n";
    //             if (seen >= length && !myIsVirtual) {
    //                 return;
    //             }
    //             // we need to find a replacement edge that has a real turn
    //             for (const E* edge : myReplacementEdges) {
    //                 into.push_back(edge);
    //                 nPushed++;
    //                 seen += edge->getLength() - REVERSAL_SLACK;
    //                 //std::cout << "insertOriginalEdges e=" << getID() << " length=" << length << " seen=" << seen << " into=" << toString(into) << "\n";
    //                 if (seen >= length && edge->isConnectedTo(*edge->getBidiEdge(), SVC_IGNORING)) {
    //                     break;
    //                 }
    //             }
    //             const int last = (int)into.size() - 1;
    //             for (int i = 0; i < nPushed; i++) {
    //                 into.push_back(into[last - i]->getBidiEdge());
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn insert_original_edges(&self, length: f64, into: &mut Vec<Ptr<E>>) {
        todo!("port RailEdge::insertOriginalEdges")
    }

    /// C++: `double RailEdge::getLength() const` [const]
    /// Returns the length of the edge
    // double getLength() const {
    //         return myOriginal == nullptr ? 0 : myOriginal->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port RailEdge::getLength")
    }

    /// C++: `bool RailEdge::isInternal() const` [const]
    // bool isInternal() const {
    //         return myOriginal->isInternal();
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        todo!("port RailEdge::isInternal")
    }

    /// C++: `bool RailEdge::prohibits(const V *const) const` [const]
    // inline bool prohibits(const V* const vehicle) const {
    // #ifdef RailEdge_DEBUG_TURNS
    //         if (myOriginal == nullptr && RailEdge_DEBUG_COND(vehicle)) {
    //             std::cout << getID() << " maxLength=" << myMaxLength << " veh=" << vehicle->getID() << " length=" << vehicle->getLength() << "\n";
    //         }
    // #endif
    //         return vehicle->getLength() > myMaxLength
    //             || (myOriginal != nullptr && myOriginal->prohibits(vehicle))
    //             || replacementProhibits(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, vehicle: Ptr<V>) -> bool {
        todo!("port RailEdge::prohibits")
    }

    /// C++: `bool RailEdge::replacementProhibits(const V *const) const` [const]
    // bool replacementProhibits(const V* const vehicle) const {
    //         for (const E* e : myReplacementEdges) {
    //             if (e->prohibits(vehicle)) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn replacement_prohibits(&self, vehicle: Ptr<V>) -> bool {
        todo!("port RailEdge::replacementProhibits")
    }

    /// C++: `bool RailEdge::restricts(const V *const) const` [const]
    // inline bool restricts(const V* const vehicle) const {
    //         return (myOriginal != nullptr && myOriginal->restricts(vehicle)) || replacementRestricts(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, vehicle: Ptr<V>) -> bool {
        todo!("port RailEdge::restricts")
    }

    /// C++: `bool RailEdge::replacementRestricts(const V *const) const` [const]
    // bool replacementRestricts(const V* const vehicle) const {
    //         for (const E* e : myReplacementEdges) {
    //             if (e->restricts(vehicle)) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn replacement_restricts(&self, vehicle: Ptr<V>) -> bool {
        todo!("port RailEdge::replacementRestricts")
    }

    /// C++: `const ConstEdgePairVector & RailEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const]
    // const ConstEdgePairVector& getViaSuccessors(SUMOVehicleClass vClass = SVC_IGNORING, bool ignoreTransientPermissions = false) const {
    //         if (vClass == SVC_IGNORING || myOriginal == nullptr || myOriginal->isTazConnector()) { // || !MSNet::getInstance()->hasPermissions()) {
    //             return myViaSuccessors;
    //         }
    // #ifdef HAVE_FOX
    //         FXMutexLock lock(mySuccessorMutex);
    // #endif
    //         auto i = myClassesViaSuccessorMap.find(vClass);
    //         if (i != myClassesViaSuccessorMap.end()) {
    //             // can use cached value
    //             return i->second;
    //         }
    //         // instantiate vector
    //         ConstEdgePairVector& result = myClassesViaSuccessorMap[vClass];
    //         // this vClass is requested for the first time. rebuild all successors
    //         for (const auto& viaPair : myViaSuccessors) {
    //             if (viaPair.first->myOriginal == nullptr
    //                     || viaPair.first->myOriginal->isTazConnector()
    //                     || myOriginal->isConnectedTo(*viaPair.first->myOriginal, vClass, ignoreTransientPermissions)) {
    //                 result.push_back(viaPair);
    //             }
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &ReversedEdge_ConstEdgePairVector {
        todo!("port RailEdge::getViaSuccessors")
    }

    /// C++: `bool RailEdge::isVirtual() const` [const]
    // bool isVirtual() const {
    //         return myIsVirtual;
    //     }
    #[allow(unused_variables)]
    pub fn is_virtual(&self) -> bool {
        todo!("port RailEdge::isVirtual")
    }

}

