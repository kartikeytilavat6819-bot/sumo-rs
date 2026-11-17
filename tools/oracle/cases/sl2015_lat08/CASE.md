# sl2015_lat08 (SL2015 stress, lateral resolution 0.8)

Covers the SL2015 sublane model at `--lateral-resolution 0.8` with bicycles and motorcycles sharing the
three lanes with cars. Found by lc03 (SL2015 audit); the audited SL2015 code paths are updateExpectedSublaneSpeeds,
checkBlocking, keepLatGap, computeSpeedLat, commitManoeuvre, and the latAlignment switch in _wantsChangeSublane.

- Net: `input.osm` (one 300 m oneway primary, 3 lanes), `ref_simple.net.xml` from pristine netconvert.
- Routes: `routes.rou`. 28 vehicles. vTypes vary lcSublane, lcPushy, lcAssertive, lcImpatience, lcAccelLat
  and latAlignment (nice, compact, arbitrary, left, center, right). Slow vehicles force lateral moves.
- Flags: `sim.args` = `--end 120 --lateral-resolution 0.8`.
- Reference: `make_sim_refs.sh sl2015_lat08` (tripinfo, fcd, stat), pristine SUMO 1.28.0.
