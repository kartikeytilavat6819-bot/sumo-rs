#!/bin/bash
# Probe the no_permitted_route_lane row: run the official sumo via a wrapper
# that records its own stdout/stderr and exit code, then replay the runner.
set -u
TEST=tools/oracle/complex_bundle/vehicle/moveToXY/no_permitted_route_lane
D=/tmp/oc100
rm -rf "$D"
mkdir -p "$D"
cat > "$D/sumo_wrapper.sh" <<'EOF'
#!/bin/bash
"${SUMO_PRISTINE_BIN}/sumo" "$@" > /tmp/oc100/sumo.stdout 2> /tmp/oc100/sumo.stderr
rc=$?
echo "$rc" > /tmp/oc100/sumo.exit
exit $rc
EOF
chmod +x "$D/sumo_wrapper.sh"
cp -r "$TEST" "$D/run"
NC="${SUMO_PRISTINE_BIN}/netconvert"
[ -x "$NC" ] || NC="${CARGO_TARGET_DIR}/debug/netconvert"
(cd "$D/run" && SUMO_BINARY="$D/sumo_wrapper.sh" NETCONVERT_BINARY="$NC" PYTHONPATH="${SUMO_HOME}/tools:${PYTHONPATH:-}" python3 runner.py > console.txt 2>&1)
echo "===== runner console ====="
cat "$D/run/console.txt"
echo "===== official sumo exit code ====="
cat "$D/sumo.exit" 2>/dev/null || echo "(no exit recorded)"
echo "===== official sumo stdout ====="
cat "$D/sumo.stdout" 2>/dev/null || true
echo "===== official sumo stderr ====="
cat "$D/sumo.stderr" 2>/dev/null || true
