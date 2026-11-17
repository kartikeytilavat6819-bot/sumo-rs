#!/bin/bash
# TEMPORARY (agent tr05): TraCI infra probes, C++ vs ours, junction net + infra.add.xml. Removed in the final commit.
set -u
cd tools/oracle/cases/traci_basic
NET=../junction/ref_simple.net.xml; ROU=../junction/routes.rou
echo "== C++ load of infra.add.xml"
timeout 60 "$SUMO_PRISTINE_BIN/sumo" -n $NET -r $ROU --additional-files "$PWD/infra.add.xml" --end 3 --no-step-log true --no-warnings true 2>&1 | head -20
echo "== ours load of infra.add.xml"
timeout 60 "$CARGO_TARGET_DIR/debug/sumo" -n $NET -r $ROU --additional-files "$PWD/infra.add.xml" --end 3 --no-step-log true --no-warnings true 2>&1 | head -20
cd ../../../..
python3 tools/oracle/cases/traci_basic/client.py --case junction --filter "infra " --port 10000
