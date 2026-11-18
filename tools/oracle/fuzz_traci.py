"""Differential fuzzing of the TraCI control path.

For each random seed, one TraCI session is driven against our `sumo` and one
against the official `$SUMO_PRISTINE_BIN/sumo` (same small junction net + infra
add-ons, same deterministic random sequence of get/set calls, stepping in
between).  Every returned value and every exception (type *and* message) is
compared.  Divergences are printed as NEW-PROBE lines so they can be promoted
into client.py probes and fixed in the Rust traci-server / libsumo port.

A crash of the *official* server is ignored from that call on (its connection is
dead); a crash of *ours* is reported as a real difference.
"""
import argparse
import os
import random
import subprocess
import sys
from pathlib import Path

# FUZZ_ROOT lets the gate runner execute an embedded copy of this file from /tmp
# while still resolving the case net/routes relative to the real repo checkout.
HERE = Path(os.environ.get("FUZZ_ROOT", str(Path(__file__).resolve().parent)))
sys.path.insert(0, str(HERE / "cases" / "traci_basic"))
import client as C  # noqa: E402  (sumo_command / kill_cpp / PRISTINE_BIN)

import traci  # noqa: E402
import traci.connection as TCONN  # noqa: E402
from traci.exceptions import FatalTraCIError, TraCIException  # noqa: E402

CASES = HERE / "cases"
CASE = CASES / "junction"
NET = CASE / "ref_simple.net.xml"
ROU = CASE / "routes.rou"
INFRA = HERE / "cases" / "traci_basic" / "infra.add.xml"

V = "v0"
LANE = "-103224#1_0"
LANE2 = "-103224#2_0"
EDGE = "-103224#1"
EDGE2 = "-103224#2"
EDGE3 = "-103224#3"

# IDs used as random arguments.  Valid IDs are deliberately repeated so most
# calls exercise real objects; invalid / empty IDs cover error paths.
IDS = {
    "vid": [V, V, V, V, "nope", "v4", ""],
    "lid": [LANE, LANE, LANE, LANE2, "nope", ":-137712_0_0", ""],
    "eid": [EDGE, EDGE, EDGE, EDGE2, EDGE3, "nope", ""],
    "tlid": ["none", "nope", ""],
    "vtid": ["DEFAULT_VEHTYPE", "DEFAULT_VEHTYPE", "nope", ""],
    "route": ["r0", "r1", "nope", ""],
    "jid": ["-137721", "-137719", "nope", ""],
    "il": ["il0", "il0", "nope", ""],
    "la": ["la0", "la0", "nope", ""],
    "me": ["me0", "me0", "nope", ""],
    "person": ["p0", "nope", ""],
    "poi": ["p1", "nope", ""],
    "poly": ["pg1", "nope", ""],
    "bus": ["bs0", "nope", ""],
}
NUMS = [-1.0, 0.0, 0.5, 1.0, 5.0, 20.0, 50.0, 1000.0, 1e6]
POS = [-5.0, 0.0, 5.0, 10.0, 1e6]
KEYS = ["laneChangeModel.tau", "carFollowModel.tau", "device.tripinfo.waitingTime",
        "hasVehiclePermission", "nokey", "nope", "myKey"]
VALS = ["0.5", "1", "-1", "x", "", "2.5"]
COLORS = [(255, 0, 0, 255), (0, 255, 0, 255), (1, 2, 3, 255)]
STATES = ["G", "r", "g", "y", "GGrgry", "Z", ""]


def set_case(name):
    """Switch the net/route/IDs so the signal case exercises a real traffic light."""
    global CASE, NET, ROU, IDS
    CASE = CASES / name
    NET = CASE / "ref_simple.net.xml"
    ROU = CASE / "routes.rou"
    IDS = dict(IDS)
    if name == "signal":
        IDS.update({
            "tlid": ["1802245009", "1802245009", "nope", ""],
            "lid": ["-324752522_0", "-324752522_0", "nope", ""],
            "eid": ["-324752522", "-324752522", "nope", ""],
            "jid": ["nope", ""],
        })


# --- random argument generators: (rng, IDS) -> tuple of positional args ------
def _zero(r, i):
    return ()


def _pick(name):
    return lambda r, i: (r.choice(i[name]),)


def _pick2(a, b):
    return lambda r, i: (r.choice(i[a]), r.choice(i[b]))


