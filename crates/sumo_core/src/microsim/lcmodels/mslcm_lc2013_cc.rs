//! Port of microsim/lcmodels/MSLCM_LC2013_CC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A lane change model developed by D. Krajzewicz, J. Erdmann et al. between 2004 and 2013, extended for atomic lane change for platoons
/// C++ class `MSLCM_LC2013_CC` (microsim/lcmodels/MSLCM_LC2013_CC.h)
#[repr(C)]
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
        let mut base = MSLCM_LC2013::new(v);
        base.base.vt = &MSLCM_LC2013_CC_VT;
        MSLCM_LC2013_CC { base }
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
            // determinism: the dynamic_cast is replaced by a model-id check, which identifies MSCFModel_CC exactly
            let cf = veh.get_car_follow_model();
            if cf.get_model_id() == SUMO_TAG_CF_CC as i32 {
                let cfm = crate::microsim::cfmodels::mscf_model::cf_cast::<crate::microsim::cfmodels::mscf_model_cc::MSCFModel_CC>(cf);
                let left = (state & LCA_LEFT) != 0;
                return cfm.commit_to_lane_change(veh, left);
            }
        }
        0
    }

}

