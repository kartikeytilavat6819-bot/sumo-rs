#!/bin/bash
# oc44 differential fuzz measurement (branch-only; removed before final commit).
# Gate env already: cd /tmp/src, CARGO_TARGET_DIR=/tmp/target,
# SUMO_PRISTINE_BIN=/tmp/sumo_pkg/sumo/bin, SUMO_HOME=/tmp/sumo_pkg/sumo.
# Outer gate timeout is 600s, so keep the internal budget below that.
FUZZ_FIRST=${FUZZ_FIRST:-7000} FUZZ_BUDGET=${FUZZ_BUDGET:-150} bash tools/oracle/fuzz_sim.sh 30
