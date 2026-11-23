#!/usr/bin/env bash
# oc98: turn the 5 tt_rail_rerouter_overtaking_follower_* rows into real sim rows. NOT run automatically.
# Run from the repo root. The importer dropped the parent's `-a input_additional.add.xml,input_routes.rou.xml`
# (overtakingReroute rerouter + vTypes + routes), so the harness invocation aborts on both binaries.
# Probe result (agent_run.sh "== REAL ..."): with the parent files restored the official and our binaries give
# identical tripinfo/stopinfo/vehroute on all five, so after this step the rows are genuine faithfulness tests.
# Afterwards regenerate the C++ references from the official binary, e.g.:
#   bash tools/oracle/make_sim_refs.sh tt_rail_rerouter_overtaking_follower_delay_low ... (all five)
set -eu
PK=tools/kaggle/oc98_overtaking
for c in tt_rail_rerouter_overtaking_follower_arrival_delay_low tt_rail_rerouter_overtaking_follower_delay_high \
         tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_default tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_high \
         tt_rail_rerouter_overtaking_follower_delay_low; do
  d=tools/oracle/cases/$c
  cp "$PK/input_additional.add.xml" "$PK/input_routes.rou.xml" "$d/"
  sed -E "s#-a [^ ]+#-a $d/input_additional.add.xml,$d/input_routes.rou.xml,$d/input_additional2.add.xml#" "$d/sim.args" > "$d/sim.args.new"
  mv "$d/sim.args.new" "$d/sim.args"
  rm -f "$d/EXPECT_EXIT" "$d/ref.errors" "$d/ref.output"
  echo "prepared $c: $(cat "$d/sim.args" | cut -c1-200)"
done
