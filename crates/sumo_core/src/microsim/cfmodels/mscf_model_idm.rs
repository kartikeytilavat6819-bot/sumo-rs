//! Port of microsim/cfmodels/MSCFModel_IDM.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use crate::microsim::ms_globals::MSGlobals;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;
use utils::geom::NUMERICAL_EPS;

/// The Intelligent Driver Model (IDM) car-following model
/// C++ class `MSCFModel_IDM` (microsim/cfmodels/MSCFModel_IDM.h)
#[repr(C)]
pub struct MSCFModel_IDM {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// whether the model is IDMM or IDM
    pub myIDMM: bool,
    /// The IDM delta exponent
    pub myDelta: f64,
    /// The IDMM adaptation factor beta
    pub myAdaptationFactor: f64,
    /// The IDMM adaptation time tau
    pub myAdaptationTime: f64,
    /// The number of iterations in speed calculations
    pub myIterations: i32,
    /// A computational shortcut
    pub myTwoSqrtAccelDecel: f64,
}

impl std::ops::Deref for MSCFModel_IDM {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_IDM {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_IDM {
    /// C++: `void MSCFModel_IDM::MSCFModel_IDM(const MSVehicleType *, bool)`
    /// Constructor
    // MSCFModel_IDM::MSCFModel_IDM(const MSVehicleType* vtype, bool idmm) :
    //     MSCFModel(vtype),
    //     myIDMM(idmm),
    //     myDelta(idmm ? 4.0 : vtype->getParameter().getCFParam(SUMO_ATTR_CF_IDM_DELTA, 4.)),
    //     myAdaptationFactor(idmm ? vtype->getParameter().getCFParam(SUMO_ATTR_CF_IDMM_ADAPT_FACTOR, 1.8) : 1.0),
    //     myAdaptationTime(idmm ? vtype->getParameter().getCFParam(SUMO_ATTR_CF_IDMM_ADAPT_TIME, 600.0) : 0.0),
    //     myIterations(MAX2(1, int(TS / vtype->getParameter().getCFParam(SUMO_ATTR_CF_IDM_STEPPING, .25) + .5))),
    //     myTwoSqrtAccelDecel(double(2 * sqrt(myAccel * myDecel))) {
    //     // IDM does not drive very precise and may violate minGap on occasion
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    //     if (TS / myIterations > 0.25) {
    //         WRITE_WARNINGF("Stepping duration of % for % model in vType % is unsafe", (TS / myIterations), myIDMM ? "IDMM" : "IDM", vtype->getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>, idmm: bool) -> Self {
        let p = vtype.get_parameter();
        let mut base = MSCFModel::new(vtype);
        let myIDMM = idmm;
        let myDelta = if idmm { 4.0 } else { p.get_cf_param(SUMO_ATTR_CF_IDM_DELTA, 4.) };
        let myAdaptationFactor = if idmm { p.get_cf_param(SUMO_ATTR_CF_IDMM_ADAPT_FACTOR, 1.8) } else { 1.0 };
        let myAdaptationTime = if idmm { p.get_cf_param(SUMO_ATTR_CF_IDMM_ADAPT_TIME, 600.0) } else { 0.0 };
        let iterations0 = (ts() / p.get_cf_param(SUMO_ATTR_CF_IDM_STEPPING, 0.25) + 0.5) as i32;
        let myIterations = if 1 > iterations0 { 1 } else { iterations0 };
        let myTwoSqrtAccelDecel = 2. * (base.myAccel * base.myDecel).sqrt();
        // IDM does not drive very precise and may violate minGap on occasion
        base.myCollisionMinGapFactor = p.get_cf_param(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
        if ts() / myIterations as f64 > 0.25 {
            utils::common::msg_handler::write_warning(&format!("Stepping duration of {} for {} model in vType {} is unsafe", f64_to_string_prec(ts() / myIterations as f64, 6), if myIDMM { "IDMM" } else { "IDM" }, base.myType.get_id()));
        }
        MSCFModel_IDM { base, myIDMM, myDelta, myAdaptationFactor, myAdaptationTime, myIterations, myTwoSqrtAccelDecel }
    }

