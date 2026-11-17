"""TraCI oracle: drive our sumo.exe and the C++ sumo over TraCI and compare the answers.

Each probe runs in its own server session (prelude: two simulation steps), so one crash on our side
only loses its own row. Usage:
  python tools/oracle/cases/traci_basic/client.py                # both cases, both sides, one table each
  python tools/oracle/cases/traci_basic/client.py --case signal --only ours
Options: --ours <sumo.exe> (default: $CARGO_TARGET_DIR/debug/sumo.exe)  --port <n> (default 8813)
The C++ side runs in WSL: the untouched official SUMO 1.28.0 at $HOME/sumo-pristine.
"""
import argparse
import json
import os
import subprocess
import sys
import time
import xml.etree.ElementTree as ET
from collections import Counter
from pathlib import Path

import traci
from traci import constants as tc
from traci.exceptions import TraCIException

CASE = Path(__file__).resolve().parent
CASES_DIR = CASE.parent
DEFAULT_OURS = Path(os.environ.get("CARGO_TARGET_DIR", "C:/Users/karti/sumo-rs-target")) / "debug" / "sumo.exe"
# Linux runner (Kaggle): official binary dir, run directly instead of through WSL
PRISTINE_BIN = os.environ.get("SUMO_PRISTINE_BIN")


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
    "signal_actuated": ("-324752522_0", "1802245009"),
}


def wsl_path(p):
    p = str(Path(p).resolve()).replace("\\", "/")
    return "/mnt/" + p[0].lower() + p[2:]


def sumo_command(side, port, net, rou, ours, extra=()):
    if side == "ours":
        return [str(ours), "-n", str(net), "-r", str(rou), "--remote-port", str(port),
                "--no-step-log", "true", "--no-warnings", "true", *extra]
    if PRISTINE_BIN:
        return [f"{PRISTINE_BIN}/sumo", "-n", str(net), "-r", str(rou), "--remote-port", str(port),
                "--no-step-log", "true", "--no-warnings", "true", *extra]
    flags = "".join(f" {x}" for x in extra)
    # reference = untouched official SUMO 1.28.0; ~/sumo-src is an instrumentation copy and must never be the oracle
    inner = (f"SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo "
             f"$HOME/sumo-pristine/bin/sumo -n {wsl_path(net)} -r {wsl_path(rou)} "
             f"--remote-port {port} --no-step-log true --no-warnings true{flags}")
    return ["wsl", "-e", "bash", "-lc", inner]


# the tripinfo device exists only when tripinfo-output or duration-log.statistics is set (C++ buildVehicleDevices)
TRIPINFO_PROBE = "vehicle.getParameter(v0, device.tripinfo.waitingTime)"


STATE_PROBE_FILE = os.environ.get("TRACI_STATE_FILE", "traci_probe_state.xml")


