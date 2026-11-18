#!/bin/bash
# agent oc34 shortcut-audit measurement: run TraCI probes relevant to shortcuts and print DIFFs.
export PYTHONPATH="/tmp/sumo_pkg:${PYTHONPATH}"
cd /tmp/src 2>/dev/null || cd "$(dirname "$0")/../.."
FILTER="findIntermodalRoute,getOption,getConstraints,subscribe(nope),_getUniversal,loadState,moveToXY,setFlow,setParameter(nope),wrong variable,unsupported variable,setMode(nope),getParameter(v0, laneChangeModel,device.tripinfo"
timeout 560 python tools/oracle/cases/traci_basic/client.py --case junction --filter "$FILTER" 2>&1 | grep -E "DIFF|match|MISMATCH|TOTAL|^\[" | head -120
