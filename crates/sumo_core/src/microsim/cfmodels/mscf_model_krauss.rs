//! Port of microsim/cfmodels/MSCFModel_Krauss.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use super::mscf_model_krauss_orig1::KRAUSS_ORIG1_VT;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;
use utils::common::sumo_vehicle_class::SVC_BICYCLE;

/// Krauss car-following model, with acceleration decrease and faster start
/// C++ class `MSCFModel_Krauss` (microsim/cfmodels/MSCFModel_Krauss.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_Krauss {
    /// C++ base class `MSCFModel_KraussOrig1`
    pub base: MSCFModel_KraussOrig1,
    /// The vehicle's update period for dawdling
    pub myDawdleStep: SUMOTime,
}

impl std::ops::Deref for MSCFModel_Krauss {
    type Target = MSCFModel_KraussOrig1;
    fn deref(&self) -> &MSCFModel_KraussOrig1 {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Krauss {
    fn deref_mut(&mut self) -> &mut MSCFModel_KraussOrig1 {
        &mut self.base
    }
}

impl MSCFModel_Krauss {
    /// C++: `void MSCFModel_Krauss::MSCFModel_Krauss(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_Krauss::MSCFModel_Krauss(const MSVehicleType* vtype) :
    //     MSCFModel_KraussOrig1(vtype),
    //     myDawdleStep(TIME2STEPS(vtype->getParameter().getCFParam(SUMO_ATTR_SIGMA_STEP, TS))) {
    //     if (myDawdleStep % DELTA_T != 0) {
    //         SUMOTime rem = myDawdleStep % DELTA_T;
    //         if (rem < DELTA_T / 2) {
    //             myDawdleStep += -rem;
    //         } else {
    //             myDawdleStep += DELTA_T - rem;
    //         }
    //         WRITE_WARNINGF(TL("Rounding 'sigmaStep' to % for vType '%'"), STEPS2TIME(myDawdleStep), vtype->getID());
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let myDawdleStep0 = time2steps(vtype.get_parameter().get_cf_param(SUMO_ATTR_SIGMA_STEP, ts()));
        let base = MSCFModel_KraussOrig1::new(vtype);
        let mut myDawdleStep = myDawdleStep0;
        if myDawdleStep % delta_t() != 0 {
            let rem = myDawdleStep % delta_t();
            if rem < delta_t() / 2 {
                myDawdleStep += -rem;
            } else {
                myDawdleStep += delta_t() - rem;
            }
            utils::common::msg_handler::write_warning(&format!("Rounding 'sigmaStep' to {} for vType '{}'", f64_to_string_prec(steps2time(myDawdleStep), 6), base.myType.get_id()));
        }
        let mut this = MSCFModel_Krauss { base, myDawdleStep };
        this.base.base.vt = &KRAUSS_VT;
        this
    }

    /// C++: `void MSCFModel_Krauss::~MSCFModel_Krauss() noexcept` [virtual]
    /// Destructor
    // MSCFModel_Krauss::~MSCFModel_Krauss() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_Krauss::patchSpeedBeforeLC(const MSVehicle *, double, double) const` [const, virtual]
    /// apply dawdling
    // double
    // MSCFModel_Krauss::patchSpeedBeforeLC(const MSVehicle* veh, double vMin, double vMax) const {
    //     const double sigma = (veh->passingMinor()
    //                           ? veh->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_SIGMA_MINOR, myDawdle)
    //                           : myDawdle);
    //     double vDawdle;
    //     if (myDawdleStep > DELTA_T) {
    //         VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //         if (SIMSTEP % myDawdleStep == vars->updateOffset) {
    //             const double vD = MAX2(vMin, dawdle2(vMax, sigma, veh->getRNG()));
    //             const double a1 = SPEED2ACCEL(vMax - veh->getSpeed());
    //             const double a2 = SPEED2ACCEL(vD - vMax);
    //             const double accelMax = (veh->getLane()->getVehicleMaxSpeed(veh) - veh->getSpeed()) / STEPS2TIME(myDawdleStep);
    //             // avoid exceeding maxSpeed before the next sigmaStep
    //             vars->accelDawdle = MIN2(a1, accelMax) + a2;
    //             vDawdle = veh->getSpeed() + ACCEL2SPEED(vars->accelDawdle);
    //             //std::cout << SIMTIME << " v=" << veh->getSpeed() << " updated vD=" << vD<< " a1=" << a1 << " a2=" << a2 << " aM=" << accelMax << " accelDawdle=" << vars->accelDawdle << " vDawdle=" << vDawdle << "\n";
    //         } else {
    //             const double safeAccel = SPEED2ACCEL(vMax - veh->getSpeed());
    //             const double accel = MIN2(safeAccel, vars->accelDawdle);
    //             vDawdle = MAX2(vMin, MIN2(vMax, veh->getSpeed() + ACCEL2SPEED(accel)));
    //             //std::cout << SIMTIME << " v=" << veh->getSpeed() << " safeAccel=" << safeAccel << " accel=" << accel << " vDawdle=" << vDawdle << "\n";
    //         }
    //     } else {
    //         vDawdle = MAX2(vMin, dawdle2(vMax, sigma, veh->getRNG()));
    //         //const double accel1 = SPEED2ACCEL(vMax - veh->getSpeed());
    //         //const double accel2 = SPEED2ACCEL(vDawdle - vMax);
    //         //std::cout << SIMTIME << " v=" << veh->getSpeed() << " updated vDawdle=" << vDawdle << " a1=" << accel1 << " a2=" << accel2 << " accelDawdle=" << SPEED2ACCEL(vDawdle - veh->getSpeed()) << "\n";
    //     }
    //     return vDawdle;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_before_lc(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        let sigma = if veh.passing_minor() {
            veh.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_SIGMA_MINOR, self.myDawdle)
        } else {
            self.myDawdle
        };
        let vDawdle;
        if self.myDawdleStep > delta_t() {
            let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_Krauss_VehicleVariables);
            if MSNet::get_instance().get_current_time_step() % self.myDawdleStep == vars.updateOffset {
                let vD = max2(vMin, self.dawdle2(vMax, sigma, veh.get_rng()));
                let a1 = speed2accel(vMax - veh.get_speed());
                let a2 = speed2accel(vD - vMax);
                let accelMax = (veh.get_lane().get_vehicle_max_speed(traffic_object(veh)) - veh.get_speed()) / steps2time(self.myDawdleStep);
                // avoid exceeding maxSpeed before the next sigmaStep
                vars.m().accelDawdle = min2(a1, accelMax) + a2;
                vDawdle = veh.get_speed() + accel2speed(vars.accelDawdle);
            } else {
                let safeAccel = speed2accel(vMax - veh.get_speed());
                let accel = min2(safeAccel, vars.accelDawdle);
                vDawdle = max2(vMin, min2(vMax, veh.get_speed() + accel2speed(accel)));
            }
        } else {
            vDawdle = max2(vMin, self.dawdle2(vMax, sigma, veh.get_rng()));
        }
        vDawdle
    }

    /// C++: `double MSCFModel_Krauss::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling) this uses the maximumSafeStopSpeed
    // double
    // MSCFModel_Krauss::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason usage) const {
    //     // NOTE: This allows return of smaller values than minNextSpeed().
    //     // Only relevant for the ballistic update: We give the argument headway=veh->getActionStepLengthSecs(), to assure that
    //     // the stopping position is approached with a uniform deceleration also for tau!=veh->getActionStepLengthSecs().
    //     applyHeadwayPerceptionError(veh, speed, gap);
    //     const bool relaxEmergency = usage != FUTURE; // do not relax insertionStopSpeed
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, veh->getActionStepLengthSecs(), relaxEmergency), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        // applyHeadwayPerceptionError may change the gap, so work on a local copy
        let mut gap = gap2pred;
        self.apply_headway_perception_error(veh, speed, &mut gap);
        // do not relax insertionStopSpeed
        let relaxEmergency = usage != MSCFModel_CalcReason::FUTURE;
        min2(self.maximum_safe_stop_speed(gap, decel, speed, false, veh.get_action_step_length_secs(), relaxEmergency), self.max_next_speed(speed, veh))
    }

    /// C++: `double MSCFModel_Krauss::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling) this uses the maximumSafeFollowSpeed
    // double
    // MSCFModel_Krauss::followSpeed(const MSVehicle* const veh, double speed, double gap, double predSpeed, double predMaxDecel, const MSVehicle* const pred, const CalcReason /*usage*/) const {
    //     //gDebugFlag1 = DEBUG_COND;
    //     applyHeadwayAndSpeedDifferencePerceptionErrors(veh, speed, gap, predSpeed, predMaxDecel, pred);
    //     //gDebugFlag1 = DEBUG_COND; // enable for DEBUG_EMERGENCYDECEL
    //     const double vsafe = maximumSafeFollowSpeed(gap, speed, predSpeed, predMaxDecel);
    //     //gDebugFlag1 = false;
    //     const double vmax = maxNextSpeed(speed, veh);
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MIN2(vsafe, vmax);
    //     } else {
    //         // ballistic
    //         // XXX: the euler variant can break as strong as it wishes immediately! The ballistic cannot, refs. #2575.
    //         return MAX2(MIN2(vsafe, vmax), minNextSpeedEmergency(speed));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let mut gap = gap2pred;
        let mut predSpeed = predSpeed;
        self.apply_headway_and_speed_difference_perception_errors(veh, speed, &mut gap, &mut predSpeed, predMaxDecel, pred);
        let vsafe = self.maximum_safe_follow_speed(gap, speed, predSpeed, predMaxDecel, false);
        let vmax = self.max_next_speed(speed, veh);
        if g_semi_implicit_euler_update() {
            min2(vsafe, vmax)
        } else {
            // ballistic
            // XXX: the euler variant can break as strong as it wishes immediately! The ballistic cannot, refs. #2575.
            max2(min2(vsafe, vmax), self.min_next_speed_emergency(speed, Ptr::default()))
        }
    }

    /// C++: `int MSCFModel_Krauss::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_KRAUSS;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_KRAUSS as i32
    }

    /// C++: `MSCFModel * MSCFModel_Krauss::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_Krauss::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Krauss(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Krauss::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `VehicleVariables * MSCFModel_Krauss::createVehicleVariables() const` [const, virtual]
    // VehicleVariables* createVehicleVariables() const {
    //         if (myDawdleStep > DELTA_T) {
    //             return new VehicleVariables(myDawdleStep);
    //         }
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        if self.myDawdleStep > delta_t() {
            // shortcut: allocated with Box::into_raw like C++ `new`; the owning vehicle must free it once ported
            Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Krauss_VehicleVariables::new(self.myDawdleStep))) as *mut MSCFModel_VehicleVariables)
        } else {
            Ptr::default()
        }
    }

    /// C++: `double MSCFModel_Krauss::dawdle2(double, double, SumoRNG *) const` [const, protected]
    /// Applies driver imperfection (dawdling / sigma)
    // double
    // MSCFModel_Krauss::dawdle2(double speed, double sigma, SumoRNG* rng) const {
    //     if (!MSGlobals::gSemiImplicitEulerUpdate) {
    //         // in case of the ballistic update, negative speeds indicate
    //         // a desired stop before the completion of the next timestep.
    //         // We do not allow dawdling to overwrite this indication
    //         if (speed < 0) {
    //             return speed;
    //         }
    //     }
    //     // generate random number out of [0,1)
    //     const double random = RandHelper::rand(rng);
    //     // Dawdle.
    //     if (speed < myAccel) {
    //         // we should not prevent vehicles from driving just due to dawdling
    //         //  if someone is starting, he should definitely start
    //         // (but what about slow-to-start?)!!!
    //         speed -= ACCEL2SPEED(sigma * speed * random);
    //     } else {
    //         speed -= ACCEL2SPEED(sigma * myAccel * random);
    //     }
    //     return MAX2(0., speed);
    // }
    #[allow(unused_variables)]
    pub fn dawdle2(&self, speed: f64, sigma: f64, rng: Ptr<SumoRNG>) -> f64 {
        let mut speed = speed;
        if !g_semi_implicit_euler_update() {
            // in case of the ballistic update, negative speeds indicate
            // a desired stop before the completion of the next timestep.
            // We do not allow dawdling to overwrite this indication
            if speed < 0. {
                return speed;
            }
        }
        // generate random number out of [0,1)
        let random = rng.m().rand();
        // Dawdle.
        if speed < self.myAccel {
            // we should not prevent vehicles from driving just due to dawdling
            //  if someone is starting, he should definitely start
            speed -= accel2speed(sigma * speed * random);
        } else {
            speed -= accel2speed(sigma * self.myAccel * random);
        }
        max2(0., speed)
    }

}