def save_load_probe(t):
    t.simulation.saveState(STATE_PROBE_FILE)
    tags = sorted(Counter(el.tag for el in ET.parse(STATE_PROBE_FILE).getroot().iter()).items())
    t.simulation.loadState(STATE_PROBE_FILE)
    t.simulationStep(11)
    return (t.simulation.getTime(), sorted(t.vehicle.getIDList()), tags)


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

    def veh_highlight_fade(t):
        t.vehicle.highlight("v0", (0, 255, 0, 255), 5.0, 200, 3.0)
        t.simulationStep(11)
        return (sorted(i for i in t.polygon.getIDList() if "_hl" in i), t.polygon.getShape("v0_hl0"), t.polygon.getColor("v0_hl0"))

    def veh_highlight_default(t):
        t.vehicle.highlight("v0")
        return (sorted(i for i in t.polygon.getIDList() if "_hl" in i), t.polygon.getShape("v0_hl0"), t.polygon.getColor("v0_hl0"))

    edge = lane.rsplit("_", 1)[0]
    tl = tl_id or "none"

    def edge_set_max_speed(t):
        t.edge.setMaxSpeed(edge, 5.0)
        t.simulationStep(11)
        return t.vehicle.getSpeed("v0")

    def lane_set_max_speed(t):
        t.lane.setMaxSpeed(lane, 5.0)
        t.simulationStep(11)
        return t.lane.getMaxSpeed(lane)

    def lane_set_max_speed_effect(t):
        t.lane.setMaxSpeed(lane, 5.0)
        t.simulationStep(11)
        return t.vehicle.getSpeed("v0")

    def slow_down(t):
        t.vehicle.slowDown("v0", 3.0, 5.0)
        t.simulationStep(11)
        return t.vehicle.getSpeed("v0")

    def move_to_lane(t):
        t.vehicle.moveTo("v0", lane, t.vehicle.getLanePosition("v0"))
        t.simulationStep(11)
        return t.vehicle.getLaneID("v0")

    def set_color(t):
        t.vehicle.setColor("v0", (255, 0, 0, 255))
        return t.vehicle.getColor("v0")

    def set_type(t):
        t.vehicle.setType("v0", "DEFAULT_VEHTYPE")
        return t.vehicle.getTypeID("v0")

    def change_lane_effect(t):
        t.vehicle.changeLane("v0", t.vehicle.getLaneIndex("v0") ^ 1, 5.0)
        t.simulationStep(11)
        return t.vehicle.getLaneID("v0")

    def vtype_set_max_speed(t):
        t.vehicletype.setMaxSpeed("DEFAULT_VEHTYPE", 20.0)
        t.simulationStep(11)
        return t.vehicletype.getMaxSpeed("DEFAULT_VEHTYPE")

    def tl_set_phase(t):
        t.trafficlight.setPhase(tl, 1)
        return t.trafficlight.getPhase(tl)

    def tl_set_phase_duration(t):
        t.trafficlight.setPhaseDuration(tl, 7.0)
        t.simulationStep(11)
        return t.trafficlight.getPhaseDuration(tl)

    def tl_set_state(t):
        s = t.trafficlight.getRedYellowGreenState(tl)
        t.trafficlight.setRedYellowGreenState(tl, "G" * len(s))
        t.simulationStep(11)
        return t.trafficlight.getRedYellowGreenState(tl)

    def poi_add(t):
        t.poi.add("p0", 1.0, 2.0, (255, 0, 0, 255))
        return t.poi.getPosition("p0")

    def sim_subscribe_departed_3(t):
        t.simulation.subscribe([tc.VAR_DEPARTED_VEHICLES_IDS])
        for s in (11, 12, 13):
            t.simulationStep(s)
        return t.simulation.getSubscriptionResults()

    def veh_subscribe_3(t):
        t.vehicle.subscribe("v0", [tc.VAR_SPEED, tc.VAR_POSITION])
        for s in (11, 12, 13):
            t.simulationStep(s)
        return t.vehicle.getSubscriptionResults("v0")

    def veh_subscribe_context(t):
        t.vehicle.subscribeContext("v0", tc.CMD_GET_VEHICLE_VARIABLE, 0.0, [tc.VAR_SPEED])
        t.simulationStep(11)
        return t.vehicle.getContextSubscriptionResults("v0")

    def edge_subscribe(t):
        t.edge.subscribe(edge, [tc.LAST_STEP_VEHICLE_NUMBER])
        t.simulationStep(11)
        return t.edge.getSubscriptionResults(edge)

    def sim_find_route(t):
        return t.simulation.findRoute(edge, t.vehicle.getRoute("v0")[-1]).edges

    def sim_convert_road(t):
        x, y = t.vehicle.getPosition("v0")
        return t.simulation.convertRoad(x, y)

    def sim_distance_2d(t):
        x, y = t.vehicle.getPosition("v0")
        return t.simulation.getDistance2D(x, y, x + 3.0, y + 4.0)

    def run_n(t, n):
        t0 = int(t.simulation.getTime())
        for s in range(t0 + 1, t0 + n + 1):
            t.simulationStep(s)

    def summary(t):
        ids = sorted(t.vehicle.getIDList())
        return ([(i, t.vehicle.getPosition(i), t.vehicle.getSpeed(i), t.vehicle.getLaneID(i)) for i in ids],
                t.simulation.getArrivedNumber(), sorted(t.simulation.getArrivedIDList()))

    def spawn(t):
        t.vehicle.add("vfx", t.route.getIDList()[0])
        run_n(t, 1)

    def trace_tl(t, n):
        out = []
        for _ in range(n):
            run_n(t, 1)
            out.append((t.trafficlight.getRedYellowGreenState(tl), t.trafficlight.getPhase(tl),
                        t.trafficlight.getNextSwitch(tl)))
        return out

    def lane_close(t):
        t.lane.setDisallowed(lane, ["all"])
        run_n(t, 30)
        return summary(t)

    def lane_close_reopen(t):
        t.lane.setDisallowed(lane, ["all"])
        run_n(t, 15)
        t.lane.setDisallowed(lane, [])
        run_n(t, 30)
        return summary(t)

    def lane_disallow_passenger(t):
        t.lane.setDisallowed(lane, ["passenger"])
        run_n(t, 30)
        return summary(t)

    def edge_max_speed_5_30(t):
        t.edge.setMaxSpeed(edge, 5.0)
        run_n(t, 30)
        return summary(t)

    def edge_max_speed_20_30(t):
        t.edge.setMaxSpeed(edge, 20.0)
        run_n(t, 30)
        return summary(t)

    def lane_max_speed_3_30(t):
        t.lane.setMaxSpeed(lane, 3.0)
        run_n(t, 30)
        return summary(t)

    def lane_max_speed_reset(t):
        t.lane.setMaxSpeed(lane, 5.0)
        run_n(t, 10)
        t.lane.setMaxSpeed(lane, 30.0)
        run_n(t, 20)
        return summary(t)

    def tl_set_program_30(t):
        t.trafficlight.setProgram(tl, "0")
        return trace_tl(t, 30)

    def tl_set_phase_held(t):
        t.trafficlight.setPhase(tl, 1)
        return trace_tl(t, 30)

    def tl_set_phase_then_duration(t):
        t.trafficlight.setPhase(tl, 2)
        t.trafficlight.setPhaseDuration(tl, 20.0)
        return trace_tl(t, 30)

    def tl_set_phase_duration_7(t):
        t.trafficlight.setPhaseDuration(tl, 7.0)
        return trace_tl(t, 30)

    def tl_set_phase_0_trace(t):
        t.trafficlight.setPhase(tl, 0)
        return trace_tl(t, 30)

    def tl_set_state_all_g_held(t):
        s = t.trafficlight.getRedYellowGreenState(tl)
        out = []
        for _ in range(30):
            t.trafficlight.setRedYellowGreenState(tl, "G" * len(s))
            run_n(t, 1)
            out.append(t.trafficlight.getRedYellowGreenState(tl))
        return out

    def tl_set_state_all_r_held(t):
        s = t.trafficlight.getRedYellowGreenState(tl)
        out = []
        for _ in range(30):
            t.trafficlight.setRedYellowGreenState(tl, "r" * len(s))
            run_n(t, 1)
            out.append(t.trafficlight.getRedYellowGreenState(tl))
        return out

    def tl_set_state_wrong_len(t):
        t.trafficlight.setRedYellowGreenState(tl, "G")
        return trace_tl(t, 5)

    def veh_speed_mode_30(t):
        spawn(t)
        t.vehicle.setSpeedMode("vfx", 0)
        t.vehicle.setSpeed("vfx", 20.0)
        run_n(t, 30)
        return summary(t)

    def veh_speed_release(t):
        spawn(t)
        t.vehicle.setSpeed("vfx", 0.0)
        run_n(t, 10)
        t.vehicle.setSpeed("vfx", -1.0)
        run_n(t, 20)
        return summary(t)

    def veh_lane_change_mode_off(t):
        spawn(t)
        t.vehicle.setLaneChangeMode("vfx", 0)
        run_n(t, 30)
        return summary(t)

    def veh_change_lane_after_mode_off(t):
        spawn(t)
        t.vehicle.setLaneChangeMode("vfx", 0)
        t.vehicle.changeLane("vfx", t.vehicle.getLaneIndex("vfx") ^ 1, 5.0)
        run_n(t, 30)
        return summary(t)

    def veh_slow_down_30(t):
        spawn(t)
        t.vehicle.slowDown("vfx", 3.0, 5.0)
        run_n(t, 30)
        return summary(t)

    def veh_change_target_30(t):
        spawn(t)
        t.vehicle.changeTarget("vfx", t.vehicle.getRoute("vfx")[-1])
        run_n(t, 30)
        return summary(t)

    def veh_set_route_prefix(t):
        spawn(t)
        t.vehicle.setRoute("vfx", t.vehicle.getRoute("vfx")[:1])
        run_n(t, 30)
        return summary(t)

    def veh_set_route_reversed(t):
        spawn(t)
        t.vehicle.setRoute("vfx", t.vehicle.getRoute("vfx")[::-1])
        run_n(t, 30)
        return summary(t)

    def veh_move_keep_route(keep):
        def run(t):
            spawn(t)
            x, y = t.vehicle.getPosition("vfx")
            t.vehicle.moveToXY("vfx", t.vehicle.getRoadID("vfx"), t.vehicle.getLaneIndex("vfx"), x + 1.0, y,
                               keepRoute=keep)
            run_n(t, 30)
            return summary(t)
        return run

    def veh_add_depart_args(t):
        t.vehicle.add("vnew", t.route.getIDList()[0], departLane=0, departSpeed=5.0)
        run_n(t, 30)
        return summary(t)

    def veh_add_depart_best_max(t):
        t.vehicle.add("vnew", t.route.getIDList()[0], departLane="best", departSpeed="max")
        run_n(t, 30)
        return summary(t)

    def veh_remove_in_crossing(t):
        chosen = None
        for _ in range(200):
            run_n(t, 1)
            cross = sorted(i for i in t.vehicle.getIDList() if t.vehicle.getRoadID(i).startswith(":"))
            if cross:
                chosen = cross[0]
                t.vehicle.remove(chosen)
                break
        run_n(t, 30)
        return (chosen, summary(t))

    def veh_remove_v0_30(t):
        t.vehicle.remove("v0")
        run_n(t, 30)
        return summary(t)

    def sim_step_jump(t):
        r = t.simulationStep(40)
        return (r, t.simulation.getTime(), summary(t))

    def sim_step_fraction(t):
        t.simulationStep(25.25)
        return (t.simulation.getTime(), summary(t))

    def veh_subscribe_stream_50(t):
        spawn(t)
        t.vehicle.subscribe("vfx", [tc.VAR_SPEED, tc.VAR_POSITION, tc.VAR_LANE_ID, tc.VAR_ROAD_ID])
        out = []
        for _ in range(50):
            run_n(t, 1)
            out.append(t.vehicle.getSubscriptionResults("vfx"))
        return out

    def sim_subscribe_stream_50(t):
        t.simulation.subscribe([tc.VAR_DEPARTED_VEHICLES_IDS, tc.VAR_ARRIVED_VEHICLES_IDS])
        out = []
        for _ in range(50):
            run_n(t, 1)
            out.append(t.simulation.getSubscriptionResults())
        return out

    def edge_subscribe_stream_30(t):
        t.edge.subscribe(edge, [tc.LAST_STEP_VEHICLE_NUMBER, tc.LAST_STEP_MEAN_SPEED])
        out = []
        for _ in range(30):
            run_n(t, 1)
            out.append(t.edge.getSubscriptionResults(edge))
        return out

    def veh_context_stream_20(t):
        spawn(t)
        t.vehicle.subscribeContext("vfx", tc.CMD_GET_VEHICLE_VARIABLE, 50.0, [tc.VAR_SPEED])
        out = []
        for _ in range(20):
            run_n(t, 1)
            out.append(t.vehicle.getContextSubscriptionResults("vfx"))
        return out

    def veh_context_stream_20_wide(t):
        spawn(t)
        t.vehicle.subscribeContext("vfx", tc.CMD_GET_VEHICLE_VARIABLE, 100.0, [tc.VAR_POSITION, tc.VAR_SPEED])
        out = []
        for _ in range(20):
            run_n(t, 1)
            out.append(t.vehicle.getContextSubscriptionResults("vfx"))
        return out

    def vtype_max_speed_30(t):
        t.vehicletype.setMaxSpeed("DEFAULT_VEHTYPE", 20.0)
        run_n(t, 30)
        return summary(t)

    def two_stops(t):
        last = t.vehicle.getRoute("v0")[-1]
        t.vehicle.setStop("v0", last, pos=10.0)
        t.vehicle.setStop("v0", last, pos=20.0)
        return last

    def stop_replace(t):
        last = two_stops(t)
        t.vehicle.replaceStop("v0", 1, last, pos=30.0, duration=2.0)
        return t.vehicle.getNextStops("v0")

    def stop_insert(t):
        last = two_stops(t)
        t.vehicle.insertStop("v0", 1, last, pos=30.0, duration=2.0)
        return t.vehicle.getNextStops("v0")

    def reroute_between_stops(t):
        two_stops(t)
        t.vehicle.rerouteTraveltime("v0")
        return t.vehicle.getRoute("v0")

    def tl_set_param_get(key, value):
        def f(t):
            t.trafficlight.setParameter(tl, key, value)
            return t.trafficlight.getParameter(tl, key)
        return f

    base = {
        "tl param: getParameter max-gap": lambda t: t.trafficlight.getParameter(tl, "max-gap"),
        "tl param: getParameter cycleTime (simple)": lambda t: t.trafficlight.getParameter(tl, "cycleTime"),
        "tl param: getParameter typeName (simple)": lambda t: t.trafficlight.getParameter(tl, "typeName"),
        "tl param: getParameter NEMA.phaseCall": lambda t: t.trafficlight.getParameter(tl, "NEMA.phaseCall"),
        "tl param: getParameter condition.none": lambda t: t.trafficlight.getParameter(tl, "condition.none"),
        "tl param: setParameter max-gap 4.5 + get": tl_set_param_get("max-gap", "4.5"),
        "tl param: setParameter inactive-threshold 7 + get": tl_set_param_get("inactive-threshold", "7"),
        "tl param: setParameter cycleTime 42 + get": tl_set_param_get("cycleTime", "42"),
        "tl param: setParameter passing-time (refused)": lambda t: t.trafficlight.setParameter(tl, "passing-time", "5"),
        "tl param: setParameter typeName (refused)": lambda t: t.trafficlight.setParameter(tl, "typeName", "x"),
        "tl param: setParameter NEMA.cycleLength (not NEMA)": lambda t: t.trafficlight.setParameter(tl, "NEMA.cycleLength", "90"),
        "tl param: setProgram unknown id": lambda t: t.trafficlight.setProgram(tl, "nope"),
        "effect: lane.setDisallowed(all) +30 + summary": lane_close,
        "effect: lane.setDisallowed(all) +15, reopen +30": lane_close_reopen,
        "effect: lane.setDisallowed(passenger) +30": lane_disallow_passenger,
        "effect: lane.setDisallowed(nope, all)": lambda t: t.lane.setDisallowed("nope", ["all"]),
        "effect: edge.setMaxSpeed(5) +30 + summary": edge_max_speed_5_30,
        "effect: edge.setMaxSpeed(20) +30 + summary": edge_max_speed_20_30,
        "effect: lane.setMaxSpeed(3) +30 + summary": lane_max_speed_3_30,
        "effect: lane.setMaxSpeed(5)+10, (30)+20": lane_max_speed_reset,
        "effect: trafficlight.setProgram(0) +30 trace": tl_set_program_30,
        "effect: trafficlight.setProgram(nope)": lambda t: t.trafficlight.setProgram(tl, "nope"),
        "effect: trafficlight.setPhase(1) held +30": tl_set_phase_held,
        "effect: setPhase(2)+setPhaseDuration(20) +30": tl_set_phase_then_duration,
        "effect: trafficlight.setPhaseDuration(7) +30": tl_set_phase_duration_7,
        "effect: trafficlight.setPhase(0) +30 trace": tl_set_phase_0_trace,
        "effect: setRedYellowGreenState(all G) held 30": tl_set_state_all_g_held,
        "effect: setRedYellowGreenState(all r) held 30": tl_set_state_all_r_held,
        "effect: setRedYellowGreenState(wrong len) +5": tl_set_state_wrong_len,
        "effect: trafficlight.setPhase(tl, 99)": lambda t: t.trafficlight.setPhase(tl, 99),
        "effect: vehicle.setSpeedMode(0)+setSpeed(20) +30": veh_speed_mode_30,
        "effect: setSpeed(0)+10, setSpeed(-1)+20": veh_speed_release,
        "effect: setLaneChangeMode(0) +30": veh_lane_change_mode_off,
        "effect: setLaneChangeMode(0)+changeLane +30": veh_change_lane_after_mode_off,
        "effect: vehicle.slowDown(3,5) +30": veh_slow_down_30,
        "effect: vehicle.changeTarget(last) +30": veh_change_target_30,
        "effect: vehicle.setRoute(prefix) +30": veh_set_route_prefix,
        "effect: vehicle.setRoute(reversed) +30": veh_set_route_reversed,
        "effect: moveToXY(keepRoute=0) +30": veh_move_keep_route(0),
        "effect: moveToXY(keepRoute=1) +30": veh_move_keep_route(1),
        "effect: moveToXY(keepRoute=2) +30": veh_move_keep_route(2),
        "effect: vehicle.add(departLane=0,speed=5) +30": veh_add_depart_args,
        "effect: vehicle.add(best, max) +30": veh_add_depart_best_max,
        "effect: vehicle.remove in crossing +30": veh_remove_in_crossing,
        "effect: vehicle.remove(v0) +30": veh_remove_v0_30,
        "effect: simulationStep(40) jump": sim_step_jump,
        "effect: simulationStep(25.25) fractional": sim_step_fraction,
        "effect: vehicle.subscribe stream 50": veh_subscribe_stream_50,
        "effect: simulation.subscribe stream 50": sim_subscribe_stream_50,
        "effect: edge.subscribe stream 30": edge_subscribe_stream_30,
        "effect: vehicle.subscribeContext(50) stream 20": veh_context_stream_20,
        "effect: vehicle.subscribeContext(100) stream 20": veh_context_stream_20_wide,
        "effect: vehicletype.setMaxSpeed(20) +30": vtype_max_speed_30,
        "getVersion": lambda t: t.getVersion(),
        "simulationStep(11) returns": lambda t: t.simulationStep(11),
        "simulation.getTime": lambda t: t.simulation.getTime(),
        "simulation.getLoadedNumber": lambda t: t.simulation.getLoadedNumber(),
        "simulation.getArrivedNumber": lambda t: t.simulation.getArrivedNumber(),
        "vehicle.getIDList": lambda t: t.vehicle.getIDList(),
        "vehicle.getPosition(v0)": lambda t: t.vehicle.getPosition("v0"),
        "vehicle.getSpeed(v0)": lambda t: t.vehicle.getSpeed("v0"),
        TRIPINFO_PROBE: lambda t: t.vehicle.getParameter("v0", "device.tripinfo.waitingTime"),
        "vehicle.getParameter(v0, laneChangeModel.nope)": lambda t: t.vehicle.getParameter("v0", "laneChangeModel.nope"),
        "vehicle.getParameter(v0, carFollowModel.caccCommunicationsOverrideMode)": lambda t: t.vehicle.getParameter("v0", "carFollowModel.caccCommunicationsOverrideMode"),
        # LC2013 probability state is long long: C++ prints integer division (0.5 -> 0), not a fraction
        "lcm: set speedGainProbabilityRight 0.5 + get": lambda t: (t.vehicle.setParameter("v0", "laneChangeModel.speedGainProbabilityRight", "0.5"), t.vehicle.getParameter("v0", "laneChangeModel.speedGainProbabilityRight"))[1],
        "lcm: set speedGainProbabilityLeft 1.5 + get": lambda t: (t.vehicle.setParameter("v0", "laneChangeModel.speedGainProbabilityLeft", "1.5"), t.vehicle.getParameter("v0", "laneChangeModel.speedGainProbabilityLeft"))[1],
        "lcm: set keepRightProbability -2.5 + get": lambda t: (t.vehicle.setParameter("v0", "laneChangeModel.keepRightProbability", "-2.5"), t.vehicle.getParameter("v0", "laneChangeModel.keepRightProbability"))[1],
        "lcm: set keepRightProbability 2.5 + get": lambda t: (t.vehicle.setParameter("v0", "laneChangeModel.keepRightProbability", "2.5"), t.vehicle.getParameter("v0", "laneChangeModel.keepRightProbability"))[1],
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
        "edge.getIDList": lambda t: t.edge.getIDList(),
        "edge.getLastStepVehicleNumber(edge)": lambda t: t.edge.getLastStepVehicleNumber(edge),
        "edge.getLastStepMeanSpeed(edge)": lambda t: t.edge.getLastStepMeanSpeed(edge),
        "edge.getLastStepVehicleIDs(edge)": lambda t: t.edge.getLastStepVehicleIDs(edge),
        "edge.getTraveltime(edge)": lambda t: t.edge.getTraveltime(edge),
        "edge.getWaitingTime(edge)": lambda t: t.edge.getWaitingTime(edge),
        "edge.setMaxSpeed(edge,5) + step + getSpeed(v0)": edge_set_max_speed,
        "edge.getLastStepVehicleNumber(nope)": lambda t: t.edge.getLastStepVehicleNumber("nope"),
        "edge.getTraveltime(nope)": lambda t: t.edge.getTraveltime("nope"),
        "edge.getWaitingTime(nope)": lambda t: t.edge.getWaitingTime("nope"),
        "edge.setMaxSpeed(nope, 5)": lambda t: t.edge.setMaxSpeed("nope", 5.0),
        "edge.subscribe(edge, lastStepVehicleNumber) + step": edge_subscribe,
        "lane.setMaxSpeed(lane,5) + step + getMaxSpeed": lane_set_max_speed,
        "lane.setMaxSpeed(lane,5) + step + getSpeed(v0)": lane_set_max_speed_effect,
        "lane.getMaxSpeed(lane)": lambda t: t.lane.getMaxSpeed(lane),
        "lane.getWidth(lane)": lambda t: t.lane.getWidth(lane),
        "lane.getLastStepVehicleIDs(lane)": lambda t: t.lane.getLastStepVehicleIDs(lane),
        "lane.setMaxSpeed(nope, 5)": lambda t: t.lane.setMaxSpeed("nope", 5.0),
        "lane.getMaxSpeed(nope)": lambda t: t.lane.getMaxSpeed("nope"),
        "vehicle.getLaneID(v0)": lambda t: t.vehicle.getLaneID("v0"),
        "vehicle.getLanePosition(v0)": lambda t: t.vehicle.getLanePosition("v0"),
        "vehicle.getAngle(v0)": lambda t: t.vehicle.getAngle("v0"),
        "vehicle.getRoadID(v0)": lambda t: t.vehicle.getRoadID("v0"),
        "vehicle.getRouteID(v0)": lambda t: t.vehicle.getRouteID("v0"),
        "vehicle.getAcceleration(v0)": lambda t: t.vehicle.getAcceleration("v0"),
        "vehicle.getSignals(v0)": lambda t: t.vehicle.getSignals("v0"),
        "vehicle.getLeader(v0)": lambda t: t.vehicle.getLeader("v0"),
        "vehicle.getLeader(v0, 200)": lambda t: t.vehicle.getLeader("v0", 200.0),
        "vehicle.getDistance(v0)": lambda t: t.vehicle.getDistance("v0"),
        "vehicle.getTypeID(v0)": lambda t: t.vehicle.getTypeID("v0"),
        "vehicle.highlight(v0, fade) + step + polygons": veh_highlight_fade,
        "vehicle.highlight(v0, default) + polygons": veh_highlight_default,
        "vehicle.highlight(nope)": lambda t: t.vehicle.highlight("nope"),
        "vehicle.getLaneID(nope)": lambda t: t.vehicle.getLaneID("nope"),
        "vehicle.getLanePosition(nope)": lambda t: t.vehicle.getLanePosition("nope"),
        "vehicle.getAngle(nope)": lambda t: t.vehicle.getAngle("nope"),
        "vehicle.getRouteID(nope)": lambda t: t.vehicle.getRouteID("nope"),
        "vehicle.getLeader(nope)": lambda t: t.vehicle.getLeader("nope"),
        "vehicle.slowDown(v0,3,5) + step + getSpeed": slow_down,
        "vehicle.slowDown(nope,3,5)": lambda t: t.vehicle.slowDown("nope", 3.0, 5.0),
        "vehicle.moveTo(v0, lane, pos) + step + getLaneID": move_to_lane,
        "vehicle.moveTo(nope, lane, 0)": lambda t: t.vehicle.moveTo("nope", lane, 0.0),
        "vehicle.setColor(v0) + getColor": set_color,
        "vehicle.getColor(v0)": lambda t: t.vehicle.getColor("v0"),
        "vehicle.getColor(nope)": lambda t: t.vehicle.getColor("nope"),
        "vehicle.setType(v0, DEFAULT) + getTypeID": set_type,
        "vehicle.setType(v0, nope)": lambda t: t.vehicle.setType("v0", "nope"),
        "vehicle.changeLane(v0, 0, 1e30) duration range": lambda t: t.vehicle.changeLane("v0", 0, 1e30),
        "vehicle.slowDown(v0, 3, 1e30) duration range": lambda t: t.vehicle.slowDown("v0", 3.0, 1e30),
        "vehicle.setAcceleration(v0, 1, 1e30) duration range": lambda t: t.vehicle.setAcceleration("v0", 1.0, 1e30),
        "vehicle.setBoardingDuration(v0, 1e30) range": lambda t: t.vehicle.setBoardingDuration("v0", 1e30),
        "vehicletype.setBoardingDuration(DEFAULT, 1e30) range": lambda t: t.vehicletype.setBoardingDuration("DEFAULT_VEHTYPE", 1e30),
        "vehicle.setRouteID(v0, nope)": lambda t: t.vehicle.setRouteID("v0", "nope"),
        "vehicle.resume(v0) without stops": lambda t: t.vehicle.resume("v0"),
        "vehicle.changeLane(v0) + step + getLaneID": change_lane_effect,
        "vehicle.subscribe(v0) 3 steps + results": veh_subscribe_3,
        "vehicle.subscribeContext(v0, speed) + step": veh_subscribe_context,
        "vehicletype.getIDList": lambda t: t.vehicletype.getIDList(),
        "vehicletype.getMaxSpeed(DEFAULT)": lambda t: t.vehicletype.getMaxSpeed("DEFAULT_VEHTYPE"),
        "vehicletype.setMaxSpeed(DEFAULT,20) + step + getMaxSpeed": vtype_set_max_speed,
        "vehicletype.getLength(DEFAULT)": lambda t: t.vehicletype.getLength("DEFAULT_VEHTYPE"),
        "vehicletype.getMaxSpeed(nope)": lambda t: t.vehicletype.getMaxSpeed("nope"),
        "vehicletype.getLength(nope)": lambda t: t.vehicletype.getLength("nope"),
        "simulation.getDepartedIDList": lambda t: t.simulation.getDepartedIDList(),
        "simulation.getArrivedIDList": lambda t: t.simulation.getArrivedIDList(),
        "simulation.getCollidingVehiclesIDList": lambda t: t.simulation.getCollidingVehiclesIDList(),
        "simulation.subscribe(departedIDs) 3 steps": sim_subscribe_departed_3,
        "simulation.convert2D(edge, 0)": lambda t: t.simulation.convert2D(edge, 0.0),
        "simulation.convert2D(nope, 0)": lambda t: t.simulation.convert2D("nope", 0.0),
        "simulation.convertRoad(v0 pos)": sim_convert_road,
        "simulation.findRoute(edge, v0 last).edges": sim_find_route,
        "simulation.findRoute(nope, edge)": lambda t: t.simulation.findRoute("nope", edge).edges,
        "simulation.getDistance2D(3,4 offset)": sim_distance_2d,
        "simulation.getDeltaT": lambda t: t.simulation.getDeltaT(),
        "simulation.getCurrentTime": lambda t: t.simulation.getCurrentTime(),
        "simulation.saveState + loadState + step": save_load_probe,
        "trafficlight.setPhase(tl,1) + getPhase": tl_set_phase,
        "trafficlight.setPhaseDuration(tl,7) + step + get": tl_set_phase_duration,
        "trafficlight.getNextSwitch(tl)": lambda t: t.trafficlight.getNextSwitch(tl),
        "trafficlight.setRedYellowGreenState(all G) + step": tl_set_state,
        "trafficlight.setPhase(nope, 0)": lambda t: t.trafficlight.setPhase("nope", 0),
        "trafficlight.getNextSwitch(nope)": lambda t: t.trafficlight.getNextSwitch("nope"),
        "junction.getPosition(first)": lambda t: t.junction.getPosition(t.junction.getIDList()[0]),
        "junction.getPosition(nope)": lambda t: t.junction.getPosition("nope"),
        "poi.add(p0) + getPosition": poi_add,
        "poi.getPosition(nope)": lambda t: t.poi.getPosition("nope"),
        "vehicle.setStop(v0) x2 + getNextStops": lambda t: (two_stops(t), t.vehicle.getNextStops("v0"))[1],
        "vehicle.replaceStop(v0, 1, last) between stops + getNextStops": stop_replace,
        "vehicle.insertStop(v0, 1, last) between stops + getNextStops": stop_insert,
        "vehicle.replaceStop(v0, 5, last) index out of range": lambda t: (two_stops(t), t.vehicle.replaceStop("v0", 5, t.vehicle.getRoute("v0")[-1]))[1],
        "vehicle.insertStop(v0, 9, last) index out of range": lambda t: (two_stops(t), t.vehicle.insertStop("v0", 9, t.vehicle.getRoute("v0")[-1]))[1],
        "vehicle.replaceStop(v0, 1, nope edge)": lambda t: (two_stops(t), t.vehicle.replaceStop("v0", 1, "nope"))[1],
        "vehicle.rerouteTraveltime(v0) between stops + getRoute": reroute_between_stops,
    }
    base.update(domain_probes(lane, edge, tl, run_n, summary))
    return base