def _pick3(a, b, c):
    return lambda r, i: (r.choice(i[a]), r.choice(i[b]), r.choice(i[c]))


def _num(r, i):
    return (r.choice(NUMS),)


def _num2(r, i):
    return (r.choice(NUMS), r.choice(NUMS))


def _vnum(r, i):
    return (r.choice(i["vid"]), r.choice(NUMS))


def _lnum(r, i):
    return (r.choice(i["lid"]), r.choice(NUMS))


def _enum(r, i):
    return (r.choice(i["eid"]), r.choice(NUMS))


def _vkey(r, i):
    return (r.choice(i["vid"]), r.choice(KEYS))


def _vkeyval(r, i):
    return (r.choice(i["vid"]), r.choice(KEYS), r.choice(VALS))


def _vpos(r, i):
    return (r.choice(i["vid"]), r.choice(POS))


def _vepos(r, i):
    return (r.choice(i["vid"]), r.choice(i["eid"]), r.choice(POS))


def _xy(r, i):
    return (r.choice(NUMS), r.choice(NUMS))


def _xy2(r, i):
    return (r.choice(NUMS), r.choice(NUMS), r.choice(NUMS), r.choice(NUMS))


def _routes(r, i):
    return (r.choice(i["route"]), r.choice([["-103224#1"], ["nope"], [], [EDGE, EDGE3]]))


def _vstate(r, i):
    return (r.choice(i["vid"]), r.choice(STATES))


# SPECS: (domain, method, arg-generator).  One is chosen per generated call.
SPECS = []


def add(dom, gen, *methods):
    for m in methods:
        SPECS.append((dom, m, gen))


# simulation
for m in ["getTime", "getCurrentTime", "getDeltaT", "getEndTime", "getLoadedNumber",
          "getLoadedIDList", "getDepartedNumber", "getDepartedIDList", "getArrivedNumber",
          "getArrivedIDList", "getMinExpectedNumber", "getNetBoundary", "getScale",
          "getCollidingVehiclesNumber", "getCollidingVehiclesIDList", "getCollisions",
          "getPendingVehicles", "getBusStopIDList", "getStartingTeleportNumber",
          "getEndingTeleportNumber", "getParkingStartingVehiclesNumber",
          "getStopStartingVehiclesNumber", "getDepartedPersonNumber", "getArrivedPersonNumber",
          "getIDList", "getIDCount"]:
    add("simulation", _zero, m)
add("simulation", _num, "getOption", "getParameter")
add("simulation", _pick("bus"), "getBusStopWaiting", "getBusStopWaitingIDList")
add("simulation", _xy2, "getDistance2D")
add("simulation", _num2, "convert2D", "convert3D")
add("simulation", _xy, "convertRoad", "convertGeo")
add("simulation", _pick2("eid", "eid"), "findRoute", "findIntermodalRoute")

# vehicle
add("vehicle", _zero, "getIDList", "getIDCount")
for m in ["getPosition", "getPosition3D", "getSpeed", "getLaneID", "getLanePosition", "getAngle",
          "getRoadID", "getRouteID", "getRoute", "getAcceleration", "getSignals", "getDistance",
          "getTypeID", "getColor", "getLaneIndex", "getMaxSpeed", "getMinGap", "getLength",
          "getWidth", "getHeight", "getMass", "getSpeedFactor", "getSpeedMode", "getLaneChangeMode",
          "getVehicleClass", "getEmissionClass", "getShapeClass", "getTau", "getAccel", "getDecel",
          "getEmergencyDecel", "getApparentDecel", "getImpatience", "getImperfection",
          "getWaitingTime", "getAccumulatedWaitingTime", "getActionStepLength", "getLateralAlignment",
          "getMaxSpeedLat", "getMinGapLat", "getSpeedDeviation", "getBoardingDuration",
          "getRoutingMode", "getVia", "getNextStops", "getStops", "getLine", "getPersonNumber",
          "getPersonCapacity", "getCO2Emission", "getFuelConsumption", "getElectricityConsumption",
          "getNoiseEmission", "getNOxEmission", "getHCEmission", "getPMxEmission", "getCOEmission",
          "getSlope", "getLateralSpeed", "getLateralLanePosition", "getDepartDelay", "getDeparture",
          "getTimeLoss", "getStopSpeed", "getStopState", "getStopDelay", "getStopArrivalDelay",
          "getLastActionTime", "getAllowedSpeed", "getBestLanes", "getNextLinks", "getNextTLS",
          "getJunctionFoes", "getAdaptedTraveltime", "getEffort", "getSecureGap", "getSpeedWithoutTraCI"]:
    add("vehicle", _pick("vid"), m)
