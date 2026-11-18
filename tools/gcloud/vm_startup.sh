#!/bin/bash
# GCE startup script for the sumo-rs gate VM: toolchain + oracle wheel on first boot, idle auto-shutdown always.
if [ ! -f /opt/gate_ready ]; then
  apt-get update -q && apt-get install -yq build-essential git python3-pip python3-venv python-is-python3 pkg-config curl \
    libxrender1 libgl1 libglu1-mesa libxft2 libxext6 libfontconfig1 libxcursor1 libxinerama1 libxfixes3
  id gate || useradd -m -s /bin/bash gate
  sudo -u gate bash -lc 'curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal'
  sudo -u gate bash -lc 'python3 -m pip install -q --break-system-packages --target /home/gate/sumo_pkg eclipse-sumo==1.28.0'
  python3 -m pip install -q --break-system-packages traci==1.28.0 sumolib==1.28.0
  touch /opt/gate_ready
fi
# user's order: keep the disk below 90 %. Frees only what no running job holds (slot / cpp-copy locks), cheapest first.
cat > /usr/local/bin/disk_guard.sh <<'EOF'
#!/bin/bash
LIMIT=90
ok() { [ "$(df --output=pcent / | tail -1 | tr -dc 0-9)" -lt $LIMIT ]; }
ok && exit 0
# 1. score/duarouter results of idle slots (rewritten by every gate anyway)
for s in 0 1 2 3 4 5 6 7; do flock -n /home/gate/slot$s.lock rm -rf /tmp/score$s /tmp/dua$s; done; ok && exit 0
# 2. instrumented C++ copies (2.2 GB each) unused for 60 min, oldest first; vm_cpp_trace.sh rebuilds them on demand
for l in $(ls -tr /home/gate/cpp-*.lock 2>/dev/null); do
  [ -n "$(find "$l" -mmin +60)" ] && flock -n "$l" rm -rf "${l%.lock}" && rm -f "$l"; ok && exit 0
done
# 3. idle slots' incremental caches, then their whole target dirs (next gate there builds cold)
for s in 0 1 2 3 4 5 6 7; do flock -n /home/gate/slot$s.lock rm -rf /home/gate/s$s/target/debug/incremental; ok && exit 0; done
for s in 0 1 2 3 4 5 6 7; do flock -n /home/gate/slot$s.lock rm -rf /home/gate/s$s/target; ok && exit 0; done
echo "disk_guard: still >= $LIMIT % with every idle slot cleaned" | logger
EOF
chmod +x /usr/local/bin/disk_guard.sh
# stop the VM after 30 idle minutes (no gate running) so it never bills for nothing
cat > /usr/local/bin/idle_stop.sh <<'EOF'
#!/bin/bash
idle=0
while true; do
  sleep 60
  /usr/local/bin/disk_guard.sh
  if pgrep -f 'vm_gate_run|/home/gate/run_|cargo|sumo ' >/dev/null; then idle=0; else idle=$((idle+1)); fi
  [ $idle -ge 30 ] && [ ! -f /etc/gate_always_on ] && shutdown -h now
done
EOF
chmod +x /usr/local/bin/idle_stop.sh
nohup /usr/local/bin/idle_stop.sh >/dev/null 2>&1 &
