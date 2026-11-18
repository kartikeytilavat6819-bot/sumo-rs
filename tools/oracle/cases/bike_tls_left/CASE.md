# bike_tls_left

Bicycles making left turns at a signalized four-arm junction (`C` type
`traffic_light`), with conflicting straight-going cars on the other arms.
Tests bicycle gap acceptance, waiting at red and the left-turn path through
the junction; the internal-link FCD geometry and tripinfo durations are the
hooks.

- Net: plain XML node/edge/connection files, see `net.args`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_tls_left`.
