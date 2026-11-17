#!/bin/bash
# TEMPORARY (agent tr05): TraCI infra probes, C++ vs ours, junction net + infra.add.xml. Removed in the final commit.
set -u
python3 -m pip install -q traci 2>&1 | tail -2 || python3 -m pip install -q --break-system-packages traci 2>&1 | tail -2
python3 tools/oracle/cases/traci_basic/client.py --case junction --filter "infra " --port 10000
