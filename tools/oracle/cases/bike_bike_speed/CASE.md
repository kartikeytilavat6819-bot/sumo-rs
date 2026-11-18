# bike_bike_speed

Bicycles on a road whose edge type restricts use to `bicycle` and caps speed
at 5 m/s (`bikeway`), next to a passenger-only road at 13.89 m/s (`carroad`),
via a netconvert `--type-files` type map. Bikes and cars both have maxSpeed
above the edge limit, so FCD speeds and arrival times expose the per-edge
speed cap and the vClass permission.

- Net: plain XML node/edge/type files, see `net.args`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_bike_speed`.
