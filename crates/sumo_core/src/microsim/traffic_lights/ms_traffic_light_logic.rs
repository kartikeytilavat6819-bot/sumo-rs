//! Port of microsim/traffic_lights/MSTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<MSLane *> MSTrafficLightLogic::LaneVector`
pub type MSTrafficLightLogic_LaneVector = Vec<Ptr<MSLane>>;

/// C++ typedef `std::vector<LaneVector> MSTrafficLightLogic::LaneVectorVector`
pub type MSTrafficLightLogic_LaneVectorVector = Vec<LaneVector>;

/// C++ typedef `std::vector<MSLink *> MSTrafficLightLogic::LinkVector`
pub type MSTrafficLightLogic_LinkVector = Vec<Ptr<MSLink>>;

/// C++ typedef `std::vector<LinkVector> MSTrafficLightLogic::LinkVectorVector`
pub type MSTrafficLightLogic_LinkVectorVector = Vec<MSTrafficLightLogic_LinkVector>;

/// C++ typedef `std::vector<MSPhaseDefinition *> MSTrafficLightLogic::Phases`
pub type MSTrafficLightLogic_Phases = Vec<Ptr<MSPhaseDefinition>>;

/// C++ typedef `std::vector<const SUMOVehicle *> MSTrafficLightLogic::VehicleVector`
pub type MSTrafficLightLogic_VehicleVector = Vec<Ptr<SUMOVehicle>>;

// C++ static member MSTrafficLightLogic::myEmptyLaneVector
const MY_EMPTY_LANE_VECTOR: &MSTrafficLightLogic_LaneVector = &Vec::new();

/// The parent class for traffic light logics
/// C++ class `MSTrafficLightLogic` (microsim/traffic_lights/MSTrafficLightLogic.h)
pub struct MSTrafficLightLogic {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// The id of the logic
    pub myProgramID: String,
    /// the offset parameter of the current program
    pub myOffset: SUMOTime,
    /// The type of the logic
    pub myLogicType: TrafficLightType,
    /// The list of LinkVectors; each vector contains the links that belong to the same link index
    pub myLinks: MSTrafficLightLogic_LinkVectorVector,
    /// The list of LaneVectors; each vector contains the incoming lanes that belong to the same link index
    pub myLanes: MSTrafficLightLogic_LaneVectorVector,
    /// number of controlled links
    pub myNumLinks: i32,
    /// A list of duration overrides
    pub myOverridingTimes: Vec<SUMOTime>,
    /// A value for enlarge the current duration
    pub myCurrentDurationIncrement: SUMOTime,
    /// The current switch command
    pub mySwitchCommand: Ptr<MSTrafficLightLogic_SwitchCommand>,
    /// The cycle time (without changes)
    pub myDefaultCycleTime: SUMOTime,
    /// list of indices that are ignored in mesoscopic simulatino
    pub myIgnoredIndices: BTreeSet<i32>,
    /// whether the current program is active
    pub myAmActive: bool,
}

impl std::ops::Deref for MSTrafficLightLogic {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MSTrafficLightLogic (port as module-level statics, see AGENTS.md):
//   static const LaneVector MSTrafficLightLogic::myEmptyLaneVector;

impl MSTrafficLightLogic {
    /// C++: `void MSTrafficLightLogic::MSTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const SUMOTime, const TrafficLightType, const SUMOTime, const Parameterised::Map &)`
    /// Constructor
    // MSTrafficLightLogic::MSTrafficLightLogic(MSTLLogicControl& tlcontrol, const std::string& id,
    //         const std::string& programID, const SUMOTime offset, const TrafficLightType logicType, const SUMOTime delay,
    //         const Parameterised::Map& parameters) :
    //     Named(id), Parameterised(parameters),
    //     myProgramID(programID),
    //     myOffset(offset),
    //     myLogicType(logicType),
    //     myCurrentDurationIncrement(-1),
    //     myDefaultCycleTime(0),
    //     myAmActive(true) {
    //     mySwitchCommand = new SwitchCommand(tlcontrol, this, delay);
    //     MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(mySwitchCommand, delay);
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, offset: SUMOTime, logicType: TrafficLightType, delay: SUMOTime, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        // shortcut: C++ parameters argument is Unmapped (unit) in this skeleton, so the Parameterised::Map is not stored; upgrade when the signature takes ParameterisedMap
        // shortcut: C++ also schedules `new SwitchCommand(tlcontrol, this, delay)` on MSNet's begin-of-timestep events here; a by-value constructor has no stable address, so callers must call schedule_switch_command on the boxed logic
        let _ = (tlcontrol, delay);
        MSTrafficLightLogic {
            base: Named::new(id),
            base2: Parameterised::new(),
            myProgramID: programID.to_string(),
            myOffset: offset,
            myLogicType: logicType,
            myLinks: Vec::new(),
            myLanes: Vec::new(),
            myNumLinks: 0,
            myOverridingTimes: Vec::new(),
            myCurrentDurationIncrement: -1,
            mySwitchCommand: Ptr::null(),
            myDefaultCycleTime: 0,
            myIgnoredIndices: BTreeSet::new(),
            myAmActive: true,
        }
    }

    /// Schedules the initial SwitchCommand (the part of the C++ constructor that needs `this`).
    /// Must be called once the logic has its final address (i.e. after it is boxed).
    // shortcut: this is the second half of the C++ constructor (new SwitchCommand(tlcontrol, this, delay) + addEvent); callers must invoke it after boxing, upgrade when constructors return Box<Self>
    pub fn schedule_switch_command(&mut self, tlcontrol: &mut MSTLLogicControl, delay: SUMOTime) {
        let this = Ptr::from_mut(self);
        self.mySwitchCommand = Ptr::from_raw(Box::into_raw(Box::new(MSTrafficLightLogic_SwitchCommand::new(tlcontrol, this, delay))));
        let events = MSNet::get_instance().m().get_begin_of_timestep_events();
        events.m().add_event(self.mySwitchCommand.cast::<()>(), delay);
    }

