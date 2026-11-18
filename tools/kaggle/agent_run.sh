#!/bin/bash
# oc69 measurement: dump rust vs cpp gneJ8 internals for ticket6497.
B=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$B/../.." && pwd)
cd "$ROOT" || exit 1
export NETOPTS2_OUT=/tmp/oc69_netopts
bash tools/oracle/run_netopts_new.sh itt_nc_netconvert_bugs_5xxx_ticket6497 > /dev/null 2>&1
d=/tmp/oc69_netopts/itt_nc_netconvert_bugs_5xxx_ticket6497
echo "=== CPP internals gneJ8 ==="
grep -E 'id=":gneJ8_|from="gneE6"|from=":gneJ8_5"|from=":gneJ8_8"' "$d/cpp.net.xml" | head -30
echo "=== RUST internals gneJ8 ==="
grep -E 'id=":gneJ8_|from="gneE6"|from=":gneJ8_5"|from=":gneJ8_8"' "$d/rust.net.xml" | head -30
