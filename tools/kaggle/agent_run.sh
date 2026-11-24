#!/usr/bin/env bash
# oc121 (temporary): meta_write_config with the gate's cached reference binary vs pristine
c=duarouter_itt_duarouter_meta_write_config
ls -d /home/gate/pcache/bin* 2>/dev/null | head -n 4
for rb in "$SUMO_PRISTINE_BIN/duarouter" $(ls -d /home/gate/pcache/bin*/duarouter 2>/dev/null | head -n 2); do
  echo "== REF_BIN=$rb ($("$rb" --version 2>&1 | head -n 1 | cut -c1-80))"
  OUT=/tmp/oc121p REF_BIN=$rb BIN=${CARGO_TARGET_DIR}/debug/duarouter bash tools/oracle/run_duarouter.sh $c 2>&1 | head -n 1 | cut -c1-200
  cat /tmp/oc121p/$c/config.cfg.diff 2>/dev/null | head -n 12 | cut -c1-260
done
echo "AGENT RUN DONE"
