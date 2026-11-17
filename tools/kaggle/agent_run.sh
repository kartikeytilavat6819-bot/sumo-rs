#!/bin/bash
# TEMPORARY (agent bo01): debug vs release parity for the base-offset cast fixes. Removed in the final commit.
set -u
CARGO_TARGET_DIR=$CARGO_TARGET_DIR-rel cargo build --release -p sumo_core >/tmp/bo01_rel_build.log 2>&1 || { tail -40 /tmp/bo01_rel_build.log; exit 1; }
PROFILE=debug bash tools/oracle/run_all.sh >/tmp/bo01_dbg.txt 2>&1
PROFILE=release CARGO_TARGET_DIR=$CARGO_TARGET_DIR-rel bash tools/oracle/run_all.sh >/tmp/bo01_rel.txt 2>&1
grep -E ' (PASS|FAIL|SKIP|sim-[A-Z]+) ' /tmp/bo01_dbg.txt | awk '{print $1, $2, $3}' | sort >/tmp/bo01_dbg_v.txt
grep -E ' (PASS|FAIL|SKIP|sim-[A-Z]+) ' /tmp/bo01_rel.txt | awk '{print $1, $2, $3}' | sort >/tmp/bo01_rel_v.txt
echo "== verdict diff (debug vs release)"
diff /tmp/bo01_dbg_v.txt /tmp/bo01_rel_v.txt && echo "no differing verdict rows"
echo "== debug summary"; grep -E 'PASS=' /tmp/bo01_dbg.txt | tail -1
echo "== release summary"; grep -E 'PASS=' /tmp/bo01_rel.txt | tail -1
echo "== FAIL rows (debug)"; grep -E ' (FAIL|sim-[A-Z]+) ' /tmp/bo01_dbg.txt | cut -c1-160 | head -30
