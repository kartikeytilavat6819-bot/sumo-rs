#!/bin/bash
# Probe the known-failing complex TraCI tests: official vs ours, print the first differing lines.
# Run from the repo root on the gate VM after the build (CARGO_TARGET_DIR, SUMO_PRISTINE_BIN, SUMO_HOME set).
set -u
OUT=/tmp/oc117
mkdir -p "$OUT"
export COMPLEX_OUT="$OUT/complex"
export COMPLEX_TIMEOUT=${COMPLEX_TIMEOUT:-120}
for t in misc/concurrent person/person simulation/loadState/persons_include simulation/simulation trafficlight/railsignal_state vehicle/cfmodel/setParameter vehicle/distance_after_reroute vehicle/insertStop/second_unreachable_ignore_rerouter vehicle/insertStop/stop_forbidden_ignore_rerouter_copy vehicle/keepAfterArrival vehicle/keepAfterArrival_state vehicle/replaceStop/first_parameter_only vehicle/setStop/newStop_fast_forced vehicle/setStop/newStop_fast_forced2; do
  rm -f "$COMPLEX_OUT/$t/diff.txt"
  COMPLEX_FILTER="$t" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL) ' | head -1
  d="$COMPLEX_OUT/$t/diff.txt"
  if [ -f "$d" ]; then
    echo "--- first differing lines for $t"
    head -40 "$d"
  fi
done

# setStop first-call state: lane, lane position and speed right before the first setStop, both binaries
PROBE_DIR="$OUT/setstop_probe"
rm -rf "$PROBE_DIR"; mkdir -p "$PROBE_DIR"
cp -r tools/oracle/complex_bundle/vehicle/setStop/newStop_fast_forced/. "$PROBE_DIR/"
python3 - "$PROBE_DIR/runner.py" <<'PY'
import sys
p = sys.argv[1]
src = open(p, encoding="utf-8").read()
marker = '    traci.vehicle.setStop(vehID, "SC", pos + 10, duration=3)\n'
probe = ('    print("PROBE lane=%s lanePos=%s road=%s speed=%s" % (traci.vehicle.getLaneID(vehID), '
         'traci.vehicle.getLanePosition(vehID), traci.vehicle.getRoadID(vehID), traci.vehicle.getSpeed(vehID)))\n')
open(p, "w", encoding="utf-8").write(src.replace(marker, probe + marker, 1))
PY
for side in official ours; do
  if [ "$side" = official ]; then bin="$SUMO_PRISTINE_BIN/sumo"; else bin="$CARGO_TARGET_DIR/debug/sumo"; fi
  (cd "$PROBE_DIR" && SUMO_BINARY="$bin" NETCONVERT_BINARY="${bin%sumo}netconvert" PYTHONPATH="$SUMO_HOME/tools" timeout 120 python3 runner.py 2>&1 | grep -m3 "PROBE") | sed "s/^/$side: /"
done

# insertStop: the router's own answer for the query that fails (A1B1 -> B2C2), both binaries
IS_DIR="$OUT/insertstop_probe"
rm -rf "$IS_DIR"; mkdir -p "$IS_DIR"
cp -r tools/oracle/complex_bundle/vehicle/insertStop/second_unreachable_ignore_rerouter/. "$IS_DIR/"
python3 - "$IS_DIR/runner.py" <<'PY'
import sys
p = sys.argv[1]
src = open(p, encoding="utf-8").read()
marker = 'traci.vehicle.insertStop(vehID, 1, "busStop_B2C2"'
probe = ('try:\n'
         '    print("FINDROUTE", traci.simulation.findRoute("A1B1", "B2C2", vType=traci.vehicle.getTypeID(vehID)).edges)\n'
         'except Exception as e:\n'
         '    print("FINDROUTE-ERR", type(e).__name__, e)\n')
i = src.find(marker)
line_start = src.rfind("\n", 0, i) + 1
indent = src[line_start:i]  # the marker line's own indentation (the call sits inside an if block)
probe = "".join(indent + ln if ln.strip() else ln for ln in probe.splitlines(True))
open(p, "w", encoding="utf-8").write(src[:line_start] + probe + src[line_start:])
PY
for side in official ours; do
  if [ "$side" = official ]; then bin="$SUMO_PRISTINE_BIN/sumo"; else bin="$CARGO_TARGET_DIR/debug/sumo"; fi
  (cd "$IS_DIR" && SUMO_BINARY="$bin" NETCONVERT_BINARY="${bin%sumo}netconvert" PYTHONPATH="$SUMO_HOME/tools" timeout 120 python3 runner.py 2>&1 | grep -E -m6 "^FINDROUTE|Error|Traceback|line [0-9]+, in <module>") | sed "s/^/$side: /"
done

# distance_after_reroute: per-step lane/pos/speed/route around the route switch, both binaries
DR_DIR="$OUT/distance_probe"
rm -rf "$DR_DIR"; mkdir -p "$DR_DIR"
cp -r tools/oracle/complex_bundle/vehicle/distance_after_reroute/. "$DR_DIR/"
python3 - "$DR_DIR/runner.py" <<'PY'
import sys
p = sys.argv[1]
src = open(p, encoding="utf-8").read()
marker = "        traci.simulationStep()\n"
probe = ('        if 119 <= step <= 124:\n'
         '            print("STATE", step, traci.vehicle.getLaneID("Stapler_00"), round(traci.vehicle.getLanePosition("Stapler_00"), 2), '
         'round(traci.vehicle.getSpeed("Stapler_00"), 2), traci.vehicle.getRoute("Stapler_00")[:6])\n')
open(p, "w", encoding="utf-8").write(src.replace(marker, marker + probe, 1))
PY
for side in official ours; do
  if [ "$side" = official ]; then bin="$SUMO_PRISTINE_BIN/sumo"; else bin="$CARGO_TARGET_DIR/debug/sumo"; fi
  (cd "$DR_DIR" && SUMO_BINARY="$bin" NETCONVERT_BINARY="${bin%sumo}netconvert" PYTHONPATH="$SUMO_HOME/tools" timeout 120 python3 runner.py 2>&1 | grep -E -m12 "^STATE|Error|Traceback") | sed "s/^/$side: /"
done
