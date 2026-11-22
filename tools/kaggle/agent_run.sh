#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc111err
rm -rf "$OUT"; mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/netconvert"
OFF="$SUMO_PRISTINE_BIN/netconvert"
for c in itt_nc_netconvert_errors_invalid_argument_vclass itt_nc_netconvert_errors_false_plain_output itt_nc_netconvert_function_keep_edges_boundary__deprecated_ itt_nc_netconvert_function_keep_edges_boundary_malformed_boundary1 itt_nc_netconvert_function_keep_edges_boundary_malformed_boundary2; do
  D="tools/oracle/cases/$c"
  ARGS=$(cat "$D/net.args")
  echo "==== $c"
  $OUR $ARGS -o "$OUT/$c.our.net.xml" >"$OUT/$c.our.log" 2>&1
  echo "our rc=$? stderr: $(grep -i 'error\|panicked' "$OUT/$c.our.log" | head -2 | tr '\n' ' ')"
  $OFF $ARGS -o "$OUT/$c.ref.net.xml" >"$OUT/$c.ref.log" 2>&1
  echo "ref rc=$? stderr: $(grep -i 'error\|panicked' "$OUT/$c.ref.log" | head -2 | tr '\n' ' ')"
done