add("vehicle", _vnum, "getLeader", "getFollower")
add("vehicle", _pick2("vid", "vid"), "getNeighbors")
add("vehicle", _vkey, "getParameter", "getParameterWithKey")
add("vehicle", _vpos, "getDrivingDistance2D")
add("vehicle", _vepos, "getDrivingDistance")
add("vehicle", lambda r, i: (r.choice(i["vid"]), r.choice(NUMS), r.choice(POS), r.choice(NUMS), r.choice(NUMS)), "getFollowSpeed")
add("vehicle", lambda r, i: (r.choice(i["vid"]), r.choice([0, 1, 2, 99]), r.choice(KEYS)), "getStopParameter")
# vehicle setters (read-back happens through a later getter in the sequence)
for m in ["setSpeed", "setMaxSpeed", "setMinGap", "setLength", "setWidth", "setHeight", "setMass",
          "setSpeedFactor", "setAccel", "setDecel", "setEmergencyDecel", "setApparentDecel", "setTau",
          "setImpatience", "setImperfection", "setMaxSpeedLat", "setMinGapLat", "setBoardingDuration",
          "setActionStepLength", "setPreviousSpeed"]:
    add("vehicle", _vnum, m)
for m in ["setSpeedMode", "setLaneChangeMode", "setRoutingMode"]:
    add("vehicle", lambda r, i: (r.choice(i["vid"]), r.choice([0, 1, 31, 99, -1])), m)
for m in ["setVehicleClass", "setEmissionClass", "setShapeClass", "setType", "setLine"]:
    add("vehicle", lambda r, i: (r.choice(i["vid"]), r.choice(["bus", "passenger", "nope", ""])), m)
add("vehicle", _vkeyval, "setParameter")
add("vehicle", lambda r, i: (r.choice(i["vid"]), r.choice(COLORS)), "setColor")
add("vehicle", lambda r, i: (r.choice(i["vid"]), r.choice(["center", "left", "right", "nope"])), "setLateralAlignment")
add("vehicle", lambda r, i: (r.choice(i["vid"]), r.choice([0, 1, 4, 31, 255])), "setSignals")

# lane
add("lane", _zero, "getIDList", "getIDCount")
for m in ["getLength", "getMaxSpeed", "getWidth", "getAllowed", "getDisallowed", "getShape",
          "getLinks", "getLinkNumber", "getInternalFoes", "getEdgeID", "getLastStepVehicleNumber",
          "getLastStepMeanSpeed", "getLastStepVehicleIDs", "getLastStepHaltingNumber",
          "getLastStepOccupancy", "getLastStepLength", "getWaitingTime", "getTraveltime",
          "getBidiLane", "getPendingVehicles", "getCO2Emission", "getFuelConsumption",
          "getNoiseEmission", "getElectricityConsumption", "getFriction"]:
    add("lane", _pick("lid"), m)
add("lane", lambda r, i: (r.choice(i["lid"]), r.choice([0, 1, 2, -1])), "getChangePermissions")
add("lane", _lnum, "getAngle")
add("lane", _pick2("lid", "lid"), "getFoes")
add("lane", lambda r, i: (r.choice(i["lid"]), r.choice(KEYS)), "getParameter")
add("lane", _lnum, "setMaxSpeed", "setLength", "setFriction")
add("lane", lambda r, i: (r.choice(i["lid"]), r.choice([["passenger"], ["bus"], ["all"], ["nope"], []])),
    "setAllowed", "setDisallowed")
add("lane", lambda r, i: (r.choice(i["lid"]), r.choice([["passenger"], ["all"]]), r.choice([0, 1, 2])),
    "setChangePermissions")

