//! Port of microsim/MSJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_internal_junction::MSInternalJunction;
use crate::microsim::ms_logic_junction::MSLogicJunction;
use crate::microsim::ms_no_logic_junction::MSNoLogicJunction;
use crate::microsim::ms_right_of_way_junction::MSRightOfWayJunction;

/// The base class for an intersection
/// C++ class `MSJunction` (microsim/MSJunction.h)
#[repr(C)]
pub struct MSJunction {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// Tye type of this junction
    pub myType: SumoXMLNodeType,
    /// The position of the junction
    pub myPosition: Position,
    /// The secondary position of the junction
    pub myPosition2: Position,
    /// The shape of the junction
    pub myShape: PositionVector,
    /// The (optional) junction name
    pub myName: String,
    pub myEmptyLinks: Vec<Ptr<MSLink>>,
    pub myEmptyLanes: Vec<Ptr<MSLane>>,
    /// incoming edges
    pub myIncoming: ConstMSEdgeVector,
    /// outgoing edges
    pub myOutgoing: ConstMSEdgeVector,
}

impl std::ops::Deref for MSJunction {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSJunction {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl crate::utils::router::router_edge::RouterNode for MSJunction {
    type Edge = MSEdge;
    fn id(&self) -> &str {
        self.get_id()
    }
    fn position(&self) -> &Position {
        self.get_position(false)
    }
    fn incoming(&self) -> &[Ptr<MSEdge>] {
        self.get_incoming()
    }
    fn outgoing(&self) -> &[Ptr<MSEdge>] {
        self.get_outgoing()
    }
}

impl MSJunction {
    /// Concrete class follows the node type, as in NLJunctionControlBuilder::closeJunction: these types get MSNoLogicJunction
    fn is_no_logic_type(&self) -> bool {
        matches!(
            self.myType,
            SumoXMLNodeType::NOJUNCTION | SumoXMLNodeType::DEAD_END | SumoXMLNodeType::DEAD_END_DEPRECATED | SumoXMLNodeType::DISTRICT | SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION
        )
    }

    /// C++ dynamic_cast<const MSRightOfWayJunction*>(this): every logic type except INTERNAL is built as MSRightOfWayJunction
    pub fn as_right_of_way(&self) -> Option<&MSRightOfWayJunction> {
        if self.is_no_logic_type() || self.myType == SumoXMLNodeType::INTERNAL { None } else { Ptr::new(self).cast::<MSRightOfWayJunction>().get() }
    }

    /// C++ dynamic_cast<const MSInternalJunction*>(this): built only for INTERNAL nodes
    pub fn as_internal(&self) -> Option<&MSInternalJunction> {
        if self.myType == SumoXMLNodeType::INTERNAL { Ptr::new(self).cast::<MSInternalJunction>().get() } else { None }
    }

    /// C++: `void MSJunction::MSJunction(const std::string &, SumoXMLNodeType, const Position &, const PositionVector &, const std::string &)`
    /// Constructor
    // MSJunction::MSJunction(const std::string& id, SumoXMLNodeType type, const Position& position,
    //                        const PositionVector& shape, const std::string& name) :
    //     Named(id),
    //     myType(type),
    //     myPosition(position),
    //     myShape(shape),
    //     myName(name) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, type_: SumoXMLNodeType, position: &Position, shape: &PositionVector, name: &str) -> Self {
        MSJunction {
            base: Named::new(id),
            base2: Parameterised::new(),
            myType: type_,
            myPosition: *position,
            myPosition2: Position::default(),
            myShape: shape.clone(),
            myName: name.to_string(),
            myEmptyLinks: Vec::new(),
            myEmptyLanes: Vec::new(),
            myIncoming: Vec::new(),
            myOutgoing: Vec::new(),
        }
    }

    /// C++: `void MSJunction::~MSJunction() noexcept` [virtual]
    /// Destructor.
    // MSJunction::~MSJunction() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release.
    }

    /// C++: `void MSJunction::postloadInit()` [virtual]
    /// performs some initialisation after the loading (e.g., link map computation)
    // void
    // MSJunction::postloadInit() {}
    #[allow(unused_variables)]
    pub fn postload_init(&mut self) {
        // C++ postloadInit body is empty; subclasses override it.
    }

    /// C++: `void MSJunction::addSecondaryPosition(const Position &)`
    /// used by the gui
    // void addSecondaryPosition(const Position& pos) {
    //         myPosition2 = pos;
    //     }
    #[allow(unused_variables)]
    pub fn add_secondary_position(&mut self, pos: &Position) {
        self.myPosition2 = *pos;
    }

