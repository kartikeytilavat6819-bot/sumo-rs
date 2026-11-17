#!/bin/bash
# TEMPORARY tl01 measurement: TL state per step (ours vs official C++) for the two failing TLS rows.
set -u
ROOT=$(pwd)
OURS=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
CPP=$SUMO_PRISTINE_BIN/sumo
OUT=/tmp/tl01
mkdir -p "$OUT"
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
for c in tls_waut tls_actuated_loops; do
  python tools/kaggle/agent_trace.py "tools/oracle/cases/$c" "$OURS" "$CPP" "$OUT" 2>&1 | head -60
done
