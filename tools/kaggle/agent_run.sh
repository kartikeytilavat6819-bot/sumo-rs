#!/bin/bash
set -u
cd /c/Users/karti/sumo-rs 2>/dev/null || cd "$(dirname "$0")/../.."
ROOT=$(pwd)
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/sumo-rs-target}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
bash tools/oracle/run_netopts_new.sh itt_nc_netconvert_import_OSM_railway_RE70 itt_nc_netconvert_import_OSM_railway_signal_dir_override_repair 2>&1 | tail -30
echo "===== RE70 additional.xml cpp ====="
grep 'trainStop\|busStop' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/cpp_out/additional.xml 2>/dev/null
echo "===== RE70 additional.xml rust ====="
grep 'trainStop\|busStop' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/rust_out/additional.xml 2>/dev/null
echo "===== RE70 ptlines cpp ====="
grep 'ptLine\|route\|busStop' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/cpp_out/ptlines.xml 2>/dev/null
echo "===== RE70 ptlines rust ====="
grep 'ptLine\|route\|busStop' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/rust_out/ptlines.xml 2>/dev/null
echo "===== RE70 cpp.log all warnings ====="
grep -iE 'stop|line|bidi|route|revise' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/cpp.log 2>/dev/null
echo "===== RE70 rust.log relevant ====="
grep -iE 'stop|line|bidi|route|revise|DBG|DEBUG' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_RE70/rust.log 2>/dev/null
echo "===== signal_dir lane shape ====="
grep -o '43595467_0"[^>]*' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_signal_dir_override_repair/cpp.net.xml 2>/dev/null
grep -o '43595467_0"[^>]*' /tmp/netopts2/itt_nc_netconvert_import_OSM_railway_signal_dir_override_repair/rust.net.xml 2>/dev/null
