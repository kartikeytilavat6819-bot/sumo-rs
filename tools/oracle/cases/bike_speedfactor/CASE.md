# bike_speedfactor

Bicycle vTypes with `speedFactor` (1.3, 0.7, 1.0) and `speedDev` (0.05, 0.15,
0.3) plus a default, on a single 400 m edge. The fixed `--seed 42` makes the
per-vehicle speedFactor drawn from the normal distribution reproducible;
tripinfo `speedFactor` and FCD speeds measure the sampling.

- Net: plain XML (1-lane edge `E0`), see `net.args`.
- Sim: `--end 120 --seed 42`. Refs: `make_sim_refs.sh bike_speedfactor`.
