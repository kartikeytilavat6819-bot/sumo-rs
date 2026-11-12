//! Port of microsim/traffic_lights/MSSOTLPolicy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Class for a low-level policy.
/// C++ class `MSSOTLPolicy` (microsim/traffic_lights/MSSOTLPolicy.h)
#[repr(C)]
pub struct MSSOTLPolicy {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The sensitivity of this policy
    pub theta_sensitivity: f64,
    /// The name of the policy
    pub myName: String,
    /// A pointer to the policy desirability object.'s an optional component related to the computeDesirability() method and it's necessary only when the policy is used in combination with an high level policy.
    pub myDesirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>,
}

impl std::ops::Deref for MSSOTLPolicy {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPolicy {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl MSSOTLPolicy {
    /// C++: `void MSSOTLPolicy::init()` [virtual, protected]
    // virtual void init() {}
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        todo!("port MSSOTLPolicy::init")
    }

    /// C++: `void MSSOTLPolicy::MSSOTLPolicy(std::string, const Parameterised::Map &)`
    /// Simple constructor
    // MSSOTLPolicy::MSSOTLPolicy(std::string name,
    //                            const Parameterised::Map& parameters) :
    //     Parameterised(parameters), myName(name) {
    //     theta_sensitivity = 0;
    // }
    #[allow(unused_variables)]
    pub fn new(name: String, parameters: &ParameterisedMap) -> Self {
        todo!("port MSSOTLPolicy::MSSOTLPolicy")
    }

    /// C++: `void MSSOTLPolicy::MSSOTLPolicy(std::string, MSSOTLPolicyDesirability *)`
    /// Constructor when the policy is a low-level policy used by an high level policy
    // MSSOTLPolicy::MSSOTLPolicy(std::string name,
    //                            MSSOTLPolicyDesirability* desirabilityAlgorithm) :
    //     Parameterised(), myName(name), myDesirabilityAlgorithm(
    //         desirabilityAlgorithm) {
    //     theta_sensitivity = 0;
    // }
    #[allow(unused_variables)]
    pub fn new_2(name: String, desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>) -> Self {
        todo!("port MSSOTLPolicy::MSSOTLPolicy")
    }

    /// C++: `void MSSOTLPolicy::MSSOTLPolicy(std::string, MSSOTLPolicyDesirability *, const Parameterised::Map &)`
    /// Constructor when the policy is a low-level policy used by an high level policy
    // MSSOTLPolicy::MSSOTLPolicy(std::string name,
    //                            MSSOTLPolicyDesirability* desirabilityAlgorithm,
    //                            const Parameterised::Map& parameters) :
    //     Parameterised(parameters), myName(name), myDesirabilityAlgorithm(
    //         desirabilityAlgorithm) {
    //     theta_sensitivity = StringUtils::toDouble(getParameter("THETA_INIT", "0.5"));
    // }
    #[allow(unused_variables)]
    pub fn new_3(name: String, desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>, parameters: &ParameterisedMap) -> Self {
        todo!("port MSSOTLPolicy::MSSOTLPolicy")
    }

    /// C++: `void MSSOTLPolicy::~MSSOTLPolicy() noexcept` [virtual]
    // MSSOTLPolicy::~MSSOTLPolicy(void) {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSSOTLPolicy::~MSSOTLPolicy")
    }

