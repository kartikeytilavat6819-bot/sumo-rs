#!/usr/bin/env bash
# oc98 probe (temporary): new full-gate failures set_variable/Vehicle/ChangeRoute/FalseEdge and tt_output_route_probe_*
OUT=/tmp/oc98; mkdir -p "$OUT"
echo "== TraCI FalseEdge (suite runner: first diff lines)"
TRACI_ONLY=set_variable/Vehicle/ChangeRoute/FalseEdge TRACI_SUITE_OUT="$OUT/tc" bash tools/oracle/run_traci_suite.sh 2>&1 | tail -n 14 | cut -c1-260 | sed 's/^/     /'
for t in set_variable/Vehicle/ChangeRoute/FalseEdge; do
  for f in "$OUT"/tc/*/diff* "$OUT"/tc/*/*/diff*; do [ -f "$f" ] && { echo "-- $f"; head -n 20 "$f" | cut -c1-240 | sed 's/^/     /'; }; done
done
for s in two_same devnull_output null_output odd_interval; do
  c=tt_output_route_probe_$s; d=tools/oracle/cases/$c
  for side in off ours; do
    bin=${SUMO_PRISTINE_BIN}/sumo; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/sumo
    rm -f "$d/detector.xml"
    timeout 300 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --tripinfo-output "$OUT/$c.$side.ti.xml" \
        --no-step-log true $(cat "$d/sim.args") > "$OUT/$c.$side.log" 2>&1
    echo "$?" > "$OUT/$c.$side.rc"; [ -f "$d/detector.xml" ] && mv "$d/detector.xml" "$OUT/$c.$side.probe.xml"
  done
  echo "== $c rc official=$(cat $OUT/$c.off.rc) ours=$(cat $OUT/$c.ours.rc)"
  echo "-- log official (first 3):"; head -n 3 "$OUT/$c.off.log" | cut -c1-240 | sed 's/^/     /'
  echo "-- log ours (first 3):";     head -n 3 "$OUT/$c.ours.log" | cut -c1-240 | sed 's/^/     /'
  echo "-- probe diff official < ours > (first 12):"
  diff <(grep -av "generated on" "$OUT/$c.off.probe.xml" 2>/dev/null) <(grep -av "generated on" "$OUT/$c.ours.probe.xml" 2>/dev/null) | head -n 12 | cut -c1-240 | sed 's/^/     /'
  echo "-- tripinfo vs stored ref: official / ours"
  python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.off.ti.xml" 2>&1 | head -n 2 | cut -c1-200 | sed 's/^/     off: /'
  python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ours.ti.xml" 2>&1 | head -n 2 | cut -c1-200 | sed 's/^/     ours: /'
done
echo "AGENT RUN DONE"
