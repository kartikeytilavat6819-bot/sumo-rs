//! Port of microsim/cfmodels/MSCFModel_W99.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;

fn w99_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_W99>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage) }
fn w99_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_W99>(m).stop_speed(veh, speed, gap, decel, usage) }
fn w99_interaction_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, vL: f64) -> f64 { cf_cast::<MSCFModel_W99>(m).interaction_gap(veh, vL) }
fn w99_get_model_id(m: &MSCFModel) -> i32 { cf_cast::<MSCFModel_W99>(m).get_model_id() }
fn w99_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> { cf_cast::<MSCFModel_W99>(m).duplicate(vtype) }

/// Virtual table of MSCFModel_W99: its overrides on top of the MSCFModel default table
pub const W99_VT: MSCFModelVt = MSCFModelVt {
    follow_speed: w99_follow_speed,
    stop_speed_2: w99_stop_speed_2,
    interaction_gap: w99_interaction_gap,
    get_model_id: w99_get_model_id,
    duplicate: w99_duplicate,
    ..DEFAULT_MSCF_MODEL_VT
};
use utils::common::sumo_vehicle_class::SVC_BICYCLE;

/// C++ `#define INTERACTION_GAP 150` (MSCFModel_W99.cpp)
const INTERACTION_GAP: f64 = 150.;

/// The W99 Model car-following model
/// C++ class `MSCFModel_W99` (microsim/cfmodels/MSCFModel_W99.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_W99 {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    pub myCC1: f64,
    pub myCC2: f64,
    pub myCC3: f64,
    pub myCC4: f64,
    pub myCC5: f64,
    pub myCC6: f64,
    pub myCC7: f64,
    pub myCC8: f64,
    pub myCC9: f64,
}

impl std::ops::Deref for MSCFModel_W99 {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_W99 {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_W99 {
    /// C++: `void MSCFModel_W99::MSCFModel_W99(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_W99::MSCFModel_W99(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     myCC1(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC1,  1.30)),
    //     myCC2(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC2,  8.00)),
    //     myCC3(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC3, -12.00)),
    //     myCC4(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC4, -0.25)),
    //     myCC5(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC5,  0.35)),
    //     myCC6(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC6,  6.00)),
    //     myCC7(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC7,  0.25)),
    //     myCC8(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC8,  2.00)),
    //     myCC9(vtype->getParameter().getCFParam(SUMO_ATTR_CF_W99_CC9,  1.50)) {
    //     // translate some values to make them show up correctly in the gui
    //     myHeadwayTime = myCC1;
    //     myAccel = myCC8;
    //     // W99 does not drive very precise and may violate minGap on occasion
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let p = vtype.get_parameter();
        let myCC1 = p.get_cf_param(SUMO_ATTR_CF_W99_CC1, 1.30);
        let myCC2 = p.get_cf_param(SUMO_ATTR_CF_W99_CC2, 8.00);
        let myCC3 = p.get_cf_param(SUMO_ATTR_CF_W99_CC3, -12.00);
        let myCC4 = p.get_cf_param(SUMO_ATTR_CF_W99_CC4, -0.25);
        let myCC5 = p.get_cf_param(SUMO_ATTR_CF_W99_CC5, 0.35);
        let myCC6 = p.get_cf_param(SUMO_ATTR_CF_W99_CC6, 6.00);
        let myCC7 = p.get_cf_param(SUMO_ATTR_CF_W99_CC7, 0.25);
        let myCC8 = p.get_cf_param(SUMO_ATTR_CF_W99_CC8, 2.00);
        let myCC9 = p.get_cf_param(SUMO_ATTR_CF_W99_CC9, 1.50);
        let collisionMinGapFactor = p.get_cf_param(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
        let mut base = MSCFModel::new(vtype);
        base.vt = &W99_VT;
        // translate some values to make them show up correctly in the gui
        base.myHeadwayTime = myCC1;
        base.myAccel = myCC8;
        // W99 does not drive very precise and may violate minGap on occasion
        base.myCollisionMinGapFactor = collisionMinGapFactor;
        MSCFModel_W99 {
            base,
            myCC1,
            myCC2,
            myCC3,
            myCC4,
            myCC5,
            myCC6,
            myCC7,
            myCC8,
            myCC9,
        }
    }

