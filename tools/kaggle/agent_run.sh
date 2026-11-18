#!/bin/bash
# oc48 differential TraCI fuzzing: compare our sumo to the official SUMO over random call sequences.
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
OUT=${OUT:-/tmp/agent_run}
mkdir -p "$OUT"
echo "ours=$OURS pristine=$SUMO_PRISTINE_BIN"
if [ -z "$SUMO_PRISTINE_BIN" ] || [ ! -x "$SUMO_PRISTINE_BIN/sumo" ]; then
  echo "NO_PRISTINE - set SUMO_PRISTINE_BIN to the official SUMO bin dir"
  exit 2
fi
python3 -m pip install -q traci >/dev/null 2>&1
export PYTHONUNBUFFERED=1
python3 tools/oracle/fuzz_traci.py --ours "$OURS" --port 19500 --seeds 1,2,3,4,5 --n 45 --step-every 5 2>&1 | head -200
