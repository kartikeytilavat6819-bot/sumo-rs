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
| reroute `/ref` | SKIP (no input.osm) | **PASS** | `--end 200 --additional-files …reroute.add.xml` |
| reroute `/ref_closing` | SKIP (no input.osm) | **PASS** | `sim_closing.args`: closing rerouter only |
| reroute `/ref_device` | SKIP (no input.osm) | **FAIL** | `sim_device.args`: `<param>`-form rerouting device. Diff is exactly `devices`: ref has `routing_v1`, we do not. FCD matches, so the traffic is identical — the device is never created. Silent defect, caught only because `devices` is now compared. |
| vtypes | SKIP (no input.osm) | **FAIL** (new) | `--end 120 --additional-files tools/oracle/cases/vtypes/vtypes.add.xml` |
| depart | — | SKIP: 15 variants, see below | none (one reference per route file) |
| netconv_edge | SKIP | SKIP (no sim refs; its own `run_all.sh` checks netconvert) | — |

The sim stage now runs **once per committed reference variant**: `ref.tripinfo.xml` uses `sim.args`,
`ref_<v>.tripinfo.xml` uses `sim_<v>.args`. That is what brought `reroute/ref_closing` (the
closing-rerouter-only variant) onto the board; it was previously measured by nothing and now PASSes.

Exact totals: **`run_all.sh` PASS=12 FAIL=3 SKIP=6**. The three fails are `real_dense/ref` (`v15`
arrival speed), `vtypes/ref` (EIDM timeLoss 0.03) and `reroute/ref_device` (rerouting device never
created). Before the Euler fix: PASS=11 FAIL=3. Before the variant/coverage work: **PASS=8 FAIL=2 SKIP=0**
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
| signal | **PASS** | **PASS** | now exact: the t=34 divergence was explicit-vs-semi-implicit Euler, not RNG |
| stress | PASS | PASS | new |
| reroute `/ref` and `/ref_closing` | PASS | PASS | new; both variants exact, see below |
| vtypes | CLOSE | — | `EIDM_1` timeLoss 4.05 vs 4.02; arrival now matches |

## Newly visible cases: first differences

- **signal: now exact.** The divergence at t=34 was `semi_implicit_euler_update()` in `ms_vehicle.rs`
  hardcoded to `false` instead of reading `MSGlobals` (C++ default: true). Explicit Euler integrates
  position from the old speed, so every vehicle drifted slightly and the car-following model amplified it
  to 0.26 m by t=34. The dawdle draw *looked* wrong because the speed history was already wrong; the RNG
  stream was never the cause. tripinfo, FCD and statistics now all MATCH.
- **stress**: no difference. Tripinfo, FCD and statistics all match the C++ reference exactly. `stress/REPORT.md`
  (first diff `s24` depart 43 vs 48, 107 vs 93 arrivals) does not reproduce with the current binary. The prebuilt
  binary from Nov 15 22:52 also matches, so the report predates both builds.
- **reroute**: no difference. The `--additional-files` panic in `reroute/REPORT.md` no longer reproduces. The case
  still does not test rerouting: the C++ reference shows `rerouteNo=0` for every vehicle because SUMO 1.28 ignores
  `device.rerouting.*` given as vType attributes. Not measured: `ref_closing.*` (closing-only variant).
- **vtypes: CLOSE.** `EIDM_1` arrival now matches; remaining `timeLoss 4.05 vs 4.02` and small `Wiedemann_0/1`
  timeLoss deltas (0.02/0.01). Not a flag mismatch: make_sim_refs.sh reproduces the committed reference from
  C++ with the same flags.
- **reroute**: both variants exact (`ref` and `ref_closing`). The case does not exercise rerouting: the C++
  reference shows `rerouteNo=0` for every vehicle because SUMO 1.28 ignores `device.rerouting.*` given as
  vType attributes — a genuine test-design limitation, recorded rather than papered over.
- **depart**: 15/15. The two failures in `depart/REPORT.md` (`departLane=1` silently dropping `v3`, `departSpeed=0`
  arriving 1 s late) no longer reproduce; the current binary errors on `departLane=1` exactly like C++.

## References

Nothing regenerated. `make_sim_refs.sh` (now reads every case's `sim.args`) re-derived the committed sim references
from C++ sumo 1.28.0 with the documented flags into a scratch dir: tripinfo, FCD and statistics MATCH for junction,
roundabout, signal, real, real_dense, stress, reroute; tripinfo MATCH for vtypes.

## What is left, in order

1. `real_dense` `v15` arrival speed 5.29 vs 7.6 — the only exact-comparator FAIL (agent op02). Diverges at
   t=98 on internal lane `:42428297_1_3`, +0.22 m/s inherited from `v09`; we accelerate 0.2–0.5 m/s faster
   on junction-internal lanes.
2. Stale MSGlobals accessors: 38 local `fn g_*()` across 18 files. Four proven wrong and fixed:
   `g_ignore_junction_blocker` (0), `g_using_internal_lanes` (false in `ms_full_export.rs`, true elsewhere),
   `g_compute_lc` (false), `semi_implicit_euler_update` (false — this one alone made `signal` exact).
   This is the highest-yield bug class in the port; finish the sweep (agent op01).
3. `vtypes` `EIDM_1` timeLoss 4.05 vs 4.02 — CLOSE, not exact (agent vt01).
4. `libsumo` API surface: 102 `panic!("shortcut:")` bodies, mostly wiring (agent ls01).
5. Router/rail/intermodal stubs: 164, mostly off the car path (agent rt02).
6. `devices/` stubs: 24 (agent rng01).
7. UB writes through `&self`: ~20 remaining, release-only risk (agent ub01).
8. Reroute: a rerouting case needs `<param key="device.rerouting.probability" value="1"/>` form; the current
   case proves the rerouter *loads* but never proves rerouting happens (`rerouteNo=0` on both sides).

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
