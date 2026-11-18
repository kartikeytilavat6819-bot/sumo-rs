#!/bin/bash
# Full official-test-suite census on a gate VM (user: "import all the tests, then we know exactly what does not
# work"). Imports EVERY official sumo / netconvert / duarouter TextTest into a scratch copy (nothing is committed:
# the references of ~8000 tests would bloat the repo), generates references with the official binaries only,
# runs our port against all of them and writes reports to /home/gate/census/report/.
#   sudo -iu gate bash census_run.sh <src.tgz>
set -u
C=/home/gate/census; R=$C/report; mkdir -p $C $R
export PATH=$HOME/.cargo/bin:$PATH SUMO_HOME=/home/gate/sumo_pkg/sumo SUMO_PRISTINE_BIN=/home/gate/sumo_pkg/sumo/bin \
       CARGO_TARGET_DIR=$C/target CARGO_BUILD_JOBS=12 CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_DEV_DEBUG=0
J=${CENSUS_JOBS:-14}
log() { echo "$(date +%H:%M:%S) $*" | tee -a $R/progress.log; }

log "official tests: SUMO v1_28_0 tests/ (sparse clone)"
if [ ! -d $C/sumo-tests/tests ]; then
  git clone -q --depth 1 --filter=blob:none --sparse -b v1_28_0 https://github.com/eclipse-sumo/sumo.git $C/sumo-tests &&
  git -C $C/sumo-tests sparse-checkout set tests || { log "CLONE FAILED"; exit 1; }
fi
T=$C/sumo-tests/tests

log "source"
rm -rf $C/src; mkdir -p $C/src; tar -xzf "$1" -C $C/src || { log "SRC CORRUPT"; exit 1; }
cd $C/src
find tools -type f \( -name '*.sh' -o -name '*.args' \) -exec sed -i 's/\r$//' {} +

log "build (opt-level 1)"
cargo build -q -p sumo -p netconvert -p duarouter > $R/build.log 2>&1 || { log "BUILD FAILED"; exit 1; }
(cd $CARGO_TARGET_DIR/debug && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe)

log "import sumo suite"
subs=$(cd $T/sumo && ls -d */ | tr -d / | tr '\n' ' ')
python3 tools/oracle/import_texttest.py --tests-sumo $T/sumo $subs > $R/import_sumo.log 2>&1
log "sumo cases now: $(ls -d tools/oracle/cases/tt_* | wc -l) tt_ cases"
log "references (official sumo, $J jobs)"
GEN_REF_JOBS=$J python3 tools/oracle/gen_tt_refs_vm.py > $R/genref.log 2>&1; log "$(tail -1 $R/genref.log)"

log "import netconvert + duarouter suites"
python3 tools/oracle/import_texttest_tools.py --app netconvert $(for d in $T/netconvert/*/; do printf -- '--subtree %s ' "$d"; done) > $R/import_netconvert.log 2>&1
python3 tools/oracle/import_texttest_tools.py --app duarouter $(for d in $T/duarouter/*/; do printf -- '--subtree %s ' "$d"; done) > $R/import_duarouter.log 2>&1

log "run_all over every case ($J shards)"
RUN_SHARDS=$J SCORE_OUT=$C/score bash tools/oracle/run_all.sh > $R/run_all.txt 2>&1; log "$(grep '^PASS=' $R/run_all.txt)"
log "netconvert rows"
NETOPTS2_OUT=$C/netopts NETOPTS2_JOBS=$J bash tools/oracle/run_netopts_new.sh > $R/netopts.txt 2>&1
cp $C/netopts/summary.tsv $R/netopts_summary.tsv 2>/dev/null; log "$(grep 'TOTAL' $R/netopts.txt)"
log "duarouter cases"
OUT=$C/dua bash tools/oracle/run_duarouter.sh > $R/duarouter.txt 2>&1
cp $C/dua/results.tsv $R/duarouter_results.tsv 2>/dev/null
log "duarouter: PASS=$(grep -c $'\tPASS\t' $R/duarouter_results.tsv) of $(($(wc -l < $R/duarouter_results.tsv) - 1))"
log "CENSUS DONE"
