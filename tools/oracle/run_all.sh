#!/bin/bash
# Scoreboard: run every oracle case through our binaries and print PASS/FAIL per stage.
# Usage: bash tools/oracle/run_all.sh [--build]
# Runs the checkout this script lives in. CARGO_TARGET_DIR selects the binaries (build them from the same code).
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target-main}
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-8}
BIN="$CARGO_TARGET_DIR/debug"
if [ "${1:-}" = "--build" ]; then
  cargo build -q -p sumo -p netconvert 2>&1 | grep -E '^error' | head -5
else
  # Refuse to print a scoreboard from binaries older than the sources: it looks authoritative and is
  # not. This has produced false readings for the lead and for several agents.
  if [ -x "$BIN/sumo.exe" ]; then
    newer=$(find crates -name '*.rs' -newer "$BIN/sumo.exe" -print -quit 2>/dev/null)
    if [ -n "$newer" ]; then
      echo "STALE BINARY: $BIN/sumo.exe is older than $newer"
      echo "             run 'bash tools/oracle/run_all.sh --build', or the numbers below measure other code."
    fi
  else
    echo "MISSING BINARY: $BIN/sumo.exe — build first (--build)."
  fi
fi
OUT=${SCORE_OUT:-/c/Users/karti/Temp/score}/run-$$
mkdir -p "$OUT"
PASS=0; FAIL=0; SKIP=0

line() { printf '%-22s %-10s %s\n' "$1" "$2" "$3"; }

for dir in tools/oracle/cases/*/; do
  c=$(basename "$dir")
  # depart needs one sim per route file (15 references): run_all_variants.sh, not this loop
  if [ "$c" = depart ]; then
    line "$c" variants SKIP "15 variants: cases/depart/run_all_variants.sh"; SKIP=$((SKIP+1)); continue
  fi
  # --- net stage: regenerate the net with netconvert and compare element by element.
  # Cases with an OSM input use --osm-files; cases built from plain XML put their full input arguments
  # (--node-files/--edge-files/--connection-files) in net.args and are run without --osm-files.
  NETEXTRA=""
  [ -f "$dir/net.args" ] && NETEXTRA=$(cat "$dir/net.args")
  if [ -f "$dir/input.osm" ] && [ -f "$dir/ref_simple.net.xml" ]; then
    "$BIN/netconvert.exe" --osm-files "$dir/input.osm" --simple-projection $NETEXTRA \
        -o "$OUT/$c.net.xml" >/dev/null 2>&1
    if python tools/oracle/compare_net.py "$dir/ref_simple.net.xml" "$OUT/$c.net.xml" >"$OUT/$c.net.log" 2>&1; then
      line "$c" net PASS; PASS=$((PASS+1))
    else
      line "$c" net FAIL "$(grep SUMMARY "$OUT/$c.net.log" | head -1)"; FAIL=$((FAIL+1))
    fi
  elif [ -f "$dir/ref_simple.net.xml" ] && [ -f "$dir/net.args" ]; then
    "$BIN/netconvert.exe" $NETEXTRA -o "$OUT/$c.net.xml" >/dev/null 2>&1
    if python tools/oracle/compare_net.py "$dir/ref_simple.net.xml" "$OUT/$c.net.xml" >"$OUT/$c.net.log" 2>&1; then
      line "$c" net PASS; PASS=$((PASS+1))
    else
      line "$c" net FAIL "$(grep SUMMARY "$OUT/$c.net.log" | head -1)"; FAIL=$((FAIL+1))
    fi
  elif [ -f "$dir/ref_plain.net.xml" ] && [ -f "$dir/net_plain.args" ]; then
    # plain-XML import: net_plain.args lists the exact node/edge/connection/type/tllogic files (make_plain_net_refs.sh)
    "$BIN/netconvert.exe" $(cat "$dir/net_plain.args") -o "$OUT/$c.plain.net.xml" >/dev/null 2>&1
    if python tools/oracle/compare_net.py "$dir/ref_plain.net.xml" "$OUT/$c.plain.net.xml" >"$OUT/$c.plain.net.log" 2>&1; then
      line "$c" net-plain PASS; PASS=$((PASS+1))
    else
      line "$c" net-plain FAIL "$(grep SUMMARY "$OUT/$c.plain.net.log" | head -1)"; FAIL=$((FAIL+1))
    fi
  else
    line "$c" net SKIP "no input.osm and no net.args"; SKIP=$((SKIP+1))
  fi
  # --- sim stage; one run per committed reference variant (ref.*, ref_<v>.*)
  if [ -f "$dir/routes.rou" ] && [ -f "$dir/ref_simple.net.xml" ] && [ -f "$dir/ref.tripinfo.xml" ]; then
    for ref in "$dir"ref*.tripinfo.xml; do
      tag=$(basename "$ref" .tripinfo.xml)          # ref or ref_<v>
      if [ "$tag" = "ref" ]; then
        EXTRA=""; [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
      else
        argsf="$dir/sim_${tag#ref_}.args"
        [ -f "$argsf" ] || { line "$c/$tag" sim SKIP "no ${argsf##*/}"; SKIP=$((SKIP+1)); continue; }
        EXTRA=$(cat "$argsf")
      fi
      "$BIN/sumo.exe" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
          --tripinfo-output "$OUT/$c.$tag.ti.xml" --fcd-output "$OUT/$c.$tag.fcd.xml" \
          --statistic-output "$OUT/$c.$tag.stat.xml" --no-step-log true $EXTRA \
          >"$OUT/$c.$tag.sumo.log" 2>&1
      if [ $? -ne 0 ]; then
        line "$c/$tag" sim-PANIC "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.$tag.sumo.log" | cut -c1-90)"; FAIL=$((FAIL+1))
        continue
      fi
      r="PASS"
      python tools/oracle/compare_sim.py --tripinfo "$ref" "$OUT/$c.$tag.ti.xml" >"$OUT/$c.$tag.ti.log" 2>&1 || r="FAIL"
      if [ -f "$dir/$tag.fcd.xml" ]; then
        python tools/oracle/compare_sim.py --fcd "$dir/$tag.fcd.xml" "$OUT/$c.$tag.fcd.xml" >"$OUT/$c.$tag.fcd.log" 2>&1 || r="FAIL"
      fi
      if [ -f "$dir/$tag.stat.xml" ]; then
        python tools/oracle/compare_stat.py "$dir/$tag.stat.xml" "$OUT/$c.$tag.stat.xml" >"$OUT/$c.$tag.stat.log" 2>&1 || r="FAIL"
      fi
      if [ "$r" = "PASS" ]; then
        line "$c/$tag" sim PASS; PASS=$((PASS+1))
      else
        d=$( (head -1 "$OUT/$c.$tag.ti.log"; head -1 "$OUT/$c.$tag.fcd.log"; tail -1 "$OUT/$c.$tag.stat.log") 2>/dev/null | grep -vE '^$' | head -1 | cut -c1-90)
        line "$c/$tag" sim FAIL "$d"; FAIL=$((FAIL+1))
      fi
    done
  else
    line "$c" sim SKIP "no routes/refs"; SKIP=$((SKIP+1))
  fi
done
echo "-------------------------------------------"
echo "PASS=$PASS FAIL=$FAIL SKIP=$SKIP"
