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
        todo!("port MSSimpleTrafficLightLogic::MSSimpleTrafficLightLogic")
    }

    /// C++: `void MSSimpleTrafficLightLogic::~MSSimpleTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSSimpleTrafficLightLogic::~MSSimpleTrafficLightLogic() {
    //     deletePhases();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSSimpleTrafficLightLogic::~MSSimpleTrafficLightLogic")
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
        todo!("port MSSimpleTrafficLightLogic::trySwitch")
    }

    /// C++: `int MSSimpleTrafficLightLogic::getPhaseNumber() const` [const, virtual]
    /// Returns the number of phases
    // int
    // MSSimpleTrafficLightLogic::getPhaseNumber() const {
    //     return (int) myPhases.size();
    // }
    #[allow(unused_variables)]
    pub fn get_phase_number(&self) -> i32 {
        todo!("port MSSimpleTrafficLightLogic::getPhaseNumber")
    }

    /// C++: `const Phases & MSSimpleTrafficLightLogic::getPhases() const` [const, virtual]
    /// Returns the phases of this tls program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &MSTrafficLightLogic_Phases {
        todo!("port MSSimpleTrafficLightLogic::getPhases")
    }

    /// C++: `Phases & MSSimpleTrafficLightLogic::getPhases()`
    /// Returns the phases of this tls program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases_2(&mut self) -> &mut MSTrafficLightLogic_Phases {
        todo!("port MSSimpleTrafficLightLogic::getPhases")
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
        todo!("port MSSimpleTrafficLightLogic::getPhase")
    }

    /// C++: `int MSSimpleTrafficLightLogic::getCurrentPhaseIndex() const` [const, virtual]
    /// Returns the current index within the program
    // int
    // MSSimpleTrafficLightLogic::getCurrentPhaseIndex() const {
    //     return myStep;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_index(&self) -> i32 {
        todo!("port MSSimpleTrafficLightLogic::getCurrentPhaseIndex")
    }

    /// C++: `const MSPhaseDefinition & MSSimpleTrafficLightLogic::getCurrentPhaseDef() const` [const, virtual]
    /// Returns the definition of the current phase
    // const MSPhaseDefinition&
    // MSSimpleTrafficLightLogic::getCurrentPhaseDef() const {
    //     return *myPhases[myStep];
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        todo!("port MSSimpleTrafficLightLogic::getCurrentPhaseDef")
    }

    /// C++: `void MSSimpleTrafficLightLogic::resetLastSwitch(SUMOTime)` [virtual]
    /// @}
    // void
    // MSSimpleTrafficLightLogic::resetLastSwitch(SUMOTime t) {
    //     myPhases[myStep]->myLastSwitch = t;
    // }
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        todo!("port MSSimpleTrafficLightLogic::resetLastSwitch")
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
        todo!("port MSSimpleTrafficLightLogic::getPhaseIndexAtTime")
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
        todo!("port MSSimpleTrafficLightLogic::getOffsetFromIndex")
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
        todo!("port MSSimpleTrafficLightLogic::getIndexFromOffset")
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
        todo!("port MSSimpleTrafficLightLogic::changeStepAndDuration")
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
        todo!("port MSSimpleTrafficLightLogic::setPhases")
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
        todo!("port MSSimpleTrafficLightLogic::saveState")
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
        todo!("port MSSimpleTrafficLightLogic::loadState")
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
        todo!("port MSSimpleTrafficLightLogic::mapTimeInCycle")
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
        todo!("port MSSimpleTrafficLightLogic::getParameter")
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
        todo!("port MSSimpleTrafficLightLogic::setParameter")
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
        todo!("port MSSimpleTrafficLightLogic::getEarliest")
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
        todo!("port MSSimpleTrafficLightLogic::getLatest")
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
        todo!("port MSSimpleTrafficLightLogic::saveStateAttrs")
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
        todo!("port MSSimpleTrafficLightLogic::deletePhases")
    }

}

