#!/bin/bash
# oc69 measurement: detailed diffs for failing imported netconvert rows.
B=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$B/../.." && pwd)
cd "$ROOT" || exit 1
export NETOPTS2_OUT=/tmp/oc69_netopts
ROWS="itt_nc_netconvert_bugs_5xxx_ticket6497 \
itt_nc_netconvert_export_openDRIVE_lefthand \
itt_nc_netconvert_export_openDRIVE_lefthand_LHLL \
itt_nc_netconvert_export_openDRIVE_lefthand_sidewalk \
itt_nc_netconvert_export_openDRIVE_lefthand_sidewalk_LHLL \
itt_nc_netconvert_export_openDRIVE_osm_traffic_signs \
itt_nc_netconvert_export_openDRIVE_roadObjects \
itt_nc_netconvert_meta_help \
itt_nc_netconvert_meta_write_config \
itt_nc_netconvert_meta_write_template_full"
bash tools/oracle/run_netopts_new.sh $ROWS > /tmp/oc69_run.out 2>&1
for r in $ROWS; do
  d=/tmp/oc69_netopts/$r
  echo "===== $r ====="
  grep -E "$r " /tmp/oc69_run.out | head -1
  if [ -f "$d/net.log" ]; then echo "--- net.log ---"; head -40 "$d/net.log"; fi
  # diff extra output files
  for f in $(cd "$d/cpp_out" 2>/dev/null && find . -type f | sort); do
    if [ -f "$d/rust_out/$f" ]; then
      if ! cmp -s "$d/cpp_out/$f" "$d/rust_out/$f"; then
        echo "--- DIFF $f ---"
        diff "$d/cpp_out/$f" "$d/rust_out/$f" | head -30
      fi
    else
      echo "--- MISSING rust_out/$f ---"
    fi
  done
  echo "--- cpp.log tail ---"; tail -4 "$d/cpp.log" 2>/dev/null
  echo "--- rust.log tail ---"; tail -4 "$d/rust.log" 2>/dev/null
done
