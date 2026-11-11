import sys
sys.path.insert(0, 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/.scratch')
from fill import fill

P = 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/crates/sumo_core/src/microsim/traffic_lights/ms_phase_definition.rs'

path_statics_old = """// C++ static members of MSPhaseDefinition (port as module-level statics, see AGENTS.md):
//   static const SUMOTime MSPhaseDefinition::UNSPECIFIED_DURATION;
//   static const SUMOTime MSPhaseDefinition::OVERRIDE_DURATION;
"""
path_statics_new = """// C++ static members of MSPhaseDefinition (port as module-level statics, see AGENTS.md):
//   static const SUMOTime MSPhaseDefinition::UNSPECIFIED_DURATION;
//   static const SUMOTime MSPhaseDefinition::OVERRIDE_DURATION;
/// C++ MSPhaseDefinition::UNSPECIFIED_DURATION
pub const UNSPECIFIED_DURATION: SUMOTime = -1;
/// C++ MSPhaseDefinition::OVERRIDE_DURATION = TIME2STEPS(-1)
pub const OVERRIDE_DURATION: SUMOTime = -1000;
"""

bodies = {
    'port MSPhaseDefinition::MSPhaseDefinition': """
Self {
    duration: _duration,
    lastDuration: UNSPECIFIED_DURATION,
    minDuration: _duration,
    maxDuration: _duration,
    earliestEnd: UNSPECIFIED_DURATION,
    latestEnd: UNSPECIFIED_DURATION,
    earlyTarget: String::new(),
    finalTarget: String::new(),
    myLastSwitch: UNSPECIFIED_DURATION,
    myLastEnd: UNSPECIFIED_DURATION,
    nextPhases: Vec::new(),
    name: _name.to_string(),
    yellow: UNSPECIFIED_DURATION,
    red: UNSPECIFIED_DURATION,
    vehext: UNSPECIFIED_DURATION,
    myTransientNotDecisional: false,
    myCommit: false,
    myUndefined: false,
    myTargetLaneSet: Vec::new(),
    myState: state.to_string(),
}
""",
    'port MSPhaseDefinition::~MSPhaseDefinition': """
// C++ destructor has an empty body; Rust drops the fields automatically.
""",
    'port MSPhaseDefinition::getState': "&self.myState",
    'port MSPhaseDefinition::setState': "self.myState = _state.to_string();",
    'port MSPhaseDefinition::getTargetLaneSet': "&self.myTargetLaneSet",
    'port MSPhaseDefinition::getNextPhases': "&self.nextPhases",
    'port MSPhaseDefinition::getName': "&self.name",
    'port MSPhaseDefinition::setName': "self.name = _name.to_string();",
    'port MSPhaseDefinition::isGreenPhase': """
if !self.myState.contains(['g', 'G']) {
    return false;
}
if self.myState.contains(['y', 'Y']) {
    return false;
}
true
""",
    'port MSPhaseDefinition::isAllRedPhase': """
self.myState.len() == self.myState.chars().filter(|c| *c == 'r').count()
""",
    'port MSPhaseDefinition::getSignalState': """
// C++ `char` is signed, so the cast goes through i8.
(self.myState.as_bytes()[pos as usize] as i8) as LinkState
""",
    'port MSPhaseDefinition::isActuated': """
self.minDuration != self.maxDuration || self.minDuration == OVERRIDE_DURATION
""",
    'port MSPhaseDefinition::operator!=': """
self.myState != pd.myState || self.name != pd.name
""",
    'port MSPhaseDefinition::isUndefined': "self.myUndefined",
    'port MSPhaseDefinition::isTarget': "!self.myTargetLaneSet.is_empty()",
    'port MSPhaseDefinition::isTransient': "self.myTransientNotDecisional",
    'port MSPhaseDefinition::isDecisional': "!self.myTransientNotDecisional",
    'port MSPhaseDefinition::isCommit': "self.myCommit",
}

src = open(P, encoding='utf-8', newline='').read()
path_statics_old = path_statics_old.replace('\n', '\r\n')
path_statics_new = path_statics_new.replace('\n', '\r\n')
assert src.count(path_statics_old) == 1, 'statics block not found'
src = src.replace(path_statics_old, path_statics_new, 1)
with open(P, 'w', encoding='utf-8', newline='') as f:
    f.write(src)
fill(P, bodies)
