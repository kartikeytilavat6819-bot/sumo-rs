//! Port of netimport/NIVisumTL.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::options::options_cont::get_string;

/// C++ `throw`: raised as a panic carrying the ProcessError until a top-level catch exists
fn throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| std::panic::panic_any(e))
}


/// Intermediate class for storing visum traffic lights during their import
/// C++ class `NIVisumTL` (netimport/NIVisumTL.h)
pub struct NIVisumTL {
    /// The name of traffic light
    pub myName: String,
    /// The cycle time of traffic light in seconds
    pub myCycleTime: SUMOTime,
    /// The offset in the plan
    pub myOffset: SUMOTime,
    /// The all-red time (unused here)
    pub myIntermediateTime: SUMOTime,
    /// Toggles the usage either of phases or of time periods in signal groups
    pub myPhaseDefined: bool,
    /// Vector of nodes belonging to this traffic light
    pub myNodes: Vec<Ptr<NBNode>>,
    /// Map of used phases if phases defined
    pub myPhases: BTreeMap<String, Ptr<NIVisumTL_Phase>>,
    /// Map of used signal groups
    pub mySignalGroups: BTreeMap<String, Ptr<NIVisumTL_SignalGroup>>,
}

impl NIVisumTL {
    /// C++: `void NIVisumTL::NIVisumTL(const std::string &, SUMOTime, SUMOTime, SUMOTime, bool)`
    /// Constructor
    // NIVisumTL::NIVisumTL(const std::string& name, SUMOTime cycleTime, SUMOTime offset,
    //                      SUMOTime intermediateTime, bool phaseDefined)
    //     : myName(name), myCycleTime(cycleTime), myOffset(offset),
    //       myIntermediateTime(intermediateTime), myPhaseDefined(phaseDefined) {
    // }
    #[allow(unused_variables)]
    pub fn new(name: &str, cycleTime: SUMOTime, offset: SUMOTime, intermediateTime: SUMOTime, phaseDefined: bool) -> Self {
        NIVisumTL {
            myName: name.to_string(),
            myCycleTime: cycleTime,
            myOffset: offset,
            myIntermediateTime: intermediateTime,
            myPhaseDefined: phaseDefined,
            myNodes: Vec::new(),
            myPhases: BTreeMap::new(),
            mySignalGroups: BTreeMap::new(),
        }
    }

