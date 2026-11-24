#!/usr/bin/env bash
# oc121 (temporary): regenerate route_probe refs with the OFFICIAL binary, exact harness command; print small files
OUT=/tmp/oc121; rm -rf $OUT; mkdir -p "$OUT"
strip() { awk '/<!--/{c=1} !c{print} /-->/{c=0}' "$1" | grep -v '^$'; }
for d in tools/oracle/cases/tt_output_route_probe_*; do
  c=$(basename $d); p=$OUT/$c; mkdir -p $p
  rm -f $d/detector.xml
  timeout 300 ${SUMO_PRISTINE_BIN}/sumo -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
    --tripinfo-output $p/ref.tripinfo.xml --fcd-output $p/ref.fcd.xml --statistic-output $p/ref.stat.xml \
    --no-step-log true $(cat $d/sim.args) >$OUT/$c.out 2>$OUT/$c.err; echo "== $c rc=$? stderr=$(wc -c <$OUT/$c.err) fcd=$(wc -l <$p/ref.fcd.xml) stat=$(wc -l <$p/ref.stat.xml)"
  [ -f $d/detector.xml ] && mv $d/detector.xml $p/ref.probe.xml
  rm -f $d/NUL
  echo "-- tripinfo"; strip $p/ref.tripinfo.xml
  [ -f $p/ref.probe.xml ] && { echo "-- probe"; strip $p/ref.probe.xml; }
done
echo "AGENT RUN DONE"
