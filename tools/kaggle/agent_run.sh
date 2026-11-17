#!/bin/bash
# pt02 diagnostics (removed before merge): busride ours vs pristine C++, bus L1f.0 trajectory.
C=tools/oracle/cases/busride
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=${OUT:-/tmp/agent_pt02}
for side in ours pristine; do
  mkdir -p $OUT/$side
  if [ $side = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  $BIN -n $C/ref_simple.net.xml -r $C/routes.rou --end 130 --additional-files $C/busride.add.xml \
    --fcd-output $OUT/$side/fcd.xml --no-step-log true > $OUT/$side/log.txt 2>&1
  echo "== $side exit=$?"
  python3 - "$OUT/$side/fcd.xml" <<'EOF'
import sys, xml.etree.ElementTree as ET
prev = None
for ts in ET.parse(sys.argv[1]).getroot():
    t = float(ts.get('time'))
    for v in ts:
        if v.get('id') == 'L1f.0' and t <= 130 and (int(t) % 3 == 0 or 14 <= t <= 17 or 40 <= t <= 46 or 108 <= t <= 113):
            print(f"t={t:.0f} x={float(v.get('x')):.2f} y={float(v.get('y')):.2f} v={float(v.get('speed')):.2f}")
EOF
done
