//! Port of microsim/MSBitSetLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::SUMO_MAX_CONNECTIONS;

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
    pub myLogic: Vec<[bool; N]>,
    /// internal lanes logic
    pub myInternalLinksFoes: Vec<[bool; N]>,
    pub myConts: [bool; SUMO_MAX_CONNECTIONS],
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
    pub fn new(nLinks: i32, logic: &Vec<[bool; N]>, foes: &Vec<[bool; N]>, conts: [bool; SUMO_MAX_CONNECTIONS]) -> Self {
        // shortcut: std::bitset<N> is modelled as [bool; N] (no bit packing), upgrade if memory use matters
        MSBitSetLogic {
            base: MSJunctionLogic::new(nLinks),
            myLogic: logic.clone(),
            myInternalLinksFoes: foes.clone(),
            myConts: conts,
        }
    }

    /// C++: `const MSLogicJunction::LinkBits & MSBitSetLogic::getResponseFor(int) const` [const, virtual]
    /// Returns the response for the given link
    // const MSLogicJunction::LinkBits& getResponseFor(int linkIndex) const {
    //         return myLogic[linkIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_response_for(&self, linkIndex: i32) -> &[bool; N] {
        &self.myLogic[linkIndex as usize]
    }

    /// C++: `const MSLogicJunction::LinkBits & MSBitSetLogic::getFoesFor(int) const` [const, virtual]
    /// Returns the foes for the given link
    // const MSLogicJunction::LinkBits& getFoesFor(int linkIndex) const {
    //         return myInternalLinksFoes[linkIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_foes_for(&self, linkIndex: i32) -> &[bool; N] {
        &self.myInternalLinksFoes[linkIndex as usize]
    }

    /// C++: `bool MSBitSetLogic::getIsCont(int) const` [const, virtual]
    // bool getIsCont(int linkIndex) const {
    //         return myConts.test(linkIndex);
    //     }
    #[allow(unused_variables)]
    pub fn get_is_cont(&self, linkIndex: i32) -> bool {
        self.myConts[linkIndex as usize]
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
        for i in &self.myLogic {
            if i.iter().any(|b| *b) {
                return true;
            }
        }
        false
    }

    /// C++: `void MSBitSetLogic::MSBitSetLogic<N>(const MSBitSetLogic<N> &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSBitSetLogic<N>) -> Self {
        panic!("MSBitSetLogic copy constructor is invalidated")
    }

    /// C++: `MSBitSetLogic<N> & MSBitSetLogic::operator=(const MSBitSetLogic<N> &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSBitSetLogic<N>) -> &mut MSBitSetLogic<N> {
        panic!("MSBitSetLogic assignment operator is invalidated")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_foes_and_is_cont() {
        // values follow the C++ bitset semantics: any() over each row, test() on conts
        let mut conts = [false; SUMO_MAX_CONNECTIONS];
        conts[2] = true;
        let mut logic = [false; 4];
        let mut b = MSBitSetLogic::<4> {
            base: MSJunctionLogic { myNLinks: 2 },
            myLogic: vec![[false; 4], [false; 4]],
            myInternalLinksFoes: vec![[false; 4], [false; 4]],
            myConts: conts,
        };
        assert!(!b.has_foes());
        assert!(b.get_is_cont(2));
        assert!(!b.get_is_cont(1));
        logic[3] = true;
        b.myLogic[1] = logic;
        assert!(b.has_foes());
        assert!(b.get_response_for(1)[3]);
    }
}
