#!/usr/bin/env bash
# oc121 (temporary): regenerate route_probe refs with the OFFICIAL binary, exact harness command
OUT=/tmp/oc121; rm -rf $OUT; mkdir -p "$OUT/pack"
for d in tools/oracle/cases/tt_output_route_probe_*; do
  c=$(basename $d); p=$OUT/pack/$c; mkdir -p $p
  rm -f $d/detector.xml
  timeout 300 ${SUMO_PRISTINE_BIN}/sumo -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
    --tripinfo-output $p/ref.tripinfo.xml --fcd-output $p/ref.fcd.xml --statistic-output $p/ref.stat.xml \
    --no-step-log true $(cat $d/sim.args) >$OUT/$c.out 2>$OUT/$c.err; echo "== $c rc=$?"
  head -n 3 $OUT/$c.err | cut -c1-200
  [ -f $d/detector.xml ] && mv $d/detector.xml $p/ref.probe.xml
  rm -f $d/NUL
  ls -l $p | awk '{print "   "$5" "$9}'
done
(cd $OUT/pack && tar czf ../pack.tgz .) ; ls -l $OUT/pack.tgz
echo "BEGIN64"; base64 -w 180 $OUT/pack.tgz; echo "END64"
echo "AGENT RUN DONE"
