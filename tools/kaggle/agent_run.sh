#!/bin/bash
# Temporary per-city scoreboard print for this branch. Removed before merge.
bash tools/oracle/run_all.sh 2>&1 | grep -E '^city_atlanta_motorway|PASS='
