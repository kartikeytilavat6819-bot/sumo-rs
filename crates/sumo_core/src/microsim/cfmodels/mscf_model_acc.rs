//! Port of microsim/cfmodels/MSCFModel_ACC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSCFModel_ACC::ACCVehicleVariables` (microsim/cfmodels/MSCFModel_ACC.h)
#[repr(C)]
pub struct MSCFModel_ACC_ACCVehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    /// The vehicle's ACC control mode. 0 for speed control and 1 for gap control
    pub ACC_ControlMode: i32,
    pub lastUpdateTime: SUMOTime,
}

impl std::ops::Deref for MSCFModel_ACC_ACCVehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_ACC_ACCVehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_ACC_ACCVehicleVariables {
    /// C++: `void ACCVehicleVariables::ACCVehicleVariables()`
    // ACCVehicleVariables() : ACC_ControlMode(0) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port ACCVehicleVariables::ACCVehicleVariables")
    }

    /// C++: `void ACCVehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_ACC::ACCVehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "ACC");
    //     std::ostringstream internals;
    //     internals << ACC_ControlMode << " ";
    //     internals << lastUpdateTime;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        todo!("port ACCVehicleVariables::saveState")
    }

    /// C++: `void ACCVehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_ACC::ACCVehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "ACC") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for ACC", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> ACC_ControlMode;
    //     bis >> lastUpdateTime;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port ACCVehicleVariables::loadState")
    }

}

/// The ACC car-following model
/// C++ class `MSCFModel_ACC` (microsim/cfmodels/MSCFModel_ACC.h)
#[repr(C)]
pub struct MSCFModel_ACC {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    pub mySpeedControlGain: f64,
    pub myGapClosingControlGainSpeed: f64,
    pub myGapClosingControlGainSpace: f64,
    pub myGapControlGainSpeed: f64,
    pub myGapControlGainSpace: f64,
    pub myCollisionAvoidanceGainSpeed: f64,
    pub myCollisionAvoidanceGainSpace: f64,
    pub myApplyDriverstate: f64,
    pub myEmergencyThreshold: f64,
}

impl std::ops::Deref for MSCFModel_ACC {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_ACC {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_ACC {
    /// C++: `void MSCFModel_ACC::MSCFModel_ACC(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_ACC::MSCFModel_ACC(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     mySpeedControlGain(vtype->getParameter().getCFParam(SUMO_ATTR_SC_GAIN, DEFAULT_SC_GAIN)),
    //     myGapClosingControlGainSpeed(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_SPEED, DEFAULT_GCC_GAIN_SPEED)),
    //     myGapClosingControlGainSpace(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_SPACE, DEFAULT_GCC_GAIN_SPACE)),
    //     myGapControlGainSpeed(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_SPEED, DEFAULT_GC_GAIN_SPEED)),
    //     myGapControlGainSpace(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_SPACE, DEFAULT_GC_GAIN_SPACE)),
    //     myCollisionAvoidanceGainSpeed(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_SPEED, DEFAULT_CA_GAIN_SPEED)),
    //     myCollisionAvoidanceGainSpace(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_SPACE, DEFAULT_CA_GAIN_SPACE)),
    //     myApplyDriverstate(vtype->getParameter().getCFParam(SUMO_ATTR_APPLYDRIVERSTATE, 0)),
    //     myEmergencyThreshold(vtype->getParameter().getCFParam(SUMO_ATTR_CA_OVERRIDE, DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD)) {
    //     // ACC does not drive very precise and often violates minGap
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        todo!("port MSCFModel_ACC::MSCFModel_ACC")
    }

    /// C++: `void MSCFModel_ACC::~MSCFModel_ACC() noexcept` [virtual]
    /// Destructor
    // MSCFModel_ACC::~MSCFModel_ACC() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_ACC::~MSCFModel_ACC")
    }

