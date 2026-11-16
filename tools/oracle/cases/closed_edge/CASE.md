# Closed-edge enforcement case

Tests whether a closed lane blocks traffic that is already committed to it, not whether routing avoids it at
departure.

- Net: `input.osm`/`ref_simple.net.xml` shared with `reroute_alt` (edges 10 → {11 top, 12 bottom} → 13).
- Closure: `closed_lane.add.xml` — `<closingLaneReroute id="11_0" disallow="all"/>` on rerouter `edges="10"`,
  active from t=5 (after `v0` departs and while it is still on edge 10).
- Routes: `v0` on `10 11 13` (committed to the closed lane), `v2` on `10 12 13` (control, unaffected).
- No rerouting device: nothing reroutes at departure, so any avoidance must come from the closure itself.
- Flags: `sim.args` = `--end 200 --additional-files tools/oracle/cases/closed_edge/closed_lane.add.xml`.
