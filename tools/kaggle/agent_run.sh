#!/bin/bash
set -u
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
cd "$ROOT" || exit 1
export NETOPTS2_OUT=/tmp/oc68_dbg
bash tools/oracle/run_netopts_new.sh itt_nc_netconvert_function_railway_repair_stops >/dev/null 2>&1
grep 'OC68' "$NETOPTS2_OUT/itt_nc_netconvert_function_railway_repair_stops/rust.log" | head -10
echo DONE
