references come ONLY from ~/sumo-pristine; ~/sumo-src is for instrumentation only

# Self-improving verification loop

One command measures everything:

```bash
bash tools/oracle/run_all.sh --build
```

The authoritative full-gate measurement runs on the VM: `powershell -File tools/gcloud/vm_gate.ps1`
(see `SCOREBOARD.md` for the latest result).

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

Measured by the VM gate (`tools/gcloud/vm_gate.ps1`) on main at `fbc6f780` (branch `f9476459`):
**PASS=350 FAIL=7 SKIP=63; depart 15/15; duarouter 16/16; unit tests ok.** Only these seven rows fail.
All seven fail at the sim stage; their net stages PASS. Full verdicts and SKIP reasons: `SCOREBOARD.md`.

| Row | Owner |
|---|---|
| city_delft_campus_crossings/ref sim (car8 arrival 387 vs 388) | cd01 |
| city_grenoble_tram_bus/ref sim (L1f.0 devices field) | cd01 |
| city_florence_pedestrians/ref sim (car12 depart 72 vs 73) | fp02 |
| city_german_level_crossing/ref sim (car81 arrivalSpeed 7.65 vs 7.90) | rx01 |
| city_phoenix_scale_grid/ref sim-PANIC (mslcm_lc2013.rs) | px01 |
| jmparams/ref sim (fa.10 arrivalSpeed 12.72 vs 11.61) | jm01 |
| routedist/ref sim-PANIC (route 'rd' for flow 'fDist' not known) | jm01 |

Fixed since the table below was written, do **not** reopen:

- the nine rows that failed on 933e39f6 (busride, paris_etoile, portland_actuated, containers, rerouter_parking,
  taxi x2, tls_actuated_loops, tls_waut) — all PASS on fbc6f780

- junction `v0` 18 s vs 20 s — stale local `g_ignore_junction_blocker()` stub returning 0
- roundabout `v1` release 15 s early — same mechanism
- `--additional-files` panic `XMLSubSys::runParser not wired` — loads and runs
- release-build segfault — `Ptr::new(self).m()` UB writes converted to `Cell`
- malformed OSM panic — errors and exits 1 like C++
- traffic lights never switching — `schedule_initial_switch` never called; also was missing for
  actuated / delay_based / NEMA / SOTL / swarm / deterministic / wave / off
- signal t=34 divergence — was `semi_implicit_euler_update()` hardcoded `false`; the RNG-stream
  hypothesis was investigated and eliminated, do not repeat it
- `reroute/ref_device` — the routing device was never created because `init_devices` in
  `ms_base_vehicle.rs` built only tripinfo and FCD. Fixed; the device is now present but its
  *behaviour* is still inert, which `reroute_alt` now measures
- `stress` "107 vs 93 arrivals", `depart` "15 variants failing", `reroute` both variants panicking —
  all three were stale reports; each now measures exact or 15/15 PASS
- the "556 `todo!()`" figure — all of it is prose in `GENERATED SKELETON` file headers. Real `todo!()`
  calls: **0**. Use the counter the merge script prints, not a raw grep.

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
