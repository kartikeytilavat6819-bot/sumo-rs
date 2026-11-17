//! Port of microsim/cfmodels/MSCFModel_CC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use super::cc_const::*;
use super::cc_vehicle_variables::*;
use super::mscf_model_krauss::MSCFModel_Krauss;
use utils::common::time::{accel2speed, speed2accel, ts};

/// A set of automatic Cruise Controllers, including classic Cruise Control (CC), Adaptive Cruise Control (ACC) and Cooperative Adaptive Cruise Control (CACC). Take as references the chapters 5, 6 and 7 of the Rajamani's book "Vehicle dynamics and control" (2011). This model is meant to be used for simulation of platooning systems in mixed scenarios, so with automatic and driver controlled vehicles. The platooning manager is a distributed application implemented for veins (so for omnet++) supported by a 802.11p based communication protocol, which will determine the actions to be performed (such as switching on the automatic controller, or the lane to move to) and communicate them to this car following models via TraCI
/// C++ class `MSCFModel_CC` (microsim/cfmodels/MSCFModel_CC.h)
#[repr(C)]
pub struct MSCFModel_CC {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// the car following model which drives the car when automated cruising is disabled, i.e., the human driver
    pub myHumanDriver: Ptr<MSCFModel>,
    /// The maximum deceleration that the CC can output
    pub myCcDecel: f64,
    /// The maximum acceleration that the CC can output
    pub myCcAccel: f64,
    /// the constant gap for CACC
    pub myConstantSpacing: f64,
    /// design constant for CC
    pub myKp: f64,
    /// design constant for ACC
    pub myLambda: f64,
    /// design constant for CACC
    pub myC1: f64,
    /// design constant for CACC
    pub myXi: f64,
    /// design constant for CACC
    pub myOmegaN: f64,
    /// engine time constant used for actuation lag
    pub myTau: f64,
    /// number of lanes in the highway, in the absence of on-/off-ramps. This is used to move to the correct lane even when a lane is added for on-/off-ramps
    pub myLanesCount: i32,
    /// Ploeg's CACC parameters
    pub myPloegH: f64,
    pub myPloegKp: f64,
    pub myPloegKd: f64,
    /// flatbed CACC parameters
    pub myFlatbedKa: f64,
    pub myFlatbedKv: f64,
    pub myFlatbedKp: f64,
    pub myFlatbedH: f64,
    pub myFlatbedD: f64,
}

fn cc_finalize_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
    cf_cast::<MSCFModel_CC>(m).finalize_speed(veh, vPos)
}
fn cc_free_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_CC>(m).free_speed(veh, speed, seen, maxSpeed, onInsertion, usage)
}
fn cc_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_CC>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage)
}
fn cc_insertion_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
    cf_cast::<MSCFModel_CC>(m).insertion_follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred)
}
fn cc_interaction_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, vL: f64) -> f64 {
    cf_cast::<MSCFModel_CC>(m).interaction_gap(veh, vL)
}
fn cc_get_secure_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
    cf_cast::<MSCFModel_CC>(m).get_secure_gap(veh, pred, speed, leaderSpeed, leaderMaxDecel)
}
fn cc_max_next_speed(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    cf_cast::<MSCFModel_CC>(m).max_next_speed(speed, veh)
}
fn cc_min_next_speed(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    cf_cast::<MSCFModel_CC>(m).min_next_speed(speed, veh)
}
fn cc_get_parameter(m: &MSCFModel, veh: Ptr<MSVehicle>, key: &str) -> String {
    cf_cast::<MSCFModel_CC>(m).get_parameter(veh, key)
}
fn cc_set_parameter(m: &MSCFModel, veh: Ptr<MSVehicle>, key: &str, value: &str) -> Result<(), InvalidArgument> {
    cf_cast::<MSCFModel_CC>(m).set_parameter(veh, key, value)
}
fn cc_get_model_id(m: &MSCFModel) -> i32 {
    cf_cast::<MSCFModel_CC>(m).get_model_id()
}
fn cc_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
    cf_cast::<MSCFModel_CC>(m).duplicate(vtype)
}
fn cc_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> {
    cf_cast::<MSCFModel_CC>(m).create_vehicle_variables()
}

fn cc_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_CC>(m).stop_speed(veh, speed, gap, decel, usage)
}

/// Virtual table of MSCFModel_CC: its overrides on top of the MSCFModel table
pub const CC_VT: MSCFModelVt = MSCFModelVt {
    stop_speed_2: cc_stop_speed_2,
    finalize_speed: cc_finalize_speed,
    free_speed: cc_free_speed,
    follow_speed: cc_follow_speed,
    insertion_follow_speed: cc_insertion_follow_speed,
    interaction_gap: cc_interaction_gap,
    get_secure_gap: cc_get_secure_gap,
    max_next_speed: cc_max_next_speed,
    min_next_speed: cc_min_next_speed,
    get_model_id: cc_get_model_id,
    duplicate: cc_duplicate,
    create_vehicle_variables: cc_create_vehicle_variables,
    get_parameter: cc_get_parameter,
    set_parameter: cc_set_parameter,
    ..DEFAULT_MSCF_MODEL_VT
};

