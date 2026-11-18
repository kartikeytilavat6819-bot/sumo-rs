#!/bin/bash
d=tools/oracle/cases/out_bt
$CARGO_TARGET_DIR/debug/sumo -n $d/ref_simple.net.xml -r $d/routes.rou --end 150 --device.btreceiver.probability 0.5 --device.btsender.probability 1 --device.btreceiver.range 40 --device.btreceiver.offtime 0.5 --bt-output /tmp/z01.bt.xml --no-step-log >/dev/null 2>&1
grep -c "<bt " /tmp/z01.bt.xml $d/ref.bt.xml
grep -c "<seen " /tmp/z01.bt.xml $d/ref.bt.xml
grep -c "<recognitionPoint" /tmp/z01.bt.xml $d/ref.bt.xml
sed -n '/<bt-output>/,$p' /tmp/z01.bt.xml > /tmp/a.xml; sed -n '/<bt-output>/,$p' $d/ref.bt.xml > /tmp/b.xml
diff /tmp/a.xml /tmp/b.xml | cut -c1-900 | head -16
