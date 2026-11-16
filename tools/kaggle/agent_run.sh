#!/bin/bash
# TEMPORARY (removed before final commit): London full run vs regenerated reference.
cd tools/oracle/cases/city_london_lh || exit 1
${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo -n ref_simple.net.xml -r routes.rou --end 600 --no-step-log true --tripinfo-output /tmp/l.tripinfo.xml --fcd-output /tmp/l.fcd.xml --statistic-output /tmp/l.stat.xml > /tmp/l.out 2>&1
echo "sumo_exit=$?"
cd ../..
python3 compare_sim.py --fcd cases/city_london_lh/ref.fcd.xml /tmp/l.fcd.xml 2>&1 | head -3
python3 compare_sim.py --tripinfo cases/city_london_lh/ref.tripinfo.xml /tmp/l.tripinfo.xml 2>&1 | head -3
grep -E '<vehicles ' /tmp/l.stat.xml
