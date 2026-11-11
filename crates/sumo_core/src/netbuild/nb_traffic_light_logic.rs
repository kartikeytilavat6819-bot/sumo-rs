//! Port of netbuild/NBTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<PhaseDefinition> NBTrafficLightLogic::PhaseDefinitionVector`
pub type NBTrafficLightLogic_PhaseDefinitionVector = Vec<NBTrafficLightLogic_PhaseDefinition>;

/// A SUMO-compliant built logic for a traffic light
/// C++ class `NBTrafficLightLogic` (netbuild/NBTrafficLightLogic.h)
pub struct NBTrafficLightLogic {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// The number of participating links
    pub myNumLinks: i32,
    /// The tls program's subid
    pub mySubID: String,
    /// The tls program's offset
    pub myOffset: SUMOTime,
    /// The junction logic's storage for traffic light phase list
    pub myPhases: NBTrafficLightLogic_PhaseDefinitionVector,
    /// The algorithm type for the traffic light
    pub myType: TrafficLightType,
}

impl std::ops::Deref for NBTrafficLightLogic {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NBTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl NBTrafficLightLogic {
    /// C++: `void NBTrafficLightLogic::NBTrafficLightLogic(const std::string &, const std::string &, int, SUMOTime, TrafficLightType)`
    /// Constructor
    // NBTrafficLightLogic::NBTrafficLightLogic(const std::string& id,
    //         const std::string& subid, int noLinks,
    //         SUMOTime offset, TrafficLightType type) :
    //     Named(id), myNumLinks(noLinks), mySubID(subid),
    //     myOffset(offset),
    //     myType(type) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, subid: &str, noLinks: i32, offset: SUMOTime, type_: TrafficLightType) -> Self {
        todo!("port NBTrafficLightLogic::NBTrafficLightLogic")
    }

    /// C++: `void NBTrafficLightLogic::NBTrafficLightLogic(const NBTrafficLightLogic *)`
    /// Copy Constructor
    // NBTrafficLightLogic::NBTrafficLightLogic(const NBTrafficLightLogic* logic) :
    //     Named(logic->getID()),
    //     myNumLinks(logic->myNumLinks),
    //     mySubID(logic->getProgramID()),
    //     myOffset(logic->getOffset()),
    //     myPhases(logic->myPhases.begin(), logic->myPhases.end()),
    //     myType(logic->getType()) {
    //     updateParameters(logic->getParametersMap());
    // }
    #[allow(unused_variables)]
    pub fn new_2(logic: Ptr<NBTrafficLightLogic>) -> Self {
        todo!("port NBTrafficLightLogic::NBTrafficLightLogic")
    }

