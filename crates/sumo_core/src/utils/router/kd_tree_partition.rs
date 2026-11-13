//! Port of utils/router/KDTreePartition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};
/// C++ `delete p` for a pointer created with Box::into_raw; deleting nullptr is a no-op
fn delete_ptr<T>(p: Ptr<T>) {
    if !p.is_null() {
        unsafe { drop(Box::from_raw(p.as_raw())) };
    }
}
// C++ function-local `static int cellCounter`
static mut CELL_COUNTER: i32 = 0;

/// C++ enum `Axis`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum Axis {
    X = 0,
    Y = 1,
}

/// Represents an element of the node partition (i.e. a node set)
/// C++ class `KDTreePartition::Cell` (utils/router/KDTreePartition.h)
pub struct KDTreePartition_Cell {
    /// The cells
    pub myCells: Ptr<i32>,
    /// The cells of all partitions at all levels of the k-d tree subdivisional scheme
    pub myLevelCells: Ptr<i32>,
    /// The container with all nodes, sorted wrt to the k-d tree subdivisional scheme
    pub mySortedNodes: Ptr<i32>,
    /// The total number of levels of the k-d tree
    pub myNumberOfLevels: i32,
    /// The level
    pub myLevel: i32,
    /// The number
    pub myNumber: i32,
    /// The axis of the spatial extension
    pub myAxis: Axis,
    /// The from-index (inclusive)
    pub myFromInclusive: usize,
    /// The to-index (exclusive)
    pub myToExclusive: usize,
    /// Those nodes on the cell boundary, which are from-nodes of outgoing boundary edges
    pub myBoundaryFromNodes: i32,
    /// The incoming edges on the cell boundary
    pub myIncomingBoundaryEdges: i32,
    pub myOutgoingBoundaryEdges: i32,
    /// The super cell
    pub mySupercell: Ptr<KDTreePartition_Cell>,
    /// The left (cell's axis is X) or lower (cell's axis is Y) subcell
    pub myLeftOrLowerSubcell: Ptr<KDTreePartition_Cell>,
    /// The right (cell's axis is X) or upper (cell's axis is Y) subcell
    pub myRightOrUpperSubcell: Ptr<KDTreePartition_Cell>,
    /// The minimum x-value of a node in the cell
    pub myMinX: f64,
    /// The maximum x-value of a node in the cell
    pub myMaxX: f64,
    /// The minimum y-value of a node in the cell
    pub myMinY: f64,
    /// The maximum y-value of a node in the cell
    pub myMaxY: f64,
    /// The boolean flag indicating whether the information about the cell's spatial extent is complete or not
    pub myHasCompleteSpatialInfo: bool,
    /// The boolean flag indicating whether the cell's nodes are sorted wrt to the cell's axis or not
    pub myHasNodesSortedWrtToMyAxis: bool,
    /// The boolean flag indicating whether this cell is a left/lower cell or not
    pub myIsLeftOrLowerCell: bool,
    /// The boolean flag indicating whether edge permissions need to be considered or not
    pub myHavePermissions: bool,
    /// The boolean flag indicating whether edge restrictions need to be considered or not
    pub myHaveRestrictions: bool,
    /// The coordinate in the axis' direction of the node at the median index
    pub myMedianCoordinate: f64,
}

