#!/bin/bash
# Regenerates the C++ reference outputs (tripinfo, fcd, statistics) for each sim case with the exact flags
# run_all.sh uses: cases/<case>/sim.args is read here too, so the reference and the harness cannot disagree.
# Flags per case:
#   junction, roundabout, signal, real   none
#   real_dense   --end 125                              (REPORT.md: the 120 s horizon split id sets)
#   stress       --end 200
#   reroute      --end 200 --additional-files tools/oracle/cases/reroute/reroute.add.xml
#   vtypes       --end 120 --additional-files tools/oracle/cases/vtypes/vtypes.add.xml
#                (tripinfo only, like its committed reference: no fcd/stat files are written)
#   sublanes     --end 120 --lateral-resolution 0.8
#   lanechange   --end 120
#   lefthand     --end 120   (reference net built with netconvert --lefthand; see its CASE.md)
#   reroute_alt  --end 200 --additional-files tools/oracle/cases/reroute_alt/closing.add.xml
#   depart       15 variants via cases/depart/run_ref.sh (--seed 42 --end 120), into cases/depart/ref/
# Only outputs the case already compares are written (fcd/stat appear only if the committed ref has them).
# Usage: bash tools/oracle/make_sim_refs.sh [case ...]     (default: all cases above). Overwrites references!
# DEST=/c/... writes <DEST>/<case>/ instead, to check committed references without touching them.
set -u
cd "$(dirname "$0")/../.." || exit 1
ROOT=$(pwd)
W=/mnt/c${ROOT#/c}
LOG=/c/Users/karti/AppData/Local/Temp/opencode
CASES=${*:-junction roundabout signal real real_dense stress reroute vtypes depart sublanes lanechange lefthand reroute_alt}
wp() { echo "/mnt/c${1#/c}"; }

for c in $CASES; do
  src=tools/oracle/cases/$c
  if [ "$c" = depart ]; then
    if [ -n "${DEST:-}" ]; then echo "depart: DEST not supported (run_ref.sh writes ref/)"; continue; fi
    wsl -e bash -lc "cd $W/$src && bash run_ref.sh" >"$LOG/ref_depart.log" 2>&1
    echo "depart: $(grep -c 'ref_exit=' "$LOG/ref_depart.log") variants"
    continue
  fi
  o=$ROOT/$src
  [ -n "${DEST:-}" ] && o=$DEST/$c
  mkdir -p "$o"
  outs="--tripinfo-output $(wp "$o")/ref.tripinfo.xml"
  [ -f "$src/ref.fcd.xml" ] && outs="$outs --fcd-output $(wp "$o")/ref.fcd.xml"
  [ -f "$src/ref.stat.xml" ] && outs="$outs --statistic-output $(wp "$o")/ref.stat.xml"
  flags=$(cat "$src/sim.args" 2>/dev/null)
  wsl -e bash -lc "cd $W && SUMO_HOME=\$HOME/sumo-src \$HOME/sumo-src/bin/sumo -n $src/ref_simple.net.xml -r $src/routes.rou $flags $outs --no-step-log true" >"$LOG/ref_$c.log" 2>&1
  echo "$c: exit $?  flags: ${flags:-none}  $(ls "$o"/ref.*.xml 2>/dev/null | wc -l) ref files"
done