VTYPE_PAIRS = [
    ("Accel", 2.5), ("Decel", 4.5), ("EmergencyDecel", 8.0), ("ApparentDecel", 5.0), ("Imperfection", 0.4),
    ("Tau", 1.2), ("MinGap", 3.0), ("Width", 2.0), ("Height", 1.8), ("Length", 6.0), ("Mass", 1500.0),
    ("Color", (10, 20, 30, 255)), ("VehicleClass", "bus"), ("EmissionClass", "HBEFA3/PC_G_EU4"),
    ("ShapeClass", "bus"), ("SpeedFactor", 1.2), ("SpeedDeviation", 0.1), ("ActionStepLength", 0.5),
    ("Impatience", 0.5), ("BoardingDuration", 1.0), ("Scale", 1.5), ("LateralAlignment", "center"),
    ("MinGapLat", 0.8), ("MaxSpeedLat", 2.0), ("MaxSpeed", 20.0),
]

SIM_GETTERS = [
    "getTime", "getEndTime", "getCurrentTime", "getLoadedNumber", "getLoadedIDList", "getDepartedNumber",
    "getDepartedIDList", "getArrivedNumber", "getArrivedIDList", "getDepartedPersonNumber", "getDepartedPersonIDList",
    "getArrivedPersonNumber", "getArrivedPersonIDList", "getParkingStartingVehiclesNumber",
    "getParkingStartingVehiclesIDList", "getParkingEndingVehiclesNumber", "getParkingEndingVehiclesIDList",
    "getStopStartingVehiclesNumber", "getStopStartingVehiclesIDList", "getStopEndingVehiclesNumber",
    "getStopEndingVehiclesIDList", "getCollidingVehiclesNumber", "getCollidingVehiclesIDList",
    "getEmergencyStoppingVehiclesNumber", "getEmergencyStoppingVehiclesIDList", "getMinExpectedNumber",
    "getBusStopIDList", "getStartingTeleportNumber", "getStartingTeleportIDList", "getEndingTeleportNumber",
    "getEndingTeleportIDList", "getCollisions", "getPendingVehicles", "getScale", "getDeltaT", "getNetBoundary",
]


