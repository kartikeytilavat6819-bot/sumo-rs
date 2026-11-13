"""Lockstep oracle comparator for SUMO outputs (tripinfo.xml, fcd.xml).

Usage:
  python compare_sim.py --tripinfo REF.xml CAND.xml
  python compare_sim.py --fcd REF.xml CAND.xml

Exit status 0 when identical within tolerance, 1 on the first difference.
"""

import argparse
import sys
import xml.etree.ElementTree as ET
from itertools import zip_longest

ABS_TOL = 1e-6
REL_TOL = 1e-9

TRIPINFO_FIELDS = (
    "depart", "departLane", "departSpeed", "arrival", "arrivalLane",
    "arrivalSpeed", "duration", "routeLength", "waitingTime", "timeLoss",
)
FCD_FIELDS = ("x", "y", "angle", "speed", "lane", "pos")


def values_equal(a, b):
    if a is None or b is None:
        return a is b
    try:
        fa = float(a)
        fb = float(b)
    except ValueError:
        return a == b
    if fa == fb:
        return True
    return abs(fa - fb) <= ABS_TOL + REL_TOL * max(abs(fa), abs(fb))


def _field_diffs(ref_attrs, cand_attrs, fields):
    for f in fields:
        a, b = ref_attrs.get(f), cand_attrs.get(f)
        if not values_equal(a, b):
            yield f, a, b


def _tripinfos(path):
    """id -> list of attribute dicts. A list, not a dict: SUMO can emit the same
    vehicle id twice (reinserted/teleported vehicles, or repeated departures),
    and silently keeping only the last one would hide a real divergence."""
    out = {}
    for e in ET.parse(path).getroot().iter("tripinfo"):
        out.setdefault(e.get("id"), []).append(dict(e.attrib))
    return out


def compare_tripinfo(ref_path, cand_path):
    """Return the first difference as a string, or None if the files match."""
    ref = _tripinfos(ref_path)
    cand = _tripinfos(cand_path)
    for vid, ref_list in ref.items():
        cand_list = cand.get(vid)
        if cand_list is None:
            return f"vehicle {vid} missing in candidate"
        if len(ref_list) != len(cand_list):
            return (f"vehicle {vid} occurs {len(ref_list)}x in ref but "
                    f"{len(cand_list)}x in candidate")
        for ref_attrs, cand_attrs in zip(ref_list, cand_list):
            for f, a, b in _field_diffs(ref_attrs, cand_attrs, TRIPINFO_FIELDS):
                return f"vehicle {vid} field {f}: ref={a!r} cand={b!r}"
    for vid in cand:
        if vid not in ref:
            return f"vehicle {vid} extra in candidate"
    return None


def _timesteps(path):
    for _, el in ET.iterparse(path, events=("end",)):
        if el.tag == "timestep":
            vehicles = [(v.get("id"), dict(v.attrib)) for v in el if v.tag == "vehicle"]
            yield el.get("time"), vehicles
            el.clear()


def compare_fcd(ref_path, cand_path):
    """Stream both files in lockstep; return the first differing timestep/vehicle, or None."""
    for ref_step, cand_step in zip_longest(_timesteps(ref_path), _timesteps(cand_path), fillvalue=(None, None)):
        ta, va = ref_step
        tb, vb = cand_step
        if ta is None:
            return f"extra timestep in candidate at time {tb}"
        if tb is None:
            return f"missing timestep in candidate at time {ta}"
        if not values_equal(ta, tb):
            return f"timestep time differs: ref={ta} cand={tb}"
        da = dict(va)
        db = dict(vb)
        for vid in dict.fromkeys([i for i, _ in va] + [i for i, _ in vb]):
            a, b = da.get(vid), db.get(vid)
            if a is None:
                return f"t={ta} vehicle {vid} extra in candidate"
            if b is None:
                return f"t={ta} vehicle {vid} missing in candidate"
            for f, x, y in _field_diffs(a, b, FCD_FIELDS):
                return f"t={ta} vehicle {vid} field {f}: ref={x!r} cand={y!r}"
    return None


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    g = p.add_mutually_exclusive_group(required=True)
    g.add_argument("--tripinfo", action="store_true")
    g.add_argument("--fcd", action="store_true")
    p.add_argument("ref")
    p.add_argument("cand")
    args = p.parse_args(argv)
    fn = compare_tripinfo if args.tripinfo else compare_fcd
    diff = fn(args.ref, args.cand)
    if diff is None:
        print("MATCH")
        return 0
    print(f"DIFF: {diff}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