    /// C++: `void NIVisumTL::~NIVisumTL() noexcept`
    /// Destructor
    // NIVisumTL::~NIVisumTL() {
    //     for (std::map<std::string, Phase*>::iterator i = myPhases.begin(); i != myPhases.end(); ++i) {
    //         delete i->second;
    //     }
    //     for (std::map<std::string, SignalGroup*>::iterator i = mySignalGroups.begin(); i != mySignalGroups.end(); ++i) {
    //         delete i->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor deletes the phases and signal groups; they are leaked by addPhase/addSignalGroup (see there)
    }

    /// C++: `void NIVisumTL::addNode(NBNode *)`
    /// Adds a node to control
    // void addNode(NBNode* n) {
    //         myNodes.push_back(n);
    //     }
    #[allow(unused_variables)]
    pub fn add_node(&mut self, n: Ptr<NBNode>) {
        self.myNodes.push(n);
    }

    /// C++: `void NIVisumTL::addSignalGroup(const std::string &, SUMOTime, SUMOTime, SUMOTime)`
    /// Adds a signal group
    // void
    // NIVisumTL::addSignalGroup(const std::string& name, SUMOTime startTime, SUMOTime endTime, SUMOTime yellowTime) {
    //     mySignalGroups[name] = new NIVisumTL::SignalGroup(name, startTime, endTime, yellowTime);
    // }
    #[allow(unused_variables)]
    pub fn add_signal_group(&mut self, name: &str, startTime: SUMOTime, endTime: SUMOTime, yellowTime: SUMOTime) {
        // shortcut: C++ deletes these in ~NIVisumTL; they are leaked here until that destructor is ported
        self.mySignalGroups.insert(name.to_string(), Ptr::from_mut(Box::leak(Box::new(NIVisumTL_SignalGroup::new(name, startTime, endTime, yellowTime)))));
    }

    /// C++: `void NIVisumTL::addPhase(const std::string &, SUMOTime, SUMOTime, SUMOTime)`
    /// Adds a phase
    // void
    // NIVisumTL::addPhase(const std::string& name, SUMOTime startTime, SUMOTime endTime, SUMOTime yellowTime) {
    //     myPhases[name] = new NIVisumTL::Phase(startTime, endTime, yellowTime);
    // }
    #[allow(unused_variables)]
    pub fn add_phase(&mut self, name: &str, startTime: SUMOTime, endTime: SUMOTime, yellowTime: SUMOTime) {
        // shortcut: C++ deletes these in ~NIVisumTL; they are leaked here until that destructor is ported
        self.myPhases.insert(name.to_string(), Ptr::from_mut(Box::leak(Box::new(NIVisumTL_Phase::new(startTime, endTime, yellowTime)))));
    }

    /// C++: `std::map<std::string, Phase *> & NIVisumTL::getPhases()`
    /// Returns the map of named phases
    // std::map<std::string, Phase*>& getPhases() {
    //         return myPhases;
    //     }
    #[allow(unused_variables)]
    pub fn get_phases(&mut self) -> &mut BTreeMap<String, Ptr<NIVisumTL_Phase>> {
        &mut self.myPhases
    }

    /// C++: `SignalGroup & NIVisumTL::getSignalGroup(const std::string &)`
    /// Returns the named signal group
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_signal_group(&mut self, name: &str) -> &mut NIVisumTL_SignalGroup {
        // shortcut: C++ has no body for getSignalGroup and operator[] would insert a null entry; a missing group panics here
        self.mySignalGroups.get(name).expect("unknown signal group").m()
    }

    /// C++: `void NIVisumTL::build(NBEdgeCont &, NBTrafficLightLogicCont &)`
    /// build the traffic light and add it to the given container
    // void
    // NIVisumTL::build(NBEdgeCont& ec, NBTrafficLightLogicCont& tlc) {
    //     for (std::vector<NBNode*>::iterator ni = myNodes.begin(); ni != myNodes.end(); ni++) {
    //         NBNode* node = (*ni);
    //         if (node == nullptr) {
    //             WRITE_WARNINGF(TL("invalid node for traffic light '%'"), myName);
    //             continue;
    //         }
    //         TrafficLightType type = SUMOXMLDefinitions::TrafficLightTypes.get(OptionsCont::getOptions().getString("tls.default-type"));
    //         NBLoadedTLDef* def = new NBLoadedTLDef(ec, node->getID(), node, myOffset, type);
    //         tlc.insert(def);
    //         def->setCycleDuration(myCycleTime);
    //         // signalgroups
    //         for (std::map<std::string, SignalGroup*>::iterator gi = mySignalGroups.begin(); gi != mySignalGroups.end(); gi++) {
    //             std::string groupName = (*gi).first;
    //             NIVisumTL::SignalGroup& SG = *(*gi).second;
    //             def->addSignalGroup(groupName);
    //             def->addToSignalGroup(groupName, SG.connections());
    //             // phases
    //             SUMOTime yellowTime = -1;
    //             if (myPhaseDefined) {
    //                 for (std::map<std::string, Phase*>::iterator pi = SG.phases().begin(); pi != SG.phases().end(); pi++) {
    //                     NIVisumTL::Phase& PH = *(*pi).second;
    //                     def->addSignalGroupPhaseBegin(groupName, PH.getStartTime(), NBTrafficLightDefinition::TLCOLOR_GREEN);
    //                     def->addSignalGroupPhaseBegin(groupName, PH.getEndTime(), NBTrafficLightDefinition::TLCOLOR_RED);
    //                     yellowTime = MAX2(PH.getYellowTime(), yellowTime);
    //                 }
    //             } else {
    //                 def->addSignalGroupPhaseBegin(groupName, SG.getStartTime(), NBTrafficLightDefinition::TLCOLOR_GREEN);
    //                 def->addSignalGroupPhaseBegin(groupName, SG.getEndTime(), NBTrafficLightDefinition::TLCOLOR_RED);
    //                 yellowTime = MAX2(SG.getYellowTime(), yellowTime);
    //             }
    //             // yellowTime can be -1 if not given in the input; it will be "patched" later
    //             def->setSignalYellowTimes(groupName, myIntermediateTime, yellowTime);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build(&mut self, ec: &mut NBEdgeCont, tlc: &mut NBTrafficLightLogicCont) {
        for node in self.myNodes.clone() {
            if node.is_null() {
                write_warning(&format!("invalid node for traffic light '{}'", self.myName));
                continue;
            }
            let type_ = throw(TrafficLightTypes.get(&get_string("tls.default-type")));
            let leaked: &'static mut NBLoadedTLDef = Box::leak(Box::new(NBLoadedTLDef::new_2(ec, node.m().get_id(), node, self.myOffset, type_)));
            let def = Ptr::from_mut(leaked);
            tlc.insert(Ptr::from_mut(&mut def.m().base), false);
            def.m().set_cycle_duration(self.myCycleTime);
            // signalgroups
            for (groupName, sg) in self.mySignalGroups.clone() {
                let SG = sg.m();
                def.m().add_signal_group(&groupName);
                def.m().add_to_signal_group_2(&groupName, SG.connections());
                // phases
                let mut yellowTime: SUMOTime = -1;
                if self.myPhaseDefined {
                    for (_, ph) in SG.phases().clone() {
                        let PH = ph.m();
                        def.m().add_signal_group_phase_begin(&groupName, PH.get_start_time(), NBTrafficLightDefinition_TLColor::TLCOLOR_GREEN);
                        def.m().add_signal_group_phase_begin(&groupName, PH.get_end_time(), NBTrafficLightDefinition_TLColor::TLCOLOR_RED);
                        yellowTime = std::cmp::max(PH.get_yellow_time(), yellowTime);
                    }
                } else {
                    def.m().add_signal_group_phase_begin(&groupName, SG.get_start_time(), NBTrafficLightDefinition_TLColor::TLCOLOR_GREEN);
                    def.m().add_signal_group_phase_begin(&groupName, SG.get_end_time(), NBTrafficLightDefinition_TLColor::TLCOLOR_RED);
                    yellowTime = std::cmp::max(SG.get_yellow_time(), yellowTime);
                }
                // yellowTime can be -1 if not given in the input; it will be "patched" later
                def.m().set_signal_yellow_times(&groupName, self.myIntermediateTime, yellowTime);
            }
        }
    }

}

