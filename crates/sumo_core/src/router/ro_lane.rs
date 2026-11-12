//! Port of router/ROLane.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A single lane the router may use
/// C++ class `ROLane` (router/ROLane.h)
#[repr(C)]
pub struct ROLane {
    /// C++ base class `Named`
    pub base: Named,
    /// The parent edge of this lane
    pub myEdge: Ptr<ROEdge>,
    /// The length of the lane
    pub myLength: f64,
    /// The maximum speed allowed on the lane
    pub myMaxSpeed: f64,
    /// The encoding of allowed vehicle classes
    pub myPermissions: SVCPermissions,
    pub myOutgoingLanes: Vec<(Ptr<ROLane>, Ptr<ROEdge>)>,
    /// shape for this lane
    pub myShape: PositionVector,
    /// precomputed myShape.length / myLength
    pub myLengthGeometryFactor: f64,
}

impl std::ops::Deref for ROLane {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for ROLane {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl ROLane {
    /// C++: `void ROLane::ROLane(const std::string &, ROEdge *, double, double, SVCPermissions, const PositionVector &)`
    /// Constructor
    // ROLane(const std::string& id, ROEdge* edge, double length, double maxSpeed, SVCPermissions permissions, const PositionVector& shape) :
    //         Named(id), myEdge(edge), myLength(length), myMaxSpeed(maxSpeed), myPermissions(permissions), myShape(shape),
    //         myLengthGeometryFactor(MAX2(NUMERICAL_EPS, myShape.length() / myLength)) // factor should not be 0
    //     { }
    #[allow(unused_variables)]
    pub fn new(id: &str, edge: Ptr<ROEdge>, length: f64, maxSpeed: f64, permissions: SVCPermissions, shape: &PositionVector) -> Self {
        let factor = shape.length() / length;
        ROLane {
            base: Named::new(id),
            myEdge: edge,
            myLength: length,
            myMaxSpeed: maxSpeed,
            myPermissions: permissions,
            myOutgoingLanes: Vec::new(),
            myShape: shape.clone(),
            // factor should not be 0; MAX2(NUMERICAL_EPS, factor)
            myLengthGeometryFactor: if utils::geom::NUMERICAL_EPS > factor { utils::geom::NUMERICAL_EPS } else { factor },
        }
    }

    /// C++: `void ROLane::~ROLane() noexcept` [virtual]
    /// Destructor
    // ~ROLane() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `double ROLane::getLength() const` [const]
    /// Returns the length of the lane
    // double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `double ROLane::getSpeed() const` [const]
    /// Returns the maximum speed allowed on this lane
    // double getSpeed() const {
    //         return myMaxSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        self.myMaxSpeed
    }

    /// C++: `SVCPermissions ROLane::getPermissions() const` [const]
    /// Returns the list of allowed vehicle classes
    // inline SVCPermissions getPermissions() const {
    //         return myPermissions;
    //     }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
        self.myPermissions
    }

    /// C++: `void ROLane::setPermissions(SVCPermissions)`
    // void  setPermissions(SVCPermissions permissions) {
    //         myPermissions = permissions;
    //     }
    #[allow(unused_variables)]
    pub fn set_permissions(&mut self, permissions: SVCPermissions) {
        self.myPermissions = permissions;
    }

    /// C++: `ROEdge & ROLane::getEdge() const` [const]
    /// Returns the lane's edge
    // ROEdge& getEdge() const {
    //         return *myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> &mut ROEdge {
        self.myEdge.m()
    }

    /// C++: `const std::vector<std::pair<const ROLane *, const ROEdge *>> & ROLane::getOutgoingViaLanes() const` [const]
    /// get the map of outgoing lanes to via edges
    // const std::vector<std::pair<const ROLane*, const ROEdge*> >& getOutgoingViaLanes() const {
    //         return myOutgoingLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_outgoing_via_lanes(&self) -> &Vec<(Ptr<ROLane>, Ptr<ROEdge>)> {
        &self.myOutgoingLanes
    }

    /// C++: `void ROLane::addOutgoingLane(ROLane *, ROEdge *)`
    // void addOutgoingLane(ROLane* lane, ROEdge* via = nullptr) {
    //         myOutgoingLanes.push_back(std::make_pair(lane, via));
    //     }
    #[allow(unused_variables)]
    pub fn add_outgoing_lane(&mut self, lane: Ptr<ROLane>, via: Ptr<ROEdge>) {
        self.myOutgoingLanes.push((lane, via));
    }

    /// C++: `LinkState ROLane::getIncomingLinkState() const` [const]
    /// get the state of the link from the logical predecessor to this lane (ignored for routing)
    // inline LinkState getIncomingLinkState() const {
    //         return LINKSTATE_MAJOR;
    //     }
    #[allow(unused_variables)]
    pub fn get_incoming_link_state(&self) -> LinkState {
        LINKSTATE_MAJOR
    }

    /// C++: `bool ROLane::allowsVehicleClass(SUMOVehicleClass) const` [const]
    // inline bool allowsVehicleClass(SUMOVehicleClass vclass) const {
    //         return (myPermissions & vclass) == vclass;
    //     }
    #[allow(unused_variables)]
    pub fn allows_vehicle_class(&self, vclass: SUMOVehicleClass) -> bool {
        (self.myPermissions & vclass) == vclass
    }

    /// C++: `const PositionVector & ROLane::getShape() const` [const]
    // const PositionVector& getShape() const {
    //         return myShape;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape(&self) -> &PositionVector {
        &self.myShape
    }

    /// C++: `double ROLane::interpolateLanePosToGeometryPos(double) const` [const]
    // inline double interpolateLanePosToGeometryPos(double lanePos) const {
    //         return lanePos * myLengthGeometryFactor;
    //     }
    #[allow(unused_variables)]
    pub fn interpolate_lane_pos_to_geometry_pos(&self, lanePos: f64) -> f64 {
        lanePos * self.myLengthGeometryFactor
    }

    /// C++: `const Position ROLane::geometryPositionAtOffset(double, double) const` [const]
    // inline const Position geometryPositionAtOffset(double offset, double lateralOffset = 0) const {
    //         return myShape.positionAtOffset(interpolateLanePosToGeometryPos(offset), lateralOffset);
    //     }
    #[allow(unused_variables)]
    pub fn geometry_position_at_offset(&self, offset: f64, lateralOffset: f64) -> Position {
        self.myShape.position_at_offset(self.interpolate_lane_pos_to_geometry_pos(offset), lateralOffset)
    }

    /// C++: `void ROLane::ROLane(const ROLane &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROLane) -> Self {
        // C++ copy constructor is private and never defined: copying is disallowed
        panic!("ROLane is not copyable (private copy constructor in C++)")
    }

    /// C++: `ROLane & ROLane::operator=(const ROLane &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROLane) -> &mut ROLane {
        // C++ assignment operator is private and never defined: assignment is disallowed
        panic!("ROLane is not assignable (private operator= in C++)")
    }

}

