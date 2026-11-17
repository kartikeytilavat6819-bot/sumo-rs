#!/bin/bash
# TEMPORARY agent run (city save/load roundtrip vs pristine). Removed before the final commit.
set -u
OUT=/tmp/rt2
mkdir -p "$OUT"
export PYTHONPATH=$SUMO_HOME/tools:${PYTHONPATH:-}
for c in city_bengaluru_blr city_manhattan_midtown city_munich city_plano; do
  d=tools/oracle/cases/$c
  base="-n $d/ref_simple.net.xml -r $d/routes.rou --end 600 --no-step-log true --no-warnings true"
  for side in ours cpp; do
    if [ $side = ours ]; then B=/tmp/target/debug; else B=$SUMO_PRISTINE_BIN; fi
    o=$OUT/$c/$side; mkdir -p "$o"
    "$B/sumo" $base --save-state.times 300 --save-state.files "$o/s.xml" --tripinfo-output "$o/save.ti.xml" >"$o/save.log" 2>&1
    "$B/sumo" $base --load-state "$o/s.xml" --tripinfo-output "$o/load.ti.xml" >"$o/load.log" 2>&1
  done
  echo "== $c state body"; python3 tools/oracle/cases/state_roundtrip/compare.py state $OUT/$c/ours/s.xml $OUT/$c/cpp/s.xml | head -1
  diff <(grep -v -e '<!--' -e '<?xml' -e '<snapshot' -e 'tripinfo-output' -e 'save-state.files' $OUT/$c/cpp/s.xml) <(grep -v -e '<!--' -e '<?xml' -e '<snapshot' -e 'tripinfo-output' -e 'save-state.files' $OUT/$c/ours/s.xml) | head -6 | cut -c1-200
  python3 - "$OUT/$c" <<'EOF'
import sys, xml.etree.ElementTree as ET
o = sys.argv[1]
def ti(p):
    return {e.get("id"): dict(e.attrib) for e in ET.parse(p).getroot().iter("tripinfo")}
a, b = ti(o + "/ours/load.ti.xml"), ti(o + "/cpp/load.ti.xml")
diff = [i for i in sorted(set(a) | set(b)) if a.get(i) != b.get(i)]
print("load tripinfo: ours=%d cpp=%d differing_ids=%d" % (len(a), len(b), len(diff)))
EOF
done
echo "AGENT RUN DONE"