    /// C++: `void MSCFModel_IDM::MSCFModel_IDM(const MSVehicleType *, double, double, double, double, double, double, double, double)`
    /// Constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(vtype: Ptr<MSVehicleType>, accel: f64, decel: f64, emergencyDecel: f64, apparentDecel: f64, headwayTime: f64, adaptationFactor: f64, adaptationTime: f64, internalStepping: f64) -> Self {
        // shortcut: declared in MSCFModel_IDM.h but never defined in the C++ sources, so C++ cannot call it (link error); panic mirrors that
        panic!("MSCFModel_IDM(vtype, accel, decel, ...) is declared but not defined in C++")
    }

    /// C++: `void MSCFModel_IDM::~MSCFModel_IDM() noexcept` [virtual]
    /// Destructor
    // MSCFModel_IDM::~MSCFModel_IDM() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_IDM::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences
    // double
    // MSCFModel_IDM::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     const double vNext = MSCFModel::finalizeSpeed(veh, vPos);
    //     if (myAdaptationFactor != 1.) {
    //         VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //         vars->levelOfService += (vNext / veh->getLane()->getVehicleMaxSpeed(veh) - vars->levelOfService) / myAdaptationTime * TS;
    //     }
    //     return vNext;
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        let vNext = self.base.finalize_speed_base(veh, vPos);
        if self.myAdaptationFactor != 1. {
            let vars = veh.get_car_follow_variables().cast::<MSCFModel_IDM_VehicleVariables>();
            vars.m().levelOfService += (vNext / veh.get_lane().get_vehicle_max_speed(traffic_object(veh)) - vars.levelOfService) / self.myAdaptationTime * ts();
        }
        vNext
    }

    /// C++: `double MSCFModel_IDM::freeSpeed(const MSVehicle *const, double, double, double, const bool, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed without a leader
    // double
    // MSCFModel_IDM::freeSpeed(const MSVehicle* const veh, double speed, double seen, double maxSpeed, const bool /*onInsertion*/, const CalcReason /*usage*/) const {
    //     if (maxSpeed < 0.) {
    //         // can occur for ballistic update (in context of driving at red light)
    //         return maxSpeed;
    //     }
    //     const double secGap = getSecureGap(veh, nullptr, maxSpeed, 0, myDecel);
    //     double vSafe;
    //     if (speed <= maxSpeed) {
    //         // accelerate
    //         vSafe = _v(veh, 1e6, speed, maxSpeed, veh->getLane()->getVehicleMaxSpeed(veh), false);
    //     } else {
    //         // decelerate
    //         // @note relax gap to avoid emergency braking
    //         // @note since the transition point does not move we set the leader speed to 0
    //         vSafe = _v(veh, MAX2(seen, secGap), speed, 0, veh->getLane()->getVehicleMaxSpeed(veh), false);
    //     }
    //     if (seen < secGap) {
    //         // avoid overshoot when close to change in speed limit
    //         vSafe = MIN2(vSafe, maxSpeed);
    //     }
    //     //std::cout << SIMTIME << " speed=" << speed << " maxSpeed=" << maxSpeed << " seen=" << seen << " secGap=" << secGap << " vSafe=" << vSafe << "\n";
    //     return vSafe;
    // }
    #[allow(unused_variables)]
    pub fn free_speed(&self, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
        if maxSpeed < 0. {
            // can occur for ballistic update (in context of driving at red light)
            return maxSpeed;
        }
        let secGap = self.get_secure_gap(veh, Ptr::null(), maxSpeed, 0., self.myDecel);
        let mut vSafe;
        if speed <= maxSpeed {
            // accelerate
            vSafe = self.v(veh, 1e6, speed, maxSpeed, veh.get_lane().get_vehicle_max_speed(traffic_object(veh)), false);
        } else {
            // decelerate
            // @note relax gap to avoid emergency braking
            // @note since the transition point does not move we set the leader speed to 0
            vSafe = self.v(veh, max2(seen, secGap), speed, 0., veh.get_lane().get_vehicle_max_speed(traffic_object(veh)), false);
        }
        if seen < secGap {
            // avoid overshoot when close to change in speed limit
            vSafe = min2(vSafe, maxSpeed);
        }
        vSafe
    }

