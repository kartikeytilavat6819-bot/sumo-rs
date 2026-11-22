#!/usr/bin/env bash
# oc104 probe: official vs ours on the target rows (SSM, rerouting A* landmark, taxi).
# Runs from repo root. Outputs under /tmp/oc104. Prints exit codes, first stderr lines,
# and the first differing lines against the committed refs.
OUT=/tmp/oc104; mkdir -p "$OUT"
OFF="${SUMO_PRISTINE_BIN:?}/sumo"
OURS="${CARGO_TARGET_DIR:-target}/debug/sumo"
for c in $(cat .oc_haiku_rows.txt); do
  d="tools/oracle/cases/$c"
  [ -d "$d" ] || { echo "== $c: missing case dir"; continue; }
  EXTRA=""; [ -f "$d/sim.args" ] && EXTRA=$(tr -d '\r' < "$d/sim.args")
  for side in off ours; do
    bin=$OFF; [ $side = ours ] && bin=$OURS
    rdir="$OUT/$c.$side"; rm -rf "$rdir"; mkdir -p "$rdir"
    # shellcheck disable=SC2086
    (cd "$rdir" && timeout 600 "$bin" -n "$ROOT/$d/ref_simple.net.xml" -r "$ROOT/$d/routes.rou" \
        --no-step-log true $EXTRA >out.txt 2>err.txt; echo $? > rc.txt)
  done
  echo "== $c: exit off=$(cat $OUT/$c.off/rc.txt) ours=$(cat $OUT/$c.ours/rc.txt)"
  echo "  -- off stderr:"; head -n 4 "$OUT/$c.off/err.txt" | cut -c1-200 | sed 's/^/     /'
  echo "  -- ours stderr:"; head -n 4 "$OUT/$c.ours/err.txt" | cut -c1-200 | sed 's/^/     /'
  if [ -f "$d/ref.errors" ]; then
    if diff -q "$d/ref.errors" "$OUT/$c.ours/err.txt" >/dev/null; then echo "  ref.errors: MATCH"
    else echo "  ref.errors vs ours (first diff):"; diff "$d/ref.errors" "$OUT/$c.ours/err.txt" | head -n 6 | cut -c1-200 | sed 's/^/     /'; fi
  fi
  if [ -f "$d/ref.output" ]; then
    if diff -q "$d/ref.output" "$OUT/$c.ours/out.txt" >/dev/null; then echo "  ref.output: MATCH"
    else echo "  ref.output vs ours (first diff):"; diff "$d/ref.output" "$OUT/$c.ours/out.txt" | head -n 6 | cut -c1-200 | sed 's/^/     /'; fi
  fi
done
