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
    """SUMO writes times >1 day as D:HH:MM:SS; plain seconds otherwise."""
    if ":" not in v:
        return float(v)
    sec = 0.0
    for part in v.split(":"):
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
        end = float(perf[0].get("end"))
        if args.end is not None and abs(end - args.end) > 1e-6:
            raise SystemExit(f"INCOMPLETE {args.stat}: end={end} but --end={args.end}")
    if args.fcd:
        root = parse(args.fcd)
        times = [float(t.get("time")) for t in root.iter("timestep")]
        if not times:
            raise SystemExit(f"INCOMPLETE {args.fcd}: no timesteps")
        target = args.end if args.end is not None else (max(arrivals) if arrivals else None)
        if target is not None and times[-1] < target - 1.0:
            raise SystemExit(f"INCOMPLETE {args.fcd}: last timestep {times[-1]} < {target}")
    print("COMPLETE")


def main(argv=None):
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