    /// C++: `void MSTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with information about incoming lanes
    // void
    // MSTrafficLightLogic::init(NLDetectorBuilder&) {
    //     const Phases& phases = getPhases();
    //     if (phases.size() > 0 && (MSGlobals::gUseMesoSim || MSGlobals::gTLSPenalty > 0)) {
    //         initMesoTLSPenalties();
    //     }
    //     if (phases.size() > 1) {
    //         bool haveWarnedAboutUnusedStates = false;
    //         for (int i = 0; i < (int)phases.size(); ++i) {
    //             // warn about unused states
    //             std::vector<int> nextPhases;
    //             nextPhases.push_back((i + 1) % phases.size());
    //             bool iNextDefault = true;
    //             if (phases[i]->nextPhases.size() > 0) {
    //                 nextPhases = phases[i]->nextPhases;
    //                 iNextDefault = false;
    //             }
    //             for (int iNext : nextPhases) {
    //                 if (iNext < 0 || iNext >= (int)phases.size()) {
    //                     throw ProcessError("Invalid nextPhase " + toString(iNext) + " in tlLogic '" + getID()
    //                                        + "', program '" + getProgramID() + "' with " + toString(phases.size()) + " phases");
    //                 }
    //                 if (iNext == i && (int)nextPhases.size() == 1 && (int)phases.size() > 1) {
    //                     WRITE_WARNINGF("Phase % only loops backs to itself in tlLogic '%', program '%'.", i, getID(), getProgramID());
    //                 }
    //                 const std::string optionalFrom = iNextDefault ? "" : " from phase " + toString(i);
    //                 const std::string& state1 = phases[i]->getState();
    //                 const std::string& state2 = phases[iNext]->getState();
    //                 if (state1.size() != state2.size()) {
    //                     throw ProcessError("Mismatching phase state length in tlLogic '" + getID()
    //                                        + "', program '" + getProgramID() + "' in phases " + toString(i) + " and " + toString(iNext));
    //                 }
    //                 if (!haveWarnedAboutUnusedStates && state1.size() > myLanes.size() + myIgnoredIndices.size()) {
    //                     WRITE_WARNINGF(TL("Unused states in tlLogic '%', program '%' in phase % after tl-index %"),
    //                                    getID(), getProgramID(), i, (int)myLanes.size() - 1);
    //                     haveWarnedAboutUnusedStates = true;
    //                 }
    //                 // detect illegal states
    //                 const std::string::size_type illegal = state1.find_first_not_of(SUMOXMLDefinitions::ALLOWED_TLS_LINKSTATES);
    //                 if (std::string::npos != illegal) {
    //                     throw ProcessError("Illegal character '" + toString(state1[illegal]) + "' in tlLogic '" + getID()
    //                                        + "', program '" + getProgramID() + "' in phase " + toString(i));
    //                 }
    //                 // warn about transitions from green to red without intermediate yellow
    //                 bool haveWarned = false;
    //                 for (int j = 0; j < (int)MIN3(state1.size(), state2.size(), myLanes.size()) && !haveWarned; ++j) {
    //                     if ((LinkState)state2[j] == LINKSTATE_TL_RED
    //                             && ((LinkState)state1[j] == LINKSTATE_TL_GREEN_MAJOR
    //                                 || (LinkState)state1[j] == LINKSTATE_TL_GREEN_MINOR)) {
    //                         for (LaneVector::const_iterator it = myLanes[j].begin(); it != myLanes[j].end(); ++it) {
    //                             if ((*it)->getPermissions() != SVC_PEDESTRIAN) {
    //                                 if (getLogicType() != TrafficLightType::NEMA) {
    //                                     WRITE_WARNINGF(TL("Missing yellow phase in tlLogic '%', program '%' for tl-index % when switching% to phase %."),
    //                                                    getID(), getProgramID(), j, optionalFrom, iNext);
    //                                     // one warning per program is enough
    //                                     haveWarned = true;
    //                                 }
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // warn about links that never get the green light
    //     std::vector<bool> foundGreen(phases.front()->getState().size(), false);
    //     for (int i = 0; i < (int)phases.size(); ++i) {
    //         const std::string& state = phases[i]->getState();
    //         for (int j = 0; j < (int)state.size(); ++j) {
    //             LinkState ls = (LinkState)state[j];
    //             if (ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR || ls == LINKSTATE_TL_OFF_BLINKING || ls == LINKSTATE_TL_OFF_NOSIGNAL || ls == LINKSTATE_STOP) {
    //                 foundGreen[j] = true;
    //             }
    //         }
    //     }
    //     std::vector<bool> usedIndices(phases.front()->getState().size(), false);
    //     for (auto lv : myLinks) {
    //         for (const MSLink* link : lv) {
    //             if (link->getTLIndex() >= 0) {
    //                 usedIndices[link->getTLIndex()] = true;
    //             }
    //         }
    //     }
    //     for (int j = 0; j < (int)foundGreen.size(); ++j) {
    //         if (!foundGreen[j] && usedIndices[j]) {
    //             WRITE_WARNINGF(TL("Missing green phase in tlLogic '%', program '%' for tl-index %."), getID(), getProgramID(), j);
    //             break;
    //         }
    //     }
    //     // check direct conflict (two green links targeting the same lane)
    //     const int numLinks = (int)myLinks.size();
    //     std::set<const MSLane*> unsafeGreen;
    //     int firstUnsafePhase = -1;
    //     const MSLane* firstUnsafeLane = nullptr;
    //     int firstUnsafeOrigins = 0;
    //     int unsafeGreenPhases = 0;
    //     for (int i = 0; i < (int)phases.size(); ++i) {
    //         std::map<const MSLane*, int, ComparatorNumericalIdLess> greenLanes;
    //         const std::string& state = phases[i]->getState();
    //         for (int j = 0; j < numLinks; ++j) {
    //             if (state[j] == LINKSTATE_TL_GREEN_MAJOR) {
    //                 for (const MSLink* link : myLinks[j]) {
    //                     if (link->isInternalJunctionLink()) {
    //                         // links from an internal junction have implicit priority in case of conflict
    //                         continue;
    //                     }
    //                     greenLanes[link->getLane()] += 1;
    //                 }
    //             }
    //         }
    //         bool unsafe = false;
    //         for (auto item : greenLanes) {
    //             if (item.second > 1) {
    //                 if (unsafeGreenPhases == 0 && !unsafe) {
    //                     firstUnsafePhase = i;
    //                     firstUnsafeLane = item.first;
    //                     firstUnsafeOrigins = item.second;
    //                 }
    //                 unsafe = true;
    //                 unsafeGreen.insert(item.first);
    //             }
    //         }
    //         if (unsafe) {
    //             unsafeGreenPhases++;
    //         }
    //     }
    //     if (unsafeGreenPhases > 0) {
    //         const std::string furtherAffected = unsafeGreen.size() > 1 || unsafeGreenPhases > 1 ? TLF(" Overall % lanes in % phases are unsafe.", unsafeGreen.size(), unsafeGreenPhases) : "";
    //         WRITE_WARNINGF(TL("Unsafe green phase % in tlLogic '%', program '%'. Lane '%' is targeted by % 'G'-links. (use 'g' instead)%"),
    //                        firstUnsafePhase, getID(), getProgramID(), firstUnsafeLane->getID(), firstUnsafeOrigins, furtherAffected);
    //     }
    //
    //     // check incompatible junction logic
    //     // this can happen if the network was built with a very different signal
    //     // plan from the one currently being used.
    //     // Connections that never had a common green phase during network building may
    //     // have a symmetric response relation to avoid certain kinds of jam but this
    //     // can lead to deadlock if a different program gives minor green to both
    //     // connections at the same time
    //     // Note: mutual conflict between 'g' and 'G' is expected for traffic_light_right_on_red
    //
    //     const bool mustCheck = MSNet::getInstance()->hasInternalLinks();
    //     // The checks only runs for definitions from additional file and this is sufficient.
    //     // The distinction is implicit because original logics are loaded earlier and at that time hasInternalLinks is always false
    //     // Also, when the network has no internal links, mutual conflicts are not built by netconvert
    //     //std::cout << "init tlLogic=" << getID() << " prog=" << getProgramID() << " links=" << myLinks.size() << " internal=" << MSNet::getInstance()->hasInternalLinks() << "\n";
    //     if (mustCheck && phases.size() > 0) {
    //         // see NBNode::tlsConflict
    //         std::set<const MSJunction*> controlledJunctions;
    //         for (int j = 0; j < numLinks; ++j) {
    //             for (int k = 0; k < (int)myLinks[j].size(); ++k) {
    //                 MSLink* link = myLinks[j][k];
    //                 assert(link->getJunction() != nullptr);
    //                 controlledJunctions.insert(link->getJunction());
    //             }
    //         }
    //         const std::string minor = "gos";
    //         for (const MSJunction* junction : controlledJunctions) {
    //             const MSJunctionLogic* logic = junction->getLogic();
    //             if (logic != nullptr) {
    //                 // find symmetrical response
    //                 const int logicSize = logic->getLogicSize();
    //                 bool foundProblem = false;
    //                 std::vector<int> tlIndex;
    //                 for (int u = 0; u < logicSize && !foundProblem; u++) {
    //                     const MSLogicJunction::LinkBits& response = logic->getResponseFor(u);
    //                     for (int v = 0; v < logicSize && !foundProblem; v++) {
    //                         if (response.test(v)) {
    //                             if (logic->getResponseFor(v).test(u)) {
    //                                 // get tls link index for links u and v
    //                                 if (tlIndex.size() == 0) {
    //                                     // init tlindex for all links  once
    //                                     tlIndex.resize(logicSize, -1);
    //                                     for (int j = 0; j < numLinks; ++j) {
    //                                         for (int k = 0; k < (int)myLinks[j].size(); ++k) {
    //                                             MSLink* link = myLinks[j][k];
    //                                             if (link->getJunction() == junction) {
    //                                                 if (link->fromInternalLane()) {
    //                                                     // internal links may have their own control (i.e. for indirect left turn) but they also have their own conflict matrix
    //                                                     continue;
    //                                                 }
    //                                                 if (link->isCont() && link->getViaLane() != nullptr && link->getViaLane()->getLinkCont()[0]->getTLIndex() >= 0) {
    //                                                     // if the internal junction link is controlled, the first-part indices are not deadlock-relevant
    //                                                     continue;
    //                                                 }
    //                                                 tlIndex[link->getIndex()] = link->getTLIndex();
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                                 const int tlu = tlIndex[u];
    //                                 const int tlv = tlIndex[v];
    //                                 if (tlu >= 0 && tlv >= 0) {
    //                                     int phaseIndex = 0;
    //                                     for (MSPhaseDefinition* p : phases) {
    //                                         if (minor.find(p->getState()[tlu]) != std::string::npos
    //                                                 && minor.find(p->getState()[tlv]) != std::string::npos) {
    //                                             WRITE_WARNING(TLF("Program '%' at tlLogic '%' is incompatible with logic at junction '%' (mutual conflict between link indices %,% tl indices %,% phase %).\n"
    //                                                               "  To avoid deadlock/collisions, either: rebuild the signal plan with a newer version of netconvert/netedit\n"
    //                                                               "  or rebuild the network with option '--tls.ignore-internal-junction-jam' or include the program when building.",
    //                                                               getProgramID(), getID(), junction->getID(), u, v, tlu, tlv, phaseIndex));
    //                                             // only one warning per program
    //                                             foundProblem = true;
    //                                             break;
    //                                         }
    //                                         phaseIndex++;
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     myNumLinks = (int)myLinks.size();
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        let phases: MSTrafficLightLogic_Phases = self.get_phases().clone();
        if !phases.is_empty() && (utils::options::options_cont::get_bool("mesosim") || utils::options::options_cont::get_float("meso-tls-penalty") > 0.0) {
            // MSGlobals::gUseMesoSim, MSGlobals::gTLSPenalty
            self.init_meso_tls_penalties();
        }
        if phases.len() > 1 {
            let mut haveWarnedAboutUnusedStates = false;
            for i in 0..phases.len() {
                // warn about unused states
                let mut nextPhases: Vec<i32> = vec![((i + 1) % phases.len()) as i32];
                let mut iNextDefault = true;
                if phases[i].nextPhases.len() > 0 {
                    nextPhases = phases[i].nextPhases.clone();
                    iNextDefault = false;
                }
                for iNext in nextPhases.clone() {
                    if iNext < 0 || iNext as usize >= phases.len() {
                        panic!("{}", ProcessError::new(format!("Invalid nextPhase {} in tlLogic '{}', program '{}' with {} phases", iNext, self.get_id(), self.get_program_id(), phases.len())));
                    }
                    let iNextU = iNext as usize;
                    if iNextU == i && nextPhases.len() == 1 && phases.len() > 1 {
                        utils::write_warningf!("Phase % only loops backs to itself in tlLogic '%', program '%'.", i as i32, self.get_id(), self.get_program_id());
                    }
                    let optionalFrom = if iNextDefault { String::new() } else { format!(" from phase {}", i) };
                    let state1: String = phases[i].get_state().to_string();
                    let state2: String = phases[iNextU].get_state().to_string();
                    if state1.len() != state2.len() {
                        panic!("{}", ProcessError::new(format!("Mismatching phase state length in tlLogic '{}', program '{}' in phases {} and {}", self.get_id(), self.get_program_id(), i, iNext)));
                    }
                    if !haveWarnedAboutUnusedStates && state1.len() > self.myLanes.len() + self.myIgnoredIndices.len() {
                        utils::write_warningf!("Unused states in tlLogic '%', program '%' in phase % after tl-index %", self.get_id(), self.get_program_id(), i as i32, self.myLanes.len() as i32 - 1);
                        haveWarnedAboutUnusedStates = true;
                    }
                    // detect illegal states
                    if let Some(illegal) = state1.find(|c: char| !utils::xml::sumo_xml_definitions::ALLOWED_TLS_LINKSTATES.contains(c)) {
                        panic!("{}", ProcessError::new(format!("Illegal character '{}' in tlLogic '{}', program '{}' in phase {}", state1.as_bytes()[illegal] as char, self.get_id(), self.get_program_id(), i)));
                    }
                    // warn about transitions from green to red without intermediate yellow
                    let mut haveWarned = false;
                    let limit = std::cmp::min(std::cmp::min(state1.len(), state2.len()), self.myLanes.len());
                    for j in 0..limit {
                        if haveWarned {
                            break;
                        }
                        let s1 = (state1.as_bytes()[j] as i8) as LinkState;
                        let s2 = (state2.as_bytes()[j] as i8) as LinkState;
                        if s2 == LINKSTATE_TL_RED && (s1 == LINKSTATE_TL_GREEN_MAJOR || s1 == LINKSTATE_TL_GREEN_MINOR) {
                            for lane in self.myLanes[j].clone() {
                                if lane.get_permissions() != SVC_PEDESTRIAN {
                                    if self.get_logic_type() != TrafficLightType::NEMA {
                                        utils::write_warningf!("Missing yellow phase in tlLogic '%', program '%' for tl-index % when switching% to phase %.", self.get_id(), self.get_program_id(), j as i32, optionalFrom.clone(), iNext);
                                        // one warning per program is enough
                                        haveWarned = true;
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        // warn about links that never get the green light
        let mut foundGreen: Vec<bool> = vec![false; phases[0].get_state().len()];
        for i in 0..phases.len() {
            let state = phases[i].get_state().to_string();
            for j in 0..state.len() {
                let ls = (state.as_bytes()[j] as i8) as LinkState;
                if ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR || ls == LINKSTATE_TL_OFF_BLINKING || ls == LINKSTATE_TL_OFF_NOSIGNAL || ls == LINKSTATE_STOP {
                    foundGreen[j] = true;
                }
            }
        }
        let mut usedIndices: Vec<bool> = vec![false; phases[0].get_state().len()];
        for lv in &self.myLinks {
            for link in lv {
                if link.get_tl_index() >= 0 {
                    usedIndices[link.get_tl_index() as usize] = true;
                }
            }
        }
        for j in 0..foundGreen.len() {
            if !foundGreen[j] && usedIndices[j] {
                utils::write_warningf!("Missing green phase in tlLogic '%', program '%' for tl-index %.", self.get_id(), self.get_program_id(), j as i32);
                break;
            }
        }
        // check direct conflict (two green links targeting the same lane)
        let numLinks = self.myLinks.len();
        let mut unsafeGreen: BTreeSet<Ptr<MSLane>> = BTreeSet::new();
        let mut firstUnsafePhase: i32 = -1;
        let mut firstUnsafeLane: Ptr<MSLane> = Ptr::null();
        let mut firstUnsafeOrigins: i32 = 0;
        let mut unsafeGreenPhases: i32 = 0;
        for i in 0..phases.len() {
            // determinism: C++ orders greenLanes by numerical lane id (ComparatorNumericalIdLess); here the map is ordered by address
            let mut greenLanes: BTreeMap<Ptr<MSLane>, i32> = BTreeMap::new();
            let state = phases[i].get_state().to_string();
            for j in 0..numLinks {
                if ((state.as_bytes()[j] as i8) as LinkState) == LINKSTATE_TL_GREEN_MAJOR {
                    for link in &self.myLinks[j] {
                        if link.is_internal_junction_link() {
                            // links from an internal junction have implicit priority in case of conflict
                            continue;
                        }
                        *greenLanes.entry(link.get_lane()).or_insert(0) += 1;
                    }
                }
            }
            let mut isUnsafe = false;
            for (lane, count) in greenLanes.iter() {
                if *count > 1 {
                    if unsafeGreenPhases == 0 && !isUnsafe {
                        firstUnsafePhase = i as i32;
                        firstUnsafeLane = *lane;
                        firstUnsafeOrigins = *count;
                    }
                    isUnsafe = true;
                    unsafeGreen.insert(*lane);
                }
            }
            if isUnsafe {
                unsafeGreenPhases += 1;
            }
        }
        if unsafeGreenPhases > 0 {
            let furtherAffected = if unsafeGreen.len() > 1 || unsafeGreenPhases > 1 {
                format!(" Overall {} lanes in {} phases are unsafe.", unsafeGreen.len(), unsafeGreenPhases)
            } else {
                String::new()
            };
            utils::write_warningf!("Unsafe green phase % in tlLogic '%', program '%'. Lane '%' is targeted by % 'G'-links. (use 'g' instead)%", firstUnsafePhase, self.get_id(), self.get_program_id(), firstUnsafeLane.get_id(), firstUnsafeOrigins, furtherAffected);
        }
        // shortcut: the incompatible-junction-logic check (symmetric response between junction links) is skipped because MSLogicJunction::LinkBits is still Unmapped; upgrade when the response bitset is mapped
        self.myNumLinks = self.myLinks.len() as i32;
    }

    /// C++: `void MSTrafficLightLogic::initMesoTLSPenalties()`
    /// initialize optional meso penalties
    // void MSTrafficLightLogic::initMesoTLSPenalties() {
    //     if (myLogicType == TrafficLightType::RAIL_SIGNAL) {
    //         return;
    //     }
    //     // set mesoscopic time penalties
    //     const Phases& phases = getPhases();
    //     const int numLinks = (int)myLinks.size();
    //     // warning already given if not all states are used
    //     assert(numLinks <= (int)phases.front()->getState().size());
    //     SUMOTime duration = 0;
    //     std::vector<double> firstRedDuration(numLinks, 0);
    //     std::vector<double> redDuration(numLinks, 0);
    //     std::vector<double> totalRedDuration(numLinks, 0);
    //     std::vector<double> penalty(numLinks, 0);
    //     for (int i = 0; i < (int)phases.size(); ++i) {
    //         const std::string& state = phases[i]->getState();
    //         duration += phases[i]->duration;
    //         // warn about transitions from green to red without intermediate yellow
    //         for (int j = 0; j < numLinks; ++j) {
    //             double& red = redDuration[j];
    //             if ((LinkState)state[j] == LINKSTATE_TL_RED
    //                     || (LinkState)state[j] == LINKSTATE_TL_REDYELLOW) {
    //                 red += STEPS2TIME(phases[i]->duration);
    //                 totalRedDuration[j] += STEPS2TIME(phases[i]->duration);
    //             } else if (red > 0) {
    //                 if (firstRedDuration[j] == 0) {
    //                     // store for handling wrap-around
    //                     firstRedDuration[j] = red;
    //                 } else {
    //                     // vehicle may arive in any second or the red duration
    //                     // compute the sum over [0,red]
    //                     penalty[j] += 0.5 * (red * red + red);
    //                 }
    //                 red = 0;
    //             }
    //         }
    //     }
    //     // final phase and wrap-around to first phase
    //     for (int j = 0; j < numLinks; ++j) {
    //         double red = redDuration[j] + firstRedDuration[j];
    //         if (red) {
    //             penalty[j] += 0.5 * (red * red + red);
    //         }
    //     }
    //     double tlsPenalty = MSGlobals::gTLSPenalty;
    //     const double durationSeconds = STEPS2TIME(duration);
    //     std::set<const MSJunction*> controlledJunctions;
    //     std::set<const MSEdge*> shortEdges;;
    //     const bool overridePenalty = StringUtils::toBool(getParameter(MESegment::OVERRIDE_TLS_PENALTIES, "0"));
    //     for (int j = 0; j < numLinks; ++j) {
    //         for (int k = 0; k < (int)myLinks[j].size(); ++k) {
    //             MSLink* link = myLinks[j][k];
    //             MSEdge& edge = link->getLaneBefore()->getEdge();
    //             if (MSGlobals::gUseMesoSim) {
    //                 const MESegment::MesoEdgeType& edgeType = MSNet::getInstance()->getMesoType(edge.getEdgeType());
    //                 tlsPenalty = edgeType.tlsPenalty;
    //                 double greenFraction = (durationSeconds - totalRedDuration[j]) / durationSeconds;
    //                 if (edgeType.tlsFlowPenalty == 0) {
    //                     greenFraction = 1;
    //                 } else {
    //                     greenFraction = MAX2(MIN2(greenFraction / edgeType.tlsFlowPenalty, 1.0), 0.01);
    //                 }
    //                 if (greenFraction == 0.01) {
    //                     WRITE_WARNINGF(TL("Green fraction is only 1% for link % in tlLogic '%', program '%'."), "%", j, getID(), getProgramID());
    //                 }
    //                 link->setGreenFraction(greenFraction);
    //                 if (tlsPenalty > 0 && edge.getLength() < SHORT_EDGE && shortEdges.count(&edge) == 0) {
    //                     shortEdges.insert(&edge);
    //                     WRITE_WARNINGF(TL("Edge '%' is shorter than %m (%m) and will cause incorrect flow reduction with option --meso-tls-penalty"), edge.getID(), SHORT_EDGE, edge.getLength());
    //                 }
    //                 if (overridePenalty) {
    //                     MESegment* last = MSGlobals::gMesoNet->getSegmentForEdge(edge, std::numeric_limits<double>::max());
    //                     last->overrideTLSPenalty();
    //                 }
    //             }
    //             link->setMesoTLSPenalty(TIME2STEPS(tlsPenalty * penalty[j] / durationSeconds));
    //             controlledJunctions.insert(link->getLane()->getEdge().getFromJunction()); // MSLink::myJunction is not yet initialized
    //             //std::cout << " tls=" << getID() << " i=" << j << " link=" << link->getDescription() << " p=" << penalty[j] << " fr=" << firstRedDuration[j] << " r=" << redDuration[j] << " tr=" << totalRedDuration[j] << " durSecs=" << durationSeconds << " tlsPen=" << STEPS2TIME(link->getMesoTLSPenalty()) << " gF=" << myLinks[j][k]->getGreenFraction() << "\n";
    //         }
    //     }
    //     // initialize empty-net travel times
    //     // XXX refactor after merging sharps (links know their incoming edge)
    //     for (std::set<const MSJunction*>::iterator it = controlledJunctions.begin(); it != controlledJunctions.end(); ++it) {
    //         const ConstMSEdgeVector incoming = (*it)->getIncoming();
    //         for (ConstMSEdgeVector::const_iterator it_e = incoming.begin(); it_e != incoming.end(); ++it_e) {
    //             const_cast<MSEdge*>(*it_e)->recalcCache();
    //         }
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn init_meso_tls_penalties(&mut self) {
        if matches!(self.myLogicType, TrafficLightType::RAIL_SIGNAL) {
            return;
        }
        // set mesoscopic time penalties
        let phases: MSTrafficLightLogic_Phases = self.get_phases().clone();
        let numLinks = self.myLinks.len();
        // warning already given if not all states are used
        assert!(numLinks <= phases[0].get_state().len());
        let mut duration: SUMOTime = 0;
        let mut firstRedDuration: Vec<f64> = vec![0.0; numLinks];
        let mut redDuration: Vec<f64> = vec![0.0; numLinks];
        let mut totalRedDuration: Vec<f64> = vec![0.0; numLinks];
        let mut penalty: Vec<f64> = vec![0.0; numLinks];
        for i in 0..phases.len() {
            let state = phases[i].get_state().to_string();
            duration += phases[i].duration;
            // warn about transitions from green to red without intermediate yellow
            for j in 0..numLinks {
                if ((state.as_bytes()[j] as i8) as LinkState) == LINKSTATE_TL_RED
                    || ((state.as_bytes()[j] as i8) as LinkState) == LINKSTATE_TL_REDYELLOW {
                    redDuration[j] += steps2time(phases[i].duration);
                    totalRedDuration[j] += steps2time(phases[i].duration);
                } else if redDuration[j] > 0.0 {
                    if firstRedDuration[j] == 0.0 {
                        // store for handling wrap-around
                        firstRedDuration[j] = redDuration[j];
                    } else {
                        // vehicle may arive in any second or the red duration
                        // compute the sum over [0,red]
                        penalty[j] += 0.5 * (redDuration[j] * redDuration[j] + redDuration[j]);
                    }
                    redDuration[j] = 0.0;
                }
            }
        }
        // final phase and wrap-around to first phase
        for j in 0..numLinks {
            let red = redDuration[j] + firstRedDuration[j];
            if red != 0.0 {
                penalty[j] += 0.5 * (red * red + red);
            }
        }
        // MSGlobals::gTLSPenalty
        let mut tlsPenalty: f64 = utils::options::options_cont::get_float("meso-tls-penalty");
        let durationSeconds = steps2time(duration);
        let mut controlledJunctions: BTreeSet<Ptr<MSJunction>> = BTreeSet::new();
        let mut shortEdges: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
        // SHORT_EDGE = (default SUMOVTypeParameter length 5 + minGap 2.5) * 2
        let shortEdgeLength: f64 = (5.0 + 2.5) * 2.0;
        // shortcut: MESegment::OVERRIDE_TLS_PENALTIES ("meso.tls.control") is read from the logic parameters, which are not stored yet, so it is always off
        let overridePenalty = utils::common::string_utils::to_bool(&self.base2.get_parameter("meso.tls.control", "0")).unwrap_or_else(|e| panic!("{}", e));
        for j in 0..numLinks {
            for k in 0..self.myLinks[j].len() {
                let link: Ptr<MSLink> = self.myLinks[j][k];
                let edge: Ptr<MSEdge> = Ptr::from_mut(link.get_lane_before().get_edge());
                let mut greenFraction: f64;
                if utils::options::options_cont::get_bool("mesosim") {
                    // MSGlobals::gUseMesoSim
                    let net = MSNet::get_instance();
                    let edgeType = net.m().get_meso_type(edge.get_edge_type());
                    tlsPenalty = edgeType.tlsPenalty;
                    greenFraction = (durationSeconds - totalRedDuration[j]) / durationSeconds;
                    if edgeType.tlsFlowPenalty == 0.0 {
                        greenFraction = 1.0;
                    } else {
                        greenFraction = f64::max(f64::min(greenFraction / edgeType.tlsFlowPenalty, 1.0), 0.01);
                    }
                    if greenFraction == 0.01 {
                        utils::write_warningf!("Green fraction is only 1% for link % in tlLogic '%', program '%'.", "%", j as i32, self.get_id(), self.get_program_id());
                    }
                    link.m().set_green_fraction(greenFraction);
                    if tlsPenalty > 0.0 && edge.get_length() < shortEdgeLength && !shortEdges.contains(&edge) {
                        shortEdges.insert(edge);
                        utils::write_warningf!("Edge '%' is shorter than %m (%m) and will cause incorrect flow reduction with option --meso-tls-penalty", edge.get_id(), shortEdgeLength, edge.get_length());
                    }
                    if overridePenalty {
                        // shortcut: MESegment::overrideTLSPenalty needs MSGlobals::gMesoNet, which is not reachable from this port yet; upgrade when MSGlobals statics are ported
                    }
                }
                link.m().set_meso_tls_penalty(time2steps(tlsPenalty * penalty[j] / durationSeconds));
                controlledJunctions.insert(link.get_lane().get_edge().get_from_junction());
            }
        }
        // initialize empty-net travel times
        // XXX refactor after merging sharps (links know their incoming edge)
        for junction in controlledJunctions {
            let incoming: Vec<Ptr<MSEdge>> = junction.get_incoming().clone();
            for e in incoming {
                e.m().recalc_cache();
            }
        }
    }

    /// C++: `void MSTrafficLightLogic::~MSTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSTrafficLightLogic::~MSTrafficLightLogic() {
    //     // no need to do something about mySwitchCommand here,
    //     // it is handled by the event control
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no work to do: mySwitchCommand is handled by the event control
    }

    /// C++: `void MSTrafficLightLogic::addLink(MSLink *, MSLane *, int)` [virtual]
    /// Adds a link on building
    // void
    // MSTrafficLightLogic::addLink(MSLink* link, MSLane* lane, int pos) {
    //     // !!! should be done within the loader (checking necessary)
    //     myLinks.reserve(pos + 1);
    //     while ((int)myLinks.size() <= pos) {
    //         myLinks.push_back(LinkVector());
    //     }
    //     myLinks[pos].push_back(link);
    //     //
    //     myLanes.reserve(pos + 1);
    //     while ((int)myLanes.size() <= pos) {
    //         myLanes.push_back(LaneVector());
    //     }
    //     myLanes[pos].push_back(lane);
    //     link->setTLState((LinkState) getCurrentPhaseDef().getState()[pos], MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<MSLink>, lane: Ptr<MSLane>, pos: i32) {
        while (self.myLinks.len() as i32) <= pos {
            self.myLinks.push(Vec::new());
        }
        self.myLinks[pos as usize].push(link);
        while (self.myLanes.len() as i32) <= pos {
            self.myLanes.push(Vec::new());
        }
        self.myLanes[pos as usize].push(lane);
        let state: LinkState = (self.get_current_phase_def().get_state().as_bytes()[pos as usize] as i8) as LinkState;
        let now = MSNet::get_instance().get_current_time_step();
        link.m().set_tl_state(state, now);
    }

    /// C++: `void MSTrafficLightLogic::ignoreLinkIndex(int)`
    /// ignore pedestrian crossing index in mesosim
    // void
    // MSTrafficLightLogic::ignoreLinkIndex(int pos) {
    //     myIgnoredIndices.insert(pos);
    // }
    #[allow(unused_variables)]
    pub fn ignore_link_index(&mut self, pos: i32) {
        self.myIgnoredIndices.insert(pos);
    }

    /// C++: `void MSTrafficLightLogic::adaptLinkInformationFrom(const MSTrafficLightLogic &)` [virtual]
    /// Applies information about controlled links and lanes from the given logic
    // void
    // MSTrafficLightLogic::adaptLinkInformationFrom(const MSTrafficLightLogic& logic) {
    //     myLinks = logic.myLinks;
    //     myLanes = logic.myLanes;
    //     myIgnoredIndices = logic.myIgnoredIndices;
    // }
    #[allow(unused_variables)]
    pub fn adapt_link_information_from(&mut self, logic: &MSTrafficLightLogic) {
        self.myLinks = logic.myLinks.clone();
        self.myLanes = logic.myLanes.clone();
        self.myIgnoredIndices = logic.myIgnoredIndices.clone();
    }

    /// C++: `std::map<MSLink *, LinkState> MSTrafficLightLogic::collectLinkStates() const` [const]
    /// Returns the (uncontrolled) states of the controlled links
    // std::map<MSLink*, LinkState>
    // MSTrafficLightLogic::collectLinkStates() const {
    //     std::map<MSLink*, LinkState> ret;
    //     for (LinkVectorVector::const_iterator i1 = myLinks.begin(); i1 != myLinks.end(); ++i1) {
    //         const LinkVector& l = (*i1);
    //         for (LinkVector::const_iterator i2 = l.begin(); i2 != l.end(); ++i2) {
    //             ret[*i2] = (*i2)->getState();
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn collect_link_states(&self) -> BTreeMap<Ptr<MSLink>, LinkState> {
        let mut ret: BTreeMap<Ptr<MSLink>, LinkState> = BTreeMap::new();
        for l in &self.myLinks {
            for link in l {
                ret.insert(*link, link.get_state());
            }
        }
        ret
    }

    /// C++: `void MSTrafficLightLogic::resetLinkStates(const std::map<MSLink *, LinkState> &) const` [const]
    /// Resets the states of controlled links
    // void
    // MSTrafficLightLogic::resetLinkStates(const std::map<MSLink*, LinkState>& vals) const {
    //     for (LinkVectorVector::const_iterator i1 = myLinks.begin(); i1 != myLinks.end(); ++i1) {
    //         const LinkVector& l = (*i1);
    //         for (LinkVector::const_iterator i2 = l.begin(); i2 != l.end(); ++i2) {
    //             assert(vals.find(*i2) != vals.end());
    //             (*i2)->setTLState(vals.find(*i2)->second, MSNet::getInstance()->getCurrentTimeStep());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_link_states(&self, vals: &BTreeMap<Ptr<MSLink>, LinkState>) {
        let now = MSNet::get_instance().get_current_time_step();
        for l in &self.myLinks {
            for link in l {
                assert!(vals.contains_key(link));
                link.m().set_tl_state(vals[link], now);
            }
        }
    }

    /// C++: `SUMOTime MSTrafficLightLogic::trySwitch()` [virtual, pure virtual]
    /// Switches to the next phase
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        panic!("MSTrafficLightLogic::trySwitch is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `void MSTrafficLightLogic::activateProgram()` [virtual]
    /// called when switching programs
    // void
    // MSTrafficLightLogic::activateProgram() {
    //     myAmActive = true;
    //     // updated the traffic light logic stored in the link
    //     for (const LinkVector& currGroup : myLinks) {
    //         for (MSLink* link : currGroup) {
    //             link->setTLLogic(this);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn activate_program(&mut self) {
        self.myAmActive = true;
        // updated the traffic light logic stored in the link
        let this = Ptr::from_mut(self);
        for currGroup in &self.myLinks {
            for link in currGroup {
                link.m().set_tl_logic(this);
            }
        }
    }

    /// C++: `void MSTrafficLightLogic::deactivateProgram()` [virtual]
    // void
    // MSTrafficLightLogic::deactivateProgram() {
    //     myAmActive = false;
    // }
    #[allow(unused_variables)]
    pub fn deactivate_program(&mut self) {
        self.myAmActive = false;
    }

    /// C++: `bool MSTrafficLightLogic::setTrafficLightSignals(SUMOTime) const` [const]
    /// Applies the current signal states to controlled links
    // bool
    // MSTrafficLightLogic::setTrafficLightSignals(SUMOTime t) const {
    //     // get the current traffic light signal combination
    //     const std::string& state = getCurrentPhaseDef().getState();
    //     // go through the links
    //     for (int i = 0; i < (int)myLinks.size(); i++) {
    //         const LinkVector& currGroup = myLinks[i];
    //         LinkState ls = (LinkState) state[i];
    //         for (LinkVector::const_iterator j = currGroup.begin(); j != currGroup.end(); j++) {
    //             (*j)->setTLState(ls, t);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn set_traffic_light_signals(&self, t: SUMOTime) -> bool {
        // get the current traffic light signal combination
        let state = self.get_current_phase_def().get_state().to_string();
        // go through the links
        for i in 0..self.myLinks.len() {
            let currGroup = &self.myLinks[i];
            let ls: LinkState = (state.as_bytes()[i] as i8) as LinkState;
            for link in currGroup {
                link.m().set_tl_state(ls, t);
            }
        }
        true
    }

    /// C++: `const std::string & MSTrafficLightLogic::getProgramID() const` [const]
    /// Returns this tl-logic's id
    // const std::string& getProgramID() const {
    //         return myProgramID;
    //     }
    #[allow(unused_variables)]
    pub fn get_program_id(&self) -> &str {
        &self.myProgramID
    }

    /// C++: `const LaneVectorVector & MSTrafficLightLogic::getLaneVectors() const` [const]
    /// Returns the list of lists of all lanes controlled by this tls
    // const LaneVectorVector& getLaneVectors() const {
    //         return myLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_vectors(&self) -> &MSTrafficLightLogic_LaneVectorVector {
        &self.myLanes
    }

    /// C++: `const LaneVector & MSTrafficLightLogic::getLanesAt(int) const` [const]
    /// Returns the list of lanes that are controlled by the signals at the given position
    // const LaneVector& getLanesAt(int i) const {
    //         if (i < (int)myLanes.size()) {
    //             return myLanes[i];
    //         } else {
    //             return myEmptyLaneVector;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_lanes_at(&self, i: i32) -> &LaneVector {
        if (i as usize) < self.myLanes.len() {
            &self.myLanes[i as usize]
        } else {
            MY_EMPTY_LANE_VECTOR
        }
    }

    /// C++: `const LinkVectorVector & MSTrafficLightLogic::getLinks() const` [const]
    /// Returns the list of lists of all affected links
    // const LinkVectorVector& getLinks() const {
    //         return myLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_links(&self) -> &MSTrafficLightLogic_LinkVectorVector {
        &self.myLinks
    }

