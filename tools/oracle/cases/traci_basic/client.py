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
from traci.exceptions import TraCIException

CASE = Path(__file__).resolve().parent
CASES_DIR = CASE.parent
DEFAULT_OURS = Path(os.environ.get("CARGO_TARGET_DIR", "C:/Users/karti/sumo-rs-target")) / "debug" / "sumo.exe"

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
