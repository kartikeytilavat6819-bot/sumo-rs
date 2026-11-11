//! Port of microsim/cfmodels/MSCFModel_KraussPS.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;
use utils::common::sumo_vehicle_class::SVC_BICYCLE;

/// Krauss car-following model, changing accel and speed by slope
/// C++ class `MSCFModel_KraussPS` (microsim/cfmodels/MSCFModel_KraussPS.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_KraussPS {
    /// C++ base class `MSCFModel_Krauss`
    pub base: MSCFModel_Krauss,
}

impl std::ops::Deref for MSCFModel_KraussPS {
    type Target = MSCFModel_Krauss;
    fn deref(&self) -> &MSCFModel_Krauss {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_KraussPS {
    fn deref_mut(&mut self) -> &mut MSCFModel_Krauss {
        &mut self.base
    }
}

impl MSCFModel_KraussPS {
    /// C++: `void MSCFModel_KraussPS::MSCFModel_KraussPS(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_KraussPS::MSCFModel_KraussPS(const MSVehicleType* vtype) :
    //     MSCFModel_Krauss(vtype) {
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        MSCFModel_KraussPS { base: MSCFModel_Krauss::new(vtype) }
    }

    /// C++: `void MSCFModel_KraussPS::~MSCFModel_KraussPS() noexcept` [virtual]
    /// Destructor
    // MSCFModel_KraussPS::~MSCFModel_KraussPS() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_KraussPS::maxNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    /// Returns the maximum speed given the current speed
    // double
    // MSCFModel_KraussPS::maxNextSpeed(double speed, const MSVehicle* const veh) const {
    //     const double aBound = getCurrentAccel(speed);
    //     const double aMax = MAX2(0., aBound - GRAVITY * sin(DEG2RAD(veh->getSlope())));
    //     // special case for bicycles where getMaxSpeed() is not a a technical but a rather an individual power limit
    //     const double typeMax = myType->getMaxSpeed() * (veh->getVClass() == SVC_BICYCLE ? veh->getChosenSpeedFactor() : 1);
    //     // assuming drag force is proportional to the square of speed
    //     const double vMax = MAX2(
    //                             sqrt(aMax / getMaxAccel()) * typeMax,
    //                             // prevent emergency braking when inclination changes suddenly (momentum)
    //                             speed - ACCEL2SPEED(getMaxDecel()));
    //     return MAX2(
    //                // prevent stalling at low speed
    //                aBound / 2,
    //                MIN2(speed + ACCEL2SPEED(aMax), vMax));
    // }
    #[allow(unused_variables)]
    pub fn max_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        let aBound = self.get_current_accel(speed);
        let aMax = max2(0., aBound - utils::geom::geom_helper::GRAVITY * (veh.get_slope() * std::f64::consts::PI / 180.).sin());
        // special case for bicycles where getMaxSpeed() is not a a technical but a rather an individual power limit
        let typeMax = self.myType.get_max_speed() * (if veh.get_v_class() == SVC_BICYCLE { veh.get_chosen_speed_factor() } else { 1. });
        // assuming drag force is proportional to the square of speed
        let vMax = max2(
            (aMax / self.get_max_accel()).sqrt() * typeMax,
            // prevent emergency braking when inclination changes suddenly (momentum)
            speed - accel2speed(self.get_max_decel()),
        );
        max2(
            // prevent stalling at low speed
            aBound / 2.,
            min2(speed + accel2speed(aMax), vMax),
        )
    }

    /// C++: `int MSCFModel_KraussPS::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_KRAUSS_PLUS_SLOPE;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_KRAUSS_PLUS_SLOPE as i32
    }

    /// C++: `MSCFModel * MSCFModel_KraussPS::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_KraussPS::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_KraussPS(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_KraussPS::new(vtype))) as *mut MSCFModel)
    }

}

