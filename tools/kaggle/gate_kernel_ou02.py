# Copy of gate_kernel.py for agent ou02: same build and gates, plus the output-file oracle
# (tools/oracle/run_outputs.sh) against the C++ references committed under tools/oracle/cases/<case>/ref_out/.
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
# scripts, *.args and reference outputs are committed from Windows with CRLF; Linux bash chokes on the \r
sh(f"cd {SRC}/tools && find . -type f \\( -name '*.sh' -o -name '*.args' \\) -exec sed -i 's/\\r$//' {{}} +", 3)
sh("curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal", 3)
env = f"export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/target CARGO_BUILD_JOBS=4 SCORE_OUT=/tmp/score; cd {SRC};"
if sh(env + " cargo build -p sumo -p netconvert --message-format short 2>&1 | grep -E '^(error|warning: unused)|: error' | head -40; test -x /tmp/target/debug/sumo", 45) != 0:
    LOG.write("BUILD FAILED\n")
    raise SystemExit(0)
# run_all.sh and the depart runner call sumo.exe / netconvert.exe
sh("cd /tmp/target/debug && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe", 2)
sh(env + " bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|PASS=|STALE|MISSING'", 40)
sh(env + " bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1", 3)
sh(env + " OUT=/tmp/score/outputs BIN=/tmp/target/debug/sumo REFS=committed bash tools/oracle/run_outputs.sh 2>&1 | tail -40", 45)
sh("cat /tmp/score/outputs/results.tsv", 60)
sh(env + " cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10", 12)
# debug probe (not committed): dump the vehicle list the lane noise sum iterates, for the lane that differs in `full`
DBG_LANE = "397795464#1_0"
p = f"{SRC}/crates/sumo_core/src/microsim/ms_lane.rs"
s = open(p).read()
nl = "\r\n" if "\r\n" in s else "\n"
anchor = "        let vehs = self.get_vehicles_secure();" + nl + "        if vehs.len() == 0 {"
dbg = nl.join([
    f'        if self.get_id() == "{DBG_LANE}" {{',
    '            eprintln!("DBG t={} lane={} myVehicles={} partial={} tmp={}", steps2time(MSNet::get_instance().get_current_time_step()), self.get_id(), self.myVehicles.len(), self.myPartialVehicles.len(), self.myTmpVehicles.len());',
    '            for v in self.myVehicles.iter() { eprintln!("DBG  veh={} pos={} speed={} noise={}", v.get_id(), v.get_position_on_lane(), v.get_speed(), v.get_harmonoise_noise_emissions()); }',
    '            for v in self.myPartialVehicles.iter() { eprintln!("DBG  PARTIAL veh={} pos={} noise={}", v.get_id(), v.get_position_on_lane(), v.get_harmonoise_noise_emissions()); }',
    '            for v in self.myTmpVehicles.iter() { eprintln!("DBG  TMP veh={} noise={}", v.get_id(), v.get_harmonoise_noise_emissions()); }',
    '        }',
    '',
])
if anchor not in s:
    LOG.write("DBG anchor not found\n")
else:
    open(p, "w").write(s.replace(anchor, dbg + anchor, 1))
    sh(env + " cargo build -p sumo --message-format short 2>&1 | grep -E ': error' | head -20", 20)
    sh(f"cd {SRC} && /tmp/target/debug/sumo -n tools/oracle/cases/real/ref_simple.net.xml -r tools/oracle/cases/real/routes.rou --full-output /tmp/score/dbg_full.xml --no-step-log true 2> {W}/noise_dbg.txt >/dev/null; grep -c DBG {W}/noise_dbg.txt; cp /tmp/score/dbg_full.xml {W}/dbg_full.xml", 5)
LOG.write("GATE DONE\n")
