"""Per-record comparison of battery, emission, tripinfo, chargingstation outputs at full precision.

Usage: python compare_precision.py REF_PREFIX CAND_PREFIX
Compares REF_PREFIX.{battery,emission,tripinfo,chargingstations}.xml with the candidate files.
Numeric attributes match if |a-b| <= TOL * max(|a|, |b|, 1e-12); other attributes must be equal.
Exit 0 only if every output matches.
"""
import sys
import xml.etree.ElementTree as ET

TOL = 1e-6
SUFFIXES = ["battery", "emission", "tripinfo", "chargingstations"]


def records(path):
    out = []
    time = None
    for el in ET.parse(path).getroot().iter():
        if el.tag == "timestep":
            time = el.get("time")
        out.append((time, el.tag, dict(el.attrib)))
    return out


def num(s):
    try:
        return float(s)
    except (TypeError, ValueError):
        return None


def same(a, b):
    if a == b:
        return True
    fa, fb = num(a), num(b)
    if fa is None or fb is None:
        return False
    return abs(fa - fb) <= TOL * max(abs(fa), abs(fb), 1e-12)


def compare(name, ref_path, cand_path):
    ref, cand = records(ref_path), records(cand_path)
    if len(ref) != len(cand):
        print(f"{name}: FAIL record count ref={len(ref)} cand={len(cand)}")
        return 1
    for i, ((t1, g1, a1), (t2, g2, a2)) in enumerate(zip(ref, cand)):
        if t1 != t2 or g1 != g2 or set(a1) != set(a2):
            print(f"{name}: FAIL structure at record {i}: ref={(t1, g1, sorted(a1))} cand={(t2, g2, sorted(a2))}")
            return 1
        for k in a1:
            if not same(a1[k], a2[k]):
                print(f"{name}: FAIL record {i} time={t1} {g1} id={a1.get('id', '-')} {k}: ref={a1[k]} cand={a2[k]}")
                return 1
    print(f"{name}: MATCH ({len(ref)} elements, tol {TOL})")
    return 0


def main():
    ref_prefix, cand_prefix = sys.argv[1:3]
    fails = 0
    for s in SUFFIXES:
        fails += compare(s, f"{ref_prefix}.{s}.xml", f"{cand_prefix}.{s}.xml")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
