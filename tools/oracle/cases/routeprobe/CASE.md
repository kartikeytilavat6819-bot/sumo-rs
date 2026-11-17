# routeprobe

`routeProbe` device on edge `E0` plus a `rerouter` on the same edge that switches vehicles to the
alternative route with `<routeProbReroute id="alt" probability="1"/>` at t=40.

Net (diamond): `E0 n0→n1`, then `E1 n1→n2→E3` (north) or `E2 n1→n3→E4` (south). All vehicles are
loaded with route `E0 E1 E3`; `alt` = `E0 E2 E4` is defined in the additional file (a route referenced
by `routeProbReroute` must be parsed before the rerouter sees it, so it cannot live in `routes.rou`).

Expected in `ref.tripinfo.xml`: `v0`/`v1` (pass `E0` before t=40) arrive on `E3_0`, `rerouteNo=0`;
`v2`…`v7` arrive on `E4_0`, `rerouteNo=1`.

`ref.probe.xml` is the C++ routeProbe output (`probe.xml` next to the additional file). run_all does
not compare it; it is committed as evidence that the device sampled the routes
(`<routeDistribution id="rp0_0.00">` with `E0 E1 E3`).

Sim: `--end 200 --additional-files .../routeprobe.add.xml`.
Refs: `make_sim_refs.sh routeprobe` (pristine C++ 1.28.0), net via `net_ref.sh`.
