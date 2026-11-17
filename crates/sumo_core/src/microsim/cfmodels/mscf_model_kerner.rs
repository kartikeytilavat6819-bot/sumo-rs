//! Port of microsim/cfmodels/MSCFModel_Kerner.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_default;

/// car-following model by B. Kerner
/// C++ class `MSCFModel_Kerner` (microsim/cfmodels/MSCFModel_Kerner.h)
#[repr(C)]
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

fn kerner_finalize_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
    cf_cast::<MSCFModel_Kerner>(m).finalize_speed(veh, vPos)
}
fn kerner_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_Kerner>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage)
}
fn kerner_get_model_id(m: &MSCFModel) -> i32 {
    cf_cast::<MSCFModel_Kerner>(m).get_model_id()
}
fn kerner_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
    cf_cast::<MSCFModel_Kerner>(m).duplicate(vtype)
}
fn kerner_save_vehicle_variables(m: &MSCFModel, vars: &MSCFModel_VehicleVariables, out: &mut OutputDevice) {
    unsafe { &*(vars as *const MSCFModel_VehicleVariables as *const MSCFModel_Kerner_VehicleVariables) }.save_state(out, m)
}
fn kerner_load_vehicle_variables(_m: &MSCFModel, vars: &mut MSCFModel_VehicleVariables, attrs: &SUMOSAXAttributes) {
    unsafe { &mut *(vars as *mut MSCFModel_VehicleVariables as *mut MSCFModel_Kerner_VehicleVariables) }.load_state(attrs).unwrap_or_else(|e| panic!("{}", e))
}
fn kerner_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> {
    cf_cast::<MSCFModel_Kerner>(m).create_vehicle_variables()
}

fn kerner_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_Kerner>(m).stop_speed(veh, speed, gap, decel, usage)
}

/// Virtual table of MSCFModel_Kerner: its overrides on top of the MSCFModel table
pub const KERNER_VT: MSCFModelVt = MSCFModelVt {
    stop_speed_2: kerner_stop_speed_2,
    finalize_speed: kerner_finalize_speed,
    follow_speed: kerner_follow_speed,
    get_model_id: kerner_get_model_id,
    duplicate: kerner_duplicate,
    create_vehicle_variables: kerner_create_vehicle_variables,
    save_vehicle_variables: kerner_save_vehicle_variables,
    load_vehicle_variables: kerner_load_vehicle_variables,
    ..DEFAULT_MSCF_MODEL_VT
};

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
        let myK = vtype.get_parameter().get_cf_param(SUMO_ATTR_K, 0.5);
        let myPhi = vtype.get_parameter().get_cf_param(SUMO_ATTR_CF_KERNER_PHI, 5.0);
        let mut base = MSCFModel::new(vtype);
        let myTauDecel = base.myDecel * base.myHeadwayTime;
        // Kerner does not drive very precise and may violate minGap on occasion
        base.myCollisionMinGapFactor = vtype.get_parameter().get_cf_param(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
        let mut this = MSCFModel_Kerner { base, myK, myPhi, myTauDecel };
        this.base.vt = &KERNER_VT;
        this
    }

    /// C++: `void MSCFModel_Kerner::~MSCFModel_Kerner() noexcept` [virtual]
    /// Destructor
    // MSCFModel_Kerner::~MSCFModel_Kerner() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
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
        let vNext = self.base.finalize_speed_base(veh, vPos);
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_Kerner_VehicleVariables);
        vars.m().rand = veh.get_rng().m().rand();
        vNext
    }

    /// C++: `double MSCFModel_Kerner::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_Kerner::followSpeed(const MSVehicle* const veh, double speed, double gap, double predSpeed, double /*predMaxDecel*/, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //     return MIN2(_v(veh, speed, maxNextSpeed(speed, veh), gap, predSpeed), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        min2(self.v(veh, speed, self.max_next_speed(speed, veh), gap2pred, predSpeed), self.max_next_speed(speed, veh))
    }

    /// C++: `double MSCFModel_Kerner::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_Kerner::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double /*decel*/, const CalcReason /*usage*/) const {
    //     return MIN2(_v(veh, speed, maxNextSpeed(speed, veh), gap, 0), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        min2(self.v(veh, speed, self.max_next_speed(speed, veh), gap2pred, 0.), self.max_next_speed(speed, veh))
    }

    /// C++: `int MSCFModel_Kerner::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_BKERNER;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_BKERNER as i32
    }

    /// C++: `MSCFModel * MSCFModel_Kerner::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_Kerner::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Kerner(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Kerner::new(vtype))) as *mut MSCFModel)
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
        let rand = utils::common::rand_helper::rand();
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Kerner_VehicleVariables { base: MSCFModel_VehicleVariables {}, rand })) as *mut MSCFModel_VehicleVariables)
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
        if predSpeed == 0. && gap < 0.01 {
            return 0.;
        }
        // !!! in the following, the prior step is not considered!!!
        let G = max2(0., speed2dist(self.myK * speed) + self.myPhi / self.myAccel * speed * (speed - predSpeed));
        let vcond = if gap > G {
            speed + accel2speed(self.myAccel)
        } else {
            speed + max2(accel2speed(-self.myDecel), min2(accel2speed(self.myAccel), predSpeed - speed))
        };
        let vsafe = -1. * self.myTauDecel + (self.myTauDecel * self.myTauDecel + (predSpeed * predSpeed) + (2. * self.myDecel * gap)).sqrt();
        let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_Kerner_VehicleVariables);
        let va = max2(0., min3(vfree, vsafe, vcond)) + vars.rand;
        let v = max2(0., min2(min2(vfree, va), min2(speed + accel2speed(self.myAccel), vsafe)));
        v
    }

}

/// C++ class `MSCFModel_Kerner::VehicleVariables` (microsim/cfmodels/MSCFModel_Kerner.h)
#[repr(C)]
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
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "BKerner");
        let internals = f64_to_string_default(self.rand);
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
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
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "BKerner" {
            return Err(utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for BKerner", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        if let Some(v) = bis.next().and_then(|s| s.parse::<f64>().ok()) {
            self.rand = v;
        }
        Ok(())
    }

}

