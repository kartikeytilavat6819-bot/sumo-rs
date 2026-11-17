#!/bin/bash
# sl07 temporary: save/load roundtrip over every sim case (removed in the final commit).
cd "$(dirname "$0")/../.." || exit 1
skip=" city_phoenix_scale_grid city_delft_campus_crossings city_grenoble_tram_bus lc_opposite sublane_bikes tls_nema_loops city_rotterdam_rail_yard "
cases=(city_vienna_scale_downtown)
export STATE_OUT=/tmp/sl07state
rm -rf "$STATE_OUT"
timeout 1500 bash tools/oracle/run_state.sh "${cases[@]}" >/tmp/sl07_state_run.txt 2>&1
res=$STATE_OUT/results.txt
echo "cases=${#cases[@]}"
if [ -f "$res" ]; then
  awk '/ state=/{n++; if ($0 !~ /state=PASS/) sf++; if ($0 !~ /load=PASS/) lf++} END{print "variants=" n+0, "state_nonPASS=" sf+0, "load_nonPASS=" lf+0}' "$res"
  grep -vE 'state=PASS +load=PASS' "$res" | cut -c1-260
  grep -E 'load=(FAIL|NOLOAD)' "$res" | awk '{print $1}' | while read -r cv; do
    c=${cv%%/*}; t=${cv#*/}
    echo "== load $cv"; tail -4 "$STATE_OUT/$c/$t.load.txt" 2>/dev/null | cut -c1-240
    grep -hE 'WARN|rror|panicked' "$STATE_OUT/$c/$t.load_ours.log" 2>/dev/null | head -6 | cut -c1-240
    echo "ours log tail:"; tail -3 "$STATE_OUT/$c/$t.load_ours.log" 2>/dev/null | cut -c1-240
    echo "pristine log tail:"; tail -3 "$STATE_OUT/$c/$t.load_pristine.log" 2>/dev/null | cut -c1-240
    echo "pristine save log tail:"; tail -3 "$STATE_OUT/$c/$t.save_pristine.log" 2>/dev/null | cut -c1-240
    echo "free: $(free -m 2>/dev/null | sed -n 2p)"
    echo "lines ours=$(wc -l < "$STATE_OUT/$c/$t.load_ours.ti.xml" 2>/dev/null) pristine=$(wc -l < "$STATE_OUT/$c/$t.load_pristine.ti.xml" 2>/dev/null)"
  done
else
  tail -20 /tmp/sl07_state_run.txt | cut -c1-260
fi
echo "STATE SUMMARY DONE"
