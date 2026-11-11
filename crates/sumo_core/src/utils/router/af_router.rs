//! Port of utils/router/AFRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `typename KDTreePartition<E, N, V>::Cell AFRouter::Cell`
pub type AFRouter_Cell = KDTreePartition_Cell;

/// C++ typedef `typename AFInfo<E>::FlagInfo AFRouter::FlagInfo`
pub type AFRouter_FlagInfo = AFInfo_FlagInfo;

/// C++ typedef `AbstractLookupTable<FlippedEdge<E, N, V>, V> AFRouter::FlippedLookupTable`
pub type AFRouter_FlippedLookupTable = Unmapped /* AbstractLookupTable<FlippedEdge<E, N, V>, V> */;

/// C++ typedef `AbstractLookupTable<E, V> AFRouter::LookupTable`
pub type AFRouter_LookupTable = Unmapped /* AbstractLookupTable<E, V> */;

/// Computes the shortest path through a network with an arc flag routing algorithm (Hilger et al.) in its multi-level variant (also called "stripped SHARC" by Delling et al.)
/// C++ class `AFRouter` (utils/router/AFRouter.h)
pub struct AFRouter<E, N, V, M> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    /// Edge infos containing the associated edge and its arc flags
    pub myFlagInfos: Ptr<i32>,
    /// The partition
    pub myPartition: Ptr<KDTreePartition<E, N, V>>,
    /// The comparator for edge information
    pub myComparator: AStarRouter_EdgeInfoComparator,
    /// The lookup table for travel time heuristics
    pub myLookupTable: Rc<AStarRouter_LookupTable>,
    /// The maximum speed in the network
    pub myMaxSpeed: f64,
    /// The validity duration of one weight interval
    pub myWeightPeriod: SUMOTime,
    /// The validity duration of the current flag infos (exclusive)
    pub myValidUntil: SUMOTime,
    /// The builder
    pub myBuilder: Ptr<AFBuilder<E, N, V, M>>,
    /// The type of this router
    pub myType: String,
    /// Counters for performance logging
    pub myQueryVisits: i64,
    pub myNumQueries: i64,
    /// The time spent querying in milliseconds
    pub myQueryStartTime: i64,
    pub myQueryTimeSum: i64,
    /// The cell of the last settled edge
    pub myLastSettledEdgeCell: Ptr<KDTreePartition_Cell>,
    /// The last flag context
    pub myLastFlagContext: Unmapped /* std::tuple<int, int, bool> */,
    /// The cell of the target edge at SHARC level 0
    pub myTargetEdgeCellLevel0: Ptr<KDTreePartition_Cell>,
    _p: std::marker::PhantomData<(E, N, V, M,)>,
}

impl<E, N, V, M> std::ops::Deref for AFRouter<E, N, V, M> {
    type Target = SUMOAbstractRouter<E, V>;
    fn deref(&self) -> &SUMOAbstractRouter<E, V> {
        &self.base
    }
}
impl<E, N, V, M> std::ops::DerefMut for AFRouter<E, N, V, M> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, V> {
        &mut self.base
    }
}

