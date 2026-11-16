# dv02 copy of gate_kernel.py: same build, gate and retry logic, plus the output cases.
# It runs our sumo and returns the candidate XML under /kaggle/working/out (no C++ here: compare locally).
import glob, os, shlex, subprocess, time

W = "/kaggle/working"
SRC = "/tmp/src"
OUT = f"{W}/out"
OURS = "/tmp/target/debug/sumo"
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
sh(f"cd {SRC}/tools && find . -type f \\( -name '*.sh' -o -name '*.args' \\) -exec sed -i 's/\\r$//' {{}} +", 3)
sh("curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal", 3)
env = f"export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/target CARGO_BUILD_JOBS=4 SCORE_OUT=/tmp/score; cd {SRC};"
if sh(env + " cargo build -p sumo -p netconvert --message-format short 2>&1 | grep -E '^(error|warning: unused)|: error' | head -40; test -x /tmp/target/debug/sumo", 45) != 0:
    LOG.write("BUILD FAILED\n")
    raise SystemExit(0)
sh("cd /tmp/target/debug && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe", 2)
sh(env + " bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|PASS=|STALE|MISSING'", 40)
sh(env + " bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1", 3)
sh(env + " cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10", 12)

# output cases: each run writes into OUT/<tag>/, the log of the run is OUT/<tag>/run.log
os.makedirs(OUT, exist_ok=True)


def run(tag, args, outs):
    d = f"{OUT}/{tag}"
    os.makedirs(d, exist_ok=True)
    cmd = [OURS, *args, "--no-step-log", "true"]
    r = subprocess.run(cmd, cwd=SRC, capture_output=True, text=True)
    text = r.stdout + r.stderr
    with open(f"{d}/run.log", "w") as fh:
        fh.write(text)
    panic = next((l for l in text.splitlines() if "panicked at" in l), "")
    sizes = {o: os.path.getsize(f"{d}/{o}") if os.path.exists(f"{d}/{o}") else -1 for o in outs}
    LOG.write(f"RUN {tag} rc={r.returncode} sizes={sizes} panic={panic[:160]}\n")
    LOG.flush()


def sim(case):
    p = f"{SRC}/tools/oracle/cases/{case}/sim.args"
    return shlex.split(open(p).read()) if os.path.exists(p) else []


ev = "tools/oracle/cases/ev"
run("ev", ["-n", f"{ev}/ref_simple.net.xml", "-r", f"{ev}/routes.rou", *sim("ev"),
           "--battery-output", f"{OUT}/ev/battery.xml",
           "--chargingstations-output", f"{OUT}/ev/chargingstations.xml",
           "--tripinfo-output", f"{OUT}/ev/tripinfo.xml",
           "--fcd-output", f"{OUT}/ev/fcd.xml",
           "--statistic-output", f"{OUT}/ev/stat.xml"],
    ["battery.xml", "chargingstations.xml", "tripinfo.xml", "fcd.xml", "stat.xml"])

for c in ["real", "signal", "stress"]:
    base = ["-n", f"tools/oracle/cases/{c}/ref_simple.net.xml", "-r", f"tools/oracle/cases/{c}/routes.rou", *sim(c)]
    run(f"{c}_vehroute", base + ["--vehroute-output", f"{OUT}/{c}_vehroute/vehroute.xml"], ["vehroute.xml"])
    run(f"{c}_vehroute_exit", base + ["--vehroute-output", f"{OUT}/{c}_vehroute_exit/vehroute.xml",
                                       "--vehroute-output.exit-times", "true"], ["vehroute.xml"])

LOG.write("GATE DONE\n")
