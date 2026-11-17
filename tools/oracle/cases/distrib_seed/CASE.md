# distrib_seed

Distributions with a fixed seed: a `vTypeDistribution` (`vd`: 60% `vdA` with `speedFactor="normc(...)"`, 40% `vdB`)
and a `routeDistribution` (`rd`: 70% two edges `D0 D1`, 30% `D0`).

- `v0..v2` reference `type="vd" route="rd"` directly; `fi` is a 20-vehicle `<flow number>` inside `<interval 10–120>`.
- Sim: `--end 200 --seed 42`. Reference: `make_sim_refs.sh distrib_seed` (pristine C++ 1.28.0). Net: `net_ref.sh`.
