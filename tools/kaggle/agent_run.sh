#!/usr/bin/env bash
# oc121 (temporary): real duarouter oracle with the DEFAULT OUT (as in the full gate); show env, diff, configs
c=duarouter_itt_duarouter_meta_write_config
echo "TMPDIR=${TMPDIR:-<unset>} PWD=$PWD SUMO_HOME=${SUMO_HOME:-<unset>}"
BIN=${CARGO_TARGET_DIR}/debug/duarouter bash tools/oracle/run_duarouter.sh $c 2>&1 | tail -n 3
s=${TMPDIR:-/tmp}/duarouter_oracle/$c
echo "-- diff"; cat $s/config.cfg.diff 2>/dev/null | cut -c1-300
for side in ref cand; do echo "-- $side"; grep -v 'generated on' $s/$side/config.cfg | sed -n 6,9p | cut -c1-300; done
echo "AGENT RUN DONE"
