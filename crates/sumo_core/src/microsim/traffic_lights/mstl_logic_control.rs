//! Port of microsim/traffic_lights/MSTLLogicControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::to_double;
use utils::common::time::{delta_t, string2time, steps2time, time2steps, SUMOTIME_DAY, SUMOTIME_MAX};
use utils::common::to_string::to_string;

const TRACI_PROGRAM: &str = "online";

/// A class that stores and controls tls and switching of their programs
/// C++ class `MSTLLogicControl` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl {
    /// A map of ids to corresponding WAUTs
    pub myWAUTs: BTreeMap<String, Ptr<MSTLLogicControl_WAUT>>,
    /// A list of currently running switching procedures
    pub myCurrentlySwitched: Vec<MSTLLogicControl_WAUTSwitchProcess>,
    /// A map from ids to the corresponding variants
    pub myLogics: BTreeMap<String, Ptr<MSTLLogicControl_TLSLogicVariants>>,
    /// Information whether the net was completely loaded
    pub myNetWasLoaded: bool,
}

impl MSTLLogicControl {
    /// C++: `void MSTLLogicControl::MSTLLogicControl()`
    /// Constructor
    // MSTLLogicControl::MSTLLogicControl()
    //     : myNetWasLoaded(false) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            myWAUTs: BTreeMap::new(),
            myCurrentlySwitched: Vec::new(),
            myLogics: BTreeMap::new(),
            myNetWasLoaded: false,
        }
    }

    /// C++: `void MSTLLogicControl::~MSTLLogicControl()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // delete tls
        let logics = std::mem::take(&mut self.myLogics);
        for (_id, variants) in logics {
            variants.m().drop_();
            unsafe { drop(Box::from_raw(variants.as_raw())) };
        }
        // delete WAUTs
        let wauts = std::mem::take(&mut self.myWAUTs);
        for (_id, waut) in wauts {
            unsafe { drop(Box::from_raw(waut.as_raw())) };
        }
    }

    /// C++: `bool MSTLLogicControl::closeNetworkReading()`
    /// Lets MSTLLogicControl know that the network has been loaded
    // bool
    // MSTLLogicControl::closeNetworkReading() {
    //     bool hadErrors = false;
    //     for (const auto& it : myLogics) {
    //         hadErrors |= !it.second->checkOriginalTLS();
    //         it.second->saveInitialStates();
    //     }
    //     myNetWasLoaded = true;
    //     return !hadErrors;
    // }
    #[allow(unused_variables)]
    pub fn close_network_reading(&mut self) -> bool {
        let mut had_errors = false;
        for it in self.myLogics.values() {
            had_errors |= !it.check_original_tls();
            it.m().save_initial_states();
        }
        self.myNetWasLoaded = true;
        !had_errors
    }

    /// C++: `void MSTLLogicControl::setTrafficLightSignals(SUMOTime) const` [const]
    /// Lets all running (current) tls programs apply their current signal states to links they control
    // void
    // MSTLLogicControl::setTrafficLightSignals(SUMOTime t) const {
    //     for (std::map<std::string, TLSLogicVariants*>::const_iterator i = myLogics.begin(); i != myLogics.end(); ++i) {
    //         (*i).second->getActive()->setTrafficLightSignals(t);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_traffic_light_signals(&self, t: SUMOTime) {
        for variants in self.myLogics.values() {
            variants.get_active().set_traffic_light_signals(t);
        }
    }

    /// C++: `std::vector<MSTrafficLightLogic *> MSTLLogicControl::getAllLogics() const` [const]
    /// Returns a vector which contains all logics
    // std::vector<MSTrafficLightLogic*>
    // MSTLLogicControl::getAllLogics() const {
    //     std::vector<MSTrafficLightLogic*> ret;
    //     std::map<std::string, TLSLogicVariants*>::const_iterator i;
    //     for (i = myLogics.begin(); i != myLogics.end(); ++i) {
    //         std::vector<MSTrafficLightLogic*> s = (*i).second->getAllLogics();
    //         copy(s.begin(), s.end(), back_inserter(ret));
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_all_logics(&self) -> Vec<Ptr<MSTrafficLightLogic>> {
        let mut ret = Vec::new();
        for variants in self.myLogics.values() {
            ret.extend(variants.get_all_logics());
        }
        ret
    }

    /// C++: `TLSLogicVariants & MSTLLogicControl::get(const std::string &) const` [const]
    /// Returns the variants of a named tls
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get(&self, id: &str) -> &mut MSTLLogicControl_TLSLogicVariants {
        match self.myLogics.get(id) {
            Some(variants) => variants.m(),
            None => panic!("The tls '{}' is not known.", id),
        }
    }

    /// C++: `MSTrafficLightLogic * MSTLLogicControl::get(const std::string &, const std::string &) const` [const]
    /// Returns a single program (variant) defined by the tls id and the program programID
    // MSTrafficLightLogic*
    // MSTLLogicControl::get(const std::string& id, const std::string& programID) const {
    //     std::map<std::string, TLSLogicVariants*>::const_iterator i = myLogics.find(id);
    //     if (i == myLogics.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second->getLogic(programID);
    // }
    #[allow(unused_variables)]
    pub fn get_2(&self, id: &str, programID: &str) -> Ptr<MSTrafficLightLogic> {
        match self.myLogics.get(id) {
            Some(variants) => variants.get_logic(programID),
            None => Ptr::null(),
        }
    }

    /// C++: `MSTrafficLightLogic * MSTLLogicControl::getActive(const std::string &) const` [const]
    /// Returns the active program of a named tls
    // MSTrafficLightLogic*
    // MSTLLogicControl::getActive(const std::string& id) const {
    //     std::map<std::string, TLSLogicVariants*>::const_iterator i = myLogics.find(id);
    //     if (i == myLogics.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second->getActive();
    // }
    #[allow(unused_variables)]
    pub fn get_active(&self, id: &str) -> Ptr<MSTrafficLightLogic> {
        match self.myLogics.get(id) {
            Some(variants) => variants.get_active(),
            None => Ptr::null(),
        }
    }

    /// C++: `std::vector<std::string> MSTLLogicControl::getAllTLIds() const` [const]
    /// Returns the ids of all existing variants-structures, which are the ids of their contained tls logics (not the logic's program-ids)
    // std::vector<std::string>
    // MSTLLogicControl::getAllTLIds() const {
    //     std::vector<std::string> ret;
    //     for (std::map<std::string, TLSLogicVariants*>::const_iterator i = myLogics.begin(); i != myLogics.end(); ++i) {
    //         ret.push_back((*i).first);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_all_tl_ids(&self) -> Vec<String> {
        self.myLogics.keys().cloned().collect()
    }

    /// C++: `bool MSTLLogicControl::add(const std::string &, const std::string &, MSTrafficLightLogic *, bool)`
    /// Adds a tls program to the container
    // bool
    // MSTLLogicControl::add(const std::string& id, const std::string& programID,
    //                       MSTrafficLightLogic* logic, bool newDefault) {
    //     std::map<std::string, TLSLogicVariants*>::iterator it = myLogics.find(id);
    //     TLSLogicVariants* tlmap;
    //     if (it == myLogics.end()) {
    //         tlmap = myLogics[id] = new TLSLogicVariants();
    //     } else {
    //         tlmap = it->second;
    //     }
    //     return tlmap->addLogic(programID, logic, myNetWasLoaded, newDefault);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, id: &str, programID: &str, logic: Ptr<MSTrafficLightLogic>, newDefault: bool) -> bool {
        if !self.myLogics.contains_key(id) {
            let variants = Ptr::from_raw(Box::into_raw(Box::new(MSTLLogicControl_TLSLogicVariants::new())));
            self.myLogics.insert(id.to_string(), variants);
        }
        let tlmap = self.myLogics[id];
        tlmap.m().add_logic(programID, logic, self.myNetWasLoaded, newDefault)
    }

    /// C++: `bool MSTLLogicControl::knows(const std::string &) const` [const]
    /// Returns the information whether the named tls is stored
    // bool
    // MSTLLogicControl::knows(const std::string& id) const {
    //     return myLogics.count(id) != 0;
    // }
    #[allow(unused_variables)]
    pub fn knows(&self, id: &str) -> bool {
        self.myLogics.contains_key(id)
    }

    /// C++: `bool MSTLLogicControl::isActive(const MSTrafficLightLogic *) const` [const]
    /// Returns whether the given tls program is the currently active for his tls
    // bool
    // MSTLLogicControl::isActive(const MSTrafficLightLogic* tl) const {
    //     std::map<std::string, TLSLogicVariants*>::const_iterator i = myLogics.find(tl->getID());
    //     if (i == myLogics.end()) {
    //         return false;
    //     }
    //     return (*i).second->isActive(tl);
    // }
    #[allow(unused_variables)]
    pub fn is_active(&self, tl: Ptr<MSTrafficLightLogic>) -> bool {
        match self.myLogics.get(tl.get_id()) {
            Some(variants) => variants.is_active(tl),
            None => false,
        }
    }

    /// C++: `void MSTLLogicControl::switchTo(const std::string &, const std::string &)`
    /// Switches the named (id) tls to the named (programID) program
    // void
    // MSTLLogicControl::switchTo(const std::string& id, const std::string& programID) {
    //     // try to get the tls program definitions
    //     std::map<std::string, TLSLogicVariants*>::iterator i = myLogics.find(id);
    //     // handle problems
    //     if (i == myLogics.end()) {
    //         throw ProcessError(TLF("Could not switch tls '%' to program '%': No such tls exists.", id, programID));
    //     }
    //     (*i).second->switchTo(*this, programID);
    // }
    #[allow(unused_variables)]
    pub fn switch_to(&mut self, id: &str, programID: &str) {
        // try to get the tls program definitions; handle problems
        if !self.myLogics.contains_key(id) {
            panic!("{}", utils::tlf!("Could not switch tls '%' to program '%': No such tls exists.", id, programID));
        }
        let variants = self.myLogics[id];
        variants.m().switch_to(self, programID);
    }

    /// C++: `void MSTLLogicControl::addWAUT(SUMOTime, const std::string &, const std::string &, SUMOTime)`
    /// Adds a WAUT definition
    // void
    // MSTLLogicControl::addWAUT(SUMOTime refTime, const std::string& id,
    //                           const std::string& startProg, SUMOTime period) {
    //     // check whether the waut was already defined
    //     if (myWAUTs.find(id) != myWAUTs.end()) {
    //         // report an error if so
    //         throw InvalidArgument(TLF("Waut '%' was already defined.", id));
    //     }
    //     WAUT* w = new WAUT;
    //     w->id = id;
    //     w->refTime = refTime;
    //     w->startProg = startProg;
    //     w->period = period;
    //     myWAUTs[id] = w;
    // }
    #[allow(unused_variables)]
    pub fn add_waut(&mut self, refTime: SUMOTime, id: &str, startProg: &str, period: SUMOTime) {
        // check whether the waut was already defined
        if self.myWAUTs.contains_key(id) {
            panic!("{}", utils::tlf!("Waut '%' was already defined.", id));
        }
        let w = MSTLLogicControl_WAUT {
            id: id.to_string(),
            startProg: startProg.to_string(),
            refTime,
            period,
            switches: Vec::new(),
            junctions: Vec::new(),
        };
        self.myWAUTs.insert(id.to_string(), Ptr::from_raw(Box::into_raw(Box::new(w))));
    }

    /// C++: `void MSTLLogicControl::addWAUTSwitch(const std::string &, SUMOTime, const std::string &)`
    /// Adds a WAUT switch step to a previously built WAUT
    // void
    // MSTLLogicControl::addWAUTSwitch(const std::string& wautid,
    //                                 SUMOTime when, const std::string& to) {
    //     // try to get the waut
    //     if (myWAUTs.find(wautid) == myWAUTs.end()) {
    //         // report an error if the waut is not known
    //         throw InvalidArgument(TLF("Waut '%' was not yet defined.", wautid));
    //     }
    //     // build and save the waut switch definition
    //     WAUT* waut = myWAUTs[wautid];
    //     WAUTSwitch s;
    //     s.to = to;
    //     s.when = (waut->refTime + when);
    //     if (waut->period > 0) {
    //         s.when = s.when % waut->period;
    //     }
    //     myWAUTs[wautid]->switches.push_back(s);
    // }
    #[allow(unused_variables)]
    pub fn add_waut_switch(&mut self, wautid: &str, when: SUMOTime, to: &str) {
        // try to get the waut
        if !self.myWAUTs.contains_key(wautid) {
            panic!("{}", utils::tlf!("Waut '%' was not yet defined.", wautid));
        }
        // build and save the waut switch definition
        let waut = self.myWAUTs[wautid];
        let mut s = MSTLLogicControl_WAUTSwitch {
            to: to.to_string(),
            when: waut.refTime + when,
        };
        if waut.period > 0 {
            s.when = s.when % waut.period;
        }
        waut.m().switches.push(s);
    }

    /// C++: `void MSTLLogicControl::addWAUTJunction(const std::string &, const std::string &, const std::string &, bool)`
    /// Adds a tls to the list of tls to be switched by the named WAUT
    // void
    // MSTLLogicControl::addWAUTJunction(const std::string& wautid,
    //                                   const std::string& tls,
    //                                   const std::string& proc,
    //                                   bool synchron) {
    //     // try to get the waut
    //     if (myWAUTs.find(wautid) == myWAUTs.end()) {
    //         // report an error if the waut is not known
    //         throw InvalidArgument(TLF("Waut '%' was not yet defined.", wautid));
    //     }
    //     // try to get the tls to switch
    //     if (myLogics.find(tls) == myLogics.end()) {
    //         // report an error if the tls is not known
    //         throw InvalidArgument(TLF("TLS '%' to switch in WAUT '%' was not yet defined.", tls, wautid));
    //     }
    //     WAUTJunction j;
    //     j.junction = tls;
    //     j.procedure = proc;
    //     j.synchron = synchron;
    //     myWAUTs[wautid]->junctions.push_back(j);
    //
    //     std::string initProg = myWAUTs[wautid]->startProg;
    //     std::vector<WAUTSwitch>::const_iterator first = myWAUTs[wautid]->switches.end();
    //     SUMOTime minExecTime = -1;
    //     for (std::vector<WAUTSwitch>::const_iterator i = myWAUTs[wautid]->switches.begin(); i != myWAUTs[wautid]->switches.end(); ++i) {
    //         if ((*i).when > MSNet::getInstance()->getCurrentTimeStep() && (minExecTime == -1 || (*i).when < minExecTime)) {
    //             minExecTime = (*i).when;
    //             first = i;
    //         }
    //         if (first != myWAUTs[wautid]->switches.begin()) {
    //             initProg = (*(first - 1)).to;
    //         }
    //     }
    //     // activate the first one
    //     switchTo(tls, initProg);
    // }
    #[allow(unused_variables)]
    pub fn add_waut_junction(&mut self, wautid: &str, tls: &str, proc: &str, synchron: bool) {
        // try to get the waut
        if !self.myWAUTs.contains_key(wautid) {
            panic!("{}", utils::tlf!("Waut '%' was not yet defined.", wautid));
        }
        // try to get the tls to switch
        if !self.myLogics.contains_key(tls) {
            panic!("{}", utils::tlf!("TLS '%' to switch in WAUT '%' was not yet defined.", tls, wautid));
        }
        let waut = self.myWAUTs[wautid];
        waut.m().junctions.push(MSTLLogicControl_WAUTJunction {
            junction: tls.to_string(),
            procedure: proc.to_string(),
            synchron,
        });
        let mut initProg = waut.startProg.clone();
        let now = MSNet::get_instance().get_current_time_step();
        let len = waut.switches.len();
        // first == len stands for switches.end()
        let mut first = len;
        let mut minExecTime: SUMOTime = -1;
        for i in 0..len {
            let when = waut.switches[i].when;
            if when > now && (minExecTime == -1 || when < minExecTime) {
                minExecTime = when;
                first = i;
            }
            if first != 0 {
                initProg = waut.switches[first - 1].to.clone();
            }
        }
        // activate the first one
        self.switch_to(tls, &initProg);
    }

    /// C++: `void MSTLLogicControl::closeWAUT(const std::string &)`
    /// Closes loading of a WAUT
    // void
    // MSTLLogicControl::closeWAUT(const std::string& wautid) {
    //     // try to get the waut
    //     if (myWAUTs.find(wautid) == myWAUTs.end()) {
    //         // report an error if the waut is not known
    //         throw InvalidArgument(TLF("Waut '%' was not yet defined.", wautid));
    //     }
    //     WAUT* w = myWAUTs.find(wautid)->second;
    //     std::string initProg = myWAUTs[wautid]->startProg;
    //     // get the switch to be performed as first
    //     SUMOTime minExecTime = SUMOTime_MAX;
    //     int firstIndex = -1;
    //     int i = 0;
    //     for (const WAUTSwitch& s : w->switches) {
    //         if (s.when >= SIMSTEP && s.when < minExecTime) {
    //             minExecTime = s.when;
    //             firstIndex = i;
    //         }
    //         i++;
    //     }
    //     // activate the first one
    //     if (firstIndex >= 0) {
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(
    //                 new SwitchInitCommand(*this, wautid, firstIndex), MAX2(SIMSTEP, minExecTime));
    //     }
    //     /*
    //     // set the current program to all junctions
    //     for(std::vector<WAUTJunction>::const_iterator i=w->junctions.begin(); i!=w->junctions.end(); ++i) {
    //         switchTo((*i).junction, initProg);
    //     }
    //     */
    // }
    #[allow(unused_variables)]
    pub fn close_waut(&mut self, wautid: &str) {
        // try to get the waut
        if !self.myWAUTs.contains_key(wautid) {
            panic!("{}", utils::tlf!("Waut '%' was not yet defined.", wautid));
        }
        let w = self.myWAUTs[wautid];
        let now = MSNet::get_instance().get_current_time_step();
        // get the switch to be performed as first
        let mut minExecTime: SUMOTime = SUMOTIME_MAX;
        let mut firstIndex: i32 = -1;
        for (i, s) in w.switches.iter().enumerate() {
            if s.when >= now && s.when < minExecTime {
                minExecTime = s.when;
                firstIndex = i as i32;
            }
        }
        // activate the first one
        if firstIndex >= 0 {
            let cmd = Box::into_raw(Box::new(MSTLLogicControl_SwitchInitCommand::new(self, wautid, firstIndex)));
            MSNet::get_instance()
                .m()
                .get_begin_of_timestep_events()
                .m()
                .add_event(Ptr::from_raw(cmd as *mut dyn crate::microsim::ms_event_control::Command), std::cmp::max(now, minExecTime));
        }
    }

    /// C++: `void MSTLLogicControl::check2Switch(SUMOTime)`
    /// Checks whether any WAUT is trying to switch a tls into another program
    // void
    // MSTLLogicControl::check2Switch(SUMOTime step) {
    //     for (std::vector<WAUTSwitchProcess>::iterator i = myCurrentlySwitched.begin(); i != myCurrentlySwitched.end();) {
    //         const WAUTSwitchProcess& proc = *i;
    //         if (proc.proc->trySwitch(step)) {
    //             delete proc.proc;
    //             // do not switch away from TraCI control
    //             if (getActive(proc.to->getID())->getProgramID() != TRACI_PROGRAM) {
    //                 switchTo(proc.to->getID(), proc.to->getProgramID());
    //             }
    //             i = myCurrentlySwitched.erase(i);
    //         } else {
    //             ++i;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check2_switch(&mut self, step: SUMOTime) {
        let mut i = 0;
        while i < self.myCurrentlySwitched.len() {
            if self.myCurrentlySwitched[i].proc.try_switch(step) {
                let to = self.myCurrentlySwitched[i].to;
                let toId = to.get_id().to_string();
                // do not switch away from TraCI control
                if self.get_active(&toId).get_program_id() != TRACI_PROGRAM {
                    let toProgramId = to.get_program_id().to_string();
                    self.switch_to(&toId, &toProgramId);
                }
                self.myCurrentlySwitched.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// C++: `std::pair<SUMOTime, MSPhaseDefinition> MSTLLogicControl::getPhaseDef(const std::string &) const` [const]
    /// return the complete phase definition for a named traffic lights logic
    // std::pair<SUMOTime, MSPhaseDefinition>
    // MSTLLogicControl::getPhaseDef(const std::string& tlid) const {
    //     MSTrafficLightLogic* tl = getActive(tlid);
    //     return std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), tl->getCurrentPhaseDef());
    // }
    #[allow(unused_variables)]
    pub fn get_phase_def(&self, tlid: &str) -> (SUMOTime, MSPhaseDefinition) {
        let tl = self.get_active(tlid);
        (MSNet::get_instance().get_current_time_step(), clone_phase_def(tl.get_current_phase_def()))
    }

    /// C++: `void MSTLLogicControl::switchOffAll()`
    /// switch all logic variants to 'off'
    // void
    // MSTLLogicControl::switchOffAll() {
    //     for (const auto& logic : myLogics) {
    //         if (logic.second->getActive()->getLogicType() == TrafficLightType::RAIL_SIGNAL) {
    //             // there is no sensible fall-back behavior when switching of rail
    //             // signals so they should ignore tls.all-off
    //             continue;
    //         }
    //         logic.second->addLogic("off", new MSOffTrafficLightLogic(*this, logic.first), true, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn switch_off_all(&mut self) {
        let entries: Vec<(String, Ptr<MSTLLogicControl_TLSLogicVariants>)> =
            self.myLogics.iter().map(|(id, v)| (id.clone(), *v)).collect();
        for (id, variants) in entries {
            if matches!(variants.get_active().get_logic_type(), TrafficLightType::RAIL_SIGNAL) {
                // there is no sensible fall-back behavior when switching of rail
                // signals so they should ignore tls.all-off
                continue;
            }
            let off: Ptr<MSTrafficLightLogic> = Ptr::from_raw(Box::into_raw(Box::new(MSOffTrafficLightLogic::new(self, &id))) as *mut MSTrafficLightLogic);
            variants.m().add_logic("off", off, true, true);
        }
    }

    /// C++: `void MSTLLogicControl::saveState(OutputDevice &)`
    /// Saves the current tls states into the given stream
    // void
    // MSTLLogicControl::saveState(OutputDevice& out) {
    //     MSRailSignalConstraint::saveState(out); // always saves vehicle tracker states
    //     for (const auto& logic : myLogics) {
    //         logic.second->saveState(out);
    //     }
    //     MSDriveWay::saveState(out);
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        MSRailSignalConstraint::save_state(out); // always saves vehicle tracker states
        for logic in self.myLogics.values() {
            logic.m().save_state(out);
        }
        MSDriveWay::save_state(out);
    }

    /// C++: `void MSTLLogicControl::clearState(SUMOTime, bool)`
    /// Clear all tls states before quick-loading state
    // void
    // MSTLLogicControl::clearState(SUMOTime time, bool quickReload) {
    //     MSRailSignalConstraint::clearState();
    //     if (quickReload) {
    //         for (const auto& variants : myLogics) {
    //             for (auto& logic : variants.second->getAllLogics()) {
    //                 if (logic->getLogicType() == TrafficLightType::OFF
    //                         || logic->getLogicType() == TrafficLightType::RAIL_SIGNAL
    //                         || logic->getLogicType() == TrafficLightType::RAIL_CROSSING) {
    //                     continue;
    //                 }
    //                 int step = 0;
    //                 const SUMOTime cycleTime = logic->getDefaultCycleTime();
    //                 auto& phases = logic->getPhases();
    //                 SUMOTime offset = logic->getOffset();
    //                 if (offset >= 0) {
    //                     offset = (time + cycleTime - (offset % cycleTime)) % cycleTime;
    //                 } else {
    //                     offset = (time + ((-offset) % cycleTime)) % cycleTime;
    //                 }
    //
    //                 while (offset >= phases[step]->duration) {
    //                     offset -= phases[step]->duration;
    //                     step++;
    //                 }
    //                 SUMOTime nextSwitch = time + phases[step]->duration - offset;
    //                 SUMOTime timeInCycle = (time - offset) % logic->getDefaultCycleTime();
    //                 logic->loadState(*this, time, step, offset, nextSwitch, timeInCycle, logic->isActive());
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, time: SUMOTime, quickReload: bool) {
        MSRailSignalConstraint::clear_state();
        if quickReload {
            let variants_list: Vec<Ptr<MSTLLogicControl_TLSLogicVariants>> = self.myLogics.values().copied().collect();
            for variants in variants_list {
                for logic in variants.get_all_logics() {
                    if matches!(
                        logic.get_logic_type(),
                        TrafficLightType::OFF | TrafficLightType::RAIL_SIGNAL | TrafficLightType::RAIL_CROSSING
                    ) {
                        continue;
                    }
                    let mut step: i32 = 0;
                    let cycleTime = logic.get_default_cycle_time();
                    let mut offset = logic.get_offset();
                    if offset >= 0 {
                        offset = (time + cycleTime - (offset % cycleTime)) % cycleTime;
                    } else {
                        offset = (time + ((-offset) % cycleTime)) % cycleTime;
                    }
                    while offset >= logic.get_phases()[step as usize].duration {
                        offset -= logic.get_phases()[step as usize].duration;
                        step += 1;
                    }
                    let nextSwitch = time + logic.get_phases()[step as usize].duration - offset;
                    let timeInCycle = (time - offset) % logic.get_default_cycle_time();
                    let active = logic.is_active();
                    logic.m().load_state(self, time, step, offset, nextSwitch, timeInCycle, active);
                }
            }
        }
    }

    /// C++: `SUMOTime MSTLLogicControl::initWautSwitch(SwitchInitCommand &)`
    /// Initialises switching a WAUT
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn init_waut_switch(&mut self, cmd: &mut MSTLLogicControl_SwitchInitCommand) -> SUMOTime {
        let wautid = cmd.get_wautid().to_string();
        let index = cmd.get_index();
        let waut = self.myWAUTs[&wautid];
        // copy of the switch, as in C++
        let sTo = waut.switches[*index as usize].to.clone();
        for i in 0..waut.junctions.len() {
            let junction = waut.junctions[i].junction.clone();
            let procedure = waut.junctions[i].procedure.clone();
            let synchron = waut.junctions[i].synchron;
            // get the current program and the one to instantiate
            let vars = self.myLogics[&junction];
            let from = vars.get_active();
            let to = vars.m().get_logic_instantiating_off(self, &sTo);
            let proc = match procedure.as_str() {
                "GSP" => MSTLLogicControl_WAUTSwitchProcedureVariant::GSP(MSTLLogicControl_WAUTSwitchProcedure_GSP::new(
                    self,
                    waut.m(),
                    from,
                    to,
                    synchron,
                )),
                "Stretch" => MSTLLogicControl_WAUTSwitchProcedureVariant::Stretch(MSTLLogicControl_WAUTSwitchProcedure_Stretch::new(
                    self,
                    waut.m(),
                    from,
                    to,
                    synchron,
                )),
                _ => MSTLLogicControl_WAUTSwitchProcedureVariant::JustSwitch(MSTLLogicControl_WAUTSwitchProcedure_JustSwitch::new(
                    self,
                    waut.m(),
                    from,
                    to,
                    synchron,
                )),
            };
            self.myCurrentlySwitched.push(MSTLLogicControl_WAUTSwitchProcess {
                junction,
                from,
                to,
                proc,
            });
        }
        *index += 1;
        if *index == waut.switches.len() as i32 {
            if waut.period <= 0 {
                return 0;
            } else {
                *index = 0; // start over
                let period = waut.period;
                for ws in waut.m().switches.iter_mut() {
                    ws.when += period;
                }
            }
        }
        let idx = *index as usize;
        waut.switches[idx].when - MSNet::get_instance().get_current_time_step()
    }

    /// C++: `void MSTLLogicControl::MSTLLogicControl(const MSTLLogicControl &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl) -> Self {
        panic!("MSTLLogicControl copy constructor is private and invalidated in C++")
    }

    /// C++: `MSTLLogicControl & MSTLLogicControl::operator=(const MSTLLogicControl &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl) -> &mut MSTLLogicControl {
        panic!("MSTLLogicControl assignment operator is private and invalidated in C++")
    }

}

