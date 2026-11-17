#!/bin/bash
# temporary (sl05): routes_forms edge-speed diagnostics
cd "$(dirname "$0")/../.." || exit 1
export STATE_OUT=/tmp/sl05
bash tools/oracle/run_state.sh routes_forms >/tmp/sl05_rs.txt 2>&1
D=/tmp/sl05/routes_forms/sim
for f in $D.ours.state.xml $D.pristine.state.xml; do
  echo "== $f"
  grep -h 'routingengine' $f | cut -c1-200
  grep -h '<edge id="-103224#0"' $f | sed 's/pastSpeed="\([^"]*\)"/PAST \1/' | cut -c1-2500
  grep -h '<edge id="-103224#0"' $f | sed 's/.*pastSpeed="\([^"]*\)".*/\1/' | tr ' ' '\n' | awk '{s+=$1;n++} END{printf "n=%d mean=%.6f\n", n, s/n}'
done
grep -h 'edge id' $D.ours.state.xml | wc -l
grep -h 'edge id' $D.pristine.state.xml | wc -l
echo "AGENT RUN DONE"
