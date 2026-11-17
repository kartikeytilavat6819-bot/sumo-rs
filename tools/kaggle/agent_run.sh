#!/bin/bash
# cy07 scale cases: wall-clock of ours (release) vs official C++ 1.28.0 on the three large city cases.
# Sim only (same flags as run_all: sim.args, outputs to /tmp), median of 3 runs per binary.
# Runs from the repo root on the gate VM (CARGO_TARGET_DIR and SUMO_PRISTINE_BIN set by vm_gate_run.sh).
set -u
CASES="city_dhaka_scale_jam"
PRI=${SUMO_PRISTINE_BIN:?}/sumo
T=${CARGO_TARGET_DIR:-/tmp/target}
PROF=release
if ! timeout 240 cargo build -q --release -p sumo -p netconvert > /tmp/cy07_rel.log 2>&1; then
  echo "release build not done in 240 s, timing debug instead (NOT comparable with PERF.md)"
  PROF=debug
fi
OURS=$T/$PROF/sumo
echo "profile=$PROF"
t() { # t <bin> <case> <tag> : wall seconds of one sim run, outputs /tmp/cy07_<case>_<tag>.*
  local d=tools/oracle/cases/$2 s e rc
  s=$(date +%s.%N)
  "$1" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") \
    --tripinfo-output /tmp/cy07_$2_$3.ti.xml --statistic-output /tmp/cy07_$2_$3.st.xml --no-step-log true > /tmp/cy07_$2_$3.log 2>&1
  rc=$?
  e=$(date +%s.%N)
  [ $rc -eq 0 ] || echo "EXIT $rc $1 $2" >&2
  echo "$e - $s" | bc
}
med() { sort -n | sed -n 2p; }
for c in $CASES; do
  o=$(for i in 1 2 3; do t "$OURS" "$c" ours; done | med)
  p=$(for i in 1 2 3; do t "$PRI" "$c" cpp; done | med)
  echo "TIMING $c ours_$PROF=${o}s pristine=${p}s ratio=$(echo "scale=2; $o / $p" | bc)"
  echo "TIMING $c ours tripinfo=$(grep -c '<tripinfo ' /tmp/cy07_${c}_ours.ti.xml) $(grep -o 'teleports total="[0-9]*" jam="[0-9]*" yield="[0-9]*"' /tmp/cy07_${c}_ours.st.xml)"
done
