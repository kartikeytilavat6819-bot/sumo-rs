#!/usr/bin/env bash
# oc121 (temporary): run the FULL duarouter oracle like vm_gate_run.sh and show the meta_write_config diff
c=duarouter_itt_duarouter_meta_write_config
OUT=/tmp/dua_oc121 BIN=${CARGO_TARGET_DIR}/debug/duarouter timeout 1300 bash tools/oracle/run_duarouter.sh >/dev/null 2>&1
grep -c "	PASS	" /tmp/dua_oc121/results.tsv
grep -v "	PASS	" /tmp/dua_oc121/results.tsv | cut -f1,2,5 | head -n 5
s=/tmp/dua_oc121/$c
echo "-- diff"; cat $s/config.cfg.diff 2>/dev/null | cut -c1-300
for side in ref cand; do echo "-- $side"; grep -v 'generated on' $s/$side/config.cfg 2>/dev/null | sed -n 6,9p | cut -c1-300; done
echo "AGENT RUN DONE"
