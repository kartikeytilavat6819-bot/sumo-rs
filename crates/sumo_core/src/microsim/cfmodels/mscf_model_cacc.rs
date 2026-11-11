//! Port of microsim/cfmodels/MSCFModel_CACC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSCFModel_CACC::CommunicationsOverrideMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSCFModel_CACC_CommunicationsOverrideMode {
    CACC_NO_OVERRIDE = 0,
    CACC_MODE_NO_LEADER = 1,
    CACC_MODE_LEADER_NO_CAV = 2,
    CACC_MODE_LEADER_CAV = 3,
}

/// C++ enum `MSCFModel_CACC::VehicleMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSCFModel_CACC_VehicleMode {
    CC_MODE = 0,
    ACC_MODE = 1,
    CACC_GAP_MODE = 2,
    CACC_GAP_CLOSING_MODE = 3,
    CACC_COLLISION_AVOIDANCE_MODE = 4,
}

/// C++ class `MSCFModel_CACC::CACCVehicleVariables` (microsim/cfmodels/MSCFModel_CACC.h)
pub struct MSCFModel_CACC_CACCVehicleVariables {
    /// C++ base class `MSCFModel_ACC::ACCVehicleVariables`
    pub base: MSCFModel_ACC_ACCVehicleVariables,
    /// The vehicle's CACC precious time step gap error
    pub CACC_ControlMode: i32,
    pub CACC_CommunicationsOverrideMode: MSCFModel_CACC_CommunicationsOverrideMode,
}

impl std::ops::Deref for MSCFModel_CACC_CACCVehicleVariables {
    type Target = MSCFModel_ACC_ACCVehicleVariables;
    fn deref(&self) -> &MSCFModel_ACC_ACCVehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_CACC_CACCVehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_ACC_ACCVehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_CACC_CACCVehicleVariables {
    /// C++: `void CACCVehicleVariables::CACCVehicleVariables()`
    // CACCVehicleVariables() : CACC_ControlMode(0), CACC_CommunicationsOverrideMode(CACC_NO_OVERRIDE) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port CACCVehicleVariables::CACCVehicleVariables")
    }

    /// C++: `void CACCVehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_CACC::CACCVehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "CACC");
    //     std::ostringstream internals;
    //     internals << CACC_ControlMode << " ";
    //     internals << CACC_CommunicationsOverrideMode;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        todo!("port CACCVehicleVariables::saveState")
    }

    /// C++: `void CACCVehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_CACC::CACCVehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "CACC") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for CACC", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> CACC_ControlMode;
    //     int mode;
    //     bis >> mode;
    //     CACC_CommunicationsOverrideMode = (CommunicationsOverrideMode)mode;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port CACCVehicleVariables::loadState")
    }

}

/// The CACC car-following model
/// C++ class `MSCFModel_CACC` (microsim/cfmodels/MSCFModel_CACC.h)
pub struct MSCFModel_CACC {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    pub acc_CFM: MSCFModel_ACC,
    pub mySpeedControlGain: f64,
    pub myGapClosingControlGainGap: f64,
    pub myGapClosingControlGainGapDot: f64,
    pub myGapControlGainGap: f64,
    pub myGapControlGainGapDot: f64,
    pub myCollisionAvoidanceGainGap: f64,
    pub myCollisionAvoidanceGainGapDot: f64,
    pub myHeadwayTimeACC: f64,
    pub myApplyDriverstate: f64,
    pub myEmergencyThreshold: f64,
    pub mySpeedControlMinGap: f64,
}

