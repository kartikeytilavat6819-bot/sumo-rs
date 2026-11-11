//! Port of microsim/cfmodels/MSCFModel_Kerner.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// car-following model by B. Kerner
/// C++ class `MSCFModel_Kerner` (microsim/cfmodels/MSCFModel_Kerner.h)
pub struct MSCFModel_Kerner {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// Kerner's k
    pub myK: f64,
    /// Kerner's phi
    pub myPhi: f64,
    /// The precomputed value for myDecel*myTau
    pub myTauDecel: f64,
}

impl std::ops::Deref for MSCFModel_Kerner {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Kerner {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_Kerner {
    /// C++: `void MSCFModel_Kerner::MSCFModel_Kerner(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_Kerner::MSCFModel_Kerner(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     myK(vtype->getParameter().getCFParam(SUMO_ATTR_K, 0.5)),
    //     myPhi(vtype->getParameter().getCFParam(SUMO_ATTR_CF_KERNER_PHI, 5.0)),
    //     myTauDecel(myDecel * myHeadwayTime) {
    //     // Kerner does not drive very precise and may violate minGap on occasion
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        todo!("port MSCFModel_Kerner::MSCFModel_Kerner")
    }

    /// C++: `void MSCFModel_Kerner::~MSCFModel_Kerner() noexcept` [virtual]
    /// Destructor
    // MSCFModel_Kerner::~MSCFModel_Kerner() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_Kerner::~MSCFModel_Kerner")
    }

    /// C++: `double MSCFModel_Kerner::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences
    // double
    // MSCFModel_Kerner::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     const double vNext = MSCFModel::finalizeSpeed(veh, vPos);
    //     VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //     vars->rand = RandHelper::rand(veh->getRNG());
    //     return vNext;
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        todo!("port MSCFModel_Kerner::finalizeSpeed")
    }

    /// C++: `double MSCFModel_Kerner::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_Kerner::followSpeed(const MSVehicle* const veh, double speed, double gap, double predSpeed, double /*predMaxDecel*/, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //     return MIN2(_v(veh, speed, maxNextSpeed(speed, veh), gap, predSpeed), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_Kerner::followSpeed")
    }

    /// C++: `double MSCFModel_Kerner::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_Kerner::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double /*decel*/, const CalcReason /*usage*/) const {
    //     return MIN2(_v(veh, speed, maxNextSpeed(speed, veh), gap, 0), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_Kerner::stopSpeed")
    }

    /// C++: `int MSCFModel_Kerner::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_BKERNER;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_Kerner::getModelID")
    }

    /// C++: `MSCFModel * MSCFModel_Kerner::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_Kerner::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Kerner(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_Kerner::duplicate")
    }

    /// C++: `MSCFModel::VehicleVariables * MSCFModel_Kerner::createVehicleVariables() const` [const, virtual]
    // MSCFModel::VehicleVariables*
    // MSCFModel_Kerner::createVehicleVariables() const {
    //     VehicleVariables* ret = new VehicleVariables();
    //     /// XXX should use egoVehicle->getRNG()
    //     ret->rand = RandHelper::rand();
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        todo!("port MSCFModel_Kerner::createVehicleVariables")
    }

    /// C++: `double MSCFModel_Kerner::_v(const MSVehicle *const, double, double, double, double) const` [const, private]
    /// Returns the "safe" velocity
    // double
    // MSCFModel_Kerner::_v(const MSVehicle* const veh, double speed, double vfree, double gap, double predSpeed) const {
    //     if (predSpeed == 0 && gap < 0.01) {
    //         return 0;
    //     }
    //     // !!! in the following, the prior step is not considered!!!
    //     double G = MAX2((double) 0, (double)(SPEED2DIST(myK * speed) + myPhi / myAccel * speed * (speed - predSpeed)));
    //     double vcond = gap > G ? speed + ACCEL2SPEED(myAccel) : speed + MAX2(ACCEL2SPEED(-myDecel), MIN2(ACCEL2SPEED(myAccel), predSpeed - speed));
    //     double vsafe = (double)(-1. * myTauDecel + sqrt(myTauDecel * myTauDecel + (predSpeed * predSpeed) + (2. * myDecel * gap)));
    //     VehicleVariables* vars = (VehicleVariables*)veh->getCarFollowVariables();
    //     double va = MAX2((double) 0, MIN3(vfree, vsafe, vcond)) + vars->rand;
    //     //std::cout << SIMTIME << " veh=" << veh->getID() << " speed=" << speed << " gap=" << gap << " G=" << G << " predSpeed=" << predSpeed << " vfree=" << vfree << " vsafe=" << vsafe << " vcond=" << vcond << " rand=" << vars->rand << "\n";
    //     double v = MAX2((double) 0, MIN4(vfree, va, speed + ACCEL2SPEED(myAccel), vsafe));
    //     return v;
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, speed: f64, vfree: f64, gap: f64, predSpeed: f64) -> f64 {
        todo!("port MSCFModel_Kerner::_v")
    }

}

/// C++ class `MSCFModel_Kerner::VehicleVariables` (microsim/cfmodels/MSCFModel_Kerner.h)
pub struct MSCFModel_Kerner_VehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    pub rand: f64,
}

impl std::ops::Deref for MSCFModel_Kerner_VehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Kerner_VehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_Kerner_VehicleVariables {
    /// C++: `void VehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_Kerner::VehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "BKerner");
    //     std::ostringstream internals;
    //     internals << rand;
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
    // MSCFModel_Kerner::VehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "BKerner") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for BKerner", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> rand;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port VehicleVariables::loadState")
    }

}

