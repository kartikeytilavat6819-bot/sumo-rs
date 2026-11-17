//! Port of microsim/traffic_lights/MSRailCrossing.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

fn rail_crossing_init(l: &mut MSTrafficLightLogic, nb: &mut NLDetectorBuilder) {
    Ptr::from_mut(l).cast::<MSRailCrossing>().m().init(nb)
}
fn rail_crossing_add_link(l: &mut MSTrafficLightLogic, link: Ptr<MSLink>, lane: Ptr<MSLane>, pos: i32) {
    Ptr::from_mut(l).cast::<MSRailCrossing>().m().add_link(link, lane, pos)
}
fn rail_crossing_adapt_link_information_from(l: &mut MSTrafficLightLogic, logic: &MSTrafficLightLogic) {
    Ptr::from_mut(l).cast::<MSRailCrossing>().m().adapt_link_information_from(logic)
}
fn rail_crossing_try_switch(l: &mut MSTrafficLightLogic) -> SUMOTime {
    Ptr::from_mut(l).cast::<MSRailCrossing>().m().try_switch()
}
fn rail_crossing_get_phase_index_at_time(l: &MSTrafficLightLogic, simStep: SUMOTime) -> SUMOTime {
    Ptr::new(l).cast::<MSRailCrossing>().get().unwrap().get_phase_index_at_time(simStep)
}
fn rail_crossing_get_offset_from_index(l: &MSTrafficLightLogic, index: i32) -> SUMOTime {
    Ptr::new(l).cast::<MSRailCrossing>().get().unwrap().get_offset_from_index(index)
}
fn rail_crossing_get_index_from_offset(l: &MSTrafficLightLogic, offset: SUMOTime) -> i32 {
    Ptr::new(l).cast::<MSRailCrossing>().get().unwrap().get_index_from_offset(offset)
}
fn rail_crossing_change_step_and_duration(l: &mut MSTrafficLightLogic, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
    Ptr::from_mut(l).cast::<MSRailCrossing>().m().change_step_and_duration(tlcontrol, simStep, step, stepDuration)
}

/// Virtual table of MSRailCrossing (overrides init, addLink, adaptLinkInformationFrom, trySwitch,
/// getPhaseIndexAtTime, getOffsetFromIndex, getIndexFromOffset, changeStepAndDuration)
static MS_RAIL_CROSSING_TRAFFIC_LIGHT_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    try_switch: rail_crossing_try_switch,
    get_phase_index_at_time: rail_crossing_get_phase_index_at_time,
    get_offset_from_index: rail_crossing_get_offset_from_index,
    get_index_from_offset: rail_crossing_get_index_from_offset,
    change_step_and_duration: rail_crossing_change_step_and_duration,
    add_link: rail_crossing_add_link,
    init: rail_crossing_init,
    adapt_link_information_from: rail_crossing_adapt_link_information_from,
    ..MS_SIMPLE_TRAFFIC_LIGHT_LOGIC_VT
};

/// A signal for rails
/// C++ class `MSRailCrossing` (microsim/traffic_lights/MSRailCrossing.h)
#[repr(C)]
pub struct MSRailCrossing {
    /// C++ base class `MSSimpleTrafficLightLogic`
    pub base: MSSimpleTrafficLightLogic,
    /// The incoming rail links
    pub myIncomingRailLinks: Vec<Ptr<MSLink>>,
    /// minimum time gap between closing the crossing (end of yellow time) and train passing the crossing
    pub myTimeGap: SUMOTime,
    /// minimum distance between the train and the crossing which triggers closing (-1 means time only)
    pub mySpaceGap: f64,
    /// minimum green time
    pub myMinGreenTime: SUMOTime,
    /// red time after the train has left
    pub myOpeningDelay: SUMOTime,
    /// red-yellow time after the delay while opening
    pub myOpeningTime: SUMOTime,
    /// yellow time
    pub myYellowTime: SUMOTime,
}

