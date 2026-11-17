# routedist

Diamond net: `E0` splits at `n1` into `UP1 UP2` or `DN1 DN2`, rejoins at `n2`, then `E3`.

- Flow `fDist`: `route="rd"` (routeDistribution, `rUp` 0.70 / `rDn` 0.30) and `type="vd"` (vTypeDistribution, `vdCar` 0.50 / `vdBus` 0.50).
- Vehicles `dep1` (`departEdge=2`), `arr1` (`arrivalEdge=2`), `both1` (`departEdge=1 arrivalEdge=3`) on route `rFull`.

Sim: `--end 150 --seed 9`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
