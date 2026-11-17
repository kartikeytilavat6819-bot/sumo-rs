//! Port of netgen/NGRandomNetBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::rand_helper;
use utils::geom::geom_helper;

/// A class that builds random network using an algorithm by Markus Hartinger.
/// C++ class `NGRandomNetBuilder` (netgen/NGRandomNetBuilder.h)
pub struct NGRandomNetBuilder {
    /// The network to fill
    pub myNet: Ptr<NGNet>,
    /// The list of outer nodes
    pub myOuterNodes: NGNodeList,
    /// The list of outer links
    pub myOuterLinks: NGEdgeList,
    pub myConNodes: NGNodeList,
    /// Minimum angle allowed between two links
    pub myMinLinkAngle: f64,
    /// Minimum distance allowed between two nodes
    pub myMinDistance: f64,
    /// Maximum distance allowed between two nodes
    pub myMaxDistance: f64,
    /// Probability of connecting to a existing node if possible
    pub myConnectivity: f64,
    /// Number of tries to create a new node
    pub myNumTries: i32,
    /// Number of nodes to be created
    pub myNumNodes: i32,
    /// The distribution of number of neighbours
    pub myNeighbourDistribution: RandomDistributor<i32>,
}

impl NGRandomNetBuilder {
    /// C++: `void NGRandomNetBuilder::NGRandomNetBuilder(NGNet &, double, double, double, double, int, const RandomDistributor<int> &)`
    /// Constructor
    // NGRandomNetBuilder::NGRandomNetBuilder(NGNet& net, double minAngle, double minDistance,
    //                                        double maxDistance, double connectivity,
    //                                        int numTries, const RandomDistributor<int>& neighborDist)
    //     : myNet(net), myMinLinkAngle(minAngle), myMinDistance(minDistance),
    //       myMaxDistance(maxDistance), myConnectivity(connectivity), myNumTries(numTries),
    //       myNeighbourDistribution(neighborDist) {
    // }
    #[allow(unused_variables)]
    pub fn new(net: &mut NGNet, minAngle: f64, minDistance: f64, maxDistance: f64, connectivity: f64, numTries: i32, neighborDist: &RandomDistributor<i32>) -> Self {
        NGRandomNetBuilder {
            myNet: Ptr::from_mut(net),
            myOuterNodes: NGNodeList::new(),
            myOuterLinks: NGEdgeList::new(),
            myConNodes: NGNodeList::new(),
            myMinLinkAngle: minAngle,
            myMinDistance: minDistance,
            myMaxDistance: maxDistance,
            myConnectivity: connectivity,
            myNumTries: numTries,
            myNumNodes: 0,
            myNeighbourDistribution: neighborDist.clone(),
        }
    }

    /// C++: `void NGRandomNetBuilder::createNet(int, bool)`
    /// Builds a NGNet using the set values
    // void
    // NGRandomNetBuilder::createNet(int numNodes, bool gridMode) {
    //     myNumNodes = numNodes;
    //
    //     NGNode* outerNode = new NGNode(myNet.getNextFreeID());
    //     outerNode->setX(0);
    //     outerNode->setY(0);
    //     outerNode->setMaxNeighbours(4);
    //
    //     myNet.add(outerNode);
    //     myOuterNodes.push_back(outerNode);
    //
    //     bool created = true;
    //     while ((myNet.nodeNo() < numNodes) && (myOuterNodes.size() > 0)) {
    //         // brings last element to front
    //         if (!created) {
    //             myOuterNodes.push_front(myOuterNodes.back());
    //             myOuterNodes.pop_back();
    //         }
    //         outerNode = myOuterNodes.back();
    //         findPossibleOuterNodes(outerNode);
    //         created = false;
    //         if ((myConNodes.size() > 0) && (RandHelper::rand() < myConnectivity)) {
    //             // create link
    //             NGEdge* newLink = new NGEdge(myNet.getNextFreeID(), outerNode, myConNodes.back());
    //             if (canConnect(outerNode, myConNodes.back())) {
    //                 // add link
    //                 myNet.add(newLink);
    //                 myOuterLinks.push_back(newLink);
    //                 // check nodes for being outer node
    //                 if ((int)outerNode->getLinks().size() >= outerNode->getMaxNeighbours()) {
    //                     removeOuterNode(outerNode);
    //                 }
    //                 if ((int)myConNodes.back()->getLinks().size() >= myConNodes.back()->getMaxNeighbours()) {
    //                     removeOuterNode(myConNodes.back());
    //                 }
    //                 created = true;
    //             } else {
    //                 delete newLink;
    //             }
    //         } else {
    //             int count = 0;
    //             do {
    //                 created = createNewNode(outerNode, gridMode);
    //                 count++;
    //             } while ((count <= myNumTries) && !created);
    //             if (!created) {
    //                 outerNode->setMaxNeighbours((int)outerNode->getLinks().size());
    //                 myOuterNodes.remove(outerNode);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_net(&mut self, numNodes: i32, gridMode: bool) {
        self.myNumNodes = numNodes;

