#!/bin/bash
# Temporary per-city scoreboard print for this branch. Removed before merge.
bash tools/oracle/run_all.sh 2>&1 | grep -E '^city_(atlanta_motorway|london_lh|milton_keynes|barcelona_eixample)|PASS='