/// Base class for things to execute if a tls switches to a new phase
/// C++ class `MSTLLogicControl::OnSwitchAction` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_OnSwitchAction {
}

impl MSTLLogicControl_OnSwitchAction {
    /// C++: `void OnSwitchAction::~OnSwitchAction() noexcept` [virtual]
    /// Destructor.
    // virtual ~OnSwitchAction() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ virtual destructor has an empty body
    }

    /// C++: `void OnSwitchAction::execute()` [virtual, pure virtual]
    /// Executes the action
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn execute(&mut self) {
        panic!("OnSwitchAction::execute is pure virtual; call it on a concrete action")
    }

}

/// A definition of a stretch - Bereich
/// C++ class `WAUTSwitchProcedure_Stretch::StretchRange` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct WAUTSwitchProcedure_Stretch_StretchRange {
    /// The begin of a stretch/cut area
    pub begin: SUMOTime,
    /// The end of a stretch/cut area
    pub end: SUMOTime,
    /// The weight factor of a stretch/cut area
    pub fac: f64,
}

impl WAUTSwitchProcedure_Stretch_StretchRange {
}

/// This event-class is used to initialise a WAUT switch at a certain time.
/// C++ class `MSTLLogicControl::SwitchInitCommand` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_SwitchInitCommand {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    /// The control to call
    pub myParent: Ptr<MSTLLogicControl>,
    /// The id of the WAUT that shall switch
    pub myWAUTID: String,
    /// The current index within the WAUT switch table
    pub myIndex: i32,
}

