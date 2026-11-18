#!/bin/bash
# Random-scenario differential tester: official SUMO vs our binaries on random small networks + traffic.
# Runs on the VM from tools/kaggle/agent_run.sh (our binaries in $CARGO_TARGET_DIR/debug, official in
# $SUMO_PRISTINE_BIN, SUMO_HOME set), or anywhere both exist.
#   fuzz_sim.sh [N=60]                run seeds FIRST..FIRST+N-1, one line per seed; then shrink one seed per failure signature
#   fuzz_sim.sh --gen <seed> <dir>    only (re)generate the case for <seed> into <dir>: ref_simple.net.xml,
#                                     routes.rou, sim.args  (deterministic: same pristine tools => same files)
# Env: FUZZ_BUDGET (s, default 1500: total wall clock for fuzz+shrink), FUZZ_OUT (work dir), FUZZ_FIRST (first seed).
# Comparison = run_all's: compare_sim.py tripinfo + fcd, compare_stat.py statistics.
set -u
cd "$(dirname "$0")/../.." || exit 1
ORA=tools/oracle
OFF=${SUMO_PRISTINE_BIN:?}; OURS=${CARGO_TARGET_DIR:-/tmp/target}/debug
PY=$(command -v python3 || command -v python)
export PATH="$OFF:$PATH"            # randomTrips finds the official duarouter through PATH/SUMO_HOME
OUT=${FUZZ_OUT:-/tmp/fuzz_sim}; mkdir -p "$OUT"
r() { R=$(( RANDOM % $1 )); }   # bash RANDOM is deterministic after RANDOM=<seed>; no subshell, or the stream would not advance

gen_case() {  # gen_case <seed> <dir>; prints the parameter string
  local s=$1 d=$2 mode lanes jt nil p ff vc step lc lat ttp ped
  mkdir -p "$d"; RANDOM=$s; r 1; r 1; r 1   # warm-up: first draws after seeding track the seed
  r 4; mode=(grid grid spider rand); mode=${mode[$R]}
  r 3; lanes=$((R + 1))
  r 5; jt=(priority traffic_light right_before_left allway_stop traffic_light); jt=${jt[$R]}
  r 2; nil=$R
  local net="--default.lanenumber $lanes --default-junction-type $jt --seed $s"
  [ $nil = 1 ] && net="$net --no-internal-links"
  [ $jt = traffic_light ] && { r 2; [ $R = 1 ] && net="$net --tls.default-type actuated"; }
  case $mode in
    grid)   r 3; net="--grid --grid.number $((R + 2)) --grid.length 150 $net";;
    spider) net="--spider --spider.arm-number 4 --spider.circle-number 2 --spider.space-radius 150 $net";;
    rand)   net="--rand --rand.iterations 14 --rand.max-distance 300 --rand.grid $net";;
  esac
  "$OFF/netgenerate" $net -o "$d/ref_simple.net.xml" >"$d/gen.log" 2>&1 || { echo "GENFAIL(netgenerate)"; return 1; }
  r 5; p=(2 2.5 3.5 5 6); p=${p[$R]}; local e=200; [ "$p" = 2 -o "$p" = 2.5 ] && e=120
  r 4; ff=(1 2 5 10); ff=${ff[$R]}
  r 6; vc=(passenger passenger bus truck passenger delivery); vc=${vc[$R]}
  r 4; ped=$R    # 0 => also persons
  local rt="-n $d/ref_simple.net.xml -e $e --seed $s --fringe-factor $ff --validate"
  "$PY" "$SUMO_HOME/tools/randomTrips.py" $rt -p $p --vehicle-class $vc --prefix v \
      -o "$d/trips.xml" -r "$d/veh.rou.xml" >"$d/rt.log" 2>&1 || { echo "GENFAIL(randomTrips)"; return 1; }
  local rf="$d/veh.rou.xml" vc2
  r 3; vc2=(none motorcycle bicycle truck); vc2=${vc2[$R]}   # mixed traffic: second class on a slower period
  if [ $vc2 != none ]; then   # distinct departs would clash on ids, hence the prefix
    "$PY" "$SUMO_HOME/tools/randomTrips.py" $rt -p 9 --vehicle-class $vc2 --prefix w --seed $((s + 1)) -o "$d/trips2.xml" -r "$d/veh2.rou.xml" >>"$d/rt.log" 2>&1       && rf="$d/veh.rou.xml,$d/veh2.rou.xml"
  fi
  if [ $ped = 0 ]; then
    "$PY" "$SUMO_HOME/tools/randomTrips.py" $rt -p 25 --pedestrians --prefix p -o "$d/ptrips.xml" -r "$d/ped.rou.xml" >>"$d/rt.log" 2>&1 \
      && rf="$rf,$d/ped.rou.xml"
  fi
  r 4; lc=(LC2013 SL2015 LC2013 default); lc=${lc[$R]}
  # merge into one routes.rou, sorted by depart (run_all takes a single routes.rou)
  "$PY" - "$rf" "$d/routes.rou" "$lc" <<'PYEOF'
