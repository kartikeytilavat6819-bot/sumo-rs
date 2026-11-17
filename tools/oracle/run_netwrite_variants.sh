#!/bin/bash
# netconvert output variants (plain-output-prefix, opendrive, dlr-navteq, matsim, amitran, street-sign, ptstop)
# on one small plain net (cases/vtypes/plain): ours vs pristine refs, compared file by file.
# Refs:    NC=$SUMO_PRISTINE_BIN/netconvert bash tools/oracle/run_netwrite_variants.sh ref
# Compare: NC=$CARGO_TARGET_DIR/debug/netconvert bash tools/oracle/run_netwrite_variants.sh
# Run from the repo root. Verdicts per variant: PASS, or FAIL with the reason (exit code, file set, content).
# Lines that carry the generation date are ignored (OpenDRIVE header and the net comment).
B=$(cd "$(dirname "$0")" && pwd)
cd "$B/../.." || exit 1
MODE=${1:-compare}
NC=${NC:-${CARGO_TARGET_DIR:-/tmp/target}/debug/netconvert}
PLAIN=tools/oracle/cases/vtypes/plain
REF=tools/oracle/cases/netwrite_variants/ref
OUT=${NWV_OUT:-/tmp/nwv}
[ "$MODE" = ref ] && BASE=$REF || BASE=$OUT
mkdir -p "$BASE" "$OUT"
NET=(--node-files $PLAIN/net.nod.xml --edge-files $PLAIN/net.edg.xml --connection-files $PLAIN/net.con.xml --type-files $PLAIN/net.typ.xml --tllogic-files $PLAIN/net.tll.xml)
NAMES=(plain opendrive dlr matsim amitran street_sign ptstop)
OPTS=(
  "--plain-output-prefix @D/plain/net"
  "--opendrive-output @D/net.xodr"
  "--dlr-navteq-output @D/dlr"
  "--matsim-output @D/matsim.xml"
  "--amitran-output @D/amitran.xml"
  "--street-sign-output @D/signs.xml"
  "--ptstop-output @D/ptstops.xml"
)
norm() { sed -e "s#$REF#BASE#g" -e "s#$OUT#BASE#g" -e 's/\r$//' "$1" | grep -v -e 'generated on' -e 'date='; }
fail=0
for i in "${!NAMES[@]}"; do
  name=${NAMES[$i]}
  d=$BASE/$name
  rm -rf "$d"; mkdir -p "$d/plain"
  opt=${OPTS[$i]//@D/$d}
  # shellcheck disable=SC2086
  "$NC" "${NET[@]}" -o "$d/net.xml" $opt > "$OUT/$name.log" 2>&1
  echo $? > "$BASE/$name.rc"
  if [ "$MODE" = ref ]; then continue; fi
  verdict=PASS; why=
  if [ "$(tr -d '\r' < "$REF/$name.rc")" != "$(tr -d '\r' < "$BASE/$name.rc")" ]; then
    verdict=FAIL; why="exit code $(cat "$BASE/$name.rc") vs pristine $(cat "$REF/$name.rc")"
  else
    lr=$(cd "$REF/$name" && find . -type f ! -name net.xml | sort)
    lo=$(cd "$BASE/$name" && find . -type f ! -name net.xml | sort)
    if [ "$lr" != "$lo" ]; then
      verdict=FAIL; why="file set differs"
    else
      for f in $lr; do
        if ! diff -q <(norm "$REF/$name/$f") <(norm "$BASE/$name/$f") > /dev/null; then
          verdict=FAIL; why="$f differs"; break
        fi
      done
    fi
  fi
  [ "$verdict" = FAIL ] && fail=1
  echo "$name $verdict $why"
done
[ "$MODE" = ref ] && echo "refs written to $REF" && exit 0
exit $fail
