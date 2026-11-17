#!/bin/bash
# du05 temporary (removed in the final commit): repeat the duarouter oracle, valgrind every case, release parity.
cd "$(dirname "$0")/../.." || exit 1
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/duarouter
if ! command -v valgrind >/dev/null 2>&1; then
  sudo apt-get update -y >/tmp/du05_apt.log 2>&1; sudo apt-get install -y valgrind >>/tmp/du05_apt.log 2>&1
  command -v valgrind >/dev/null 2>&1 || { echo "valgrind missing:"; tail -8 /tmp/du05_apt.log; }
fi
CASES=$(cd tools/oracle/cases && ls -d duarouter_* | tr '\n' ' ')

for i in 1 2 3 4 5; do
  OUT=/tmp/dua_rep$i BIN=$BIN bash tools/oracle/run_duarouter.sh >/dev/null 2>&1
  echo "debug run $i: PASS=$(grep -c $'\tPASS\t' /tmp/dua_rep$i/results.tsv) of $(($(wc -l < /tmp/dua_rep$i/results.tsv) - 1))"
  grep -v $'\tPASS\t' /tmp/dua_rep$i/results.tsv | tail -n +2 | cut -f1,2,3,4,5 | cut -c1-220
done

VG=/tmp/du05_vg; mkdir -p $VG
printf '#!/bin/bash\nexec timeout 900 valgrind -q --error-exitcode=9 --track-origins=yes %s "$@"\n' "$BIN" > $VG/duarouter
chmod +x $VG/duarouter
OUT=/tmp/dua_vg BIN=$VG/duarouter bash tools/oracle/run_duarouter.sh >/dev/null 2>&1
echo "valgrind run: PASS=$(grep -c $'\tPASS\t' /tmp/dua_vg/results.tsv) of $(($(wc -l < /tmp/dua_vg/results.tsv) - 1))"
shown=0
for c in $CASES; do
  if grep -q 'Invalid \(read\|write\|free\)\|Mismatched\|uninitialised' /tmp/dua_vg/$c/cand.log 2>/dev/null; then
    echo "== valgrind errors in $c: $(grep -c 'Invalid \(read\|write\|free\)' /tmp/dua_vg/$c/cand.log) invalid"
    grep -m1 -A22 'Invalid \(read\|write\|free\)\|Mismatched\|uninitialised' /tmp/dua_vg/$c/cand.log | sed -E 's/^==[0-9]+== //' | cut -c1-200
    shown=$((shown + 1)); [ $shown -ge 3 ] && break
  fi
done

cargo build --release -p duarouter --message-format short >/tmp/du05_rel.log 2>&1; echo "release build rc=$?"
grep -aE ': error|^error' /tmp/du05_rel.log | head -20
RBIN=${CARGO_TARGET_DIR:-/tmp/target}/release/duarouter
NOMM=$(echo $CASES | tr ' ' '\n' | grep -v mapmatch | tr '\n' ' ')
for i in 1 2 3 4 5; do
  OUT=/tmp/dua_rel$i BIN=$RBIN bash tools/oracle/run_duarouter.sh $NOMM >/dev/null 2>&1
  echo "release run $i (MapMatcher-free): PASS=$(grep -c $'\tPASS\t' /tmp/dua_rel$i/results.tsv) of $(($(wc -l < /tmp/dua_rel$i/results.tsv) - 1))"
  grep -v $'\tPASS\t' /tmp/dua_rel$i/results.tsv | tail -n +2 | cut -f1,2,3,4,5 | cut -c1-220
done