impl std::ops::Deref for MSRailCrossing {
    type Target = MSSimpleTrafficLightLogic;
    fn deref(&self) -> &MSSimpleTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailCrossing {
    fn deref_mut(&mut self) -> &mut MSSimpleTrafficLightLogic {
        &mut self.base
    }
}

impl MSRailCrossing {
    /// C++: `void MSRailCrossing::MSRailCrossing(MSTLLogicControl &, const std::string &, const std::string &, SUMOTime, const Parameterised::Map &)`
    /// Constructor
    // MSRailCrossing::MSRailCrossing(MSTLLogicControl& tlcontrol,
    //                                const std::string& id, const std::string& programID, SUMOTime delay,
    //                                const Parameterised::Map& parameters) :
    //     MSSimpleTrafficLightLogic(tlcontrol, id, programID, 0, TrafficLightType::RAIL_CROSSING, Phases(), 0, delay, parameters) {
    //     // dummy phase, used to avoid crashing in MSTrafficLightLogic::setTrafficLightSignals()
    //     myPhases.push_back(new MSPhaseDefinition(1, std::string(SUMO_MAX_CONNECTIONS, 'X')));
    //     myDefaultCycleTime = 1;
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        let mut base = MSSimpleTrafficLightLogic::new(tlcontrol, id, programID, 0, TrafficLightType::RAIL_CROSSING, &Vec::new(), 0, delay, parameters);
        base.base.vt = &MS_RAIL_CROSSING_TRAFFIC_LIGHT_LOGIC_VT;
        // dummy phase, used to avoid crashing in MSTrafficLightLogic::setTrafficLightSignals()
        base.myPhases.push(Ptr::from_raw(Box::into_raw(Box::new(MSPhaseDefinition::new(1, &"X".repeat(utils::common::std_defs::SUMO_MAX_CONNECTIONS), "")))));
        base.myDefaultCycleTime = 1;
        Self {
            base,
            myIncomingRailLinks: Vec::new(),
            myTimeGap: 0,
            mySpaceGap: 0.,
            myMinGreenTime: 0,
            myOpeningDelay: 0,
            myOpeningTime: 0,
            myYellowTime: 0,
        }
    }

    /// C++: `void MSRailCrossing::~MSRailCrossing() noexcept` [virtual]
    /// Destructor
    // MSRailCrossing::~MSRailCrossing() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the base class owns the phases
    }

    /// C++: `void MSRailCrossing::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the rail signal with information about adjacent rail signals
    // void
    // MSRailCrossing::init(NLDetectorBuilder&) {
    //     const Parameterised::Map test = getParametersMap();
    //     myTimeGap = string2time(getParameter("time-gap", "15"));
    //     //use time-gap by default
    //     mySpaceGap = StringUtils::toDouble(getParameter("space-gap", "-1"));
    //     myMinGreenTime = string2time(getParameter("min-green", "5"));
    //     myOpeningDelay = string2time(getParameter("opening-delay", "3"));
    //     myOpeningTime = string2time(getParameter("opening-time", "3")); // red-yellow while opening
    //     /// XXX compute reasonable time depending on link length
    //     myYellowTime = string2time(getParameter("yellow-time", "5"));
    //     delete myPhases.front();
    //     myPhases.clear();
    //     myPhases.push_back(new MSPhaseDefinition(1, std::string(myLinks.size(), LINKSTATE_TL_GREEN_MAJOR)));
    //     myPhases.push_back(new MSPhaseDefinition(myYellowTime, std::string(myLinks.size(), LINKSTATE_TL_YELLOW_MINOR)));
    //     myPhases.push_back(new MSPhaseDefinition(1, std::string(myLinks.size(), LINKSTATE_TL_RED)));
    //     myPhases.push_back(new MSPhaseDefinition(myOpeningTime, std::string(myLinks.size(), LINKSTATE_TL_REDYELLOW)));
    //     // init phases
    //     updateCurrentPhase();
    //     setTrafficLightSignals(MSNet::getInstance()->getCurrentTimeStep());
    //     myNumLinks = (int)myLinks.size();
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        let _test = self.base.base.base2.get_parameters_map();
        self.myTimeGap = string2time(&self.base.base.base2.get_parameter("time-gap", "15")).unwrap();
        // use time-gap by default
        self.mySpaceGap = utils::common::string_utils::to_double(&self.base.base.base2.get_parameter("space-gap", "-1")).unwrap();
        self.myMinGreenTime = string2time(&self.base.base.base2.get_parameter("min-green", "5")).unwrap();
        self.myOpeningDelay = string2time(&self.base.base.base2.get_parameter("opening-delay", "3")).unwrap();
        self.myOpeningTime = string2time(&self.base.base.base2.get_parameter("opening-time", "3")).unwrap(); // red-yellow while opening
        // XXX compute reasonable time depending on link length
        self.myYellowTime = string2time(&self.base.base.base2.get_parameter("yellow-time", "5")).unwrap();
        // shortcut: C++ throws ProcessError on a malformed time/double here; unwrap panics instead, upgrade when init() can return Result
        unsafe { drop(Box::from_raw(self.myPhases[0].as_raw())) };
        self.myPhases.clear();
        let n = self.myLinks.len();
        let mk = |d: SUMOTime, s: String| Ptr::from_raw(Box::into_raw(Box::new(MSPhaseDefinition::new(d, &s, ""))));
        self.myPhases.push(mk(1, (LINKSTATE_TL_GREEN_MAJOR as u8 as char).to_string().repeat(n)));
        let yellowTime = self.myYellowTime;
