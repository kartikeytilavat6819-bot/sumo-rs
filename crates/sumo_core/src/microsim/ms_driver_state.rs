//! Port of microsim/MSDriverState.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Default values for the MSDriverState parameters
/// C++ class `DriverStateDefaults` (microsim/MSDriverState.h)
pub struct DriverStateDefaults {
}

// C++ static members of DriverStateDefaults (port as module-level statics, see AGENTS.md):
//   static double DriverStateDefaults::minAwareness;
//   static double DriverStateDefaults::initialAwareness;
//   static double DriverStateDefaults::errorTimeScaleCoefficient;
//   static double DriverStateDefaults::errorNoiseIntensityCoefficient;
//   static double DriverStateDefaults::speedDifferenceErrorCoefficient;
//   static double DriverStateDefaults::speedDifferenceChangePerceptionThreshold;
//   static double DriverStateDefaults::headwayChangePerceptionThreshold;
//   static double DriverStateDefaults::headwayErrorCoefficient;
//   static double DriverStateDefaults::freeSpeedErrorCoefficient;
//   static double DriverStateDefaults::maximalReactionTimeFactor;

impl DriverStateDefaults {
}

/// Provides an interface to an error whose fluctuation is controlled via the driver's 'awareness', which can be controlled externally,
/// C++ class `MSSimpleDriverState` (microsim/MSDriverState.h)
pub struct MSSimpleDriverState {
    /// Vehicle corresponding to this driver state
    pub myVehicle: Ptr<MSVehicle>,
    /// Driver's 'awareness' [0,1]
    pub myAwareness: f64,
    /// Minimal value for 'awareness' [0,1]
    pub myMinAwareness: f64,
    /// Initial value for 'awareness' [0,1]
    pub myInitialAwareness: f64,
    /// Driver's 'error',
    pub myError: OUProcess,
    /// Coefficient controlling the impact of awareness on the time scale of the error process
    pub myErrorTimeScaleCoefficient: f64,
    /// Coefficient controlling the impact of awareness on the noise intensity of the error process
    pub myErrorNoiseIntensityCoefficient: f64,
    /// Scaling coefficients for the magnitude of errors
    pub mySpeedDifferenceErrorCoefficient: f64,
    pub myHeadwayErrorCoefficient: f64,
    pub myFreeSpeedErrorCoefficient: f64,
    /// Thresholds above a change in the corresponding quantity is perceived.
    pub myHeadwayChangePerceptionThreshold: f64,
    pub mySpeedDifferenceChangePerceptionThreshold: f64,
    /// Action step length (~current maximal reaction time) induced by awareness level
    pub myActionStepLength: f64,
    /// Maximal reaction time (value set for the actionStepLength at awareness=1)
    pub myOriginalReactionTime: f64,
    /// Maximal reaction time (value set for the actionStepLength at awareness=myMinAwareness)
    pub myMaximalReactionTime: f64,
    /// Elapsed time since the last state update
    pub myStepDuration: f64,
    /// Time point of the last state update
    pub myLastUpdateTime: f64,
    /// The assumed gaps to different objects
    pub myAssumedGap: BTreeMap<Ptr<()>, f64>,
    /// The last perceived speed differences to the corresponding objects
    pub myLastPerceivedSpeedDifference: BTreeMap<Ptr<()>, f64>,
    /// Used to prevent infinite loops in debugging outputs,
    pub myDebugLock: bool,
}

