//! Port of microsim/cfmodels/MSCFModel_KraussX.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use super::mscf_model_krauss::KRAUSS_VT;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;
use utils::common::sumo_vehicle_class::SVC_BICYCLE;

/// Krauss car-following model, changing accel and speed by slope
/// C++ class `MSCFModel_KraussX` (microsim/cfmodels/MSCFModel_KraussX.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_KraussX {
    /// C++ base class `MSCFModel_Krauss`
    pub base: MSCFModel_Krauss,
    /// extension parameter nr1
    pub myTmp1: f64,
    pub myTmp2: f64,
}

impl std::ops::Deref for MSCFModel_KraussX {
    type Target = MSCFModel_Krauss;
    fn deref(&self) -> &MSCFModel_Krauss {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_KraussX {
    fn deref_mut(&mut self) -> &mut MSCFModel_Krauss {
        &mut self.base
    }
}

fn krauss_x_patch_speed_before_lc(m: &MSCFModel, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
    cf_cast::<MSCFModel_KraussX>(m).patch_speed_before_lc(veh, vMin, vMax)
}
fn krauss_x_get_model_id(m: &MSCFModel) -> i32 {
    cf_cast::<MSCFModel_KraussX>(m).get_model_id()
}
fn krauss_x_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
    cf_cast::<MSCFModel_KraussX>(m).duplicate(vtype)
}

/// Virtual table of MSCFModel_KraussX: its overrides on top of the MSCFModel_Krauss table
pub const KRAUSS_X_VT: MSCFModelVt = MSCFModelVt {
    patch_speed_before_lc: krauss_x_patch_speed_before_lc,
    get_model_id: krauss_x_get_model_id,
    duplicate: krauss_x_duplicate,
    ..KRAUSS_VT
};

impl MSCFModel_KraussX {
    /// C++: `void MSCFModel_KraussX::MSCFModel_KraussX(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_KraussX::MSCFModel_KraussX(const MSVehicleType* vtype):
    //     MSCFModel_Krauss(vtype),
    //     myTmp1(vtype->getParameter().getCFParam(SUMO_ATTR_TMP1, 0.0)),
    //     myTmp2(vtype->getParameter().getCFParam(SUMO_ATTR_TMP2, 0.0)) {
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let myTmp1 = vtype.get_parameter().get_cf_param(SUMO_ATTR_TMP1, 0.0);
        let myTmp2 = vtype.get_parameter().get_cf_param(SUMO_ATTR_TMP2, 0.0);
        let mut this = MSCFModel_KraussX { base: MSCFModel_Krauss::new(vtype), myTmp1, myTmp2 };
        this.base.base.base.vt = &KRAUSS_X_VT;
        this
    }

    /// C++: `void MSCFModel_KraussX::~MSCFModel_KraussX() noexcept` [virtual]
    /// Destructor
    // MSCFModel_KraussX::~MSCFModel_KraussX() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_KraussX::patchSpeedBeforeLC(const MSVehicle *, double, double) const` [const, virtual]
    /// apply dawdling
    // double
    // MSCFModel_KraussX::patchSpeedBeforeLC(const MSVehicle* veh, double vMin, double vMax) const {
    //     return dawdleX(veh->getSpeed(), vMin, vMax, veh->getRNG());
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_before_lc(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        self.dawdle_x(veh.get_speed(), vMin, vMax, veh.get_rng())
    }

    /// C++: `int MSCFModel_KraussX::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_KRAUSSX;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_KRAUSSX as i32
    }

    /// C++: `MSCFModel * MSCFModel_KraussX::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_KraussX::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_KraussX(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_KraussX::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `double MSCFModel_KraussX::dawdleX(double, double, double, SumoRNG *) const` [const, private]
    /// Applies driver imperfection (dawdling / sigma)
    // double
    // MSCFModel_KraussX::dawdleX(double vOld, double vMin, double vMax, SumoRNG* rng) const {
    //     double speed = vMax;
    //     if (!MSGlobals::gSemiImplicitEulerUpdate) {
    //         // in case of the ballistic update, negative speeds indicate
    //         // a desired stop before the completion of the next timestep.
    //         // We do not allow dawdling to overwrite this indication
    //         if (speed < 0) {
    //             return speed;
    //         }
    //     }
    //     // extra slow to start
    //     if (vOld < myAccel) {
    //         speed -= ACCEL2SPEED(myTmp1 * myAccel);
    //     }
    //     const double random = RandHelper::rand(rng);
    //     speed -= ACCEL2SPEED(myDawdle * myAccel * random);
    //     // overbraking
    //     if (vOld > vMax) {
    //         speed -= ACCEL2SPEED(myTmp2 * myAccel * random);
    //         //std::cout << " vMin=" << vMin << " vMax=" << vMax << "speed=" << speed << " d1=" << ACCEL2SPEED(myDawdle * myAccel * random) << " d2=" << ACCEL2SPEED(myTmp2 * myAccel * random) << " unexpectedDecel=" << (speed < vMin) << "\n";
    //         if (MSGlobals::gSemiImplicitEulerUpdate) {
    //             speed = MAX2(0.0, speed);
    //         }
    //     }
    //     speed = MAX2(vMin, speed);
    //     return speed;
    // }
    #[allow(unused_variables)]
    pub fn dawdle_x(&self, vOld: f64, vMin: f64, vMax: f64, rng: Ptr<SumoRNG>) -> f64 {
        let mut speed = vMax;
        if !g_semi_implicit_euler_update() {
            // in case of the ballistic update, negative speeds indicate
            // a desired stop before the completion of the next timestep.
            // We do not allow dawdling to overwrite this indication
            if speed < 0. {
                return speed;
            }
        }
        // extra slow to start
        if vOld < self.myAccel {
            speed -= accel2speed(self.myTmp1 * self.myAccel);
        }
        let random = rng.m().rand();
        speed -= accel2speed(self.myDawdle * self.myAccel * random);
        // overbraking
        if vOld > vMax {
            speed -= accel2speed(self.myTmp2 * self.myAccel * random);
            if g_semi_implicit_euler_update() {
                speed = max2(0.0, speed);
            }
        }
        speed = max2(vMin, speed);
        speed
    }

}

