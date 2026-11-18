//! Port of utils/router/AFRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::af_build::AFBuild_FlippedOperation;
use crate::utils::router::router_edge::{RouterEdge, RouterNode, RouterVehicle};
use utils::common::msg_handler::{with_instance, write_message, MsgType};
use utils::common::to_string::to_string;
use utils::geom::NUMERICAL_EPS;

const UNREACHABLE: f64 = f64::MAX / 1000.0;

/// C++ MAX2 (std::max)
fn max2(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn current_millis() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

// std::push_heap over the whole slice (libstdc++ algorithm, so ties land where C++ puts them)
fn std_push_heap<T: Copy, F: FnMut(T, T) -> bool>(v: &mut [T], mut comp: F) {
    let last = v.len() as isize - 1;
    let value = v[last as usize];
    push_heap_at(v, last, 0, value, &mut comp);
}

fn push_heap_at<T: Copy, F: FnMut(T, T) -> bool>(v: &mut [T], mut hole: isize, top: isize, value: T, comp: &mut F) {
    let mut parent = (hole - 1) / 2;
    while hole > top && comp(v[parent as usize], value) {
        v[hole as usize] = v[parent as usize];
        hole = parent;
        parent = (hole - 1) / 2;
    }
    v[hole as usize] = value;
}

// std::pop_heap over the whole slice (libstdc++ algorithm)
fn std_pop_heap<T: Copy, F: FnMut(T, T) -> bool>(v: &mut [T], mut comp: F) {
    if v.len() > 1 {
        let last = v.len() - 1;
        let value = v[last];
        v[last] = v[0];
        adjust_heap(&mut v[..last], 0, last as isize, value, &mut comp);
    }
}

fn adjust_heap<T: Copy, F: FnMut(T, T) -> bool>(v: &mut [T], mut hole: isize, len: isize, value: T, comp: &mut F) {
    let top = hole;
    let mut second = hole;
    while second < (len - 1) / 2 {
        second = 2 * (second + 1);
        if comp(v[second as usize], v[(second - 1) as usize]) {
            second -= 1;
        }
        v[hole as usize] = v[second as usize];
        hole = second;
    }
    if (len & 1) == 0 && second == (len - 2) / 2 {
        second = 2 * (second + 1);
        v[hole as usize] = v[(second - 1) as usize];
        hole = second - 1;
    }
    push_heap_at(v, hole, top, value, comp);
}

/// C++ typedef `typename KDTreePartition<E, N, V>::Cell AFRouter::Cell`
pub type AFRouter_Cell = KDTreePartition_Cell;

/// C++ typedef `typename AFInfo<E>::FlagInfo AFRouter::FlagInfo`
pub type AFRouter_FlagInfo = AFInfo_FlagInfo;

/// C++ typedef `AbstractLookupTable<FlippedEdge<E, N, V>, V> AFRouter::FlippedLookupTable`
pub type AFRouter_FlippedLookupTable = Unmapped /* AbstractLookupTable<FlippedEdge<E, N, V>, V> */;

/// C++ typedef `AbstractLookupTable<E, V> AFRouter::LookupTable`
pub type AFRouter_LookupTable<E, V> = AbstractLookupTable<E, V>;

/// Computes the shortest path through a network with an arc flag routing algorithm (Hilger et al.) in its multi-level variant (also called "stripped SHARC" by Delling et al.)
/// C++ class `AFRouter` (utils/router/AFRouter.h)
#[repr(C)]
pub struct AFRouter<E, N, V, M> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    /// Edge infos containing the associated edge and its arc flags
    pub myFlagInfos: Ptr<Vec<Ptr<AFInfo_FlagInfo>>>,
    /// The partition
    pub myPartition: Ptr<KDTreePartition<E, N, V>>,
    /// The comparator for edge information
    pub myComparator: AFRouter_EdgeInfoComparator,
    /// The lookup table for travel time heuristics
    pub myLookupTable: Option<Rc<AFRouter_LookupTable<E, V>>>,
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
    pub myLastFlagContext: (i32, i32, bool),
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

/// The `AFRouter::compute` reached from the base router's compute slot (C++ virtual dispatch)
pub fn afr_compute<E, N, V, M>(base: &mut SUMOAbstractRouter<E, V>, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool
where E: RouterEdge<V, Node = N>, V: RouterVehicle, N: RouterNode<Edge = E>
{
    Ptr::from_mut(base).cast::<AFRouter<E, N, V, M>>().m().compute(from, to, vehicle, msTime, into, silent)
}

impl<E, N, V, M> AFRouter<E, N, V, M> where E: RouterEdge<V, Node = N>, V: RouterVehicle, N: RouterNode<Edge = E> {
    /// Installs this router's compute into the base, so a `SUMOAbstractRouter` pointer (C++ virtual call) reaches it.
    pub fn with_compute(mut self) -> Self {
        self.base.myCompute = Some(afr_compute::<E, N, V, M>);
        self.base.myClone = Some(|base| Ptr::from_mut(base).cast::<AFRouter<E, N, V, M>>().m().clone());
        self
    }
}

impl<E, N, V, M> AFRouter<E, N, V, M> where E: RouterEdge<V, Node = N>, V: RouterVehicle {
    /// C++: `typename SUMOAbstractRouter<E, V>::EdgeInfo * AFRouter::edgeInfo(const E *const)`
    /// Returns the edge information for the passed edge
    // typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo(const E* const edge) {
    //         return &(this->myEdgeInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn edge_info(&mut self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        Ptr::from_mut(&mut self.myEdgeInfos[edge.numerical_id() as usize])
    }

    /// C++: `const typename SUMOAbstractRouter<E, V>::EdgeInfo * AFRouter::edgeInfo(const E *const) const` [const]
    /// Returns the edge information for the passed edge
    // const typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo(const E* const edge) const {
    //         return &(this->myEdgeInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn edge_info_2(&self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        Ptr::new(&self.myEdgeInfos[edge.numerical_id() as usize])
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
    pub fn new(edges: &Vec<Ptr<E>>, partition: Ptr<KDTreePartition<E, N, V>>, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, flippedOperation: AFBuild_FlippedOperation<E, N, V>, weightPeriod: SUMOTime, lookup: Option<Rc<AFRouter_LookupTable<E, V>>>, flippedLookup: Rc<AFBuild_FlippedLookupTable>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut router = Self {
            base: SUMOAbstractRouter::new("arcFlagRouter", unbuildIsWarning, operation, None, havePermissions, haveRestrictions),
            myFlagInfos: Ptr::null(),
            myPartition: partition,
            myComparator: AFRouter_EdgeInfoComparator {},
            myLookupTable: lookup,
            myMaxSpeed: NUMERICAL_EPS,
            myWeightPeriod: weightPeriod,
            myValidUntil: 0,
            myBuilder: Ptr::from_raw(Box::into_raw(AFBuilder::new(partition.get_number_of_levels(), edges, unbuildIsWarning, flippedOperation, flippedLookup, havePermissions, haveRestrictions, Ptr::null()))),
            myType: "arcFlagRouter".to_string(),
            myQueryVisits: 0,
            myNumQueries: 0,
            myQueryStartTime: 0,
            myQueryTimeSum: 0,
            myLastSettledEdgeCell: Ptr::null(),
            myLastFlagContext: (0, 0, false),
            myTargetEdgeCellLevel0: Ptr::null(),
            _p: std::marker::PhantomData,
        };
        for &edge in edges {
            router.myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(edge.cast()));
            router.myMaxSpeed = max2(router.myMaxSpeed, edge.speed_limit() * max2(1.0, edge.length_geometry_factor()));
        }
        router
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
    pub fn new_2(edgeInfos: &Vec<SUMOAbstractRouter_EdgeInfo>, edges: &Vec<Ptr<E>>, partition: Ptr<KDTreePartition<E, N, V>>, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, flippedOperation: AFBuild_FlippedOperation<E, N, V>, weightPeriod: SUMOTime, lookup: Option<Rc<AFRouter_LookupTable<E, V>>>, flippedLookup: Rc<AFBuild_FlippedLookupTable>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut router = Self {
            base: SUMOAbstractRouter::new("arcFlagRouter", unbuildIsWarning, operation, None, havePermissions, haveRestrictions),
            myFlagInfos: Ptr::null(),
            myPartition: partition,
            myComparator: AFRouter_EdgeInfoComparator {},
            myLookupTable: lookup,
            myMaxSpeed: NUMERICAL_EPS,
            myWeightPeriod: weightPeriod,
            myValidUntil: 0,
            myBuilder: Ptr::from_raw(Box::into_raw(AFBuilder::new(partition.get_number_of_levels(), edges, unbuildIsWarning, flippedOperation, flippedLookup, havePermissions, haveRestrictions, Ptr::null()))),
            myType: "arcFlagRouter".to_string(),
            myQueryVisits: 0,
            myNumQueries: 0,
            myQueryStartTime: 0,
            myQueryTimeSum: 0,
            myLastSettledEdgeCell: Ptr::null(),
            myLastFlagContext: (0, 0, false),
            myTargetEdgeCellLevel0: Ptr::null(),
            _p: std::marker::PhantomData,
        };
        for edgeInfo in edgeInfos {
            router.myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(edgeInfo.edge));
            router.myMaxSpeed = max2(router.myMaxSpeed, edgeInfo.edge.cast::<E>().speed_limit() * edgeInfo.edge.cast::<E>().length_geometry_factor());
        }
        router
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
    pub fn new_3(edgeInfos: &Vec<SUMOAbstractRouter_EdgeInfo>, partition: Ptr<KDTreePartition<E, N, V>>, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, flagInfos: Ptr<Vec<Ptr<AFInfo_FlagInfo>>>, lookup: Option<Rc<AFRouter_LookupTable<E, V>>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut router = Self {
            base: SUMOAbstractRouter::new("arcFlagRouterClone", unbuildIsWarning, operation, None, havePermissions, haveRestrictions),
            myFlagInfos: flagInfos,
            myPartition: partition,
            myComparator: AFRouter_EdgeInfoComparator {},
            myLookupTable: lookup,
            myMaxSpeed: NUMERICAL_EPS,
            myWeightPeriod: SUMOTIME_MAX,
            myValidUntil: SUMOTIME_MAX,
            myBuilder: Ptr::null(),
            myType: "arcFlagRouterClone".to_string(),
            myQueryVisits: 0,
            myNumQueries: 0,
            myQueryStartTime: 0,
            myQueryTimeSum: 0,
            myLastSettledEdgeCell: Ptr::null(),
            myLastFlagContext: (0, 0, false),
            myTargetEdgeCellLevel0: Ptr::null(),
            _p: std::marker::PhantomData,
        };
        for edgeInfo in edgeInfos {
            router.myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(edgeInfo.edge));
            router.myMaxSpeed = max2(router.myMaxSpeed, edgeInfo.edge.cast::<E>().speed_limit() * edgeInfo.edge.cast::<E>().length_geometry_factor());
        }
        router
    }

    /// C++: `void AFRouter::~AFRouter<E, N, V, M>()` [virtual]
    /// Destructor
    // virtual ~AFRouter() {
    //         delete myBuilder;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myBuilder.is_null() {
            let mut builder = unsafe { Box::from_raw(self.myBuilder.as_raw()) };
            builder.drop_();
        }
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
        let unbuildIsWarning = self.myErrorMsgHandler == with_instance(MsgType::Warning, |h| Ptr::from_mut(h));
        // I am either a clone myself, or I am already initialized and time-independent
        // (i.e., I have been created with a maximum weight period)
        if self.myWeightPeriod == SUMOTIME_MAX && !self.myFlagInfos.is_null() {
            // we only need the arc infos once:
            let router = Self::new_3(&self.myEdgeInfos, self.myPartition, unbuildIsWarning, self.myOperation, self.myFlagInfos, self.myLookupTable.clone(), self.myHavePermissions, self.myHaveRestrictions);
            return Ptr::from_raw(Box::into_raw(Box::new(router))).cast();
        }
        // I am not a clone: I am either uninitialized, or initialized but time-dependent:
        // create another such guy (also flagged as a non-clone)
        let router = Self::new_2(&self.myEdgeInfos, self.myBuilder.m().get_edges(), self.myPartition, unbuildIsWarning, self.myOperation, self.myBuilder.m().get_arc_flag_build().m().get_flipped_operation(), self.myWeightPeriod, self.myLookupTable.clone(), self.myBuilder.m().get_arc_flag_build().m().get_flipped_lookup(), self.myHavePermissions, self.myHaveRestrictions);
        Ptr::from_raw(Box::into_raw(Box::new(router))).cast()
    }

}