# edge
add("edge", _zero, "getIDList", "getIDCount")
for m in ["getLaneNumber", "getStreetName", "getMeanFriction", "getLastStepOccupancy", "getLastStepLength",
          "getFromJunction", "getToJunction", "getBidiEdge", "getLastStepPersonIDs", "getPendingVehicles",
          "getWaitingTime", "getLastStepHaltingNumber", "getLastStepVehicleNumber", "getLastStepMeanSpeed",
          "getLastStepVehicleIDs", "getTraveltime", "getCO2Emission", "getFuelConsumption", "getNoiseEmission",
          "getElectricityConsumption"]:
    add("edge", _pick("eid"), m)
add("edge", _enum, "getEffort", "getAdaptedTraveltime", "getAngle")
add("edge", lambda r, i: (r.choice(i["eid"]), r.choice(KEYS)), "getParameter")
add("edge", _enum, "setEffort", "setMaxSpeed")
add("edge", lambda r, i: (r.choice(i["eid"]), r.choice([["passenger"], ["bus"], ["all"], ["nope"]])),
    "setAllowed", "setDisallowed")

# traffic light (junction has none -> exercises error paths; also harmless on signal net)
add("trafficlight", _zero, "getIDList", "getIDCount")
for m in ["getRedYellowGreenState", "getPhase", "getPhaseDuration", "getNextSwitch", "getSpentDuration",
          "getProgram", "getControlledLanes", "getControlledLinks",
          "getAllProgramLogics", "getCompleteRedYellowGreenDefinition", "getPhaseName", "getParameter",
          "getServedPersonCount", "getBlockingVehicles", "getPriorityVehicles"]:
    add("trafficlight", _pick("tlid"), m)
add("trafficlight", lambda r, i: (r.choice(i["tlid"]), r.choice([0, 1, 99, -1])), "setPhase")
add("trafficlight", _vnum, "setPhaseDuration")  # uses vid list as tls id (same "nope"/"" shape)
add("trafficlight", _vstate, "setRedYellowGreenState")
add("trafficlight", lambda r, i: (r.choice(i["tlid"]), r.choice(["0", "nope", ""])), "setProgram", "setPhaseName")
add("trafficlight", lambda r, i: (r.choice(i["tlid"]), r.choice([0, 1, 99]), r.choice(STATES)), "setLinkState")

# route
add("route", _zero, "getIDList", "getIDCount")
add("route", _pick("route"), "getEdges", "getParameter")
add("route", _routes, "add")
add("route", _pick("route"), "remove")

# vehicle type
add("vehicletype", _zero, "getIDList", "getIDCount")
for m in ["getMaxSpeed", "getLength", "getAccel", "getDecel", "getEmergencyDecel", "getApparentDecel",
          "getTau", "getMinGap", "getWidth", "getHeight", "getMass", "getColor", "getVehicleClass",
          "getEmissionClass", "getShapeClass", "getSpeedFactor", "getSpeedDeviation", "getActionStepLength",
          "getImpatience", "getBoardingDuration", "getScale", "getLateralAlignment", "getMinGapLat",
          "getMaxSpeedLat", "getImperfection", "getPersonCapacity", "getParameter"]:
    add("vehicletype", _pick("vtid"), m)
for m in ["setMaxSpeed", "setLength", "setAccel", "setDecel", "setTau", "setMinGap", "setWidth",
          "setHeight", "setMass", "setSpeedFactor", "setSpeedDeviation", "setActionStepLength",
          "setImpatience", "setBoardingDuration", "setScale", "setMinGapLat", "setMaxSpeedLat",
          "setImperfection"]:
    add("vehicletype", lambda r, i: (r.choice(i["vtid"]), r.choice(NUMS)), m)
add("vehicletype", lambda r, i: (r.choice(i["vtid"]), r.choice(KEYS), r.choice(VALS)), "setParameter")

# person
add("person", _zero, "getIDList", "getIDCount")
for m in ["getPosition", "getRoadID", "getLaneID", "getLanePosition", "getSpeed", "getAngle", "getSlope",
          "getTypeID", "getVehicle", "getEdges", "getNextEdge", "getRemainingStages",
          "getWaitingTime", "getParameter"]:
    add("person", _pick("person"), m)
add("person", lambda r, i: (r.choice(i["person"]), r.choice([0, 1, 5, -1])), "getStage")
add("person", lambda r, i: (r.choice(i["person"]), r.choice(i["eid"]), r.choice(POS)), "getWalkingDistance")

# poi / polygon
add("poi", _zero, "getIDList", "getIDCount")
for m in ["getPosition", "getType", "getColor", "getWidth", "getHeight", "getAngle", "getImageFile", "getParameter"]:
    add("poi", _pick("poi"), m)