    /// C++: `void MSCFModel_W99::~MSCFModel_W99() noexcept` [virtual]
    /// Destructor
    // MSCFModel_W99::~MSCFModel_W99() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_W99::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_W99::followSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double /*predMaxDecel*/, const MSVehicle* const pred, const CalcReason /*usage*/) const {
    //     const double dx = gap2pred + myType->getMinGap();
    //     const double dv = predSpeed - speed;
    //
    //     const double cc0 = myType->getMinGap();
    //
    //     const double leaderAccel = pred != nullptr ? pred->getAcceleration() : 0;
    //     const double oldAccel = veh->getAcceleration();
    //
    //     const double rndVal = speed > 0 ? RandHelper::rand(veh->getRNG()) - 0.5 : 0.5;
    //     double sdxc, sdxo, sdxv;
    //     computeThresholds(speed, predSpeed, leaderAccel, rndVal, sdxc, sdxo, sdxv);
    //
    //     const double sdv = myCC6 * dx * dx / 10000;
    //     const double sdvc = speed > 0 ? myCC4 - sdv : 0; //minimum closing dv
    //     const double sdvo = predSpeed > myCC5 ? sdv + myCC5 : sdv; //minimum opening dv
    //
    //     double accel = 0; // resulting acceleration
    //     int status = 0;
    //
    //     if (dv < sdvo && dx <= sdxc) {
    //         // 'Decelerate - Increase Distance';
    //         accel = 0;
    //         // code in addtion to w99-demo to fix collision (#10472)
    //         if (dx - SPEED2DIST(speed) < myType->getMinGap() * myCollisionMinGapFactor) {
    //             // prevent crashin in the next step
    //             accel = -SPEED2ACCEL(speed);
    //             status = 9;
    //         }
    //         if (predSpeed > 0) {
    //             if (dv < 0) {
    //                 if (dx > cc0) {
    //                     accel = MIN2(leaderAccel + dv * dv / (cc0 - dx), 0.0);
    //                     status = 0;
    //                 } else {
    //                     accel = MIN2(leaderAccel + 0.5 * (dv - sdvo), 0.0);
    //                     status = 1;
    //                 }
    //             }
    //             if (accel > -myCC7) {
    //                 accel = -myCC7;
    //                 status = 2;
    //             } else {
    //                 accel = MAX2(accel, -10 + 0.5 * sqrt(speed));
    //                 status = 3;
    //             }
    //         }
    //
    //     } else if (dv < sdvc && dx < sdxv) {
    //         // 'Decelerate - Decrease Distance';
    //         accel = 0.5 * dv * dv / (sdxc - dx - 0.1);
    //         status = 4;
    //         // deceleration is capped by myEmergencyDecel in MSCFModel::finalizeSpeed
    //     } else if (dv < sdvo && dx < sdxo) {
    //         // 'Accelerate/Decelerate - Keep Distance';
    //         if (oldAccel <= 0) {
    //             accel = MIN2(oldAccel, -myCC7);
    //             status = 5;
    //         } else {
    //             accel = MAX2(oldAccel, myCC7);
    //             status = 6;
    //             // acceleration is capped at desired speed in MSCFModel::finalizeSpeed
    //         }
    //     } else {
    //         // 'Accelerate/Relax - Increase/Keep Speed';
    //         if (dx > sdxc) {
    //             //if (follower.status === 'w') {
    //             //    accel = cc7;
    //             //} else
    //             {
    //                 const double accelMax = myCC8 + myCC9 * MIN2(speed, 80 / 3.6) + RandHelper::rand(veh->getRNG());
    //                 if (dx < sdxo) {
    //                     accel = MIN2(dv * dv / (sdxo - dx), accelMax);
    //                     status = 7;
    //                 } else {
    //                     accel = accelMax;
    //                     status = 8;
    //                 }
    //             }
    //             // acceleration is capped at desired speed in MSCFModel::finalizeSpeed
    //         }
    //     }
    //     double vNew = speed + ACCEL2SPEED(accel);
    // #ifdef DEBUG_FOLLOW_SPEED
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " W99::fS veh=" << veh->getID() << " pred=" << Named::getIDSecure(pred)
    //                   << " v=" << speed << " pV=" << predSpeed << " g=" << gap2pred
    //                   << " dv=" << dv << " dx=" << dx
    //                   << " sdxc=" << sdxc << " sdxo=" << sdxo << " sdxv=" << sdxv << " sdv=" << sdv << " sdvo=" << sdvo << " sdvc=" << sdvc
    //                   << " st=" << status
    //                   << " a=" << accel << " V=" << vNew << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(status);
    // #endif
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         vNew = MAX2(0.0, vNew);
    //     }
    //     return vNew;
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let dx = gap2pred + self.myType.get_min_gap();
        let dv = predSpeed - speed;

        let cc0 = self.myType.get_min_gap();

        let leaderAccel = if !pred.is_null() { pred.get_acceleration() } else { 0. };
        let oldAccel = veh.get_acceleration();

        let rndVal = if speed > 0. { veh.get_rng().m().rand() - 0.5 } else { 0.5 };
        let mut sdxc = 0.;
        let mut sdxo = 0.;
        let mut sdxv = 0.;
        self.compute_thresholds(speed, predSpeed, leaderAccel, rndVal, &mut sdxc, &mut sdxo, &mut sdxv);

        let sdv = self.myCC6 * dx * dx / 10000.;
        let sdvc = if speed > 0. { self.myCC4 - sdv } else { 0. }; //minimum closing dv
        let sdvo = if predSpeed > self.myCC5 { sdv + self.myCC5 } else { sdv }; //minimum opening dv

        let mut accel = 0.; // resulting acceleration
        let mut status = 0;

