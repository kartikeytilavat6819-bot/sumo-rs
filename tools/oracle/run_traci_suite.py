#!/usr/bin/env python3
"""TraCI test-client oracle.

For every official TraCI test, run the OFFICIAL TraCITestClient against (a) our
sumo and (b) the pristine C++ sumo, then compare the normalized outputs.  The
only normalization mirrors tests/traci/config.traci: timestamps, version
banners, socket/broken-pipe chatter and timing numbers.

Prints one line per failing test plus a final
    traci suite oracle: PASS=x of y
and writes the full result table to $TRACI_SUITE_OUT/results.tsv.
"""
import difflib
import os
import re
import shlex
import shutil
import socket
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed

HERE = os.path.dirname(os.path.abspath(__file__))
TARGET = os.environ.get("CARGO_TARGET_DIR", "/tmp/target")
OUR_SUMO = os.environ.get("OUR_SUMO", os.path.join(TARGET, "debug", "sumo"))
PRISTINE = os.environ.get("SUMO_PRISTINE_BIN", "/usr/bin")
OFF_SUMO = os.environ.get("OFFICIAL_SUMO", os.path.join(PRISTINE, "sumo"))
CLIENT = os.environ.get("TRACI_CLIENT", os.path.join(PRISTINE, "TraCITestClient"))
OUT = os.environ.get("TRACI_SUITE_OUT", "/tmp/traci_suite")
LIMIT = int(os.environ.get("TRACI_LIMIT", "0"))
ONLY = os.environ.get("TRACI_ONLY", "")
JOBS = int(os.environ.get("TRACI_JOBS", "4"))
KILL_AFTER = int(os.environ.get("TRACI_KILL_AFTER", "40"))

COPY_NAMES = [
    "net.net.xml", "net2.net.xml", "three_split.net.xml", "routes.rou.xml",
    "additional.xml", "testclient.prog", "input_additional.add.xml",
    "input_additional2.add.xml", "input_routes.rou.xml",
]
EXTRA_OUTPUTS = [
    "netstate_dump.xml", "simstate.xml", "detector.xml", "tls_state.xml",
    "vehroutes.xml", "tripinfos.xml", "fcd.xml", "lanechanges.xml", "log.txt",
]


def find_tests():
    candidates = [
        os.environ.get("TRACI_TESTS", ""),
        os.path.join(os.environ.get("SUMO_HOME", ""), "tests", "traci"),
        os.path.join(PRISTINE, "..", "tests", "traci"),
        os.path.join(HERE, "traci_inputs"),
    ]
    for c in candidates:
        if c and os.path.isfile(os.path.join(c, "options.traci")):
            return os.path.abspath(c)
    return None


def free_port():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind(("127.0.0.1", 0))
    port = s.getsockname()[1]
    s.close()
    return port


def prepare(run_dir, test_dir, root):
    """TextTest-style: files from the test dir win, ancestors fill the gaps."""
    os.makedirs(run_dir, exist_ok=True)
    chain = []
    d = os.path.abspath(test_dir)
    r = os.path.abspath(root)
    while d.startswith(r):
        chain.append(d)
        if d == r:
            break
        d = os.path.dirname(d)
    chain.reverse()
    for d in chain:
        for name in os.listdir(d):
            src = os.path.join(d, name)
            if not os.path.isfile(src):
                continue
            dst = os.path.join(run_dir, name)
            if os.path.splitext(name)[1].lower() in (".prog", ".traci", ".xml", ".txt", ".sumo", ".cfg", ".netgcfg", ".netccfg"):
                with open(src, "r", encoding="utf-8", errors="replace") as f:
                    data = f.read().replace("\r\n", "\n")
                with open(dst, "w", encoding="utf-8", newline="\n") as f:
                    f.write(data)
            else:
                shutil.copyfile(src, dst)


def norm_client(text):
    out = []
    for ln in text.splitlines():
        if ln.startswith("TraCITestClient output file. Date:"):
            ln = "TraCITestClient output file. Date: <DATE>"
        ln = re.sub(r"(getVersion: \d+, SUMO).*", r"\1", ln)
        if "Socket" in ln or "Broken pipe" in ln or "Connection reset" in ln:
            continue
        ln = re.sub(r"\d+\.\d+s(?:\b|$)", "(TIME)", ln)
        out.append(ln.rstrip())
    return "\n".join(out)


def norm_log(text):
    out = []
    for ln in text.splitlines():
        if "Eclipse SUMO" in ln or "Build features" in ln:
            continue
        if "TraCIAPI is deprecated" in ln:
            continue
        ln = re.sub(r"\d+\.\d+ms", "(TIME)", ln)
        ln = re.sub(r"\d+\.\d+s", "(TIME)", ln)
        ln = re.sub(r"Starting server on port \d+", "Starting server on port <PORT>", ln)
        ln = re.sub(r"output:port \d+", "output:port <PORT>", ln)
        out.append(ln.rstrip())
    return "\n".join(out)


def norm_xml(text):
    out = []
    for ln in text.splitlines():
        if ln.lstrip().startswith("<!--"):
            continue
        out.append(ln.rstrip())
    return "\n".join(out)


def read(path):
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as f:
            return f.read()
    except OSError:
        return ""


