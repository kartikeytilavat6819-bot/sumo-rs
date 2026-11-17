# mapmatch_xy_sim

Two 1-lane edges in a line: `A` (0,0)→(400,0) and `B` (400,0)→(400,300). Three `<trip>` elements use `fromXY`, `viaXY` and `toXY` positions off the centre line, mapped to edges with `--mapmatch.distance 50` (MSRouteHandler parseGeoEdges / MapMatcher).

- `ref` (`sim.args`): `--end 60 --seed 11 --mapmatch.distance 50`.
- Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh` for the net, `make_sim_refs.sh` for the outputs).