    /// C++: `double MSCFModel_IDM::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_IDM::followSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred, const CalcReason /*usage*/) const {
    //     applyHeadwayAndSpeedDifferencePerceptionErrors(veh, speed, gap2pred, predSpeed, predMaxDecel, pred);
    //     return _v(veh, gap2pred, speed, predSpeed, veh->getLane()->getVehicleMaxSpeed(veh));
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let mut gap2pred = gap2pred;
        let mut predSpeed = predSpeed;
        self.apply_headway_and_speed_difference_perception_errors(veh, speed, &mut gap2pred, &mut predSpeed, predMaxDecel, pred);
        self.v(veh, gap2pred, speed, predSpeed, veh.get_lane().get_vehicle_max_speed(traffic_object(veh)), true)
    }

    /// C++: `double MSCFModel_IDM::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_IDM::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     applyHeadwayPerceptionError(veh, speed, gap);
    //     if (gap < 0.01) {
    //         return 0;
    //     }
    //     double result = _v(veh, gap, speed, 0, veh->getLane()->getVehicleMaxSpeed(veh), false);
    //     //std::cout << SIMTIME << " stopSpeed speed=" << speed << " gap=" << gap << " decel=" << decel << " result=" << result << "\n";
    //     if (gap > 0 && speed < NUMERICAL_EPS && result < NUMERICAL_EPS) {
    //         // ensure that stops can be reached:
    //         //std::cout << " switching to krauss: " << veh->getID() << " gap=" << gap << " speed=" << speed << " res1=" << result << " res2=" << maximumSafeStopSpeed(gap, speed, false, veh->getActionStepLengthSecs())<< "\n";
    //         result = maximumSafeStopSpeed(gap, decel, speed, false, veh->getActionStepLengthSecs());
    //     }
    //     // avoid overshooting the stop location
    //     if (gap >= 0) {
    //         result = MIN2(result, DIST2SPEED(gap));
    //         //if (result * TS > gap) {
    //         //    std::cout << "Maximum stop speed exceeded for gap=" << gap << " result=" << result << " veh=" << veh->getID() << " speed=" << speed << " t=" << SIMTIME << "\n";
    //         //}
    //     }
    //
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        let mut gap = gap;
        self.apply_headway_perception_error(veh, speed, &mut gap);
        if gap < 0.01 {
            return 0.;
        }
        let mut result = self.v(veh, gap, speed, 0., veh.get_lane().get_vehicle_max_speed(traffic_object(veh)), false);
        //std::cout << SIMTIME << " stopSpeed speed=" << speed << " gap=" << gap << " decel=" << decel << " result=" << result << "\n";
        if gap > 0. && speed < NUMERICAL_EPS && result < NUMERICAL_EPS {
            // ensure that stops can be reached:
            result = self.maximum_safe_stop_speed(gap, decel, speed, false, veh.get_action_step_length_secs(), true);
        }
        // avoid overshooting the stop location
        if gap >= 0. {
            result = min2(result, dist2speed(gap));
        }
        result
    }

    /// C++: `double MSCFModel_IDM::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_IDM::interactionGap(const MSVehicle* const veh, double vL) const {
    //     // Resolve the IDM equation to gap. Assume predecessor has
    //     // speed != 0 and that vsafe will be the current speed plus acceleration,
    //     // i.e that with this gap there will be no interaction.
    //     const double acc = myAccel * (1. - pow(veh->getSpeed() / veh->getLane()->getVehicleMaxSpeed(veh), myDelta));
    //     const double vNext = veh->getSpeed() + acc;
    //     const double gap = (vNext - vL) * (veh->getSpeed() + vL) / (2 * myDecel) + vL;
    //
    //     // Don't allow timeHeadWay < deltaT situations.
    //     return MAX2(gap, SPEED2DIST(vNext));
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        // Resolve the IDM equation to gap. Assume predecessor has
        // speed != 0 and that vsafe will be the current speed plus acceleration,
        // i.e that with this gap there will be no interaction.
        let acc = self.myAccel * (1. - (arg0.get_speed() / arg0.get_lane().get_vehicle_max_speed(traffic_object(arg0))).powf(self.myDelta));
        let vNext = arg0.get_speed() + acc;
        let gap = (vNext - vL) * (arg0.get_speed() + vL) / (2. * self.myDecel) + vL;

        // Don't allow timeHeadWay < deltaT situations.
        max2(gap, speed2dist(vNext))
    }

    /// C++: `double MSCFModel_IDM::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling) This method is used during the insertion stage. Whereas the method followSpeed returns the desired speed which may be lower than the safe speed, this method only considers safety constraints
    // double
    // MSCFModel_IDM::insertionFollowSpeed(const MSVehicle* const v, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred) const {
    //     // see definition of s in _v()
    //     double s = MAX2(0., speed * myHeadwayTime + speed * (speed - predSpeed) / myTwoSqrtAccelDecel);
    //     if (gap2pred >= s) {
    //         // followSpeed always stays below speed because s*s / (gap2pred * gap2pred) > 0. This would prevent insertion with maximum speed at all distances
    //         return speed;
    //     } else {
    //         // we cannot call follow speed directly because it assumes that 'speed'
    //         // is the current speed rather than the desired insertion speed.
    //         // If the safe speed is much lower than the desired speed, the
    //         // followSpeed function would still return a new speed that involves
    //         // reasonable braking rather than the actual safe speed (and cause
    //         // emergency braking in a subsequent step)
    //         const double speed2 = followSpeed(v, speed, gap2pred, predSpeed, predMaxDecel, pred, CalcReason::FUTURE);
    //         const double speed3 = followSpeed(v, speed2, gap2pred, predSpeed, predMaxDecel, pred, CalcReason::FUTURE);
    //         if (speed2 - speed3 < ACCEL2SPEED(1)) {
    //             return speed2;
    //         } else {
    // #ifdef DEBUG_INSERTION_SPEED
    //             std::cout << SIMTIME << " veh=" << v->getID() << " speed=" << speed << " gap2pred=" << gap2pred << " predSpeed=" << predSpeed << " predMaxDecel=" << predMaxDecel << " pred=" << Named::getIDSecure(pred) << " s=" << s << " speed2=" << speed2 << " speed3=" << speed3 << "\n";
    // #endif
    //             return insertionFollowSpeed(v, speed2, gap2pred, predSpeed, predMaxDecel, pred);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insertion_follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
        // see definition of s in _v()
        let s = max2(0., speed * self.myHeadwayTime + speed * (speed - predSpeed) / self.myTwoSqrtAccelDecel);
        if gap2pred >= s {
            // followSpeed always stays below speed because s*s / (gap2pred * gap2pred) > 0. This would prevent insertion with maximum speed at all distances
            speed
        } else {
            // we cannot call follow speed directly because it assumes that 'speed'
            // is the current speed rather than the desired insertion speed.
            // If the safe speed is much lower than the desired speed, the
            // followSpeed function would still return a new speed that involves
            // reasonable braking rather than the actual safe speed (and cause
            // emergency braking in a subsequent step)
            let speed2 = self.follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, MSCFModel_CalcReason::FUTURE);
            let speed3 = self.follow_speed(veh, speed2, gap2pred, predSpeed, predMaxDecel, pred, MSCFModel_CalcReason::FUTURE);
            if speed2 - speed3 < accel2speed(1.) {
                speed2
            } else {
                self.insertion_follow_speed(veh, speed2, gap2pred, predSpeed, predMaxDecel, pred)
            }
        }
    }

    /// C++: `double MSCFModel_IDM::insertionStopSpeed(const MSVehicle *const, double, double) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching an obstacle at insertion without constraints due to acceleration capabilities and previous speeds.
    // double
    // MSCFModel_IDM::insertionStopSpeed(const MSVehicle* const veh, double speed, double gap) const {
    //     // we want to insert the vehicle in an equilibrium state
    //     double result = MSCFModel::insertionStopSpeed(veh, speed, gap);
    //     int i = 0;
    //     while (result - speed < -ACCEL2SPEED(myDecel) && ++i < 10) {
    //         speed = result;
    //         result = MSCFModel::insertionStopSpeed(veh, speed, gap);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn insertion_stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64) -> f64 {
        // we want to insert the vehicle in an equilibrium state
        let mut speed = speed;
        let mut result = self.base.insertion_stop_speed_base(veh, speed, gap);
        let mut i = 0;
        while result - speed < -accel2speed(self.myDecel) && { i += 1; i < 10 } {
            speed = result;
            result = self.base.insertion_stop_speed_base(veh, speed, gap);
        }
        result
    }

    /// C++: `double MSCFModel_IDM::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    /// Returns the minimum gap to reserve if the leader is braking at maximum (>=0)
    // double
    // MSCFModel_IDM::getSecureGap(const MSVehicle* const /*veh*/, const MSVehicle* const /*pred*/, const double speed, const double leaderSpeed, const double /*leaderMaxDecel*/) const {
    //     const double delta_v = speed - leaderSpeed;
    //     return MAX2(0.0, speed * myHeadwayTime + speed * delta_v / myTwoSqrtAccelDecel);
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        let delta_v = speed - leaderSpeed;
        max2(0.0, speed * self.myHeadwayTime + speed * delta_v / self.myTwoSqrtAccelDecel)
    }

    /// C++: `int MSCFModel_IDM::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return myAdaptationFactor == 1. ? SUMO_TAG_CF_IDM : SUMO_TAG_CF_IDMM;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        if self.myAdaptationFactor == 1. { SUMO_TAG_CF_IDM as i32 } else { SUMO_TAG_CF_IDMM as i32 }
    }

    /// C++: `double MSCFModel_IDM::minNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    // double
    // MSCFModel_IDM::minNextSpeed(double speed, const MSVehicle* const /*veh*/) const {
    //     // permit exceeding myDecel when approaching stops
    //     const double decel = MAX2(myDecel, MIN2(myEmergencyDecel, 1.5));
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MAX2(speed - ACCEL2SPEED(decel), 0.);
    //     } else {
    //         // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
    //         return speed - ACCEL2SPEED(decel);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        // permit exceeding myDecel when approaching stops
        let decel = max2(self.myDecel, min2(self.myEmergencyDecel, 1.5));
        if MSGlobals::semi_implicit_euler_update() {
            max2(speed - accel2speed(decel), 0.)
        } else {
            // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
            speed - accel2speed(decel)
        }
    }

    /// C++: `MSCFModel * MSCFModel_IDM::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_IDM::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_IDM(vtype, myIDMM);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_IDM::new(vtype, self.myIDMM))) as *mut MSCFModel)
    }

    /// C++: `VehicleVariables * MSCFModel_IDM::createVehicleVariables() const` [const, virtual]
    // VehicleVariables* createVehicleVariables() const {
    //         if (myAdaptationFactor != 1.) {
    //             return new VehicleVariables();
    //         }
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        if self.myAdaptationFactor != 1. {
            // shortcut: allocated with Box::into_raw like C++ `new`; the owning vehicle must free it once ported
            Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_IDM_VehicleVariables::new())) as *mut MSCFModel_VehicleVariables)
        } else {
            Ptr::default()
        }
    }

    /// C++: `double MSCFModel_IDM::_v(const MSVehicle *const, const double, const double, const double, const double, const bool) const` [const, private]
    // double
    // MSCFModel_IDM::_v(const MSVehicle* const veh, const double gap2pred, const double egoSpeed,
    //                   const double predSpeed, const double desSpeed, const bool respectMinGap) const {
    // // this is more or less based on http://www.vwi.tu-dresden.de/~treiber/MicroApplet/IDM.html
    // // and http://arxiv.org/abs/cond-mat/0304337
    // // we assume however constant speed for the leader
    //     double headwayTime = myHeadwayTime;
    //     if (myAdaptationFactor != 1.) {
    //         const VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //         headwayTime *= myAdaptationFactor + vars->levelOfService * (1. - myAdaptationFactor);
    //     }
    //     double newSpeed = egoSpeed;
    //     double gap = gap2pred;
    //     if (respectMinGap) {
    //         // gap2pred comes with minGap already subtracted so we need to add it here again
    //         gap += myType->getMinGap();
    //     }
    // #ifdef DEBUG_V
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << veh->getID() << " gap2pred=" << gap2pred << " egoSpeed=" << egoSpeed << " predSpeed=" << predSpeed << " desSpeed=" << desSpeed << " rMG=" << respectMinGap << " hw=" << headwayTime << "\n";
    //     }
    // #endif
    //     for (int i = 0; i < myIterations; i++) {
    //         const double delta_v = newSpeed - predSpeed;
    //         double s = MAX2(0., newSpeed * headwayTime + newSpeed * delta_v / myTwoSqrtAccelDecel);
    //         if (respectMinGap) {
    //             s += myType->getMinGap();
    //         }
    //         gap = MAX2(NUMERICAL_EPS, gap); // avoid singularity
    //         const double acc = myAccel * (1. - pow(newSpeed / MAX2(NUMERICAL_EPS, desSpeed), myDelta) - (s * s) / (gap * gap));
    // #ifdef DEBUG_V
    //         if (DEBUG_COND) {
    //             std::cout << "   i=" << i << " gap=" << gap << " t=" << myHeadwayTime << " t2=" << headwayTime << " s=" << s << " pow=" << pow(newSpeed / desSpeed, myDelta) << " gapDecel=" << (s * s) / (gap * gap) << " a=" << acc;
    //         }
    // #endif
    //         newSpeed = MAX2(0.0, newSpeed + ACCEL2SPEED(acc) / myIterations);
    // #ifdef DEBUG_V
    //         if (DEBUG_COND) {
    //             std::cout << " v2=" << newSpeed << " gLC=" << MSGlobals::gComputeLC << "\n";
    //         }
    // #endif
    //         //TODO use more realistic position update which takes accelerated motion into account
    //         gap -= MAX2(0., SPEED2DIST(newSpeed - predSpeed) / myIterations);
    //     }
    //     return MAX2(0., newSpeed);
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, gap2pred: f64, mySpeed: f64, predSpeed: f64, desSpeed: f64, respectMinGap: bool) -> f64 {
        // this is more or less based on http://www.vwi.tu-dresden.de/~treiber/MicroApplet/IDM.html
        // and http://arxiv.org/abs/cond-mat/0304337
        // we assume however constant speed for the leader
        let mut headwayTime = self.myHeadwayTime;
        if self.myAdaptationFactor != 1. {
            let vars = veh.get_car_follow_variables().cast::<MSCFModel_IDM_VehicleVariables>();
            headwayTime *= self.myAdaptationFactor + vars.levelOfService * (1. - self.myAdaptationFactor);
        }
        let mut newSpeed = mySpeed;
        let mut gap = gap2pred;
        if respectMinGap {
            // gap2pred comes with minGap already subtracted so we need to add it here again
            gap += self.myType.get_min_gap();
        }
        for _i in 0..self.myIterations {
            let delta_v = newSpeed - predSpeed;
            let mut s = max2(0., newSpeed * headwayTime + newSpeed * delta_v / self.myTwoSqrtAccelDecel);
            if respectMinGap {
                s += self.myType.get_min_gap();
            }
            gap = max2(NUMERICAL_EPS, gap); // avoid singularity
            let acc = self.myAccel * (1. - (newSpeed / max2(NUMERICAL_EPS, desSpeed)).powf(self.myDelta) - (s * s) / (gap * gap));
            newSpeed = max2(0.0, newSpeed + accel2speed(acc) / self.myIterations as f64);
            //TODO use more realistic position update which takes accelerated motion into account
            gap -= max2(0., speed2dist(newSpeed - predSpeed) / self.myIterations as f64);
        }
        max2(0., newSpeed)
    }

    /// C++: `MSCFModel_IDM & MSCFModel_IDM::operator=(const MSCFModel_IDM &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSCFModel_IDM) -> &mut MSCFModel_IDM {
        // shortcut: C++ operator= is private and never defined (C++ cannot call it); panic mirrors that
        panic!("MSCFModel_IDM::operator= is private and not defined in C++")
    }

}

/// C++ class `MSCFModel_IDM::VehicleVariables` (microsim/cfmodels/MSCFModel_IDM.h)
#[repr(C)]
pub struct MSCFModel_IDM_VehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    /// state variable for remembering speed deviation history (lambda)
    pub levelOfService: f64,
}

impl std::ops::Deref for MSCFModel_IDM_VehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_IDM_VehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_IDM_VehicleVariables {
    /// C++: `void VehicleVariables::VehicleVariables()`
    // VehicleVariables() : levelOfService(1.) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSCFModel_IDM_VehicleVariables {
            base: MSCFModel_VehicleVariables {},
            levelOfService: 1.,
        }
    }

    /// C++: `void VehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_IDM::VehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "IDMM");
    //     std::ostringstream internals;
    //     internals << levelOfService;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "IDMM");
        let internals = f64_to_string_prec(self.levelOfService, 6);
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
    }

    /// C++: `void VehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_IDM::VehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "IDMM") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for IDMM", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> levelOfService;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "IDMM" {
            return Err(utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for IDMM", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        if let Some(v) = bis.next().and_then(|s| s.parse::<f64>().ok()) {
            self.levelOfService = v;
        }
        Ok(())
    }

}

