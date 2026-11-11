"""Split the sumo_core skeleton into N batches of roughly equal todo!() count for parallel agents.

Files keep a priority order (product pipeline: network import/build -> simulation -> TraCI -> rest)
and each batch is a contiguous run of that order, so related files stay together.
Usage: python tools/make_batches.py 30   -> writes tools/batches/batch_01.txt ...
"""
import os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(ROOT, "crates", "sumo_core", "src")
PRIORITY = ["utils/vehicle", "utils/router", "utils/emissions", "utils/shapes", "utils/handlers",
            "netbuild", "netimport", "netwrite", "netload", "microsim/cfmodels", "microsim/lcmodels", "microsim",
            "microsim/traffic_lights", "microsim/devices", "microsim/output", "microsim/trigger",
            "microsim/transportables", "microsim/actions", "microsim/logging", "microsim/engine",
            "libsumo", "traci_server", "mesosim", "router", "duarouter", "od", "jtrrouter", "marouter",
            "dfrouter", "polyconvert", "netgen", "activitygen"]

def rank(rel):
    d = os.path.dirname(rel).replace("\\", "/")
    best = max((p for p in PRIORITY if d == p or d.startswith(p + "/")), key=len, default=None)
    return (PRIORITY.index(best) if best else len(PRIORITY), rel)

files = []
for d, _, fs in os.walk(SRC):
    for f in fs:
        if f.endswith(".rs") and f not in ("mod.rs", "lib.rs", "prelude.rs"):
            p = os.path.join(d, f)
            n = open(p, encoding="utf8").read().count("todo!(")
            if n:
                files.append((os.path.relpath(p, ROOT).replace("\\", "/"), n))
files.sort(key=lambda x: rank(os.path.relpath(x[0], "crates/sumo_core/src")))
n_batches = int(sys.argv[1]) if len(sys.argv) > 1 else 30
total = sum(n for _, n in files)
target = total / n_batches
out = os.path.join(ROOT, "tools", "batches")
os.makedirs(out, exist_ok=True)
for f in os.listdir(out):
    os.remove(os.path.join(out, f))
batch, acc, idx = [], 0, 1
for i, (p, n) in enumerate(files):
    batch.append(p); acc += n
    remaining_files = len(files) - i - 1
    if (acc >= target * idx and idx < n_batches) or remaining_files == 0:
        open(os.path.join(out, f"batch_{idx:02d}.txt"), "w", newline="\n").write("\n".join(batch) + "\n")
        print(f"batch_{idx:02d}: {len(batch):3d} files, {sum(m for q, m in files if q in set(batch)):4d} todos, first {batch[0].split('src/')[1]}")
        batch, idx = [], idx + 1
print(f"total {total} todos in {len(files)} files")