    /// C++: `void NBTrafficLightLogic::~NBTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // NBTrafficLightLogic::~NBTrafficLightLogic() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBTrafficLightLogic::~NBTrafficLightLogic")
    }

    /// C++: `void NBTrafficLightLogic::addStep(const SUMOTime, const std::string &, const std::vector<int> &, const std::string &, const int)`
    /// Adds a phase to the logic (static)
    // void
    // NBTrafficLightLogic::addStep(const SUMOTime duration, const std::string& state, const std::vector<int>& next, const std::string& name, const int index) {
    //     addStep(duration, state,
    //             NBTrafficLightDefinition::UNSPECIFIED_DURATION,
    //             NBTrafficLightDefinition::UNSPECIFIED_DURATION,
    //             NBTrafficLightDefinition::UNSPECIFIED_DURATION,
    //             NBTrafficLightDefinition::UNSPECIFIED_DURATION,
    //             NBTrafficLightDefinition::UNSPECIFIED_DURATION,
    //             NBTrafficLightDefinition::UNSPECIFIED_DURATION,
    //             NBTrafficLightDefinition::UNSPECIFIED_DURATION,
    //             name, next, index);
    // }
    #[allow(unused_variables)]
    pub fn add_step(&mut self, duration: SUMOTime, state: &str, next: &Vec<i32>, name: &str, index: i32) {
        todo!("port NBTrafficLightLogic::addStep")
    }

    /// C++: `void NBTrafficLightLogic::addStep(const SUMOTime, const std::string &, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const std::string &, const std::vector<int> &, int)`
    /// Adds a phase to the logic (actuated)
    // void
    // NBTrafficLightLogic::addStep(const SUMOTime duration, const std::string& state, const SUMOTime minDur, const SUMOTime maxDur, const SUMOTime earliestEnd,
    //                              const SUMOTime latestEnd, const SUMOTime vehExt, const SUMOTime yellow, const SUMOTime red,
    //                              const std::string& name,
    //                              const std::vector<int>& next,
    //                              int index) {
    //     // check state size
    //     if (myNumLinks == 0) {
    //         // initialize
    //         myNumLinks = (int)state.size();
    //     } else if ((int)state.size() != myNumLinks) {
    //         throw ProcessError("When adding phase to tlLogic '" + getID() + "': state length of " + toString(state.size()) +
    //                            " does not match declared number of links " + toString(myNumLinks));
    //     }
    //     // check state contents
    //     const std::string::size_type illegal = state.find_first_not_of(SUMOXMLDefinitions::ALLOWED_TLS_LINKSTATES);
    //     if (std::string::npos != illegal) {
    //         throw ProcessError(TLF("When adding phase: illegal character '%' in state", toString(state[illegal])));
    //     }
    //     // interpret index
    //     if (index < 0 || index >= (int)myPhases.size()) {
    //         // insert at the end
    //         index = (int)myPhases.size();
    //     }
    //     myPhases.insert(myPhases.begin() + index, PhaseDefinition(duration, state, minDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, next, name));
    // }
    #[allow(unused_variables)]
    pub fn add_step_2(&mut self, duration: SUMOTime, state: &str, minDur: SUMOTime, maxDur: SUMOTime, earliestEnd: SUMOTime, latestEnd: SUMOTime, vehExt: SUMOTime, yellow: SUMOTime, red: SUMOTime, name: &str, next: &Vec<i32>, index: i32) {
        todo!("port NBTrafficLightLogic::addStep")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseState(int, int, LinkState)`
    /// Modifies the state for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseState(int phaseIndex, int tlIndex, LinkState linkState) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     std::string& phaseState = myPhases[phaseIndex].state;
    //     assert(tlIndex < (int)phaseState.size());
    //     phaseState[tlIndex] = (char)linkState;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_state(&mut self, phaseIndex: i32, tlIndex: i32, linkState: LinkState) {
        todo!("port NBTrafficLightLogic::setPhaseState")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseDuration(int, SUMOTime)`
    /// Modifies the duration for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseDuration(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].duration = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_duration(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseDuration")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseMinDuration(int, SUMOTime)`
    /// Modifies the min duration for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseMinDuration(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].minDur = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_min_duration(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseMinDuration")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseMaxDuration(int, SUMOTime)`
    /// Modifies the max duration for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseMaxDuration(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].maxDur = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_max_duration(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseMaxDuration")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseEarliestEnd(int, SUMOTime)`
    /// Modifies the min duration for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseEarliestEnd(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].earliestEnd = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_earliest_end(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseEarliestEnd")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseLatestEnd(int, SUMOTime)`
    /// Modifies the max duration for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseLatestEnd(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].latestEnd = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_latest_end(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseLatestEnd")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseVehExt(int, SUMOTime)`
    /// Modifies the veh ex for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseVehExt(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].vehExt = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_veh_ext(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseVehExt")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseYellow(int, SUMOTime)`
    /// Modifies the veh ex for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseYellow(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].yellow = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_yellow(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseYellow")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseRed(int, SUMOTime)`
    /// Modifies the veh ex for an existing phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseRed(int phaseIndex, SUMOTime duration) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].red = duration;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_red(&mut self, phaseIndex: i32, duration: SUMOTime) {
        todo!("port NBTrafficLightLogic::setPhaseRed")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseNext(int, const std::vector<int> &)`
    /// Modifies the next phase (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseNext(int phaseIndex, const std::vector<int>& next) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].next = next;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_next(&mut self, phaseIndex: i32, next: &Vec<i32>) {
        todo!("port NBTrafficLightLogic::setPhaseNext")
    }

    /// C++: `void NBTrafficLightLogic::setPhaseName(int, const std::string &)`
    /// Modifies the phase name (used by netedit)
    // void
    // NBTrafficLightLogic::setPhaseName(int phaseIndex, const std::string& name) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     myPhases[phaseIndex].name = name;
    // }
    #[allow(unused_variables)]
    pub fn set_phase_name(&mut self, phaseIndex: i32, name: &str) {
        todo!("port NBTrafficLightLogic::setPhaseName")
    }

    /// C++: `void NBTrafficLightLogic::overrideState(int, const char)`
    /// override state with the given character(used by netedit)
    // void
    // NBTrafficLightLogic::overrideState(int phaseIndex, const char c) {
    //     assert(phaseIndex < (int)myPhases.size());
    //     for (int i = 0; i < (int)myPhases[phaseIndex].state.size(); i++) {
    //         myPhases[phaseIndex].state[i] = c;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn override_state(&mut self, phaseIndex: i32, c: i8) {
        todo!("port NBTrafficLightLogic::overrideState")
    }

    /// C++: `void NBTrafficLightLogic::deletePhase(int)`
    // void
    // NBTrafficLightLogic::deletePhase(int index) {
    //     if (index >= (int)myPhases.size()) {
    //         throw InvalidArgument("Index " + toString(index) + " out of range for logic with "
    //                               + toString(myPhases.size()) + " phases.");
    //     }
    //     myPhases.erase(myPhases.begin() + index);
    // }
    #[allow(unused_variables)]
    pub fn delete_phase(&mut self, index: i32) {
        todo!("port NBTrafficLightLogic::deletePhase")
    }

    /// C++: `void NBTrafficLightLogic::swapPhase(int, int)`
    // void
    // NBTrafficLightLogic::swapPhase(int indexPhaseA, int indexPhaseB) {
    //     if (indexPhaseA >= (int)myPhases.size()) {
    //         throw InvalidArgument("Index " + toString(indexPhaseA) + " out of range for logic with "
    //                               + toString(myPhases.size()) + " phases.");
    //     }
    //     if (indexPhaseB >= (int)myPhases.size()) {
    //         throw InvalidArgument("Index " + toString(indexPhaseB) + " out of range for logic with "
    //                               + toString(myPhases.size()) + " phases.");
    //     }
    //     // declare auxiliar PhaseDefinition and swap
    //     const auto auxPhase = myPhases.at(indexPhaseA);
    //     myPhases.at(indexPhaseA) = myPhases.at(indexPhaseB);
    //     myPhases.at(indexPhaseB) = auxPhase;
    // }
    #[allow(unused_variables)]
    pub fn swap_phase(&mut self, indexPhaseA: i32, indexPhaseB: i32) {
        todo!("port NBTrafficLightLogic::swapPhase")
    }

    /// C++: `void NBTrafficLightLogic::swapfirstPhase()`
    /// swap first phase
    // void
    // NBTrafficLightLogic::swapfirstPhase() {
    //     const auto firstPhase = myPhases.front();
    //     myPhases.erase(myPhases.begin());
    //     myPhases.push_back(firstPhase);
    // }
    #[allow(unused_variables)]
    pub fn swapfirst_phase(&mut self) {
        todo!("port NBTrafficLightLogic::swapfirstPhase")
    }

    /// C++: `void NBTrafficLightLogic::swaplastPhase()`
    /// swap first phase
    // void
    // NBTrafficLightLogic::swaplastPhase() {
    //     const auto lastPhase = myPhases.back();
    //     myPhases.pop_back();
    //     myPhases.insert(myPhases.begin(), lastPhase);
    // }
    #[allow(unused_variables)]
    pub fn swaplast_phase(&mut self) {
        todo!("port NBTrafficLightLogic::swaplastPhase")
    }

    /// C++: `void NBTrafficLightLogic::setStateLength(int, LinkState)`
    // void
    // NBTrafficLightLogic::setStateLength(int numLinks, LinkState fill) {
    //     if (myNumLinks > numLinks) {
    //         for (PhaseDefinition& p : myPhases) {
    //             p.state = p.state.substr(0, numLinks);
    //         }
    //     } else {
    //         std::string add(numLinks - myNumLinks, (char)fill);
    //         for (PhaseDefinition& p : myPhases) {
    //             p.state = p.state + add;
    //         }
    //     }
    //     myNumLinks = numLinks;
    // }
    #[allow(unused_variables)]
    pub fn set_state_length(&mut self, numLinks: i32, fill: LinkState) {
        todo!("port NBTrafficLightLogic::setStateLength")
    }

    /// C++: `void NBTrafficLightLogic::deleteStateIndex(int)`
    /// remove the index from all phase states
    // void
    // NBTrafficLightLogic::deleteStateIndex(int index) {
    //     assert(index >= 0);
    //     assert(index < myNumLinks);
    //     for (PhaseDefinition& p : myPhases) {
    //         p.state.erase(index, 1);
    //     }
    //     myNumLinks--;
    // }
    #[allow(unused_variables)]
    pub fn delete_state_index(&mut self, index: i32) {
        todo!("port NBTrafficLightLogic::deleteStateIndex")
    }

    /// C++: `void NBTrafficLightLogic::resetPhases()`
    /// deletes all phases and reset the expect number of links
    // void
    // NBTrafficLightLogic::resetPhases() {
    //     myNumLinks = 0;
    //     myPhases.clear();
    // }
    #[allow(unused_variables)]
    pub fn reset_phases(&mut self) {
        todo!("port NBTrafficLightLogic::resetPhases")
    }

    /// C++: `void NBTrafficLightLogic::closeBuilding(bool)`
    /// closes the building process
    // void
    // NBTrafficLightLogic::closeBuilding(bool checkVarDurations) {
    //     for (int i = 0; i < (int)myPhases.size() - 1;) {
    //         if (myPhases[i].state != myPhases[i + 1].state || myPhases[i].next.size() > 0 || myPhases[i + 1].next.size() > 0 || myPhases[i].name != myPhases[i + 1].name) {
    //             ++i;
    //             continue;
    //         }
    //         myPhases[i].duration += myPhases[i + 1].duration;
    //         if (myPhases[i + 1].minDur != NBTrafficLightDefinition::UNSPECIFIED_DURATION) {
    //             if (myPhases[i].minDur != NBTrafficLightDefinition::UNSPECIFIED_DURATION) {
    //                 myPhases[i].minDur += myPhases[i + 1].minDur;
    //             } else {
    //                 myPhases[i].minDur = myPhases[i + 1].minDur;
    //             }
    //         }
    //         if (myPhases[i + 1].maxDur != NBTrafficLightDefinition::UNSPECIFIED_DURATION) {
    //             if (myPhases[i].maxDur != NBTrafficLightDefinition::UNSPECIFIED_DURATION) {
    //                 myPhases[i].maxDur += myPhases[i + 1].maxDur;
    //             } else {
    //                 myPhases[i].maxDur = myPhases[i + 1].maxDur;
    //             }
    //         }
    //         myPhases.erase(myPhases.begin() + i + 1);
    //     }
    //     // check if actuated lights are defined correctly
    //     if (checkVarDurations) {
    //         if (myType != TrafficLightType::STATIC) {
    //             bool found = false;
    //             for (auto p : myPhases) {
    //                 if (p.minDur != NBTrafficLightDefinition::UNSPECIFIED_DURATION
    //                         || p.maxDur != NBTrafficLightDefinition::UNSPECIFIED_DURATION) {
    //                     found = true;
    //                     break;
    //                 }
    //             }
    //             if (!found) {
    //                 WRITE_WARNINGF(TL("Non-static traffic light '%' does not define variable phase length."), getID());
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_building(&mut self, checkVarDurations: bool) {
        todo!("port NBTrafficLightLogic::closeBuilding")
    }

    /// C++: `SUMOTime NBTrafficLightLogic::getDuration() const` [const]
    /// Returns the duration of the complete cycle
    // SUMOTime
    // NBTrafficLightLogic::getDuration() const {
    //     SUMOTime duration = 0;
    //     for (PhaseDefinitionVector::const_iterator i = myPhases.begin(); i != myPhases.end(); ++i) {
    //         duration += (*i).duration;
    //     }
    //     return duration;
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        todo!("port NBTrafficLightLogic::getDuration")
    }

    /// C++: `void NBTrafficLightLogic::setOffset(SUMOTime)`
    /// Sets the offset of this tls
    // void setOffset(SUMOTime offset) {
    //         myOffset = offset;
    //     }
    #[allow(unused_variables)]
    pub fn set_offset(&mut self, offset: SUMOTime) {
        todo!("port NBTrafficLightLogic::setOffset")
    }

    /// C++: `const std::string & NBTrafficLightLogic::getProgramID() const` [const]
    /// Returns the ProgramID
    // const std::string& getProgramID() const {
    //         return mySubID;
    //     }
    #[allow(unused_variables)]
    pub fn get_program_id(&self) -> &str {
        todo!("port NBTrafficLightLogic::getProgramID")
    }

    /// C++: `const std::vector<PhaseDefinition> & NBTrafficLightLogic::getPhases() const` [const]
    /// Returns the phases
    // const std::vector<PhaseDefinition>& getPhases() const {
    //         return myPhases;
    //     }
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &Vec<NBTrafficLightLogic_PhaseDefinition> {
        todo!("port NBTrafficLightLogic::getPhases")
    }

    /// C++: `void NBTrafficLightLogic::copyPhase(const int, const int)`
    /// copy phase values in other
    // void copyPhase(const int origin, const int destination) {
    //         myPhases.at(destination) = myPhases.at(origin);
    //     }
    #[allow(unused_variables)]
    pub fn copy_phase(&mut self, origin: i32, destination: i32) {
        todo!("port NBTrafficLightLogic::copyPhase")
    }

    /// C++: `SUMOTime NBTrafficLightLogic::getOffset() const` [const]
    /// Returns the offset of first switch
    // SUMOTime getOffset() const {
    //         return myOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_offset(&self) -> SUMOTime {
        todo!("port NBTrafficLightLogic::getOffset")
    }

    /// C++: `int NBTrafficLightLogic::getNumLinks()`
    /// Returns the number of participating links
    // int getNumLinks() {
    //         return myNumLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_num_links(&mut self) -> i32 {
        todo!("port NBTrafficLightLogic::getNumLinks")
    }

    /// C++: `TrafficLightType NBTrafficLightLogic::getType() const` [const]
    /// get the algorithm type (static etc..)
    // TrafficLightType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> TrafficLightType {
        todo!("port NBTrafficLightLogic::getType")
    }

    /// C++: `void NBTrafficLightLogic::setType(TrafficLightType)`
    /// set the algorithm type (static etc..)
    // void setType(TrafficLightType type) {
    //         myType = type;
    //     }
    #[allow(unused_variables)]
    pub fn set_type(&mut self, type_: TrafficLightType) {
        todo!("port NBTrafficLightLogic::setType")
    }

    /// C++: `void NBTrafficLightLogic::setProgramID(const std::string &)`
    /// Sets the programID
    // void setProgramID(const std::string& programID) {
    //         mySubID = programID;
    //     }
    #[allow(unused_variables)]
    pub fn set_program_id(&mut self, programID: &str) {
        todo!("port NBTrafficLightLogic::setProgramID")
    }

    /// C++: `NBTrafficLightLogic & NBTrafficLightLogic::operator=(const NBTrafficLightLogic &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBTrafficLightLogic) -> &mut NBTrafficLightLogic {
        todo!("port NBTrafficLightLogic::operator=")
    }

}

