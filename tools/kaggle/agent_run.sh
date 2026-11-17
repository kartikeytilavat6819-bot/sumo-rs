#!/bin/bash
# TEMPORARY tl02 measurement: fn.0 kinematics and TL state, ours vs official C++ (tls_waut).
set -u
OURS=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
CPP=$SUMO_PRISTINE_BIN/sumo
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
python tools/kaggle/agent_trace.py "tools/oracle/cases/tls_waut" "$OURS" "$CPP" fn.0 2>&1 | head -120