impl std::ops::Deref for MSCFModel_CACC {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_CACC {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

// C++ static members of MSCFModel_CACC (port as module-level statics, see AGENTS.md):
//   static std::map<std::string, CommunicationsOverrideMode> MSCFModel_CACC::CommunicationsOverrideModeMap;
//   static std::map<VehicleMode, std::string> MSCFModel_CACC::VehicleModeNames;

impl MSCFModel_CACC {
    /// C++: `void MSCFModel_CACC::MSCFModel_CACC(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_CACC::MSCFModel_CACC(const MSVehicleType* vtype) :
    //     MSCFModel(vtype), acc_CFM(MSCFModel_ACC(vtype)),
    //     mySpeedControlGain(vtype->getParameter().getCFParam(SUMO_ATTR_SC_GAIN_CACC, DEFAULT_SC_GAIN_CACC)),
    //     myGapClosingControlGainGap(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_GAP_CACC, DEFAULT_GCC_GAIN_GAP_CACC)),
    //     myGapClosingControlGainGapDot(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_GAP_DOT_CACC, DEFAULT_GCC_GAIN_GAP_DOT_CACC)),
    //     myGapControlGainGap(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_GAP_CACC, DEFAULT_GC_GAIN_GAP_CACC)),
    //     myGapControlGainGapDot(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_GAP_DOT_CACC, DEFAULT_GC_GAIN_GAP_DOT_CACC)),
    //     myCollisionAvoidanceGainGap(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_GAP_CACC, DEFAULT_CA_GAIN_GAP_CACC)),
    //     myCollisionAvoidanceGainGapDot(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_GAP_DOT_CACC, DEFAULT_CA_GAIN_GAP_DOT_CACC)),
    //     myHeadwayTimeACC(vtype->getParameter().getCFParam(SUMO_ATTR_HEADWAY_TIME_CACC_TO_ACC, DEFAULT_HEADWAYTIME_ACC)),
    //     myApplyDriverstate(vtype->getParameter().getCFParam(SUMO_ATTR_APPLYDRIVERSTATE, 0)),
    //     myEmergencyThreshold(vtype->getParameter().getCFParam(SUMO_ATTR_CA_OVERRIDE, DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD)),
    //     mySpeedControlMinGap(vtype->getParameter().getCFParam(SUMO_ATTR_SC_MIN_GAP, DEFAULT_SC_MIN_GAP)) {
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    //     acc_CFM.setHeadwayTime(myHeadwayTimeACC);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        todo!("port MSCFModel_CACC::MSCFModel_CACC")
    }

    /// C++: `void MSCFModel_CACC::~MSCFModel_CACC() noexcept` [virtual]
    /// Destructor
    // MSCFModel_CACC::~MSCFModel_CACC() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_CACC::~MSCFModel_CACC")
    }

    /// C++: `double MSCFModel_CACC::freeSpeed(const MSVehicle *const, double, double, double, const bool, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed without a leader
    // double
    // MSCFModel_CACC::freeSpeed(const MSVehicle* const veh, double speed, double seen, double maxSpeed, const bool onInsertion, const CalcReason usage) const {
    //     if (!MSGlobals::gComputeLC && usage == CalcReason::CURRENT) {
    //         CACCVehicleVariables* vars = (CACCVehicleVariables*)veh->getCarFollowVariables();
    //         if (vars->lastUpdateTime != MSNet::getInstance()->getCurrentTimeStep()) {
    //             // _v was not called in this step
    //             const_cast<SUMOVehicleParameter&>(veh->getParameter()).setParameter("caccVehicleMode", VehicleModeNames[CC_MODE]);
    //         }
    //     }
    //     return MSCFModel::freeSpeed(veh, speed, seen, maxSpeed, onInsertion, usage);
    // }
    #[allow(unused_variables)]
    pub fn free_speed(&self, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_CACC::freeSpeed")
    }