    /// C++: `bool MSSOTLPolicy::canRelease(SUMOTime, bool, bool, const MSPhaseDefinition *, int)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn can_release(&mut self, elapsed: SUMOTime, thresholdPassed: bool, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
        todo!("port MSSOTLPolicy::canRelease")
    }

    /// C++: `int MSSOTLPolicy::decideNextPhase(SUMOTime, const MSPhaseDefinition *, int, int, bool, bool, int)` [virtual]
    // int MSSOTLPolicy::decideNextPhase(SUMOTime elapsed,
    //                                   const MSPhaseDefinition* stage, int currentPhaseIndex,
    //                                   int phaseMaxCTS, bool thresholdPassed, bool pushButtonPressed, int vehicleCount) {
    //
    //     //If the junction was in a commit step
    //     //=> go to the target step that gives green to the set with the current highest CTS
    //     //   and return computeReturnTime()
    //     if (stage->isCommit()) {
    //         // decide which chain to activate. Gotta work on this
    //         return phaseMaxCTS;
    //     }
    //     if (stage->isTransient()) {
    //         //If the junction was in a transient step
    //         //=> go to the next step and return computeReturnTime()
    //         return currentPhaseIndex + 1;
    //     }
    //
    //     if (stage->isDecisional()) {
    // #ifdef SWARM_DEBUG
    //         std::ostringstream phero_str;
    //         phero_str << "getCurrentPhaseElapsed()=" << time2string(elapsed) << " isThresholdPassed()=" << thresholdPassed << " countVehicles()=" << vehicleCount;
    //         WRITE_MESSAGE("MSSOTLPolicy::decideNextPhase: " + phero_str.str());
    // #endif
    //         if (canRelease(elapsed, thresholdPassed, pushButtonPressed, stage, vehicleCount)) {
    //             return currentPhaseIndex + 1;
    //         }
    //     }
    //
    //     return currentPhaseIndex;
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self, elapsed: SUMOTime, stage: Ptr<MSPhaseDefinition>, currentPhaseIndex: i32, phaseMaxCTS: i32, thresholdPassed: bool, pushButtonPressed: bool, vehicleCount: i32) -> i32 {
        todo!("port MSSOTLPolicy::decideNextPhase")
    }

    /// C++: `double MSSOTLPolicy::getThetaSensitivity()` [virtual]
    // virtual double getThetaSensitivity() {
    //         return theta_sensitivity;
    //     }
    #[allow(unused_variables)]
    pub fn get_theta_sensitivity(&mut self) -> f64 {
        todo!("port MSSOTLPolicy::getThetaSensitivity")
    }

    /// C++: `void MSSOTLPolicy::setThetaSensitivity(double)` [virtual]
    // virtual void setThetaSensitivity(double val) {
    //         theta_sensitivity = val;
    //     }
    #[allow(unused_variables)]
    pub fn set_theta_sensitivity(&mut self, val: f64) {
        todo!("port MSSOTLPolicy::setThetaSensitivity")
    }

    /// C++: `std::string MSSOTLPolicy::getName()`
    // std::string getName() {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&mut self) -> String {
        todo!("port MSSOTLPolicy::getName")
    }

    /// C++: `MSSOTLPolicyDesirability * MSSOTLPolicy::getDesirabilityAlgorithm()`
    // MSSOTLPolicyDesirability* getDesirabilityAlgorithm() {
    //         return myDesirabilityAlgorithm;
    //     }
    #[allow(unused_variables)]
    pub fn get_desirability_algorithm(&mut self) -> Ptr<MSSOTLPolicyDesirability> {
        todo!("port MSSOTLPolicy::getDesirabilityAlgorithm")
    }

    /// C++: `double MSSOTLPolicy::computeDesirability(double, double, double, double)`
    /// Computes the desirability of this policy, necessary when used in combination with an high level policy.
    // double MSSOTLPolicy::computeDesirability(double vehInMeasure, double vehOutMeasure, double vehInDispersionMeasure,	double vehOutDispersionMeasure) {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "\nMSSOTLPolicy::computeStimulus\n" << getName();
    //     WRITE_MESSAGE(str.str());
    // #endif
    //     return myDesirabilityAlgorithm->computeDesirability(vehInMeasure, vehOutMeasure, vehInDispersionMeasure, vehOutDispersionMeasure);
    // }
    #[allow(unused_variables)]
    pub fn compute_desirability(&mut self, vehInMeasure: f64, vehOutMeasure: f64, vehInDispersionMeasure: f64, vehOutDispersionMeasure: f64) -> f64 {
        todo!("port MSSOTLPolicy::computeDesirability")
    }

    /// C++: `double MSSOTLPolicy::computeDesirability(double, double)`
    // double MSSOTLPolicy::computeDesirability(double vehInMeasure, double vehOutMeasure) {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "\nMSSOTLPolicy::computeStimulus\n" << getName();
    //     WRITE_MESSAGE(str.str());
    // #endif
    //     return myDesirabilityAlgorithm->computeDesirability(vehInMeasure, vehOutMeasure, 0, 0);
    // }
    #[allow(unused_variables)]
    pub fn compute_desirability_2(&mut self, vehInMeasure: f64, vehOutMeasure: f64) -> f64 {
        todo!("port MSSOTLPolicy::computeDesirability")
    }

}

/// C++ class `PushButtonLogic` (microsim/traffic_lights/MSSOTLPolicy.h)
pub struct PushButtonLogic {
    pub m_pushButtonScaleFactor: f64,
    pub m_prefix: String,
}

