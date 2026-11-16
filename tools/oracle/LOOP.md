# Self-improving verification loop

One command measures everything:

```bash
bash tools/oracle/run_all.sh --build
```

It runs every case in `tools/oracle/cases/` through the Rust binaries and prints a scoreboard:

```
junction/ref  sim  PASS
real/ref      sim  PASS
...
PASS=11 FAIL=3 SKIP=6
```

The example above is the shape of the output, not the current scoreboard — that lives in `SCOREBOARD.md`
and is the only file whose numbers you should trust. The sim stage now runs one row **per committed
reference variant** (`ref` uses `sim.args`, `ref_<v>` uses `sim_<v>.args`).

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

## Known failing rows and their owners

Current state, 2026-11-16 — **three** rows fail. Everything else in that old table is fixed and must not
be re-investigated:

| Row | Symptom | Owner |
|---|---|---|
| real_dense/ref sim | `v15.arrivalSpeed` 5.29 vs 7.6 | rd02 |
| signal/ref sim | RNG stream one draw out of phase; FCD first diff t=34 (0.26 m) | rng01 |
| vtypes/ref sim | `EIDM_1` arrival 77.00 vs 76.00 — one step early | vt01 |

Fixed since the table below was written, do **not** reopen:

- junction `v0` 18 s vs 20 s — stale local `g_ignore_junction_blocker()` stub returning 0
- roundabout `v1` release 15 s early — same mechanism
- `--additional-files` panic `XMLSubSys::runParser not wired` — loads and runs
- release-build segfault — `Ptr::new(self).m()` UB writes converted to `Cell`
- malformed OSM panic — errors and exits 1 like C++
- traffic lights never switching — `schedule_initial_switch` never called; also was missing for
  actuated / delay_based / NEMA / SOTL / swarm / deterministic / wave / off
- `stress` "107 vs 93 arrivals", `depart` "15 variants failing", `reroute` both variants panicking —
  all three were stale reports; each now measures exact or 15/15 PASS

<details>
<summary>Original table (2026-11-15, historical)</summary>

| Row | Symptom | Owner |
|---|---|---|
| junction sim | v0 arrival 18 s vs 20 s; fcd diverges t=8 | y01 / gap02 |
| roundabout sim | waitingTime 6 s vs 369 s under load | y01 |
| real_dense sim | 24 of 38 vehicles arrive; waitingTime 0 | fx02 |
| signal sim | vehicles ignore red at t=87 | tl04 |
| (any `--additional-files`) | panic `XMLSubSys::runParser not wired` | afp01 |
| (release build) | segfault on net load | rel01 |
| malformed OSM | panic instead of error | fx03 |

</details>
