//! Port of microsim/MSNoLogicJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This junctions let all vehicles past through so they only should be used on junctions where incoming vehicles are no foes to each other (may drive simultaneously).
/// C++ class `MSNoLogicJunction` (microsim/MSNoLogicJunction.h)
pub struct MSNoLogicJunction {
    /// C++ base class `MSJunction`
    pub base: MSJunction,
    /// Lanes incoming to the junction
    pub myIncomingLanes: Vec<Ptr<MSLane>>,
    /// The junctions internal lanes
    pub myInternalLanes: Vec<Ptr<MSLane>>,
}

impl std::ops::Deref for MSNoLogicJunction {
    type Target = MSJunction;
    fn deref(&self) -> &MSJunction {
        &self.base
    }
}
impl std::ops::DerefMut for MSNoLogicJunction {
    fn deref_mut(&mut self) -> &mut MSJunction {
        &mut self.base
    }
}

impl MSNoLogicJunction {
    /// C++: `void MSNoLogicJunction::~MSNoLogicJunction() noexcept` [virtual]
    /// Destructor.
    // MSNoLogicJunction::~MSNoLogicJunction() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSNoLogicJunction::~MSNoLogicJunction")
    }

    /// C++: `void MSNoLogicJunction::MSNoLogicJunction(const std::string &, SumoXMLNodeType, const Position &, const PositionVector &, const std::string &, std::vector<MSLane *>, std::vector<MSLane *>)`
    /// Constructor
    // MSNoLogicJunction::MSNoLogicJunction(const std::string& id,
    //                                      SumoXMLNodeType type,
    //                                      const Position& position,
    //                                      const PositionVector& shape,
    //                                      const std::string& name,
    //                                      std::vector<MSLane*> incoming, std::vector<MSLane*> internal):
    //     MSJunction(id, type, position, shape, name),
    //     myIncomingLanes(incoming),
    //     myInternalLanes(internal) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, type_: SumoXMLNodeType, position: &Position, shape: &PositionVector, name: &str, incoming: Vec<Ptr<MSLane>>, internal: Vec<Ptr<MSLane>>) -> Self {
        todo!("port MSNoLogicJunction::MSNoLogicJunction")
    }

    /// C++: `void MSNoLogicJunction::postloadInit()` [virtual]
    /// Initialises the junction after the net was completely loaded
    // void
    // MSNoLogicJunction::postloadInit() {
    //     // inform links where they have to report approaching vehicles to
    //     for (const MSLane* const l : myIncomingLanes) {
    //         for (MSLink* const link : l->getLinkCont()) {
    //             link->setRequestInformation(-1, false, false, std::vector<MSLink*>(), std::vector<MSLane*>());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn postload_init(&mut self) {
        todo!("port MSNoLogicJunction::postloadInit")
    }

    /// C++: `const std::vector<MSLane *> MSNoLogicJunction::getInternalLanes() const` [const, virtual]
    /// Returns all internal lanes on the junction
    // const std::vector<MSLane*>
    // MSNoLogicJunction::getInternalLanes() const {
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
        todo!("port MSNoLogicJunction::getInternalLanes")
    }

    /// C++: `void MSNoLogicJunction::MSNoLogicJunction(const MSNoLogicJunction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSNoLogicJunction) -> Self {
        todo!("port MSNoLogicJunction::MSNoLogicJunction")
    }

    /// C++: `MSNoLogicJunction & MSNoLogicJunction::operator=(const MSNoLogicJunction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSNoLogicJunction) -> &mut MSNoLogicJunction {
        todo!("port MSNoLogicJunction::operator=")
    }

}

