#!/bin/bash
# C++ netconvert reference for emergency_lane (WSL, pristine sumo 1.28.0). Writes ref_simple.net.xml. Run from the repo root.
cd "$(dirname "$0")/../../../.." || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert $(cat tools/oracle/cases/emergency_lane/net.args) -o tools/oracle/cases/emergency_lane/ref_simple.net.xml