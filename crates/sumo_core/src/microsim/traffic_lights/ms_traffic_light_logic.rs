//! Port of microsim/traffic_lights/MSTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<MSLane *> MSTrafficLightLogic::LaneVector`
pub type MSTrafficLightLogic_LaneVector = Vec<Ptr<MSLane>>;

/// C++ typedef `std::vector<LaneVector> MSTrafficLightLogic::LaneVectorVector`
pub type MSTrafficLightLogic_LaneVectorVector = Vec<MSTrafficLightLogic_LaneVector>;

/// C++ typedef `std::vector<MSLink *> MSTrafficLightLogic::LinkVector`
pub type MSTrafficLightLogic_LinkVector = Vec<Ptr<MSLink>>;

/// C++ typedef `std::vector<LinkVector> MSTrafficLightLogic::LinkVectorVector`
pub type MSTrafficLightLogic_LinkVectorVector = Vec<MSTrafficLightLogic_LinkVector>;

/// C++ typedef `std::vector<MSPhaseDefinition *> MSTrafficLightLogic::Phases`
pub type MSTrafficLightLogic_Phases = Vec<Ptr<MSPhaseDefinition>>;

/// C++ typedef `std::vector<const SUMOVehicle *> MSTrafficLightLogic::VehicleVector`
pub type MSTrafficLightLogic_VehicleVector = Vec<Ptr<SUMOVehicle>>;

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
        todo!("port MSTrafficLightLogic::MSTrafficLightLogic")
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
        todo!("port MSTrafficLightLogic::init")
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
        todo!("port MSTrafficLightLogic::initMesoTLSPenalties")
    }

    /// C++: `void MSTrafficLightLogic::~MSTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSTrafficLightLogic::~MSTrafficLightLogic() {
    //     // no need to do something about mySwitchCommand here,
    //     // it is handled by the event control
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTrafficLightLogic::~MSTrafficLightLogic")
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
        todo!("port MSTrafficLightLogic::addLink")
    }

    /// C++: `void MSTrafficLightLogic::ignoreLinkIndex(int)`
    /// ignore pedestrian crossing index in mesosim
    // void
    // MSTrafficLightLogic::ignoreLinkIndex(int pos) {
    //     myIgnoredIndices.insert(pos);
    // }
    #[allow(unused_variables)]
    pub fn ignore_link_index(&mut self, pos: i32) {
        todo!("port MSTrafficLightLogic::ignoreLinkIndex")
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
        todo!("port MSTrafficLightLogic::adaptLinkInformationFrom")
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
        todo!("port MSTrafficLightLogic::collectLinkStates")
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
        todo!("port MSTrafficLightLogic::resetLinkStates")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::trySwitch()` [virtual, pure virtual]
    /// Switches to the next phase
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        todo!("port MSTrafficLightLogic::trySwitch")
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
        todo!("port MSTrafficLightLogic::activateProgram")
    }

    /// C++: `void MSTrafficLightLogic::deactivateProgram()` [virtual]
    // void
    // MSTrafficLightLogic::deactivateProgram() {
    //     myAmActive = false;
    // }
    #[allow(unused_variables)]
    pub fn deactivate_program(&mut self) {
        todo!("port MSTrafficLightLogic::deactivateProgram")
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
        todo!("port MSTrafficLightLogic::setTrafficLightSignals")
    }

    /// C++: `const std::string & MSTrafficLightLogic::getProgramID() const` [const]
    /// Returns this tl-logic's id
    // const std::string& getProgramID() const {
    //         return myProgramID;
    //     }
    #[allow(unused_variables)]
    pub fn get_program_id(&self) -> &str {
        todo!("port MSTrafficLightLogic::getProgramID")
    }

    /// C++: `const LaneVectorVector & MSTrafficLightLogic::getLaneVectors() const` [const]
    /// Returns the list of lists of all lanes controlled by this tls
    // const LaneVectorVector& getLaneVectors() const {
    //         return myLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_vectors(&self) -> &MSTrafficLightLogic_LaneVectorVector {
        todo!("port MSTrafficLightLogic::getLaneVectors")
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
    pub fn get_lanes_at(&self, i: i32) -> &MSTrafficLightLogic_LaneVector {
        todo!("port MSTrafficLightLogic::getLanesAt")
    }

    /// C++: `const LinkVectorVector & MSTrafficLightLogic::getLinks() const` [const]
    /// Returns the list of lists of all affected links
    // const LinkVectorVector& getLinks() const {
    //         return myLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_links(&self) -> &MSTrafficLightLogic_LinkVectorVector {
        todo!("port MSTrafficLightLogic::getLinks")
    }

    /// C++: `const LinkVector & MSTrafficLightLogic::getLinksAt(int) const` [const]
    /// Returns the list of links that are controlled by the signals at the given position
    // const LinkVector& getLinksAt(int i) const {
    //         return myLinks[i];
    //     }
    #[allow(unused_variables)]
    pub fn get_links_at(&self, i: i32) -> &MSTrafficLightLogic_LinkVector {
        todo!("port MSTrafficLightLogic::getLinksAt")
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
        todo!("port MSTrafficLightLogic::getLinkIndex")
    }

    /// C++: `int MSTrafficLightLogic::getPhaseNumber() const` [const, virtual, pure virtual]
    /// Returns the number of phases
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phase_number(&self) -> i32 {
        todo!("port MSTrafficLightLogic::getPhaseNumber")
    }

    /// C++: `const Phases & MSTrafficLightLogic::getPhases() const` [const, virtual, pure virtual]
    /// Returns the phases of this tls program
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &MSTrafficLightLogic_Phases {
        todo!("port MSTrafficLightLogic::getPhases")
    }

    /// C++: `const MSPhaseDefinition & MSTrafficLightLogic::getPhase(int) const` [const, virtual, pure virtual]
    /// Returns the definition of the phase from the given position within the plan
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phase(&self, givenstep: i32) -> &MSPhaseDefinition {
        todo!("port MSTrafficLightLogic::getPhase")
    }

    /// C++: `TrafficLightType MSTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic
    // TrafficLightType getLogicType() const {
    //         return myLogicType;
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> TrafficLightType {
        todo!("port MSTrafficLightLogic::getLogicType")
    }

    /// C++: `int MSTrafficLightLogic::getCurrentPhaseIndex() const` [const, virtual, pure virtual]
    /// Returns the current index within the program
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_phase_index(&self) -> i32 {
        todo!("port MSTrafficLightLogic::getCurrentPhaseIndex")
    }

    /// C++: `const MSPhaseDefinition & MSTrafficLightLogic::getCurrentPhaseDef() const` [const, virtual, pure virtual]
    /// Returns the definition of the current phase
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        todo!("port MSTrafficLightLogic::getCurrentPhaseDef")
    }

    /// C++: `void MSTrafficLightLogic::resetLastSwitch(SUMOTime)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        todo!("port MSTrafficLightLogic::resetLastSwitch")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getMinDur(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getMinDur(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.minDuration;
    // }
    #[allow(unused_variables)]
    pub fn get_min_dur(&self, step: i32) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getMinDur")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getMaxDur(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getMaxDur(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.maxDuration;
    // }
    #[allow(unused_variables)]
    pub fn get_max_dur(&self, step: i32) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getMaxDur")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getEarliestEnd(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getEarliestEnd(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.earliestEnd;
    // }
    #[allow(unused_variables)]
    pub fn get_earliest_end(&self, step: i32) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getEarliestEnd")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getLatestEnd(int) const` [const, virtual]
    // SUMOTime
    // MSTrafficLightLogic::getLatestEnd(int step) const {
    //     const MSPhaseDefinition& p = step < 0 ? getCurrentPhaseDef() : getPhase(step);
    //     return p.latestEnd;
    // }
    #[allow(unused_variables)]
    pub fn get_latest_end(&self, step: i32) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getLatestEnd")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getDefaultCycleTime() const` [const]
    /// Returns the cycle time (in ms)
    // SUMOTime getDefaultCycleTime() const {
    //         return myDefaultCycleTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_cycle_time(&self) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getDefaultCycleTime")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getTimeInCycle() const` [const]
    /// return time within the current cycle
    // SUMOTime
    // MSTrafficLightLogic::getTimeInCycle() const {
    //     return mapTimeInCycle(SIMSTEP);
    // }
    #[allow(unused_variables)]
    pub fn get_time_in_cycle(&self) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getTimeInCycle")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::mapTimeInCycle(SUMOTime) const` [const, virtual]
    /// map the given time into the current cycle
    // SUMOTime
    // MSTrafficLightLogic::mapTimeInCycle(SUMOTime t) const {
    //     return (t - myOffset) % myDefaultCycleTime;
    // }
    #[allow(unused_variables)]
    pub fn map_time_in_cycle(&self, t: SUMOTime) -> SUMOTime {
        todo!("port MSTrafficLightLogic::mapTimeInCycle")
    }

    /// C++: `int MSTrafficLightLogic::getNumLinks() const` [const]
    /// return the number of controlled link indices
    // int getNumLinks() const {
    //         return myNumLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_num_links(&self) -> i32 {
        todo!("port MSTrafficLightLogic::getNumLinks")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getNextSwitchTime() const` [const]
    /// Returns the assumed next switch time
    // SUMOTime
    // MSTrafficLightLogic::getNextSwitchTime() const {
    //     return mySwitchCommand != nullptr ? mySwitchCommand->getNextSwitchTime() : -1;
    // }
    #[allow(unused_variables)]
    pub fn get_next_switch_time(&self) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getNextSwitchTime")
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
        todo!("port MSTrafficLightLogic::getSpentDuration")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getPhaseIndexAtTime(SUMOTime) const` [const, virtual, pure virtual]
    /// Returns the index of the logic at the given simulation step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_phase_index_at_time(&self, simStep: SUMOTime) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getPhaseIndexAtTime")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getOffsetFromIndex(int) const` [const, virtual, pure virtual]
    /// Returns the position (start of a phase during a cycle) from of a given step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_offset_from_index(&self, index: i32) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getOffsetFromIndex")
    }

    /// C++: `int MSTrafficLightLogic::getIndexFromOffset(SUMOTime) const` [const, virtual, pure virtual]
    /// Returns the step (the phasenumber) of a given position of the cycle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_index_from_offset(&self, offset: SUMOTime) -> i32 {
        todo!("port MSTrafficLightLogic::getIndexFromOffset")
    }

    /// C++: `SUMOTime MSTrafficLightLogic::getOffset() const` [const]
    /// @}
    // SUMOTime getOffset() const {
    //         return myOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_offset(&self) -> SUMOTime {
        todo!("port MSTrafficLightLogic::getOffset")
    }

    /// C++: `void MSTrafficLightLogic::addOverridingDuration(SUMOTime)`
    /// Changes the duration of the next phase
    // void
    // MSTrafficLightLogic::addOverridingDuration(SUMOTime duration) {
    //     myOverridingTimes.push_back(duration);
    // }
    #[allow(unused_variables)]
    pub fn add_overriding_duration(&mut self, duration: SUMOTime) {
        todo!("port MSTrafficLightLogic::addOverridingDuration")
    }

    /// C++: `void MSTrafficLightLogic::setCurrentDurationIncrement(SUMOTime)`
    /// Delays current phase by the given delay
    // void
    // MSTrafficLightLogic::setCurrentDurationIncrement(SUMOTime delay) {
    //     myCurrentDurationIncrement = delay;
    // }
    #[allow(unused_variables)]
    pub fn set_current_duration_increment(&mut self, delay: SUMOTime) {
        todo!("port MSTrafficLightLogic::setCurrentDurationIncrement")
    }

    /// C++: `void MSTrafficLightLogic::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual, pure virtual]
    /// Changes the current phase and her duration
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn change_step_and_duration(&mut self, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
        todo!("port MSTrafficLightLogic::changeStepAndDuration")
    }

    /// C++: `bool MSTrafficLightLogic::isSelected() const` [const]
    /// whether this logic is selected in the GUI
    // bool
    // MSTrafficLightLogic::isSelected() const {
    //     return MSNet::getInstance()->isSelected(this);
    // }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        todo!("port MSTrafficLightLogic::isSelected")
    }

    /// C++: `bool MSTrafficLightLogic::isActive() const` [const]
    /// whether this logic is the active program
    // bool isActive() const {
    //         return myAmActive;
    //     }
    #[allow(unused_variables)]
    pub fn is_active(&self) -> bool {
        todo!("port MSTrafficLightLogic::isActive")
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
        todo!("port MSTrafficLightLogic::getsMajorGreen")
    }

    /// C++: `std::map<std::string, double> MSTrafficLightLogic::getDetectorStates() const` [const, virtual]
    /// return activation state of all detectors that affect this traffic light
    // virtual std::map<std::string, double> getDetectorStates() const {
    //         return std::map<std::string, double>();
    //     }
    #[allow(unused_variables)]
    pub fn get_detector_states(&self) -> BTreeMap<String, f64> {
        todo!("port MSTrafficLightLogic::getDetectorStates")
    }

    /// C++: `double MSTrafficLightLogic::getDetectorState(const std::string) const` [const, virtual]
    /// return activation state of a specific detector that affect this traffic light
    // virtual double getDetectorState(const std::string) const {
    //         return 0.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_detector_state(&self, arg0: String) -> f64 {
        todo!("port MSTrafficLightLogic::getDetectorState")
    }

    /// C++: `double MSTrafficLightLogic::getTLQueueLength(const std::string) const` [const, virtual]
    /// return the estimated queue length at the upcoming traffic light
    // virtual double getTLQueueLength(const std::string) const {
    //         return 0.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_queue_length(&self, arg0: String) -> f64 {
        todo!("port MSTrafficLightLogic::getTLQueueLength")
    }

    /// C++: `std::map<std::string, double> MSTrafficLightLogic::getConditions() const` [const, virtual]
    /// return all named conditions defined for this traffic light
    // virtual std::map<std::string, double> getConditions() const {
    //         return std::map<std::string, double>();
    //     }
    #[allow(unused_variables)]
    pub fn get_conditions(&self) -> BTreeMap<String, f64> {
        todo!("port MSTrafficLightLogic::getConditions")
    }

    /// C++: `VehicleVector MSTrafficLightLogic::getBlockingVehicles(int)` [virtual]
    /// return vehicles that block the intersection/rail signal for vehicles that wish to pass the given linkIndex
    // virtual VehicleVector getBlockingVehicles(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return VehicleVector();
    //     }
    #[allow(unused_variables)]
    pub fn get_blocking_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        todo!("port MSTrafficLightLogic::getBlockingVehicles")
    }

    /// C++: `VehicleVector MSTrafficLightLogic::getRivalVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and are in conflict with vehicles that wish to pass the given linkIndex
    // virtual VehicleVector getRivalVehicles(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return VehicleVector();
    //     }
    #[allow(unused_variables)]
    pub fn get_rival_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        todo!("port MSTrafficLightLogic::getRivalVehicles")
    }

    /// C++: `VehicleVector MSTrafficLightLogic::getPriorityVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // virtual VehicleVector getPriorityVehicles(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return VehicleVector();
    //     }
    #[allow(unused_variables)]
    pub fn get_priority_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        todo!("port MSTrafficLightLogic::getPriorityVehicles")
    }

    /// C++: `std::vector<const MSDriveWay *> MSTrafficLightLogic::getBlockingDriveWays(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // virtual std::vector<const MSDriveWay*> getBlockingDriveWays(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return std::vector<const MSDriveWay*>();
    //     }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_ways(&mut self, linkIndex: i32) -> Vec<Ptr<MSDriveWay>> {
        todo!("port MSTrafficLightLogic::getBlockingDriveWays")
    }

    /// C++: `std::string MSTrafficLightLogic::getRequestedDriveWay(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // virtual std::string getRequestedDriveWay(int linkIndex) {
    //         UNUSED_PARAMETER(linkIndex);
    //         return "";
    //     }
    #[allow(unused_variables)]
    pub fn get_requested_drive_way(&mut self, linkIndex: i32) -> String {
        todo!("port MSTrafficLightLogic::getRequestedDriveWay")
    }

    /// C++: `void MSTrafficLightLogic::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the current tls states into the given stream
    // virtual void saveState(OutputDevice& /*out*/) const {}
    #[allow(unused_variables)]
    pub fn save_state(&self, arg0: &mut OutputDevice) {
        todo!("port MSTrafficLightLogic::saveState")
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
        todo!("port MSTrafficLightLogic::loadState")
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
        todo!("port MSTrafficLightLogic::computeCycleTime")
    }

    /// C++: `void MSTrafficLightLogic::MSTrafficLightLogic(const MSTrafficLightLogic &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MSTrafficLightLogic) -> Self {
        todo!("port MSTrafficLightLogic::MSTrafficLightLogic")
    }

    /// C++: `MSTrafficLightLogic & MSTrafficLightLogic::operator=(const MSTrafficLightLogic &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSTrafficLightLogic) -> &mut MSTrafficLightLogic {
        todo!("port MSTrafficLightLogic::operator=")
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
        todo!("port SwitchCommand::SwitchCommand")
    }

    /// C++: `void SwitchCommand::~SwitchCommand() noexcept` [virtual]
    /// Destructor
    // MSTrafficLightLogic::SwitchCommand::~SwitchCommand() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SwitchCommand::~SwitchCommand")
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
        todo!("port SwitchCommand::execute")
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
        todo!("port SwitchCommand::deschedule")
    }

    /// C++: `SUMOTime SwitchCommand::getNextSwitchTime() const` [const]
    /// Returns the assumed next switch time
    // SUMOTime getNextSwitchTime() const {
    //             return myAssumedNextSwitch;
    //         }
    #[allow(unused_variables)]
    pub fn get_next_switch_time(&self) -> SUMOTime {
        todo!("port SwitchCommand::getNextSwitchTime")
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
        todo!("port SwitchCommand::shiftTime")
    }

    /// C++: `void SwitchCommand::SwitchCommand(const SwitchCommand &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTrafficLightLogic_SwitchCommand) -> Self {
        todo!("port SwitchCommand::SwitchCommand")
    }

    /// C++: `SwitchCommand & SwitchCommand::operator=(const SwitchCommand &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTrafficLightLogic_SwitchCommand) -> &mut MSTrafficLightLogic_SwitchCommand {
        todo!("port SwitchCommand::operator=")
    }

}

