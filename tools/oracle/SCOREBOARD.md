# Scoreboard — current state

Re-measured after the junction fix (`g_ignore_junction_blocker` stale stub) and the
traffic-light initial-switch fix. Reproduce with:

```bash
bash tools/oracle/run_all.sh            # exact comparator: any diff = FAIL
python tools/oracle/acceptance.py <case> <cand.ti.xml> <cand.stat.xml>   # tolerance: PASS/CLOSE/FAIL
```

`run_all.sh` reads an optional `cases/<case>/sim.args` and appends it to the sumo call
(used by `real_dense` for `--end 125`).

## Exact comparator (`run_all.sh`)

| case | net | sim |
|---|---|---|
| junction | PASS | **PASS** (was FAIL: `v0` 18.00 s vs 20.00) |
| roundabout | PASS | **PASS** (was FAIL: `v1` 20.00 vs 35.00 + teleport warning) |
| real (Times Square) | PASS | PASS — byte-identical |
| real_dense | PASS | FAIL — 1 tripinfo diff, 1 FCD diff (see below) |
| signal | PASS | FAIL — 1 tripinfo diff, FCD first diff t=34 |

Exact totals: **PASS=8 FAIL=2** (was PASS=6 FAIL=4).

## Product gate (`acceptance.py`)

| case | tripinfo | statistics | note |
|---|---|---|---|
| junction | PASS | PASS | |
| roundabout | PASS | PASS | |
| real | PASS | PASS | |
| real_dense | FAIL | CLOSE | `v15.arrivalSpeed` 5.29 vs 7.6; stat diffs are 0.03 |
| signal | CLOSE | CLOSE | `v1` timeLoss 3.99 vs 4.35; FCD 0.26 m at t=34 |

## What is left, in order

1. `real_dense` `v15` arrival speed (agent rd02).
2. `signal` t=34: RNG stream one draw out of phase (agent rng01).
3. Stale MSGlobals accessors: 38 local `fn g_*()` across 18 files, 6 fixed. Proven wrong:
   `g_using_internal_lanes()` = false in `ms_full_export.rs` (true elsewhere), `g_compute_lc()` = false
   in `mscf_model.rs` (C++ default true) (agent op01).
4. `libsumo` API surface: 102 `panic!("shortcut:")` bodies, mostly wiring (agent ls01).
5. Router/rail/intermodal stubs: 164, mostly off the car path (agent rt02).
6. UB writes through `&self`: ~20 sites, release-only risk (agent ub01).

## Traps that have cost real time here

- **Stale binary.** Shared target dir gets overwritten by other agents. Build before trusting a run.
- **Stale local global stub.** `g_*()` copied into a file with a hardcoded permissive default
  (`0`/`false`) instead of calling `ms_globals`: no panic, no warning, traffic just flows too freely.
  This was the junction defect.
- **Ported but never called.** `schedule_initial_switch` was a perfect translation that nothing invoked.
- **Reference generated with different flags than the harness runs.** `real_dense`'s reference needs
  `--end 125`; without it the id sets disagree and it looks like an extra vehicle.