add("poi", lambda r, i: (r.choice(i["poi"]), r.choice(NUMS), r.choice(NUMS)), "setPosition")
add("polygon", _zero, "getIDList", "getIDCount")
for m in ["getShape", "getColor", "getFilled", "getType", "getLineWidth", "getParameter"]:
    add("polygon", _pick("poly"), m)
add("polygon", lambda r, i: (r.choice(i["poly"]), r.choice([[(0.0, 0.0), (1.0, 0.0)], [(0.0, 0.0)], "nope"])), "setShape")

# detectors / other infrastructure
add("inductionloop", _zero, "getIDList", "getIDCount")
for m in ["getPosition", "getLaneID", "getLastStepVehicleNumber", "getLastStepMeanSpeed", "getLastStepOccupancy",
          "getLastStepMeanLength", "getTimeSinceDetection", "getVehicleData", "getIntervalOccupancy",
          "getIntervalMeanSpeed", "getIntervalVehicleNumber", "getIntervalVehicleIDs", "getLastIntervalOccupancy",
          "getLastIntervalVehicleNumber", "getParameter"]:
    add("inductionloop", _pick("il"), m)
add("inductionloop", _vnum, "overrideTimeSinceDetection")
add("lanearea", _zero, "getIDList", "getIDCount")
for m in ["getPosition", "getLaneID", "getLength", "getLastStepVehicleNumber", "getLastStepMeanSpeed",
          "getLastStepOccupancy", "getLastStepHaltingNumber", "getJamLengthVehicle", "getJamLengthMeters",
          "getIntervalOccupancy", "getIntervalMeanSpeed", "getIntervalVehicleNumber", "getLastIntervalOccupancy",
          "getParameter"]:
    add("lanearea", _pick("la"), m)
add("lanearea", lambda r, i: (r.choice(i["la"]), r.choice([0, 3, -1])), "overrideVehicleNumber")
add("multientryexit", _zero, "getIDList", "getIDCount")
for m in ["getEntryLanes", "getExitLanes", "getEntryPositions", "getExitPositions", "getLastStepVehicleNumber",
          "getLastStepMeanSpeed", "getLastIntervalMeanTravelTime", "getParameter"]:
    add("multientryexit", _pick("me"), m)
add("junction", _zero, "getIDList", "getIDCount")
for m in ["getPosition", "getShape", "getIncomingEdges", "getOutgoingEdges", "getParameter"]:
    add("junction", _pick("jid"), m)


def build_calls(seed, n=45):
    """Deterministic call list for one seed: (label, callable(traci))."""
    r = random.Random(seed)
    calls = []
    for _ in range(n):
        dom, meth, gen = r.choice(SPECS)
        args = gen(r, IDS)
        label = f"{dom}.{meth}{args}"
        calls.append((label, lambda t, d=dom, m=meth, a=args: getattr(getattr(t, d), m)(*a)))
    return calls


def classify(fn, t):
    """Return (result dict, connection_dead)."""
    try:
        v = fn(t)
        return {"type": type(v).__name__, "value": repr(v)}, False
    except FatalTraCIError as e:
        return {"type": "FatalTraCIError", "value": str(e)}, True
    except TraCIException as e:
        return {"type": "TraCIException", "value": str(e)}, False
    except Exception as e:  # e.g. ValueError from the python client
        return {"type": type(e).__name__, "value": str(e)}, False


def dump_log(side, port, proc=None, n=30):
    log = Path(os.environ.get("TEMP", "/tmp")) / f"fuzz_{side}_{port}.log"
    code = proc.poll() if proc is not None else "n/a"
    try:
        lines = log.read_text(encoding="utf-8", errors="replace").splitlines()
        print(f"  {side} log tail (port {port}, exit={code}): " + " | ".join(lines[-n:]))
    except OSError as e:
        print(f"  {side} log unavailable (exit={code}): {e}")


