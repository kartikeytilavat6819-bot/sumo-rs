"""Make small agent batches (~N todo!() each) from the files of the given big batches.

Usage: python tools/make_small_batches.py <size> <batch numbers...>  -> tools/batches/small_NNN.txt
Files with more than <size> todo!()s get a batch to themselves (agent works top-down).
"""
import os, sys
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
size, nums = int(sys.argv[1]), sys.argv[2:]
files = []
for n in nums:
    for line in open(os.path.join(ROOT, "tools", "batches", f"batch_{int(n):02d}.txt")):
        p = line.strip()
        if p:
            c = open(os.path.join(ROOT, p), encoding="utf8").read().count("todo!(")
            if c:
                files.append((p, c))
out_dir = os.path.join(ROOT, "tools", "batches")
start = 1 + sum(1 for f in os.listdir(out_dir) if f.startswith("small_"))
batches, cur, acc = [], [], 0
for p, c in files:
    if c >= size:
        batches.append([p]); continue
    cur.append(p); acc += c
    if acc >= size:
        batches.append(cur); cur, acc = [], 0
if cur:
    batches.append(cur)
for i, b in enumerate(batches, start):
    open(os.path.join(out_dir, f"small_{i:03d}.txt"), "w", newline="\n").write("\n".join(b) + "\n")
    print(f"small_{i:03d}: {', '.join(x.split('src/')[1] for x in b)}")