impl crate::microsim::ms_event_control::Command for MSTLLogicControl_SwitchInitCommand {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSTLLogicControl_SwitchInitCommand::execute(self, currentTime)
    }
}

impl std::ops::Deref for MSTLLogicControl_SwitchInitCommand {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSTLLogicControl_SwitchInitCommand {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSTLLogicControl_SwitchInitCommand {
    /// C++: `void SwitchInitCommand::SwitchInitCommand(MSTLLogicControl &, const std::string &, int)`
    /// Constructor
    // SwitchInitCommand(MSTLLogicControl& p, const std::string& wautid, int index)
    //             : myParent(p), myWAUTID(wautid), myIndex(index) { }
    #[allow(unused_variables)]
    pub fn new(p: &mut MSTLLogicControl, wautid: &str, index: i32) -> Self {
        Self {
            base: (),
            myParent: Ptr::from_mut(p),
            myWAUTID: wautid.to_string(),
            myIndex: index,
        }
    }

    /// C++: `void SwitchInitCommand::~SwitchInitCommand() noexcept` [virtual]
    /// Destructor
    // ~SwitchInitCommand() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `SUMOTime SwitchInitCommand::execute(SUMOTime)` [virtual]
    /// Begins a WAUT switch by executing the command.
    // SUMOTime execute(SUMOTime) {
    //             return myParent.initWautSwitch(*this);
    //         }
    #[allow(unused_variables)]
    pub fn execute(&mut self, arg0: SUMOTime) -> SUMOTime {
        let mut parent = self.myParent;
        parent.m().init_waut_switch(self)
    }

    /// C++: `const std::string & SwitchInitCommand::getWAUTID() const` [const]
    /// Returns the WAUT-id
    // const std::string& getWAUTID() const {
    //             return myWAUTID;
    //         }
    #[allow(unused_variables)]
    pub fn get_wautid(&self) -> &str {
        &self.myWAUTID
    }

    /// C++: `int & SwitchInitCommand::getIndex()`
    /// Returns a reference to the index
    // int& getIndex() {
    //             return myIndex;
    //         }
    #[allow(unused_variables)]
    pub fn get_index(&mut self) -> &mut i32 {
        &mut self.myIndex
    }

    /// C++: `void SwitchInitCommand::SwitchInitCommand(const SwitchInitCommand &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl_SwitchInitCommand) -> Self {
        panic!("MSTLLogicControl_SwitchInitCommand copy constructor is private and invalidated in C++")
    }

    /// C++: `SwitchInitCommand & SwitchInitCommand::operator=(const SwitchInitCommand &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl_SwitchInitCommand) -> &mut MSTLLogicControl_SwitchInitCommand {
        panic!("MSTLLogicControl_SwitchInitCommand assignment operator is private and invalidated in C++")
    }

}

/// Storage for all programs of a single tls
/// C++ class `MSTLLogicControl::TLSLogicVariants` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_TLSLogicVariants {
    /// The currently used program
    pub myCurrentProgram: Ptr<MSTrafficLightLogic>,
    /// The program that would be used in the absence of TraCI
    pub myDefaultProgram: Ptr<MSTrafficLightLogic>,
    /// A map of subkeys to programs
    pub myVariants: BTreeMap<String, Ptr<MSTrafficLightLogic>>,
    /// Originally loaded link states
    pub myOriginalLinkStates: BTreeMap<Ptr<MSLink>, LinkState>,
    /// The list of actions/commands to execute on switch
    pub mySwitchActions: Vec<Ptr<MSTLLogicControl_OnSwitchAction>>,
}