fn krauss_patch_speed_before_lc(m: &MSCFModel, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
    cf_cast::<MSCFModel_Krauss>(m).patch_speed_before_lc(veh, vMin, vMax)
}
fn krauss_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_Krauss>(m).stop_speed(veh, speed, gap, decel, usage)
}
fn krauss_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_Krauss>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage)
}
fn krauss_get_model_id(m: &MSCFModel) -> i32 {
    cf_cast::<MSCFModel_Krauss>(m).get_model_id()
}
fn krauss_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
    cf_cast::<MSCFModel_Krauss>(m).duplicate(vtype)
}
fn krauss_save_vehicle_variables(m: &MSCFModel, vars: &MSCFModel_VehicleVariables, out: &mut OutputDevice) {
    unsafe { &*(vars as *const MSCFModel_VehicleVariables as *const MSCFModel_Krauss_VehicleVariables) }.save_state(out, m)
}
fn krauss_load_vehicle_variables(_m: &MSCFModel, vars: &mut MSCFModel_VehicleVariables, attrs: &SUMOSAXAttributes) {
    unsafe { &mut *(vars as *mut MSCFModel_VehicleVariables as *mut MSCFModel_Krauss_VehicleVariables) }.load_state(attrs).unwrap_or_else(|e| panic!("{}", e))
}
fn krauss_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> {
    cf_cast::<MSCFModel_Krauss>(m).create_vehicle_variables()
}

