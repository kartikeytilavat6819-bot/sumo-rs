//! Port of microsim/cfmodels/MSCFModel_SmartSK.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use utils::common::time::{accel2speed, ts};
use utils::common::to_string::f64_to_string_default;

/// The original Krauss (1998) car-following model and parameter
/// C++ class `MSCFModel_SmartSK` (microsim/cfmodels/MSCFModel_SmartSK.h)
#[repr(C)]
pub struct MSCFModel_SmartSK {
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
    /// maxDeltaGap is the theoretical maximum change in gap that can happen in one time step
    pub myS2Sspeed: f64,
    /// maxDeltaGap is the theoretical maximum change in gap that can happen in one time step
    pub maxDeltaGap: f64,
}

fn smart_sk_finalize_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
    cf_cast::<MSCFModel_SmartSK>(m).finalize_speed(veh, vPos)
}
fn smart_sk_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_SmartSK>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage)
}
fn smart_sk_patch_speed_before_lc(m: &MSCFModel, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
    cf_cast::<MSCFModel_SmartSK>(m).patch_speed_before_lc(veh, vMin, vMax)
}
fn smart_sk_get_model_id(m: &MSCFModel) -> i32 {
    cf_cast::<MSCFModel_SmartSK>(m).get_model_id()
}
fn smart_sk_get_imperfection(m: &MSCFModel) -> f64 {
    cf_cast::<MSCFModel_SmartSK>(m).get_imperfection()
}
fn smart_sk_set_max_decel(m: &mut MSCFModel, decel: f64) {
    cf_cast_mut::<MSCFModel_SmartSK>(m).set_max_decel(decel)
}
fn smart_sk_set_imperfection(m: &mut MSCFModel, imperfection: f64) {
    cf_cast_mut::<MSCFModel_SmartSK>(m).set_imperfection(imperfection)
}
fn smart_sk_set_headway_time(m: &mut MSCFModel, headwayTime: f64) {
    cf_cast_mut::<MSCFModel_SmartSK>(m).set_headway_time(headwayTime)
}
fn smart_sk_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
    cf_cast::<MSCFModel_SmartSK>(m).duplicate(vtype)
}
fn smart_sk_save_vehicle_variables(m: &MSCFModel, vars: &MSCFModel_VehicleVariables, out: &mut OutputDevice) {
    unsafe { &*(vars as *const MSCFModel_VehicleVariables as *const MSCFModel_SmartSK_SSKVehicleVariables) }.save_state(out, m)
}
fn smart_sk_load_vehicle_variables(_m: &MSCFModel, vars: &mut MSCFModel_VehicleVariables, attrs: &SUMOSAXAttributes) {
    unsafe { &mut *(vars as *mut MSCFModel_VehicleVariables as *mut MSCFModel_SmartSK_SSKVehicleVariables) }.load_state(attrs).unwrap_or_else(|e| panic!("{}", e))
}
fn smart_sk_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> {
    cf_cast::<MSCFModel_SmartSK>(m).create_vehicle_variables()
}

fn smart_sk_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_SmartSK>(m).stop_speed(veh, speed, gap, decel, usage)
}

/// Virtual table of MSCFModel_SmartSK: its overrides on top of the MSCFModel table
pub const SMART_SK_VT: MSCFModelVt = MSCFModelVt {
    stop_speed_2: smart_sk_stop_speed_2,
    finalize_speed: smart_sk_finalize_speed,
    follow_speed: smart_sk_follow_speed,
    patch_speed_before_lc: smart_sk_patch_speed_before_lc,
    get_model_id: smart_sk_get_model_id,
    get_imperfection: smart_sk_get_imperfection,
    set_max_decel: smart_sk_set_max_decel,
    set_imperfection: smart_sk_set_imperfection,
    set_headway_time: smart_sk_set_headway_time,
    duplicate: smart_sk_duplicate,
    create_vehicle_variables: smart_sk_create_vehicle_variables,
    save_vehicle_variables: smart_sk_save_vehicle_variables,
    load_vehicle_variables: smart_sk_load_vehicle_variables,
    ..DEFAULT_MSCF_MODEL_VT
};

