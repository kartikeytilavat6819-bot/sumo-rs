#!/bin/bash
cd "$(cd "$(dirname "$0")" && pwd)" || exit 1
# Reproduce run_netopts_new.sh's shell setup without running the rows.
B=tools/oracle
NAMES=(); INS=(); OPTS=()
row() { NAMES+=("$1"); INS+=("$2"); OPTS+=("$3"); }
source "$B/import_texttest_net.rows"
echo "IMPORTED_NAMES=${#NAMES[@]}"
MINE=0
for n in "${NAMES[@]}"; do
  case "$n" in
    itt_nc_netconvert_import_dlr_navteq_*|itt_nc_netconvert_import_matsim_*) MINE=$((MINE+1));;
  esac
done
echo "MINE=$MINE"
