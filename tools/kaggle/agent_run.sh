#!/bin/bash
# oc17 measurement: run the opendriveOutput netconvert option row and show the net.xodr diff.
cd "$(dirname "$0")/../.." || exit 1
export NETOPTS2_OUT=/tmp/netopts2_oc17
bash tools/oracle/run_netopts_new.sh opendriveOutput
echo "=== raw diff cpp vs rust net.xodr (first 80 lines) ==="
diff "$NETOPTS2_OUT/opendriveOutput/cpp_out/net.xodr" "$NETOPTS2_OUT/opendriveOutput/rust_out/net.xodr" | head -80
echo "=== normalized diff (drop date/paths/CR, first 80 lines) ==="
D="$NETOPTS2_OUT/opendriveOutput"
norm='s#'"$D"'/cpp_out#OUT#g; s#'"$D"'/rust_out#OUT#g; s#'"$D"'/cpp.net.xml#NET#g; s#'"$D"'/rust.net.xml#NET#g; /generated on/d; s/\r$//'
diff <(sed -e "$norm" "$D/cpp_out/net.xodr") <(sed -e "$norm" "$D/rust_out/net.xodr") | head -80
