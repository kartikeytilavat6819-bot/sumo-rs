//! Port of microsim/MSInternalJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A class which realises junctions that do regard any kind of a right-of-way. The rules for the right-of-way themselves are stored within the associated "MSJunctionLogic" - structure.
/// C++ class `MSInternalJunction` (microsim/MSInternalJunction.h)
pub struct MSInternalJunction {
    /// C++ base class `MSLogicJunction`
    pub base: MSLogicJunction,
    pub myInternalLinkFoes: Vec<Ptr<MSLink>>,
    pub myInternalLaneFoes: Vec<Ptr<MSLane>>,
}

impl std::ops::Deref for MSInternalJunction {
    type Target = MSLogicJunction;
    fn deref(&self) -> &MSLogicJunction {
        &self.base
    }
}
impl std::ops::DerefMut for MSInternalJunction {
    fn deref_mut(&mut self) -> &mut MSLogicJunction {
        &mut self.base
    }
}

impl MSInternalJunction {
    /// C++: `void MSInternalJunction::MSInternalJunction(const std::string &, SumoXMLNodeType, const Position &, const PositionVector &, std::vector<MSLane *>, std::vector<MSLane *>)`
    /// Constructor
    // MSInternalJunction::MSInternalJunction(const std::string& id,
    //                                        SumoXMLNodeType type,
    //                                        const Position& position,
    //                                        const PositionVector& shape,
    //                                        std::vector<MSLane*> incoming,
    //                                        std::vector<MSLane*> internal)
    //     : MSLogicJunction(id, type, position, shape, "", incoming, internal) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, type_: SumoXMLNodeType, position: &Position, shape: &PositionVector, incoming: Vec<Ptr<MSLane>>, internal: Vec<Ptr<MSLane>>) -> Self {
        todo!("port MSInternalJunction::MSInternalJunction")
    }

    /// C++: `void MSInternalJunction::~MSInternalJunction() noexcept` [virtual]
    /// Destructor.
    // MSInternalJunction::~MSInternalJunction() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSInternalJunction::~MSInternalJunction")
    }

    /// C++: `void MSInternalJunction::postloadInit()` [virtual]
    // void
    // MSInternalJunction::postloadInit() {
    //     if (myIncomingLanes.size() == 0) {
    //         throw ProcessError(TLF("Internal junction % has no incoming lanes", getID()));
    //     }
    //     // the first lane in the list of incoming lanes is special. It defines the
    //     // link that needs to do all the checking for this internal junction
    //     const MSLane* specialLane = myIncomingLanes[0];
    //     assert(specialLane->getLinkCont().size() == 1);
    //     MSLink* thisLink = specialLane->getLinkCont()[0];
    //     const MSRightOfWayJunction* parent = dynamic_cast<const MSRightOfWayJunction*>(specialLane->getEdge().getToJunction());
    //     if (parent == nullptr) {
    //         // parent has type traffic_light_unregulated
    //         return;
    //     }
    //     const int ownLinkIndex = specialLane->getIncomingLanes()[0].viaLink->getIndex();
    //     const MSLogicJunction::LinkBits& response = parent->getLogic()->getResponseFor(ownLinkIndex);
    //     // inform links where they have to report approaching vehicles to
    //     //std::cout << " special=" << specialLane->getID() << " incoming=" << toString(myIncomingLanes) << " internal=" << toString(myInternalLanes) << "\n";
    //     for (MSLane* const lane : myInternalLanes) {
    //         for (MSLink* const link : lane->getLinkCont()) {
    //             if (link->getViaLane() != nullptr) {
    //                 const int foeIndex = lane->getIncomingLanes()[0].viaLink->getIndex();
    //                 //std::cout << "       response=" << response << " index=" << ownLinkIndex << " foeIndex=" << foeIndex << " ibct=" << indirectBicycleTurn(specialLane, thisLink, *i, *q) << "\n";
    //                 if (response.test(foeIndex) || indirectBicycleTurn(specialLane, thisLink, lane, link)) {
    //                     // only respect vehicles before internal junctions if they
    //                     // have priority (see the analogous foeLinks.test() when
    //                     // initializing myLinkFoeInternalLanes in MSRightOfWayJunction
    //                     // Indirect left turns for bicycles are a special case
    //                     // because they both intersect on their second part with the first part of the other one
    //                     // and only one of them has priority
    //                     myInternalLaneFoes.push_back(lane);
    //                 }
    //                 if (std::find(myInternalLaneFoes.begin(), myInternalLaneFoes.end(), link->getViaLane()) == myInternalLaneFoes.end()) {
    //                     myInternalLaneFoes.push_back(link->getViaLane());
    //                 }
    //             } else {
    //                 if (std::find(myInternalLaneFoes.begin(), myInternalLaneFoes.end(), lane) == myInternalLaneFoes.end()) {
    //                     myInternalLaneFoes.push_back(lane);
    //                     if (lane->isCrossing()) {
    //                         // also add to myInternalLinkFoes (the origin
    //                         // walkingArea is not part of myIncomingLanes)
    //                         myInternalLinkFoes.push_back(lane->getIncomingLanes()[0].viaLink);
    //                     }
    //                 }
    //             }
    //         }
    //         //std::cout << "  intLane=" << lane->getID() << " foes=" << toString(myInternalLaneFoes) << "\n";
    //
    //     }
    //     for (std::vector<MSLane*>::const_iterator i = myIncomingLanes.begin() + 1; i != myIncomingLanes.end(); ++i) {
    //         for (MSLink* const link : (*i)->getLinkCont()) {
    //             // link indices of internal junctions may not be initialized yet
    //             int linkIndex = link->getCorrespondingEntryLink()->getIndex();
    //             // links that target a shared walkingarea always have index -1
    //             if (linkIndex != -1 && response.test(linkIndex)) {
    //                 myInternalLinkFoes.push_back(link);
    //                 const MSLane* via = link->getViaLane();
    //                 if (via != nullptr && via->getLinkCont().front()->getViaLane() != nullptr) {
    //                     // we added the entry link, also use the internalJunctionLink that follows
    //                     myInternalLinkFoes.push_back(via->getLinkCont().front());
    //                 }
    //             }
    //         }
    //     }
    //     // thisLinks is itself an exitLink of the preceding internal lane
    //     thisLink->setRequestInformation(ownLinkIndex, true, false, myInternalLinkFoes, myInternalLaneFoes, thisLink->getViaLane()->getLogicalPredecessorLane());
    //     assert(thisLink->getViaLane()->getLinkCont().size() == 1);
    //     MSLink* exitLink = thisLink->getViaLane()->getLinkCont()[0];
    //     exitLink->setRequestInformation(ownLinkIndex, false, false, std::vector<MSLink*>(),
    //                                     myInternalLaneFoes, thisLink->getViaLane());
    //     for (const auto& ili : exitLink->getLane()->getIncomingLanes()) {
    //         if (ili.lane->isWalkingArea()) {
    //             exitLink->addWalkingAreaFoeExit(ili.lane);
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn postload_init(&mut self) {
        todo!("port MSInternalJunction::postloadInit")
    }

    /// C++: `const std::vector<MSLink *> & MSInternalJunction::getFoeLinks(const MSLink *const) const` [const, virtual]
    // const std::vector<MSLink*>& getFoeLinks(const MSLink* const srcLink) const {
    //         UNUSED_PARAMETER(srcLink);
    //         return myInternalLinkFoes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_links(&self, srcLink: Ptr<MSLink>) -> &Vec<Ptr<MSLink>> {
        todo!("port MSInternalJunction::getFoeLinks")
    }

    /// C++: `const std::vector<MSLane *> & MSInternalJunction::getFoeInternalLanes(const MSLink *const) const` [const, virtual]
    // const std::vector<MSLane*>& getFoeInternalLanes(const MSLink* const srcLink) const {
    //         UNUSED_PARAMETER(srcLink);
    //         return myInternalLaneFoes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_internal_lanes(&self, srcLink: Ptr<MSLink>) -> &Vec<Ptr<MSLane>> {
        todo!("port MSInternalJunction::getFoeInternalLanes")
    }

    /// C++: `bool MSInternalJunction::indirectBicycleTurn(const MSLane *, const MSLink *, const MSLane *, const MSLink *) const` [const, private]
    // bool
    // MSInternalJunction::indirectBicycleTurn(const MSLane* specialLane, const MSLink* thisLink, const MSLane* foeFirstPart, const MSLink* foeLink) const {
    //     if (specialLane->getPermissions() == SVC_BICYCLE && foeFirstPart->getPermissions() == SVC_BICYCLE
    //             && thisLink->getDirection() == LinkDirection::LEFT && foeLink->getDirection() == LinkDirection::LEFT
    //             && thisLink->getViaLane() != nullptr
    //             && thisLink->getViaLane()->getShape().intersects(foeFirstPart->getShape())) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn indirect_bicycle_turn(&self, specialLane: Ptr<MSLane>, thisLink: Ptr<MSLink>, foeFirstPart: Ptr<MSLane>, foeLink: Ptr<MSLink>) -> bool {
        todo!("port MSInternalJunction::indirectBicycleTurn")
    }

    /// C++: `void MSInternalJunction::MSInternalJunction(const MSInternalJunction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSInternalJunction) -> Self {
        todo!("port MSInternalJunction::MSInternalJunction")
    }

    /// C++: `MSInternalJunction & MSInternalJunction::operator=(const MSInternalJunction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSInternalJunction) -> &mut MSInternalJunction {
        todo!("port MSInternalJunction::operator=")
    }

}

