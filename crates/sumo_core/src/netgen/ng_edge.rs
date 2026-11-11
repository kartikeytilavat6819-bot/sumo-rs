//! Port of netgen/NGEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::list<NGEdge *> NGEdgeList`
pub type NGEdgeList = VecDeque<Ptr<NGEdge>>;

/// A netgen-representation of an edge
/// C++ class `NGEdge` (netgen/NGEdge.h)
pub struct NGEdge {
    /// C++ base class `Named`
    pub base: Named,
    /// The node the edge starts at
    pub myStartNode: Ptr<NGNode>,
    /// The node the edge ends at
    pub myEndNode: Ptr<NGNode>,
    /// The id when building the reverse edge
    pub myReverseID: String,
}

impl std::ops::Deref for NGEdge {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NGEdge {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl NGEdge {
    /// C++: `void NGEdge::NGEdge(const std::string &, NGNode *, NGNode *, const std::string &)`
    /// Constructor
    // NGEdge::NGEdge(const std::string& id, NGNode* startNode, NGNode* endNode, const std::string& reverseID) :
    //     Named(id), myStartNode(startNode), myEndNode(endNode), myReverseID(reverseID == "" ? "-" + id : reverseID) {
    //     myStartNode->addLink(this);
    //     myEndNode->addLink(this);
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, startNode: Ptr<NGNode>, endNode: Ptr<NGNode>, reverseID: &str) -> Self {
        todo!("port NGEdge::NGEdge")
    }

    /// C++: `void NGEdge::~NGEdge() noexcept` [virtual]
    /// Destructor
    // NGEdge::~NGEdge() {
    //     myStartNode->removeLink(this);
    //     myEndNode->removeLink(this);
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NGEdge::~NGEdge")
    }

    /// C++: `NGNode * NGEdge::getStartNode() const` [const]
    /// Returns this link's start node
    // NGNode* getStartNode() const {
    //         return myStartNode;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_node(&self) -> Ptr<NGNode> {
        todo!("port NGEdge::getStartNode")
    }

    /// C++: `NGNode * NGEdge::getEndNode() const` [const]
    /// Returns this link's end node
    // NGNode* getEndNode() const {
    //         return myEndNode;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_node(&self) -> Ptr<NGNode> {
        todo!("port NGEdge::getEndNode")
    }

    /// C++: `NBEdge * NGEdge::buildNBEdge(NBNetBuilder &, std::string, const bool) const` [const]
    /// Builds and returns this link's netbuild-representation
    // NBEdge*
    // NGEdge::buildNBEdge(NBNetBuilder& nb, std::string type, const bool reversed) const {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.getBool("random-type") && nb.getTypeCont().size() > 1) {
    //         auto it = nb.getTypeCont().begin();
    //         std::advance(it, RandHelper::rand((int)nb.getTypeCont().size()));
    //         type = it->first;
    //     }
    //     int priority = nb.getTypeCont().getEdgeTypePriority(type);
    //     if (priority > 1 && oc.getBool("rand.random-priority")) {
    //         priority = RandHelper::rand(priority) + 1;
    //     }
    //     int lanenumber = nb.getTypeCont().getEdgeTypeNumLanes(type);
    //     if (lanenumber > 1 && oc.getBool("rand.random-lanenumber")) {
    //         lanenumber = RandHelper::rand(lanenumber) + 1;
    //     }
    //
    //     SVCPermissions permissions = nb.getTypeCont().getEdgeTypePermissions(type);
    //     LaneSpreadFunction lsf = nb.getTypeCont().getEdgeTypeSpreadType(type);
    //     if (lsf == LaneSpreadFunction::SPREAD_UNKNOWN) {
    //         lsf = SUMOXMLDefinitions::LaneSpreadFunctions.get(oc.getString("default.spreadtype"));
    //     }
    //     if (isRailway(permissions) && nb.getTypeCont().getEdgeTypeIsOneWay(type)) {
    //         lsf = LaneSpreadFunction::CENTER;
    //     }
    //     const double maxSegmentLength = oc.getFloat("geometry.max-segment-length");
    //     NBNode* from = nb.getNodeCont().retrieve(reversed ? myEndNode->getID() : myStartNode->getID());
    //     NBNode* to = nb.getNodeCont().retrieve(reversed ? myStartNode->getID() : myEndNode->getID());
    //     PositionVector shape;
    //     if (maxSegmentLength > 0) {
    //         shape.push_back(from->getPosition());
    //         shape.push_back(to->getPosition());
    //         // shape is already cartesian but we must use a copy because the original will be modified
    //         NBNetBuilder::addGeometrySegments(shape, PositionVector(shape), maxSegmentLength);
    //     }
    //     NBEdge* result = new NBEdge(
    //         reversed ? myReverseID : myID,
    //         from, to,
    //         type, nb.getTypeCont().getEdgeTypeSpeed(type), NBEdge::UNSPECIFIED_FRICTION, lanenumber,
    //         priority, nb.getTypeCont().getEdgeTypeWidth(type), NBEdge::UNSPECIFIED_OFFSET, shape, lsf);
    //     result->setPermissions(permissions);
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn build_nb_edge(&self, nb: &mut NBNetBuilder, type_: String, reversed: bool) -> Ptr<NBEdge> {
        todo!("port NGEdge::buildNBEdge")
    }

}

