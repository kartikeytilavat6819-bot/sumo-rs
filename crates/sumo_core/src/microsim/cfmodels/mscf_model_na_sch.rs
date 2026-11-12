//! Port of microsim/cfmodels/MSCFModel_NaSch.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The Nagel-Schreckenberg (1992) cellular automaton car-following model
/// C++ class `MSCFModel_NaSch` (microsim/cfmodels/MSCFModel_NaSch.h)
#[repr(C)]
pub struct MSCFModel_NaSch {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    /// The probability of randomized deceleration by one cell per step ("sigma")
    pub myDawdle: f64,
    /// The speed gained per step by accelerating at myAccel, i.e. one cell's worth of speed
    pub myCellSpeed: f64,
}

impl std::ops::Deref for MSCFModel_NaSch {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_NaSch {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_NaSch {
    /// C++: `void MSCFModel_NaSch::MSCFModel_NaSch(const MSVehicleType *, double)`
    /// Constructor
    // MSCFModel_NaSch::MSCFModel_NaSch(const MSVehicleType* vtype, double dawdle) :
    //     MSCFModel(vtype),
    //     myDawdle(dawdle),
    //     // The classic NaSch cell length is the distance covered by accelerating
    //     // for one simulation step, i.e. the "accel" attribute doubles as the
    //     // cell size (in meters) when using the default step-length of 1s,
    //     // see https://github.com/eclipse-sumo/sumo/issues/12182
    //     myCellSpeed(ACCEL2SPEED(myAccel)) {
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>, dawdle: f64) -> Self {
        todo!("port MSCFModel_NaSch::MSCFModel_NaSch")
    }

    /// C++: `void MSCFModel_NaSch::~MSCFModel_NaSch() noexcept` [virtual]
    /// Destructor
    // MSCFModel_NaSch::~MSCFModel_NaSch() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_NaSch::~MSCFModel_NaSch")
    }

