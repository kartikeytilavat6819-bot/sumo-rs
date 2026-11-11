//! Port of microsim/cfmodels/MSCFModel_Krauss.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Krauss car-following model, with acceleration decrease and faster start
/// C++ class `MSCFModel_Krauss` (microsim/cfmodels/MSCFModel_Krauss.h)
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
        todo!("port MSCFModel_Krauss::MSCFModel_Krauss")
    }

    /// C++: `void MSCFModel_Krauss::~MSCFModel_Krauss() noexcept` [virtual]
    /// Destructor
    // MSCFModel_Krauss::~MSCFModel_Krauss() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_Krauss::~MSCFModel_Krauss")
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
        todo!("port MSCFModel_Krauss::patchSpeedBeforeLC")
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
        todo!("port MSCFModel_Krauss::stopSpeed")
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
        todo!("port MSCFModel_Krauss::followSpeed")
    }

    /// C++: `int MSCFModel_Krauss::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_KRAUSS;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_Krauss::getModelID")
    }

    /// C++: `MSCFModel * MSCFModel_Krauss::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_Krauss::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Krauss(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_Krauss::duplicate")
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
        todo!("port MSCFModel_Krauss::createVehicleVariables")
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
        todo!("port MSCFModel_Krauss::dawdle2")
    }

}

/// C++ class `MSCFModel_Krauss::VehicleVariables` (microsim/cfmodels/MSCFModel_Krauss.h)
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
        todo!("port VehicleVariables::VehicleVariables")
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
        todo!("port VehicleVariables::saveState")
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
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port VehicleVariables::loadState")
    }

}

