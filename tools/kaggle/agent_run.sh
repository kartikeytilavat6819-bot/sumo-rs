#!/bin/bash
# TEMP ev01 precision-6 measurement: remove before final commit
D=tools/oracle/cases/ev
OUT=/tmp/ev6
mkdir -p $OUT
sed 's#Energy/unknown#MMPEVEM/unknown#' $D/routes.rou > $OUT/routes_mm.rou
grep -c MMPEVEM $OUT/routes_mm.rou
run() {
  "$1" -n $D/ref_simple.net.xml -r "$2" --end 120 --additional-files $D/chargingstation.add.xml \
    --precision 6 --battery-output "$3.battery.xml" --tripinfo-output "$3.tripinfo.xml" \
    --emission-output "$3.emission.xml" --chargingstations-output "$3.chargingstations.xml" \
    --fcd-output "$3.fcd.xml" > "$3.log" 2>&1
  echo "$3 exit=$?"
  grep -i 'panick' "$3.log" | head -3
}
for s in energy mmpevem; do
  R=$D/routes.rou
  [ $s = mmpevem ] && R=$OUT/routes_mm.rou
  run /tmp/target/debug/sumo $R $OUT/ours_$s
  run "$SUMO_PRISTINE_BIN/sumo" $R $OUT/ref_$s
  python3 $D/compare_precision.py $OUT/ref_$s $OUT/ours_$s
done