impl MSTLLogicControl_TLSLogicVariants {
    /// C++: `void TLSLogicVariants::TLSLogicVariants()`
    /// Constructor
    // MSTLLogicControl::TLSLogicVariants::TLSLogicVariants() :
    //     myCurrentProgram(nullptr),
    //     myDefaultProgram(nullptr) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            myCurrentProgram: Ptr::null(),
            myDefaultProgram: Ptr::null(),
            myVariants: BTreeMap::new(),
            myOriginalLinkStates: BTreeMap::new(),
            mySwitchActions: Vec::new(),
        }
    }

    /// C++: `void TLSLogicVariants::~TLSLogicVariants()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: owned programs and switch actions are only forgotten, not freed (their subclasses cannot be dropped through the base pointer), upgrade when logic ownership is ported
        self.myVariants.clear();
        self.mySwitchActions.clear();
    }

    /// C++: `bool TLSLogicVariants::checkOriginalTLS() const` [const]
    /// Verifies traffic lights loaded from the network
    // bool
    // MSTLLogicControl::TLSLogicVariants::checkOriginalTLS() const {
    //     bool hadErrors = false;
    //     for (std::map<std::string, MSTrafficLightLogic*>::const_iterator j = myVariants.begin(); j != myVariants.end(); ++j) {
    //         const MSTrafficLightLogic::Phases& phases = (*j).second->getPhases();
    //         int linkNo = (int)(*j).second->getLinks().size();
    //         bool hadProgramErrors = false;
    //         for (MSTrafficLightLogic::Phases::const_iterator i = phases.begin(); i != phases.end(); ++i) {
    //             if ((int)(*i)->getState().length() < linkNo) {
    //                 hadProgramErrors = true;
    //             }
    //         }
    //         if (hadProgramErrors) {
    //             WRITE_ERRORF(TL("Mismatching phase size in tls '%', program '%'."), (*j).second->getID(), (*j).first);
    //             hadErrors = true;
    //         }
    //     }
    //     return !hadErrors;
    // }
    #[allow(unused_variables)]
    pub fn check_original_tls(&self) -> bool {
        let mut had_errors = false;
        for (id, logic) in self.myVariants.iter() {
            let phases = logic.get_phases();
            let link_no = logic.get_links().len();
            let mut had_program_errors = false;
            for phase in phases.iter() {
                if phase.get_state().len() < link_no {
                    had_program_errors = true;
                }
            }
            if had_program_errors {
                utils::write_errorf!("Mismatching phase size in tls '%', program '%'.", logic.get_id(), id);
                had_errors = true;
            }
        }
        !had_errors
    }

    /// C++: `bool TLSLogicVariants::addLogic(const std::string &, MSTrafficLightLogic *, bool, bool)`
    /// Adds a logic (program). In case of an error the logic gets deleted.
    // bool
    // MSTLLogicControl::TLSLogicVariants::addLogic(const std::string& programID,
    //         MSTrafficLightLogic* logic, bool netWasLoaded, bool isNewDefault) {
    //     if (myVariants.find(programID) != myVariants.end()) {
    //         delete logic;
    //         return false;
    //     }
    //     // assert the links are set
    //     if (netWasLoaded) {
    //         // this one has not yet its links set
    //         if (myCurrentProgram == nullptr) {
    //             const std::string id = logic->getID();
    //             delete logic;
    //             throw ProcessError(TLF("No initial signal plan loaded for tls '%'.", id));
    //         }
    //         logic->adaptLinkInformationFrom(*myCurrentProgram);
    //         if (logic->getLinks().size() > logic->getPhase(0).getState().size()) {
    //             const std::string id = logic->getID();
    //             delete logic;
    //             throw ProcessError(TLF("Mismatching phase size in tls '%', program '%'.", id, programID));
    //         }
    //     }
    //     // add to the list of active
    //     if (myVariants.size() == 0 || isNewDefault) {
    //         if (myCurrentProgram != nullptr) {
    //             myCurrentProgram->deactivateProgram();
    //         }
    //         myCurrentProgram = logic;
    //         myCurrentProgram->activateProgram();
    //         if (myVariants.size() == 0) {
    //             myDefaultProgram = logic;
    //         }
    //     }
    //     // add to the list of logic
    //     myVariants[programID] = logic;
    //     if (myVariants.size() == 1 || isNewDefault) {
    //         logic->setTrafficLightSignals(MSNet::getInstance()->getCurrentTimeStep());
    //         executeOnSwitchActions();
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_logic(&mut self, programID: &str, logic: Ptr<MSTrafficLightLogic>, netWasLoaded: bool, isNewDefault: bool) -> bool {
        if self.myVariants.contains_key(programID) {
            // shortcut: a rejected logic is leaked, not deleted (its subclass cannot be freed through the base pointer), upgrade when logic ownership is ported
            return false;
        }
        // assert the links are set
        if netWasLoaded {
            // this one has not yet its links set
            if self.myCurrentProgram.is_null() {
                let id = logic.get_id().to_string();
                panic!("{}", utils::tlf!("No initial signal plan loaded for tls '%'.", id));
            }
            logic.m().adapt_link_information_from(&*self.myCurrentProgram);
            if logic.get_links().len() > logic.get_phase(0).get_state().len() {
                let id = logic.get_id().to_string();
                panic!("{}", utils::tlf!("Mismatching phase size in tls '%', program '%'.", id, programID));
            }
        }
        // add to the list of active
        if self.myVariants.is_empty() || isNewDefault {
            if !self.myCurrentProgram.is_null() {
                self.myCurrentProgram.m().deactivate_program();
            }
            self.myCurrentProgram = logic;
            self.myCurrentProgram.m().activate_program();
            if self.myVariants.is_empty() {
                self.myDefaultProgram = logic;
            }
        }
        // add to the list of logic
        self.myVariants.insert(programID.to_string(), logic);
        if self.myVariants.len() == 1 || isNewDefault {
            logic.m().set_traffic_light_signals(MSNet::get_instance().get_current_time_step());
            self.execute_on_switch_actions();
        }
        true
    }

    /// C++: `MSTrafficLightLogic * TLSLogicVariants::getLogic(const std::string &) const` [const]
    // MSTrafficLightLogic*
    // MSTLLogicControl::TLSLogicVariants::getLogic(const std::string& programID) const {
    //     if (myVariants.find(programID) == myVariants.end()) {
    //         return nullptr;
    //     }
    //     return myVariants.find(programID)->second;
    // }
    #[allow(unused_variables)]
    pub fn get_logic(&self, programID: &str) -> Ptr<MSTrafficLightLogic> {
        self.myVariants.get(programID).copied().unwrap_or(Ptr::null())
    }

    /// C++: `void TLSLogicVariants::addSwitchCommand(OnSwitchAction *)`
    // void
    // MSTLLogicControl::TLSLogicVariants::addSwitchCommand(OnSwitchAction* c) {
    //     mySwitchActions.push_back(c);
    // }
    #[allow(unused_variables)]
    pub fn add_switch_command(&mut self, c: Ptr<MSTLLogicControl_OnSwitchAction>) {
        self.mySwitchActions.push(c);
    }

    /// C++: `std::vector<MSTrafficLightLogic *> TLSLogicVariants::getAllLogics() const` [const]
    // std::vector<MSTrafficLightLogic*>
    // MSTLLogicControl::TLSLogicVariants::getAllLogics() const {
    //     std::vector<MSTrafficLightLogic*> ret;
    //     std::map<std::string, MSTrafficLightLogic*>::const_iterator i;
    //     for (i = myVariants.begin(); i != myVariants.end(); ++i) {
    //         ret.push_back((*i).second);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_all_logics(&self) -> Vec<Ptr<MSTrafficLightLogic>> {
        self.myVariants.values().copied().collect()
    }

    /// C++: `void TLSLogicVariants::saveInitialStates()`
    // void
    // MSTLLogicControl::TLSLogicVariants::saveInitialStates() {
    //     myOriginalLinkStates = myCurrentProgram->collectLinkStates();
    // }
    #[allow(unused_variables)]
    pub fn save_initial_states(&mut self) {
        self.myOriginalLinkStates = self.myCurrentProgram.collect_link_states();
    }

    /// C++: `void TLSLogicVariants::saveState(OutputDevice &)`
    // void
    // MSTLLogicControl::TLSLogicVariants::saveState(OutputDevice& out) {
    //     for (const auto& item : myVariants) {
    //         item.second->saveState(out);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        for item in self.myVariants.values() {
            item.m().save_state(out);
        }
    }

    /// C++: `bool TLSLogicVariants::isActive(const MSTrafficLightLogic *) const` [const]
    // bool
    // MSTLLogicControl::TLSLogicVariants::isActive(const MSTrafficLightLogic* tl) const {
    //     return tl == myCurrentProgram;
    // }
    #[allow(unused_variables)]
    pub fn is_active(&self, tl: Ptr<MSTrafficLightLogic>) -> bool {
        tl == self.myCurrentProgram
    }

    /// C++: `MSTrafficLightLogic * TLSLogicVariants::getActive() const` [const]
    // MSTrafficLightLogic*
    // MSTLLogicControl::TLSLogicVariants::getActive() const {
    //     return myCurrentProgram;
    // }
    #[allow(unused_variables)]
    pub fn get_active(&self) -> Ptr<MSTrafficLightLogic> {
        self.myCurrentProgram
    }

    /// C++: `MSTrafficLightLogic * TLSLogicVariants::getDefault() const` [const]
    /// return the default program (that last used program except TRACI_PROGRAM)
    // MSTrafficLightLogic*
    // MSTLLogicControl::TLSLogicVariants::getDefault() const {
    //     return myDefaultProgram;
    // }
    #[allow(unused_variables)]
    pub fn get_default(&self) -> Ptr<MSTrafficLightLogic> {
        self.myDefaultProgram
    }

    /// C++: `void TLSLogicVariants::switchTo(MSTLLogicControl &, const std::string &)`
    // void
    // MSTLLogicControl::TLSLogicVariants::switchTo(MSTLLogicControl& tlc, const std::string& programID) {
    //     // set the found wished sub-program as this tls' current one
    //     const std::string state = myCurrentProgram->getCurrentPhaseDef().getState();
    //     myCurrentProgram->deactivateProgram();
    //     myCurrentProgram = getLogicInstantiatingOff(tlc, programID);
    //     myCurrentProgram->activateProgram();
    //     myCurrentProgram->setTrafficLightSignals(MSNet::getInstance()->getCurrentTimeStep());
    //     if (state != myCurrentProgram->getCurrentPhaseDef().getState()) {
    //         myCurrentProgram->resetLastSwitch(SIMSTEP);
    //     };
    //     executeOnSwitchActions();
    // }
    #[allow(unused_variables)]
    pub fn switch_to(&mut self, tlc: &mut MSTLLogicControl, programID: &str) {
        // set the found wished sub-program as this tls' current one
        let state = self.myCurrentProgram.get_current_phase_def().get_state().to_string();
        self.myCurrentProgram.m().deactivate_program();
        self.myCurrentProgram = self.get_logic_instantiating_off(tlc, programID);
        self.myCurrentProgram.m().activate_program();
        self.myCurrentProgram.m().set_traffic_light_signals(MSNet::get_instance().get_current_time_step());
        if state.as_str() != self.myCurrentProgram.get_current_phase_def().get_state() {
            self.myCurrentProgram.m().reset_last_switch(MSNet::get_instance().get_current_time_step());
        }
        self.execute_on_switch_actions();
    }

    /// C++: `void TLSLogicVariants::switchToLoaded(MSTrafficLightLogic *)`
    /// set current program upon loading saved state
    // void
    // MSTLLogicControl::TLSLogicVariants::switchToLoaded(MSTrafficLightLogic* tl) {
    //     // setting tl as active and updating signal states happens on the calling side (MSTrafficLightLogic::loadState)
    //     if (myCurrentProgram != tl) {
    //         myCurrentProgram->deactivateProgram();
    //         myCurrentProgram = tl;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn switch_to_loaded(&mut self, tl: Ptr<MSTrafficLightLogic>) {
        // setting tl as active and updating signal states happens on the calling side (MSTrafficLightLogic::loadState)
        if self.myCurrentProgram != tl {
            self.myCurrentProgram.m().deactivate_program();
            self.myCurrentProgram = tl;
        }
    }

    /// C++: `MSTrafficLightLogic * TLSLogicVariants::getLogicInstantiatingOff(MSTLLogicControl &, const std::string &)`
    // MSTrafficLightLogic*
    // MSTLLogicControl::TLSLogicVariants::getLogicInstantiatingOff(MSTLLogicControl& tlc, const std::string& programID) {
    //     if (myVariants.find(programID) == myVariants.end()) {
    //         if (programID == "off") {
    //             // build an off-tll if this switch indicates it
    //             MSTrafficLightLogic* tlLogic = new MSOffTrafficLightLogic(tlc, myCurrentProgram->getID());
    //             if (!addLogic("off", tlLogic, true, true)) {
    //                 // inform the user if this fails
    //                 throw ProcessError(TLF("Could not build an off-state for tls '%'.", myCurrentProgram->getID()));
    //             }
    //         } else {
    //             // inform the user about a missing logic
    //             throw ProcessError(TLF("Can not switch tls '%' to program '%';\n The program is not known.", myCurrentProgram->getID(), programID));
    //         }
    //     }
    //     return getLogic(programID);
    // }
    #[allow(unused_variables)]
    pub fn get_logic_instantiating_off(&mut self, tlc: &mut MSTLLogicControl, programID: &str) -> Ptr<MSTrafficLightLogic> {
        if !self.myVariants.contains_key(programID) {
            if programID == "off" {
                // build an off-tll if this switch indicates it
                let off: Ptr<MSTrafficLightLogic> = Ptr::from_raw(Box::into_raw(Box::new(MSOffTrafficLightLogic::new(
                    tlc,
                    self.myCurrentProgram.get_id(),
                ))) as *mut MSTrafficLightLogic);
                if !self.add_logic("off", off, true, true) {
                    // inform the user if this fails
                    panic!("{}", utils::tlf!("Could not build an off-state for tls '%'.", self.myCurrentProgram.get_id()));
                }
            } else {
                // inform the user about a missing logic
                panic!(
                    "{}",
                    utils::tlf!(
                        "Can not switch tls '%' to program '%';\n The program is not known.",
                        self.myCurrentProgram.get_id(),
                        programID
                    )
                );
            }
        }
        self.get_logic(programID)
    }

    /// C++: `void TLSLogicVariants::setStateInstantiatingOnline(MSTLLogicControl &, const std::string &)`
    // void
    // MSTLLogicControl::TLSLogicVariants::setStateInstantiatingOnline(MSTLLogicControl& tlc,
    //         const std::string& state) {
    //     // build only once...
    //     MSTrafficLightLogic* logic = getLogic(TRACI_PROGRAM);
    //     if (logic == nullptr) {
    //         MSPhaseDefinition* phase = new MSPhaseDefinition(SUMOTime_DAY, state);
    //         phase->earliestEnd = SUMOTime_DAY; // prevent immediate switch
    //         std::vector<MSPhaseDefinition*> phases;
    //         phases.push_back(phase);
    //         logic = new MSSimpleTrafficLightLogic(tlc, myCurrentProgram->getID(), TRACI_PROGRAM, 0, TrafficLightType::STATIC, phases, 0,
    //                                               MSNet::getInstance()->getCurrentTimeStep() + DELTA_T,
    //                                               Parameterised::Map());
    //         if (addLogic(TRACI_PROGRAM, logic, true, true)) {
    //             MSNet::getInstance()->createTLWrapper(logic);
    //         }
    //     } else {
    //         MSPhaseDefinition nphase(SUMOTime_DAY, state);
    //         *(dynamic_cast<MSSimpleTrafficLightLogic*>(logic)->getPhases()[0]) = nphase;
    //         switchTo(tlc, TRACI_PROGRAM);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_state_instantiating_online(&mut self, tlc: &mut MSTLLogicControl, state: &str) {
        // build only once...
        let logic = self.get_logic(TRACI_PROGRAM);
        if logic.is_null() {
            let mut phase = MSPhaseDefinition::new(SUMOTIME_DAY, state, "");
            phase.earliestEnd = SUMOTIME_DAY; // prevent immediate switch
            let phases: MSTrafficLightLogic_Phases = vec![Ptr::from_raw(Box::into_raw(Box::new(phase)))];
            let now = MSNet::get_instance().get_current_time_step();
            let newLogic: Ptr<MSTrafficLightLogic> = Ptr::from_raw(Box::into_raw(Box::new(MSSimpleTrafficLightLogic::new(
                tlc,
                self.myCurrentProgram.get_id(),
                TRACI_PROGRAM,
                0,
                TrafficLightType::STATIC,
                &phases,
                0,
                now + delta_t(),
                &(),
            ))) as *mut MSTrafficLightLogic);
            if self.add_logic(TRACI_PROGRAM, newLogic, true, true) {
                MSNet::get_instance().m().create_tl_wrapper(newLogic);
            }
        } else {
            let nphase = MSPhaseDefinition::new(SUMOTIME_DAY, state, "");
            let phase0 = logic.get_phases()[0];
            *phase0.m() = nphase;
            self.switch_to(tlc, TRACI_PROGRAM);
        }
    }

    /// C++: `void TLSLogicVariants::executeOnSwitchActions() const` [const]
    // void
    // MSTLLogicControl::TLSLogicVariants::executeOnSwitchActions() const {
    //     for (std::vector<OnSwitchAction*>::const_iterator i = mySwitchActions.begin(); i != mySwitchActions.end(); ++i) {
    //         (*i)->execute();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn execute_on_switch_actions(&self) {
        for action in self.mySwitchActions.iter() {
            action.m().execute();
        }
    }

    /// C++: `void TLSLogicVariants::addLink(MSLink *, MSLane *, int)`
    // void
    // MSTLLogicControl::TLSLogicVariants::addLink(MSLink* link, MSLane* lane, int pos) {
    //     for (std::map<std::string, MSTrafficLightLogic*>::iterator i = myVariants.begin(); i != myVariants.end(); ++i) {
    //         (*i).second->addLink(link, lane, pos);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<MSLink>, lane: Ptr<MSLane>, pos: i32) {
        for logic in self.myVariants.values() {
            logic.m().add_link(link, lane, pos);
        }
    }

    /// C++: `void TLSLogicVariants::ignoreLinkIndex(int)`
    // void
    // MSTLLogicControl::TLSLogicVariants::ignoreLinkIndex(int pos) {
    //     for (std::map<std::string, MSTrafficLightLogic*>::iterator i = myVariants.begin(); i != myVariants.end(); ++i) {
    //         (*i).second->ignoreLinkIndex(pos);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn ignore_link_index(&mut self, pos: i32) {
        for logic in self.myVariants.values() {
            logic.m().ignore_link_index(pos);
        }
    }

    /// C++: `void TLSLogicVariants::TLSLogicVariants(const TLSLogicVariants &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl_TLSLogicVariants) -> Self {
        panic!("MSTLLogicControl_TLSLogicVariants copy constructor is private and invalidated in C++")
    }

    /// C++: `TLSLogicVariants & TLSLogicVariants::operator=(const TLSLogicVariants &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl_TLSLogicVariants) -> &mut MSTLLogicControl_TLSLogicVariants {
        panic!("MSTLLogicControl_TLSLogicVariants assignment operator is private and invalidated in C++")
    }

}

