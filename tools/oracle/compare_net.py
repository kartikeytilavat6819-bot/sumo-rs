#!/usr/bin/env python3
"""Structurally compare two SUMO .net.xml files (formatting ignored).

Usage: compare_net.py REFERENCE.net.xml CANDIDATE.net.xml
Exit: 0 identical, 1 differences, 2 parse error.
"""
import argparse
import sys
import xml.etree.ElementTree as ET
from collections import OrderedDict

FLOAT_ATTRS = {"x", "y", "speed", "length", "width", "offset", "duration", "minDur", "maxDur"}
SHAPE_ATTRS = {"shape"}
MAX_REPORT = 50


def float_eq(a, b):
    return abs(a - b) <= 1e-6 + 1e-9 * max(abs(a), abs(b))


def parse_points(text):
    pts = []
    for p in text.split():
        x, y = p.split(",")
        pts.append((float(x), float(y)))
    return pts


def values_equal(attr, a, b):
    if attr in FLOAT_ATTRS:
        try:
            return float_eq(float(a), float(b))
        except ValueError:
            return a == b
    if attr in SHAPE_ATTRS:
        try:
            pa, pb = parse_points(a), parse_points(b)
        except ValueError:
            return a == b
        return len(pa) == len(pb) and all(
            float_eq(x1, x2) and float_eq(y1, y2) for (x1, y1), (x2, y2) in zip(pa, pb))
    return a == b


def add(records, etype, key, attrs):
    k = (etype, key)
    n = 1
    while k in records:  # duplicate ids: number them the same way on both sides
        n += 1
        k = (etype, f"{key}#{n}")
    records[k] = attrs


def collect(root):
    records = OrderedDict()
    for el in root:
        tag = el.tag
        if tag in ("junction", "node", "crossing", "walkingArea", "tlLogic"):
            add(records, tag, el.get("id", ""), dict(el.attrib))
            if tag == "tlLogic":
                for i, ph in enumerate(el.findall("phase")):
                    add(records, "phase", f"{el.get('id', '')}:{i}", dict(ph.attrib))
        elif tag == "edge":
            add(records, "edge", el.get("id", ""), dict(el.attrib))
            for lane in el.findall("lane"):
                add(records, "lane", lane.get("id", ""), dict(lane.attrib))
        elif tag == "connection":
            key = "%s->%s %s->%s" % (el.get("from", ""), el.get("to", ""),
                                     el.get("fromLane", ""), el.get("toLane", ""))
            add(records, "connection", key, dict(el.attrib))
    return records


def compare(ref, cand):
    diffs = []  # (etype, id, attr, ref_value, cand_value)
    for key, ra in ref.items():
        if key not in cand:
            diffs.append((*key, "-", "present", "absent"))
            continue
        ca = cand[key]
        for attr in sorted(set(ra) | set(ca)):
            if attr not in ra:
                diffs.append((*key, attr, "absent", ca[attr]))
            elif attr not in ca:
                diffs.append((*key, attr, ra[attr], "absent"))
            elif not values_equal(attr, ra[attr], ca[attr]):
                diffs.append((*key, attr, ra[attr], ca[attr]))
    for key in cand:
        if key not in ref:
            diffs.append((*key, "-", "absent", "present"))
    return diffs


def load(path):
    try:
        root = ET.parse(path).getroot()
    except (OSError, ET.ParseError) as e:
        print(f"parse error: {path}: {e}", file=sys.stderr)
        sys.exit(2)
    if root.tag != "net":
        print(f"parse error: {path}: root element is <{root.tag}>, expected <net>", file=sys.stderr)
        sys.exit(2)
    return root


def main():
    ap = argparse.ArgumentParser(description="Compare two SUMO .net.xml files structurally.")
    ap.add_argument("reference")
    ap.add_argument("candidate")
    args = ap.parse_args()

    ref_root = load(args.reference)
    cand_root = load(args.candidate)
    ref = collect(ref_root)
    cand = collect(cand_root)
    diffs = compare(ref, cand)

    groups = OrderedDict()
    for etype, eid, attr, rv, cv in diffs:
        groups.setdefault((etype, eid, attr), []).append((rv, cv))

    for i, ((etype, eid, attr), vals) in enumerate(groups.items()):
        if i >= MAX_REPORT:
            break
        rv, cv = vals[0]
        extra = f"  (x{len(vals)})" if len(vals) > 1 else ""
        print(f"{etype} {eid!r} {attr}: ref={rv!r} cand={cv!r}{extra}")
    if len(groups) > MAX_REPORT:
        print(f"... {len(groups) - MAX_REPORT} more groups not shown")

    print(f"SUMMARY: {len(diffs)} differences in {len(groups)} groups "
          f"(ref {len(ref)} elements, cand {len(cand)} elements)")
    return 1 if diffs else 0


if __name__ == "__main__":
    sys.exit(main())
