#!/bin/bash
# Temporary (agent vs01): trace of OpenDRIVE geometry for opendrive_4arm. Removed in the final commit.
set -u
mkdir -p /tmp/dbg4 && cp tools/oracle/import_cases/opendrive_4arm/* /tmp/dbg4/
cd /tmp/dbg4 && "$CARGO_TARGET_DIR/debug/netconvert" --opendrive opendrive.xodr --opendrive.position-ids -o dbg.net.xml > dbg.log 2>&1
grep -c DBG dbg.log
grep DBG dbg.log | head -60
grep -v DBG dbg.log | head -5
