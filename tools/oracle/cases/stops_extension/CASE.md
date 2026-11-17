# stops_extension

Stops with `until` + `extension` on the `stops_parking` three-edge chain, and a too-short bus stop.

- `e0`: lane stop at `E1_0` pos 200, `until=20 extension=15`.
- `e1`: lane stop at `E2_0` pos 150, `until=40 extension=10`, 3 s duration.
- `s0`: `busStop="short"` (E1_0, 10–11 m, 1 m long). Vehicle length is 5 m, so C++ warns once:
  `busStop 'short' on lane 'E1_0' is too short for vehicle 's0'.` (stderr only; `run_all` does not compare it).

Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
