#!/bin/bash
set -u
OUT=${NETOPTS2_OUT:-/tmp/netopts2}
CASE=tools/oracle/cases/itt_nc_netconvert_bugs_5xxx_ticket6497
bash tools/oracle/run_netopts_new.sh itt_nc_netconvert_bugs_5xxx_ticket6497 itt_nc_netconvert_function_railway_signals_guess_by_stops_split itt_nc_netconvert_import_dlr_navteq_vclass_cycles_only_on_1__deprecated__dismissing__removing_peds_cycles itt_nc_netconvert_import_dlr_navteq_vclass_cycles_only_on_1__deprecated__removing_peds_cycles itt_nc_netconvert_import_dlr_navteq_vclass_peds_cycles_only_on_1__deprecated__dismissing__removing_peds_cycles itt_nc_netconvert_import_dlr_navteq_vclass_peds_cycles_only_on_1__deprecated__removing_peds_cycles itt_nc_netconvert_import_dlr_navteq_vclass_peds_only_on_1__deprecated__dismissing__removing_peds_cycles itt_nc_netconvert_import_dlr_navteq_vclass_peds_only_on_1__deprecated__removing_peds_cycles
echo "=== summary ==="
cat "$OUT/summary.tsv"
echo "=== ticket6497 net diff (head) ==="
diff "$OUT/itt_nc_netconvert_bugs_5xxx_ticket6497/cpp.net.xml" "$OUT/itt_nc_netconvert_bugs_5xxx_ticket6497/rust.net.xml" | head -40
echo "=== OUR needsCont/rightTurnConflict trace ==="
grep -E "DEBUG|needsCont|NC-TRACE|RTC-TRACE" "$OUT/itt_nc_netconvert_bugs_5xxx_ticket6497/rust.log" | head -120
echo "=== build instrumented official netconvert ==="
CPP_TRACE_BIN=$(bash tools/gcloud/vm_cpp_trace.sh oc91 tools/kaggle/cpp_trace.patch netconvert)
echo "instrumented binary: $CPP_TRACE_BIN"
"$CPP_TRACE_BIN" --write-license -s "$CASE/input_net.net.xml" -x "$CASE/input_plain.con.xml" -o "$OUT/cpp_trace.net.xml" > "$OUT/cpp_trace.log" 2>&1
echo "=== OFFICIAL needsCont/rightTurnConflict trace ==="
grep -E "CPP-NC|CPP-RTC" "$OUT/cpp_trace.log" | head -120
echo "=== deprecated error rows: rust logs ==="
for r in itt_nc_netconvert_import_dlr_navteq_vclass_cycles_only_on_1__deprecated__dismissing__removing_peds_cycles itt_nc_netconvert_import_dlr_navteq_vclass_peds_only_on_1__deprecated__dismissing__removing_peds_cycles; do
  echo "--- $r cpp >>"; tail -5 "$OUT/$r/cpp.log"
  echo "--- $r rust >>"; tail -12 "$OUT/$r/rust.log"
done
echo "=== railway net diff (head) ==="
diff "$OUT/itt_nc_netconvert_function_railway_signals_guess_by_stops_split/cpp.net.xml" "$OUT/itt_nc_netconvert_function_railway_signals_guess_by_stops_split/rust.net.xml" | head -60
