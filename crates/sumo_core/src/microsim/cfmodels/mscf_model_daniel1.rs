//! Port of microsim/cfmodels/MSCFModel_Daniel1.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The original Krauss (1998) car-following model and parameter
/// C++ class `MSCFModel_Daniel1` (microsim/cfmodels/MSCFModel_Daniel1.h)
pub struct MSCFModel_Daniel1 {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// The vehicle's dawdle-parameter. 0 for no dawdling, 1 for max.
    pub myDawdle: f64,
    /// The precomputed value for myDecel*myTau
    pub myTauDecel: f64,
    /// temporary (testing) parameter
    pub myTmp1: f64,
    /// temporary (testing) parameter
    pub myTmp2: f64,
    /// temporary (testing) parameter
    pub myTmp3: f64,
    /// temporary (testing) parameter
    pub myTmp4: f64,
    /// temporary (testing) parameter
    pub myTmp5: f64,
}

impl std::ops::Deref for MSCFModel_Daniel1 {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Daniel1 {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_Daniel1 {
    /// C++: `void MSCFModel_Daniel1::MSCFModel_Daniel1(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_Daniel1::MSCFModel_Daniel1(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     myDawdle(vtype->getParameter().getCFParam(SUMO_ATTR_SIGMA, SUMOVTypeParameter::getDefaultImperfection(vtype->getParameter().vehicleClass))),
    //     myTauDecel(myDecel * myHeadwayTime),
    //     myTmp1(vtype->getParameter().getCFParam(SUMO_ATTR_TMP1, 1.0)),
    //     myTmp2(vtype->getParameter().getCFParam(SUMO_ATTR_TMP2, 1.0)),
    //     myTmp3(vtype->getParameter().getCFParam(SUMO_ATTR_TMP3, 1.0)),
    //     myTmp4(vtype->getParameter().getCFParam(SUMO_ATTR_TMP4, 1.0)),
    //     myTmp5(vtype->getParameter().getCFParam(SUMO_ATTR_TMP5, 1.0)) {
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        todo!("port MSCFModel_Daniel1::MSCFModel_Daniel1")
    }

    /// C++: `void MSCFModel_Daniel1::~MSCFModel_Daniel1() noexcept` [virtual]
    /// Destructor
    // MSCFModel_Daniel1::~MSCFModel_Daniel1() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_Daniel1::~MSCFModel_Daniel1")
    }

    /// C++: `double MSCFModel_Daniel1::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences
    // double
    // MSCFModel_Daniel1::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     const double oldV = veh->getSpeed(); // save old v for optional acceleration computation
    //     const double vSafe = MIN2(vPos, veh->processNextStop(vPos)); // process stops
    //     // we need the acceleration for emission computation;
    //     //  in this case, we neglect dawdling, nonetheless, using
    //     //  vSafe does not incorporate speed reduction due to interaction
    //     //  on lane changing
    //     const double vMin = getSpeedAfterMaxDecel(oldV);
    //     const double vMax = MIN3(veh->getLane()->getVehicleMaxSpeed(veh), maxNextSpeed(oldV, veh), vSafe);
    // #ifdef _DEBUG
    //     if (vMin > vMax) {
    //         WRITE_WARNINGF(TL("Maximum speed of vehicle '%' is lower than the minimum speed (min: %, max: %)."), veh->getID(), toString(vMin), toString(vMax));
    //     }
    // #endif
    //     return veh->getLaneChangeModel().patchSpeed(vMin, MAX2(vMin, dawdle(vMax, veh->getRNG())), vMax, *this);
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        todo!("port MSCFModel_Daniel1::finalizeSpeed")
    }

    /// C++: `double MSCFModel_Daniel1::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_Daniel1::followSpeed(const MSVehicle* const veh, double speed, double gap, double predSpeed, double /*predMaxDecel*/, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //     return MIN2(_vsafe(gap, predSpeed), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_Daniel1::followSpeed")
    }

    /// C++: `double MSCFModel_Daniel1::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_Daniel1::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double /*decel*/, const CalcReason /*usage*/) const {
    //     return MIN2(_vsafe(gap, 0), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_Daniel1::stopSpeed")
    }

    /// C++: `int MSCFModel_Daniel1::getModelID() const` [const, virtual]
    /// Returns the model's name
    // virtual int getModelID() const {
    //         return SUMO_TAG_CF_DANIEL1;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_Daniel1::getModelID")
    }

    /// C++: `double MSCFModel_Daniel1::getImperfection() const` [const, virtual]
    /// Get the driver's imperfection
    // double getImperfection() const {
    //         return myDawdle;
    //     }
    #[allow(unused_variables)]
    pub fn get_imperfection(&self) -> f64 {
        todo!("port MSCFModel_Daniel1::getImperfection")
    }

    /// C++: `void MSCFModel_Daniel1::setMaxDecel(double)` [virtual]
    /// Sets a new value for maximum deceleration [m/s^2]
    // void setMaxDecel(double decel) {
    //         myDecel = decel;
    //         myTauDecel = myDecel * myHeadwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_decel(&mut self, decel: f64) {
        todo!("port MSCFModel_Daniel1::setMaxDecel")
    }

    /// C++: `void MSCFModel_Daniel1::setImperfection(double)` [virtual]
    /// Sets a new value for driver imperfection
    // void setImperfection(double imperfection) {
    //         myDawdle = imperfection;
    //     }
    #[allow(unused_variables)]
    pub fn set_imperfection(&mut self, imperfection: f64) {
        todo!("port MSCFModel_Daniel1::setImperfection")
    }

    /// C++: `void MSCFModel_Daniel1::setHeadwayTime(double)` [virtual]
    /// Sets a new value for driver reaction time [s]
    // void setHeadwayTime(double headwayTime) {
    //         myHeadwayTime = headwayTime;
    //         myTauDecel = myDecel * headwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_time(&mut self, headwayTime: f64) {
        todo!("port MSCFModel_Daniel1::setHeadwayTime")
    }

    /// C++: `MSCFModel * MSCFModel_Daniel1::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_Daniel1::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Daniel1(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_Daniel1::duplicate")
    }

    /// C++: `double MSCFModel_Daniel1::_vsafe(double, double) const` [const, virtual, private]
    /// Returns the "safe" velocity
    // double MSCFModel_Daniel1::_vsafe(double gap, double predSpeed) const {
    //     if (predSpeed == 0 && gap < 0.01) {
    //         return 0;
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
    pub fn vsafe(&self, gap: f64, predSpeed: f64) -> f64 {
        todo!("port MSCFModel_Daniel1::_vsafe")
    }

    /// C++: `double MSCFModel_Daniel1::dawdle(double, SumoRNG *) const` [const, virtual, private]
    /// Applies driver imperfection (dawdling / sigma)
    // double
    // MSCFModel_Daniel1::dawdle(double speed, SumoRNG* rng) const {
    //     return MAX2(0., speed - ACCEL2SPEED(myDawdle * myAccel * RandHelper::rand(rng)));
    // }
    #[allow(unused_variables)]
    pub fn dawdle(&self, speed: f64, rng: Ptr<SumoRNG>) -> f64 {
        todo!("port MSCFModel_Daniel1::dawdle")
    }

}

