#!/bin/bash
# TEMPORARY agent run (state body diff). Removed before the final commit.
set -u
OUT=/tmp/rt
d=tools/oracle/cases/city_bengaluru_blr
base="-n $d/ref_simple.net.xml -r $d/routes.rou --end 600 --no-step-log true --no-warnings true"
o=$OUT/bng; mkdir -p $o
"$SUMO_PRISTINE_BIN/sumo" $base --save-state.times 300 --save-state.files "$o/cpp.xml" >/dev/null 2>&1
/tmp/target/debug/sumo $base --save-state.times 300 --save-state.files "$o/ours.xml" >/dev/null 2>&1
echo "== ours head"; head -12 $o/ours.xml | cut -c1-200
echo "== cpp head"; head -12 $o/cpp.xml | cut -c1-200
echo "== unified diff (body, first 80 lines)"
diff <(grep -v '^\s*$' $o/cpp.xml) <(grep -v '^\s*$' $o/ours.xml) | head -80 | cut -c1-220
echo "== element names only: ours vs cpp"
diff <(grep -o '<[a-zA-Z_]*' $o/cpp.xml | sort | uniq -c) <(grep -o '<[a-zA-Z_]*' $o/ours.xml | sort | uniq -c)
echo "AGENT RUN DONE"
