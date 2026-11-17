# Runs on a Kaggle CPU notebook (4 cores, ~31 GB RAM): build ours and install C++ SUMO on the same machine,
# then time both on the same inputs. Writes bench.txt (table + log) and results.json to /kaggle/working.
import glob, json, os, re, shutil, statistics, subprocess, sys, time

W = "/kaggle/working"
SRC = "/tmp/src"
B = "/tmp/bench"
LOG = open(f"{W}/bench.txt", "w")
REPS = 3
END = "3600"
TIMEOUT = 1500
RES = {"scenarios": {}, "traci": {}, "perf": []}


def say(s):
    LOG.write(s + "\n")
    LOG.flush()


def sh(cmd, tail=60):
    t = time.time()
    r = subprocess.run(cmd, shell=True, capture_output=True, text=True, executable="/bin/bash")
    out = (r.stdout + r.stderr).splitlines()
    say(f"$ {cmd[:200]}\n  exit={r.returncode} {time.time() - t:.0f}s")
    say("\n".join(out[-tail:]) + "\n")
    return r.returncode


def hms(s):
    t = 0.0
    for p in s.split(":"):
        t = t * 60 + float(p)
    return t


def run(binary, args, cwd):
    # /usr/bin/time -v: wall clock and peak RSS of the child process
    t = time.perf_counter()
    try:
        r = subprocess.run(["/usr/bin/time", "-v", binary] + args, cwd=cwd, capture_output=True, text=True, timeout=TIMEOUT)
    except subprocess.TimeoutExpired:
        return {"ok": False, "why": "TIMEOUT"}
    el = re.search(r"Elapsed \(wall clock\) time.*?: (\S+)", r.stderr)
    rss = re.search(r"Maximum resident set size \(kbytes\): (\d+)", r.stderr)
    wall = hms(el.group(1)) if el else time.perf_counter() - t
    ok = r.returncode == 0
    tail = " | ".join(r.stderr.strip().splitlines()[-3:])[:200]
    return {"ok": ok, "wall": wall, "rss_mb": int(rss.group(1)) / 1024 if rss else None,
            "why": "" if ok else f"exit={r.returncode} {tail}", "stdout": r.stdout}


tgz = glob.glob("/kaggle/input/**/src.tgz", recursive=True)
if not tgz:
    say("NO src.tgz in /kaggle/input")
    raise SystemExit(1)
os.makedirs(SRC, exist_ok=True)
sh(f"tar -xzf {tgz[0]} -C {SRC} && cat {SRC}/GATE_BRANCH.txt", 5)

# 1. build ours (release, same machine as the C++ run)
sh("curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal", 3)
ENV = f"export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/target CARGO_BUILD_JOBS=4; cd {SRC};"
if sh(ENV + " cargo build --release -p sumo -p netconvert --message-format short 2>&1 | grep -E '^error|: error' | head -40; test -x /tmp/target/release/sumo", 45) != 0:
    say("BUILD FAILED")
    raise SystemExit(0)
OURS = "/tmp/target/release/sumo"
sh(ENV + " /tmp/target/release/sumo --version 2>&1 | head -2", 3)

# 2. C++ SUMO from PyPI (eclipse-sumo 1.28.0 = the version the oracle references were made with)
sh("apt-get install -y -q time >/dev/null 2>&1; ls -l /usr/bin/time", 3)
sh("pip install -q eclipse-sumo==1.28.0 traci==1.28.0 sumolib==1.28.0 || pip install -q eclipse-sumo traci sumolib", 10)
out = subprocess.run([sys.executable, "-m", "pip", "show", "eclipse-sumo"], capture_output=True, text=True).stdout
loc = re.search(r"Location: (.*)", out).group(1).strip()
SH = os.path.join(loc, "sumo")
THEIRS = os.path.join(SH, "bin", "sumo")
ver = subprocess.run([THEIRS, "--version"], capture_output=True, text=True).stdout.strip().splitlines()[0]
RES["cpp_version"] = ver
say(f"C++ SUMO: {ver} ({THEIRS})")
if not os.path.isfile(THEIRS):
    say("C++ SUMO NOT FOUND")
    raise SystemExit(0)

# 3. inputs: three committed cases plus a scaled-up demand generated with C++ randomTrips.py
CASES = f"{SRC}/tools/oracle/cases"
SCEN = {
    "city_plano": (f"{CASES}/city_plano/ref_simple.net.xml", f"{CASES}/city_plano/routes.rou"),
    "city_munich": (f"{CASES}/city_munich/ref_simple.net.xml", f"{CASES}/city_munich/routes.rou"),
    "stress": (f"{CASES}/stress/ref_simple.net.xml", f"{CASES}/stress/routes.rou"),
}
os.makedirs(f"{B}/scaled_plano", exist_ok=True)
rt = os.path.join(SH, "tools", "randomTrips.py")
if not os.path.isfile(rt):
    rt = (glob.glob(f"{loc}/**/randomTrips.py", recursive=True) or [""])[0]
env = dict(os.environ, SUMO_HOME=SH, PATH=f"{SH}/bin:" + os.environ["PATH"])
rr = subprocess.run([sys.executable, rt, "-n", SCEN["city_plano"][0], "-e", END, "-p", "0.5", "--seed", "1",
                     "-r", f"{B}/scaled_plano/routes.rou.xml"], cwd=f"{B}/scaled_plano", env=env, capture_output=True, text=True)