impl<E, N, V, M> AFRouter<E, N, V, M> {
    /// C++: `typename SUMOAbstractRouter<E, V>::EdgeInfo * AFRouter::edgeInfo(const E *const)`
    /// Returns the edge information for the passed edge
    // typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo(const E* const edge) {
    //         return &(this->myEdgeInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn edge_info(&mut self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        todo!("port AFRouter::edgeInfo")
    }

    /// C++: `const typename SUMOAbstractRouter<E, V>::EdgeInfo * AFRouter::edgeInfo(const E *const) const` [const]
    /// Returns the edge information for the passed edge
    // const typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo(const E* const edge) const {
    //         return &(this->myEdgeInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn edge_info_2(&self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        todo!("port AFRouter::edgeInfo")
    }

    /// C++: `void AFRouter::AFRouter<E, N, V, M>(const int &, const KDTreePartition<E, N, V> *, bool, typename SUMOAbstractRouter<E, V>::Operation, typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation, SUMOTime, const std::shared_ptr<const LookupTable>, const std::shared_ptr<const FlippedLookupTable>, const bool, const bool)`
    /// Constructor
    // AFRouter(const std::vector<E*>& edges,
    //              const KDTreePartition<E, N, V>* partition,
    //              bool unbuildIsWarning,
    //              typename SUMOAbstractRouter<E, V>::Operation operation, typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation flippedOperation,
    //              SUMOTime weightPeriod, const std::shared_ptr<const LookupTable> lookup = nullptr,
    //              const std::shared_ptr<const FlippedLookupTable> flippedLookup = nullptr,
    //              const bool havePermissions = false, const bool haveRestrictions = false) :
    //         SUMOAbstractRouter<E, V>("arcFlagRouter", unbuildIsWarning, operation, nullptr, havePermissions, haveRestrictions),
    //         myFlagInfos(nullptr),
    //         myPartition(partition),
    //         myLookupTable(lookup),
    //         myMaxSpeed(NUMERICAL_EPS),
    //         myWeightPeriod(weightPeriod),
    //         myValidUntil(0),
    //         myBuilder(new AFBuilder<E, N, V, M>(myPartition->getNumberOfLevels(), edges, unbuildIsWarning,
    //                                             flippedOperation, flippedLookup, havePermissions, haveRestrictions)),
    //         myType("arcFlagRouter"),
    //         myQueryVisits(0),
    //         myNumQueries(0),
    //         myQueryStartTime(0),
    //         myQueryTimeSum(0),
    // #ifdef AFRO_DEBUG_LEVEL_2
    //         myFlagContextStartTime(0),
    //         myFlagContextTimeSum(0),
    // #endif
    //         myLastSettledEdgeCell(nullptr),
    //         myTargetEdgeCellLevel0(nullptr) {
    //         for (const E* const edge : edges) {
    //             this->myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(edge));
    //             myMaxSpeed = MAX2(myMaxSpeed, edge->getSpeedLimit() * MAX2(1.0, edge->getLengthGeometryFactor()));
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &i32, partition: Ptr<KDTreePartition<E, N, V>>, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, flippedOperation: SUMOAbstractRouter_Operation, weightPeriod: SUMOTime, lookup: Rc<AStarRouter_LookupTable>, flippedLookup: Rc<AFBuild_FlippedLookupTable>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port AFRouter::AFRouter<E, N, V, M>")
    }

    /// C++: `void AFRouter::AFRouter<E, N, V, M>(const int &, const int &, const KDTreePartition<E, N, V> *, bool, typename SUMOAbstractRouter<E, V>::Operation, typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation, SUMOTime, const std::shared_ptr<const LookupTable>, const std::shared_ptr<const FlippedLookupTable>, const bool, const bool)`
    /// "Normal" cloning constructor for uninitialized or time-dependent instances
    // AFRouter(const std::vector<typename SUMOAbstractRouter<E, V>::EdgeInfo>& edgeInfos,
    //              const std::vector<E*>& edges,
    //              const KDTreePartition<E, N, V>* partition,
    //              bool unbuildIsWarning,
    //              typename SUMOAbstractRouter<E, V>::Operation operation,
    //              typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation flippedOperation,
    //              SUMOTime weightPeriod, const std::shared_ptr<const LookupTable> lookup = nullptr,
    //              const std::shared_ptr<const FlippedLookupTable> flippedLookup = nullptr,
    //              const bool havePermissions = false, const bool haveRestrictions = false) :
    //         SUMOAbstractRouter<E, V>("arcFlagRouter", unbuildIsWarning, operation, nullptr, havePermissions, haveRestrictions),
    //         myFlagInfos(nullptr),
    //         myPartition(partition),
    //         myLookupTable(lookup),
    //         myMaxSpeed(NUMERICAL_EPS),
    //         myWeightPeriod(weightPeriod),
    //         myValidUntil(0),
    //         myBuilder(new AFBuilder<E, N, V, M>(myPartition->getNumberOfLevels(), edges, unbuildIsWarning,
    //                                             flippedOperation, flippedLookup, havePermissions, haveRestrictions)),
    //         myType("arcFlagRouter"),
    //         myQueryVisits(0),
    //         myNumQueries(0),
    //         myQueryStartTime(0),
    //         myQueryTimeSum(0),
    // #ifdef AFRO_DEBUG_LEVEL_2
    //         myFlagContextStartTime(0),
    //         myFlagContextTimeSum(0),
    // #endif
    //         myLastSettledEdgeCell(nullptr),
    //         myTargetEdgeCellLevel0(nullptr) {
    //         for (const auto& edgeInfo : edgeInfos) {
    //             this->myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(edgeInfo.edge));
    //             myMaxSpeed = MAX2(myMaxSpeed, edgeInfo.edge->getSpeedLimit() * edgeInfo.edge->getLengthGeometryFactor());
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new_2(edgeInfos: &i32, edges: &i32, partition: Ptr<KDTreePartition<E, N, V>>, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, flippedOperation: SUMOAbstractRouter_Operation, weightPeriod: SUMOTime, lookup: Rc<AStarRouter_LookupTable>, flippedLookup: Rc<AFBuild_FlippedLookupTable>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port AFRouter::AFRouter<E, N, V, M>")
    }

    /// C++: `void AFRouter::AFRouter<E, N, V, M>(const int &, const KDTreePartition<E, N, V> *, bool, typename SUMOAbstractRouter<E, V>::Operation, int *, const std::shared_ptr<const LookupTable>, const bool, const bool)`
    /// Special cloning constructor, only for time-independent instances which never rebuild arc infos
    // AFRouter(const std::vector<typename SUMOAbstractRouter<E, V>::EdgeInfo>& edgeInfos,
    //              const KDTreePartition<E, N, V>* partition,
    //              bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation operation,
    //              std::vector<FlagInfo*>* flagInfos,
    //              const std::shared_ptr<const LookupTable> lookup = nullptr,
    //              const bool havePermissions = false, const bool haveRestrictions = false) :
    //         SUMOAbstractRouter<E, V>("arcFlagRouterClone", unbuildIsWarning, operation, nullptr, havePermissions, haveRestrictions),
    //         myFlagInfos(flagInfos),
    //         myPartition(partition),
    //         myLookupTable(lookup),
    //         myMaxSpeed(NUMERICAL_EPS),
    //         myWeightPeriod(SUMOTime_MAX),
    //         myValidUntil(SUMOTime_MAX),
    //         myBuilder(nullptr),
    //         myType("arcFlagRouterClone"),
    //         myQueryVisits(0),
    //         myNumQueries(0),
    //         myQueryStartTime(0),
    //         myQueryTimeSum(0),
    // #ifdef AFRO_DEBUG_LEVEL_2
    //         myFlagContextStartTime(0),
    //         myFlagContextTimeSum(0),
    // #endif
    //         myLastSettledEdgeCell(nullptr),
    //         myTargetEdgeCellLevel0(nullptr) {
    //         for (const auto& edgeInfo : edgeInfos) {
    //             this->myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(edgeInfo.edge));
    //             myMaxSpeed = MAX2(myMaxSpeed, edgeInfo.edge->getSpeedLimit() * edgeInfo.edge->getLengthGeometryFactor());
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new_3(edgeInfos: &i32, partition: Ptr<KDTreePartition<E, N, V>>, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, flagInfos: Ptr<i32>, lookup: Rc<AStarRouter_LookupTable>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port AFRouter::AFRouter<E, N, V, M>")
    }

    /// C++: `void AFRouter::~AFRouter<E, N, V, M>()` [virtual]
    /// Destructor
    // virtual ~AFRouter() {
    //         delete myBuilder;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port AFRouter::~AFRouter<E, N, V, M>")
    }

    /// C++: `SUMOAbstractRouter<E, V> * AFRouter::clone()` [virtual]
    /// Cloning method
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         // I am either a clone myself, or I am already initialized and time-independent
    //         // (i.e., I have been created with a maximum weight period)
    //         if (myWeightPeriod == SUMOTime_MAX && myFlagInfos != nullptr) {
    //             // we only need the arc infos once:
    //             return new AFRouter(this->myEdgeInfos, myPartition, this->myErrorMsgHandler == MsgHandler::getWarningInstance(),
    //                                 this->myOperation, myFlagInfos, myLookupTable, this->myHavePermissions, this->myHaveRestrictions);
    //         }
    //         // I am not a clone: I am either uninitialized, or initialized but time-dependent:
    //         // create another such guy (also flagged as a non-clone)
    //         return new AFRouter(this->myEdgeInfos, myBuilder->getEdges(), myPartition,
    //                             this->myErrorMsgHandler == MsgHandler::getWarningInstance(),
    //                             this->myOperation, myBuilder->getArcFlagBuild()->getFlippedOperation(),
    //                             myWeightPeriod, myLookupTable, myBuilder->getArcFlagBuild()->getFlippedLookup(),
    //                             this->myHavePermissions, this->myHaveRestrictions);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        todo!("port AFRouter::clone")
    }

    /// C++: `int AFRouter::partitionLevel2SHARCLevel(int, int)` [static]
    /// Converts a partition level number to a SHARC level number
    // static int partitionLevel2SHARCLevel(int partitionLevel, int numberOfPartitionLevels) {
    //         // heads up: partition levels must start at zero, with zero being an illegal argument
    //         // (since it would corresponds to level L = V, which is not a valid SHARC level)
    //         if (partitionLevel <= 0) {
    //             throw std::invalid_argument("partitionLevel2SHARCLevel: given partition level is zero (0) or below. This does not correspond to a valid SHARC level. Partition levels valid for conversion to SHARC levels go from one to number of partition levels minus one.");
    //         }
    //         // heads up: partition levels must start at zero
    //         if (partitionLevel > numberOfPartitionLevels - 1) {
    //             throw std::invalid_argument("partitionLevel2SHARCLevel: given partition level exceeds the number of partition levels minus one. Most likely you did not start the partition level numbering at zero (0), which is required here.");
    //         }
    //         return (numberOfPartitionLevels - 1) - partitionLevel;
    //     }
    #[allow(unused_variables)]
    pub fn partition_level2_sharc_level(partitionLevel: i32, numberOfPartitionLevels: i32) -> i32 {
        todo!("port AFRouter::partitionLevel2SHARCLevel")
    }

    /// C++: `int AFRouter::sHARCLevel2PartitionLevel(int, int)` [static]
    /// Converts a SHARC level number to a partition level number
    // static int sHARCLevel2PartitionLevel(int sHARCLevel, int numberOfPartitionLevels) {
    //         int numberOfSHARCLevels = numberOfPartitionLevels - 1;
    //         if (sHARCLevel < 0) {
    //             throw std::invalid_argument("sHARCLevel2PartitionLevel: given SHARC level is negative.");
    //         }
    //         // heads up: SHARC levels must start at zero (0),
    //         // and end at number of partition levels minus two
    //         if (sHARCLevel > numberOfSHARCLevels - 1) {
    //             throw std::invalid_argument("sHARCLevel2PartitionLevel: given SHARC level exceeds the number of SHARC levels minus one. Most likely you did not start the SHARC level numbering at zero (0), which is required here.");
    //         }
    //         return numberOfSHARCLevels - sHARCLevel;
    //     }
    #[allow(unused_variables)]
    pub fn s_harc_level2_partition_level(sHARCLevel: i32, numberOfPartitionLevels: i32) -> i32 {
        todo!("port AFRouter::sHARCLevel2PartitionLevel")
    }

    /// C++: `bool AFRouter::flag(const FlagInfo *, const std::tuple<int, int, bool>)` [static]
    /// Returns the arc flag of the edge in flagInfo wrt flagContext
    // static bool flag(const FlagInfo* flagInfo, const std::tuple<int, int, bool> flagContext) {
    //         assert(flagInfo);
    //         return flagInfo->arcFlags.empty() ? true : /* play it safe */
    //                (flagInfo->arcFlags)[std::get<0>(flagContext) /* assumed to be the SHARC level */ * 2
    //                                                              + std::get<1>(flagContext) /* assumed to be the cell index */];
    //
    //     }
    #[allow(unused_variables)]
    pub fn flag(flagInfo: Ptr<AFInfo_FlagInfo>, flagContext: Unmapped /* std::tuple<int, int, bool> */) -> bool {
        todo!("port AFRouter::flag")
    }

    /// C++: `int & AFRouter::flags(const E *)`
    /// Returns the arc flags of the passed edge
    // template<class E, class N, class V, class M>
    // std::vector<bool>& AFRouter<E, N, V, M>::flags(const E* edge) {
    //     assert(edge);
    //     if (!myFlagInfos) {
    //         throw std::runtime_error("flag infos not initialized, call compute() at least once before calling flags().");
    //     }
    //     return ((*myFlagInfos)[edge->getNumericalID()])->arcFlags;
    // }
    #[allow(unused_variables)]
    pub fn flags(&mut self, edge: Ptr<E>) -> &mut i32 {
        todo!("port AFRouter::flags")
    }

    /// C++: `void AFRouter::reset(const V *const)` [virtual]
    /// Trigger arc flags rebuild
    // virtual void reset(const V* const vehicle) {
    //         if (myValidUntil == 0) {
    //             myValidUntil = myWeightPeriod;
    //         }
    //         assert(myBuilder);
    // #ifdef AFRO_DEBUG_LEVEL_0
    //         long long int firstCallStart = 0;
    //         long long int firstCallTime = 0;
    //         firstCallStart = SysUtils::getCurrentMillis();
    //         std::cout << "Calling arc flag router for the first time during current weight period (arc flags build). This might take a while... " << std::endl;
    // #endif
    //         myFlagInfos = &(myBuilder->build(myValidUntil - myWeightPeriod, vehicle));
    // #ifdef AFRO_DEBUG_LEVEL_0
    //         firstCallTime = (SysUtils::getCurrentMillis() - firstCallStart);
    //         std::cout << "Time spent for arc flags build: " << elapsedMs2string(firstCallTime) << std::endl;
    // #endif
    //     }
    #[allow(unused_variables)]
    pub fn reset(&mut self, vehicle: Ptr<V>) {
        todo!("port AFRouter::reset")
    }

    /// C++: `void AFRouter::init(const int, const SUMOTime)`
    /// Initialize the arc flag router param[in] edgeID The edge id(entifier) param[in] msTime The start time of the routes in milliseconds
    // template<class E, class N, class V, class M>
    // void AFRouter<E, N, V, M>::init(const int edgeID, const SUMOTime msTime) {
    //     // all EdgeInfos touched in the previous query are either in myFrontierList or myFound: clean those up
    //     myTargetEdgeCellLevel0 = nullptr;
    //     for (auto& edgeInfo : this->myFrontierList) {
    //         edgeInfo->reset();
    //     }
    //     this->myFrontierList.clear();
    //     for (auto& edgeInfo : this->myFound) {
    //         edgeInfo->reset();
    //     }
    //     this->myFound.clear();
    //     if (edgeID > -1) {
    //         // add begin node
    //         auto& fromInfo = this->myEdgeInfos[edgeID];
    //         fromInfo.heuristicEffort = 0.;
    //         fromInfo.effort = 0.;
    //         fromInfo.leaveTime = STEPS2TIME(msTime);
    //         fromInfo.prev = nullptr;
    //         this->myFrontierList.push_back(&fromInfo);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, edgeID: i32, msTime: SUMOTime) {
        todo!("port AFRouter::init")
    }

    /// C++: `std::tuple<int, int, bool> AFRouter::flagContext(const E *, const E *)`
    /// Returns the flag context for a route query from given settled edge to the target edge
    // template<class E, class N, class V, class M>
    // std::tuple<int, int, bool> AFRouter<E, N, V, M>::flagContext(const E* settledEdge, const E* targetEdge) {
    //     assert(settledEdge != nullptr && targetEdge != nullptr);
    //     int sHARCLevel = 0; // lowest level with smallest cells
    //     const Cell* settledEdgeCell = nullptr;
    //     const Cell* targetEdgeCell = nullptr;
    //     if (myLastSettledEdgeCell
    //             && myLastSettledEdgeCell->contains(settledEdge->getFromJunction())) {
    //         // exploit the partial locality of Dijkstra's algorithm: settled edge is still
    //         // in the same cell as the last one? Then we can simply return the
    //         // last flagContext tuple again.
    //         return myLastFlagContext;
    //     }
    //     int numberOfPartitionLevels = myPartition->getNumberOfLevels();
    //     if (numberOfPartitionLevels <= 4) { // small number of bottom cells -> go through them, no use of k-d tree
    //         int partitionLevel = sHARCLevel2PartitionLevel(sHARCLevel, myPartition->getNumberOfLevels());
    //         const std::vector<const Cell*>& levelCells = myPartition->getCellsAtLevel(partitionLevel);
    //         typename std::vector<const Cell*>::const_iterator first = levelCells.begin();
    //         typename std::vector<const Cell*>::const_iterator last = levelCells.end();
    //         typename std::vector<const Cell*>::const_iterator iter;
    //         // go through the cells of the level
    //         for (iter = first; iter != last; iter++) {
    //             // myPartition is assumed to partition a non-reversed (forward) graph
    //             if (!settledEdgeCell
    //                     && (*iter)->contains(settledEdge->getFromJunction())) {
    //                 settledEdgeCell = *iter;
    //             }
    //             if (!targetEdgeCell && myTargetEdgeCellLevel0) {
    //                 targetEdgeCell = myTargetEdgeCellLevel0;
    //             } else if (!targetEdgeCell
    //                        && (*iter)->contains(targetEdge->getFromJunction())) {
    //                 myTargetEdgeCellLevel0 = *iter;
    //                 targetEdgeCell = myTargetEdgeCellLevel0;
    //             }
    //             if (settledEdgeCell && targetEdgeCell) {
    //                 assert(myTargetEdgeCellLevel0);
    //                 break;
    //             }
    //         }
    //     } else { // larger number of bottom cells -> use a k-d tree
    //         settledEdgeCell = myPartition->searchNode(settledEdge->getFromJunction());
    //         if (!targetEdgeCell && myTargetEdgeCellLevel0) {
    //             // search only once per query
    //             targetEdgeCell = myTargetEdgeCellLevel0;
    //         } else if (!targetEdgeCell) {
    //             myTargetEdgeCellLevel0 = myPartition->searchNode(targetEdge->getFromJunction());
    //             targetEdgeCell = myTargetEdgeCellLevel0; // myTargetEdgeCellLevel0 is reset in init()
    //         }
    //     }
    //     assert(settledEdgeCell && targetEdgeCell); // we should find both edges on each level
    //     while (settledEdgeCell->getSupercell() != targetEdgeCell->getSupercell()) {
    //         settledEdgeCell = settledEdgeCell->getSupercell();
    //         targetEdgeCell = targetEdgeCell->getSupercell();
    //         sHARCLevel++;
    //     }
    //     myLastSettledEdgeCell = settledEdgeCell;
    //     std::tuple<int, int, bool> flagContext = std::make_tuple(sHARCLevel, targetEdgeCell->isLeftOrLowerCell() ? 0 : 1,
    //             settledEdgeCell == targetEdgeCell);
    //     myLastFlagContext = flagContext;
    //     return flagContext;
    // }
    #[allow(unused_variables)]
    pub fn flag_context(&mut self, settledEdge: Ptr<E>, targetEdge: Ptr<E>) -> Unmapped /* std::tuple<int, int, bool> */ {
        todo!("port AFRouter::flagContext")
    }

    /// C++: `std::tuple<int, int, bool> AFRouter::flagContextNaive(const E *, const E *)`
    /// Kept for runtime comparisons
    // template<class E, class N, class V, class M>
    // std::tuple<int, int, bool> AFRouter<E, N, V, M>::flagContextNaive(const E* settledEdge, const E* targetEdge) {
    //     assert(settledEdge != nullptr && targetEdge != nullptr);
    //     int sHARCLevel;
    //     for (sHARCLevel = 0; sHARCLevel < myPartition->getNumberOfLevels() - 1; sHARCLevel++) {
    //         int partitionLevel = sHARCLevel2PartitionLevel(sHARCLevel, myPartition->getNumberOfLevels());
    //         const std::vector<const Cell*>& levelCells = myPartition->getCellsAtLevel(partitionLevel);
    //         typename std::vector<const Cell*>::const_iterator first = levelCells.begin();
    //         typename std::vector<const Cell*>::const_iterator last = levelCells.end();
    //         typename std::vector<const Cell*>::const_iterator iter;
    //         const Cell* settledEdgeCell = nullptr;
    //         const Cell* targetEdgeCell = nullptr;
    //         // go through the cells of the level
    //         for (iter = first; iter != last; iter++) {
    //             // myPartition is assumed to partition a non-reversed (forward) graph
    //             if (!settledEdgeCell && (*iter)->contains(settledEdge->getFromJunction())) {
    //                 settledEdgeCell = *iter;
    //             }
    //             if (!targetEdgeCell && (*iter)->contains(targetEdge->getFromJunction())) {
    //                 targetEdgeCell = *iter;
    //             }
    //             if (settledEdgeCell && targetEdgeCell) {
    //                 break;
    //             }
    //         }
    //         assert(settledEdgeCell && targetEdgeCell); // we should find both edges on each level
    //         if (settledEdgeCell->getSupercell() == targetEdgeCell->getSupercell()) {
    //             return std::make_tuple(sHARCLevel, targetEdgeCell->isLeftOrLowerCell() ? 0 : 1,
    //                                    settledEdgeCell == targetEdgeCell);
    //         }
    //     }
    //     // we should never arrive here
    //     throw std::runtime_error("flagContext: relevant level could not be determined.");
    // }
    #[allow(unused_variables)]
    pub fn flag_context_naive(&mut self, settledEdge: Ptr<E>, targetEdge: Ptr<E>) -> Unmapped /* std::tuple<int, int, bool> */ {
        todo!("port AFRouter::flagContextNaive")
    }

    /// C++: `bool AFRouter::compute(const E *, const E *, const V *const, SUMOTime, int &, bool)`
    /// Builds the route between the given edges using the minimum travel time param[in] from The from-/start/source/head edge param[in] to The to-/end/target/tail edge param[in] vehicle The vehicle param[in] msTime The start time of the routes in milliseconds param[out] into The vector of edges, into which the solution route is written
    // bool compute(const E* from, const E* to, const V* const vehicle,
    //                  SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         assert(from != nullptr && to != nullptr);
    //         // check whether from and to can be used
    //         if (this->isProhibited(from, vehicle, STEPS2TIME(msTime))) {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + from->getID() + "'.");
    //             }
    //             return false;
    //         }
    //         // technically, a temporary permission might be lifted by the time of arrival
    //         if (this->isProhibited(to, vehicle, STEPS2TIME(msTime))) {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on destination edge '" + to->getID() + "'.");
    //             }
    //             return false;
    //         }
    //
    //         if (msTime >= myValidUntil) {
    //             assert(myBuilder != nullptr); // only time independent clones do not have a builder
    //             while (msTime >= myValidUntil) {
    //                 myValidUntil += myWeightPeriod;
    //             }
    //             reset(vehicle);
    //         }
    //         // rewind routing start time to building time (this can only be a gross approximation
    //         // of time-dependent routing)
    //         msTime = myValidUntil - myWeightPeriod;
    //
    //         double length = 0.; // dummy for the via edge cost update
    //         this->startQuery();
    //         const SUMOVehicleClass vClass = vehicle == 0 ? SVC_IGNORING : vehicle->getVClass();
    //         this->init(from->getNumericalID(), msTime);
    //         this->myAmClean = false;
    //         // loop
    //         int num_visited = 0;
    // #ifdef AFRO_DEBUG_LEVEL_1
    //         int numberOfFollowers = 0;
    //         int numberOfAvoidedFollowers = 0;
    //         int numberOfEmptyFlagVectors = 0;
    // #endif
    //         const bool mayRevisit = myLookupTable != nullptr && !myLookupTable->consistent();
    //         const double speed = vehicle == nullptr ? myMaxSpeed : MIN2(vehicle->getMaxSpeed(), myMaxSpeed * vehicle->getChosenSpeedFactor());
    //
    //         while (!this->myFrontierList.empty()) {
    //             num_visited += 1;
    //             // use the edge with the minimal length
    //             auto* const minimumInfo = this->myFrontierList.front();
    //             const E* const minEdge = minimumInfo->edge;
    //             // check whether the destination edge was already reached
    //             if (minEdge == to) {
    //                 this->buildPathFrom(minimumInfo, into);
    //                 this->endQuery(num_visited);
    // #ifdef AFRO_DEBUG_LEVEL_1
    //                 std::cout << "Found to, to->getID(): " << to->getID() << std::endl;
    //                 std::cout << static_cast<double>(numberOfFollowers - numberOfAvoidedFollowers) / static_cast<double>(num_visited)
    //                           << " followers considered (out of " << static_cast<double>(numberOfFollowers) / static_cast<double>(num_visited) << ") on average." << std::endl;
    //                 std::cout << static_cast<double>(numberOfFollowers - numberOfAvoidedFollowers)
    //                           << " followers considered (out of " << static_cast<double>(numberOfFollowers) << ")." << std::endl;
    //                 std::cout << numberOfEmptyFlagVectors << " out of " << numberOfFollowers << " flag vectors of followers were unassigned (i.e., empty)." << std::endl;
    //                 std::cout << "num_visited: " << num_visited << std::endl;
    // #endif
    //                 return true;
    //             }
    //             std::pop_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //             this->myFrontierList.pop_back();
    //             this->myFound.push_back(minimumInfo);
    //             minimumInfo->visited = true;
    //
    //             const double effortDelta = this->getEffort(minEdge, vehicle, minimumInfo->leaveTime);
    //             const double leaveTime = minimumInfo->leaveTime + this->getTravelTime(minEdge, vehicle, minimumInfo->leaveTime, effortDelta);
    //
    //             // admissible A* heuristic: straight line distance at maximum speed
    //             // this is calculated from the end of minEdge so it possibly includes via efforts to the followers
    //             double heuristic_remaining = 0.;
    //             double heuristicEffort = minimumInfo->effort + effortDelta + heuristic_remaining;
    //             // check all ways from the edge with the minimal length
    //             for (const std::pair<const E*, const E*>& follower : minEdge->getViaSuccessors(vClass)) {
    //                 auto& followerInfo = this->myEdgeInfos[follower.first->getNumericalID()];
    //                 const FlagInfo* followerFlagInfo = (*myFlagInfos)[follower.first->getNumericalID()];
    //                 // check whether it can be used
    //                 if (this->isProhibited(follower.first, vehicle, leaveTime)) {
    //                     continue;
    //                 }
    // #ifdef AFRO_DEBUG_LEVEL_1
    //                 numberOfFollowers++;
    //                 if (followerFlagInfo->arcFlags.empty()) {
    //                     numberOfEmptyFlagVectors++;
    //                 }
    // #endif
    // #ifdef AFRO_DEBUG_LEVEL_2
    //                 myFlagContextStartTime = SysUtils::getCurrentMillis();
    // #endif
    //                 std::tuple<int, int, bool> flagContext = this->flagContext(follower.first, to);
    //                 //std::tuple<int, int, bool> flagContext = this->flagContextNaive(follower.first, to);
    // #ifdef AFRO_DEBUG_LEVEL_2
    //                 myFlagContextTimeSum += (SysUtils::getCurrentMillis() - myFlagContextStartTime);
    // #endif
    //                 if (!flag(followerFlagInfo, flagContext)) {
    // #ifdef AFRO_DEBUG_LEVEL_1
    //                     numberOfAvoidedFollowers++;
    // #endif
    //                     continue;
    //                 }
    //
    //                 // admissible A* heuristic: straight line distance at maximum speed
    //                 // this is calculated from the end of minEdge so it possibly includes via efforts to the followers
    //                 if (heuristic_remaining == 0 && std::get<0>(flagContext) == 0 && std::get<2>(flagContext)) {
    //                     // arrived at the target cell at level 0? use heuristic
    //                     heuristic_remaining =
    //                         (myLookupTable == nullptr ? minEdge->getDistanceTo(to) / speed :
    //                          myLookupTable->lowerBound(minEdge, to, speed, vehicle->getChosenSpeedFactor(),
    //                                                    minEdge->getMinimumTravelTime(nullptr), to->getMinimumTravelTime(nullptr)));
    //                     if (heuristic_remaining == UNREACHABLE) {
    //                         break; // -> skip remaining followers, continue with next min heap element
    //                     }
    //                     heuristicEffort += heuristic_remaining;
    //                 }
    //
    //                 double effort = minimumInfo->effort + effortDelta;
    //                 double time = leaveTime;
    //                 this->updateViaEdgeCost(follower.second, vehicle, time, effort, length);
    //                 const double oldEffort = followerInfo.effort;
    //                 if ((!followerInfo.visited || mayRevisit) && effort < oldEffort) {
    //                     followerInfo.effort = effort;
    //                     // if we use the effort including the via effort below we would count the via twice as shown by the ticket676 test
    //                     // but we should never get below the real effort, see #12463
    //                     followerInfo.heuristicEffort = MAX2(MIN2(heuristicEffort, followerInfo.heuristicEffort), effort);
    //                     followerInfo.leaveTime = time;
    //                     followerInfo.prev = minimumInfo;
    //                     if (oldEffort == std::numeric_limits<double>::max()) {
    //                         this->myFrontierList.push_back(&followerInfo);
    //                         std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //                     } else {
    //                         auto fi = std::find(this->myFrontierList.begin(), this->myFrontierList.end(), &followerInfo);
    //                         if (fi == this->myFrontierList.end()) {
    //                             assert(mayRevisit);
    //                             this->myFrontierList.push_back(&followerInfo);
    //                             std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //                         } else {
    //                             std::push_heap(this->myFrontierList.begin(), fi + 1, myComparator);
    //                         }
    //                     }
    //                 }
    //             } // for followers
    //         }
    //         this->endQuery(num_visited);
    // #ifdef AFRO_DEBUG_LEVEL_1
    //         std::cout << "Queue ran empty, no solution." << std::endl;
    //         std::cout << static_cast<double>(numberOfFollowers - numberOfAvoidedFollowers) / static_cast<double>(num_visited)
    //                   << " followers considered (out of " << static_cast<double>(numberOfFollowers) / static_cast<double>(num_visited) << ") on average." << std::endl;
    //         std::cout << static_cast<double>(numberOfFollowers - numberOfAvoidedFollowers)
    //                   << " followers considered (out of " << static_cast<double>(numberOfFollowers) << ")." << std::endl;
    //         std::cout << numberOfEmptyFlagVectors << " out of " << numberOfFollowers << " flag vectors of followers were unassigned (i.e., empty)." << std::endl;
    //         std::cout << "num_visited: " << num_visited << std::endl;
    // #endif
    //         if (!silent) {
    //             this->myErrorMsgHandler->informf("No connection between edge '%' and edge '%' found.", from->getID(), to->getID());
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut i32, silent: bool) -> bool {
        todo!("port AFRouter::compute")
    }

    /// C++: `void AFRouter::startQuery()`
    /// Start timer for query time sum
    // template<class E, class N, class V, class M>
    // void AFRouter<E, N, V, M>::startQuery() {
    //     myNumQueries++;
    //     myQueryStartTime = SysUtils::getCurrentMillis();
    //     SUMOAbstractRouter<E, V>::startQuery();
    // }
    #[allow(unused_variables)]
    pub fn start_query(&mut self) {
        todo!("port AFRouter::startQuery")
    }

    /// C++: `void AFRouter::endQuery(int)`
    /// Stop timer for query time sum
    // template<class E, class N, class V, class M>
    // void AFRouter<E, N, V, M>::endQuery(int visits) {
    //     myQueryVisits += visits;
    //     myQueryTimeSum += (SysUtils::getCurrentMillis() - myQueryStartTime);
    //     SUMOAbstractRouter<E, V>::endQuery(visits);
    // }
    #[allow(unused_variables)]
    pub fn end_query(&mut self, visits: i32) {
        todo!("port AFRouter::endQuery")
    }

    /// C++: `void AFRouter::reportStatistics()`
    /// Report query time statistics
    // template<class E, class N, class V, class M>
    // void AFRouter<E, N, V, M>::reportStatistics() {
    //     if (myNumQueries > 0) {
    //         WRITE_MESSAGE(myType + " answered " + toString(myNumQueries) + " queries and explored " + toString((double)myQueryVisits / (double)myNumQueries) + " edges on average.");
    //         WRITE_MESSAGE(myType + " spent " + elapsedMs2string(myQueryTimeSum) + " answering queries (" + toString((double)myQueryTimeSum / (double)myNumQueries) + " ms on average).");
    // #ifdef AFRO_DEBUG_LEVEL_2
    //         WRITE_MESSAGE("flagContext spent " + elapsedMs2string(myFlagContextTimeSum) + " (" + toString((double)myFlagContextTimeSum / (double)myNumQueries) + " ms on average).");
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn report_statistics(&mut self) {
        todo!("port AFRouter::reportStatistics")
    }

    /// C++: `void AFRouter::resetStatistics()`
    /// Reset query time statistics
    // template<class E, class N, class V, class M>
    // void AFRouter<E, N, V, M>::resetStatistics() {
    //     myNumQueries = 0;
    //     myQueryVisits = 0;
    //     myQueryTimeSum = 0;
    //     myQueryStartTime = 0;
    // }
    #[allow(unused_variables)]
    pub fn reset_statistics(&mut self) {
        todo!("port AFRouter::resetStatistics")
    }

    /// C++: `void AFRouter::setBulkMode(const bool)` [virtual]
    /// Bulk mode is not supported
    // virtual void setBulkMode(const bool mode) {
    //         UNUSED_PARAMETER(mode);
    //         throw std::runtime_error("Bulk mode is not supported by the arc flag router.");
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        todo!("port AFRouter::setBulkMode")
    }

}

/// Class to compare (and so sort) nodes by their effort
/// C++ class `AFRouter::EdgeInfoComparator` (utils/router/AFRouter.h)
pub struct AFRouter_EdgeInfoComparator {
}

impl AFRouter_EdgeInfoComparator {
    /// C++: `bool EdgeInfoComparator::operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo *, const typename SUMOAbstractRouter<E, V>::EdgeInfo *) const` [const]
    /// Comparing method
    // bool operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo1, const typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo2) const {
    //             if (edgeInfo1->heuristicEffort == edgeInfo2->heuristicEffort) {
    //                 return edgeInfo1->edge->getNumericalID() > edgeInfo2->edge->getNumericalID();
    //             }
    //             return edgeInfo1->heuristicEffort > edgeInfo2->heuristicEffort;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, edgeInfo1: Ptr<SUMOAbstractRouter_EdgeInfo>, edgeInfo2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        todo!("port EdgeInfoComparator::operator()")
    }

}