impl std::ops::Deref for MSCFModel_CC {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_CC {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_CC {
    /// C++: `void MSCFModel_CC::MSCFModel_CC(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_CC::MSCFModel_CC(const MSVehicleType* vtype) : MSCFModel(vtype),
    //     myCcDecel(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_CCDECEL, 1.5)),
    //     myCcAccel(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_CCACCEL, 1.5)),
    //     myConstantSpacing(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_CONSTSPACING, 5.0)),
    //     myKp(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_KP, 1.0)),
    //     myLambda(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_LAMBDA, 0.1)),
    //     myC1(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_C1, 0.5)),
    //     myXi(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_XI, 1.0)),
    //     myOmegaN(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_OMEGAN, 0.2)),
    //     myTau(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_TAU, 0.5)),
    //     myLanesCount((int)vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_LANES_COUNT, -1)),
    //     myPloegH(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_PLOEG_H, 0.5)),
    //     myPloegKp(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_PLOEG_KP, 0.2)),
    //     myPloegKd(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_PLOEG_KD, 0.7)),
    //     myFlatbedKa(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_FLATBED_KA, 2.4)),
    //     myFlatbedKv(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_FLATBED_KV, 0.6)),
    //     myFlatbedKp(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_FLATBED_KP, 12.0)),
    //     myFlatbedH(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_FLATBED_H, 4.0)),
    //     myFlatbedD(vtype->getParameter().getCFParam(SUMO_ATTR_CF_CC_FLATBED_D, 5.0)) {
    //
    //     //if the lanes count has not been specified in the attributes of the model, lane changing cannot properly work
    //     if (myLanesCount == -1) {
    //         throw ProcessError(TL("The number of lanes needs to be specified in the attributes of carFollowing-CC with the \"lanesCount\" attribute"));
    //     }
    //
    //     //instantiate the driver model. For now, use Krauss as default, then needs to be parameterized
    //     myHumanDriver = new MSCFModel_Krauss(vtype);
    //
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let base = MSCFModel::new(vtype);
        let myCcDecel = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_CCDECEL, 1.5);
        let myCcAccel = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_CCACCEL, 1.5);
        let myConstantSpacing = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_CONSTSPACING, 5.0);
        let myKp = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_KP, 1.0);
        let myLambda = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_LAMBDA, 0.1);
        let myC1 = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_C1, 0.5);
        let myXi = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_XI, 1.0);
        let myOmegaN = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_OMEGAN, 0.2);
        let myTau = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_TAU, 0.5);
        let myLanesCount = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_LANES_COUNT, -1.) as i32;
        let myPloegH = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_PLOEG_H, 0.5);
        let myPloegKp = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_PLOEG_KP, 0.2);
        let myPloegKd = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_PLOEG_KD, 0.7);
        let myFlatbedKa = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_FLATBED_KA, 2.4);
        let myFlatbedKv = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_FLATBED_KV, 0.6);
        let myFlatbedKp = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_FLATBED_KP, 12.0);
        let myFlatbedH = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_FLATBED_H, 4.0);
        let myFlatbedD = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_CC_FLATBED_D, 5.0);

        //if the lanes count has not been specified in the attributes of the model, lane changing cannot properly work
        if myLanesCount == -1 {
            panic!("{}", utils::common::util_exceptions::ProcessError::new("The number of lanes needs to be specified in the attributes of carFollowing-CC with the \"lanesCount\" attribute"));
        }

        //instantiate the driver model. For now, use Krauss as default, then needs to be parameterized
        let myHumanDriver = Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Krauss::new(vtype))) as *mut MSCFModel);
        let mut this = MSCFModel_CC {
            base,
            myHumanDriver,
            myCcDecel,
            myCcAccel,
            myConstantSpacing,
            myKp,
            myLambda,
            myC1,
            myXi,
            myOmegaN,
            myTau,
            myLanesCount,
            myPloegH,
            myPloegKp,
            myPloegKd,
            myFlatbedKa,
            myFlatbedKv,
            myFlatbedKp,
            myFlatbedH,
            myFlatbedD,
        };
        this.base.vt = &CC_VT;
        this
    }

    /// C++: `void MSCFModel_CC::~MSCFModel_CC() noexcept` [virtual]
    /// Destructor
    // MSCFModel_CC::~MSCFModel_CC() {
    //     delete myHumanDriver;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: myHumanDriver is leaked (allocated with Box::into_raw in new, like C++ `new`); free it once model ownership is ported
    }

    /// C++: `double MSCFModel_CC::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    // double
    // MSCFModel_CC::getSecureGap(const MSVehicle* const veh, const MSVehicle* const pred, const double speed, const double leaderSpeed, const double leaderMaxDecel) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //
    //     const double tolerance = 0.8;
    //     switch (vars->activeController) {
    //         case Plexe::CACC:
    //         case Plexe::FAKED_CACC:
    //             return vars->caccSpacing * tolerance;
    //         case Plexe::ACC:
    //             return (vars->accHeadwayTime * speed + 2) * tolerance;
    //         case Plexe::PLOEG:
    //             return (vars->ploegH * speed + 2) * tolerance;
    //         case Plexe::CONSENSUS:
    //             return d_i_j(vars->vehicles, vars->h, 1, 0) * tolerance;
    //         case Plexe::FLATBED:
    //             return (vars->flatbedD - vars->flatbedH * (speed - leaderSpeed)) * tolerance;
    //         case Plexe::DRIVER:
    //             return myHumanDriver->getSecureGap(veh, pred, speed, leaderSpeed, leaderMaxDecel);
    //         default:
    //             throw InvalidArgument("Unsupported activeController" + toString(vars->activeController));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, arg1: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);

        let tolerance = 0.8;
        match vars.activeController {
            ACTIVE_CONTROLLER::CACC | ACTIVE_CONTROLLER::FAKED_CACC => vars.caccSpacing * tolerance,
            ACTIVE_CONTROLLER::ACC => (vars.accHeadwayTime * speed + 2.) * tolerance,
            ACTIVE_CONTROLLER::PLOEG => (vars.ploegH * speed + 2.) * tolerance,
            ACTIVE_CONTROLLER::CONSENSUS => self.d_i_j(Ptr::from_raw(vars.vehicles.as_ptr() as *mut VEHICLE_DATA), vars.h, 1, 0) * tolerance,
            ACTIVE_CONTROLLER::FLATBED => (vars.flatbedD - vars.flatbedH * (speed - leaderSpeed)) * tolerance,
            ACTIVE_CONTROLLER::DRIVER => self.myHumanDriver.get_secure_gap(veh, arg1, speed, leaderSpeed, leaderMaxDecel),
        }
    }

    /// C++: `double MSCFModel_CC::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences
    // double
    // MSCFModel_CC::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     double vNext;
    //     //acceleration computed by the controller
    //     double controllerAcceleration;
    //     //acceleration after engine actuation
    //     double engineAcceleration;
    //
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //
    //     //call processNextStop() to ensure vehicle removal in case of crash
    //     veh->processNextStop(vPos);
    //
    //     //check whether the vehicle has collided and set the flag in case
    //     if (!vars->crashed) {
    //         for (const MSStop& s : veh->getStops()) {
    //             if (s.pars.collision) {
    //                 vars->crashed = true;
    //             }
    //         }
    //     }
    //
    //     if (vars->activeController != Plexe::DRIVER && !vars->useFixedAcceleration) {
    //         veh->setChosenSpeedFactor(vars->ccDesiredSpeed / veh->getLane()->getSpeedLimit());
    //     }
    //
    //     if (vars->platoonFixedLane >= 0) {
    //         performPlatoonLaneChange(veh);
    //     }
    //
    //     if (vars->autoLaneChange) {
    //         performAutoLaneChange(veh);
    //     }
    //
    //     if (vars->activeController != Plexe::DRIVER) {
    //         controllerAcceleration = SPEED2ACCEL(vPos - veh->getSpeed());
    //         controllerAcceleration = std::min(vars->uMax, std::max(vars->uMin, controllerAcceleration));
    //         //compute the actual acceleration applied by the engine
    //         engineAcceleration = vars->engine->getRealAcceleration(veh->getSpeed(), veh->getAcceleration(), controllerAcceleration, MSNet::getInstance()->getCurrentTimeStep());
    //         vNext = MAX2(double(0), veh->getSpeed() + ACCEL2SPEED(engineAcceleration));
    //         vars->controllerAcceleration = controllerAcceleration;
    //     } else {
    //         vNext = myHumanDriver->finalizeSpeed(veh, vPos);
    //     }
    //
    //     return vNext;
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        let vNext: f64;
        //acceleration computed by the controller
        let mut controllerAcceleration: f64;
        //acceleration after engine actuation
        let engineAcceleration: f64;

        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);

        //call processNextStop() to ensure vehicle removal in case of crash
        veh.m().process_next_stop(vPos);

        //check whether the vehicle has collided and set the flag in case
        if !vars.crashed {
            for s in veh.get_stops().iter() {
                if s.pars.collision {
                    vars.m().crashed = true;
                }
            }
        }

        if vars.activeController != ACTIVE_CONTROLLER::DRIVER && vars.useFixedAcceleration == 0 {
            veh.m().set_chosen_speed_factor(vars.ccDesiredSpeed / veh.get_lane().get_speed_limit());
        }

        if vars.platoonFixedLane >= 0 {
            self.perform_platoon_lane_change(veh);
        }

        if vars.autoLaneChange {
            self.perform_auto_lane_change(veh);
        }

        if vars.activeController != ACTIVE_CONTROLLER::DRIVER {
            controllerAcceleration = speed2accel(vPos - veh.get_speed());
            // std::min(a, b) == (b < a ? b : a); std::max(a, b) == (a < b ? b : a)
            let lower = if vars.uMin < controllerAcceleration { controllerAcceleration } else { vars.uMin };
            controllerAcceleration = if lower < vars.uMax { lower } else { vars.uMax };
            //compute the actual acceleration applied by the engine
            engineAcceleration = vars.engine.m().get_real_acceleration(veh.get_speed(), veh.get_acceleration(), controllerAcceleration, MSNet::get_instance().get_current_time_step());
            vNext = max2(0., veh.get_speed() + accel2speed(engineAcceleration));
            vars.m().controllerAcceleration = controllerAcceleration;
        } else {
            vNext = self.myHumanDriver.finalize_speed(veh, vPos);
        }

        vNext
    }

    /// C++: `double MSCFModel_CC::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_CC::followSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred, const CalcReason usage) const {
    //
    //     UNUSED_PARAMETER(pred);
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //
    //     if (vars->activeController != Plexe::DRIVER) {
    //         return _v(veh, gap2pred, speed, predSpeed);
    //     } else {
    //         return myHumanDriver->followSpeed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);

        if vars.activeController != ACTIVE_CONTROLLER::DRIVER {
            self.v(veh, gap2pred, speed, predSpeed)
        } else {
            self.myHumanDriver.follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage)
        }
    }

    /// C++: `double MSCFModel_CC::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Overload base MSCFModel::insertionFollowSpeed method to inject automated vehicles as soon as they are requested, without checking for safe speed constraints
    // double
    // MSCFModel_CC::insertionFollowSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const /*pred*/) const {
    //     UNUSED_PARAMETER(veh);
    //     UNUSED_PARAMETER(gap2pred);
    //     UNUSED_PARAMETER(predSpeed);
    //     UNUSED_PARAMETER(predMaxDecel);
    //     //by returning speed + 1, we tell sumo that "speed" is always a safe speed
    //     return speed + 1;
    // }
    #[allow(unused_variables)]
    pub fn insertion_follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
        //by returning speed + 1, we tell sumo that "speed" is always a safe speed
        speed + 1.
    }

    /// C++: `double MSCFModel_CC::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        if vars.activeController != ACTIVE_CONTROLLER::DRIVER {
            let mut gap2pred = gap2pred;
            let mut relSpeed = 0.;
            self.get_radar_measurements(veh, &mut gap2pred, &mut relSpeed);
            if gap2pred == -1. {
                gap2pred = f64::MAX;
            }
            self.v(veh, gap2pred, speed, speed + relSpeed)
        } else {
            self.myHumanDriver.stop_speed_2(veh, speed, gap2pred, decel, usage)
        }
    }

    /// C++: `double MSCFModel_CC::freeSpeed(const MSVehicle *const, double, double, double, const bool, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed without a leader
    // double MSCFModel_CC::freeSpeed(const MSVehicle* const veh, double speed, double seen, double maxSpeed, const bool onInsertion, const CalcReason usage) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     if (vars->activeController != Plexe::DRIVER) {
    //         double gap2pred, relSpeed;
    //         getRadarMeasurements(veh, gap2pred, relSpeed);
    //         if (gap2pred == -1) {
    //             gap2pred = std::numeric_limits<double>().max();
    //         }
    //         return _v(veh, gap2pred, speed, speed + relSpeed);
    //     } else {
    //         return MSCFModel::freeSpeed(veh, speed, seen, maxSpeed, onInsertion, usage);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn free_speed(&self, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        if vars.activeController != ACTIVE_CONTROLLER::DRIVER {
            let mut gap2pred = 0.;
            let mut relSpeed = 0.;
            self.get_radar_measurements(veh, &mut gap2pred, &mut relSpeed);
            if gap2pred == -1. {
                gap2pred = f64::MAX;
            }
            self.v(veh, gap2pred, speed, speed + relSpeed)
        } else {
            self.base.free_speed_base(veh, speed, seen, maxSpeed, onInsertion, usage)
        }
    }

    /// C++: `double MSCFModel_CC::maxNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    // double
    // MSCFModel_CC::maxNextSpeed(double speed, const MSVehicle* const veh) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     if (vars->engineModel == CC_ENGINE_MODEL_FOLM) {
    //         return speed + (double) ACCEL2SPEED(getMaxAccel());
    //     } else {
    //         return speed + (double) ACCEL2SPEED(20);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn max_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        if vars.engineModel == 0 /* CC_ENGINE_MODEL_FOLM */ {
            speed + accel2speed(self.get_max_accel())
        } else {
            speed + accel2speed(20.)
        }
    }

    /// C++: `double MSCFModel_CC::minNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    // double
    // MSCFModel_CC::minNextSpeed(double speed, const MSVehicle* const veh) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     if (vars->engineModel == CC_ENGINE_MODEL_FOLM) {
    //         return MSCFModel::minNextSpeed(speed, veh);
    //     } else {
    //         return MAX2((double)0, speed - (double) ACCEL2SPEED(20));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        if vars.engineModel == 0 /* CC_ENGINE_MODEL_FOLM */ {
            self.base.min_next_speed_base(speed, veh)
        } else {
            max2(0., speed - accel2speed(20.))
        }
    }

    /// C++: `double MSCFModel_CC::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_CC::interactionGap(const MSVehicle* const veh, double vL) const {
    //
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     if (vars->activeController != Plexe::DRIVER) {
    //         //maximum radar range is CC is enabled
    //         return 250;
    //     } else {
    //         return myHumanDriver->interactionGap(veh, vL);
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        let vars = Ptr::from_raw(arg0.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        if vars.activeController != ACTIVE_CONTROLLER::DRIVER {
            //maximum radar range is CC is enabled
            250.
        } else {
            self.myHumanDriver.interaction_gap(arg0, vL)
        }
    }

    /// C++: `int MSCFModel_CC::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_CC;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_CC as i32
    }

    /// C++: `MSCFModel * MSCFModel_CC::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_CC::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_CC(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_CC::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `VehicleVariables * MSCFModel_CC::createVehicleVariables() const` [const, virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        let mut vars = CC_VehicleVariables::new();
        vars.ccKp = self.myKp;
        vars.accLambda = self.myLambda;
        vars.caccSpacing = self.myConstantSpacing;
        vars.caccC1 = self.myC1;
        vars.caccXi = self.myXi;
        vars.caccOmegaN = self.myOmegaN;
        vars.engineTau = self.myTau;
        //we cannot invoke recomputeParameters() because we have no pointer to the MSVehicle class
        vars.caccAlpha1 = 1. - vars.caccC1;
        vars.caccAlpha2 = vars.caccC1;
        vars.caccAlpha3 = -(2. * vars.caccXi - vars.caccC1 * (vars.caccXi + (vars.caccXi * vars.caccXi - 1.).sqrt())) * vars.caccOmegaN;
        vars.caccAlpha4 = -(vars.caccXi + (vars.caccXi * vars.caccXi - 1.).sqrt()) * vars.caccOmegaN * vars.caccC1;
        vars.caccAlpha5 = -vars.caccOmegaN * vars.caccOmegaN;

        vars.ploegH = self.myPloegH;
        vars.ploegKp = self.myPloegKp;
        vars.ploegKd = self.myPloegKd;
        vars.flatbedKa = self.myFlatbedKa;
        vars.flatbedKv = self.myFlatbedKv;
        vars.flatbedKp = self.myFlatbedKp;
        vars.flatbedD = self.myFlatbedD;
        vars.flatbedH = self.myFlatbedH;
        //by default use a first order lag model for the engine
        let mut folm = crate::microsim::engine::first_order_lag_model::FirstOrderLagModel::new();
        folm.set_parameter_2("tau_s".to_string(), vars.engineTau);
        folm.set_parameter_2("dt_s".to_string(), ts());
        folm.base.set_maximum_acceleration(self.myAccel);
        folm.base.set_maximum_deceleration(self.myDecel);
        vars.engine = Ptr::from_raw(Box::into_raw(Box::new(folm)) as *mut crate::microsim::engine::generic_engine_model::GenericEngineModel);
        vars.engineModel = 0; // CC_ENGINE_MODEL_FOLM
        Ptr::from_raw(Box::into_raw(Box::new(vars)) as *mut MSCFModel_VehicleVariables)
    }

    /// C++: `double MSCFModel_CC::getCACCConstantSpacing(const MSVehicle *) const` [const]
    /// returns CACC desired constant spacing
    // double
    // MSCFModel_CC::getCACCConstantSpacing(const MSVehicle* veh) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     return vars->caccSpacing;
    // }
    #[allow(unused_variables)]
    pub fn get_cacc_constant_spacing(&self, veh: Ptr<MSVehicle>) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        vars.caccSpacing
    }

    /// C++: `std::string MSCFModel_CC::getParameter(const MSVehicle *, const std::string &) const` [const, virtual]
    /// try to get the given parameter for this carFollowingModel
    // std::string MSCFModel_CC::getParameter(const MSVehicle* veh, const std::string& key) const {
    //     // vehicle variables used to set the parameter
    //     CC_VehicleVariables* vars;
    //     ParBuffer buf;
    //
    //     vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     if (key.compare(PAR_SPEED_AND_ACCELERATION) == 0) {
    //         Position velocity = veh->getVelocityVector();
    //         buf << veh->getSpeed() << veh->getAcceleration() <<
    //             vars->controllerAcceleration << veh->getPosition().x() <<
    //             veh->getPosition().y() <<
    //             STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep()) <<
    //             velocity.x() << velocity.y() << veh->getAngle();
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_CRASHED) == 0) {
    //         return vars->crashed ? "1" : "0";
    //     }
    //     if (key.compare(PAR_RADAR_DATA) == 0) {
    //         double distance, relSpeed;
    //         getRadarMeasurements(veh, distance, relSpeed);
    //         buf << distance << relSpeed;
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_LANES_COUNT) == 0) {
    //         buf << veh->getLane()->getEdge().getLanes().size();
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_DISTANCE_TO_END) == 0) {
    //         //route of the vehicle
    //         ConstMSRoutePtr route = veh->getRoutePtr();
    //         //last lane of the route of this vehicle
    //         const MSLane* lastLane = route->getEdges().back()->getLanes()[0];
    //         //distance to trip end
    //         buf << route->getDistanceBetween(veh->getPositionOnLane(), lastLane->getLength(), veh->getLane(), lastLane);
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_DISTANCE_FROM_BEGIN) == 0) {
    //         //route of the vehicle
    //         ConstMSRoutePtr route = veh->getRoutePtr();
    //         //first lane of the route of this vehicle
    //         const MSLane* firstLane = route->getEdges().front()->getLanes()[0];
    //         //distance to trip start
    //         buf << route->getDistanceBetween(0., veh->getPositionOnLane(), firstLane, veh->getLane());
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_CC_DESIRED_SPEED) == 0) {
    //         buf << (double)vars->ccDesiredSpeed;
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_ACTIVE_CONTROLLER) == 0) {
    //         buf << (int)vars->activeController;
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_ACC_HEADWAY_TIME) == 0) {
    //         buf << (double)vars->accHeadwayTime;
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_ACC_ACCELERATION) == 0) {
    //         buf << getACCAcceleration(veh);
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_CACC_SPACING) == 0) {
    //         buf << vars->caccSpacing;
    //         return buf.str();
    //     }
    //     if (key.find(CC_PAR_VEHICLE_DATA) == 0) {
    //         ParBuffer inBuf(key);
    //         int index;
    //         inBuf >> index;
    //         struct Plexe::VEHICLE_DATA vehicle;
    //         if (index >= vars->nCars || index < 0) {
    //             vehicle.index = -1;
    //         } else {
    //             vehicle = vars->vehicles[index];
    //         }
    //         buf << vehicle.index << vehicle.speed << vehicle.acceleration <<
    //             vehicle.positionX << vehicle.positionY << vehicle.time <<
    //             vehicle.length << vehicle.u << vehicle.speedX <<
    //             vehicle.speedY << vehicle.angle;
    //         return buf.str();
    //     }
    //     if (key.compare(PAR_ENGINE_DATA) == 0) {
    //         int gear;
    //         double rpm;
    //         RealisticEngineModel* engine = dynamic_cast<RealisticEngineModel*>(vars->engine);
    //         if (engine) {
    //             engine->getEngineData(veh->getSpeed(), gear, rpm);
    //         } else {
    //             gear = -1;
    //             rpm = 0;
    //         }
    //         buf << (gear + 1) << rpm;
    //         return buf.str();
    //     }
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, veh: Ptr<MSVehicle>, key: &str) -> String {
        // vehicle variables used to set the parameter
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        let mut buf = ParBuffer::new();
        if key == PAR_SPEED_AND_ACCELERATION {
            let velocity = veh.get_velocity_vector();
            buf.shl_op(veh.get_speed()).shl_op(veh.get_acceleration()).shl_op(vars.controllerAcceleration)
                .shl_op(veh.get_position(0.).x()).shl_op(veh.get_position(0.).y())
                .shl_op(steps2time(MSNet::get_instance().get_current_time_step()))
                .shl_op(velocity.x()).shl_op(velocity.y()).shl_op(veh.get_angle());
            return buf.str();
        }
        if key == PAR_CRASHED {
            return if vars.crashed { "1" } else { "0" }.to_string();
        }
        if key == PAR_RADAR_DATA {
            let mut distance = 0.;
            let mut relSpeed = 0.;
            self.get_radar_measurements(veh, &mut distance, &mut relSpeed);
            buf.shl_op(distance).shl_op(relSpeed);
            return buf.str();
        }
        if key == PAR_LANES_COUNT {
            buf.shl_op(veh.get_lane().get_edge().get_lanes().len());
            return buf.str();
        }
        if key == PAR_DISTANCE_TO_END {
            // route of the vehicle
            let route = veh.get_route_ptr();
            // last lane of the route of this vehicle
            let lastLane = route.get_edges().last().unwrap().get_lanes()[0];
            // distance to trip end
            buf.shl_op(route.get_distance_between(veh.get_position_on_lane(), lastLane.get_length(), veh.get_lane(), lastLane, 0));
            return buf.str();
        }
        if key == PAR_DISTANCE_FROM_BEGIN {
            // route of the vehicle
            let route = veh.get_route_ptr();
            // first lane of the route of this vehicle
            let firstLane = route.get_edges().first().unwrap().get_lanes()[0];
            // distance to trip start
            buf.shl_op(route.get_distance_between(0., veh.get_position_on_lane(), firstLane, veh.get_lane(), 0));
            return buf.str();
        }
        if key == PAR_CC_DESIRED_SPEED {
            buf.shl_op(vars.ccDesiredSpeed);
            return buf.str();
        }
        if key == PAR_ACTIVE_CONTROLLER {
            buf.shl_op(vars.activeController as i32);
            return buf.str();
        }
        if key == PAR_ACC_HEADWAY_TIME {
            buf.shl_op(vars.accHeadwayTime);
            return buf.str();
        }
        if key == PAR_ACC_ACCELERATION {
            buf.shl_op(self.get_acc_acceleration(veh));
            return buf.str();
        }
        if key == PAR_CACC_SPACING {
            buf.shl_op(vars.caccSpacing);
            return buf.str();
        }
        if key.starts_with(CC_PAR_VEHICLE_DATA) {
            let mut inBuf = ParBuffer::new_2(key.to_string());
            let mut index: i32 = 0;
            inBuf.op_(&mut index);
            let vehicle = if index >= vars.nCars || index < 0 {
                // shortcut: C++ leaves the other fields uninitialized here, they are zeroed
                VEHICLE_DATA { index: -1, ..Default::default() }
            } else {
                vars.vehicles[index as usize]
            };
            buf.shl_op(vehicle.index).shl_op(vehicle.speed).shl_op(vehicle.acceleration)
                .shl_op(vehicle.positionX).shl_op(vehicle.positionY).shl_op(vehicle.time)
                .shl_op(vehicle.length).shl_op(vehicle.u).shl_op(vehicle.speedX)
                .shl_op(vehicle.speedY).shl_op(vehicle.angle);
            return buf.str();
        }
        if key == PAR_ENGINE_DATA {
            // shortcut: C++ dynamic_cast<RealisticEngineModel*>; the engine is created together with engineModel, so the model id stands in for the type check
            let (gear, rpm) = if vars.engineModel == CC_ENGINE_MODEL_REALISTIC {
                let mut gear = 0;
                let mut rpm = 0.;
                vars.engine.cast::<RealisticEngineModel>().m().get_engine_data(veh.get_speed(), &mut gear, &mut rpm);
                (gear, rpm)
            } else {
                (-1, 0.)
            };
            buf.shl_op(gear + 1).shl_op(rpm);
            return buf.str();
        }
        String::new()
    }

    /// C++: `void MSCFModel_CC::setParameter(MSVehicle *, const std::string &, const std::string &) const` [const, virtual]
    /// try to set the given parameter for this carFollowingModel
    // void MSCFModel_CC::setParameter(MSVehicle* veh, const std::string& key, const std::string& value) const {
    //     // vehicle variables used to set the parameter
    //     CC_VehicleVariables* vars;
    //
    //     ParBuffer buf(value);
    //
    //     vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     try {
    //         if (key.compare(PAR_LEADER_SPEED_AND_ACCELERATION) == 0) {
    //             double x, y, vx, vy;
    //             buf >> vars->leaderSpeed >> vars->leaderAcceleration >> x >> y >> vars->leaderDataReadTime
    //                 >> vars->leaderControllerAcceleration >> vx >> vy >> vars->leaderAngle;
    //             vars->leaderPosition = Position(x, y);
    //             vars->leaderVelocity = Position(vx, vy);
    //             vars->leaderInitialized = true;
    //             if (vars->frontInitialized) {
    //                 vars->caccInitialized = true;
    //             }
    //             return;
    //         }
    //         if (key.compare(PAR_PRECEDING_SPEED_AND_ACCELERATION) == 0) {
    //             double x, y, vx, vy;
    //             buf >> vars->frontSpeed >> vars->frontAcceleration >> x >> y >> vars->frontDataReadTime
    //                 >> vars->frontControllerAcceleration >> vx >> vy >> vars->frontAngle;
    //             vars->frontPosition = Position(x, y);
    //             vars->frontVelocity = Position(vx, vy);
    //             vars->frontInitialized = true;
    //             if (vars->leaderInitialized) {
    //                 vars->caccInitialized = true;
    //             }
    //             return;
    //         }
    //         if (key.compare(CC_PAR_VEHICLE_DATA) == 0) {
    //             struct Plexe::VEHICLE_DATA vehicle;
    //             buf >> vehicle.index >> vehicle.speed >> vehicle.acceleration >>
    //                 vehicle.positionX >> vehicle.positionY >> vehicle.time >>
    //                 vehicle.length >> vehicle.u >> vehicle.speedX >>
    //                 vehicle.speedY >> vehicle.angle;
    //             //if the index is larger than the number of cars, simply ignore the data
    //             if (vehicle.index >= vars->nCars || vehicle.index == -1) {
    //                 return;
    //             }
    //             vars->vehicles[vehicle.index] = vehicle;
    //             if (!vars->initialized[vehicle.index] && vehicle.index != vars->position) {
    //                 vars->nInitialized++;
    //             }
    //             vars->initialized[vehicle.index] = true;
    //             return;
    //         }
    //         if (key.compare(PAR_LEADER_FAKE_DATA) == 0) {
    //             buf >> vars->fakeData.leaderSpeed >> vars->fakeData.leaderAcceleration
    //                 >> vars->fakeData.leaderControllerAcceleration;
    //             if (buf.last_empty()) {
    //                 vars->useControllerAcceleration = false;
    //             }
    //             return;
    //         }
    //         if (key.compare(PAR_FRONT_FAKE_DATA) == 0) {
    //             buf >> vars->fakeData.frontSpeed >> vars->fakeData.frontAcceleration >> vars->fakeData.frontDistance
    //                 >> vars->fakeData.frontControllerAcceleration;
    //             if (buf.last_empty()) {
    //                 vars->useControllerAcceleration = false;
    //             }
    //             return;
    //         }
    //         if (key.compare(CC_PAR_VEHICLE_POSITION) == 0) {
    //             vars->position = StringUtils::toInt(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_PLATOON_SIZE) == 0) {
    //             vars->nCars = StringUtils::toInt(value.c_str());
    //             // given that we have a static matrix, check that we're not
    //             // setting a number of cars larger than the size of that matrix
    //             if (vars->nCars > MAX_N_CARS) {
    //                 vars->nCars = MAX_N_CARS;
    //                 std::stringstream warn;
    //                 warn << "MSCFModel_CC: setting a number of cars of " << vars->nCars << " out of a maximum of " << MAX_N_CARS <<
    //                      ". The CONSENSUS controller will not work properly if chosen. If you are using a different controller " <<
    //                      "you can ignore this warning";
    //                 WRITE_WARNING(warn.str());
    //             }
    //             return;
    //         }
    //         if (key.compare(PAR_ADD_MEMBER) == 0) {
    //             std::string id;
    //             int position;
    //             buf >> id >> position;
    //             vars->members[position] = id;
    //
    //             auto vehicle = findVehicle(id);
    //             if (!vehicle) {
    //                 throw libsumo::TraCIException("Adding " + id + " as member but " + id + " does not exists");
    //             }
    //             auto cfm = dynamic_cast<const MSCFModel_CC*>(&vehicle->getVehicleType().getCarFollowModel());
    //             if (!cfm) {
    //                 throw libsumo::TraCIException("Adding " + id + " as member but " + id + " is not using MSCFModel_CC");
    //             }
    //             cfm->setLeader(vehicle, veh, veh->getID());
    //             vars->isLeader = true;
    //             return;
    //         }
    //         if (key.compare(PAR_REMOVE_MEMBER) == 0) {
    //             for (auto item = vars->members.begin(); item != vars->members.end(); item++)
    //                 if (item->second.compare(value) == 0) {
    //                     auto vehicle = findVehicle(value);
    //                     if (!vehicle) {
    //                         throw libsumo::TraCIException("Removing " + value + " from members but " + value + " does not exist");
    //                     }
    //                     auto cfm = dynamic_cast<const MSCFModel_CC*>(&vehicle->getVehicleType().getCarFollowModel());
    //                     if (!cfm) {
    //                         throw libsumo::TraCIException("Removing " + value + " from members but " + value + " is not using MSCFModel_CC");
    //                     }
    //                     cfm->setLeader(vehicle, nullptr, "");
    //                     vars->members.erase(item);
    //                     break;
    //                 }
    //             return;
    //         }
    //         if (key.compare(PAR_ENABLE_AUTO_LANE_CHANGE) == 0) {
    //             vars->autoLaneChange = StringUtils::toInt(value.c_str()) == 1;
    //             // if the user enables automatic lane changing, any request for a fixed lane is deleted
    //             if (vars->autoLaneChange) {
    //                 vars->platoonFixedLane = -1;
    //             }
    //             return;
    //         }
    //         if (key.compare(PAR_PLATOON_FIXED_LANE) == 0) {
    //             vars->platoonFixedLane = StringUtils::toInt(value.c_str());
    //             // asking a leader to change the lane for the whole platoon automatically disables auto lane change
    //             if (vars->platoonFixedLane >= 0) {
    //                 vars->autoLaneChange = false;
    //             }
    //             return;
    //         }
    //         if (key.compare(CC_PAR_CACC_XI) == 0) {
    //             vars->caccXi = StringUtils::toDouble(value.c_str());
    //             recomputeParameters(veh);
    //             return;
    //         }
    //         if (key.compare(CC_PAR_CACC_OMEGA_N) == 0) {
    //             vars->caccOmegaN = StringUtils::toDouble(value.c_str());
    //             recomputeParameters(veh);
    //             return;
    //         }
    //         if (key.compare(CC_PAR_CACC_C1) == 0) {
    //             vars->caccC1 = StringUtils::toDouble(value.c_str());
    //             recomputeParameters(veh);
    //             return;
    //         }
    //         if (key.compare(CC_PAR_ENGINE_TAU) == 0) {
    //             vars->engineTau = StringUtils::toDouble(value.c_str());
    //             vars->engine->setParameter(FOLM_PAR_TAU, vars->engineTau);
    //             recomputeParameters(veh);
    //             vars->engine->setParameter(FOLM_PAR_TAU, vars->engineTau);
    //         }
    //         if (key.compare(CC_PAR_UMIN) == 0) {
    //             vars->uMin = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_UMAX) == 0) {
    //             vars->uMax = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_PLOEG_H) == 0) {
    //             vars->ploegH = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_PLOEG_KP) == 0) {
    //             vars->ploegKp = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_PLOEG_KD) == 0) {
    //             vars->ploegKd = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_FLATBED_KA) == 0) {
    //             vars->flatbedKa = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_FLATBED_KV) == 0) {
    //             vars->flatbedKv = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_FLATBED_KP) == 0) {
    //             vars->flatbedKp = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_FLATBED_H) == 0) {
    //             vars->flatbedH = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_FLATBED_D) == 0) {
    //             vars->flatbedD = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(CC_PAR_VEHICLE_ENGINE_MODEL) == 0) {
    //             if (vars->engine) {
    //                 delete vars->engine;
    //             }
    //             int engineModel = StringUtils::toInt(value.c_str());
    //             switch (engineModel) {
    //                 case CC_ENGINE_MODEL_REALISTIC: {
    //                     vars->engine = new RealisticEngineModel();
    //                     vars->engine->setParameter(ENGINE_PAR_DT, TS);
    //                     veh->getInfluencer().setSpeedMode(0);
    //                     vars->engineModel = CC_ENGINE_MODEL_REALISTIC;
    //                     break;
    //                 }
    //                 case CC_ENGINE_MODEL_FOLM:
    //                 default: {
    //                     vars->engine = new FirstOrderLagModel();
    //                     vars->engine->setParameter(FOLM_PAR_DT, TS);
    //                     vars->engine->setParameter(FOLM_PAR_TAU, vars->engineTau);
    //                     vars->engineModel = CC_ENGINE_MODEL_FOLM;
    //                     break;
    //                 }
    //             }
    //             vars->engine->setMaximumAcceleration(myAccel);
    //             vars->engine->setMaximumDeceleration(myDecel);
    //             return;
    //         }
    //         if (key.compare(CC_PAR_VEHICLE_MODEL) == 0) {
    //             vars->engine->setParameter(ENGINE_PAR_VEHICLE, value);
    //             return;
    //         }
    //         if (key.compare(CC_PAR_VEHICLES_FILE) == 0) {
    //             vars->engine->setParameter(ENGINE_PAR_XMLFILE, value);
    //             return;
    //         }
    //         if (key.compare(PAR_CACC_SPACING) == 0) {
    //             vars->caccSpacing = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(PAR_FIXED_ACCELERATION) == 0) {
    //             buf >> vars->useFixedAcceleration >> vars->fixedAcceleration;
    //             return;
    //         }
    //         if (key.compare(PAR_CC_DESIRED_SPEED) == 0) {
    //             vars->ccDesiredSpeed = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(PAR_ACTIVE_CONTROLLER) == 0) {
    //             vars->activeController = (enum Plexe::ACTIVE_CONTROLLER) StringUtils::toInt(value.c_str());
    //             return;
    //         }
    //         if (key.compare(PAR_ACC_HEADWAY_TIME) == 0) {
    //             vars->accHeadwayTime = StringUtils::toDouble(value.c_str());
    //             return;
    //         }
    //         if (key.compare(PAR_USE_CONTROLLER_ACCELERATION) == 0) {
    //             vars->useControllerAcceleration = StringUtils::toInt(value.c_str()) != 0;
    //             return;
    //         }
    //         if (key.compare(PAR_USE_AUTO_FEEDING) == 0) {
    //             int af;
    //             std::string leaderId, frontId;
    //             buf >> af;
    //             vars->autoFeed = af == 1;
    //             if (vars->autoFeed) {
    //                 vars->usePrediction = false;
    //                 buf >> leaderId;
    //                 if (buf.last_empty()) {
    //                     throw InvalidArgument("Trying to enable auto feeding without providing leader vehicle id");
    //                 }
    //                 vars->leaderVehicleId = leaderId;
    //                 vars->leaderVehicle = findVehicle(leaderId);
    //                 if (!vars->leaderVehicle) {
    //                     throw libsumo::TraCIException("Vehicle '" + leaderId + "' is not known");
    //                 }
    //                 buf >> frontId;
    //                 if (buf.last_empty()) {
    //                     throw InvalidArgument("Trying to enable auto feeding without providing front vehicle id");
    //                 }
    //                 vars->frontVehicleId = frontId;
    //                 vars->frontVehicle = findVehicle(frontId);
    //                 if (!vars->frontVehicle) {
    //                     throw libsumo::TraCIException("Vehicle '" + frontId + "' is not known");
    //                 }
    //                 vars->leaderInitialized = true;
    //                 vars->frontInitialized = true;
    //                 vars->caccInitialized = true;
    //             }
    //             return;
    //         }
    //         if (key.compare(PAR_USE_PREDICTION) == 0) {
    //             vars->usePrediction = StringUtils::toInt(value.c_str()) == 1;
    //             return;
    //         }
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Invalid value '" + value + "' for parameter '" + key + "' for vehicle '" + veh->getID() + "'");
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&self, veh: Ptr<MSVehicle>, key: &str, value: &str) -> Result<(), InvalidArgument> {
        // vehicle variables used to set the parameter
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        let mut buf = ParBuffer::new_2(value.to_string());
        // C++ catches NumberFormatException and rethrows InvalidArgument with this text
        let invalid_value = || InvalidArgument(format!("Invalid value '{}' for parameter '{}' for vehicle '{}'", value, key, veh.get_id()));
        let to_i = |s: &str| utils::common::string_utils::to_int(s).map_err(|_| invalid_value());
        let to_d = |s: &str| utils::common::string_utils::to_double(s).map_err(|_| invalid_value());
        // shortcut: C++ throws libsumo::TraCIException in the cases below, mapped to InvalidArgument with the same message
        if key == PAR_LEADER_SPEED_AND_ACCELERATION {
            let (mut x, mut y, mut vx, mut vy) = (0., 0., 0., 0.);
            buf.op_(&mut vars.m().leaderSpeed).op_(&mut vars.m().leaderAcceleration).op_(&mut x).op_(&mut y)
                .op_(&mut vars.m().leaderDataReadTime).op_(&mut vars.m().leaderControllerAcceleration)
                .op_(&mut vx).op_(&mut vy).op_(&mut vars.m().leaderAngle);
            vars.m().leaderPosition = Position::new(x, y);
            vars.m().leaderVelocity = Position::new(vx, vy);
            vars.m().leaderInitialized = true;
            if vars.frontInitialized {
                vars.m().caccInitialized = true;
            }
            return Ok(());
        }
        if key == PAR_PRECEDING_SPEED_AND_ACCELERATION {
            let (mut x, mut y, mut vx, mut vy) = (0., 0., 0., 0.);
            buf.op_(&mut vars.m().frontSpeed).op_(&mut vars.m().frontAcceleration).op_(&mut x).op_(&mut y)
                .op_(&mut vars.m().frontDataReadTime).op_(&mut vars.m().frontControllerAcceleration)
                .op_(&mut vx).op_(&mut vy).op_(&mut vars.m().frontAngle);
            vars.m().frontPosition = Position::new(x, y);
            vars.m().frontVelocity = Position::new(vx, vy);
            vars.m().frontInitialized = true;
            if vars.leaderInitialized {
                vars.m().caccInitialized = true;
            }
            return Ok(());
        }
        if key == CC_PAR_VEHICLE_DATA {
            let mut vehicle = VEHICLE_DATA::default();
            buf.op_(&mut vehicle.index).op_(&mut vehicle.speed).op_(&mut vehicle.acceleration)
                .op_(&mut vehicle.positionX).op_(&mut vehicle.positionY).op_(&mut vehicle.time)
                .op_(&mut vehicle.length).op_(&mut vehicle.u).op_(&mut vehicle.speedX)
                .op_(&mut vehicle.speedY).op_(&mut vehicle.angle);
            // if the index is larger than the number of cars, simply ignore the data
            // shortcut: C++ only rejects -1, other negative indices write out of bounds there; they are ignored here
            if vehicle.index >= vars.nCars || vehicle.index < 0 {
                return Ok(());
            }
            let idx = vehicle.index as usize;
            vars.m().vehicles[idx] = vehicle;
            if !vars.initialized[idx] && vehicle.index != vars.position {
                vars.m().nInitialized += 1;
            }
            vars.m().initialized[idx] = true;
            return Ok(());
        }
        if key == PAR_LEADER_FAKE_DATA {
            buf.op_(&mut vars.m().fakeData.leaderSpeed).op_(&mut vars.m().fakeData.leaderAcceleration)
                .op_(&mut vars.m().fakeData.leaderControllerAcceleration);
            if buf.last_empty() {
                vars.m().useControllerAcceleration = false;
            }
            return Ok(());
        }
        if key == PAR_FRONT_FAKE_DATA {
            buf.op_(&mut vars.m().fakeData.frontSpeed).op_(&mut vars.m().fakeData.frontAcceleration)
                .op_(&mut vars.m().fakeData.frontDistance).op_(&mut vars.m().fakeData.frontControllerAcceleration);
            if buf.last_empty() {
                vars.m().useControllerAcceleration = false;
            }
            return Ok(());
        }
        if key == CC_PAR_VEHICLE_POSITION {
            vars.m().position = to_i(value)?;
            return Ok(());
        }
        if key == CC_PAR_PLATOON_SIZE {
            vars.m().nCars = to_i(value)?;
            // given that we have a static matrix, check that we're not
            // setting a number of cars larger than the size of that matrix
            if vars.nCars > MAX_N_CARS {
                vars.m().nCars = MAX_N_CARS;
                utils::common::msg_handler::write_warning(&format!(
                    "MSCFModel_CC: setting a number of cars of {} out of a maximum of {}. The CONSENSUS controller will not work properly if chosen. If you are using a different controller you can ignore this warning",
                    vars.nCars, MAX_N_CARS
                ));
            }
            return Ok(());
        }
        if key == PAR_ADD_MEMBER {
            let mut id = String::new();
            let mut position: i32 = 0;
            buf.op_(&mut id).op_(&mut position);
            vars.m().members.insert(position, id.clone());
            let vehicle = self.find_vehicle(id.clone());
            if vehicle.is_null() {
                return Err(InvalidArgument(format!("Adding {} as member but {} does not exists", id, id)));
            }
            // shortcut: dynamic_cast<const MSCFModel_CC*> is not checked, MSCFModel has no runtime type id in this port
            self.set_leader(vehicle, veh, veh.get_id().to_string());
            vars.m().isLeader = true;
            return Ok(());
        }
        if key == PAR_REMOVE_MEMBER {
            let found = vars.members.iter().find(|(_, member)| member.as_str() == value).map(|(k, _)| *k);
            if let Some(k) = found {
                let vehicle = self.find_vehicle(value.to_string());
                if vehicle.is_null() {
                    return Err(InvalidArgument(format!("Removing {} from members but {} does not exist", value, value)));
                }
                // shortcut: dynamic_cast<const MSCFModel_CC*> is not checked, see PAR_ADD_MEMBER
                self.set_leader(vehicle, Ptr::null(), String::new());
                vars.m().members.remove(&k);
            }
            return Ok(());
        }
        if key == PAR_ENABLE_AUTO_LANE_CHANGE {
            vars.m().autoLaneChange = to_i(value)? == 1;
            // if the user enables automatic lane changing, any request for a fixed lane is deleted
            if vars.autoLaneChange {
                vars.m().platoonFixedLane = -1;
            }
            return Ok(());
        }
        if key == PAR_PLATOON_FIXED_LANE {
            vars.m().platoonFixedLane = to_i(value)?;
            // asking a leader to change the lane for the whole platoon automatically disables auto lane change
            if vars.platoonFixedLane >= 0 {
                vars.m().autoLaneChange = false;
            }
            return Ok(());
        }
        if key == CC_PAR_CACC_XI {
            vars.m().caccXi = to_d(value)?;
            self.recompute_parameters(veh);
            return Ok(());
        }
        if key == CC_PAR_CACC_OMEGA_N {
            vars.m().caccOmegaN = to_d(value)?;
            self.recompute_parameters(veh);
            return Ok(());
        }
        if key == CC_PAR_CACC_C1 {
            vars.m().caccC1 = to_d(value)?;
            self.recompute_parameters(veh);
            return Ok(());
        }
        if key == CC_PAR_ENGINE_TAU {
            vars.m().engineTau = to_d(value)?;
            vars.engine.m().set_parameter_2(FOLM_PAR_TAU.to_string(), vars.engineTau);
            self.recompute_parameters(veh);
            vars.engine.m().set_parameter_2(FOLM_PAR_TAU.to_string(), vars.engineTau);
            return Ok(());
        }
        if key == CC_PAR_UMIN {
            vars.m().uMin = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_UMAX {
            vars.m().uMax = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_PLOEG_H {
            vars.m().ploegH = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_PLOEG_KP {
            vars.m().ploegKp = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_PLOEG_KD {
            vars.m().ploegKd = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_FLATBED_KA {
            vars.m().flatbedKa = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_FLATBED_KV {
            vars.m().flatbedKv = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_FLATBED_KP {
            vars.m().flatbedKp = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_FLATBED_H {
            vars.m().flatbedH = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_FLATBED_D {
            vars.m().flatbedD = to_d(value)?;
            return Ok(());
        }
        if key == CC_PAR_VEHICLE_ENGINE_MODEL {
            // shortcut: C++ deletes the previous engine; it is leaked here because Ptr<GenericEngineModel> does not know its concrete type
            let engineModel = to_i(value)?;
            if engineModel == CC_ENGINE_MODEL_REALISTIC {
                let mut engine = RealisticEngineModel::new();
                engine.set_parameter_2(ENGINE_PAR_DT.to_string(), ts());
                veh.m().get_influencer().set_speed_mode(0);
                vars.m().engine = Ptr::from_raw(Box::into_raw(Box::new(engine)) as *mut GenericEngineModel);
                vars.m().engineModel = CC_ENGINE_MODEL_REALISTIC;
            } else {
                // CC_ENGINE_MODEL_FOLM and the default case
                let mut engine = FirstOrderLagModel::new();
                engine.set_parameter_2(FOLM_PAR_DT.to_string(), ts());
                engine.set_parameter_2(FOLM_PAR_TAU.to_string(), vars.engineTau);
                vars.m().engine = Ptr::from_raw(Box::into_raw(Box::new(engine)) as *mut GenericEngineModel);
                vars.m().engineModel = CC_ENGINE_MODEL_FOLM;
            }
            vars.engine.m().set_maximum_acceleration(self.myAccel);
            vars.engine.m().set_maximum_deceleration(self.myDecel);
            return Ok(());
        }
        if key == CC_PAR_VEHICLE_MODEL {
            vars.engine.m().set_parameter(ENGINE_PAR_VEHICLE.to_string(), value);
            return Ok(());
        }
        if key == CC_PAR_VEHICLES_FILE {
            vars.engine.m().set_parameter(ENGINE_PAR_XMLFILE.to_string(), value);
            return Ok(());
        }
        if key == PAR_CACC_SPACING {
            vars.m().caccSpacing = to_d(value)?;
            return Ok(());
        }
        if key == PAR_FIXED_ACCELERATION {
            buf.op_(&mut vars.m().useFixedAcceleration).op_(&mut vars.m().fixedAcceleration);
            return Ok(());
        }
        if key == PAR_CC_DESIRED_SPEED {
            vars.m().ccDesiredSpeed = to_d(value)?;
            return Ok(());
        }
        if key == PAR_ACTIVE_CONTROLLER {
            // shortcut: C++ casts any int to the enum; only the seven known controllers exist here, other values keep the current one
            let controller = match to_i(value)? {
                0 => Some(ACTIVE_CONTROLLER::DRIVER),
                1 => Some(ACTIVE_CONTROLLER::ACC),
                2 => Some(ACTIVE_CONTROLLER::CACC),
                3 => Some(ACTIVE_CONTROLLER::FAKED_CACC),
                4 => Some(ACTIVE_CONTROLLER::PLOEG),
                5 => Some(ACTIVE_CONTROLLER::CONSENSUS),
                6 => Some(ACTIVE_CONTROLLER::FLATBED),
                _ => None,
            };
            if let Some(controller) = controller {
                vars.m().activeController = controller;
            }
            return Ok(());
        }
        if key == PAR_ACC_HEADWAY_TIME {
            vars.m().accHeadwayTime = to_d(value)?;
            return Ok(());
        }
        if key == PAR_USE_CONTROLLER_ACCELERATION {
            vars.m().useControllerAcceleration = to_i(value)? != 0;
            return Ok(());
        }
        if key == PAR_USE_AUTO_FEEDING {
            let mut af: i32 = 0;
            let mut leaderId = String::new();
            let mut frontId = String::new();
            buf.op_(&mut af);
            vars.m().autoFeed = af == 1;
            if vars.autoFeed {
                vars.m().usePrediction = false;
                buf.op_(&mut leaderId);
                if buf.last_empty() {
                    return Err(InvalidArgument("Trying to enable auto feeding without providing leader vehicle id".to_string()));
                }
                vars.m().leaderVehicleId = leaderId.clone();
                vars.m().leaderVehicle = self.find_vehicle(leaderId.clone());
                if vars.leaderVehicle.is_null() {
                    return Err(InvalidArgument(format!("Vehicle '{}' is not known", leaderId)));
                }
                buf.op_(&mut frontId);
                if buf.last_empty() {
                    return Err(InvalidArgument("Trying to enable auto feeding without providing front vehicle id".to_string()));
                }
                vars.m().frontVehicleId = frontId.clone();
                vars.m().frontVehicle = self.find_vehicle(frontId.clone());
                if vars.frontVehicle.is_null() {
                    return Err(InvalidArgument(format!("Vehicle '{}' is not known", frontId)));
                }
                vars.m().leaderInitialized = true;
                vars.m().frontInitialized = true;
                vars.m().caccInitialized = true;
            }
            return Ok(());
        }
        if key == PAR_USE_PREDICTION {
            vars.m().usePrediction = to_i(value)? == 1;
            return Ok(());
        }
        Ok(())
    }

    /// C++: `void MSCFModel_CC::getVehicleInformation(const MSVehicle *, double &, double &, double &, Position &, double &) const` [const]
    /// get the information about a vehicle. This can be used by TraCI in order to get speed and acceleration of the platoon leader before sending them to other vehicles
    // void
    // MSCFModel_CC::getVehicleInformation(const MSVehicle* veh, double& speed, double& acceleration, double& controllerAcceleration, Position& position, double& time) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     speed = veh->getSpeed();
    //     acceleration = veh->getAcceleration();
    //     controllerAcceleration = vars->controllerAcceleration;
    //     position = veh->getPosition();
    //     time = STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_information(&self, veh: Ptr<MSVehicle>, speed: &mut f64, acceleration: &mut f64, controllerAcceleration: &mut f64, position: &mut Position, time: &mut f64) {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        *speed = veh.get_speed();
        *acceleration = veh.get_acceleration();
        *controllerAcceleration = vars.controllerAcceleration;
        *position = veh.get_position(0.);
        *time = utils::common::time::steps2time(MSNet::get_instance().get_current_time_step());
    }

    /// C++: `void MSCFModel_CC::switchOnACC(const MSVehicle *, double) const` [const]
    /// switch on the ACC, so disabling the human driver car control
    // void MSCFModel_CC::switchOnACC(const MSVehicle* veh, double ccDesiredSpeed)  const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     vars->ccDesiredSpeed = ccDesiredSpeed;
    //     vars->activeController = Plexe::ACC;
    // }
    #[allow(unused_variables)]
    pub fn switch_on_acc(&self, veh: Ptr<MSVehicle>, ccDesiredSpeed: f64) {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        vars.m().ccDesiredSpeed = ccDesiredSpeed;
        vars.m().activeController = ACTIVE_CONTROLLER::ACC;
    }

    /// C++: `enum Plexe::ACTIVE_CONTROLLER MSCFModel_CC::getActiveController(const MSVehicle *) const` [const]
    /// return the currently active controller
    // enum Plexe::ACTIVE_CONTROLLER MSCFModel_CC::getActiveController(const MSVehicle* veh) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     return vars->activeController;
    // }
    #[allow(unused_variables)]
    pub fn get_active_controller(&self, veh: Ptr<MSVehicle>) -> ACTIVE_CONTROLLER {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        vars.activeController
    }

    /// C++: `void MSCFModel_CC::getRadarMeasurements(const MSVehicle *, double &, double &) const` [const]
    /// return the data that is currently being measured by the radar
    // void MSCFModel_CC::getRadarMeasurements(const MSVehicle* veh, double& distance, double& relativeSpeed) const {
    //     std::pair<std::string, double> l = libsumo::Vehicle::getLeader(veh->getID(), 250);
    //     if (l.second < 0) {
    //         distance = -1;
    //         relativeSpeed = 0;
    //     } else {
    //         distance = l.second;
    //         SUMOVehicle* leader = findVehicle(l.first);
    //         relativeSpeed = leader->getSpeed() - veh->getSpeed();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_radar_measurements(&self, veh: Ptr<MSVehicle>, distance: &mut f64, relativeSpeed: &mut f64) {
        let l = crate::libsumo::vehicle::Vehicle::get_leader(&veh.get_id(), 250.);
        if l.1 < 0. {
            *distance = -1.;
            *relativeSpeed = 0.;
        } else {
            *distance = l.1;
            let leader = self.find_vehicle(l.0.clone());
            *relativeSpeed = leader.get_speed() - veh.get_speed();
        }
    }

    /// C++: `double MSCFModel_CC::getACCAcceleration(const MSVehicle *) const` [const]
    /// returns the ACC computed acceleration when the faked CACC is controlling the car. This can be used to check for vehicles in front
    // double MSCFModel_CC::getACCAcceleration(const MSVehicle* veh) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     double distance, relSpeed;
    //     getRadarMeasurements(veh, distance, relSpeed);
    //     if (distance < 0) {
    //         return 0;
    //     } else {
    //         return _acc(veh, veh->getSpeed(), relSpeed + veh->getSpeed(), distance, vars->accHeadwayTime);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_acc_acceleration(&self, veh: Ptr<MSVehicle>) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        let mut distance = 0.;
        let mut relSpeed = 0.;
        self.get_radar_measurements(veh, &mut distance, &mut relSpeed);
        if distance < 0. {
            0.
        } else {
            self.acc(veh, veh.get_speed(), relSpeed + veh.get_speed(), distance, vars.accHeadwayTime)
        }
    }

    /// C++: `int MSCFModel_CC::isPlatoonLaneChangeSafe(const MSVehicle *, bool) const` [const]
    /// computes whether a lane change for a whole platoon is safe or not. This is done by checking the lane change state and neighbors of all vehicles in the platoon added through the addPlatoonMember API. If the adjacent lane is free and there is enough safe gap for the platoon, the method returns 0, otherwise it returns the blocking reason
    // int
    // MSCFModel_CC::isPlatoonLaneChangeSafe(const MSVehicle* veh, bool left) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     if (!vars->isLeader) {
    //         // before asking the leader, be sure it is still in the simulation
    //         if (findVehicle(vars->leaderVehicleId)) {
    //             return isPlatoonLaneChangeSafe(vars->leaderVehicle, left);
    //         } else {
    //             return LCA_BLOCKED;
    //         }
    //     }
    //     int result = 0;
    //     std::pair<int, int> state = libsumo::Vehicle::getLaneChangeState(veh->getID(), left ? +1 : -1);
    //     // bit 1: query lateral direction (left:0, right:1)
    //     // bit 2: query longitudinal direction (followers:0, leaders:1)
    //     // bit 3: blocking (return all:0, return only blockers:1)
    //     auto followers = libsumo::Vehicle::getNeighbors(veh->getID(), left ? 0b100 : 0b101);
    //     auto leaders = libsumo::Vehicle::getNeighbors(veh->getID(), left ? 0b110 : 0b111);
    //     bool noNeighbors = followers.empty() && leaders.empty();
    //     if (!(state.second & LCA_BLOCKED) && noNeighbors) {
    //         // leader is not blocked. check all the members
    //         for (auto m = vars->members.begin(); m != vars->members.end(); m++) {
    //             const std::pair<int, int> mState = libsumo::Vehicle::getLaneChangeState(m->second, left ? +1 : -1);
    //             followers = libsumo::Vehicle::getNeighbors(m->second, left ? 0b100 : 0b101);
    //             leaders = libsumo::Vehicle::getNeighbors(m->second, left ? 0b110 : 0b111);
    //             noNeighbors = followers.empty() && leaders.empty();
    //             if (mState.second & LCA_BLOCKED || !noNeighbors) {
    //                 if (mState.second & LCA_BLOCKED) {
    //                     result = mState.second;
    //                 } else {
    //                     if (!followers.empty()) {
    //                         result |= left ? LCA_BLOCKED_BY_LEFT_FOLLOWER : LCA_BLOCKED_BY_RIGHT_FOLLOWER;
    //                     }
    //                     if (!leaders.empty()) {
    //                         result |= left ? LCA_BLOCKED_BY_LEFT_LEADER : LCA_BLOCKED_BY_RIGHT_LEADER;
    //                     }
    //                 }
    //                 break;
    //             }
    //         }
    //     } else {
    //         if (state.second & LCA_BLOCKED) {
    //             result = state.second;
    //         } else {
    //             if (!followers.empty()) {
    //                 result |= left ? LCA_BLOCKED_BY_LEFT_FOLLOWER : LCA_BLOCKED_BY_RIGHT_FOLLOWER;
    //             }
    //             if (!leaders.empty()) {
    //                 result |= left ? LCA_BLOCKED_BY_LEFT_LEADER : LCA_BLOCKED_BY_RIGHT_LEADER;
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn is_platoon_lane_change_safe(&self, veh: Ptr<MSVehicle>, left: bool) -> i32 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        if !vars.isLeader {
            // before asking the leader, be sure it is still in the simulation
            if !self.find_vehicle(vars.leaderVehicleId.clone()).is_null() {
                return self.is_platoon_lane_change_safe(vars.leaderVehicle, left);
            } else {
                return LCA_BLOCKED;
            }
        }
        let mut result: i32 = 0;
        let id = veh.get_id();
        let state = crate::libsumo::vehicle::Vehicle::get_lane_change_state(&id, if left { 1 } else { -1 });
        // bit 1: query lateral direction (left:0, right:1)
        // bit 2: query longitudinal direction (followers:0, leaders:1)
        // bit 3: blocking (return all:0, return only blockers:1)
        let mut followers = crate::libsumo::vehicle::Vehicle::get_neighbors(&id, if left { 0b100 } else { 0b101 });
        let mut leaders = crate::libsumo::vehicle::Vehicle::get_neighbors(&id, if left { 0b110 } else { 0b111 });
        let mut noNeighbors = followers.is_empty() && leaders.is_empty();
        if (state.1 & LCA_BLOCKED) == 0 && noNeighbors {
            // leader is not blocked. check all the members
            for (_, memberId) in vars.members.iter() {
                let mState = crate::libsumo::vehicle::Vehicle::get_lane_change_state(memberId, if left { 1 } else { -1 });
                followers = crate::libsumo::vehicle::Vehicle::get_neighbors(memberId, if left { 0b100 } else { 0b101 });
                leaders = crate::libsumo::vehicle::Vehicle::get_neighbors(memberId, if left { 0b110 } else { 0b111 });
                noNeighbors = followers.is_empty() && leaders.is_empty();
                if (mState.1 & LCA_BLOCKED) != 0 || !noNeighbors {
                    if (mState.1 & LCA_BLOCKED) != 0 {
                        result = mState.1;
                    } else {
                        if !followers.is_empty() {
                            result |= if left { LCA_BLOCKED_BY_LEFT_FOLLOWER } else { LCA_BLOCKED_BY_RIGHT_FOLLOWER };
                        }
                        if !leaders.is_empty() {
                            result |= if left { LCA_BLOCKED_BY_LEFT_LEADER } else { LCA_BLOCKED_BY_RIGHT_LEADER };
                        }
                    }
                    break;
                }
            }
        } else {
            if (state.1 & LCA_BLOCKED) != 0 {
                result = state.1;
            } else {
                if !followers.is_empty() {
                    result |= if left { LCA_BLOCKED_BY_LEFT_FOLLOWER } else { LCA_BLOCKED_BY_RIGHT_FOLLOWER };
                }
                if !leaders.is_empty() {
                    result |= if left { LCA_BLOCKED_BY_LEFT_LEADER } else { LCA_BLOCKED_BY_RIGHT_LEADER };
                }
            }
        }
        result
    }

    /// C++: `void MSCFModel_CC::setLeader(MSVehicle *, MSVehicle *const, std::string) const` [const]
    /// Sets the leader for a member of the platoon
    // void
    // MSCFModel_CC::setLeader(MSVehicle* veh, MSVehicle* const leader, std::string leaderId) const {
    //     auto* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     vars->leaderVehicle = leader;
    //     vars->leaderVehicleId = leaderId;
    //     if (leader != nullptr) {
    //         vars->isLeader = false;
    //     } else
    //         // if we are removing our leader, then this vehicle must become a leader of itself until being member of another platoon
    //     {
    //         vars->isLeader = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_leader(&self, veh: Ptr<MSVehicle>, leader: Ptr<MSVehicle>, leaderId: String) {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        vars.m().leaderVehicle = leader;
        vars.m().leaderVehicleId = leaderId;
        if !leader.is_null() {
            vars.m().isLeader = false;
        } else {
            // if we are removing our leader, then this vehicle must become a leader of itself until being member of another platoon
            vars.m().isLeader = true;
        }
    }

    /// C++: `bool MSCFModel_CC::isLeader(const MSVehicle *) const` [const]
    /// Returns whether a vehicle is a leader of a platoon or not. By default, a vehicle on its own using an ACC is a leader of itself
    // bool
    // MSCFModel_CC::isLeader(const MSVehicle* veh) const {
    //     auto vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     return vars->isLeader;
    // }
    #[allow(unused_variables)]
    pub fn is_leader(&self, veh: Ptr<MSVehicle>) -> bool {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        vars.isLeader
    }

    /// C++: `int MSCFModel_CC::commitToLaneChange(const MSVehicle *, bool) const` [const]
    /// This method can be invoked by the lane change model to do a final check on the safety of a platoon lane change maneuver. This method will cause the leader to check once again the safety status of all members. This might have changed since the decision of changing lane because two platoons might have decided to change lane exactly at the same time, when the lane was free for both, but changing lane would cause a side-to-side collision. SUMO will call this method on one platoon which will answer "still safe" and cause SUMO to start moving the vehicles on the adjacent lane. On the second platoon, calling this method will cause the platoon to find out about the vehicles that have just been moved and indicate the lane as busy, aborting the lane change.
    // int
    // MSCFModel_CC::commitToLaneChange(const MSVehicle* veh, bool left) const {
    //     auto vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     if (isLeader(veh)) {
    //         SUMOTime timestep = MSNet::getInstance()->getCurrentTimeStep();
    //         if (vars->laneChangeCommitTime == timestep) {
    //             if (vars->commitToLaneChange) {
    //                 return 0;
    //             } else {
    //                 return vars->noCommitReason;
    //             }
    //         } else {
    //             int blocked = isPlatoonLaneChangeSafe(veh, left);
    //             if (blocked == 0) {
    //                 vars->commitToLaneChange = true;
    //                 vars->laneChangeCommitTime = timestep;
    //             }
    //             return blocked;
    //         }
    //     } else {
    //         // before asking the leader, be sure it is still in the simulation
    //         if (findVehicle(vars->leaderVehicleId)) {
    //             return commitToLaneChange(vars->leaderVehicle, left);
    //         } else {
    //             return LCA_BLOCKED;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn commit_to_lane_change(&self, veh: Ptr<MSVehicle>, left: bool) -> i32 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        if self.is_leader(veh) {
            let timestep = MSNet::get_instance().get_current_time_step();
            if vars.laneChangeCommitTime == timestep {
                if vars.commitToLaneChange {
                    0
                } else {
                    vars.noCommitReason
                }
            } else {
                let blocked = self.is_platoon_lane_change_safe(veh, left);
                if blocked == 0 {
                    vars.m().commitToLaneChange = true;
                    vars.m().laneChangeCommitTime = timestep;
                }
                blocked
            }
        } else {
            // before asking the leader, be sure it is still in the simulation
            if !self.find_vehicle(vars.leaderVehicleId.clone()).is_null() {
                self.commit_to_lane_change(vars.leaderVehicle, left)
            } else {
                LCA_BLOCKED
            }
        }
    }

    /// C++: `MSVehicle * MSCFModel_CC::findVehicle(std::string) const` [const]
    /// Searches for a vehicle given its sumo id. Differently from libsumo's getVehicle, this call does not throw an exception if the vehicle does not exist, so we can check for its existence
    // MSVehicle*
    // MSCFModel_CC::findVehicle(std::string id) const {
    //     return dynamic_cast<MSVehicle*>(MSNet::getInstance()->getVehicleControl().getVehicle(id));
    // }
    #[allow(unused_variables)]
    pub fn find_vehicle(&self, id: String) -> Ptr<MSVehicle> {
        // shortcut: C++ dynamic_cast is a plain cast here, the vehicle control only holds MSVehicle objects
        MSNet::get_instance().m().get_vehicle_control().get_vehicle(&id).cast::<MSVehicle>()
    }

    /// C++: `int MSCFModel_CC::getMyLanesCount() const` [const]
    /// returns the number of lanes set in the configuration file
    // int MSCFModel_CC::getMyLanesCount() const {
    //     return myLanesCount;
    // }
    #[allow(unused_variables)]
    pub fn get_my_lanes_count(&self) -> i32 {
        self.myLanesCount
    }

    /// C++: `void MSCFModel_CC::recomputeParameters(const MSVehicle *) const` [const, private]
    /// Recomputes controller related parameters after setting them
    // void MSCFModel_CC::recomputeParameters(const MSVehicle* veh) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     vars->caccAlpha1 = 1 - vars->caccC1;
    //     vars->caccAlpha2 = vars->caccC1;
    //     vars->caccAlpha3 = -(2 * vars->caccXi - vars->caccC1 * (vars->caccXi + sqrt(vars->caccXi * vars->caccXi - 1))) * vars->caccOmegaN;
    //     vars->caccAlpha4 = -(vars->caccXi + sqrt(vars->caccXi * vars->caccXi - 1)) * vars->caccOmegaN * vars->caccC1;
    //     vars->caccAlpha5 = -vars->caccOmegaN * vars->caccOmegaN;
    // }
    #[allow(unused_variables)]
    pub fn recompute_parameters(&self, veh: Ptr<MSVehicle>) {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        vars.m().caccAlpha1 = 1. - vars.caccC1;
        vars.m().caccAlpha2 = vars.caccC1;
        vars.m().caccAlpha3 = -(2. * vars.caccXi - vars.caccC1 * (vars.caccXi + (vars.caccXi * vars.caccXi - 1.).sqrt())) * vars.caccOmegaN;
        vars.m().caccAlpha4 = -(vars.caccXi + (vars.caccXi * vars.caccXi - 1.).sqrt()) * vars.caccOmegaN * vars.caccC1;
        vars.m().caccAlpha5 = -vars.caccOmegaN * vars.caccOmegaN;
    }

    /// C++: `void MSCFModel_CC::resetConsensus(const MSVehicle *) const` [const, private]
    /// Resets the consensus controller. In particular, sets the "initialized" vector all to false. This might be useful when changing topology.
    // void MSCFModel_CC::resetConsensus(const MSVehicle* veh) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     for (int i = 0; i < MAX_N_CARS; i++) {
    //         vars->initialized[i] = false;
    //         vars->nInitialized = 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_consensus(&self, veh: Ptr<MSVehicle>) {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        for i in 0..8usize { // MAX_N_CARS == 8, the size of the per-car arrays
            vars.m().initialized[i] = false;
            vars.m().nInitialized = 0;
        }
    }

    /// C++: `void MSCFModel_CC::changeWholePlatoonLane(MSVehicle *const, int) const` [const, private]
    /// Moves an entire platoon on an adjacent lane, calling changeLane() on all members
    // void
    // MSCFModel_CC::changeWholePlatoonLane(MSVehicle* const veh, int direction) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     libsumo::Vehicle::changeLane(veh->getID(), veh->getLaneIndex() + direction, 0);
    //     for (auto& member : vars->members) {
    //         libsumo::Vehicle::changeLane(member.second, veh->getLaneIndex() + direction, 0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn change_whole_platoon_lane(&self, veh: Ptr<MSVehicle>, direction: i32) {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        crate::libsumo::vehicle::Vehicle::change_lane(&veh.get_id(), veh.get_lane_index() + direction, 0.);
        for member in vars.members.values() {
            crate::libsumo::vehicle::Vehicle::change_lane(member, veh.get_lane_index() + direction, 0.);
        }
    }

    /// C++: `void MSCFModel_CC::performAutoLaneChange(MSVehicle *const) const` [const, private]
    /// Check whether a platoon would gain speed by moving to the left or whether it should move to the right after an overtake and, if safe, do so. This method is invoked when the platoon auto lane change mechanism is enabled through the enableAutoLaneChanging API
    // void
    // MSCFModel_CC::performAutoLaneChange(MSVehicle* const veh) const {
    //     // check for left lane change
    //     std::pair<int, int> state = libsumo::Vehicle::getLaneChangeState(veh->getID(), +1);
    //     int traciState = state.first;
    //     if (traciState & LCA_LEFT && traciState & LCA_SPEEDGAIN) {
    //         // we can gain by moving left. check that all vehicles can move left
    //         if (isPlatoonLaneChangeSafe(veh, true) == 0) {
    //             changeWholePlatoonLane(veh, +1);
    //         }
    //     }
    //     // check for right lane change
    //     state = libsumo::Vehicle::getLaneChangeState(veh->getID(), -1);
    //     traciState = state.first;
    //     if (traciState & LCA_RIGHT && traciState & LCA_KEEPRIGHT) {
    //         // we should move back right. check that all vehicles can move right
    //         if (isPlatoonLaneChangeSafe(veh, false) == 0) {
    //             changeWholePlatoonLane(veh, -1);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn perform_auto_lane_change(&self, veh: Ptr<MSVehicle>) {
        // check for left lane change
        let id = veh.get_id();
        let state = crate::libsumo::vehicle::Vehicle::get_lane_change_state(&id, 1);
        let mut traciState = state.0;
        if traciState & LCA_LEFT != 0 && traciState & LCA_SPEEDGAIN != 0 {
            // we can gain by moving left. check that all vehicles can move left
            if self.is_platoon_lane_change_safe(veh, true) == 0 {
                self.change_whole_platoon_lane(veh, 1);
            }
        }
        // check for right lane change
        let state = crate::libsumo::vehicle::Vehicle::get_lane_change_state(&id, -1);
        traciState = state.0;
        if traciState & LCA_RIGHT != 0 && traciState & LCA_KEEPRIGHT != 0 {
            // we should move back right. check that all vehicles can move right
            if self.is_platoon_lane_change_safe(veh, false) == 0 {
                self.change_whole_platoon_lane(veh, -1);
            }
        }
    }

    /// C++: `void MSCFModel_CC::performPlatoonLaneChange(MSVehicle *const) const` [const, private]
    /// If safe to do so, moves a platoon to a user-desired lane. If not safe, this method continues to try at each simulation step. This method is invoked when the user requests a platoon to change lane through the performPlatoonLaneChange API, or when invoking the old setFixedLane API on a single vehicle
    // void
    // MSCFModel_CC::performPlatoonLaneChange(MSVehicle* const veh) const {
    //     auto* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     int currentLane = veh->getLaneIndex();
    //     if (currentLane == vars->platoonFixedLane) {
    //         // platoon has already reached the desired lane
    //         vars->platoonFixedLane = -1;
    //         return;
    //     }
    //     bool left = currentLane < vars->platoonFixedLane;
    //     if (isPlatoonLaneChangeSafe(veh, left) == 0) {
    //         changeWholePlatoonLane(veh, left ? +1 : -1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn perform_platoon_lane_change(&self, veh: Ptr<MSVehicle>) {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        let currentLane = veh.get_lane_index();
        if currentLane == vars.platoonFixedLane {
            // platoon has already reached the desired lane
            vars.m().platoonFixedLane = -1;
            return;
        }
        let left = currentLane < vars.platoonFixedLane;
        if self.is_platoon_lane_change_safe(veh, left) == 0 {
            self.change_whole_platoon_lane(veh, if left { 1 } else { -1 });
        }
    }

    /// C++: `double MSCFModel_CC::_v(const MSVehicle *const, double, double, double) const` [const, private]
    // double
    // MSCFModel_CC::_v(const MSVehicle* const veh, double gap2pred, double egoSpeed, double predSpeed) const {
    //
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //
    //     //acceleration computed by the controller
    //     double controllerAcceleration = vars->fixedAcceleration;
    //     //speed computed by the model
    //     double speed;
    //     //acceleration computed by the Cruise Control
    //     double ccAcceleration;
    //     //acceleration computed by the Adaptive Cruise Control
    //     double accAcceleration;
    //     //acceleration computed by the Cooperative Adaptive Cruise Control
    //     double caccAcceleration;
    //     //variables needed by CACC
    //     double predAcceleration, leaderAcceleration, leaderSpeed;
    //     //dummy variables used for auto feeding
    //     Position pos;
    //     double time;
    //     const double currentTime = STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep() + DELTA_T);
    //
    //     if (vars->crashed) {
    //         return 0;
    //     }
    //     if (vars->autoFeed) {
    //         if (!findVehicle(vars->leaderVehicleId) || !findVehicle(vars->frontVehicleId)) {
    //             // either the leader or the front vehicle have left the simulation. Disable auto feed
    //             vars->autoFeed = false;
    //             vars->leaderVehicle = nullptr;
    //             vars->frontVehicle = nullptr;
    //         }
    //     }
    //     if (vars->activeController == Plexe::DRIVER || !vars->useFixedAcceleration) {
    //         switch (vars->activeController) {
    //             case Plexe::ACC:
    //                 ccAcceleration = _cc(veh, egoSpeed, vars->ccDesiredSpeed);
    //                 accAcceleration = _acc(veh, egoSpeed, predSpeed, gap2pred, vars->accHeadwayTime);
    //                 if (gap2pred > 250 || ccAcceleration < accAcceleration) {
    //                     controllerAcceleration = ccAcceleration;
    //                 } else {
    //                     controllerAcceleration = accAcceleration;
    //                 }
    //                 break;
    //
    //             case Plexe::CACC:
    //                 if (vars->autoFeed) {
    //                     getVehicleInformation(vars->leaderVehicle, vars->leaderSpeed, vars->leaderAcceleration, vars->leaderControllerAcceleration, pos, time);
    //                     getVehicleInformation(vars->frontVehicle, vars->frontSpeed, vars->frontAcceleration, vars->frontControllerAcceleration, pos, time);
    //                 }
    //
    //                 if (vars->useControllerAcceleration) {
    //                     predAcceleration = vars->frontControllerAcceleration;
    //                     leaderAcceleration = vars->leaderControllerAcceleration;
    //                 } else {
    //                     predAcceleration = vars->frontAcceleration;
    //                     leaderAcceleration = vars->leaderAcceleration;
    //                 }
    //                 //overwrite pred speed using data obtained through wireless communication
    //                 predSpeed = vars->frontSpeed;
    //                 leaderSpeed = vars->leaderSpeed;
    //                 if (vars->usePrediction) {
    //                     predSpeed += (currentTime - vars->frontDataReadTime) * vars->frontAcceleration;
    //                     leaderSpeed += (currentTime - vars->leaderDataReadTime) * vars->leaderAcceleration;
    //                 }
    //
    //                 if (vars->caccInitialized) {
    //                     controllerAcceleration = _cacc(veh, egoSpeed, predSpeed, predAcceleration, gap2pred, leaderSpeed, leaderAcceleration, vars->caccSpacing);
    //                 } else
    //                     //do not let CACC take decisions until at least one packet has been received
    //                 {
    //                     controllerAcceleration = 0;
    //                 }
    //
    //                 break;
    //
    //             case Plexe::FAKED_CACC:
    //
    //                 if (vars->autoFeed) {
    //                     getVehicleInformation(vars->leaderVehicle, vars->fakeData.leaderSpeed, vars->fakeData.leaderAcceleration, vars->fakeData.leaderControllerAcceleration, pos, time);
    //                     getVehicleInformation(vars->frontVehicle, vars->fakeData.frontSpeed, vars->fakeData.frontAcceleration, vars->fakeData.frontControllerAcceleration, pos, time);
    //                     vars->fakeData.frontDistance = pos.distanceTo2D(veh->getPosition());
    //                 }
    //
    //                 if (vars->useControllerAcceleration) {
    //                     predAcceleration = vars->fakeData.frontControllerAcceleration;
    //                     leaderAcceleration = vars->fakeData.leaderControllerAcceleration;
    //                 } else {
    //                     predAcceleration = vars->fakeData.frontAcceleration;
    //                     leaderAcceleration = vars->fakeData.leaderAcceleration;
    //                 }
    //                 ccAcceleration = _cc(veh, egoSpeed, vars->ccDesiredSpeed);
    //                 caccAcceleration = _cacc(veh, egoSpeed, vars->fakeData.frontSpeed, predAcceleration, vars->fakeData.frontDistance, vars->fakeData.leaderSpeed, leaderAcceleration, vars->caccSpacing);
    //                 //faked CACC can be used to get closer to a platoon for joining
    //                 //using the minimum acceleration ensures that we do not exceed
    //                 //the CC desired speed
    //                 controllerAcceleration = std::min(ccAcceleration, caccAcceleration);
    //
    //                 break;
    //
    //             case Plexe::PLOEG:
    //
    //                 if (vars->autoFeed) {
    //                     getVehicleInformation(vars->frontVehicle, vars->frontSpeed, vars->frontAcceleration, vars->frontControllerAcceleration, pos, time);
    //                 }
    //
    //                 if (vars->useControllerAcceleration) {
    //                     predAcceleration = vars->frontControllerAcceleration;
    //                 } else {
    //                     predAcceleration = vars->frontAcceleration;
    //                 }
    //                 //check if we received at least one packet
    //                 if (vars->frontInitialized)
    //                     //ploeg's controller computes \dot{u}_i, so we need to sum such value to the previously computed u_i
    //                 {
    //                     controllerAcceleration = vars->controllerAcceleration + _ploeg(veh, egoSpeed, predSpeed, predAcceleration, gap2pred);
    //                 } else {
    //                     controllerAcceleration = 0;
    //                 }
    //
    //                 break;
    //
    //             case Plexe::CONSENSUS:
    //                 controllerAcceleration = _consensus(veh, egoSpeed, veh->getPosition(), currentTime);
    //                 break;
    //
    //             case Plexe::FLATBED:
    //
    //                 if (vars->autoFeed) {
    //                     getVehicleInformation(vars->leaderVehicle, vars->leaderSpeed, vars->leaderAcceleration, vars->leaderControllerAcceleration, pos, time);
    //                     getVehicleInformation(vars->frontVehicle, vars->frontSpeed, vars->frontAcceleration, vars->frontControllerAcceleration, pos, time);
    //                 }
    //
    //                 //overwrite pred speed using data obtained through wireless communication
    //                 predSpeed = vars->frontSpeed;
    //                 leaderSpeed = vars->leaderSpeed;
    //                 if (vars->usePrediction) {
    //                     predSpeed += (currentTime - vars->frontDataReadTime) * vars->frontAcceleration;
    //                     leaderSpeed += (currentTime - vars->leaderDataReadTime) * vars->leaderAcceleration;
    //                 }
    //
    //                 if (vars->caccInitialized) {
    //                     controllerAcceleration = _flatbed(veh, veh->getAcceleration(), egoSpeed, predSpeed, gap2pred, leaderSpeed);
    //                 } else
    //                     //do not let CACC take decisions until at least one packet has been received
    //                 {
    //                     controllerAcceleration = 0;
    //                 }
    //
    //                 break;
    //
    //             case Plexe::DRIVER:
    //                 std::cerr << "Switching to normal driver behavior still not implemented in MSCFModel_CC\n";
    //                 assert(false);
    //                 break;
    //
    //             default:
    //                 std::cerr << "Invalid controller selected in MSCFModel_CC\n";
    //                 assert(false);
    //                 break;
    //
    //         }
    //
    //     }
    //
    //     speed = MAX2(double(0), egoSpeed + ACCEL2SPEED(controllerAcceleration));
    //
    //     return speed;
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, gap2pred: f64, egoSpeed: f64, predSpeed: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        let mut predSpeed = predSpeed;

        // acceleration computed by the controller
        let mut controllerAcceleration = vars.fixedAcceleration;
        // acceleration computed by the Cruise Control
        let mut ccAcceleration = 0.;
        // acceleration computed by the Adaptive Cruise Control
        let mut accAcceleration = 0.;
        // acceleration computed by the Cooperative Adaptive Cruise Control
        let mut caccAcceleration = 0.;
        // variables needed by CACC
        let mut predAcceleration = 0.;
        let mut leaderAcceleration = 0.;
        let mut leaderSpeed = 0.;
        // dummy variables used for auto feeding
        let mut pos = Position::new(0., 0.);
        let mut time = 0.;
        let currentTime = steps2time(MSNet::get_instance().get_current_time_step() + delta_t());

        if vars.crashed {
            return 0.;
        }
        if vars.autoFeed {
            if self.find_vehicle(vars.leaderVehicleId.clone()).is_null() || self.find_vehicle(vars.frontVehicleId.clone()).is_null() {
                // either the leader or the front vehicle have left the simulation. Disable auto feed
                vars.m().autoFeed = false;
                vars.m().leaderVehicle = Ptr::null();
                vars.m().frontVehicle = Ptr::null();
            }
        }
        if vars.activeController == ACTIVE_CONTROLLER::DRIVER || vars.useFixedAcceleration == 0 {
            match vars.activeController {
                ACTIVE_CONTROLLER::ACC => {
                    ccAcceleration = self.cc(veh, egoSpeed, vars.ccDesiredSpeed);
                    accAcceleration = self.acc(veh, egoSpeed, predSpeed, gap2pred, vars.accHeadwayTime);
                    if gap2pred > 250. || ccAcceleration < accAcceleration {
                        controllerAcceleration = ccAcceleration;
                    } else {
                        controllerAcceleration = accAcceleration;
                    }
                }
                ACTIVE_CONTROLLER::CACC => {
                    if vars.autoFeed {
                        self.get_vehicle_information(vars.leaderVehicle, &mut vars.m().leaderSpeed, &mut vars.m().leaderAcceleration, &mut vars.m().leaderControllerAcceleration, &mut pos, &mut time);
                        self.get_vehicle_information(vars.frontVehicle, &mut vars.m().frontSpeed, &mut vars.m().frontAcceleration, &mut vars.m().frontControllerAcceleration, &mut pos, &mut time);
                    }
                    if vars.useControllerAcceleration {
                        predAcceleration = vars.frontControllerAcceleration;
                        leaderAcceleration = vars.leaderControllerAcceleration;
                    } else {
                        predAcceleration = vars.frontAcceleration;
                        leaderAcceleration = vars.leaderAcceleration;
                    }
                    // overwrite pred speed using data obtained through wireless communication
                    predSpeed = vars.frontSpeed;
                    leaderSpeed = vars.leaderSpeed;
                    if vars.usePrediction {
                        predSpeed += (currentTime - vars.frontDataReadTime) * vars.frontAcceleration;
                        leaderSpeed += (currentTime - vars.leaderDataReadTime) * vars.leaderAcceleration;
                    }
                    if vars.caccInitialized {
                        controllerAcceleration = self.cacc(veh, egoSpeed, predSpeed, predAcceleration, gap2pred, leaderSpeed, leaderAcceleration, vars.caccSpacing);
                    } else {
                        // do not let CACC take decisions until at least one packet has been received
                        controllerAcceleration = 0.;
                    }
                }
                ACTIVE_CONTROLLER::FAKED_CACC => {
                    if vars.autoFeed {
                        self.get_vehicle_information(vars.leaderVehicle, &mut vars.m().fakeData.leaderSpeed, &mut vars.m().fakeData.leaderAcceleration, &mut vars.m().fakeData.leaderControllerAcceleration, &mut pos, &mut time);
                        self.get_vehicle_information(vars.frontVehicle, &mut vars.m().fakeData.frontSpeed, &mut vars.m().fakeData.frontAcceleration, &mut vars.m().fakeData.frontControllerAcceleration, &mut pos, &mut time);
                        vars.m().fakeData.frontDistance = pos.distance_to2d(&veh.get_position(0.));
                    }
                    if vars.useControllerAcceleration {
                        predAcceleration = vars.fakeData.frontControllerAcceleration;
                        leaderAcceleration = vars.fakeData.leaderControllerAcceleration;
                    } else {
                        predAcceleration = vars.fakeData.frontAcceleration;
                        leaderAcceleration = vars.fakeData.leaderAcceleration;
                    }
                    ccAcceleration = self.cc(veh, egoSpeed, vars.ccDesiredSpeed);
                    caccAcceleration = self.cacc(veh, egoSpeed, vars.fakeData.frontSpeed, predAcceleration, vars.fakeData.frontDistance, vars.fakeData.leaderSpeed, leaderAcceleration, vars.caccSpacing);
                    // faked CACC can be used to get closer to a platoon for joining
                    // using the minimum acceleration ensures that we do not exceed
                    // the CC desired speed
                    // std::min(a, b) is (b < a) ? b : a
                    controllerAcceleration = if caccAcceleration < ccAcceleration { caccAcceleration } else { ccAcceleration };
                }
                ACTIVE_CONTROLLER::PLOEG => {
                    if vars.autoFeed {
                        self.get_vehicle_information(vars.frontVehicle, &mut vars.m().frontSpeed, &mut vars.m().frontAcceleration, &mut vars.m().frontControllerAcceleration, &mut pos, &mut time);
                    }
                    if vars.useControllerAcceleration {
                        predAcceleration = vars.frontControllerAcceleration;
                    } else {
                        predAcceleration = vars.frontAcceleration;
                    }
                    // check if we received at least one packet
                    if vars.frontInitialized {
                        // ploeg's controller computes \dot{u}_i, so we need to sum such value to the previously computed u_i
                        controllerAcceleration = vars.controllerAcceleration + self.ploeg(veh, egoSpeed, predSpeed, predAcceleration, gap2pred);
                    } else {
                        controllerAcceleration = 0.;
                    }
                }
                ACTIVE_CONTROLLER::CONSENSUS => {
                    controllerAcceleration = self.consensus(veh, egoSpeed, veh.get_position(0.), currentTime);
                }
                ACTIVE_CONTROLLER::FLATBED => {
                    if vars.autoFeed {
                        self.get_vehicle_information(vars.leaderVehicle, &mut vars.m().leaderSpeed, &mut vars.m().leaderAcceleration, &mut vars.m().leaderControllerAcceleration, &mut pos, &mut time);
                        self.get_vehicle_information(vars.frontVehicle, &mut vars.m().frontSpeed, &mut vars.m().frontAcceleration, &mut vars.m().frontControllerAcceleration, &mut pos, &mut time);
                    }
                    // overwrite pred speed using data obtained through wireless communication
                    predSpeed = vars.frontSpeed;
                    leaderSpeed = vars.leaderSpeed;
                    if vars.usePrediction {
                        predSpeed += (currentTime - vars.frontDataReadTime) * vars.frontAcceleration;
                        leaderSpeed += (currentTime - vars.leaderDataReadTime) * vars.leaderAcceleration;
                    }
                    if vars.caccInitialized {
                        controllerAcceleration = self.flatbed(veh, veh.get_acceleration(), egoSpeed, predSpeed, gap2pred, leaderSpeed);
                    } else {
                        // do not let CACC take decisions until at least one packet has been received
                        controllerAcceleration = 0.;
                    }
                }
                ACTIVE_CONTROLLER::DRIVER => {
                    eprint!("Switching to normal driver behavior still not implemented in MSCFModel_CC\n");
                    debug_assert!(false);
                }
            }
        }
        let speed = max2(0., egoSpeed + accel2speed(controllerAcceleration));
        speed
    }

    /// C++: `double MSCFModel_CC::_cc(const MSVehicle *, double, double) const` [const, private]
    /// controller for the CC which computes the acceleration to be applied. the value needs to be passed to the actuator
    // double
    // MSCFModel_CC::_cc(const MSVehicle* veh, double egoSpeed, double desSpeed) const {
    //
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     //Eq. 5.5 of the Rajamani book, with Ki = 0 and bounds on max and min acceleration
    //     return std::min(myCcAccel, std::max(-myCcDecel, -vars->ccKp * (egoSpeed - desSpeed)));
    //
    // }
    #[allow(unused_variables)]
    pub fn cc(&self, veh: Ptr<MSVehicle>, egoSpeed: f64, desSpeed: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        //Eq. 5.5 of the Rajamani book, with Ki = 0 and bounds on max and min acceleration
        // std::max(a, b) == (a < b ? b : a); std::min(a, b) == (b < a ? b : a)
        let lo = -self.myCcDecel;
        let mx = -vars.ccKp * (egoSpeed - desSpeed);
        let m = if lo < mx { mx } else { lo };
        if m < self.myCcAccel { m } else { self.myCcAccel }
    }

    /// C++: `double MSCFModel_CC::_acc(const MSVehicle *, double, double, double, double) const` [const, private]
    /// controller for the ACC which computes the acceleration to be applied. the value needs to be passed to the actuator
    // double
    // MSCFModel_CC::_acc(const MSVehicle* veh, double egoSpeed, double predSpeed, double gap2pred, double headwayTime) const {
    //
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     //Eq. 6.18 of the Rajamani book
    //     return -1.0 / headwayTime * (egoSpeed - predSpeed + vars->accLambda * (-gap2pred + headwayTime * egoSpeed + 2));
    //
    // }
    #[allow(unused_variables)]
    pub fn acc(&self, veh: Ptr<MSVehicle>, egoSpeed: f64, predSpeed: f64, gap2pred: f64, headwayTime: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        //Eq. 6.18 of the Rajamani book
        -1.0 / headwayTime * (egoSpeed - predSpeed + vars.accLambda * (-gap2pred + headwayTime * egoSpeed + 2.))
    }

    /// C++: `double MSCFModel_CC::_cacc(const MSVehicle *, double, double, double, double, double, double, double) const` [const, private]
    /// controller for the CACC which computes the acceleration to be applied. the value needs to be passed to the actuator
    // double
    // MSCFModel_CC::_cacc(const MSVehicle* veh, double egoSpeed, double predSpeed, double predAcceleration, double gap2pred, double leaderSpeed, double leaderAcceleration, double spacing) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     //compute epsilon, i.e., the desired distance error
    //     double epsilon = -gap2pred + spacing; //NOTICE: error (if any) should already be included in gap2pred
    //     //compute epsilon_dot, i.e., the desired speed error
    //     double epsilon_dot = egoSpeed - predSpeed;
    //     //Eq. 7.39 of the Rajamani book
    //     return vars->caccAlpha1 * predAcceleration + vars->caccAlpha2 * leaderAcceleration +
    //            vars->caccAlpha3 * epsilon_dot + vars->caccAlpha4 * (egoSpeed - leaderSpeed) + vars->caccAlpha5 * epsilon;
    // }
    #[allow(unused_variables)]
    pub fn cacc(&self, veh: Ptr<MSVehicle>, egoSpeed: f64, predSpeed: f64, predAcceleration: f64, gap2pred: f64, leaderSpeed: f64, leaderAcceleration: f64, spacing: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        //compute epsilon, i.e., the desired distance error
        let epsilon = -gap2pred + spacing; //NOTICE: error (if any) should already be included in gap2pred
        //compute epsilon_dot, i.e., the desired speed error
        let epsilon_dot = egoSpeed - predSpeed;
        //Eq. 7.39 of the Rajamani book
        vars.caccAlpha1 * predAcceleration + vars.caccAlpha2 * leaderAcceleration +
            vars.caccAlpha3 * epsilon_dot + vars.caccAlpha4 * (egoSpeed - leaderSpeed) + vars.caccAlpha5 * epsilon
    }

    /// C++: `double MSCFModel_CC::_ploeg(const MSVehicle *, double, double, double, double) const` [const, private]
    /// controller for the Ploeg's CACC which computes the control input variation. Opposed to other controllers, this method returns a value which needs to be summed to the previous desired acceleration.
    // double
    // MSCFModel_CC::_ploeg(const MSVehicle* veh, double egoSpeed, double predSpeed, double predAcceleration, double gap2pred) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     return (1 / vars->ploegH * (
    //                 -vars->controllerAcceleration +
    //                 vars->ploegKp * (gap2pred - (2 + vars->ploegH * egoSpeed)) +
    //                 vars->ploegKd * (predSpeed - egoSpeed - vars->ploegH * veh->getAcceleration()) +
    //                 predAcceleration
    //             )) * TS ;
    // }
    #[allow(unused_variables)]
    pub fn ploeg(&self, veh: Ptr<MSVehicle>, egoSpeed: f64, predSpeed: f64, predAcceleration: f64, gap2pred: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        (1. / vars.ploegH * (
            -vars.controllerAcceleration +
            vars.ploegKp * (gap2pred - (2. + vars.ploegH * egoSpeed)) +
            vars.ploegKd * (predSpeed - egoSpeed - vars.ploegH * veh.get_acceleration()) +
            predAcceleration
        )) * ts()
    }

    /// C++: `double MSCFModel_CC::_consensus(const MSVehicle *, double, Position, double) const` [const, private]
    /// controller based on consensus strategy
    // double
    // MSCFModel_CC::_consensus(const MSVehicle* veh, double egoSpeed, Position egoPosition, double time) const {
    //     //TODO: this controller, by using real GPS coordinates, does only work
    //     //when vehicles are traveling west-to-east on a straight line, basically
    //     //on the X axis. This needs to be fixed to consider direction as well
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*)veh->getCarFollowVariables();
    //     int index = vars->position;
    //     int nCars = vars->nCars;
    //     struct Plexe::VEHICLE_DATA* vehicles = vars->vehicles;
    //
    //     //loop variable
    //     int j;
    //     //control input
    //     double u_i = 0;
    //     //actual distance term
    //     double actualDistance = 0;
    //     //desired distance term
    //     double desiredDistance = 0;
    //     //speed error term
    //     double speedError = 0;
    //     //degree of agent i
    //     double d_i = 0;
    //
    //     //compensate my position: compute prediction of what will be my position at time of actuation
    //     Position egoVelocity = veh->getVelocityVector();
    //     egoPosition.set(egoPosition.x() + egoVelocity.x() * STEPS2TIME(DELTA_T),
    //                     egoPosition.y() + egoVelocity.y() * STEPS2TIME(DELTA_T));
    //     vehicles[index].speed = egoSpeed;
    //     vehicles[index].positionX = egoPosition.x();
    //     vehicles[index].positionY = egoPosition.y();
    //
    //     //check that data from all vehicles have been received. the control
    //     //law might actually need a subset of all the data, but d_i_j needs
    //     //the lengths of all vehicles. uninitialized values might cause problems
    //     if (vars->nInitialized != vars->nCars - 1) {
    //         return 0;
    //     }
    //
    //     //compute speed error.
    //     speedError = -vars->b[index] * (egoSpeed - vehicles[0].speed);
    //
    //     //compute desired distance term
    //     for (j = 0; j < nCars; j++) {
    //         if (j == index) {
    //             continue;
    //         }
    //         d_i += vars->L[index][j];
    //         desiredDistance -= vars->K[index][j] * vars->L[index][j] * d_i_j(vehicles, vars->h, index, j);
    //     }
    //     desiredDistance = desiredDistance / d_i;
    //
    //     //compute actual distance term
    //     for (j = 0; j < nCars; j++) {
    //         if (j == index) {
    //             continue;
    //         }
    //         //distance error for consensus with GPS equipped
    //         Position otherPosition;
    //         double dt = time - vehicles[j].time;
    //         //predict the position of the other vehicle
    //         otherPosition.setx(vehicles[j].positionX + dt * vehicles[j].speedX);
    //         otherPosition.sety(vehicles[j].positionY + dt * vehicles[j].speedY);
    //         double distance = egoPosition.distanceTo2D(otherPosition) * sgn(j - index);
    //         actualDistance -= vars->K[index][j] * vars->L[index][j] * distance;
    //     }
    //
    //     actualDistance = actualDistance / (d_i);
    //
    //     //original paper formula
    //     u_i = (speedError + desiredDistance + actualDistance) / 1000;
    //
    //     return u_i;
    // }
    #[allow(unused_variables)]
    pub fn consensus(&self, veh: Ptr<MSVehicle>, egoSpeed: f64, egoPosition: Position, time: f64) -> f64 {
        // TODO: this controller, by using real GPS coordinates, does only work
        // when vehicles are traveling west-to-east on a straight line, basically
        // on the X axis. This needs to be fixed to consider direction as well
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        let index = vars.position;
        let nCars = vars.nCars;
        let vehicles: Ptr<VEHICLE_DATA> = Ptr::from_raw(vars.m().vehicles.as_mut_ptr());

        // control input
        let mut u_i = 0.;
        // actual distance term
        let mut actualDistance = 0.;
        // desired distance term
        let mut desiredDistance = 0.;
        // speed error term
        let mut speedError = 0.;
        // degree of agent i
        let mut d_i = 0.;

        // compensate my position: compute prediction of what will be my position at time of actuation
        let mut egoPosition = egoPosition;
        let egoVelocity = veh.get_velocity_vector();
        egoPosition.set(egoPosition.x() + egoVelocity.x() * steps2time(delta_t()),
                        egoPosition.y() + egoVelocity.y() * steps2time(delta_t()));
        vars.m().vehicles[index as usize].speed = egoSpeed;
        vars.m().vehicles[index as usize].positionX = egoPosition.x();
        vars.m().vehicles[index as usize].positionY = egoPosition.y();

        // check that data from all vehicles have been received. the control
        // law might actually need a subset of all the data, but d_i_j needs
        // the lengths of all vehicles. uninitialized values might cause problems
        if vars.nInitialized != vars.nCars - 1 {
            return 0.;
        }

        // compute speed error.
        speedError = -vars.b[index as usize] * (egoSpeed - vars.vehicles[0].speed);

        // compute desired distance term
        for j in 0..nCars {
            if j == index {
                continue;
            }
            d_i += vars.L[index as usize][j as usize] as f64;
            desiredDistance -= vars.K[index as usize][j as usize] * (vars.L[index as usize][j as usize] as f64) * self.d_i_j(vehicles, vars.h, index, j);
        }
        desiredDistance = desiredDistance / d_i;

        // compute actual distance term
        for j in 0..nCars {
            if j == index {
                continue;
            }
            // distance error for consensus with GPS equipped
            let mut otherPosition = Position::new(0., 0.);
            let dt = time - vars.vehicles[j as usize].time;
            // predict the position of the other vehicle
            otherPosition.setx(vars.vehicles[j as usize].positionX + dt * vars.vehicles[j as usize].speedX);
            otherPosition.sety(vars.vehicles[j as usize].positionY + dt * vars.vehicles[j as usize].speedY);
            let distance = egoPosition.distance_to2d(&otherPosition) * (sgn(j - index) as f64);
            actualDistance -= vars.K[index as usize][j as usize] * (vars.L[index as usize][j as usize] as f64) * distance;
        }

        actualDistance = actualDistance / d_i;

        // original paper formula
        u_i = (speedError + desiredDistance + actualDistance) / 1000.;

        u_i
    }

    /// C++: `double MSCFModel_CC::d_i_j(const struct Plexe::VEHICLE_DATA *, const double *, int, int) const` [const, private]
    /// computes the desired distance between vehicle i and vehicle j
    // double
    // MSCFModel_CC::d_i_j(const struct Plexe::VEHICLE_DATA* vehicles, const double h[MAX_N_CARS], int i, int j) const {
    //
    //     int k, min_i, max_i;
    //     double d = 0;
    //     //compute indexes of the summation
    //     if (j < i) {
    //         min_i = j;
    //         max_i = i - 1;
    //     } else {
    //         min_i = i;
    //         max_i = j - 1;
    //     }
    //     //compute distance
    //     for (k = min_i; k <= max_i; k++) {
    //         d += h[k] * vehicles[0].speed + vehicles[k].length + 15;
    //     }
    //
    //     if (j < i) {
    //         return d;
    //     } else {
    //         return -d;
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn d_i_j(&self, vehicles: Ptr<VEHICLE_DATA>, h: [f64; 8], i: i32, j: i32) -> f64 {
        let (min_i, max_i) = if j < i { (j, i - 1) } else { (i, j - 1) };
        let mut d = 0.;
        //compute indexes of the summation
        //compute distance
        for k in min_i..=max_i {
            d += h[k as usize] * unsafe { (*vehicles.as_raw()).speed } + unsafe { (*vehicles.as_raw().add(k as usize)).length } + 15.;
        }
        if j < i {
            d
        } else {
            -d
        }
    }

    /// C++: `double MSCFModel_CC::_flatbed(const MSVehicle *, double, double, double, double, double) const` [const, private]
    /// flatbed platoon towing model
    // double
    // MSCFModel_CC::_flatbed(const MSVehicle* veh, double egoAcceleration, double egoSpeed, double predSpeed,
    //                        double gap2pred, double leaderSpeed) const {
    //     CC_VehicleVariables* vars = (CC_VehicleVariables*) veh->getCarFollowVariables();
    //     return (
    //                -vars->flatbedKa * egoAcceleration +
    //                vars->flatbedKv * (predSpeed - egoSpeed) +
    //                vars->flatbedKp * (gap2pred - vars->flatbedD - vars->flatbedH * (egoSpeed - leaderSpeed))
    //            );
    // }
    #[allow(unused_variables)]
    pub fn flatbed(&self, veh: Ptr<MSVehicle>, egoAcceleration: f64, egoSpeed: f64, predSpeed: f64, gap2pred: f64, leaderSpeed: f64) -> f64 {
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut CC_VehicleVariables);
        -vars.flatbedKa * egoAcceleration +
            vars.flatbedKv * (predSpeed - egoSpeed) +
            vars.flatbedKp * (gap2pred - vars.flatbedD - vars.flatbedH * (egoSpeed - leaderSpeed))
    }

    /// C++: `MSCFModel_CC & MSCFModel_CC::operator=(const MSCFModel_CC &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSCFModel_CC) -> &mut MSCFModel_CC {
        panic!("MSCFModel_CC assignment operator is invalidated (deleted) in C++")
    }

}

/// C++ `sgn` macro from MSCFModel_CC.cpp: `((x > 0) - (x < 0))`
fn sgn(x: i32) -> i32 {
    (x > 0) as i32 - (x < 0) as i32
}