say(f"randomTrips.py exit={rr.returncode} {rr.stderr.strip().splitlines()[-1:] if rr.stderr.strip() else ''}")
SCEN["scaled_plano"] = (SCEN["city_plano"][0], f"{B}/scaled_plano/routes.rou.xml")
n_veh = sum(1 for _ in open(SCEN["scaled_plano"][1]) if "<vehicle " in _)
say(f"scaled_plano: {n_veh} vehicles")
RES["scaled_vehicles"] = n_veh

# 4. timed runs: no outputs; and with --fcd-output /dev/null. Median of REPS for wall, max RSS.
for name, (net, route) in SCEN.items():
    d = f"{B}/{name}"
    os.makedirs(d, exist_ok=True)
    sc = RES["scenarios"][name] = {}
    base = ["-n", net, "-r", route, "--end", END, "--no-step-log", "true"]
    for bname, binary in [("ours", OURS), ("cpp", THEIRS)]:
        # untimed run with statistic output: vehicle count and SUMO's own performance clock
        st = run(binary, base + ["--statistic-output", f"{d}/stat_{bname}.xml"], d)
        txt = open(f"{d}/stat_{bname}.xml").read() if os.path.exists(f"{d}/stat_{bname}.xml") else ""
        m = re.search(r'<vehicles loaded="(\d+)" inserted="(\d+)"', txt)
        inserted = int(m.group(2)) if m else None
        sc[bname] = {"stat_ok": st["ok"], "inserted": inserted, "why": st["why"]}
        for cfg, extra in [("bare", []), ("fcd_null", ["--fcd-output", "/dev/null"])]:
            runs = [run(binary, base + extra, d) for _ in range(REPS)]
            bad = [r["why"] for r in runs if not r["ok"]]
            walls = [r["wall"] for r in runs if r["ok"]]
            rss = [r["rss_mb"] for r in runs if r["ok"] and r["rss_mb"] is not None]
            med = statistics.median(walls) if walls else None
            sc[bname][cfg] = {
                "wall_s": med,
                "steps_s": int(END) / med if med else None,
                "veh_s": inserted / med if med and inserted else None,
                "rss_mb": max(rss) if rss else None,
                "fail": bad[:1],
            }
            say(f"{name} {bname} {cfg}: {sc[bname][cfg]}")
    say("")

# 5. TraCI: 1000 x (simulationStep + getIDList + getPosition for every vehicle), scaled demand
import traci  # noqa: E402 (installed above)

net, route = SCEN["scaled_plano"]
for bname, binary in [("ours", OURS), ("cpp", THEIRS)]:
    port = 8813 if bname == "ours" else 8814
    lg = open(f"{B}/traci_{bname}.log", "w")
    proc = subprocess.Popen([binary, "-n", net, "-r", route, "--remote-port", str(port), "--no-step-log", "true"],
                            cwd=f"{B}/scaled_plano", stdout=lg, stderr=subprocess.STDOUT)
    try:
        for _ in range(120):
            try:
                traci.init(port)
                break
            except Exception:
                time.sleep(0.5)
        t0 = time.perf_counter()
        active = 0
        for _ in range(1000):
            traci.simulationStep()
            ids = traci.vehicle.getIDList()
            active += len(ids)
            for v in ids:
                traci.vehicle.getPosition(v)
        dt = time.perf_counter() - t0
        traci.close()
        RES["traci"][bname] = {"ms_step": dt * 1000 / 1000, "avg_active_veh": active / 1000}
        say(f"TraCI {bname}: {dt * 1000 / 1000:.2f} ms/step, avg active {active / 1000:.0f}")
    except Exception as e:
        RES["traci"][bname] = {"error": repr(e)[:200]}
        say(f"TraCI {bname} FAILED: {e!r}")
    finally:
        proc.kill()
        lg.close()

# 6. hottest functions of ours, only if perf exists on this image
if shutil.which("perf"):
    sh(f"cd {B}/scaled_plano && perf record -F 499 -o perf.data -- {OURS} -n {net} -r {route} --end 1200 "
       f"--no-step-log true >/dev/null 2>&1; perf report -i perf.data --stdio --no-children -g none 2>/dev/null "
       f"| grep -E '^ +[0-9.]+%' | head -3 > {B}/perf_top3.txt; cat {B}/perf_top3.txt", 10)
    RES["perf"] = open(f"{B}/perf_top3.txt").read().splitlines()
else:
    say("perf: not available on this image, skipped")

# 7. table
say("| scenario | config | ours wall s | C++ wall s | ours/C++ | ours steps/s | C++ steps/s | ours veh/s | C++ veh/s | ours RSS MB | C++ RSS MB |")
say("|---|---|---|---|---|---|---|---|---|---|---|")
for name, sc in RES["scenarios"].items():
    for cfg in ["bare", "fcd_null"]:
        o, c = sc["ours"][cfg], sc["cpp"][cfg]
        f = lambda v, p=2: "fail" if v is None else f"{v:.{p}f}"
        ratio = f(o["wall_s"] / c["wall_s"], 2) if o["wall_s"] and c["wall_s"] else "n/a"
        say(f"| {name} | {cfg} | {f(o['wall_s'])} | {f(c['wall_s'])} | {ratio} | {f(o['steps_s'], 1)} | "
            f"{f(c['steps_s'], 1)} | {f(o['veh_s'], 1)} | {f(c['veh_s'], 1)} | {f(o['rss_mb'], 0)} | {f(c['rss_mb'], 0)} |")
json.dump(RES, open(f"{W}/results.json", "w"), indent=1)
say("BENCH DONE")
