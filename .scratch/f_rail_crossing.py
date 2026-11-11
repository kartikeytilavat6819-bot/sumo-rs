import sys
sys.path.insert(0, 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/.scratch')
from fill import fill

D = 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/crates/sumo_core/src/microsim/traffic_lights/'
P = D + 'ms_rail_crossing.rs'

fill(P, {
    'port MSRailCrossing::MSRailCrossing': """
let mut base = MSSimpleTrafficLightLogic::new(tlcontrol, id, programID, 0, TrafficLightType::RAIL_CROSSING, &Vec::new(), 0, delay, parameters);
// dummy phase, used to avoid crashing in MSTrafficLightLogic::setTrafficLightSignals()
base.myPhases.push(Ptr::from_raw(Box::into_raw(Box::new(MSPhaseDefinition::new(1, &"X".repeat(SUMO_MAX_CONNECTIONS), "")))));
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
""",
    'port MSRailCrossing::~MSRailCrossing': """
// C++ destructor body is empty; the base class owns the phases
""",
    'port MSRailCrossing::init': """
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
self.myPhases.push(mk(self.myYellowTime, (LINKSTATE_TL_YELLOW_MINOR as u8 as char).to_string().repeat(n)));
self.myPhases.push(mk(1, (LINKSTATE_TL_RED as u8 as char).to_string().repeat(n)));
self.myPhases.push(mk(self.myOpeningTime, (LINKSTATE_TL_REDYELLOW as u8 as char).to_string().repeat(n)));
// init phases
self.update_current_phase();
let now = MSNet::get_instance().get_current_time_step();
self.set_traffic_light_signals(now);
self.myNumLinks = self.myLinks.len() as i32;
""",
    'port MSRailCrossing::setParameter': """
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
""",
    'port MSRailCrossing::addLink': """
if pos >= 0 {
    self.base.add_link(link, lane, pos);
} else {
    self.myIncomingRailLinks.push(link);
}
""",
    'port MSRailCrossing::adaptLinkInformationFrom': """
self.base.adapt_link_information_from(logic);
self.update_current_phase();
""",
    'port MSRailCrossing::updateCurrentPhase': """
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
""",
    'port MSRailCrossing::trySwitch': """
let oldStep = self.myStep;
let nextTry = self.update_current_phase();
if self.myStep != oldStep {
    self.myPhases[self.myStep as usize].m().myLastSwitch = MSNet::get_instance().get_current_time_step();
}
nextTry
""",
    'port MSRailCrossing::getPhaseIndexAtTime': """
0
""",
    'port MSRailCrossing::getOffsetFromIndex': """
0
""",
    'port MSRailCrossing::getIndexFromOffset': """
0
""",
    'port MSRailCrossing::changeStepAndDuration': """
// C++ body is empty (all parameters are UNUSED_PARAMETER)
let _ = (tlcontrol, simStep, step, stepDuration);
""",
})
print('ok rail crossing')
