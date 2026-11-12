//! Port of microsim/lcmodels/MSLCM_LC2013_CC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A lane change model developed by D. Krajzewicz, J. Erdmann et al. between 2004 and 2013, extended for atomic lane change for platoons
/// C++ class `MSLCM_LC2013_CC` (microsim/lcmodels/MSLCM_LC2013_CC.h)
pub struct MSLCM_LC2013_CC {
    /// C++ base class `MSLCM_LC2013`
    pub base: MSLCM_LC2013,
}

impl std::ops::Deref for MSLCM_LC2013_CC {
    type Target = MSLCM_LC2013;
    fn deref(&self) -> &MSLCM_LC2013 {
        &self.base
    }
}
impl std::ops::DerefMut for MSLCM_LC2013_CC {
    fn deref_mut(&mut self) -> &mut MSLCM_LC2013 {
        &mut self.base
    }
}

impl MSLCM_LC2013_CC {
    /// C++: `void MSLCM_LC2013_CC::MSLCM_LC2013_CC(MSVehicle &)`
    // MSLCM_LC2013_CC::MSLCM_LC2013_CC(MSVehicle& v) : MSLCM_LC2013(v) {}
    #[allow(unused_variables)]
    pub fn new(v: &mut MSVehicle) -> Self {
        MSLCM_LC2013_CC {
            base: MSLCM_LC2013::new(v),
        }
    }

    /// C++: `void MSLCM_LC2013_CC::~MSLCM_LC2013_CC() noexcept` [virtual]
    // MSLCM_LC2013_CC::~MSLCM_LC2013_CC() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has an empty body
    }

    /// C++: `LaneChangeModel MSLCM_LC2013_CC::getModelID() const` [const, virtual]
    /// Returns the model's id
    // LaneChangeModel getModelID() const override {
    //         return LaneChangeModel::LC2013_CC;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> LaneChangeModel {
        LaneChangeModel::LC2013_CC
    }

    /// C++: `int MSLCM_LC2013_CC::checkChangeBeforeCommitting(const MSVehicle *, int) const` [const, virtual]
    // int MSLCM_LC2013_CC::checkChangeBeforeCommitting(const MSVehicle* veh, int state) const {
    //
    //     if (state & LCA_WANTS_LANECHANGE) {
    //         auto* cfm = dynamic_cast<const MSCFModel_CC*>(&veh->getCarFollowModel());
    //
    //         if (cfm) {
    //             bool left = (state & LCA_LEFT) != 0;
    //             return cfm->commitToLaneChange(veh, left);
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn check_change_before_committing(&self, veh: Ptr<MSVehicle>, state: i32) -> i32 {
        if (state & LCA_WANTS_LANECHANGE) != 0 {
            // C++: dynamic_cast<const MSCFModel_CC*>(&veh->getCarFollowModel()); if it succeeds,
            // return cfm->commitToLaneChange(veh, (state & LCA_LEFT) != 0)
            // shortcut: no checked downcast from MSCFModel to MSCFModel_CC exists yet (MSCFModel::get_model_id is pure virtual here and MSCFModel_CC is not repr(C)), so CC car-following models never commit to a lane change; non-CC models return 0 as in C++. Upgrade when a checked downcast is available.
        }
        0
    }

}

