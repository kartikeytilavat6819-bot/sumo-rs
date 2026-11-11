//! Port of microsim/MSJunctionControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Container for junctions; performs operations on all stored junctions
/// C++ class `MSJunctionControl` (microsim/MSJunctionControl.h)
pub struct MSJunctionControl {
    /// C++ base class `NamedObjectCont<MSJunction *>`
    pub base: Unmapped /* NamedObjectCont<MSJunction *> */,
}

impl std::ops::Deref for MSJunctionControl {
    type Target = Unmapped /* NamedObjectCont<MSJunction *> */;
    fn deref(&self) -> &Unmapped /* NamedObjectCont<MSJunction *> */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSJunctionControl {
    fn deref_mut(&mut self) -> &mut Unmapped /* NamedObjectCont<MSJunction *> */ {
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
        todo!("port MSJunctionControl::MSJunctionControl")
    }

    /// C++: `void MSJunctionControl::~MSJunctionControl() noexcept` [virtual]
    /// Destructor
    // MSJunctionControl::~MSJunctionControl() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSJunctionControl::~MSJunctionControl")
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
        todo!("port MSJunctionControl::postloadInitContainer")
    }

    /// C++: `void MSJunctionControl::MSJunctionControl(const MSJunctionControl &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSJunctionControl) -> Self {
        todo!("port MSJunctionControl::MSJunctionControl")
    }

    /// C++: `MSJunctionControl & MSJunctionControl::operator=(const MSJunctionControl &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSJunctionControl) -> &mut MSJunctionControl {
        todo!("port MSJunctionControl::operator=")
    }

}

