Route validation on the `junction` net (`ref_simple.net.xml` copied from `cases/junction`).

- `vbad`: route `-103224#0 -103227#1` is disconnected (`-103224#0` ends at `-137720`, `-103227#1` starts at `-137717`, no connection between them).
- `vgood`: route `-103227#0 -103227#1 -103227#2`, valid.

Reference: C++ SUMO 1.28.0, `sumo -n ref_simple.net.xml -r routes.rou --tripinfo-output ... --fcd-output ... --statistic-output ... --no-step-log true`, no sim.args. See REPORT.md for what C++ does with the bad route.
