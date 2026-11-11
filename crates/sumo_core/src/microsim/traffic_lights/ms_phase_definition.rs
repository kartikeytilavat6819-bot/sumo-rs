//! Port of microsim/traffic_lights/MSPhaseDefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The definition of a single phase of a tls logic
/// C++ class `MSPhaseDefinition` (microsim/traffic_lights/MSPhaseDefinition.h)
pub struct MSPhaseDefinition {
    /// The duration of the phase
    pub duration: SUMOTime,
    /// The previous duration of the phase
    pub lastDuration: SUMOTime,
    /// The minimum duration of the phase
    pub minDuration: SUMOTime,
    /// The maximum duration of the phase
    pub maxDuration: SUMOTime,
    /// The minimum time within the cycle for switching (for coordinated actuation)
    pub earliestEnd: SUMOTime,
    /// The maximum time within the cycle for switching (for coordinated actuation)
    pub latestEnd: SUMOTime,
    /// The condition expression for an early switch into this phase
    pub earlyTarget: String,
    /// The condition expression for switching into this phase when the active phase must end
    pub finalTarget: String,
    /// Stores the timestep of the last on-switched of the phase
    pub myLastSwitch: SUMOTime,
    /// Stores the timestep when the previous instance of this phase was switched off
    pub myLastEnd: SUMOTime,
    /// The index of the phase that suceeds this one (or -1)
    pub nextPhases: Vec<i32>,
    /// Optional name or description for the current phase
    pub name: String,
    /// for NEMA phase
    pub yellow: SUMOTime,
    /// for NEMA phase
    pub red: SUMOTime,
    /// for NEMA phase
    pub vehext: SUMOTime,
    /// the phase is a transient one or a decisional one, compulsory directive for SOTL policies
    pub myTransientNotDecisional: bool,
    /// the phase is a commit, compulsory directive for SOTL policies
    pub myCommit: bool,
    /// Leaving the phase type as "undefined" lets SOTL policies malfunction
    pub myUndefined: bool,
    pub myTargetLaneSet: Vec<String>,
    /// The phase definition
    pub myState: String,
}

// C++ static members of MSPhaseDefinition (port as module-level statics, see AGENTS.md):
//   static const SUMOTime MSPhaseDefinition::UNSPECIFIED_DURATION;
//   static const SUMOTime MSPhaseDefinition::OVERRIDE_DURATION;

impl MSPhaseDefinition {
    /// C++: `void MSPhaseDefinition::MSPhaseDefinition(SUMOTime, const std::string &, const std::string &)`
    /// Constructor *
    // MSPhaseDefinition(SUMOTime _duration, const std::string& state, const std::string& _name = ""):
    //         duration(_duration),
    //         lastDuration(UNSPECIFIED_DURATION),
    //         minDuration(duration),
    //         maxDuration(duration),
    //         earliestEnd(UNSPECIFIED_DURATION),
    //         latestEnd(UNSPECIFIED_DURATION),
    //         myLastSwitch(UNSPECIFIED_DURATION),
    //         myLastEnd(UNSPECIFIED_DURATION),
    //         name(_name),
    //         yellow(UNSPECIFIED_DURATION),
    //         red(UNSPECIFIED_DURATION),
    //         vehext(UNSPECIFIED_DURATION),
    //         myTransientNotDecisional(false),
    //         myCommit(false),
    //         myUndefined(false),
    //         myState(state)
    //     {}
    #[allow(unused_variables)]
    pub fn new(_duration: SUMOTime, state: &str, _name: &str) -> Self {
        todo!("port MSPhaseDefinition::MSPhaseDefinition")
    }

