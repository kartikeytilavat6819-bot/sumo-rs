//! Port of microsim/MSLogicJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::bitset<256> MSLogicJunction::LinkBits`
pub type MSLogicJunction_LinkBits = Unmapped /* std::bitset<256> */;

/// A junction which may not let all vehicles through, but must perform any kind of an operation to determine which cars are allowed to drive in this step.
/// C++ class `MSLogicJunction` (microsim/MSLogicJunction.h)
pub struct MSLogicJunction {
    /// C++ base class `MSJunction`
    pub base: MSJunction,
    /// list of incoming lanes
    pub myIncomingLanes: Vec<Ptr<MSLane>>,
    /// list of internal lanes
    pub myInternalLanes: Vec<Ptr<MSLane>>,
}

impl std::ops::Deref for MSLogicJunction {
    type Target = MSJunction;
    fn deref(&self) -> &MSJunction {
        &self.base
    }
}
impl std::ops::DerefMut for MSLogicJunction {
    fn deref_mut(&mut self) -> &mut MSJunction {
        &mut self.base
    }
}

impl MSLogicJunction {
    /// C++: `void MSLogicJunction::~MSLogicJunction() noexcept` [virtual]
    /// Destructor.
    // MSLogicJunction::~MSLogicJunction() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSLogicJunction::~MSLogicJunction")
    }

    /// C++: `void MSLogicJunction::postloadInit()` [virtual]
    /// initialises the junction after the whole net has been loaded
    // void
    // MSLogicJunction::postloadInit() {
    //     /*
    //     if(getID()=="1565") {
    //         int bla = 0;
    //     }
    //     // inform links where they have to report approaching vehicles to
    //     int requestPos = 0;
    //     std::vector<MSLane*>::iterator i;
    //     // going through the incoming lanes...
    //     for(i=myIncomingLanes.begin(); i!=myIncomingLanes.end(); ++i) {
    //         const MSLinkCont &links = (*i)->getLinkCont();
    //         // ... set information for every link
    //         for(MSLinkCont::const_iterator j=links.begin(); j!=links.end(); j++) {
    //             (*j)->setRequestInformation(&myRequest, requestPos,
    //                 &myRespond, requestPos/, clearInfo/);
    //             requestPos++;
    //         }
    //     }
    //     // set information for the internal lanes
    //     requestPos = 0;
    //     for(i=myInternalLanes.begin(); i!=myInternalLanes.end(); ++i) {
    //         // ... set information about participation
    //         static_cast<MSInternalLane*>(*i)->setParentJunctionInformation(
    //             &myInnerState, requestPos++);
    //     }
    //     */
    // }
    #[allow(unused_variables)]
    pub fn postload_init(&mut self) {
        todo!("port MSLogicJunction::postloadInit")
    }

    /// C++: `const std::vector<MSLane *> MSLogicJunction::getInternalLanes() const` [const, virtual]
    /// Returns all internal lanes on the junction
    // const std::vector<MSLane*>
    // MSLogicJunction::getInternalLanes() const {
    //     // Besides the lanes im myInternal lanes, which are only the last parts of the connections,
    //     // this collects all lanes on the junction
    //     std::vector<MSLane*> allInternalLanes;
    //     for (std::vector<MSLane*>::const_iterator i = myInternalLanes.begin(); i != myInternalLanes.end(); ++i) {
    //         MSLane* l = *i;
    //         while (l != nullptr) {
    //             allInternalLanes.push_back(l);
    //             const std::vector<MSLane::IncomingLaneInfo> incoming = l->getIncomingLanes();
    //             if (incoming.size() == 0) {
    //                 break;
    //             }
    //             assert(l->getIncomingLanes().size() == 1);
    //             l = l->getIncomingLanes()[0].lane;
    //             if (!l->isInternal()) {
    //                 break;
    //             }
    //         }
    //     }
    //     return allInternalLanes;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_lanes(&self) -> Vec<Ptr<MSLane>> {
        todo!("port MSLogicJunction::getInternalLanes")
    }

    /// C++: `void MSLogicJunction::MSLogicJunction(const std::string &, SumoXMLNodeType, const Position &, const PositionVector &, const std::string &, std::vector<MSLane *>, std::vector<MSLane *>)` [protected]
    /// Constructor
    // MSLogicJunction::MSLogicJunction(const std::string& id,
    //                                  SumoXMLNodeType type,
    //                                  const Position& position,
    //                                  const PositionVector& shape,
    //                                  const std::string& name,
    //                                  std::vector<MSLane*> incoming,
    //                                  std::vector<MSLane*> internal):
    //     MSJunction(id, type, position, shape, name),
    //     myIncomingLanes(incoming),
    //     myInternalLanes(internal) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, type_: SumoXMLNodeType, position: &Position, shape: &PositionVector, name: &str, incoming: Vec<Ptr<MSLane>>, internal: Vec<Ptr<MSLane>>) -> Self {
        todo!("port MSLogicJunction::MSLogicJunction")
    }

    /// C++: `void MSLogicJunction::MSLogicJunction(const MSLogicJunction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSLogicJunction) -> Self {
        todo!("port MSLogicJunction::MSLogicJunction")
    }

    /// C++: `MSLogicJunction & MSLogicJunction::operator=(const MSLogicJunction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSLogicJunction) -> &mut MSLogicJunction {
        todo!("port MSLogicJunction::operator=")
    }

}

