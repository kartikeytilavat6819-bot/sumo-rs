#!/bin/bash
# TEMP ev01 precision-6 check: remove before final commit
D=tools/oracle/cases/ev
M=tools/oracle/cases/ev_mmpevem
OUT=/tmp/ev6
mkdir -p $OUT
run() {
  /tmp/target/debug/sumo -n $D/ref_simple.net.xml -r "$1" --end 120 --additional-files $D/chargingstation.add.xml \
    --precision 6 --battery-output "$2.battery.xml" --tripinfo-output "$2.tripinfo.xml" \
    --emission-output "$2.emission.xml" --chargingstations-output "$2.chargingstations.xml" \
    --fcd-output "$2.fcd.xml" > "$2.log" 2>&1
  echo "$2 exit=$?"
  grep -i 'panick' "$2.log" | head -3
}
run $D/routes.rou $OUT/ours_energy
python3 $D/compare_precision.py $D/ref_energy $OUT/ours_energy
run $M/routes.rou $OUT/ours_mmpevem
python3 $D/compare_precision.py $M/ref $OUT/ours_mmpevem
