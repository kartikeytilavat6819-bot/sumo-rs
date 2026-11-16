//! Port of microsim/cfmodels/MSCFModel_Wiedemann.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;

fn wiedemann_finalize_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, vPos: f64) -> f64 { cf_cast::<MSCFModel_Wiedemann>(m).finalize_speed(veh, vPos) }
fn wiedemann_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_Wiedemann>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage) }
fn wiedemann_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_Wiedemann>(m).stop_speed(veh, speed, gap, decel, usage) }
fn wiedemann_interaction_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, vL: f64) -> f64 { cf_cast::<MSCFModel_Wiedemann>(m).interaction_gap(veh, vL) }
fn wiedemann_get_secure_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 { cf_cast::<MSCFModel_Wiedemann>(m).get_secure_gap(veh, pred, speed, leaderSpeed, leaderMaxDecel) }
fn wiedemann_get_model_id(m: &MSCFModel) -> i32 { cf_cast::<MSCFModel_Wiedemann>(m).get_model_id() }
fn wiedemann_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> { cf_cast::<MSCFModel_Wiedemann>(m).duplicate(vtype) }
fn wiedemann_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> { cf_cast::<MSCFModel_Wiedemann>(m).create_vehicle_variables() }

/// Virtual table of MSCFModel_Wiedemann: its overrides on top of the MSCFModel default table
pub const WIEDEMANN_VT: MSCFModelVt = MSCFModelVt {
    finalize_speed: wiedemann_finalize_speed,
    follow_speed: wiedemann_follow_speed,
    stop_speed_2: wiedemann_stop_speed_2,
    interaction_gap: wiedemann_interaction_gap,
    get_secure_gap: wiedemann_get_secure_gap,
    get_model_id: wiedemann_get_model_id,
    duplicate: wiedemann_duplicate,
    create_vehicle_variables: wiedemann_create_vehicle_variables,
    ..DEFAULT_MSCF_MODEL_VT
};
use utils::common::sumo_vehicle_class::SVC_BICYCLE;

/// C++ `MSCFModel_Wiedemann::D_MAX` static (MSCFModel_Wiedemann.cpp)
pub const D_MAX: f64 = 150.;
const B_MIN_MULT: f64 = 0.;
const B_MIN_ADD: f64 = -1.;
const PRED_DECEL_MULT: f64 = 0.5;
const PRED_DECEL_MULT_EMERGENCY: f64 = 0.55;

/// The Wiedemann Model car-following model
/// C++ class `MSCFModel_Wiedemann` (microsim/cfmodels/MSCFModel_Wiedemann.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_Wiedemann {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// The driver's security parameter // also 'ZF1'
    pub mySecurity: f64,
    /// The driver's estimation parameter // also 'ZF2'
    pub myEstimation: f64,
    /// the minimum front-bumper to front-bumper distance when standing
    pub myAX: f64,
    /// perception threshold modifier
    pub myCX: f64,
    /// The vehicle's minimum acceleration [m/s^2] // also b_null
    pub myMinAccel: f64,
    /// The maximum deceleration when approaching
    pub myMaxApproachingDecel: f64,
}

