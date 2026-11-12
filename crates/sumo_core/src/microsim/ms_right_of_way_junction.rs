//! Port of microsim/MSRightOfWayJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSRightOfWayJunction::postloadInit")
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

