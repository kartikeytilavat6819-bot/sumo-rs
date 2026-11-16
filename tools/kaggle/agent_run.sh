#!/bin/bash
# TEMPORARY agent run (city save/load stress + TraCI probe). Removed before the final commit.
set -u
ROOT=$(pwd)
OUT=/tmp/city
mkdir -p "$OUT"
export TRACI_STATE_FILE=/tmp/traci_probe_state.xml
CMP="python3 tools/oracle/cases/state_roundtrip/compare.py"
export PYTHONPATH=$SUMO_HOME/tools:${PYTHONPATH:-}
for c in city_manhattan_midtown city_bengaluru_blr; do
  d=tools/oracle/cases/$c
  for side in ours cpp; do
    if [ $side = ours ]; then B=/tmp/target/debug; else B=$SUMO_PRISTINE_BIN; fi
    base="-n $d/ref_simple.net.xml -r $d/routes.rou --end 600 --no-step-log true --no-warnings true"
    o=$OUT/$c/$side; mkdir -p "$o"
    "$B/sumo" $base --tripinfo-output "$o/straight.ti.xml" >"$o/straight.log" 2>&1
    echo "$c $side straight exit=$?"
    "$B/sumo" $base --save-state.times 300 --save-state.files "$o/s.xml" --tripinfo-output "$o/save.ti.xml" >"$o/save.log" 2>&1
    echo "$c $side save exit=$?"
    "$B/sumo" $base --save-state.times 300 --save-state.files "$o/rng.xml" --save-state.rng true --tripinfo-output "$o/rngsave.ti.xml" >"$o/rng.log" 2>&1
    echo "$c $side save-rng exit=$?"
    "$B/sumo" $base --load-state "$o/s.xml" --tripinfo-output "$o/load.ti.xml" >"$o/load.log" 2>&1
    echo "$c $side load exit=$?"
  done
  O=$OUT/$c/ours; C=$OUT/$c/cpp
  echo "== $c (1) state @300 ours vs pristine"; $CMP state $O/s.xml $C/s.xml | head -24
  echo "== $c (1b) state @300 with rng ours vs pristine"; $CMP state $O/rng.xml $C/rng.xml | head -24
  echo "== $c (2a) ours load->end vs ours straight (vehicles running at 300)"; $CMP tripinfo $O/straight.ti.xml $O/load.ti.xml 300 | head -12
  echo "== $c (2b) pristine load->end vs pristine straight"; $CMP tripinfo $C/straight.ti.xml $C/load.ti.xml 300 | head -12
  echo "== $c (2c) ours load vs pristine load"; $CMP loads $O/straight.ti.xml $C/load.ti.xml $O/load.ti.xml 300 | head -12
done
echo "--- TraCI junction probe: saveState + loadState + step, ours vs pristine"
python3 - <<'EOF'
import os, sys
sys.path.insert(0, "tools/oracle/cases/traci_basic")
import client as c
def cmd(side, port, net, rou, ours, extra=()):
    b = "/tmp/target/debug/sumo" if side == "ours" else os.environ["SUMO_PRISTINE_BIN"] + "/sumo"
    return [b, "-n", str(net), "-r", str(rou), "--remote-port", str(port), "--no-step-log", "true", "--no-warnings", "true", *extra]
c.sumo_command = cmd
net, rou = c.CASES_DIR / "junction" / "ref_simple.net.xml", c.CASES_DIR / "junction" / "routes.rou"
fn = c.probes("-103224#1_0", "")["simulation.saveState + loadState + step"]
res = {s: c.run_probe(s, 8900 + i * 3, net, rou, "/tmp/target/debug/sumo", fn) for i, s in enumerate(["ours", "cpp"])}
print("ours:", res["ours"])
print("cpp :", res["cpp"])
print("TRACI", "MATCH" if res["ours"] == res["cpp"] else "DIFF")
EOF
echo "AGENT RUN DONE"
