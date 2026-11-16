#!/bin/bash
# TRACE ONLY (removed before final commit): London vehicle 146, t=302..304, our side.
cd tools/oracle/cases/city_london_lh || exit 1
/tmp/target/debug/sumo -n ref_simple.net.xml -r routes.rou --end 305 --no-step-log true --fcd-output /tmp/l146.fcd.xml > /tmp/l146.out 2>&1
echo "sumo_exit=$?"
grep -E '^(GA0|GV|GC|CAND) ' /tmp/l146.out | grep -E 't=(302|303|304)000|^CAND' | head -150
