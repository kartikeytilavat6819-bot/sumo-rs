#!/bin/bash
# oc51: focused lane-change / sublane measure. Runs the LC cases through our binary and compares
# against the committed pristine refs. Prints only failures and the first differing lines.
set -u
cd "$(cd "$(dirname "$0")/../.." && pwd)"
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug
[ -x "$BIN/sumo.exe" ] || ln -sf "$BIN/sumo" "$BIN/sumo.exe" 2>/dev/null
OUT=/tmp/oc51-lc; mkdir -p "$OUT"
CASES="lanechange lcparams lc_contright lc_contright_lat04 lc_dk2008 lc_dk2008_dur \
lc_drop_sl2015 lc_lc2013 lc_lc2013_lanedrop lc_lc2013_maxlat lc_lc2013_turnlanes \
lc_lc2013_turnlanes_dur lc_mixed_models lc_oppo lc_opposite lc_oppo_dur \
lc_ramp_sl2015_lat04 lc_ramp_sl2015_lat08 lc_sl2015 lc_sl2015_bikes_lat08 \
lc_sl2015_dur lc_sl2015_lanedrop_lat08 lc_sl2015_oppo_lat04 \
lc_sl2015_overtake_right_lat08 lc_sl2015_stop_lat08 lc_sl2015_turnlanes_lat04 \
lc_stop_lc2013 sl2015 sl2015_lat04 sl2015_lat08 state_lanechange state_sublane \
sublanes sublane_bikes bike_sublane_overtake cv_sublane_align hw_offramp_lc \
zipper mixed_vclass_lc rnd_lcsigma accel_lane_ramps city_phoenix_scale_grid"
for c in $CASES; do
  d=tools/oracle/cases/$c
  [ -f "$d/routes.rou" ] && [ -f "$d/ref_simple.net.xml" ] || { echo "$c SKIP-no-input"; continue; }
  for ref in "$d"/ref*.tripinfo.xml; do
    [ -f "$ref" ] || continue
    tag=$(basename "$ref" .tripinfo.xml)
    if [ "$tag" = ref ]; then EXTRA=$(cat "$d/sim.args" 2>/dev/null)
    else af="$d/sim_${tag#ref_}.args"; [ -f "$af" ] || continue; EXTRA=$(cat "$af"); fi
    timeout 600 "$BIN/sumo.exe" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$c.$tag.ti.xml" --fcd-output "$OUT/$c.$tag.fcd.xml" \
      --statistic-output "$OUT/$c.$tag.stat.xml" --no-step-log true $EXTRA \
      >"$OUT/$c.$tag.log" 2>&1
    rc=$?
    if [ $rc -ne 0 ]; then
      echo "$c/$tag PANIC rc=$rc $(grep -m1 -E 'panicked at|Error:' "$OUT/$c.$tag.log" | cut -c1-100)"
      continue
    fi
    r=PASS; detail=""
    python tools/oracle/compare_sim.py --tripinfo "$ref" "$OUT/$c.$tag.ti.xml" >"$OUT/$c.$tag.ti.log" 2>&1 || r=FAIL
    if [ "$r" = FAIL ]; then detail="ti:$(grep -vE '^$' "$OUT/$c.$tag.ti.log" | head -1 | cut -c1-120)"; fi
    if [ -f "$d/$tag.fcd.xml" ]; then
      python tools/oracle/compare_sim.py --fcd "$d/$tag.fcd.xml" "$OUT/$c.$tag.fcd.xml" >"$OUT/$c.$tag.fcd.log" 2>&1 || r=FAIL
      [ "$r" = FAIL ] && [ -z "$detail" ] && detail="fcd:$(grep -vE '^$' "$OUT/$c.$tag.fcd.log" | head -1 | cut -c1-120)"
    fi
    if [ -f "$d/$tag.stat.xml" ]; then
      python tools/oracle/compare_stat.py "$d/$tag.stat.xml" "$OUT/$c.$tag.stat.xml" >"$OUT/$c.$tag.stat.log" 2>&1 || r=FAIL
      [ "$r" = FAIL ] && [ -z "$detail" ] && detail="stat:$(grep -vE '^$' "$OUT/$c.$tag.stat.log" | head -1 | cut -c1-120)"
    fi
    [ "$r" = FAIL ] && echo "$c/$tag FAIL $detail"
  done
done
echo "OC51 LC MEASURE DONE"
