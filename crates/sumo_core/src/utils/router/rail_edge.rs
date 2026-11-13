//! Port of utils/router/RailEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<std::pair<const _RailEdge *, const _RailEdge *>> RailEdge::ConstEdgePairVector`
pub type RailEdge_ConstEdgePairVector<E, V> = Vec<(Ptr<RailEdge<E, V>>, Ptr<RailEdge<E, V>>)>;

/// C++ typedef `RailEdge<E, V> RailEdge::_RailEdge`
pub type RailEdge__RailEdge<E, V> = RailEdge<E, V>;

/// C++ `#define REVERSAL_SLACK (POSITION_EPS + NUMERICAL_EPS)`
const REVERSAL_SLACK: f64 = utils::geom::POSITION_EPS + utils::geom::NUMERICAL_EPS;

/// the edge type representing backward edges
/// C++ class `RailEdge` (utils/router/RailEdge.h)
pub struct RailEdge<E, V> {
    pub myNumericalID: i32,
    pub myID: String,
    pub myOriginal: Ptr<E>,
    pub myTurnaround: Ptr<RailEdge__RailEdge<E, V>>,
    pub myIsVirtual: bool,
    /// actual edges to return when passing this (turnaround) edge - only forward
    pub myReplacementEdges: Vec<Ptr<E>>,
    /// maximum train length for passing this (turnaround) edge
    pub myMaxLength: f64,
    /// length of the edge where this turn starts
    pub myStartLength: f64,
    /// The successors available for a given vClass
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, RailEdge_ConstEdgePairVector<E, V>>,
    pub myViaSuccessors: RailEdge_ConstEdgePairVector<E, V>,
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
        // shortcut: needs router trait bounds (lead), E::getNumericalID is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getNumericalID")
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
        // shortcut: needs router trait bounds (lead), E::getID/getLength/getRailwayRoutingEdge are not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getID")
    }

    /// C++: `void RailEdge::~RailEdge<E, V>()` [virtual]
    /// Destructor.
    // virtual ~RailEdge() {
    //         delete myTurnaround;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myTurnaround.is_null() {
            // turnarounds are created with Box::into_raw in add_virtual_turns/init; delete runs the nested destructor
            let mut turnaround = unsafe { Box::from_raw(self.myTurnaround.as_raw()) };
            turnaround.drop_();
        }
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
        if maxTrainLength > self.myMaxLength {
            self.myMaxLength = maxTrainLength;
            self.myReplacementEdges = replacementEdges.clone();
        }
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
    pub fn add_virtual_turns(&mut self, forward: Ptr<E>, backward: Ptr<E>, railEdges: &mut Vec<Ptr<RailEdge__RailEdge<E, V>>>, numericalID: &mut i32, dist: f64, maxTrainLength: f64, replacementEdges: &Vec<Ptr<E>>) {
        if dist <= 0. {
            return;
        }
        // shortcut: needs router trait bounds (lead), E::getPredecessors/getBidiEdge/isConnectedTo/getLength are not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getPredecessors")
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
    pub fn init(&mut self, railEdges: &mut Vec<Ptr<RailEdge__RailEdge<E, V>>>, numericalID: &mut i32, maxTrainLength: f64, permitReversal: bool) {
        // shortcut: needs router trait bounds (lead), E::getViaSuccessors/getBidiEdge/getRailwayRoutingEdge are not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getViaSuccessors")
    }

    /// C++: `int RailEdge::getNumericalID() const` [const]
    /// Returns the index (numeric id) of the edge
    // inline int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myNumericalID
    }

    /// C++: `const E * RailEdge::getOriginal() const` [const]
    /// Returns the original edge
    // const E* getOriginal() const {
    //         return myOriginal;
    //     }
    #[allow(unused_variables)]
    pub fn get_original(&self) -> Ptr<E> {
        self.myOriginal
    }

    /// C++: `const std::string & RailEdge::getID() const` [const]
    /// Returns the id of the edge
    // const std::string& getID() const {
    //         return myOriginal != nullptr ? myOriginal->getID() : myID;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        if self.myOriginal.is_null() {
            return &self.myID;
        }
        // shortcut: needs router trait bounds (lead), E::getID is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getID")
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
        if !self.myOriginal.is_null() {
            into.push(self.myOriginal);
        } else {
            // shortcut: needs router trait bounds (lead), E::getLength/isConnectedTo/getBidiEdge are not callable on generic E
            panic!("shortcut: needs router trait bounds (lead), E::getLength")
        }
    }

    /// C++: `double RailEdge::getLength() const` [const]
    /// Returns the length of the edge
    // double getLength() const {
    //         return myOriginal == nullptr ? 0 : myOriginal->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        if self.myOriginal.is_null() {
            return 0.;
        }
        // shortcut: needs router trait bounds (lead), E::getLength is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getLength")
    }

    /// C++: `bool RailEdge::isInternal() const` [const]
    // bool isInternal() const {
    //         return myOriginal->isInternal();
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        // shortcut: needs router trait bounds (lead), E::isInternal is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::isInternal")
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
        // shortcut: needs router trait bounds (lead), V::getLength and E::prohibits are not callable on generic E/V
        panic!("shortcut: needs router trait bounds (lead), V::getLength")
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
        if !self.myReplacementEdges.is_empty() {
            // shortcut: needs router trait bounds (lead), E::prohibits(V) is not callable on generic E
            panic!("shortcut: needs router trait bounds (lead), E::prohibits")
        }
        false
    }

    /// C++: `bool RailEdge::restricts(const V *const) const` [const]
    // inline bool restricts(const V* const vehicle) const {
    //         return (myOriginal != nullptr && myOriginal->restricts(vehicle)) || replacementRestricts(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, vehicle: Ptr<V>) -> bool {
        if !self.myOriginal.is_null() {
            // shortcut: needs router trait bounds (lead), E::restricts(V) is not callable on generic E
            panic!("shortcut: needs router trait bounds (lead), E::restricts")
        }
        self.replacement_restricts(vehicle)
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
        if !self.myReplacementEdges.is_empty() {
            // shortcut: needs router trait bounds (lead), E::restricts(V) is not callable on generic E
            panic!("shortcut: needs router trait bounds (lead), E::restricts")
        }
        false
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
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &RailEdge_ConstEdgePairVector<E, V> {
        if vClass == SVC_IGNORING || self.myOriginal.is_null() {
            return &self.myViaSuccessors;
        }
        // shortcut: needs router trait bounds (lead), E::isTazConnector/isConnectedTo are not callable on generic E; the rebuild also writes the mutable cache (needs Ptr::new(self).m())
        panic!("shortcut: needs router trait bounds (lead), E::isTazConnector")
    }

    /// C++: `bool RailEdge::isVirtual() const` [const]
    // bool isVirtual() const {
    //         return myIsVirtual;
    //     }
    #[allow(unused_variables)]
    pub fn is_virtual(&self) -> bool {
        self.myIsVirtual
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_keeps_longest_train_length() {
        let mut e: RailEdge<u8, u8> = RailEdge {
            myNumericalID: 0,
            myID: String::new(),
            myOriginal: Ptr::null(),
            myTurnaround: Ptr::null(),
            myIsVirtual: true,
            myReplacementEdges: vec![],
            myMaxLength: 10.,
            myStartLength: 0.,
            myClassesViaSuccessorMap: BTreeMap::new(),
            myViaSuccessors: vec![],
            _p: std::marker::PhantomData,
        };
        e.update(5., &vec![]);
        assert_eq!(e.myMaxLength, 10.);
        assert!(e.myReplacementEdges.is_empty());
        e.update(15., &vec![Ptr::null()]);
        assert_eq!(e.myMaxLength, 15.);
        assert_eq!(e.myReplacementEdges.len(), 1);
    }
}