import sys, xml.etree.ElementTree as ET
items = []; types = []
for f in sys.argv[1].split(","):
    for e in ET.parse(f).getroot():
        (types if e.tag in ("vType", "vTypeDistribution") else items).append(e)
for t in types:
    if sys.argv[3] != "default": t.set("laneChangeModel", sys.argv[3])
items.sort(key=lambda e: float(e.get("depart", e.get("begin", "0"))))
r = ET.Element("routes"); r.extend(types); r.extend(items)
ET.ElementTree(r).write(sys.argv[2])
PYEOF
  r 2; step=(1 0.5); step=${step[$R]}
  r 3; lat=$R; r 3; ttp=(300 -1 60); ttp=${ttp[$R]}
  r 6; local x=("" "" "--lanechange.duration 3" "--collision.action teleport" "--ignore-junction-blocker 5" "--sloppy-insert"); x=${x[$R]}
  local a="--end 300 --seed $s --step-length $step --time-to-teleport $ttp $x"
  [ $lat = 0 ] && [ $lc = SL2015 ] && a="$a --lateral-resolution 0.8"
  echo "$a" > "$d/sim.args"
  echo "$mode lanes=$lanes jt=$jt nil=$nil p=$p ff=$ff vc=$vc ped=$ped lc=$lc vc2=$vc2 | $a"
}

if [ "${1:-}" = "--gen" ]; then gen_case "$2" "$3"; exit $?; fi

START=$(date +%s); BUDGET=${FUZZ_BUDGET:-1500}
N=${1:-60}; FIRST=${FUZZ_FIRST:-1}
DIFF=""
run_cmp() {  # run_cmp <dir>; sets DIFF; returns 0 PASS, 1 FAIL, 2 PANIC/TIMEOUT, 3 official run failed
  local d=$1 args w bin rc
  args="-n $d/ref_simple.net.xml -r $d/routes.rou --no-step-log true $(cat "$d/sim.args")"
  for w in ref cand; do
    bin=$OFF; [ $w = cand ] && bin=$OURS
    timeout 120 "$bin/sumo" $args --tripinfo-output "$d/$w.ti.xml" --fcd-output "$d/$w.fcd.xml" \
        --statistic-output "$d/$w.stat.xml" >"$d/$w.log" 2>&1
    rc=$?
    if [ $rc -ne 0 ]; then
      [ $w = ref ] && { DIFF="REFFAIL rc=$rc $(grep -m1 -E 'Error' "$d/ref.log" | cut -c1-80)"; return 3; }
      DIFF="rc=$rc $(grep -m1 -E 'panicked at|Error:' "$d/cand.log" | cut -c1-100)"; return 2
    fi
  done
  DIFF=$("$PY" $ORA/compare_sim.py --tripinfo "$d/ref.ti.xml" "$d/cand.ti.xml" | grep -v '^MATCH' | head -1)
  [ -n "$DIFF" ] && { DIFF="tripinfo $DIFF"; return 1; }
  DIFF=$("$PY" $ORA/compare_sim.py --fcd "$d/ref.fcd.xml" "$d/cand.fcd.xml" | grep -v '^MATCH' | head -1)
  [ -n "$DIFF" ] && { DIFF="fcd $DIFF"; return 1; }
  if ! "$PY" $ORA/compare_stat.py "$d/ref.stat.xml" "$d/cand.stat.xml" >"$d/stat.diff" 2>&1; then
    DIFF="stat $(head -1 "$d/stat.diff")"; return 1
  fi
  DIFF=""; return 0
}
# failure signature: stage + first diff with ids/numbers/values blanked
sig() { echo "$1" | sed -E "s/'[^']*'/X/g; s/(ref|cand)=.*//; s/[0-9]+(\.[0-9]+)?/N/g" | cut -c1-60; }

