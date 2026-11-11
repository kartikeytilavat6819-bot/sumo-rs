//! Port of microsim/cfmodels/MSCFModel_PWag2009.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Scalable model based on Krauss by Peter Wagner
/// C++ class `MSCFModel_PWag2009` (microsim/cfmodels/MSCFModel_PWag2009.h)
pub struct MSCFModel_PWag2009 {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// The vehicle's dawdle-parameter. 0 for no dawdling, 1 for max.
    pub myDawdle: f64,
    /// The precomputed value for myDecel*myTau
    pub myTauDecel: f64,
    /// The precomputed value for myDecel/myTau
    pub myDecelDivTau: f64,
    /// The precomputed value for (minimum headway time)*myDecel
    pub myTauLastDecel: f64,
    /// The probability for any action
    pub myActionPointProbability: f64,
}

impl std::ops::Deref for MSCFModel_PWag2009 {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_PWag2009 {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_PWag2009 {
    /// C++: `void MSCFModel_PWag2009::MSCFModel_PWag2009(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_PWag2009::MSCFModel_PWag2009(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     myDawdle(vtype->getParameter().getCFParam(SUMO_ATTR_SIGMA, SUMOVTypeParameter::getDefaultImperfection(vtype->getParameter().vehicleClass))),
    //     myTauDecel(myDecel * myHeadwayTime),
    //     myDecelDivTau(myDecel / myHeadwayTime),
    //     myTauLastDecel(myDecel * vtype->getParameter().getCFParam(SUMO_ATTR_CF_PWAGNER2009_TAULAST, 0.3)),
    //     myActionPointProbability(vtype->getParameter().getCFParam(SUMO_ATTR_CF_PWAGNER2009_APPROB, 0.5)) {
    //     // PWag2009 does not drive very precise and may violate minGap on occasion
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        todo!("port MSCFModel_PWag2009::MSCFModel_PWag2009")
    }

    /// C++: `void MSCFModel_PWag2009::~MSCFModel_PWag2009() noexcept` [virtual]
    /// Destructor
    // MSCFModel_PWag2009::~MSCFModel_PWag2009() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_PWag2009::~MSCFModel_PWag2009")
    }

    /// C++: `double MSCFModel_PWag2009::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences
    // double
    // MSCFModel_PWag2009::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     const double vNext = MSCFModel::finalizeSpeed(veh, vPos);
    //     VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //     double apref = SPEED2ACCEL(vNext - veh->getSpeed());
    //     vars->aOld = apref;
    //     return vNext;
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        todo!("port MSCFModel_PWag2009::finalizeSpeed")
    }

    /// C++: `double MSCFModel_PWag2009::patchSpeedBeforeLC(const MSVehicle *, double, double) const` [const, virtual]
    /// apply dawdling
    // double
    // MSCFModel_PWag2009::patchSpeedBeforeLC(const MSVehicle* veh, double vMin, double vMax) const {
    //     UNUSED_PARAMETER(veh);
    //     UNUSED_PARAMETER(vMin);
    //     return vMax;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_before_lc(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        todo!("port MSCFModel_PWag2009::patchSpeedBeforeLC")
    }

    /// C++: `double MSCFModel_PWag2009::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_PWag2009::followSpeed(const MSVehicle* const veh, double speed, double gap, double predSpeed, double /*predMaxDecel*/, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //     if (predSpeed == 0 && gap < 0.01) {
    //         return 0;
    //     }
    //     const double vsafe = -myTauLastDecel + sqrt(myTauLastDecel * myTauLastDecel + predSpeed * predSpeed + 2.0 * myDecel * gap);
    //     const double asafe = SPEED2ACCEL(vsafe - speed);
    //     VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //     double apref = vars->aOld;
    //     if (apref <= asafe && RandHelper::rand(veh->getRNG()) <= myActionPointProbability * TS) {
    //         apref = myDecelDivTau * (gap + (predSpeed - speed) * myHeadwayTime - speed * myHeadwayTime) / (speed + myTauDecel);
    //         apref = MIN2(apref, myAccel);
    //         apref = MAX2(apref, -myDecel);
    //         apref += myDawdle * RandHelper::rand((double) - 1., (double)1., veh->getRNG());
    //     }
    //     if (apref > asafe) {
    //         apref = asafe;
    //     }
    //     return MAX2(0., speed + ACCEL2SPEED(apref));
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_PWag2009::followSpeed")
    }

    /// C++: `double MSCFModel_PWag2009::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_PWag2009::stopSpeed(const MSVehicle* const /* veh */, const double speed, double gap, double /*decel*/, const CalcReason /*usage*/) const {
    //     if (gap < 0.01) {
    //         return 0.;
    //     }
    //     const double vsafe = -myTauDecel + sqrt(myTauDecel * myTauDecel +  2.0 * myDecel * gap);
    //     const double asafe = SPEED2ACCEL(vsafe - speed);
    // //    VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //     double apref = myDecelDivTau * (gap  - 2 * speed * myHeadwayTime) / (speed + myTauDecel);
    //     if (apref <= asafe) {
    //         apref = MIN2(apref, myAccel);
    //         apref = MAX2(apref, -myDecel);
    //     } else {
    //         apref = asafe;
    //     }
    //     return MAX2(0., vsafe + ACCEL2SPEED(apref));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_PWag2009::stopSpeed")
    }

    /// C++: `int MSCFModel_PWag2009::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_PWAGNER2009;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_PWag2009::getModelID")
    }

    /// C++: `double MSCFModel_PWag2009::getImperfection() const` [const, virtual]
    /// Get the driver's imperfection
    // double getImperfection() const {
    //         return myDawdle;
    //     }
    #[allow(unused_variables)]
    pub fn get_imperfection(&self) -> f64 {
        todo!("port MSCFModel_PWag2009::getImperfection")
    }

    /// C++: `MSCFModel * MSCFModel_PWag2009::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_PWag2009::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_PWag2009(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_PWag2009::duplicate")
    }

    /// C++: `MSCFModel::VehicleVariables * MSCFModel_PWag2009::createVehicleVariables() const` [const, virtual]
    // virtual MSCFModel::VehicleVariables* createVehicleVariables() const {
    //         VehicleVariables* ret = new VehicleVariables();
    //         ret->aOld = 0.0;
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        todo!("port MSCFModel_PWag2009::createVehicleVariables")
    }

    /// C++: `double MSCFModel_PWag2009::_v(const MSVehicle *const, double, double, double) const` [const, private]
    /// Returns the next velocity
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, predSpeed: f64) -> f64 {
        todo!("port MSCFModel_PWag2009::_v")
    }

    /// C++: `double MSCFModel_PWag2009::dawdle(double) const` [const, private]
    /// Applies driver imperfection (dawdling / sigma)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn dawdle(&self, speed: f64) -> f64 {
        todo!("port MSCFModel_PWag2009::dawdle")
    }

}

/// C++ class `MSCFModel_PWag2009::VehicleVariables` (microsim/cfmodels/MSCFModel_PWag2009.h)
pub struct MSCFModel_PWag2009_VehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    pub aOld: f64,
}

impl std::ops::Deref for MSCFModel_PWag2009_VehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_PWag2009_VehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_PWag2009_VehicleVariables {
    /// C++: `void VehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_PWag2009::VehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "PWagner2009");
    //     std::ostringstream internals;
    //     internals << aOld;
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
    // MSCFModel_PWag2009::VehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "PWagner2009") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for PWagner2009", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> aOld;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port VehicleVariables::loadState")
    }

}

