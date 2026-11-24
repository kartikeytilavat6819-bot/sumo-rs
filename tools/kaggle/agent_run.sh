#!/usr/bin/env bash
# oc121 (temporary): duarouter meta_write_config, official vs ours, exact oracle command
c=duarouter_itt_duarouter_meta_write_config; cdir=tools/oracle/cases/$c; OUT=/tmp/oc121; rm -rf $OUT
net=$(cat $cdir/net); args=$(cat $cdir/args)
for side in ref cand; do
  mkdir -p $OUT/$side; a=${args//@CASE@/$cdir}; a=${a//@DIR@/$OUT/$side}
  bin=$SUMO_PRISTINE_BIN/duarouter; [ $side = cand ] && bin=$CARGO_TARGET_DIR/debug/duarouter
  $bin -n "$net" $a --no-step-log true >$OUT/$side.log 2>&1; echo "== $side rc=$? net='$net'"
  grep -v 'generated on' $OUT/$side/config.cfg | sed -n 5,13p | cut -c1-260
done
echo "AGENT RUN DONE"