/// A phase
/// C++ class `NIVisumTL::Phase` (netimport/NIVisumTL.h)
pub struct NIVisumTL_Phase {
    /// C++ base class `TimePeriod`
    pub base: NIVisumTL_TimePeriod,
}

impl std::ops::Deref for NIVisumTL_Phase {
    type Target = NIVisumTL_TimePeriod;
    fn deref(&self) -> &NIVisumTL_TimePeriod {
        &self.base
    }
}
impl std::ops::DerefMut for NIVisumTL_Phase {
    fn deref_mut(&mut self) -> &mut NIVisumTL_TimePeriod {
        &mut self.base
    }
}

impl NIVisumTL_Phase {
    /// C++: `void Phase::Phase(SUMOTime, SUMOTime, SUMOTime)`
    /// Constructor
    // Phase(SUMOTime startTime, SUMOTime endTime, SUMOTime yellowTime) : NIVisumTL::TimePeriod(startTime, endTime, yellowTime) {}
    #[allow(unused_variables)]
    pub fn new(startTime: SUMOTime, endTime: SUMOTime, yellowTime: SUMOTime) -> Self {
        NIVisumTL_Phase { base: NIVisumTL_TimePeriod::new(startTime, endTime, yellowTime) }
    }

    /// C++: `void Phase::~Phase() noexcept`
    /// Destructor
    // ~Phase() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has no body
    }

}

/// A signal group can be defined either by a time period or by phases
/// C++ class `NIVisumTL::SignalGroup` (netimport/NIVisumTL.h)
pub struct NIVisumTL_SignalGroup {
    /// C++ base class `TimePeriod`
    pub base: NIVisumTL_TimePeriod,
    /// Connections
    pub myConnections: NBConnectionVector,
    /// phases
    pub myPhases: BTreeMap<String, Ptr<NIVisumTL_Phase>>,
    /// name
    pub myName: String,
}