/// Virtual table of MSCFModel_Krauss: its overrides on top of the MSCFModel_KraussOrig1 table
pub const KRAUSS_VT: MSCFModelVt = MSCFModelVt {
    patch_speed_before_lc: krauss_patch_speed_before_lc,
    stop_speed_2: krauss_stop_speed_2,
    follow_speed: krauss_follow_speed,
    get_model_id: krauss_get_model_id,
    duplicate: krauss_duplicate,
    create_vehicle_variables: krauss_create_vehicle_variables,
    save_vehicle_variables: krauss_save_vehicle_variables,
    load_vehicle_variables: krauss_load_vehicle_variables,
    ..KRAUSS_ORIG1_VT
};

/// C++ class `MSCFModel_Krauss::VehicleVariables` (microsim/cfmodels/MSCFModel_Krauss.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_Krauss_VehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    /// the accleration due to dawdling
    pub accelDawdle: f64,
    pub updateOffset: SUMOTime,
}

impl std::ops::Deref for MSCFModel_Krauss_VehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Krauss_VehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_Krauss_VehicleVariables {
    /// C++: `void VehicleVariables::VehicleVariables(SUMOTime)`
    // MSCFModel_Krauss::VehicleVariables::VehicleVariables(SUMOTime dawdleStep)
    //     : accelDawdle(1e6), updateOffset(SIMSTEP % dawdleStep + DELTA_T) { }
    #[allow(unused_variables)]
    pub fn new(dawdleStep: SUMOTime) -> Self {
        MSCFModel_Krauss_VehicleVariables {
            base: MSCFModel_VehicleVariables {},
            accelDawdle: 1e6,
            updateOffset: MSNet::get_instance().get_current_time_step() % dawdleStep + delta_t(),
        }
    }

