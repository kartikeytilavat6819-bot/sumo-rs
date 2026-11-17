#!/bin/bash
# temporary measurement script: full panic text + first diff for the two failing new cases
set -u
cd "$(dirname "$0")/../.." || exit 1
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
for c in bluelight routeprobe; do
    echo "=== $c  flags: $(cat tools/oracle/cases/$c/sim.args)"
    "$B" -n tools/oracle/cases/$c/ref_simple.net.xml -r tools/oracle/cases/$c/routes.rou \
        --tripinfo-output /tmp/$c.ti.xml --fcd-output /tmp/$c.fcd.xml \
        --statistic-output /tmp/$c.stat.xml --no-step-log true $(cat tools/oracle/cases/$c/sim.args) 2>&1 | tail -6
    echo "--- diff tripinfo"
    python3 tools/oracle/compare_sim.py --tripinfo tools/oracle/cases/$c/ref.tripinfo.xml /tmp/$c.ti.xml 2>&1 | head -3
done
echo "=== collision rows actually run"
for v in none remove teleport; do
    "$B" -n tools/oracle/cases/collision/ref_simple.net.xml -r tools/oracle/cases/collision/routes.rou \
        --tripinfo-output /tmp/col_$v.ti.xml --fcd-output /tmp/col_$v.fcd.xml \
        --statistic-output /tmp/col_$v.stat.xml --no-step-log true $(cat tools/oracle/cases/collision/sim_$v.args) 2>&1 | tail -2
    echo -n "collision/$v tripinfo: "
    python3 tools/oracle/compare_sim.py --tripinfo tools/oracle/cases/collision/ref_$v.tripinfo.xml /tmp/col_$v.ti.xml >/dev/null 2>&1 && echo PASS || echo FAIL
    echo -n "collision/$v fcd: "
    python3 tools/oracle/compare_sim.py --fcd tools/oracle/cases/collision/ref_$v.fcd.xml /tmp/col_$v.fcd.xml >/dev/null 2>&1 && echo PASS || echo FAIL
done