impl KDTreePartition_Cell {
    /// C++: `void Cell::Cell(int *, int *, int *, int, int, Axis, size_t, size_t, Cell *, double, double, double, double, bool, const V *const, const bool, const bool)`
    /// Constructor
    // template<class E, class N, class V>
    // #ifdef KDTP_FOR_SYNTHETIC_NETWORKS
    // KDTreePartition<E, N, V>::Cell::Cell(std::vector<const Cell*>* cells, std::vector<std::vector<const Cell*>>* levelCells, std::vector<const N*>* sortedNodes, int numberOfLevels, int level,
    //                                      Axis axis, size_t fromInclusive, size_t toExclusive, Cell* supercell, double minX, double maxX, double minY, double maxY, std::unordered_set<const N*>* northernConflictNodes,
    //                                      std::unordered_set<const N*>* easternConflictNodes, std::unordered_set<const N*>* southernConflictNodes, std::unordered_set<const N*>* westernConflictNodes, const V* const vehicle,
    //                                      const bool havePermissions, const bool haveRestrictions) :
    //     myCells(cells),
    //     myLevelCells(levelCells),
    //     mySortedNodes(sortedNodes),
    //     myNumberOfLevels(numberOfLevels),
    //     myLevel(level), myNumber(cellCounter()++),
    //     myAxis(axis),
    //     myFromInclusive(fromInclusive),
    //     myToExclusive(toExclusive),
    //     mySupercell(supercell),
    //     myMinX(minX),
    //     myMaxX(maxX),
    //     myMinY(minY),
    //     myMaxY(maxY),
    //     myNorthernConflictNodes(northernConflictNodes),
    //     myEasternConflictNodes(easternConflictNodes),
    //     mySouthernConflictNodes(southernConflictNodes),
    //     myWesternConflictNodes(westernConflictNodes),
    //     myIsLeftOrLowerCell(isLeftOrLowerCell),
    //     myHavePermissions(havePermissions),
    //     myHaveRestrictions(haveRestrictions),
    //     myMedianCoordinate(-1.) {
    // #else
    // KDTreePartition<E, N, V>::Cell::Cell(std::vector<const Cell*>* cells, std::vector<std::vector<const Cell*>>* levelCells, std::vector<const N*>* sortedNodes, int numberOfLevels, int level,
    //                                      Axis axis, size_t fromInclusive, size_t toExclusive, Cell* supercell, double minX, double maxX, double minY, double maxY, bool isLeftOrLowerCell, const V* const vehicle,
    //                                      const bool havePermissions, const bool haveRestrictions) :
    //     myCells(cells),
    //     myLevelCells(levelCells),
    //     mySortedNodes(sortedNodes),
    //     myNumberOfLevels(numberOfLevels),
    //     myLevel(level),
    //     myNumber(cellCounter()++),
    //     myAxis(axis),
    //     myFromInclusive(fromInclusive),
    //     myToExclusive(toExclusive),
    //     mySupercell(supercell),
    //     myMinX(minX),
    //     myMaxX(maxX),
    //     myMinY(minY),
    //     myMaxY(maxY),
    //     myIsLeftOrLowerCell(isLeftOrLowerCell),
    //     myHavePermissions(havePermissions),
    //     myHaveRestrictions(haveRestrictions),
    //     myMedianCoordinate(-1.) {
    // #endif
    //     // throw invalid argument exception if fromInclusive is greater than or equal to toExclusive
    //     if (myFromInclusive >= myToExclusive) {
    //         throw std::invalid_argument("Cell::Cell: myFromInclusive greater than or equal to myToExclusive!");
    //     }
    //     myHasCompleteSpatialInfo = false;
    //     myHasNodesSortedWrtToMyAxis = false;
    //     (*myCells)[myNumber] = this;
    //     (*myLevelCells)[myLevel].push_back(this);
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Cell number: " << myNumber << ", cell's extent: minX=" << myMinX << ", maxX="
    //               << myMaxX << ", minY=" << myMinY << ", maxY=" << myMaxY << std::endl;
    //     std::vector<const N*>* cellNodes = nodes();
    //     int cnt = 0;
    //     for (const N* node : *cellNodes) {
    //         if (++cnt % 10000 == 0) {
    //             std::cout << "Testing node " << cnt << std::endl;
    //         }
    //         assert(contains(node));
    //     }
    //     delete cellNodes;
    // #endif
    //     typename std::vector<const N*>::const_iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::const_iterator last = mySortedNodes->begin() + myToExclusive;
    //     typename std::vector<const N*>::const_iterator iter;
    // #ifdef KDTP_WRITE_QGIS_FILTERS
    //     size_t numberOfNodes = myToExclusive - myFromInclusive;
    //     size_t k = 0;
    //     std::string qgisFilterString = "id IN (";
    //     // go through the nodes of the cell
    //     for (iter = first; iter != last; iter++) {
    //         k++;
    //         qgisFilterString += (*iter)->getID() + (k < numberOfNodes ? ", " : "");
    //     }
    //     qgisFilterString += ")";
    //     std::ostringstream pathAndFileName;
    //     pathAndFileName << "./filter_nodes_of_cell_" << myNumber << ".qqf";
    //     std::ofstream file;
    //     file.open(pathAndFileName.str());
    //     std::ostringstream content;
    //     content << "<Query>" << qgisFilterString << "</Query>";
    //     file << content.str();
    //     file.close();
    // #endif
    //     // compute the set of of edges inside the cell
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Computing the set of of edges inside the cell..." << std::endl;
    // #endif
    //     std::unordered_set<const E*>* edgeSet = this->edgeSet(vehicle);
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Done. Now collecting boundary edges..." << std::endl;
    //     int i = 0;
    // #endif
    //     /// go through the nodes of the cell in order to identify and collect boundary nodes / edges
    //     for (iter = first; iter != last; iter++) {
    // #if defined(KDTP_KEEP_BOUNDARY_EDGES) || defined(KDTP_KEEP_BOUNDARY_NODES) || defined(KDTP_WRITE_QGIS_FILTERS)
    //         std::unordered_set<const E*> nodeEdgeSet;
    // #endif
    // #ifdef KDTP_DEBUG_LEVEL_1
    //         if ((i++ % 10000) == 0) {
    //             std::cout << i << " cell nodes processed..." << std::endl;
    //         }
    // #endif
    // #if defined(KDTP_KEEP_BOUNDARY_EDGES) || defined(KDTP_KEEP_BOUNDARY_NODES) || defined(KDTP_WRITE_QGIS_FILTERS)
    //         const std::vector<const E*>& incomingEdges = (*iter)->getIncoming();
    //         for (const E* incomingEdge : incomingEdges) {
    //             if (isProhibited(incomingEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (incomingEdge->isInternal()) {
    //                 continue;
    //             }
    // #endif
    //             nodeEdgeSet.insert(incomingEdge);
    //         }
    //         const std::vector<const E*>& outgoingEdges = (*iter)->getOutgoing();
    //         for (const E* outgoingEdge : outgoingEdges) {
    //             if (isProhibited(outgoingEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (outgoingEdge->isInternal()) {
    //                 continue;
    //             }
    // #endif
    //             nodeEdgeSet.insert(outgoingEdge);
    //         }
    //         bool containsAll = true;
    //         for (const E* nodeEdge : nodeEdgeSet) {
    //             if (edgeSet->find(nodeEdge) == edgeSet->end()) {
    //                 containsAll = false;
    // #ifndef KDTP_KEEP_BOUNDARY_EDGES
    //                 break; // cancel at first element which is not found
    // #else
    //                 myBoundaryEdges.insert(nodeEdge); // boundary edge!
    // #endif
    //             }
    //         }
    // #if defined(KDTP_KEEP_BOUNDARY_NODES) || defined(KDTP_WRITE_QGIS_FILTERS)
    //         if (!containsAll) {
    //             myBoundaryNodes.push_back(*iter);
    //         }
    // #endif
    // #endif
    // #if defined(KDTP_KEEP_INCOMING_BOUNDARY_EDGES) || defined(KDTP_KEEP_BOUNDARY_TO_NODES)
    //         const std::vector<const E*>& incoming = (*iter)->getIncoming();
    //         std::unordered_set<const N*> boundaryToNodesSet;
    //         for (const E* nodeEdge : incoming) {
    //             if (isProhibited(nodeEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (nodeEdge->isInternal()) {
    //                 continue;
    //             }
    // #endif
    //             assert(nodeEdge->getToJunction() == *iter);
    //             if (edgeSet->find(nodeEdge) == edgeSet->end()) {
    // #ifdef KDTP_KEEP_INCOMING_BOUNDARY_EDGES
    //                 myIncomingBoundaryEdges.insert(nodeEdge); // incoming boundary edge
    // #endif
    // #ifdef KDTP_KEEP_BOUNDARY_TO_NODES
    //                 boundaryToNodesSet.insert(*iter); ; // boundary to-node!
    // #endif
    //             }
    //         }
    // #ifdef KDTP_KEEP_BOUNDARY_TO_NODES
    //         std::copy(boundaryToNodesSet.begin(), boundaryToNodesSet.end(),
    //                   std::back_inserter(myBoundaryToNodes));
    // #endif
    // #endif
    // #if defined(KDTP_KEEP_OUTGOING_BOUNDARY_EDGES) || defined(KDTP_KEEP_BOUNDARY_FROM_NODES)
    //         const std::vector<const E*>& outgoing = (*iter)->getOutgoing();
    //         std::unordered_set<const N*> boundaryFromNodesSet;
    //         for (const E* nodeEdge : outgoing) {
    //             if (isProhibited(nodeEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (nodeEdge->isInternal()) {
    //                 continue;
    //             }
    // #endif
    //             assert(nodeEdge->getFromJunction() == *iter);
    //             if (edgeSet->find(nodeEdge) == edgeSet->end()) {
    // #ifdef KDTP_KEEP_OUTGOING_BOUNDARY_EDGES
    //                 myOutgoingBoundaryEdges.insert(nodeEdge); // outgoing boundary edge
    // #endif
    // #ifdef KDTP_KEEP_BOUNDARY_FROM_NODES
    //                 boundaryFromNodesSet.insert(*iter); // boundary from-node
    // #endif
    //             }
    //         }
    // #ifdef KDTP_KEEP_BOUNDARY_FROM_NODES
    //         std::copy(boundaryFromNodesSet.begin(), boundaryFromNodesSet.end(),
    //                   std::back_inserter(myBoundaryFromNodes));
    // #endif
    // #endif
    //     }
    //     delete edgeSet;
    // #if defined(KDTP_KEEP_BOUNDARY_NODES) || defined(KDTP_WRITE_QGIS_FILTERS)
    //     size_t numberOfBoundaryNodes = myBoundaryNodes.size();
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Number of boundary nodes: " << numberOfBoundaryNodes << std::endl;
    // #endif
    // #endif
    // #ifdef KDTP_KEEP_BOUNDARY_FROM_NODES
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     size_t numberOfBoundaryFromNodes = myBoundaryFromNodes.size();
    //     std::cout << "Number of boundary from nodes: " << numberOfBoundaryFromNodes << std::endl;
    // #endif
    // #endif
    // #ifdef KDTP_KEEP_BOUNDARY_TO_NODES
    //     size_t numberOfBoundaryToNodes = myBoundaryToNodes.size();
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Number of boundary to nodes: " << numberOfBoundaryToNodes << std::endl;
    // #endif
    // #endif
    // #ifdef KDTP_KEEP_BOUNDARY_EDGES
    //     size_t numberOfBoundaryEdges = myBoundaryEdges.size();
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Number of boundary edges: " << numberOfBoundaryEdges << std::endl;
    // #endif
    // #endif
    // #ifdef KDTP_KEEP_INCOMING_BOUNDARY_EDGES
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     size_t numberOfIncomingBoundaryEdges = myIncomingBoundaryEdges.size();
    //     std::cout << "Number of incoming boundary edges: " << numberOfIncomingBoundaryEdges << std::endl;
    // #endif
    // #endif
    // #ifdef KDTP_KEEP_OUTGOING_BOUNDARY_EDGES
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     size_t numberOfOutgoingBoundaryEdges = myOutgoingBoundaryEdges.size();
    //     std::cout << "Number of outgoing boundary edges: " << numberOfOutgoingBoundaryEdges << std::endl;
    // #endif
    // #endif
    // #ifdef KDTP_WRITE_QGIS_FILTERS
    //     k = 0;
    //     qgisFilterString.clear();
    //     qgisFilterString = "id IN (";
    //     // go through the boundary nodes of the cell
    //     for (const N* node : myBoundaryNodes) {
    //         k++;
    //         qgisFilterString += node->getID() + (k < numberOfBoundaryNodes ? ", " : "");
    //     }
    // #ifndef KDTP_KEEP_BOUNDARY_NODES
    //     myBoundaryNodes.clear();
    // #endif
    //     qgisFilterString += ")";
    //     pathAndFileName.str("");
    //     pathAndFileName.clear();
    //     pathAndFileName << "./filter_boundary_nodes_of_cell_" << myNumber << ".qqf";
    //     file.clear();
    //     file.open(pathAndFileName.str());
    //     content.str("");
    //     content.clear();
    //     content << "<Query>" << qgisFilterString << "</Query>";
    //     file << content.str();
    //     file.close();
    // #endif
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Done. Now calling the constructor recursively to instantiate the subcells (left or lower one first)..." << std::endl;
    // #endif
    //     size_t medianIndex = partition();
    //     completeSpatialInfo();
    //     // create subcells
    //     if (myLevel < myNumberOfLevels - 1) {
    //         // determine min/max X/Y-values to pass on to left or lower subcell
    //         double passMinX = -1, passMaxX = -1, passMinY = -1, passMaxY = -1;
    //         if (myAxis == Axis::X) { // left subcell
    //             passMinX = myMinX;
    //             passMaxX = (*mySortedNodes)[medianIndex]->getPosition().x();
    //             passMinY = myMinY;
    //             passMaxY = myMaxY;
    //         } else { // lower subcell
    //             passMinX = myMinX;
    //             passMaxX = myMaxX;
    //             passMinY = myMinY;
    //             passMaxY = (*mySortedNodes)[medianIndex]->getPosition().y();
    //         }
    // #ifdef KDTP_FOR_SYNTHETIC_NETWORKS
    //         // notice that nodes with indexes medianIndex and medianIndex+1 may have the same coordinate in the direction of myAxis
    //         // (in that case, they are ordered / distributed between the two subcells with a tie-breaking rule)
    //         // consequently, the spatial extent of cells alone does not suffice to decide whether a node belongs to a certain cell or not
    //         // therefore, for each of the two subcells, we compute the set of nodes sharing said coordinate (called the set of "spatial conflict nodes")
    //         // the size of this set is usually very small compared to the size of the set of all nodes of the subcell
    //         // with the spatial extent of a cell and its spatial conflict nodes set, a valid test whether a node belongs to the cell becomes available
    //         // this test is quicker and requires significantly less memory than one based on pre-computed sets of all cell nodes
    //         std::pair<std::unordered_set<const N*>*, std::unordered_set<const N*>*> conflictNodes = spatialConflictNodes(medianIndex);
    //         // determine northern/eastern/southern/western conflict nodes to pass on to left or lower subcell
    //         std::unordered_set<const N*>* passNorthernConflictNodes;
    //         std::unordered_set<const N*>* passEasternConflictNodes;
    //         std::unordered_set<const N*>* passSouthernConflictNodes;
    //         std::unordered_set<const N*>* passWesternConflictNodes;
    //         if (myAxis == Axis::X) { // left subcell
    //             passNorthernConflictNodes = myNorthernConflictNodes;
    //             passEasternConflictNodes = conflictNodes.first;
    //             passSouthernConflictNodes = mySouthernConflictNodes;
    //             passWesternConflictNodes = myWesternConflictNodes;
    //         } else { // lower subcell
    //             passNorthernConflictNodes = conflictNodes.first;
    //             passEasternConflictNodes = myEasternConflictNodes;
    //             passSouthernConflictNodes = mySouthernConflictNodes;
    //             passWesternConflictNodes = myWesternConflictNodes;
    //         }
    //         myLeftOrLowerSubcell = new Cell(myCells, myLevelCells, mySortedNodes, myNumberOfLevels, myLevel + 1, KDTreePartition::flip(myAxis), myFromInclusive, medianIndex + 1, this,
    //                                         passMinX, passMaxX, passMinY, passMaxY, passNorthernConflictNodes, passEasternConflictNodes, passSouthernConflictNodes, passWesternConflictNodes, true,
    //                                         vehicle, myHavePermissions, myHaveRestrictions);
    // #else
    //         myLeftOrLowerSubcell = new Cell(myCells, myLevelCells, mySortedNodes, myNumberOfLevels, myLevel + 1, KDTreePartition::flip(myAxis), myFromInclusive, medianIndex + 1, this,
    //                                         passMinX, passMaxX, passMinY, passMaxY, true, vehicle, myHavePermissions, myHaveRestrictions);
    // #endif
    // #ifdef KDTP_DEBUG_LEVEL_1
    //         std::cout << "Left or lower call done. Now calling it for the right or upper subcell..." << std::endl;
    // #endif
    //         // determine min/max X/Y-values to pass on to right or upper subcell
    //         passMinX = -1, passMaxX = -1, passMinY = -1, passMaxY = -1;
    //         if (myAxis == Axis::X) { // right subcell
    //             passMinX = (*mySortedNodes)[medianIndex + 1]->getPosition().x();
    //             passMaxX = myMaxX;
    //             passMinY = myMinY;
    //             passMaxY = myMaxY;
    //         } else { // upper subcell
    //             passMinX = myMinX;
    //             passMaxX = myMaxX;
    //             passMinY = (*mySortedNodes)[medianIndex + 1]->getPosition().y();
    //             passMaxY = myMaxY;
    //         }
    // #ifdef KDTP_FOR_SYNTHETIC_NETWORKS
    //         // determine northern/eastern/southern/western conflict nodes to pass on to right or upper subcell
    //         if (myAxis == Axis::X) { // right subcell
    //             passNorthernConflictNodes = myNorthernConflictNodes;
    //             passEasternConflictNodes = myEasternConflictNodes;
    //             passSouthernConflictNodes = mySouthernConflictNodes;
    //             passWesternConflictNodes = conflictNodes.second;
    //         } else { // upper subcell
    //             passNorthernConflictNodes = myNorthernConflictNodes;
    //             passEasternConflictNodes = myEasternConflictNodes;
    //             passSouthernConflictNodes = conflictNodes.second;
    //             passWesternConflictNodes = myWesternConflictNodes;
    //         }
    //         myRightOrUpperSubcell = new Cell(myCells, myLevelCells, mySortedNodes, myNumberOfLevels, myLevel + 1, KDTreePartition::flip(myAxis), medianIndex + 1, myToExclusive, this,
    //                                          passMinX, passMaxX, passMinY, passMaxY, passNorthernConflictNodes, passEasternConflictNodes, passSouthernConflictNodes, passWesternConflictNodes, false,
    //                                          vehicle, myHavePermissions, myHaveRestrictions);
    // #else
    //         myRightOrUpperSubcell = new Cell(myCells, myLevelCells, mySortedNodes, myNumberOfLevels, myLevel + 1, KDTreePartition::flip(myAxis), medianIndex + 1, myToExclusive, this,
    //                                          passMinX, passMaxX, passMinY, passMaxY, false, vehicle, myHavePermissions, myHaveRestrictions);
    // #endif
    // #ifdef KDTP_DEBUG_LEVEL_1
    //         std::cout << "Right or upper call done. Returning from constructor..." << std::endl;
    // #endif
    //     } else {
    //         // leaves of the k-d tree: subcell pointers equal nullptr
    //         myLeftOrLowerSubcell = nullptr;
    //         myRightOrUpperSubcell = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(cells: Ptr<i32>, levelCells: Ptr<i32>, sortedNodes: Ptr<i32>, numberOfLevels: i32, level: i32, axis: Axis, fromInclusive: usize, toExclusive: usize, supercell: Ptr<KDTreePartition_Cell>, minX: f64, maxX: f64, minY: f64, maxY: f64, isLeftOrLowerCell: bool, vehicle: Ptr<Unmapped /* V */>, havePermissions: bool, haveRestrictions: bool) -> Self {
        // shortcut: Cell is not generic over N/E/V and mySortedNodes is an i32 placeholder, upgrade when the k-d tree storage is typed
        panic!("shortcut: Cell::new needs typed cell storage and N accessors")
    }

    /// C++: `void Cell::~Cell()` [virtual]
    /// Destructor
    // virtual ~Cell() {
    //             delete myLeftOrLowerSubcell;
    //             delete myRightOrUpperSubcell;
    //         }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        let left = self.myLeftOrLowerSubcell;
        if !left.is_null() {
            left.m().drop_();
            delete_ptr(left);
        }
        let right = self.myRightOrUpperSubcell;
        if !right.is_null() {
            right.m().drop_();
            delete_ptr(right);
        }
    }

    /// C++: `Axis Cell::getAxis() const` [const]
    /// Returns the axis of the cell's spatial extension (x or y)
    // Axis getAxis() const {
    //             return myAxis;
    //         }
    #[allow(unused_variables)]
    pub fn get_axis(&self) -> Axis {
        self.myAxis
    }

    /// C++: `int * Cell::edgeSet(const V *const) const` [const]
    /// Returns all edges situated inside the cell
    // template<class E, class N, class V>
    // std::unordered_set<const E*>* KDTreePartition<E, N, V>::Cell::edgeSet(const V* const vehicle) const {
    //     std::unordered_set<const E*>* edgeSet = new std::unordered_set<const E*>();
    //     typename std::vector<const N*>::const_iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::const_iterator last = mySortedNodes->begin() + myToExclusive;
    //     typename std::vector<const N*>::const_iterator iter;
    //     for (iter = first; iter != last; iter++) {
    //         const N* edgeNode;
    //         const std::vector<const E*>& incomingEdges = (*iter)->getIncoming();
    //         for (const E* incomingEdge : incomingEdges) {
    //             if (isProhibited(incomingEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (incomingEdge->isInternal()) {
    //                 continue;
    //             }
    // #endif
    //             edgeNode = incomingEdge->getFromJunction();
    //             if (contains(edgeNode)) {
    //                 edgeSet->insert(incomingEdge);
    //             }
    //         }
    //         const std::vector<const E*>& outgoingEdges = (*iter)->getOutgoing();
    //         for (const E* outgoingEdge : outgoingEdges) {
    //             if (isProhibited(outgoingEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (outgoingEdge->isInternal()) {
    //                 continue;
    //             }
    // #endif
    //             edgeNode = outgoingEdge->getToJunction();
    //             if (contains(edgeNode)) {
    //                 edgeSet->insert(outgoingEdge);
    //             }
    //         }
    //     }
    //     return edgeSet;
    // }
    #[allow(unused_variables)]
    pub fn edge_set(&self, vehicle: Ptr<Unmapped /* V */>) -> Ptr<i32> {
        // shortcut: E has no getFromJunction/getToJunction in RouterEdge and N has no getIncoming/getOutgoing, upgrade when those traits exist
        panic!("shortcut: Cell::edgeSet needs RouterEdge junctions and RouterNode edges")
    }

    /// C++: `size_t Cell::numberOfEdgesEndingInCell(const V *const) const` [const]
    /// Returns the number of edges ending in the cell
    // template<class E, class N, class V>
    // size_t KDTreePartition<E, N, V>::Cell::numberOfEdgesEndingInCell(const V* const vehicle) const {
    //     typename std::vector<const N*>::const_iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::const_iterator last = mySortedNodes->begin() + myToExclusive;
    //     typename std::vector<const N*>::const_iterator iter;
    //     size_t edgeCounter = 0;
    //     for (iter = first; iter != last; iter++) {
    //         const std::vector<const E*>& incomingEdges = (*iter)->getIncoming();
    //         for (const E* incomingEdge : incomingEdges) {
    //             if (isProhibited(incomingEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (incomingEdge->isInternal()) {
    //                 continue;
    //             } else {
    //                 edgeCounter++;
    //             }
    // #endif
    //         }
    //     }
    //     return edgeCounter;
    // }
    #[allow(unused_variables)]
    pub fn number_of_edges_ending_in_cell(&self, vehicle: Ptr<Unmapped /* V */>) -> usize {
        // shortcut: N has no getIncoming in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::numberOfEdgesEndingInCell needs RouterNode::incoming")
    }

    /// C++: `size_t Cell::numberOfEdgesStartingInCell(const V *const) const` [const]
    /// Returns the number of edges starting in the cell
    // template<class E, class N, class V>
    // size_t KDTreePartition<E, N, V>::Cell::numberOfEdgesStartingInCell(const V* const vehicle) const {
    //     typename std::vector<const N*>::const_iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::const_iterator last = mySortedNodes->begin() + myToExclusive;
    //     typename std::vector<const N*>::const_iterator iter;
    //     size_t edgeCounter = 0;
    //     for (iter = first; iter != last; iter++) {
    //         const std::vector<const E*>& outgoingEdges = (*iter)->getOutgoing();
    //         for (const E* outgoingEdge : outgoingEdges) {
    //             if (isProhibited(outgoingEdge, vehicle)) {
    //                 continue;
    //             }
    // #ifdef KDTP_EXCLUDE_INTERNAL_EDGES
    //             if (outgoingEdge->isInternal()) {
    //                 continue;
    //             } else {
    //                 edgeCounter++;
    //             }
    //         }
    // #endif
    //     }
    //     return edgeCounter;
    // }
    #[allow(unused_variables)]
    pub fn number_of_edges_starting_in_cell(&self, vehicle: Ptr<Unmapped /* V */>) -> usize {
        // shortcut: N has no getOutgoing in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::numberOfEdgesStartingInCell needs RouterNode::outgoing")
    }

    /// C++: `int Cell::getNumber() const` [const]
    /// Returns the cell's number
    // int getNumber() const {
    //             return myNumber;
    //         }
    #[allow(unused_variables)]
    pub fn get_number(&self) -> i32 {
        self.myNumber
    }

    /// C++: `int Cell::getLevel() const` [const]
    // int getLevel() const {
    //             return myLevel;
    //         }
    #[allow(unused_variables)]
    pub fn get_level(&self) -> i32 {
        self.myLevel
    }

    /// C++: `int Cell::getNumberOfCells() const` [const]
    /// Returns the number of cells
    // int getNumberOfCells() const {
    //             return cellCounter();
    //         }
    #[allow(unused_variables)]
    pub fn get_number_of_cells(&self) -> i32 {
        *Self::cell_counter()
    }

    /// C++: `int * Cell::nodes() const` [const]
    /// Returns a new vector of nodes in the cell
    // template<class E, class N, class V>
    // std::vector<const N*>* KDTreePartition<E, N, V>::Cell::nodes() const {
    //     typename std::vector<const N*>::const_iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::const_iterator last = mySortedNodes->begin() + myToExclusive;
    //     std::vector<const N*>* nodes = new std::vector<const N*>(first, last);
    //     return nodes;
    // }
    #[allow(unused_variables)]
    pub fn nodes(&self) -> Ptr<i32> {
        // shortcut: mySortedNodes is an i32 placeholder, upgrade when the k-d tree storage is typed as Vec<Ptr<N>>
        panic!("shortcut: Cell::nodes needs typed mySortedNodes")
    }

    /// C++: `const int & Cell::getBoundaryFromNodes() const` [const]
    /// Returns the vector of boundary nodes which are from-nodes of outgoing boundary edges
    // const std::vector<const N*>& getBoundaryFromNodes() const {
    //             return myBoundaryFromNodes;
    //         }
    #[allow(unused_variables)]
    pub fn get_boundary_from_nodes(&self) -> &i32 {
        &self.myBoundaryFromNodes
    }

    /// C++: `const int & Cell::getIncomingBoundaryEdges() const` [const]
    /// Returns the set of incoming boundary edges
    // const std::unordered_set<const E*>& getIncomingBoundaryEdges() const {
    //             return myIncomingBoundaryEdges;
    //         }
    #[allow(unused_variables)]
    pub fn get_incoming_boundary_edges(&self) -> &i32 {
        &self.myIncomingBoundaryEdges
    }

    /// C++: `const int & Cell::getOutgoingBoundaryEdges() const` [const]
    /// Returns the set of outgoing boundary edges
    // const std::unordered_set<const E*>& getOutgoingBoundaryEdges() const {
    //             return myOutgoingBoundaryEdges;
    //         }
    #[allow(unused_variables)]
    pub fn get_outgoing_boundary_edges(&self) -> &i32 {
        &self.myOutgoingBoundaryEdges
    }

    /// C++: `const Cell * Cell::getLeftOrLowerSubcell() const` [const]
    /// Returns the left (cell's axis is X) or lower (cell's axis is Y) subcell
    // const Cell* getLeftOrLowerSubcell() const {
    //             return myLeftOrLowerSubcell;
    //         }
    #[allow(unused_variables)]
    pub fn get_left_or_lower_subcell(&self) -> Ptr<KDTreePartition_Cell> {
        self.myLeftOrLowerSubcell
    }

    /// C++: `const Cell * Cell::getRightOrUpperSubcell() const` [const]
    /// Returns the right (cell's axis is X) or upper (cell's axis is Y) subcell
    // const Cell* getRightOrUpperSubcell() const {
    //             return myRightOrUpperSubcell;
    //         }
    #[allow(unused_variables)]
    pub fn get_right_or_upper_subcell(&self) -> Ptr<KDTreePartition_Cell> {
        self.myRightOrUpperSubcell
    }

    /// C++: `const Cell * Cell::getSupercell() const` [const]
    /// Returns the supercell
    // const Cell* getSupercell() const {
    //             return mySupercell;
    //         }
    #[allow(unused_variables)]
    pub fn get_supercell(&self) -> Ptr<KDTreePartition_Cell> {
        self.mySupercell
    }

    /// C++: `double Cell::getMinX() const` [const]
    /// Returns the minimum coordinate of a cell node in X-direction (aka left border coordinate)
    // double getMinX() const {
    //             return myMinX;
    //         }
    #[allow(unused_variables)]
    pub fn get_min_x(&self) -> f64 {
        self.myMinX
    }

    /// C++: `double Cell::getMaxX() const` [const]
    /// Returns the maximum coordinate of a cell node in X-direction (aka right border coordinate)
    // double getMaxX() const {
    //             return myMaxX;
    //         }
    #[allow(unused_variables)]
    pub fn get_max_x(&self) -> f64 {
        self.myMaxX
    }

    /// C++: `double Cell::getMinY() const` [const]
    /// Returns the minimum coordinate of a cell node in Y-direction (aka lower border coordinate)
    // double getMinY() const {
    //             return myMinY;
    //         }
    #[allow(unused_variables)]
    pub fn get_min_y(&self) -> f64 {
        self.myMinY
    }

    /// C++: `double Cell::getMaxY() const` [const]
    /// Returns the maximum coordinate of a cell node in Y-direction (aka upper border coordinate)
    // double getMaxY() const {
    //             return myMaxY;
    //         }
    #[allow(unused_variables)]
    pub fn get_max_y(&self) -> f64 {
        self.myMaxY
    }

    /// C++: `bool Cell::contains(const N *) const` [const]
    /// Tests whether the given node belongs to the cell
    // template<class E, class N, class V>
    // bool KDTreePartition<E, N, V>::Cell::contains(const N* node) const {
    //     if (myLevel == 0) { // p.d., the root cell contains all nodes
    //         return true;
    //     }
    //     if (!isInBounds(node)) {
    // #ifdef KDTP_DEBUG_LEVEL_2
    //         std::cout << "Not in bounds, nX: " << node->getPosition().x() << ", nY: " << node->getPosition().y() << std::endl;
    // #endif
    //         return false;
    //     }
    // #ifdef KDTP_DEBUG_LEVEL_2
    //     else {
    //         std::cout << "Node is in bounds!" << std::endl;
    //     }
    // #endif
    // #ifdef KDTP_FOR_SYNTHETIC_NETWORKS
    //     //
    //     double nodeX = node->getPosition().x();
    //     double nodeY = node->getPosition().y();
    // #ifdef KDTP_DEBUG_LEVEL_2
    //     std::cout << "Entered contains, nodeX: " << nodeX << ", nodeY: " << nodeY << std::endl;
    // #endif
    //     double northY = -1;
    //     if (myNorthernConflictNodes  && !myNorthernConflictNodes->empty()) {
    //         northY = (*(myNorthernConflictNodes->begin()))->getPosition().y();
    //     }
    //     double eastX = -1;
    //     if (myEasternConflictNodes && !myEasternConflictNodes->empty()) {
    //         eastX = (*(myEasternConflictNodes->begin()))->getPosition().x();
    //     }
    //     double southY = -1;
    //     if (mySouthernConflictNodes && !mySouthernConflictNodes->empty()) {
    //         southY = (*(mySouthernConflictNodes->begin()))->getPosition().y();
    //     }
    //     double westX = -1;
    //     if (myWesternConflictNodes && !myWesternConflictNodes->empty()) {
    //         westX = (*(myWesternConflictNodes->begin()))->getPosition().x();
    //     }
    // #ifdef KDTP_DEBUG_LEVEL_2
    //     std::cout << "northY: " << northY << ", eastX: " << eastX << ", southY: " << southY << ", westX: " << westX << std::endl;
    // #endif
    //     if (nodeX == eastX) { // we have to test the eastern conflict nodes
    // #ifdef KDTP_DEBUG_LEVEL_2
    //         std::cout << "On eastern bound, nX: " << node->getPosition().x() << ", nY: " << node->getPosition().y() << std::endl;
    // #endif
    //         return myEasternConflictNodes->find(node) != myEasternConflictNodes->end();
    //     }
    //     if (nodeX == westX) { // we have to test the western conflict nodes
    // #ifdef KDTP_DEBUG_LEVEL_2
    //         std::cout << "On western bound, nX: " << node->getPosition().x() << ", nY: " << node->getPosition().y() << std::endl;
    // #endif
    //         return myWesternConflictNodes->find(node) != myWesternConflictNodes->end();
    //     }
    //     if (nodeY == northY) { // we have to test the northern conflict nodes
    // #ifdef KDTP_DEBUG_LEVEL_2
    //         std::cout << "On northern bound, nX: " << node->getPosition().x() << ", nY: " << node->getPosition().y() << std::endl;
    // #endif
    //         return myNorthernConflictNodes->find(node) != myNorthernConflictNodes->end();
    //     }
    //     if (nodeY == southY) { // we have to test the southern conflict nodes
    // #ifdef KDTP_DEBUG_LEVEL_2
    //         std::cout << "On southern bound, nX: " << node->getPosition().x() << ", nY: " << node->getPosition().y() << std::endl;
    // #endif
    //         return mySouthernConflictNodes->find(node) != myEasternConflictNodes->end();
    //     }
    // #endif
    // #ifdef KDTP_DEBUG_LEVEL_2
    //     std::cout << "Node does not lie on a border!" << std::endl;
    // #endif
    //     return true; // if the node does not lie on a spatial border, the passed in-bounds test suffices
    // }
    #[allow(unused_variables)]
    pub fn contains(&self, node: Ptr<Unmapped /* N */>) -> bool {
        // shortcut: N has no getPosition in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::contains needs RouterNode::position")
    }

    /// C++: `bool Cell::isLeftOrLowerCell() const` [const]
    /// Returns the boolean flag indicating whether this cell is a left or lower cell or not
    // bool isLeftOrLowerCell() const {
    //             return myIsLeftOrLowerCell;
    //         }
    #[allow(unused_variables)]
    pub fn is_left_or_lower_cell(&self) -> bool {
        self.myIsLeftOrLowerCell
    }

    /// C++: `double Cell::getMedianCoordinate() const` [const]
    /// Returns the median coordinate
    // double getMedianCoordinate() const {
    //             return myMedianCoordinate;
    //         }
    #[allow(unused_variables)]
    pub fn get_median_coordinate(&self) -> f64 {
        self.myMedianCoordinate
    }

    /// C++: `size_t Cell::partition()` [private]
    /// Performs one partition step on the set of nodes sorted wrt to the k-d tree subdivision scheme
    // template<class E, class N, class V>
    // size_t KDTreePartition<E, N, V>::Cell::partition() {
    //     typename std::vector<const N*>::iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::iterator last = mySortedNodes->begin() + myToExclusive;
    //     if (myAxis == Axis::Y) {
    //         std::sort(first, last, NodeYComparator());
    //     } else {
    //         std::sort(first, last, NodeXComparator());
    //     }
    //     myHasNodesSortedWrtToMyAxis = true;
    //     if (mySupercell) {
    //         // sorting at one level destroys order at higher levels (but preserves the median split property)
    //         mySupercell->myHasNodesSortedWrtToMyAxis = false;
    //     }
    //     size_t length = myToExclusive - myFromInclusive;
    //     size_t medianIndex = myFromInclusive + (length % 2 == 0 ? length / 2 - 1 : (length + 1) / 2 - 1);
    // #ifndef KDTP_FOR_SYNTHETIC_NETWORKS
    //     // notice that nodes with indexes medianIndex and medianIndex+1 may have the same coordinate in the direction of myAxis -
    //     // in that case, they would be ordered and distributed between the two subcells with a tie-breaking rule,
    //     // and the spatial extent of cells alone would not suffice to decide whether a node belongs to a certain cell or not
    //     // for real-world road networks, this should occur very rarely
    //     // therefore, a simple but perfectly acceptable remedy is to shift the median index until the nodes with indexes
    //     // medianIndex and medianIndex+1 have different coordinates
    //     // this will distort the 50%/50% distribution of nodes just a little bit (and only under very rare circumstances)
    //     // hence we can assume zero impact on performance
    //     // note that this may happen more frequently for e.g. synthetic networks placing nodes at a regular, constant grid.
    //     // for this case, a different solution is provided, see code within #ifdef KDTP_FOR_SYNTHETIC_NETWORKS ... #endif
    //     const N* medianNode = (*mySortedNodes)[medianIndex];
    //     const N* afterMedianNode = (*mySortedNodes)[medianIndex + 1];
    //     double medianNodeCoordinate = myAxis == Axis::X ? medianNode->getPosition().x()
    //                                   : medianNode->getPosition().y();
    //     double afterMedianNodeCoordinate = myAxis == Axis::X ? afterMedianNode->getPosition().x()
    //                                        : afterMedianNode->getPosition().y();
    //     while (medianNodeCoordinate == afterMedianNodeCoordinate && medianIndex < myToExclusive - 3) {
    // #ifdef KDTP_DEBUG_LEVEL_2
    //         std::cout << "Found spatially conflicting nodes." << std::endl;
    // #endif
    //         medianIndex++;
    //         medianNode = (*mySortedNodes)[medianIndex];
    //         afterMedianNode = (*mySortedNodes)[medianIndex + 1];
    //         medianNodeCoordinate = myAxis == Axis::X ? medianNode->getPosition().x()
    //                                : medianNode->getPosition().y();
    //         afterMedianNodeCoordinate = myAxis == Axis::X ? afterMedianNode->getPosition().x()
    //                                     : afterMedianNode->getPosition().y();
    //     }
    // #endif
    //     myMedianCoordinate = medianNodeCoordinate;
    //     return medianIndex;
    // }
    #[allow(unused_variables)]
    pub fn partition(&mut self) -> usize {
        // shortcut: N has no getPosition in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::partition needs RouterNode::position")
    }

    /// C++: `int & Cell::cellCounter()` [static, private]
    /// Returns the global cell counter
    // static int& cellCounter() {
    //             static int cellCounter{ 0 };
    //             return cellCounter;
    //         }
    #[allow(unused_variables)]
    pub fn cell_counter() -> &'static mut i32 {
        // shortcut: the C++ function-local static is ported as the module-level CELL_COUNTER
        unsafe { &mut *std::ptr::addr_of_mut!(CELL_COUNTER) }
    }

    /// C++: `bool Cell::isProhibited(const E *const, const V *const) const` [const, private]
    /// Returns true iff driving the given vehicle on the given edge is prohibited
    // bool isProhibited(const E* const edge, const V* const vehicle) const {
    //             return (myHavePermissions && edge->prohibits(vehicle)) || (myHaveRestrictions && edge->restricts(vehicle));
    //         }
    #[allow(unused_variables)]
    pub fn is_prohibited<E: RouterEdge<V>, V: RouterVehicle>(&self, edge: Ptr<E>, vehicle: Ptr<V>) -> bool {
        // C++ `edge->prohibits(vehicle)` takes the edge type's default checkRestrictions (false for ROEdge)
        (self.myHavePermissions && edge.m().prohibits(vehicle, false)) || (self.myHaveRestrictions && edge.m().restricts(vehicle))
    }

    /// C++: `bool Cell::isInBounds(const N *) const` [const, private]
    /// Tests whether a given node is situated within the spatial bounds of the cell
    // template<class E, class N, class V>
    // bool KDTreePartition<E, N, V>::Cell::isInBounds(const N* node) const {
    //     double nodeX = node->getPosition().x();
    //     double nodeY = node->getPosition().y();
    //     if (nodeX < myMinX || nodeX > myMaxX || nodeY < myMinY || nodeY > myMaxY) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_in_bounds(&self, node: Ptr<Unmapped /* N */>) -> bool {
        // shortcut: N has no getPosition in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::isInBounds needs RouterNode::position")
    }

    /// C++: `void Cell::completeSpatialInfo()` [private]
    /// Completes the information about the spatial extent
    // template<class E, class N, class V>
    // void KDTreePartition<E, N, V>::Cell::completeSpatialInfo() {
    //     // complete missing information about the cell's spatial extent
    //     if (myMinX < 0) {
    //         myMinX = minAxisValue(Axis::X);
    //     }
    //     if (myMaxX < 0) {
    //         myMaxX = maxAxisValue(Axis::X);
    //     }
    //     if (myMinY < 0) {
    //         myMinY = minAxisValue(Axis::Y);
    //     }
    //     if (myMaxY < 0) {
    //         myMaxY = maxAxisValue(Axis::Y);
    //     }
    //     myHasCompleteSpatialInfo = true;
    // }
    #[allow(unused_variables)]
    pub fn complete_spatial_info(&mut self) {
        // shortcut: depends on minAxisValue/maxAxisValue, which need RouterNode::position, upgrade when RouterNode is added
        panic!("shortcut: Cell::completeSpatialInfo needs RouterNode::position")
    }

    /// C++: `double Cell::minAxisValue(Axis) const` [const, private]
    /// Returns the minimum coordinate of the cell nodes' positions, in the direction of the given axis (X or Y)
    // template<class E, class N, class V>
    // double KDTreePartition<E, N, V>::Cell::minAxisValue(Axis axis) const {
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     double returnValue = -1;
    // #endif
    //     if (myHasNodesSortedWrtToMyAxis && axis == myAxis) {
    // #ifndef KDTP_DEBUG_LEVEL_1
    //         return (myAxis == Axis::Y ? (*mySortedNodes)[myFromInclusive]->getPosition().y()
    //                 : (*mySortedNodes)[myFromInclusive]->getPosition().x());
    // #else
    //         returnValue = (myAxis == Axis::Y ? (*mySortedNodes)[myFromInclusive]->getPosition().y()
    //                        : (*mySortedNodes)[myFromInclusive]->getPosition().x());
    // #endif
    //     }
    //     double minAxisValue = std::numeric_limits<double>::max();
    //     typename std::vector<const N*>::const_iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::const_iterator last = mySortedNodes->begin() + myToExclusive;
    //     typename std::vector<const N*>::const_iterator iter;
    //     // go through the nodes of the cell
    //     for (iter = first; iter != last; iter++) {
    //         double nodeAxisValue;
    //         nodeAxisValue = (axis == Axis::Y ? (*iter)->getPosition().y() : (*iter)->getPosition().x());
    //         if (nodeAxisValue < minAxisValue) {
    //             minAxisValue = nodeAxisValue;
    //         }
    //     }
    //     assert(minAxisValue < std::numeric_limits<double>::max());
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     assert((!myHasNodesSortedWrtToMyAxis || axis != myAxis) || minAxisValue == returnValue);
    // #endif
    //     return minAxisValue;
    // }
    #[allow(unused_variables)]
    pub fn min_axis_value(&self, axis: Axis) -> f64 {
        // shortcut: N has no getPosition in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::minAxisValue needs RouterNode::position")
    }

    /// C++: `double Cell::minAxisValue() const` [const, private]
    /// Returns the minimum coordinate of the cell nodes' positions, in the direction of the cell's axis (X or Y)
    // template<class E, class N, class V>
    // double KDTreePartition<E, N, V>::Cell::minAxisValue() const {
    //     return minAxisValue(myAxis);
    // }
    #[allow(unused_variables)]
    pub fn min_axis_value_2(&self) -> f64 {
        // shortcut: N has no getPosition in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::minAxisValue needs RouterNode::position")
    }

    /// C++: `double Cell::maxAxisValue(Axis) const` [const, private]
    /// Returns the maximum coordinate of the cell nodes' positions, in the direction of the given axis (X or Y)
    // template<class E, class N, class V>
    // double KDTreePartition<E, N, V>::Cell::maxAxisValue(Axis axis) const {
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     double returnValue = -1;
    // #endif
    //     if (myHasNodesSortedWrtToMyAxis && axis == myAxis) {
    // #ifndef KDTP_DEBUG_LEVEL_1
    //         return (myAxis == Axis::Y ? (*mySortedNodes)[myToExclusive - 1]->getPosition().y()
    //                 : (*mySortedNodes)[myToExclusive - 1]->getPosition().x());
    // #else
    //         returnValue = (myAxis == Axis::Y ? (*mySortedNodes)[myToExclusive - 1]->getPosition().y()
    //                        : (*mySortedNodes)[myToExclusive - 1]->getPosition().x());
    // #endif
    //
    //     }
    //     double maxAxisValue = -1;
    //     typename std::vector<const N*>::const_iterator first = mySortedNodes->begin() + myFromInclusive;
    //     typename std::vector<const N*>::const_iterator last = mySortedNodes->begin() + myToExclusive;
    //     typename std::vector<const N*>::const_iterator iter;
    //     // go through the nodes of the cell
    //     for (iter = first; iter != last; iter++) {
    //         double nodeAxisValue;
    //         nodeAxisValue = (axis == Axis::Y ? (*iter)->getPosition().y() : (*iter)->getPosition().x());
    //         if (nodeAxisValue > maxAxisValue) {
    //             maxAxisValue = nodeAxisValue;
    //         }
    //     }
    //     assert(maxAxisValue > 0);
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     assert((!myHasNodesSortedWrtToMyAxis || axis != myAxis) || maxAxisValue == returnValue);
    // #endif
    //     return maxAxisValue;
    // }
    #[allow(unused_variables)]
    pub fn max_axis_value(&self, axis: Axis) -> f64 {
        // shortcut: N has no getPosition in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::maxAxisValue needs RouterNode::position")
    }

    /// C++: `double Cell::maxAxisValue() const` [const, private]
    /// Returns the maximum coordinate of the cell nodes' positions, in the direction of the cell's axis (X or Y)
    // template<class E, class N, class V>
    // double KDTreePartition<E, N, V>::Cell::maxAxisValue() const {
    //     return maxAxisValue(myAxis);
    // }
    #[allow(unused_variables)]
    pub fn max_axis_value_2(&self) -> f64 {
        // shortcut: N has no getPosition in a router trait yet, upgrade when RouterNode is added
        panic!("shortcut: Cell::maxAxisValue needs RouterNode::position")
    }

}

