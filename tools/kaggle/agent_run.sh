#!/bin/bash
# cv02: first divergence per new cv_* case (removed before the final commit)
BIN=$CARGO_TARGET_DIR/debug
O=/tmp/cv02; rm -rf $O; mkdir -p $O
for dir in tools/oracle/cases/cv_*/; do
  c=$(basename "$dir")
  for ref in "$dir"ref*.tripinfo.xml; do
    tag=$(basename "$ref" .tripinfo.xml)
    if [ "$tag" = ref ]; then A=$(cat "$dir/sim.args"); else A=$(cat "$dir/sim_${tag#ref_}.args"); fi
    timeout 300 "$BIN/sumo" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" --tripinfo-output $O/$c.$tag.ti.xml \
      --fcd-output $O/$c.$tag.fcd.xml --statistic-output $O/$c.$tag.stat.xml --no-step-log true $A >$O/$c.$tag.log 2>&1
    echo "== $c/$tag rc=$?"
    grep -m2 -E 'panicked|Error' $O/$c.$tag.log
    python3 tools/oracle/compare_sim.py --tripinfo "$ref" $O/$c.$tag.ti.xml 2>&1 | head -6
    [ -f "$dir/$tag.fcd.xml" ] && python3 tools/oracle/compare_sim.py --fcd "$dir/$tag.fcd.xml" $O/$c.$tag.fcd.xml 2>&1 | head -6
  done
done
