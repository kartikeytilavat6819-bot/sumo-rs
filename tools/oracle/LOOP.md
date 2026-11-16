# Self-improving verification loop

One command measures everything:

```bash
bash tools/oracle/run_all.sh --build
```

It runs every case in `tools/oracle/cases/` through the Rust binaries and prints a scoreboard:

```
junction     net  PASS
junction     sim  FAIL
real         net  PASS
real         sim  PASS
...
PASS=6 FAIL=4 SKIP=0
```

`net` = `compare_net.py` against the C++-generated `ref_simple.net.xml`.
`sim` = `compare_sim --tripinfo`, `compare_sim --fcd`, `compare_stat` against the committed references.
A panic is reported as `sim-PANIC` with the first message line.

## The loop

1. Run `run_all.sh --build`, pick the FIRST failing row.
2. Reproduce just that case; find the cause by reading the Rust against `C:\Users\karti\sumo-src\src`.
3. Fix the root cause, not the symptom. Add a `#[cfg(test)]` test pinning the wrong value.
4. Re-run `run_all.sh`. The failing row must flip to PASS and no PASS may flip to FAIL.
5. Commit; repeat from step 1.

## Rules that were learned the hard way

- **A stub returning a permissive default is the dominant bug class.** `ignoreRed → true`,
  `get_insertion_checks → NONE`, `link_bits_test → false`, `get_brutto_occupancy → 0`. They never
  crash; traffic just flows too freely. Grep single-value bodies against the C++.
- **Preserve C++ short-circuit order.** `isStopped() && myStops.begin()->…` must not pre-evaluate
  `front().unwrap()`.
- **`cargo check` is not `cargo build`.** A stale binary silently passes/fails the wrong thing.
- **Commit before merging** — the merge script takes the branch tip, not the worktree.
- **Generated references must be committed**; a reference that only exists on one machine is not a
  reference. Use `make_sim_refs.sh` / `make_net_ref.sh`.
- **Never trust a pass you cannot reproduce from a clean checkout.**

## Known failing rows and their owners (2026-11-15)

| Row | Symptom | Owner |
|---|---|---|
| junction sim | v0 arrival 18 s vs 20 s; fcd diverges t=8 | y01 / gap02 |
| roundabout sim | waitingTime 6 s vs 369 s under load | y01 |
| real_dense sim | 24 of 38 vehicles arrive; waitingTime 0 | fx02 |
| signal sim | vehicles ignore red at t=87 | tl04 |
| (any `--additional-files`) | panic `XMLSubSys::runParser not wired` | afp01 |
| (release build) | segfault on net load | rel01 |
| malformed OSM | panic instead of error | fx03 |