    /// C++: `const LinkVector & MSTrafficLightLogic::getLinksAt(int) const` [const]
    /// Returns the list of links that are controlled by the signals at the given position
    // const LinkVector& getLinksAt(int i) const {
    //         return myLinks[i];
    //     }
    #[allow(unused_variables)]
    pub fn get_links_at(&self, i: i32) -> &MSTrafficLightLogic_LinkVector {
        &self.myLinks[i as usize]
    }

    /// C++: `int MSTrafficLightLogic::getLinkIndex(const MSLink *const) const` [const]
    /// Returns the index of the given link
    // int
    // MSTrafficLightLogic::getLinkIndex(const MSLink* const link) const {
    //     int index = 0;
    //     for (LinkVectorVector::const_iterator i1 = myLinks.begin(); i1 != myLinks.end(); ++i1, ++index) {
    //         const LinkVector& l = (*i1);
    //         for (LinkVector::const_iterator i2 = l.begin(); i2 != l.end(); ++i2) {
    //             if ((*i2) == link) {
    //                 return index;
    //             }
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_link_index(&self, link: Ptr<MSLink>) -> i32 {
        let mut index: i32 = 0;
        for l in &self.myLinks {
            for l2 in l {
                if *l2 == link {
                    return index;
                }
            }
            index += 1;
        }
        -1
    }

