# st01 round-trip kernel: build the branch, run tools/oracle/cases/state_roundtrip/roundtrip.sh, copy outputs.
import glob, os, shutil, subprocess, time

W = "/kaggle/working"
SRC = "/tmp/src"
OUT = f"{W}/rt"
LOG = open(f"{W}/roundtrip.txt", "w")


def sh(cmd, tail=60):
    t = time.time()
    r = subprocess.run(cmd, shell=True, capture_output=True, text=True, executable="/bin/bash")
    LOG.write(f"$ {cmd}\n  exit={r.returncode} {time.time() - t:.0f}s\n")
    LOG.write("\n".join((r.stdout + r.stderr).splitlines()[-tail:]) + "\n\n")
    LOG.flush()
    return r


tgz = glob.glob("/kaggle/input/**/src.tgz", recursive=True)
if not tgz:
    LOG.write("NO src.tgz in /kaggle/input\n")
    raise SystemExit(1)
os.makedirs(SRC, exist_ok=True)
sh(f"tar -xzf {tgz[0]} -C {SRC} && cat {SRC}/GATE_BRANCH.txt", 5)
sh(f"cd {SRC}/tools && find . -type f \\( -name '*.sh' -o -name '*.args' \\) -exec sed -i 's/\\r$//' {{}} +", 3)
sh("curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal", 3)
env = f"export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/target CARGO_BUILD_JOBS=4; cd {SRC};"
if sh(env + " cargo build -p sumo -p netconvert 2>&1 | grep -E '^error' | head -20; test -x /tmp/target/debug/sumo", 45).returncode != 0:
    LOG.write("BUILD FAILED\n")
    raise SystemExit(0)
os.makedirs(OUT, exist_ok=True)
sh(env + f" bash tools/oracle/cases/state_roundtrip/roundtrip.sh /tmp/target/debug {OUT}", 80)
shutil.copy(f"{SRC}/GATE_BRANCH.txt", f"{W}/GATE_BRANCH.txt")
LOG.write("ROUNDTRIP DONE\n")
