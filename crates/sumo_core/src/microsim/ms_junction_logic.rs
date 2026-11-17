//! Port of microsim/MSJunctionLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_bit_set_logic::MSBitSetLogic;
use utils::common::std_defs::SUMO_MAX_CONNECTIONS;

/// C++ class `MSJunctionLogic` (microsim/MSJunctionLogic.h)
pub struct MSJunctionLogic {
    /// The logic's number of links.
    pub myNLinks: i32,
}

// C++ static members of MSJunctionLogic (port as module-level statics, see AGENTS.md):
//   static MSLogicJunction::LinkBits MSJunctionLogic::myDummyFoes;
#[allow(non_upper_case_globals)]
static myDummyFoes: MSLogicJunction_LinkBits = MSLogicJunction_LinkBits::new();

impl MSJunctionLogic {
    /// C++: `void MSJunctionLogic::~MSJunctionLogic()` [virtual]
    /// Destructor.
    // virtual ~MSJunctionLogic() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release.
    }

    /// C++: `const MSLogicJunction::LinkBits & MSJunctionLogic::getResponseFor(int) const` [const, virtual]
    /// Returns the response for the given link
    // virtual const MSLogicJunction::LinkBits& getResponseFor(int linkIndex) const {
    //         UNUSED_PARAMETER(linkIndex);
    //         return myDummyFoes;
    //     }
    #[allow(unused_variables)]
    pub fn get_response_for(&self, linkIndex: i32) -> &MSLogicJunction_LinkBits {
        self.bit_set().get_response_for(linkIndex)
    }

    /// C++: `const MSLogicJunction::LinkBits & MSJunctionLogic::getFoesFor(int) const` [const, virtual]
    /// Returns the foes for the given link
    // virtual const MSLogicJunction::LinkBits& getFoesFor(int linkIndex) const {
    //         UNUSED_PARAMETER(linkIndex);
    //         return myDummyFoes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foes_for(&self, linkIndex: i32) -> &MSLogicJunction_LinkBits {
        self.bit_set().get_foes_for(linkIndex)
    }

    /// C++: `bool MSJunctionLogic::getIsCont(int) const` [const, virtual]
    // virtual bool getIsCont(int linkIndex) const {
    //         UNUSED_PARAMETER(linkIndex);
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn get_is_cont(&self, linkIndex: i32) -> bool {
        self.bit_set().get_is_cont(linkIndex)
    }

    /// C++: `int MSJunctionLogic::getLogicSize() const` [const]
    // int getLogicSize() const {
    //         return myNLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_size(&self) -> i32 {
        self.myNLinks
    }

    /// C++: `bool MSJunctionLogic::hasFoes() const` [const, virtual]
    // virtual bool hasFoes() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn has_foes(&self) -> bool {
        self.bit_set().has_foes()
    }

    /// Virtual dispatch: MSBitSetLogic is the only subclass and every logic is built as one
    /// (NLJunctionControlBuilder::closeJunction), so the base bodies above are never reached in C++.
    fn bit_set(&self) -> &MSBitSetLogic<SUMO_MAX_CONNECTIONS> {
        // SAFETY: MSBitSetLogic is #[repr(C)] with `base: MSJunctionLogic` as its first field.
        unsafe { &*(self as *const Self as *const MSBitSetLogic<SUMO_MAX_CONNECTIONS>) }
    }

    /// C++: `void MSJunctionLogic::MSJunctionLogic(int)` [protected]
    /// Constructor.
    // MSJunctionLogic::MSJunctionLogic(int nLinks) :
    //     myNLinks(nLinks) {}
    #[allow(unused_variables)]
    pub fn new(nLinks: i32) -> Self {
        MSJunctionLogic { myNLinks: nLinks }
    }

    // The C++ copy constructor and operator= are `= delete` (private), so no Rust
    // equivalent exists: the type is intentionally non-copyable.
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_virtuals_dispatch_to_the_bitset_logic() {
        let mut foes = MSLogicJunction_LinkBits::new();
        foes.set(2, true);
        let mut resp = MSLogicJunction_LinkBits::new();
        resp.set(1, true);
        let mut conts = MSLogicJunction_LinkBits::new();
        conts.set(0, true);
        let b = MSBitSetLogic::<SUMO_MAX_CONNECTIONS>::new(1, &vec![resp], &vec![foes], conts);
        let base: &MSJunctionLogic = &b.base;
        assert!(base.get_foes_for(0).test(2));
        assert!(base.get_response_for(0).test(1));
        assert!(base.get_is_cont(0));
        assert!(base.has_foes());
    }
}
