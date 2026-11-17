#!/bin/bash
# rl01 (temporary, removed before merge): release build parity + wall-clock timing.
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_BUILD_JOBS=8
REL=${CARGO_TARGET_DIR}-rel
echo "== release build ($REL)"
CARGO_TARGET_DIR=$REL cargo build --release -p sumo -p netconvert --message-format short > /tmp/rl01_rel_build.log 2>&1
rc=$?
echo "release build rc=$rc"
grep -aE ': error|^error|panicked' /tmp/rl01_rel_build.log | head -20
if [ $rc -ne 0 ]; then echo "RELEASE BUILD FAILED"; exit 0; fi
(cd "$REL/release" && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe)
echo "== release run_all"
PROFILE=release CARGO_TARGET_DIR=$REL SCORE_OUT=/tmp/rl01_score bash tools/oracle/run_all.sh 2>&1
echo "== release depart variants"
PROFILE=release CARGO_TARGET_DIR=$REL SCORE_OUT=/tmp/rl01_score bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -3
echo "== timing: wall s, --no-step-log, args as run_all; perf: $(command -v perf || echo none)"
for rep in 1 2 3; do
  for c in city_atlanta_motorway city_paris_dense city_milton_keynes city_la_interchange city_tokyo_lefthand; do
    d=tools/oracle/cases/$c
    for who in ours-rel pristine; do
      if [ $who = ours-rel ]; then bin=$REL/release/sumo; else bin=$SUMO_PRISTINE_BIN/sumo; fi
      o=/tmp/rl01_t_$who
      s=$(date +%s.%N)
      timeout 240 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
          --tripinfo-output "$o.ti.xml" --fcd-output "$o.fcd.xml" \
          --statistic-output "$o.stat.xml" --no-step-log true $(cat "$d/sim.args") >"$o.log" 2>&1
      rc=$?
      e=$(date +%s.%N)
      echo "TIME rep=$rep $c $who rc=$rc $(awk -v a=$s -v b=$e 'BEGIN{printf "%.2f", b-a}')"
    done
  done
done
echo "AGENT_RUN END"
