#!/bin/bash
d=tools/oracle/cases/out_bt
A="-n $d/ref_simple.net.xml -r $d/routes.rou --end 150 --device.btreceiver.probability 0.5 --device.btsender.probability 1 --device.btreceiver.range 40 --device.btreceiver.offtime 0.5 --no-step-log"
$CARGO_TARGET_DIR/debug/sumo $A --bt-output /tmp/z01.bt.xml 2>/tmp/rs.trace >/dev/null
B=$(bash tools/gcloud/vm_cpp_trace.sh z01 tools/kaggle/tmp/bt.patch sumo | tail -1)
$B $A --bt-output /tmp/z01c.bt.xml 2>/tmp/cpp.trace >/dev/null
grep -c "^V" /tmp/rs.trace /tmp/cpp.trace
grep "^V" /tmp/rs.trace > /tmp/r.t; grep "^V" /tmp/cpp.trace > /tmp/c.t
diff /tmp/r.t /tmp/c.t | head -20
