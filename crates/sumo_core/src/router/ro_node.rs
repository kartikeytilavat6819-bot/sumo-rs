//! Port of router/RONode.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Base class for nodes used by the router
/// C++ class `RONode` (router/RONode.h)
#[repr(C)]
pub struct RONode {
    /// C++ base class `Named`
    pub base: Named,
    /// This node's position
    pub myPosition: Position,
    /// Incoming edges
    pub myIncoming: ConstROEdgeVector,
    /// Outgoing edges
    pub myOutgoing: ConstROEdgeVector,
    /// Flipped routing node
    pub myFlippedRoutingNode: Ptr<FlippedNode<ROEdge, RONode, ROVehicle>>,
}

impl std::ops::Deref for RONode {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for RONode {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl RONode {
    /// C++: `void RONode::RONode(const std::string &)`
    /// Constructor
    // RONode::RONode(const std::string& id)
    //     : Named(id) {}
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        RONode { base: Named::new(id), myPosition: Position::default(), myIncoming: Vec::new(), myOutgoing: Vec::new(), myFlippedRoutingNode: Ptr::null() }
    }

    /// C++: `void RONode::~RONode() noexcept` [virtual]
    /// Destructor
    // RONode::~RONode() {
    //     delete myFlippedRoutingNode;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myFlippedRoutingNode.is_null() {
            // delete myFlippedRoutingNode (allocated with Box in getFlippedRoutingNode)
            drop(unsafe { Box::from_raw(self.myFlippedRoutingNode.as_raw()) });
        }
    }

    /// C++: `void RONode::setPosition(const Position &)`
    /// Sets the position of the node
    // void
    // RONode::setPosition(const Position& p) {
    //     myPosition = p;
    // }
    #[allow(unused_variables)]
    pub fn set_position(&mut self, p: &Position) {
        self.myPosition = *p;
    }

    /// C++: `const Position & RONode::getPosition() const` [const]
    /// Returns the position of the node
    // const Position& getPosition() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position {
        &self.myPosition
    }

    /// C++: `const ConstROEdgeVector & RONode::getIncoming() const` [const]
    // inline const ConstROEdgeVector& getIncoming() const {
    //         return myIncoming;
    //     }
    #[allow(unused_variables)]
    pub fn get_incoming(&self) -> &ConstROEdgeVector {
        &self.myIncoming
    }

    /// C++: `const ConstROEdgeVector & RONode::getOutgoing() const` [const]
    // inline const ConstROEdgeVector& getOutgoing() const {
    //         return myOutgoing;
    //     }
    #[allow(unused_variables)]
    pub fn get_outgoing(&self) -> &ConstROEdgeVector {
        &self.myOutgoing
    }

    /// C++: `void RONode::addIncoming(ROEdge *)`
    // void addIncoming(ROEdge* edge) {
    //         myIncoming.push_back(edge);
    //     }
    #[allow(unused_variables)]
    pub fn add_incoming(&mut self, edge: Ptr<ROEdge>) {
        self.myIncoming.push(edge);
    }

    /// C++: `void RONode::addOutgoing(ROEdge *)`
    // void addOutgoing(ROEdge* edge) {
    //         myOutgoing.push_back(edge);
    //     }
    #[allow(unused_variables)]
    pub fn add_outgoing(&mut self, edge: Ptr<ROEdge>) {
        self.myOutgoing.push(edge);
    }

    /// C++: `FlippedNode<ROEdge, RONode, ROVehicle> * RONode::getFlippedRoutingNode() const` [const]
    // FlippedNode<ROEdge, RONode, ROVehicle>* getFlippedRoutingNode() const {
    //         if (myFlippedRoutingNode == nullptr) {
    //             myFlippedRoutingNode = new FlippedNode<ROEdge, RONode, ROVehicle>(this);
    //         }
    //         return myFlippedRoutingNode;
    //     }
    #[allow(unused_variables)]
    pub fn get_flipped_routing_node(&self) -> Ptr<FlippedNode<ROEdge, RONode, ROVehicle>> {
        let node = Ptr::new(self);
        if node.myFlippedRoutingNode.is_null() {
            node.m().myFlippedRoutingNode = Ptr::from_raw(Box::into_raw(Box::new(FlippedNode::new(node))));
        }
        self.myFlippedRoutingNode
    }

    /// C++: `void RONode::RONode(const RONode &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RONode) -> Self {
        // C++ copy constructor is private and never defined: copying is disallowed
        panic!("RONode is not copyable (private copy constructor in C++)")
    }

    /// C++: `RONode & RONode::operator=(const RONode &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RONode) -> &mut RONode {
        // C++ assignment operator is private and never defined: assignment is disallowed
        panic!("RONode is not assignable (private operator= in C++)")
    }

}