impl<E, N, V, M> AFRouter<E, N, V, M> {
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
        // heads up: partition levels must start at zero, with zero being an illegal argument
        // (since it would corresponds to level L = V, which is not a valid SHARC level)
        if partitionLevel <= 0 {
            panic!("{}", ProcessError::new("partitionLevel2SHARCLevel: given partition level is zero (0) or below. This does not correspond to a valid SHARC level. Partition levels valid for conversion to SHARC levels go from one to number of partition levels minus one."));
        }
        // heads up: partition levels must start at zero
        if partitionLevel > numberOfPartitionLevels - 1 {
            panic!("{}", ProcessError::new("partitionLevel2SHARCLevel: given partition level exceeds the number of partition levels minus one. Most likely you did not start the partition level numbering at zero (0), which is required here."));
        }
        (numberOfPartitionLevels - 1) - partitionLevel
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
        let numberOfSHARCLevels = numberOfPartitionLevels - 1;
        if sHARCLevel < 0 {
            panic!("{}", ProcessError::new("sHARCLevel2PartitionLevel: given SHARC level is negative."));
        }
        // heads up: SHARC levels must start at zero (0),
        // and end at number of partition levels minus two
        if sHARCLevel > numberOfSHARCLevels - 1 {
            panic!("{}", ProcessError::new("sHARCLevel2PartitionLevel: given SHARC level exceeds the number of SHARC levels minus one. Most likely you did not start the SHARC level numbering at zero (0), which is required here."));
        }
        numberOfSHARCLevels - sHARCLevel
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
    pub fn flag(flagInfo: Ptr<AFInfo_FlagInfo>, flagContext: (i32, i32, bool)) -> bool {
        assert!(!flagInfo.is_null());
        if flagInfo.arcFlags.is_empty() {
            true /* play it safe */
        } else {
            flagInfo.arcFlags[(flagContext.0 * 2 + flagContext.1) as usize]
        }
    }

}