    /// C++: `int MSTrafficLightLogic::getPhaseNumber() const` [const, virtual, pure virtual]
    /// Returns the number of phases
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phase_number(&self) -> i32 {
        panic!("MSTrafficLightLogic::getPhaseNumber is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `const Phases & MSTrafficLightLogic::getPhases() const` [const, virtual, pure virtual]
    /// Returns the phases of this tls program
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &MSTrafficLightLogic_Phases {
        panic!("MSTrafficLightLogic::getPhases is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `const MSPhaseDefinition & MSTrafficLightLogic::getPhase(int) const` [const, virtual, pure virtual]
    /// Returns the definition of the phase from the given position within the plan
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phase(&self, givenstep: i32) -> &MSPhaseDefinition {
        panic!("MSTrafficLightLogic::getPhase is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `TrafficLightType MSTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic
    // TrafficLightType getLogicType() const {
    //         return myLogicType;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> TrafficLightType {
        self.myLogicType
    }

    /// C++: `int MSTrafficLightLogic::getCurrentPhaseIndex() const` [const, virtual, pure virtual]
    /// Returns the current index within the program
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_phase_index(&self) -> i32 {
        panic!("MSTrafficLightLogic::getCurrentPhaseIndex is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `const MSPhaseDefinition & MSTrafficLightLogic::getCurrentPhaseDef() const` [const, virtual, pure virtual]
    /// Returns the definition of the current phase
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        panic!("MSTrafficLightLogic::getCurrentPhaseDef is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `void MSTrafficLightLogic::resetLastSwitch(SUMOTime)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        panic!("MSTrafficLightLogic::resetLastSwitch is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getMinDur(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getMinDur(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.minDuration;
    // }
    #[allow(unused_variables)]
    pub fn get_min_dur(&self, step: i32) -> SUMOTime {
        let p = if step < 0 { self.get_current_phase_def() } else { self.get_phase(step) };
        p.minDuration
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getMaxDur(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getMaxDur(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.maxDuration;
    // }
    #[allow(unused_variables)]
    pub fn get_max_dur(&self, step: i32) -> SUMOTime {
        let p = if step < 0 { self.get_current_phase_def() } else { self.get_phase(step) };
        p.maxDuration
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getEarliestEnd(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getEarliestEnd(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.earliestEnd;
    // }
    #[allow(unused_variables)]
    pub fn get_earliest_end(&self, step: i32) -> SUMOTime {
        let p = if step < 0 { self.get_current_phase_def() } else { self.get_phase(step) };
        p.earliestEnd
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getLatestEnd(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getLatestEnd(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.latestEnd;
    // }
    #[allow(unused_variables)]
    pub fn get_latest_end(&self, step: i32) -> SUMOTime {
        let p = if step < 0 { self.get_current_phase_def() } else { self.get_phase(step) };
        p.latestEnd
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getDefaultCycleTime() const` [const]
    /// Returns the cycle time (in ms)
    // SUMOTime getDefaultCycleTime() const {
    //         return myDefaultCycleTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_cycle_time(&self) -> SUMOTime {
        self.myDefaultCycleTime
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getTimeInCycle() const` [const]
    /// return time within the current cycle
    // SUMOTime
    // MSTrafficLightLogic::getTimeInCycle() const {
    //     return mapTimeInCycle(SIMSTEP);
    // }
    #[allow(unused_variables)]
    pub fn get_time_in_cycle(&self) -> SUMOTime {
        self.map_time_in_cycle(MSNet::get_instance().get_current_time_step())
    }

    /// C++: `SUMOTime MSTrafficLightLogic::mapTimeInCycle(SUMOTime) const` [const, virtual]
    /// map the given time into the current cycle
    // SUMOTime
    // MSTrafficLightLogic::mapTimeInCycle(SUMOTime t) const {
    //     return (t - myOffset) % myDefaultCycleTime;
    // }
    #[allow(unused_variables)]
    pub fn map_time_in_cycle(&self, t: SUMOTime) -> SUMOTime {
        (t - self.myOffset) % self.myDefaultCycleTime
    }

    /// C++: `int MSTrafficLightLogic::getNumLinks() const` [const]
    /// return the number of controlled link indices
    // int getNumLinks() const {
    //         return myNumLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_num_links(&self) -> i32 {
        self.myNumLinks
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getNextSwitchTime() const` [const]
    /// Returns the assumed next switch time
    // SUMOTime
    // MSTrafficLightLogic::getNextSwitchTime() const {
    //     return mySwitchCommand != nullptr ? mySwitchCommand->getNextSwitchTime() : -1;
    // }
    #[allow(unused_variables)]
    pub fn get_next_switch_time(&self) -> SUMOTime {
        if !self.mySwitchCommand.is_null() {
            self.mySwitchCommand.get_next_switch_time()
        } else {
            -1
        }
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getSpentDuration(SUMOTime) const` [const]
    /// Returns the duration spent in the current phase
    // SUMOTime
    // MSTrafficLightLogic::getSpentDuration(SUMOTime simStep) const {
    //     if (simStep == -1) {
    //         simStep = SIMSTEP;
    //     }
    //     return simStep - getCurrentPhaseDef().myLastSwitch;
    // }
    #[allow(unused_variables)]
    pub fn get_spent_duration(&self, simStep: SUMOTime) -> SUMOTime {
        let simStep = if simStep == -1 { MSNet::get_instance().get_current_time_step() } else { simStep };
        simStep - self.get_current_phase_def().myLastSwitch
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getPhaseIndexAtTime(SUMOTime) const` [const, virtual, pure virtual]
    /// Returns the index of the logic at the given simulation step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phase_index_at_time(&self, simStep: SUMOTime) -> SUMOTime {
        panic!("MSTrafficLightLogic::getPhaseIndexAtTime is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getOffsetFromIndex(int) const` [const, virtual, pure virtual]
    /// Returns the position (start of a phase during a cycle) from of a given step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_offset_from_index(&self, index: i32) -> SUMOTime {
        panic!("MSTrafficLightLogic::getOffsetFromIndex is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `int MSTrafficLightLogic::getIndexFromOffset(SUMOTime) const` [const, virtual, pure virtual]
    /// Returns the step (the phasenumber) of a given position of the cycle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_index_from_offset(&self, offset: SUMOTime) -> i32 {
        panic!("MSTrafficLightLogic::getIndexFromOffset is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getOffset() const` [const]
    /// @}
    // SUMOTime getOffset() const {
    //         return myOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_offset(&self) -> SUMOTime {
        self.myOffset
    }

    /// C++: `void MSTrafficLightLogic::addOverridingDuration(SUMOTime)`
    /// Changes the duration of the next phase
    // void
    // MSTrafficLightLogic::addOverridingDuration(SUMOTime duration) {
    //     myOverridingTimes.push_back(duration);
    // }
    #[allow(unused_variables)]
    pub fn add_overriding_duration(&mut self, duration: SUMOTime) {
        self.myOverridingTimes.push(duration);
    }

    /// C++: `void MSTrafficLightLogic::setCurrentDurationIncrement(SUMOTime)`
    /// Delays current phase by the given delay
    // void
    // MSTrafficLightLogic::setCurrentDurationIncrement(SUMOTime delay) {
    //     myCurrentDurationIncrement = delay;
    // }
    #[allow(unused_variables)]
    pub fn set_current_duration_increment(&mut self, delay: SUMOTime) {
        self.myCurrentDurationIncrement = delay;
    }

    /// C++: `void MSTrafficLightLogic::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual, pure virtual]
    /// Changes the current phase and her duration
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn change_step_and_duration(&mut self, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
        panic!("MSTrafficLightLogic::changeStepAndDuration is pure virtual; call it on a concrete traffic light logic")
    }

    /// C++: `bool MSTrafficLightLogic::isSelected() const` [const]
    /// whether this logic is selected in the GUI
    // bool
    // MSTrafficLightLogic::isSelected() const {
    //     return MSNet::getInstance()->isSelected(this);
    // }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        MSNet::get_instance().is_selected(Ptr::new(self))
    }

    /// C++: `bool MSTrafficLightLogic::isActive() const` [const]
    /// whether this logic is the active program
    // bool isActive() const {
    //         return myAmActive;
    //     }
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        self.myAmActive
    }

    /// C++: `bool MSTrafficLightLogic::getsMajorGreen(int) const` [const, virtual]
    /// whether the given link index ever turns 'G'
    // bool
    // MSTrafficLightLogic::getsMajorGreen(int linkIndex) const {
    //     if (linkIndex >= 0 && linkIndex < getNumLinks()) {
    //         for (const MSPhaseDefinition* p : getPhases()) {
    //             const std::string& s = p->getState();
    //             assert(linkIndex < (int)s.size());
    //             if (s[linkIndex] == LINKSTATE_TL_GREEN_MAJOR) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    //
    // }
    #[allow(unused_variables)]
    pub fn gets_major_green(&self, linkIndex: i32) -> bool {
        if linkIndex >= 0 && linkIndex < self.get_num_links() {
            for p in self.get_phases() {
                let s = p.get_state();
                if ((s.as_bytes()[linkIndex as usize] as i8) as LinkState) == LINKSTATE_TL_GREEN_MAJOR {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `std::map<std::string, double> MSTrafficLightLogic::getDetectorStates() const` [const, virtual]
    /// return activation state of all detectors that affect this traffic light
    // virtual std::map<std::string, double> getDetectorStates() const {
    //         return std::map<std::string, double>();
    //     }
    #[allow(unused_variables)]
    pub fn get_detector_states(&self) -> BTreeMap<String, f64> {
        // C++ default: no detectors
        BTreeMap::new()
    }

    /// C++: `double MSTrafficLightLogic::getDetectorState(const std::string) const` [const, virtual]
    /// return activation state of a specific detector that affect this traffic light
    // virtual double getDetectorState(const std::string) const {
    //         return 0.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_detector_state(&self, arg0: String) -> f64 {
        // C++ default: no detectors
        0.0
    }

    /// C++: `double MSTrafficLightLogic::getTLQueueLength(const std::string) const` [const, virtual]
    /// return the estimated queue length at the upcoming traffic light
    // virtual double getTLQueueLength(const std::string) const {
    //         return 0.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_queue_length(&self, arg0: String) -> f64 {
        // C++ default: no detectors
        0.0
    }

    /// C++: `std::map<std::string, double> MSTrafficLightLogic::getConditions() const` [const, virtual]
    /// return all named conditions defined for this traffic light
    // virtual std::map<std::string, double> getConditions() const {
    //         return std::map<std::string, double>();
    //     }
    #[allow(unused_variables)]
    pub fn get_conditions(&self) -> BTreeMap<String, f64> {
        // C++ default: no conditions
        BTreeMap::new()
    }

    /// C++: `VehicleVector MSTrafficLightLogic::getBlockingVehicles(int)` [virtual]
    /// return vehicles that block the intersection/rail signal for vehicles that wish to pass the given linkIndex
    // virtual VehicleVector getBlockingVehicles(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return VehicleVector();
    //     }
    #[allow(unused_variables)]
    pub fn get_blocking_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        // C++ default: no blocking vehicles
        Vec::new()
    }

    /// C++: `VehicleVector MSTrafficLightLogic::getRivalVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and are in conflict with vehicles that wish to pass the given linkIndex
    // virtual VehicleVector getRivalVehicles(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return VehicleVector();
    //     }
    #[allow(unused_variables)]
    pub fn get_rival_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        // C++ default: no rival vehicles
        Vec::new()
    }

    /// C++: `VehicleVector MSTrafficLightLogic::getPriorityVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // virtual VehicleVector getPriorityVehicles(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return VehicleVector();
    //     }
    #[allow(unused_variables)]
    pub fn get_priority_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        // C++ default: no priority vehicles
        Vec::new()
    }

    /// C++: `std::vector<const MSDriveWay *> MSTrafficLightLogic::getBlockingDriveWays(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // virtual std::vector<const MSDriveWay*> getBlockingDriveWays(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return std::vector<const MSDriveWay*>();
    //     }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_ways(&mut self, linkIndex: i32) -> Vec<Ptr<MSDriveWay>> {
        // C++ default: no blocking driveways
        Vec::new()
    }

    /// C++: `std::string MSTrafficLightLogic::getRequestedDriveWay(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // virtual std::string getRequestedDriveWay(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return "";
    //     }
    #[allow(unused_variables)]
    pub fn get_requested_drive_way(&mut self, linkIndex: i32) -> String {
        // C++ default: no requested driveway
        String::new()
    }

    /// C++: `void MSTrafficLightLogic::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the current tls states into the given stream
    // virtual void saveState(OutputDevice& /*out*/) const {}
    #[allow(unused_variables)]
    pub fn save_state(&self, arg0: &mut OutputDevice) {
        // C++ default body is empty; subclasses write their state
    }

    /// C++: `void MSTrafficLightLogic::loadState(MSTLLogicControl &, SUMOTime, int, SUMOTime, SUMOTime, SUMOTime, bool)` [virtual]
    /// restores the tls state
    // void
    // MSTrafficLightLogic::loadState(MSTLLogicControl& tlcontrol, SUMOTime t, int step, SUMOTime spentDuration, SUMOTime nextSwitch, SUMOTime /*timeInCycle*/, bool active) {
    //     myAmActive = active;
    //     const SUMOTime remaining = nextSwitch - t;
    //     changeStepAndDuration(tlcontrol, t, step, remaining);
    //     if (myAmActive) {
    //         setTrafficLightSignals(t - spentDuration);
    //         tlcontrol.get(getID()).switchToLoaded(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, tlcontrol: &mut MSTLLogicControl, t: SUMOTime, step: i32, spentDuration: SUMOTime, nextSwitch: SUMOTime, timeInCycle: SUMOTime, active: bool) {
        self.myAmActive = active;
        let remaining = nextSwitch - t;
        self.change_step_and_duration(tlcontrol, t, step, remaining);
        if self.myAmActive {
            self.set_traffic_light_signals(t - spentDuration);
            let id = self.get_id().to_string();
            tlcontrol.get(&id).switch_to_loaded(Ptr::from_mut(self));
        }
    }

    /// C++: `SUMOTime MSTrafficLightLogic::computeCycleTime(const Phases &)` [protected]
    // SUMOTime
    // MSTrafficLightLogic::computeCycleTime(const Phases& phases) {
    //     SUMOTime result = 0;
    //     for (const MSPhaseDefinition* p : phases) {
    //         result += p->duration;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn compute_cycle_time(&mut self, phases: &MSTrafficLightLogic_Phases) -> SUMOTime {
        let mut result: SUMOTime = 0;
        for p in phases {
            result += p.duration;
        }
        result
    }

    /// C++: `void MSTrafficLightLogic::MSTrafficLightLogic(const MSTrafficLightLogic &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MSTrafficLightLogic) -> Self {
        panic!("MSTrafficLightLogic copy constructor is invalidated")
    }

    /// C++: `MSTrafficLightLogic & MSTrafficLightLogic::operator=(const MSTrafficLightLogic &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSTrafficLightLogic) -> &mut MSTrafficLightLogic {
        panic!("MSTrafficLightLogic assignment operator is invalidated")
    }

}

/// Class realising the switch between the traffic light phases
/// C++ class `MSTrafficLightLogic::SwitchCommand` (microsim/traffic_lights/MSTrafficLightLogic.h)
pub struct MSTrafficLightLogic_SwitchCommand {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    /// The responsible traffic lights control
    pub myTLControl: Ptr<MSTLLogicControl>,
    /// The logic to be executed on a switch
    pub myTLLogic: Ptr<MSTrafficLightLogic>,
    /// Assumed switch time (may change in case of adaptive traffic lights)
    pub myAssumedNextSwitch: SUMOTime,
    /// Information whether this switch command is still valid
    pub myAmValid: bool,
}

impl std::ops::Deref for MSTrafficLightLogic_SwitchCommand {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSTrafficLightLogic_SwitchCommand {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSTrafficLightLogic_SwitchCommand {
    /// C++: `void SwitchCommand::SwitchCommand(MSTLLogicControl &, MSTrafficLightLogic *, SUMOTime)`
    /// Constructor
    // MSTrafficLightLogic::SwitchCommand::SwitchCommand(MSTLLogicControl& tlcontrol,
    //         MSTrafficLightLogic* tlLogic, SUMOTime nextSwitch) :
    //     myTLControl(tlcontrol), myTLLogic(tlLogic),
    //     myAssumedNextSwitch(nextSwitch), myAmValid(true) {
    //     // higher than default command priority of 0
    //     priority = std::numeric_limits<int>::max();
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, tlLogic: Ptr<MSTrafficLightLogic>, nextSwitch: SUMOTime) -> Self {
        // shortcut: C++ sets `priority = max` on the Command base, which is still Unmapped; the event queue therefore orders switch commands by time only, upgrade when Command is ported
        MSTrafficLightLogic_SwitchCommand {
            base: (),
            myTLControl: Ptr::from_mut(tlcontrol),
            myTLLogic: tlLogic,
            myAssumedNextSwitch: nextSwitch,
            myAmValid: true,
        }
    }

    /// C++: `void SwitchCommand::~SwitchCommand() noexcept` [virtual]
    /// Destructor
    // MSTrafficLightLogic::SwitchCommand::~SwitchCommand() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty
    }

    /// C++: `SUMOTime SwitchCommand::execute(SUMOTime)` [virtual]
    /// Executes the regarded junction's "trySwitch"- method
    // SUMOTime
    // MSTrafficLightLogic::SwitchCommand::execute(SUMOTime t) {
    //     // check whether this command has been descheduled
    //     if (!myAmValid) {
    //         return 0;
    //     }
    //     int step1 = myTLLogic->getCurrentPhaseIndex();
    //     SUMOTime next = myTLLogic->trySwitch();
    //     while (next == 0) {
    //         // skip phase and switch again
    //         next = myTLLogic->trySwitch();
    //     }
    //     int step2 = myTLLogic->getCurrentPhaseIndex();
    //     if (step1 != step2) {
    //         if (myTLLogic->isActive()) {
    //             // execute any action connected to this tls
    //             const MSTLLogicControl::TLSLogicVariants& vars = myTLControl.get(myTLLogic->getID());
    //             // set link priorities
    //             myTLLogic->setTrafficLightSignals(t);
    //             // execute switch actions
    //             vars.executeOnSwitchActions();
    //         }
    //     }
    //     myAssumedNextSwitch = t + next;
    //     return next;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        // check whether this command has been descheduled
        if !self.myAmValid {
            return 0;
        }
        let step1 = self.myTLLogic.get_current_phase_index();
        let mut next = self.myTLLogic.m().try_switch();
        while next == 0 {
            // skip phase and switch again
            next = self.myTLLogic.m().try_switch();
        }
        let step2 = self.myTLLogic.get_current_phase_index();
        if step1 != step2 {
            if self.myTLLogic.is_active() {
                // execute any action connected to this tls
                let tlid = self.myTLLogic.get_id().to_string();
                let vars = (*self.myTLControl).get(&tlid);
                // set link priorities
                self.myTLLogic.m().set_traffic_light_signals(currentTime);
                // execute switch actions
                vars.execute_on_switch_actions();
            }
        }
        self.myAssumedNextSwitch = currentTime + next;
        next
    }

    /// C++: `void SwitchCommand::deschedule(MSTrafficLightLogic *)`
    /// Marks this swicth as invalid (if the phase duration has changed, f.e.)
    // void
    // MSTrafficLightLogic::SwitchCommand::deschedule(MSTrafficLightLogic* tlLogic) {
    //     if (tlLogic == myTLLogic) {
    //         myAmValid = false;
    //         myAssumedNextSwitch = -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule(&mut self, tlLogic: Ptr<MSTrafficLightLogic>) {
        if tlLogic == self.myTLLogic {
            self.myAmValid = false;
            self.myAssumedNextSwitch = -1;
        }
    }

    /// C++: `SUMOTime SwitchCommand::getNextSwitchTime() const` [const]
    /// Returns the assumed next switch time
    // SUMOTime getNextSwitchTime() const {
    //             return myAssumedNextSwitch;
    //         }
    #[allow(unused_variables)]
    pub fn get_next_switch_time(&self) -> SUMOTime {
        self.myAssumedNextSwitch
    }

    /// C++: `SUMOTime SwitchCommand::shiftTime(SUMOTime, SUMOTime, SUMOTime)` [virtual]
    /// Reschedule or deschedule the command when quick-loading state
    // SUMOTime
    // MSTrafficLightLogic::SwitchCommand::shiftTime(SUMOTime currentTime, SUMOTime execTime, SUMOTime newTime) {
    //     if (myTLLogic->getDefaultCycleTime() == DELTA_T) {
    //         // MSRailSignal
    //         return newTime;
    //     } else {
    //         UNUSED_PARAMETER(currentTime);
    //         UNUSED_PARAMETER(execTime);
    //         // XXX changeStepAndDuration (computed as in NLJunctionControlBuilder::closeTrafficLightLogic
    //         return newTime;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn shift_time(&mut self, currentTime: SUMOTime, execTime: SUMOTime, newTime: SUMOTime) -> SUMOTime {
        // both C++ branches (MSRailSignal and the XXX fallback) return newTime
        newTime
    }

    /// C++: `void SwitchCommand::SwitchCommand(const SwitchCommand &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTrafficLightLogic_SwitchCommand) -> Self {
        panic!("SwitchCommand copy constructor is invalidated")
    }

    /// C++: `SwitchCommand & SwitchCommand::operator=(const SwitchCommand &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTrafficLightLogic_SwitchCommand) -> &mut MSTrafficLightLogic_SwitchCommand {
        panic!("SwitchCommand assignment operator is invalidated")
    }

}

