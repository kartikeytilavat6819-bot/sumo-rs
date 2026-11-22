#!/bin/bash
# oc113 measurement probe: official vs ours netconvert on the target rows (read from the row tables).
ROOT=$(cd "$(dirname "$0")/../.." && pwd); cd "$ROOT" || exit 1
RUST=${RUST_NETCONVERT:-$CARGO_TARGET_DIR/debug/netconvert}
CPP=${CPP_NETCONVERT:-$SUMO_PRISTINE_BIN/netconvert}
export SUMO_HOME=${SUMO_HOME:-$SUMO_PRISTINE_BIN/..}
OUT=/tmp/oc113; mkdir -p "$OUT"
TARGETS=$(cat .oc_haiku_rows.txt)
PLAIN="--node-files tools/oracle/cases/vtypes/plain/net.nod.xml --edge-files tools/oracle/cases/vtypes/plain/net.edg.xml --connection-files tools/oracle/cases/vtypes/plain/net.con.xml --type-files tools/oracle/cases/vtypes/plain/net.typ.xml --tllogic-files tools/oracle/cases/vtypes/plain/net.tll.xml"
# collect rows (name, input, options) with the same row() shape as run_netopts_new.sh
declare -A RIN ROPT
row() { RIN[$1]=$2; ROPT[$1]=$3; }
source tools/oracle/import_texttest_net.rows 2>/dev/null
for v in $TARGETS; do
  [ -n "${RIN[$v]+x}" ] || { echo "$v NOROW"; continue; }
  in=${RIN[$v]}; opts=${ROPT[$v]}
  d=$OUT/$v; rm -rf "$d"; mkdir -p "$d/official_out" "$d/ours_out"
  case $in in args:*) src=${in#args:} ;; *) src="" ;; esac
  co=${opts//@D/$d\/official_out}; ro=${opts//@D/$d\/ours_out}
  src=${src//tools//$ROOT/tools/}; co=${co//tools//$ROOT/tools/}; ro=${ro//tools//$ROOT/tools/}
  read -ra sa <<< "$src"; read -ra ca <<< "$co"; read -ra ra <<< "$ro"
  (cd "$d" && timeout 600 "$CPP" "${sa[@]}" "${ca[@]}" -o "$d/official.net.xml" > "$d/official.log" 2>&1); crc=$?
  (cd "$d" && timeout 600 "$RUST" "${sa[@]}" "${ra[@]}" -o "$d/ours.net.xml" > "$d/ours.log" 2>&1); rrc=$?
  echo "== $v official_exit=$crc ours_exit=$rrc"
  echo "-- official stderr:"; grep -v '^$' "$d/official.log" | head -3
  echo "-- ours stderr:"; grep -v '^$' "$d/ours.log" | head -3
  if [ $crc -eq 0 ] && [ $rrc -eq 0 ]; then
    python3 tools/oracle/compare_net.py "$d/official.net.xml" "$d/ours.net.xml" 2>&1 | head -8
  fi
done
