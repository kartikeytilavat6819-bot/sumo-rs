# Oracle: net.xml comparator

`compare_net.py` checks a candidate `.net.xml` (our port) against a reference from C++ SUMO.
It ignores formatting and compares junctions, edges, lanes, connections, tlLogic phases,
crossings and walking areas by id and attribute.

Usage: `python tools/oracle/compare_net.py REFERENCE.net.xml CANDIDATE.net.xml`

Floats (x, y, speed, length, offset, duration, ...) use `abs(a-b) <= 1e-6 + 1e-9*max(|a|,|b|)`;
`shape` is compared point by point with the same tolerance; everything else is an exact string match.
Output: one line per (element, id, attribute) group, at most 50 groups, then a SUMMARY line.
Exit codes: 0 identical, 1 differences, 2 parse error (bad XML or root not `<net>`).
Python 3.11+, stdlib only.
