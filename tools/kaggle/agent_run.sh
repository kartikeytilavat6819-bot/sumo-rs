#!/bin/bash
# Probe the gate VM for the materials needed to obtain a TraCI test client.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
echo "=== pristine bin ==="
ls "$SUMO_PRISTINE_BIN" | head -80
echo "=== SUMO_HOME top ==="
ls "$SUMO_HOME" | head -60
echo "=== SUMO_HOME/include ==="
ls "$SUMO_HOME/include" 2>/dev/null | head -40 || echo none
echo "=== SUMO_HOME/lib ==="
ls "$SUMO_HOME/lib" 2>/dev/null | head -40 || echo none
echo "=== SUMO_HOME/tools/traci ==="
ls "$SUMO_HOME/tools/traci" 2>/dev/null | head -30 || echo none
echo "=== build tools ==="
for c in cmake g++ gcc make ccache pkg-config python3; do echo "$c: $(command -v $c || echo none)"; done
echo "=== sumo source dirs on VM ==="
find /home/gate -maxdepth 4 -type d -name 'traci_testclient' 2>/dev/null | head
find / -maxdepth 4 -type d -name 'sumo-src' 2>/dev/null | head
echo "=== relevant libs ==="
find "$SUMO_HOME" -maxdepth 3 \( -name 'libtraci*' -o -name 'libsumo*' -o -name 'TraCIAPI*' -o -name 'storage.h' -o -name 'socket.h' \) 2>/dev/null | head -40
echo "=== python traci import ==="
PYTHONPATH="$SUMO_HOME/tools" python3 -c 'import traci; print("traci OK", traci.__file__)' 2>&1 | head -5
echo "PROBE DONE"
