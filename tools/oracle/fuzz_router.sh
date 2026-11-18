#!/bin/bash
# Differential fuzzing of the routing binaries: official (SUMO_PRISTINE_BIN) vs ours on random
# small networks, random trips/flows and random routing options. Compares the written route file
# and the alternatives file byte for byte after the same normalization run_duarouter.sh uses.
#   fuzz_router.sh [N=40]                 run seeds FUZZ_FIRST..FUZZ_FIRST+N-1
#   fuzz_router.sh --gen <seed> <dir>     only generate the case into <dir> (deterministic)
#   fuzz_router.sh --dump <dir>           print every input file as base64
# Env: FUZZ_OUT (work dir), FUZZ_FIRST (first seed), FUZZ_BUDGET (seconds, default 900).
# Run on the VM from tools/kaggle/agent_run.sh; our binaries in $CARGO_TARGET_DIR/debug.
set -u
cd "$(dirname "$0")/../.." || exit 1
OFF=${SUMO_PRISTINE_BIN:?}; OURS=${CARGO_TARGET_DIR:-/tmp/target}/debug
PY=$(command -v python3 || command -v python)
export PATH="$OFF:$PATH"            # randomTrips.py finds the official duarouter through PATH/SUMO_HOME
export SUMO_HOME=${SUMO_HOME:-$OFF/../lib/python3.12/site-packages/sumo}
OUT=${FUZZ_OUT:-/tmp/fuzz_router}; mkdir -p "$OUT"
r() { R=$(( RANDOM % $1 )); }       # bash RANDOM is deterministic after RANDOM=<seed>
normalize() { grep -v 'generated on' "$1" | sed -E 's#/(ref|cand)/#/SIDE/#g'; }

gen_case() {  # gen_case <seed> <dir>; prints the drawn parameters
  local s=$1 d=$2 mode lanes jt nil p e sidewalks feat algo rf rep rmloop taz unsorted
  local maxalt rcm wf vt flows igerr randflow seed input
  mkdir -p "$d"; RANDOM=$s; r 1; r 1; r 1   # warm-up
  r 6; algo=(dijkstra astar CH CHWrapper CCH arcflag); algo=${algo[$R]}
  r 3; mode=(grid grid spider rand); mode=${mode[$R]}
  local gnum=$((R + 2))
  if [ $algo = arcflag ]; then mode=grid; gnum=6; fi   # arcflag's KD-tree needs enough nodes
  r 3; lanes=$((R + 1))
  r 3; jt=(priority traffic_light right_before_left); jt=${jt[$R]}
  r 2; nil=$R
  if [ $algo = arcflag ]; then nil=0; fi
  local net="--default.lanenumber $lanes --default-junction-type $jt --seed $s"
  [ $nil = 1 ] && net="$net --no-internal-links"
  r 3; sidewalks=$R
  if [ $sidewalks = 0 ]; then
    net="$net --sidewalks.guess"
    [ $nil = 0 ] && net="$net --crossings.guess"   # crossings.guess conflicts with --no-internal-links
  fi
  case $mode in
    grid)   r 3; net="--grid --grid.number $gnum --grid.length 150 $net";;
    spider) net="--spider --spider.arm-number 4 --spider.circle-number 2 --spider.space-radius 150 $net";;
    rand)   net="--rand --rand.iterations 12 --rand.max-distance 300 --rand.grid $net";;
  esac
  "$OFF/netgenerate" $net -o "$d/ref_simple.net.xml" >"$d/gen.log" 2>&1 || { echo "GENERR(net): $(tail -3 "$d/gen.log" | tr -d '\r' | tr '\n' ' ' | cut -c1-180) | net=$net"; return 1; }

  r 4; p=(2 3 5 7); p=${p[$R]}
  r 5; e=$((R * 40 + 100))
  r 6; local ff=(1 2 3 5 8 13); ff=${ff[$R]}
  local rt="-n $d/ref_simple.net.xml -e $e --seed $s --fringe-factor $ff --validate"
  "$PY" "$SUMO_HOME/tools/randomTrips.py" $rt -p $p --prefix v \
      -o "$d/trips.xml" >"$d/rt.log" 2>&1 || { echo "GENERR(trips): $(grep -m1 -iE 'error' "$d/rt.log" | tr -d '\r' | cut -c1-140)"; return 1; }

  # ---- random option draws ----
  r 4; local rfi=$R; local rfs=(1.0 1.3 2.0 3.5); rf=${rfs[$rfi]}
  r 4; rep=$R      # 0 => --repair
  r 4; rmloop=$R   # 0 => --remove-loops
  r 4; taz=$R      # 0 => --with-taz (needs an additional file)
  r 4; unsorted=$R # 0 => --unsorted-input
  r 3; maxalt=$R; maxalt=$((maxalt + 1))
  r 4; rcm=$R; local rcms=(none logit gawron logit); rcm=${rcms[$rcm]}
  r 3; wf=$R       # 0 => weight file
  r 4; vt=$R       # 0 => vType with speedFactor/vClass
  r 3; flows=$R    # 0 => use --flow-files instead of --trip-files
  r 4; igerr=$R    # 0 => --ignore-errors
  r 4; randflow=$R # 0 => --randomize-flows (only meaningful with flows)
  r 3; local si=$R; seed=$((s + si * 17))

  input="--trip-files $d/trips.xml"
  [ $flows = 0 ] && {
    "$PY" - "$d/trips.xml" "$d/flows.xml" "$e" <<'PYEOF'
import sys, xml.etree.ElementTree as ET
r = ET.parse(sys.argv[1]).getroot(); end = int(sys.argv[3])
ts = [t for t in r.findall('trip')]
out = ET.Element('flows')
for i, t in enumerate(ts[:6]):
    ET.SubElement(out, 'flow', {'id': 'f%d' % i, 'begin': t.get('depart', '0'),
        'end': str(end), 'period': str(10 + i * 5), 'from': t.get('from'), 'to': t.get('to')})
ET.ElementTree(out).write(sys.argv[2])
PYEOF
    input="--flow-files $d/flows.xml"
  }

  # vType is added to the base input file before any TAZ rewrite so it propagates
  if [ $vt = 0 ]; then
    local basefile=$d/trips.xml; [ $flows = 0 ] && basefile=$d/flows.xml
    "$PY" - "$basefile" "$s" <<'PYEOF'
