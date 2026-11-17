# Release build: parity and wall-clock speed

Measured on the Kaggle/VM gate (Linux), commit `f29591b4` plus the PERF commit.

## Release parity

`run_all.sh` with `PROFILE=release` (release binaries from `cargo build --release`) against the same
scoreboard run in debug by the gate:

- Release-only differences found and fixed: `closed_edge/ref`, `reroute/ref`, `reroute/ref_closing`
  panicked in release at `ms_net.rs` `get_router_tt` (`myRouterTT[&rngIndex]`, key missing).
  Cause: undefined behaviour. The method takes `&self`, writes the map through `Ptr::new(self)`,
  and the optimiser assumes the write cannot happen. Fix: `myRouterTT`, `myPedestrianRouter`,
  `myIntermodalRouter` are now `RefCell`s (same class as `UB_AUDIT.md`). After the fix all three pass in release.
- Same verdict in debug and release (not fixed here): `city_paris_dense/ref` sim FAIL (MATCH),
  `peds_crossing/ref` sim-PANIC in `transportables/msp_model.rs`.
- Depart variants: 15/15 PASS in both.
- Scoreboard after the fix: release PASS=172 FAIL=2 SKIP=20, identical to debug.

## Timing (5 largest city cases by `ref_simple.net.xml` size)

Command as in `run_all.sh` (`--no-step-log true`, outputs enabled, `sim.args`). Wall seconds,
median of 3 runs. Ours = release build, pristine = C++ sumo 1.28.0.

| Case | Net bytes | Ours (release) | Pristine (C++) | Ours / pristine |
|---|---:|---:|---:|---:|
| city_atlanta_motorway | 2832840 | 0.39 | 0.67 | 0.58 |
| city_paris_dense | 2116228 | 0.37 | 0.62 | 0.60 |
| city_milton_keynes | 1972252 | 0.34 | 0.60 | 0.57 |
| city_la_interchange | 1970005 | 0.32 | 0.55 | 0.58 |
| city_tokyo_lefthand | 1896533 | 0.32 | 0.56 | 0.57 |

Ours is faster than pristine on all five; none is above the 3x threshold, so no profile was taken.
Caveat: this is one VM slot, and the sims are short (600 s simulated time).

## Scale cases (1800 s, 3000-4000 vehicles)

Sim only (`sim.args`, `--no-step-log true`, no FCD), median of 3 runs, release build on the gate VM, C++ 1.28.0 = `$SUMO_PRISTINE_BIN`.

| Case | Vehicles | Ours (release) | Pristine (C++) | Ours / pristine | Ours output vs ref |
|---|---:|---:|---:|---:|---|
| city_vienna_scale_downtown | 3001 | 12.77 s | 15.99 s | 0.79 | matches (544 trips, 323 teleports) |
| city_phoenix_scale_grid | 3273 | 37.94 s | 41.22 s | 0.92 | DIFFERS (1134 trips, 239 teleports vs ref 925 / 331): timing is of a diverging run |
| city_dhaka_scale_jam | 4000 | 34.70 s | 34.43 s | 1.00 | matches (797 trips, 662 teleports) |

Phoenix in the debug gate panics at `mslcm_lc2013.rs` (see gate row). The release binary runs to the end but its output differs, so its wall time is not a like-for-like number.
Ours is not faster than C++ on these cases (Dhaka is level, Phoenix is 8% faster on a diverging run), unlike the 600 s city cases above.
Caveat: one VM slot, and the Dhaka timing was measured alone, so it does not share a slot with the other two runs.