impl std::ops::Deref for MSCFModel_SmartSK {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_SmartSK {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_SmartSK {
    /// C++: `void MSCFModel_SmartSK::MSCFModel_SmartSK(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_SmartSK::MSCFModel_SmartSK(const MSVehicleType* vtype) :
    // // check whether setting these variables here with default values is ''good'' SUMO design
    // //        double tmp1=0.0, double tmp2=5.0, double tmp3=0.0, double tmp4, double tmp5)
    //     MSCFModel(vtype),
    //     myDawdle(vtype->getParameter().getCFParam(SUMO_ATTR_SIGMA, SUMOVTypeParameter::getDefaultImperfection(vtype->getParameter().vehicleClass))),
    //     myTauDecel(myDecel * myHeadwayTime),
    //     myTmp1(vtype->getParameter().getCFParam(SUMO_ATTR_TMP1, 1.0)),
    //     myTmp2(vtype->getParameter().getCFParam(SUMO_ATTR_TMP2, 1.0)),
    //     myTmp3(vtype->getParameter().getCFParam(SUMO_ATTR_TMP3, 1.0)),
    //     myTmp4(vtype->getParameter().getCFParam(SUMO_ATTR_TMP4, 1.0)),
    //     myTmp5(vtype->getParameter().getCFParam(SUMO_ATTR_TMP5, 1.0)) {
    //     // the variable tmp1 is the acceleration delay time, e.g. two seconds (or something like this).
    //     // for use in the upate process, a rule like if (v<myTmp1) vsafe = 0; is needed.
    //     // To have this, we have to transform myTmp1 (which is a time) into an equivalent speed. This is done by the
    //     // using the vsafe formula and computing:
    //     // v(t=myTmp1) = -myTauDecel + sqrt(myTauDecel*myTauDecel + accel*(accel + decel)*t*t + accel*decel*t*TS);
    //     double t = myTmp1;
    //     myS2Sspeed = -myTauDecel + sqrt(myTauDecel * myTauDecel + myAccel * (myAccel + myDecel) * t * t + myAccel * myDecel * t * TS);
    // #ifdef SmartSK_DEBUG
    //     std::cout << "# s2s-speed: " << myS2Sspeed << std::endl;
    // #endif
    //     if (myS2Sspeed > 5.0) {
    //         myS2Sspeed = 5.0;
    //     }
    // // double maxDeltaGap = -0.5*ACCEL2DIST(myDecel + myAccel);
    //     maxDeltaGap = -0.5 * (myDecel + myAccel) * TS * TS;
    // #ifdef SmartSK_DEBUG
    //     std::cout << "# maxDeltaGap = " << maxDeltaGap << std::endl;
    // #endif
    //     myTmp2 = TS / myTmp2;
    //     myTmp3 = sqrt(TS) * myTmp3;
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let myDawdle = vtype.get_parameter().get_cf_param(SUMO_ATTR_SIGMA, SUMOVTypeParameter::get_default_imperfection(vtype.get_parameter().vehicleClass));
        let mut base = MSCFModel::new(vtype);
        let myTauDecel = base.myDecel * base.myHeadwayTime;
        let myTmp1 = vtype.get_parameter().get_cf_param(SUMO_ATTR_TMP1, 1.0);
        let myTmp2 = vtype.get_parameter().get_cf_param(SUMO_ATTR_TMP2, 1.0);
        let myTmp3 = vtype.get_parameter().get_cf_param(SUMO_ATTR_TMP3, 1.0);
        let myTmp4 = vtype.get_parameter().get_cf_param(SUMO_ATTR_TMP4, 1.0);
        let myTmp5 = vtype.get_parameter().get_cf_param(SUMO_ATTR_TMP5, 1.0);
        // the variable tmp1 is the acceleration delay time, e.g. two seconds (or something like this).
        // for use in the upate process, a rule like if (v<myTmp1) vsafe = 0; is needed.
        // To have this, we have to transform myTmp1 (which is a time) into an equivalent speed. This is done by the
        // using the vsafe formula and computing:
        // v(t=myTmp1) = -myTauDecel + sqrt(myTauDecel*myTauDecel + accel*(accel + decel)*t*t + accel*decel*t*TS);
        let t = myTmp1;
        let mut myS2Sspeed = -myTauDecel + (myTauDecel * myTauDecel + base.myAccel * (base.myAccel + base.myDecel) * t * t + base.myAccel * base.myDecel * t * ts()).sqrt();
        if myS2Sspeed > 5.0 {
            myS2Sspeed = 5.0;
        }
        // double maxDeltaGap = -0.5*ACCEL2DIST(myDecel + myAccel);
        let maxDeltaGap = -0.5 * (base.myDecel + base.myAccel) * ts() * ts();
        let myTmp2 = ts() / myTmp2;
        let myTmp3 = ts().sqrt() * myTmp3;
        let mut this = MSCFModel_SmartSK { base, myDawdle, myTauDecel, myTmp1, myTmp2, myTmp3, myTmp4, myTmp5, myS2Sspeed, maxDeltaGap };
        this.base.vt = &SMART_SK_VT;
        this
    }