def domain_probes(lane, edge, tl, run_n, summary):
    """Setter/getter probes for Lane, Edge, Junction, Route, TrafficLight, VehicleType, Simulation.
    Labels start with 'dom ' so --filter "dom " selects only these."""
    P = {}

    def read_after(setter, getter):
        return lambda t: (setter(t), getter(t))[1]

    # lane
    P["dom lane.setAllowed(passenger) + getAllowed"] = read_after(lambda t: t.lane.setAllowed(lane, ["passenger"]), lambda t: t.lane.getAllowed(lane))
    P["dom lane.setDisallowed(bicycle) + getDisallowed"] = read_after(lambda t: t.lane.setDisallowed(lane, ["bicycle"]), lambda t: t.lane.getDisallowed(lane))
    P["dom lane.setChangePermissions(passenger,1) + get"] = read_after(lambda t: t.lane.setChangePermissions(lane, ["passenger"], 1), lambda t: t.lane.getChangePermissions(lane, 1))
    P["dom lane.setFriction(0.5) + getFriction"] = read_after(lambda t: t.lane.setFriction(lane, 0.5), lambda t: t.lane.getFriction(lane))
    P["dom lane.setLength(50) + getLength"] = read_after(lambda t: t.lane.setLength(lane, 50.0), lambda t: t.lane.getLength(lane))
    P["dom lane.setAllowed(passenger) +30 summary"] = lambda t: (t.lane.setAllowed(lane, ["passenger"]), run_n(t, 30), summary(t))[2]
    for name, fn in [
        ("getShape", lambda t: t.lane.getShape(lane)), ("getLinks", lambda t: t.lane.getLinks(lane)),
        ("getLinkNumber", lambda t: t.lane.getLinkNumber(lane)), ("getFoes", lambda t: t.lane.getFoes(lane, lane)),
        ("getInternalFoes", lambda t: t.lane.getInternalFoes(lane)), ("getPendingVehicles", lambda t: t.lane.getPendingVehicles(lane)),
        ("getBidiLane", lambda t: t.lane.getBidiLane(lane)), ("getEdgeID", lambda t: t.lane.getEdgeID(lane)),
        ("getLastStepOccupancy", lambda t: t.lane.getLastStepOccupancy(lane)), ("getLastStepLength", lambda t: t.lane.getLastStepLength(lane)),
        ("getWaitingTime", lambda t: t.lane.getWaitingTime(lane)), ("getTraveltime", lambda t: t.lane.getTraveltime(lane)),
        ("getLastStepMeanSpeed", lambda t: t.lane.getLastStepMeanSpeed(lane)), ("getLastStepHaltingNumber", lambda t: t.lane.getLastStepHaltingNumber(lane)),
        ("getAllowed", lambda t: t.lane.getAllowed(lane)), ("getDisallowed", lambda t: t.lane.getDisallowed(lane)),
        ("getChangePermissions", lambda t: t.lane.getChangePermissions(lane, 1)), ("getAngle", lambda t: t.lane.getAngle(lane, 0.0)),
        ("getWidth", lambda t: t.lane.getWidth(lane)), ("getCO2Emission", lambda t: t.lane.getCO2Emission(lane)),
        ("getFuelConsumption", lambda t: t.lane.getFuelConsumption(lane)), ("getNoiseEmission", lambda t: t.lane.getNoiseEmission(lane)),
        ("getElectricityConsumption", lambda t: t.lane.getElectricityConsumption(lane)),
    ]:
        P[f"dom lane.{name}(lane)"] = fn
    for name, fn in [
        ("getShape(nope)", lambda t: t.lane.getShape("nope")), ("getFriction(nope)", lambda t: t.lane.getFriction("nope")),
        ("getAllowed(nope)", lambda t: t.lane.getAllowed("nope")), ("setAllowed(nope)", lambda t: t.lane.setAllowed("nope", ["all"])),
        ("setFriction(lane, 2.0)", lambda t: t.lane.setFriction(lane, 2.0)), ("setLength(lane, -1)", lambda t: t.lane.setLength(lane, -1.0)),
    ]:
        P[f"dom lane.{name}"] = fn

    # edge
    P["dom edge.setAllowed(bus) +30 summary"] = lambda t: (t.edge.setAllowed(edge, ["bus"]), run_n(t, 30), summary(t))[2]
    P["dom edge.setDisallowed(passenger) +30 summary"] = lambda t: (t.edge.setDisallowed(edge, ["passenger"]), run_n(t, 30), summary(t))[2]
    P["dom edge.setEffort(5) + getEffort(0)"] = read_after(lambda t: t.edge.setEffort(edge, 5.0), lambda t: t.edge.getEffort(edge, 0.0))
    P["dom edge.adaptTraveltime(100) + getAdaptedTraveltime"] = read_after(lambda t: t.edge.adaptTraveltime(edge, 100.0), lambda t: t.edge.getAdaptedTraveltime(edge, 0.0))
    P["dom edge.setFriction(0.5) + getMeanFriction"] = read_after(lambda t: t.edge.setFriction(edge, 0.5), lambda t: t.edge.getMeanFriction(edge))
    P["dom edge.setMaxSpeed(5) + getTraveltime"] = read_after(lambda t: t.edge.setMaxSpeed(edge, 5.0), lambda t: t.edge.getTraveltime(edge))
    for name, fn in [
        ("getLaneNumber", lambda t: t.edge.getLaneNumber(edge)), ("getStreetName", lambda t: t.edge.getStreetName(edge)),
        ("getMeanFriction", lambda t: t.edge.getMeanFriction(edge)), ("getLastStepOccupancy", lambda t: t.edge.getLastStepOccupancy(edge)),
        ("getLastStepLength", lambda t: t.edge.getLastStepLength(edge)), ("getFromJunction", lambda t: t.edge.getFromJunction(edge)),
        ("getToJunction", lambda t: t.edge.getToJunction(edge)), ("getBidiEdge", lambda t: t.edge.getBidiEdge(edge)),
        ("getLastStepPersonIDs", lambda t: t.edge.getLastStepPersonIDs(edge)), ("getPendingVehicles", lambda t: t.edge.getPendingVehicles(edge)),
        ("getWaitingTime", lambda t: t.edge.getWaitingTime(edge)), ("getEffort(0)", lambda t: t.edge.getEffort(edge, 0.0)),
        ("getAdaptedTraveltime(0)", lambda t: t.edge.getAdaptedTraveltime(edge, 0.0)), ("getLastStepHaltingNumber", lambda t: t.edge.getLastStepHaltingNumber(edge)),
        ("getAngle(0)", lambda t: t.edge.getAngle(edge, 0.0)), ("getCO2Emission", lambda t: t.edge.getCO2Emission(edge)),
        ("getFuelConsumption", lambda t: t.edge.getFuelConsumption(edge)), ("getNoiseEmission", lambda t: t.edge.getNoiseEmission(edge)),
    ]:
        P[f"dom edge.{name}(edge)"] = fn

    # junction and route
    j = lambda t: t.junction.getIDList()[0]
    P["dom junction.getShape(first)"] = lambda t: t.junction.getShape(j(t))
    P["dom junction.getIncomingEdges(first)"] = lambda t: t.junction.getIncomingEdges(j(t))
    P["dom junction.getOutgoingEdges(first)"] = lambda t: t.junction.getOutgoingEdges(j(t))
    P["dom route.getEdges(first)"] = lambda t: t.route.getEdges(t.route.getIDList()[0])
    P["dom route.add(rnew, v0 route) + getEdges"] = lambda t: (t.route.add("rnew", t.vehicle.getRoute("v0")), t.route.getEdges("rnew"))[1]
    P["dom route.add(rdup) twice"] = lambda t: (t.route.add("rdup", t.vehicle.getRoute("v0")), t.route.add("rdup", t.vehicle.getRoute("v0")))
    P["dom route.getEdges(nope)"] = lambda t: t.route.getEdges("nope")
    P["dom route.remove(first)"] = lambda t: (t.route.remove(t.route.getIDList()[0]), t.route.getIDList())[1]

    # traffic light getters
    for name in ["getSubID", "getType", "getProgram", "getPhase", "getNextSwitch", "getSpentDuration", "getPhaseDuration",
                 "getControlledLanes", "getControlledLinks", "getAllProgramLogics", "getPhases", "getServedPersonCount"]:
        P[f"dom trafficlight.{name}(tl)"] = (lambda n: lambda t: getattr(t.trafficlight, n)(tl))(name)
    P["dom trafficlight.getProgram(nope)"] = lambda t: t.trafficlight.getProgram("nope")
    P["dom trafficlight.getControlledLanes(nope)"] = lambda t: t.trafficlight.getControlledLanes("nope")

    # vehicle type: each setter followed by its getter
    for suffix, val in VTYPE_PAIRS:
        P[f"dom vehicletype.set{suffix}(DEFAULT,{val}) + get"] = read_after(
            (lambda s, v: lambda t: getattr(t.vehicletype, "set" + s)("DEFAULT_VEHTYPE", v))(suffix, val),
            (lambda s: lambda t: getattr(t.vehicletype, "get" + s)("DEFAULT_VEHTYPE"))(suffix))
    for suffix, _ in VTYPE_PAIRS:
        if suffix in ("MaxSpeed",):
            continue
        P[f"dom vehicletype.get{suffix}(DEFAULT)"] = (lambda s: lambda t: getattr(t.vehicletype, "get" + s)("DEFAULT_VEHTYPE"))(suffix)
    P["dom vehicletype.copy(DEFAULT, cp) + getMaxSpeed"] = read_after(
        lambda t: t.vehicletype.copy("DEFAULT_VEHTYPE", "cp"), lambda t: t.vehicletype.getMaxSpeed("cp"))
    P["dom vehicletype.getAccel(nope)"] = lambda t: t.vehicletype.getAccel("nope")

    # simulation: zero-argument getters
    for name in SIM_GETTERS:
        P[f"dom simulation.{name}()"] = (lambda n: lambda t: getattr(t.simulation, n)())(name)
    return P


