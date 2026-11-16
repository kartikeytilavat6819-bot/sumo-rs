//! Port of microsim/MSRightOfWayJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_bit_set_logic::MSBitSetLogic;
use utils::common::std_defs::SUMO_MAX_CONNECTIONS;

/// A junction with right-of-way - rules
/// C++ class `MSRightOfWayJunction` (microsim/MSRightOfWayJunction.h)
#[repr(C)]
pub struct MSRightOfWayJunction {
    /// C++ base class `MSLogicJunction`
    pub base: MSLogicJunction,
    /// the type of the junction (its logic)
    pub myLogic: Ptr<MSJunctionLogic>,
    pub myLinkFoeLinks: BTreeMap<Ptr<MSLink>, Vec<Ptr<MSLink>>>,
    pub myLinkFoeInternalLanes: BTreeMap<Ptr<MSLink>, Vec<Ptr<MSLane>>>,
}

impl std::ops::Deref for MSRightOfWayJunction {
    type Target = MSLogicJunction;
    fn deref(&self) -> &MSLogicJunction {
        &self.base
    }
}
impl std::ops::DerefMut for MSRightOfWayJunction {
    fn deref_mut(&mut self) -> &mut MSLogicJunction {
        &mut self.base
    }
}

impl MSRightOfWayJunction {
    /// C++: `void MSRightOfWayJunction::MSRightOfWayJunction(const std::string &, SumoXMLNodeType, const Position &, const PositionVector &, const std::string &, std::vector<MSLane *>, std::vector<MSLane *>, MSJunctionLogic *)`
    /// Constructor
    // MSRightOfWayJunction::MSRightOfWayJunction(const std::string& id,
    //         SumoXMLNodeType type,
    //         const Position& position,
    //         const PositionVector& shape,
    //         const std::string& name,
    //         std::vector<MSLane*> incoming,
    //         std::vector<MSLane*> internal,
    //         MSJunctionLogic* logic) : MSLogicJunction(id, type, position, shape, name, incoming, internal),
    //     myLogic(logic) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, type_: SumoXMLNodeType, position: &Position, shape: &PositionVector, name: &str, incoming: Vec<Ptr<MSLane>>, internal: Vec<Ptr<MSLane>>, logic: Ptr<MSJunctionLogic>) -> Self {
        Self {
            base: MSLogicJunction::new(id, type_, position, shape, name, incoming, internal),
            myLogic: logic,
            myLinkFoeLinks: BTreeMap::new(),
            myLinkFoeInternalLanes: BTreeMap::new(),
        }
    }

    /// C++: `void MSRightOfWayJunction::~MSRightOfWayJunction() noexcept` [virtual]
    /// Destructor.
    // MSRightOfWayJunction::~MSRightOfWayJunction() {
    //     delete myLogic;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // delete myLogic (null-safe, like delete nullptr); then the base destructor runs
        if !self.myLogic.is_null() {
            drop(unsafe { Box::from_raw(self.myLogic.as_raw()) });
        }
        self.base.drop_();
    }

    /// C++: `void MSRightOfWayJunction::postloadInit()` [virtual]
    // void
    // MSRightOfWayJunction::postloadInit() {
    //     // inform links where they have to report approaching vehicles to
    //     int requestPos = 0;
    //     // going through the incoming lanes...
    //     int maxNo = 0;
    //     std::vector<std::pair<const MSLane*, MSLink*> > sortedLinks;
    //     for (MSLane* const lane : myIncomingLanes) {
    //         // ... set information for every link
    //         for (MSLink* const link : lane->getLinkCont()) {
    //             if (link->getLane()->isWalkingArea() ||
    //                     (lane->isWalkingArea() && !link->getLane()->isCrossing())) {
    //                 continue;
    //             }
    //             sortedLinks.emplace_back(lane, link);
    //             ++maxNo;
    //         }
    //     }
    //
    //     const bool hasFoes = myLogic->hasFoes();
    //     for (const MSLane* const lane : myIncomingLanes) {
    //         // ... set information for every link
    //         const MSLane* walkingAreaFoe = nullptr;
    //         for (MSLink* const link : lane->getLinkCont()) {
    //             if (link->getLane()->isWalkingArea()) {
    //                 if (lane->getPermissions() != SVC_PEDESTRIAN) {
    //                     // vehicular lane connects to a walkingarea
    //                     walkingAreaFoe = link->getLane();
    //                 }
    //                 continue;
    //             } else if ((lane->isWalkingArea() && !link->getLane()->isCrossing())) {
    //                 continue;
    //             }
    //             if (myLogic->getLogicSize() <= requestPos) {
    //                 throw ProcessError(TLF("Found invalid logic position of a link for junction '%' (%, max %) -> (network error)", getID(), toString(requestPos), toString(myLogic->getLogicSize())));
    //             }
    //             const MSLogicJunction::LinkBits& linkResponse = myLogic->getResponseFor(requestPos); // SUMO_ATTR_RESPONSE
    //             const MSLogicJunction::LinkBits& linkFoes = myLogic->getFoesFor(requestPos); // SUMO_ATTR_FOES
    //             bool cont = myLogic->getIsCont(requestPos);
    //             for (int c = 0; c < maxNo; ++c) {
    //                 if (linkResponse.test(c)) {
    //                     MSLink* foe = sortedLinks[c].second;
    //                     myLinkFoeLinks[link].push_back(foe);
    //                     if (MSGlobals::gUsingInternalLanes && foe->getViaLane() != nullptr) {
    //                         assert(foe->getViaLane()->getLinkCont().size() == 1);
    //                         MSLink* foeExitLink = foe->getViaLane()->getLinkCont()[0];
    //                         // add foe links after an internal junction
    //                         if (foeExitLink->getViaLane() != nullptr) {
    //                             myLinkFoeLinks[link].push_back(foeExitLink);
    //                         }
    //                     }
    //                 }
    //             }
    //             std::vector<MSLink*> foes;
    //             for (int c = 0; c < maxNo; ++c) {
    //                 if (linkFoes.test(c)) {
    //                     MSLink* foe = sortedLinks[c].second;
    //                     foes.push_back(foe);
    //                     MSLane* l = foe->getViaLane();
    //                     if (l == nullptr) {
    //                         continue;
    //                     }
    //                     // add foe links after an internal junction
    //                     for (MSLink* const vLink : l->getLinkCont()) {
    //                         if (vLink->getViaLane() != nullptr) {
    //                             foes.push_back(vLink);
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if (MSGlobals::gUsingInternalLanes && myInternalLanes.size() > 0) {
    //                 int li = 0;
    //                 for (int c = 0; c < (int)sortedLinks.size(); ++c) {
    //                     if (sortedLinks[c].second->getLane() == nullptr) { // dead end
    //                         continue;
    //                     }
    //                     if (linkFoes.test(c)) {
    //                         // vehicles waiting at an internal junction can mostly be ignored. The main exceptions are:
    //                         // - they are on the main road and the current link is from a side road
    //                         // - its a tls and the current link is on a side road relative to the internal junction
    //                         // both cases are encoded in a positive linkResponse
    //                         // (case 2 only if netconvert option --tls.ignore-internal-junction-jam was not set)
    //                         myLinkFoeInternalLanes[link].push_back(myInternalLanes[li]);
    //                         if (link->getLane()->isCrossing() || linkResponse.test(c) || sortedLinks[c].second->isIndirect() ||
    //                                 link->getLane()->getBidiLane() == sortedLinks[c].second->getLaneBefore()) {
    //                             const std::vector<MSLane::IncomingLaneInfo>& l = myInternalLanes[li]->getIncomingLanes();
    //                             if (l.size() == 1 && l[0].lane->getEdge().isInternal()) {
    //                                 myLinkFoeInternalLanes[link].push_back(l[0].lane);
    //                             }
    //                         }
    //                     }
    //                     ++li;
    //                 }
    //             }
    //             link->setRequestInformation((int)requestPos, hasFoes, cont, myLinkFoeLinks[link], myLinkFoeInternalLanes[link]);
    //             // the exit link for a link before an internal junction is handled in MSInternalJunction
    //             // so we need to skip if cont=true
    //             if (MSGlobals::gUsingInternalLanes && link->getViaLane() != nullptr && !cont) {
    //                 assert(link->getViaLane()->getLinkCont().size() == 1);
    //                 MSLink* exitLink = link->getViaLane()->getLinkCont()[0];
    //                 exitLink->setRequestInformation((int)requestPos, false, false, std::vector<MSLink*>(),
    //                                                 myLinkFoeInternalLanes[link], link->getViaLane());
    //                 for (const auto& ili : exitLink->getLane()->getIncomingLanes()) {
    //                     if (ili.lane->isWalkingArea()) {
    //                         exitLink->addWalkingAreaFoeExit(ili.lane);
    //                         break;
    //                     }
    //                 }
    //             }
    //             // the exit link for a crossing is needed for the pedestrian model
    //             if (MSGlobals::gUsingInternalLanes && link->getLane()->isCrossing()) {
    //                 MSLink* exitLink = link->getLane()->getLinkCont()[0];
    //                 exitLink->setRequestInformation((int)requestPos, false, false, std::vector<MSLink*>(),
    //                                                 myLinkFoeInternalLanes[link], link->getLane());
    //             }
    //             requestPos++;
    //         }
    //         if (walkingAreaFoe != nullptr && lane->getLinkCont().size() > 1) {
    //             for (const MSLink* const link : lane->getLinkCont()) {
    //                 if (!link->getLane()->isWalkingArea()) {
    //                     MSLink* exitLink = link->getViaLane()->getLinkCont()[0];
    //                     exitLink->addWalkingAreaFoe(walkingAreaFoe);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn postload_init(&mut self) {
        // myLogic is always the MSBitSetLogic built by NLJunctionControlBuilder::build_logic_junction
        let logicBits = self.myLogic.cast::<MSBitSetLogic<SUMO_MAX_CONNECTIONS>>();
        // inform links where they have to report approaching vehicles to
        let mut requestPos: i32 = 0;
        // going through the incoming lanes...
        let mut maxNo: i32 = 0;
        let mut sortedLinks: Vec<(Ptr<MSLane>, Ptr<MSLink>)> = Vec::new();
        let incomingLanes: Vec<Ptr<MSLane>> = self.myIncomingLanes.clone();
        for lane in incomingLanes.iter().copied() {
            // ... set information for every link
            for link in lane.get_link_cont().iter().copied() {
                if link.get_lane().is_walking_area() || (lane.is_walking_area() && !link.get_lane().is_crossing()) {
                    continue;
                }
                sortedLinks.push((lane, link));
                maxNo += 1;
            }
        }

        let hasFoes = logicBits.m().has_foes();
        for lane in incomingLanes.iter().copied() {
            // ... set information for every link
            let mut walkingAreaFoe: Ptr<MSLane> = Ptr::null();
            for link in lane.get_link_cont().iter().copied() {
                if link.get_lane().is_walking_area() {
                    if lane.get_permissions() != SVC_PEDESTRIAN {
                        // vehicular lane connects to a walkingarea
                        walkingAreaFoe = link.get_lane();
                    }
                    continue;
                } else if lane.is_walking_area() && !link.get_lane().is_crossing() {
                    continue;
                }
                if self.myLogic.get_logic_size() <= requestPos {
                    std::panic::panic_any(ProcessError::new(utils::tlf!(
                        "Found invalid logic position of a link for junction '%' (%, max %) -> (network error)",
                        self.get_id(),
                        requestPos,
                        self.myLogic.get_logic_size()
                    )));
                }
                let linkResponse = logicBits.m().get_response_for(requestPos);
                let linkFoes = logicBits.m().get_foes_for(requestPos);
                let cont = logicBits.m().get_is_cont(requestPos);
                for c in 0..maxNo {
                    if link_bits_test(linkResponse, c) {
                        let foe = sortedLinks[c as usize].1;
                        self.myLinkFoeLinks.entry(link).or_default().push(foe);
                        if MSGlobals::using_internal_lanes() && !foe.get_via_lane().is_null() {
                            let foeExitLink = foe.get_via_lane().get_link_cont()[0];
                            // add foe links after an internal junction
                            if !foeExitLink.get_via_lane().is_null() {
                                self.myLinkFoeLinks.entry(link).or_default().push(foeExitLink);
                            }
                        }
                    }
                }
                let mut foes: Vec<Ptr<MSLink>> = Vec::new();
                for c in 0..maxNo {
                    if link_bits_test(linkFoes, c) {
                        let foe = sortedLinks[c as usize].1;
                        foes.push(foe);
                        let l = foe.get_via_lane();
                        if l.is_null() {
                            continue;
                        }
                        // add foe links after an internal junction
                        for vLink in l.get_link_cont().iter().copied() {
                            if !vLink.get_via_lane().is_null() {
                                foes.push(vLink);
                            }
                        }
                    }
                }

                if MSGlobals::using_internal_lanes() && !self.myInternalLanes.is_empty() {
                    let mut li: usize = 0;
                    for c in 0..sortedLinks.len() {
                        if sortedLinks[c].1.get_lane().is_null() {
                            // dead end
                            continue;
                        }
                        if link_bits_test(linkFoes, c as i32) {
                            // vehicles waiting at an internal junction can mostly be ignored. The main exceptions are:
                            // - they are on the main road and the current link is from a side road
                            // - its a tls and the current link is on a side road relative to the internal junction
                            // both cases are encoded in a positive linkResponse
                            // (case 2 only if netconvert option --tls.ignore-internal-junction-jam was not set)
                            let inLane = self.myInternalLanes[li];
                            self.myLinkFoeInternalLanes.entry(link).or_default().push(inLane);
                            if link.get_lane().is_crossing()
                                || link_bits_test(linkResponse, c as i32)
                                || sortedLinks[c].1.is_indirect()
                                || link.get_lane().get_bidi_lane() == sortedLinks[c].1.get_lane_before()
                            {
                                let l = inLane.get_incoming_lanes();
                                if l.len() == 1 && l[0].lane.get_edge().is_internal() {
                                    let laneBefore = l[0].lane;
                                    self.myLinkFoeInternalLanes.entry(link).or_default().push(laneBefore);
                                }
                            }
                        }
                        li += 1;
                    }
                }
                let foeLinks: Vec<Ptr<MSLink>> = self.myLinkFoeLinks.entry(link).or_default().clone();
                let foeInternalLanes: Vec<Ptr<MSLane>> = self.myLinkFoeInternalLanes.entry(link).or_default().clone();
                link.m().set_request_information(requestPos, hasFoes, cont, &foeLinks, &foeInternalLanes, Ptr::null());
                // the exit link for a link before an internal junction is handled in MSInternalJunction
                // so we need to skip if cont=true
                if MSGlobals::using_internal_lanes() && !link.get_via_lane().is_null() && !cont {
                    let viaLane = link.get_via_lane();
                    let exitLink = viaLane.get_link_cont()[0];
                    let noLinks: Vec<Ptr<MSLink>> = Vec::new();
                    exitLink.m().set_request_information(requestPos, false, false, &noLinks, &foeInternalLanes, viaLane);
                    let exitLane = exitLink.get_lane();
                    for ili in exitLane.get_incoming_lanes().iter() {
                        if ili.lane.is_walking_area() {
                            exitLink.m().add_walking_area_foe_exit(ili.lane);
                            break;
                        }
                    }
                }
                // the exit link for a crossing is needed for the pedestrian model
                if MSGlobals::using_internal_lanes() && link.get_lane().is_crossing() {
                    let crossingLane = link.get_lane();
                    let exitLink = crossingLane.get_link_cont()[0];
                    let noLinks: Vec<Ptr<MSLink>> = Vec::new();
                    exitLink.m().set_request_information(requestPos, false, false, &noLinks, &foeInternalLanes, crossingLane);
                }
                requestPos += 1;
            }
            if !walkingAreaFoe.is_null() && lane.get_link_cont().len() > 1 {
                for link in lane.get_link_cont().iter().copied() {
                    if !link.get_lane().is_walking_area() {
                        let exitLink = link.get_via_lane().get_link_cont()[0];
                        exitLink.m().add_walking_area_foe(walkingAreaFoe);
                    }
                }
            }
        }
    }

    /// C++: `const std::vector<MSLink *> & MSRightOfWayJunction::getFoeLinks(const MSLink *const) const` [const, virtual]
    // const std::vector<MSLink*>& getFoeLinks(const MSLink* const srcLink) const {
    //         return myLinkFoeLinks.find(srcLink)->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_links(&self, srcLink: Ptr<MSLink>) -> &Vec<Ptr<MSLink>> {
        &self.myLinkFoeLinks[&srcLink]
    }

    /// C++: `const std::vector<MSLane *> & MSRightOfWayJunction::getFoeInternalLanes(const MSLink *const) const` [const, virtual]
    // const std::vector<MSLane*>& getFoeInternalLanes(const MSLink* const srcLink) const {
    //         return myLinkFoeInternalLanes.find(srcLink)->second;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_internal_lanes(&self, srcLink: Ptr<MSLink>) -> &Vec<Ptr<MSLane>> {
        &self.myLinkFoeInternalLanes[&srcLink]
    }

    /// C++: `const MSJunctionLogic * MSRightOfWayJunction::getLogic() const` [const, virtual]
    // const MSJunctionLogic* getLogic() const {
    //         return myLogic;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic(&self) -> Ptr<MSJunctionLogic> {
        self.myLogic
    }

    /// C++: `void MSRightOfWayJunction::MSRightOfWayJunction(const MSRightOfWayJunction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSRightOfWayJunction) -> Self {
        // C++ declares this copy constructor private and never defines it: unreachable in C++
        panic!("MSRightOfWayJunction copy constructor is invalidated")
    }

    /// C++: `MSRightOfWayJunction &MSRightOfWayJunction::operator=(const MSRightOfWayJunction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSRightOfWayJunction) -> &mut MSRightOfWayJunction {
        // C++ declares this assignment operator private and never defines it: unreachable in C++
        panic!("MSRightOfWayJunction assignment operator is invalidated")
    }

}

/// `std::bitset<SUMO_MAX_CONNECTIONS>::test` on a request row of `MSBitSetLogic`
fn link_bits_test(bits: &[bool; SUMO_MAX_CONNECTIONS], c: i32) -> bool {
    bits[c as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foe_bits_are_read_through_the_bitset_logic_not_the_dummy_base() {
        // a foes row with bits 0 and 3 set, read through the MSJunctionLogic cast as postload_init does
        let mut foes = [false; SUMO_MAX_CONNECTIONS];
        foes[0] = true;
        foes[3] = true;
        let logic = Box::new(MSBitSetLogic::<SUMO_MAX_CONNECTIONS>::new(
            1,
            &vec![[false; SUMO_MAX_CONNECTIONS]],
            &vec![foes],
            [false; SUMO_MAX_CONNECTIONS],
        ));
        let base = Ptr::from_box(&logic).cast::<MSJunctionLogic>();
        let back = base.cast::<MSBitSetLogic<SUMO_MAX_CONNECTIONS>>();
        let row = back.m().get_foes_for(0);
        assert!(link_bits_test(row, 0));
        assert!(!link_bits_test(row, 1));
        assert!(link_bits_test(row, 3));
        assert!(!back.m().has_foes());
    }
}

