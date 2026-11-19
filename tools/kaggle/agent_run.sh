#!/bin/bash
set -u
OUT=${NETOPTS2_OUT:-/tmp/netopts2}
bash tools/oracle/run_netopts_new.sh itt_nc_netconvert_bugs_5xxx_ticket6497 itt_nc_netconvert_function_railway_signals_guess_by_stops_split
echo "=== summary ==="
cat "$OUT/summary.tsv"
echo "=== ticket6497 net diff (head) ==="
diff "$OUT/itt_nc_netconvert_bugs_5xxx_ticket6497/cpp.net.xml" "$OUT/itt_nc_netconvert_bugs_5xxx_ticket6497/rust.net.xml" | head -40
echo "=== ticket6497 debug ==="
grep DEBUG "$OUT/itt_nc_netconvert_bugs_5xxx_ticket6497/rust.log" | head -40
echo "=== railway net diff (head) ==="
diff "$OUT/itt_nc_netconvert_function_railway_signals_guess_by_stops_split/cpp.net.xml" "$OUT/itt_nc_netconvert_function_railway_signals_guess_by_stops_split/rust.net.xml" | head -60