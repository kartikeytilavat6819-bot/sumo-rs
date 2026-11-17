# Lane-change case

Covers lane changing on a multi-lane edge (`MSEdgeControl::change_lanes`, LC2013 path). Before the fix,
`gComputeLC` was never set, so this path never ran.

- Net: `input.osm` (one 300 m oneway primary, 2 lanes), `ref_simple.net.xml` from C++ netconvert.
- Routes: `routes.rou` — a slow `blocker` on lane 1 forces `chaser*` vehicles on lane 1 to change to lane 0; `lane0a` runs on lane 0.
- Flags: `sim.args` = `--end 120`.
