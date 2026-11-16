references come ONLY from ~/sumo-pristine; ~/sumo-src is for instrumentation only

# Handover — current state, main `c599e28`, 2026-11-16

Read `tools/oracle/SCOREBOARD.md` for the case table, the known gaps and the Kaggle gate.
Read `tools/oracle/LOOP.md` for the procedure.

## Verified state

Lead's run on main (this worktree did not re-run the harness; the x07 binary is older than main's sources):

```
run_all.sh        PASS=71 FAIL=4 SKIP=13
depart            15/15 variants
FAIL rows         city_munich net (nc03); reroute_alt/ref_astar, ref_ch, ref_chwrapper (rr04 branch fixes)
```

Outputs (non-tripinfo): **4/32 pass** on `worktree-agent-out01`, not main. Branch triage: **0 useful**; never merge
`fix-a60ca9556cae1357c`.

TraCI counts disagree across docs (58/62 and 44/62). Treat both as stale until `TRAci_STATUS.md` is re-measured.

## The bug classes that mattered (read this before debugging anything)

1. **A local `fn g_*()` shadowing a real global with a hardcoded value.** Six found:
   `g_ignore_junction_blocker`=0, `g_using_internal_lanes`=false, `g_compute_lc`=false (not default-true —
   C++ sets it only while applying lane changes), `semi_implicit_euler_update`=false, `g_lateral_resolution`=-1
   (prelude) and `G_LATERAL_RESOLUTION`=-1 (a *second copy* in `ms_edge.rs`). None crashed; each deformed
   the simulation.
2. **A hardcoded constant standing in for an option**: `waiting-time-memory` 300 s instead of the option's
   100 s; the version string lacking `v…-0000000000`.
3. **A ported component nothing called**: `schedule_initial_switch`, `registerParent` (rerouter children),
   the rerouter's move reminder, the pans.
4. **No virtual dispatch** — C++ virtual calls through a base pointer are silent no-ops here. **Seven**
   instances found: `schedule_initial_switch` (traffic lights never switched), `registerParent` (rerouter
   children), the rerouter's `MSMoveReminder` vtable, `MSLaneChanger::change` (sublane changes never ran),
   `VehicleStateListener` (TraCI `getLoadedNumber`), `replace_route` (best lanes never refreshed),
   `getRerouteOrigin` (a reroute that should have failed succeeded). Plus `MSDevice::device_name`
   (pure virtual, cannot be called at all) and `getDevice(typeid)` (always null). **This is the dominant
   structural weakness; `PORTING.md` has a vtable plan that is not implemented.**

## Harness lessons (each of these produced a false "PASS" tonight)

- **Disk**: per-agent `CARGO_TARGET_DIR` is *required* (see below) but each is a full dependency tree —
  93 of them reached **256 GB** and left 1.2 GB free, at which point builds start failing for reasons that
  look like code errors. Sweep `C:\Users\karti\sumo-rs-target*` (keep `sumo-rs-target`,
  `sumo-rs-target-main`, and the one you are verifying with); the 410 worktrees are only 9.4 GB by comparison.
  `merge_wave2_main.sh` now warns below 5 GB free.
- `[ -f input.osm ] || continue` hid four cases from the scoreboard.
- The script measured `main` from any worktree, and a shared target dir mixed binaries across trees — six
  false readings. **Each agent gets its own `CARGO_TARGET_DIR`**; `run_all.sh` prints `STALE BINARY:` if the
  binary predates any `.rs`.
- Concurrent runs wrote to one output dir and overwrote each other mid-comparison → `SCORE_OUT`/`run-$$`.
- References regenerated with different flags than the harness runs look like product bugs: `real_dense`
  (`--end 125`) and `vtypes` (missing `--fcd-output`) both wasted hours.
- `compare_sim.py` ignored the `devices` attribute, hiding a missing routing device.
- **Stale case reports**: several `REPORT.md` verdicts (closed_edge, sublanes, zipper, reroute_alt `ref`, detectors)
  disagree with the lead's main run. Re-measure before acting on any of them.

## What is left

Taken from the lead's list, in this order:

1. The 4 FAIL rows above. `city_munich` net (nc03); `reroute_alt` stack overflow and shortcut panics (rr04 branch).
2. Outputs: 22 FAIL and 6 PANIC of 32 (`OUTPUTS.md` on `worktree-agent-out01`; owners ou02, dv02, cf02, pd01).
   The worst is silent empty output: queue, netstate, vehroute, full, amitran.
3. `battery` and `devices` never built (dv02).
4. OSM import requires `--simple-projection` (pj01, UTM).
5. Car-following: W99 `cc1`, KraussPS, CACC (cf03).
6. `left_before_right` junction (jt01).
7. TraCI (`tools/oracle/TRAci_STATUS.md`); loadState (st01); meso (ms01, scoping); rail (rb01); persons (pd01); emissions (em01).
8. `reroute_bypass_p1`: CASE.md says expected FAIL, the lead's run does not list it. Verify.
9. `reroute/ref_device` (`routing_v1` device never created) and `real_dense` `v15` (arrival 5.29 vs 7.6, op02):
   both FAIL in older docs, not in the lead's list. Re-measure.
10. Depart CRLF fix (`worktree-agent-px01`): needed for the Linux gate to pass lane_1 and pos_max.

## Fleet protocol that produced results

One owner per directory; fresh branch from `main` (signature drift broke two merges otherwise); commit every
few changes; own target dir; gate = build + full sweep + depart 15/15 + unit tests, verified **on the merged
tree**, not on the branch. Escalate to Opus for design problems (trait-bound scope, systems debugging), keep
Haiku for mechanical porting. A failing new case is a success; never regenerate a reference to make a case
pass.
