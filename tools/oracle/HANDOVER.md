# Overnight session state — 2026-11-16 (final)

Read `tools/oracle/SCOREBOARD.md` for the numbers and `tools/oracle/LOOP.md` for the procedure.
This file is the situational picture and the handover.

## Verified state

```
run_all.sh        PASS=28 FAIL=1 SKIP=9      over 19 cases
depart            15/15 variants
netconvert        5 networks + 10 edge cases + 3 option variants + --lefthand + --tls.* + one plain-XML net: MATCH
unit tests        175 passing (2 ignored)
release == debug  identical on junction, roundabout, real, signal
TraCI             58/62 probes
```

The single failing row is a **recorded C++ quirk**, not a broken test: with `device.rerouting.period=1`, C++
reverts a detour and sends the vehicle back onto the closed edge (`rerouteNo=2`), because its periodic device
reroute replans without the rerouter's prohibitions. Ours keeps the bypass. The periodic reroute is
implemented on branch `worktree-agent-pr01`; merging it fixes that row but breaks `reroute_alt` on a
Dijkstra tie-break, which is why it is not merged.

See the coverage-gap table in `SCOREBOARD.md` before treating the board as "done": rerouting has no
multi-vehicle contention case, meso and rail have no cases, TraCI is 58/62, and `gCheckRoutes` is still a
`shortcut:` treated as false.

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

## What is left

1. `reroute/ref_device`'s one-step offset (quantified above).
2. **Rerouting is only exercised on tiny networks.** `reroute_alt`/`closed_edge` prove the mechanism; no
   case exercises a reroute on the real Times Square network.
3. **TraCI: 44/62.** Failing/unknown: subscription + arrival crash, `setRoute` index-out-of-bounds at
   `ms_base_vehicle.rs:2581`, `changeTarget` (needs the router), `vehicle.add`, `getJunctionFoes` (`LinkBits`
   is `Unmapped`). Subscriptions are what a Unity client polls vehicles with.
4. `has_device()` panics for any device whose `device_name` is not wired (pure virtual, no vtable); it is on
   the parameter-output path at `ms_base_vehicle.rs:509`.
5. Genuinely blocked, documented: emissions (`utils/emissions`, ~16 k lines), CCH/RoutingKit, meso
   (loud error, engine not ported), rail constraint `dynamic_cast`, `NamedRTree`, `saveState`'s `loadState`.

## Fleet protocol that produced results

One owner per directory; fresh branch from `main` (signature drift broke two merges otherwise); commit every
few changes; own target dir; gate = build + full sweep + depart 15/15 + unit tests, verified **on the merged
tree**, not on the branch. Escalate to Opus for design problems (trait-bound scope, systems debugging), keep
Haiku for mechanical porting. A failing new case is a success; never regenerate a reference to make a case
pass.
