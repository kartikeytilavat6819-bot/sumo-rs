//! Port of microsim/MSInternalJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_bit_set_logic::MSBitSetLogic;
use crate::microsim::ms_right_of_way_junction::MSRightOfWayJunction;
use utils::common::std_defs::SUMO_MAX_CONNECTIONS;

/// A class which realises junctions that do regard any kind of a right-of-way. The rules for the right-of-way themselves are stored within the associated "MSJunctionLogic" - structure.
/// C++ class `MSInternalJunction` (microsim/MSInternalJunction.h)
#[repr(C)]
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
        MSInternalJunction {
            base: MSLogicJunction::new(id, type_, position, shape, "", incoming, internal),
            myInternalLinkFoes: Vec::new(),
            myInternalLaneFoes: Vec::new(),
        }
    }

    /// C++: `void MSInternalJunction::~MSInternalJunction() noexcept` [virtual]
    /// Destructor.
    // MSInternalJunction::~MSInternalJunction() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the members are dropped automatically.
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
        if self.base.myIncomingLanes.is_empty() {
            std::panic::panic_any(ProcessError::new(format!("Internal junction {} has no incoming lanes", self.get_id())));
        }
        // the first lane in the list of incoming lanes is special. It defines the
        // link that needs to do all the checking for this internal junction
        let specialLane = self.base.myIncomingLanes[0];
        assert!(specialLane.get_link_cont().len() == 1);
        let thisLink = specialLane.get_link_cont()[0];
        let parent = specialLane.get_edge().get_to_junction();
        // C++ dynamic_cast<const MSRightOfWayJunction*>(parent), answered by the node type in MSJunction::as_right_of_way
        if parent.is_null() || parent.as_right_of_way().is_none() {
            // parent has no right-of-way logic
            return;
        }
        let ownLinkIndex = specialLane.get_incoming_lanes()[0].viaLink.get_index();
        let logicBits = parent.cast::<MSRightOfWayJunction>().m().get_logic().cast::<MSBitSetLogic<SUMO_MAX_CONNECTIONS>>();
        let response = *logicBits.m().get_response_for(ownLinkIndex);
        // inform links where they have to report approaching vehicles to
        for lane in self.base.myInternalLanes.iter().copied() {
            for link in lane.get_link_cont().iter().copied() {
                if !link.get_via_lane().is_null() {
                    let foeIndex = lane.get_incoming_lanes()[0].viaLink.get_index();
                    // only respect vehicles before internal junctions if they
                    // have priority (see the analogous foeLinks.test() when
                    // initializing myLinkFoeInternalLanes in MSRightOfWayJunction
                    // Indirect left turns for bicycles are a special case
                    // because they both intersect on their second part with the first part of the other one
                    // and only one of them has priority
                    if response_test(&response, foeIndex) || self.indirect_bicycle_turn(specialLane, thisLink, lane, link) {
                        self.myInternalLaneFoes.push(lane);
                    }
                    if !self.myInternalLaneFoes.contains(&link.get_via_lane()) {
                        self.myInternalLaneFoes.push(link.get_via_lane());
                    }
                } else if !self.myInternalLaneFoes.contains(&lane) {
                    self.myInternalLaneFoes.push(lane);
                    if lane.is_crossing() {
                        // also add to myInternalLinkFoes (the origin
                        // walkingArea is not part of myIncomingLanes)
                        self.myInternalLinkFoes.push(lane.get_incoming_lanes()[0].viaLink);
                    }
                }
            }
        }
        for i in self.base.myIncomingLanes.iter().skip(1).copied() {
            for link in i.get_link_cont().iter().copied() {
                // link indices of internal junctions may not be initialized yet
                let linkIndex = link.get_corresponding_entry_link().get_index();
                // links that target a shared walkingarea always have index -1
                if linkIndex != -1 && response_test(&response, linkIndex) {
                    self.myInternalLinkFoes.push(link);
                    let via = link.get_via_lane();
                    if !via.is_null() && !via.get_link_cont()[0].get_via_lane().is_null() {
                        // we added the entry link, also use the internalJunctionLink that follows
                        self.myInternalLinkFoes.push(via.get_link_cont()[0]);
                    }
                }
            }
        }
        // thisLinks is itself an exitLink of the preceding internal lane
        thisLink.m().set_request_information(
            ownLinkIndex,
            true,
            false,
            &self.myInternalLinkFoes,
            &self.myInternalLaneFoes,
            thisLink.get_via_lane().get_logical_predecessor_lane(),
        );
        assert!(thisLink.get_via_lane().get_link_cont().len() == 1);
        let exitLink = thisLink.get_via_lane().get_link_cont()[0];
        exitLink.m().set_request_information(
            ownLinkIndex,
            false,
            false,
            &Vec::new(),
            &self.myInternalLaneFoes,
            thisLink.get_via_lane(),
        );
        for ili in exitLink.get_lane().get_incoming_lanes() {
            if ili.lane.is_walking_area() {
                exitLink.m().add_walking_area_foe_exit(ili.lane);
                break;
            }
        }
    }

    /// C++: `const std::vector<MSLink *> & MSInternalJunction::getFoeLinks(const MSLink *const) const` [const, virtual]
    // const std::vector<MSLink*>& getFoeLinks(const MSLink* const srcLink) const {
    //         UNUSED_PARAMETER(srcLink);
    //         return myInternalLinkFoes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_links(&self, srcLink: Ptr<MSLink>) -> &Vec<Ptr<MSLink>> {
        &self.myInternalLinkFoes
    }

    /// C++: `const std::vector<MSLane *> & MSInternalJunction::getFoeInternalLanes(const MSLink *const) const` [const, virtual]
    // const std::vector<MSLane*>& getFoeInternalLanes(const MSLink* const srcLink) const {
    //         UNUSED_PARAMETER(srcLink);
    //         return myInternalLaneFoes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_internal_lanes(&self, srcLink: Ptr<MSLink>) -> &Vec<Ptr<MSLane>> {
        &self.myInternalLaneFoes
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
        if specialLane.get_permissions() == utils::common::sumo_vehicle_class::SVC_BICYCLE
            && foeFirstPart.get_permissions() == utils::common::sumo_vehicle_class::SVC_BICYCLE
            && thisLink.get_direction() == LinkDirection::LEFT
            && foeLink.get_direction() == LinkDirection::LEFT
            && !thisLink.get_via_lane().is_null()
            && thisLink.get_via_lane().get_shape_2().intersects_pv(foeFirstPart.get_shape_2())
        {
            true
        } else {
            false
        }
    }

    /// C++: `void MSInternalJunction::MSInternalJunction(const MSInternalJunction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSInternalJunction) -> Self {
        panic!("MSInternalJunction copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `MSInternalJunction & MSInternalJunction::operator=(const MSInternalJunction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSInternalJunction) -> &mut MSInternalJunction {
        panic!("MSInternalJunction assignment operator is private and undefined in C++; it cannot be called")
    }

}

/// `std::bitset::test` on the parent's response row (MSLogicJunction::LinkBits)
fn response_test(response: &[bool; SUMO_MAX_CONNECTIONS], index: i32) -> bool {
    response[index as usize]
}