    /// C++: `void MSPhaseDefinition::~MSPhaseDefinition() noexcept` [virtual]
    /// Destructor
    // virtual ~MSPhaseDefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSPhaseDefinition::~MSPhaseDefinition")
    }

    /// C++: `const std::string & MSPhaseDefinition::getState() const` [const]
    /// Returns the state within this phase
    // const std::string& getState() const {
    //         return myState;
    //     }
    #[allow(unused_variables)]
    pub fn get_state(&self) -> &str {
        todo!("port MSPhaseDefinition::getState")
    }

    /// C++: `void MSPhaseDefinition::setState(const std::string &)`
    // void setState(const std::string& _state) {
    //         myState = _state;
    //     }
    #[allow(unused_variables)]
    pub fn set_state(&mut self, _state: &str) {
        todo!("port MSPhaseDefinition::setState")
    }

    /// C++: `const std::vector<std::string> & MSPhaseDefinition::getTargetLaneSet() const` [const]
    // const std::vector<std::string>& getTargetLaneSet() const {
    //         return myTargetLaneSet;
    //     }
    #[allow(unused_variables)]
    pub fn get_target_lane_set(&self) -> &Vec<String> {
        todo!("port MSPhaseDefinition::getTargetLaneSet")
    }

    /// C++: `const std::vector<int> & MSPhaseDefinition::getNextPhases() const` [const]
    // const std::vector<int>& getNextPhases() const {
    //         return nextPhases;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_phases(&self) -> &Vec<i32> {
        todo!("port MSPhaseDefinition::getNextPhases")
    }

    /// C++: `const std::string & MSPhaseDefinition::getName() const` [const]
    // const std::string& getName() const {
    //         return name;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        todo!("port MSPhaseDefinition::getName")
    }

    /// C++: `void MSPhaseDefinition::setName(const std::string &)`
    // void setName(const std::string& _name) {
    //         name = _name;
    //     }
    #[allow(unused_variables)]
    pub fn set_name(&mut self, _name: &str) {
        todo!("port MSPhaseDefinition::setName")
    }

    /// C++: `bool MSPhaseDefinition::isGreenPhase() const` [const]
    /// Returns whether this phase is a pure "green" phase
    // bool isGreenPhase() const {
    //         if (myState.find_first_of("gG") == std::string::npos) {
    //             return false;
    //         }
    //         if (myState.find_first_of("yY") != std::string::npos) {
    //             return false;
    //         }
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_green_phase(&self) -> bool {
        todo!("port MSPhaseDefinition::isGreenPhase")
    }

    /// C++: `bool MSPhaseDefinition::isAllRedPhase() const` [const]
    /// Returns whether this phase is an "all red" phase
    // bool isAllRedPhase() const {
    //         return (int)myState.size() == (int)std::count(myState.begin(), myState.end(), 'r');
    //     }
    #[allow(unused_variables)]
    pub fn is_all_red_phase(&self) -> bool {
        todo!("port MSPhaseDefinition::isAllRedPhase")
    }

    /// C++: `LinkState MSPhaseDefinition::getSignalState(int) const` [const]
    /// Returns the state of the tls signal at the given position
    // LinkState getSignalState(int pos) const {
    //         return (LinkState) myState[pos];
    //     }
    #[allow(unused_variables)]
    pub fn get_signal_state(&self, pos: i32) -> LinkState {
        todo!("port MSPhaseDefinition::getSignalState")
    }

    /// C++: `bool MSPhaseDefinition::isActuated() const` [const]
    // inline bool isActuated() const {
    //         return minDuration != maxDuration || minDuration == OVERRIDE_DURATION;
    //     }
    #[allow(unused_variables)]
    pub fn is_actuated(&self) -> bool {
        todo!("port MSPhaseDefinition::isActuated")
    }

    /// C++: `bool MSPhaseDefinition::operator!=(const MSPhaseDefinition &)`
    /// Comparison operator
    // bool operator!=(const MSPhaseDefinition& pd) {
    //         return myState != pd.myState || name != pd.name;
    //     }
    #[allow(unused_variables)]
    pub fn ne_op(&mut self, pd: &MSPhaseDefinition) -> bool {
        todo!("port MSPhaseDefinition::operator!=")
    }

    /// C++: `bool MSPhaseDefinition::isUndefined() const` [const]
    // bool isUndefined() const {
    //         return myUndefined;
    //     }
    #[allow(unused_variables)]
    pub fn is_undefined(&self) -> bool {
        todo!("port MSPhaseDefinition::isUndefined")
    }

    /// C++: `bool MSPhaseDefinition::isTarget() const` [const]
    // bool isTarget() const {
    //         return !myTargetLaneSet.empty();
    //     }
    #[allow(unused_variables)]
    pub fn is_target(&self) -> bool {
        todo!("port MSPhaseDefinition::isTarget")
    }

    /// C++: `bool MSPhaseDefinition::isTransient() const` [const]
    // bool isTransient() const {
    //         return myTransientNotDecisional;
    //     }
    #[allow(unused_variables)]
    pub fn is_transient(&self) -> bool {
        todo!("port MSPhaseDefinition::isTransient")
    }

    /// C++: `bool MSPhaseDefinition::isDecisional() const` [const]
    // bool isDecisional() const {
    //         return !myTransientNotDecisional;
    //     }
    #[allow(unused_variables)]
    pub fn is_decisional(&self) -> bool {
        todo!("port MSPhaseDefinition::isDecisional")
    }

    /// C++: `bool MSPhaseDefinition::isCommit() const` [const]
    // bool isCommit() const {
    //         return myCommit;
    //     }
    #[allow(unused_variables)]
    pub fn is_commit(&self) -> bool {
        todo!("port MSPhaseDefinition::isCommit")
    }

}

