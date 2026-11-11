//! Port of microsim/output/MSCrossSection.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<MSCrossSection> CrossSectionVector`
pub type CrossSectionVector = Vec<MSCrossSection>;

/// C++ typedef `CrossSectionVector::const_iterator CrossSectionVectorConstIt`
pub type CrossSectionVectorConstIt = Unmapped /* CrossSectionVector::const_iterator */;

/// C++ typedef `CrossSectionVector::iterator CrossSectionVectorIt`
pub type CrossSectionVectorIt = Unmapped /* CrossSectionVector::iterator */;

/// A simple description of a position on a lane (crossing of a lane)
/// C++ class `MSCrossSection` (microsim/output/MSCrossSection.h)
pub struct MSCrossSection {
    /// The lane to cross
    pub myLane: Ptr<MSLane>,
    /// The position at the lane
    pub myPosition: f64,
}

impl MSCrossSection {
    /// C++: `void MSCrossSection::MSCrossSection(MSLane *, double)`
    /// Constructor
    // MSCrossSection(MSLane* lane, double pos) : myLane(lane), myPosition(pos) {}
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, pos: f64) -> Self {
        todo!("port MSCrossSection::MSCrossSection")
    }

}