def run_session(side, port, ours, calls, step_every, use_infra=True):
    """Run the whole call sequence on one server; return (results, dead_from, init_error)."""
    extra = ("--additional-files", str(INFRA)) if use_infra else ()
    log = Path(os.environ.get("TEMP", "/tmp")) / f"fuzz_{side}_{port}.log"
    with open(log, "a") as fh:
        proc = subprocess.Popen(C.sumo_command(side, port, NET, ROU, ours, extra),
                                cwd=str(C.CASE), stdout=fh, stderr=subprocess.STDOUT)
        results = []
        dead_from = None
        try:
            try:
                # a crashed previous session can leave a stale connection registered even
                # when close() itself raises; clear the pool unconditionally before reconnecting
                TCONN._connections.clear()
                try:
                    traci.close()
                except Exception:
                    pass
                TCONN._connections.clear()
                traci.init(port=port, numRetries=3, host="localhost")
            except Exception as e:
                dump_log(side, port, proc)
                return [], 0, {"type": "InitFailed:" + type(e).__name__, "value": str(e)}
            try:
                traci.simulationStep(0)
                traci.simulationStep(10)
                for k, (label, fn) in enumerate(calls):
                    res, dead = classify(fn, traci)
                    results.append(res)
                    if dead:
                        dead_from = k
                        if side == "ours":
                            dump_log(side, port, proc)
                        break
                    if step_every and (k + 1) % step_every == 0:
                        try:
                            traci.simulationStep()
                        except Exception:
                            pass
            finally:
                try:
                    traci.close()
                except Exception:
                    pass
                TCONN._connections.clear()
        finally:
            try:
                proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                proc.kill()
    return results, dead_from, None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--ours", default=str(C.DEFAULT_OURS))
    ap.add_argument("--port", type=int, default=19500)
    ap.add_argument("--seeds", default="1,2,3,4,5")
    ap.add_argument("--n", type=int, default=45, help="calls per session")
    ap.add_argument("--step-every", type=int, default=5)
    ap.add_argument("--no-infra", action="store_true", help="do not load infra.add.xml")
    ap.add_argument("--case", default="junction", choices=["junction", "signal"])
    a = ap.parse_args()
    set_case(a.case)

    seeds = [int(s) for s in a.seeds.split(",") if s]
    diffs = 0
    compared = 0
    for si, seed in enumerate(seeds):
        calls = build_calls(seed, a.n)
        port = a.port + si * 4
        ours_res, ours_dead, ours_init = run_session("ours", port, a.ours, calls, a.step_every, not a.no_infra)
        C.kill_cpp(port)
        cpp_res, cpp_dead, cpp_init = run_session("cpp", port + 2, a.ours, calls, a.step_every, not a.no_infra)
        C.kill_cpp(port + 2)

        dead_label = calls[ours_dead][0] if ours_dead is not None and ours_dead < len(calls) else "-"
        dead_res = ours_res[ours_dead] if ours_dead is not None and ours_dead < len(ours_res) else "-"
        print(f"FUZZ case={a.case} seed={seed} infra={not a.no_infra}: ours={len(ours_res)} calls (dead={ours_dead}@{dead_label}, init={ours_init}) "
              f"official={len(cpp_res)} calls (dead={cpp_dead}, init={cpp_init}) our_dead_result={dead_res}")
        if ours_init or cpp_init:
            # an init failure on either side is itself a finding (unless official just died)
            if not cpp_init:
                print(f"NEW-PROBE DIFF seed={seed} init: ours {ours_init} | official {cpp_init}")
                diffs += 1
            continue
        limit = min(len(ours_res), len(cpp_res))
        for k in range(limit):
            o, c = ours_res[k], cpp_res[k]
            # ignore cases where the official SUMO itself died/crashed
            if c["type"] in ("FatalTraCIError", "InitFailed") and "Connection" in c["value"]:
                break
            compared += 1
            if o != c:
                diffs += 1
                label = calls[k][0]
                print(f"NEW-PROBE DIFF seed={seed} step={k} {label}: "
                      f"ours {o['type']} {o['value']} | official {c['type']} {c['value']}")
        # a divergence where ours died but official kept going is a real bug
        if ours_dead is not None and (cpp_dead is None or ours_dead < cpp_dead):
            k = ours_dead
            if k < len(cpp_res):
                diffs += 1
                print(f"NEW-PROBE DIFF seed={seed} step={k} OURS-CRASH {calls[k][0]}: "
                      f"ours {ours_res[k]} | official {cpp_res[k]}")
    print(f"FUZZ TOTAL: {diffs} differences across {compared} compared calls")
    return 0


if __name__ == "__main__":
    sys.exit(main())
