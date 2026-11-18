#!/bin/bash
# Gate helper for oc68: dump gneJ39 area from cpp/rust nets for railway_repair_stops.
set -u
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
cd "$ROOT" || exit 1
export NETOPTS2_OUT=/tmp/oc68_rail2
bash tools/oracle/run_netopts_new.sh itt_nc_netconvert_function_railway_repair_stops >/dev/null 2>&1
D="$NETOPTS2_OUT/itt_nc_netconvert_function_railway_repair_stops"
echo "=== CPP edges at gneJ39 +- ==="
grep -nE 'gneJ39|gneJ38|id="-a"|id="a"|id="b"|id="-b"' "$D/cpp.net.xml" | head -60
echo "=== RUST edges at gneJ39 +- ==="
grep -nE 'gneJ39|gneJ38|id="-a"|id="a"|id="b"|id="-b"' "$D/rust.net.xml" | head -60
echo "=== DONE ==="
