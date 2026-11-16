"""Element-by-element comparator for SUMO XML outputs (summary, queue, netstate, vehroute, ...).

Usage:
  python compare_xml.py REF CAND [--tol T]

Exit status: 0 MATCH, 1 DIFF (first difference printed with its element path), 2 unreadable input.

Rules:
  - Comments and processing instructions are dropped (generator header, config echo).
  - <performance> elements are skipped, and so is the wall-clock step time @duration on <step>
    (summary output): both measure real time, not simulation state.
  - Attribute values and text are compared as exact strings. --tol T only relaxes values that
    both parse as float (|a-b| <= T); by default there is no numeric tolerance.
  - Element order, child count and attribute sets must match.
"""

import argparse
import sys
import xml.etree.ElementTree as ET

IGNORED_TAGS = {"performance"}
IGNORED_ATTRS = {("step", "duration")}


def _children(el):
    return [c for c in el if c.tag not in IGNORED_TAGS]


def _attrs(el):
    return {k: v for k, v in el.attrib.items() if (el.tag, k) not in IGNORED_ATTRS}


def _value_equal(a, b, tol):
    if a == b:
        return True
    if tol is None:
        return False
    try:
        return abs(float(a) - float(b)) <= tol
    except ValueError:
        return False


def compare(ref, cand, tol=None, path=""):
    here = f"{path}/{ref.tag}"
    if ref.tag != cand.tag:
        return f"{path}: tag ref={ref.tag} cand={cand.tag}"
    ra, ca = _attrs(ref), _attrs(cand)
    if sorted(ra) != sorted(ca):
        return f"{here}: attributes ref={sorted(ra)} cand={sorted(ca)}"
    for k in ra:
        if not _value_equal(ra[k], ca[k], tol):
            return f"{here} @{k}: ref={ra[k]!r} cand={ca[k]!r}"
    if (ref.text or "").strip() != (cand.text or "").strip():
        return f"{here} text: ref={(ref.text or '').strip()!r} cand={(cand.text or '').strip()!r}"
    rc, cc = _children(ref), _children(cand)
    if len(rc) != len(cc):
        return f"{here}: child count ref={len(rc)} cand={len(cc)}"
    for a, b in zip(rc, cc):
        d = compare(a, b, tol, here)
        if d is not None:
            return d
    return None


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("ref")
    p.add_argument("cand")
    p.add_argument("--tol", type=float, default=None)
    args = p.parse_args(argv)
    try:
        ref = ET.parse(args.ref).getroot()
        cand = ET.parse(args.cand).getroot()
    except (OSError, ET.ParseError) as e:
        print(f"UNREADABLE: {e}")
        return 2
    diff = compare(ref, cand, args.tol)
    if diff is None:
        print("MATCH")
        return 0
    print(f"DIFF: {diff}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