impl<E, N, V, M> AFRouter<E, N, V, M> where E: RouterEdge<V, Node = N>, V: RouterVehicle, N: RouterNode {
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
    pub fn flags(&mut self, edge: Ptr<E>) -> &mut Vec<bool> {
        assert!(!edge.is_null());
        if self.myFlagInfos.is_null() {
            panic!("{}", ProcessError::new("flag infos not initialized, call compute() at least once before calling flags()."));
        }
        let flagInfo = self.myFlagInfos.m()[edge.numerical_id() as usize];
        &mut flagInfo.get_mut().unwrap().arcFlags
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
    pub fn reset(&mut self, vehicle: Ptr<V>)
    where N: RouterNode<Edge = E>
    {
        if self.myValidUntil == 0 {
            self.myValidUntil = self.myWeightPeriod;
        }
        assert!(!self.myBuilder.is_null());
        let flagInfos = self.myBuilder.m().build(self.myValidUntil - self.myWeightPeriod, vehicle);
        self.myFlagInfos = Ptr::from_mut(flagInfos);
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
        // all EdgeInfos touched in the previous query are either in myFrontierList or myFound: clean those up
        self.myTargetEdgeCellLevel0 = Ptr::null();
        for edgeInfo in self.myFrontierList.iter() {
            edgeInfo.m().reset();
        }
        self.myFrontierList.clear();
        for edgeInfo in self.myFound.iter() {
            edgeInfo.m().reset();
        }
        self.myFound.clear();
        if edgeID > -1 {
            // add begin node
            let fromInfo = Ptr::from_mut(&mut self.myEdgeInfos[edgeID as usize]);
            fromInfo.m().heuristicEffort = 0.;
            fromInfo.m().effort = 0.;
            fromInfo.m().leaveTime = steps2time(msTime);
            fromInfo.m().prev = Ptr::null();
            self.myFrontierList.push(fromInfo);
        }
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
    pub fn flag_context(&mut self, settledEdge: Ptr<E>, targetEdge: Ptr<E>) -> (i32, i32, bool)
    where N: RouterNode
    {
        assert!(!settledEdge.is_null() && !targetEdge.is_null());
        let mut sHARCLevel: i32 = 0; // lowest level with smallest cells
        let mut settledEdgeCell: Ptr<KDTreePartition_Cell> = Ptr::null();
        let mut targetEdgeCell: Ptr<KDTreePartition_Cell> = Ptr::null();
        if !self.myLastSettledEdgeCell.is_null() && self.myLastSettledEdgeCell.contains(Self::edge_from_junction(settledEdge)) {
            // exploit the partial locality of Dijkstra's algorithm: settled edge is still
            // in the same cell as the last one? Then we can simply return the
            // last flagContext tuple again.
            return self.myLastFlagContext;
        }
        let numberOfPartitionLevels = self.myPartition.get_number_of_levels();
        if numberOfPartitionLevels <= 4 { // small number of bottom cells -> go through them, no use of k-d tree
            let partitionLevel = Self::s_harc_level2_partition_level(sHARCLevel, self.myPartition.get_number_of_levels());
            // go through the cells of the level
            for iter in self.cells_at_level(partitionLevel) {
                // myPartition is assumed to partition a non-reversed (forward) graph
                if settledEdgeCell.is_null() && iter.contains(Self::edge_from_junction(settledEdge)) {
                    settledEdgeCell = iter;
                }
                if targetEdgeCell.is_null() && !self.myTargetEdgeCellLevel0.is_null() {
                    targetEdgeCell = self.myTargetEdgeCellLevel0;
                } else if targetEdgeCell.is_null() && iter.contains(Self::edge_from_junction(targetEdge)) {
                    self.myTargetEdgeCellLevel0 = iter;
                    targetEdgeCell = self.myTargetEdgeCellLevel0;
                }
                if !settledEdgeCell.is_null() && !targetEdgeCell.is_null() {
                    assert!(!self.myTargetEdgeCellLevel0.is_null());
                    break;
                }
            }
        } else { // larger number of bottom cells -> use a k-d tree
            settledEdgeCell = self.myPartition.search_node(Self::edge_from_junction(settledEdge));
            if targetEdgeCell.is_null() && !self.myTargetEdgeCellLevel0.is_null() {
                // search only once per query
                targetEdgeCell = self.myTargetEdgeCellLevel0;
            } else if targetEdgeCell.is_null() {
                self.myTargetEdgeCellLevel0 = self.myPartition.search_node(Self::edge_from_junction(targetEdge));
                targetEdgeCell = self.myTargetEdgeCellLevel0; // myTargetEdgeCellLevel0 is reset in init()
            }
        }
        assert!(!settledEdgeCell.is_null() && !targetEdgeCell.is_null()); // we should find both edges on each level
        while settledEdgeCell.get_supercell() != targetEdgeCell.get_supercell() {
            settledEdgeCell = settledEdgeCell.get_supercell();
            targetEdgeCell = targetEdgeCell.get_supercell();
            sHARCLevel += 1;
        }
        self.myLastSettledEdgeCell = settledEdgeCell;
        let flagContext = (sHARCLevel, if targetEdgeCell.is_left_or_lower_cell() { 0 } else { 1 }, settledEdgeCell == targetEdgeCell);
        self.myLastFlagContext = flagContext;
        flagContext
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
    pub fn flag_context_naive(&mut self, settledEdge: Ptr<E>, targetEdge: Ptr<E>) -> (i32, i32, bool)
    where N: RouterNode
    {
        assert!(!settledEdge.is_null() && !targetEdge.is_null());
        for sHARCLevel in 0..self.myPartition.get_number_of_levels() - 1 {
            let partitionLevel = Self::s_harc_level2_partition_level(sHARCLevel, self.myPartition.get_number_of_levels());
            let levelCells = self.cells_at_level(partitionLevel);
            let mut settledEdgeCell: Ptr<KDTreePartition_Cell> = Ptr::null();
            let mut targetEdgeCell: Ptr<KDTreePartition_Cell> = Ptr::null();
            // go through the cells of the level
            for iter in levelCells {
                // myPartition is assumed to partition a non-reversed (forward) graph
                if settledEdgeCell.is_null() && iter.contains(Self::edge_from_junction(settledEdge)) {
                    settledEdgeCell = iter;
                }
                if targetEdgeCell.is_null() && iter.contains(Self::edge_from_junction(targetEdge)) {
                    targetEdgeCell = iter;
                }
                if !settledEdgeCell.is_null() && !targetEdgeCell.is_null() {
                    break;
                }
            }
            assert!(!settledEdgeCell.is_null() && !targetEdgeCell.is_null()); // we should find both edges on each level
            if settledEdgeCell.get_supercell() == targetEdgeCell.get_supercell() {
                return (sHARCLevel, if targetEdgeCell.is_left_or_lower_cell() { 0 } else { 1 }, settledEdgeCell == targetEdgeCell);
            }
        }
        // we should never arrive here
        panic!("{}", ProcessError::new("flagContext: relevant level could not be determined."));
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
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool
    where N: RouterNode<Edge = E>
    {
        // check whether from and to can be used
        if self.is_prohibited(from, vehicle, steps2time(msTime)) {
            if !silent {
                self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on source edge '{}'.", Self::vehicle_id_secure(vehicle), from.id()), true);
            }
            return false;
        }
        // technically, a temporary permission might be lifted by the time of arrival
        if self.is_prohibited(to, vehicle, steps2time(msTime)) {
            if !silent {
                self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on destination edge '{}'.", Self::vehicle_id_secure(vehicle), to.id()), true);
            }
            return false;
        }

        if msTime >= self.myValidUntil {
            while msTime >= self.myValidUntil {
                self.myValidUntil += self.myWeightPeriod;
            }
            self.reset(vehicle);
        }
        // rewind routing start time to building time (this can only be a gross approximation
        // of time-dependent routing)
        let msTime = self.myValidUntil - self.myWeightPeriod;

        let mut length = 0.; // dummy for the via edge cost update
        self.start_query();
        let vClass = if vehicle.is_null() { SVC_IGNORING } else { vehicle.v_class() };
        self.init(from.numerical_id(), msTime);
        self.myAmClean = false;
        // loop
        let mut num_visited: i32 = 0;
        let mayRevisit = match &self.myLookupTable {
            Some(t) => !t.consistent(),
            None => false,
        };
        let speed = if vehicle.is_null() { self.myMaxSpeed } else { min2(Self::vehicle_max_speed(vehicle), self.myMaxSpeed * Self::vehicle_chosen_speed_factor(vehicle)) };

        let comp = self.myComparator;
        while !self.myFrontierList.is_empty() {
            num_visited += 1;
            // use the edge with the minimal length
            let minimumInfo = self.myFrontierList[0];
            let minEdge: Ptr<E> = minimumInfo.edge.cast();
            // check whether the destination edge was already reached
            if minEdge == to {
                self.build_path_from(minimumInfo, into);
                self.end_query(num_visited);
                return true;
            }
            std_pop_heap(&mut self.myFrontierList, |a, b| comp.call_op::<E, V>(a, b));
            self.myFrontierList.pop();
            self.myFound.push(minimumInfo);
            minimumInfo.m().visited = true;

            let effortDelta = self.get_effort(minEdge, vehicle, minimumInfo.leaveTime);
            let leaveTime = minimumInfo.leaveTime + self.get_travel_time(minEdge, vehicle, minimumInfo.leaveTime, effortDelta);

            // admissible A* heuristic: straight line distance at maximum speed
            // this is calculated from the end of minEdge so it possibly includes via efforts to the followers
            let mut heuristic_remaining: f64 = 0.;
            let mut heuristicEffort = minimumInfo.effort + effortDelta + heuristic_remaining;
            // check all ways from the edge with the minimal length
            for &follower in minEdge.via_successors(vClass, false) {
                let followerInfo = Ptr::from_mut(&mut self.myEdgeInfos[follower.0.numerical_id() as usize]);
                let followerFlagInfo = self.myFlagInfos.m()[follower.0.numerical_id() as usize];
                // check whether it can be used
                if self.is_prohibited(follower.0, vehicle, leaveTime) {
                    continue;
                }
                let flagContext = self.flag_context(follower.0, to);
                if !Self::flag(followerFlagInfo, flagContext) {
                    continue;
                }

                // admissible A* heuristic: straight line distance at maximum speed
                // this is calculated from the end of minEdge so it possibly includes via efforts to the followers
                if heuristic_remaining == 0. && flagContext.0 == 0 && flagContext.2 {
                    // arrived at the target cell at level 0? use heuristic
                    heuristic_remaining = match &self.myLookupTable {
                        None => minEdge.distance_to(to, false) / speed,
                        Some(t) => t.lower_bound(minEdge, to, speed, Self::vehicle_chosen_speed_factor(vehicle), Self::edge_minimum_travel_time(minEdge), Self::edge_minimum_travel_time(to)),
                    };
                    if heuristic_remaining == UNREACHABLE {
                        break; // -> skip remaining followers, continue with next min heap element
                    }
                    heuristicEffort += heuristic_remaining;
                }

                let mut effort = minimumInfo.effort + effortDelta;
                let mut time = leaveTime;
                self.update_via_edge_cost(follower.1, vehicle, &mut time, &mut effort, &mut length);
                let oldEffort = followerInfo.effort;
                if (!followerInfo.visited || mayRevisit) && effort < oldEffort {
                    followerInfo.m().effort = effort;
                    // if we use the effort including the via effort below we would count the via twice as shown by the ticket676 test
                    // but we should never get below the real effort, see #12463
                    followerInfo.m().heuristicEffort = max2(min2(heuristicEffort, followerInfo.heuristicEffort), effort);
                    followerInfo.m().leaveTime = time;
                    followerInfo.m().prev = minimumInfo;
                    if oldEffort == f64::MAX {
                        self.myFrontierList.push(followerInfo);
                        std_push_heap(&mut self.myFrontierList, |a, b| comp.call_op::<E, V>(a, b));
                    } else {
                        match self.myFrontierList.iter().position(|p| *p == followerInfo) {
                            None => {
                                assert!(mayRevisit);
                                self.myFrontierList.push(followerInfo);
                                std_push_heap(&mut self.myFrontierList, |a, b| comp.call_op::<E, V>(a, b));
                            }
                            Some(idx) => std_push_heap(&mut self.myFrontierList[..=idx], |a, b| comp.call_op::<E, V>(a, b)),
                        }
                    }
                }
            } // for followers
        }
        self.end_query(num_visited);
        if !silent {
            self.myErrorMsgHandler.m().informf("No connection between edge '%' and edge '%' found.", &[&from.id(), &to.id()]);
        }
        false
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
        self.myNumQueries += 1;
        self.myQueryStartTime = current_millis();
        self.base.start_query();
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
        self.myQueryVisits += visits as i64;
        self.myQueryTimeSum += current_millis() - self.myQueryStartTime;
        self.base.end_query(visits);
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
        if self.myNumQueries > 0 {
            write_message(&(self.myType.clone() + " answered " + &self.myNumQueries.to_string() + " queries and explored " + &to_string(&((self.myQueryVisits as f64) / (self.myNumQueries as f64))) + " edges on average."));
            write_message(&(self.myType.clone() + " spent " + &utils::common::time::elapsed_ms2string(self.myQueryTimeSum) + " answering queries (" + &to_string(&((self.myQueryTimeSum as f64) / (self.myNumQueries as f64))) + " ms on average)."));
        }
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
        self.myNumQueries = 0;
        self.myQueryVisits = 0;
        self.myQueryTimeSum = 0;
        self.myQueryStartTime = 0;
    }

    /// C++: `void AFRouter::setBulkMode(const bool)` [virtual]
    /// Bulk mode is not supported
    // virtual void setBulkMode(const bool mode) {
    //         UNUSED_PARAMETER(mode);
    //         throw std::runtime_error("Bulk mode is not supported by the arc flag router.");
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        panic!("{}", ProcessError::new("Bulk mode is not supported by the arc flag router."));
    }

    fn edge_from_junction(edge: Ptr<E>) -> Ptr<N> {
        edge.from_junction()
    }

    fn edge_minimum_travel_time(edge: Ptr<E>) -> f64 {
        edge.minimum_travel_time(Ptr::null())
    }

    fn vehicle_max_speed(vehicle: Ptr<V>) -> f64 {
        vehicle.get_max_speed()
    }

    fn vehicle_chosen_speed_factor(vehicle: Ptr<V>) -> f64 {
        vehicle.get_chosen_speed_factor()
    }

    fn vehicle_id_secure(vehicle: Ptr<V>) -> String {
        vehicle.id()
    }

    /// C++ `myPartition->getCellsAtLevel(level)`: the cells of the given partition level.
    fn cells_at_level(&self, level: i32) -> Vec<Ptr<KDTreePartition_Cell>> {
        self.myPartition.get_cells_at_level(level).clone()
    }

}

/// Class to compare (and so sort) nodes by their effort
/// C++ class `AFRouter::EdgeInfoComparator` (utils/router/AFRouter.h)
#[derive(Clone, Copy)]
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
    pub fn call_op<E: RouterEdge<V>, V>(&self, edgeInfo1: Ptr<SUMOAbstractRouter_EdgeInfo>, edgeInfo2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        if edgeInfo1.heuristicEffort == edgeInfo2.heuristicEffort {
            return edgeInfo1.edge.cast::<E>().numerical_id() > edgeInfo2.edge.cast::<E>().numerical_id();
        }
        edgeInfo1.heuristicEffort > edgeInfo2.heuristicEffort
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // Values computed by hand from AFRouter.h:
    // sharc = (levels - 1) - partition, partition = (levels - 1) - sharc, flag index = sharc * 2 + (0 | 1)
    type Af = AFRouter<(), (), (), ()>;

    #[test]
    fn partition_to_sharc_levels() {
        assert_eq!(Af::partition_level2_sharc_level(1, 5), 3);
        assert_eq!(Af::partition_level2_sharc_level(4, 5), 0);
        assert_eq!(Af::partition_level2_sharc_level(1, 3), 1);
        assert_eq!(Af::partition_level2_sharc_level(2, 3), 0);
    }

    #[test]
    fn sharc_to_partition_levels() {
        assert_eq!(Af::s_harc_level2_partition_level(0, 5), 4);
        assert_eq!(Af::s_harc_level2_partition_level(3, 5), 1);
        assert_eq!(Af::s_harc_level2_partition_level(0, 3), 2);
        assert_eq!(Af::s_harc_level2_partition_level(1, 3), 1);
    }

    #[test]
    fn partition_and_sharc_levels_round_trip() {
        for levels in 2..=6 {
            for p in 1..levels {
                let s = Af::partition_level2_sharc_level(p, levels);
                assert_eq!(Af::s_harc_level2_partition_level(s, levels), p);
            }
        }
    }

    #[test]
    #[should_panic(expected = "given partition level is zero (0) or below")]
    fn partition_level_zero_is_rejected() {
        Af::partition_level2_sharc_level(0, 5);
    }

    #[test]
    #[should_panic(expected = "exceeds the number of partition levels minus one")]
    fn partition_level_above_top_is_rejected() {
        Af::partition_level2_sharc_level(5, 5);
    }

    #[test]
    #[should_panic(expected = "exceeds the number of SHARC levels minus one")]
    fn sharc_level_above_top_is_rejected() {
        Af::s_harc_level2_partition_level(4, 5);
    }

    #[test]
    fn flag_without_arc_flags_is_always_set() {
        let mut info = AFInfo_FlagInfo::new(Ptr::null());
        assert!(Af::flag(Ptr::from_mut(&mut info), (5, 1, false)));
    }

    #[test]
    fn flag_index_is_sharc_level_times_two_plus_cell() {
        let mut info = AFInfo_FlagInfo::new(Ptr::null());
        info.arcFlags = vec![false, true, false, true];
        assert!(!Af::flag(Ptr::from_mut(&mut info), (0, 0, false))); // index 0
        assert!(Af::flag(Ptr::from_mut(&mut info), (0, 1, false))); // index 1
        assert!(!Af::flag(Ptr::from_mut(&mut info), (1, 0, true))); // index 2
        assert!(Af::flag(Ptr::from_mut(&mut info), (1, 1, false))); // index 3
    }
}

