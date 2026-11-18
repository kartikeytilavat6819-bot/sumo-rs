#!/usr/bin/env python3
"""Compare the official and our `sumo` binary on tests/complex/traci runner.py scripts.

Each runner is executed twice in identical sandboxes (only SUMO_BINARY differs).
stdout+stderr, generated output files and console diagnostics are normalized for
timings, dynamic ports, absolute paths and generated-on metadata, then compared.

Env: COMPLEX_OUT (results root), COMPLEX_FILTER (test-path substring),
COMPLEX_LIMIT (max tests), COMPLEX_TIMEOUT (seconds per run, default 45).
"""
import difflib
import os
import pathlib
import re
import shutil
import signal
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
BUNDLE = HERE / "complex_bundle"
OUT = pathlib.Path(os.environ.get("COMPLEX_OUT", "/tmp/complex_oracle"))
FILTER = os.environ.get("COMPLEX_FILTER", "")
LIMIT = int(os.environ.get("COMPLEX_LIMIT", "0"))
TIMEOUT = int(os.environ.get("COMPLEX_TIMEOUT", "45"))
OURS = os.environ.get("CARGO_TARGET_DIR", "/tmp/target") + "/debug/sumo"
PRISTINE = os.environ.get("SUMO_PRISTINE_BIN", "/tmp/sumo_pkg/sumo/bin")
OFFICIAL = PRISTINE + "/sumo"

SKIP = re.compile(r"sumo-gui|traci\.gui|libsumo|libtraci|matplotlib|numpy|scipy|pandas|webservice|GUISIM_BINARY", re.I)
TEXT_EXT = {".xml", ".txt", ".csv", ".log", ".sumocfg", ".py", ".rou", ".net", ".add", ".cfg"}


def normalize(data: bytes, paths: list[str]) -> bytes:
    try:
        s = data.decode("utf-8")
    except UnicodeDecodeError:
        return data
    for p in paths:
        s = s.replace(p, "<PATH>")
    s = re.sub(r"Starting server on port \d+", "Starting server on port <PORT>", s)
    s = re.sub(r"<remote-port value=\"\d+\"/>", '<remote-port value="<PORT>"/>', s)
    s = re.sub(r"\b\d+\s?ms\b", "<TIME> ms", s)
    s = re.sub(r"Real time factor: [0-9.]+", "Real time factor: <RATE>", s)
    s = re.sub(r"UPS: [0-9.]+", "UPS: <RATE>", s)
    s = re.sub(r"UPS-Persons: [0-9.]+", "UPS-Persons: <RATE>", s)
    s = re.sub(r'vehicleUpdatesPerSecond="[^"]*"', 'vehicleUpdatesPerSecond="<RATE>"', s)
    s = re.sub(r"Duration: [0-9.]+s", "Duration: <TIME>", s)
    s = re.sub(r"<!--\s*generated on .*?-->", "<!-- generated on <DATE> -->", s, flags=re.S)
    s = re.sub(r"<performance\s+[^>]*/>", "<performance/>", s)
    s = re.sub(r'(\bversion=")[^"]*(")', r"\1<VERSION>\2", s)
    return s.encode("utf-8")


def snapshot(sandbox: pathlib.Path, paths: list[str]) -> dict[str, bytes]:
    out = {}
    for f in sorted(sandbox.rglob("*")):
        if not f.is_file():
            continue
        rel = str(f.relative_to(sandbox))
        data = f.read_bytes()
        if f.suffix.lower() in TEXT_EXT or f.name == "console.txt":
            data = normalize(data, paths)
        out[rel] = data
    return out


def run_one(src: pathlib.Path, sandbox: pathlib.Path, binary: str, nc: str, side: str):
    if sandbox.exists():
        shutil.rmtree(sandbox)
    shutil.copytree(src, sandbox)
    env = dict(os.environ)
    env["SUMO_BINARY"] = binary
    env["NETCONVERT_BINARY"] = nc
    env["PYTHONPATH"] = ":".join(filter(None, [os.environ.get("SUMO_HOME", "") + "/tools", env.get("PYTHONPATH", "")]))
    with open(sandbox / "console.txt", "wb") as log:
        try:
            p = subprocess.Popen(
                [sys.executable, "runner.py"], cwd=sandbox, env=env,
                stdout=log, stderr=subprocess.STDOUT, start_new_session=True,
            )
            try:
                rc = p.wait(timeout=TIMEOUT)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid, signal.SIGKILL)
                p.wait()
                log.write(f"\n[oracle: {side} timed out after {TIMEOUT}s]\n".encode())
                rc = 124
        except Exception as exc:  # noqa: BLE001 - a broken sandbox is a test failure
            log.write(f"\n[oracle: launch failed: {exc}]\n".encode())
            rc = 127
        log.flush()
    return rc


def diff_text(a: dict[str, bytes], b: dict[str, bytes]) -> str:
    chunks = []
    for k in sorted(set(a) | set(b)):
        av, bv = a.get(k), b.get(k)
        if av == bv:
            continue
        if av is None:
            chunks.append(f"=== {k}: only in ours")
            continue
        if bv is None:
            chunks.append(f"=== {k}: only in official")
            continue
        al = av.decode("utf-8", "replace").splitlines()
        bl = bv.decode("utf-8", "replace").splitlines()
        if al == bl:
            continue
        chunks.append(f"=== {k}")
        chunks.extend(list(difflib.unified_diff(al, bl, "official", "ours", lineterm="", n=0))[:40])
    return "\n".join(chunks)


def first_diff(a: dict[str, bytes], b: dict[str, bytes]) -> str:
    d = diff_text(a, b)
    if not d:
        return ""
    lines = [ln for ln in d.splitlines() if ln.startswith(("+", "-", "===")) and not ln.startswith(("+++", "---"))]
    return " | ".join(lines[:5])[:400]


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    nc = OFFICIAL.replace("/sumo", "/netconvert")
    if not os.path.exists(nc):
        nc = os.environ.get("CARGO_TARGET_DIR", "/tmp/target") + "/debug/netconvert"
    tests = sorted(p.parent for p in BUNDLE.rglob("runner.py"))
    tests = [t for t in tests if not SKIP.search((t / "runner.py").read_text(encoding="utf-8", errors="replace"))]
    if FILTER:
        tests = [t for t in tests if FILTER in str(t.relative_to(BUNDLE))]
    if LIMIT:
        tests = tests[:LIMIT]
    passed = 0
    fails = []
    for t in tests:
        name = str(t.relative_to(BUNDLE)).replace("\\", "/")
        base = OUT / name
        for side, binary in (("official", OFFICIAL), ("ours", OURS)):
            run_one(t, base / side, binary, nc, side)
        paths = [str(base / "official"), str(base / "ours"), OURS, OFFICIAL, PRISTINE,
                 os.environ.get("SUMO_HOME", "/tmp/sumo_pkg/sumo")]
        a = snapshot(base / "official", paths)
        b = snapshot(base / "ours", paths)
        d = first_diff(a, b)
        if d:
            (base / "diff.txt").write_text(diff_text(a, b), encoding="utf-8")
            fails.append((name, d))
            print(f"FAIL {name} | {d}", flush=True)
        else:
            passed += 1
            print(f"PASS {name}", flush=True)
    print(f"complex suite oracle: PASS={passed} of {len(tests)}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
