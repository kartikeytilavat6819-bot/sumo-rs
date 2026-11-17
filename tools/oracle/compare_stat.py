#!/usr/bin/env python3
"""Structurally compare two SUMO --statistic-output XML files (formatting ignored).

Usage: compare_stat.py REFERENCE.xml CANDIDATE.xml
Exit: 0 identical, 1 differences, 2 parse error (bad XML or root not <statistics>).
<performance> holds wall-clock timings and is skipped.
"""
import argparse
import sys
import xml.etree.ElementTree as ET
from collections import OrderedDict

MAX_REPORT = 50
SKIP_TAGS = {"performance"}


def float_eq(a, b):
    return a == b or abs(a - b) <= 1e-6 + 1e-9 * max(abs(a), abs(b))


def values_equal(a, b):
    try:
        return float_eq(float(a), float(b))
    except ValueError:
        return a == b


def collect(root):
    records = OrderedDict()
    for el in root:
        if el.tag in SKIP_TAGS:
            continue
        key = (el.tag, el.get("class", ""))
        n = 1
        while key in records:  # duplicate keys: number them the same way on both sides
            n += 1
            key = (el.tag, f"{el.get('class', '')}#{n}")
        records[key] = dict(el.attrib)
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
            elif not values_equal(ra[attr], ca[attr]):
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
    if root.tag != "statistics":
        print(f"parse error: {path}: root element is <{root.tag}>, expected <statistics>", file=sys.stderr)
        sys.exit(2)
    return root


def main():
    ap = argparse.ArgumentParser(description="Compare two SUMO --statistic-output XML files.")
    ap.add_argument("reference")
    ap.add_argument("candidate")
    args = ap.parse_args()

    ref = collect(load(args.reference))
    cand = collect(load(args.candidate))
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
