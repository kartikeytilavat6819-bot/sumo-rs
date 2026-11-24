#!/usr/bin/env bash
# oc121 (temporary): run the real duarouter oracle for meta_write_config and show its diff + both configs
c=duarouter_itt_duarouter_meta_write_config
BIN=${CARGO_TARGET_DIR}/debug/duarouter OUT=/tmp/oc121o bash tools/oracle/run_duarouter.sh $c 2>&1 | tail -n 3
s=/tmp/oc121o/$c
echo "-- diff"; cat $s/config.cfg.diff | cut -c1-300
for side in ref cand; do echo "-- $side"; grep -v 'generated on' $s/$side/config.cfg | sed -n 6,9p | cut -c1-300; done
echo "AGENT RUN DONE"
