# sumo-rs

A port of [Eclipse SUMO](https://github.com/eclipse-sumo/sumo) (Simulation of Urban MObility) 1.28.0 from C++ to Rust.

The goal is a faithful port: same inputs, same outputs, checked against SUMO's own test suite.

## Status

Verified against the official SUMO **1.28.0** binary on its own test suite (full run of 2026-11-24, commit `acf684f05`):
every case that a faithful port can reproduce gives identical output.

| Suite | Result |
|---|---|
| sumo (TextTest cases) | all pass except 6 documented in NOT_PORTED.md section 3 |
| netconvert | 1,589 / 1,590 (the remaining case is skipped by the harness) |
| duarouter | 559 / 559 |
| dfrouter, jtrrouter, marouter, od2trips, activitygen, netgenerate, polyconvert | all pass |
| TraCI | 609 / 609 |
| complex TraCI scripts | 461 / 463 (the other 2 are documented in NOT_PORTED.md) |

See [PORTING.md](PORTING.md) for what has been ported and `python tools/progress.py` for per-module progress.

## Performance vs the official C++ binary

One-hour scenarios on generated grid networks (official `netgenerate` + `randomTrips.py`/`duarouter`, seed 42),
Rust release build vs official sumo 1.28.0, same arguments, on an idle 16-vCPU / 39 GB VM. Time = mean of 3 runs,
peak memory = max of 3 runs. Trip and statistics outputs were byte-identical in every scenario.

| Scenario | Vehicles | C++ time | Rust time | C++ peak memory | Rust peak memory |
|---|---|---|---|---|---|
| 15x15 grid, micro | 3,600 | 12.6 s | 12.4 s | 92 MB | 70 MB |
| 15x15 grid, micro | 12,000 | 60.4 s | 57.3 s | 102 MB | 114 MB |
| 25x25 grid, micro | 18,001 | 196.1 s | 206.3 s | 194 MB | 228 MB |
| 25x25 grid, meso (`--mesosim`) | 18,001 | 4.3 s | 3.8 s | 182 MB | 159 MB |

In short: same results, about the same speed (within roughly 5-13% either way) and similar memory. On very short runs
(under a second) Rust starts faster, because the C++ binary loads 73 shared libraries and the Rust binary 5.
The scenario generator and benchmark script are `tools/bench/bigbench.sh`.

## Not ported

A few parts are deliberately not ported (the JuPedSim pedestrian model, XSD schema validation) and some official cases cannot be reproduced because the official binary itself crashes on them. See [NOT_PORTED.md](NOT_PORTED.md) for the list, the affected test cases and how to add them later.

## Build and test

```sh
cargo build --release
cargo test
python tests/run_tests.py basic/simple_nets   # run SUMO's texttest cases against the Rust binary
```

`tests/run_tests.py` expects a checkout of SUMO next to this repo at `../sumo-src` (or set `SUMO_SRC`).

## License

Derived from Eclipse SUMO, so it is licensed the same way: EPL-2.0 OR GPL-2.0-or-later. See [LICENSE](LICENSE).
