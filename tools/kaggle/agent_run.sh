#!/usr/bin/env bash
# sw02 scratch: UB check (foeConflictIndex >= myConflicts.size()) on instrumented official C++. Removed before merge.
O=/tmp/sw02; mkdir -p $O
BIN=$(bash tools/gcloud/vm_cpp_trace.sh sw02 tools/kaggle/sw02_ub.patch)
echo "bin=$BIN"
for c in city_seattle_downtown_hills city_seattle_first_hill; do
  C=tools/oracle/cases/$c
  timeout 900 "$BIN" -n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output $O/$c.ti.xml --fcd-output $O/$c.fcd.xml --statistic-output $O/$c.stat.xml --no-step-log true $(cat $C/sim.args) > $O/$c.log 2>&1
  echo "$c: cpp rc=$? UBOOB=$(grep -c '^UBOOB' $O/$c.log) trips=$(grep -c '<tripinfo ' $O/$c.ti.xml) refTrips=$(grep -c '<tripinfo ' $C/ref.tripinfo.xml)"
  grep -m3 '^UBOOB' $O/$c.log
done
diff <(grep '<tripinfo ' $O/city_seattle_first_hill.ti.xml) <(grep '<tripinfo ' tools/oracle/cases/city_seattle_first_hill/ref.tripinfo.xml) | head -5