    /// C++: `double MSCFModel_CACC::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_CACC::followSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred, const CalcReason usage) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayAndSpeedDifferencePerceptionErrors(veh, speed, gap2pred, predSpeed, predMaxDecel, pred);
    //     }
    //
    //     const double desSpeed = veh->getLane()->getVehicleMaxSpeed(veh);
    //     const double vCACC = _v(veh, pred, gap2pred, speed, predSpeed, desSpeed, true, usage);
    //     // using onInsertion=true disables emergencyOverides emergency deceleration smoothing
    //     const double vSafe = maximumSafeFollowSpeed(gap2pred, speed, predSpeed, predMaxDecel, true);
    //
    // #if DEBUG_CACC == 1
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << veh->getID() << " pred=" << Named::getIDSecure(pred)
    //                   << " v=" << speed << " vL=" << predSpeed << " gap=" << gap2pred
    //                   << " predDecel=" << predMaxDecel << " vCACC=" << vCACC << " vSafe=" << vSafe << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(pred);
    // #endif
    //     const double speedOverride = MIN2(myEmergencyThreshold, gap2pred);
    //     if (vSafe + speedOverride < vCACC) {
    // #if DEBUG_CACC == 1
    //         if (DEBUG_COND) {
    //             std::cout << "Apply Safe speed, override=" << speedOverride << "\n";
    //         }
    // #endif
    //         return MAX2(0.0, vSafe + speedOverride);
    //     }
    //     return vCACC;
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_CACC::followSpeed")
    }

    /// C++: `double MSCFModel_CACC::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_CACC::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayPerceptionError(veh, speed, gap);
    //     }
    //
    //     // NOTE: This allows return of smaller values than minNextSpeed().
    //     // Only relevant for the ballistic update: We give the argument headway=TS, to assure that
    //     // the stopping position is approached with a uniform deceleration also for tau!=TS.
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, veh->getActionStepLengthSecs()), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_CACC::stopSpeed")
    }

    /// C++: `double MSCFModel_CACC::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    /// Returns the a gap such that the gap mode acceleration of the follower is zero
    // double
    // MSCFModel_CACC::getSecureGap(const MSVehicle* const veh, const MSVehicle* const pred, const double speed, const double leaderSpeed, const double leaderMaxDecel) const {
    //     // Accel in gap mode should vanish:
    //     double desSpacing;
    //     if (pred->getCarFollowModel().getModelID() != SUMO_TAG_CF_CACC) {
    //         //      0 = myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * (g - myHeadwayTime * speed);
    //         // <=>  myGapControlGainSpace * g = - myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * myHeadwayTime * speed;
    //         // <=>  g = - myGapControlGainSpeed * (leaderSpeed - speed) / myGapControlGainSpace + myHeadwayTime * speed;
    //         desSpacing = acc_CFM.myGapControlGainSpeed * (speed - leaderSpeed) / acc_CFM.myGapControlGainSpace + myHeadwayTimeACC * speed; // MSCFModel_ACC::accelGapControl
    //     } else {
    //         desSpacing = myHeadwayTime * speed; // speedGapControl
    //     }
    //     const double desSpacingDefault = MSCFModel::getSecureGap(veh, pred, speed, leaderSpeed, leaderMaxDecel);
    // #if DEBUG_CACC_SECURE_GAP == 1
    //     std::cout << SIMTIME << "MSCFModel_ACC::getSecureGap speed=" << speed << " leaderSpeed=" << leaderSpeed
    //               << " desSpacing=" << desSpacing << " desSpacingDefault=" << desSpacingDefault << "\n";
    // #endif
    //     return MAX2(desSpacing, desSpacingDefault);
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        todo!("port MSCFModel_CACC::getSecureGap")
    }

    /// C++: `double MSCFModel_CACC::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Computes the vehicle's acceptable speed at insertion
    // double
    // MSCFModel_CACC::insertionFollowSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred) const {
    // #if DEBUG_CACC_INSERTION_FOLLOW_SPEED == 1
    //     if (DEBUG_COND) {
    //         std::cout << "MSCFModel_ACC::insertionFollowSpeed(), speed=" << speed << " gap2pred=" << gap2pred << " predSpeed=" << predSpeed << "\n";
    //     }
    // #endif
    //     // iterate to find a stationary value for
    //     //    speed = followSpeed(v, speed, gap2pred, predSpeed, predMaxDecel, nullptr, CalcReason::FUTURE)
    //     const int max_iter = 50;
    //     int n_iter = 0;
    //     const double tol = 0.1;
    //     double damping = 0.8;
    //
    //     double res = speed;
    //     while (n_iter < max_iter) {
    //         // proposed acceleration
    //         const double vCACC = _v(veh, pred, gap2pred, res, predSpeed, speed, true);
    //         const double vSafe = maximumSafeFollowSpeed(gap2pred, res, predSpeed, predMaxDecel, true);
    //         const double a = MIN2(vCACC, vSafe) - res;
    //         res = res + damping * a;
    // #if DEBUG_CACC_INSERTION_FOLLOW_SPEED == 1
    //         if (DEBUG_COND) {
    //             std::cout << "   n_iter=" << n_iter << " vSafe=" << vSafe << " vCACC=" << vCACC << " a=" << a << " damping=" << damping << " res=" << res << std::endl;
    //         }
    // #endif
    //         damping *= 0.9;
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
        todo!("port MSCFModel_CACC::insertionFollowSpeed")
    }

    /// C++: `double MSCFModel_CACC::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_CACC::interactionGap(const MSVehicle* const /* veh */, double /* vL */) const {
    //     /*maximum radar range is CACC is enabled*/
    //     return 250;
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        todo!("port MSCFModel_CACC::interactionGap")
    }

    /// C++: `void MSCFModel_CACC::setHeadwayTime(double)` [virtual]
    /// Sets a new value for desired headway [s]
    // void setHeadwayTime(double headwayTime) {
    //         myHeadwayTime = headwayTime;
    //         myHeadwayTimeACC = headwayTime;
    //         acc_CFM.setHeadwayTime(headwayTime);
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_time(&mut self, headwayTime: f64) {
        todo!("port MSCFModel_CACC::setHeadwayTime")
    }

    /// C++: `std::string MSCFModel_CACC::getParameter(const MSVehicle *, const std::string &) const` [const, virtual]
    /// try to get the given parameter for this carFollowingModel
    // std::string
    // MSCFModel_CACC::getParameter(const MSVehicle* veh, const std::string& key) const {
    //     CACCVehicleVariables* vars = (CACCVehicleVariables*) veh->getCarFollowVariables();
    //
    //     if (key.compare("caccCommunicationsOverrideMode") == 0) {
    //         return toString(vars->CACC_CommunicationsOverrideMode);
    //     }
    //
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, veh: Ptr<MSVehicle>, key: &str) -> String {
        todo!("port MSCFModel_CACC::getParameter")
    }

    /// C++: `void MSCFModel_CACC::setParameter(MSVehicle *, const std::string &, const std::string &) const` [const, virtual]
    /// try to set the given parameter for this carFollowingModel
    // void
    // MSCFModel_CACC::setParameter(MSVehicle* veh, const std::string& key, const std::string& value) const {
    //     CACCVehicleVariables* vars = (CACCVehicleVariables*) veh->getCarFollowVariables();
    //
    //     try {
    //         if (key.compare("caccCommunicationsOverrideMode") == 0) {
    //             vars->CACC_CommunicationsOverrideMode = CommunicationsOverrideModeMap[value];
    //         }
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Invalid value '" + value + "' for parameter '" + key + "' for vehicle '" + veh->getID() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&self, veh: Ptr<MSVehicle>, key: &str, value: &str) {
        todo!("port MSCFModel_CACC::setParameter")
    }

    /// C++: `int MSCFModel_CACC::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_CACC;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_CACC::getModelID")
    }

    /// C++: `double MSCFModel_CACC::maximumLaneSpeedCF(const MSVehicle *const, double, double) const` [const, virtual]
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
        todo!("port MSCFModel_CACC::maximumLaneSpeedCF")
    }

    /// C++: `MSCFModel * MSCFModel_CACC::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_CACC::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_CACC(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_CACC::duplicate")
    }

    /// C++: `MSCFModel::VehicleVariables * MSCFModel_CACC::createVehicleVariables() const` [const, virtual]
    // virtual MSCFModel::VehicleVariables* createVehicleVariables() const {
    //         CACCVehicleVariables* ret = new CACCVehicleVariables();
    //         ret->ACC_ControlMode = 0;
    //         ret->CACC_ControlMode = 0;
    //         ret->CACC_CommunicationsOverrideMode = CACC_NO_OVERRIDE;
    //         ret->lastUpdateTime = 0;
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        todo!("port MSCFModel_CACC::createVehicleVariables")
    }

    /// C++: `double MSCFModel_CACC::_v(const MSVehicle *const, const MSVehicle *const, const double, const double, const double, const double, const bool, const CalcReason) const` [const, private]
    // double
    // MSCFModel_CACC::_v(const MSVehicle* const veh, const MSVehicle* const pred, const double gap2pred, const double speed,
    //                    const double predSpeed, const double desSpeed, const bool /* respectMinGap */, const CalcReason usage) const {
    //     double newSpeed = 0.0;
    //     VehicleMode vehMode = CC_MODE;
    //
    // #if DEBUG_CACC == 1
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " MSCFModel_CACC::_v() for veh '" << veh->getID()
    //                   << " gap=" << gap2pred << " speed="  << speed << " predSpeed=" << predSpeed
    //                   << " desSpeed=" << desSpeed << std::endl;
    //     }
    // #endif
    //
    //     /* Velocity error */
    //     double vErr = speed - desSpeed;
    //     bool setControlMode = false;
    //     CACCVehicleVariables* vars = (CACCVehicleVariables*)veh->getCarFollowVariables();
    //     if (vars->lastUpdateTime != MSNet::getInstance()->getCurrentTimeStep()) {
    //         vars->lastUpdateTime = MSNet::getInstance()->getCurrentTimeStep();
    //         setControlMode = true;
    //     }
    //
    //     CommunicationsOverrideMode commMode = vars->CACC_CommunicationsOverrideMode;
    //
    //     if (commMode == CACC_NO_OVERRIDE) { // old CACC logic (rely on time gap from predecessor)
    //         // @note: using (gap2pred + minGap) here increases oscillations but may
    //         // actually be a good idea once the acceleration-spike-problem is fixed
    //         double time_gap = gap2pred / MAX2(NUMERICAL_EPS, speed);
    //         double spacingErr = gap2pred - myHeadwayTime * speed;
    //
    //         if (time_gap > 2 && spacingErr > mySpeedControlMinGap) {
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying speedControl" << "  time_gap=" << time_gap << std::endl;
    //             }
    // #endif
    //             // Find acceleration - Speed control law
    //             newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //             // Set cl to vehicle parameters
    //             if (setControlMode) {
    //                 vars->CACC_ControlMode = 0;
    //             }
    //         } else if (time_gap < 1.5) {
    //             // Find acceleration - Gap control law
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        speedGapControl" << "  time_gap=" << time_gap << std::endl;
    //             }
    // #endif
    //
    //             newSpeed = speedGapControl(veh, gap2pred, speed, predSpeed, desSpeed, vErr, pred, vehMode);
    //             // Set cl to vehicle parameters
    //             if (setControlMode) {
    //                 vars->CACC_ControlMode = 1;
    //             }
    //         } else {
    //             // Follow previous applied law
    //             int cm = vars->CACC_ControlMode;
    //             if (!cm) {
    //                 // CACC_ControlMode = speed control
    //
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying speedControl (previous)" <<  "  time_gap=" << time_gap << std::endl;
    //                 }
    // #endif
    //                 newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //             } else {
    //                 // CACC_ControlMode = gap control
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        previous speedGapControl (previous)" << "  time_gap=" << time_gap << std::endl;
    //                 }
    // #endif
    //                 newSpeed = speedGapControl(veh, gap2pred, speed, predSpeed, desSpeed, vErr, pred, vehMode);
    //             }
    //         }
    //     } else if (commMode == CACC_MODE_NO_LEADER) {
    //         newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //     } else if (commMode == CACC_MODE_LEADER_NO_CAV) {
    //         newSpeed = acc_CFM._v(veh, gap2pred, speed, predSpeed, desSpeed, true);
    //         vehMode = ACC_MODE;
    //     } else if (commMode == CACC_MODE_LEADER_CAV) {
    //         double desSpacing = myHeadwayTime * speed;
    //         double spacingErr = gap2pred - desSpacing;
    //         double accel = veh->getAcceleration();
    //         double speedErr = predSpeed - speed - myHeadwayTime * accel;
    //
    //         if ((spacingErr > 0 && spacingErr < 0.2) && (vErr < 0.1)) {
    //             // gap mode
    //             //newSpeed = speed + 0.45 * spacingErr + 0.0125 *speedErr;
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying CACC_GAP_MODE " << std::endl;
    //             }
    //             newSpeed = speed + myGapControlGainGap * spacingErr + myGapControlGainGapDot * speedErr;
    //             vehMode = CACC_GAP_MODE;
    //         } else if (spacingErr < 0) {
    //             // collision avoidance mode
    //             //newSpeed = speed + 0.45 * spacingErr + 0.05 *speedErr;
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying CACC_COLLISION_AVOIDANCE_MODE " << std::endl;
    //             }
    //             newSpeed = speed + myCollisionAvoidanceGainGap * spacingErr + myCollisionAvoidanceGainGapDot * speedErr;
    //             vehMode = CACC_COLLISION_AVOIDANCE_MODE;
    //         } else {
    //             // gap closing mode
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying CACC_GAP_CLOSING_MODE " << std::endl;
    //             }
    //             newSpeed = speed + myGapClosingControlGainGap * spacingErr + myGapClosingControlGainGapDot * speedErr;
    //             vehMode = CACC_GAP_CLOSING_MODE;
    //         }
    //     }
    //
    //     if (setControlMode && !MSGlobals::gComputeLC && usage == CalcReason::CURRENT) {
    //         const_cast<SUMOVehicleParameter&>(veh->getParameter()).setParameter("caccVehicleMode", VehicleModeNames[vehMode]);
    //     }
    //
    //     //std::cout << veh->getID() << " commMode: " << commMode << ", caccVehicleMode: " << VehicleModeNames[vehMode]
    //     //            << ", gap2pred: " << gap2pred << ", newSpeed: " << newSpeed << std::endl;
    //
    //     // newSpeed is meant for step length 0.1 but this would cause extreme
    //     // accelerations at lower step length
    //     double newSpeedScaled = newSpeed;
    //     if (DELTA_T < 100) {
    //         const double accel01 = (newSpeed - speed) * 10;
    //         newSpeedScaled = speed + ACCEL2SPEED(accel01);
    //     }
    //
    // #if DEBUG_CACC == 1
    //     if (DEBUG_COND) {
    //         std::cout << "        result: rawAccel=" << SPEED2ACCEL(newSpeed - speed) << " newSpeed=" << newSpeed << " newSpeedScaled=" << newSpeedScaled << "\n";
    //     }
    // #endif
    //
    //     return MAX2(0., newSpeedScaled);
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, gap2pred: f64, mySpeed: f64, predSpeed: f64, desSpeed: f64, respectMinGap: bool, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_CACC::_v")
    }

    /// C++: `double MSCFModel_CACC::speedSpeedControl(const double, double, VehicleMode &) const` [const, private]
    // double
    // MSCFModel_CACC::speedSpeedControl(const double speed, double vErr, VehicleMode& vehMode) const {
    //     // Speed control law
    //     vehMode = CC_MODE;
    //     double sclAccel = mySpeedControlGain * vErr;
    //     double newSpeed = speed + ACCEL2SPEED(sclAccel);
    //     return newSpeed;
    // }
    #[allow(unused_variables)]
    pub fn speed_speed_control(&self, speed: f64, vErr: f64, vehMode: &mut MSCFModel_CACC_VehicleMode) -> f64 {
        todo!("port MSCFModel_CACC::speedSpeedControl")
    }

    /// C++: `double MSCFModel_CACC::speedGapControl(const MSVehicle *const, const double, const double, const double, const double, double, const MSVehicle *const, VehicleMode &) const` [const, private]
    // double
    // MSCFModel_CACC::speedGapControl(const MSVehicle* const veh, const double gap2pred,
    //                                 const double speed, const double predSpeed, const double desSpeed, double vErr,
    //                                 const MSVehicle* const pred, VehicleMode& vehMode) const {
    //     // Gap control law
    //     double newSpeed = 0.0;
    //
    //     if (pred != nullptr) {
    //         if (pred->getCarFollowModel().getModelID() != SUMO_TAG_CF_CACC) {
    //             vehMode = ACC_MODE;
    //             newSpeed = acc_CFM._v(veh, gap2pred, speed, predSpeed, desSpeed, true);
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        acc control mode" << std::endl;
    //             }
    // #endif
    //         } else {
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        CACC control mode" << std::endl;
    //             }
    // #endif
    //             double desSpacing = myHeadwayTime * speed;
    //             double spacingErr = gap2pred - desSpacing;
    //             double accel = veh->getAcceleration();
    //             double speedErr = predSpeed - speed - myHeadwayTime * accel;
    //
    //             if ((spacingErr > 0 && spacingErr < 0.2) && (vErr < 0.1)) {
    //                 // gap mode
    //                 //newSpeed = speed + 0.45 * spacingErr + 0.0125 *speedErr;
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying gap control:  err=" << spacingErr << " speedErr=" << speedErr << std::endl;
    //                 }
    // #endif
    //                 newSpeed = speed + myGapControlGainGap * spacingErr + myGapControlGainGapDot * speedErr;
    //
    //                 vehMode = CACC_GAP_MODE;
    //             } else if (spacingErr < 0) {
    //                 // collision avoidance mode
    //                 //newSpeed = speed + 0.45 * spacingErr + 0.05 *speedErr;
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying collision avoidance:  err=" << spacingErr << " speedErr=" << speedErr << "\n";
    //                 }
    // #endif
    //                 newSpeed = speed + myCollisionAvoidanceGainGap * spacingErr + myCollisionAvoidanceGainGapDot * speedErr;
    //                 vehMode = CACC_COLLISION_AVOIDANCE_MODE;
    //             } else {
    //                 // gap closing mode
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying gap closing err=" << spacingErr << " speedErr=" << speedErr << " predSpeed=" << predSpeed << " speed=" << speed << " accel=" << accel << "\n";
    //                 }
    // #endif
    //                 newSpeed = speed + myGapClosingControlGainGap * spacingErr + myGapClosingControlGainGapDot * speedErr;
    //
    //                 vehMode = CACC_GAP_CLOSING_MODE;
    //             }
    //         }
    //     } else {
    //         /* no leader */
    // #if DEBUG_CACC == 1
    //         if (DEBUG_COND) {
    //             std::cout << "        no leader" << std::endl;
    //         }
    // #endif
    //         newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //     }
    //
    //     return newSpeed;
    // }
    #[allow(unused_variables)]
    pub fn speed_gap_control(&self, veh: Ptr<MSVehicle>, gap2pred: f64, speed: f64, predSpeed: f64, desSpeed: f64, vErr: f64, pred: Ptr<MSVehicle>, vehMode: &mut MSCFModel_CACC_VehicleMode) -> f64 {
        todo!("port MSCFModel_CACC::speedGapControl")
    }

    /// C++: `MSCFModel_CACC & MSCFModel_CACC::operator=(const MSCFModel_CACC &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSCFModel_CACC) -> &mut MSCFModel_CACC {
        todo!("port MSCFModel_CACC::operator=")
    }

}

