import sys
sys.path.insert(0, 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/.scratch')
from fill import fill

D = 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/crates/sumo_core/src/microsim/traffic_lights/'

# three constructors share one marker, so fill them in declaration order
P = D + 'mssotl_congestion_policy.rs'
src = open(P, encoding='utf-8', newline='').read()
nl = '\r\n' if '\r\n' in src else '\n'
marker = '        todo!("port MSSOTLCongestionPolicy::MSSOTLCongestionPolicy")'
assert src.count(marker) == 3
bodies = [
    """
MSSOTLPolicy::new("Congestion".to_string(), parameters).into_congestion()
""",
    """
let mut base = MSSOTLPolicy::new_2("Congestion".to_string(), desirabilityAlgorithm);
base.get_desirability_algorithm().m().set_key_prefix("CONGESTION".to_string());
Self { base }
""",
    """
let mut base = MSSOTLPolicy::new_3("Congestion".to_string(), desirabilityAlgorithm, parameters);
base.get_desirability_algorithm().m().set_key_prefix("CONGESTION".to_string());
Self { base }
""",
]
bodies[0] = """
Self { base: MSSOTLPolicy::new("Congestion".to_string(), parameters) }
"""
for body in bodies:
    lines = [('        ' + l) if l.strip() else '' for l in body.strip('\n').split('\n')]
    src = src.replace(marker, nl.join(lines), 1)
with open(P, 'w', encoding='utf-8', newline='') as f:
    f.write(src)

fill(P, {
    'port MSSOTLCongestionPolicy::decideNextPhase': """
if stage.is_commit() {
    // decide which chain to activate. Gotta work on this
    return currentPhaseIndex;
}
if stage.is_transient() {
    // If the junction was in a transient step
    // => go to the next step and return computeReturnTime()
    return currentPhaseIndex + 1;
}
if stage.is_decisional() {
    if self.can_release(elapsed, thresholdPassed, pushButtonPressed, stage, vehicleCount) {
        return currentPhaseIndex + 1;
    }
}
currentPhaseIndex
""",
    'port MSSOTLCongestionPolicy::canRelease': """
elapsed >= stage.minDuration
""",
})
print('ok congestion')
