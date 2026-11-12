//! Port of microsim/traffic_lights/MSOffTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A traffic lights logic which represents a tls in an off-mode
/// C++ class `MSOffTrafficLightLogic` (microsim/traffic_lights/MSOffTrafficLightLogic.h)
#[repr(C)]
pub struct MSOffTrafficLightLogic {
    /// C++ base class `MSTrafficLightLogic`
    pub base: MSTrafficLightLogic,
    /// The phase definition (only one)
    pub myPhaseDefinition: MSTrafficLightLogic_Phases,
}

impl std::ops::Deref for MSOffTrafficLightLogic {
    type Target = MSTrafficLightLogic;
    fn deref(&self) -> &MSTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSOffTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSTrafficLightLogic {
        &mut self.base
    }
}

impl MSOffTrafficLightLogic {
    /// C++: `void MSOffTrafficLightLogic::MSOffTrafficLightLogic(MSTLLogicControl &, const std::string &)`
    /// Constructor
    // MSOffTrafficLightLogic::MSOffTrafficLightLogic(MSTLLogicControl& tlcontrol, const std::string& id) :
    //     MSTrafficLightLogic(tlcontrol, id, "off", 0, TrafficLightType::OFF, 0, Parameterised::Map()) {
    //     myDefaultCycleTime = TIME2STEPS(120);
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str) -> Self {
        let mut base = MSTrafficLightLogic::new(tlcontrol, id, "off", 0, TrafficLightType::OFF, 0, &ParameterisedMap::new());
        base.myDefaultCycleTime = time2steps(120.);
        Self { base, myPhaseDefinition: Vec::new() }
    }

    /// C++: `void MSOffTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with information about incoming lanes
    // void
    // MSOffTrafficLightLogic::init(NLDetectorBuilder&) {
    //     rebuildPhase();
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        self.rebuild_phase();
    }

    /// C++: `void MSOffTrafficLightLogic::~MSOffTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSOffTrafficLightLogic::~MSOffTrafficLightLogic() {
    //     for (MSTrafficLightLogic::Phases::const_iterator i = myPhaseDefinition.begin(); i != myPhaseDefinition.end(); ++i) {
    //         delete *i;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for i in self.myPhaseDefinition.iter() {
            // C++ `delete *i`: the phases were allocated with Box::into_raw in rebuild_phase
            unsafe { drop(Box::from_raw((*i).as_raw())) };
        }
    }

    /// C++: `void MSOffTrafficLightLogic::adaptLinkInformationFrom(const MSTrafficLightLogic &)` [virtual]
    /// Applies information about controlled links and lanes from the given logic
    // void
    // MSOffTrafficLightLogic::adaptLinkInformationFrom(const MSTrafficLightLogic& logic) {
    //     MSTrafficLightLogic::adaptLinkInformationFrom(logic);
    //     rebuildPhase();
    // }
    #[allow(unused_variables)]
    pub fn adapt_link_information_from(&mut self, logic: &MSTrafficLightLogic) {
        self.base.adapt_link_information_from(logic);
        self.rebuild_phase();
    }

    /// C++: `SUMOTime MSOffTrafficLightLogic::trySwitch()` [virtual]
    /// Switches to the next phase
    // SUMOTime trySwitch() override {
    //         return 120 * DELTA_T;
    //     }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        120 * delta_t()
    }

    /// C++: `int MSOffTrafficLightLogic::getPhaseNumber() const` [const, virtual]
    /// Returns the number of phases
    // int
    // MSOffTrafficLightLogic::getPhaseNumber() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_number(&self) -> i32 {
        0
    }

    /// C++: `const Phases & MSOffTrafficLightLogic::getPhases() const` [const, virtual]
    /// Returns the phases of this tls program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &MSTrafficLightLogic_Phases {
        &self.myPhaseDefinition
    }

    /// C++: `const MSPhaseDefinition & MSOffTrafficLightLogic::getPhase(int) const` [const, virtual]
    /// Returns the definition of the phase from the given position within the plan
    // const MSPhaseDefinition&
    // MSOffTrafficLightLogic::getPhase(int) const {
    //     return *myPhaseDefinition[0];
    // }
    #[allow(unused_variables)]
    pub fn get_phase(&self, givenstep: i32) -> &MSPhaseDefinition {
        &*self.myPhaseDefinition[0]
    }

    /// C++: `const std::string MSOffTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic as a string
    // const std::string getLogicType() const {
    //         return "offTrafficLightLogic";
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> String {
        "offTrafficLightLogic".to_string()
    }

    /// C++: `int MSOffTrafficLightLogic::getCurrentPhaseIndex() const` [const, virtual]
    /// Returns the current index within the program
    // int
    // MSOffTrafficLightLogic::getCurrentPhaseIndex() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_index(&self) -> i32 {
        0
    }

    /// C++: `const MSPhaseDefinition & MSOffTrafficLightLogic::getCurrentPhaseDef() const` [const, virtual]
    /// Returns the definition of the current phase
    // const MSPhaseDefinition&
    // MSOffTrafficLightLogic::getCurrentPhaseDef() const {
    //     return *myPhaseDefinition[0];
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        &*self.myPhaseDefinition[0]
    }

    /// C++: `void MSOffTrafficLightLogic::resetLastSwitch(SUMOTime)` [virtual]
    /// @}
    // void
    // MSOffTrafficLightLogic::resetLastSwitch(SUMOTime t) {
    //     myPhaseDefinition[0]->myLastSwitch = t;
    // }
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        self.myPhaseDefinition[0].m().myLastSwitch = t;
    }

    /// C++: `SUMOTime MSOffTrafficLightLogic::getPhaseIndexAtTime(SUMOTime) const` [const, virtual]
    /// Returns the index of the logic at the given simulation step
    // SUMOTime
    // MSOffTrafficLightLogic::getPhaseIndexAtTime(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_index_at_time(&self, simStep: SUMOTime) -> SUMOTime {
        0
    }

    /// C++: `SUMOTime MSOffTrafficLightLogic::getOffsetFromIndex(int) const` [const, virtual]
    /// Returns the position (start of a phase during a cycle) from of a given step
    // SUMOTime
    // MSOffTrafficLightLogic::getOffsetFromIndex(int) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_offset_from_index(&self, index: i32) -> SUMOTime {
        0
    }

    /// C++: `int MSOffTrafficLightLogic::getIndexFromOffset(SUMOTime) const` [const, virtual]
    /// Returns the step (the phasenumber) of a given position of the cycle
    // int
    // MSOffTrafficLightLogic::getIndexFromOffset(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_index_from_offset(&self, offset: SUMOTime) -> i32 {
        0
    }

    /// C++: `void MSOffTrafficLightLogic::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual]
    /// Changes the current phase and her duration
    // void changeStepAndDuration(MSTLLogicControl& tlcontrol, SUMOTime simStep, int step, SUMOTime stepDuration) override {
    //         UNUSED_PARAMETER(tlcontrol);
    //         UNUSED_PARAMETER(simStep);
    //         UNUSED_PARAMETER(step);
    //         UNUSED_PARAMETER(stepDuration);
    //     }
    #[allow(unused_variables)]
    pub fn change_step_and_duration(&mut self, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
        // C++ body is empty (all parameters are UNUSED_PARAMETER)
        let _ = (tlcontrol, simStep, step, stepDuration);
    }

    /// C++: `void MSOffTrafficLightLogic::rebuildPhase()` [private]
    /// (Re)builds the internal phase definition
    // void
    // MSOffTrafficLightLogic::rebuildPhase() {
    //     int no = (int)getLinks().size();
    //     std::string state;
    //     for (int i = 0; i < no; ++i) {
    //         bool foundMajor = false;
    //         bool foundMinor = false;
    //         bool foundAllwayStop = false;
    //         for (const MSLink* l : myLinks[i]) {
    //             /// @note. all links for the same index should have the same
    //             if (l->getOffState() == LINKSTATE_TL_OFF_BLINKING) {
    //                 foundMinor = true;
    //             } else if (l->getOffState() == LINKSTATE_TL_OFF_NOSIGNAL) {
    //                 foundMajor = true;
    //             } else if (l->getOffState() == LINKSTATE_ALLWAY_STOP) {
    //                 foundAllwayStop = true;
    //             } else {
    //                 WRITE_WARNINGF(TL("Invalid 'off'-state for link % at junction '%'"), toString(l->getIndex()), l->getJunction()->getID());
    //             }
    //         }
    //         if (foundMajor && foundMinor) {
    //             WRITE_WARNINGF(TL("Inconsistent 'off'-states for linkIndex % at tlLogic '%'"), toString(i), getID());
    //         }
    //         state += toString(foundAllwayStop ? LINKSTATE_ALLWAY_STOP : (foundMinor ? LINKSTATE_TL_OFF_BLINKING : LINKSTATE_TL_OFF_NOSIGNAL));
    //     }
    //     for (MSTrafficLightLogic::Phases::const_iterator i = myPhaseDefinition.begin(); i != myPhaseDefinition.end(); ++i) {
    //         delete *i;
    //     }
    //     myPhaseDefinition.clear();
    //     myPhaseDefinition.push_back(new MSPhaseDefinition(TIME2STEPS(120), state));
    // }
    #[allow(unused_variables)]
    pub fn rebuild_phase(&mut self) {
        let no = self.get_links().len() as i32;
        let mut state = String::new();
        for i in 0..no {
            let mut foundMajor = false;
            let mut foundMinor = false;
            let mut foundAllwayStop = false;
            for l in self.get_links()[i as usize].iter() {
                // all links for the same index should have the same
                if l.get_off_state() == LINKSTATE_TL_OFF_BLINKING {
                    foundMinor = true;
                } else if l.get_off_state() == LINKSTATE_TL_OFF_NOSIGNAL {
                    foundMajor = true;
                } else if l.get_off_state() == LINKSTATE_ALLWAY_STOP {
                    foundAllwayStop = true;
                } else {
                    utils::write_warningf!("Invalid 'off'-state for link % at junction '%'", l.get_index(), l.get_junction().get_id());
                }
            }
            if foundMajor && foundMinor {
                utils::write_warningf!("Inconsistent 'off'-states for linkIndex % at tlLogic '%'", i, self.get_id());
            }
            // toString(LinkState) prints the link state character (LinkStates bijection)
            let linkState = if foundAllwayStop { LINKSTATE_ALLWAY_STOP } else if foundMinor { LINKSTATE_TL_OFF_BLINKING } else { LINKSTATE_TL_OFF_NOSIGNAL };
            state += LinkStates.get_string(linkState);
        }
        for i in self.myPhaseDefinition.iter() {
            // C++ `delete *i`
            unsafe { drop(Box::from_raw((*i).as_raw())) };
        }
        self.myPhaseDefinition.clear();
        self.myPhaseDefinition.push(Ptr::from_raw(Box::into_raw(Box::new(MSPhaseDefinition::new(time2steps(120.), &state, "")))));
    }

}

