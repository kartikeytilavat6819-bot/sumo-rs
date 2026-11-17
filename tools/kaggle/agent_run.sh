#!/bin/bash
# temporary (sl05): meso load diagnostics
cd "$(dirname "$0")/../.." || exit 1
export STATE_OUT=/tmp/sl05
bash tools/oracle/run_state.sh meso_junction meso_city_plano >/tmp/sl05_rs.txt 2>&1
grep -E 'variants=|DIFF' /tmp/sl05_rs.txt | head -5
for p in meso_junction/sim:v3 meso_city_plano/sim:103; do
  D=/tmp/sl05/${p%%:*}; ID=${p##*:}
  echo "== $D id=$ID"
  grep -h "id=\"$ID\"" $D.ours.state.xml | cut -c1-600
  echo "-- load ours log"; head -c 800 $D.load_ours.log
  echo "-- load pristine log"; head -c 800 $D.load_pristine.log
  echo "-- ti ours"; grep -h "id=\"$ID\"" $D.load_ours.ti.xml | head -2 | cut -c1-300
  echo "-- ti pristine"; grep -h "id=\"$ID\"" $D.load_pristine.ti.xml | head -2 | cut -c1-300
  echo "-- ours ti count $(grep -c '<tripinfo ' $D.load_ours.ti.xml) pristine $(grep -c '<tripinfo ' $D.load_pristine.ti.xml)"
  echo "-- end time ours"; tail -c 300 $D.load_ours.log
done
echo "AGENT RUN DONE"
