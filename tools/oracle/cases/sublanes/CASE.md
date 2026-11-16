# Sublanes case

Covers `--lateral-resolution 0.8` (sublane model). Before the fix, `g_lateral_resolution` returned -1, which
disabled every sublane foe check, so nothing exercised that path.

- Net: `input.osm` (hand-written: one 300 m oneway primary, 2 lanes), `ref_simple.net.xml` from C++ netconvert.
- Routes: `routes.rou` — 10 vehicles on edge `100`, mixed lanes, two slow vehicles (`s0`, `s1`) to force lateral moves.
- Flags: `sim.args` = `--end 120 --lateral-resolution 0.8`.
- Reference: `make_sim_refs.sh sublanes` (tripinfo, fcd, stat).