impl MSSimpleDriverState {
    /// C++: `void MSSimpleDriverState::MSSimpleDriverState(MSVehicle *)`
    // MSSimpleDriverState::MSSimpleDriverState(MSVehicle* veh) :
    //     myVehicle(veh),
    //     myAwareness(1.),
    //     myMinAwareness(DriverStateDefaults::minAwareness),
    //     myError(0., 1., 1.),
    //     myErrorTimeScaleCoefficient(DriverStateDefaults::errorTimeScaleCoefficient),
    //     myErrorNoiseIntensityCoefficient(DriverStateDefaults::errorNoiseIntensityCoefficient),
    //     mySpeedDifferenceErrorCoefficient(DriverStateDefaults::speedDifferenceErrorCoefficient),
    //     myHeadwayErrorCoefficient(DriverStateDefaults::headwayErrorCoefficient),
    //     myFreeSpeedErrorCoefficient(DriverStateDefaults::freeSpeedErrorCoefficient),
    //     myHeadwayChangePerceptionThreshold(DriverStateDefaults::headwayChangePerceptionThreshold),
    //     mySpeedDifferenceChangePerceptionThreshold(DriverStateDefaults::speedDifferenceChangePerceptionThreshold),
    //     myOriginalReactionTime(veh->getActionStepLengthSecs()),
    //     myMaximalReactionTime(DriverStateDefaults::maximalReactionTimeFactor * myOriginalReactionTime),
    // //    myActionStepLength(TS),
    //     myStepDuration(TS),
    //     myLastUpdateTime(SIMTIME - TS),
    //     myDebugLock(false) {
    // #ifdef DEBUG_DRIVERSTATE
    //     std::cout << "Constructing driver state for veh '" << veh->getID() << "'." << std::endl;
    // #endif
    //     updateError();
    //     updateReactionTime();
    // }
    #[allow(unused_variables)]
    pub fn new(veh: Ptr<MSVehicle>) -> Self {
        todo!("port MSSimpleDriverState::MSSimpleDriverState")
    }

