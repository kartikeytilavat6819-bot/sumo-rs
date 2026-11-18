# bike_rerouting

Bicycles with the rerouting device (`--device.rerouting.probability 1`,
`--device.rerouting.bike-speeds`) on a network with a direct path `AB BD DE`
and a detour `AB BC CD DE`. A rerouter on the approach `AB` closes `BD` at
t=10, forcing bicycles to reroute onto the detour while keeping destination
`DE`. Tripinfo `rerouteNo` and FCD paths are the hooks.

- Net: plain XML node/edge files, see `net.args`.
- Sim: `--end 200 ... --additional-files bike_rerouting.add.xml`.
- Refs: `make_sim_refs.sh bike_rerouting`.
