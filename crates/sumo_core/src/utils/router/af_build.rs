//! Port of utils/router/AFBuild.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterNode, RouterVehicle};
use crate::utils::router::sumo_abstract_router::{SUMOAbstractRouter, SUMOAbstractRouter_TypedOperation};

/// C++ `UNREACHABLE` (as used by AFRouter / AFBuild)
const UNREACHABLE: f64 = f64::MAX / 1000.0;

/// C++ `SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation`: the travel-time operation on the
/// backward graph (a plain function pointer, not the untyped logging placeholder).
pub type AFBuild_FlippedOperation<E, N, V> = SUMOAbstractRouter_TypedOperation<FlippedEdge<E, N, V>, V>;

/// C++ typedef `typename AFInfo<FlippedEdge<E, N, V>>::ArcInfo AFBuild::ArcInfo`
pub type AFBuild_ArcInfo = AFInfo_ArcInfo;

/// C++ typedef `typename KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>::Cell AFBuild::Cell`
pub type AFBuild_Cell = KDTreePartition_Cell;

/// C++ typedef `typename AFInfo<E>::FlagInfo AFBuild::FlagInfo`
pub type AFBuild_FlagInfo = AFInfo_FlagInfo;

/// C++ typedef `AbstractLookupTable<FlippedEdge<E, N, V>, V> AFBuild::FlippedLookupTable`
pub type AFBuild_FlippedLookupTable = Unmapped /* AbstractLookupTable<FlippedEdge<E, N, V>, V> */;

/// Builds the flags for (multi-level) arc flag routing (Hilger et al.) in its multi-level variant (also called "stripped SHARC" by Delling et al.)
/// C++ class `AFBuild` (utils/router/AFBuild.h)
pub struct AFBuild<E, N, V, M> {
    /// Maximum difference of two path lengths considered to be equal
    pub EPS: f64,
    /// The flipped edges
    pub myFlippedEdges: Vec<Ptr<FlippedEdge<E, N, V>>>,
    /// The partition for the backward graph with flipped edges
    pub myFlippedPartition: Ptr<KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>>,
    /// The number of levels
    pub myNumberOfLevels: i32,
    /// The handler for routing errors
    pub myErrorMsgHandler: Ptr<MsgHandler>,
    /// The object's operation to perform on a backward graph with flipped edges
    pub myFlippedOperation: AFBuild_FlippedOperation<E, N, V>,
    /// The lookup table for travel time heuristics
    pub myFlippedLookupTable: Rc<AFBuild_FlippedLookupTable>,
    /// The boolean flag indicating whether edge permissions need to be considered or not
    pub myHavePermissions: bool,
    /// The boolean flag indicating whether edge restrictions need to be considered or not
    pub myHaveRestrictions: bool,
    /// The list of explicitly prohibited edges
    pub myProhibited: Ptr<i32>,
    /// The node-to-edge router (for a backward graph with flipped edges)
    pub myNode2EdgeRouter: Ptr<Node2EdgeRouter<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V, M>>,
    pub myCentralizedSPTree: Ptr<AFCentralizedSPTree<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>>,
    /// The container of arc informations (for the centralized shortest path tree)
    pub myArcInfos: Ptr<Vec<Ptr<AFBuild_ArcInfo>>>,
    _p: std::marker::PhantomData<(E, N, V, M,)>,
}

fn delete_ptr<T>(p: Ptr<T>) {
    if !p.is_null() {
        unsafe { drop(Box::from_raw(p.as_raw())) };
    }
}

