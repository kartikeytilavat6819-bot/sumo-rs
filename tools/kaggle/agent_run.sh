#!/bin/bash
# pt02 diagnostics (removed before merge): busride ours vs pristine C++, who is near bs0 (x 380..530) over time.
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
for ts in ET.parse(sys.argv[1]).getroot():
    t = float(ts.get('time'))
    if int(t) not in (20, 30, 36, 45, 60, 90, 105):
        continue
    for v in ts:
        x = float(v.get('x'))
        if 380 <= x <= 530:
            print(f"t={t:.0f} {v.tag} {v.get('id')} x={x:.2f} v={float(v.get('speed')):.2f}")
EOF
done
