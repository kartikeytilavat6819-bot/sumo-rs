//! Port of microsim/MSJunctionControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Container for junctions; performs operations on all stored junctions
/// C++ class `MSJunctionControl` (microsim/MSJunctionControl.h)
pub struct MSJunctionControl {
    /// C++ base class `NamedObjectCont<MSJunction *>` (its std::map<std::string, T> IDMap, ported as an ordered map)
    pub base: BTreeMap<String, Ptr<MSJunction>>,
}

impl std::ops::Deref for MSJunctionControl {
    type Target = BTreeMap<String, Ptr<MSJunction>>;
    fn deref(&self) -> &BTreeMap<String, Ptr<MSJunction>> {
        &self.base
    }
}
impl std::ops::DerefMut for MSJunctionControl {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, Ptr<MSJunction>> {
        &mut self.base
    }
}

impl MSJunctionControl {
    /// C++: `void MSJunctionControl::MSJunctionControl()`
    /// Constructor
    // MSJunctionControl::MSJunctionControl() {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSJunctionControl { base: BTreeMap::new() }
    }

    /// C++: `void MSJunctionControl::~MSJunctionControl() noexcept` [virtual]
    /// Destructor
    // MSJunctionControl::~MSJunctionControl() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release.
    }

    /// C++: `void MSJunctionControl::postloadInitContainer()`
    /// Closes building of junctions
    // void
    // MSJunctionControl::postloadInitContainer() {
    //     // initialize normal junctions before internal junctions
    //     // (to allow calling getIndex() during initialization of internal junction links)
    //     for (const auto& i : *this) {
    //         if (i.second->getType() != SumoXMLNodeType::INTERNAL) {
    //             i.second->postloadInit();
    //         }
    //     }
    //     for (const auto& i : *this) {
    //         if (i.second->getType() == SumoXMLNodeType::INTERNAL) {
    //             i.second->postloadInit();
    //         }
    //     }
    //     MSLink::recheckSetRequestInformation();
    // }
    #[allow(unused_variables)]
    pub fn postload_init_container(&mut self) {
        // initialize normal junctions before internal junctions
        // (to allow calling getIndex() during initialization of internal junction links)
        for i in self.base.values() {
            if i.get_type() != SumoXMLNodeType::INTERNAL {
                i.m().postload_init();
            }
        }
        for i in self.base.values() {
            if i.get_type() == SumoXMLNodeType::INTERNAL {
                i.m().postload_init();
            }
        }
        MSLink::recheck_set_request_information();
    }

    // The C++ copy constructor and operator= are declared private and never defined: the type is non-copyable.
}

