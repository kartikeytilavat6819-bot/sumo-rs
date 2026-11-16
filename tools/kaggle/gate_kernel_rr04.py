# rr04 diagnostic kernel: build the snapshot, then run the astar/CH variant sims directly (no run_all)
# and capture a gdb backtrace for each, so the stack overflow and the CH shortcut can be read from /kaggle/working.
import glob, os, subprocess, time

W = "/kaggle/working"
SRC = "/tmp/src"
LOG = open(f"{W}/gate.txt", "w")


def sh(cmd, tail=60):
    t = time.time()
    r = subprocess.run(cmd, shell=True, capture_output=True, text=True, executable="/bin/bash")
    out = (r.stdout + r.stderr).splitlines()
    LOG.write(f"$ {cmd[:300]}\n  exit={r.returncode} {time.time() - t:.0f}s\n")
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
env = f"export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/target CARGO_BUILD_JOBS=4; cd {SRC};"
if sh(env + " cargo build -p sumo --message-format short 2>&1 | grep -E '^(error|warning: unused)|: error' | head -40; test -x /tmp/target/debug/sumo", 45) != 0:
    LOG.write("BUILD FAILED\n")
    raise SystemExit(0)
sh("apt-get install -y -q gdb >/dev/null 2>&1; which gdb", 3)

CASES = [("reroute_alt", "astar"), ("reroute_alt", "ch"), ("reroute_alt", "chwrapper"),
         ("reroute_bypass", "astar"), ("reroute_bypass", "ch")]
for case, v in CASES:
    d = f"tools/oracle/cases/{case}"
    base = f"cd {SRC} && /tmp/target/debug/sumo -n {d}/ref_simple.net.xml -r {d}/routes.rou $(cat {d}/sim_{v}.args) --no-step-log true"
    sh(f"{base} --tripinfo-output {W}/{case}_{v}.tripinfo.xml > {W}/{case}_{v}.stdout 2>&1", 10)
    sh(f"cd {SRC} && gdb -batch -ex run -ex 'bt 25' --args /tmp/target/debug/sumo -n {d}/ref_simple.net.xml -r {d}/routes.rou $(cat {d}/sim_{v}.args) --no-step-log true > {W}/{case}_{v}.gdb.txt 2>&1; grep -m1 -E 'SIG|exited' {W}/{case}_{v}.gdb.txt", 3)
LOG.write("GATE DONE\n")
