#!/bin/bash
# oc30 measurement: compare findIntermodalRoute probes between our sumo and pristine C++.
python3 tools/oracle/cases/traci_basic/client.py --case junction --filter "findIntermodalRoute" 2>&1 | grep -E "findIntermodalRoute|NEW-PROBE|MATCH|DIFF|match|TOTAL|ours:" | head -60
