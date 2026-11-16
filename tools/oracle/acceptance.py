"""Tolerance-aware acceptance report.

The exact comparator (compare_sim/compare_stat) is the correctness oracle: any difference is a
difference. This script answers the *product* question instead: "is the difference big enough to
matter?" It grades each case PASS (exact) / CLOSE (within tolerance) / FAIL (beyond tolerance).

Tolerances (deliberately conservative, and stated here so they can be argued with):
  - arrival / duration / departDelay: 1.0 s absolute
  - speeds: 0.5 m/s absolute
  - distances (routeLength): 5 % relative
  - statistics aggregates: 10 % relative

Usage: python tools/oracle/acceptance.py CASE_DIR CAND_TRIPINFO CAND_STAT [REF_PREFIX]
"""
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

TOL_ABS = {"arrival": 1.0, "duration": 1.0, "departDelay": 1.0, "arrivalSpeed": 0.5,
           "waitingTime": 1.0, "timeLoss": 1.0, "departSpeed": 0.5}
TOL_REL = {"routeLength": 0.05}
TOL_STAT_REL = 0.10


def _f(v):
    try:
        return float(v)
    except (TypeError, ValueError):
        return None


def tripinfos(path):
    out = {}
    for e in ET.parse(path).getroot().iter("tripinfo"):
        out.setdefault(e.get("id"), []).append(dict(e.attrib))
    return out


def compare(ref_path, cand_path):
    ref, cand = tripinfos(ref_path), tripinfos(cand_path)
    worst, detail = 0.0, []
    for vid in sorted(set(ref) | set(cand)):
        if vid not in cand:
            return "FAIL", f"{vid} missing", 9e9
        if vid not in ref:
            return "FAIL", f"{vid} extra", 9e9
        for ra, ca in zip(ref[vid], cand[vid]):
            for key, tol in TOL_ABS.items():
                a, b = _f(ra.get(key)), _f(ca.get(key))
                if a is None or b is None:
                    continue
                d = abs(a - b)
                if d > tol:
                    detail.append(f"{vid}.{key} {a} vs {b} (>{tol})")
                worst = max(worst, d / tol)
            for key, rel in TOL_REL.items():
                a, b = _f(ra.get(key)), _f(ca.get(key))
                if not a:
                    continue
                d = abs(a - b) / abs(a)
                if d > rel:
                    detail.append(f"{vid}.{key} {a} vs {b} (>{rel:.0%})")
                worst = max(worst, d / rel)
    return ("PASS" if worst == 0 else ("CLOSE" if not detail else "FAIL")), "; ".join(detail[:3]), worst


def compare_stat(ref_path, cand_path):
    ref = ET.parse(ref_path).getroot()
    cand = ET.parse(cand_path).getroot()
    worst, detail = 0.0, []
    for rel in ref:
        if rel.tag == "performance":
            # wall-clock / throughput of the machine, not the simulation: realTimeFactor,
            # clockDuration and vehicleUpdatesPerSecond differ by nature and must not grade the port
            continue
        c = cand.find(rel.tag)
        if c is None:
            detail.append(f"{rel.tag} missing")
            continue
        for k, v in rel.attrib.items():
            a, b = _f(v), _f(c.get(k))
            if a is None or b is None:
                continue
            if a == 0 and b == 0:
                continue
            d = abs(a - b) / max(abs(a), 1e-9)
            if d > TOL_STAT_REL:
                detail.append(f"{rel.tag}.{k} {a} vs {b} (>{TOL_STAT_REL:.0%})")
            worst = max(worst, d / TOL_STAT_REL)
    return ("PASS" if worst == 0 else ("CLOSE" if not detail else "FAIL")), "; ".join(detail[:3]), worst


def main():
    if len(sys.argv) < 4:
        print(__doc__)
        return 2
    case, cand_ti, cand_stat = sys.argv[1], sys.argv[2], sys.argv[3]
    cd = Path("tools/oracle/cases") / case
    v1, d1, _ = compare(cd / "ref.tripinfo.xml", cand_ti)
    v2, d2, _ = compare_stat(cd / "ref.stat.xml", cand_stat) if (cd / "ref.stat.xml").exists() else ("-", "", 0)
    verdict = "PASS" if v1 == "PASS" and v2 in ("PASS", "-") else ("FAIL" if "FAIL" in (v1, v2) else "CLOSE")
    print(f"{case:<14} tripinfo={v1:<5} statistics={v2:<5} => {verdict}")
    for d in (d1, d2):
        if d:
            print(f"    {d}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
