//! Port of microsim/MSJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The base class for an intersection
/// C++ class `MSJunction` (microsim/MSJunction.h)
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

impl MSJunction {
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
        todo!("port MSJunction::MSJunction")
    }

    /// C++: `void MSJunction::~MSJunction() noexcept` [virtual]
    /// Destructor.
    // MSJunction::~MSJunction() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSJunction::~MSJunction")
    }

    /// C++: `void MSJunction::postloadInit()` [virtual]
    /// performs some initialisation after the loading (e.g., link map computation)
    // void
    // MSJunction::postloadInit() {}
    #[allow(unused_variables)]
    pub fn postload_init(&mut self) {
        todo!("port MSJunction::postloadInit")
    }

    /// C++: `void MSJunction::addSecondaryPosition(const Position &)`
    /// used by the gui
    // void addSecondaryPosition(const Position& pos) {
    //         myPosition2 = pos;
    //     }
    #[allow(unused_variables)]
    pub fn add_secondary_position(&mut self, pos: &Position) {
        todo!("port MSJunction::addSecondaryPosition")
    }

    /// C++: `const Position & MSJunction::getPosition(bool) const` [const]
    /// returns the junction's position
    // const Position&
    // MSJunction::getPosition(bool secondaryShape) const {
    //     return secondaryShape ? myPosition2 : myPosition;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, secondaryShape: bool) -> &Position {
        todo!("port MSJunction::getPosition")
    }

    /// C++: `const PositionVector & MSJunction::getShape() const` [const]
    /// Returns this junction's shape
    // const PositionVector& getShape() const {
    //         return myShape;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape(&self) -> &PositionVector {
        todo!("port MSJunction::getShape")
    }

    /// C++: `const std::string & MSJunction::getName() const` [const]
    /// return the junction name
    // const std::string& getName() const {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        todo!("port MSJunction::getName")
    }

    /// C++: `const std::vector<MSLink *> & MSJunction::getFoeLinks(const MSLink *const) const` [const, virtual]
    // virtual const std::vector<MSLink*>& getFoeLinks(const MSLink* const /*srcLink*/) const {
    //         return myEmptyLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_links(&self, arg0: Ptr<MSLink>) -> &Vec<Ptr<MSLink>> {
        todo!("port MSJunction::getFoeLinks")
    }

    /// C++: `const std::vector<MSLane *> & MSJunction::getFoeInternalLanes(const MSLink *const) const` [const, virtual]
    // virtual const std::vector<MSLane*>& getFoeInternalLanes(const MSLink* const /*srcLink*/) const {
    //         return myEmptyLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_internal_lanes(&self, arg0: Ptr<MSLink>) -> &Vec<Ptr<MSLane>> {
        todo!("port MSJunction::getFoeInternalLanes")
    }

    /// C++: `const ConstMSEdgeVector & MSJunction::getIncoming() const` [const]
    // inline const ConstMSEdgeVector& getIncoming() const {
    //         return myIncoming;
    //     }
    #[allow(unused_variables)]
    pub fn get_incoming(&self) -> &ConstMSEdgeVector {
        todo!("port MSJunction::getIncoming")
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
        todo!("port MSJunction::getNrOfIncomingLanes")
    }

    /// C++: `const ConstMSEdgeVector & MSJunction::getOutgoing() const` [const]
    // inline const ConstMSEdgeVector& getOutgoing() const {
    //         return myOutgoing;
    //     }
    #[allow(unused_variables)]
    pub fn get_outgoing(&self) -> &ConstMSEdgeVector {
        todo!("port MSJunction::getOutgoing")
    }

    /// C++: `const std::vector<MSLane *> MSJunction::getInternalLanes() const` [const, virtual]
    /// Returns all internal lanes on the junction
    // virtual const std::vector<MSLane*> getInternalLanes() const {
    //         return myEmptyLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_internal_lanes(&self) -> Vec<Ptr<MSLane>> {
        todo!("port MSJunction::getInternalLanes")
    }

    /// C++: `void MSJunction::addIncoming(MSEdge *)`
    // void addIncoming(MSEdge* edge) {
    //         myIncoming.push_back(edge);
    //     }
    #[allow(unused_variables)]
    pub fn add_incoming(&mut self, edge: Ptr<MSEdge>) {
        todo!("port MSJunction::addIncoming")
    }

    /// C++: `void MSJunction::addOutgoing(MSEdge *)`
    // void addOutgoing(MSEdge* edge) {
    //         myOutgoing.push_back(edge);
    //     }
    #[allow(unused_variables)]
    pub fn add_outgoing(&mut self, edge: Ptr<MSEdge>) {
        todo!("port MSJunction::addOutgoing")
    }

    /// C++: `SumoXMLNodeType MSJunction::getType() const` [const]
    /// return the type of this Junction
    // SumoXMLNodeType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> SumoXMLNodeType {
        todo!("port MSJunction::getType")
    }

    /// C++: `void MSJunction::passedJunction(const MSVehicle *)`
    /// erase vehicle from myLinkLeaders
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn passed_junction(&mut self, vehicle: Ptr<MSVehicle>) {
        todo!("port MSJunction::passedJunction")
    }

    /// C++: `const MSJunctionLogic * MSJunction::getLogic() const` [const, virtual]
    // virtual const MSJunctionLogic* getLogic() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic(&self) -> Ptr<MSJunctionLogic> {
        todo!("port MSJunction::getLogic")
    }

    /// C++: `void MSJunction::MSJunction(const MSJunction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSJunction) -> Self {
        todo!("port MSJunction::MSJunction")
    }

    /// C++: `MSJunction & MSJunction::operator=(const MSJunction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSJunction) -> &mut MSJunction {
        todo!("port MSJunction::operator=")
    }

}

