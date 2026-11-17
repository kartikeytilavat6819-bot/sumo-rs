#!/bin/bash
# rc01 diagnostics (removed before merge): rerouter_parking panic backtrace, containers first diff.
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=${OUT:-/tmp/agent_run}
mkdir -p $OUT
stops() {
python3 - "$1" <<'EOF'
import sys, xml.etree.ElementTree as ET
prev = None
for ev, el in ET.iterparse(sys.argv[1]):
    if el.tag != 'timestep':
        continue
    t = el.get('time')
    for v in el.findall('vehicle'):
        if v.get('id') != 'truck0':
            continue
        sp = float(v.get('speed'))
        if sp == 0 and prev != 'stop':
            print('truck0 stop start t=%s lane=%s pos=%s' % (t, v.get('lane'), v.get('pos')))
            prev = 'stop'
        elif sp > 0 and prev == 'stop':
            print('truck0 stop end   t=%s lane=%s pos=%s' % (t, v.get('lane'), v.get('pos')))
            prev = 'go'
    el.clear()
EOF
}
for n in rerouter_parking containers; do
  C=tools/oracle/cases/$n
  RUST_BACKTRACE=1 $OURS -n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true \
      --tripinfo-output $OUT/$n.ours.ti.xml --fcd-output $OUT/$n.ours.fcd.xml $(cat $C/sim.args) > $OUT/$n.ours.log 2>&1
  echo "== $n ours exit=$?"
  grep -m1 -A60 'panicked' $OUT/$n.ours.log | grep -E 'panicked|\.rs:' | cut -c1-200 | head -12
  if [ "$n" = containers ]; then
    echo "-- ref stops:"; stops $C/ref.fcd.xml
    echo "-- ours stops:"; stops $OUT/$n.ours.fcd.xml
    echo "-- RC01 canLeave:"; grep 'RC01 canLeave' $OUT/$n.ours.log | head -10
    echo "-- RC01 nm 135..205:"; grep 'RC01 nm' $OUT/$n.ours.log | awk '{split($3,a,"="); if (a[2]+0>=135000 && a[2]+0<=205000) print}' | head -30
    echo "-- panics:"; grep -m3 -E 'panicked|msp_model' $OUT/$n.ours.log | cut -c1-200
    grep -E 'containerinfo|transport ' $OUT/$n.ours.ti.xml | cut -c1-200 | head -6
  fi
  echo "-- v2 ref/ours:"; grep 'id="v2"' $C/ref.tripinfo.xml | cut -c1-260; grep 'id="v2"' $OUT/$n.ours.ti.xml | cut -c1-260
  python tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $OUT/$n.ours.ti.xml 2>&1 | head -6
done
