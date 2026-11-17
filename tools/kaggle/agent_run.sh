#!/bin/bash
# cv01 temporary: detail for the new output/device cases from the gate's run_all logs (removed before final commit)
R=$(ls -td ${SCORE_OUT:-/tmp/score}/run-* | head -1)
echo "RUNDIR $R"
for c in out_detectors out_ssm_measures out_rerouting_weights out_meandata out_collision out_tls out_bt out_railsignal out_basic dev_bluelight_sublane out_person out_probes; do
  for f in "$R"/$c.*.log; do
    [ -f "$f" ] || continue
    case $f in *.sumo.log) grep -m3 -E 'panicked|Error' "$f" | sed "s#^#$(basename $f): #" | cut -c1-300; continue;; esac
    if grep -qvE '^(MATCH|OK|PASS|COMPLETE)' "$f" && ! grep -qiE '^(match|identical|PASS)' "$f"; then :; fi
    echo "== $(basename $f): $(head -c 600 "$f" | tr '\n' ' ')"
  done
done