    /// C++: `double MSCFModel_NaSch::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_NaSch::followSpeed(const MSVehicle* const veh, double speed, double gap2pred,
    //                              double /*predSpeed*/, double /*predMaxDecel*/, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //     return vsafe(veh, speed, gap2pred);
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_NaSch::followSpeed")
    }

    /// C++: `double MSCFModel_NaSch::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling) This method is used during the insertion stage. Whereas the method followSpeed returns the desired speed which may be lower than the safe speed, this method only considers safety constraints
    // double
    // MSCFModel_NaSch::insertionFollowSpeed(const MSVehicle* const veh, double speed, double gap2pred, double /*predSpeed*/, double /*predMaxDecel*/, const MSVehicle* const /*pred*/) const {
    //     return vsafe(veh, speed, gap2pred);
    // }
    #[allow(unused_variables)]
    pub fn insertion_follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
        todo!("port MSCFModel_NaSch::insertionFollowSpeed")
    }

    /// C++: `double MSCFModel_NaSch::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_NaSch::stopSpeed(const MSVehicle* const veh, const double speed, double gap2pred, double /*decel*/, const CalcReason /*usage*/) const {
    //     return vsafe(veh, speed, gap2pred);
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_NaSch::stopSpeed")
    }

    /// C++: `double MSCFModel_NaSch::patchSpeedBeforeLC(const MSVehicle *, double, double) const` [const, virtual]
    /// NaSch rule 3 ("randomization"): dawdle by exactly one cell with probability sigma
    // double
    // MSCFModel_NaSch::patchSpeedBeforeLC(const MSVehicle* veh, double vMin, double vMax) const {
    //     // NaSch rule 3 (randomization): with probability sigma, dawdle by exactly
    //     // one cell -- not a continuously distributed amount as in the Krauss dawdle.
    //     if (vMax > 0. && myDawdle > 0. && RandHelper::rand(veh->getRNG()) < myDawdle) {
    //         return MAX2(vMin, roundToCell(vMax - myCellSpeed));
    //     }
    //     return vMax;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_before_lc(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        todo!("port MSCFModel_NaSch::patchSpeedBeforeLC")
    }

    /// C++: `double MSCFModel_NaSch::minNextSpeedEmergency(double, const MSVehicle *const) const` [const, virtual]
    /// NaSch does not impose an extra bound on emergency braking: a vehicle may always come to a full (Euler) resp. arbitrarily quick (ballistic) stop within a single step if the gap requires it.
    // double
    // MSCFModel_NaSch::minNextSpeedEmergency(double /*speed*/, const MSVehicle* const /*veh*/) const {
    //     // NaSch rule 4 (movement) permits vehicles to stop dead within a single
    //     // step whenever the gap requires it, i.e. emergency deceleration is not
    //     // bounded, see issue #12182 ("do not bound emergency decel").
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return 0.;
    //     }
    //     return -std::numeric_limits<double>::max();
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed_emergency(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        todo!("port MSCFModel_NaSch::minNextSpeedEmergency")
    }

    /// C++: `int MSCFModel_NaSch::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_NASCH;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_NaSch::getModelID")
    }

    /// C++: `double MSCFModel_NaSch::getImperfection() const` [const, virtual]
    /// Get the driver's imperfection
    // double getImperfection() const {
    //         return myDawdle;
    //     }
    #[allow(unused_variables)]
    pub fn get_imperfection(&self) -> f64 {
        todo!("port MSCFModel_NaSch::getImperfection")
    }

    /// C++: `void MSCFModel_NaSch::setImperfection(double)` [virtual]
    /// Sets a new value for driver imperfection
    // void setImperfection(double imperfection) {
    //         myDawdle = imperfection;
    //     }
    #[allow(unused_variables)]
    pub fn set_imperfection(&mut self, imperfection: f64) {
        todo!("port MSCFModel_NaSch::setImperfection")
    }

    /// C++: `MSCFModel * MSCFModel_NaSch::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_NaSch::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_NaSch(vtype, myDawdle);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_NaSch::duplicate")
    }

    /// C++: `double MSCFModel_NaSch::roundToCell(double) const` [const, protected]
    /// Rounds a speed down to the nearest multiple of the cell speed (the speed gained by accelerating for exactly one simulation step). NaSch speeds always correspond to a whole number of cells per step.
    // double
    // MSCFModel_NaSch::roundToCell(double speed) const {
    //     if (speed <= 0. || myCellSpeed <= 0.) {
    //         return 0.;
    //     }
    //     return floor(speed / myCellSpeed + NUMERICAL_EPS) * myCellSpeed;
    // }
    #[allow(unused_variables)]
    pub fn round_to_cell(&self, speed: f64) -> f64 {
        todo!("port MSCFModel_NaSch::roundToCell")
    }

    /// C++: `double MSCFModel_NaSch::vsafe(const MSVehicle *const, double, double) const` [const, protected]
    /// the safe speed considering only the gap to the leader/obstacle (NaSch rule 1+2)
    // double
    // MSCFModel_NaSch::vsafe(const MSVehicle* const veh, double speed, double gap) const {
    //     if (gap < NUMERICAL_EPS) {
    //         return 0.;
    //     }
    //     // NaSch rule 1 (acceleration) followed by rule 2 (slowing down): unlike the
    //     // Krauss-derived models, the leader's speed does not enter the computation at
    //     // all -- the gap alone caps how far ego may move this step, see issue #12182
    //     // ("replace leaderSpeed with gap").
    //     const double vAccel = maxNextSpeed(speed, veh);
    //     const double vGapLimited = MIN2(vAccel, DIST2SPEED(gap));
    //     return roundToCell(MAX2(0., vGapLimited));
    // }
    #[allow(unused_variables)]
    pub fn vsafe(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64) -> f64 {
        todo!("port MSCFModel_NaSch::vsafe")
    }

}