impl std::ops::Deref for NIVisumTL_SignalGroup {
    type Target = NIVisumTL_TimePeriod;
    fn deref(&self) -> &NIVisumTL_TimePeriod {
        &self.base
    }
}
impl std::ops::DerefMut for NIVisumTL_SignalGroup {
    fn deref_mut(&mut self) -> &mut NIVisumTL_TimePeriod {
        &mut self.base
    }
}

impl NIVisumTL_SignalGroup {
    /// C++: `void SignalGroup::SignalGroup(const std::string &, SUMOTime, SUMOTime, SUMOTime)`
    /// constructor
    // SignalGroup(const std::string& name, SUMOTime startTime, SUMOTime endTime, SUMOTime yellowTime)
    //             : NIVisumTL::TimePeriod(startTime, endTime, yellowTime), myName(name) {}
    #[allow(unused_variables)]
    pub fn new(name: &str, startTime: SUMOTime, endTime: SUMOTime, yellowTime: SUMOTime) -> Self {
        NIVisumTL_SignalGroup {
            base: NIVisumTL_TimePeriod::new(startTime, endTime, yellowTime),
            myConnections: Vec::new(),
            myPhases: BTreeMap::new(),
            myName: name.to_string(),
        }
    }

    /// C++: `void SignalGroup::~SignalGroup() noexcept`
    /// destructor
    // ~SignalGroup() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has no body
    }

    /// C++: `NBConnectionVector & SignalGroup::connections()`
    /// Returns the connections vector
    // NBConnectionVector& connections() {
    //             return myConnections;
    //         }
    #[allow(unused_variables)]
    pub fn connections(&mut self) -> &mut NBConnectionVector {
        &mut self.myConnections
    }

    /// C++: `std::map<std::string, Phase *> & SignalGroup::phases()`
    /// Returns the phases map
    // std::map<std::string, Phase*>& phases() {
    //             return myPhases;
    //         }
    #[allow(unused_variables)]
    pub fn phases(&mut self) -> &mut BTreeMap<String, Ptr<NIVisumTL_Phase>> {
        &mut self.myPhases
    }

}

/// A time period with a start and an end time
/// C++ class `NIVisumTL::TimePeriod` (netimport/NIVisumTL.h)
pub struct NIVisumTL_TimePeriod {
    /// Start time
    pub myStartTime: SUMOTime,
    /// End time
    pub myEndTime: SUMOTime,
    /// Yellow time
    pub myYellowTime: SUMOTime,
}

impl NIVisumTL_TimePeriod {
    /// C++: `void TimePeriod::TimePeriod(SUMOTime, SUMOTime, SUMOTime)`
    /// Constructor
    // TimePeriod(SUMOTime startTime, SUMOTime endTime, SUMOTime yellowTime)
    //             : myStartTime(startTime), myEndTime(endTime), myYellowTime(yellowTime) {}
    #[allow(unused_variables)]
    pub fn new(startTime: SUMOTime, endTime: SUMOTime, yellowTime: SUMOTime) -> Self {
        NIVisumTL_TimePeriod { myStartTime: startTime, myEndTime: endTime, myYellowTime: yellowTime }
    }

    /// C++: `void TimePeriod::~TimePeriod() noexcept`
    /// Destructor
    // ~TimePeriod() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has no body
    }

    /// C++: `SUMOTime TimePeriod::getStartTime()`
    /// Returns the stored start time
    // SUMOTime getStartTime() {
    //             return myStartTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_start_time(&mut self) -> SUMOTime {
        self.myStartTime
    }

    /// C++: `SUMOTime TimePeriod::getEndTime()`
    /// Returns the stored end time
    // SUMOTime getEndTime() {
    //             return myEndTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_end_time(&mut self) -> SUMOTime {
        self.myEndTime
    }

    /// C++: `SUMOTime TimePeriod::getYellowTime()`
    /// Returns the stored yellow time
    // SUMOTime getYellowTime() {
    //             return myYellowTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_yellow_time(&mut self) -> SUMOTime {
        self.myYellowTime
    }

    /// C++: `TimePeriod & TimePeriod::operator=(const TimePeriod &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIVisumTL_TimePeriod) -> &mut NIVisumTL_TimePeriod {
        self.myStartTime = s.myStartTime;
        self.myEndTime = s.myEndTime;
        self.myYellowTime = s.myYellowTime;
        self
    }

}

