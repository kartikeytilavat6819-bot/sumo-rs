#!/bin/bash
# Regenerates the C++ reference outputs (pristine SUMO 1.28.0, run inside WSL) for the public-transport cases:
#   <case>/ref_net_out/<variant>/   net.net.xml + the output files of each line of <case>/net_outputs.txt
#   busride                         ref_simple.net.xml (net.args), ref.tripinfo.xml, ref_sim_out/ (sim_outputs.txt)
# Usage: bash tools/oracle/make_pt_refs.sh [case ...]   default: city_zurich_tram city_karlsruhe_tram busride
# Overwrites references! Run from the repo (WSL: bash tools/oracle/make_pt_refs.sh). Every run must exit 0.
set -u
cd "$(dirname "$0")/../.." || exit 1
PRI=$HOME/sumo-pristine
export SUMO_HOME=$PRI/lib/python3.12/site-packages/sumo
T=/mnt/c/Users/karti/AppData/Local/Temp/opencode/ptrefs
mkdir -p "$T"
CASES=${*:-city_zurich_tram city_karlsruhe_tram busride}
fail() { echo "FAILED: $*" >&2; exit 1; }
for c in $CASES; do
  d=tools/oracle/cases/$c
  if [ -f "$d/net_outputs.txt" ]; then
    while read -r v args; do
      v=${v%$'\r'}; args=${args%$'\r'}
      [ -n "$v" ] || continue
      o=$T/$c.$v; rm -rf "$o"; mkdir -p "$o"
      # shellcheck disable=SC2086
      $PRI/bin/netconvert --osm-files "$d/input.osm" --simple-projection ${args//@OUT@/$o} \
          -o "$o/net.net.xml" > "$o/nc.log" 2>&1 < /dev/null || fail "$c $v: netconvert exit (see $o/nc.log)"
      rm -rf "$d/ref_net_out/$v"; mkdir -p "$d/ref_net_out/$v"
      cp "$o"/*.xml "$d/ref_net_out/$v/" || fail "$c $v: no outputs"
      echo "$c net-$v: $(ls "$d/ref_net_out/$v" | tr '\n' ' ')"
    done < "$d/net_outputs.txt"
  fi
  if [ "$c" = busride ]; then
    $PRI/bin/netconvert $(cat "$d/net.args") -o "$d/ref_simple.net.xml" > "$T/$c.nc.log" 2>&1 < /dev/null || fail "$c netconvert"
    o=$T/$c.sim; rm -rf "$o"; mkdir -p "$o"
    # shellcheck disable=SC2046
    $PRI/bin/sumo -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") \
        --tripinfo-output "$d/ref.tripinfo.xml" $(sed "s#@OUT@#$o#g" "$d/sim_outputs.txt" | tr -d '\r') \
        --no-step-log true > "$o/sumo.log" 2>&1 < /dev/null || fail "$c sumo exit (see $o/sumo.log)"
    grep -q 'Interrupt signal' "$o/sumo.log" && fail "$c: C++ interrupted, rerun"
    rm -rf "$d/ref_sim_out"; mkdir -p "$d/ref_sim_out"
    for f in $(sed "s#@OUT@/##g" "$d/sim_outputs.txt" | tr -d '\r' | tr ' ' '\n' | grep '\.xml$'); do
      cp "$o/$f" "$d/ref_sim_out/" || fail "$c: missing $f"
    done
    echo "$c sim: $(ls "$d/ref_sim_out" | tr '\n' ' ')"
  fi
done
