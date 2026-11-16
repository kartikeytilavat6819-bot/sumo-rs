# sigma_seed — RNG stream parity: dawdling (`sigma`) and speedFactor draws under `--seed`

## Net and case

Chain `n0 → n1 → n2`, edges `E0`, `E1` (1000 m each, 1 lane, 13.89 m/s), built from plain XML (`net.args`).

- `routes.rou`: one `vType id="passenger"` with `sigma="0.5"` (default dawdling, stated explicitly),
  `accel=2.6 decel=4.5 length=5 minGap=2.5 maxSpeed=20`.
- Ten vehicles `s0..s9`, type `passenger`, route `E0 E1`, depart every 2 s from t=0, `departSpeed="max"`.
  The 2 s spacing puts them in car-following, so the dawdle draw affects the speed profile.
- Two seeds, each with its own C++ reference:
  - `sim.args`: `--end 200 --seed 42` → `ref.*`
  - `sim_seed7.args`: `--end 200 --seed 7` → `ref_seed7.*`
  - `sim_ref.sh` takes `seed7` as an argument to produce the second set.

## Proof the RNG is exercised (C++ 1.28.0)

- **speedFactor draws** (per vehicle, from the RNG): `departSpeed` is `13.89 × speedFactor` and differs by seed.
  seed 42: `s0` 13.17, `s1` 14.15, `s2` 15.11, `s3` 12.24. seed 7: `s0` 15.37, `s1` 13.82, `s2` 14.88, `s3` 14.27.
- **Dawdling draws** (sigma 0.5): distinct FCD speeds over the run are 348 (seed 42) and 438 (seed 7), so the
  car-following speeds vary. Minimum FCD speed is 9.27 / 10.22 m/s. Neither run is a constant-speed profile.
- **Seed sensitivity:** our seed-42 tripinfo compared against the seed-7 reference gives `DIFF: vehicle s0 field
  departSpeed: ref='15.37' cand='13.17'`. So the comparator distinguishes the two seeds and a match on the right seed
  is not vacuous.

## Verdict (ours, `sumo.exe` from `sumo-rs-target-x07/debug`)

**PASS for both seeds.** No panic, exit 0.

| run | tripinfo | FCD | statistics |
|---|---|---|---|
| `--seed 42` (`ref.*`) | MATCH | MATCH | 0 differences (9 elements) |
| `--seed 7` (`ref_seed7.*`) | MATCH | MATCH | 0 differences (9 elements) |

So our RNG stream (speedFactor draws and dawdle draws) matches C++ for both seeds on this case.

## Reproduce

- Net: `bash …/sigma_seed/net_ref.sh` (WSL)
- Refs: `bash …/sigma_seed/sim_ref.sh` and `bash …/sigma_seed/sim_ref.sh seed7` (WSL)
- Ours: `sumo.exe -n ref_simple.net.xml -r routes.rou --tripinfo-output … --fcd-output … --statistic-output … --no-step-log true --end 200 --seed 42` (and `--seed 7`)
- Compare: `python tools/oracle/compare_sim.py --tripinfo|--fcd …`, `python tools/oracle/compare_stat.py ref.stat.xml ours.stat.xml`
