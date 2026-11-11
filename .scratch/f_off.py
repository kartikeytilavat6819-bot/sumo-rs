import sys
sys.path.insert(0, 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/.scratch')
from fill import fill

P = 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/crates/sumo_core/src/microsim/traffic_lights/ms_off_traffic_light_logic.rs'

bodies = {
    'port MSOffTrafficLightLogic::MSOffTrafficLightLogic': """
let mut base = MSTrafficLightLogic::new(tlcontrol, id, "off", 0, TrafficLightType::OFF, 0, &());
base.myDefaultCycleTime = time2steps(120.);
Self { base, myPhaseDefinition: Vec::new() }
""",
    'port MSOffTrafficLightLogic::init': """
self.rebuild_phase();
""",
    'port MSOffTrafficLightLogic::~MSOffTrafficLightLogic': """
for i in self.myPhaseDefinition.iter() {
    // C++ `delete *i`: the phases were allocated with Box::into_raw in rebuild_phase
    unsafe { drop(Box::from_raw((*i).as_raw())) };
}
""",
    'port MSOffTrafficLightLogic::adaptLinkInformationFrom': """
self.base.adapt_link_information_from(logic);
self.rebuild_phase();
""",
    'port MSOffTrafficLightLogic::trySwitch': """
120 * delta_t()
""",
    'port MSOffTrafficLightLogic::getPhaseNumber': """
0
""",
    'port MSOffTrafficLightLogic::getPhases': """
&self.myPhaseDefinition
""",
    'port MSOffTrafficLightLogic::getPhase': """
&*self.myPhaseDefinition[0]
""",
    'port MSOffTrafficLightLogic::getLogicType': """
"offTrafficLightLogic".to_string()
""",
    'port MSOffTrafficLightLogic::getCurrentPhaseIndex': """
0
""",
    'port MSOffTrafficLightLogic::getCurrentPhaseDef': """
&*self.myPhaseDefinition[0]
""",
    'port MSOffTrafficLightLogic::resetLastSwitch': """
self.myPhaseDefinition[0].m().myLastSwitch = t;
""",
    'port MSOffTrafficLightLogic::getPhaseIndexAtTime': """
0
""",
    'port MSOffTrafficLightLogic::getOffsetFromIndex': """
0
""",
    'port MSOffTrafficLightLogic::getIndexFromOffset': """
0
""",
    'port MSOffTrafficLightLogic::changeStepAndDuration': """
// C++ body is empty (all parameters are UNUSED_PARAMETER)
let _ = (tlcontrol, simStep, step, stepDuration);
""",
    'port MSOffTrafficLightLogic::rebuildPhase': """
let no = self.get_links().len() as i32;
let mut state = String::new();
for i in 0..no {
    let mut foundMajor = false;
    let mut foundMinor = false;
    let mut foundAllwayStop = false;
    for l in self.get_links()[i as usize].iter() {
        // all links for the same index should have the same
        if l.get_off_state() == LINKSTATE_TL_OFF_BLINKING {
            foundMinor = true;
        } else if l.get_off_state() == LINKSTATE_TL_OFF_NOSIGNAL {
            foundMajor = true;
        } else if l.get_off_state() == LINKSTATE_ALLWAY_STOP {
            foundAllwayStop = true;
        } else {
            utils::write_warningf!("Invalid 'off'-state for link % at junction '%'", l.get_index(), l.get_junction().get_id());
        }
    }
    if foundMajor && foundMinor {
        utils::write_warningf!("Inconsistent 'off'-states for linkIndex % at tlLogic '%'", i, self.get_id());
    }
    // toString(LinkState) prints the link state character (LinkStates bijection)
    let linkState = if foundAllwayStop { LINKSTATE_ALLWAY_STOP } else if foundMinor { LINKSTATE_TL_OFF_BLINKING } else { LINKSTATE_TL_OFF_NOSIGNAL };
    state += LinkStates.get_string(linkState).unwrap();
}
for i in self.myPhaseDefinition.iter() {
    // C++ `delete *i`
    unsafe { drop(Box::from_raw((*i).as_raw())) };
}
self.myPhaseDefinition.clear();
self.myPhaseDefinition.push(Ptr::from_raw(Box::into_raw(Box::new(MSPhaseDefinition::new(time2steps(120.), &state, "")))));
""",
}

fill(P, bodies)
