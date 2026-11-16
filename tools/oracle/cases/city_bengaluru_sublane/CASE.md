# Case: city_bengaluru_sublane (Bengaluru MG Road block, left-hand traffic, sublane model)

Same net and routes as `city_bengaluru_blr` (`net.args` = `--lefthand`, `routes.rou`). The only difference is
`sim.args` = `--end 600 --lateral-resolution 0.8`, which enables the sublane lane-change model.

Reference: C++ sumo 1.28.0 from `~/sumo-pristine` (WSL), via `tools/oracle/make_sim_refs.sh city_bengaluru_sublane`.
Compare: see `city_plano/CASE.md`.