    /// C++: `void MSSimpleDriverState::~MSSimpleDriverState() noexcept` [virtual]
    // virtual ~MSSimpleDriverState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSSimpleDriverState::~MSSimpleDriverState")
    }

    /// C++: `double MSSimpleDriverState::getMinAwareness() const` [const]
    /// @{
    // inline double getMinAwareness() const {
    //         return myMinAwareness;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_awareness(&self) -> f64 {
        todo!("port MSSimpleDriverState::getMinAwareness")
    }

    /// C++: `double MSSimpleDriverState::getInitialAwareness() const` [const]
    // inline double getInitialAwareness() const {
    //         return myInitialAwareness;
    //     }
    #[allow(unused_variables)]
    pub fn get_initial_awareness(&self) -> f64 {
        todo!("port MSSimpleDriverState::getInitialAwareness")
    }

    /// C++: `double MSSimpleDriverState::getErrorTimeScaleCoefficient() const` [const]
    // inline double getErrorTimeScaleCoefficient() const {
    //         return myErrorTimeScaleCoefficient;
    //     }
    #[allow(unused_variables)]
    pub fn get_error_time_scale_coefficient(&self) -> f64 {
        todo!("port MSSimpleDriverState::getErrorTimeScaleCoefficient")
    }

    /// C++: `double MSSimpleDriverState::getErrorNoiseIntensityCoefficient() const` [const]
    // inline double getErrorNoiseIntensityCoefficient() const {
    //         return myErrorNoiseIntensityCoefficient;
    //     }
    #[allow(unused_variables)]
    pub fn get_error_noise_intensity_coefficient(&self) -> f64 {
        todo!("port MSSimpleDriverState::getErrorNoiseIntensityCoefficient")
    }

    /// C++: `double MSSimpleDriverState::getErrorTimeScale() const` [const]
    // inline double getErrorTimeScale() const {
    //         return myError.getTimeScale();
    //     }
    #[allow(unused_variables)]
    pub fn get_error_time_scale(&self) -> f64 {
        todo!("port MSSimpleDriverState::getErrorTimeScale")
    }

    /// C++: `double MSSimpleDriverState::getErrorNoiseIntensity() const` [const]
    // inline double getErrorNoiseIntensity() const {
    //         return myError.getNoiseIntensity();
    //     }
    #[allow(unused_variables)]
    pub fn get_error_noise_intensity(&self) -> f64 {
        todo!("port MSSimpleDriverState::getErrorNoiseIntensity")
    }

    /// C++: `double MSSimpleDriverState::getSpeedDifferenceErrorCoefficient() const` [const]
    // inline double getSpeedDifferenceErrorCoefficient() const {
    //         return mySpeedDifferenceErrorCoefficient;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_difference_error_coefficient(&self) -> f64 {
        todo!("port MSSimpleDriverState::getSpeedDifferenceErrorCoefficient")
    }

    /// C++: `double MSSimpleDriverState::getHeadwayErrorCoefficient() const` [const]
    // inline double getHeadwayErrorCoefficient() const {
    //         return myHeadwayErrorCoefficient;
    //     }
    #[allow(unused_variables)]
    pub fn get_headway_error_coefficient(&self) -> f64 {
        todo!("port MSSimpleDriverState::getHeadwayErrorCoefficient")
    }

    /// C++: `double MSSimpleDriverState::getFreeSpeedErrorCoefficient() const` [const]
    // inline double getFreeSpeedErrorCoefficient() const {
    //         return myFreeSpeedErrorCoefficient;
    //     }
    #[allow(unused_variables)]
    pub fn get_free_speed_error_coefficient(&self) -> f64 {
        todo!("port MSSimpleDriverState::getFreeSpeedErrorCoefficient")
    }

    /// C++: `double MSSimpleDriverState::getSpeedDifferenceChangePerceptionThreshold() const` [const]
    // inline double getSpeedDifferenceChangePerceptionThreshold() const {
    //         return mySpeedDifferenceChangePerceptionThreshold;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_difference_change_perception_threshold(&self) -> f64 {
        todo!("port MSSimpleDriverState::getSpeedDifferenceChangePerceptionThreshold")
    }

    /// C++: `double MSSimpleDriverState::getHeadwayChangePerceptionThreshold() const` [const]
    // inline double getHeadwayChangePerceptionThreshold() const {
    //         return myHeadwayChangePerceptionThreshold;
    //     }
    #[allow(unused_variables)]
    pub fn get_headway_change_perception_threshold(&self) -> f64 {
        todo!("port MSSimpleDriverState::getHeadwayChangePerceptionThreshold")
    }

    /// C++: `double MSSimpleDriverState::getAwareness() const` [const]
    // inline double getAwareness() const {
    //         return myAwareness;
    //     }
    #[allow(unused_variables)]
    pub fn get_awareness(&self) -> f64 {
        todo!("port MSSimpleDriverState::getAwareness")
    }

    /// C++: `double MSSimpleDriverState::getMaximalReactionTime() const` [const]
    // inline double getMaximalReactionTime() const {
    //         return myMaximalReactionTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_maximal_reaction_time(&self) -> f64 {
        todo!("port MSSimpleDriverState::getMaximalReactionTime")
    }

    /// C++: `double MSSimpleDriverState::getOriginalReactionTime() const` [const]
    // inline double getOriginalReactionTime() const {
    //         return myOriginalReactionTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_original_reaction_time(&self) -> f64 {
        todo!("port MSSimpleDriverState::getOriginalReactionTime")
    }

    /// C++: `double MSSimpleDriverState::getActionStepLength() const` [const]
    // inline double getActionStepLength() const {
    //         return myActionStepLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_action_step_length(&self) -> f64 {
        todo!("port MSSimpleDriverState::getActionStepLength")
    }

    /// C++: `double MSSimpleDriverState::getErrorState() const` [const]
    // inline double getErrorState() const {
    //         return myError.getState();
    //     }
    #[allow(unused_variables)]
    pub fn get_error_state(&self) -> f64 {
        todo!("port MSSimpleDriverState::getErrorState")
    }

    /// C++: `void MSSimpleDriverState::setMinAwareness(const double)`
    /// @{
    // inline void setMinAwareness(const double value)  {
    //         myMinAwareness = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_min_awareness(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setMinAwareness")
    }

    /// C++: `void MSSimpleDriverState::setInitialAwareness(const double)`
    // inline void setInitialAwareness(const double value)  {
    //         myInitialAwareness = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_initial_awareness(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setInitialAwareness")
    }

    /// C++: `void MSSimpleDriverState::setErrorTimeScaleCoefficient(const double)`
    // inline void setErrorTimeScaleCoefficient(const double value)  {
    //         myErrorTimeScaleCoefficient = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_error_time_scale_coefficient(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setErrorTimeScaleCoefficient")
    }

    /// C++: `void MSSimpleDriverState::setErrorNoiseIntensityCoefficient(const double)`
    // inline void setErrorNoiseIntensityCoefficient(const double value)  {
    //         myErrorNoiseIntensityCoefficient = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_error_noise_intensity_coefficient(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setErrorNoiseIntensityCoefficient")
    }

    /// C++: `void MSSimpleDriverState::setSpeedDifferenceErrorCoefficient(const double)`
    // inline void setSpeedDifferenceErrorCoefficient(const double value)  {
    //         mySpeedDifferenceErrorCoefficient = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_speed_difference_error_coefficient(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setSpeedDifferenceErrorCoefficient")
    }

    /// C++: `void MSSimpleDriverState::setHeadwayErrorCoefficient(const double)`
    // inline void setHeadwayErrorCoefficient(const double value)  {
    //         myHeadwayErrorCoefficient = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_error_coefficient(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setHeadwayErrorCoefficient")
    }

    /// C++: `void MSSimpleDriverState::setFreeSpeedErrorCoefficient(const double)`
    // inline void setFreeSpeedErrorCoefficient(const double value)  {
    //         myFreeSpeedErrorCoefficient = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_free_speed_error_coefficient(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setFreeSpeedErrorCoefficient")
    }

    /// C++: `void MSSimpleDriverState::setSpeedDifferenceChangePerceptionThreshold(const double)`
    // inline void setSpeedDifferenceChangePerceptionThreshold(const double value)  {
    //         mySpeedDifferenceChangePerceptionThreshold = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_speed_difference_change_perception_threshold(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setSpeedDifferenceChangePerceptionThreshold")
    }

    /// C++: `void MSSimpleDriverState::setHeadwayChangePerceptionThreshold(const double)`
    // inline void setHeadwayChangePerceptionThreshold(const double value)  {
    //         myHeadwayChangePerceptionThreshold = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_change_perception_threshold(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setHeadwayChangePerceptionThreshold")
    }

    /// C++: `void MSSimpleDriverState::setMaximalReactionTime(const double)`
    // inline void setMaximalReactionTime(const double value)  {
    //         myMaximalReactionTime = value;
    //         updateReactionTime();
    //     }
    #[allow(unused_variables)]
    pub fn set_maximal_reaction_time(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setMaximalReactionTime")
    }

    /// C++: `void MSSimpleDriverState::setOriginalReactionTime(const double)`
    // inline void setOriginalReactionTime(const double value)  {
    //         myOriginalReactionTime = value;
    //         updateReactionTime();
    //     }
    #[allow(unused_variables)]
    pub fn set_original_reaction_time(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setOriginalReactionTime")
    }

    /// C++: `void MSSimpleDriverState::setAwareness(const double)`
    // void
    // MSSimpleDriverState::setAwareness(const double value) {
    //     assert(value >= 0.);
    //     assert(value <= 1.);
    // #ifdef DEBUG_AWARENESS
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << myVehicle->getID() << ", setAwareness(" << MAX2(value, myMinAwareness) << ")" << std::endl;
    //     }
    // #endif
    //     myAwareness = MAX2(value, myMinAwareness);
    //     if (myAwareness == 1.) {
    //         myError.setState(0.);
    //     }
    //     updateReactionTime();
    // }
    #[allow(unused_variables)]
    pub fn set_awareness(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setAwareness")
    }

    /// C++: `void MSSimpleDriverState::setErrorState(const double)`
    // inline void setErrorState(const double state) {
    //         myError.setState(state);
    //     }
    #[allow(unused_variables)]
    pub fn set_error_state(&mut self, state: f64) {
        todo!("port MSSimpleDriverState::setErrorState")
    }

    /// C++: `void MSSimpleDriverState::setErrorTimeScale(const double)`
    // inline void setErrorTimeScale(const double value)  {
    //         myError.setTimeScale(value);
    //     }
    #[allow(unused_variables)]
    pub fn set_error_time_scale(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setErrorTimeScale")
    }

    /// C++: `void MSSimpleDriverState::setErrorNoiseIntensity(const double)`
    // inline void setErrorNoiseIntensity(const double value)  {
    //         myError.setNoiseIntensity(value);
    //     }
    #[allow(unused_variables)]
    pub fn set_error_noise_intensity(&mut self, value: f64) {
        todo!("port MSSimpleDriverState::setErrorNoiseIntensity")
    }

    /// C++: `void MSSimpleDriverState::update()`
    /// Trigger updates for the errorProcess, assumed gaps, etc
    // void
    // MSSimpleDriverState::update() {
    // #ifdef DEBUG_AWARENESS
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << myVehicle->getID() << ", DriverState::update()" << std::endl;
    //     }
    // #endif
    //     // Adapt step duration
    //     updateStepDuration();
    //     // Update error
    //     updateError();
    //     // Update actionStepLength, aka reaction time
    //     updateReactionTime();
    //     // Update assumed gaps
    //     updateAssumedGaps();
    // #ifdef DEBUG_AWARENESS
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " stepDuration=" << myStepDuration << ", error=" << myError.getState() << std::endl;
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self) {
        todo!("port MSSimpleDriverState::update")
    }

    /// C++: `void MSSimpleDriverState::updateAssumedGaps()`
    /// Update the assumed gaps to the known objects according to the corresponding perceived speed differences.
    // void
    // MSSimpleDriverState::updateAssumedGaps() {
    //     for (auto& p : myAssumedGap) {
    //         const void* objID = p.first;
    //         const auto speedDiff = myLastPerceivedSpeedDifference.find(objID);
    //         double assumedSpeedDiff;
    //         if (speedDiff != myLastPerceivedSpeedDifference.end()) {
    //             // update the assumed gap with the last perceived speed difference
    //             assumedSpeedDiff = speedDiff->second;
    //         } else {
    //             // Assume the object is not moving, if no perceived speed difference is known.
    //             assumedSpeedDiff = -myVehicle->getSpeed();
    //         }
    //         p.second += SPEED2DIST(assumedSpeedDiff);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_assumed_gaps(&mut self) {
        todo!("port MSSimpleDriverState::updateAssumedGaps")
    }

    /// C++: `double MSSimpleDriverState::getPerceivedOwnSpeed(double)`
    /// apply perception error to own speed
    // double
    // MSSimpleDriverState::getPerceivedOwnSpeed(double speed) {
    //     return speed + myFreeSpeedErrorCoefficient * myError.getState() * sqrt(speed);
    // }
    #[allow(unused_variables)]
    pub fn get_perceived_own_speed(&mut self, speed: f64) -> f64 {
        todo!("port MSSimpleDriverState::getPerceivedOwnSpeed")
    }

    /// C++: `double MSSimpleDriverState::getPerceivedSpeedDifference(const double, const double, const void *)`
    /// This method checks whether the errorneous speed difference that would be perceived for this step differs sufficiently from the previously perceived to be actually perceived. If so, it sets the flag myReactionFlag[objID]=true, which should be checked just after the call to this method because it will be overwritten by subsequent calls.
    // double
    // MSSimpleDriverState::getPerceivedSpeedDifference(const double trueSpeedDifference, const double trueGap, const void* objID) {
    // #ifdef DEBUG_PERCEPTION_ERRORS
    //     if (DEBUG_COND) {
    //         if (!debugLocked()) {
    //             std::cout << SIMTIME << " getPerceivedSpeedDifference() for veh '" << myVehicle->getID() << "'\n"
    //                       << "    trueGap=" << trueGap << " trueSpeedDifference=" << trueSpeedDifference << " objID=" << objID << std::endl;
    //         }
    //     }
    // #endif
    //     const double perceivedSpeedDifference = trueSpeedDifference + mySpeedDifferenceErrorCoefficient * myError.getState() * trueGap;
    //     const auto lastPerceivedSpeedDifference = myLastPerceivedSpeedDifference.find(objID);
    //     if (lastPerceivedSpeedDifference == myLastPerceivedSpeedDifference.end()
    //             || fabs(perceivedSpeedDifference - lastPerceivedSpeedDifference->second) > mySpeedDifferenceChangePerceptionThreshold * trueGap * (1.0 - myAwareness)) {
    //
    // #ifdef DEBUG_PERCEPTION_ERRORS
    //         if (DEBUG_COND) {
    //             if (!debugLocked()) {
    //                 std::cout << "    new perceived speed difference (=" << perceivedSpeedDifference << ") differs significantly from the last perceived (="
    //                           << (lastPerceivedSpeedDifference == myLastPerceivedSpeedDifference.end() ? "NA" : toString(lastPerceivedSpeedDifference->second)) << ")"
    //                           << std::endl;
    //             }
    //         }
    // #endif
    //
    //         // new perceived speed difference differs significantly from the previous
    //         myLastPerceivedSpeedDifference[objID] = perceivedSpeedDifference;
    //         return perceivedSpeedDifference;
    //     } else {
    // #ifdef DEBUG_PERCEPTION_ERRORS
    //         if (!debugLocked()) {
    //             std::cout << "    new perceived speed difference (=" << perceivedSpeedDifference << ") does *not* differ significantly from the last perceived (="
    //                       << (lastPerceivedSpeedDifference->second) << ")" << std::endl;
    //         }
    // #endif
    //         // new perceived speed difference doesn't differ significantly from the previous
    //         return lastPerceivedSpeedDifference->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_perceived_speed_difference(&mut self, trueSpeedDifference: f64, trueGap: f64, objID: Ptr<()>) -> f64 {
        todo!("port MSSimpleDriverState::getPerceivedSpeedDifference")
    }

    /// C++: `double MSSimpleDriverState::getPerceivedHeadway(const double, const void *)`
    // double
    // MSSimpleDriverState::getPerceivedHeadway(const double trueGap, const void* objID) {
    // #ifdef DEBUG_PERCEPTION_ERRORS
    //     if (DEBUG_COND) {
    //         if (!debugLocked()) {
    //             std::cout << SIMTIME << " getPerceivedHeadway() for veh '" << myVehicle->getID() << "'\n"
    //                       << "    trueGap=" << trueGap << " objID=" << objID << std::endl;
    //         }
    //     }
    // #endif
    //
    //     const double perceivedGap = trueGap + myHeadwayErrorCoefficient * myError.getState() * trueGap;
    //     const auto assumedGap = myAssumedGap.find(objID);
    //     if (assumedGap == myAssumedGap.end()
    //             || fabs(perceivedGap - assumedGap->second) > myHeadwayChangePerceptionThreshold * trueGap * (1.0 - myAwareness)) {
    //
    // #ifdef DEBUG_PERCEPTION_ERRORS
    //         if (!debugLocked()) {
    //             std::cout << "    new perceived gap (=" << perceivedGap << ") differs significantly from the assumed (="
    //                       << (assumedGap == myAssumedGap.end() ? "NA" : toString(assumedGap->second)) << ")" << std::endl;
    //         }
    // #endif
    //
    //         // new perceived gap differs significantly from the previous
    //         myAssumedGap[objID] = perceivedGap;
    //         return perceivedGap;
    //     } else {
    //
    // #ifdef DEBUG_PERCEPTION_ERRORS
    //         if (DEBUG_COND) {
    //             if (!debugLocked()) {
    //                 std::cout << "    new perceived gap (=" << perceivedGap << ") does *not* differ significantly from the assumed (="
    //                           << (assumedGap->second) << ")" << std::endl;
    //             }
    //         }
    // #endif
    //         // new perceived gap doesn't differ significantly from the previous
    //         return myAssumedGap[objID];
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_perceived_headway(&mut self, trueGap: f64, objID: Ptr<()>) -> f64 {
        todo!("port MSSimpleDriverState::getPerceivedHeadway")
    }

    /// C++: `void MSSimpleDriverState::lockDebug()`
    /// @}
    // inline void lockDebug() {
    //         myDebugLock = true;
    //     }
    #[allow(unused_variables)]
    pub fn lock_debug(&mut self) {
        todo!("port MSSimpleDriverState::lockDebug")
    }

    /// C++: `void MSSimpleDriverState::unlockDebug()`
    // inline void unlockDebug() {
    //         myDebugLock = false;
    //     }
    #[allow(unused_variables)]
    pub fn unlock_debug(&mut self) {
        todo!("port MSSimpleDriverState::unlockDebug")
    }

    /// C++: `bool MSSimpleDriverState::debugLocked() const` [const]
    // inline bool debugLocked() const {
    //         return myDebugLock;
    //     }
    #[allow(unused_variables)]
    pub fn debug_locked(&self) -> bool {
        todo!("port MSSimpleDriverState::debugLocked")
    }

    /// C++: `void MSSimpleDriverState::updateStepDuration()` [private]
    // void
    // MSSimpleDriverState::updateStepDuration() {
    //     myStepDuration = SIMTIME - myLastUpdateTime;
    //     myLastUpdateTime = SIMTIME;
    // }
    #[allow(unused_variables)]
    pub fn update_step_duration(&mut self) {
        todo!("port MSSimpleDriverState::updateStepDuration")
    }

    /// C++: `void MSSimpleDriverState::updateError()` [private]
    // void
    // MSSimpleDriverState::updateError() {
    //     if (myAwareness == 1.0 || myAwareness == 0.0) {
    //         myError.setState(0.);
    //     } else {
    //         myError.setTimeScale(myErrorTimeScaleCoefficient * myAwareness);
    //         myError.setNoiseIntensity(myErrorNoiseIntensityCoefficient * (1. - myAwareness));
    //         myError.step(myStepDuration);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_error(&mut self) {
        todo!("port MSSimpleDriverState::updateError")
    }

    /// C++: `void MSSimpleDriverState::updateReactionTime()` [private]
    // void
    // MSSimpleDriverState::updateReactionTime() {
    //     if (myAwareness == 1.0 || myAwareness == 0.0) {
    //         myActionStepLength = myOriginalReactionTime;
    //     } else {
    //         const double theta = (myAwareness - myMinAwareness) / (1.0 - myMinAwareness);
    //         myActionStepLength = myOriginalReactionTime + theta * (myMaximalReactionTime - myOriginalReactionTime);
    //         // Round to multiple of simstep length
    //         int quotient;
    //         remquo(myActionStepLength, TS, &quotient);
    //         myActionStepLength = TS * MAX2(quotient, 1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_reaction_time(&mut self) {
        todo!("port MSSimpleDriverState::updateReactionTime")
    }

}