        if dv < sdvo && dx <= sdxc {
            // 'Decelerate - Increase Distance';
            accel = 0.;
            // code in addtion to w99-demo to fix collision (#10472)
            if dx - speed2dist(speed) < self.myType.get_min_gap() * self.myCollisionMinGapFactor {
                // prevent crashin in the next step
                accel = -speed2accel(speed);
                status = 9;
            }
            if predSpeed > 0. {
                if dv < 0. {
                    if dx > cc0 {
                        accel = min2(leaderAccel + dv * dv / (cc0 - dx), 0.0);
                        status = 0;
                    } else {
                        accel = min2(leaderAccel + 0.5 * (dv - sdvo), 0.0);
                        status = 1;
                    }
                }
                if accel > -self.myCC7 {
                    accel = -self.myCC7;
                    status = 2;
                } else {
                    accel = max2(accel, -10. + 0.5 * speed.sqrt());
                    status = 3;
                }
            }
        } else if dv < sdvc && dx < sdxv {
            // 'Decelerate - Decrease Distance';
            accel = 0.5 * dv * dv / (sdxc - dx - 0.1);
            status = 4;
            // deceleration is capped by myEmergencyDecel in MSCFModel::finalizeSpeed
        } else if dv < sdvo && dx < sdxo {
            // 'Accelerate/Decelerate - Keep Distance';
            if oldAccel <= 0. {
                accel = min2(oldAccel, -self.myCC7);
                status = 5;
            } else {
                accel = max2(oldAccel, self.myCC7);
                status = 6;
                // acceleration is capped at desired speed in MSCFModel::finalizeSpeed
            }
        } else {
            // 'Accelerate/Relax - Increase/Keep Speed';
            if dx > sdxc {
                let accelMax = self.myCC8 + self.myCC9 * min2(speed, 80. / 3.6) + veh.get_rng().m().rand();
                if dx < sdxo {
                    accel = min2(dv * dv / (sdxo - dx), accelMax);
                    status = 7;
                } else {
                    accel = accelMax;
                    status = 8;
                }
                // acceleration is capped at desired speed in MSCFModel::finalizeSpeed
            }
        }
        let mut vNew = speed + accel2speed(accel);
        let _ = status; // status is only used for the debug output in C++
        if g_semi_implicit_euler_update() {
            vNew = max2(0.0, vNew);
        }
        vNew
    }

    /// C++: `double MSCFModel_W99::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_W99::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     // see reasoning in MSCFModel_Wiedemann::stopSpeed
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, veh->getActionStepLengthSecs()), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        // see reasoning in MSCFModel_Wiedemann::stopSpeed
        min2(self.maximum_safe_stop_speed(gap, decel, speed, false, veh.get_action_step_length_secs(), true), self.max_next_speed(speed, veh))
    }

    /// C++: `double MSCFModel_W99::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_W99::interactionGap(const MSVehicle* const, double vL) const {
    //     UNUSED_PARAMETER(vL);
    //     return INTERACTION_GAP;
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        // W99 uses a fixed interaction gap
        INTERACTION_GAP
    }

    /// C++: `int MSCFModel_W99::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_W99;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_W99 as i32
    }

    /// C++: `MSCFModel * MSCFModel_W99::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_W99::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_W99(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_W99::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `void MSCFModel_W99::computeThresholds(double, double, double, double, double &, double &, double &) const` [const, private]
    /// @}
    // void
    // MSCFModel_W99::computeThresholds(double speed, double predSpeed, double leaderAccel, double rndVal,
    //                                  double& sdxc, double& sdxo, double& sdxv) const {
    //
    //     const double dv = predSpeed - speed;
    //     sdxc = myType->getMinGap(); // cc0
    //     if (predSpeed > 0) {
    //         const double v_slower = (dv >= 0 || leaderAccel < 1) ? speed : predSpeed + dv * rndVal;
    //         sdxc += myCC1 * MAX2(0.0, v_slower);
    //     }
    //     sdxo = sdxc + myCC2; //maximum following distance (upper limit of car-following process)
    //     sdxv = sdxo + myCC3 * (dv - myCC4);
    // }
    #[allow(unused_variables)]
    pub fn compute_thresholds(&self, speed: f64, predSpeed: f64, leaderAccel: f64, rndVal: f64, sdxc: &mut f64, sdxo: &mut f64, sdxv: &mut f64) {
        let dv = predSpeed - speed;
        *sdxc = self.myType.get_min_gap(); // cc0
        if predSpeed > 0. {
            let v_slower = if dv >= 0. || leaderAccel < 1. { speed } else { predSpeed + dv * rndVal };
            *sdxc += self.myCC1 * max2(0.0, v_slower);
        }
        *sdxo = *sdxc + self.myCC2; //maximum following distance (upper limit of car-following process)
        *sdxv = *sdxo + self.myCC3 * (dv - self.myCC4);
    }

    /// C++: `MSCFModel_W99 & MSCFModel_W99::operator=(const MSCFModel_W99 &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSCFModel_W99) -> &mut MSCFModel_W99 {
        // C++ implicit copy assignment: copy every member
        *self = s.clone();
        self
    }

}

