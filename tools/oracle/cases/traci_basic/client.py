"""TraCI oracle: drive our sumo.exe and the C++ sumo over TraCI and compare the answers.

Each probe runs in its own server session (prelude: two simulation steps), so one crash on our side
only loses its own row. Usage:
  python tools/oracle/cases/traci_basic/client.py                # both cases, both sides, one table each
  python tools/oracle/cases/traci_basic/client.py --case signal --only ours
Options: --ours <sumo.exe> (default: $CARGO_TARGET_DIR/debug/sumo.exe)  --port <n> (default 8813)
The C++ side runs in WSL: $HOME/sumo-src/bin/sumo with SUMO_HOME=$HOME/sumo-src.
"""
import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

import traci
from traci import constants as tc
from traci.exceptions import TraCIException

CASE = Path(__file__).resolve().parent
CASES_DIR = CASE.parent
DEFAULT_OURS = Path(os.environ.get("CARGO_TARGET_DIR", "C:/Users/karti/sumo-rs-target")) / "debug" / "sumo.exe"


def warn_if_stale(ours: Path) -> None:
    """Print the binary in use and shout if any .rs is newer than it.

    Two different defaults exist across this harness (`sumo-rs-target` here, `sumo-rs-target-main` in
    run_all.sh), so it is easy to measure a binary that is not the code under test — that mistake produced
    a false 40/62 reading on the night this was added.
    """
    print(f"ours: {ours}")
    try:
        btime = ours.stat().st_mtime
    except OSError:
        print("  MISSING BINARY — build first")
        return
    repo = CASES_DIR.parent.parent.parent
    newer = None
    for p in (repo / "crates").rglob("*.rs"):
        if p.stat().st_mtime > btime:
            newer = p
            break
    if newer is not None:
        print(f"  STALE BINARY: older than {newer.relative_to(repo)} — results measure other code")


# case -> (lane for the lane getters, traffic light id in that net or "")
CASE_PARAMS = {
    "junction": ("-103224#1_0", ""),
    "signal": ("-324752522_0", "1802245009"),
}


def wsl_path(p):
    p = str(Path(p).resolve()).replace("\\", "/")
    return "/mnt/" + p[0].lower() + p[2:]


def sumo_command(side, port, net, rou, ours):
    if side == "ours":
        return [str(ours), "-n", str(net), "-r", str(rou), "--remote-port", str(port),
                "--no-step-log", "true", "--no-warnings", "true"]
    inner = (f"SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n {wsl_path(net)} -r {wsl_path(rou)} "
             f"--remote-port {port} --no-step-log true --no-warnings true")
    return ["wsl", "-e", "bash", "-lc", inner]


def probes(lane, tl_id):
    """label -> function(traci) run after the prelude. Each returns the value to record."""
    def set_speed_chain(t):
        t.vehicle.setSpeed("v0", 5.0)
        t.simulationStep(11)
        return t.vehicle.getSpeed("v0")

    def sub_vehicle(t):
        t.vehicle.subscribe("v0", [tc.VAR_SPEED, tc.VAR_LANE_ID, tc.VAR_POSITION])
        t.simulationStep(11)
        return t.vehicle.getSubscriptionResults("v0")

    def sub_simulation(t):
        t.simulation.subscribe([tc.VAR_DEPARTED_VEHICLES_NUMBER])
        t.simulationStep(11)
        return t.simulation.getSubscriptionResults()

    def sub_param(t):
        t.vehicle.subscribeParameterWithKey("v0", "nokey")
        t.simulationStep(11)
        return t.vehicle.getSubscriptionResults("v0")

    def change_lane(t):
        t.vehicle.changeLane("v0", t.vehicle.getLaneIndex("v0") ^ 1, 5.0)
        t.simulationStep(11)
        return t.vehicle.getLaneIndex("v0")

    def change_target_last(t):
        t.vehicle.changeTarget("v0", t.vehicle.getRoute("v0")[-1])
        t.simulationStep(11)
        return t.vehicle.getRoute("v0")

    def set_route_current(t):
        t.vehicle.setRoute("v0", t.vehicle.getRoute("v0"))
        t.simulationStep(11)
        return t.vehicle.getRoute("v0")

    def move_to_xy(t):
        x, y = t.vehicle.getPosition("v0")
        t.vehicle.moveToXY("v0", t.vehicle.getRoadID("v0"), t.vehicle.getLaneIndex("v0"), x + 1.0, y)
        return t.vehicle.getPosition("v0")

    def add_vehicle(t):
        t.vehicle.add("vnew", t.route.getIDList()[0])
        t.simulationStep(11)
        return t.vehicle.getIDList()

    def remove_vehicle(t):
        t.vehicle.remove("v0")
        t.simulationStep(11)
        return t.vehicle.getIDList()

    return {
        "getVersion": lambda t: t.getVersion(),
        "simulationStep(11) returns": lambda t: t.simulationStep(11),
        "simulation.getTime": lambda t: t.simulation.getTime(),
        "simulation.getLoadedNumber": lambda t: t.simulation.getLoadedNumber(),
        "simulation.getArrivedNumber": lambda t: t.simulation.getArrivedNumber(),
        "vehicle.getIDList": lambda t: t.vehicle.getIDList(),
        "vehicle.getPosition(v0)": lambda t: t.vehicle.getPosition("v0"),
        "vehicle.getSpeed(v0)": lambda t: t.vehicle.getSpeed("v0"),
        "lane.getLastStepVehicleNumber": lambda t: t.lane.getLastStepVehicleNumber(lane),
        "lane.getLength": lambda t: t.lane.getLength(lane),
        "trafficlight.getIDList": lambda t: t.trafficlight.getIDList(),
        "trafficlight.getRedYellowGreenState": lambda t: t.trafficlight.getRedYellowGreenState(tl_id or "none"),
        "vehicle.setSpeed(v0,5) + step + getSpeed": set_speed_chain,
        "vehicle.getSpeed(nope)": lambda t: t.vehicle.getSpeed("nope"),
        "lane.getLength(nope)": lambda t: t.lane.getLength("nope"),
        "trafficlight.getRedYellowGreenState(nope)": lambda t: t.trafficlight.getRedYellowGreenState("nope"),
        "vehicle.subscribe(v0) + step + results": sub_vehicle,
        "simulation.subscribe(departed) + step": sub_simulation,
        "vehicle.subscribeParameterWithKey(v0) + step": sub_param,
        "vehicle.subscribe(nope) + step + results": lambda t: (t.vehicle.subscribe("nope", [tc.VAR_SPEED]), t.simulationStep(11), t.vehicle.getSubscriptionResults("nope"))[2],
        "vehicle.changeLane(v0) + step + getLaneIndex": change_lane,
        "vehicle.changeTarget(v0, last) + step + getRoute": change_target_last,
        "vehicle.setRoute(v0, current) + step + getRoute": set_route_current,
        "vehicle.moveToXY(v0, edge, lane, x+1, y)": move_to_xy,
        "vehicle.moveToXY(v0, nope edge)": lambda t: t.vehicle.moveToXY("v0", "nope", 0, 0.0, 0.0),
        "vehicle.add(vnew, route0) + step + getIDList": add_vehicle,
        "vehicle.add(v0, dup)": lambda t: t.vehicle.add("v0", "r0"),
        "vehicle.add(vx, nope route)": lambda t: t.vehicle.add("vx", "nope"),
        "vehicle.remove(v0) + step + getIDList": remove_vehicle,
        "vehicle.remove(nope)": lambda t: t.vehicle.remove("nope"),
        "vehicle.getJunctionFoes(v0)": lambda t: t.vehicle.getJunctionFoes("v0"),
    }


