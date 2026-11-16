# Performance: sumo-rs vs C++ SUMO

Run: `tools/kaggle/remote_bench.ps1 -Branch worktree-agent-pf01` (commit 6d05bae7), Kaggle CPU notebook
(4 cores, ~31 GB RAM). Both simulators ran on the same machine, one after the other, same inputs.

- Ours: `cargo build --release -p sumo -p netconvert` (sumo v1.28.0-0000000000).
- C++: `eclipse-sumo==1.28.0` from PyPI (Eclipse SUMO sumo 1.28.0, the version the oracle references use).
- Each timed config is the median of 3 runs. RSS is the max of the 3 runs (`/usr/bin/time -v`).
- Flags: `--no-step-log true --end 3600`, no outputs (`bare`), or `--fcd-output /dev/null` (`fcd_null`).
- Scenarios: `city_plano` and `city_munich` (ref_simple.net.xml + routes.rou), `stress`, and
  `scaled_plano` (city_plano net, randomTrips.py `-p 0.5 --seed 1 -e 3600`, 7200 vehicles).

## Results

Ratio = C++ wall / ours wall. Above 1 means ours is faster.

| scenario | config | ours wall s | C++ wall s | speed-up (C++/ours) | ours steps/s | C++ steps/s | ours veh/s | C++ veh/s | ours RSS MB | C++ RSS MB |
|---|---|---|---|---|---|---|---|---|---|---|
| city_plano | bare | 0.22 | 0.38 | 1.73x | 16364 | 9474 | 682 | 395 | 20 | 76 |
| city_plano | fcd_null | 0.29 | 0.52 | 1.79x | 12414 | 6923 | 517 | 288 | 21 | 76 |
| city_munich | bare | 0.42 | 0.61 | 1.45x | 8571 | 5902 | 357 | 246 | 21 | 77 |
| city_munich | fcd_null | 0.59 | 0.89 | 1.51x | 6102 | 4045 | 254 | 169 | 22 | 77 |
| stress | bare | 0.05 | 0.11 | 2.20x | 72000 | 32727 | 2400 | 1091 | 12 | 66 |
| stress | fcd_null | 0.06 | 0.14 | 2.33x | 60000 | 25714 | 2000 | 857 | 12 | 66 |
| scaled_plano | bare | 45.26 | 54.43 | 1.20x | 79.5 | 66.1 | 68.2 | 56.7 | 79 | 130 |
| scaled_plano | fcd_null | 60.47 | 80.99 | 1.34x | 59.5 | 44.4 | 51.0 | 38.1 | 82 | 132 |

Steps/s is simulated steps per wall second (3600 steps per run). Veh/s is vehicles inserted per wall second.

### TraCI (scaled_plano, 1000 steps)

Each step: `simulationStep`, `vehicle.getIDList`, `vehicle.getPosition` for every active vehicle.
Average active vehicles: 392.

| simulator | ms/step |
|---|---|
| ours | 27.84 |
| C++ | 28.62 |

Both are about 28 ms per step. The per-vehicle `getPosition` calls from the Python client
dominate this loop, so this number mostly measures client round-trips, not the simulator.

Peak RSS: ours 12–82 MB, C++ 66–132 MB.

## Caveats

- Wall times come from `/usr/bin/time`, with 10 ms resolution. The stress runs (0.05–0.14 s) are
  within about 5–10% noise.
- The scaled_plano fcd_null run was slower than bare for ours (60 s vs 45 s). That gap is larger than
  the fcd write should cost, so run-to-run variance on the shared VM is probably large. Treat the
  scaled figures as ±20%.
- Only one machine, one Kaggle image, one seed. These are not a published benchmark.
- The city and stress runs use `--end 3600`. The oracle references use `--end 600`, so these timings
  are not the oracle runs.

## Hottest functions (ours)

Not measured. `perf` is not installed on the Kaggle image, so the profile step was skipped.

## Versions

- Ours: sumo-rs at commit 6d05bae7, release build, rustc 1.100.0 (2026-11-09 stable).
- C++: Eclipse SUMO sumo 1.28.0 (eclipse-sumo PyPI wheel).
