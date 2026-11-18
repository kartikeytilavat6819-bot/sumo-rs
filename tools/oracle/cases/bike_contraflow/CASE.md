# bike_contraflow

Bicycles riding against the direction of a one-way street: cars use `E0`
(a0->a1, passenger-only lane), while bikes use the parallel contra-flow lane
`E1` (a1->a0, bicycle-only). Tests that a bicycle-only reverse lane is usable
and that opposing car/bike traffic never conflicts. FCD positions and tripinfo
routing are the hooks.

- Net: plain XML node/edge files, see `net.args`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_contraflow`.
