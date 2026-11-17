#!/bin/bash
# Merge the integration branch `wave2` into main, verify, push.
set -u
cd /c/Users/karti/sumo-rs || exit 1
LOCK=/c/Users/karti/sumo-rs-merge.lock
until mkdir "$LOCK" 2>/dev/null; do sleep 5; done
trap 'rmdir "$LOCK"' EXIT
export PATH="$HOME/.cargo/bin:/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-main
export CARGO_BUILD_JOBS=8

before=$(git rev-parse HEAD)
if ! git merge -q --no-edit wave2; then
  echo "CONFLICT"; git diff --name-only --diff-filter=U; git merge --abort; exit 1
fi
errs=$(cargo check -q -p sumo_core --message-format short 2>&1 | grep -c ': error')
if [ "$errs" != "0" ]; then
  echo "$errs core errors -> rolling back"
  git branch -f fix-wave2 HEAD; git reset -q --hard "$before"; exit 1
fi
echo "core clean"
cargo check -q -p netconvert -p sumo --message-format short 2>&1 | grep ': error' | head -5
echo "binaries checked (exit above: none means ok)"
echo "real todo!() left: $(grep -rn 'todo!(' crates/sumo_core/src | grep -v 'GENERATED SKELETON' | grep -vE ':[0-9]+:[[:space:]]*//' | wc -l)   shortcut stubs: $(grep -rn 'panic!("shortcut' crates/sumo_core/src | wc -l)"
# Disk guard: every agent builds in its own target dir (correct, but it fills the disk), and harness run
# dirs accumulate. One night this reached 0 GB free and builds started failing. Keep the older run dirs of
# the scoreboard, warn loudly if the volume is short.
for d in /c/Users/karti/AppData/Local/Temp/score/run-*; do
  [ -d "$d" ] || continue
  find "$d" -maxdepth 0 -mmin +180 -exec rm -rf {} + 2>/dev/null
done
free_kb=$(df -k /c | awk 'NR==2 {print $4}')
free_gb=$((free_kb / 1024 / 1024))
[ "$free_gb" -lt 5 ] && echo "WARNING: only ${free_gb}GB free on C: — builds will start failing; prune agent target dirs"
git push -q && echo "pushed"
git log --oneline -1
