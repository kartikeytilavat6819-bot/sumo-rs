//! Port of microsim/cfmodels/MSCFModel_KraussOrig1.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The original Krauss (1998) car-following model and parameter
/// C++ class `MSCFModel_KraussOrig1` (microsim/cfmodels/MSCFModel_KraussOrig1.h)
pub struct MSCFModel_KraussOrig1 {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// The vehicle's dawdle-parameter. 0 for no dawdling, 1 for max.
    pub myDawdle: f64,
    /// The precomputed value for myDecel*myTau
    pub myTauDecel: f64,
}

impl std::ops::Deref for MSCFModel_KraussOrig1 {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_KraussOrig1 {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_KraussOrig1 {
    /// C++: `void MSCFModel_KraussOrig1::MSCFModel_KraussOrig1(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_KraussOrig1::MSCFModel_KraussOrig1(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     myDawdle(vtype->getParameter().getCFParam(SUMO_ATTR_SIGMA, SUMOVTypeParameter::getDefaultImperfection(vtype->getParameter().vehicleClass))),
    //     myTauDecel(myDecel * myHeadwayTime) {
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        todo!("port MSCFModel_KraussOrig1::MSCFModel_KraussOrig1")
    }

    /// C++: `void MSCFModel_KraussOrig1::~MSCFModel_KraussOrig1() noexcept` [virtual]
    /// Destructor
    // MSCFModel_KraussOrig1::~MSCFModel_KraussOrig1() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_KraussOrig1::~MSCFModel_KraussOrig1")
    }

    /// C++: `double MSCFModel_KraussOrig1::patchSpeedBeforeLC(const MSVehicle *, double, double) const` [const, virtual]
    /// apply dawdling
    // double
    // MSCFModel_KraussOrig1::patchSpeedBeforeLC(const MSVehicle* veh, double vMin, double vMax) const {
    //     UNUSED_PARAMETER(veh);
    //     const double vDawdle = MAX2(vMin, dawdle(vMax, veh->getRNG()));
    //     return vDawdle;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_before_lc(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        todo!("port MSCFModel_KraussOrig1::patchSpeedBeforeLC")
    }

    /// C++: `double MSCFModel_KraussOrig1::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_KraussOrig1::followSpeed(const MSVehicle* const veh, double speed, double gap, double predSpeed, double predMaxDecel, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MIN2(vsafe(gap, predSpeed, predMaxDecel), maxNextSpeed(speed, veh)); // XXX: and why not cap with minNextSpeed!? (Leo)
    //     } else {
    //         return MAX2(MIN2(maximumSafeFollowSpeed(gap, speed, predSpeed, predMaxDecel), maxNextSpeed(speed, veh)), minNextSpeed(speed));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_KraussOrig1::followSpeed")
    }

    /// C++: `double MSCFModel_KraussOrig1::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_KraussOrig1::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MIN2(vsafe(gap, 0., 0.), maxNextSpeed(speed, veh));
    //     } else {
    //         // XXX: using this here is probably in the spirit of Krauss, but we should consider,
    //         // if the original vsafe should be kept instead (Leo), refs. #2575
    //         return MIN2(maximumSafeStopSpeedBallistic(gap, decel, speed), maxNextSpeed(speed, veh));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_KraussOrig1::stopSpeed")
    }

    /// C++: `int MSCFModel_KraussOrig1::getModelID() const` [const, virtual]
    /// Returns the model's name
    // virtual int getModelID() const {
    //         return SUMO_TAG_CF_KRAUSS_ORIG1;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_KraussOrig1::getModelID")
    }

    /// C++: `double MSCFModel_KraussOrig1::getImperfection() const` [const, virtual]
    /// Get the driver's imperfection
    // double getImperfection() const {
    //         return myDawdle;
    //     }
    #[allow(unused_variables)]
    pub fn get_imperfection(&self) -> f64 {
        todo!("port MSCFModel_KraussOrig1::getImperfection")
    }

    /// C++: `void MSCFModel_KraussOrig1::setMaxDecel(double)` [virtual]
    /// Sets a new value for maximum deceleration [m/s^2]
    // void setMaxDecel(double decel) {
    //         myDecel = decel;
    //         myTauDecel = myDecel * myHeadwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_decel(&mut self, decel: f64) {
        todo!("port MSCFModel_KraussOrig1::setMaxDecel")
    }

    /// C++: `void MSCFModel_KraussOrig1::setImperfection(double)` [virtual]
    /// Sets a new value for driver imperfection
    // void setImperfection(double imperfection) {
    //         myDawdle = imperfection;
    //     }
    #[allow(unused_variables)]
    pub fn set_imperfection(&mut self, imperfection: f64) {
        todo!("port MSCFModel_KraussOrig1::setImperfection")
    }

    /// C++: `void MSCFModel_KraussOrig1::setHeadwayTime(double)` [virtual]
    /// Sets a new value for desired headway [s]
    // void setHeadwayTime(double headwayTime) {
    //         myHeadwayTime = headwayTime;
    //         myTauDecel = myDecel * headwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_time(&mut self, headwayTime: f64) {
        todo!("port MSCFModel_KraussOrig1::setHeadwayTime")
    }

    /// C++: `MSCFModel * MSCFModel_KraussOrig1::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_KraussOrig1::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_KraussOrig1(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_KraussOrig1::duplicate")
    }

    /// C++: `double MSCFModel_KraussOrig1::vsafe(double, double, double) const` [const, virtual, protected]
    /// Returns the "safe" velocity
    // double MSCFModel_KraussOrig1::vsafe(double gap, double predSpeed, double /* predMaxDecel */) const {
    //     if (predSpeed == 0 && gap < 0.01) {
    //         return 0;
    //     } else if (predSpeed == 0 &&  gap <= ACCEL2SPEED(myDecel)) {
    //         // workaround for #2310
    //         return MIN2(ACCEL2SPEED(myDecel), DIST2SPEED(gap));
    //     }
    //     double vsafe = (double)(-1. * myTauDecel
    //                             + sqrt(
    //                                 myTauDecel * myTauDecel
    //                                 + (predSpeed * predSpeed)
    //                                 + (2. * myDecel * gap)
    //                             ));
    //     assert(vsafe >= 0);
    //     return vsafe;
    // }
    #[allow(unused_variables)]
    pub fn vsafe(&self, gap: f64, predSpeed: f64, predMaxDecel: f64) -> f64 {
        todo!("port MSCFModel_KraussOrig1::vsafe")
    }

    /// C++: `double MSCFModel_KraussOrig1::dawdle(double, SumoRNG *) const` [const, virtual, protected]
    /// Applies driver imperfection (dawdling / sigma)
    // double
    // MSCFModel_KraussOrig1::dawdle(double speed, SumoRNG* rng) const {
    //     if (!MSGlobals::gSemiImplicitEulerUpdate) {
    //         // in case of the ballistic update, negative speeds indicate
    //         // a desired stop before the completion of the next timestep.
    //         // We do not allow dawdling to overwrite this indication
    //         if (speed < 0) {
    //             return speed;
    //         }
    //     }
    //     return MAX2(0., speed - ACCEL2SPEED(myDawdle * myAccel * RandHelper::rand(rng)));
    // }
    #[allow(unused_variables)]
    pub fn dawdle(&self, speed: f64, rng: Ptr<SumoRNG>) -> f64 {
        todo!("port MSCFModel_KraussOrig1::dawdle")
    }

}

