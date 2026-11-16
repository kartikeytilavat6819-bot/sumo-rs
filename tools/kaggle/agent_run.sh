#!/bin/bash
# TEMPORARY (removed before final commit): London run; ISLR branch values for vehicle 146 at t=303..304.
cd tools/oracle/cases/city_london_lh || exit 1
${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo -n ref_simple.net.xml -r routes.rou --end 600 --no-step-log true --tripinfo-output /tmp/l.tripinfo.xml --fcd-output /tmp/l.fcd.xml --statistic-output /tmp/l.stat.xml > /tmp/l.out 2>&1
echo "sumo_exit=$?"
grep -E '^ISLR ' /tmp/l.out | cut -c1-260 | head -20
grep -E '^ISL t=' /tmp/l.out | cut -c1-200 | head -6
cd ../..
python3 compare_sim.py --fcd cases/city_london_lh/ref.fcd.xml /tmp/l.fcd.xml 2>&1 | head -2
