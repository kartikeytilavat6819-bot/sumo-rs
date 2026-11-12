//! Port of netgen/NGNode.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::list<NGNode *> NGNodeList`
pub type NGNodeList = VecDeque<Ptr<NGNode>>;

/// A netgen-representation of a node
/// C++ class `NGNode` (netgen/NGNode.h)
#[repr(C)]
pub struct NGNode {
    /// C++ base class `Named`
    pub base: Named,
    /// Integer x-position (x-id)
    pub myXID: i32,
    /// Integer y-position (y-id)
    pub myYID: i32,
    /// List of connected links
    pub myLinkList: NGEdgeList,
    /// The position of the node
    pub myPosition: Position,
    /// The maximum number of neighbours
    pub myMaxNeighbours: i32,
    /// Information whether this is the center of a cpider-net
    pub myAmCenter: bool,
    /// Information whether this is the center of a cpider-net
    pub myAmFringe: bool,
}

impl std::ops::Deref for NGNode {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NGNode {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl NGNode {
    /// C++: `void NGNode::NGNode(const std::string &)`
    /// Constructor
    // NGNode::NGNode(const std::string& id)
    //     : Named(id), myXID(-1), myYID(-1), myAmCenter(false), myAmFringe(false) {}
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        todo!("port NGNode::NGNode")
    }

    /// C++: `void NGNode::NGNode(const std::string &, int, int)`
    /// Constructor
    // NGNode::NGNode(const std::string& id, int xIDa, int yIDa)
    //     : Named(id), myXID(xIDa), myYID(yIDa), myAmCenter(false), myAmFringe(false) {}
    #[allow(unused_variables)]
    pub fn new_2(id: &str, xPos: i32, yPos: i32) -> Self {
        todo!("port NGNode::NGNode")
    }

    /// C++: `void NGNode::NGNode(const std::string &, int, int, bool)`
    /// Constructor
    // NGNode::NGNode(const std::string& id, int xIDa, int yIDa, bool amCenter)
    //     : Named(id), myXID(xIDa), myYID(yIDa), myAmCenter(amCenter), myAmFringe(false) {}
    #[allow(unused_variables)]
    pub fn new_3(id: &str, xID: i32, yID: i32, amCenter: bool) -> Self {
        todo!("port NGNode::NGNode")
    }

    /// C++: `void NGNode::~NGNode() noexcept` [virtual]
    /// Destructor
    // NGNode::~NGNode() {
    //     NGEdgeList::iterator li;
    //     while (myLinkList.size() != 0) {
    //         li = myLinkList.begin();
    //         delete (*li);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NGNode::~NGNode")
    }

