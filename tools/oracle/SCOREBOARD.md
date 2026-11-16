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
`ref.tripinfo.xml` exist; it appends `cases/<case>/sim.args` to the sumo call. The net stage runs only with
`input.osm`, and appends `cases/<case>/net.args` to the netconvert call (that is how `lefthand` gets
`--lefthand`; without it the mirrored net can never match).

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

Exact totals: **`run_all.sh` PASS=28 FAIL=1 SKIP=9** over 19 cases. The single failure is a **recorded
behavioural divergence**, not a broken test:

| Row | Symptom | Cause |
|---|---|---|
| `reroute_bypass_p1/ref` | with `device.rerouting.period=1`, C++ **reverts** the detour and sends the vehicle back onto the closed edge (`rerouteNo=2`, arrival 53.00); we keep the bypass (61.00) | C++'s periodic device reroute replans **without** the rerouter's prohibitions, while a rerouter-triggered reroute honours them. Ours propagates the prohibition to both paths |

`reroute_bypass` (same net, `period=1000`) PASSES and proves a genuine mid-route detour:
`LEAD IN AB OUT` → `LEAD IN AC CB OUT`, `rerouteNo=1`, matching C++ on tripinfo, FCD and statistics. Its net
is built from plain XML (`net.args`) and is re-verified against C++ netconvert on every sweep: 0 differences
in 42 elements.

**Every other row matches C++ 1.28.0 on tripinfo, FCD and statistics.** A zero in the FAIL column would not
mean zero untested surface — see the coverage-gap table below.

- **`reroute_alt`** — `v0` rerouted at t=0 to `10 → 12 → 13`, arrival 30.00
- **`closed_edge`** — a lane closed **after** departure; `v0` rerouted off it at t=8
- **`real/ref_reroute`** — mid-route closure on the Times Square network; C++ `rerouteNo=1` for `v0`, ours
  matches on tripinfo, FCD and statistics. Note the limit: the closed edge is `v0`'s destination, so SUMO
  terminates its route rather than detouring — the Times Square extracts have no bypass. The mechanism fires
  at real-network scale; a genuine detour is still unproven.

Getting rerouting to work required **four** missing-dispatch fixes, any one of which silently disabled the
feature:
1. the rerouter was never registered as a move reminder (`register_at`),
2. its `MSMoveReminder` vtable was never installed, so `notify_enter` went to the base no-op,
3. `setPermissions` had no scheduler (C++ uses an unported `WrappingCommand`),
4. `replace_route_edges` called the base `replace_route`, skipping `MSVehicle`'s best-lane refresh.

`lanechange` and `lefthand` PASS, so the `gComputeLC`/`change_lanes` fix and left-hand-traffic net generation
(`--lefthand` via `net.args`) are proven rather than asserted. `sublanes` passes too, after `ms_edge.rs` was found
to hardcode a second copy of the lateral-resolution global.

Before this change: **PASS=8 FAIL=2 SKIP=0**
(measured with the original script on the same binaries; stress, reroute, vtypes, ref_closing and depart were invisible).
Gate: all 8 previous PASS rows still PASS.

### depart (`cases/depart/run_all_variants.sh`)

15 variants (`lane_*`, `pos_*`, `speed_*`), each with its own C++ reference in `depart/ref/<variant>/`:
**PASS=15 FAIL=0** (`lane_1` and `pos_max` pass by error parity: both sides exit with the same `Error:` line).
Outputs go to `/c/Users/karti/Temp/score/depart`, not the tracked `ours/` directory.

The 15 references were **re-derived from C++ sumo 1.28.0 on 2026-11-16** into a scratch directory and compared
with the committed ones using the same comparators: tripinfo MATCH for all 15, statistics MATCH for all 15,
and the `Error:` text matches for the two error-parity variants. So the 15/15 verdict measures genuine C++
output, not a stale reference. (Do not validate them with a raw `diff` — the `<configuration>` block records
the input paths, so re-deriving from a different directory always differs there.)

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

- **netconv_edge: every generated network now matches.** The harness compares the two nets with
  `compare_net.py` (element level — a raw diff always differs because the header carries the generation
  stamp and the recorded input/output paths). Result: `net=MATCH` on all 7 cases that produce a network,
  `net=skip` on the 3 that error on both sides, and `net=MATCH` on the three `real` option variants
  (`real_base`, `real_noturn`, `real_corner3`) — 1165 elements, Times Square OSM.
  Both runners were pointing at another agent's worktree (`agent-rb06`) and were repointed to derive paths
  from their own location.