self.myPhases.push(mk(yellowTime, (LINKSTATE_TL_YELLOW_MINOR as u8 as char).to_string().repeat(n)));
        self.myPhases.push(mk(1, (LINKSTATE_TL_RED as u8 as char).to_string().repeat(n)));
        let openingTime = self.myOpeningTime;
self.myPhases.push(mk(openingTime, (LINKSTATE_TL_REDYELLOW as u8 as char).to_string().repeat(n)));
        // init phases
        self.update_current_phase();
        let now = MSNet::get_instance().get_current_time_step();
        self.set_traffic_light_signals(now);
        self.myNumLinks = self.myLinks.len() as i32;
    }

    /// C++: `void MSRailCrossing::setParameter(const std::string &, const std::string &)` [virtual]
    /// Sets a parameter and updates internal constants
    // void
    // MSRailCrossing::setParameter(const std::string& key, const std::string& value) {
    //     // some pre-defined parameters can be updated at runtime
    //     if (key == "time-gap") {
    //         myTimeGap = string2time(value);
    //     } else if (key == "space-gap") {
    //         mySpaceGap = StringUtils::toDouble(value);
    //     } else if (key == "min-green") {
    //         myMinGreenTime = string2time(value);
    //     } else if (key == "opening-delay") {
    //         myOpeningDelay = string2time(value);
    //     } else if (key == "opening-time") {
    //         myOpeningTime = string2time(value); // TODO update phases
    //     } else if (key == "yellow-time") {
    //         myYellowTime = string2time(value); // TODO update phases
    //     }
    //     Parameterised::setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        // some pre-defined parameters can be updated at runtime
        if key == "time-gap" {
            self.myTimeGap = string2time(value).unwrap();
        } else if key == "space-gap" {
            self.mySpaceGap = utils::common::string_utils::to_double(value).unwrap();
        } else if key == "min-green" {
            self.myMinGreenTime = string2time(value).unwrap();
        } else if key == "opening-delay" {
            self.myOpeningDelay = string2time(value).unwrap();
        } else if key == "opening-time" {
            self.myOpeningTime = string2time(value).unwrap(); // TODO update phases
        } else if key == "yellow-time" {
            self.myYellowTime = string2time(value).unwrap(); // TODO update phases
        }
        self.base.base.base2.set_parameter(key, value);
    }

    /// C++: `void MSRailCrossing::addLink(MSLink *, MSLane *, int)` [virtual]
    /// Adds a link on building
    // void
    // MSRailCrossing::addLink(MSLink* link, MSLane* lane, int pos) {
    //     if (pos >= 0) {
    //         MSTrafficLightLogic::addLink(link, lane, pos);
    //     } else {
    //         myIncomingRailLinks.push_back(link);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<MSLink>, lane: Ptr<MSLane>, pos: i32) {
        if pos >= 0 {
            self.base.add_link(link, lane, pos);
        } else {
            self.myIncomingRailLinks.push(link);
        }
    }

    /// C++: `void MSRailCrossing::adaptLinkInformationFrom(const MSTrafficLightLogic &)` [virtual]
    /// Applies information about controlled links and lanes from the given logic
    // void
    // MSRailCrossing::adaptLinkInformationFrom(const MSTrafficLightLogic& logic) {
    //     MSTrafficLightLogic::adaptLinkInformationFrom(logic);
    //     updateCurrentPhase();
    // }
    #[allow(unused_variables)]
    pub fn adapt_link_information_from(&mut self, logic: &MSTrafficLightLogic) {
        self.base.adapt_link_information_from(logic);
        self.update_current_phase();
    }

    /// C++: `SUMOTime MSRailCrossing::updateCurrentPhase()`
    /// updates the current phase of the signal
    // SUMOTime
    // MSRailCrossing::updateCurrentPhase() {
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     SUMOTime stayRedUntil = now;
    //     // check rail links for approaching foes to determine whether and how long
    //     // the crossing must remain closed
    //     for (const MSLink* const link : myIncomingRailLinks) {
    //         for (const auto& it_avi : link->getApproaching()) {
    //             const MSLink::ApproachingVehicleInformation& avi = it_avi.second;
    //             if (avi.arrivalTime - myYellowTime - now < myTimeGap) {
    //                 stayRedUntil = MAX2(stayRedUntil, avi.leavingTime + myOpeningDelay);
    //             }
    //             if (mySpaceGap >= 0 && avi.dist < mySpaceGap) {
    //                 // TODO maybe check the incoming lanes because stopped vehicles do not register at the oncoming junction
    //                 stayRedUntil = MAX2(stayRedUntil, avi.leavingTime + myOpeningDelay);
    //             }
    //         }
    //         if (link->getViaLane() != nullptr && link->getViaLane()->getVehicleNumberWithPartials() > 0) {
    //             // do not open if there is still a train on the crossing
    //             stayRedUntil = MAX2(stayRedUntil, now + DELTA_T + myOpeningDelay);
    //         }
    //     }
    //     //if (getID() == "cluster_1088529493_1260626727") std::cout << SIMTIME << " stayRedUntil=" << stayRedUntil;
    //     const SUMOTime wait = stayRedUntil - now;
    //
    //     if (myStep == 0) {
    //         // 'G': check whether the crossing can stay open
    //         if (wait == 0) {
    //             return DELTA_T;
    //         } else {
    //             myStep++;
    //             return myYellowTime;
    //         }
    //     } else if (myStep == 1) {
    //         // 'y': yellow time is over. switch to red
    //         myStep++;
    //         return MAX2(DELTA_T, wait);
    //     } else if (myStep == 2) {
    //         // 'r': check whether we may open again
    //         if (wait == 0) {
    //             myStep++;
    //             return myOpeningTime;
    //         } else {
    //             return wait;
    //         }
    //     } else { // (myStep == 3)
    //         // 'u': opening time is over, switch to green
    //         if (wait == 0) {
    //             myStep = 0;
    //             return myMinGreenTime;
    //         } else {
    //             // train approached during opening sequence, close again
    //             myStep = 2;
    //             return wait;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_current_phase(&mut self) -> SUMOTime {
        let now = MSNet::get_instance().get_current_time_step();
        let mut stayRedUntil = now;
        // check rail links for approaching foes to determine whether and how long
        // the crossing must remain closed
        for link in self.myIncomingRailLinks.iter() {
            for avi in link.get_approaching_2().values() {
                if avi.arrivalTime - self.myYellowTime - now < self.myTimeGap {
                    stayRedUntil = std::cmp::max(stayRedUntil, avi.leavingTime + self.myOpeningDelay);
                }
                if self.mySpaceGap >= 0. && avi.dist < self.mySpaceGap {
                    // TODO maybe check the incoming lanes because stopped vehicles do not register at the oncoming junction
                    stayRedUntil = std::cmp::max(stayRedUntil, avi.leavingTime + self.myOpeningDelay);
                }
            }
            if !link.get_via_lane().is_null() && link.get_via_lane().get_vehicle_number_with_partials() > 0 {
                // do not open if there is still a train on the crossing
                stayRedUntil = std::cmp::max(stayRedUntil, now + delta_t() + self.myOpeningDelay);
            }
        }
        let wait = stayRedUntil - now;

        if self.myStep == 0 {
            // 'G': check whether the crossing can stay open
            if wait == 0 {
                return delta_t();
            } else {
                self.myStep += 1;
                return self.myYellowTime;
            }
        } else if self.myStep == 1 {
            // 'y': yellow time is over. switch to red
            self.myStep += 1;
            return std::cmp::max(delta_t(), wait);
        } else if self.myStep == 2 {
            // 'r': check whether we may open again
            if wait == 0 {
                self.myStep += 1;
                return self.myOpeningTime;
            } else {
                return wait;
            }
        } else {
            // (myStep == 3) 'u': opening time is over, switch to green
            if wait == 0 {
                self.myStep = 0;
                return self.myMinGreenTime;
            } else {
                // train approached during opening sequence, close again
                self.myStep = 2;
                return wait;
            }
        }
    }

    /// C++: `SUMOTime MSRailCrossing::trySwitch()` [virtual]
    /// Switches to the next phase
    // SUMOTime
    // MSRailCrossing::trySwitch() {
    //     const int oldStep = myStep;
    //     SUMOTime nextTry = updateCurrentPhase();
    //     //if (getID() == "cluster_1088529493_1260626727") std::cout << " myStep=" << myStep << " nextTry=" << nextTry << "\n";
    //     if (myStep != oldStep) {
    //         myPhases[myStep]->myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    //     return nextTry;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        let oldStep = self.myStep;
        let nextTry = self.update_current_phase();
        if self.myStep != oldStep {
            self.myPhases[self.myStep as usize].m().myLastSwitch = MSNet::get_instance().get_current_time_step();
        }
        nextTry
    }

    /// C++: `SUMOTime MSRailCrossing::getPhaseIndexAtTime(SUMOTime) const` [const, virtual]
    /// Returns the index of the logic at the given simulation step
    // SUMOTime
    // MSRailCrossing::getPhaseIndexAtTime(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_index_at_time(&self, simStep: SUMOTime) -> SUMOTime {
        0
    }

    /// C++: `SUMOTime MSRailCrossing::getOffsetFromIndex(int) const` [const, virtual]
    /// Returns the position (start of a phase during a cycle) from of a given step
    // SUMOTime
    // MSRailCrossing::getOffsetFromIndex(int) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_offset_from_index(&self, index: i32) -> SUMOTime {
        0
    }

    /// C++: `int MSRailCrossing::getIndexFromOffset(SUMOTime) const` [const, virtual]
    /// Returns the step (the phasenumber) of a given position of the cycle
    // int
    // MSRailCrossing::getIndexFromOffset(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_index_from_offset(&self, offset: SUMOTime) -> i32 {
        0
    }

    /// C++: `void MSRailCrossing::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual]
    /// Changes the current phase and her duration
    // void changeStepAndDuration(MSTLLogicControl& tlcontrol, SUMOTime simStep, int step, SUMOTime stepDuration) {
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

}

