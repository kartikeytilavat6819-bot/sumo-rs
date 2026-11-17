//! Port of microsim/MSLogicJunction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::SUMO_MAX_CONNECTIONS;

/// C++ `std::bitset<SUMO_MAX_CONNECTIONS>`: bit `i` lives in word `i / 64`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LinkBits([u64; SUMO_MAX_CONNECTIONS / 64]);

const _: () = assert!(SUMO_MAX_CONNECTIONS % 64 == 0);

impl LinkBits {
    /// C++ `std::bitset<N>()`: all bits clear.
    pub const fn new() -> Self {
        LinkBits([0; SUMO_MAX_CONNECTIONS / 64])
    }

    /// C++ `bitset::set(pos, value)`.
    pub fn set(&mut self, pos: i32, value: bool) {
        let word = pos as usize / 64;
        let mask = 1u64 << (pos as usize % 64);
        if value {
            self.0[word] |= mask;
        } else {
            self.0[word] &= !mask;
        }
    }

    /// C++ `bitset::test(pos)`; a position past the end panics (C++ throws `std::out_of_range`).
    pub fn test(&self, pos: i32) -> bool {
        (self.0[pos as usize / 64] >> (pos as usize % 64)) & 1 == 1
    }

    /// C++ `bitset::none()`.
    pub fn none(&self) -> bool {
        self.0.iter().all(|w| *w == 0)
    }

    /// C++ `bitset::any()`.
    pub fn any(&self) -> bool {
        !self.none()
    }
}

/// C++ typedef `std::bitset<256> MSLogicJunction::LinkBits`
pub type MSLogicJunction_LinkBits = LinkBits;

/// A junction which may not let all vehicles through, but must perform any kind of an operation to determine which cars are allowed to drive in this step.
/// C++ class `MSLogicJunction` (microsim/MSLogicJunction.h)
#[repr(C)]
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
        // C++ destructor body is empty; members are dropped with the struct.
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
        // C++ body is entirely commented out in SUMO; nothing to do.
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
        // Besides the lanes im myInternal lanes, which are only the last parts of the connections,
        // this collects all lanes on the junction
        let mut allInternalLanes: Vec<Ptr<MSLane>> = Vec::new();
        for &i in self.myInternalLanes.iter() {
            let mut l = i;
            while !l.is_null() {
                allInternalLanes.push(l);
                if l.get_incoming_lanes().is_empty() {
                    break;
                }
                assert!(l.get_incoming_lanes().len() == 1);
                l = l.get_incoming_lanes()[0].lane;
                if !l.is_internal() {
                    break;
                }
            }
        }
        allInternalLanes
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
        Self {
            base: MSJunction::new(id, type_, position, shape, name),
            myIncomingLanes: incoming,
            myInternalLanes: internal,
        }
    }

    /// C++: `void MSLogicJunction::MSLogicJunction(const MSLogicJunction &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSLogicJunction) -> Self {
        // C++ copy constructor is private and never defined; it must not be called.
        panic!("MSLogicJunction copy constructor is private and undefined in C++")
    }

    /// C++: `MSLogicJunction & MSLogicJunction::operator=(const MSLogicJunction &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSLogicJunction) -> &mut MSLogicJunction {
        // C++ assignment operator is private and never defined; it must not be called.
        panic!("MSLogicJunction assignment operator is private and undefined in C++")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_bits_set_test_none_across_word_boundaries() {
        let mut b = LinkBits::new();
        assert!(b.none());
        assert!(!b.any());
        for pos in [0, 63, 64, 255] {
            assert!(!b.test(pos));
        }
        b.set(63, true);
        assert!(b.test(63) && !b.test(62) && !b.test(64));
        assert!(!b.none() && b.any());
        b.set(0, true);
        b.set(64, true);
        b.set(255, true);
        for pos in [0, 63, 64, 255] {
            assert!(b.test(pos));
        }
        assert!(!b.test(1) && !b.test(128) && !b.test(254));
        b.set(63, false);
        assert!(!b.test(63) && b.test(64));
        b.set(0, false);
        b.set(64, false);
        b.set(255, false);
        assert!(b.none());
    }

    #[test]
    fn link_bits_is_copy_and_new_is_clear() {
        let mut b = LinkBits::new();
        b.set(200, true);
        let c = b;
        assert!(c.test(200));
        assert!(LinkBits::new().none());
    }
}

