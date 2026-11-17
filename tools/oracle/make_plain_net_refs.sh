#!/bin/bash
# Plain-XML net reference for cases with no input.osm and no net.args (C++ 1.28.0, run inside WSL).
# 1. pristine netconvert -s ref_simple.net.xml -p plain/net dumps the net as plain nodes/edges/connections/types/tllogics
# 2. net_plain.args lists those files (repo-relative, for run_all.sh)
# 3. pristine netconvert on exactly those files writes ref_plain.net.xml (the oracle for our plain import)
# Usage (WSL): bash tools/oracle/make_plain_net_refs.sh <case> [<case> ...]
set -eu
cd "$(dirname "$0")/../.."
P=$HOME/sumo-pristine/bin/netconvert
export SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo
for C in "$@"; do
  D=tools/oracle/cases/$C
  mkdir -p "$D/plain"
  "$P" -s "$D/ref_simple.net.xml" -p "$D/plain/net" --plain-output.lanes >/dev/null
  A="--node-files $D/plain/net.nod.xml --edge-files $D/plain/net.edg.xml --connection-files $D/plain/net.con.xml"
  [ -f "$D/plain/net.typ.xml" ] && A="$A --type-files $D/plain/net.typ.xml"
  [ -f "$D/plain/net.tll.xml" ] && A="$A --tllogic-files $D/plain/net.tll.xml"
  rm -f "$D/plain/net.netccfg"
  printf '%s\n' "$A" > "$D/net_plain.args"
  "$P" $A -o "$D/ref_plain.net.xml" >/dev/null
  echo "$C: $(cat "$D/net_plain.args")"
  ls -la "$D/ref_plain.net.xml"
done
