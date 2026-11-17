#!/bin/bash
# TEMPORARY (ub03): release-vs-debug verdict parity for the UB audit. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
export PATH="$HOME/.cargo/bin:$PATH"
CT=${CARGO_TARGET_DIR:-/tmp/target}
export CARGO_TARGET_DIR=$CT
export SCORE_OUT=/tmp/ub03score
mkdir -p "$SCORE_OUT"
echo "== build debug"
cargo build -q -p sumo -p netconvert 2>&1 | grep -E '^(error|warning: unused)' | head -5
echo "== build release"
CARGO_TARGET_DIR=$CT-rel cargo build -q --release -p sumo -p netconvert 2>&1 | grep -E '^error' | head -5
echo "== run_all debug"
PROFILE=debug bash tools/oracle/run_all.sh > /tmp/ub03_debug.txt 2>&1
echo "== run_all release"
CARGO_TARGET_DIR=$CT-rel PROFILE=release bash tools/oracle/run_all.sh > /tmp/ub03_rel.txt 2>&1
echo "debug:   $(tail -1 /tmp/ub03_debug.txt)"
echo "release: $(tail -1 /tmp/ub03_rel.txt)"
echo "== verdict diff (case kind verdict), empty means identical"
diff <(awk 'NF>=3{print $1,$2,$3}' /tmp/ub03_debug.txt) <(awk 'NF>=3{print $1,$2,$3}' /tmp/ub03_rel.txt) && echo SAME_VERDICTS
echo "== FAIL rows (release)"
awk '$3 ~ /FAIL|PANIC/ || $2 ~ /PANIC/' /tmp/ub03_rel.txt | head -20
