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

On five sample scenarios (up to ~3,000 vehicles) the Rust release build produced byte-identical trip and statistics
output and ran 1.2x-3.6x faster than the official binary. See [PORTING.md](PORTING.md) for what has been ported and
`python tools/progress.py` for per-module progress.

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
