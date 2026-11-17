import os, sys

# TEMPORARY tl02 measurement: per-step fn.0 kinematics and TL state via TraCI, first differing row.
# usage: agent_trace.py <case> <ours-bin> <cpp-bin> <veh-id>

def trace(binary, case, port, end, veh_id):
    import traci
    cmd = [binary, '-n', f'{case}/ref_simple.net.xml', '-r', f'{case}/routes.rou',
           '--no-step-log', 'true'] + open(f'{case}/sim.args').read().split()
    traci.start(cmd, port=port)
    tl, veh = [], []
    while traci.simulation.getTime() < end:
        traci.simulationStep()
        t = traci.simulation.getTime()
        for tlid in traci.trafficlight.getIDList():
            tl.append(f"{t:.2f} {tlid} prog={traci.trafficlight.getProgram(tlid)} "
                      f"phase={traci.trafficlight.getPhase(tlid)} "
                      f"state={traci.trafficlight.getRedYellowGreenState(tlid)}")
        if veh_id in traci.vehicle.getIDList():
            veh.append(f"{t:.4f} {veh_id} pos={traci.vehicle.getLanePosition(veh_id):.4f} "
                       f"speed={traci.vehicle.getSpeed(veh_id):.4f} acc={traci.vehicle.getAcceleration(veh_id):.4f} "
                       f"lane={traci.vehicle.getLaneID(veh_id)} leader={traci.vehicle.getLeader(veh_id)} "
                       f"nextTLS={traci.vehicle.getNextTLS(veh_id)}")
    traci.close()
    return tl, veh

def first_diff(name, a, b):
    print(f'[{name}] rows ours={len(a)} cpp={len(b)}')
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            print(f'[{name}] first diff at row {i}:')
            for j in range(max(0, i - 4), min(len(a), i + 2)):
                print(f'  ours {a[j]}\n  cpp  {b[j]}')
            return
    print(f'[{name}] identical')

def main():
    case, ours, cpp, veh_id = sys.argv[1:5]
    args = open(f'{case}/sim.args').read().split()
    end = float(args[args.index('--end') + 1]) if '--end' in args else 0
    at, av = trace(ours, case, 8901, end, veh_id)
    bt, bv = trace(cpp, case, 8902, end, veh_id)
    first_diff('TL', at, bt)
    first_diff('VEH', av, bv)

if __name__ == '__main__':
    main()
