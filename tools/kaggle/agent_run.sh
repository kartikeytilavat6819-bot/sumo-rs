#!/bin/bash
# Temporary agent script (vs04): importer oracle + varwidth diff. Removed in the final commit.
bash tools/oracle/run_importers.sh "$CARGO_TARGET_DIR/debug/netconvert" "$SUMO_PRISTINE_BIN/netconvert" /tmp/import_out 2>&1 | tail -40
echo "--- varwidth diff"
head -60 /tmp/import_out/opendrive_4arm_varwidth/diff.txt
echo "--- varwidth ours log"
tail -5 /tmp/import_out/opendrive_4arm_varwidth/ours/netconvert.log
