#!/bin/bash
# du05 temporary (removed in the final commit): repeat the duarouter oracle, AddressSanitizer every case, release parity.
cd "$(dirname "$0")/../.." || exit 1
TGT=${CARGO_TARGET_DIR:-/tmp/target}
BIN=$TGT/debug/duarouter
CASES=$(cd tools/oracle/cases && ls -d duarouter_* | tr '\n' ' ')

for i in 1 2 3 4 5; do
  OUT=/tmp/dua_rep$i BIN=$BIN bash tools/oracle/run_duarouter.sh >/dev/null 2>&1
  echo "debug run $i: PASS=$(grep -c $'\tPASS\t' /tmp/dua_rep$i/results.tsv) of $(($(wc -l < /tmp/dua_rep$i/results.tsv) - 1))"
  grep -v $'\tPASS\t' /tmp/dua_rep$i/results.tsv | tail -n +2 | cut -f1,2,3,4,5 | cut -c1-220
done

rustup toolchain install nightly --profile minimal -c rust-src >/tmp/du05_nightly.log 2>&1; echo "nightly rc=$?"
RUSTFLAGS="-Zsanitizer=address" cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu -p duarouter --target-dir $TGT-asan >/tmp/du05_asan.log 2>&1; echo "asan build rc=$?"
grep -aE '^error' /tmp/du05_asan.log | head -10
export ASAN_OPTIONS=detect_leaks=0
ABIN=$TGT-asan/x86_64-unknown-linux-gnu/debug/duarouter
if [ -x "$ABIN" ]; then
  OUT=/tmp/dua_asan BIN=$ABIN bash tools/oracle/run_duarouter.sh >/dev/null 2>&1
  echo "asan run: PASS=$(grep -c $'\tPASS\t' /tmp/dua_asan/results.tsv) of $(($(wc -l < /tmp/dua_asan/results.tsv) - 1))"
  grep -v $'\tPASS\t' /tmp/dua_asan/results.tsv | tail -n +2 | cut -f1,2,3,4,5 | cut -c1-220
  shown=0
  for c in $CASES; do
    if grep -q 'ERROR: AddressSanitizer' /tmp/dua_asan/$c/cand.log 2>/dev/null; then
      echo "== ASan error in $c"
      grep -m1 -A24 'ERROR: AddressSanitizer' /tmp/dua_asan/$c/cand.log | cut -c1-200
      shown=$((shown + 1)); [ $shown -ge 3 ] && break
    fi
  done
else
  echo "asan binary missing"; tail -20 /tmp/du05_asan.log
fi

cargo build --release -p duarouter --message-format short >/tmp/du05_rel.log 2>&1; echo "release build rc=$?"
grep -aE ': error|^error' /tmp/du05_rel.log | head -20
RBIN=$TGT/release/duarouter
NOMM=$(echo $CASES | tr ' ' '\n' | grep -v mapmatch | tr '\n' ' ')
for i in 1 2 3 4 5; do
  OUT=/tmp/dua_rel$i BIN=$RBIN bash tools/oracle/run_duarouter.sh $NOMM >/dev/null 2>&1
  echo "release run $i (MapMatcher-free): PASS=$(grep -c $'\tPASS\t' /tmp/dua_rel$i/results.tsv) of $(($(wc -l < /tmp/dua_rel$i/results.tsv) - 1))"
  grep -v $'\tPASS\t' /tmp/dua_rel$i/results.tsv | tail -n +2 | cut -f1,2,3,4,5 | cut -c1-220
done
