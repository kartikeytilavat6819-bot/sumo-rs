"""Exact comparison of battery-output and chargingstations-output (and tripinfo <battery> elements).

Usage: python compare_ev.py REF.battery.xml CAND.battery.xml REF.chargingstations.xml CAND.chargingstations.xml
Exit 0 only if every timestep vehicle record and every charging-station record matches exactly.
"""
import sys
import xml.etree.ElementTree as ET


def battery_records(path):
    recs = []
    for ts in ET.parse(path).getroot().iter("timestep"):
        for v in ts.iter("vehicle"):
            recs.append((ts.get("time"), tuple(sorted(v.attrib.items()))))
    return recs


def station_records(path):
    recs = []
    for cs in ET.parse(path).getroot().iter("chargingStation"):
        recs.append(("station", tuple(sorted(cs.attrib.items()))))
        for v in cs.iter("vehicle"):
            recs.append(("vehicle", tuple(sorted(v.attrib.items()))))
    return recs


def diff(name, ref, cand):
    if ref == cand:
        print(f"{name}: MATCH ({len(ref)} records)")
        return 0
    print(f"{name}: FAIL ref={len(ref)} cand={len(cand)}")
    for i, (a, b) in enumerate(zip(ref, cand)):
        if a != b:
            print(f"  first diff at record {i}:\n    ref ={a}\n    cand={b}")
            break
    else:
        print(f"  common prefix equal; extra records in {'ref' if len(ref) > len(cand) else 'cand'}")
    return 1


def main():
    rb, cb, rc, cc = sys.argv[1:5]
    fails = diff("battery", battery_records(rb), battery_records(cb))
    fails += diff("chargingstations", station_records(rc), station_records(cc))
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