    /// C++: `double MSCFModel_ACC::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_ACC::followSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred, const CalcReason /*usage*/) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayAndSpeedDifferencePerceptionErrors(veh, speed, gap2pred, predSpeed, predMaxDecel, pred);
    //     }
    //     const double desSpeed = MIN2(veh->getLane()->getSpeedLimit(), veh->getMaxSpeed());
    //     const double vACC = _v(veh, gap2pred, speed, predSpeed, desSpeed, true);
    //     const double vSafe = maximumSafeFollowSpeed(gap2pred, speed, predSpeed, predMaxDecel);
    //     if (vSafe + myEmergencyThreshold < vACC) {
    //         //ACCVehicleVariables* vars = (ACCVehicleVariables*)veh->getCarFollowVariables();
    //         //std::cout << SIMTIME << " veh=" << veh->getID() << " v=" << speed << " vL=" << predSpeed << " gap=" << gap2pred << " vACC=" << vACC << " vSafe=" << vSafe << " cm=" << vars->ACC_ControlMode << "\n";
    //         return vSafe + myEmergencyThreshold;
    //     }
    //     return vACC;
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_ACC::followSpeed")
    }

    /// C++: `double MSCFModel_ACC::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_ACC::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayPerceptionError(veh, speed, gap);
    //     }
    //     // NOTE: This allows return of smaller values than minNextSpeed().
    //     // Only relevant for the ballistic update: We give the argument headway=TS, to assure that
    //     // the stopping position is approached with a uniform deceleration also for tau!=TS.
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, veh->getActionStepLengthSecs()), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_ACC::stopSpeed")
    }

    /// C++: `double MSCFModel_ACC::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    /// Returns the a gap such that the gap mode acceleration of the follower is zero
    // double
    // MSCFModel_ACC::getSecureGap(const MSVehicle* const /*veh*/, const MSVehicle* const /*pred*/, const double speed, const double leaderSpeed, const double /* leaderMaxDecel */) const {
    //     // Accel in gap mode should vanish:
    //     //      0 = myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * (g - myHeadwayTime * speed);
    //     // <=>  myGapControlGainSpace * g = - myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * myHeadwayTime * speed;
    //     // <=>  g = - myGapControlGainSpeed * (leaderSpeed - speed) / myGapControlGainSpace + myHeadwayTime * speed;
    //     return myGapControlGainSpeed * (speed - leaderSpeed) / myGapControlGainSpace + myHeadwayTime * speed;
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        todo!("port MSCFModel_ACC::getSecureGap")
    }

    /// C++: `double MSCFModel_ACC::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Computes the vehicle's acceptable speed at insertion
    // double
    // MSCFModel_ACC::insertionFollowSpeed(const MSVehicle* const v, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const /*pred*/) const {
    // //#ifdef DEBUG_ACC
    // //        std::cout << "MSCFModel_ACC::insertionFollowSpeed(), speed="<<speed<< std::endl;
    // //#endif
    //     // iterate to find a stationary value for
    //     //    speed = followSpeed(v, speed, gap2pred, predSpeed, predMaxDecel, nullptr, CalcReason::FUTURE)
    //     const int max_iter = 50;
    //     int n_iter = 0;
    //     const double tol = 0.1;
    //     const double damping = 0.1;
    //
    //     double res = speed;
    //     while (n_iter < max_iter) {
    //         // proposed acceleration
    //         const double a = SPEED2ACCEL(followSpeed(v, res, gap2pred, predSpeed, predMaxDecel, nullptr, CalcReason::FUTURE) - res);
    //         res = res + damping * a;
    // //#ifdef DEBUG_ACC
    // //        std::cout << "   n_iter=" << n_iter << ", a=" << a << ", res=" << res << std::endl;
    // //#endif
    //         if (fabs(a) < tol) {
    //             break;
    //         } else {
    //             n_iter++;
    //         }
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn insertion_follow_speed(&self, v: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
        todo!("port MSCFModel_ACC::insertionFollowSpeed")
    }

    /// C++: `double MSCFModel_ACC::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_ACC::interactionGap(const MSVehicle* const /*veh */, double /* vL */) const {
    //     /*maximum radar range is ACC is enabled*/
    //     return 250;
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        todo!("port MSCFModel_ACC::interactionGap")
    }

    /// C++: `double MSCFModel_ACC::maximumLaneSpeedCF(const MSVehicle *const, double, double) const` [const, virtual]
    /// Returns the maximum velocity the CF-model wants to achieve in the next step
    // double maximumLaneSpeedCF(const MSVehicle* const veh, double maxSpeed, double maxSpeedLane) const {
    //         double result = MIN2(maxSpeed, maxSpeedLane);
    //         if (myApplyDriverstate) {
    //             applyOwnSpeedPerceptionError(veh, result);
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn maximum_lane_speed_cf(&self, veh: Ptr<MSVehicle>, maxSpeed: f64, maxSpeedLane: f64) -> f64 {
        todo!("port MSCFModel_ACC::maximumLaneSpeedCF")
    }

    /// C++: `int MSCFModel_ACC::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_ACC;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_ACC::getModelID")
    }

    /// C++: `MSCFModel * MSCFModel_ACC::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_ACC::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_ACC(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_ACC::duplicate")
    }

    /// C++: `VehicleVariables * MSCFModel_ACC::createVehicleVariables() const` [const, virtual]
    // VehicleVariables* createVehicleVariables() const {
    //         ACCVehicleVariables* ret = new ACCVehicleVariables();
    //         ret->ACC_ControlMode = 0;
    //         ret->lastUpdateTime = 0;
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        todo!("port MSCFModel_ACC::createVehicleVariables")
    }

    /// C++: `double MSCFModel_ACC::_v(const MSVehicle *const, const double, const double, const double, const double, const bool) const` [const, private]
    // double
    // MSCFModel_ACC::_v(const MSVehicle* const veh, const double gap2pred, const double speed,
    //                   const double predSpeed, const double desSpeed, const bool /* respectMinGap */) const {
    //
    //     double accelACC = 0;
    //     double gapLimit_SC = GAP_THRESHOLD_SPEEDCTRL; // lower gap limit in meters to enable speed control law
    //     double gapLimit_GC = GAP_THRESHOLD_GAPCTRL; // upper gap limit in meters to enable gap control law
    //
    // #ifdef DEBUG_ACC
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " MSCFModel_ACC::_v() for veh '" << veh->getID() << "'\n"
    //                   << "        gap=" << gap2pred << " speed="  << speed << " predSpeed=" << predSpeed
    //                   << " desSpeed=" << desSpeed << " tau=" << myHeadwayTime << std::endl;
    //     }
    // #endif
    //
    //
    //     /* Velocity error */
    //     double vErr = speed - desSpeed;
    //     int setControlMode = 0;
    //     ACCVehicleVariables* vars = (ACCVehicleVariables*) veh->getCarFollowVariables();
    //     if (vars->lastUpdateTime != MSNet::getInstance()->getCurrentTimeStep()) {
    //         vars->lastUpdateTime = MSNet::getInstance()->getCurrentTimeStep();
    //         setControlMode = 1;
    //     }
    //     if (gap2pred > gapLimit_SC) {
    //
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying speedControl" << std::endl;
    //         }
    // #endif
    //         // Find acceleration - Speed control law
    //         accelACC = accelSpeedControl(vErr);
    //         // Set cl to vehicle parameters
    //         if (setControlMode) {
    //             vars->ACC_ControlMode = 0;
    //         }
    //     } else if (gap2pred < gapLimit_GC) {
    //         // Find acceleration - Gap control law
    //         accelACC = accelGapControl(veh, gap2pred, speed, predSpeed, vErr);
    //         // Set cl to vehicle parameters
    //         if (setControlMode) {
    //             vars->ACC_ControlMode = 1;
    //         }
    //     } else {
    //         // Follow previous applied law
    //         int cm = vars->ACC_ControlMode;
    //         if (!cm) {
    //
    // #ifdef DEBUG_ACC
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying speedControl" << std::endl;
    //             }
    // #endif
    //             accelACC = accelSpeedControl(vErr);
    //         } else {
    //             accelACC = accelGapControl(veh, gap2pred, speed, predSpeed, vErr);
    //         }
    //
    //     }
    //
    //     double newSpeed = speed + ACCEL2SPEED(accelACC);
    //
    // #ifdef DEBUG_ACC
    //     if (DEBUG_COND) {
    //         std::cout << "        result: accel=" << accelACC << " newSpeed="  << newSpeed << std::endl;
    //     }
    // #endif
    //
    //     return MAX2(0., newSpeed);
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, gap2pred: f64, mySpeed: f64, predSpeed: f64, desSpeed: f64, respectMinGap: bool) -> f64 {
        todo!("port MSCFModel_ACC::_v")
    }

    /// C++: `double MSCFModel_ACC::accelSpeedControl(double) const` [const, private]
    // double MSCFModel_ACC::accelSpeedControl(double vErr) const {
    //     // Speed control law
    //     return mySpeedControlGain * vErr;
    // }
    #[allow(unused_variables)]
    pub fn accel_speed_control(&self, vErr: f64) -> f64 {
        todo!("port MSCFModel_ACC::accelSpeedControl")
    }

    /// C++: `double MSCFModel_ACC::accelGapControl(const MSVehicle *const, const double, const double, const double, double) const` [const, private]
    // double
    // MSCFModel_ACC::accelGapControl(const MSVehicle* const /* veh */, const double gap2pred, const double speed, const double predSpeed, double vErr) const {
    //     // Gap control law
    //     double gclAccel = 0.0;
    //     const double deltaVel = predSpeed - speed;
    //
    //     // see dynamic gap margin definition from (Xiao et. al, 2018)[3], equation 5 reformulated as min/max to avoid discontinuities
    //     const double d0 = MAX2(0., MIN2(75. / speed - 5., 2.));
    //     // this is equation 4, gap2pred is the difference in vehicle positions minus the length
    //     const double spacingErr = gap2pred - myHeadwayTime * speed - d0;
    //
    //
    //     if (fabs(spacingErr) < 0.2 && fabs(vErr) < 0.1) {
    //         // gap mode
    //         gclAccel = myGapControlGainSpeed * deltaVel + myGapControlGainSpace * spacingErr;
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying gap control:  spacingErr=" << spacingErr << " speedErr=" << vErr << std::endl;
    //         }
    // #endif
    //     } else if (spacingErr < 0)  {
    //         // collision avoidance mode
    //         gclAccel = myCollisionAvoidanceGainSpeed * deltaVel + myCollisionAvoidanceGainSpace * spacingErr;
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying collision avoidance:  spacingErr=" << spacingErr << " speedErr=" << vErr << std::endl;
    //         }
    // #endif
    //     } else {
    //         // gap closing mode
    //         gclAccel = myGapClosingControlGainSpeed * deltaVel + myGapClosingControlGainSpace * spacingErr;
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying gap closing:  spacingErr=" << spacingErr << " speedErr=" << vErr << std::endl;
    //         }
    // #endif
    //     }
    //     return gclAccel;
    // }
    #[allow(unused_variables)]
    pub fn accel_gap_control(&self, veh: Ptr<MSVehicle>, gap2pred: f64, speed: f64, predSpeed: f64, vErr: f64) -> f64 {
        todo!("port MSCFModel_ACC::accelGapControl")
    }

    /// C++: `MSCFModel_ACC & MSCFModel_ACC::operator=(const MSCFModel_ACC &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSCFModel_ACC) -> &mut MSCFModel_ACC {
        todo!("port MSCFModel_ACC::operator=")
    }

}

