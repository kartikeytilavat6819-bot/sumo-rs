#!/bin/bash
# Scoreboard: run every oracle case through our binaries and print PASS/FAIL per stage.
# Usage: bash tools/oracle/run_all.sh [--build]
# Runs the checkout this script lives in. CARGO_TARGET_DIR selects the binaries (build them from the same code).
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target-main}
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-8}
PROFILE=${PROFILE:-debug}   # PROFILE=release runs the release binaries
BIN="$CARGO_TARGET_DIR/$PROFILE"
if [ "${1:-}" = "--build" ]; then
  cargo build -q $([ "$PROFILE" = release ] && echo --release) -p sumo -p netconvert 2>&1 | grep -E '^error' | head -5
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

# RUN_SHARDS=N: run N copies in parallel (case i goes to shard i%N), then merge their rows and totals
if [ -n "${RUN_SHARDS:-}" ] && [ -z "${RUN_SHARD:-}" ]; then
  tmp=$(mktemp -d)
  for s in $(seq 0 $((RUN_SHARDS - 1))); do RUN_SHARD=$s bash "$PWD/tools/oracle/run_all.sh" > "$tmp/$s" 2>&1 & done
  wait
  grep -hvE '^PASS=|^-+$' "$tmp"/* | sort -u
  echo "-------------------------------------------"
  awk -F'[= ]' '/^PASS=/{p+=$2; f+=$4; s+=$6} END{print "PASS=" p " FAIL=" f " SKIP=" s}' "$tmp"/*
  rm -rf "$tmp"; exit 0
fi
shard_i=-1

for dir in tools/oracle/cases/*/; do
  c=$(basename "$dir")
  shard_i=$((shard_i + 1))
  [ -n "${RUN_SHARD:-}" ] && [ $((shard_i % RUN_SHARDS)) -ne "$RUN_SHARD" ] && continue
  # TT_FILTER (newline list, set by the gate for agent branches): official-suite tt_ cases outside it are not run
  case $c in tt_*) [ -n "${TT_FILTER+x}" ] && ! grep -qxF "$c" <<<"$TT_FILTER" && continue;; esac
  # depart needs one sim per route file (15 references): run_all_variants.sh, not this loop
  if [ "$c" = depart ]; then
    line "$c" variants SKIP "15 variants: cases/depart/run_all_variants.sh"; SKIP=$((SKIP+1)); continue
  fi
  # a PENDING marker file holds the reason a case is parked (e.g. a missing port)
  # "parked: gate runtime budget" (oc49 importer) is obsolete: the gate now shards and filters tt_ cases
  if [ -f "$dir/PENDING" ] && ! grep -q '^parked: gate runtime budget' "$dir/PENDING"; then
    line "$c" pending SKIP "$(head -1 "$dir/PENDING" | cut -c1-80)"; SKIP=$((SKIP+1)); continue
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
  # --- net outputs: cases/<c>/net_outputs.txt lines "<variant> <netconvert args, @OUT@ = run dir>". Each variant
  # writes net.net.xml plus its own output files; every file in ref_net_out/<variant>/ must match its candidate.
  if [ -f "$dir/net_outputs.txt" ] && [ -f "$dir/input.osm" ]; then
    while read -r v args; do
      v=${v%$'\r'}; args=${args%$'\r'}
      [ -n "$v" ] || continue
      D="$OUT/$c.net_$v"; rm -rf "$D"; mkdir -p "$D"
      read -ra A <<< "${args//@OUT@/$D}"
      "$BIN/netconvert.exe" --osm-files "$dir/input.osm" --simple-projection "${A[@]}" \
          -o "$D/net.net.xml" >"$D/nc.log" 2>&1 </dev/null
      r="PASS"; detail=""
      for f in "$dir/ref_net_out/$v"/*; do
        n=$(basename "$f")
        if [ "$n" = net.net.xml ]; then
          python tools/oracle/compare_net.py "$f" "$D/$n" >"$D/$n.log" 2>&1 || r="FAIL"
        else
          python tools/oracle/compare_xml.py "$f" "$D/$n" >"$D/$n.log" 2>&1 || r="FAIL"
        fi
        [ "$r" = FAIL ] && [ -z "$detail" ] && detail="$n: $(grep -v '^$' "$D/$n.log" | head -1 | cut -c1-120)"
      done
      if [ "$r" = PASS ]; then
        line "$c" "net-$v" PASS; PASS=$((PASS+1))
      else
        line "$c" "net-$v" FAIL "$detail"; FAIL=$((FAIL+1))
      fi
    done < "$dir/net_outputs.txt"
  fi
  # --- expected-error stage: the official SUMO exits non-zero for this TextTest row. Score the
  # exit code and the error/warning lines against the committed ref.errors / ref.output (from
  # errors.sumo / output.sumo), not against tripinfo/fcd/statistic. Streams are compared
  # separately, each in order, exactly like texttest. Cases without EXPECT_EXIT are unaffected.
  if [ -f "$dir/EXPECT_EXIT" ] && [ -f "$dir/ref.errors" ]; then
    EXTRA=""; [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
    timeout "${SIM_TIMEOUT:-600}" "$BIN/sumo.exe" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
        --no-step-log true $EXTRA >"$OUT/$c.err.out" 2>"$OUT/$c.err.log"
    rc=$?
    if [ $rc -eq 124 ]; then
      line "$c" err-TIMEOUT "no exit after ${SIM_TIMEOUT:-600}s"; FAIL=$((FAIL+1)); continue
    fi
    if [ $rc -eq 0 ]; then
      line "$c" err-NOEXIT "official exits non-zero, ours exited 0"; FAIL=$((FAIL+1)); continue
    fi
    r="PASS"; detail=""
    # -f, not -s: an empty official stream expects an empty stream from us, so compare it too.
    if [ -f "$dir/ref.errors" ]; then
      if ! python tools/oracle/park_tt_errors.py --compare "$dir/ref.errors" "$OUT/$c.err.log" >"$OUT/$c.err.diff" 2>&1; then
        r="FAIL"; detail="stderr: $(head -2 "$OUT/$c.err.diff" | tr '\n' ' ' | cut -c1-150)"
      fi
    fi
    if [ -f "$dir/ref.output" ]; then
      if ! python tools/oracle/park_tt_errors.py --compare "$dir/ref.output" "$OUT/$c.err.out" >"$OUT/$c.out.diff" 2>&1; then
        r="FAIL"; [ -z "$detail" ] && detail="stdout: $(head -2 "$OUT/$c.out.diff" | tr '\n' ' ' | cut -c1-150)"
      fi
    fi
    if [ "$r" = "PASS" ]; then
      line "$c" err PASS; PASS=$((PASS+1))
    else
      line "$c" err FAIL "$detail"; FAIL=$((FAIL+1))
    fi
    continue
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
      # extra_outputs.txt: "<option> <suffix>" lines, each output compared against $dir/$tag.<suffix>.xml
      XOUT=""
      [ -f "$dir/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}; XOUT="$XOUT $opt $OUT/$c.$tag.$suf.xml"; done < "$dir/extra_outputs.txt"
      # extra sim outputs (cases/<c>/sim_outputs.txt, @OUT@ = run dir): written by the reference run only
      SIMOUT=""
      if [ "$tag" = "ref" ] && [ -f "$dir/sim_outputs.txt" ]; then
        SIMOUT="$OUT/$c.sim_out"; rm -rf "$SIMOUT"; mkdir -p "$SIMOUT"
        EXTRA="$EXTRA $(sed "s#@OUT@#$SIMOUT#g" "$dir/sim_outputs.txt" | tr -d '\r')"
      fi
      # add_outputs.txt: "<file> <suffix>" lines; <file> is written by the case's additional file next to itself
      # (detectors, laneData, timedEvent...), compared against $dir/$tag.<suffix>.xml and removed after the run
      [ -f "$dir/add_outputs.txt" ] && while read -r af suf; do rm -f "$dir/${af%$'\r'}"; done < "$dir/add_outputs.txt"
      # a hang in our binary must not stall the whole run (a NEMA case looped forever)
      timeout "${SIM_TIMEOUT:-600}" "$BIN/sumo.exe" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
          --tripinfo-output "$OUT/$c.$tag.ti.xml" --fcd-output "$OUT/$c.$tag.fcd.xml" \
          --statistic-output "$OUT/$c.$tag.stat.xml" --no-step-log true $EXTRA $XOUT \
          >"$OUT/$c.$tag.sumo.log" 2>&1
      rc=$?
      if [ $rc -eq 124 ]; then line "$c/$tag" sim-TIMEOUT "no exit after ${SIM_TIMEOUT:-600}s"; FAIL=$((FAIL+1)); continue; fi
      if [ $rc -ne 0 ]; then
        line "$c/$tag" sim-PANIC "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.$tag.sumo.log" | cut -c1-90)"; FAIL=$((FAIL+1))
        continue
      fi
      r="PASS"
      python tools/oracle/compare_sim.py --tripinfo "$ref" "$OUT/$c.$tag.ti.xml" >"$OUT/$c.$tag.ti.log" 2>&1 || r="FAIL"
      if [ -n "$SIMOUT" ]; then
        for sf in "$dir"/ref_sim_out/*; do
          n=$(basename "$sf")
          python tools/oracle/compare_xml.py "$sf" "$SIMOUT/$n" >"$SIMOUT/$n.log" 2>&1 || r="FAIL"
        done
      fi
      if [ -f "$dir/$tag.fcd.xml" ]; then
        python tools/oracle/compare_sim.py --fcd "$dir/$tag.fcd.xml" "$OUT/$c.$tag.fcd.xml" >"$OUT/$c.$tag.fcd.log" 2>&1 || r="FAIL"
      fi
      if [ -f "$dir/$tag.stat.xml" ]; then
        python tools/oracle/compare_stat.py "$dir/$tag.stat.xml" "$OUT/$c.$tag.stat.xml" >"$OUT/$c.$tag.stat.log" 2>&1 || r="FAIL"
      fi
      xd=""
      [ -f "$dir/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}  # git archive (core.autocrlf) gives the VM CRLF .txt files
        [ -f "$dir/$tag.$suf.xml" ] || continue
        python tools/oracle/compare_xml.py "$dir/$tag.$suf.xml" "$OUT/$c.$tag.$suf.xml" >"$OUT/$c.$tag.$suf.log" 2>&1 || { r="FAIL"; xd="$xd | $suf: $(grep -v '^$' "$OUT/$c.$tag.$suf.log" | head -1 | cut -c1-300)"; }
      done < "$dir/extra_outputs.txt"
      [ -f "$dir/add_outputs.txt" ] && while read -r af suf; do
        af=${af%$'\r'}; suf=${suf%$'\r'}
        if [ -f "$dir/$tag.$suf.xml" ]; then
          python tools/oracle/compare_xml.py "$dir/$tag.$suf.xml" "$dir/$af" >"$OUT/$c.$tag.$suf.log" 2>&1 || { r="FAIL"; xd="$xd | $suf: $(grep -v '^$' "$OUT/$c.$tag.$suf.log" | head -1 | cut -c1-300)"; }
        fi
        rm -f "$dir/$af"
      done < "$dir/add_outputs.txt"
      # edgeData output: the variant's additional file writes out_edgedata.xml next to itself (C++ resolves the path that way)
      if [ -f "$dir/$tag.edge.xml" ]; then
        python tools/oracle/compare_xml.py "$dir/$tag.edge.xml" "$dir/out_edgedata.xml" >"$OUT/$c.$tag.edge.log" 2>&1 || r="FAIL"
        rm -f "$dir/out_edgedata.xml"
      fi
      if [ "$r" = "PASS" ]; then
        line "$c/$tag" sim PASS; PASS=$((PASS+1))
      else
        d=$( (head -1 "$OUT/$c.$tag.ti.log"; head -1 "$OUT/$c.$tag.fcd.log"; tail -1 "$OUT/$c.$tag.stat.log") 2>/dev/null | grep -vE '^$' | head -1 | cut -c1-90)
        [ -f "$OUT/$c.$tag.edge.log" ] && d="$d | edge: $(grep -v '^$' "$OUT/$c.$tag.edge.log" | head -2 | tr '\n' ' ' | cut -c1-900)"
        d="$d$xd"
        [ -n "$SIMOUT" ] && d="$d | out: $(grep -h -m1 -E 'DIFF|UNREADABLE' "$SIMOUT"/*.log | cut -c1-160)"
        line "$c/$tag" sim "FAIL $d"; FAIL=$((FAIL+1))
      fi
    done
  else
    line "$c" sim SKIP "no routes/refs"; SKIP=$((SKIP+1))
  fi
done
echo "-------------------------------------------"
echo "PASS=$PASS FAIL=$FAIL SKIP=$SKIP"
