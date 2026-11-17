#!/usr/bin/env python3
# Round-trip checks.
#   compare.py tripinfo <straight.ti.xml> <load.ti.xml> <t>
#       vehicles running at t (depart < t < arrival) in the straight run must appear in the loaded run with identical attributes
#   compare.py state <ours.xml> <reference.xml>
#       state bodies must be identical; the <?xml ...?> and <snapshot ...> header lines are ignored
import difflib
import sys
import xml.etree.ElementTree as ET


def tripinfo_by_id(path):
    return {el.get("id"): dict(el.attrib) for el in ET.parse(path).getroot().iter("tripinfo")}


def check_tripinfo(straight_path, load_path, t):
    straight = tripinfo_by_id(straight_path)
    loaded = tripinfo_by_id(load_path)
    running = [i for i, a in straight.items() if float(a["depart"]) < t < float(a["arrival"])]
    bad = []
    for i in sorted(running):
        if i not in loaded:
            bad.append(f"{i}: missing in loaded run")
        elif loaded[i] != straight[i]:
            keys = sorted(set(straight[i]) | set(loaded[i]))
            diff = [k for k in keys if straight[i].get(k) != loaded[i].get(k)]
            bad.append(f"{i}: differs in {diff}")
    print(f"running at t={t}: {len(running)}  loaded: {len(loaded)}  mismatches: {len(bad)}")
    for line in bad[:20]:
        print("  " + line)
    print("RESULT " + ("IDENTICAL" if running and not bad else "DIFFERENT" if bad else "NO VEHICLES RUNNING AT t"))
    return 0 if running and not bad else 1


def check_loads(straight_path, cpp_load_path, our_load_path, t):
    # C++ itself does not reproduce the straight run after loading (timeLoss/arrival drift), so the bar is C++ behaviour
    running = [i for i, a in tripinfo_by_id(straight_path).items() if float(a["depart"]) < t < float(a["arrival"])]
    cpp = tripinfo_by_id(cpp_load_path)
    ours = tripinfo_by_id(our_load_path)
    bad = []
    for i in sorted(running):
        if i not in ours:
            bad.append(f"{i}: missing in our loaded run")
        elif ours[i] != cpp.get(i):
            keys = sorted(set(cpp.get(i, {})) | set(ours[i]))
            bad.append(f"{i}: differs from C++ in {[k for k in keys if cpp.get(i, {}).get(k) != ours[i].get(k)]}")
    print(f"running at t={t}: {len(running)}  mismatches vs C++ load: {len(bad)}")
    for line in bad[:20]:
        print("  " + line)
    print("RESULT " + ("IDENTICAL" if running and not bad else "DIFFERENT" if bad else "NO VEHICLES RUNNING AT t"))
    return 0 if running and not bad else 1


def state_body(path):
    # drop the XML header, the <snapshot> tag and the <configuration> block (paths, output names and timestamps)
    body, in_config = [], False
    for l in open(path, encoding="utf-8").read().splitlines():
        s = l.strip()
        if s.startswith(("<configuration", "<sumoConfiguration")):
            in_config = True
        if in_config:
            in_config = not s.startswith(("</configuration", "</sumoConfiguration"))
            continue
        if s.startswith("<?xml") or s.startswith("<!--") or s.startswith("<snapshot"):
            continue
        body.append(l)
    return body


def check_state(ours_path, ref_path):
    a, b = state_body(ours_path), state_body(ref_path)
    diff = list(difflib.unified_diff(b, a, "reference", "ours", lineterm="", n=0))
    print(f"state lines: ours={len(a)} reference={len(b)} differing_lines={sum(1 for l in diff if l[:1] in '+-' and l[:3] not in ('+++', '---'))}")
    for line in diff[:20]:
        print(line)
    print("RESULT " + ("IDENTICAL" if not diff else "DIFFERENT"))
    return 0 if not diff else 1


if __name__ == "__main__":
    if sys.argv[1] == "tripinfo":
        sys.exit(check_tripinfo(sys.argv[2], sys.argv[3], float(sys.argv[4])))
    if sys.argv[1] == "loads":
        sys.exit(check_loads(sys.argv[2], sys.argv[3], sys.argv[4], float(sys.argv[5])))
    sys.exit(check_state(sys.argv[2], sys.argv[3]))
