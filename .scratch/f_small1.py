import sys
sys.path.insert(0, 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/.scratch')
from fill import fill

D = 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/crates/sumo_core/src/microsim/traffic_lights/'

fill(D + 'mssotl_policy3_d_stimulus.rs', {
    'port MSSOTLPolicy3DStimulus::MSSOTLPolicy3DStimulus': """
let mut base = MSSOTLPolicy5DStimulus::new(keyPrefix, parameters);
base.set_stim_cox_exp_dispersion_in_def_val(0.);
base.set_stim_cox_exp_dispersion_out_def_val(0.);
Self { base }
""",
})

# two constructors share one marker: fill them in order (new, then new_2)
P = D + 'mssotl_phase_traffic_light_logic.rs'
src = open(P, encoding='utf-8', newline='').read()
nl = '\r\n' if '\r\n' in src else '\n'
marker = '        todo!("port MSSOTLPhaseTrafficLightLogic::MSSOTLPhaseTrafficLightLogic")'
assert src.count(marker) == 2
bodies = [
    """
let base = MSSOTLTrafficLightLogic::new(tlcontrol, id, programID, TrafficLightType::SOTL_PHASE, phases, step, delay, parameters);
utils::common::msg_handler::write_message(&format!("*** Intersection {} will run using MSSOTLPhaseTrafficLightLogic ***", id));
Self { base }
""",
    """
Self { base: MSSOTLTrafficLightLogic::new_2(tlcontrol, id, programID, TrafficLightType::SOTL_PHASE, phases, step, delay, parameters, sensors) }
""",
]
for body in bodies:
    lines = [('        ' + l) if l.strip() else '' for l in body.strip('\n').split('\n')]
    src = src.replace(marker, nl.join(lines), 1)
with open(P, 'w', encoding='utf-8', newline='') as f:
    f.write(src)
fill(P, {
    'port MSSOTLPhaseTrafficLightLogic::canRelease': """
if self.get_current_phase_elapsed() >= self.get_current_phase_def().minDuration {
    return self.is_threshold_passed();
}
false
""",
})
print('ok phase tl')
