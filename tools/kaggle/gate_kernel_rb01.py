# Runs on a Kaggle CPU notebook (4 cores, ~31 GB RAM): build a source snapshot and run the oracle gate.
# rb01 variant: same gate as gate_kernel.py, plus the rail case outputs copied to /kaggle/working/rail_out.
import glob, os, shutil, subprocess, time

W = "/kaggle/working"
SRC = "/tmp/src"
LOG = open(f"{W}/gate.txt", "w")


def sh(cmd, tail=60):
    t = time.time()
    r = subprocess.run(cmd, shell=True, capture_output=True, text=True, executable="/bin/bash")
    out = (r.stdout + r.stderr).splitlines()
    LOG.write(f"$ {cmd}\n  exit={r.returncode} {time.time() - t:.0f}s\n")
    LOG.write("\n".join(out[-tail:]) + "\n\n")
    LOG.flush()
    return r.returncode


tgz = glob.glob("/kaggle/input/**/src.tgz", recursive=True)
if not tgz:
    LOG.write("NO src.tgz in /kaggle/input\n")
    raise SystemExit(1)
os.makedirs(SRC, exist_ok=True)
sh(f"tar -xzf {tgz[0]} -C {SRC} && cat {SRC}/GATE_BRANCH.txt", 5)
# scripts and *.args are committed from Windows with CRLF; Linux bash chokes on the \r
sh(f"cd {SRC}/tools && find . -type f \\( -name '*.sh' -o -name '*.args' \\) -exec sed -i 's/\\r$//' {{}} +", 3)
sh("curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal", 3)
env = f"export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/target CARGO_BUILD_JOBS=4 SCORE_OUT=/tmp/score; cd {SRC};"
if sh(env + " cargo build -p sumo -p netconvert --message-format short 2>&1 | grep -E '^(error|warning: unused)|: error' | head -40; test -x /tmp/target/debug/sumo", 45) != 0:
    LOG.write("BUILD FAILED\n")
    raise SystemExit(0)
# run_all.sh and the depart runner call sumo.exe / netconvert.exe
sh("cd /tmp/target/debug && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe", 2)
sh(env + " bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|PASS=|STALE|MISSING|rail'", 40)
# rail case: every output file of the run, for inspection from the repo
os.makedirs(f"{W}/rail_out", exist_ok=True)
for f in glob.glob("/tmp/score/run-*/rail.*"):
    shutil.copy(f, f"{W}/rail_out/")
sh("ls -la /kaggle/working/rail_out | head -30", 30)
# rail diagnostics: ours vs pristine C++ (pip eclipse-sumo 1.28.0), summaries copied to rail_out
sh("python -m pip install -q --target /tmp/sumo_pkg eclipse-sumo==1.28.0 2>&1 | tail -2", 4)
sh(env + " SUMO_PRISTINE_BIN=/tmp/sumo_pkg/sumo/bin SUMO_HOME=/tmp/sumo_pkg/sumo OURS=/tmp/target/debug/sumo OUT=/tmp/agent_run bash tools/oracle/cases/rail/agent_run.sh", 60)
sh("cp /tmp/agent_run/*.log /tmp/agent_run/*.ti.xml /kaggle/working/rail_out/ 2>/dev/null; cat /tmp/agent_run/*.log | head -5; echo copied", 5)
# insertion-time foe checks of the rail case (env-gated, temporary): compared with the C++ INS trace
sh(env + " RB01_INS=1 RB01_NE=1 /tmp/target/debug/sumo -n tools/oracle/cases/rail/ref_simple.net.xml -r tools/oracle/cases/rail/routes.rou $(cat tools/oracle/cases/rail/sim.args) --no-step-log true > /kaggle/working/rail_out/ins_rust.log 2>&1; grep -c '^INS' /kaggle/working/rail_out/ins_rust.log", 3)
shutil.copy(f"/tmp/agent_run/summary.txt", f"{W}/rail_out/") if os.path.exists("/tmp/agent_run/summary.txt") else None
sh(env + " bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1", 3)
sh(env + " cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10", 12)
LOG.write("GATE DONE\n")
