//! Port of microsim/traffic_lights/MSTLLogicControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSTLLogicControl::MSTLLogicControl")
    }

    /// C++: `void MSTLLogicControl::~MSTLLogicControl()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTLLogicControl::~MSTLLogicControl")
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
        todo!("port MSTLLogicControl::closeNetworkReading")
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
        todo!("port MSTLLogicControl::setTrafficLightSignals")
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
        todo!("port MSTLLogicControl::getAllLogics")
    }

    /// C++: `TLSLogicVariants & MSTLLogicControl::get(const std::string &) const` [const]
    /// Returns the variants of a named tls
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get(&self, id: &str) -> &mut MSTLLogicControl_TLSLogicVariants {
        todo!("port MSTLLogicControl::get")
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
        todo!("port MSTLLogicControl::get")
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
        todo!("port MSTLLogicControl::getActive")
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
        todo!("port MSTLLogicControl::getAllTLIds")
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
        todo!("port MSTLLogicControl::add")
    }

    /// C++: `bool MSTLLogicControl::knows(const std::string &) const` [const]
    /// Returns the information whether the named tls is stored
    // bool
    // MSTLLogicControl::knows(const std::string& id) const {
    //     return myLogics.count(id) != 0;
    // }
    #[allow(unused_variables)]
    pub fn knows(&self, id: &str) -> bool {
        todo!("port MSTLLogicControl::knows")
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
        todo!("port MSTLLogicControl::isActive")
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
        todo!("port MSTLLogicControl::switchTo")
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
        todo!("port MSTLLogicControl::addWAUT")
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
        todo!("port MSTLLogicControl::addWAUTSwitch")
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
        todo!("port MSTLLogicControl::addWAUTJunction")
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
        todo!("port MSTLLogicControl::closeWAUT")
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
        todo!("port MSTLLogicControl::check2Switch")
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
        todo!("port MSTLLogicControl::getPhaseDef")
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
        todo!("port MSTLLogicControl::switchOffAll")
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
        todo!("port MSTLLogicControl::saveState")
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
        todo!("port MSTLLogicControl::clearState")
    }

    /// C++: `SUMOTime MSTLLogicControl::initWautSwitch(SwitchInitCommand &)`
    /// Initialises switching a WAUT
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn init_waut_switch(&mut self, cmd: &mut MSTLLogicControl_SwitchInitCommand) -> SUMOTime {
        todo!("port MSTLLogicControl::initWautSwitch")
    }

    /// C++: `void MSTLLogicControl::MSTLLogicControl(const MSTLLogicControl &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl) -> Self {
        todo!("port MSTLLogicControl::MSTLLogicControl")
    }

    /// C++: `MSTLLogicControl & MSTLLogicControl::operator=(const MSTLLogicControl &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl) -> &mut MSTLLogicControl {
        todo!("port MSTLLogicControl::operator=")
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
        todo!("port OnSwitchAction::~OnSwitchAction")
    }

    /// C++: `void OnSwitchAction::execute()` [virtual, pure virtual]
    /// Executes the action
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn execute(&mut self) {
        todo!("port OnSwitchAction::execute")
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
        todo!("port SwitchInitCommand::SwitchInitCommand")
    }

    /// C++: `void SwitchInitCommand::~SwitchInitCommand() noexcept` [virtual]
    /// Destructor
    // ~SwitchInitCommand() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SwitchInitCommand::~SwitchInitCommand")
    }

    /// C++: `SUMOTime SwitchInitCommand::execute(SUMOTime)` [virtual]
    /// Begins a WAUT switch by executing the command.
    // SUMOTime execute(SUMOTime) {
    //             return myParent.initWautSwitch(*this);
    //         }
    #[allow(unused_variables)]
    pub fn execute(&mut self, arg0: SUMOTime) -> SUMOTime {
        todo!("port SwitchInitCommand::execute")
    }

    /// C++: `const std::string & SwitchInitCommand::getWAUTID() const` [const]
    /// Returns the WAUT-id
    // const std::string& getWAUTID() const {
    //             return myWAUTID;
    //         }
    #[allow(unused_variables)]
    pub fn get_wautid(&self) -> &str {
        todo!("port SwitchInitCommand::getWAUTID")
    }

    /// C++: `int & SwitchInitCommand::getIndex()`
    /// Returns a reference to the index
    // int& getIndex() {
    //             return myIndex;
    //         }
    #[allow(unused_variables)]
    pub fn get_index(&mut self) -> &mut i32 {
        todo!("port SwitchInitCommand::getIndex")
    }

    /// C++: `void SwitchInitCommand::SwitchInitCommand(const SwitchInitCommand &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl_SwitchInitCommand) -> Self {
        todo!("port SwitchInitCommand::SwitchInitCommand")
    }

    /// C++: `SwitchInitCommand & SwitchInitCommand::operator=(const SwitchInitCommand &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl_SwitchInitCommand) -> &mut MSTLLogicControl_SwitchInitCommand {
        todo!("port SwitchInitCommand::operator=")
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
        todo!("port TLSLogicVariants::TLSLogicVariants")
    }

    /// C++: `void TLSLogicVariants::~TLSLogicVariants()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port TLSLogicVariants::~TLSLogicVariants")
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
        todo!("port TLSLogicVariants::checkOriginalTLS")
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
        todo!("port TLSLogicVariants::addLogic")
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
        todo!("port TLSLogicVariants::getLogic")
    }

    /// C++: `void TLSLogicVariants::addSwitchCommand(OnSwitchAction *)`
    // void
    // MSTLLogicControl::TLSLogicVariants::addSwitchCommand(OnSwitchAction* c) {
    //     mySwitchActions.push_back(c);
    // }
    #[allow(unused_variables)]
    pub fn add_switch_command(&mut self, c: Ptr<MSTLLogicControl_OnSwitchAction>) {
        todo!("port TLSLogicVariants::addSwitchCommand")
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
        todo!("port TLSLogicVariants::getAllLogics")
    }

    /// C++: `void TLSLogicVariants::saveInitialStates()`
    // void
    // MSTLLogicControl::TLSLogicVariants::saveInitialStates() {
    //     myOriginalLinkStates = myCurrentProgram->collectLinkStates();
    // }
    #[allow(unused_variables)]
    pub fn save_initial_states(&mut self) {
        todo!("port TLSLogicVariants::saveInitialStates")
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
        todo!("port TLSLogicVariants::saveState")
    }

    /// C++: `bool TLSLogicVariants::isActive(const MSTrafficLightLogic *) const` [const]
    // bool
    // MSTLLogicControl::TLSLogicVariants::isActive(const MSTrafficLightLogic* tl) const {
    //     return tl == myCurrentProgram;
    // }
    #[allow(unused_variables)]
    pub fn is_active(&self, tl: Ptr<MSTrafficLightLogic>) -> bool {
        todo!("port TLSLogicVariants::isActive")
    }

    /// C++: `MSTrafficLightLogic * TLSLogicVariants::getActive() const` [const]
    // MSTrafficLightLogic*
    // MSTLLogicControl::TLSLogicVariants::getActive() const {
    //     return myCurrentProgram;
    // }
    #[allow(unused_variables)]
    pub fn get_active(&self) -> Ptr<MSTrafficLightLogic> {
        todo!("port TLSLogicVariants::getActive")
    }

    /// C++: `MSTrafficLightLogic * TLSLogicVariants::getDefault() const` [const]
    /// return the default program (that last used program except TRACI_PROGRAM)
    // MSTrafficLightLogic*
    // MSTLLogicControl::TLSLogicVariants::getDefault() const {
    //     return myDefaultProgram;
    // }
    #[allow(unused_variables)]
    pub fn get_default(&self) -> Ptr<MSTrafficLightLogic> {
        todo!("port TLSLogicVariants::getDefault")
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
        todo!("port TLSLogicVariants::switchTo")
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
        todo!("port TLSLogicVariants::switchToLoaded")
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
        todo!("port TLSLogicVariants::getLogicInstantiatingOff")
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
        todo!("port TLSLogicVariants::setStateInstantiatingOnline")
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
        todo!("port TLSLogicVariants::executeOnSwitchActions")
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
        todo!("port TLSLogicVariants::addLink")
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
        todo!("port TLSLogicVariants::ignoreLinkIndex")
    }

    /// C++: `void TLSLogicVariants::TLSLogicVariants(const TLSLogicVariants &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl_TLSLogicVariants) -> Self {
        todo!("port TLSLogicVariants::TLSLogicVariants")
    }

    /// C++: `TLSLogicVariants & TLSLogicVariants::operator=(const TLSLogicVariants &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl_TLSLogicVariants) -> &mut MSTLLogicControl_TLSLogicVariants {
        todo!("port TLSLogicVariants::operator=")
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
        todo!("port WAUTSwitchProcedure::WAUTSwitchProcedure")
    }

    /// C++: `void WAUTSwitchProcedure::~WAUTSwitchProcedure() noexcept` [virtual]
    /// Destructor
    // virtual ~WAUTSwitchProcedure() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port WAUTSwitchProcedure::~WAUTSwitchProcedure")
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
        todo!("port WAUTSwitchProcedure::trySwitch")
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
        todo!("port WAUTSwitchProcedure::isPosAtGSP")
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
        todo!("port WAUTSwitchProcedure::getDiffToStartOfPhase")
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
        todo!("port WAUTSwitchProcedure::switchToPos")
    }

    /// C++: `SUMOTime WAUTSwitchProcedure::getGSPTime(const MSTrafficLightLogic &) const` [const, protected]
    /// Returns the GSP-value
    // SUMOTime
    // MSTLLogicControl::WAUTSwitchProcedure::getGSPTime(const MSTrafficLightLogic& logic) const {
    //     return string2time(logic.getParameter("GSP", "0"));
    // }
    #[allow(unused_variables)]
    pub fn get_gsp_time(&self, logic: &MSTrafficLightLogic) -> SUMOTime {
        todo!("port WAUTSwitchProcedure::getGSPTime")
    }

    /// C++: `void WAUTSwitchProcedure::adaptLogic(SUMOTime)` [virtual, protected]
    /// Changes the destination program's phase to which the tls was switched
    // virtual void adaptLogic(SUMOTime step) {
    //             UNUSED_PARAMETER(step);
    //         }
    #[allow(unused_variables)]
    pub fn adapt_logic(&mut self, step: SUMOTime) {
        todo!("port WAUTSwitchProcedure::adaptLogic")
    }

    /// C++: `void WAUTSwitchProcedure::WAUTSwitchProcedure(const WAUTSwitchProcedure &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTLLogicControl_WAUTSwitchProcedure) -> Self {
        todo!("port WAUTSwitchProcedure::WAUTSwitchProcedure")
    }

    /// C++: `WAUTSwitchProcedure & WAUTSwitchProcedure::operator=(const WAUTSwitchProcedure &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTLLogicControl_WAUTSwitchProcedure) -> &mut MSTLLogicControl_WAUTSwitchProcedure {
        todo!("port WAUTSwitchProcedure::operator=")
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
        todo!("port WAUTSwitchProcedure_GSP::WAUTSwitchProcedure_GSP")
    }

    /// C++: `void WAUTSwitchProcedure_GSP::~WAUTSwitchProcedure_GSP() noexcept` [virtual]
    /// Destructor
    // MSTLLogicControl::WAUTSwitchProcedure_GSP::~WAUTSwitchProcedure_GSP() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port WAUTSwitchProcedure_GSP::~WAUTSwitchProcedure_GSP")
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
        todo!("port WAUTSwitchProcedure_GSP::adaptLogic")
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
        todo!("port WAUTSwitchProcedure_JustSwitch::WAUTSwitchProcedure_JustSwitch")
    }

    /// C++: `void WAUTSwitchProcedure_JustSwitch::~WAUTSwitchProcedure_JustSwitch() noexcept` [virtual]
    /// Destructor
    // MSTLLogicControl::WAUTSwitchProcedure_JustSwitch::~WAUTSwitchProcedure_JustSwitch() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port WAUTSwitchProcedure_JustSwitch::~WAUTSwitchProcedure_JustSwitch")
    }

    /// C++: `bool WAUTSwitchProcedure_JustSwitch::trySwitch(SUMOTime)` [virtual]
    /// Determines whether a switch is possible.
    // bool
    // MSTLLogicControl::WAUTSwitchProcedure_JustSwitch::trySwitch(SUMOTime) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self, step: SUMOTime) -> bool {
        todo!("port WAUTSwitchProcedure_JustSwitch::trySwitch")
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
        todo!("port WAUTSwitchProcedure_Stretch::WAUTSwitchProcedure_Stretch")
    }

    /// C++: `void WAUTSwitchProcedure_Stretch::~WAUTSwitchProcedure_Stretch() noexcept` [virtual]
    /// Destructor
    // MSTLLogicControl::WAUTSwitchProcedure_Stretch::~WAUTSwitchProcedure_Stretch() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port WAUTSwitchProcedure_Stretch::~WAUTSwitchProcedure_Stretch")
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
        todo!("port WAUTSwitchProcedure_Stretch::adaptLogic")
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
        todo!("port WAUTSwitchProcedure_Stretch::stretchLogic")
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
        todo!("port WAUTSwitchProcedure_Stretch::cutLogic")
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
    pub proc: Ptr<MSTLLogicControl_WAUTSwitchProcedure>,
}

impl MSTLLogicControl_WAUTSwitchProcess {
}