impl<E, N, V, M> AFBuild<E, N, V, M> {
    /// C++: `void AFBuild::AFBuild<E, N, V, M>(const int &, const KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V> *const, int, bool, typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation, const std::shared_ptr<const FlippedLookupTable>, const bool, const bool, const int *)`
    /// Constructor
    // AFBuild(
    //         const std::vector<FlippedEdge<E, N, V>*>& flippedEdges,
    //         const KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>* const flippedPartition,
    //         int numberOfLevels, bool unbuildIsWarning,
    //         typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation flippedOperation,
    //         const std::shared_ptr<const FlippedLookupTable> flippedLookup = nullptr,
    //         const bool havePermissions = false, const bool haveRestrictions = false,
    //         const std::map<const FlippedEdge<E, N, V>*, RouterProhibition>* toProhibit = nullptr) :
    //         myFlippedEdges(flippedEdges),
    //         myFlippedPartition(flippedPartition),
    //         myNumberOfLevels(numberOfLevels),
    //         myErrorMsgHandler(unbuildIsWarning ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //         myFlippedOperation(flippedOperation),
    //         myFlippedLookupTable(flippedLookup),
    //         myHavePermissions(havePermissions),
    //         myHaveRestrictions(haveRestrictions),
    //         myProhibited(toProhibit),
    //         myNode2EdgeRouter(new Node2EdgeRouter<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V, M>(myFlippedEdges,
    //                           unbuildIsWarning, myFlippedOperation, myFlippedLookupTable, myHavePermissions, myHaveRestrictions)) {
    //         myCentralizedSPTree = new AFCentralizedSPTree<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>(myFlippedEdges,
    //                 myArcInfos, unbuildIsWarning, myNode2EdgeRouter, myHavePermissions, myHaveRestrictions);
    //         if (toProhibit) {
    //             myNode2EdgeRouter->prohibit(*toProhibit);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(flippedEdges: &Vec<Ptr<FlippedEdge<E, N, V>>>,
 flippedPartition: Ptr<KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>>, numberOfLevels: i32, unbuildIsWarning: bool, flippedOperation: AFBuild_FlippedOperation<E, N, V>, flippedLookup: Rc<AFBuild_FlippedLookupTable>, havePermissions: bool, haveRestrictions: bool, toProhibit: Ptr<i32>) -> Self
    where E: RouterEdge<V, Node = N>, V: RouterVehicle
    {
        let msgInstance = if unbuildIsWarning {
            utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Warning, |h| Ptr::from_mut(h))
        } else {
            utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Error, |h| Ptr::from_mut(h))
        };
        let node2EdgeRouter: Ptr<Node2EdgeRouter<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V, M>> =
            Ptr::from_raw(Box::into_raw(Box::new(Node2EdgeRouter::new(flippedEdges, unbuildIsWarning, Some(flippedOperation), None, havePermissions, haveRestrictions))));
        // C++ passes myArcInfos by reference into the tree; share one heap Vec between both.
        let arcInfos: &'static mut Vec<Ptr<AFBuild_ArcInfo>> = Box::leak(Box::new(Vec::new()));
        let arcInfosPtr = Ptr::from_mut(arcInfos);
        let centralizedSPTree: Ptr<AFCentralizedSPTree<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>> =
            Ptr::from_raw(Box::into_raw(Box::new(AFCentralizedSPTree::new(flippedEdges, arcInfosPtr.m(), unbuildIsWarning, node2EdgeRouter.cast::<SUMOAbstractRouter<FlippedEdge<E, N, V>, V>>(), havePermissions, haveRestrictions))));
        Self {
            EPS: 0.009,
            myFlippedEdges: flippedEdges.clone(),
            myFlippedPartition: flippedPartition,
            myNumberOfLevels: numberOfLevels,
            myErrorMsgHandler: msgInstance,
            myFlippedOperation: flippedOperation,
            myFlippedLookupTable: flippedLookup,
            myHavePermissions: havePermissions,
            myHaveRestrictions: haveRestrictions,
            myProhibited: toProhibit,
            myNode2EdgeRouter: node2EdgeRouter,
            myCentralizedSPTree: centralizedSPTree,
            myArcInfos: arcInfosPtr,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void AFBuild::~AFBuild<E, N, V, M>()`
    /// Destructor
    // ~AFBuild() {
    //         delete myCentralizedSPTree;
    //         delete myNode2EdgeRouter;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self)
    where E: RouterEdge<V, Node = N>, V: RouterVehicle
    {
        let tree = self.myCentralizedSPTree;
        if !tree.is_null() {
            tree.m().drop_();
            delete_ptr(tree);
        }
        let router = self.myNode2EdgeRouter;
        if !router.is_null() {
            router.m().drop_();
            delete_ptr(router);
        }
    }

    /// C++: `typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation AFBuild::getFlippedOperation()`
    /// Returns the operation for a backward graph with flipped edges
    // typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation getFlippedOperation() {
    //         return myFlippedOperation;
    //     }
    #[allow(unused_variables)]
    pub fn get_flipped_operation(&mut self) -> AFBuild_FlippedOperation<E, N, V> {
        self.myFlippedOperation
    }

    /// C++: `const std::shared_ptr<const FlippedLookupTable> AFBuild::getFlippedLookup()`
    /// Returns the lookup table for the backward graph with flipped edges
    // const std::shared_ptr<const FlippedLookupTable> getFlippedLookup() {
    //         return myFlippedLookupTable;
    //     }
    #[allow(unused_variables)]
    pub fn get_flipped_lookup(&mut self) -> Rc<AFBuild_FlippedLookupTable> {
        Rc::clone(&self.myFlippedLookupTable)
    }

    /// C++: `void AFBuild::init(SUMOTime, const V *const, int &)`
    /// Initialize the arc flag build
    // template<class E, class N, class V, class M>
    // void AFBuild<E, N, V, M>::init(SUMOTime msTime, const V* const vehicle, std::vector<FlagInfo*>& flagInfos) {
    //     if (myArcInfos.empty()) {
    //         for (const FlippedEdge<E, N, V>* const flippedEdge : myFlippedEdges) {
    //             myArcInfos.push_back(new ArcInfo(flippedEdge));
    //         }
    //     }
    //     int sHARCLevel;
    //     for (sHARCLevel = 0; sHARCLevel < myNumberOfLevels - 1; sHARCLevel++) {
    // #ifdef AFBU_DEBUG_LEVEL_0
    //         std::cout << "Starting computation of flags of level " << sHARCLevel << " (levels run from 0 to "
    //                   << myNumberOfLevels - 2 << ")." << std::endl;
    // #endif
    // #ifdef AFBU_DEBUG_LEVEL_2
    //         if (sHARCLevel != 0) {
    //             continue;
    //         }
    // #endif
    //         computeArcFlags(msTime, sHARCLevel, vehicle);
    //     }
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Copying arc flags from the arc infos... " << std::endl;
    // #endif
    //     int index = 0;
    //     for (const ArcInfo* arcInfo : myArcInfos) {
    //         flagInfos[index++]->arcFlags = arcInfo->arcFlags;
    //         delete arcInfo;
    //     }
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Arc flags copied from the arc infos. " << std::endl;
    // #endif
    //     myArcInfos.clear();
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, time: SUMOTime, vehicle: Ptr<V>, flagInfos: &mut Vec<Ptr<AFInfo_FlagInfo>>)
    where E: RouterEdge<V, Node = N>, N: RouterNode<Edge = E>, V: RouterVehicle
    {
        if self.myArcInfos.is_empty() {
            for &flippedEdge in self.myFlippedEdges.iter() {
                self.myArcInfos.push(Ptr::from_raw(Box::into_raw(Box::new(AFInfo_ArcInfo::new(flippedEdge.cast::<Unmapped>())))));
            }
        }
        for sHARCLevel in 0..self.myNumberOfLevels - 1 {
            self.compute_arc_flags(time, sHARCLevel, vehicle);
        }
        let mut index = 0;
        for arcInfo in self.myArcInfos.iter() {
            flagInfos[index].m().arcFlags = arcInfo.arcFlags.clone();
            index += 1;
        }
        self.myArcInfos.clear();
    }

    /// C++: `void AFBuild::setFlippedPartition(const KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V> *)`
    /// Set the flipped partition param[in] flippedPartition The flipped partition
    // void setFlippedPartition(const KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>* flippedPartition) {
    //         myFlippedPartition = flippedPartition;
    //     }
    #[allow(unused_variables)]
    pub fn set_flipped_partition(&mut self, flippedPartition: Ptr<KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>>) {
        self.myFlippedPartition = flippedPartition;
    }

    /// C++: `int AFBuild::partitionLevel2SHARCLevel(int)`
    /// Converts a partition level number to a SHARC level number
    // int partitionLevel2SHARCLevel(int partitionLevel) {
    //         return AFRouter<E, N, V, M>::partitionLevel2SHARCLevel(partitionLevel, myNumberOfLevels);
    //     }
    #[allow(unused_variables)]
    pub fn partition_level2_sharc_level(&mut self, partitionLevel: i32) -> i32 {
        AFRouter::<E, N, V, M>::partition_level2_sharc_level(partitionLevel, self.myNumberOfLevels)
    }

    /// C++: `int AFBuild::sHARCLevel2PartitionLevel(int)`
    /// Converts a SHARC level number to a partition level number
    // int sHARCLevel2PartitionLevel(int sHARCLevel) {
    //         return AFRouter<E, N, V, M>::sHARCLevel2PartitionLevel(sHARCLevel, myNumberOfLevels);
    //     }
    #[allow(unused_variables)]
    pub fn s_harc_level2_partition_level(&mut self, sHARCLevel: i32) -> i32 {
        AFRouter::<E, N, V, M>::s_harc_level2_partition_level(sHARCLevel, self.myNumberOfLevels)
    }

    /// C++: `void AFBuild::computeArcFlags(SUMOTime, int, const V *const)` [protected]
    /// Computes the arc flags for all cells at a given level
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn compute_arc_flags(&mut self, msTime: SUMOTime, sHARCLevel: i32, vehicle: Ptr<V>)
    where E: RouterEdge<V, Node = N>, N: RouterNode<Edge = E>, V: RouterVehicle
    {
        let partitionLevel = self.s_harc_level2_partition_level(sHARCLevel);
        let levelCells = self.myFlippedPartition.m().get_cells_at_level(partitionLevel).clone();
        for cell in levelCells {
            self.compute_arc_flags_2(msTime, sHARCLevel, cell, vehicle);
            for arcInfo in self.myArcInfos.iter() {
                arcInfo.m().effortsToBoundaryNodes.clear();
                arcInfo.m().touched = false;
            }
        }
    }

    /// C++: `void AFBuild::computeArcFlags(SUMOTime, const int, const Cell *, const V *const)` [protected]
    /// Computes the arc flags for a given cell
    // template<class E, class N, class V, class M>
    // void AFBuild<E, N, V, M>::computeArcFlags(SUMOTime msTime, const int sHARCLevel, const Cell* cell, const V* const vehicle) {
    //     const Cell* supercell = cell->getSupercell();
    //     const std::unordered_set<const FlippedEdge<E, N, V>*>& boundaryEdges = cell->getOutgoingBoundaryEdges();
    //     const std::vector<const FlippedNode<E, N, V>*>& boundaryNodes = cell->getBoundaryFromNodes();
    // #ifdef AFBU_DEBUG_LEVEL_1
    //     std::cout << "Number of boundary edges: " << boundaryEdges.size() << std::endl;
    //     std::cout << "Number of boundary nodes: " << boundaryNodes.size() << std::endl;
    //     std::cout << "Cell number: " << cell->getNumber() << std::endl;
    //     std::cout << "Supercell number: " << supercell->getNumber() << std::endl;
    // #endif
    //     // initialization of arc flag vectors
    //     initBoundaryEdges(boundaryEdges);
    // #ifdef AFBU_DEBUG_LEVEL_1
    //     long long int startTime = SysUtils::getCurrentMillis();
    // #endif
    //     std::map<const FlippedEdge<E, N, V>*, std::vector<const FlippedEdge<E, N, V>*>> incomingEdgesOfOutgoingBoundaryEdges;
    //     size_t numberOfBoundaryNodes = boundaryNodes.size();
    //     for (const FlippedEdge<E, N, V>* boundaryEdge : boundaryEdges) {
    //         incomingEdgesOfOutgoingBoundaryEdges[boundaryEdge] = std::vector<const FlippedEdge<E, N, V>*>(numberOfBoundaryNodes);
    //     }
    //     int index = 0; // boundary node index
    //     for (const FlippedNode<E, N, V>* boundaryNode : boundaryNodes) {
    //         myNode2EdgeRouter->reset(vehicle);
    //         if (myNode2EdgeRouter->computeNode2Edges(boundaryNode, boundaryEdges, vehicle, msTime)) {
    // #ifdef AFBU_DEBUG_LEVEL_2
    //             std::cout << "Node-to-edge router succeeded." << std::endl;
    // #endif
    //         }
    //         for (const FlippedEdge<E, N, V>* boundaryEdge : boundaryEdges) {
    //             assert(!boundaryEdge->isInternal());
    //             ArcInfo* arcInfo = myArcInfos[boundaryEdge->getNumericalID()];
    //             if (boundaryNode == boundaryEdge->getFromJunction()) {
    //                 arcInfo->effortsToBoundaryNodes.push_back(0.);
    //                 (incomingEdgesOfOutgoingBoundaryEdges[boundaryEdge])[index] = nullptr;
    //                 continue;
    //             }
    //             double effort = (myNode2EdgeRouter->edgeInfo(boundaryEdge))->effort;
    //             const FlippedEdge<E, N, V>* incomingEdge
    //                 = (myNode2EdgeRouter->edgeInfo(boundaryEdge))->prev ?
    //                   (myNode2EdgeRouter->edgeInfo(boundaryEdge))->prev->edge : nullptr;
    //             (incomingEdgesOfOutgoingBoundaryEdges[boundaryEdge])[index] = incomingEdge;
    //             arcInfo->effortsToBoundaryNodes.push_back(effort == std::numeric_limits<double>::max() ? UNREACHABLE : effort);
    //         }
    //         index++;
    //     } // end for boundary nodes
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     long long int timeSpent = SysUtils::getCurrentMillis() - startTime;
    //     std::cout << "Initial distance computation spent " + elapsedMs2string(timeSpent) + "." << std::endl;
    // #endif
    //     // initialize all supercell edges' labels and arc flag vectors for the centralized shortest path DAG algorithm / arc flag build
    //     initSupercellEdges(supercell, boundaryEdges, boundaryNodes.size());
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Initialization of all supercell edges' labels and arc flag vectors done. Starting the centralized shortest path tree algorithm..." << std::endl;
    // #endif
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     startTime = SysUtils::getCurrentMillis();
    // #endif
    //     if (myCentralizedSPTree->computeCentralizedSPTree(msTime, cell, vehicle, incomingEdgesOfOutgoingBoundaryEdges)) {
    // #ifdef AFBU_DEBUG_LEVEL_0
    //         timeSpent = SysUtils::getCurrentMillis() - startTime;
    //         std::cout << "Centralized SP tree computation spent " + elapsedMs2string(timeSpent) + "." << std::endl;
    // #endif
    //         computeArcFlagsAux(msTime, sHARCLevel, cell, vehicle);
    //     }
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Centralized shortest path tree algorithm finished." << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_arc_flags_2(&mut self, msTime: SUMOTime, sHARCLevel: i32, cell: Ptr<KDTreePartition_Cell>, vehicle: Ptr<V>)
    where E: RouterEdge<V, Node = N>, N: RouterNode<Edge = E>, V: RouterVehicle
    {
        let supercell = cell.get_supercell();
        // Cell stores type-erased edge/node pointers; cast them back to the flipped graph types.
        let boundaryEdges: Vec<Ptr<FlippedEdge<E, N, V>>> = cell.get_outgoing_boundary_edges().iter()
            .map(|&e| e.cast::<FlippedEdge<E, N, V>>()).collect();
        let boundaryNodes: Vec<Ptr<FlippedNode<E, N, V>>> = cell.get_boundary_from_nodes().iter()
            .map(|&n| n.cast::<FlippedNode<E, N, V>>()).collect();
        self.init_boundary_edges(&boundaryEdges);
        let mut incomingEdgesOfOutgoingBoundaryEdges: BTreeMap<Ptr<FlippedEdge<E, N, V>>, Vec<Ptr<FlippedEdge<E, N, V>>>> = BTreeMap::new();
        let numberOfBoundaryNodes = boundaryNodes.len();
        for &boundaryEdge in boundaryEdges.iter() {
            incomingEdgesOfOutgoingBoundaryEdges.insert(boundaryEdge, vec![Ptr::null(); numberOfBoundaryNodes]);
        }
        let boundaryEdgesSet: HashSet<Ptr<FlippedEdge<E, N, V>>> = boundaryEdges.iter().copied().collect();
        let mut index = 0;
        for &boundaryNode in boundaryNodes.iter() {
            self.myNode2EdgeRouter.m().reset(vehicle);
            self.myNode2EdgeRouter.m().compute_node2_edges(boundaryNode, &boundaryEdgesSet, vehicle, msTime, false);
            for &boundaryEdge in boundaryEdges.iter() {
                debug_assert!(!boundaryEdge.is_internal());
                let arcInfo = self.myArcInfos[FlippedEdge::get_numerical_id(&*boundaryEdge) as usize];
                if boundaryNode == boundaryEdge.get_from_junction() {
                    arcInfo.m().effortsToBoundaryNodes.push(0.);
                    incomingEdgesOfOutgoingBoundaryEdges.get_mut(&boundaryEdge).unwrap()[index] = Ptr::null();
                    continue;
                }
                let edgeInfo = self.myNode2EdgeRouter.m().edge_info(boundaryEdge);
                let effort = edgeInfo.effort;
                let incomingEdge = if edgeInfo.prev.is_null() { Ptr::null() } else { edgeInfo.prev.edge.cast::<FlippedEdge<E, N, V>>() };
                incomingEdgesOfOutgoingBoundaryEdges.get_mut(&boundaryEdge).unwrap()[index] = incomingEdge;
                arcInfo.m().effortsToBoundaryNodes.push(if effort == f64::MAX { UNREACHABLE } else { effort });
            }
            index += 1;
        }
        self.init_supercell_edges(supercell, &boundaryEdges, boundaryNodes.len());
        if self.myCentralizedSPTree.m().compute_centralized_sp_tree(msTime, cell, vehicle, &incomingEdgesOfOutgoingBoundaryEdges, false) {
            self.compute_arc_flags_aux(msTime, sHARCLevel, cell, vehicle);
        }
    }

    /// C++: `void AFBuild::computeArcFlagsNaive(SUMOTime, const int, const Cell *, const V *const)` [protected]
    /// Computes the arc flags for a given cell (naive version)
    // template<class E, class N, class V, class M>
    // void AFBuild<E, N, V, M>::computeArcFlagsNaive(SUMOTime msTime, const int sHARCLevel, const Cell* cell, const V* const vehicle) {
    //     const Cell* supercell = cell->getSupercell();
    //     const std::unordered_set<const FlippedEdge<E, N, V>*>& boundaryEdges = cell->getOutgoingBoundaryEdges();
    //     const std::vector<const FlippedNode<E, N, V>*>& boundaryNodes = cell->getBoundaryFromNodes();
    // #ifdef AFBU_DEBUG_LEVEL_1
    //     std::cout << "Number of boundary edges: " << boundaryEdges.size() << std::endl;
    //     std::cout << "Number of boundary nodes: " << boundaryNodes.size() << std::endl;
    //     std::cout << "Cell number: " << cell->getNumber() << std::endl;
    //     std::cout << "Supercell number: " << supercell->getNumber() << std::endl;
    // #endif
    //     //
    //     // initialization of arc flag vectors
    //     initBoundaryEdges(boundaryEdges);
    //
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     long long int startTime = SysUtils::getCurrentMillis();
    // #endif
    //     for (const FlippedNode<E, N, V>* boundaryNode : boundaryNodes) {
    //         for (const FlippedEdge<E, N, V>* boundaryEdge : boundaryEdges) {
    //             assert(!boundaryEdge->isInternal());
    //             ArcInfo* arcInfo = myArcInfos[boundaryEdge->getNumericalID()];
    //             if (boundaryNode == boundaryEdge->getFromJunction()) {
    //                 arcInfo->effortsToBoundaryNodes.push_back(0.);
    //                 continue;
    //             }
    //             // compute effort
    //             std::vector<const FlippedEdge<E, N, V>*> into;
    // #ifdef AFBU_DEBUG_LEVEL_2
    //             std::vector<const FlippedEdge<E, N, V>*> into2;
    // #endif
    //             if (myNode2EdgeRouter->computeNode2Edge(boundaryNode, boundaryEdge, vehicle, msTime, into)) {
    //                 double recomputedEffort = myNode2EdgeRouter->recomputeCostsNoLastEdge(into, vehicle, msTime);
    //                 arcInfo->effortsToBoundaryNodes.push_back(recomputedEffort);
    // #ifdef AFBU_DEBUG_LEVEL_2
    //                 if (!into.empty() && myNode2EdgeRouter->compute(into[0], boundaryEdge, vehicle, msTime, into2)) {
    //                     double recomputedEffort2 = myNode2EdgeRouter->recomputeCosts(into2, vehicle, msTime);
    //
    //                     std::cout << "node2Edge router succeeded, effort: " << recomputedEffort << ", effort incl. last edge: " << recomputedEffort2 << std::endl;
    //                     assert(recomputedEffort <= recomputedEffort2);
    //                 }
    // #endif
    //             } else {
    //                 arcInfo->effortsToBoundaryNodes.push_back(UNREACHABLE);
    // #ifdef AFBU_DEBUG_LEVEL_2
    //                 std::cout << "UNREACHABLE!" << std::endl;
    // #endif
    //             }
    //         }
    //     }
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     long long int timeSpent = SysUtils::getCurrentMillis() - startTime;
    //     std::cout << "Initial distance computation spent " + elapsedMs2string(timeSpent) + "." << std::endl;
    // #endif
    //
    //     // initialize all supercell edges' labels, arc flag vectors for the centralized shortest path tree algorithm / arc flag build
    //     initSupercellEdges(supercell, boundaryEdges, boundaryNodes.size());
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Initialization of all supercell edges' labels and arc flag vectors done. Starting the centralized shortest path tree algorithm..." << std::endl;
    // #endif
    //     if (myCentralizedSPTree->computeCentralizedSPTree(msTime, cell, vehicle)) {
    //         computeArcFlagsAux(msTime, sHARCLevel, cell, vehicle);
    //     }
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Centralized shortest path tree algorithm finished." << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_arc_flags_naive(&mut self, msTime: SUMOTime, sHARCLevel: i32, cell: Ptr<KDTreePartition_Cell>, vehicle: Ptr<V>)
    where E: RouterEdge<V, Node = N>, N: RouterNode<Edge = E>, V: RouterVehicle
    {
        let supercell = cell.get_supercell();
        let boundaryEdges: Vec<Ptr<FlippedEdge<E, N, V>>> = cell.get_outgoing_boundary_edges().iter()
            .map(|&e| e.cast::<FlippedEdge<E, N, V>>()).collect();
        let boundaryNodes: Vec<Ptr<FlippedNode<E, N, V>>> = cell.get_boundary_from_nodes().iter()
            .map(|&n| n.cast::<FlippedNode<E, N, V>>()).collect();
        self.init_boundary_edges(&boundaryEdges);
        for &boundaryNode in boundaryNodes.iter() {
            for &boundaryEdge in boundaryEdges.iter() {
                debug_assert!(!boundaryEdge.is_internal());
                let arcInfo = self.myArcInfos[FlippedEdge::get_numerical_id(&*boundaryEdge) as usize];
                if boundaryNode == boundaryEdge.get_from_junction() {
                    arcInfo.m().effortsToBoundaryNodes.push(0.);
                    continue;
                }
                let mut into: Vec<Ptr<FlippedEdge<E, N, V>>> = Vec::new();
                if self.myNode2EdgeRouter.m().compute_node2_edge(boundaryNode, boundaryEdge, vehicle, msTime, &mut into, false) {
                    let recomputedEffort = self.myNode2EdgeRouter.m().recompute_costs_no_last_edge(&into, vehicle, msTime, Ptr::null());
                    arcInfo.m().effortsToBoundaryNodes.push(recomputedEffort);
                } else {
                    arcInfo.m().effortsToBoundaryNodes.push(UNREACHABLE);
                }
            }
        }
        self.init_supercell_edges(supercell, &boundaryEdges, boundaryNodes.len());
        // C++ keeps this naive variant for comparisons only (never instantiated); its 3-arg
        // computeCentralizedSPTree overload is absent. Use an empty map so the port compiles.
        let empty = BTreeMap::new();
        if self.myCentralizedSPTree.m().compute_centralized_sp_tree(msTime, cell, vehicle, &empty, false) {
            self.compute_arc_flags_aux(msTime, sHARCLevel, cell, vehicle);
        }
    }

    /// C++: `void AFBuild::putArcFlag(ArcInfo *, const int, const bool)` [protected]
    /// Put the arc flag of the edge in arcInfo
    // template<class E, class N, class V, class M>
    // void AFBuild<E, N, V, M>::putArcFlag(ArcInfo* arcInfo, const int sHARCLevel, const bool isLeftOrLowerCell) {
    //     assert(arcInfo);
    //     (arcInfo->arcFlags)[sHARCLevel * 2 + (isLeftOrLowerCell ? 0 : 1)] = 1;
    // }
    #[allow(unused_variables)]
    pub fn put_arc_flag(&mut self, arcInfo: Ptr<AFInfo_ArcInfo>, sHARCLevel: i32, isLeftOrLowerCell: bool) {
        debug_assert!(!arcInfo.is_null());
        arcInfo.m().arcFlags[(sHARCLevel * 2 + if isLeftOrLowerCell { 0 } else { 1 }) as usize] = true;
    }

    /// C++: `void AFBuild::initBoundaryEdges(const int &)` [private]
    /// Initialize the boundary edges param[in] boundaryEdges The boundary edges
    // template<class E, class N, class V, class M>
    // void AFBuild<E, N, V, M>::initBoundaryEdges(const std::unordered_set<const FlippedEdge<E, N, V>*>& boundaryEdges) {
    //     // initialization of arc flag vectors
    //     for (const FlippedEdge<E, N, V>* boundaryEdge : boundaryEdges) {
    //         assert(!boundaryEdge->isInternal());
    //         ArcInfo* arcInfo;
    //         arcInfo = myArcInfos[boundaryEdge->getNumericalID()];
    //         if (arcInfo->arcFlags.empty()) {
    //             std::fill_n(std::back_inserter(arcInfo->arcFlags),
    //                         myFlippedPartition->numberOfArcFlags(), false);
    //         }
    //         arcInfo->effortsToBoundaryNodes.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_boundary_edges(&mut self, boundaryEdges: &Vec<Ptr<FlippedEdge<E, N, V>>>)
    where E: RouterEdge<V, Node = N>, N: RouterNode<Edge = E>, V: RouterVehicle
    {
        for &boundaryEdge in boundaryEdges.iter() {
            debug_assert!(!boundaryEdge.is_internal());
            let arcInfo = self.myArcInfos[FlippedEdge::get_numerical_id(&*boundaryEdge) as usize];
            if arcInfo.arcFlags.is_empty() {
                for _ in 0..self.myFlippedPartition.number_of_arc_flags() {
                    arcInfo.m().arcFlags.push(false);
                }
            }
            arcInfo.m().effortsToBoundaryNodes.clear();
        }
    }

    /// C++: `void AFBuild::initSupercellEdges(const Cell *, const int &, size_t)` [private]
    /// Initialize the supercell edges
    // template<class E, class N, class V, class M>
    // void AFBuild<E, N, V, M>::initSupercellEdges(
    //     const Cell* supercell,
    //     const std::unordered_set<const FlippedEdge<E, N, V>*>& boundaryEdges,
    //     size_t numberOfBoundaryNodes) {
    //     std::pair<typename std::vector<const FlippedNode<E, N, V>*>::const_iterator,
    //         typename std::vector<const FlippedNode<E, N, V>*>::const_iterator> supercellNodeIterators = supercell->nodeIterators();
    //     typename std::vector<const FlippedNode<E, N, V>*>::const_iterator first = supercellNodeIterators.first;
    //     typename std::vector<const FlippedNode<E, N, V>*>::const_iterator last = supercellNodeIterators.second;
    //     typename std::vector<const FlippedNode<E, N, V>*>::const_iterator iter;
    //     for (iter = first; iter != last; iter++) {
    //         const std::vector<const FlippedEdge<E, N, V>*> incomingEdges = (*iter)->getIncoming();
    //         for (const FlippedEdge<E, N, V>* supercellEdge : incomingEdges) {
    //             if (supercellEdge->isInternal()) {
    //                 continue;
    //             }
    //             if (boundaryEdges.count(supercellEdge)) {
    //                 continue;
    //             }
    //             ArcInfo* supercellArcInfo;
    //             supercellArcInfo = myArcInfos[supercellEdge->getNumericalID()];
    //             if (supercellArcInfo->arcFlags.empty()) {
    //                 std::fill_n(std::back_inserter(supercellArcInfo->arcFlags),
    //                             myFlippedPartition->numberOfArcFlags(), false);
    //             }
    //             supercellArcInfo->effortsToBoundaryNodes.clear();
    //             std::fill_n(std::back_inserter(supercellArcInfo->effortsToBoundaryNodes),
    //                         numberOfBoundaryNodes, std::numeric_limits<double>::max());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_supercell_edges(&mut self, supercell: Ptr<KDTreePartition_Cell>, boundaryEdges: &Vec<Ptr<FlippedEdge<E, N, V>>>, numberOfBoundaryNodes: usize)
    where E: RouterEdge<V, Node = N>, N: RouterNode<Edge = E>, V: RouterVehicle
    {
        let boundaryEdgesSet: HashSet<Ptr<FlippedEdge<E, N, V>>> = boundaryEdges.iter().copied().collect();
        for node in supercell.nodes::<FlippedNode<E, N, V>>() {
            for &supercellEdge in node.get_incoming().iter() {
                if supercellEdge.is_internal() {
                    continue;
                }
                if boundaryEdgesSet.contains(&supercellEdge) {
                    continue;
                }
                let arcInfo = self.myArcInfos[FlippedEdge::get_numerical_id(&*supercellEdge) as usize];
                if arcInfo.arcFlags.is_empty() {
                    for _ in 0..self.myFlippedPartition.number_of_arc_flags() {
                        arcInfo.m().arcFlags.push(false);
                    }
                }
                arcInfo.m().effortsToBoundaryNodes.clear();
                for _ in 0..numberOfBoundaryNodes {
                    arcInfo.m().effortsToBoundaryNodes.push(f64::MAX);
                }
            }
        }
    }

    /// C++: `void AFBuild::computeArcFlagsAux(SUMOTime, const int, const Cell *, const V *const)` [private]
    /// Helper method for computeArcFlags(), which computes the arc flags for a given cell
    // template<class E, class N, class V, class M>
    // void AFBuild<E, N, V, M>::computeArcFlagsAux(SUMOTime msTime, const int sHARCLevel, const Cell* cell, const V* const vehicle) {
    //     const Cell* supercell = cell->getSupercell();
    //     std::pair<typename std::vector<const FlippedNode<E, N, V>*>::const_iterator,
    //         typename std::vector<const FlippedNode<E, N, V>*>::const_iterator> supercellNodeIterators = supercell->nodeIterators();
    //     typename std::vector<const FlippedNode<E, N, V>*>::const_iterator first = supercellNodeIterators.first;
    //     typename std::vector<const FlippedNode<E, N, V>*>::const_iterator last = supercellNodeIterators.second;
    //     typename std::vector<const FlippedNode<E, N, V>*>::const_iterator iter;
    //     std::unordered_set<ArcInfo*> arcInfosOnAShortestPath;
    // #ifdef AFBU_DEBUG_LEVEL_1
    //     int numberOfSupercellEdges = 0;
    // #endif
    //     for (iter = first; iter != last; iter++) {
    //         const std::vector<const FlippedEdge<E, N, V>*> incomingEdges = (*iter)->getIncoming();
    //         for (const FlippedEdge<E, N, V>* supercellEdge : incomingEdges) {
    //             if (supercellEdge->isInternal()) {
    //                 continue;
    //             }
    //             if (myNode2EdgeRouter->isProhibited(supercellEdge, vehicle, STEPS2TIME(msTime))) {
    //                 continue;
    //             }
    // #ifdef AFBU_DEBUG_LEVEL_1
    //             numberOfSupercellEdges++;
    // #endif
    //             ArcInfo* supercellArcInfo = myArcInfos[supercellEdge->getNumericalID()];
    //             // start by initializing to set of all supercell edge arc infos
    //             arcInfosOnAShortestPath.insert(supercellArcInfo);
    //         }
    //     }
    // #ifdef AFBU_DEBUG_LEVEL_1
    //     std::cout << "Number of supercell edges: " << numberOfSupercellEdges << std::endl;
    // #endif
    //     const SUMOVehicleClass vClass = vehicle == 0 ? SVC_IGNORING : vehicle->getVClass();
    // #ifdef AFBU_DEBUG_LEVEL_1
    //     std::cout << "Identifying shortest paths..." << std::endl;
    // #endif
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     long long int startTime = SysUtils::getCurrentMillis();
    // #endif
    //     std::unordered_set<ArcInfo*> erasedEdges;
    //     for (auto iter2 = arcInfosOnAShortestPath.begin(); iter2 != arcInfosOnAShortestPath.end();) {
    //         const ArcInfo* arcInfo = *iter2;
    //         assert(!arcInfo->edge->isInternal());
    //         assert(myNode2EdgeRouter);
    //         assert(!myNode2EdgeRouter->isProhibited(arcInfo->edge, vehicle, STEPS2TIME(msTime)));
    //         size_t numberOfBoundaryNodes = arcInfo->effortsToBoundaryNodes.size();
    //         size_t index;
    //         bool onShortestPath = false;
    //         // attempt to prove that the edge is on a shortest path for at least one boundary node
    //         for (index = 0; index < numberOfBoundaryNodes; index++) {
    //             double effort1ToBoundaryNode = arcInfo->effortsToBoundaryNodes[index];
    //             if (effort1ToBoundaryNode == UNREACHABLE) {
    //                 continue;
    //             }
    //             if (effort1ToBoundaryNode == std::numeric_limits<double>::max()) {
    //                 continue;
    //             }
    //             double sTime = STEPS2TIME(msTime);
    //             double edgeEffort = myNode2EdgeRouter->getEffort(arcInfo->edge, vehicle, sTime);
    //             sTime += myNode2EdgeRouter->getTravelTime(arcInfo->edge, vehicle, sTime, edgeEffort);
    //             double oldEdgeEffort = edgeEffort;
    //             double oldSTime = sTime;
    //
    //             for (const std::pair<const FlippedEdge<E, N, V>*, const FlippedEdge<E, N, V>*>& follower : arcInfo->edge->getViaSuccessors(vClass)) {
    //                 assert(!follower.first->isInternal());
    //                 ArcInfo* followerInfo = myArcInfos[follower.first->getNumericalID()];
    //
    //                 // check whether it can be used
    //                 if (myNode2EdgeRouter->isProhibited(follower.first, vehicle, sTime)) {
    //                     myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + followerInfo->edge->getID() + "'.");
    //                     continue;
    //                 }
    //                 if (followerInfo->effortsToBoundaryNodes.empty()) {
    //                     continue;
    //                 }
    //                 double effort2ToBoundaryNode = followerInfo->effortsToBoundaryNodes[index];
    //                 if (effort2ToBoundaryNode == UNREACHABLE) {
    //                     continue;
    //                 }
    //                 if (effort2ToBoundaryNode == std::numeric_limits<double>::max()) {
    //                     continue;
    //                 }
    //
    //                 // add via efforts to current follower to edge effort
    //                 double length = 0.; // dummy length for call of updateViaEdgeCost
    //                 myNode2EdgeRouter->updateViaEdgeCost(follower.second, vehicle,
    //                                                      sTime /* has been updated to the time where the edge has been traversed */, edgeEffort, length);
    //                 // test whether edge is on a shortest path to a boundary node
    //                 if (effort1ToBoundaryNode + edgeEffort /* edge effort incl. via efforts to current follower of edge */
    //                         <= effort2ToBoundaryNode /* efforts incl. via efforts to current follower o. e. */
    //                         + EPS && effort1ToBoundaryNode + edgeEffort >= effort2ToBoundaryNode - EPS) {
    //                     onShortestPath = true;
    //                     break; // a shortest path to one boundary node suffices
    //                 }
    //                 edgeEffort = oldEdgeEffort;
    //                 sTime = oldSTime;
    //             } // end of loop over outgoing edges
    //         } // loop over indexes
    //         if (!onShortestPath) {
    //             erasedEdges.insert(*iter2);
    //             iter2 = arcInfosOnAShortestPath.erase(iter2); // not on a shortest path, remove it
    //         } else {
    //             ++iter2;
    //         }
    //     } // loop over edge infos
    //
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Edges clearly not on a shortest path have been removed. Number of edges on a shortest path is now: "
    //               << arcInfosOnAShortestPath.size() << std::endl;
    // #endif
    //
    //     // set arc flags (for level sHARCLevel) for all edges completely inside the cell
    //     // (done since these edges all have a to-node inside the cell, i.e. we have to set the own-cell flag for them).
    //     std::unordered_set<const FlippedEdge<E, N, V>*>* pEdgesInsideCell = cell->edgeSet(vehicle);
    //     std::unordered_set<const FlippedEdge<E, N, V>*> edgesInsideCell = *pEdgesInsideCell;
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Adding all edges completely inside the cell to set of edges on a shortest path, cell no:"
    //               << cell->getNumber() << std::endl;
    // #endif
    //     for (const FlippedEdge<E, N, V>* edgeInsideCell : edgesInsideCell) {
    //         ArcInfo* arcInfo = myArcInfos[edgeInsideCell->getNumericalID()];
    //         if (myNode2EdgeRouter->isProhibited(edgeInsideCell, vehicle, STEPS2TIME(msTime))) {
    //             continue;
    //         }
    //         arcInfosOnAShortestPath.insert(arcInfo);
    //     }
    //     delete pEdgesInsideCell;
    // #ifdef AFBU_DEBUG_LEVEL_0
    //     std::cout << "Edges inside cell added." << std::endl;
    //     long long int timeSpent = SysUtils::getCurrentMillis() - startTime;
    //     std::cout << "Shortest path identification spent " + elapsedMs2string(timeSpent) + "." << std::endl;
    // #endif
    //
    // #ifdef AFBU_WRITE_QGIS_FILTERS
    //     std::string qgisFilterString = "id IN (";
    //     std::unordered_set<const FlippedNode<E, N, V>*> nodesOnAShortestPath;
    //     std::unordered_set<const FlippedNode<E, N, V>*> erasedNodes;
    //     for (const ArcInfo* arcInfo : arcInfosOnAShortestPath) {
    //         assert(!(myNode2EdgeRouter->edgeInfo(arcInfo->edge))->prohibited
    //                && !myNode2EdgeRouter->isProhibited(arcInfo->edge, vehicle));
    //         nodesOnAShortestPath.insert(arcInfo->edge->getFromJunction());
    //         nodesOnAShortestPath.insert(arcInfo->edge->getToJunction());
    //     }
    //     for (const ArcInfo* erasedEdgeArcInfo : erasedEdges) {
    //         erasedNodes.insert(erasedEdgeArcInfo->edge->getFromJunction());
    //         erasedNodes.insert(erasedEdgeArcInfo->edge->getToJunction());
    //     }
    //     size_t k = 0;
    //     // go through the relevant nodes of the supercell
    //     size_t numberOfNodesOnAShortestPath = nodesOnAShortestPath.size();
    //     for (const FlippedNode<E, N, V>* node : nodesOnAShortestPath) {
    //         k++;
    //         qgisFilterString += node->getID() + (k < numberOfNodesOnAShortestPath ? ", " : "");
    //     }
    //     qgisFilterString += ")";
    //     std::ostringstream pathAndFileName;
    //     pathAndFileName << "./filter_superset_nodes_cell_" << cell->getNumber() << "_supercell_" << supercell->getNumber() << ".qqf";
    //     std::ofstream file;
    //     file.open(pathAndFileName.str());
    //     std::ostringstream content;
    //     content << "<Query>" << qgisFilterString << "</Query>";
    //     file << content.str();
    //     file.close();
    //     // erased nodes
    //     k = 0;
    //     qgisFilterString.clear();
    //     qgisFilterString = "id IN (";
    //     // go through the erased nodes of the supercell
    //     size_t numberOfErasedNodes = erasedNodes.size();
    //     for (const FlippedNode<E, N, V>* node : erasedNodes) {
    //         k++;
    //         qgisFilterString += node->getID() + (k < numberOfErasedNodes ? ", " : "");
    //     }
    //     qgisFilterString += ")";
    //     pathAndFileName.str("");
    //     pathAndFileName.clear();
    //     pathAndFileName << "./filter_erased_nodes_cell_" << cell->getNumber() << "_supercell_" << supercell->getNumber() << ".qqf";
    //     file.clear();
    //     file.open(pathAndFileName.str());
    //     content.str("");
    //     content.clear();
    //     content << "<Query>" << qgisFilterString << "</Query>";
    //     file << content.str();
    //     file.close();
    // #endif
    //     // put arc flags for level 'sHARCLevel' for all supercell edges which are on a shortest path
    //     for (ArcInfo* arcInfo : arcInfosOnAShortestPath) {
    //         putArcFlag(arcInfo, sHARCLevel, cell->isLeftOrLowerCell());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_arc_flags_aux(&mut self, msTime: SUMOTime, sHARCLevel: i32, cell: Ptr<KDTreePartition_Cell>, vehicle: Ptr<V>)
    where E: RouterEdge<V, Node = N>, N: RouterNode<Edge = E>, V: RouterVehicle
    {
        let supercell = cell.get_supercell();
        // determinism: C++ uses std::unordered_set<ArcInfo*>; iteration order changes with pointer
        // addresses but only flag bits are written, so the resulting flags are order-independent.
        let mut arcInfosOnAShortestPath: HashSet<Ptr<AFInfo_ArcInfo>> = HashSet::new();
        for node in supercell.nodes::<FlippedNode<E, N, V>>() {
            for &supercellEdge in node.get_incoming().iter() {
                if supercellEdge.is_internal() {
                    continue;
                }
                if self.myNode2EdgeRouter.is_prohibited(supercellEdge, vehicle, utils::common::time::steps2time(msTime)) {
                    continue;
                }
                arcInfosOnAShortestPath.insert(self.myArcInfos[FlippedEdge::get_numerical_id(&*supercellEdge) as usize]);
            }
        }
        let vClass = if vehicle.is_null() { SVC_IGNORING } else { vehicle.v_class() };
        let mut survivors: Vec<Ptr<AFInfo_ArcInfo>> = Vec::new();
        'outer: for arcInfo in arcInfosOnAShortestPath.iter().copied() {
            let edge = arcInfo.edge.cast::<FlippedEdge<E, N, V>>();
            debug_assert!(!edge.is_internal());
            let numberOfBoundaryNodes = arcInfo.effortsToBoundaryNodes.len();
            let mut onShortestPath = false;
            'indices: for index in 0..numberOfBoundaryNodes {
                let effort1 = arcInfo.effortsToBoundaryNodes[index];
                if effort1 == UNREACHABLE || effort1 == f64::MAX {
                    continue 'indices;
                }
                let mut sTime = utils::common::time::steps2time(msTime);
                let mut edgeEffort = self.myNode2EdgeRouter.get_effort(edge, vehicle, sTime);
                sTime += self.myNode2EdgeRouter.get_travel_time(edge, vehicle, sTime, edgeEffort);
                let oldEdgeEffort = edgeEffort;
                let oldSTime = sTime;
                for follower in edge.get_via_successors(vClass, false) {
                    debug_assert!(!follower.0.is_internal());
                    let followerInfo = self.myArcInfos[FlippedEdge::get_numerical_id(&*follower.0) as usize];
                    if self.myNode2EdgeRouter.is_prohibited(follower.0, vehicle, sTime) {
                        let vehicleId = if vehicle.is_null() { "NULL".to_string() } else { vehicle.get_id() };
                        let msg = format!("Vehicle '{}' is not allowed on source edge '{}'.", vehicleId, followerInfo.edge.cast::<FlippedEdge<E, N, V>>().id());
                        self.myErrorMsgHandler.m().inform(&msg, true);
                        continue;
                    }
                    if followerInfo.effortsToBoundaryNodes.is_empty() {
                        continue;
                    }
                    let effort2 = followerInfo.effortsToBoundaryNodes[index];
                    if effort2 == UNREACHABLE || effort2 == f64::MAX {
                        continue;
                    }
                    let mut length = 0.;
                    self.myNode2EdgeRouter.update_via_edge_cost(follower.1, vehicle, &mut sTime, &mut edgeEffort, &mut length);
                    if effort1 + edgeEffort <= effort2 + self.EPS && effort1 + edgeEffort >= effort2 - self.EPS {
                        onShortestPath = true;
                        break 'indices;
                    }
                    edgeEffort = oldEdgeEffort;
                    sTime = oldSTime;
                }
            }
            if !onShortestPath {
                continue 'outer; // not on a shortest path, erase it
            }
            survivors.push(arcInfo);
        }
        arcInfosOnAShortestPath = survivors.into_iter().collect();
        let edgesInsideCell = cell.edge_set::<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>(vehicle);
        for edgeInsideCell in edgesInsideCell.iter().copied() {
            let arcInfo = self.myArcInfos[FlippedEdge::get_numerical_id(&*edgeInsideCell) as usize];
            if self.myNode2EdgeRouter.is_prohibited(edgeInsideCell, vehicle, utils::common::time::steps2time(msTime)) {
                continue;
            }
            arcInfosOnAShortestPath.insert(arcInfo);
        }
        for arcInfo in arcInfosOnAShortestPath.iter().copied() {
            self.put_arc_flag(arcInfo, sHARCLevel, cell.is_left_or_lower_cell());
        }
    }

}

