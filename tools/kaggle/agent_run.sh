#!/bin/bash
# sl07 temporary: save/load roundtrip over every sim case (removed in the final commit).
cd "$(dirname "$0")/../.." || exit 1
skip=" city_phoenix_scale_grid city_delft_campus_crossings city_grenoble_tram_bus lc_opposite sublane_bikes tls_nema_loops city_rotterdam_rail_yard "
cases=()
for d in tools/oracle/cases/*/; do
  c=$(basename "$d")
  case "$skip" in *" $c "*) continue ;; esac
  cases+=("$c")
done
export STATE_OUT=/tmp/sl07state
rm -rf "$STATE_OUT"
timeout 1500 bash tools/oracle/run_state.sh "${cases[@]}" >/tmp/sl07_state_run.txt 2>&1
res=$(ls -d "$STATE_OUT"/state-*/results.txt 2>/dev/null | head -1)
echo "cases=${#cases[@]} results=$res"
[ -n "$res" ] && { echo "variants=$(grep -c ' state=' "$res") state_PASS=$(grep -c 'state=PASS' "$res") load_PASS=$(grep -c 'load=PASS' "$res")"; grep -vE 'state=PASS +load=PASS' "$res" | cut -c1-260; }
echo "STATE SUMMARY DONE"
