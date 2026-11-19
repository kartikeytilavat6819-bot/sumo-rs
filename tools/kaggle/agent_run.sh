#!/bin/bash
set -u
cd /c/Users/karti/sumo-rs 2>/dev/null || cd "$(dirname "$0")/../.."
ROOT=$(pwd)
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
bash tools/oracle/run_netopts_new.sh itt_nc_netconvert_import_OSM_railway_RE70 itt_nc_netconvert_import_OSM_railway_signal_dir_override_repair 2>&1 | tail -60
echo "=== RE70 diff additional.xml ==="
diff /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/cpp_out/additional.xml /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/rust_out/additional.xml 2>/dev/null | head -40
echo "=== signal_dir diff (net.xml lane shape) ==="
grep -n "43595467_0" /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_signal_dir_override_repair/cpp.net.xml /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_signal_dir_override_repair/rust.net.xml | head -20