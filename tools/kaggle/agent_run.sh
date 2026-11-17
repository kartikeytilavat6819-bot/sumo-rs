#!/usr/bin/env bash
# TEMPORARY (du01): build duarouter and run its route oracle on the VM. Remove before merge.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export OUT=/tmp/du01_oracle
if ! cargo build -p duarouter > /tmp/du01_build.log 2>&1; then
  echo "DUAROUTER BUILD FAILED"
  grep -E '^error' -A6 /tmp/du01_build.log | head -120
  exit 0
fi
bash tools/oracle/run_duarouter.sh 2>&1
for d in "$OUT"/duarouter_*/; do
  for f in "$d"*.diff; do
    [ -f "$f" ] && { echo "== $f"; head -8 "$f"; }
  done
done
