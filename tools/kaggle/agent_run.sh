#!/bin/bash
# oc110: measure meta option/help/config behaviour: ours vs official 1.28.0
set -u
OUT=/tmp/oc110
mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/sumo"
REF="$SUMO_PRISTINE_BIN/sumo"

runpair() {
  tag=$1; shift
  "$OUR" "$@" >"$OUT/$tag.our.out" 2>"$OUT/$tag.our.err"; echo $? >"$OUT/$tag.our.rc"
  "$REF" "$@" >"$OUT/$tag.ref.out" 2>"$OUT/$tag.ref.err"; echo $? >"$OUT/$tag.ref.rc"
  echo "=== $tag rc ours=$(cat "$OUT/$tag.our.rc") ref=$(cat "$OUT/$tag.ref.rc")"
  for s in out err; do
    if diff -q "$OUT/$tag.our.$s" "$OUT/$tag.ref.$s" >/dev/null 2>&1; then
      echo "  $s: SAME"
    else
      echo "  $s: DIFF"; diff "$OUT/$tag.ref.$s" "$OUT/$tag.our.$s" | head -30
    fi
  done
}

runpair noop
runpair help --help
runpair version --version

# config writing (official exits early after writing the file)
CFG=tools/oracle/cases/tt_meta_bool_value/sumo.sumocfg
runpair cfg_deprecated -c tools/oracle/cases/tt_meta_deprecated_option/sumo.sumocfg --save-configuration "$OUT/dep.cfg"
runpair cfg_verbose -c tools/oracle/cases/tt_meta_verbose/sumo.sumocfg --save-configuration "$OUT/ver.cfg" --verbose
runpair tmpl --save-template "$OUT/tmpl.cfg"
runpair schema --save-schema "$OUT/schema.cfg"

# error-exit cases: compare stderr through the official expectation normaliser
for c in tt_meta_xml_validation tt_meta_xml_validation_net tt_meta_xml_invalid_schema tt_meta_umlaut; do
  d="tools/oracle/cases/$c"
  extra=$(cat "$d/sim.args")
  "$OUR" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $extra >"$OUT/$c.our.out" 2>"$OUT/$c.our.err"
  rc=$?
  echo "=== $c ours rc=$rc"
  python3 tools/oracle/park_tt_errors.py --compare "$d/ref.errors" "$OUT/$c.our.err" | head -12
done
