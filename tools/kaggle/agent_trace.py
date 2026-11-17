import os, sys, xml.etree.ElementTree as ET

# TEMPORARY tl01 measurement: per-step TL state and induction-loop gaps via TraCI,
# plus first differing FCD timestep. usage: agent_trace.py <case> <ours-bin> <cpp-bin> <outdir>

def trace(binary, case, port, end):
    import traci
    cmd = [binary, '-n', f'{case}/ref_simple.net.xml', '-r', f'{case}/routes.rou',
           '--no-step-log', 'true'] + open(f'{case}/sim.args').read().split()
    traci.start(cmd, port=port)
    tl, det = [], []
    while traci.simulation.getTime() < end:
        traci.simulationStep()
        t = traci.simulation.getTime()
        for tlid in traci.trafficlight.getIDList():
            tl.append(f"{t:.2f} {tlid} phase={traci.trafficlight.getPhase(tlid)} "
                      f"next={traci.trafficlight.getNextSwitch(tlid):.2f} "
                      f"state={traci.trafficlight.getRedYellowGreenState(tlid)} "
                      f"dur={traci.trafficlight.getPhaseDuration(tlid):.2f}")
        for d in sorted(traci.inductionloop.getIDList()):
            det.append(f"{t:.2f} {d} gap={traci.inductionloop.getTimeSinceDetection(d):.2f} "
                       f"pos={traci.inductionloop.getPosition(d):.2f} "
                       f"len={traci.inductionloop.getLength(d):.2f}")
    traci.close()
    return tl, det

def first_diff(name, a, b):
    print(f'[{name}] rows ours={len(a)} cpp={len(b)}')
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            print(f'[{name}] first diff at row {i}:')
            for j in range(max(0, i - 12), min(len(a), i + 3)):
                print(f'  {j} ours: {a[j]}\n  {j} cpp : {b[j]}')
            return
    print(f'[{name}] identical')

def fcd_steps(path):
    steps = {}
    for ev, el in ET.iterparse(path):
        if el.tag == 'timestep':
            steps[el.get('time')] = [tuple(sorted(v.attrib.items())) for v in el]
            el.clear()
    return steps

def main():
    case, ours, cpp, out = sys.argv[1:5]
    args = open(f'{case}/sim.args').read().split()
    end = float(args[args.index('--end') + 1]) if '--end' in args else 0
    os.makedirs(out, exist_ok=True)
    tag = os.path.basename(case)
    at, ad = trace(ours, case, 8901, end)
    bt, bd = trace(cpp, case, 8902, end)
    open(f'{out}/{tag}.ours.tl', 'w').write('\n'.join(at) + '\n')
    open(f'{out}/{tag}.cpp.tl', 'w').write('\n'.join(bt) + '\n')
    first_diff(f'{tag} TL', at, bt)
    first_diff(f'{tag} DET', ad, bd)

if __name__ == '__main__':
    main()
