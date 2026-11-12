//! Port of netgen/NGNode.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::options::options_cont;
use utils::xml::sumo_xml_definitions::{NodeTypes, TrafficLightTypes};

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
        NGNode { base: Named::new(id), myXID: -1, myYID: -1, myLinkList: VecDeque::new(), myPosition: Position::default(), myMaxNeighbours: 0, myAmCenter: false, myAmFringe: false }
    }

    /// C++: `void NGNode::NGNode(const std::string &, int, int)`
    /// Constructor
    // NGNode::NGNode(const std::string& id, int xIDa, int yIDa)
    //     : Named(id), myXID(xIDa), myYID(yIDa), myAmCenter(false), myAmFringe(false) {}
    #[allow(unused_variables)]
    pub fn new_2(id: &str, xPos: i32, yPos: i32) -> Self {
        NGNode { base: Named::new(id), myXID: xPos, myYID: yPos, myLinkList: VecDeque::new(), myPosition: Position::default(), myMaxNeighbours: 0, myAmCenter: false, myAmFringe: false }
    }

    /// C++: `void NGNode::NGNode(const std::string &, int, int, bool)`
    /// Constructor
    // NGNode::NGNode(const std::string& id, int xIDa, int yIDa, bool amCenter)
    //     : Named(id), myXID(xIDa), myYID(yIDa), myAmCenter(amCenter), myAmFringe(false) {}
    #[allow(unused_variables)]
    pub fn new_3(id: &str, xID: i32, yID: i32, amCenter: bool) -> Self {
        NGNode { base: Named::new(id), myXID: xID, myYID: yID, myLinkList: VecDeque::new(), myPosition: Position::default(), myMaxNeighbours: 0, myAmCenter: amCenter, myAmFringe: false }
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
        // delete (*li): ~NGEdge removes the link from both end nodes, then the storage is freed
        while self.myLinkList.len() != 0 {
            let li = *self.myLinkList.front().unwrap();
            li.m().drop_();
            drop(unsafe { Box::from_raw(li.as_raw()) });
        }
    }

    /// C++: `const Position & NGNode::getPosition() const` [const]
    /// Returns this node's position
    // const Position& getPosition() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position {
        &self.myPosition
    }

    /// C++: `int NGNode::getMaxNeighbours()`
    /// Returns this node's maximum neighbour number
    // int getMaxNeighbours() {
    //         return myMaxNeighbours;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_neighbours(&mut self) -> i32 {
        self.myMaxNeighbours
    }

    /// C++: `void NGNode::setMaxNeighbours(int)`
    /// Sets this node's maximum neighbour number
    // void setMaxNeighbours(int value) {
    //         myMaxNeighbours = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_neighbours(&mut self, value: i32) {
        self.myMaxNeighbours = value;
    }

    /// C++: `void NGNode::setX(double)`
    /// Sets a new value for x-position
    // void setX(double x) {
    //         myPosition.set(x, myPosition.y());
    //     }
    #[allow(unused_variables)]
    pub fn set_x(&mut self, x: f64) {
        let y = self.myPosition.y();
        self.myPosition.set(x, y);
    }

    /// C++: `void NGNode::setY(double)`
    /// Sets a new value for y-position
    // void setY(double y) {
    //         myPosition.set(myPosition.x(), y);
    //     }
    #[allow(unused_variables)]
    pub fn set_y(&mut self, y: f64) {
        let x = self.myPosition.x();
        self.myPosition.set(x, y);
    }

    /// C++: `void NGNode::setFringe()`
    /// mark node as fringe
    // void setFringe() {
    //         myAmFringe = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_fringe(&mut self) {
        self.myAmFringe = true;
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
        let mut pos = self.myPosition + *perturb;
        GeoConvHelper::with_processing(|g| {
            g.x2cartesian(&mut pos, true);
        });
        // the center will have no logic!
        if self.myAmCenter {
            return Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(self.get_id(), &pos, SumoXMLNodeType::NOJUNCTION))));
        }
        let typeS = if options_cont::is_set("default-junction-type") { options_cont::get_string("default-junction-type") } else { String::new() };

        let node: Ptr<NBNode>;
        if NodeTypes.has_string(&typeS) {
            let type_ = NodeTypes.get(&typeS).unwrap();
            node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(self.get_id(), &pos, type_))));

            // check whether it is a traffic light junction
            if NBNode::is_traffic_light(type_) {
                let tlType = TrafficLightTypes.get(&options_cont::get_string("tls.default-type")).unwrap();
                let tlDef: Ptr<NBOwnTLDef> = Ptr::from_raw(Box::into_raw(Box::new(NBOwnTLDef::new_2(self.get_id(), node, 0, tlType))));
                if !nb.get_tl_logic_cont().insert(tlDef.cast::<NBTrafficLightDefinition>(), false) {
                    // actually, nothing should fail here
                    drop(unsafe { Box::from_raw(tlDef.as_raw()) });
                    std::panic::panic_any(ProcessError::default_msg());
                }
            }
        } else {
            // otherwise netbuild may guess SumoXMLNodeType::TRAFFIC_LIGHT without actually building one
            node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new(self.get_id(), &pos, SumoXMLNodeType::PRIORITY))));
        }
        if self.myAmFringe {
            node.m().set_fringe_type(FringeType::OUTER);
        }

        node
    }

    /// C++: `void NGNode::addLink(NGEdge *)`
    /// Adds the given link to the internal list
    // void
    // NGNode::addLink(NGEdge* link) {
    //     myLinkList.push_back(link);
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<NGEdge>) {
        self.myLinkList.push_back(link);
    }

    /// C++: `void NGNode::removeLink(NGEdge *)`
    /// Removes the given link
    // void
    // NGNode::removeLink(NGEdge* link) {
    //     myLinkList.remove(link);
    // }
    #[allow(unused_variables)]
    pub fn remove_link(&mut self, link: Ptr<NGEdge>) {
        self.myLinkList.retain(|l| *l != link);
    }

    /// C++: `const NGEdgeList & NGNode::getLinks() const` [const]
    // const NGEdgeList& getLinks() const {
    //         return myLinkList;
    //     }
    #[allow(unused_variables)]
    pub fn get_links(&self) -> &NGEdgeList {
        &self.myLinkList
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
        let this = Ptr::new(self);
        for ngEdge in self.myLinkList.iter() {
            if ngEdge.get_start_node() == this && ngEdge.get_end_node() == node {
                return true;
            }
            if !withDir && ngEdge.get_end_node() == this && ngEdge.get_start_node() == node {
                return true;
            }
        }
        false
    }

    /// C++: `bool NGNode::samePos(int, int) const` [const]
    /// Returns whether the node has the given position
    // bool samePos(int xPos, int yPos) const {
    //         return myXID == xPos && myYID == yPos;
    //     }
    #[allow(unused_variables)]
    pub fn same_pos(&self, xPos: i32, yPos: i32) -> bool {
        self.myXID == xPos && self.myYID == yPos
    }

}

