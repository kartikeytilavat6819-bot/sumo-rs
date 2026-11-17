#!/bin/bash
# agent-nw01 temporary runner (removed before merge): netwrite output variants, ours vs pristine refs.
cd "$(dirname "$0")/../.." || exit 1
echo "== ours"
NC=${CARGO_TARGET_DIR:-/tmp/target}/debug/netconvert NWV_OUT=/tmp/nwv_ours bash tools/oracle/run_netwrite_variants.sh
echo "ours exit=$?"
echo "== pristine self-check"
NC=$SUMO_PRISTINE_BIN/netconvert NWV_OUT=/tmp/nwv_pristine bash tools/oracle/run_netwrite_variants.sh
echo "pristine exit=$?"