/// A WAUT definition
/// C++ class `MSTLLogicControl::WAUT` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUT {
    /// The id of the WAUT
    pub id: String,
    /// The name of the start program
    pub startProg: String,
    /// The reference time (offset to the switch times)
    pub refTime: SUMOTime,
    /// The period with which to repeat switches
    pub period: SUMOTime,
    /// The list of switches to be done by the WAUT
    pub switches: Vec<MSTLLogicControl_WAUTSwitch>,
    /// The list of switches assigned to the WAUT
    pub junctions: Vec<MSTLLogicControl_WAUTJunction>,
}

impl MSTLLogicControl_WAUT {
}

/// Storage for a junction assigned to a WAUT
/// C++ class `MSTLLogicControl::WAUTJunction` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUTJunction {
    /// The junction name
    pub junction: String,
    /// The procedure to switch the junction with
    pub procedure: String,
    /// Information whether this junction shall be switched synchron
    pub synchron: bool,
}

impl MSTLLogicControl_WAUTJunction {
}

/// Storage for a WAUTs switch point
/// C++ class `MSTLLogicControl::WAUTSwitch` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUTSwitch {
    /// The time the WAUT shall switch the TLS
    pub when: SUMOTime,
    /// The program name the WAUT shall switch the TLS to
    pub to: String,
}

impl MSTLLogicControl_WAUTSwitch {
}

/// This is the abstract base class for switching from one tls program to another.
/// C++ class `MSTLLogicControl::WAUTSwitchProcedure` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUTSwitchProcedure {
    /// The current program of the tls to switch
    pub myFrom: Ptr<MSTrafficLightLogic>,
    /// The program to switch the tls to
    pub myTo: Ptr<MSTrafficLightLogic>,
    /// Information whether to switch synchron (?)
    pub mySwitchSynchron: bool,
    /// The WAUT responsible for switching
    pub myWAUT: Ptr<MSTLLogicControl_WAUT>,
    /// The control the logic belongs to
    pub myControl: Ptr<MSTLLogicControl>,
}

impl MSTLLogicControl_WAUTSwitchProcedure {
    /// C++: `void WAUTSwitchProcedure::WAUTSwitchProcedure(MSTLLogicControl &, WAUT &, MSTrafficLightLogic *, MSTrafficLightLogic *, bool)`
    /// Constructor
    // WAUTSwitchProcedure(MSTLLogicControl& control, WAUT& waut,
    //                             MSTrafficLightLogic* from, MSTrafficLightLogic* to,
    //                             bool synchron)
    //             : myFrom(from), myTo(to), mySwitchSynchron(synchron), myWAUT(waut), myControl(control) { }
    #[allow(unused_variables)]
    pub fn new(control: &mut MSTLLogicControl, waut: &mut MSTLLogicControl_WAUT, from: Ptr<MSTrafficLightLogic>, to: Ptr<MSTrafficLightLogic>, synchron: bool) -> Self {
        Self {
            myFrom: from,
            myTo: to,
            mySwitchSynchron: synchron,
            myWAUT: Ptr::from_mut(waut),
            myControl: Ptr::from_mut(control),
        }
    }

