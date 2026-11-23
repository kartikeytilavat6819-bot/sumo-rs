# sumo-rs

A work-in-progress port of [Eclipse SUMO](https://github.com/eclipse-sumo/sumo) (Simulation of Urban MObility) from C++ to Rust.

The goal is a faithful port: same inputs, same outputs, checked against SUMO's own test suite.

## Status

See [PORTING.md](PORTING.md) for what has been ported. Run `python tools/progress.py` for a progress bar (C++ lines ported, per module).

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