/// An Ornstein-Uhlenbeck stochastic process
/// C++ class `OUProcess` (microsim/MSDriverState.h)
pub struct OUProcess {
    /// The current state of the process
    pub myState: f64,
    /// The time scale of the process
    pub myTimeScale: f64,
    /// The noise intensity of the process
    pub myNoiseIntensity: f64,
}

// C++ static members of OUProcess (port as module-level statics, see AGENTS.md):
//   static SumoRNG OUProcess::myRNG;

impl OUProcess {
    /// C++: `void OUProcess::OUProcess(double, double, double)`
    /// constructor
    // OUProcess::OUProcess(double initialState, double timeScale, double noiseIntensity)
    //     : myState(initialState),
    //       myTimeScale(timeScale),
    //       myNoiseIntensity(noiseIntensity) {}
    #[allow(unused_variables)]
    pub fn new(initialState: f64, timeScale: f64, noiseIntensity: f64) -> Self {
        todo!("port OUProcess::OUProcess")
    }

    /// C++: `void OUProcess::~OUProcess() noexcept`
    /// destructor
    // OUProcess::~OUProcess() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port OUProcess::~OUProcess")
    }

    /// C++: `void OUProcess::step(double)`
    /// evolve for a time step of length dt.
    // void
    // OUProcess::step(double dt) {
    // #ifdef DEBUG_OUPROCESS
    //     const double oldstate = myState;
    // #endif
    //     myState = exp(-dt / myTimeScale) * myState + myNoiseIntensity * sqrt(2 * dt / myTimeScale) * RandHelper::randNorm(0, 1, &myRNG);
    // #ifdef DEBUG_OUPROCESS
    //     std::cout << "  OU-step (" << dt << " s.): " << oldstate << "->" << myState << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn step(&mut self, dt: f64) {
        todo!("port OUProcess::step")
    }

    /// C++: `double OUProcess::step(double, double, double, double)` [static]
    /// static version of the step()
    // double
    // OUProcess::step(double state, double dt, double timeScale, double noiseIntensity) {
    //     /// see above
    //     return exp(-dt / timeScale) * state + noiseIntensity * sqrt(2 * dt / timeScale) * RandHelper::randNorm(0, 1, &myRNG);
    // }
    #[allow(unused_variables)]
    pub fn step_2(state: f64, dt: f64, timeScale: f64, noiseIntensity: f64) -> f64 {
        todo!("port OUProcess::step")
    }

    /// C++: `void OUProcess::setTimeScale(double)`
    /// set the process' timescale to a new value
    // void setTimeScale(double timeScale) {
    //         myTimeScale = timeScale;
    //     }
    #[allow(unused_variables)]
    pub fn set_time_scale(&mut self, timeScale: f64) {
        todo!("port OUProcess::setTimeScale")
    }

    /// C++: `void OUProcess::setNoiseIntensity(double)`
    /// set the process' noise intensity to a new value
    // void setNoiseIntensity(double noiseIntensity) {
    //         myNoiseIntensity = noiseIntensity;
    //     }
    #[allow(unused_variables)]
    pub fn set_noise_intensity(&mut self, noiseIntensity: f64) {
        todo!("port OUProcess::setNoiseIntensity")
    }

    /// C++: `void OUProcess::setState(double)`
    /// set the process' state to a new value
    // void setState(double state) {
    //         myState = state;
    //     }
    #[allow(unused_variables)]
    pub fn set_state(&mut self, state: f64) {
        todo!("port OUProcess::setState")
    }

    /// C++: `double OUProcess::getNoiseIntensity() const` [const]
    // inline double getNoiseIntensity() const {
    //         return myNoiseIntensity;
    //     }
    #[allow(unused_variables)]
    pub fn get_noise_intensity(&self) -> f64 {
        todo!("port OUProcess::getNoiseIntensity")
    }

    /// C++: `double OUProcess::getTimeScale() const` [const]
    // inline double getTimeScale() const {
    //         return myTimeScale;
    //     }
    #[allow(unused_variables)]
    pub fn get_time_scale(&self) -> f64 {
        todo!("port OUProcess::getTimeScale")
    }

    /// C++: `double OUProcess::getState() const` [const]
    /// Obtain the current state of the process
    // double
    // OUProcess::getState() const {
    //     return myState;
    // }
    #[allow(unused_variables)]
    pub fn get_state(&self) -> f64 {
        todo!("port OUProcess::getState")
    }

    /// C++: `SumoRNG * OUProcess::getRNG()` [static]
    // static SumoRNG* getRNG() {
    //         return &myRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_rng() -> Ptr<SumoRNG> {
        todo!("port OUProcess::getRNG")
    }

}

