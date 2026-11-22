#!/bin/bash
# oc98 rail-rerouter diagnostics. Run from the repo root after the build.
# Prints, for each of the two target cases and both binaries, the warnings, the
# vehroute replacements (who rerouted the train and to which edges), and the
# tripinfo line. This shows whether the on-init reroute through MSTriggeredRerouter
# fires on our side and which route the train actually takes.
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
REF=${SUMO_PRISTINE_BIN:?SUMO_PRISTINE_BIN not set}/sumo
OUT=${OUT:-/tmp/agent_run_oc98}
mkdir -p "$OUT"

for case in tt_rail_rerouter_closing_detour_onInit tt_rail_rerouter_closing_short_closing_wait; do
  C=tools/oracle/cases/$case
  echo "################ $case"
  for side in ours pristine; do
    if [ "$side" = ours ]; then BIN=$OURS; else BIN=$REF; fi
    if [ "$side" = ours ]; then DBG=OC98DBG=1; else DBG=; fi
    timeout 300 env $DBG "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
      $(cat "$C/sim.args") --no-step-log true --end 400 \
      --tripinfo-output "$OUT/$case.$side.ti.xml" \
      --vehroute-output "$OUT/$case.$side.vr.xml" \
      --fcd-output "$OUT/$case.$side.fcd.xml" \
      >"$OUT/$case.$side.log" 2>&1
    echo "== $side exit=$?"
    echo "-- OC98DBG (our side only):"
    grep 'OC98DBG' "$OUT/$case.$side.log" | head -40
    echo "-- warnings/first log lines:"
    grep -E 'Warning|Error|skips stop|Terminating|no valid route|could not' "$OUT/$case.$side.log" | head -10
    echo "-- vehroutes (route elements):"
    grep -E '<route' "$OUT/$case.$side.vr.xml" | head -10
    echo "-- tripinfo:"
    grep '<tripinfo ' "$OUT/$case.$side.ti.xml" | head -3
    echo "-- first fcd timestamps (vehicle position/edge):"
    grep -E '<timestep ' "$OUT/$case.$side.fcd.xml" | head -4
    grep -E '<vehicle ' "$OUT/$case.$side.fcd.xml" | head -4
  done
done

# ticket17583: 26 vehicles on a circular track with two destProbRerouters.
# Official: none arrive before t=100. Ours: v.25 arrives. Show every tripinfo,
# the flow statistics, and the routes the rerouters assign.
C=tools/oracle/cases/tt_bugs_17xxx_ticket17583
echo "################ tt_bugs_17xxx_ticket17583"
for side in ours pristine; do
  if [ "$side" = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  timeout 300 "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
    $(cat "$C/sim.args") --no-step-log true \
    --tripinfo-output "$OUT/17583.$side.ti.xml" \
    --vehroute-output "$OUT/17583.$side.vr.xml" \
    --statistic-output "$OUT/17583.$side.stat.xml" \
    --fcd-output "$OUT/17583.$side.fcd.xml" \
    >"$OUT/17583.$side.log" 2>&1
  echo "== $side exit=$?"
  echo "-- tripinfos:"
  grep '<tripinfo ' "$OUT/17583.$side.ti.xml"
  echo "-- vehicles stat:"
  grep '<vehicles ' "$OUT/17583.$side.stat.xml"
  echo "-- v.25 route replacements (first 12 route tags):"
  grep -E '<route' "$OUT/17583.$side.vr.xml" | head -12
  echo "-- v.25 FCD path (0.5s samples from t=60, edge/lane changes):"
  python3 - "$OUT/17583.$side.fcd.xml" <<'PY'
import sys, xml.etree.ElementTree as ET
t = ET.parse(sys.argv[1]).getroot()
last = -1
for ts in t.iter('timestep'):
    sec = float(ts.get('time'))
    if sec < 60: continue
    dec = round(sec * 2)
    for v in ts.iter('vehicle'):
        if v.get('id') == 'v.25' and dec != last:
            last = dec
            print(f"{sec:.1f} spd={v.get('speed')} pos={v.get('pos')} lane={v.get('lane')}")
PY
done

# ticket4299: ES1 waitingTime 72.00 (ref) vs 75.80 (ours), step-length 0.1.
# Dump tripinfo and a fine ES1 speed/pos/lane trace around t=94..102 so the
# extra waiting steps can be located on the two sides.
C=tools/oracle/cases/tt_bugs_4xxx_ticket4299
echo "################ tt_bugs_4xxx_ticket4299"
for side in ours pristine; do
  if [ "$side" = ours ]; then BIN=$OURS; DBG=OC98DBG=1; else BIN=$SUMO_PRISTINE_BIN/sumo; DBG=; fi
  timeout 300 env $DBG "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
    $(cat "$C/sim.args") --no-step-log true \
    --tripinfo-output "$OUT/4299.$side.ti.xml" \
    --fcd-output "$OUT/4299.$side.fcd.xml" \
    >"$OUT/4299.$side.log" 2>&1
  echo "== $side exit=$?"
  grep '<tripinfo ' "$OUT/4299.$side.ti.xml"
  echo "-- warnings:"
  grep -i 'warn\|collision' "$OUT/4299.$side.log" | head -5
  echo "-- ES1 fine trace (0.2s samples in t=94..102):"
  python3 - "$side" "$OUT/4299.$side.fcd.xml" <<'PY'
import sys, xml.etree.ElementTree as ET
side = sys.argv[1]
t = ET.parse(sys.argv[2]).getroot()
shown = -1
for ts in t.iter('timestep'):
    sec = float(ts.get('time'))
    if not (94 <= sec <= 102): continue
    dec = round(sec * 5)
    for v in ts.iter('vehicle'):
        if v.get('id') == 'ES1' and dec != shown:
            shown = dec
            print(f"{side} {sec:.1f} spd={v.get('speed')} pos={v.get('pos')} lane={v.get('lane')}")
PY
done

# oc105 expected-error measurement (from wave2): compare our sumo with the
# official binary on the SSM and rerouting expected-error rows.
OC105_OUT=/tmp/agent_run_oc105
mkdir -p "$OC105_OUT"
for c in tt_devices_ssm_bugs_ticket13918 tt_devices_rerouting_reroute_congestions_exit_times; do
  D="tools/oracle/cases/$c"
  FLAGS=$(cat "$D/sim.args")
  echo "==== $c"
  echo "sim.args: $FLAGS"
  for side in our official; do
    if [ "$side" = our ]; then BIN="$OURS"; else BIN="$REF"; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $FLAGS \
      >"$OC105_OUT/$c.$side.out" 2>"$OC105_OUT/$c.$side.err"
    echo "$side exit=$?"
    echo "$side stderr:"; head -20 "$OC105_OUT/$c.$side.err"
    echo "$side stdout:"; head -12 "$OC105_OUT/$c.$side.out"
  done
done
# rail stop-skipping decision (per-stop delay/cost) — printed last so it
# survives the gate's tail truncation
echo "==== OC98 RAIL STOP-SKIP DECISION"
for case in tt_rail_rerouter_closing_detour_onInit tt_rail_rerouter_closing_short_closing_wait; do
  echo "---- $case"
  grep 'OC98DBG reroute-stop\|OC98DBG reroute info\|OC98DBG reroute panic\|OC98DBG interval\|OC98DBG getClosed\|OC98DBG prohibit\|OC98DBG railEdge' "$OUT/$case.ours.log" | head -60
done
echo "==== OC98 4299 LINK-LEADER FOES"
grep 'OC98DBG 4299' "$OUT/4299.ours.log" | head -80
echo "OC98 DIAGNOSTIC DONE"