    /// C++: `void WAUTSwitchProcedure::~WAUTSwitchProcedure() noexcept` [virtual]
    /// Destructor
    // virtual ~WAUTSwitchProcedure() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `bool WAUTSwitchProcedure::trySwitch(SUMOTime)` [virtual]
    /// Determines whether a switch is possible.
    // bool
    // MSTLLogicControl::WAUTSwitchProcedure::trySwitch(SUMOTime step) {
    //     // switch to the next programm if the GSP is reached
    //     if (isPosAtGSP(step, *myFrom)) {
    //         // adapt program's state
    //         if (mySwitchSynchron) {
    //             adaptLogic(step);
    //         } else {
    //             switchToPos(step, *myTo, getGSPTime(*myTo));
    //         }
    //         // switch to destination program
    //         return true;
    //     }
    //     // do not switch, yet
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self, step: SUMOTime) -> bool {
        self.try_switch_with(step, &mut |p: &mut MSTLLogicControl_WAUTSwitchProcedure, s: SUMOTime| p.adapt_logic(s))
    }

    /// C++: `bool WAUTSwitchProcedure::isPosAtGSP(SUMOTime, const MSTrafficLightLogic &)` [protected]
    /// Checks, whether the position of a signal program is at the GSP ("Good Switching Point")
    // bool
    // MSTLLogicControl::WAUTSwitchProcedure::isPosAtGSP(SUMOTime currentTime, const MSTrafficLightLogic& logic) {
    //     const SUMOTime gspTime = getGSPTime(logic) % logic.getDefaultCycleTime();
    //     const SUMOTime programTime = logic.getOffsetFromIndex(logic.getCurrentPhaseIndex()) + logic.getSpentDuration(currentTime);
    //     return gspTime == programTime;
    // }
    #[allow(unused_variables)]
    pub fn is_pos_at_gsp(&mut self, currentTime: SUMOTime, logic: &MSTrafficLightLogic) -> bool {
        // switch to the next programm if the GSP is reached
        let gspTime = self.get_gsp_time(logic) % logic.get_default_cycle_time();
        let programTime = logic.get_offset_from_index(logic.get_current_phase_index()) + logic.get_spent_duration(currentTime);
        gspTime == programTime
    }

    /// C++: `SUMOTime WAUTSwitchProcedure::getDiffToStartOfPhase(MSTrafficLightLogic &, SUMOTime)` [protected]
    /// Returns the difference between a given time and the start of the phase
    // SUMOTime
    // MSTLLogicControl::WAUTSwitchProcedure::getDiffToStartOfPhase(MSTrafficLightLogic& logic, SUMOTime toTime) {
    //     int stepOfMyPos = logic.getIndexFromOffset(toTime);
    //     SUMOTime startOfPhase = logic.getOffsetFromIndex(stepOfMyPos);
    //     assert(toTime >= startOfPhase);
    //     return toTime - startOfPhase;
    // }
    #[allow(unused_variables)]
    pub fn get_diff_to_start_of_phase(&mut self, logic: &mut MSTrafficLightLogic, toTime: SUMOTime) -> SUMOTime {
        let stepOfMyPos = logic.get_index_from_offset(toTime);
        let startOfPhase = logic.get_offset_from_index(stepOfMyPos);
        assert!(toTime >= startOfPhase);
        toTime - startOfPhase
    }

    /// C++: `void WAUTSwitchProcedure::switchToPos(SUMOTime, MSTrafficLightLogic &, SUMOTime)` [protected]
    /// switches the given logic directly to the given position
    // void
    // MSTLLogicControl::WAUTSwitchProcedure::switchToPos(SUMOTime simStep, MSTrafficLightLogic& logic, SUMOTime toTime) {
    //     int stepTo = logic.getIndexFromOffset(toTime);
    //     SUMOTime diff = getDiffToStartOfPhase(logic, toTime);
    //     const MSPhaseDefinition& phase = logic.getPhase(stepTo);
    //     SUMOTime leftDuration = phase.duration - diff;
    //     logic.changeStepAndDuration(myControl, simStep, stepTo, leftDuration);
    // }
    #[allow(unused_variables)]
    pub fn switch_to_pos(&mut self, simStep: SUMOTime, logic: &mut MSTrafficLightLogic, toTime: SUMOTime) {
        let stepTo = logic.get_index_from_offset(toTime);
        let diff = self.get_diff_to_start_of_phase(logic, toTime);
        let leftDuration = logic.get_phase(stepTo).duration - diff;
        let control = self.myControl;
        logic.change_step_and_duration(control.m(), simStep, stepTo, leftDuration);
    }

    /// C++: `SUMOTime WAUTSwitchProcedure::getGSPTime(const MSTrafficLightLogic &) const` [const, protected]
    /// Returns the GSP-value
    // SUMOTime
    // MSTLLogicControl::WAUTSwitchProcedure::getGSPTime(const MSTrafficLightLogic& logic) const {
    //     return string2time(logic.getParameter("GSP", "0"));
    // }
    #[allow(unused_variables)]
    pub fn get_gsp_time(&self, logic: &MSTrafficLightLogic) -> SUMOTime {
        string2time(&logic.base2.get_parameter("GSP", "0")).unwrap()
    }

    /// C++: `void WAUTSwitchProcedure::adaptLogic(SUMOTime)` [virtual, protected]
    /// Changes the destination program's phase to which the tls was switched
    // virtual void adaptLogic(SUMOTime step) {
    //             UNUSED_PARAMETER(step);
    //         }
    #[allow(unused_variables)]
    pub fn adapt_logic(&mut self, step: SUMOTime) {
        panic!("WAUTSwitchProcedure::adaptLogic is pure virtual; call it on a concrete switch procedure")
    }

    /// C++: `void WAUTSwitchProcedure::WAUTSwitchProcedure(const WAUTSwitchProcedure &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl_WAUTSwitchProcedure) -> Self {
        panic!("MSTLLogicControl_WAUTSwitchProcedure copy constructor is private and invalidated in C++")
    }

    /// C++: `WAUTSwitchProcedure & WAUTSwitchProcedure::operator=(const WAUTSwitchProcedure &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl_WAUTSwitchProcedure) -> &mut MSTLLogicControl_WAUTSwitchProcedure {
        panic!("MSTLLogicControl_WAUTSwitchProcedure assignment operator is private and invalidated in C++")
    }

}

/// This class switches using the GSP algorithm.
/// C++ class `MSTLLogicControl::WAUTSwitchProcedure_GSP` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUTSwitchProcedure_GSP {
    /// C++ base class `WAUTSwitchProcedure`
    pub base: MSTLLogicControl_WAUTSwitchProcedure,
}

impl std::ops::Deref for MSTLLogicControl_WAUTSwitchProcedure_GSP {
    type Target = MSTLLogicControl_WAUTSwitchProcedure;
    fn deref(&self) -> &MSTLLogicControl_WAUTSwitchProcedure {
        &self.base
    }
}
impl std::ops::DerefMut for MSTLLogicControl_WAUTSwitchProcedure_GSP {
    fn deref_mut(&mut self) -> &mut MSTLLogicControl_WAUTSwitchProcedure {
        &mut self.base
    }
}

impl MSTLLogicControl_WAUTSwitchProcedure_GSP {
    /// C++: `void WAUTSwitchProcedure_GSP::WAUTSwitchProcedure_GSP(MSTLLogicControl &, WAUT &, MSTrafficLightLogic *, MSTrafficLightLogic *, bool)`
    /// Constructor
    // MSTLLogicControl::WAUTSwitchProcedure_GSP::WAUTSwitchProcedure_GSP(
    //     MSTLLogicControl& control, WAUT& waut,
    //     MSTrafficLightLogic* from, MSTrafficLightLogic* to, bool synchron)
    //     : MSTLLogicControl::WAUTSwitchProcedure(control, waut, from, to, synchron) {}
    #[allow(unused_variables)]
    pub fn new(control: &mut MSTLLogicControl, waut: &mut MSTLLogicControl_WAUT, from: Ptr<MSTrafficLightLogic>, to: Ptr<MSTrafficLightLogic>, synchron: bool) -> Self {
        Self {
            base: MSTLLogicControl_WAUTSwitchProcedure::new(control, waut, from, to, synchron),
        }
    }

    /// C++: `void WAUTSwitchProcedure_GSP::~WAUTSwitchProcedure_GSP() noexcept` [virtual]
    /// Destructor
    // MSTLLogicControl::WAUTSwitchProcedure_GSP::~WAUTSwitchProcedure_GSP() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `void WAUTSwitchProcedure_GSP::adaptLogic(SUMOTime)` [virtual, protected]
    /// Stretches the destination program's phase to which the tls was switched
    // void
    // MSTLLogicControl::WAUTSwitchProcedure_GSP::adaptLogic(SUMOTime step) {
    //     const SUMOTime gspTo = getGSPTime(*myTo) % myTo->getDefaultCycleTime();
    //     const SUMOTime currentPosTo = myTo->getOffsetFromIndex(myTo->getCurrentPhaseIndex()) + myTo->getSpentDuration(step);
    //     SUMOTime deltaToStretch = gspTo - currentPosTo;
    //     if (deltaToStretch < 0) {
    //         deltaToStretch += myTo->getDefaultCycleTime();
    //     }
    //     const int stepTo = myTo->getIndexFromOffset(gspTo);
    //     const SUMOTime newdur = myTo->getPhase(stepTo).duration - getDiffToStartOfPhase(*myTo, gspTo) + deltaToStretch;
    //     myTo->changeStepAndDuration(myControl, step, stepTo, newdur);
    // }
    #[allow(unused_variables)]
    pub fn adapt_logic(&mut self, step: SUMOTime) {
        gsp_adapt_logic(&mut self.base, step);
    }

}

/// This class simply switches to the next program
/// C++ class `MSTLLogicControl::WAUTSwitchProcedure_JustSwitch` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUTSwitchProcedure_JustSwitch {
    /// C++ base class `WAUTSwitchProcedure`
    pub base: MSTLLogicControl_WAUTSwitchProcedure,
}

impl std::ops::Deref for MSTLLogicControl_WAUTSwitchProcedure_JustSwitch {
    type Target = MSTLLogicControl_WAUTSwitchProcedure;
    fn deref(&self) -> &MSTLLogicControl_WAUTSwitchProcedure {
        &self.base
    }
}
impl std::ops::DerefMut for MSTLLogicControl_WAUTSwitchProcedure_JustSwitch {
    fn deref_mut(&mut self) -> &mut MSTLLogicControl_WAUTSwitchProcedure {
        &mut self.base
    }
}

impl MSTLLogicControl_WAUTSwitchProcedure_JustSwitch {
    /// C++: `void WAUTSwitchProcedure_JustSwitch::WAUTSwitchProcedure_JustSwitch(MSTLLogicControl &, WAUT &, MSTrafficLightLogic *, MSTrafficLightLogic *, bool)`
    /// Constructor
    // MSTLLogicControl::WAUTSwitchProcedure_JustSwitch::WAUTSwitchProcedure_JustSwitch(
    //     MSTLLogicControl& control, WAUT& waut,
    //     MSTrafficLightLogic* from, MSTrafficLightLogic* to, bool synchron)
    //     : MSTLLogicControl::WAUTSwitchProcedure(control, waut, from, to, synchron) {}
    #[allow(unused_variables)]
    pub fn new(control: &mut MSTLLogicControl, waut: &mut MSTLLogicControl_WAUT, from: Ptr<MSTrafficLightLogic>, to: Ptr<MSTrafficLightLogic>, synchron: bool) -> Self {
        Self {
            base: MSTLLogicControl_WAUTSwitchProcedure::new(control, waut, from, to, synchron),
        }
    }

    /// C++: `void WAUTSwitchProcedure_JustSwitch::~WAUTSwitchProcedure_JustSwitch() noexcept` [virtual]
    /// Destructor
    // MSTLLogicControl::WAUTSwitchProcedure_JustSwitch::~WAUTSwitchProcedure_JustSwitch() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `bool WAUTSwitchProcedure_JustSwitch::trySwitch(SUMOTime)` [virtual]
    /// Determines whether a switch is possible.
    // bool
    // MSTLLogicControl::WAUTSwitchProcedure_JustSwitch::trySwitch(SUMOTime) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self, step: SUMOTime) -> bool {
        true
    }

}

/// This class switches using the Stretch algorithm.
/// C++ class `MSTLLogicControl::WAUTSwitchProcedure_Stretch` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUTSwitchProcedure_Stretch {
    /// C++ base class `WAUTSwitchProcedure`
    pub base: MSTLLogicControl_WAUTSwitchProcedure,
    /// the given Stretch-areas for the "to" program, this is 0-based indexed, while the input is 1-based
    pub myStretchRanges: Vec<WAUTSwitchProcedure_Stretch_StretchRange>,
}

