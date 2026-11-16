# Kaggle CPU notebook: gate several branch snapshots in one session, reusing one cargo target dir.
# Input: a dataset holding manifest.txt (one "<id> <branch> <sha>" per line) and <id>.tgz snapshots.
# Output: /kaggle/working/results/<id>.result, one per snapshot (gate.txt format, starts with "<branch> <sha>").
import filecmp, glob, os, shutil, subprocess, time

W = "/kaggle/working"
RES = f"{W}/results"
SRC = "/tmp/src"          # every snapshot is synced into the SAME path so cargo can build incrementally
NEW = "/tmp/new"
os.makedirs(RES, exist_ok=True)


def sh(log, cmd, tail=60):
    t = time.time()
    r = subprocess.run(cmd, shell=True, capture_output=True, text=True, executable="/bin/bash")
    out = (r.stdout + r.stderr).splitlines()
    log.write(f"$ {cmd}\n  exit={r.returncode} {time.time() - t:.0f}s\n" + "\n".join(out[-tail:]) + "\n\n")
    log.flush()
    return r.returncode


def sync(new, dst):
    """Copy only files whose content changed (they get a fresh mtime); delete files that disappeared.

    git archive stamps every file with the commit time, so extracting straight over the old tree could leave a
    changed file *older* than the last build and cargo would silently skip it. Content comparison avoids that.
    """
    changed = 0
    for root, _, files in os.walk(new):
        rel = os.path.relpath(root, new)
        os.makedirs(os.path.join(dst, rel), exist_ok=True)
        for f in files:
            a, b = os.path.join(root, f), os.path.join(dst, rel, f)
            if not os.path.exists(b) or not filecmp.cmp(a, b, shallow=False):
                shutil.copyfile(a, b)
                changed += 1
    for root, _, files in os.walk(dst):
        rel = os.path.relpath(root, dst)
        for f in files:
            if not os.path.exists(os.path.join(new, rel, f)):
                os.remove(os.path.join(root, f))
    return changed


inp = glob.glob("/kaggle/input/**/manifest.txt", recursive=True)
setup = open(f"{RES}/_setup.log", "w")
if not inp:
    setup.write("NO manifest.txt\n")
    raise SystemExit(1)
base = os.path.dirname(inp[0])
sh(setup, "curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal", 3)
# emission models (PHEMlight5) read data files relative to SUMO_HOME; the oracle wheel ships them
sh("python -m pip install -q --target /tmp/sumo_pkg eclipse-sumo==1.28.0 2>&1 | tail -2; ls /tmp/sumo_pkg/sumo/data/emissions/PHEMlight5 | wc -l", 4)
env = f"export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/target CARGO_BUILD_JOBS=4 SCORE_OUT=/tmp/score SUMO_HOME=/tmp/sumo_pkg/sumo; cd {SRC};"

for line in open(inp[0]).read().split("\n"):
    if not line.strip():
        continue
    gid, branch, sha = line.split()[:3]
    log = open(f"{RES}/{gid}.result.tmp", "w")
    log.write(f"{branch} {sha}\n")
    shutil.rmtree(NEW, ignore_errors=True)
    os.makedirs(NEW)
    subprocess.run(f"tar -xzf {base}/{gid}.tgz -C {NEW}", shell=True)
    # scripts and *.args are committed from Windows with CRLF; Linux bash chokes on the \r
    subprocess.run(f"cd {NEW}/tools && find . -type f \\( -name '*.sh' -o -name '*.args' \\) -exec sed -i 's/\\r$//' {{}} +", shell=True)
    log.write(f"files changed vs previous snapshot: {sync(NEW, SRC)}\n\n")
    if sh(log, env + " cargo build -p sumo -p netconvert --message-format short 2>&1 | grep -E ': error|^error' | head -40; test -x /tmp/target/debug/sumo -a /tmp/target/debug/sumo -nt Cargo.toml", 45) != 0:
        log.write("BUILD FAILED\n")
    else:
        sh(log, "cd /tmp/target/debug && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe", 2)
        sh(log, env + " bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|PASS=|STALE|MISSING'", 40)
        sh(log, env + " bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1", 3)
        sh(log, env + " cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10", 12)
        log.write("GATE DONE\n")
    log.close()
    os.replace(f"{RES}/{gid}.result.tmp", f"{RES}/{gid}.result")
