//! Port of utils/router/AFBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `typename AFInfo<E>::FlagInfo AFBuilder::FlagInfo`
pub type AFBuilder_FlagInfo = AFInfo_FlagInfo;

/// C++ typedef `AbstractLookupTable<FlippedEdge<E, N, V>, V> AFBuilder::FlippedLookupTable`
pub type AFBuilder_FlippedLookupTable = Unmapped /* AbstractLookupTable<FlippedEdge<E, N, V>, V> */;

/// Builds arc flags for shortest path search with the arc flag router
/// C++ class `AFBuilder` (utils/router/AFBuilder.h)
pub struct AFBuilder<E, N, V, M> {
    /// The edges
    pub myEdges: Ptr<i32>,
    /// The flipped (backward) edges
    pub myFlippedEdges: i32,
    /// The k-d tree partition of the backward graph with flipped edges
    pub myFlippedPartition: Ptr<KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>>,
    /// The flag informations
    pub myFlagInfos: i32,
    /// The arc flag build
    pub myArcFlagBuild: Ptr<AFBuild<E, N, V, M>>,
    /// The number of levels of the k-d tree partition of the network
    pub myNumberOfLevels: i32,
    /// The number of arc flags per each edge
    pub myNumberOfArcFlags: i32,
    pub myAmClean: bool,
    _p: std::marker::PhantomData<(E, N, V, M,)>,
}

impl<E, N, V, M> AFBuilder<E, N, V, M> {
    /// C++: `void AFBuilder::AFBuilder<E, N, V, M>(int, const int &, bool, typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation, const std::shared_ptr<const FlippedLookupTable>, const bool, const bool, const int *)`
    /// Constructor
    // AFBuilder(int numberOfLevels, const std::vector<E*>& edges, bool unbuildIsWarning,
    //               typename SUMOAbstractRouter<FlippedEdge<E, N, V>, V>::Operation flippedOperation,
    //               const std::shared_ptr<const FlippedLookupTable> flippedLookup = nullptr,
    //               const bool havePermissions = false, const bool haveRestrictions = false,
    //               const std::map<const FlippedEdge<E, N, V>*, RouterProhibition>* toProhibit = nullptr) :
    //         myEdges(edges),
    //         myNumberOfLevels(numberOfLevels),
    //         myNumberOfArcFlags(2 * (myNumberOfLevels - 1)),
    // #ifdef AFBL_DEBUG_LEVEL_0
    //         myArcFlagsFileName("arcflags.csv"),
    // #endif
    //         myAmClean(true) {
    //         for (const E* const edge : edges) {
    //             myFlagInfos.push_back(new FlagInfo(edge));
    //         }
    //         // build the backward graph with flipped edges / nodes in advance
    // #ifdef AFBL_DEBUG_LEVEL_0
    //         std::cout << "Building flipped edges (" << edges.size() << " edges) / nodes..." << std::endl;
    // #endif
    //         for (const E* const edge : edges) {
    //             myFlippedEdges.push_back(edge->getFlippedRoutingEdge());
    //         }
    //         for (FlippedEdge<E, N, V>* flippedEdge : myFlippedEdges) {
    //             flippedEdge->init();
    //         }
    // #ifdef AFBL_DEBUG_LEVEL_0
    //         std::cout << "Flipped edges / nodes are ready." << std::endl;
    // #endif
    //         myFlippedPartition = new KDTreePartition<FlippedEdge<E, N, V>, FlippedNode<E, N, V>, V>(myNumberOfLevels,
    //                 myFlippedEdges, havePermissions, haveRestrictions);
    // #ifdef AFBL_DEBUG_LEVEL_0
    //         std::cout << "Instantiating arc flag build..." << std::endl;
    // #endif
    //         myArcFlagBuild = new AFBuild<E, N, V, M>(myFlippedEdges, myFlippedPartition, numberOfLevels, unbuildIsWarning,
    //                 flippedOperation, flippedLookup, havePermissions, haveRestrictions, toProhibit);
    //
    // #ifdef AFBL_DEBUG_LEVEL_0
    //         std::cout << "Arc flag build is instantiated (but still uninitialized)." << std::endl;
    // #endif
    //     }
    #[allow(unused_variables)]
    pub fn new(numberOfLevels: i32, edges: &i32, unbuildIsWarning: bool, flippedOperation: SUMOAbstractRouter_Operation, flippedLookup: Rc<AFBuild_FlippedLookupTable>, havePermissions: bool, haveRestrictions: bool, toProhibit: Ptr<i32>) -> Self {
        todo!("port AFBuilder::AFBuilder<E, N, V, M>")
    }

