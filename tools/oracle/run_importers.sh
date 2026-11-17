#!/bin/bash
# Import-case oracle: for each tools/oracle/import_cases/<case> run the official netconvert (ref)
# and ours with the case's options.netconvert, then compare the nets with compare_net.py.
# Usage: run_importers.sh OURS_NETCONVERT REF_NETCONVERT [OUT_DIR]
set -u
OURS=$(realpath "$1")
REF=$(realpath "$2")
OUT=${3:-/tmp/import_out}
HERE=$(cd "$(dirname "$0")" && pwd)
pass=0
fail=0
rm -rf "$OUT"
mkdir -p "$OUT"
for dir in "$HERE"/import_cases/*/; do
  name=$(basename "$dir")
  opts=$(tr -d '\r' < "$dir/options.netconvert" | tr '\n' ' ')
  extra=""
  case "$opts" in *--output*|*" -o "*) ;; *) extra="-o net.net.xml" ;; esac
  for side in ref ours; do
    bin=$REF
    [ "$side" = ours ] && bin=$OURS
    mkdir -p "$OUT/$name/$side"
    cp "$dir"/* "$OUT/$name/$side/"
    (cd "$OUT/$name/$side" && "$bin" $opts $extra > netconvert.log 2>&1)
    if [ "$side" = ref ]; then rc_ref=$?; else rc_ours=$?; fi
  done
  # error parity: the official netconvert rejects this input too (no net, same exit code)
  if [ ! -f "$OUT/$name/ref/net.net.xml" ] && [ ! -f "$OUT/$name/ours/net.net.xml" ] && [ "$rc_ref" = "$rc_ours" ]; then
    echo "PASS $name (both exit $rc_ref without net.net.xml)"
    pass=$((pass + 1))
    continue
  fi
  if [ ! -f "$OUT/$name/ref/net.net.xml" ] || [ ! -f "$OUT/$name/ours/net.net.xml" ]; then
    echo "FAIL $name: no net.net.xml (ref: $(test -f "$OUT/$name/ref/net.net.xml" && echo ok || echo missing), ours: $(test -f "$OUT/$name/ours/net.net.xml" && echo ok || echo missing))"
    tail -3 "$OUT/$name/ours/netconvert.log"
    fail=$((fail + 1))
    continue
  fi
  if python3 "$HERE/compare_net.py" "$OUT/$name/ref/net.net.xml" "$OUT/$name/ours/net.net.xml" > "$OUT/$name/diff.txt" 2>&1; then
    echo "PASS $name"
    pass=$((pass + 1))
  else
    echo "FAIL $name: $(head -1 "$OUT/$name/diff.txt")"
    fail=$((fail + 1))
  fi
done
echo "IMPORTERS PASS=$pass FAIL=$fail"
[ "$fail" -eq 0 ]
