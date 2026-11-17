//! Port of utils/router/FlippedNode.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterNode};

/// C++ typedef `std::vector<const FlippedEdge<E, N, V> *> FlippedNode::ConstFlippedEdgeVector`
pub type FlippedNode_ConstFlippedEdgeVector<E, N, V> = Vec<Ptr<FlippedEdge<E, N, V>>>;

/// the node type representing nodes used for backward search
/// C++ class `FlippedNode` (utils/router/FlippedNode.h)
pub struct FlippedNode<E, N, V> {
    /// The original node
    pub myOriginalNode: Ptr<N>,
    /// The incoming edges
    pub myIncoming: FlippedNode_ConstFlippedEdgeVector<E, N, V>,
    /// The outgoing edges
    pub myOutgoing: FlippedNode_ConstFlippedEdgeVector<E, N, V>,
    _p: std::marker::PhantomData<(E, N, V,)>,
}

impl<E, N, V> FlippedNode<E, N, V> {
    /// C++: `void FlippedNode::FlippedNode<E, N, V>(const N *)`
    /// Constructor
    // FlippedNode(const N* originalNode) :
    //         myOriginalNode(originalNode) {}
    #[allow(unused_variables)]
    pub fn new(originalNode: Ptr<N>) -> Self {
        Self {
            myOriginalNode: originalNode,
            myIncoming: Vec::new(),
            myOutgoing: Vec::new(),
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void FlippedNode::~FlippedNode<E, N, V>()`
    /// Destructor
    // ~FlippedNode() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {

    }

    /// C++: `const Position & FlippedNode::getPosition() const` [const]
    /// Returns the position of the node
    // const Position& getPosition() const {
    //         return myOriginalNode->getPosition();
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position
    where
        N: RouterNode,
    {
        self.myOriginalNode.position()
    }

    /// C++: `const std::string & FlippedNode::getID() const` [const]
    /// Returns the id(entifier) of the node
    // const std::string& getID() const {
    //         return myOriginalNode->getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str
    where
        N: RouterNode,
    {
        self.myOriginalNode.id()
    }

    /// C++: `const ConstFlippedEdgeVector & FlippedNode::getIncoming() const` [const]
    /// Returns the incoming edges of the node
    // const ConstFlippedEdgeVector& getIncoming() const {
    //         if (myIncoming.empty()) {
    //             const std::vector<const E*>& incoming = myOriginalNode->getOutgoing();
    //             for (const E* edge : incoming) {
    //                 myIncoming.push_back(edge->getFlippedRoutingEdge());
    //             }
    //         }
    //         return myIncoming;
    //     }
    #[allow(unused_variables)]
    pub fn get_incoming(&self) -> &FlippedNode_ConstFlippedEdgeVector<E, N, V>
    where
        N: RouterNode<Edge = E>,
        E: RouterEdge<V, Node = N>,
    {
        if self.myIncoming.is_empty() {
            // C++ mutable cache: the const method fills myIncoming in place
            let me: &mut FlippedNode<E, N, V> = unsafe { &mut *(self as *const FlippedNode<E, N, V> as *mut FlippedNode<E, N, V>) };
            for edge in self.myOriginalNode.outgoing().iter() {
                me.myIncoming.push(edge.flipped_routing_edge());
            }
        }
        &self.myIncoming
    }

    /// C++: `const ConstFlippedEdgeVector & FlippedNode::getOutgoing() const` [const]
    /// Returns the outgoing edges of the node
    // const ConstFlippedEdgeVector& getOutgoing() const {
    //         if (myOutgoing.empty()) {
    //             const std::vector<const E*>& outgoing = myOriginalNode->getIncoming();
    //             for (const E* edge : outgoing) {
    //                 myOutgoing.push_back(edge->getFlippedRoutingEdge());
    //             }
    //         }
    //         return myOutgoing;
    //     }
    #[allow(unused_variables)]
    pub fn get_outgoing(&self) -> &FlippedNode_ConstFlippedEdgeVector<E, N, V>
    where
        N: RouterNode<Edge = E>,
        E: RouterEdge<V, Node = N>,
    {
        if self.myOutgoing.is_empty() {
            // C++ mutable cache: the const method fills myOutgoing in place
            let me: &mut FlippedNode<E, N, V> = unsafe { &mut *(self as *const FlippedNode<E, N, V> as *mut FlippedNode<E, N, V>) };
            for edge in self.myOriginalNode.incoming().iter() {
                me.myOutgoing.push(edge.flipped_routing_edge());
            }
        }
        &self.myOutgoing
    }

    /// C++: `const N * FlippedNode::getOriginalNode() const` [const]
    /// Returns the original node
    // const N* getOriginalNode() const {
    //         return myOriginalNode;
    //     }
    #[allow(unused_variables)]
    pub fn get_original_node(&self) -> Ptr<N> {
        self.myOriginalNode
    }

}

