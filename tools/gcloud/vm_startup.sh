#!/bin/bash
# GCE startup script for the sumo-rs gate VM: toolchain + oracle wheel on first boot, idle auto-shutdown always.
if [ ! -f /opt/gate_ready ]; then
  apt-get update -q && apt-get install -yq build-essential git python3-pip python3-venv pkg-config curl
  id gate || useradd -m -s /bin/bash gate
  sudo -u gate bash -lc 'curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal'
  sudo -u gate bash -lc 'python3 -m pip install -q --break-system-packages --target /home/gate/sumo_pkg eclipse-sumo==1.28.0'
  touch /opt/gate_ready
fi
# stop the VM after 30 idle minutes (no gate running) so it never bills for nothing
cat > /usr/local/bin/idle_stop.sh <<'EOF'
#!/bin/bash
idle=0
while true; do
  sleep 60
  if pgrep -f 'vm_gate_run|cargo|sumo ' >/dev/null; then idle=0; else idle=$((idle+1)); fi
  [ $idle -ge 30 ] && shutdown -h now
done
EOF
chmod +x /usr/local/bin/idle_stop.sh
nohup /usr/local/bin/idle_stop.sh >/dev/null 2>&1 &