    /// C++: `const Position & NGNode::getPosition() const` [const]
    /// Returns this node's position
    // const Position& getPosition() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position {
        todo!("port NGNode::getPosition")
    }

    /// C++: `int NGNode::getMaxNeighbours()`
    /// Returns this node's maximum neighbour number
    // int getMaxNeighbours() {
    //         return myMaxNeighbours;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_neighbours(&mut self) -> i32 {
        todo!("port NGNode::getMaxNeighbours")
    }

    /// C++: `void NGNode::setMaxNeighbours(int)`
    /// Sets this node's maximum neighbour number
    // void setMaxNeighbours(int value) {
    //         myMaxNeighbours = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_neighbours(&mut self, value: i32) {
        todo!("port NGNode::setMaxNeighbours")
    }

    /// C++: `void NGNode::setX(double)`
    /// Sets a new value for x-position
    // void setX(double x) {
    //         myPosition.set(x, myPosition.y());
    //     }
    #[allow(unused_variables)]
    pub fn set_x(&mut self, x: f64) {
        todo!("port NGNode::setX")
    }

    /// C++: `void NGNode::setY(double)`
    /// Sets a new value for y-position
    // void setY(double y) {
    //         myPosition.set(myPosition.x(), y);
    //     }
    #[allow(unused_variables)]
    pub fn set_y(&mut self, y: f64) {
        todo!("port NGNode::setY")
    }

    /// C++: `void NGNode::setFringe()`
    /// mark node as fringe
    // void setFringe() {
    //         myAmFringe = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_fringe(&mut self) {
        todo!("port NGNode::setFringe")
    }

    /// C++: `NBNode * NGNode::buildNBNode(NBNetBuilder &, const Position &) const` [const]
    /// Builds and returns this node's netbuild-representation
    // NBNode*
    // NGNode::buildNBNode(NBNetBuilder& nb, const Position& perturb) const {
    //     Position pos(myPosition + perturb);
    //     GeoConvHelper::getProcessing().x2cartesian(pos);
    //     // the center will have no logic!
    //     if (myAmCenter) {
    //         return new NBNode(myID, pos, SumoXMLNodeType::NOJUNCTION);
    //     }
    //     NBNode* node = nullptr;
    //     std::string typeS = OptionsCont::getOptions().isSet("default-junction-type") ?
    //                         OptionsCont::getOptions().getString("default-junction-type") : "";
    //
    //     if (SUMOXMLDefinitions::NodeTypes.hasString(typeS)) {
    //         const SumoXMLNodeType type = SUMOXMLDefinitions::NodeTypes.get(typeS);
    //         node = new NBNode(myID, pos, type);
    //
    //         // check whether it is a traffic light junction
    //         if (NBNode::isTrafficLight(type)) {
    //             const TrafficLightType tlType = SUMOXMLDefinitions::TrafficLightTypes.get(
    //                                                 OptionsCont::getOptions().getString("tls.default-type"));
    //             NBTrafficLightDefinition* tlDef = new NBOwnTLDef(myID, node, 0, tlType);
    //             if (!nb.getTLLogicCont().insert(tlDef)) {
    //                 // actually, nothing should fail here
    //                 delete tlDef;
    //                 throw ProcessError();
    //             }
    //         }
    //     } else {
    //         // otherwise netbuild may guess SumoXMLNodeType::TRAFFIC_LIGHT without actually building one
    //         node = new NBNode(myID, pos, SumoXMLNodeType::PRIORITY);
    //     }
    //     if (myAmFringe) {
    //         node->setFringeType(FringeType::OUTER);
    //     }
    //
    //     return node;
    // }
    #[allow(unused_variables)]
    pub fn build_nb_node(&self, nb: &mut NBNetBuilder, perturb: &Position) -> Ptr<NBNode> {
        todo!("port NGNode::buildNBNode")
    }

    /// C++: `void NGNode::addLink(NGEdge *)`
    /// Adds the given link to the internal list
    // void
    // NGNode::addLink(NGEdge* link) {
    //     myLinkList.push_back(link);
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<NGEdge>) {
        todo!("port NGNode::addLink")
    }

    /// C++: `void NGNode::removeLink(NGEdge *)`
    /// Removes the given link
    // void
    // NGNode::removeLink(NGEdge* link) {
    //     myLinkList.remove(link);
    // }
    #[allow(unused_variables)]
    pub fn remove_link(&mut self, link: Ptr<NGEdge>) {
        todo!("port NGNode::removeLink")
    }

    /// C++: `const NGEdgeList & NGNode::getLinks() const` [const]
    // const NGEdgeList& getLinks() const {
    //         return myLinkList;
    //     }
    #[allow(unused_variables)]
    pub fn get_links(&self) -> &NGEdgeList {
        todo!("port NGNode::getLinks")
    }

    /// C++: `bool NGNode::connected(const NGNode *const, const bool) const` [const]
    /// Returns whether the other node is connected
    // bool
    // NGNode::connected(const NGNode* const node, const bool withDir) const {
    //     for (NGEdge* ngEdge : myLinkList) {
    //         if (ngEdge->getStartNode() == this && ngEdge->getEndNode() == node) {
    //             return true;
    //         }
    //         if (!withDir && ngEdge->getEndNode() == this && ngEdge->getStartNode() == node) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn connected(&self, node: Ptr<NGNode>, withDir: bool) -> bool {
        todo!("port NGNode::connected")
    }

    /// C++: `bool NGNode::samePos(int, int) const` [const]
    /// Returns whether the node has the given position
    // bool samePos(int xPos, int yPos) const {
    //         return myXID == xPos && myYID == yPos;
    //     }
    #[allow(unused_variables)]
    pub fn same_pos(&self, xPos: i32, yPos: i32) -> bool {
        todo!("port NGNode::samePos")
    }

}