shrink() {  # shrink <dir> <signature>: smaller --end, then fewer vehicles/persons, while the signature stays
  local d=$1 want=$2 e n ch i
  still() { run_cmp "$d"; local r=$?; [ $r -eq 1 -o $r -eq 2 ] && [ "$(sig "$DIFF")" = "$want" ]; }
  for e in 20 40 60 100 150 200; do
    cp "$d/sim.args" "$d/sim.args.bak"; sed -i -E "s/--end [0-9.]+/--end $e/" "$d/sim.args"
    still && break; mv "$d/sim.args.bak" "$d/sim.args"
  done
  n=$("$PY" -c "import xml.etree.ElementTree as ET;print(sum(e.tag not in('vType','vTypeDistribution') for e in ET.parse('$d/routes.rou').getroot()))")
  ch=$(( (n + 1) / 2 ))
  while [ $ch -ge 1 ]; do
    i=0
    while [ $i -lt $n ] && [ $(( $(date +%s) - START )) -lt $(( BUDGET + 250 )) ]; do   # try deleting items [i, i+ch)
      cp "$d/routes.rou" "$d/routes.keep"
      "$PY" - "$d/routes.rou" $i $ch <<'PYEOF'
import sys, xml.etree.ElementTree as ET
t = ET.parse(sys.argv[1]); r = t.getroot(); i, c = int(sys.argv[2]), int(sys.argv[3])
it = [e for e in r if e.tag not in ("vType", "vTypeDistribution")]
for e in it[i:i + c]: r.remove(e)
t.write(sys.argv[1])
PYEOF
      if still; then n=$(( n - ch )); else cp "$d/routes.keep" "$d/routes.rou"; i=$(( i + ch )); fi
    done
    ch=$(( ch / 2 ))
  done
  run_cmp "$d"; echo "  shrunk: $(cat "$d/sim.args") items=$n sig=[$want] diff: $DIFF"
  echo "  --- routes.rou ($d)"; cat "$d/routes.rou"; echo; echo "  ---"
}

set +u; declare -A SEEN; FAILS=""; P=0; F=0
for ((s = FIRST; s < FIRST + N; s++)); do
  [ $(( $(date +%s) - START )) -gt $(( BUDGET * 6 / 10 )) ] && { echo "budget: stopped fuzzing at seed $s"; break; }
  d=$OUT/seed$s; rm -rf "$d"
  params=$(gen_case $s "$d") || { echo "seed $s $params"; continue; }
  run_cmp "$d"; r=$?
  case $r in
    0) st=PASS; P=$((P+1));;
    1) st=FAIL; F=$((F+1));;
    2) st=PANIC; F=$((F+1));;
    3) st=REFFAIL;;
  esac
  echo "seed $s $params => $st ${DIFF:+| $DIFF}" | cut -c1-600
  if [ $r -eq 1 ] || [ $r -eq 2 ]; then
    k=$(sig "$DIFF")
    [ -z "${SEEN[$k]:-}" ] && { SEEN[$k]=$s; FAILS="$FAILS $s"; }
  fi
done
echo "=== PASS=$P FAIL=$F; distinct signatures: ${#SEEN[@]}"
for k in "${!SEEN[@]}"; do echo "  sig [$k] first seed ${SEEN[$k]}"; done
for s in $FAILS; do
  [ $(( $(date +%s) - START )) -gt $BUDGET ] && { echo "budget: no time to shrink seed $s"; continue; }
  d=$OUT/seed$s; run_cmp "$d"
  echo "--- shrink seed $s"
  shrink "$d" "$(sig "$DIFF")"
done