def run_probe(side, port, net, rou, ours, fn, extra=()):
    """One fresh server for one probe. Returns {"type", "value"}."""
    log = Path(os.environ.get("TEMP", ".")) / f"traci_{side}_{net.parent.name}.log"
    with open(log, "a") as fh:
        proc = subprocess.Popen(sumo_command(side, port, net, rou, ours, extra), cwd=str(CASE), stdout=fh, stderr=subprocess.STDOUT)
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


SHARD = [0, 1]  # (index, count) set by --shard; the default runs every probe


def kill_cpp(port):
    # per-port only: other agents run harnesses concurrently and must not lose their sumo
    cmd = f"pkill -f -- '--remote-port {port} ' || true"
    if PRISTINE_BIN:
        subprocess.run(["bash", "-c", cmd], capture_output=True)
    else:
        subprocess.run(["wsl", "-e", "bash", "-lc", cmd], capture_output=True)


def run_side(side, port, ours, case, flt=(), skip=()):
    net, rou = CASES_DIR / case / "ref_simple.net.xml", CASES_DIR / case / "routes.rou"
    lane, tl_id = CASE_PARAMS[case]
    out = {}
    picked = [(l, f) for l, f in probes(lane, tl_id).items()
              if (not flt or any(s in l for s in flt)) and not any(s in l for s in skip)]
    picked = [p for k, p in enumerate(picked) if k % SHARD[1] == SHARD[0]]
    for i, (label, fn) in enumerate(picked):
        # fresh port per probe: a WSL sumo left behind by a previous probe would still hold the old one
        extra = ("--duration-log.statistics", "true") if label == TRIPINFO_PROBE else ()
        out[label] = run_probe(side, port + i, net, rou, ours, fn, extra)
        if side == "cpp":
            kill_cpp(port + i)
        time.sleep(0.5)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", choices=["ours", "cpp"])
    ap.add_argument("--case", default="all", choices=sorted(CASE_PARAMS) + ["all"])
    ap.add_argument("--ours", default=str(DEFAULT_OURS))
    ap.add_argument("--port", type=int, default=8813)
    ap.add_argument("--filter", default="", help="comma-separated substrings; only probes whose label contains one run")
    ap.add_argument("--skip", default="", help="comma-separated substrings; probes whose label contains one are not run")
    ap.add_argument("--shard", default="0/1", help="i/n: run only every n-th probe, starting at i (parallel runs)")
    a = ap.parse_args()
    warn_if_stale(Path(a.ours))
    flt = tuple(s for s in a.filter.split(",") if s)
    skip = tuple(s for s in a.skip.split(",") if s)
    shard, shards = (int(x) for x in a.shard.split("/"))
    SHARD[:] = [shard, shards]

    if a.only:
        print(json.dumps(run_side(a.only, a.port, a.ours, a.case, flt, skip), indent=2))
        return 0

    total = bad_total = 0
    for case in ([a.case] if a.case != "all" else sorted(CASE_PARAMS)):
        cpp = run_side("cpp", a.port, a.ours, case, flt, skip)
        ours = run_side("ours", a.port, a.ours, case, flt, skip)
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