/// The definition of a single phase of the logic
/// C++ class `NBTrafficLightLogic::PhaseDefinition` (netbuild/NBTrafficLightLogic.h)
pub struct NBTrafficLightLogic_PhaseDefinition {
    /// The duration of the phase in s
    pub duration: SUMOTime,
    /// The state definition
    pub state: String,
    /// minimum duration (for actuated)
    pub minDur: SUMOTime,
    /// maximum duration duration (for actuated)
    pub maxDur: SUMOTime,
    /// minimum duration (for actuated)
    pub earliestEnd: SUMOTime,
    /// maximum duration duration (for actuated)
    pub latestEnd: SUMOTime,
    /// veh ext (for NEMA)
    pub vehExt: SUMOTime,
    /// yellow (for NEMA)
    pub yellow: SUMOTime,
    /// red (for NEMA)
    pub red: SUMOTime,
    /// next phase indices or empty list
    pub next: Vec<i32>,
    /// option phase name
    pub name: String,
}

impl NBTrafficLightLogic_PhaseDefinition {
    /// C++: `void PhaseDefinition::PhaseDefinition(const SUMOTime, const std::string &, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const SUMOTime, const std::vector<int> &, const std::string &)`
    /// Constructor
    // PhaseDefinition(const SUMOTime duration_, const std::string& state_, const SUMOTime minDur_, const SUMOTime maxDur_,
    //                         const SUMOTime earliestEnd_, const SUMOTime latestEnd_, const SUMOTime vehExt_, const SUMOTime yellow_,
    //                         const SUMOTime red_, const std::vector<int>& next_, const std::string& name_) :
    //             duration(duration_),
    //             state(state_),
    //             minDur(minDur_),
    //             maxDur(maxDur_),
    //             earliestEnd(earliestEnd_),
    //             latestEnd(latestEnd_),
    //             vehExt(vehExt_),
    //             yellow(yellow_),
    //             red(red_),
    //             next(next_),
    //             name(name_)
    //         { }
    #[allow(unused_variables)]
    pub fn new(duration_: SUMOTime, state_: &str, minDur_: SUMOTime, maxDur_: SUMOTime, earliestEnd_: SUMOTime, latestEnd_: SUMOTime, vehExt_: SUMOTime, yellow_: SUMOTime, red_: SUMOTime, next_: &Vec<i32>, name_: &str) -> Self {
        todo!("port PhaseDefinition::PhaseDefinition")
    }

    /// C++: `void PhaseDefinition::~PhaseDefinition() noexcept`
    /// Destructor
    // ~PhaseDefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PhaseDefinition::~PhaseDefinition")
    }

    /// C++: `bool PhaseDefinition::operator!=(const PhaseDefinition &) const` [const]
    /// Comparison operator
    // bool operator!=(const PhaseDefinition& pd) const {
    //             return ((pd.duration != duration) ||
    //                     (pd.minDur != minDur) ||
    //                     (pd.maxDur != maxDur) ||
    //                     (pd.earliestEnd != earliestEnd) ||
    //                     (pd.latestEnd != latestEnd) ||
    //                     (pd.state != state) ||
    //                     (pd.next != next) ||
    //                     (pd.name != name));
    //         }
    #[allow(unused_variables)]
    pub fn ne_op(&self, pd: &NBTrafficLightLogic_PhaseDefinition) -> bool {
        todo!("port PhaseDefinition::operator!=")
    }

}

