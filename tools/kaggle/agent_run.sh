#!/bin/bash
# TEMPORARY (ft01): measure the calibrator sim diff. Removed before the final commit.
cd "$(dirname "$0")/../.." || exit 1
B=$CARGO_TARGET_DIR/debug
C=tools/oracle/cases/calibrator
O=/tmp/ft01_cal
mkdir -p $O
$B/sumo -n $C/ref_simple.net.xml -r $C/routes.rou --additional-files $C/calibrator.add.xml --end 200 \
    --tripinfo-output $O/ti.xml --fcd-output $O/fcd.xml --statistic-output $O/stat.xml --no-step-log true > $O/ours.log 2>&1
echo "ours exit=$?"
tail -5 $O/ours.log
python3 tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $O/ti.xml 2>&1 | head -12
python3 tools/oracle/compare_sim.py --fcd $C/ref.fcd.xml $O/fcd.xml 2>&1 | head -12
echo "ref tripinfo count: $(grep -c '<tripinfo ' $C/ref.tripinfo.xml)  ours: $(grep -c '<tripinfo ' $O/ti.xml)"
echo "ref fcd timesteps: $(grep -c '<timestep' $C/ref.fcd.xml)  ours: $(grep -c '<timestep' $O/fcd.xml)"
grep -m3 -E 'vehicle id' $C/ref.fcd.xml | head -3
grep -m3 -E 'vehicle id' $O/fcd.xml | head -3