def run_probe(side, port, net, rou, ours, fn):
    """One fresh server for one probe. Returns {"type", "value"}."""
    log = Path(os.environ.get("TEMP", ".")) / f"traci_{side}_{net.parent.name}.log"
    with open(log, "a") as fh:
        proc = subprocess.Popen(sumo_command(side, port, net, rou, ours), cwd=str(CASE), stdout=fh, stderr=subprocess.STDOUT)
        try:
            traci.init(port=port, numRetries=20, host="localhost")
            try:
                traci.simulationStep(0)
                traci.simulationStep(10)
                v = fn(traci)
                return {"type": type(v).__name__, "value": repr(v)}
            except TraCIException as e:
                return {"type": "TraCIException", "value": str(e)}
            except Exception as e:
                return {"type": type(e).__name__, "value": str(e)}
            finally:
                try:
                    traci.close()
                except Exception:
                    pass
        finally:
            try:
                proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                proc.kill()


def run_side(side, port, ours, case):
    net, rou = CASES_DIR / case / "ref_simple.net.xml", CASES_DIR / case / "routes.rou"
    lane, tl_id = CASE_PARAMS[case]
    out = {}
    for i, (label, fn) in enumerate(probes(lane, tl_id).items()):
        # fresh port per probe: a WSL sumo left behind by a previous probe would still hold the old one
        out[label] = run_probe(side, port + i, net, rou, ours, fn)
        if side == "cpp":
            subprocess.run(["wsl", "-e", "bash", "-lc", "pkill -f 'bin/sumo' || true"], capture_output=True)
        time.sleep(0.5)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", choices=["ours", "cpp"])
    ap.add_argument("--case", default="all", choices=sorted(CASE_PARAMS) + ["all"])
    ap.add_argument("--ours", default=str(DEFAULT_OURS))
    ap.add_argument("--port", type=int, default=8813)
    a = ap.parse_args()
    warn_if_stale(Path(a.ours))

    if a.only:
        print(json.dumps(run_side(a.only, a.port, a.ours, a.case), indent=2))
        return 0

    total = bad_total = 0
    for case in ([a.case] if a.case != "all" else sorted(CASE_PARAMS)):
        cpp = run_side("cpp", a.port, a.ours, case)
        ours = run_side("ours", a.port, a.ours, case)
        print(f"[{case}] {'command':<42} {'C++':<38} {'ours':<38} verdict")
        bad = 0
        for label in cpp:
            c, o = cpp[label], ours[label]
            same = c == o
            bad += not same
            print(f"{label:<42} {(c['type'] + ' ' + c['value'])[:38]:<38} {(o['type'] + ' ' + o['value'])[:38]:<38} {'MATCH' if same else 'DIFF'}")
        print(f"[{case}] {len(cpp) - bad}/{len(cpp)} match\n")
        total += len(cpp)
        bad_total += bad
    print(f"TOTAL {total - bad_total}/{total} match")
    return 1 if bad_total else 0


if __name__ == "__main__":
    sys.exit(main())
