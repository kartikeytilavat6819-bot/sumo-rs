#!/bin/bash
# tp01 TEMPORARY: panic text for person_public_car (remove before merge)
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
cd tools/oracle/cases/person_public_car || exit 1
RUST_BACKTRACE=1 $B -n ref_simple.net.xml -r routes.rou --end 250 --additional-files busline.add.xml --tripinfo-output /tmp/pp.xml --no-step-log true 2>&1 | grep -v '^$' | head -60