def run_server(binary, options, run_dir, port, tag):
    stdout = os.path.join(run_dir, "stdout_" + tag)
    stderr = os.path.join(run_dir, "stderr_" + tag)
    fo = open(stdout, "w")
    fe = open(stderr, "w")
    try:
        p = subprocess.Popen(
            [binary] + shlex.split(options) + ["--remote-port", str(port)],
            cwd=run_dir, stdout=fo, stderr=fe)
    except OSError as e:
        fo.close(); fe.close()
        return None, str(e), stdout, stderr
    return p, None, stdout, stderr


def run_client(run_dir, port, tag):
    out = os.path.join(run_dir, "testclient_out.txt")
    if os.path.exists(out):
        os.remove(out)
    args = [CLIENT, "-def", "testclient.prog", "-o", out, "-p", str(port)]
    result = 2
    for retry in range(10):
        time.sleep(retry * 0.1)
        try:
            result = subprocess.call(
                args, cwd=run_dir,
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                timeout=KILL_AFTER)
        except subprocess.TimeoutExpired:
            result = -1
            break
        if result != 2:
            break
    return out, result


def run_variant(binary, options, test_dir, root, variant_dir, tag):
    prepare(variant_dir, test_dir, root)
    port = free_port()
    p, err, stdout, stderr = run_server(binary, options, variant_dir, port, tag)
    client_out, rc = run_client(variant_dir, port, tag)
    if p is None:
        data = {"error": err}
    else:
        try:
            p.wait(timeout=KILL_AFTER)
        except subprocess.TimeoutExpired:
            p.kill()
            p.wait()
    parts = {}
    parts["client"] = read(client_out)
    parts["stdout"] = read(stdout)
    parts["stderr"] = read(stderr)
    for name in EXTRA_OUTPUTS:
        path = os.path.join(variant_dir, name)
        if os.path.exists(path):
            parts[name] = read(path)
    return parts, rc


def compare(test_dir, root, work):
    rel = os.path.relpath(test_dir, root)
    options = read(os.path.join(test_dir, "options.traci")).strip()
    options = " ".join(options.splitlines())
    ours_dir = os.path.join(work, "ours", rel)
    off_dir = os.path.join(work, "off", rel)
    ours, rc1 = run_variant(OUR_SUMO, options, test_dir, root, ours_dir, "ours")
    off, rc2 = run_variant(OFF_SUMO, options, test_dir, root, off_dir, "off")
    keys = sorted(set(ours) | set(off))
    diffs = []
    for k in keys:
        if k == "error":
            continue
        if k == "client":
            a, b = norm_client(ours.get(k, "")), norm_client(off.get(k, ""))
        elif k in ("stdout", "stderr"):
            a, b = norm_log(ours.get(k, "")), norm_log(off.get(k, ""))
        else:
            a, b = norm_xml(ours.get(k, "")), norm_xml(off.get(k, ""))
        if a != b:
            d = list(difflib.unified_diff(
                b.splitlines(), a.splitlines(),
                fromfile="official/" + k, tofile="ours/" + k, lineterm=""))
            diffs.append("\n".join(d[:25]))
    if "error" in ours or "error" in off:
        diffs.append("error: ours=%r off=%r" % (ours.get("error"), off.get("error")))
    if rc1 != rc2:
        diffs.append("client rc: ours=%s off=%s" % (rc1, rc2))
    return rel, diffs


def main():
    root = find_tests()
    if not root:
        print("traci suite oracle: NO TESTS FOUND")
        return 2
    if not os.path.isfile(CLIENT):
        print("traci suite oracle: NO CLIENT at " + CLIENT)
        return 2
    os.makedirs(OUT, exist_ok=True)
    work = os.path.join(OUT, "work")
    shutil.rmtree(work, ignore_errors=True)
    tests = []
    for d, dirs, files in os.walk(root):
        if "testclient.prog" in files and "options.traci" in files:
            tests.append(d)
    tests.sort()
    if ONLY:
        pats = ONLY.split(",")
        tests = [t for t in tests if any(p in t for p in pats)]
    if LIMIT:
        tests = tests[:LIMIT]
    results = []
    with ThreadPoolExecutor(max_workers=JOBS) as ex:
        futures = {ex.submit(compare, t, root, work): t for t in tests}
        for fut in as_completed(futures):
            try:
                rel, diffs = fut.result()
            except Exception as e:  # noqa: BLE001 - report, don't crash the suite
                rel = os.path.relpath(futures[fut], root)
                diffs = ["EXCEPTION: %r" % (e,)]
            results.append((rel, diffs))
    results.sort()
    tsv = os.path.join(OUT, "results.tsv")
    with open(tsv, "w", encoding="utf-8") as f:
        for rel, diffs in results:
            f.write("%s\t%s\n" % (rel, "PASS" if not diffs else "FAIL"))
    npass = sum(1 for _, d in results if not d)
    show = int(os.environ.get("TRACI_SHOW", "80"))
    nshown = 0
    for rel, diffs in results:
        if diffs and nshown < show:
            print("FAIL %s" % rel)
            print("\n".join("    " + ln for ln in "\n".join(diffs).splitlines()[:8]))
            nshown += 1
    print("traci suite oracle: PASS=%d of %d" % (npass, len(results)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
