#!/bin/bash
# Gate helper for oc68: reproduce the two renaming panics with a backtrace.
set -u
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
cd "$ROOT" || exit 1
export RUST_BACKTRACE=1
export NETOPTS2_OUT=/tmp/oc68_bt
bash tools/oracle/run_netopts_new.sh \
  itt_nc_netconvert_function_renaming_joinedTLS_renumber \
  itt_nc_netconvert_function_renaming_prefix_joinedTLS >/dev/null 2>&1
for row in itt_nc_netconvert_function_renaming_joinedTLS_renumber itt_nc_netconvert_function_renaming_prefix_joinedTLS; do
  echo "=== $row ==="
  grep -m1 -A40 panicked "$NETOPTS2_OUT/$row/rust.log"
done
echo "=== DONE ==="
