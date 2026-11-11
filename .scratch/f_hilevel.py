import sys
sys.path.insert(0, 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/.scratch')
from fill import fill

D = 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/crates/sumo_core/src/microsim/traffic_lights/'
P = D + 'mssotl_hi_level_traffic_light_logic.rs'

# two constructors share one marker: fill in declaration order (new, then new_2)
src = open(P, encoding='utf-8', newline='').read()
nl = '\r\n' if '\r\n' in src else '\n'
marker = '        todo!("port MSSOTLHiLevelTrafficLightLogic::MSSOTLHiLevelTrafficLightLogic")'
assert src.count(marker) == 2
bodies = [
    """
Self {
    base: MSSOTLTrafficLightLogic::new(tlcontrol, id, programID, logicType, phases, step, delay, parameters),
    myPolicies: Vec::new(),
    myCurrentPolicy: Ptr::null(),
}
""",
    """
Self {
    base: MSSOTLTrafficLightLogic::new_2(tlcontrol, id, programID, logicType, phases, step, delay, parameters, sensors),
    myPolicies: Vec::new(),
    myCurrentPolicy: Ptr::null(),
}
""",
]
for body in bodies:
    lines = [('        ' + l) if l.strip() else '' for l in body.strip('\n').split('\n')]
    src = src.replace(marker, nl.join(lines), 1)
with open(P, 'w', encoding='utf-8', newline='') as f:
    f.write(src)

fill(P, {
    'port MSSOTLHiLevelTrafficLightLogic::~MSSOTLHiLevelTrafficLightLogic': """
for policy in self.myPolicies.iter() {
    // C++ `delete policy`: policies are owned by this logic
    unsafe { drop(Box::from_raw((*policy).as_raw())) };
}
""",
    'port MSSOTLHiLevelTrafficLightLogic::init': """
self.base.init(nb);
""",
    'port MSSOTLHiLevelTrafficLightLogic::addPolicy': """
self.myPolicies.push(policy);
""",
    'port MSSOTLHiLevelTrafficLightLogic::activate': """
self.myCurrentPolicy = policy;
""",
    'port MSSOTLHiLevelTrafficLightLogic::decidePolicy': """
// shortcut: pure virtual with no C++ body; a subclass must override it, so calling the base is a bug (as in C++)
panic!("pure virtual method MSSOTLHiLevelTrafficLightLogic::decidePolicy called")
""",
})
print('ok hilevel')
