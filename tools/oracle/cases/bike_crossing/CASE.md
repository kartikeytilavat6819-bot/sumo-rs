# bike_crossing

Bicycles crossing straight through a signalized four-arm intersection while
perpendicular car platoons cross on the conflicting signal phase. Tests the
bicycle crossing path and signal timing through the junction; FCD internal-link
positions and tripinfo waiting times are the hooks.

- Net: plain XML node/edge/connection files, see `net.args`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_crossing`.
