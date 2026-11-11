import sys
sys.path.insert(0, 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/.scratch')
from fill import fill

D = 'C:/Users/karti/sumo-rs/.claude/worktrees/agent-a862f446a19b63d41/crates/sumo_core/src/microsim/traffic_lights/'
P = D + 'mssotl_policy5_d_family_stimulus.rs'

fill(P, {
    'port MSSOTLPolicy5DFamilyStimulus::getMessage': """
let mut ot = String::new();
for i in 0..self.family.len() {
    ot += &format!(" gaussian {}:{}", i, self.family[i].m().get_message());
}
ot
""",
    'port MSSOTLPolicy5DFamilyStimulus::getFamilies': """
self.family.clone()
""",
})

# two overloads share one marker: fill in declaration order (2-arg, then 4-arg)
src = open(P, encoding='utf-8', newline='').read()
nl = '\r\n' if '\r\n' in src else '\n'
marker = '        todo!("port MSSOTLPolicy5DFamilyStimulus::computeDesirability")'
assert src.count(marker) == 2
bodies = [
    """
self.compute_desirability_2(vehInMeasure, vehOutMeasure, 0., 0.)
""",
    """
let mut best_stimulus = -1.;
for it in self.family.iter() {
    let temp_stimulus = it.m().compute_desirability_2(vehInMeasure, vehOutMeasure, vehInDispersionMeasure, vehOutDispersionMeasure);
    if temp_stimulus > best_stimulus {
        best_stimulus = temp_stimulus;
    }
}
best_stimulus
""",
]
for body in bodies:
    lines = [('        ' + l) if l.strip() else '' for l in body.strip('\n').split('\n')]
    src = src.replace(marker, nl.join(lines), 1)
with open(P, 'w', encoding='utf-8', newline='') as f:
    f.write(src)
print('ok family')