    /// C++: `void VehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_Krauss::VehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "Krauss");
    //     std::ostringstream internals;
    //     internals << accelDawdle << " ";
    //     internals << updateOffset;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "Krauss");
        let internals = format!("{} {}", f64_to_string_prec(self.accelDawdle, 6), self.updateOffset);
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
    }

    /// C++: `void VehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_Krauss::VehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "Krauss") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for Krauss", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> accelDawdle;
    //     bis >> updateOffset;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "Krauss" {
            return Err(utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for Krauss", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        if let Some(v) = bis.next().and_then(|s| s.parse::<f64>().ok()) {
            self.accelDawdle = v;
        }
        if let Some(v) = bis.next().and_then(|s| s.parse::<SUMOTime>().ok()) {
            self.updateOffset = v;
        }
        Ok(())
    }

}

#[cfg(test)]
mod vt_dispatch_tests {
    use super::*;

    #[test]
    fn base_pointer_reaches_krauss_overrides() {
        let vtp = SUMOVTypeParameter::new("passenger", utils::common::sumo_vehicle_class::SVC_PASSENGER);
        let vtype = Ptr::from_mut(Box::leak(Box::new(MSVehicleType::new(&vtp))));
        let krauss = Ptr::from_mut(Box::leak(Box::new(MSCFModel_Krauss::new(vtype))));
        let cf: Ptr<MSCFModel> = krauss.cast::<MSCFModel>();
        assert_eq!(cf.m().get_model_id(), SUMO_TAG_CF_KRAUSS as i32);
        // base MSCFModel::getImperfection returns -1; KraussOrig1 returns myDawdle
        cf.m().set_imperfection(0.3);
        assert_eq!(cf.m().get_imperfection(), 0.3);
    }

    #[test]
    fn most_derived_table_wins_through_base_pointer() {
        let vtp = SUMOVTypeParameter::new("passenger", utils::common::sumo_vehicle_class::SVC_PASSENGER);
        let vtype = Ptr::from_mut(Box::leak(Box::new(MSVehicleType::new(&vtp))));
        let ps = Ptr::from_mut(Box::leak(Box::new(MSCFModel_KraussPS::new(vtype))));
        let cf: Ptr<MSCFModel> = ps.cast::<MSCFModel>();
        assert_eq!(cf.m().get_model_id(), SUMO_TAG_CF_KRAUSS_PLUS_SLOPE as i32);
    }

    #[test]
    #[should_panic(expected = "MSCFModel::getModelID is pure virtual")]
    fn bare_base_model_is_pure_virtual() {
        let vtp = SUMOVTypeParameter::new("passenger", utils::common::sumo_vehicle_class::SVC_PASSENGER);
        let vtype = Ptr::from_mut(Box::leak(Box::new(MSVehicleType::new(&vtp))));
        let base = MSCFModel::new(vtype);
        base.get_model_id();
    }
}