/// Partitions the router's network wrt a k-d tree subdivision scheme
/// C++ class `KDTreePartition` (utils/router/KDTreePartition.h)
pub struct KDTreePartition<E, N, V> {
    /// The root of the k-d tree
    pub myRoot: Ptr<KDTreePartition_Cell>,
    /// The number of levels
    pub myNumberOfLevels: i32,
    /// The reference to a constant container with pointers to edges
    pub myEdges: Ptr<i32>,
    /// The container with all nodes, sorted wrt to the k-d tree subdivision scheme
    pub mySortedNodes: i32,
    /// The cells
    pub myCells: Vec<Ptr<KDTreePartition_Cell>>,
    /// The cells of all partitions at all levels of the k-d tree subdivision scheme
    pub myLevelCells: Vec<Vec<Ptr<KDTreePartition_Cell>>>,
    /// The boolean flag indicating whether edge permissions need to be considered or not
    pub myHavePermissions: bool,
    /// The boolean flag indicating whether edge restrictions need to be considered or not
    pub myHaveRestrictions: bool,
    /// The boolean flag indicating whether the k-d tree is a clean (empty, uninitialized) instance or not
    pub myAmClean: bool,
    _p: std::marker::PhantomData<(E, N, V,)>,
}

impl<E, N, V> KDTreePartition<E, N, V> {
    /// C++: `void KDTreePartition::KDTreePartition<E, N, V>(int, const int &, const bool, const bool)`
    /// Constructor
    // template<class E, class N, class V>
    // KDTreePartition<E, N, V>::KDTreePartition(int numberOfLevels, const std::vector<E*>& edges,
    //         const bool havePermissions, const bool haveRestrictions) :
    //     myNumberOfLevels(numberOfLevels),
    //     myEdges(edges),
    //     myHavePermissions(havePermissions),
    //     myHaveRestrictions(haveRestrictions),
    //     myAmClean(true) { /* still uninitialized */
    //     if (numberOfLevels <= 0) {
    //         throw std::invalid_argument("KDTreePartition::KDTreePartition: zero or negative number of levels has been passed!");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(numberOfLevels: i32, edges: &Vec<Ptr<E>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        // shortcut: needs typed cell storage (mySortedNodes is an i32 placeholder), upgrade when the k-d tree storage is typed
        panic!("shortcut: KDTreePartition::new needs typed storage")
    }

    /// C++: `void KDTreePartition::~KDTreePartition<E, N, V>()`
    /// Destructor
    // ~KDTreePartition() {
    //         std::vector<const N*>().swap(mySortedNodes);
    //         std::vector<const Cell*>().swap(myCells);
    //         std::vector<std::vector<const Cell*>>().swap(myLevelCells);
    //         delete myRoot;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        let root = self.myRoot;
        if !root.is_null() {
            root.m().drop_();
            delete_ptr(root);
        }
    }

    /// C++: `void KDTreePartition::init(const V *const)`
    /// Initialize the k-d tree wrt to the given vehicle's permissions
    // template<class E, class N, class V>
    // void KDTreePartition<E, N, V>::init(const V* const vehicle) {
    //     size_t edgeCounter = 0;
    //     std::unordered_set<const N*> nodeSet;
    //     // extract nodes from edges
    //     mySortedNodes.resize(myEdges.size() * 2);
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Extracting nodes from edges..." << std::endl;
    // #endif
    //     for (E* edge : myEdges) {
    //         if ((myHavePermissions && edge->prohibits(vehicle))
    //                 || (myHaveRestrictions && edge->restricts(vehicle))) {
    //             continue;
    //         }
    //         const N* node = edge->getFromJunction();
    //         typename std::unordered_set<const N*>::const_iterator it = nodeSet.find(node);
    //         if (it == nodeSet.end()) {
    //             nodeSet.insert(node);
    //             assert(edgeCounter < mySortedNodes.size());
    //             mySortedNodes[edgeCounter++] = node;
    //         }
    //         node = edge->getToJunction();
    //         it = nodeSet.find(node);
    //         if (it == nodeSet.end()) {
    //             nodeSet.insert(node);
    //             assert(edgeCounter < mySortedNodes.size());
    //             mySortedNodes[edgeCounter++] = node;
    //         }
    //     }
    //     mySortedNodes.shrink_to_fit();
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Nodes extracted from edges." << std::endl;
    // #endif
    //     mySortedNodes.resize(edgeCounter);
    //     myCells.resize(numberOfCells());
    //     myLevelCells.resize(myNumberOfLevels);
    //     // call the recursive cell constructor at the root (instantiates the whole k-d tree of cells)
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Calling root cell constructor..." << std::endl;
    // #endif
    // #ifdef KDTP_FOR_SYNTHETIC_NETWORKS
    //     myRoot = new Cell(&myCells, &myLevelCells, &mySortedNodes, myNumberOfLevels, 0, Axis::Y, 0, mySortedNodes.size(), nullptr, -1, -1, -1, -1,
    //                       nullptr, nullptr, nullptr, nullptr, false, vehicle, havePermissions, haveRestrictions);
    // #else
    //     myRoot = new Cell(&myCells, &myLevelCells, &mySortedNodes, myNumberOfLevels, 0, Axis::Y, 0, mySortedNodes.size(), nullptr, -1, -1, -1, -1,
    //                       false, vehicle, myHavePermissions, myHaveRestrictions);
    // #endif
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Done." << std::endl;
    // #endif
    //     assert(myCells[0] == myRoot);
    //     assert(myRoot->getNumber() == 0);
    //     // nodes are now sorted wrt to the k-d tree's subdivisional scheme
    // #ifdef KDTP_DEBUG_LEVEL_0
    //     const N* node = mySortedNodes[0];
    //     std::vector<int>* numbers = cellNumbers(node);
    //     int i;
    //     for (i = 0; i < myNumberOfLevels; ++i) {
    //         std::cout << "Cell numbers of test node: " << (*numbers)[i] << std::endl;
    //     }
    //     delete numbers;
    //     for (i = 0; i < myNumberOfLevels; ++i) {
    //         const std::vector<const Cell*>& levelCells = getCellsAtLevel(i);
    //         size_t size = levelCells.size();
    //         std::cout << "Level " << i << " has " << size << " cells." << std::endl;
    //         std::cout << "The numbers of the cells are: " << std::endl;
    //         size_t k = 0;
    //         for (const Cell* cell : levelCells) {
    //             std::cout << cell->getNumber() << (k++ < size ? ", " : "") << std::endl;
    //         }
    //     }
    // #endif
    //     myAmClean = false;
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, vehicle: Ptr<V>) {
        // shortcut: needs RouterNode accessors and typed mySortedNodes/Cell constructor, upgrade when both exist
        panic!("shortcut: KDTreePartition::init needs RouterNode and typed storage")
    }

    /// C++: `void KDTreePartition::reset(const V *const)`
    /// Delete the k-d tree, and create a new one param[in] vehicle The vehicle
    // void reset(const V* const vehicle) {
    //         delete myRoot;
    //         init(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn reset(&mut self, vehicle: Ptr<V>) {
        // shortcut: depends on init, upgrade when init is ported
        panic!("shortcut: KDTreePartition::reset needs init")
    }

    /// C++: `const Cell * KDTreePartition::getRoot() const` [const]
    /// Returns the root of the k-d tree
    // const Cell* getRoot() const {
    //         return myRoot;
    //     }
    #[allow(unused_variables)]
    pub fn get_root(&self) -> Ptr<KDTreePartition_Cell> {
        self.myRoot
    }

    /// C++: `const int & KDTreePartition::getCells()`
    /// Returns all cells
    // const std::vector<const Cell*>& getCells() {
    //         return myCells;
    //     }
    #[allow(unused_variables)]
    pub fn get_cells(&mut self) -> &Vec<Ptr<KDTreePartition_Cell>> {
        &self.myCells
    }

    /// C++: `const Cell * KDTreePartition::cell(int) const` [const]
    /// Returns the cell with the given number
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn cell(&self, number: i32) -> Ptr<KDTreePartition_Cell> {
        self.myCells[number as usize]
    }

    /// C++: `const int & KDTreePartition::getCellsAtLevel(int) const` [const]
    /// Returns all cells of a level
    // const std::vector<const Cell*>& getCellsAtLevel(int level) const {
    //         return myLevelCells[level];
    //     }
    #[allow(unused_variables)]
    pub fn get_cells_at_level(&self, level: i32) -> &Vec<Ptr<KDTreePartition_Cell>> {
        &self.myLevelCells[level as usize]
    }

    /// C++: `int * KDTreePartition::cellNumbers(const N *) const` [const]
    /// Returns the numbers of the cells which the given node is situated in
    // template<class E, class N, class V>
    // std::vector<int>* KDTreePartition<E, N, V>::cellNumbers(const N* node) const {
    //     std::vector<int>* nodeCellNumbers = new std::vector<int>(myNumberOfLevels);
    //     int i;
    //     for (i = 0; i < myNumberOfLevels; ++i) {
    //         (*nodeCellNumbers)[i] = -1;
    //     }
    //     cellNumbersAux(node, myCells[0], 0, nodeCellNumbers);
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "The following entries in nodeCellNumbers should all be set (!=-1): " << std::endl;
    //     for (i = 0; i < myNumberOfLevels; ++i) {
    //         assert((*nodeCellNumbers)[i] != -1);
    //         std::cout << "nodeCellNumbers[" << i << "]:" << (*nodeCellNumbers)[i] << std::endl;
    //     }
    // #endif
    //     return nodeCellNumbers;
    // }
    #[allow(unused_variables)]
    pub fn cell_numbers(&self, node: Ptr<N>) -> Ptr<i32> {
        // shortcut: needs RouterNode::position for cellNumbersAux, upgrade when RouterNode is added
        panic!("shortcut: KDTreePartition::cellNumbers needs RouterNode::position")
    }

    /// C++: `Axis KDTreePartition::flip(Axis)` [static]
    /// Returns the conjugated 2D axis
    // static Axis flip(Axis axis) {
    //         return axis == Axis::X ? Axis::Y : Axis::X;
    //     }
    #[allow(unused_variables)]
    pub fn flip(axis: Axis) -> Axis {
        if axis == Axis::X {
            Axis::Y
        } else {
            Axis::X
        }
    }

    /// C++: `int KDTreePartition::numberOfArcFlags() const` [const]
    /// Returns the number of arc flags required in a multi-level approach
    // int numberOfArcFlags() const {
    //         return 2 * (myNumberOfLevels - 1);
    //     }
    #[allow(unused_variables)]
    pub fn number_of_arc_flags(&self) -> i32 {
        2 * (self.myNumberOfLevels - 1)
    }

    /// C++: `int KDTreePartition::getNumberOfLevels() const` [const]
    // int getNumberOfLevels() const {
    //         return myNumberOfLevels;
    //     }
    #[allow(unused_variables)]
    pub fn get_number_of_levels(&self) -> i32 {
        self.myNumberOfLevels
    }

    /// C++: `size_t KDTreePartition::numberOfCells() const` [const]
    /// Returns the number of cells at all levels
    // size_t numberOfCells() const {
    //         return static_cast<size_t>(std::lround(std::pow(2, myNumberOfLevels)) - 1);
    //     }
    #[allow(unused_variables)]
    pub fn number_of_cells(&self) -> usize {
        (2f64.powi(self.myNumberOfLevels).round() as i64 - 1) as usize
    }

    /// C++: `size_t KDTreePartition::numberOfRegions() const` [const]
    /// Returns the number of regions, i.e. the number of cells at the shallowest (bottom/leaf) level
    // size_t numberOfRegions() const {
    //         return static_cast<size_t>(std::lround(std::pow(2, myNumberOfLevels - 1)));
    //     }
    #[allow(unused_variables)]
    pub fn number_of_regions(&self) -> usize {
        2f64.powi(self.myNumberOfLevels - 1).round() as usize
    }

    /// C++: `bool KDTreePartition::isClean()`
    /// Returns true iff the k-d tree is uninitialized
    // bool isClean() {
    //         return myAmClean;
    //     }
    #[allow(unused_variables)]
    pub fn is_clean(&mut self) -> bool {
        self.myAmClean
    }

    /// C++: `const Cell * KDTreePartition::searchNode(const N *) const` [const]
    /// Search a node in the bottom cells (i.e., return the respective cell)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn search_node(&self, node: Ptr<N>) -> Ptr<KDTreePartition_Cell> {
        // shortcut: needs RouterNode::position via cell contains, upgrade when RouterNode is added
        panic!("shortcut: KDTreePartition::searchNode needs RouterNode::position")
    }

    /// C++: `void KDTreePartition::cellNumbersAux(const N *, const Cell *, int, int *) const` [const, private]
    /// Helper method for {
    // template<class E, class N, class V>
    // void KDTreePartition<E, N, V>::cellNumbersAux(const N* node, const Cell* cell, int level, std::vector<int>* nodeCellNumbers) const {
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     std::cout << "Call ...aux, level: " << level << ", cell->getAxis(): " << (cell->getAxis() == Axis::X ? "X" : "Y") << std::endl;
    // #endif
    //     assert(level < myNumberOfLevels);
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     Axis flippedCellAxis = flip(cell->getAxis());
    //     double nodeAxisValue = flippedCellAxis == Axis::Y ? node->getPosition().y() : node->getPosition().x();
    //     std::cout << "Flipped axis of cell (=parent cell axis): " << (flippedCellAxis == Axis::X ? "X" : "Y")
    //               << ", cell min axis value in flipped axis: " << (flippedCellAxis == Axis::X ? cell->getMinX() : cell->getMinY())
    //               << ", node axis value in flipped axis: " << nodeAxisValue << ", cell max axis value in flipped axis: "
    //               << (flippedCellAxis == Axis::X ? cell->getMaxX() : cell->getMaxY()) << std::endl;
    // #endif
    //     if (cell->contains(node)) {
    //         // node is inside the cell, hence cell is one of the cells of node
    //         (*nodeCellNumbers)[level] = cell->getNumber();
    // #ifdef KDTP_DEBUG_LEVEL_1
    //         std::cout << "Just filled nodeCellNumbers[" << level << "]:" << (*nodeCellNumbers)[level] << std::endl;
    // #endif
    //         // only one of the below two calls may actually alter array nodeCellNumbers
    //         const Cell* leftOrLowerSubcell = cell->getLeftOrLowerSubcell();
    //         if (leftOrLowerSubcell) { // no more calls at shallowest (i.e. leaf) level
    //             cellNumbersAux(node, leftOrLowerSubcell, level + 1, nodeCellNumbers);
    //         }
    //         const Cell* rightOrUpperSubcell = cell->getRightOrUpperSubcell();
    //         if (rightOrUpperSubcell) {
    //             cellNumbersAux(node, rightOrUpperSubcell, level + 1, nodeCellNumbers);
    //         }
    //     }
    // #ifdef KDTP_DEBUG_LEVEL_1
    //     else { // node is not inside the cell, no need to look for node in subcells (consequently, do nothing more, return)
    //         std::cout << "Not inside cell, do nothing." << std::endl;
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn cell_numbers_aux(&self, node: Ptr<N>, cell: Ptr<KDTreePartition_Cell>, level: i32, nodeCellNumbers: Ptr<i32>) {
        // shortcut: needs RouterNode::position, upgrade when RouterNode is added
        panic!("shortcut: KDTreePartition::cellNumbersAux needs RouterNode::position")
    }

}

