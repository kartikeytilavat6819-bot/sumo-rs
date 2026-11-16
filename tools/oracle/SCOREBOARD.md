# Scoreboard — current state

Re-measured 2026-11-16 with the harness that now runs every sim case. Binaries: `cargo build -p sumo -p netconvert`
from this branch (code = main `c4dcf59`, harness-only change), shared target `/c/Users/karti/sumo-rs-target`.
Reproduce with:

```bash
CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target bash tools/oracle/run_all.sh          # exact comparator
bash tools/oracle/cases/depart/run_all_variants.sh                                    # 15 depart variants
python tools/oracle/acceptance.py <case> <cand.ti.xml> <cand.stat.xml>                # tolerance: PASS/CLOSE/FAIL
```

`run_all.sh` runs the checkout it lives in. The sim stage runs whenever `ref_simple.net.xml`, `routes.rou` and
`ref.tripinfo.xml` exist; it appends `cases/<case>/sim.args` to the sumo call. The net stage runs only with `input.osm`.

## Exact comparator (`run_all.sh`)

| case | net | sim | flags (`sim.args`) |
|---|---|---|---|
| junction | PASS | PASS | — |
| roundabout | PASS | PASS | — |
| real (Times Square) | PASS | PASS — byte-identical | — |
| real_dense | PASS | FAIL — 1 tripinfo diff, FCD diff (see below) | `--end 125` |
| signal | PASS | FAIL — 1 tripinfo diff, FCD first diff t=34 | — |
| stress | SKIP (no input.osm) | **PASS** (new) | `--end 200` |
| reroute `/ref` | SKIP (no input.osm) | **PASS** (new) | `--end 200 --additional-files tools/oracle/cases/reroute/reroute.add.xml` |
| reroute `/ref_closing` | SKIP (no input.osm) | **PASS** (new) | `sim_closing.args`: closing rerouter only |
| vtypes | SKIP (no input.osm) | **FAIL** (new) | `--end 120 --additional-files tools/oracle/cases/vtypes/vtypes.add.xml` |
| depart | — | SKIP: 15 variants, see below | none (one reference per route file) |
| netconv_edge | SKIP | SKIP (no sim refs; its own `run_all.sh` checks netconvert) | — |

The sim stage now runs **once per committed reference variant**: `ref.tripinfo.xml` uses `sim.args`,
`ref_<v>.tripinfo.xml` uses `sim_<v>.args`. That is what brought `reroute/ref_closing` (the
closing-rerouter-only variant) onto the board; it was previously measured by nothing and now PASSes.

Exact totals: **`run_all.sh` PASS=11 FAIL=3 SKIP=6**. Before this change: **PASS=8 FAIL=2 SKIP=0**
(measured with the original script on the same binaries; stress, reroute, vtypes, ref_closing and depart were invisible).
Gate: all 8 previous PASS rows still PASS.

### depart (`cases/depart/run_all_variants.sh`)

15 variants (`lane_*`, `pos_*`, `speed_*`), each with its own C++ reference in `depart/ref/<variant>/`:
**PASS=15 FAIL=0** (`lane_1` and `pos_max` pass by error parity: both sides exit with the same `Error:` line).
Outputs go to `/c/Users/karti/Temp/score/depart`, not the tracked `ours/` directory. The C++ references were not re-derived in this run.

## Product gate (`acceptance.py`)

| case | tripinfo | statistics | note |
|---|---|---|---|
| junction | PASS | PASS | |
| roundabout | PASS | PASS | |
| real | PASS | PASS | |
| real_dense | FAIL | CLOSE | `v15.arrivalSpeed` 5.29 vs 7.6; stat diffs 0.03 |
| signal | CLOSE | CLOSE | unchanged from previous measurement (`v1` timeLoss, FCD t=34) |
| stress | PASS | PASS | new |
| reroute | PASS | PASS | new; does not exercise rerouting (see below) |
| vtypes | CLOSE | — | new; no statistics reference |

## Newly visible cases: first differences

