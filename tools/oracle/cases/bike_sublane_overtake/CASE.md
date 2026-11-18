# bike_sublane_overtake

Sublane model at `--lateral-resolution 0.8` on a 2-lane, 400 m edge. Narrow
bicycles (`bike0..bike3`) overtake crawling passenger cars (`slow0`, `slow1`,
maxSpeed 2 m/s) by shifting laterally within the same lane. FCD lateral
positions and speeds measure the overtake.

- Net: plain XML (2-lane edge `E0`), see `net.args`.
- Sim: `--end 150 --lateral-resolution 0.8`. Refs: `make_sim_refs.sh bike_sublane_overtake`.
