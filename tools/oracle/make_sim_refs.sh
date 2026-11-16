#!/bin/bash
# Generate and commit the C++ reference outputs (tripinfo, fcd, statistics) for every
# oracle case, so verification is reproducible on any machine.
set -u
cd /c/Users/karti/sumo-rs || exit 1
for c in junction roundabout signal; do
  D=/mnt/c/Users/karti/sumo-rs/tools/oracle/cases/$c
  wsl -e bash -lc "SUMO_HOME=\$HOME/sumo-src \$HOME/sumo-src/bin/sumo \
    -n $D/ref_simple.net.xml -r $D/routes.rou \
    --tripinfo-output $D/ref.tripinfo.xml --fcd-output $D/ref.fcd.xml \
    --statistic-output $D/ref.stat.xml --no-step-log true" >/tmp/ref_$c.log 2>&1
  echo "$c: exit $?  $(ls -la tools/oracle/cases/$c/ref.*.xml 2>/dev/null | wc -l) ref files"
done
