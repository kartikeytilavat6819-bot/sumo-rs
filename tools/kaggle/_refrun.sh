#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc56 || exit 1
for c in $(ls -d tools/oracle/cases/tt_rail_routing_* | xargs -n1 basename); do
  bash tools/oracle/make_sim_refs.sh "$c" >/tmp/refrun.log 2>&1 || echo "REF_FAIL $c: $(grep -m1 '^Error:' /c/Users/karti/AppData/Local/Temp/opencode/ref_$c.log)"
done
