#!/usr/bin/env bash
# oc98 probe (temporary): the seven tt_output_route_probe_* rows exactly like the run_all.sh simulation stage
# (-n ref_simple.net.xml -r routes.rou --tripinfo/--fcd/--statistic-output --no-step-log true + sim.args), official and ours,
# scored like run_all: compare_sim --tripinfo, compare_xml of the probe file against ref.probe.xml
OUT=/tmp/oc98; mkdir -p "$OUT"
for s in devnull_output null_output odd_interval two_same two_same_b5 two_same_begin vTypes; do
  c=tt_output_route_probe_$s; d=tools/oracle/cases/$c
  for side in off ours; do
    bin=${SUMO_PRISTINE_BIN}/sumo; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/sumo
    rm -f "$d/detector.xml" "$d"/*.probe.out
    timeout 300 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --tripinfo-output "$OUT/$c.$side.ti.xml" \
        --fcd-output "$OUT/$c.$side.fcd.xml" --statistic-output "$OUT/$c.$side.stat.xml" --no-step-log true $(cat "$d/sim.args") > "$OUT/$c.$side.log" 2>&1
    echo "$?" > "$OUT/$c.$side.rc"
    [ -f "$d/detector.xml" ] && mv "$d/detector.xml" "$OUT/$c.$side.probe.xml"
  done
  echo "== $c rc official=$(cat $OUT/$c.off.rc) ours=$(cat $OUT/$c.ours.rc) probe_file official=$([ -f $OUT/$c.off.probe.xml ] && echo y || echo n) ours=$([ -f $OUT/$c.ours.probe.xml ] && echo y || echo n)"
  for side in off ours; do
    r=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.$side.ti.xml" 2>&1 | head -n 1 | cut -c1-200)
    echo "   $side tripinfo: ${r:-OK}"
    if [ -f "$d/ref.probe.xml" ]; then
      r=$(python tools/oracle/compare_xml.py "$d/ref.probe.xml" "$OUT/$c.$side.probe.xml" 2>&1 | grep -v '^$' | head -n 2 | cut -c1-260 | tr '\n' ' ')
      echo "   $side probe vs ref.probe.xml: ${r:-OK}"
    fi
  done
  echo "   ours log (first 2):"; head -n 2 "$OUT/$c.ours.log" | cut -c1-240 | sed 's/^/      /'
done
echo "AGENT RUN DONE"