impl std::ops::Deref for MSCFModel_Wiedemann {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Wiedemann {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

// C++ static members of MSCFModel_Wiedemann (port as module-level statics, see AGENTS.md):
//   static const double MSCFModel_Wiedemann::D_MAX;

impl MSCFModel_Wiedemann {
    /// C++: `void MSCFModel_Wiedemann::MSCFModel_Wiedemann(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_Wiedemann::MSCFModel_Wiedemann(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     mySecurity(vtype->getParameter().getCFParam(SUMO_ATTR_CF_WIEDEMANN_SECURITY, 0.5)),
    //     myEstimation(vtype->getParameter().getCFParam(SUMO_ATTR_CF_WIEDEMANN_ESTIMATION, 0.5)),
    //     myAX(vtype->getLength() + 1. + 2. * mySecurity),
    //     myCX(25. *(1. + mySecurity + myEstimation)),
    //     myMinAccel(0.2 * myAccel),
    //     myMaxApproachingDecel((myDecel + myEmergencyDecel) / 2) {
    //     // Wiedemann does not drive very precise and may violate minGap on occasion
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let p = vtype.get_parameter();
        let mySecurity = p.get_cf_param(SUMO_ATTR_CF_WIEDEMANN_SECURITY, 0.5);
        let myEstimation = p.get_cf_param(SUMO_ATTR_CF_WIEDEMANN_ESTIMATION, 0.5);
        let collisionMinGapFactor = p.get_cf_param(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
        let vtypeLength = vtype.get_length();
        // Wiedemann does not drive very precise and may violate minGap on occasion
        let mut base = MSCFModel::new(vtype);
        base.vt = &WIEDEMANN_VT;
        base.myCollisionMinGapFactor = collisionMinGapFactor;
        let myAX = vtypeLength + 1. + 2. * mySecurity;
        let myCX = 25. * (1. + mySecurity + myEstimation);
        let myMinAccel = 0.2 * base.myAccel;
        let myMaxApproachingDecel = (base.myDecel + base.myEmergencyDecel) / 2.;
        MSCFModel_Wiedemann {
            base,
            mySecurity,
            myEstimation,
            myAX,
            myCX,
            myMinAccel,
            myMaxApproachingDecel,
        }
    }

    /// C++: `void MSCFModel_Wiedemann::~MSCFModel_Wiedemann() noexcept` [virtual]
    /// Destructor
    // MSCFModel_Wiedemann::~MSCFModel_Wiedemann() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_Wiedemann::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences
    // double
    // MSCFModel_Wiedemann::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     const double vNext = MSCFModel::finalizeSpeed(veh, vPos);
    //     VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //     vars->accelSign = vNext > veh->getSpeed() ? 1. : -1.;
    //     return vNext;
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        let vNext = self.base.finalize_speed_base(veh, vPos);
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_Wiedemann_VehicleVariables);
        vars.m().accelSign = if vNext > veh.get_speed() { 1. } else { -1. };
        vNext
    }