    /// C++: `void MSCFModel_SmartSK::~MSCFModel_SmartSK() noexcept` [virtual]
    /// Destructor
    // MSCFModel_SmartSK::~MSCFModel_SmartSK() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_SmartSK::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences
    // double
    // MSCFModel_SmartSK::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     const double vNext = MSCFModel::finalizeSpeed(veh, vPos);
    //     updateMyHeadway(veh);
    //     SSKVehicleVariables* vars = (SSKVehicleVariables*)veh->getCarFollowVariables();
    // #ifdef SmartSK_DEBUG
    //     if (vars->ggOld.size() > 1) {
    //         std::cout << "# more than one entry in ggOld list. Speed is " << vPos << ", corresponding dist is " << vars->ggOld[(int) vPos] << "\n";
    //         for (std::map<int, double>::iterator I = vars->ggOld.begin(); I != vars->ggOld.end(); I++) {
    //             std::cout << "# " << (*I).first << ' ' << (*I).second << std::endl;
    //         }
    //     }
    // #endif
    //     vars->gOld = vars->ggOld[(int) vPos];
    //     vars->ggOld.clear();
    //     return vNext;
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        let vNext = self.base.finalize_speed_base(veh, vPos);
        self.update_my_headway(veh);
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_SmartSK_SSKVehicleVariables);
        // ggOld[(int) vPos] inserts a 0 entry when the key is missing, as std::map::operator[] does
        let gOld = *vars.m().ggOld.entry(vPos as i32).or_insert(0.);
        vars.m().gOld = gOld;
        vars.m().ggOld.clear();
        vNext
    }

    /// C++: `double MSCFModel_SmartSK::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_SmartSK::followSpeed(const MSVehicle* const veh, double speed, double gap, double predSpeed, double /*predMaxDecel*/, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //     SSKVehicleVariables* vars = (SSKVehicleVariables*)veh->getCarFollowVariables();
    //
    // // if (((gap - vars->gOld) < maxDeltaGap) && (speed>=5.0) && gap>=5.0) {
    //     if ((gap - vars->gOld) < maxDeltaGap) {
    //         double tTauTest = gap / speed;
    // // allow  headway only to decrease only, never to increase. Increase is handled automatically by the headway dynamics in finalizeSpeed()!!!
    //         if ((tTauTest < vars->myHeadway) && (tTauTest > TS)) {
    //             vars->myHeadway = tTauTest;
    //         }
    //     }
    //
    //     double vsafe = _vsafe(veh, gap, predSpeed);
    //     if ((speed <= 0.0) && (vsafe < myS2Sspeed)) {
    //         vsafe = 0;
    //     }
    //
    //     double vNew = MAX2(getSpeedAfterMaxDecel(speed), MIN2(vsafe, maxNextSpeed(speed, veh)));
    //     // there must be a better place to do the following assignment!!!
    //     vars->gOld = gap;
    //     vars->ggOld[(int)vNew] = gap;
    //     return vNew;
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_SmartSK_SSKVehicleVariables);
        // if (((gap - vars->gOld) < maxDeltaGap) && (speed>=5.0) && gap>=5.0) {
        if (gap2pred - vars.gOld) < self.maxDeltaGap {
            let tTauTest = gap2pred / speed;
            // allow  headway only to decrease only, never to increase. Increase is handled automatically by the headway dynamics in finalizeSpeed()!!!
            if tTauTest < vars.myHeadway && tTauTest > ts() {
                vars.m().myHeadway = tTauTest;
            }
        }

        let mut vsafe = self.vsafe(veh, gap2pred, predSpeed);
        if speed <= 0.0 && vsafe < self.myS2Sspeed {
            vsafe = 0.;
        }

        let vNew = max2(self.get_speed_after_max_decel(speed), min2(vsafe, self.max_next_speed(speed, veh)));
        // there must be a better place to do the following assignment!!!
        vars.m().gOld = gap2pred;
        vars.m().ggOld.insert(vNew as i32, gap2pred);
        vNew
    }

    /// C++: `double MSCFModel_SmartSK::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_SmartSK::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double /*decel*/, const CalcReason /*usage*/) const {
    //     SSKVehicleVariables* vars = (SSKVehicleVariables*)veh->getCarFollowVariables();
    //
    // // if (((gap - vars->gOld) < maxDeltaGap) && (speed>=5.0) && gap>=5.0) {
    //     if ((gap - vars->gOld) < maxDeltaGap) {
    //         double tTauTest = gap / speed;
    // // allow  headway only to decrease only, never to increase. Increase is handled automatically by the headway dynamics in finalizeSpeed()!!!
    //         if ((tTauTest < vars->myHeadway) && (tTauTest > TS)) {
    //             vars->myHeadway = tTauTest;
    //         }
    //     }
    //
    //     return MAX2(getSpeedAfterMaxDecel(speed), MIN2(_vsafe(veh, gap, 0), maxNextSpeed(speed, veh)));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_SmartSK_SSKVehicleVariables);
        // if (((gap - vars->gOld) < maxDeltaGap) && (speed>=5.0) && gap>=5.0) {
        if (gap2pred - vars.gOld) < self.maxDeltaGap {
            let tTauTest = gap2pred / speed;
            // allow  headway only to decrease only, never to increase. Increase is handled automatically by the headway dynamics in finalizeSpeed()!!!
            if tTauTest < vars.myHeadway && tTauTest > ts() {
                vars.m().myHeadway = tTauTest;
            }
        }

        max2(self.get_speed_after_max_decel(speed), min2(self.vsafe(veh, gap2pred, 0.), self.max_next_speed(speed, veh)))
    }

    /// C++: `int MSCFModel_SmartSK::getModelID() const` [const, virtual]
    /// Returns the model's name
    // virtual int getModelID() const {
    //         return SUMO_TAG_CF_SMART_SK;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_SMART_SK as i32
    }

    /// C++: `double MSCFModel_SmartSK::getImperfection() const` [const, virtual]
    /// Get the driver's imperfection
    // double getImperfection() const {
    //         return myDawdle;
    //     }
    #[allow(unused_variables)]
    pub fn get_imperfection(&self) -> f64 {
        self.myDawdle
    }

    /// C++: `void MSCFModel_SmartSK::setMaxDecel(double)` [virtual]
    /// Sets a new value for maximum deceleration [m/s^2]
    // void setMaxDecel(double decel) {
    //         myDecel = decel;
    //         myTauDecel = myDecel * myHeadwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_decel(&mut self, decel: f64) {
        self.myDecel = decel;
        self.myTauDecel = self.myDecel * self.myHeadwayTime;
    }

    /// C++: `void MSCFModel_SmartSK::setImperfection(double)` [virtual]
    /// Sets a new value for driver imperfection
    // void setImperfection(double imperfection) {
    //         myDawdle = imperfection;
    //     }
    #[allow(unused_variables)]
    pub fn set_imperfection(&mut self, imperfection: f64) {
        self.myDawdle = imperfection;
    }

    /// C++: `void MSCFModel_SmartSK::setHeadwayTime(double)` [virtual]
    /// Sets a new value for desired headway [s]
    // void setHeadwayTime(double headwayTime) {
    //         myHeadwayTime = headwayTime;
    //         myTauDecel = myDecel * headwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_time(&mut self, headwayTime: f64) {
        self.myHeadwayTime = headwayTime;
        self.myTauDecel = self.myDecel * headwayTime;
    }

    /// C++: `double MSCFModel_SmartSK::patchSpeedBeforeLC(const MSVehicle *, double, double) const` [const, virtual]
    /// apply dawdling
    // double
    // MSCFModel_SmartSK::patchSpeedBeforeLC(const MSVehicle* veh, double /*vMin*/, double /*vMax*/) const {
    //     return dawdle(veh->getSpeed(), veh->getRNG());
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_before_lc(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        self.dawdle(veh.get_speed(), veh.get_rng())
    }

    /// C++: `MSCFModel * MSCFModel_SmartSK::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_SmartSK::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_SmartSK(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_SmartSK::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `double MSCFModel_SmartSK::_vsafe(const MSVehicle *const, double, double) const` [const, virtual, private]
    /// Returns the "safe" velocity
    // double MSCFModel_SmartSK::_vsafe(const MSVehicle* const veh, double gap, double predSpeed) const {
    //     if (predSpeed == 0 && gap < 0.01) {
    //         return 0;
    //     }
    //     SSKVehicleVariables* vars = (SSKVehicleVariables*)veh->getCarFollowVariables();
    //     // this is the most obvious change to the normal SK: the model uses the variable vars->myHeadway instead of the constant
    //     // myHeadwayTime as the "desired headway" tau
    //     double bTau = myDecel * (vars->myHeadway);
    //     double vsafe = (double)(-1. * bTau
    //                             + sqrt(
    //                                 bTau * bTau
    //                                 + (predSpeed * predSpeed)
    //                                 + (2. * myDecel * gap)
    //                             ));
    //     assert(vsafe >= 0);
    //     return vsafe;
    // }
    #[allow(unused_variables)]
    pub fn vsafe(&self, veh: Ptr<MSVehicle>, gap: f64, predSpeed: f64) -> f64 {
        if predSpeed == 0. && gap < 0.01 {
            return 0.;
        }
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_SmartSK_SSKVehicleVariables);
        // this is the most obvious change to the normal SK: the model uses the variable vars->myHeadway instead of the constant
        // myHeadwayTime as the "desired headway" tau
        let bTau = self.myDecel * vars.myHeadway;
        let vsafe = -1. * bTau + (bTau * bTau + (predSpeed * predSpeed) + (2. * self.myDecel * gap)).sqrt();
        debug_assert!(vsafe >= 0.);
        vsafe
    }

    /// C++: `double MSCFModel_SmartSK::dawdle(double, SumoRNG *) const` [const, virtual, private]
    /// Applies driver imperfection (dawdling / sigma)
    // double
    // MSCFModel_SmartSK::dawdle(double speed, SumoRNG* rng) const {
    //     return MAX2(0., speed - ACCEL2SPEED(myDawdle * myAccel * RandHelper::rand(rng)));
    // }
    #[allow(unused_variables)]
    pub fn dawdle(&self, speed: f64, rng: Ptr<SumoRNG>) -> f64 {
        max2(0., speed - accel2speed(self.myDawdle * self.myAccel * rng.m().rand()))
    }

    /// C++: `void MSCFModel_SmartSK::updateMyHeadway(const MSVehicle *const) const` [const, virtual, private]
    // virtual void updateMyHeadway(const MSVehicle* const veh) const {
    //         // this is the point were the preferred headway changes slowly:
    //         SSKVehicleVariables* vars = (SSKVehicleVariables*)veh->getCarFollowVariables();
    //         double tTau = vars->myHeadway;
    //         tTau = tTau + (myHeadwayTime - tTau) * myTmp2 + myTmp3 * tTau * RandHelper::rand(double(-1.0), double(1.0), veh->getRNG());
    //         if (tTau < TS) { // this ensures the SK safety condition
    //             tTau = TS;
    //         }
    //         vars->myHeadway = tTau;
    //     }
    #[allow(unused_variables)]
    pub fn update_my_headway(&self, veh: Ptr<MSVehicle>) {
        // this is the point were the preferred headway changes slowly:
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_SmartSK_SSKVehicleVariables);
        let mut tTau = vars.myHeadway;
        tTau = tTau + (self.myHeadwayTime - tTau) * self.myTmp2 + self.myTmp3 * tTau * veh.get_rng().m().rand_range(-1.0, 1.0);
        if tTau < ts() {
            // this ensures the SK safety condition
            tTau = ts();
        }
        vars.m().myHeadway = tTau;
    }

    /// C++: `MSCFModel::VehicleVariables * MSCFModel_SmartSK::createVehicleVariables() const` [const, virtual, private]
    // virtual MSCFModel::VehicleVariables* createVehicleVariables() const {
    //         SSKVehicleVariables* ret = new SSKVehicleVariables();
    //         ret->gOld = 0.0;
    //         ret->myHeadway = myHeadwayTime;
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        let vars = MSCFModel_SmartSK_SSKVehicleVariables { base: MSCFModel_VehicleVariables {}, gOld: 0.0, myHeadway: self.myHeadwayTime, ggOld: BTreeMap::new() };
        Ptr::from_raw(Box::into_raw(Box::new(vars)) as *mut MSCFModel_VehicleVariables)
    }

}

