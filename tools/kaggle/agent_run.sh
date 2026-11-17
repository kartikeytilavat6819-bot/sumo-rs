#!/bin/bash
# Temporary (agent vs03): importer oracle run. Removed in the final commit.
set -u
bash tools/oracle/run_importers.sh "$CARGO_TARGET_DIR/debug/netconvert" "$SUMO_PRISTINE_BIN/netconvert" /tmp/import_out
for d in /tmp/import_out/*/; do
  if [ -s "$d/diff.txt" ]; then
    echo "== $(basename "$d")"
    head -12 "$d/diff.txt"
  fi
done
