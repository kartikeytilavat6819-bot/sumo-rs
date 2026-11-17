import os, sys, xml.etree.ElementTree as ET

# TEMPORARY tl01 measurement: per-step TL state via TraCI + first differing FCD timestep.
# usage: agent_trace.py <case> <ours-bin> <cpp-bin> <outdir>

def tl_trace(binary, case, port, end):
    import traci
    cmd = [binary, '-n', f'{case}/ref_simple.net.xml', '-r', f'{case}/routes.rou',
           '--no-step-log', 'true'] + open(f'{case}/sim.args').read().split()
    traci.start(cmd, port=port)
    rows = []
    while traci.simulation.getTime() < end:
        traci.simulationStep()
        t = traci.simulation.getTime()
        for tl in traci.trafficlight.getIDList():
            rows.append(f"{t:.2f} {tl} phase={traci.trafficlight.getPhase(tl)} "
                        f"next={traci.trafficlight.getNextSwitch(tl):.2f} "
                        f"state={traci.trafficlight.getRedYellowGreenState(tl)} "
                        f"dur={traci.trafficlight.getPhaseDuration(tl):.2f}")
    traci.close()
    return rows

def fcd_steps(path):
    steps = {}
    for ev, el in ET.iterparse(path):
        if el.tag == 'timestep':
            t = el.get('time')
            steps[t] = [tuple(sorted(v.attrib.items())) for v in el]
            el.clear()
    return steps

def main():
    case, ours, cpp, out = sys.argv[1:5]
    end = float(open(f'{case}/sim.args').read().split('--end')[1].split()[0]) if '--end' in open(f'{case}/sim.args').read() else 0
    os.makedirs(out, exist_ok=True)
    tag = os.path.basename(case)
    a = tl_trace(ours, case, 8901, end)
    b = tl_trace(cpp, case, 8902, end)
    open(f'{out}/{tag}.ours.tl', 'w').write('\n'.join(a) + '\n')
    open(f'{out}/{tag}.cpp.tl', 'w').write('\n'.join(b) + '\n')
    print(f'[{tag}] TL rows ours={len(a)} cpp={len(b)}')
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            print(f'[{tag}] first TL diff at row {i}:')
            for j in range(max(0, i - 12), min(len(a), i + 3)):
                print(f'  {j} ours: {a[j]}\n  {j} cpp : {b[j]}')
            break
    else:
        if len(a) != len(b):
            print(f'[{tag}] TL rows differ in length only')
        else:
            print(f'[{tag}] TL states identical')

if __name__ == '__main__':
    main()