impl std::ops::Deref for MSTLLogicControl_WAUTSwitchProcedure_Stretch {
    type Target = MSTLLogicControl_WAUTSwitchProcedure;
    fn deref(&self) -> &MSTLLogicControl_WAUTSwitchProcedure {
        &self.base
    }
}
impl std::ops::DerefMut for MSTLLogicControl_WAUTSwitchProcedure_Stretch {
    fn deref_mut(&mut self) -> &mut MSTLLogicControl_WAUTSwitchProcedure {
        &mut self.base
    }
}

impl MSTLLogicControl_WAUTSwitchProcedure_Stretch {
    /// C++: `void WAUTSwitchProcedure_Stretch::WAUTSwitchProcedure_Stretch(MSTLLogicControl &, WAUT &, MSTrafficLightLogic *, MSTrafficLightLogic *, bool)`
    /// Constructor
    // MSTLLogicControl::WAUTSwitchProcedure_Stretch::WAUTSwitchProcedure_Stretch(
    //     MSTLLogicControl& control, WAUT& waut,
    //     MSTrafficLightLogic* from, MSTrafficLightLogic* to, bool synchron)
    //     : MSTLLogicControl::WAUTSwitchProcedure(control, waut, from, to, synchron) {
    //     int idx = 1;
    //     while (myTo->hasParameter("B" + toString(idx) + ".begin")) {
    //         StretchRange def;
    //         def.begin = string2time(myTo->getParameter("B" + toString(idx) + ".begin"));
    //         def.end = string2time(myTo->getParameter("B" + toString(idx) + ".end"));
    //         def.fac = StringUtils::toDouble(myTo->getParameter("B" + toString(idx) + ".factor"));
    //         myStretchRanges.emplace_back(def);
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn new(control: &mut MSTLLogicControl, waut: &mut MSTLLogicControl_WAUT, from: Ptr<MSTrafficLightLogic>, to: Ptr<MSTrafficLightLogic>, synchron: bool) -> Self {
        let base = MSTLLogicControl_WAUTSwitchProcedure::new(control, waut, from, to, synchron);
        let mut myStretchRanges = Vec::new();
        let mut idx = 1;
        while base.myTo.base2.has_parameter(&format!("B{}.begin", idx)) {
            let def = WAUTSwitchProcedure_Stretch_StretchRange {
                begin: string2time(&base.myTo.base2.get_parameter(&format!("B{}.begin", idx), "")).unwrap(),
                end: string2time(&base.myTo.base2.get_parameter(&format!("B{}.end", idx), "")).unwrap(),
                fac: to_double(&base.myTo.base2.get_parameter(&format!("B{}.factor", idx), "")).unwrap(),
            };
            myStretchRanges.push(def);
            // shortcut: the C++ loop never increments idx (it would loop forever on B1.begin), incremented here, verify against upstream SUMO
            idx += 1;
        }
        Self {
            base,
            myStretchRanges,
        }
    }

    /// C++: `void WAUTSwitchProcedure_Stretch::~WAUTSwitchProcedure_Stretch() noexcept` [virtual]
    /// Destructor
    // MSTLLogicControl::WAUTSwitchProcedure_Stretch::~WAUTSwitchProcedure_Stretch() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `void WAUTSwitchProcedure_Stretch::adaptLogic(SUMOTime)` [virtual, protected]
    /// Determines the destination program's changes and applies them
    // void
    // MSTLLogicControl::WAUTSwitchProcedure_Stretch::adaptLogic(SUMOTime step) {
    //     SUMOTime gspTo = getGSPTime(*myTo);
    //     SUMOTime cycleTime = myTo->getDefaultCycleTime();
    //     // the position, where the logic has to be after synchronisation
    //     SUMOTime posAfterSyn = myTo->getPhaseIndexAtTime(step);
    //     // calculate the difference, that has to be equalized
    //     SUMOTime deltaToCut = 0;
    //     if (posAfterSyn < gspTo) {
    //         deltaToCut = posAfterSyn + cycleTime - gspTo;
    //     } else {
    //         deltaToCut =  posAfterSyn - gspTo;
    //     }
    //     // test, wheter cutting of the Signalplan is possible
    //     SUMOTime deltaPossible = 0;
    //     for (const StretchRange& def : myStretchRanges) {
    //         assert(def.end >= def.begin);
    //         deltaPossible += def.end - def.begin;
    //     }
    //     int stretchUmlaufAnz = (int) StringUtils::toDouble(myTo->getParameter("StretchUmlaufAnz", ""));
    //     deltaPossible = stretchUmlaufAnz * deltaPossible;
    //     if ((deltaPossible > deltaToCut) && (deltaToCut < (cycleTime / 2))) {
    //         cutLogic(step, gspTo, deltaToCut);
    //     } else {
    //         SUMOTime deltaToStretch = (cycleTime - deltaToCut) % cycleTime;
    //         stretchLogic(step, gspTo, deltaToStretch);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn adapt_logic(&mut self, step: SUMOTime) {
        stretch_adapt_logic(&mut self.base, step, &self.myStretchRanges);
    }

    /// C++: `void WAUTSwitchProcedure_Stretch::stretchLogic(SUMOTime, SUMOTime, SUMOTime)` [protected]
    /// Stretches the logic to synchronize
    // void
    // MSTLLogicControl::WAUTSwitchProcedure_Stretch::stretchLogic(SUMOTime step, SUMOTime startPos, SUMOTime allStretchTime) {
    //     int currStep = myTo->getIndexFromOffset(startPos);
    //     SUMOTime durOfPhase = myTo->getPhase(currStep).duration;
    //     SUMOTime remainingStretchTime = allStretchTime;
    //     SUMOTime StretchTimeOfPhase = 0;
    //     int stretchUmlaufAnz = (int) StringUtils::toDouble(myTo->getParameter("StretchUmlaufAnz", ""));
    //     double facSum = 0;
    //     for (const StretchRange& def : myStretchRanges) {
    //         facSum += def.fac;
    //     }
    //     facSum *= stretchUmlaufAnz;
    //
    //     //switch to startPos and stretch this phase, if there is a end of "bereich" between startpos and end of phase
    //     SUMOTime diffToStart = getDiffToStartOfPhase(*myTo, startPos);
    //     for (const StretchRange& def : myStretchRanges) {
    //         SUMOTime endOfPhase = (startPos + durOfPhase - diffToStart);
    //         if (def.end <= endOfPhase && def.end >= startPos) {
    //             double actualfac = def.fac / facSum;
    //             facSum = facSum - def.fac;
    //             StretchTimeOfPhase = TIME2STEPS(int(STEPS2TIME(remainingStretchTime) * actualfac + 0.5));
    //             remainingStretchTime = allStretchTime - StretchTimeOfPhase;
    //         }
    //     }
    //     if (facSum == 0) {
    //         WRITE_WARNINGF(TL("The computed factor sum in WAUT '%' at time '%' equals zero;\n assuming an error in WAUT definition."), myWAUT.id, toString(STEPS2TIME(step)));
    //         return;
    //     }
    //     durOfPhase = durOfPhase - diffToStart + StretchTimeOfPhase;
    //     myTo->changeStepAndDuration(myControl, step, currStep, durOfPhase);
    //
    //     currStep = (currStep + 1) % (int)myTo->getPhases().size();
    //     // stretch all other phases, if there is a "bereich"
    //     while (remainingStretchTime > 0) {
    //         for (int i = currStep; i < (int)myTo->getPhases().size() && remainingStretchTime > 0; i++) {
    //             durOfPhase = myTo->getPhase(i).duration;
    //             SUMOTime beginOfPhase = myTo->getOffsetFromIndex(i);
    //             SUMOTime endOfPhase = beginOfPhase + durOfPhase;
    //             for (const StretchRange& def : myStretchRanges) {
    //                 if ((beginOfPhase <= def.end) && (endOfPhase >= def.end)) {
    //                     double actualfac = def.fac / facSum;
    //                     StretchTimeOfPhase = TIME2STEPS(int(STEPS2TIME(remainingStretchTime) * actualfac + 0.5));
    //                     facSum -= def.fac;
    //                     durOfPhase += StretchTimeOfPhase;
    //                     remainingStretchTime -= StretchTimeOfPhase;
    //                 }
    //             }
    //             myTo->addOverridingDuration(durOfPhase);
    //         }
    //         currStep = 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn stretch_logic(&mut self, step: SUMOTime, startPos: SUMOTime, allStretchTime: SUMOTime) {
        stretch_stretch_logic(&mut self.base, &self.myStretchRanges, step, startPos, allStretchTime);
    }

    /// C++: `void WAUTSwitchProcedure_Stretch::cutLogic(SUMOTime, SUMOTime, SUMOTime)` [protected]
    /// Cuts the logic to synchronize
    // void
    // MSTLLogicControl::WAUTSwitchProcedure_Stretch::cutLogic(SUMOTime step, SUMOTime startPos, SUMOTime allCutTime) {
    //     int actStep = myTo->getIndexFromOffset(startPos);
    //     // switches to startPos and cuts this phase, if there is a "Bereich"
    //     SUMOTime toCut = 0;
    //     for (const StretchRange& def : myStretchRanges) {
    //         int stepOfBegin = myTo->getIndexFromOffset(def.begin);
    //         if (stepOfBegin == actStep) {
    //             if (def.begin < startPos) {
    //                 toCut = def.end - startPos;
    //             } else {
    //                 toCut = def.end - def.begin;
    //             }
    //             toCut = MIN2(allCutTime, toCut);
    //             allCutTime = allCutTime - toCut;
    //         }
    //     }
    //     SUMOTime remainingDur = myTo->getPhase(actStep).duration - getDiffToStartOfPhase(*myTo, startPos);
    //     SUMOTime newDur = remainingDur - toCut;
    //     myTo->changeStepAndDuration(myControl, step, actStep, newDur);
    //
    //     // changes the duration of all other phases
    //     int currStep = (actStep + 1) % (int)myTo->getPhases().size();
    //     while (allCutTime > 0) {
    //         for (int i = currStep; i < (int) myTo->getPhases().size(); i++) {
    //             SUMOTime beginOfPhase = myTo->getOffsetFromIndex(i);
    //             SUMOTime durOfPhase = myTo->getPhase(i).duration;
    //             SUMOTime endOfPhase = beginOfPhase + durOfPhase;
    //             for (const StretchRange& def : myStretchRanges) {
    //                 if ((beginOfPhase <= def.begin) && (endOfPhase >= def.end)) {
    //                     SUMOTime maxCutOfPhase = MIN2(def.end - def.begin, allCutTime);
    //                     allCutTime = allCutTime - maxCutOfPhase;
    //                     durOfPhase = durOfPhase - maxCutOfPhase;
    //                 }
    //             }
    //             myTo->addOverridingDuration(durOfPhase);
    //         }
    //         currStep = 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cut_logic(&mut self, step: SUMOTime, startPos: SUMOTime, allCutTime: SUMOTime) {
        stretch_cut_logic(&mut self.base, &self.myStretchRanges, step, startPos, allCutTime);
    }

}

/// An initialised switch process
/// C++ class `MSTLLogicControl::WAUTSwitchProcess` (microsim/traffic_lights/MSTLLogicControl.h)
pub struct MSTLLogicControl_WAUTSwitchProcess {
    /// The id of the junction to switch
    pub junction: String,
    /// The current program of the tls
    pub from: Ptr<MSTrafficLightLogic>,
    /// The program to switch the tls to
    pub to: Ptr<MSTrafficLightLogic>,
    /// The used procedure
    pub proc: MSTLLogicControl_WAUTSwitchProcedureVariant,
}

impl MSTLLogicControl_WAUTSwitchProcess {
}


