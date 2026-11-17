# ft07_flows

Flows on the `cfmodels_new` net (`ref_simple.net.xml`, copied): `vehsPerHour` flow, `probability` flow, `period="exp(0.5)"` flow, and one vehicle with `insertionChecks="none"`. `--seed 42`, `--end 200`.

- Base (`ref.*`): no extra options.
- `scale`: `--scale 0.5`.
- `ignore`: `--ignore-route-errors` with `badroute.add.xml` (a vehicle whose route is disconnected).

Refs: `bash tools/oracle/make_sim_refs.sh ft07_flows` (C++ 1.28.0 from ~/sumo-pristine, WSL). `NEW_CASE` marks a new case.
