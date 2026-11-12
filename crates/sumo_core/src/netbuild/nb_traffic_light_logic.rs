//! Port of netbuild/NBTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<PhaseDefinition> NBTrafficLightLogic::PhaseDefinitionVector`
pub type NBTrafficLightLogic_PhaseDefinitionVector = Vec<NBTrafficLightLogic_PhaseDefinition>;

/// A SUMO-compliant built logic for a traffic light
/// C++ class `NBTrafficLightLogic` (netbuild/NBTrafficLightLogic.h)
#[repr(C)]
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
        Self {
            base: Named::new(id),
            base2: Parameterised::new(),
            myNumLinks: noLinks,
            mySubID: subid.to_string(),
            myOffset: offset,
            myPhases: Vec::new(),
            myType: type_,
        }
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
        let mut ret = Self {
            base: Named::new(logic.get_id()),
            base2: Parameterised::new(),
            myNumLinks: logic.myNumLinks,
            mySubID: logic.get_program_id().to_string(),
            myOffset: logic.get_offset(),
            myPhases: logic.myPhases.clone(),
            myType: logic.get_type(),
        };
        ret.base2.update_parameters(logic.base2.get_parameters_map());
        ret
    }

    /// C++: `void NBTrafficLightLogic::~NBTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // NBTrafficLightLogic::~NBTrafficLightLogic() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        use crate::netbuild::nb_traffic_light_definition::NBTrafficLightDefinition_UNSPECIFIED_DURATION as U;
        self.add_step_2(duration, state, U, U, U, U, U, U, U, name, next, index)
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
        // check state size
        if self.myNumLinks == 0 {
            // initialize
            self.myNumLinks = state.len() as i32;
        } else if state.len() as i32 != self.myNumLinks {
            panic!("{}", ProcessError::new(format!("When adding phase to tlLogic '{}': state length of {} does not match declared number of links {}", self.get_id(), state.len(), self.myNumLinks)));
        }
        // check state contents
        if let Some(illegal) = state.find(|c: char| !utils::xml::sumo_xml_definitions::ALLOWED_TLS_LINKSTATES.contains(c)) {
            let ch = state[illegal..].chars().next().unwrap();
            panic!("{}", ProcessError::new(format!("When adding phase: illegal character '{}' in state", ch)));
        }
        // interpret index
        let mut index = index;
        if index < 0 || index >= self.myPhases.len() as i32 {
            // insert at the end
            index = self.myPhases.len() as i32;
        }
        self.myPhases.insert(index as usize, NBTrafficLightLogic_PhaseDefinition::new(duration, state, minDur, maxDur, earliestEnd, latestEnd, vehExt, yellow, red, next, name));
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
        let phaseState = &mut self.myPhases[phaseIndex as usize].state;
        let i = tlIndex as usize;
        // C++ writes one char in place; the state is ASCII so byte index == char index
        phaseState.replace_range(i..i + 1, &((linkState as u8) as char).to_string());
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
        self.myPhases[phaseIndex as usize].duration = duration;
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
        self.myPhases[phaseIndex as usize].minDur = duration;
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
        self.myPhases[phaseIndex as usize].maxDur = duration;
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
        self.myPhases[phaseIndex as usize].earliestEnd = duration;
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
        self.myPhases[phaseIndex as usize].latestEnd = duration;
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
        self.myPhases[phaseIndex as usize].vehExt = duration;
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
        self.myPhases[phaseIndex as usize].yellow = duration;
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
        self.myPhases[phaseIndex as usize].red = duration;
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
        self.myPhases[phaseIndex as usize].next = next.clone();
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
        self.myPhases[phaseIndex as usize].name = name.to_string();
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
        let p = &mut self.myPhases[phaseIndex as usize];
        let ch = (c as u8) as char;
        // every byte of the (ASCII) state is replaced by c
        p.state = p.state.chars().map(|_| ch).collect();
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
        if index >= self.myPhases.len() as i32 {
            panic!("{}", InvalidArgument(format!("Index {} out of range for logic with {} phases.", index, self.myPhases.len())).0);
        }
        self.myPhases.remove(index as usize);
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
        if indexPhaseA >= self.myPhases.len() as i32 {
            panic!("{}", InvalidArgument(format!("Index {} out of range for logic with {} phases.", indexPhaseA, self.myPhases.len())).0);
        }
        if indexPhaseB >= self.myPhases.len() as i32 {
            panic!("{}", InvalidArgument(format!("Index {} out of range for logic with {} phases.", indexPhaseB, self.myPhases.len())).0);
        }
        // declare auxiliar PhaseDefinition and swap
        self.myPhases.swap(indexPhaseA as usize, indexPhaseB as usize);
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
        let firstPhase = self.myPhases.remove(0);
        self.myPhases.push(firstPhase);
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
        let lastPhase = self.myPhases.pop().unwrap();
        self.myPhases.insert(0, lastPhase);
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
        if self.myNumLinks > numLinks {
            for p in self.myPhases.iter_mut() {
                // substr(0, numLinks) clamps like C++
                p.state = p.state.chars().take(numLinks as usize).collect();
            }
        } else {
            let add: String = std::iter::repeat((fill as u8) as char).take((numLinks - self.myNumLinks) as usize).collect();
            for p in self.myPhases.iter_mut() {
                p.state.push_str(&add);
            }
        }
        self.myNumLinks = numLinks;
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
        for p in self.myPhases.iter_mut() {
            p.state.remove(index as usize);
        }
        self.myNumLinks -= 1;
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
        self.myNumLinks = 0;
        self.myPhases.clear();
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
        use crate::netbuild::nb_traffic_light_definition::NBTrafficLightDefinition_UNSPECIFIED_DURATION as U;
        let mut i: i32 = 0;
        while i < self.myPhases.len() as i32 - 1 {
            let iu = i as usize;
            if self.myPhases[iu].state != self.myPhases[iu + 1].state || self.myPhases[iu].next.len() > 0 || self.myPhases[iu + 1].next.len() > 0 || self.myPhases[iu].name != self.myPhases[iu + 1].name {
                i += 1;
                continue;
            }
            let nextDuration = self.myPhases[iu + 1].duration;
            self.myPhases[iu].duration += nextDuration;
            if self.myPhases[iu + 1].minDur != U {
                if self.myPhases[iu].minDur != U {
                    let v = self.myPhases[iu + 1].minDur;
                    self.myPhases[iu].minDur += v;
                } else {
                    self.myPhases[iu].minDur = self.myPhases[iu + 1].minDur;
                }
            }
            if self.myPhases[iu + 1].maxDur != U {
                if self.myPhases[iu].maxDur != U {
                    let v = self.myPhases[iu + 1].maxDur;
                    self.myPhases[iu].maxDur += v;
                } else {
                    self.myPhases[iu].maxDur = self.myPhases[iu + 1].maxDur;
                }
            }
            self.myPhases.remove(iu + 1);
        }
        // check if actuated lights are defined correctly
        if checkVarDurations {
            if self.myType != TrafficLightType::STATIC {
                let found = self.myPhases.iter().any(|p| p.minDur != U || p.maxDur != U);
                if !found {
                    write_warningf!("Non-static traffic light '%' does not define variable phase length.", self.get_id());
                }
            }
        }
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
        let mut duration: SUMOTime = 0;
        for p in self.myPhases.iter() {
            duration += p.duration;
        }
        duration
    }

    /// C++: `void NBTrafficLightLogic::setOffset(SUMOTime)`
    /// Sets the offset of this tls
    // void setOffset(SUMOTime offset) {
    //         myOffset = offset;
    //     }
    #[allow(unused_variables)]
    pub fn set_offset(&mut self, offset: SUMOTime) {
        self.myOffset = offset;
    }

    /// C++: `const std::string & NBTrafficLightLogic::getProgramID() const` [const]
    /// Returns the ProgramID
    // const std::string& getProgramID() const {
    //         return mySubID;
    //     }
    #[allow(unused_variables)]
    pub fn get_program_id(&self) -> &str {
        &self.mySubID
    }

    /// C++: `const std::vector<PhaseDefinition> & NBTrafficLightLogic::getPhases() const` [const]
    /// Returns the phases
    // const std::vector<PhaseDefinition>& getPhases() const {
    //         return myPhases;
    //     }
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &Vec<NBTrafficLightLogic_PhaseDefinition> {
        &self.myPhases
    }

    /// C++: `void NBTrafficLightLogic::copyPhase(const int, const int)`
    /// copy phase values in other
    // void copyPhase(const int origin, const int destination) {
    //         myPhases.at(destination) = myPhases.at(origin);
    //     }
    #[allow(unused_variables)]
    pub fn copy_phase(&mut self, origin: i32, destination: i32) {
        self.myPhases[destination as usize] = self.myPhases[origin as usize].clone();
    }

    /// C++: `SUMOTime NBTrafficLightLogic::getOffset() const` [const]
    /// Returns the offset of first switch
    // SUMOTime getOffset() const {
    //         return myOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_offset(&self) -> SUMOTime {
        self.myOffset
    }

    /// C++: `int NBTrafficLightLogic::getNumLinks()`
    /// Returns the number of participating links
    // int getNumLinks() {
    //         return myNumLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_num_links(&mut self) -> i32 {
        self.myNumLinks
    }

    /// C++: `TrafficLightType NBTrafficLightLogic::getType() const` [const]
    /// get the algorithm type (static etc..)
    // TrafficLightType getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> TrafficLightType {
        self.myType
    }

    /// C++: `void NBTrafficLightLogic::setType(TrafficLightType)`
    /// set the algorithm type (static etc..)
    // void setType(TrafficLightType type) {
    //         myType = type;
    //     }
    #[allow(unused_variables)]
    pub fn set_type(&mut self, type_: TrafficLightType) {
        self.myType = type_;
    }

    /// C++: `void NBTrafficLightLogic::setProgramID(const std::string &)`
    /// Sets the programID
    // void setProgramID(const std::string& programID) {
    //         mySubID = programID;
    //     }
    #[allow(unused_variables)]
    pub fn set_program_id(&mut self, programID: &str) {
        self.mySubID = programID.to_string();
    }

    /// C++: `NBTrafficLightLogic & NBTrafficLightLogic::operator=(const NBTrafficLightLogic &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBTrafficLightLogic) -> &mut NBTrafficLightLogic {
        // C++ declares operator= private with no definition, so it can never be called
        unreachable!("NBTrafficLightLogic::operator= is declared private and not defined in C++")
    }

}

/// The definition of a single phase of the logic
/// C++ class `NBTrafficLightLogic::PhaseDefinition` (netbuild/NBTrafficLightLogic.h)
#[derive(Clone)]
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
        Self {
            duration: duration_,
            state: state_.to_string(),
            minDur: minDur_,
            maxDur: maxDur_,
            earliestEnd: earliestEnd_,
            latestEnd: latestEnd_,
            vehExt: vehExt_,
            yellow: yellow_,
            red: red_,
            next: next_.clone(),
            name: name_.to_string(),
        }
    }

    /// C++: `void PhaseDefinition::~PhaseDefinition() noexcept`
    /// Destructor
    // ~PhaseDefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        pd.duration != self.duration
            || pd.minDur != self.minDur
            || pd.maxDur != self.maxDur
            || pd.earliestEnd != self.earliestEnd
            || pd.latestEnd != self.latestEnd
            || pd.state != self.state
            || pd.next != self.next
            || pd.name != self.name
    }

}

