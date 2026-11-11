//! Port of microsim/MSBitSetLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<std::bitset<N>> MSBitSetLogic::Foes`
pub type MSBitSetLogic_Foes = Unmapped /* std::vector<std::bitset<N>> */;

/// C++ typedef `std::vector<std::bitset<N>> MSBitSetLogic::Logic`
pub type MSBitSetLogic_Logic = Unmapped /* std::vector<std::bitset<N>> */;

/// C++ typedef `MSBitSetLogic<256> MSBitsetLogic`
pub type MSBitsetLogic = Unmapped /* MSBitSetLogic<256> */;

/// N is sum of the number of links of the junction's inLanes.
/// C++ class `MSBitSetLogic` (microsim/MSBitSetLogic.h)
pub struct MSBitSetLogic<const N: usize> {
    /// C++ base class `MSJunctionLogic`
    pub base: MSJunctionLogic,
    /// junctions logic based on std::bitset
    pub myLogic: MSBitSetLogic_Logic,
    /// internal lanes logic
    pub myInternalLinksFoes: MSBitSetLogic_Foes,
    pub myConts: Unmapped /* std::bitset<256> */,
}

impl<const N: usize> std::ops::Deref for MSBitSetLogic<N> {
    type Target = MSJunctionLogic;
    fn deref(&self) -> &MSJunctionLogic {
        &self.base
    }
}
impl<const N: usize> std::ops::DerefMut for MSBitSetLogic<N> {
    fn deref_mut(&mut self) -> &mut MSJunctionLogic {
        &mut self.base
    }
}

impl<const N: usize> MSBitSetLogic<N> {
    /// C++: `void MSBitSetLogic::MSBitSetLogic<N>(int, const Logic &, const Foes &, std::bitset<256>)`
    /// Use this constructor only.
    // MSBitSetLogic(int nLinks, const Logic& logic, const Foes& foes,
    //                   std::bitset<SUMO_MAX_CONNECTIONS> conts)
    //         : MSJunctionLogic(nLinks), myLogic(logic),
    //           myInternalLinksFoes(foes), myConts(conts) {}
    #[allow(unused_variables)]
    pub fn new(nLinks: i32, logic: &MSBitSetLogic_Logic, foes: &MSBitSetLogic_Foes, conts: Unmapped /* std::bitset<256> */) -> Self {
        todo!("port MSBitSetLogic::MSBitSetLogic<N>")
    }

    /// C++: `const MSLogicJunction::LinkBits & MSBitSetLogic::getResponseFor(int) const` [const, virtual]
    /// Returns the response for the given link
    // const MSLogicJunction::LinkBits& getResponseFor(int linkIndex) const {
    //         return myLogic[linkIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_response_for(&self, linkIndex: i32) -> &MSLogicJunction_LinkBits {
        todo!("port MSBitSetLogic::getResponseFor")
    }

    /// C++: `const MSLogicJunction::LinkBits & MSBitSetLogic::getFoesFor(int) const` [const, virtual]
    /// Returns the foes for the given link
    // const MSLogicJunction::LinkBits& getFoesFor(int linkIndex) const {
    //         return myInternalLinksFoes[linkIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_foes_for(&self, linkIndex: i32) -> &MSLogicJunction_LinkBits {
        todo!("port MSBitSetLogic::getFoesFor")
    }

    /// C++: `bool MSBitSetLogic::getIsCont(int) const` [const, virtual]
    // bool getIsCont(int linkIndex) const {
    //         return myConts.test(linkIndex);
    //     }
    #[allow(unused_variables)]
    pub fn get_is_cont(&self, linkIndex: i32) -> bool {
        todo!("port MSBitSetLogic::getIsCont")
    }

    /// C++: `bool MSBitSetLogic::hasFoes() const` [const, virtual]
    // bool hasFoes() const {
    //         for (const auto& i : myLogic) {
    //             if (i.any()) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn has_foes(&self) -> bool {
        todo!("port MSBitSetLogic::hasFoes")
    }

    /// C++: `void MSBitSetLogic::MSBitSetLogic<N>(const MSBitSetLogic<N> &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSBitSetLogic<N>) -> Self {
        todo!("port MSBitSetLogic::MSBitSetLogic<N>")
    }

    /// C++: `MSBitSetLogic<N> & MSBitSetLogic::operator=(const MSBitSetLogic<N> &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSBitSetLogic<N>) -> &mut MSBitSetLogic<N> {
        todo!("port MSBitSetLogic::operator=")
    }

}

