#!/bin/bash
# oc116 probe (temporary, not for merge): runs the 4 polyconvert itt targets, the duarouter/jtrrouter oracles and the
# depart variants; prints each failing row's first difference. Writes only under /tmp/oc116. Repo root on the gate VM.
OUT=/tmp/oc116
rm -rf "$OUT"; mkdir -p "$OUT"
cd "$(dirname "$0")/../.." || exit 1
export SUMO_HOME=${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug
echo "== polyconvert itt targets"
for c in OSM_buildings_from_relation OSM_inner_shape OSM_water_incomplete XML_lanepos; do
  d=$OUT/pc/$c; mkdir -p $d/ref $d/cand
  a=$(cat tools/oracle/cases/polyconvert_itt_polyconvert_import_$c/args | tr -d '\r')
  "$SUMO_PRISTINE_BIN/polyconvert" ${a//@DIR@/$d/ref} >$d/ref.log 2>&1; rr=$?
  "$BIN/polyconvert" ${a//@DIR@/$d/cand} >$d/cand.log 2>&1; cr=$?
  echo "-- $c rc ref=$rr cand=$cr $(grep -m1 -A1 'panicked at' $d/cand.log | tr '\n' ' ' | cut -c1-200)"
  diff <(grep -v 'generated on' $d/ref/routes.rou.xml 2>/dev/null) <(grep -v 'generated on' $d/cand/routes.rou.xml 2>/dev/null) | head -8 | cut -c1-200
done
for t in duarouter jtrrouter; do
  echo "== $t oracle"
  OUT=$OUT/$t bash tools/oracle/run_$t.sh >$OUT/$t.stdout 2>&1
  awk -F'\t' 'NR>1 && $2!="PASS" {print $1"\t"$2"\t"$5}' $OUT/$t/results.tsv | cut -c1-240 | head -20
  awk -F'\t' 'NR>1 && $2=="FAIL" {print $1}' $OUT/$t/results.tsv | head -4 | while read c; do
    echo "-- $t $c"; diff -r $OUT/$t/$c/ref $OUT/$t/$c/cand | head -8 | cut -c1-200
  done
done
echo "== depart"
SCORE_OUT=$OUT/score bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | grep -v PASS | tail -20 | cut -c1-200
