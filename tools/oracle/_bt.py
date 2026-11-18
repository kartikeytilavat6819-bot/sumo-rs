import os, subprocess, shlex
REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
d = os.path.join(REPO, "tools/oracle/cases/tt_output_meandata_edgebased_teleporting_across")
flags = shlex.split(open(os.path.join(d, "sim.args"), encoding="utf-8").read().strip())
cmd = [r"C:\Users\karti\sumo-rs-target\debug\sumo.exe",
       "-n", os.path.relpath(os.path.join(d, "ref_simple.net.xml"), REPO),
       "-r", os.path.relpath(os.path.join(d, "routes.rou"), REPO),
       "--tripinfo-output", "/tmp/ti.xml", "--fcd-output", "/tmp/fc.xml",
       "--statistic-output", "/tmp/st.xml", "--no-step-log", "true"] + flags
env = dict(os.environ); env["RUST_BACKTRACE"] = "full"
r = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True, env=env)
print("RC", r.returncode)
print(r.stderr[:12000])
