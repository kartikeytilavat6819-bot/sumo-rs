#!/bin/bash
# oc111 probe: the 17 itt_nc_* netconvert rows, official vs ours with the same args, under /tmp/oc111.
# One compact block per row so all 17 fit in the gate tail.
ROOT=$PWD
OUT=/tmp/oc111
RUST=$CARGO_TARGET_DIR/debug/netconvert
CPP=$SUMO_PRISTINE_BIN/netconvert
ROWNAMES="itt_nc_netconvert_bugs_0xxx_2468327 itt_nc_netconvert_bugs_15xxx_ticket17161 itt_nc_netconvert_errors_aggregate itt_nc_netconvert_errors_false_output itt_nc_netconvert_errors_false_plain_output itt_nc_netconvert_errors_invalid_argument_vclass itt_nc_netconvert_errors_unknown_option itt_nc_netconvert_errors_unknown_vclass itt_nc_netconvert_function_crossings_guessed_invalid_options itt_nc_netconvert_function_crossings_specified_error_empty_edges itt_nc_netconvert_function_keep_edges__deprecated_ itt_nc_netconvert_function_keep_edges_boundary__deprecated_ itt_nc_netconvert_function_keep_edges_boundary_malformed_boundary1 itt_nc_netconvert_function_keep_edges_boundary_malformed_boundary2 itt_nc_netconvert_function_opposites_different_loadedLengths_nofix itt_nc_netconvert_function_tls_programs_green_time0 itt_nc_netconvert_meta_depending_option"
row() { ROW_IN=$1; ROW_OPTS=$2; }
# paths in the row are repo-relative; make them absolute so each run can cd into its own output dir
abs() { echo "$1" | sed "s# tools/# $ROOT/tools/#g; s#=tools/#=$ROOT/tools/#g; s#^tools/#$ROOT/tools/#"; }
for v in $ROWNAMES; do
  line=$(grep -m1 "^row $v " "$ROOT/tools/oracle/import_texttest_net.rows")
  if [ -z "$line" ]; then echo "$v: row missing"; continue; fi
  eval "row ${line#row $v }"
  d=$OUT/$v; rm -rf "$d"; mkdir -p "$d/official" "$d/ours"
  src=""; case $ROW_IN in args:*) src=$(abs "${ROW_IN#args:}") ;; esac
  opts=$(abs "$ROW_OPTS")
  co=${opts//@D/$d\/official}; ro=${opts//@D/$d\/ours}
  read -ra sa <<< "$src"; read -ra ca <<< "$co"; read -ra ra <<< "$ro"
  (cd "$d/ours" && timeout 900 "$RUST" "${sa[@]}" "${ra[@]}" -o "$d/ours/net.net.xml" > "$d/ours.stderr" 2>&1); rrc=$?
  (cd "$d/official" && timeout 900 "$CPP" "${sa[@]}" "${ca[@]}" -o "$d/official/net.net.xml" > "$d/official.stderr" 2>&1); crc=$?
  nd=$(diff "$d/official.stderr" "$d/ours.stderr" | grep -c '^[<>]')
  echo "=== $v exit off=$crc ours=$rrc stderr_diff_lines=$nd"
  if [ "$nd" != 0 ]; then diff "$d/official.stderr" "$d/ours.stderr" | head -n 4 | cut -c1-200; fi
  if [ "$crc" = 0 ] && [ -s "$d/official/net.net.xml" ] && [ -s "$d/ours/net.net.xml" ]; then
    (cd "$ROOT" && python3 tools/oracle/compare_net.py "$d/official/net.net.xml" "$d/ours/net.net.xml" 2>&1 | head -n 2)
  fi
done