    /// C++: `void AFBuilder::~AFBuilder<E, N, V, M>()`
    /// Destructor
    // template<class E, class N, class V, class M>
    // AFBuilder<E, N, V, M>::~AFBuilder() {
    //     delete myArcFlagBuild;
    //     delete myFlippedPartition;
    //     for (FlagInfo* flagInfo : myFlagInfos) {
    //         delete flagInfo;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port AFBuilder::~AFBuilder<E, N, V, M>")
    }

    /// C++: `AFBuild<E, N, V, M> * AFBuilder::getArcFlagBuild()`
    /// Returns the arc flag build
    // AFBuild<E, N, V, M>* getArcFlagBuild() {
    //         return myArcFlagBuild;
    //     }
    #[allow(unused_variables)]
    pub fn get_arc_flag_build(&mut self) -> Ptr<AFBuild<E, N, V, M>> {
        todo!("port AFBuilder::getArcFlagBuild")
    }

    /// C++: `const int & AFBuilder::getEdges()`
    /// Returns the edges
    // const std::vector<E*>& getEdges() {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&mut self) -> &i32 {
        todo!("port AFBuilder::getEdges")
    }

    /// C++: `void AFBuilder::reset()`
    /// Resets the builder
    // template<class E, class N, class V, class M>
    // void AFBuilder<E, N, V, M>::reset() {
    //     for (FlagInfo* flagInfo : myFlagInfos) {
    //         flagInfo->reset();
    //     }
    //     myAmClean = true;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port AFBuilder::reset")
    }

    /// C++: `int & AFBuilder::build(SUMOTime, const V *const)`
    /// Build the arc flag information for the arc flag router
    // template<class E, class N, class V, class M>
    // std::vector<typename AFInfo<E>::FlagInfo*>& AFBuilder<E, N, V, M>::build(SUMOTime msTime, const V* const vehicle) {
    //     if (!myAmClean) {
    //         reset();
    //     }
    //     assert(myFlippedPartition);
    //     if (myFlippedPartition->isClean()) {
    //         myFlippedPartition->init(vehicle);
    //         myArcFlagBuild->setFlippedPartition(myFlippedPartition);
    //     } else {
    //         myFlippedPartition->reset(vehicle);
    //     }
    //     assert(myArcFlagBuild);
    // #ifdef AFBL_DEBUG_LEVEL_0
    //     bool fileExists = this->fileExists(myArcFlagsFileName);
    //     if (fileExists && myAmClean) {
    //         std::cout << "Loading arc flags from file " << myArcFlagsFileName << std::endl;
    //         loadFlagsFromCsv(myArcFlagsFileName);
    //         std::cout << "Arc flags loaded." << std::endl;
    //     } else {
    // #endif
    //         myArcFlagBuild->init(msTime, vehicle, myFlagInfos);
    // #ifdef AFBL_DEBUG_LEVEL_0
    //     }
    // #endif
    //     delete myFlippedPartition;
    //     myFlippedPartition = nullptr;
    //
    // #ifdef AFBL_DEBUG_LEVEL_0
    //     if (!fileExists) {
    //         std::cout << "Saving arc flags..." << std::endl;
    //         // save flag vectors in a CSV file (one column, flag vectors in the order of edges)
    //         saveFlagsToCsv(myArcFlagsFileName);
    //         std::cout << "Arc flags have been saved." << std::endl;
    //     }
    // #endif
    //     myAmClean = false;
    //     return myFlagInfos;
    // }
    #[allow(unused_variables)]
    pub fn build(&mut self, msTime: SUMOTime, vehicle: Ptr<V>) -> &mut i32 {
        todo!("port AFBuilder::build")
    }

    /// C++: `int AFBuilder::sHARCLevel2PartitionLevel(int)`
    /// Converts a SHARC level number to a partition level number
    // int sHARCLevel2PartitionLevel(int sHARCLevel) {
    //         return AFRouter<E, N, V, M>::sHARCLevel2PartitionLevel(sHARCLevel, myNumberOfLevels);
    //     }
    #[allow(unused_variables)]
    pub fn s_harc_level2_partition_level(&mut self, sHARCLevel: i32) -> i32 {
        todo!("port AFBuilder::sHARCLevel2PartitionLevel")
    }

}

