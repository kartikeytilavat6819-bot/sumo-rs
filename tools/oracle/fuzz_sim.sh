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
pick() { echo $(( ($1 * $2 + $3) % $4 )); }   # pick seed mult add mod

gen_case() {  # gen_case <seed> <dir>; prints the parameter string
  local s=$1 d=$2 mode lanes jt nil p ff vc step lc lat ttp ped
  mkdir -p "$d"
  mode=(grid grid spider rand); mode=${mode[$(pick $s 7 1 4)]}
  lanes=$(( $(pick $s 5 2 3) + 1 ))
  jt=(priority traffic_light right_before_left allway_stop); jt=${jt[$(pick $s 3 0 4)]}
  nil=$(pick $s 11 3 2)
  local net="--default.lanenumber $lanes --default-junction-type $jt --seed $s"
  [ $nil = 1 ] && net="$net --no-internal-links"
  case $mode in
    grid)   net="--grid --grid.number $(( $(pick $s 3 1 3) + 2 )) --grid.length 150 $net";;
    spider) net="--spider --spider.arm-number 4 --spider.circle-number 2 --spider.space-radius 150 $net";;
    rand)   net="--rand --rand.iterations 14 --rand.max-distance 300 --rand.grid $net";;
  esac
  "$OFF/netgenerate" $net -o "$d/ref_simple.net.xml" >"$d/gen.log" 2>&1 || { echo "GENFAIL(netgenerate)"; return 1; }
  p=(3.5 4 5 6); p=${p[$(pick $s 13 1 4)]}
  ff=(1 2 5 10); ff=${ff[$(pick $s 17 2 4)]}
  vc=(passenger passenger bus truck passenger delivery); vc=${vc[$(pick $s 19 0 6)]}
  ped=$(pick $s 23 5 4)    # 0 => also persons
  local rt="-n $d/ref_simple.net.xml -e 200 --seed $s --fringe-factor $ff --validate"
  "$PY" "$SUMO_HOME/tools/randomTrips.py" $rt -p $p --vehicle-class $vc --prefix v \
      -o "$d/trips.xml" -r "$d/veh.rou.xml" >"$d/rt.log" 2>&1 || { echo "GENFAIL(randomTrips)"; return 1; }
  local rf="$d/veh.rou.xml"
  if [ $ped = 0 ]; then
    "$PY" "$SUMO_HOME/tools/randomTrips.py" $rt -p 25 --pedestrians --prefix p -o "$d/ptrips.xml" -r "$d/ped.rou.xml" >>"$d/rt.log" 2>&1 \
      && rf="$d/veh.rou.xml,$d/ped.rou.xml"
  fi
  # merge into one routes.rou, sorted by depart (run_all takes a single routes.rou)
  "$PY" - "$rf" "$d/routes.rou" <<'PYEOF'
import sys, xml.etree.ElementTree as ET
items = []; types = []
for f in sys.argv[1].split(","):
    for e in ET.parse(f).getroot():
        (types if e.tag in ("vType", "vTypeDistribution") else items).append(e)
items.sort(key=lambda e: float(e.get("depart", e.get("begin", "0"))))
r = ET.Element("routes"); r.extend(types); r.extend(items)
ET.ElementTree(r).write(sys.argv[2])
PYEOF
  step=(1 0.5); step=${step[$(pick $s 29 1 2)]}
  lc=(LC2013 SL2015 LC2013 default); lc=${lc[$(pick $s 31 0 4)]}
  lat=$(pick $s 37 2 4); ttp=(300 -1 60); ttp=${ttp[$(pick $s 41 1 3)]}
  local a="--end 300 --seed $s --step-length $step --time-to-teleport $ttp"
  [ $lc != default ] && a="$a --lanechange.model $lc"
  [ $lat = 0 ] && a="$a --lateral-resolution 0.8"
  echo "$a" > "$d/sim.args"
  echo "$mode lanes=$lanes jt=$jt nil=$nil p=$p ff=$ff vc=$vc ped=$ped | $a"
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

declare -A SEEN; FAILS=""; P=0; F=0
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
