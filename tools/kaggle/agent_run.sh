#!/usr/bin/env bash
# lb01 probe run: the 8 TraCI probes added for the libsumo shortcut ports, both sides, base port 9300.
python tools/oracle/cases/traci_basic/client.py --case all --port 9300 --filter "duration range,BoardingDuration,setRouteID,without stops"
