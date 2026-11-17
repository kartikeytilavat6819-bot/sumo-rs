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
#   closed_edge  --end 200 --additional-files tools/oracle/cases/closed_edge/closed_lane.add.xml
#   calibrator_multi  --end 200 --additional-files tools/oracle/cases/calibrator_multi/calibrator_multi.add.xml (tripinfo only)
#   depart       15 variants via cases/depart/run_ref.sh (--seed 42 --end 120), into cases/depart/ref/
#   device_toc, device_ssm, device_stationfinder, device_driverstate, device_glosa, device_friction   (--end per sim.args)
# Only outputs the case already compares are written (fcd/stat appear only if the committed ref has them).
# A case with extra_outputs.txt (lines "<option> <suffix>") writes fcd+stat and each <suffix> as ref.<suffix>.xml.
# Usage: bash tools/oracle/make_sim_refs.sh [case ...]     (default: all cases above). Overwrites references!
# DEST=/c/... writes <DEST>/<case>/ instead, to check committed references without touching them.
set -u
cd "$(dirname "$0")/../.." || exit 1
ROOT=$(pwd)
W=/mnt/c${ROOT#/c}
LOG=/c/Users/karti/AppData/Local/Temp/opencode
CASES=${*:-junction roundabout signal real real_dense stress reroute vtypes depart sublanes lanechange lefthand reroute_alt closed_edge calibrator_multi device_toc device_ssm device_stationfinder device_driverstate device_glosa device_friction}
wp() { echo "/mnt/c${1#/c}"; }
# A C++ run counts only when it exits 0 without "Interrupt signal" (WSL signals end a run early and still exit 0)
# and its outputs pass check_ref_complete.py. Retries once on a signal exit; any failure stops the script.
run_ref() {  # run_ref <log> <wsl-cmd> <end or empty> <outdir> <tag>
  local log=$1 cmd=$2 end=$3 od=$4 tag=$5 rc=0 try
  for try in 1 2; do
    wsl -e bash -lc "$cmd" </dev/null >"$log" 2>&1; rc=$?
    grep -q 'Interrupt signal' "$log" && rc=130
    [ $rc -eq 0 ] && break
    [ $rc -lt 128 ] && break
    echo "  $tag: signal/interrupt (exit $rc), retry $try" >&2
  done
  [ $rc -eq 0 ] || { echo "FAILED $tag: C++ exit $rc (see $log)" >&2; exit 1; }
  local chk=(--tripinfo "$od/$tag.tripinfo.xml")
  [ -f "$od/$tag.fcd.xml" ] && chk+=(--fcd "$od/$tag.fcd.xml")
  [ -f "$od/$tag.stat.xml" ] && chk+=(--stat "$od/$tag.stat.xml")
  for x in "$od/$tag".*.xml; do
    case $x in *.tripinfo.xml|*.fcd.xml|*.stat.xml) ;; *) [ -f "$x" ] && chk+=(--xml "$x");; esac
  done
  python tools/oracle/check_ref_complete.py ${end:+--end "$end"} "${chk[@]}" || { echo "FAILED $tag: incomplete output" >&2; exit 1; }
}

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
  # NEW_CASE marker: a brand-new case dir has no committed refs yet, so write every output and every sim_<v>.args variant
  want() { [ -f "$1" ] || [ -f "$(dirname "$1")/NEW_CASE" ]; }
  outs="--tripinfo-output $(wp "$o")/ref.tripinfo.xml"
  want "$src/ref.fcd.xml" && outs="$outs --fcd-output $(wp "$o")/ref.fcd.xml"
  want "$src/ref.stat.xml" && outs="$outs --statistic-output $(wp "$o")/ref.stat.xml"
  [ -f "$src/extra_outputs.txt" ] && while read -r opt suf; do outs="$outs $opt $(wp "$o")/ref.$suf.xml"; done < "$src/extra_outputs.txt"
  flags=$(cat "$src/sim.args" 2>/dev/null)
  run_ref "$LOG/ref_$c.log" "cd $W && SUMO_HOME=\$HOME/sumo-pristine/lib/python3.12/site-packages/sumo setsid --wait \${SUMO_ORACLE_BIN:-\$HOME/sumo-pristine/bin}/sumo -n $src/ref_simple.net.xml -r $src/routes.rou $flags $outs --no-step-log true" "$(echo "$flags" | sed -n 's/.*--end \([0-9.]*\).*/\1/p')" "$o" ref
  echo "$c: exit $?  flags: ${flags:-none}  $(ls "$o"/ref.*.xml 2>/dev/null | wc -l) ref files"
  # Variants: run_all pairs ref_<v>.tripinfo.xml with sim_<v>.args, so each variant reference must be
  # regenerable the same way. Without this loop a variant reference can only be reproduced by hand.
  if [ -f "$src/NEW_CASE" ]; then
    vlist=$(for f in "$src"/sim_*.args; do [ -f "$f" ] || continue; b=$(basename "$f" .args); echo "${b#sim_}"; done)
  else
    vlist=$(for vf in "$src"/ref_*.tripinfo.xml; do [ -f "$vf" ] || continue; b=$(basename "$vf" .tripinfo.xml); echo "${b#ref_}"; done)
  fi
  for v in $vlist; do
    vflags=$(cat "$src/sim_$v.args" 2>/dev/null)
    vouts="--tripinfo-output $(wp "$o")/ref_$v.tripinfo.xml"
    # run_all always passes --fcd-output, which adds fcd_<id> to every device list: write a scratch fcd when none is wanted
    if want "$src/ref_$v.fcd.xml"; then vouts="$vouts --fcd-output $(wp "$o")/ref_$v.fcd.xml"; else vouts="$vouts --fcd-output /tmp/fcd_$v.xml"; fi
    want "$src/ref_$v.stat.xml" && vouts="$vouts --statistic-output $(wp "$o")/ref_$v.stat.xml"
    [ -f "$src/extra_outputs.txt" ] && while read -r opt suf; do vouts="$vouts $opt $(wp "$o")/ref_$v.$suf.xml"; done < "$src/extra_outputs.txt"
    run_ref "$LOG/ref_${c}_$v.log" "cd $W && SUMO_HOME=\$HOME/sumo-pristine/lib/python3.12/site-packages/sumo setsid --wait \${SUMO_ORACLE_BIN:-\$HOME/sumo-pristine/bin}/sumo -n $src/ref_simple.net.xml -r $src/routes.rou $vflags $vouts --no-step-log true" "$(echo "$vflags" | sed -n 's/.*--end \([0-9.]*\).*/\1/p')" "$o" "ref_$v"
    echo "  $c/$v: exit $?  flags: ${vflags:-none}"
  done
done
