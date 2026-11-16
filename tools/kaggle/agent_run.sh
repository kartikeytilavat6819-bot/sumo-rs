#!/bin/bash
# TEMP ev01 measurement: remove before final commit
OUT=/tmp/ev01_out
mkdir -p $OUT
D=tools/oracle/cases/ev
/tmp/target/debug/sumo -n $D/ref_simple.net.xml -r $D/routes.rou --end 120 \
  --additional-files $D/chargingstation.add.xml \
  --tripinfo-output $OUT/tripinfo.xml --battery-output $OUT/battery.xml --fcd-output $OUT/fcd.xml \
  --chargingstations-output $OUT/cs.xml > $OUT/log.txt 2>&1
echo "exit=$?"
grep -i 'panick\|error' $OUT/log.txt | head -5
echo "--- ours tripinfo devices:"
grep -o 'devices="[^"]*"' $OUT/tripinfo.xml
echo "--- ref tripinfo devices:"
grep -o 'devices="[^"]*"' $D/ref.tripinfo.xml
echo "--- ours battery.xml bytes: $(wc -c < $OUT/battery.xml)"
python3 $D/compare_ev.py $D/ref.battery.xml $OUT/battery.xml $D/ref.chargingstations.xml $OUT/cs.xml
