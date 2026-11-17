# stops_jump

Stops with `jump` and `jumpUntil` on a three-edge chain (`E0 E1 E2`, 300 m each, from `stops_parking`'s net).

- `j0`: `jump=20` on a lane stop at `E0_0` pos 250, then a plain stop on `E2_0`.
- `j1`: `jumpUntil=60` on a lane stop at `E1_0` pos 200.
- `j2`: `jump=10` on a lane stop at `E2_0` pos 100.

Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