        let mut outerNode: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new(&self.myNet.m().get_next_free_id()))));
        outerNode.m().set_x(0.);
        outerNode.m().set_y(0.);
        outerNode.m().set_max_neighbours(4);

        self.myNet.m().add(outerNode);
        self.myOuterNodes.push_back(outerNode);

        let mut created = true;
        while (self.myNet.node_no() < numNodes) && (self.myOuterNodes.len() > 0) {
            // brings last element to front
            if !created {
                let back = *self.myOuterNodes.back().unwrap();
                self.myOuterNodes.push_front(back);
                self.myOuterNodes.pop_back();
            }
            outerNode = *self.myOuterNodes.back().unwrap();
            self.find_possible_outer_nodes(outerNode);
            created = false;
            if (self.myConNodes.len() > 0) && (rand_helper::rand() < self.myConnectivity) {
                // create link
                let conBack = *self.myConNodes.back().unwrap();
                let newLink: Ptr<NGEdge> = Ptr::from_raw(Box::into_raw(NGEdge::new(&self.myNet.m().get_next_free_id(), outerNode, conBack, "")));
                if self.can_connect(outerNode, conBack) {
                    // add link
                    self.myNet.m().add_2(newLink);
                    self.myOuterLinks.push_back(newLink);
                    // check nodes for being outer node
                    if (outerNode.get_links().len() as i32) >= outerNode.m().get_max_neighbours() {
                        self.remove_outer_node(outerNode);
                    }
                    if (conBack.get_links().len() as i32) >= conBack.m().get_max_neighbours() {
                        self.remove_outer_node(conBack);
                    }
                    created = true;
                } else {
                    // delete newLink
                    newLink.m().drop_();
                    drop(unsafe { Box::from_raw(newLink.as_raw()) });
                }
            } else {
                let mut count = 0;
                loop {
                    created = self.create_new_node(outerNode, gridMode);
                    count += 1;
                    if !((count <= self.myNumTries) && !created) {
                        break;
                    }
                }
                if !created {
                    outerNode.m().set_max_neighbours(outerNode.get_links().len() as i32);
                    self.myOuterNodes.retain(|n| *n != outerNode);
                }
            }
        }
    }

    /// C++: `void NGRandomNetBuilder::removeOuterNode(NGNode *)` [private]
    /// Removes the given node from the list of outer nodes
    // void
    // NGRandomNetBuilder::removeOuterNode(NGNode* node) {
    //     for (NGNodeList::iterator ni = myOuterNodes.begin(); ni != myOuterNodes.end(); ++ni) {
    //         if (*ni == node) {
    //             myOuterNodes.erase(ni);
    //             return;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_outer_node(&mut self, node: Ptr<NGNode>) {
        if let Some(pos) = self.myOuterNodes.iter().position(|ni| *ni == node) {
            self.myOuterNodes.remove(pos);
        }
    }

    /// C++: `bool NGRandomNetBuilder::checkAngles(const NGNode *const)` [private]
    /// Checks whether the angle of this node's connections are valid
    // bool
    // NGRandomNetBuilder::checkAngles(const NGNode* const node) {
    //     bool check = true;
    //
    //     if (node->getLinks().size() >  1) {
    //         // loop over all links
    //         NGNode* ni;
    //         for (NGEdge* const li : node->getLinks()) {
    //             // calc vector of currentnode
    //             if (li->getStartNode() == node) {
    //                 ni = li->getEndNode();
    //             } else {
    //                 ni = li->getStartNode();
    //             }
    //             Position v1(
    //                 ni->getPosition().x() - node->getPosition().x(),
    //                 ni->getPosition().y() - node->getPosition().y());
    //             // loop over all links
    //             for (NGEdge* const lj : node->getLinks()) {
    //                 if (li != lj) {
    //                     if (lj->getStartNode() == node) {
    //                         ni = lj->getEndNode();
    //                     } else {
    //                         ni = lj->getStartNode();
    //                     }
    //                     Position v2(
    //                         ni->getPosition().x() - node->getPosition().x(),
    //                         ni->getPosition().y() - node->getPosition().y());
    //                     if (fabs(GeomHelper::angle2D(v1, v2)) < myMinLinkAngle) {
    //                         check = false;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return check;
    // }
    #[allow(unused_variables)]
    pub fn check_angles(&mut self, node: Ptr<NGNode>) -> bool {
        let mut check = true;

        if node.get_links().len() > 1 {
            // loop over all links
            for li in node.get_links().iter().copied() {
                // calc vector of currentnode
                let mut ni: Ptr<NGNode> = if li.get_start_node() == node { li.get_end_node() } else { li.get_start_node() };
                let v1 = Position::new(
                    ni.get_position().x() - node.get_position().x(),
                    ni.get_position().y() - node.get_position().y());
                // loop over all links
                for lj in node.get_links().iter().copied() {
                    if li != lj {
                        ni = if lj.get_start_node() == node { lj.get_end_node() } else { lj.get_start_node() };
                        let v2 = Position::new(
                            ni.get_position().x() - node.get_position().x(),
                            ni.get_position().y() - node.get_position().y());
                        if geom_helper::angle2d(&v1, &v2).abs() < self.myMinLinkAngle {
                            check = false;
                        }
                    }
                }
            }
        }
        check
    }

    /// C++: `bool NGRandomNetBuilder::canConnect(NGNode *, NGNode *)` [private]
    /// Checks whether connecting the given two nodes complies with the set restrictions
    // bool
    // NGRandomNetBuilder::canConnect(NGNode* baseNode, NGNode* newNode) {
    //     bool connectable = true;
    //     const PositionVector n(baseNode->getPosition(), newNode->getPosition());
    //
    //     // check for range between Basenode and Newnode
    //     if (connectable) {
    //         double dist = n.length();
    //         if ((dist < myMinDistance) || (dist > myMaxDistance)) {
    //             connectable = false;
    //         }
    //     }
    //
    //     // check for angle restrictions
    //     if (connectable) {
    //         connectable = checkAngles(baseNode);
    //     }
    //     if (connectable) {
    //         connectable = checkAngles(newNode);
    //     }
    //
    //     // check for intersections and range restrictions with outer links
    //     if (connectable) {
    //         NGEdgeList::iterator li;
    //         li = myOuterLinks.begin();
    //         while (connectable && (li != myOuterLinks.end())) {
    //             // check intersection only if links don't share a node
    //             const NGNode* const start = (*li)->getStartNode();
    //             const NGNode* const end = (*li)->getEndNode();
    //             const Position& p1 = start->getPosition();
    //             const Position& p2 = end->getPosition();
    //             if ((baseNode != start) && (baseNode != end) && (newNode != start) && (newNode != end)) {
    //                 connectable = !n.intersects(p1, p2);
    //             }
    //             // check NewNode-To-Links distance only, if NewNode isn't part of link
    //             if (connectable && (newNode != start) && (newNode != end)) {
    //                 const double offset = GeomHelper::nearest_offset_on_line_to_point2D(p1, p2, n[1]);
    //                 if (offset != GeomHelper::INVALID_OFFSET) {
    //                     const Position p = PositionVector(p1, p2).positionAtOffset2D(offset);
    //                     const double dist = p.distanceTo2D(n[1]);
    //                     if (dist < myMinDistance) {
    //                         connectable = false;
    //                     }
    //                 }
    //             }
    //             ++li;
    //         }
    //     }
    //     return connectable;
    // }
    #[allow(unused_variables)]
    pub fn can_connect(&mut self, baseNode: Ptr<NGNode>, newNode: Ptr<NGNode>) -> bool {
        let mut connectable = true;
        let n = PositionVector::from(vec![*baseNode.get_position(), *newNode.get_position()]);

        // check for range between Basenode and Newnode
        if connectable {
            let dist = n.length();
            if (dist < self.myMinDistance) || (dist > self.myMaxDistance) {
                connectable = false;
            }
        }

        // check for angle restrictions
        if connectable {
            connectable = self.check_angles(baseNode);
        }
        if connectable {
            connectable = self.check_angles(newNode);
        }

        // check for intersections and range restrictions with outer links
        if connectable {
            for li in self.myOuterLinks.iter().copied() {
                if !connectable {
                    break;
                }
                // check intersection only if links don't share a node
                let start = li.get_start_node();
                let end = li.get_end_node();
                let p1 = *start.get_position();
                let p2 = *end.get_position();
                if (baseNode != start) && (baseNode != end) && (newNode != start) && (newNode != end) {
                    connectable = !n.intersects(&p1, &p2);
                }
                // check NewNode-To-Links distance only, if NewNode isn't part of link
                if connectable && (newNode != start) && (newNode != end) {
                    let offset = geom_helper::nearest_offset_on_line_to_point2d(&p1, &p2, &n[1], true);
                    if offset != geom_helper::INVALID_OFFSET {
                        let p = PositionVector::from(vec![p1, p2]).position_at_offset2d(offset, 0., false);
                        let dist = p.distance_to2d(&n[1]);
                        if dist < self.myMinDistance {
                            connectable = false;
                        }
                    }
                }
            }
        }
        connectable
    }

    /// C++: `bool NGRandomNetBuilder::createNewNode(NGNode *, bool)` [private]
    /// Creates new random node
    // bool
    // NGRandomNetBuilder::createNewNode(NGNode* baseNode, bool gridMode) {
    //     // calculate position of new node based on BaseNode
    //     double dist = RandHelper::rand(myMinDistance, myMaxDistance);
    //     double angle = RandHelper::rand((double)(2 * M_PI));
    //     if (gridMode) {
    //         // dist must be a multiple of minDist
    //         dist = MAX2(1, int(dist / myMinDistance)) * myMinDistance;
    //         // angle must be a multiple of 90 degrees
    //         angle = RandHelper::rand(4) * 0.5 * M_PI;
    //     }
    //     double x = baseNode->getPosition().x() + dist * cos(angle);
    //     double y = baseNode->getPosition().y() + dist * sin(angle);
    //     NGNode* newNode = new NGNode(myNet.getNextFreeID());
    //     newNode->setX(x);
    //     newNode->setY(y);
    //     newNode->setMaxNeighbours(myNeighbourDistribution.get());
    //     NGEdge* newLink = new NGEdge(myNet.getNextFreeID(), baseNode, newNode);
    //     if (canConnect(baseNode, newNode)) {
    //         // add node
    //         myNet.add(newNode);
    //         myOuterNodes.push_front(newNode);
    //         // add link
    //         myNet.add(newLink);
    //         myOuterLinks.push_back(newLink);
    //         // check basenode for being outer node
    //         if ((int)baseNode->getLinks().size() >= baseNode->getMaxNeighbours()) {
    //             removeOuterNode(baseNode);
    //         }
    //         return true;
    //     } else {
    //         delete newLink;
    //         delete newNode;
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_new_node(&mut self, baseNode: Ptr<NGNode>, gridMode: bool) -> bool {
        // calculate position of new node based on BaseNode
        let mut dist = rand_helper::with_default(|r| r.rand_range(self.myMinDistance, self.myMaxDistance));
        let mut angle = rand_helper::with_default(|r| r.rand_max(2. * std::f64::consts::PI));
        if gridMode {
            // dist must be a multiple of minDist
            dist = std::cmp::max(1, (dist / self.myMinDistance) as i32) as f64 * self.myMinDistance;
            // angle must be a multiple of 90 degrees
            angle = rand_helper::with_default(|r| r.rand_int(4)) as f64 * 0.5 * std::f64::consts::PI;
        }
        let x = baseNode.get_position().x() + dist * angle.cos();
        let y = baseNode.get_position().y() + dist * angle.sin();
        let newNode: Ptr<NGNode> = Ptr::from_raw(Box::into_raw(Box::new(NGNode::new(&self.myNet.m().get_next_free_id()))));
        newNode.m().set_x(x);
        newNode.m().set_y(y);
        newNode.m().set_max_neighbours(self.myNeighbourDistribution.get(None).unwrap());
        let newLink: Ptr<NGEdge> = Ptr::from_raw(Box::into_raw(NGEdge::new(&self.myNet.m().get_next_free_id(), baseNode, newNode, "")));
        if self.can_connect(baseNode, newNode) {
            // add node
            self.myNet.m().add(newNode);
            self.myOuterNodes.push_front(newNode);
            // add link
            self.myNet.m().add_2(newLink);
            self.myOuterLinks.push_back(newLink);
            // check basenode for being outer node
            if (baseNode.get_links().len() as i32) >= baseNode.m().get_max_neighbours() {
                self.remove_outer_node(baseNode);
            }
            true
        } else {
            // delete newLink
            newLink.m().drop_();
            drop(unsafe { Box::from_raw(newLink.as_raw()) });
            // delete newNode
            newNode.m().drop_();
            drop(unsafe { Box::from_raw(newNode.as_raw()) });
            false
        }
    }

    /// C++: `void NGRandomNetBuilder::findPossibleOuterNodes(NGNode *)` [private]
    /// finds possible connections between Node and OuterNodes complying with restrictions
    // void
    // NGRandomNetBuilder::findPossibleOuterNodes(NGNode* node) {
    //     myConNodes.clear();
    //     NGNodeList::iterator ni;
    //     for (ni = myOuterNodes.begin(); ni != myOuterNodes.end(); ++ni) {
    //         NGNode* on = *ni;
    //         if (!node->connected(on)) {
    //             if ((node->getMaxNeighbours() > (int)node->getLinks().size()) &&
    //                     (on->getMaxNeighbours() > (int)on->getLinks().size())) {
    //                 if (canConnect(node, on)) {
    //                     myConNodes.push_back(on);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_possible_outer_nodes(&mut self, node: Ptr<NGNode>) {
        self.myConNodes.clear();
        // can_connect does not change myOuterNodes, so iterating a copy is equivalent
        let outer: Vec<Ptr<NGNode>> = self.myOuterNodes.iter().copied().collect();
        for on in outer {
            if !node.connected(on, false) {
                if (node.m().get_max_neighbours() > node.get_links().len() as i32) && (on.m().get_max_neighbours() > on.get_links().len() as i32) {
                    if self.can_connect(node, on) {
                        self.myConNodes.push_back(on);
                    }
                }
            }
        }
    }

    /// C++: `void NGRandomNetBuilder::NGRandomNetBuilder(const NGRandomNetBuilder &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NGRandomNetBuilder) -> Self {
        panic!("NGRandomNetBuilder copy constructor is invalidated in SUMO")
    }

    /// C++: `NGRandomNetBuilder & NGRandomNetBuilder::operator=(const NGRandomNetBuilder &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NGRandomNetBuilder) -> &mut NGRandomNetBuilder {
        panic!("NGRandomNetBuilder assignment operator is invalidated in SUMO")
    }

}