import sys, xml.etree.ElementTree as ET
f, s = sys.argv[1], int(sys.argv[2])
vc = ['passenger', 'bus', 'truck'][s % 3]
sf = 0.8 + (s % 5) * 0.1
T = ET.parse(f); root = T.getroot()
root.insert(0, ET.Element('vType', {'id': 'fuzzv', 'vClass': vc, 'speedFactor': '%.2f' % sf}))
for el in list(root):
    if el.tag in ('trip', 'flow'): el.set('type', 'fuzzv')
T.write(f)
PYEOF
  fi

  if [ $taz = 0 ]; then
    "$PY" - "$d/ref_simple.net.xml" "$d/trips.xml" "$d/flows.xml" "$d/taz.add.xml" "$flows" "$d/trips_taz.xml" "$d/flows_taz.xml" <<'PYEOF'
import sys, xml.etree.ElementTree as ET
net, tripf, flowf, addf, flows, ttrip, tflow = sys.argv[1:8]
N = ET.parse(net).getroot()
add = ET.Element('additional')
for e in N.findall('edge'):
    eid = e.get('id')
    if eid and not eid.startswith(':'):
        ET.SubElement(add, 'taz', {'id': 'tz_' + eid, 'edges': eid})
ET.ElementTree(add).write(addf)
base, target = (flowf, tflow) if flows == '0' else (tripf, ttrip)
T = ET.parse(base); root = T.getroot()
for el in root.findall('flow') + root.findall('trip'):
    for k in ('from', 'to'):
        v = el.get(k)
        if v:
            el.set(k + 'Taz', 'tz_' + v); del el.attrib[k]
ET.ElementTree(root).write(target)
PYEOF
    if [ $flows = 0 ]; then input="--flow-files $d/flows_taz.xml"; else input="--trip-files $d/trips_taz.xml"; fi
  fi

  local a="--net-file $d/ref_simple.net.xml $input --seed $seed --output-file @DIR@/routes.rou.xml --no-step-log true"
  a="$a --routing-algorithm $algo"
  [ "$rf" != 1.0 ] && a="$a --weights.random-factor $rf"
  [ $rep = 0 ] && a="$a --repair true"
  [ $rmloop = 0 ] && a="$a --remove-loops true"
  [ $unsorted = 0 ] && a="$a --unsorted-input true"
  [ $igerr = 0 ] && a="$a --ignore-errors true"
  [ $flows = 0 ] && [ $randflow = 0 ] && a="$a --randomize-flows true"
  if [ $maxalt -gt 1 ]; then
    a="$a --max-alternatives $maxalt --alternatives-output @DIR@/alts.xml"
    if [ $rcm = logit ]; then
      a="$a --route-choice-method logit --logit.beta -2 --logit.gamma 0.8 --logit.theta -1"
    elif [ $rcm = gawron ]; then
      a="$a --route-choice-method gawron --gawron.beta 0.6 --gawron.a 0.3"
    fi
  fi
  if [ $wf = 0 ]; then
    "$PY" - "$d/ref_simple.net.xml" "$d/weights.xml" "$s" <<'PYEOF'
import sys, random, xml.etree.ElementTree as ET
r = ET.parse(sys.argv[1]).getroot(); rng = random.Random(int(sys.argv[3]))
out = ET.Element('meandata'); iv = ET.SubElement(out, 'interval', {'begin': '0', 'end': '3600'})
for e in r.findall('edge'):
    eid = e.get('id')
    if not eid or eid.startswith(':'):
        continue
    ET.SubElement(iv, 'edge', {'id': eid, 'traveltime': '%.2f' % (rng.uniform(1, 40))})
