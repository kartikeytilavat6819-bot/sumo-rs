# oc110: measure meta stages: ours vs official 1.28.0 (normalised where texttest does)
set -u
OUT=/tmp/oc110
mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/sumo"
REF="$SUMO_PRISTINE_BIN/sumo"
PY=python3; command -v python3 >/dev/null 2>&1 || PY=python
cmp_norm() { # ref-file got-file
  $PY tools/oracle/park_tt_errors.py --compare "$1" "$2" >"$OUT/cmp.txt" 2>&1
  echo "  cmp rc=$?: $(head -2 "$OUT/cmp.txt" | tr '\n' ' ' | cut -c1-160)"
}

echo "=== STDOUT cases (normalised) ==="
for c in tt_meta_help tt_meta_version tt_meta_noop; do
  d="tools/oracle/cases/$c"
  "$OUR" $(cat "$d/sim.args") >"$OUT/$c.out" 2>"$OUT/$c.err"; echo "$c rc=$?"
  echo -n " stdout: "; cmp_norm "$d/ref.output" "$OUT/$c.out"
  if [ -s "$d/ref.errors" ]; then echo -n " stderr: "; cmp_norm "$d/ref.errors" "$OUT/$c.err"; fi
done

echo "=== CONFIG cases (written XML) ==="
for c in tt_meta_bool_value tt_meta_deprecated_option tt_meta_verbose tt_meta_write_config \
         tt_meta_write_schema tt_meta_write_template_full tt_meta_write_template_commented_full \
         tt_meta_umlaut_encoding tt_meta_space tt_meta_option_list_append tt_meta_rel_path; do
  d="tools/oracle/cases/$c"
  CFG=oc110_$c.cfg
  ARGS=$(cat "$d/sim.args" | sed -E \
    -e "s#(--save-(configuration|template|schema)|-C)[[:space:]]+[^[:space:]]+#\1 $CFG#g" \
    -e "s#(--save-(configuration|template|schema)|-C)=[^[:space:]]+#\1=$CFG#g")
  rm -f "$CFG"
  "$OUR" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $ARGS >"$OUT/$c.cfg.out" 2>"$OUT/$c.cfg.err"
  rc=$?
  if [ ! -f "$CFG" ]; then echo "$c rc=$rc NO CFG $(head -1 "$OUT/$c.cfg.err")"; continue; fi
  $PY tools/oracle/compare_xml.py "$d/ref.cfg" "$CFG" >"$OUT/$c.cfgdiff" 2>&1
  echo "$c rc=$rc cmp=$?: $(head -1 "$OUT/$c.cfgdiff")"
  rm -f "$CFG"
done

echo "=== EXPECT_EXIT case (umlaut) ==="
d=tools/oracle/cases/tt_meta_umlaut
"$OUR" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $(cat "$d/sim.args") >"$OUT/umlaut.out" 2>"$OUT/umlaut.err"
echo "umlaut rc=$?"
cmp_norm "$d/ref.errors" "$OUT/umlaut.err"
