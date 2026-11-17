#!/bin/bash
# TEMPORARY (ub03): release-vs-debug verdict parity for the UB audit. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
export PATH="$HOME/.cargo/bin:$PATH"
CT=${CARGO_TARGET_DIR:-/tmp/target}
export CARGO_TARGET_DIR=$CT
export SCORE_OUT=/tmp/ub03score
mkdir -p "$SCORE_OUT"
echo "== build debug"
cargo build -p sumo -p netconvert > /tmp/ub03_bd.txt 2>&1; echo "rc=$?"; grep -E '^error' /tmp/ub03_bd.txt | head -3
echo "== build release"
CARGO_TARGET_DIR=$CT-rel cargo build --release -p sumo -p netconvert > /tmp/ub03_br.txt 2>&1; echo "rc=$?"; grep -E '^error' /tmp/ub03_br.txt | head -3; tail -1 /tmp/ub03_br.txt
ls -l --time-style=+%T "$CT"/debug/sumo* "$CT"-rel/release/sumo* 2>&1 | head -6
PROFILE=debug bash tools/oracle/run_all.sh > /tmp/ub03_debug.txt 2>&1
CARGO_TARGET_DIR=$CT-rel PROFILE=release bash tools/oracle/run_all.sh > /tmp/ub03_rel.txt 2>&1
echo "debug:   $(tail -1 /tmp/ub03_debug.txt)"
echo "release: $(tail -1 /tmp/ub03_rel.txt)"
VF='$2 ~ /^(net|net-.*|sim|sim-PANIC|variants)$/'
awk "$VF"'{print $1,$2,$3}' /tmp/ub03_debug.txt > /tmp/ub03_dv.txt
awk "$VF"'{print $1,$2,$3}' /tmp/ub03_rel.txt > /tmp/ub03_rv.txt
echo "verdict rows differing: $(diff /tmp/ub03_dv.txt /tmp/ub03_rv.txt | grep -c '^[<>]')"
diff /tmp/ub03_dv.txt /tmp/ub03_rv.txt | head -8
for d in $(ls -td "$SCORE_OUT"/run-* | head -2); do
  echo "-- $d"; grep -h -m2 -E 'SUMMARY|panicked|rror' "$d/junction.net.log" 2>/dev/null | cut -c1-200
done
echo "AGENT RUN DONE"