ET.ElementTree(out).write(sys.argv[2])
PYEOF
    a="$a --weight-files $d/weights.xml --weight-attribute traveltime"
  fi
  [ $taz = 0 ] && a="$a --additional-files $d/taz.add.xml --with-taz true"
  echo "$a" > "$d/dua.args"
  echo "mode=$mode lanes=$lanes jt=$jt nil=$nil sidewalk=$sidewalks p=$p e=$e ff=$ff algo=$algo rf=$rf rep=$rep rmloop=$rmloop taz=$taz unsorted=$unsorted maxalt=$maxalt rcm=$rcm wf=$wf vt=$vt flows=$flows igerr=$igerr randflow=$randflow seed=$seed"
}

run_cmp() {  # run_cmp <dir>; sets DIFF; 0 PASS, 1 FAIL, 2 PANIC/TIMEOUT, 3 REFFAIL
  local d=$1 w bin rc f
  local args="$(cat "$d/dua.args")"
  for w in ref cand; do
    bin=$OFF; [ $w = cand ] && bin=$OURS
    rm -rf "$d/$w"; mkdir -p "$d/$w"
    local wa=${args//@DIR@/$d/$w}
    timeout ${FUZZ_TMO:-60} "$bin/duarouter" $wa >"$d/$w.log" 2>&1
    rc=$?
    if [ $rc -ne 0 ]; then
      if [ $w = ref ]; then DIFF="REFERR rc=$rc $(grep -m1 -iE 'error' "$d/ref.log" | tr -d '\r' | cut -c1-90)"; return 3; fi
      DIFF="rc=$rc $(grep -m1 -E 'panicked at|Error:' "$d/cand.log" | tr -d '\r' | cut -c1-120)"; return 2
    fi
    if grep -q 'panicked at' "$d/$w.log"; then
      if [ $w = ref ]; then DIFF="REFPANIC"; return 3; fi
      DIFF="PANIC $(grep -m1 -A1 'panicked at' "$d/cand.log" | tr -d '\r' | tr '\n' ' ' | cut -c1-120)"; return 2
    fi
  done
  if [ "$(ls "$d/ref" | tr '\n' ' ')" != "$(ls "$d/cand" | tr '\n' ' ')" ]; then
    DIFF="files differ ref=[$(ls "$d/ref" | tr '\n' ' ')] cand=[$(ls "$d/cand" | tr '\n' ' ')]"; return 1
  fi
  for f in $(ls "$d/ref"); do
    if ! diff <(normalize "$d/ref/$f") <(normalize "$d/cand/$f") >"$d/$f.diff" 2>&1; then
      DIFF="$f: $(head -1 "$d/$f.diff" | tr -d '\r' | cut -c1-160)"; return 1
    fi
  done
  DIFF=""; return 0
}

dump_case() {  # dump_case <dir>: print every input file as base64 so the case can be rebuilt off-VM
  local d=$1 f
  for f in ref_simple.net.xml trips.xml flows.xml weights.xml taz.add.xml trips_taz.xml flows_taz.xml dua.args; do
    [ -f "$d/$f" ] || continue
    printf 'FILE %s %s\n' "$f" "$(base64 -w0 "$d/$f")"
  done
}

if [ "${1:-}" = "--gen" ]; then gen_case "$2" "$3"; exit $?; fi
if [ "${1:-}" = "--dump" ]; then dump_case "$2"; exit $?; fi

START=$(date +%s); BUDGET=${FUZZ_BUDGET:-900}
N=${1:-40}; FIRST=${FUZZ_FIRST:-1}
P=0; F=0; R=0
for ((s = FIRST; s < FIRST + N; s++)); do
  [ $(( $(date +%s) - START )) -gt $BUDGET ] && { echo "budget: stopped at seed $s"; break; }
  d=$OUT/seed$s; rm -rf "$d"
  params=$(gen_case $s "$d") || { echo "seed $s $params"; continue; }
  run_cmp "$d"; rc=$?
  case $rc in
    0) st=PASS; P=$((P+1));;
    1) st=DIFF; F=$((F+1));;
    2) st=PANIC; F=$((F+1));;
    3) st=REFERR; R=$((R+1));;
  esac
  if [ $rc -eq 1 ] || [ $rc -eq 2 ]; then : > "$d/.diff_found"; fi
  echo "seed $s $st ${DIFF:+| $DIFF} | $params" | cut -c1-500
done
echo "=== PASS=$P DIFF=$F REFERR=$R"
for s in $(seq $FIRST $((FIRST + N - 1))); do
  d=$OUT/seed$s; [ -f "$d/.diff_found" ] || continue
  echo "===== DIFF SEED $s"
  echo "--- diff"; cat "$d"/*.diff 2>/dev/null | head -20
  dump_case "$d"
done