impl PushButtonLogic {
    /// C++: `void PushButtonLogic::init(std::string, const Parameterised *)` [protected]
    // void PushButtonLogic::init(std::string prefix, const Parameterised* parameterised) {
    //     m_prefix = prefix;
    //     m_pushButtonScaleFactor = StringUtils::toDouble(parameterised->getParameter("PUSH_BUTTON_SCALE_FACTOR", "1"));
    //     WRITE_MESSAGE(m_prefix + "::PushButtonLogic::init use " + parameterised->getParameter("USE_PUSH_BUTTON", "0") + " scale " + parameterised->getParameter("PUSH_BUTTON_SCALE_FACTOR", "1"));
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, prefix: String, parameterised: Ptr<Parameterised>) {
        todo!("port PushButtonLogic::init")
    }

    /// C++: `bool PushButtonLogic::pushButtonLogic(SUMOTime, bool, const MSPhaseDefinition *)` [protected]
    // bool PushButtonLogic::pushButtonLogic(SUMOTime elapsed, bool pushButtonPressed, const MSPhaseDefinition* stage) {
    //     //pushbutton logic
    //     if (pushButtonPressed && (double)elapsed >= (double)stage->duration * m_pushButtonScaleFactor) {
    //         //If the stage duration has been passed
    // #ifdef SWARM_DEBUG
    //         std::ostringstream oss;
    //         oss << m_prefix << "::pushButtonLogic pushButtonPressed elapsed " << elapsed << " stage duration " << (stage->duration * m_pushButtonScaleFactor);
    //         WRITE_MESSAGE(oss.str());
    // #endif
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn push_button_logic(&mut self, elapsed: SUMOTime, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>) -> bool {
        todo!("port PushButtonLogic::pushButtonLogic")
    }

}

/// C++ class `SigmoidLogic` (microsim/traffic_lights/MSSOTLPolicy.h)
pub struct SigmoidLogic {
    pub m_useSigmoid: bool,
    pub m_k: f64,
    pub m_prefix: String,
}

impl SigmoidLogic {
    /// C++: `void SigmoidLogic::init(std::string, const Parameterised *)` [protected]
    // void SigmoidLogic::init(std::string prefix, const Parameterised* parameterised) {
    //     m_prefix = prefix;
    //     m_useSigmoid = parameterised->getParameter("PLATOON_USE_SIGMOID", "0") != "0";
    //     m_k = StringUtils::toDouble(parameterised->getParameter("PLATOON_SIGMOID_K_VALUE", "1"));
    // //  DBG(
    //     WRITE_MESSAGE(m_prefix + "::SigmoidLogic::init use " + parameterised->getParameter("PLATOON_USE_SIGMOID", "0") + " k " + parameterised->getParameter("PLATOON_SIGMOID_K_VALUE", "1"));
    // //    for (int elapsed = 10; elapsed < 51; ++elapsed)
    // //    {
    // //        double sigmoidValue = 1.0 / (1.0 + exp(-m_k * (elapsed - 31)));
    // //        std::ostringstream oss;
    // //        oss << "elapsed " << elapsed << " value " << sigmoidValue;
    // //        WRITE_MESSAGE(oss.str())
    // //    }
    // //  )
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, prefix: String, parameterised: Ptr<Parameterised>) {
        todo!("port SigmoidLogic::init")
    }

    /// C++: `bool SigmoidLogic::sigmoidLogic(SUMOTime, const MSPhaseDefinition *, int)` [protected]
    // bool SigmoidLogic::sigmoidLogic(SUMOTime elapsed, const MSPhaseDefinition* stage, int vehicleCount) {
    //     //use the sigmoid logic
    //     if (m_useSigmoid && vehicleCount == 0) {
    //         double sigmoidValue = 1.0 / (1.0 + exp(-m_k * STEPS2TIME(elapsed - stage->duration)));
    //         double rnd = RandHelper::rand();
    // //    DBG(
    //         std::ostringstream oss;
    //         oss << m_prefix << "::sigmoidLogic [k=" << m_k << " elapsed " << elapsed << " stage->duration " << stage->duration << " ] value "
    //             << sigmoidValue;
    //         oss << " rnd " << rnd << " retval " << (rnd < sigmoidValue ? "true" : "false");
    //         WRITE_MESSAGE(oss.str())
    // //    );
    //         return rnd < sigmoidValue;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn sigmoid_logic(&mut self, elapsed: SUMOTime, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
        todo!("port SigmoidLogic::sigmoidLogic")
    }

}

