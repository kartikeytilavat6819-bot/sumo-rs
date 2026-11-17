//! Port of netgen/NGNet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::rand_helper::rand;
use utils::common::std_defs::INVALID_DOUBLE;
use utils::common::string_utils::to_double;
use utils::common::to_string::f64_to_string;
use utils::distribution::distribution::Distribution;
use utils::distribution::distribution_parameterized::DistributionParameterized;
use utils::options::options_cont::{get_bool, get_float, get_int, get_string};
use utils::xml::sumo_xml_definitions::SumoXMLNodeType;

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
        NGNet {
            myLastID: 0,
            myAlphaIDs: get_bool("alphanumerical-ids"),
            myNetBuilder: Ptr::from_mut(nb),
            myNodeList: NGNodeList::new(),
            myEdgeList: NGEdgeList::new(),
        }
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
        for e in self.myEdgeList.drain(..) {
            // delete *ni: run the destructor body, then free the allocation
            let mut b = unsafe { Box::from_raw(e.as_raw()) };
            b.drop_();
        }
        for n in self.myNodeList.drain(..) {
            let mut b = unsafe { Box::from_raw(n.as_raw()) };
            b.drop_();
        }
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
        for ni in self.myNodeList.iter() {
            if ni.same_pos(xPos, yPos) {
                return *ni;
            }
        }
        Ptr::null()
    }

    /// C++: `std::string NGNet::getNextFreeID()`
    /// Returns the next free id
    // std::string
    // NGNet::getNextFreeID() {
    //     return toString<int>(++myLastID);
    // }
    #[allow(unused_variables)]
    pub fn get_next_free_id(&mut self) -> String {
        self.myLastID += 1;
        self.myLastID.to_string()
    }

    /// C++: `double NGNet::radialToX(double, double)`
    /// Returns the x-position resulting from the given radius and angle
    // double
    // NGNet::radialToX(double radius, double phi) {
    //     return cos(phi) * radius;
    // }
    #[allow(unused_variables)]
    pub fn radial_to_x(&mut self, radius: f64, phi: f64) -> f64 {
        phi.cos() * radius
    }

    /// C++: `double NGNet::radialToY(double, double)`
    /// Returns the y-position resulting from the given radius and angle
    // double
    // NGNet::radialToY(double radius, double phi) {
    //     return sin(phi) * radius;
    // }
    #[allow(unused_variables)]
    pub fn radial_to_y(&mut self, radius: f64, phi: f64) -> f64 {
        phi.sin() * radius
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
        for ix in 0..numX {
            let nodeIDStart: String = if self.myAlphaIDs { self.alphabetical_code(ix, numX) } else { format!("{}/", ix) };
            for iy in 0..numY {
                // create Node
                let node: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new_2(&format!("{}{}", nodeIDStart, iy), ix, iy))));
                node.m().set_x(ix as f64 * spaceX + xAttachLength);
                node.m().set_y(iy as f64 * spaceY + yAttachLength);
                self.myNodeList.push_back(node);
                // create Links
                if ix > 0 {
                    let a = self.find_node(ix - 1, iy);
                    self.connect(a, node);
                }
                if iy > 0 {
                    let a = self.find_node(ix, iy - 1);
                    self.connect(a, node);
                }
            }
        }
        if yAttachLength > 0.0 {
            for ix in 0..numX {
                // create nodes
                let topNode: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new_2(&format!("top{}", ix), ix, numY))));
                let bottomNode: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new_2(&format!("bottom{}", ix), ix, numY + 1))));
                topNode.m().set_x(ix as f64 * spaceX + xAttachLength);
                bottomNode.m().set_x(ix as f64 * spaceX + xAttachLength);
                topNode.m().set_y((numY - 1) as f64 * spaceY + 2.0 * yAttachLength);
                bottomNode.m().set_y(0.0);
                topNode.m().set_fringe();
                bottomNode.m().set_fringe();
                self.myNodeList.push_back(topNode);
                self.myNodeList.push_back(bottomNode);
                // create links
                let a = self.find_node(ix, numY - 1);
                self.connect(a, topNode);
                let b = self.find_node(ix, 0);
                self.connect(bottomNode, b);
            }
        }
        if xAttachLength > 0.0 {
            for iy in 0..numY {
                // create nodes
                let leftNode: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new_2(&format!("left{}", iy), numX, iy))));
                let rightNode: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new_2(&format!("right{}", iy), numX + 1, iy))));
                leftNode.m().set_x(0.0);
                rightNode.m().set_x((numX - 1) as f64 * spaceX + 2.0 * xAttachLength);
                leftNode.m().set_y(iy as f64 * spaceY + yAttachLength);
                rightNode.m().set_y(iy as f64 * spaceY + yAttachLength);
                leftNode.m().set_fringe();
                rightNode.m().set_fringe();
                self.myNodeList.push_back(leftNode);
                self.myNodeList.push_back(rightNode);
                // create links
                let a = self.find_node(0, iy);
                self.connect(leftNode, a);
                let b = self.find_node(numX - 1, iy);
                self.connect(b, rightNode);
            }
        }
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
    pub fn create_spider_web(&mut self, mut numRadDiv: i32, mut numCircles: i32, mut spaceRad: f64, hasCenter: bool, attachLength: f64) {
        if numRadDiv < 3 {
            numRadDiv = 3;
        }
        if numCircles < 1 {
            numCircles = 1;
        }

        let angle: f64 = 2.0 * std::f64::consts::PI / (numRadDiv as f64); // angle between radial divisions
        let mut attachCircle: i32 = -1;
        if attachLength > 0.0 {
            numCircles += 1;
            attachCircle = numCircles;
        }
        for ic in 1..numCircles + 1 {
            let nodeIDStart: String = self.alphabetical_code(ic, numCircles);
            if ic == attachCircle {
                spaceRad = attachLength;
            }
            for ir in 1..numRadDiv + 1 {
                // create Node
                let nodeID: String = if self.myAlphaIDs { format!("{}{}", nodeIDStart, ir) } else { format!("{}/{}", ir, ic) };
                let node: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new_2(&nodeID, ir, ic))));
                let x = self.radial_to_x((ic as f64) * spaceRad, ((ir - 1) as f64) * angle);
                node.m().set_x(x);
                let y = self.radial_to_y((ic as f64) * spaceRad, ((ir - 1) as f64) * angle);
                node.m().set_y(y);
                self.myNodeList.push_back(node);
                // create Links
                if ir > 1 && ic != attachCircle {
                    let a = self.find_node(ir - 1, ic);
                    self.connect(a, node);
                }
                if ic > 1 {
                    let a = self.find_node(ir, ic - 1);
                    self.connect(a, node);
                }
                if ir == numRadDiv && ic != attachCircle {
                    let b = self.find_node(1, ic);
                    self.connect(node, b);
                }
            }
        }
        if hasCenter {
            // node
            let node: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new_3(if self.myAlphaIDs { "A1" } else { "1" }, 0, 0, true))));
            node.m().set_x(0.0);
            node.m().set_y(0.0);
            self.myNodeList.push_back(node);
            // links
            for ir in 1..numRadDiv + 1 {
                let b = self.find_node(ir, 1);
                self.connect(node, b);
            }
        }
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
        let nbp: Ptr<NBNetBuilder> = self.myNetBuilder;
        let mut perturbx: DistributionParameterized = Self::get_distribution("perturb-x");
        let mut perturby: DistributionParameterized = Self::get_distribution("perturb-y");
        let mut perturbz: DistributionParameterized = Self::get_distribution("perturb-z");
        for ngNode in self.myNodeList.iter() {
            // we need to sample in separate instructions because evaluation order is compiler dependent
            let px: f64 = perturbx.sample(None).unwrap();
            let mut perturb = Position::new(px, 0.0);
            let py: f64 = perturby.sample(None).unwrap();
            perturb.sety(py);
            let pz: f64 = perturbz.sample(None).unwrap();
            perturb.setz(pz);
            let nbNode = ngNode.build_nb_node(nbp.m(), &perturb);
            nbp.m().get_node_cont().insert_2(nbNode);
        }
        let typ: String = get_string("default.type");
        let bidiProb: f64 = get_float("bidi-probability");
        for ngEdge in self.myEdgeList.iter() {
            let nbEdge = ngEdge.build_nb_edge(nbp.m(), typ.clone(), false);
            nbp.m().get_edge_cont().insert(nbEdge, false);
            // now, let's append the reverse directions...
            if !ngEdge.get_end_node().connected(ngEdge.get_start_node(), true) && utils::common::rand_helper::rand() <= bidiProb {
                let revEdge = ngEdge.build_nb_edge(nbp.m(), typ.clone(), true);
                nbp.m().get_edge_cont().insert(revEdge, false);
            }
        }
        // add splits depending on turn-lane options
        let turnLanes: i32 = get_int("turn-lanes");
        let lefthand: bool = get_bool("lefthand");
        if turnLanes > 0 {
            let turnLaneLength: f64 = get_float("turn-lanes.length");
            let allEdges: EdgeVector = nbp.m().get_edge_cont().begin().map(|(_, e)| *e).collect();
            for e in allEdges {
                if e.get_to_node().geometry_like() {
                    continue;
                }
                let mut splits: Vec<NBEdgeCont_Split> = Vec::new();
                let mut split = NBEdgeCont_Split {
                    lanes: Vec::new(),
                    pos: INVALID_DOUBLE,
                    speed: INVALID_DOUBLE,
                    friction: INVALID_DOUBLE,
                    node: Ptr::null(),
                    idBefore: String::new(),
                    idAfter: String::new(),
                    nameID: String::new(),
                    offset: 0.0,
                    offsetFactor: 1,
                };
                for i in 0..(e.get_num_lanes() + turnLanes) {
                    split.lanes.push(i);
                }
                split.pos = (0.0f64).max(e.get_length() - turnLaneLength);
                split.speed = e.get_speed();
                let splitPos: Position = e.get_geometry().position_at_offset(split.pos, 0.0);
                split.node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(&format!("{}.{}", e.get_id(), f64_to_string(split.pos)), &splitPos, Ptr::null()))));
                split.idBefore = e.get_id().to_string();
                split.idAfter = split.node.get_id().to_string();
                split.offsetFactor = if lefthand { -1 } else { 1 };
                if turnLaneLength <= e.get_length() / 2.0 {
                    split.offset = -0.5 * (split.offsetFactor as f64) * (turnLanes as f64) * e.get_lane_width_2(0);
                    if e.get_from_node().geometry_like() {
                        // shift the reverse direction explicitly as it will not get a turn lane
                        let mut reverse: Ptr<NBEdge> = Ptr::null();
                        for reverseCand in e.get_from_node().get_incoming_edges().iter() {
                            if reverseCand.get_from_node().as_raw() == e.get_to_node().as_raw() {
                                reverse = *reverseCand;
                            }
                        }
                        if !reverse.is_null() {
                            let mut g: PositionVector = reverse.get_geometry().clone();
                            g.move2side(-split.offset, 100.0);
                            reverse.m().set_geometry(&g, false);
                        }
                    }
                }
                splits.push(split);
                nbp.m().get_edge_cont().process_splits(e, splits, nbp.m().get_node_cont(), nbp.m().get_district_cont(), nbp.m().get_tl_logic_cont());
            }
        }
    }

    /// C++: `void NGNet::add(NGNode *)`
    /// Adds the given node to the network
    // void
    // NGNet::add(NGNode* node) {
    //     myNodeList.push_back(node);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, node: Ptr<NGNode>) {
        self.myNodeList.push_back(node);
    }

    /// C++: `void NGNet::add(NGEdge *)`
    /// Adds the given edge to the network
    // void
    // NGNet::add(NGEdge* edge) {
    //     myEdgeList.push_back(edge);
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, edge: Ptr<NGEdge>) {
        self.myEdgeList.push_back(edge);
    }

    /// C++: `int NGNet::nodeNo() const` [const]
    /// Returns the number of stored nodes
    // int
    // NGNet::nodeNo() const {
    //     return (int)myNodeList.size();
    // }
    #[allow(unused_variables)]
    pub fn node_no(&self) -> i32 {
        self.myNodeList.len() as i32
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
        let sep = if self.myAlphaIDs { "" } else { "to" };
        let id1: String = format!("{}{}{}", node1.get_id(), sep, node2.get_id());
        let id2: String = format!("{}{}{}", node2.get_id(), sep, node1.get_id());
        let link1: Ptr<NGEdge> = Ptr::from_raw(Box::into_raw(NGEdge::new(&id1, node1, node2, &id2)));
        self.myEdgeList.push_back(link1);
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
    pub fn alphabetical_code(&mut self, mut i: i32, iMax: i32) -> String {
        // lazy mans 26th root to determine number of characters for x-label
        let mut xn: i32 = 1;
        while 26f64.powf(xn as f64) < iMax as f64 {
            xn += 1;
        }
        let mut result: String = String::new();
        for _ in 0..xn {
            result = format!("{}{}", (b'A' + (i % 26) as u8) as char, result);
            i /= 26;
        }
        result
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
    pub fn get_distribution(option: &str) -> DistributionParameterized {
        let val: String = get_string(option);
        match to_double(&val) {
            Ok(d) => DistributionParameterized::with_mean_deviation(option, 0.0, d),
            Err(_) => DistributionParameterized::from_description(&val).unwrap_or_else(|e| panic!("{}", e)),
        }
    }

    /// C++: `void NGNet::NGNet(const NGNet &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NGNet) -> Self {
        panic!("NGNet is not copyable (private copy constructor in C++)")
    }

    /// C++: `NGNet & NGNet::operator=(const NGNet &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NGNet) -> &mut NGNet {
        panic!("NGNet is not assignable (private operator= in C++)")
    }

}

