"""Completeness check for C++ reference outputs (a truncated reference is not an oracle).

Usage:
  python check_ref_complete.py [--end SECONDS] [--tripinfo F] [--fcd F] [--stat F] [--xml F ...]

Checks, each failing with exit 1 and a message:
  * every given XML parses (a truncated file has no closing root tag);
  * --stat: its <performance end="..."> equals --end (when --end is given);
  * --fcd: its last <timestep time="..."> reaches the target: --end, else the largest
    arrival in --tripinfo. Tolerance: one second (the step length of every case).
"""

import argparse
import sys
import xml.etree.ElementTree as ET


def parse(path):
    try:
        return ET.parse(path).getroot()
    except ET.ParseError as e:
        raise SystemExit(f"INCOMPLETE {path}: {e}")


def time_seconds(v):
    """SUMO writes plain seconds, or H:MM:SS.ss / D:H:MM:SS.ss with -H."""
    if ":" not in v:
        return float(v)
    parts = v.split(":")
    if len(parts) == 4:  # D:HH:MM:SS
        return float(parts[0]) * 86400 + float(parts[1]) * 3600 + float(parts[2]) * 60 + float(parts[3])
    sec = 0.0
    for part in parts:
        sec = sec * 60 + float(part)
    return sec


def check(args):
    for x in args.xml:
        parse(x)
    if args.tripinfo:
        root = parse(args.tripinfo)
        arrivals = [time_seconds(t.get("arrival")) for t in root.iter("tripinfo") if t.get("arrival")]
    else:
        arrivals = []
    if args.stat:
        root = parse(args.stat)
        perf = [p for p in root.iter("performance")]
        if not perf:
            raise SystemExit(f"INCOMPLETE {args.stat}: no <performance> element")
        end = time_seconds(perf[0].get("end"))
        if args.end is not None and abs(end - args.end) > 1e-6:
            raise SystemExit(f"INCOMPLETE {args.stat}: end={end} but --end={args.end}")
    if args.fcd:
        root = parse(args.fcd)
        times = [time_seconds(t.get("time")) for t in root.iter("timestep")]
        if not times:
            raise SystemExit(f"INCOMPLETE {args.fcd}: no timesteps")
        target = args.end if args.end is not None else (max(arrivals) if arrivals else None)
        if target is not None and times[-1] < target - 1.0:
            raise SystemExit(f"INCOMPLETE {args.fcd}: last timestep {times[-1]} < {target}")
    print("COMPLETE")


def self_check():
    assert time_seconds("12.5") == 12.5
    assert time_seconds("00:00:12.50") == 12.5
    assert time_seconds("00:16:40.00") == 1000.0
    assert time_seconds("1:00:00:00.00") == 86400.0


def main(argv=None):
    self_check()
    p = argparse.ArgumentParser()
    p.add_argument("--end", type=float)
    p.add_argument("--tripinfo")
    p.add_argument("--fcd")
    p.add_argument("--stat")
    p.add_argument("--xml", nargs="*", default=[])
    check(p.parse_args(argv))
    return 0


if __name__ == "__main__":
    sys.exit(main())