// ---------------------------------------------------------------------------
// Ported helpers (no direct C++ counterpart; they implement virtual dispatch
// and the C++ copy semantics that the skeleton's types cannot express directly)
// ---------------------------------------------------------------------------

/// The concrete switch procedure of a running WAUT switch. C++ stores a
/// `WAUTSwitchProcedure*` and calls the virtual `trySwitch`; the Rust port
/// dispatches on the concrete type instead.
pub enum MSTLLogicControl_WAUTSwitchProcedureVariant {
    JustSwitch(MSTLLogicControl_WAUTSwitchProcedure_JustSwitch),
    GSP(MSTLLogicControl_WAUTSwitchProcedure_GSP),
    Stretch(MSTLLogicControl_WAUTSwitchProcedure_Stretch),
}

impl MSTLLogicControl_WAUTSwitchProcedureVariant {
    /// C++ `WAUTSwitchProcedure::trySwitch(SUMOTime)` [virtual], dispatched on the concrete type
    pub fn try_switch(&mut self, step: SUMOTime) -> bool {
        match self {
            Self::JustSwitch(p) => p.try_switch(step),
            Self::GSP(p) => p.try_switch(step),
            Self::Stretch(p) => p.try_switch(step),
        }
    }
}

impl MSTLLogicControl_WAUTSwitchProcedure {
    /// C++ `WAUTSwitchProcedure::trySwitch`; `adapt` plays the role of the virtual `adaptLogic` call
    fn try_switch_with(&mut self, step: SUMOTime, adapt: &mut dyn FnMut(&mut MSTLLogicControl_WAUTSwitchProcedure, SUMOTime)) -> bool {
        // switch to the next programm if the GSP is reached
        let from = self.myFrom;
        if self.is_pos_at_gsp(step, &*from) {
            // adapt program's state
            if self.mySwitchSynchron {
                adapt(&mut *self, step);
            } else {
                let to = self.myTo;
                let gspTime = self.get_gsp_time(&*to);
                self.switch_to_pos(step, to.m(), gspTime);
            }
            // switch to destination program
            return true;
        }
        // do not switch, yet
        false
    }
}

impl MSTLLogicControl_WAUTSwitchProcedure_GSP {
    /// C++ `WAUTSwitchProcedure_GSP` inherits `trySwitch`; the virtual `adaptLogic` is GSP's
    pub fn try_switch(&mut self, step: SUMOTime) -> bool {
        self.base.try_switch_with(step, &mut |p: &mut MSTLLogicControl_WAUTSwitchProcedure, s: SUMOTime| gsp_adapt_logic(p, s))
    }
}

impl MSTLLogicControl_WAUTSwitchProcedure_Stretch {
    /// C++ `WAUTSwitchProcedure_Stretch` inherits `trySwitch`; the virtual `adaptLogic` is Stretch's
    pub fn try_switch(&mut self, step: SUMOTime) -> bool {
        let ranges = &self.myStretchRanges;
        self.base.try_switch_with(step, &mut |p: &mut MSTLLogicControl_WAUTSwitchProcedure, s: SUMOTime| stretch_adapt_logic(p, s, ranges))
    }
}

/// C++ `WAUTSwitchProcedure_GSP::adaptLogic`
fn gsp_adapt_logic(p: &mut MSTLLogicControl_WAUTSwitchProcedure, step: SUMOTime) {
    let to = p.myTo;
    let gspTo = p.get_gsp_time(&*to) % to.get_default_cycle_time();
    let currentPosTo = to.get_offset_from_index(to.get_current_phase_index()) + to.get_spent_duration(step);
    let mut deltaToStretch = gspTo - currentPosTo;
    if deltaToStretch < 0 {
        deltaToStretch += to.get_default_cycle_time();
    }
    let stepTo = to.get_index_from_offset(gspTo);
    let newdur = to.get_phase(stepTo).duration - p.get_diff_to_start_of_phase(to.m(), gspTo) + deltaToStretch;
    to.m().change_step_and_duration(p.myControl.m(), step, stepTo, newdur);
}

/// C++ `WAUTSwitchProcedure_Stretch::adaptLogic`
fn stretch_adapt_logic(p: &mut MSTLLogicControl_WAUTSwitchProcedure, step: SUMOTime, ranges: &[WAUTSwitchProcedure_Stretch_StretchRange]) {
    let to = p.myTo;
    let gspTo = p.get_gsp_time(&*to);
    let cycleTime = to.get_default_cycle_time();
    // the position, where the logic has to be after synchronisation
    let posAfterSyn = to.get_phase_index_at_time(step);
    // calculate the difference, that has to be equalized
    let deltaToCut: SUMOTime = if posAfterSyn < gspTo {
        posAfterSyn + cycleTime - gspTo
    } else {
        posAfterSyn - gspTo
    };
    // test, wheter cutting of the Signalplan is possible
    let mut deltaPossible: SUMOTime = 0;
    for def in ranges {
        debug_assert!(def.end >= def.begin);
        deltaPossible += def.end - def.begin;
    }
    let stretchUmlaufAnz = to_double(&to.base2.get_parameter("StretchUmlaufAnz", "")).unwrap() as i32;
    deltaPossible = stretchUmlaufAnz as SUMOTime * deltaPossible;
    if deltaPossible > deltaToCut && deltaToCut < (cycleTime / 2) {
        stretch_cut_logic(p, ranges, step, gspTo, deltaToCut);
    } else {
        let deltaToStretch = (cycleTime - deltaToCut) % cycleTime;
        stretch_stretch_logic(p, ranges, step, gspTo, deltaToStretch);
    }
}

/// C++ `WAUTSwitchProcedure_Stretch::cutLogic`
fn stretch_cut_logic(p: &mut MSTLLogicControl_WAUTSwitchProcedure, ranges: &[WAUTSwitchProcedure_Stretch_StretchRange], step: SUMOTime, startPos: SUMOTime, mut allCutTime: SUMOTime) {
    let to = p.myTo;
    let actStep = to.get_index_from_offset(startPos);
    // switches to startPos and cuts this phase, if there is a "Bereich"
    let mut toCut: SUMOTime = 0;
    for def in ranges {
        let stepOfBegin = to.get_index_from_offset(def.begin);
        if stepOfBegin == actStep {
            if def.begin < startPos {
                toCut = def.end - startPos;
            } else {
                toCut = def.end - def.begin;
            }
            toCut = std::cmp::min(allCutTime, toCut);
            allCutTime = allCutTime - toCut;
        }
    }
    let remainingDur = to.get_phase(actStep).duration - p.get_diff_to_start_of_phase(to.m(), startPos);
    let newDur = remainingDur - toCut;
    to.m().change_step_and_duration(p.myControl.m(), step, actStep, newDur);

    // changes the duration of all other phases
    let mut currStep = (actStep + 1) % (to.get_phases().len() as i32);
    while allCutTime > 0 {
        for i in currStep..(to.get_phases().len() as i32) {
            let beginOfPhase = to.get_offset_from_index(i);
            let mut durOfPhase = to.get_phase(i).duration;
            let endOfPhase = beginOfPhase + durOfPhase;
            for def in ranges {
                if beginOfPhase <= def.begin && endOfPhase >= def.end {
                    let maxCutOfPhase = std::cmp::min(def.end - def.begin, allCutTime);
                    allCutTime = allCutTime - maxCutOfPhase;
                    durOfPhase = durOfPhase - maxCutOfPhase;
                }
            }
            to.m().add_overriding_duration(durOfPhase);
        }
        currStep = 0;
    }
}

/// C++ `WAUTSwitchProcedure_Stretch::stretchLogic`
fn stretch_stretch_logic(p: &mut MSTLLogicControl_WAUTSwitchProcedure, ranges: &[WAUTSwitchProcedure_Stretch_StretchRange], step: SUMOTime, startPos: SUMOTime, allStretchTime: SUMOTime) {
    let to = p.myTo;
    let mut currStep = to.get_index_from_offset(startPos);
    let mut durOfPhase = to.get_phase(currStep).duration;
    let mut remainingStretchTime = allStretchTime;
    let mut StretchTimeOfPhase: SUMOTime = 0;
    let stretchUmlaufAnz = to_double(&to.base2.get_parameter("StretchUmlaufAnz", "")).unwrap() as i32;
    let mut facSum: f64 = 0.0;
    for def in ranges {
        facSum += def.fac;
    }
    facSum *= stretchUmlaufAnz as f64;

    //switch to startPos and stretch this phase, if there is a end of "bereich" between startpos and end of phase
    let diffToStart = p.get_diff_to_start_of_phase(to.m(), startPos);
    for def in ranges {
        let endOfPhase = startPos + durOfPhase - diffToStart;
        if def.end <= endOfPhase && def.end >= startPos {
            let actualfac = def.fac / facSum;
            facSum = facSum - def.fac;
            StretchTimeOfPhase = time2steps(((steps2time(remainingStretchTime) * actualfac + 0.5) as i32) as f64);
            remainingStretchTime = allStretchTime - StretchTimeOfPhase;
        }
    }
    if facSum == 0.0 {
        utils::write_warningf!(
            "The computed factor sum in WAUT '%' at time '%' equals zero;\n assuming an error in WAUT definition.",
            p.myWAUT.id,
            to_string(&steps2time(step))
        );
        return;
    }
    durOfPhase = durOfPhase - diffToStart + StretchTimeOfPhase;
    to.m().change_step_and_duration(p.myControl.m(), step, currStep, durOfPhase);

    currStep = (currStep + 1) % (to.get_phases().len() as i32);
    // stretch all other phases, if there is a "bereich"
    while remainingStretchTime > 0 {
        let mut i = currStep;
        while i < (to.get_phases().len() as i32) && remainingStretchTime > 0 {
            durOfPhase = to.get_phase(i).duration;
            let beginOfPhase = to.get_offset_from_index(i);
            let endOfPhase = beginOfPhase + durOfPhase;
            for def in ranges {
                if beginOfPhase <= def.end && endOfPhase >= def.end {
                    let actualfac = def.fac / facSum;
                    StretchTimeOfPhase = time2steps(((steps2time(remainingStretchTime) * actualfac + 0.5) as i32) as f64);
                    facSum -= def.fac;
                    durOfPhase += StretchTimeOfPhase;
                    remainingStretchTime -= StretchTimeOfPhase;
                }
            }
            to.m().add_overriding_duration(durOfPhase);
            i += 1;
        }
        currStep = 0;
    }
}

/// Field-wise copy of an MSPhaseDefinition (C++ copies it by value; the type has no Clone derive)
fn clone_phase_def(p: &MSPhaseDefinition) -> MSPhaseDefinition {
    MSPhaseDefinition {
        duration: p.duration,
        lastDuration: p.lastDuration,
        minDuration: p.minDuration,
        maxDuration: p.maxDuration,
        earliestEnd: p.earliestEnd,
        latestEnd: p.latestEnd,
        earlyTarget: p.earlyTarget.clone(),
        finalTarget: p.finalTarget.clone(),
        myLastSwitch: p.myLastSwitch,
        myLastEnd: p.myLastEnd,
        nextPhases: p.nextPhases.clone(),
        name: p.name.clone(),
        yellow: p.yellow,
        red: p.red,
        vehext: p.vehext,
        myTransientNotDecisional: p.myTransientNotDecisional,
        myCommit: p.myCommit,
        myUndefined: p.myUndefined,
        myTargetLaneSet: p.myTargetLaneSet.clone(),
        myState: p.myState.clone(),
    }
}