/// Class to compare (and so sort) nodes by x-coordinate value
/// C++ class `KDTreePartition::NodeXComparator` (utils/router/KDTreePartition.h)
pub struct KDTreePartition_NodeXComparator {
}

impl KDTreePartition_NodeXComparator {
    /// C++: `bool NodeXComparator::operator()(const N *, const N *) const` [const]
    /// Comparing method for X-axis
    // bool operator()(const N* firstNode, const N* secondNode) const {
    //             if (firstNode->getPosition().x() == secondNode->getPosition().x()) { // tie
    //                 std::string str = firstNode->getID();
    //                 std::intmax_t firstValue = std::strtoimax(str.c_str(), nullptr, 10);
    //                 str = secondNode->getID();
    //                 std::intmax_t secondValue = std::strtoimax(str.c_str(), nullptr, 10);
    //                 return firstValue < secondValue;
    //             }
    //             return firstNode->getPosition().x() < secondNode->getPosition().x();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, firstNode: Ptr<Unmapped /* N */>, secondNode: Ptr<Unmapped /* N */>) -> bool {
        // shortcut: needs RouterNode::position and getID, upgrade when RouterNode is added
        panic!("shortcut: NodeXComparator needs RouterNode::position and id")
    }

}

/// Class to compare (and so sort) nodes by y-coordinate value
/// C++ class `KDTreePartition::NodeYComparator` (utils/router/KDTreePartition.h)
pub struct KDTreePartition_NodeYComparator {
}

impl KDTreePartition_NodeYComparator {
    /// C++: `bool NodeYComparator::operator()(const N *, const N *) const` [const]
    /// Comparing method for Y-axis
    // bool operator()(const N* firstNode, const N* secondNode) const {
    //             if (firstNode->getPosition().y() == secondNode->getPosition().y()) { // tie
    //                 std::string str = firstNode->getID();
    //                 std::intmax_t firstValue = std::strtoimax(str.c_str(), nullptr, 10);
    //                 str = secondNode->getID();
    //                 std::intmax_t secondValue = std::strtoimax(str.c_str(), nullptr, 10);
    //                 return firstValue < secondValue;
    //             }
    //             return firstNode->getPosition().y() < secondNode->getPosition().y();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, firstNode: Ptr<Unmapped /* N */>, secondNode: Ptr<Unmapped /* N */>) -> bool {
        // shortcut: needs RouterNode::position and getID, upgrade when RouterNode is added
        panic!("shortcut: NodeYComparator needs RouterNode::position and id")
    }

}

