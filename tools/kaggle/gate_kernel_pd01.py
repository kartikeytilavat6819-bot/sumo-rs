# Runs on a Kaggle CPU notebook (4 cores, ~31 GB RAM): build a source snapshot and run the oracle gate.
# The snapshot arrives as a private dataset holding src.tgz (git archive of one branch).
import glob, os, subprocess, time

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
sh(env + " bash tools/oracle/run_all.sh > /kaggle/working/run_all.txt 2>&1; grep -E 'persons|FAIL|PANIC|PASS=|STALE|MISSING' /kaggle/working/run_all.txt", 60)
sh(f"mkdir -p {W}/persons && cp -r /tmp/score/run-*/persons.* {W}/persons/ 2>/dev/null; cp -r /tmp/src/tools/oracle/cases/persons/ref_simple.net.xml {W}/persons/ 2>/dev/null; ls {W}/persons | head -80", 80)
sh(env + " bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1", 3)
sh(env + " cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10", 12)
LOG.write("GATE DONE\n")
