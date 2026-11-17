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
    # `devices` catches a whole defect class the numeric fields cannot see: a device that was
    # configured but never created leaves every number identical (silent no-op). The rerouting
    # device goes missing this way in reroute/ref_device.
    "devices",
)
# attributes of the <emissions> child of tripinfo (MSDevice_Emissions::generateOutput); compared as "emissions.<attr>"
TRIPINFO_EMISSION_FIELDS = tuple("emissions." + a for a in (
    "CO_abs", "CO2_abs", "HC_abs", "PMx_abs", "NOx_abs", "fuel_abs", "electricity_abs",
))
PERSON_FIELDS = ("depart", "duration", "waitingTime", "timeLoss", "traveltime", "speedFactor")
PERSON_STAGE_FIELDS = ("depart", "arrival", "arrivalPos", "duration", "routeLength", "timeLoss",
                       "waitingTime", "maxSpeed", "vehicle")
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
        attrs = dict(e.attrib)
        for child in e.findall("emissions"):
            for k, v in child.attrib.items():
                attrs["emissions." + k] = v
        out.setdefault(e.get("id"), []).append(attrs)
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
            for f, a, b in _field_diffs(ref_attrs, cand_attrs, TRIPINFO_FIELDS + TRIPINFO_EMISSION_FIELDS):
                return f"vehicle {vid} field {f}: ref={a!r} cand={b!r}"
    for vid in cand:
        if vid not in ref:
            return f"vehicle {vid} extra in candidate"
    return compare_persons(ref_path, cand_path)


def _persons(path):
    """id -> list of <personinfo> elements (the stages are their children)."""
    out = {}
    for e in ET.parse(path).getroot().iter("personinfo"):
        out.setdefault(e.get("id"), []).append(e)
    return out


def compare_persons(ref_path, cand_path):
    ref = _persons(ref_path)
    cand = _persons(cand_path)
    for pid, ref_list in ref.items():
        cand_list = cand.get(pid)
        if cand_list is None:
            return f"person {pid} missing in candidate"
        if len(ref_list) != len(cand_list):
            return f"person {pid} occurs {len(ref_list)}x in ref but {len(cand_list)}x in candidate"
        for ref_el, cand_el in zip(ref_list, cand_list):
            for f, a, b in _field_diffs(ref_el.attrib, cand_el.attrib, PERSON_FIELDS):
                return f"person {pid} field {f}: ref={a!r} cand={b!r}"
            ref_stages = list(ref_el)
            cand_stages = list(cand_el)
            if [s.tag for s in ref_stages] != [s.tag for s in cand_stages]:
                return (f"person {pid} stages: ref={[s.tag for s in ref_stages]} "
                        f"cand={[s.tag for s in cand_stages]}")
            for i, (rs, cs) in enumerate(zip(ref_stages, cand_stages)):
                for f, a, b in _field_diffs(rs.attrib, cs.attrib, PERSON_STAGE_FIELDS):
                    return f"person {pid} stage {i} ({rs.tag}) field {f}: ref={a!r} cand={b!r}"
    for pid in cand:
        if pid not in ref:
            return f"person {pid} extra in candidate"
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
