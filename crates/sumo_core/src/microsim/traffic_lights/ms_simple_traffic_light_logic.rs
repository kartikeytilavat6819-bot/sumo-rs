//! Port of microsim/traffic_lights/MSSimpleTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A fixed traffic light logic
/// C++ class `MSSimpleTrafficLightLogic` (microsim/traffic_lights/MSSimpleTrafficLightLogic.h)
pub struct MSSimpleTrafficLightLogic {
    /// C++ base class `MSTrafficLightLogic`
    pub base: MSTrafficLightLogic,
    /// The list of phases this logic uses
    pub myPhases: MSTrafficLightLogic_Phases,
    /// The current step
    pub myStep: i32,
    /// whether coordination parameters earliestEnd, latestEnd are
    pub myCoordinated: bool,
}

impl std::ops::Deref for MSSimpleTrafficLightLogic {
    type Target = MSTrafficLightLogic;
    fn deref(&self) -> &MSTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSimpleTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSTrafficLightLogic {
        &mut self.base
    }
}

impl MSSimpleTrafficLightLogic {
    /// C++: `void MSSimpleTrafficLightLogic::MSSimpleTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const SUMOTime, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor
    // MSSimpleTrafficLightLogic::MSSimpleTrafficLightLogic(MSTLLogicControl& tlcontrol,
    //         const std::string& id, const std::string& programID, const SUMOTime offset, const TrafficLightType logicType, const Phases& phases,
    //         int step, SUMOTime delay,
    //         const Parameterised::Map& parameters) :
    //     MSTrafficLightLogic(tlcontrol, id, programID, offset, logicType, delay, parameters),
    //     myPhases(phases),
    //     myStep(step) {
    //     myDefaultCycleTime = computeCycleTime(myPhases);
    //     if (myStep < (int)myPhases.size()) {
    //         myPhases[myStep]->myLastSwitch = SIMSTEP;
    //     }
    //     // the following initializations are only used by 'actuated' and 'delay_based' but do not affect 'static'
    //     if (hasParameter(toString(SUMO_ATTR_CYCLETIME))) {
    //         myDefaultCycleTime = TIME2STEPS(StringUtils::toDouble(Parameterised::getParameter(toString(SUMO_ATTR_CYCLETIME), "")));
    //     }
    //     myCoordinated = StringUtils::toBool(Parameterised::getParameter("coordinated", "false"));
    //     if (myPhases.size() > 0) {
    //         SUMOTime earliest = SIMSTEP + getEarliest(-1);
    //         if (earliest > getNextSwitchTime()) {
    //             mySwitchCommand->deschedule(this);
    //             mySwitchCommand = new SwitchCommand(tlcontrol, this, earliest);
    //             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(mySwitchCommand, earliest);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, offset: SUMOTime, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        let mut base = MSTrafficLightLogic::new(tlcontrol, id, programID, offset, logicType, delay, parameters);
        let myPhases: MSTrafficLightLogic_Phases = phases.clone();
        base.myDefaultCycleTime = base.compute_cycle_time(&myPhases);
        let mut this = MSSimpleTrafficLightLogic {
            base,
            myPhases,
            myStep: step,
            myCoordinated: false,
        };
        // shortcut: the C++ constructor's switch scheduling needs the final address, so callers must call schedule_initial_switch after boxing the logic
        let now = MSNet::get_instance().get_current_time_step();
        if (step as usize) < this.myPhases.len() {
            this.myPhases[step as usize].m().myLastSwitch = now;
        }
        // the following initializations are only used by 'actuated' and 'delay_based' but do not affect 'static'
        if this.base.base2.has_parameter("cycleTime") {
            this.base.myDefaultCycleTime = time2steps(utils::common::string_utils::to_double(&this.base.base2.get_parameter("cycleTime", "")).unwrap_or_else(|e| panic!("{}", e)));
        }
        this.myCoordinated = utils::common::string_utils::to_bool(&this.base.base2.get_parameter("coordinated", "false")).unwrap_or_else(|e| panic!("{}", e));
        this
    }

    /// Second half of the C++ constructor: schedules the first switch (base constructor) and moves it to
    /// `earliest` when that is later. Must run once the logic has its final address (after boxing).
    // shortcut: callers must invoke this right after boxing the logic, upgrade when constructors return Box<Self>
    pub fn schedule_initial_switch(&mut self, tlcontrol: &mut MSTLLogicControl, delay: SUMOTime) {
        self.base.schedule_switch_command(tlcontrol, delay);
        if !self.myPhases.is_empty() {
            let now = MSNet::get_instance().get_current_time_step();
            let earliest: SUMOTime = now + self.get_earliest(-1);
            if earliest > self.base.get_next_switch_time() {
                let this = Ptr::from_mut(&mut self.base);
                self.base.mySwitchCommand.deschedule(this);
                self.base.mySwitchCommand = Ptr::from_raw(Box::into_raw(Box::new(MSTrafficLightLogic_SwitchCommand::new(tlcontrol, this, earliest))));
                let events = MSNet::get_instance().m().get_begin_of_timestep_events();
                events.m().add_event(Ptr::from_raw(self.base.mySwitchCommand.as_raw() as *mut dyn crate::microsim::ms_event_control::Command), earliest);
            }
        }
    }

    /// Applies the signals of the current phase to the controlled links (MSTrafficLightLogic::setTrafficLightSignals,
    /// with the phase taken from this subclass).
    pub fn apply_phase_signals(&self, t: SUMOTime) {
        let state = self.get_current_phase_def().get_state().to_string();
        for i in 0..self.base.myLinks.len() {
            let ls: LinkState = (state.as_bytes()[i] as i8) as LinkState;
            for link in &self.base.myLinks[i] {
                link.m().set_tl_state(ls, t);
            }
        }
    }

    /// C++: `void MSSimpleTrafficLightLogic::~MSSimpleTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSSimpleTrafficLightLogic::~MSSimpleTrafficLightLogic() {
    //     deletePhases();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.delete_phases();
    }

    /// C++: `SUMOTime MSSimpleTrafficLightLogic::trySwitch()` [virtual]
    /// Switches to the next phase
    // SUMOTime
    // MSSimpleTrafficLightLogic::trySwitch() {
    //     // check whether the current duration shall be increased
    //     if (myCurrentDurationIncrement > 0) {
    //         SUMOTime delay = myCurrentDurationIncrement;
    //         myCurrentDurationIncrement = 0;
    //         return delay;
    //     }
    //
    //     // increment the index
    //     if (myPhases[myStep]->nextPhases.size() > 0 && myPhases[myStep]->nextPhases.front() >= 0) {
    //         myStep = myPhases[myStep]->nextPhases.front();
    //     } else {
    //         myStep++;
    //     }
    //     // if the last phase was reached ...
    //     if (myStep >= (int)myPhases.size()) {
    //         // ... set the index to the first phase
    //         myStep = 0;
    //     }
    //     assert((int)myPhases.size() > myStep);
    //     //stores the time the phase started
    //     myPhases[myStep]->myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //     // check whether the next duration was overridden
    //     if (myOverridingTimes.size() > 0) {
    //         SUMOTime nextDuration = myOverridingTimes[0];
    //         myOverridingTimes.erase(myOverridingTimes.begin());
    //         return nextDuration;
    //     }
    //     // return offset to the next switch
    //     return myPhases[myStep]->duration;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        // check whether the current duration shall be increased
        if self.base.myCurrentDurationIncrement > 0 {
            let delay = self.base.myCurrentDurationIncrement;
            self.base.myCurrentDurationIncrement = 0;
            return delay;
        }
        // increment the index
        if self.myPhases[self.myStep as usize].nextPhases.len() > 0 && self.myPhases[self.myStep as usize].nextPhases[0] >= 0 {
            self.myStep = self.myPhases[self.myStep as usize].nextPhases[0];
        } else {
            self.myStep += 1;
        }
        // if the last phase was reached ...
        if self.myStep >= self.myPhases.len() as i32 {
            // ... set the index to the first phase
            self.myStep = 0;
        }
        assert!(self.myPhases.len() as i32 > self.myStep);
        // stores the time the phase started
        let now = MSNet::get_instance().get_current_time_step();
        self.myPhases[self.myStep as usize].m().myLastSwitch = now;
        // check whether the next duration was overridden
        if self.base.myOverridingTimes.len() > 0 {
            let nextDuration = self.base.myOverridingTimes[0];
            self.base.myOverridingTimes.remove(0);
            return nextDuration;
        }
        // return offset to the next switch
        self.myPhases[self.myStep as usize].duration
    }

    /// C++: `int MSSimpleTrafficLightLogic::getPhaseNumber() const` [const, virtual]
    /// Returns the number of phases
    // int
    // MSSimpleTrafficLightLogic::getPhaseNumber() const {
    //     return (int) myPhases.size();
    // }
    #[allow(unused_variables)]
    pub fn get_phase_number(&self) -> i32 {
        self.myPhases.len() as i32
    }

    /// C++: `const Phases & MSSimpleTrafficLightLogic::getPhases() const` [const, virtual]
    /// Returns the phases of this tls program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &MSTrafficLightLogic_Phases {
        &self.myPhases
    }

    /// C++: `Phases & MSSimpleTrafficLightLogic::getPhases()`
    /// Returns the phases of this tls program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases_2(&mut self) -> &mut MSTrafficLightLogic_Phases {
        &mut self.myPhases
    }

    /// C++: `const MSPhaseDefinition & MSSimpleTrafficLightLogic::getPhase(int) const` [const, virtual]
    /// Returns the definition of the phase from the given position within the plan
    // const MSPhaseDefinition&
    // MSSimpleTrafficLightLogic::getPhase(int givenStep) const {
    //     assert((int)myPhases.size() > givenStep);
    //     return *myPhases[givenStep];
    // }
    #[allow(unused_variables)]
    pub fn get_phase(&self, givenstep: i32) -> &MSPhaseDefinition {
        assert!(self.myPhases.len() as i32 > givenstep);
        &*self.myPhases[givenstep as usize]
    }

    /// C++: `int MSSimpleTrafficLightLogic::getCurrentPhaseIndex() const` [const, virtual]
    /// Returns the current index within the program
    // int
    // MSSimpleTrafficLightLogic::getCurrentPhaseIndex() const {
    //     return myStep;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_index(&self) -> i32 {
        self.myStep
    }

    /// C++: `const MSPhaseDefinition & MSSimpleTrafficLightLogic::getCurrentPhaseDef() const` [const, virtual]
    /// Returns the definition of the current phase
    // const MSPhaseDefinition&
    // MSSimpleTrafficLightLogic::getCurrentPhaseDef() const {
    //     return *myPhases[myStep];
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        &*self.myPhases[self.myStep as usize]
    }

    /// C++: `void MSSimpleTrafficLightLogic::resetLastSwitch(SUMOTime)` [virtual]
    /// @}
    // void
    // MSSimpleTrafficLightLogic::resetLastSwitch(SUMOTime t) {
    //     myPhases[myStep]->myLastSwitch = t;
    // }
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        self.myPhases[self.myStep as usize].m().myLastSwitch = t;
    }

    /// C++: `SUMOTime MSSimpleTrafficLightLogic::getPhaseIndexAtTime(SUMOTime) const` [const, virtual]
    /// Returns the index of the logic at the given simulation step
    // SUMOTime
    // MSSimpleTrafficLightLogic::getPhaseIndexAtTime(SUMOTime simStep) const {
    //     SUMOTime position = 0;
    //     if (myStep > 0) {
    //         for (int i = 0; i < myStep; i++) {
    //             position = position + getPhase(i).duration;
    //         }
    //     }
    //     position = position + simStep - getPhase(myStep).myLastSwitch;
    //     position = position % myDefaultCycleTime;
    //     assert(position <= myDefaultCycleTime);
    //     return position;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_index_at_time(&self, simStep: SUMOTime) -> SUMOTime {
        let mut position: SUMOTime = 0;
        if self.myStep > 0 {
            for i in 0..self.myStep {
                position = position + self.get_phase(i).duration;
            }
        }
        let now = MSNet::get_instance().get_current_time_step();
        position = position + simStep - self.get_phase(self.myStep).myLastSwitch;
        position = position % self.base.myDefaultCycleTime;
        assert!(position <= self.base.myDefaultCycleTime);
        position
    }

    /// C++: `SUMOTime MSSimpleTrafficLightLogic::getOffsetFromIndex(int) const` [const, virtual]
    /// Returns the position (start of a phase during a cycle) from of a given step
    // SUMOTime
    // MSSimpleTrafficLightLogic::getOffsetFromIndex(int index) const {
    //     assert(index < (int)myPhases.size());
    //     if (index == 0) {
    //         return 0;
    //     }
    //     SUMOTime pos = 0;
    //     for (int i = 0; i < index; i++) {
    //         pos += getPhase(i).duration;
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn get_offset_from_index(&self, index: i32) -> SUMOTime {
        assert!(index < self.myPhases.len() as i32);
        if index == 0 {
            return 0;
        }
        let mut pos: SUMOTime = 0;
        for i in 0..index {
            pos += self.get_phase(i).duration;
        }
        pos
    }

    /// C++: `int MSSimpleTrafficLightLogic::getIndexFromOffset(SUMOTime) const` [const, virtual]
    /// Returns the step (the phasenumber) of a given position of the cycle
    // int
    // MSSimpleTrafficLightLogic::getIndexFromOffset(SUMOTime offset) const {
    //     offset = offset % myDefaultCycleTime;
    //     if (offset == myDefaultCycleTime) {
    //         return 0;
    //     }
    //     SUMOTime testPos = 0;
    //     for (int i = 0; i < (int)myPhases.size(); i++) {
    //         testPos = testPos + getPhase(i).duration;
    //         if (testPos > offset) {
    //             return i;
    //         }
    //         if (testPos == offset) {
    //             assert((int)myPhases.size() > (i + 1));
    //             return (i + 1);
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_index_from_offset(&self, offset: SUMOTime) -> i32 {
        let offset = offset % self.base.myDefaultCycleTime;
        if offset == self.base.myDefaultCycleTime {
            return 0;
        }
        let mut testPos: SUMOTime = 0;
        for i in 0..self.myPhases.len() {
            testPos = testPos + self.get_phase(i as i32).duration;
            if testPos > offset {
                return i as i32;
            }
            if testPos == offset {
                assert!(self.myPhases.len() > i + 1);
                return (i + 1) as i32;
            }
        }
        0
    }

    /// C++: `void MSSimpleTrafficLightLogic::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual]
    /// Changes the current phase and her duration
    // void
    // MSSimpleTrafficLightLogic::changeStepAndDuration(MSTLLogicControl& tlcontrol,
    //         SUMOTime simStep, int step, SUMOTime stepDuration) {
    //     mySwitchCommand->deschedule(this);
    //     mySwitchCommand = new SwitchCommand(tlcontrol, this, stepDuration + simStep);
    //     if (step >= 0 && step != myStep) {
    //         myStep = step;
    //         myPhases[myStep]->myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //         if (myAmActive) {
    //             // when loading from state, the last loaded program isn't always the active one
    //             setTrafficLightSignals(simStep);
    //         }
    //         tlcontrol.get(getID()).executeOnSwitchActions();
    //     }
    //     MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(
    //         mySwitchCommand, stepDuration + simStep);
    // }
    #[allow(unused_variables)]
    pub fn change_step_and_duration(&mut self, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
        let this = Ptr::from_mut(&mut self.base);
        self.base.mySwitchCommand.deschedule(this);
        self.base.mySwitchCommand = Ptr::from_raw(Box::into_raw(Box::new(MSTrafficLightLogic_SwitchCommand::new(tlcontrol, this, stepDuration + simStep))));
        if step >= 0 && step != self.myStep {
            self.myStep = step;
            let now = MSNet::get_instance().get_current_time_step();
            self.myPhases[self.myStep as usize].m().myLastSwitch = now;
            if self.base.myAmActive {
                // when loading from state, the last loaded program isn't always the active one
                self.apply_phase_signals(simStep);
            }
            let id = self.base.get_id().to_string();
            tlcontrol.get(&id).execute_on_switch_actions();
        }
        let events = MSNet::get_instance().m().get_begin_of_timestep_events();
        events.m().add_event(Ptr::from_raw(self.base.mySwitchCommand.as_raw() as *mut dyn crate::microsim::ms_event_control::Command), stepDuration + simStep);
    }

    /// C++: `void MSSimpleTrafficLightLogic::setPhases(const Phases &, int)`
    /// Replaces the phases and set the phase index
    // void
    // MSSimpleTrafficLightLogic::setPhases(const Phases& phases, int step) {
    //     assert(step < (int)phases.size());
    //     SUMOTime lastSwitch = myPhases[myStep]->getState() == phases[step]->getState() ? myPhases[myStep]->myLastSwitch : SIMSTEP;
    //     deletePhases();
    //     myPhases = phases;
    //     myStep = step;
    //     myDefaultCycleTime = computeCycleTime(myPhases);
    //     myPhases[myStep]->myLastSwitch = lastSwitch;
    // }
    #[allow(unused_variables)]
    pub fn set_phases(&mut self, phases: &MSTrafficLightLogic_Phases, index: i32) {
        let now = MSNet::get_instance().get_current_time_step();
        let lastSwitch: SUMOTime = if self.myPhases[self.myStep as usize].get_state() == phases[index as usize].get_state() {
            self.myPhases[self.myStep as usize].myLastSwitch
        } else {
            now
        };
        self.delete_phases();
        self.myPhases = phases.clone();
        self.myStep = index;
        self.base.myDefaultCycleTime = self.base.compute_cycle_time(&self.myPhases);
        self.myPhases[self.myStep as usize].m().myLastSwitch = lastSwitch;
    }

    /// C++: `void MSSimpleTrafficLightLogic::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the current tls states into the given stream
    // void
    // MSSimpleTrafficLightLogic::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_TLLOGIC);
    //     saveStateAttrs(out);
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_TLLOGIC);
        self.save_state_attrs(out);
        out.close_tag("");
    }

    /// C++: `void MSSimpleTrafficLightLogic::loadState(MSTLLogicControl &, SUMOTime, int, SUMOTime, SUMOTime, SUMOTime, bool)` [virtual]
    /// restores the tls state, including the start of the current phase
    // void
    // MSSimpleTrafficLightLogic::loadState(MSTLLogicControl& tlcontrol, SUMOTime t, int step, SUMOTime spentDuration, SUMOTime nextSwitch, SUMOTime timeInCycle, bool active) {
    //     MSTrafficLightLogic::loadState(tlcontrol, t, step, spentDuration, nextSwitch, timeInCycle, active);
    //     myPhases[myStep]->myLastSwitch = t - spentDuration;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, tlcontrol: &mut MSTLLogicControl, t: SUMOTime, step: i32, spentDuration: SUMOTime, nextSwitch: SUMOTime, timeInCycle: SUMOTime, active: bool) {
        // MSTrafficLightLogic::loadState, with the virtual calls dispatched to this subclass
        self.base.myAmActive = active;
        let remaining = nextSwitch - t;
        self.change_step_and_duration(tlcontrol, t, step, remaining);
        if self.base.myAmActive {
            // when loading from state, the last loaded program isn't always the active one
            self.apply_phase_signals(t - spentDuration);
            let id = self.base.get_id().to_string();
            tlcontrol.get(&id).switch_to_loaded(Ptr::from_mut(&mut self.base));
        }
        self.myPhases[self.myStep as usize].m().myLastSwitch = t - spentDuration;
    }

    /// C++: `SUMOTime MSSimpleTrafficLightLogic::mapTimeInCycle(SUMOTime) const` [const, virtual]
    // SUMOTime
    // MSSimpleTrafficLightLogic::mapTimeInCycle(SUMOTime t) const {
    //     return (myCoordinated
    //             ? (t - myOffset) % myDefaultCycleTime
    //             : (t - myPhases[0]->myLastSwitch) % myDefaultCycleTime);
    // }
    #[allow(unused_variables)]
    pub fn map_time_in_cycle(&self, t: SUMOTime) -> SUMOTime {
        if self.myCoordinated {
            (t - self.base.myOffset) % self.base.myDefaultCycleTime
        } else {
            (t - self.myPhases[0].myLastSwitch) % self.base.myDefaultCycleTime
        }
    }

    /// C++: `const std::string MSSimpleTrafficLightLogic::getParameter(const std::string &, const std::string) const` [const, virtual]
    /// gets a parameter
    // const std::string
    // MSSimpleTrafficLightLogic::getParameter(const std::string& key, const std::string defaultValue) const {
    //     if (key == "cycleTime") {
    //         return toString(STEPS2TIME(myDefaultCycleTime));
    //     } else if (key == "offset") {
    //         return toString(STEPS2TIME(myOffset));
    //     } else if (key == "coordinated") {
    //         return toString(myCoordinated);
    //     } else if (key == "cycleSecond") {
    //         return toString(STEPS2TIME(getTimeInCycle()));
    //     } else if (key == "typeName") {
    //         return toString(this->getLogicType());
    //     }
    //     return Parameterised::getParameter(key, defaultValue);
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str, defaultValue: String) -> String {
        if key == "cycleTime" {
            utils::common::to_string::to_string(&steps2time(self.base.myDefaultCycleTime))
        } else if key == "offset" {
            utils::common::to_string::to_string(&steps2time(self.base.myOffset))
        } else if key == "coordinated" {
            utils::common::to_string::to_string(&self.myCoordinated)
        } else if key == "cycleSecond" {
            let now = MSNet::get_instance().get_current_time_step();
            utils::common::to_string::to_string(&steps2time(self.map_time_in_cycle(now)))
        } else if key == "typeName" {
            TrafficLightTypes.get_string(self.base.get_logic_type()).to_string()
        } else {
            self.base.base2.get_parameter(key, &defaultValue)
        }
    }

    /// C++: `void MSSimpleTrafficLightLogic::setParameter(const std::string &, const std::string &)` [virtual]
    /// Sets a parameter and updates internal constants
    // void
    // MSSimpleTrafficLightLogic::setParameter(const std::string& key, const std::string& value) {
    //     if (key == "cycleTime") {
    //         myDefaultCycleTime = string2time(value);
    //         Parameterised::setParameter(key, value);
    //     } else if (key == "cycleSecond" || key == "typeName") {
    //         throw InvalidArgument(key + " cannot be changed dynamically for traffic light '" + getID() + "'");
    //     } else if (key == "offset") {
    //         myOffset = string2time(value);
    //     } else if (key == "coordinated") {
    //         myCoordinated = StringUtils::toBool(value);
    //         Parameterised::setParameter(key, value);
    //     } else {
    //         Parameterised::setParameter(key, value);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        if key == "cycleTime" {
            self.base.myDefaultCycleTime = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            self.base.base2.set_parameter(key, value);
        } else if key == "cycleSecond" || key == "typeName" {
            panic!("{}", ProcessError::InvalidArgument(format!("{} cannot be changed dynamically for traffic light '{}'", key, self.base.get_id())));
        } else if key == "offset" {
            self.base.myOffset = string2time(value).unwrap_or_else(|e| panic!("{}", e));
        } else if key == "coordinated" {
            self.myCoordinated = utils::common::string_utils::to_bool(value).unwrap_or_else(|e| panic!("{}", e));
            self.base.base2.set_parameter(key, value);
        } else {
            self.base.base2.set_parameter(key, value);
        }
    }

    /// C++: `SUMOTime MSSimpleTrafficLightLogic::getEarliest(SUMOTime) const` [const, protected]
    /// the minimum duration for keeping the current phase when considering 'earliestEnd'
    // SUMOTime
    // MSSimpleTrafficLightLogic::getEarliest(SUMOTime prevStart) const {
    //     SUMOTime earliest = getEarliestEnd();
    //     if (earliest == MSPhaseDefinition::UNSPECIFIED_DURATION) {
    //         return 0;
    //     } else {
    //         if (prevStart >= SIMSTEP - getTimeInCycle() && prevStart < getCurrentPhaseDef().myLastEnd) {
    //             // phase was started and ended once already in the current cycle
    //             // it should not end a second time in the same cycle
    //             earliest += myDefaultCycleTime;
    // #ifdef DEBUG_COORDINATION
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " tl=" << getID() << " getEarliest phase=" << myStep
    //                           << " prevStart= " << STEPS2TIME(prevStart)
    //                           << " prevEnd= " << STEPS2TIME(getCurrentPhaseDef().myLastEnd)
    //                           << " cycleStart=" << STEPS2TIME(SIMSTEP - getTimeInCycle()) << " started Twice - move into next cycle\n";
    //             }
    // #endif
    //         } else {
    //             SUMOTime latest = getLatestEnd();
    //             if (latest != MSPhaseDefinition::UNSPECIFIED_DURATION) {
    //                 const SUMOTime minRemaining = getMinDur() - (SIMSTEP - getCurrentPhaseDef().myLastSwitch);
    //                 const SUMOTime minEnd = getTimeInCycle() + minRemaining;
    //                 if (latest > earliest && latest < minEnd) {
    //                     // cannot terminate phase between earliest and latest -> move end into next cycle
    //                     earliest += myDefaultCycleTime;
    //                 } else if (latest < earliest && latest >= minEnd) {
    //                     // can ignore earliest since it counts from the previous cycle
    //                     earliest -= myDefaultCycleTime;
    //                 }
    // #ifdef DEBUG_COORDINATION
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " tl=" << getID() << " getEarliest phase=" << myStep << " latest=" << STEPS2TIME(latest) << " minEnd="
    //                               << STEPS2TIME(minEnd) << " earliest=" << STEPS2TIME(earliest) << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //         const SUMOTime maxRemaining = getMaxDur() - (SIMSTEP - getCurrentPhaseDef().myLastSwitch);
    //         return MIN2(earliest - getTimeInCycle(), maxRemaining);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_earliest(&self, prevStart: SUMOTime) -> SUMOTime {
        // phase end times are read from this subclass (the C++ getEarliestEnd/getMinDur/... calls are virtual-free here)
        let now = MSNet::get_instance().get_current_time_step();
        let mut earliest = self.get_current_phase_def().earliestEnd;
        if earliest == UNSPECIFIED_DURATION {
            return 0;
        } else {
            let timeInCycle = self.map_time_in_cycle(now);
            if prevStart >= now - timeInCycle && prevStart < self.get_current_phase_def().myLastEnd {
                // phase was started and ended once already in the current cycle
                // it should not end a second time in the same cycle
                earliest += self.base.myDefaultCycleTime;
            } else {
                let latest = self.get_current_phase_def().latestEnd;
                if latest != UNSPECIFIED_DURATION {
                    let minRemaining = self.get_current_phase_def().minDuration - (now - self.get_current_phase_def().myLastSwitch);
                    let minEnd = timeInCycle + minRemaining;
                    if latest > earliest && latest < minEnd {
                        // cannot terminate phase between earliest and latest -> move end into next cycle
                        earliest += self.base.myDefaultCycleTime;
                    } else if latest < earliest && latest >= minEnd {
                        // can ignore earliest since it counts from the previous cycle
                        earliest -= self.base.myDefaultCycleTime;
                    }
                }
            }
            let maxRemaining = self.get_current_phase_def().maxDuration - (now - self.get_current_phase_def().myLastSwitch);
            std::cmp::min(earliest - timeInCycle, maxRemaining)
        }
    }

    /// C++: `SUMOTime MSSimpleTrafficLightLogic::getLatest() const` [const, protected]
    /// the maximum duration for keeping the current phase when considering 'latestEnd'
    // SUMOTime
    // MSSimpleTrafficLightLogic::getLatest() const {
    //     const SUMOTime latest = getLatestEnd();
    //     if (latest == MSPhaseDefinition::UNSPECIFIED_DURATION) {
    //         return SUMOTime_MAX; // no restriction
    //     } else {
    //         if (latest < getEarliestEnd()) {
    //             const SUMOTime running = SIMSTEP - getCurrentPhaseDef().myLastSwitch;
    //             if (running < getTimeInCycle()) {
    //                 // phase was started in the current cycle so the restriction does not apply yet
    //                 return SUMOTime_MAX;
    //             }
    //         }
    // #ifdef DEBUG_COORDINATION
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " tl=" << getID() << " getLatest phase=" << myStep << " latest=" << STEPS2TIME(latest)
    //                       << " cycTime=" << STEPS2TIME(getTimeInCycle()) << " res=" << STEPS2TIME(latest - getTimeInCycle()) << "\n";
    //         }
    // #endif
    //         if (latest == myDefaultCycleTime && getTimeInCycle() == 0) {
    //             // special case: end on cylce time wrap-around
    //             return 0;
    //         }
    //         return MAX2(SUMOTime(0), latest - getTimeInCycle());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_latest(&self) -> SUMOTime {
        let now = MSNet::get_instance().get_current_time_step();
        let latest = self.get_current_phase_def().latestEnd;
        if latest == UNSPECIFIED_DURATION {
            return SUMOTIME_MAX; // no restriction
        } else {
            let timeInCycle = self.map_time_in_cycle(now);
            if latest < self.get_current_phase_def().earliestEnd {
                let running = now - self.get_current_phase_def().myLastSwitch;
                if running < timeInCycle {
                    // phase was started in the current cycle so the restriction does not apply yet
                    return SUMOTIME_MAX;
                }
            }
            if latest == self.base.myDefaultCycleTime && timeInCycle == 0 {
                // special case: end on cylce time wrap-around
                return 0;
            }
            std::cmp::max(0, latest - timeInCycle)
        }
    }

    /// C++: `void MSSimpleTrafficLightLogic::saveStateAttrs(OutputDevice &) const` [const, protected]
    // void
    // MSSimpleTrafficLightLogic::saveStateAttrs(OutputDevice& out) const {
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     out.writeAttr(SUMO_ATTR_PROGRAMID, getProgramID());
    //     out.writeAttr(SUMO_ATTR_PHASE, getCurrentPhaseIndex());
    //     out.writeAttr(SUMO_ATTR_DURATION, getSpentDuration());
    //     out.writeAttr(SUMO_ATTR_UNTIL, getNextSwitchTime());
    //     out.writeAttr(SUMO_ATTR_CYCLETIME, getTimeInCycle());
    //     out.writeAttr(SUMO_ATTR_ACTIVE, myAmActive);
    // }
    #[allow(unused_variables)]
    pub fn save_state_attrs(&self, out: &mut OutputDevice) {
        let now = MSNet::get_instance().get_current_time_step();
        let timeInCycle = self.map_time_in_cycle(now);
        let spentDuration: SUMOTime = now - self.get_current_phase_def().myLastSwitch;
        out.write_attr(SUMO_ATTR_ID, self.base.get_id());
        out.write_attr(SUMO_ATTR_PROGRAMID, self.base.get_program_id());
        out.write_attr(SUMO_ATTR_PHASE, &self.get_current_phase_index());
        out.write_attr(SUMO_ATTR_DURATION, &spentDuration);
        out.write_attr(SUMO_ATTR_UNTIL, &self.base.get_next_switch_time());
        out.write_attr(SUMO_ATTR_CYCLETIME, &timeInCycle);
        out.write_attr(SUMO_ATTR_ACTIVE, &self.base.myAmActive);
    }

    /// C++: `void MSSimpleTrafficLightLogic::deletePhases()` [private]
    /// frees memory responsibilities
    // void
    // MSSimpleTrafficLightLogic::deletePhases() {
    //     for (int i = 0; i < (int)myPhases.size(); i++) {
    //         delete myPhases[i];
    //     }
    // }
    #[allow(unused_variables)]
    pub fn delete_phases(&mut self) {
        // C++ deletes every phase definition; they were allocated with Box::into_raw by their creator
        for i in 0..self.myPhases.len() {
            let p = self.myPhases[i];
            unsafe {
                drop(Box::from_raw(p.as_raw()));
            }
        }
    }

}