- **netconvert message text still differs** (owner mp01): C++ aggregates repeated messages —
  `Warning: 6 total messages of type: Could not build program '%' for traffic light '%'` — while we print
  each occurrence; and one warning is written without a trailing newline, so ours reads
  `...it will not be build.Success.` with the run-completion text glued on. Both are output-text fidelity,
  not network correctness.

  Warning **counts** already agree: with a working `SUMO_HOME` (copy `data/{xsd,typemap}` out of WSL and point
  the variable at it — a UNC path to the WSL filesystem does not work) both sides emit **15** warnings on
  `real/input.osm`, and our net still matches C++ exactly (0 differences, 1165 elements). Running the Windows
  exe from WSL without a usable `SUMO_HOME` adds two `Environment variable SUMO_HOME is not set` lines; those
  are artefacts, not defects. With `SUMO_HOME` set the XML-validation warning disappears and the typemap is
  loaded from file (the `Discarding unknown compound ...` lines only come from typemap processing).
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
2. Stale MSGlobals accessors: 38 local `fn g_*()` across 18 files; 18 were stale/wrong, 12 fixed.
   Proven wrong and fixed: `g_ignore_junction_blocker` (0), `g_lateral_resolution` (-1, which through the
   shared prelude also reached `ms_link.rs` so sublane foe checks never ran), `g_using_internal_lanes`
   (false in `ms_full_export.rs`), `mslcm_sl2015`'s left-hand flag (always off), the taxi device's own copy
   of the taxi-classes value (separate from the router's, started at 0).
   **`gComputeLC` is NOT default-true** — C++ sets it true only while lane changes are being applied and back
   to false afterwards. `MSEdgeControl::change_lanes` never set it, so it was permanently false and lane
   changing never ran the C++ path. Corrected 2026-11-16; do not "fix" it to a constant `true`.
   This is the highest-yield bug class in the port; 6 stale accessors remain in files owned by other agents.
3. `vtypes` `EIDM_1` timeLoss 4.05 vs 4.02 — CLOSE, not exact (agent vt01).
4. `libsumo` API surface: 102 `panic!("shortcut:")` bodies, mostly wiring (agent ls01).
5. Router/rail/intermodal stubs: 164, mostly off the car path (agent rt02).
6. `devices/` stubs: 24 (agent rng01).
7. UB writes through `&self`: ~20 remaining, release-only risk (agent ub01).
8. Reroute: a rerouting case needs `<param key="device.rerouting.probability" value="1"/>` form; the current
   case proves the rerouter *loads* but never proves rerouting happens (`rerouteNo=0` on both sides).

## Coverage gaps that are known, not accidental

| Gap | Why it matters | What would cover it |
|---|---|---|
| **closed-edge enforcement is never exercised** | every closure in our cases exists only as a *router prohibition*; SUMO never holds a vehicle for it, so "can a vehicle drive through a closed edge" has never been tested. A closure with `disallow="all"` on an edge a vehicle is already committed to would test it. | a new case: `keepClear`/`disallow="all"` closure on an edge an inserted vehicle must traverse |
| **meso** | `--mesosim` is a separate engine (see the loud-unsupported table) | would need the meso port first |
| **rail** | rail signal constraints need a `dynamic_cast` audit; no case uses a rail network | a small rail net + `--railsignal-moving-block` |
| **TraCI writes** | the 32-probe harness exercises `setSpeed`; `changeLane`, `moveToXY`, route setting and subscriptions are unmeasured | extend `cases/traci_basic/client.py` |
| **`depart` variants** | 15/15 pass, but they are compared to references that are only re-derived on demand | re-run `run_ref.sh` periodically |

## Known unsupported features — loud, not silentMeasured on 2026-11-16, so nobody has to rediscover them:

| Feature | Our behaviour | C++ | Status |
|---|---|---|---|
| `--mesosim true` | **hard error**: `mesoscopic simulation is not wired into MSNet yet` (`ms_net.rs:401`) | runs (5 tripinfo entries on the junction case) | meso is not ported. It fails loudly, which is the correct behaviour for a missing subsystem — a silent fallback to microsim would be worse. The eight `g_use_meso_sim() -> false` stubs are consistent with this: meso can never be reached. |
| `--lateral-resolution` | crashed (index out of bounds) | runs | being fixed by sl01 — this one is a *bug*, not a missing feature |
| emissions output | `shortcut:` panic on the typed getters | implements `utils/emissions` (~16 k lines) | unported module, documented |
| `saveState`/`loadState` | panic | works | `MSStateHandler` being ported by sh01 |
| CCH routing | `shortcut:` (needs RoutingKit types) | works | unported dependency |

Reproduce the meso result: run our `sumo.exe` on any case with `--mesosim true`; it exits 1 with that message.
C++ on the same case exits 0. Meso is a separate simulation mode, so it does not affect the microsim path that
every other row on this page measures.

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