    /// C++: `const Position & MSJunction::getPosition(bool) const` [const]
    /// returns the junction's position
    // const Position&
    // MSJunction::getPosition(bool secondaryShape) const {
    //     return secondaryShape ? myPosition2 : myPosition;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, secondaryShape: bool) -> &Position {
        if secondaryShape { &self.myPosition2 } else { &self.myPosition }
    }

    /// C++: `const PositionVector & MSJunction::getShape() const` [const]
    /// Returns this junction's shape
    // const PositionVector& getShape() const {
    //         return myShape;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape(&self) -> &PositionVector {
        &self.myShape
    }

    /// C++: `const std::string & MSJunction::getName() const` [const]
    /// return the junction name
    // const std::string& getName() const {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        self.myName.as_str()
    }

    /// C++: `const std::vector<MSLink *> & MSJunction::getFoeLinks(const MSLink *const) const` [const, virtual]
    // virtual const std::vector<MSLink*>& getFoeLinks(const MSLink* const /*srcLink*/) const {
    //         return myEmptyLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_links(&self, arg0: Ptr<MSLink>) -> &Vec<Ptr<MSLink>> {
        if let Some(rw) = self.as_right_of_way() {
            return rw.get_foe_links(arg0);
        }
        if let Some(ij) = self.as_internal() {
            return ij.get_foe_links(arg0);
        }
        &self.myEmptyLinks
    }

    /// C++: `const std::vector<MSLane *> & MSJunction::getFoeInternalLanes(const MSLink *const) const` [const, virtual]
    // virtual const std::vector<MSLane*>& getFoeInternalLanes(const MSLink* const /*srcLink*/) const {
    //         return myEmptyLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_internal_lanes(&self, arg0: Ptr<MSLink>) -> &Vec<Ptr<MSLane>> {
        if let Some(rw) = self.as_right_of_way() {
            return rw.get_foe_internal_lanes(arg0);
        }
        if let Some(ij) = self.as_internal() {
            return ij.get_foe_internal_lanes(arg0);
        }
        &self.myEmptyLanes
    }

    /// C++: `const ConstMSEdgeVector & MSJunction::getIncoming() const` [const]
    // inline const ConstMSEdgeVector& getIncoming() const {
    //         return myIncoming;
    //     }
    #[allow(unused_variables)]
    pub fn get_incoming(&self) -> &ConstMSEdgeVector {
        &self.myIncoming
    }

    /// C++: `int MSJunction::getNrOfIncomingLanes() const` [const]
    // int
    // MSJunction::getNrOfIncomingLanes() const {
    //     int nr = 0;
    //     for (const MSEdge* e : myIncoming) {
    //         nr += (int)e->getLanes().size();
    //     }
    //     return nr;
    // }
    #[allow(unused_variables)]
    pub fn get_nr_of_incoming_lanes(&self) -> i32 {
        let mut nr: i32 = 0;
        for e in &self.myIncoming {
            nr += e.get_lanes().len() as i32;
        }
        nr
    }

    /// C++: `const ConstMSEdgeVector & MSJunction::getOutgoing() const` [const]
    // inline const ConstMSEdgeVector& getOutgoing() const {
    //         return myOutgoing;
    //     }
    #[allow(unused_variables)]
    pub fn get_outgoing(&self) -> &ConstMSEdgeVector {
        &self.myOutgoing
    }

    /// C++: `const std::vector<MSLane *> MSJunction::getInternalLanes() const` [const, virtual]
    /// Returns all internal lanes on the junction
    // virtual const std::vector<MSLane*> getInternalLanes() const {
    //         return myEmptyLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_internal_lanes(&self) -> Vec<Ptr<MSLane>> {
        if self.is_no_logic_type() {
            Ptr::new(self).cast::<MSNoLogicJunction>().get().unwrap().get_internal_lanes()
        } else {
            Ptr::new(self).cast::<MSLogicJunction>().get().unwrap().get_internal_lanes()
        }
    }

    /// C++: `void MSJunction::addIncoming(MSEdge *)`
    // void addIncoming(MSEdge* edge) {
    //         myIncoming.push_back(edge);
    //     }
    #[allow(unused_variables)]
    pub fn add_incoming(&mut self, edge: Ptr<MSEdge>) {
        self.myIncoming.push(edge);
    }

    /// C++: `void MSJunction::addOutgoing(MSEdge *)`
    // void addOutgoing(MSEdge* edge) {
    //         myOutgoing.push_back(edge);
    //     }
    #[allow(unused_variables)]
    pub fn add_outgoing(&mut self, edge: Ptr<MSEdge>) {
        self.myOutgoing.push(edge);
    }

    /// C++: `SumoXMLNodeType MSJunction::getType() const` [const]
    /// return the type of this Junction
    // SumoXMLNodeType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> SumoXMLNodeType {
        self.myType
    }

    /// C++: `void MSJunction::passedJunction(const MSVehicle *)`
    /// erase vehicle from myLinkLeaders
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn passed_junction(&mut self, vehicle: Ptr<MSVehicle>) {
        // C++ declares MSJunction::passedJunction but no definition exists in the sources; nothing to port.
    }

    /// C++: `const MSJunctionLogic * MSJunction::getLogic() const` [const, virtual]
    // virtual const MSJunctionLogic* getLogic() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic(&self) -> Ptr<MSJunctionLogic> {
        match self.as_right_of_way() {
            Some(rw) => rw.get_logic(),
            None => Ptr::null(),
        }
    }

    // The C++ copy constructor and operator= are `= delete` (private): the type is intentionally non-copyable.
}

