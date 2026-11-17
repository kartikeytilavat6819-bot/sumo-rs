# Case: city_manhattan_sublane (Midtown Manhattan grid, sublane model)

Same net and routes as `city_manhattan_midtown` (right-hand traffic). The only difference is
`sim.args` = `--end 600 --lateral-resolution 0.8`, which enables the sublane lane-change model.

Reference: C++ sumo 1.28.0 from `~/sumo-pristine` (WSL), via `tools/oracle/make_sim_refs.sh city_manhattan_sublane`.
Compare: see `city_plano/CASE.md`.