/// C++ class `MSCFModel_SmartSK::SSKVehicleVariables` (microsim/cfmodels/MSCFModel_SmartSK.h)
#[repr(C)]
pub struct MSCFModel_SmartSK_SSKVehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    pub gOld: f64,
    pub myHeadway: f64,
    pub ggOld: BTreeMap<i32, f64>,
}

impl std::ops::Deref for MSCFModel_SmartSK_SSKVehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_SmartSK_SSKVehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_SmartSK_SSKVehicleVariables {
    /// C++: `void SSKVehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_SmartSK::SSKVehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "SmartSK");
    //     std::ostringstream internals;
    //     internals << gOld << " ";
    //     internals << myHeadway << " ";
    //     internals << ggOld.size() << " ";
    //     for (auto item : ggOld) {
    //         internals << item.first << " " << item.second << " ";
    //     }
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "SmartSK");
        let mut internals = format!("{} ", f64_to_string_default(self.gOld));
        internals.push_str(&format!("{} ", f64_to_string_default(self.myHeadway)));
        internals.push_str(&format!("{} ", self.ggOld.len()));
        for (k, v) in &self.ggOld {
            internals.push_str(&format!("{} {} ", k, f64_to_string_default(*v)));
        }
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
    }

    /// C++: `void SSKVehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_SmartSK::SSKVehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "SmartSK") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for SmartSK", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> gOld;
    //     bis >> myHeadway;
    //     int ggOldSize;
    //     bis >> ggOldSize;
    //     for (int i = 0; i < ggOldSize; i++) {
    //         int k;
    //         double v;
    //         bis >> k;
    //         bis >> v;
    //         ggOld[k] = v;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "SmartSK" {
            return Err(utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for SmartSK", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        // a failed C++ stream extraction leaves 0 in the target (C++11), hence unwrap_or(0)
        self.gOld = bis.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.);
        self.myHeadway = bis.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.);
        let ggOldSize: i32 = bis.next().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
        for _ in 0..ggOldSize {
            let k: i32 = bis.next().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
            let v: f64 = bis.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.);
            self.ggOld.insert(k, v);
        }
        Ok(())
    }

}