    /// C++: `double MSCFModel_Wiedemann::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_Wiedemann::followSpeed(const MSVehicle* const veh, double /* speed */, double gap2pred, double predSpeed, double /*predMaxDecel*/, const MSVehicle* const pred, const CalcReason /*usage*/) const {
    //     return _v(veh, predSpeed, gap2pred, pred != nullptr ? pred->getAcceleration() : 0);
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        self.v(veh, predSpeed, gap2pred, if !pred.is_null() { pred.get_acceleration() } else { 0. })
    }

    /// C++: `double MSCFModel_Wiedemann::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_Wiedemann::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     /* Wiedemann does not handle approaching junctions or stops very well:
    //      * regime approaching() fails when dv = 0 (i.e. a vehicle inserted with speed 0 does not accelerate to reach a stop)
    //      * for dv ~ 0 the standard decision tree will switch to following() which
    //      * does a lousy job of closing in on a stop / junction
    //      * hence we borrow from Krauss here
    //      */
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, veh->getActionStepLengthSecs()), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        // Wiedemann does not handle approaching junctions or stops very well, so it borrows the Krauss stop logic
        min2(self.maximum_safe_stop_speed(gap, decel, speed, false, veh.get_action_step_length_secs(), true), self.max_next_speed(speed, veh))
    }

    /// C++: `double MSCFModel_Wiedemann::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_Wiedemann::interactionGap(const MSVehicle* const, double vL) const {
    //     UNUSED_PARAMETER(vL);
    //     return D_MAX;
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        D_MAX
    }

    /// C++: `double MSCFModel_Wiedemann::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    /// Returns the minimum gap to reserve if the leader is braking at maximum (>=0)
    // double
    // MSCFModel_Wiedemann::getSecureGap(const MSVehicle* const veh, const MSVehicle* const pred, const double speed, const double leaderSpeed, const double leaderMaxDecel) const {
    //     const double bx = (1 + 7 * mySecurity) * sqrt(speed);
    //     const double abx = myAX + bx - myType->getLength(); // abx is the brutto gap
    //     return MAX2(abx, MSCFModel::getSecureGap(veh, pred, speed, leaderSpeed, leaderMaxDecel));
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        let bx = (1. + 7. * self.mySecurity) * speed.sqrt();
        let abx = self.myAX + bx - self.myType.get_length(); // abx is the brutto gap
        max2(abx, self.base.get_secure_gap_base(veh, pred, speed, leaderSpeed, leaderMaxDecel))
    }

    /// C++: `int MSCFModel_Wiedemann::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_WIEDEMANN;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_WIEDEMANN as i32
    }

    /// C++: `MSCFModel * MSCFModel_Wiedemann::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_Wiedemann::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Wiedemann(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Wiedemann::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `VehicleVariables * MSCFModel_Wiedemann::createVehicleVariables() const` [const, virtual]
    // VehicleVariables* createVehicleVariables() const {
    //         return new VehicleVariables();
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        // shortcut: allocated with Box::into_raw like C++ `new`; the owning vehicle must free it once ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Wiedemann_VehicleVariables::new())) as *mut MSCFModel_VehicleVariables)
    }

    /// C++: `double MSCFModel_Wiedemann::_v(const MSVehicle *, double, double, double) const` [const, private]
    // double
    // MSCFModel_Wiedemann::_v(const MSVehicle* veh, double predSpeed, double gap, double predAccel) const {
    //     const VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //     const double dx = gap + myType->getLength(); // wiedemann uses brutto gap
    //     const double v = veh->getSpeed();
    //     const double vpref = veh->getMaxSpeed();
    //     const double dv = v - predSpeed;
    //     // desired minimum following distance at low speed difference
    //     const double bx = (1 + 7 * mySecurity) * sqrt(v); // Harding propose a factor of  *.8 here
    //     const double abx = myAX + bx; // Harding propose a factor of  *.8 here
    //     const double ex = 2 - myEstimation; // + RandHelper::randNorm(0.5, 0.15)
    //     const double sdx = myAX + ex * bx; /// the distance at which we drift out of following
    //     const double sdv_root = (dx - myAX) / myCX;
    //     const double sdv = sdv_root * sdv_root;
    //     const double cldv = sdv * ex * ex;
    //     const double opdv = cldv * (-1 - 2 * RandHelper::randNorm(0.5, 0.15, veh->getRNG()));
    //     // D_MAX is too low to brake safely when driving at speeds above 36m/s
    //     const double dmax = MAX2(D_MAX, brakeGap(v, myDecel, 0));
    //     // select the regime, get new acceleration, compute new speed based
    //     double accel;
    //     int branch = 0;
    //     if (dx <= abx) {
    //         accel = emergency(dv, dx, predAccel, v, gap, abx, bx);
    //         branch = 1;
    //     } else if (dx < sdx) {
    //         if (dv > cldv) {
    //             accel = approaching(dv, dx, abx, predAccel);
    //             branch = 2;
    //         } else if (dv > opdv) {
    //             accel = following(vars->accelSign);
    //             branch = 3;
    //         } else {
    //             accel = fullspeed(v, vpref, dx, abx);
    //             branch = 4;
    //         }
    //     } else {
    //         if (dv > sdv && dx < dmax) { //@note other versions have an disjunction instead of conjunction
    //             accel = approaching(dv, dx, abx, predAccel);
    //             branch = 5;
    //         } else {
    //             accel = fullspeed(v, vpref, dx, abx);
    //             branch = 6;
    //         }
    //     }
    //     // since we have hard constraints on accel we may as well use them here
    // #ifdef DEBUG_V
    //     const double rawAccel = accel;
    // #endif
    //     accel = MAX2(MIN2(accel, myAccel), -myEmergencyDecel);
    //     const double vNew = MAX2(0., v + ACCEL2SPEED(accel)); // don't allow negative speeds
    // #ifdef DEBUG_V
    //     if (veh->isSelected() && !MSGlobals::gComputeLC) {
    //         std::cout << SIMTIME << " Wiedemann::_v veh=" << veh->getID()
    //                   << " v=" << v << " pV=" << predSpeed << " pA=" << predAccel << " gap=" << gap
    //                   << " dv=" << dv << " dx=" << dx << " ax=" << myAX << " bx=" << bx << " abx=" << abx
    //                   << " sdx=" << sdx << " sdv=" << sdv << " cldv=" << cldv << " opdv=" << opdv
    //                   << " branch=" << branch << " rawAccel=" << rawAccel
    //                   << " accel=" << accel << " vNew=" << vNew << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(branch);
    // #endif
    //     return vNew;
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, predSpeed: f64, gap: f64, predAccel: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_Wiedemann_VehicleVariables);
        let dx = gap + self.myType.get_length(); // wiedemann uses brutto gap
        let v = veh.get_speed();
        let vpref = veh.get_max_speed();
        let dv = v - predSpeed;
        // desired minimum following distance at low speed difference
        let bx = (1. + 7. * self.mySecurity) * v.sqrt(); // Harding propose a factor of  *.8 here
        let abx = self.myAX + bx; // Harding propose a factor of  *.8 here
        let ex = 2. - self.myEstimation; // + RandHelper::randNorm(0.5, 0.15)
        let sdx = self.myAX + ex * bx; // the distance at which we drift out of following
        let sdv_root = (dx - self.myAX) / self.myCX;
        let sdv = sdv_root * sdv_root;
        let cldv = sdv * ex * ex;
        let opdv = cldv * (-1. - 2. * veh.get_rng().m().rand_norm(0.5, 0.15));
        // D_MAX is too low to brake safely when driving at speeds above 36m/s
        let dmax = max2(D_MAX, self.brake_gap_2(v, self.myDecel, 0.));
        // select the regime, get new acceleration, compute new speed based
        let mut accel;
        let mut branch = 0;
        if dx <= abx {
            accel = self.emergency(dv, dx, predAccel, v, gap, abx, bx);
            branch = 1;
        } else if dx < sdx {
            if dv > cldv {
                accel = self.approaching(dv, dx, abx, predAccel);
                branch = 2;
            } else if dv > opdv {
                accel = self.following(vars.accelSign);
                branch = 3;
            } else {
                accel = self.fullspeed(v, vpref, dx, abx);
                branch = 4;
            }
        } else {
            if dv > sdv && dx < dmax {
                //@note other versions have an disjunction instead of conjunction
                accel = self.approaching(dv, dx, abx, predAccel);
                branch = 5;
            } else {
                accel = self.fullspeed(v, vpref, dx, abx);
                branch = 6;
            }
        }
        // since we have hard constraints on accel we may as well use them here
        accel = max2(min2(accel, self.myAccel), -self.myEmergencyDecel);
        let vNew = max2(0., v + accel2speed(accel)); // don't allow negative speeds
        let _ = branch; // branch is only used for the debug output in C++
        vNew
    }

    /// C++: `double MSCFModel_Wiedemann::fullspeed(double, double, double, double) const` [const, private]
    /// @{
    // double
    // MSCFModel_Wiedemann::fullspeed(double v, double vpref, double dx, double abx) const {
    //     // maximum acceleration is reduced with increasing speed
    //     double bmax = 0.2 + 0.8 * myAccel * (7 - sqrt(v));
    //     // if veh just drifted out of a 'following' process the acceleration is reduced
    //     double accel = dx <= 2 * abx ? MIN2(myMinAccel, bmax * (dx - abx) / abx) : bmax;
    //     if (v > vpref) {
    //         accel = - accel;
    //     }
    //     return accel;
    // }
    #[allow(unused_variables)]
    pub fn fullspeed(&self, v: f64, vpref: f64, dx: f64, bx: f64) -> f64 {
        // maximum acceleration is reduced with increasing speed
        let bmax = 0.2 + 0.8 * self.myAccel * (7. - v.sqrt());
        // if veh just drifted out of a 'following' process the acceleration is reduced
        let mut accel = if dx <= 2. * bx { min2(self.myMinAccel, bmax * (dx - bx) / bx) } else { bmax };
        if v > vpref {
            accel = -accel;
        }
        accel
    }

    /// C++: `double MSCFModel_Wiedemann::following(double) const` [const, private]
    // double
    // MSCFModel_Wiedemann::following(double sign) const {
    //     return myMinAccel * sign;
    // }
    #[allow(unused_variables)]
    pub fn following(&self, sign: f64) -> f64 {
        self.myMinAccel * sign
    }

    /// C++: `double MSCFModel_Wiedemann::approaching(double, double, double, double) const` [const, private]
    // double
    // MSCFModel_Wiedemann::approaching(double dv, double dx, double abx, double predAccel) const {
    //     // there is singularity in the formula. we do the sanity check outside
    //     assert(abx < dx);
    //     // @note: the original model does not have a limit on maximum deceleration here.
    //     // We add this to avoid cascading emergency deceleration
    //     // also, the multiplier on predAccel is always 1 in the original model
    //     return MAX2(0.5 * dv * dv / (abx - dx) + predAccel * PRED_DECEL_MULT, -myMaxApproachingDecel);
    // }
    #[allow(unused_variables)]
    pub fn approaching(&self, dv: f64, dx: f64, abx: f64, predAccel: f64) -> f64 {
        // there is singularity in the formula. we do the sanity check outside
        // @note: the original model does not have a limit on maximum deceleration here.
        // We add this to avoid cascading emergency deceleration
        // also, the multiplier on predAccel is always 1 in the original model
        max2(0.5 * dv * dv / (abx - dx) + predAccel * PRED_DECEL_MULT, -self.myMaxApproachingDecel)
    }

    /// C++: `double MSCFModel_Wiedemann::emergency(double, double, double, double, double, double, double) const` [const, private]
    // double
    // MSCFModel_Wiedemann::emergency(double dv, double dx, double predAccel, double v, double gap, double abx, double bx) const {
    //     // wiedemann assumes that dx will always be larger than myAX (sumo may
    //     // violate this assumption when crashing (-:
    //     //
    //     // predAccel is called b_(n-1) in the literature and it's multipleir is always 1
    //
    //     if (dx > myAX) {
    //         const double bmin = B_MIN_ADD + B_MIN_MULT * v;
    //         const double accel = (0.5 * dv * dv / (myAX - dx)
    //                               + predAccel * PRED_DECEL_MULT_EMERGENCY
    //                               + bmin * (abx - gap) / bx);
    //         return accel;
    //     } else {
    //         return -myEmergencyDecel;
    //     }
    //
    //     // emergency according to C.Werner
    //     //return -myEmergencyDecel;
    // }
    #[allow(unused_variables)]
    pub fn emergency(&self, dv: f64, dx: f64, predAccel: f64, v: f64, gap: f64, abx: f64, bx: f64) -> f64 {
        // wiedemann assumes that dx will always be larger than myAX (sumo may
        // violate this assumption when crashing (-:
        //
        // predAccel is called b_(n-1) in the literature and it's multipleir is always 1
        if dx > self.myAX {
            let bmin = B_MIN_ADD + B_MIN_MULT * v;
            let accel = 0.5 * dv * dv / (self.myAX - dx) + predAccel * PRED_DECEL_MULT_EMERGENCY + bmin * (abx - gap) / bx;
            accel
        } else {
            -self.myEmergencyDecel
        }
    }

    /// C++: `MSCFModel_Wiedemann & MSCFModel_Wiedemann::operator=(const MSCFModel_Wiedemann &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSCFModel_Wiedemann) -> &mut MSCFModel_Wiedemann {
        // C++ implicit copy assignment: copy every member
        *self = s.clone();
        self
    }

}

/// @}
/// C++ class `MSCFModel_Wiedemann::VehicleVariables` (microsim/cfmodels/MSCFModel_Wiedemann.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_Wiedemann_VehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    /// state variable for remembering the drift direction
    pub accelSign: f64,
}

impl std::ops::Deref for MSCFModel_Wiedemann_VehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Wiedemann_VehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_Wiedemann_VehicleVariables {
    /// C++: `void VehicleVariables::VehicleVariables()`
    // VehicleVariables() : accelSign(1) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSCFModel_Wiedemann_VehicleVariables {
            base: MSCFModel_VehicleVariables {},
            accelSign: 1.,
        }
    }

    /// C++: `void VehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_Wiedemann::VehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "Wiedemann");
    //     std::ostringstream internals;
    //     internals << accelSign;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "Wiedemann");
        let internals = f64_to_string_prec(self.accelSign, 6);
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
    }

    /// C++: `void VehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_Wiedemann::VehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "Wiedemann") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for Wiedemann", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> accelSign;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "Wiedemann" {
            return Err(utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for Wiedemann", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        if let Some(v) = state.split_whitespace().next().and_then(|s| s.parse::<f64>().ok()) {
            self.accelSign = v;
        }
        Ok(())
    }

}

