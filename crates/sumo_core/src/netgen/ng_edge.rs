//! Port of netgen/NGEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::rand_helper::with_default;
use utils::common::sumo_vehicle_class::is_railway;
use utils::options::options_cont::{get_bool, get_float, get_string};
use utils::xml::sumo_xml_definitions::LaneSpreadFunctions;

/// C++ typedef `std::list<NGEdge *> NGEdgeList`
pub type NGEdgeList = VecDeque<Ptr<NGEdge>>;

/// A netgen-representation of an edge
/// C++ class `NGEdge` (netgen/NGEdge.h)
#[repr(C)]
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
    // Returns Box<Self>: the constructor registers `this` with both nodes via addLink.
    #[allow(unused_variables)]
    pub fn new(id: &str, startNode: Ptr<NGNode>, endNode: Ptr<NGNode>, reverseID: &str) -> Box<Self> {
        let b = Box::new(NGEdge {
            base: Named::new(id),
            myStartNode: startNode,
            myEndNode: endNode,
            myReverseID: if reverseID == "" { format!("-{id}") } else { reverseID.to_string() },
        });
        startNode.m().add_link(Ptr::from_box(&b));
        endNode.m().add_link(Ptr::from_box(&b));
        b
    }

    /// C++: `void NGEdge::~NGEdge() noexcept` [virtual]
    /// Destructor
    // NGEdge::~NGEdge() {
    //     myStartNode->removeLink(this);
    //     myEndNode->removeLink(this);
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        let this = Ptr::from_mut(self);
        let (start, end) = (self.myStartNode, self.myEndNode);
        start.m().remove_link(this);
        end.m().remove_link(this);
    }

    /// C++: `NGNode * NGEdge::getStartNode() const` [const]
    /// Returns this link's start node
    // NGNode* getStartNode() const {
    //         return myStartNode;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_node(&self) -> Ptr<NGNode> {
        self.myStartNode
    }

    /// C++: `NGNode * NGEdge::getEndNode() const` [const]
    /// Returns this link's end node
    // NGNode* getEndNode() const {
    //         return myEndNode;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_node(&self) -> Ptr<NGNode> {
        self.myEndNode
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
        let mut type_ = type_;
        if get_bool("random-type") && nb.get_type_cont().size() > 1 {
            let n = nb.get_type_cont().size();
            let idx = with_default(|r| r.rand_int(n));
            type_ = nb.get_type_cont().begin().nth(idx as usize).unwrap().0.clone();
        }
        let mut priority = nb.get_type_cont().get_edge_type_priority(&type_);
        if priority > 1 && get_bool("rand.random-priority") {
            priority = with_default(|r| r.rand_int(priority)) + 1;
        }
        let mut lanenumber = nb.get_type_cont().get_edge_type_num_lanes(&type_);
        if lanenumber > 1 && get_bool("rand.random-lanenumber") {
            lanenumber = with_default(|r| r.rand_int(lanenumber)) + 1;
        }

        let permissions: SVCPermissions = nb.get_type_cont().get_edge_type_permissions(&type_);
        let mut lsf: LaneSpreadFunction = nb.get_type_cont().get_edge_type_spread_type(&type_);
        if lsf == LaneSpreadFunction::SPREAD_UNKNOWN {
            lsf = LaneSpreadFunctions.get(&get_string("default.spreadtype")).unwrap();
        }
        if is_railway(permissions) && nb.get_type_cont().get_edge_type_is_one_way(&type_) {
            lsf = LaneSpreadFunction::CENTER;
        }
        let maxSegmentLength: f64 = get_float("geometry.max-segment-length");
        let from: Ptr<NBNode> = nb.get_node_cont().retrieve(if reversed { self.myEndNode.get_id() } else { self.myStartNode.get_id() });
        let to: Ptr<NBNode> = nb.get_node_cont().retrieve(if reversed { self.myStartNode.get_id() } else { self.myEndNode.get_id() });
        let mut shape: PositionVector = PositionVector::default();
        if maxSegmentLength > 0.0 {
            shape.push(from.get_position().clone());
            shape.push(to.get_position().clone());
            // shape is already cartesian but we must use a copy because the original will be modified
            let copy = PositionVector::from(shape.to_vec());
            NBNetBuilder::add_geometry_segments(&mut shape, &copy, maxSegmentLength);
        }
        let id: &str = if reversed { self.myReverseID.as_str() } else { self.get_id() };
        let speed = nb.get_type_cont().get_edge_type_speed(&type_);
        let width = nb.get_type_cont().get_edge_type_width(&type_);
        let mut result = NBEdge::new_2(id, from, to, type_, speed, UNSPECIFIED_FRICTION, lanenumber, priority, width, UNSPECIFIED_OFFSET, shape, lsf, "", "", false);
        result.set_permissions(permissions, -1);
        Ptr::from_raw(Box::into_raw(result))
    }

}