- **stress**: no difference. Tripinfo, FCD and statistics all match the C++ reference exactly. `stress/REPORT.md`
  (first diff `s24` depart 43 vs 48, 107 vs 93 arrivals) does not reproduce with the current binary. The prebuilt
  binary from Nov 15 22:52 also matches, so the report predates both builds.
- **reroute**: no difference. The `--additional-files` panic in `reroute/REPORT.md` no longer reproduces. The case
  still does not test rerouting: the C++ reference shows `rerouteNo=0` for every vehicle because SUMO 1.28 ignores
  `device.rerouting.*` given as vType attributes. Not measured: `ref_closing.*` (closing-only variant).
- **vtypes: FAIL.** First difference `EIDM_1 arrival` ref 77.00 vs ours 76.00. Full list (checked with a
  non-stopping script): `EIDM_1` arrival 77 vs 76, duration 27 vs 26, timeLoss 4.05 vs 3.74, arrivalSpeed
  13.89 vs 14.04; `Wiedemann_0` timeLoss 1.31 vs 1.33; `Wiedemann_1` timeLoss 1.46 vs 1.45. The other 7 vehicles
  match. Not a flag mismatch: make_sim_refs.sh reproduces the committed reference from C++ with the same flags.
- **depart**: 15/15. The two failures in `depart/REPORT.md` (`departLane=1` silently dropping `v3`, `departSpeed=0`
  arriving 1 s late) no longer reproduce; the current binary errors on `departLane=1` exactly like C++.

## References

Nothing regenerated. `make_sim_refs.sh` (now reads every case's `sim.args`) re-derived the committed sim references
from C++ sumo 1.28.0 with the documented flags into a scratch dir: tripinfo, FCD and statistics MATCH for junction,
roundabout, signal, real, real_dense, stress, reroute; tripinfo MATCH for vtypes.

## What is left, in order

1. `vtypes` `EIDM_1` arrives 1 s early (the one new FAIL). Extended-IDM path; no `vtypes`-specific owner yet.
2. `real_dense` `v15` arrival speed (agent rd02).
3. `signal` t=34: RNG stream one draw out of phase (agent rng01).
4. Stale MSGlobals accessors: 38 local `fn g_*()` across 18 files, 6 fixed. Proven wrong:
   `g_using_internal_lanes()` = false in `ms_full_export.rs` (true elsewhere), `g_compute_lc()` = false
   in `mscf_model.rs` (C++ default true) (agent op01).
5. `libsumo` API surface: 102 `panic!("shortcut:")` bodies, mostly wiring (agent ls01).
6. Router/rail/intermodal stubs: 164, mostly off the car path (agent rt02).
7. UB writes through `&self`: ~20 sites, release-only risk (agent ub01).
8. Reroute: a rerouting case needs `<param key="device.rerouting.probability" value="1"/>` form; not yet tested.

## Traps that have cost real time here

- **Stale binary.** Shared target dir gets overwritten by other agents. Build before trusting a run.
- **Stale case reports.** `REPORT.md` files record the binary they were run with. `stress`, `reroute`, `vtypes` and
  `depart` reports describe failures and panics that the current binary does not show. Re-run before acting on them.
- **Harness measured the wrong tree.** `run_all.sh` used to `cd` to the main checkout and a fixed target dir, so a
  worktree's scoreboard measured main's code. It now runs its own checkout; set `CARGO_TARGET_DIR` to the build under test.
- **Silently skipped cases.** A `[ -f input.osm ] || continue` guard hid four cases from the scoreboard. Every case
  now prints a row, including SKIPs with their reason.
- **Stub returning a permissive default.** `g_*()` copied into a file with a hardcoded permissive default
  (`0`/`false`) instead of calling `ms_globals`: no panic, no warning, traffic just flows too freely.
  This was the junction defect.
- **Ported but never called.** `schedule_initial_switch` was a perfect translation that nothing invoked.
- **Reference generated with different flags than the harness runs.** `real_dense`'s reference needs
  `--end 125`; without it the id sets disagree and it looks like an extra vehicle. `make_sim_refs.sh` now reads
  `sim.args`, so the two cannot drift.
