//! Port of netgen/NGNet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The class storing the generated network
/// C++ class `NGNet` (netgen/NGNet.h)
pub struct NGNet {
    /// The last ID given to node or link
    pub myLastID: i32,
    /// Whether to use alphanumericalIDs
    pub myAlphaIDs: bool,
    /// The builder used to build NB*-structures
    pub myNetBuilder: Ptr<NBNetBuilder>,
    /// The list of nodes
    pub myNodeList: NGNodeList,
    /// The list of links
    pub myEdgeList: NGEdgeList,
}

impl NGNet {
    /// C++: `void NGNet::NGNet(NBNetBuilder &)`
    /// Constructor
    // NGNet::NGNet(NBNetBuilder& nb) :
    //     myLastID(0),
    //     myAlphaIDs(OptionsCont::getOptions().getBool("alphanumerical-ids")),
    //     myNetBuilder(nb) {
    // }
    #[allow(unused_variables)]
    pub fn new(nb: &mut NBNetBuilder) -> Self {
        todo!("port NGNet::NGNet")
    }

    /// C++: `void NGNet::~NGNet() noexcept`
    /// Destructor
    // NGNet::~NGNet() {
    //     for (NGEdgeList::iterator ni = myEdgeList.begin(); ni != myEdgeList.end(); ++ni) {
    //         delete *ni;
    //     }
    //     for (NGNodeList::iterator ni = myNodeList.begin(); ni != myNodeList.end(); ++ni) {
    //         delete *ni;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NGNet::~NGNet")
    }

    /// C++: `NGNode * NGNet::findNode(int, int)`
    /// Returns the node at the given position
    // NGNode*
    // NGNet::findNode(int xID, int yID) {
    //     for (NGNodeList::iterator ni = myNodeList.begin(); ni != myNodeList.end(); ++ni) {
    //         if ((*ni)->samePos(xID, yID)) {
    //             return *ni;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn find_node(&mut self, xPos: i32, yPos: i32) -> Ptr<NGNode> {
        todo!("port NGNet::findNode")
    }

    /// C++: `std::string NGNet::getNextFreeID()`
    /// Returns the next free id
    // std::string
    // NGNet::getNextFreeID() {
    //     return toString<int>(++myLastID);
    // }
    #[allow(unused_variables)]
    pub fn get_next_free_id(&mut self) -> String {
        todo!("port NGNet::getNextFreeID")
    }

    /// C++: `double NGNet::radialToX(double, double)`
    /// Returns the x-position resulting from the given radius and angle
    // double
    // NGNet::radialToX(double radius, double phi) {
    //     return cos(phi) * radius;
    // }
    #[allow(unused_variables)]
    pub fn radial_to_x(&mut self, radius: f64, phi: f64) -> f64 {
        todo!("port NGNet::radialToX")
    }

    /// C++: `double NGNet::radialToY(double, double)`
    /// Returns the y-position resulting from the given radius and angle
    // double
    // NGNet::radialToY(double radius, double phi) {
    //     return sin(phi) * radius;
    // }
    #[allow(unused_variables)]
    pub fn radial_to_y(&mut self, radius: f64, phi: f64) -> f64 {
        todo!("port NGNet::radialToY")
    }

    /// C++: `void NGNet::createChequerBoard(int, int, double, double, double, double)`
    /// Creates a grid network
    // void
    // NGNet::createChequerBoard(int numX, int numY, double spaceX, double spaceY, double xAttachLength, double yAttachLength) {
    //
    //     for (int ix = 0; ix < numX; ix++) {
    //         const std::string nodeIDStart = (myAlphaIDs ? alphabeticalCode(ix, numX) : toString<int>(ix) + "/");
    //         for (int iy = 0; iy < numY; iy++) {
    //             // create Node
    //             NGNode* node = new NGNode(nodeIDStart + toString(iy), ix, iy);
    //             node->setX(ix * spaceX + xAttachLength);
    //             node->setY(iy * spaceY + yAttachLength);
    //             myNodeList.push_back(node);
    //             // create Links
    //             if (ix > 0) {
    //                 connect(findNode(ix - 1, iy), node);
    //             }
    //             if (iy > 0) {
    //                 connect(findNode(ix, iy - 1), node);
    //             }
    //         }
    //     }
    //     if (yAttachLength > 0.0) {
    //         for (int ix = 0; ix < numX; ix++) {
    //             // create nodes
    //             NGNode* topNode = new NGNode("top" + toString<int>(ix), ix, numY);
    //             NGNode* bottomNode = new NGNode("bottom" + toString<int>(ix), ix, numY + 1);
    //             topNode->setX(ix * spaceX + xAttachLength);
    //             bottomNode->setX(ix * spaceX + xAttachLength);
    //             topNode->setY((numY - 1) * spaceY + 2 * yAttachLength);
    //             bottomNode->setY(0);
    //             topNode->setFringe();
    //             bottomNode->setFringe();
    //             myNodeList.push_back(topNode);
    //             myNodeList.push_back(bottomNode);
    //             // create links
    //             connect(findNode(ix, numY - 1), topNode);
    //             connect(bottomNode, findNode(ix, 0));
    //         }
    //     }
    //     if (xAttachLength > 0.0) {
    //         for (int iy = 0; iy < numY; iy++) {
    //             // create nodes
    //             NGNode* leftNode = new NGNode("left" + toString<int>(iy), numX, iy);
    //             NGNode* rightNode = new NGNode("right" + toString<int>(iy), numX + 1, iy);
    //             leftNode->setX(0);
    //             rightNode->setX((numX - 1) * spaceX + 2 * xAttachLength);
    //             leftNode->setY(iy * spaceY + yAttachLength);
    //             rightNode->setY(iy * spaceY + yAttachLength);
    //             leftNode->setFringe();
    //             rightNode->setFringe();
    //             myNodeList.push_back(leftNode);
    //             myNodeList.push_back(rightNode);
    //             // create links
    //             connect(leftNode, findNode(0, iy));
    //             connect(findNode(numX - 1, iy), rightNode);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_chequer_board(&mut self, numX: i32, numY: i32, spaceX: f64, spaceY: f64, xAttachLength: f64, yAttachLength: f64) {
        todo!("port NGNet::createChequerBoard")
    }

    /// C++: `void NGNet::createSpiderWeb(int, int, double, bool, double)`
    /// Creates a spider network
    // void
    // NGNet::createSpiderWeb(int numRadDiv, int numCircles, double spaceRad, bool hasCenter, double attachLength) {
    //     if (numRadDiv < 3) {
    //         numRadDiv = 3;
    //     }
    //     if (numCircles < 1) {
    //         numCircles = 1;
    //     }
    //
    //     int ir, ic;
    //     double angle = (double)(2 * M_PI / numRadDiv); // angle between radial divisions
    //     NGNode* node;
    //     int attachCircle = -1;
    //     if (attachLength > 0) {
    //         numCircles += 1;
    //         attachCircle = numCircles;
    //     }
    //     for (ic = 1; ic < numCircles + 1; ic++) {
    //         const std::string nodeIDStart = alphabeticalCode(ic, numCircles);
    //         if (ic == attachCircle) {
    //             spaceRad = attachLength;
    //         }
    //         for (ir = 1; ir < numRadDiv + 1; ir++) {
    //             // create Node
    //             const std::string nodeID = (myAlphaIDs ?
    //                                         nodeIDStart + toString<int>(ir) :
    //                                         toString<int>(ir) + "/" + toString<int>(ic));
    //             node = new NGNode(nodeID, ir, ic);
    //             node->setX(radialToX((ic) * spaceRad, (ir - 1) * angle));
    //             node->setY(radialToY((ic) * spaceRad, (ir - 1) * angle));
    //             myNodeList.push_back(node);
    //             // create Links
    //             if (ir > 1 && ic != attachCircle) {
    //                 connect(findNode(ir - 1, ic), node);
    //             }
    //             if (ic > 1) {
    //                 connect(findNode(ir, ic - 1), node);
    //             }
    //             if (ir == numRadDiv && ic != attachCircle) {
    //                 connect(node, findNode(1, ic));
    //             }
    //         }
    //     }
    //     if (hasCenter) {
    //         // node
    //         node = new NGNode(myAlphaIDs ? "A1" : "1", 0, 0, true);
    //         node->setX(0);
    //         node->setY(0);
    //         myNodeList.push_back(node);
    //         // links
    //         for (ir = 1; ir < numRadDiv + 1; ir++) {
    //             connect(node, findNode(ir, 1));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_spider_web(&mut self, numRadDiv: i32, numCircles: i32, spaceRad: f64, hasCenter: bool, attachLength: f64) {
        todo!("port NGNet::createSpiderWeb")
    }

    /// C++: `void NGNet::toNB() const` [const]
    /// Converts the stored network into its netbuilder-representation
    // void
    // NGNet::toNB() const {
    //     Distribution_Parameterized perturbx = getDistribution("perturb-x");
    //     Distribution_Parameterized perturby = getDistribution("perturb-y");
    //     Distribution_Parameterized perturbz = getDistribution("perturb-z");
    //     for (const NGNode* const ngNode : myNodeList) {
    //         // we need to sample in separate instructions because evaluation order is compiler dependent
    //         Position perturb(perturbx.sample(), 0.);
    //         perturb.sety(perturby.sample());
    //         perturb.setz(perturbz.sample());
    //         myNetBuilder.getNodeCont().insert(ngNode->buildNBNode(myNetBuilder, perturb));
    //     }
    //     const std::string type = OptionsCont::getOptions().getString("default.type");
    //     const double bidiProb = OptionsCont::getOptions().getFloat("bidi-probability");
    //     for (const NGEdge* const ngEdge : myEdgeList) {
    //         myNetBuilder.getEdgeCont().insert(ngEdge->buildNBEdge(myNetBuilder, type));
    //         // now, let's append the reverse directions...
    //         if (!ngEdge->getEndNode()->connected(ngEdge->getStartNode(), true) && RandHelper::rand() <= bidiProb) {
    //             myNetBuilder.getEdgeCont().insert(ngEdge->buildNBEdge(myNetBuilder, type, true));
    //         }
    //     }
    //     // add splits depending on turn-lane options
    //     const int turnLanes = OptionsCont::getOptions().getInt("turn-lanes");
    //     const bool lefthand =  OptionsCont::getOptions().getBool("lefthand");
    //     if (turnLanes > 0) {
    //         const double turnLaneLength = OptionsCont::getOptions().getFloat("turn-lanes.length");
    //         NBEdgeCont& ec = myNetBuilder.getEdgeCont();
    //         EdgeVector allEdges;
    //         for (auto it = ec.begin(); it != ec.end(); ++it) {
    //             allEdges.push_back(it->second);
    //         }
    //         for (NBEdge* e : allEdges) {
    //             if (e->getToNode()->geometryLike()) {
    //                 continue;
    //             }
    //             std::vector<NBEdgeCont::Split> splits;
    //             NBEdgeCont::Split split;
    //             for (int i = 0; i < e->getNumLanes() + turnLanes; ++i) {
    //                 split.lanes.push_back(i);
    //             }
    //             split.pos = MAX2(0.0, e->getLength() - turnLaneLength);
    //             split.speed = e->getSpeed();
    //             split.node = new NBNode(e->getID() + "." + toString(split.pos), e->getGeometry().positionAtOffset(split.pos));
    //             split.idBefore = e->getID();
    //             split.idAfter = split.node->getID();
    //             split.offsetFactor = lefthand ? -1 : 1;
    //             if (turnLaneLength <= e->getLength() / 2) {
    //                 split.offset = -0.5 * split.offsetFactor * turnLanes * e->getLaneWidth(0);
    //                 if (e->getFromNode()->geometryLike()) {
    //                     // shift the reverse direction explicitly as it will not get a turn lane
    //                     NBEdge* reverse = nullptr;
    //                     for (NBEdge* reverseCand : e->getFromNode()->getIncomingEdges()) {
    //                         if (reverseCand->getFromNode() == e->getToNode()) {
    //                             reverse = reverseCand;
    //                         }
    //                     }
    //                     if (reverse != nullptr) {
    //                         PositionVector g = reverse->getGeometry();
    //                         g.move2side(-split.offset);
    //                         reverse->setGeometry(g);
    //                     }
    //                 }
    //             }
    //             splits.push_back(split);
    //             ec.processSplits(e, splits,
    //                              myNetBuilder.getNodeCont(),
    //                              myNetBuilder.getDistrictCont(),
    //                              myNetBuilder.getTLLogicCont());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn to_nb(&self) {
        todo!("port NGNet::toNB")
    }

    /// C++: `void NGNet::add(NGNode *)`
    /// Adds the given node to the network
    // void
    // NGNet::add(NGNode* node) {
    //     myNodeList.push_back(node);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, node: Ptr<NGNode>) {
        todo!("port NGNet::add")
    }

    /// C++: `void NGNet::add(NGEdge *)`
    /// Adds the given edge to the network
    // void
    // NGNet::add(NGEdge* edge) {
    //     myEdgeList.push_back(edge);
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, edge: Ptr<NGEdge>) {
        todo!("port NGNet::add")
    }

    /// C++: `int NGNet::nodeNo() const` [const]
    /// Returns the number of stored nodes
    // int
    // NGNet::nodeNo() const {
    //     return (int)myNodeList.size();
    // }
    #[allow(unused_variables)]
    pub fn node_no(&self) -> i32 {
        todo!("port NGNet::nodeNo")
    }

    /// C++: `void NGNet::connect(NGNode *, NGNode *)` [private]
    /// Connects both nodes with two edges, one for each direction
    // void
    // NGNet::connect(NGNode* node1, NGNode* node2) {
    //     std::string id1 = node1->getID() + (myAlphaIDs ? "" : "to") + node2->getID();
    //     std::string id2 = node2->getID() + (myAlphaIDs ? "" : "to") + node1->getID();
    //     NGEdge* link1 = new NGEdge(id1, node1, node2, id2);
    //     myEdgeList.push_back(link1);
    // }
    #[allow(unused_variables)]
    pub fn connect(&mut self, node1: Ptr<NGNode>, node2: Ptr<NGNode>) {
        todo!("port NGNet::connect")
    }

    /// C++: `std::string NGNet::alphabeticalCode(int, int)` [private]
    /// return a letter code for the given integer index
    // std::string
    // NGNet::alphabeticalCode(int i, int iMax) {
    //     // lazy mans 26th root to determine number of characters for x-label
    //     int xn = 1;
    //     for (; std::pow(26, xn) < iMax; xn++) {};
    //     std::string result = "";
    //     for (int j = 0; j < xn; j++) {
    //         result = char('A' + (i % 26)) + result;
    //         i /= 26;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn alphabetical_code(&mut self, i: i32, iMax: i32) -> String {
        todo!("port NGNet::alphabeticalCode")
    }

    /// C++: `Distribution_Parameterized NGNet::getDistribution(const std::string &)` [static, private]
    /// get distribution from option
    // Distribution_Parameterized
    // NGNet::getDistribution(const std::string& option) {
    //     const std::string& val = OptionsCont::getOptions().getString(option);
    //     try {
    //         return Distribution_Parameterized(option, 0, StringUtils::toDouble(val));
    //     } catch (NumberFormatException&) {
    //         return Distribution_Parameterized(val);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_distribution(option: &str) -> Unmapped /* Distribution_Parameterized */ {
        todo!("port NGNet::getDistribution")
    }

    /// C++: `void NGNet::NGNet(const NGNet &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NGNet) -> Self {
        todo!("port NGNet::NGNet")
    }

    /// C++: `NGNet & NGNet::operator=(const NGNet &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NGNet) -> &mut NGNet {
        todo!("port NGNet::operator=")
    }

}

