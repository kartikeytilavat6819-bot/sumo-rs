# Signal case: t=34 FCD divergence

## Conclusion

The first FCD divergence at t=34 (v1 entering the minor-link branch of the Krauss
model) was **not** an RNG problem. It was explicit-vs-semi-implicit Euler integration:
`semi_implicit_euler_update()` in `ms_vehicle.rs` was hardcoded to `false`, while the C++
default (`MSGlobals::gSemiImplicitEulerUpdate`) is `true`. Explicit Euler drifted positions
slightly (v1 was 0.26 m off at t=34), and the car-following model amplified that. The dawdle
draw looked different only because its speed history was different.

Reported by the agent that applied the fix, measured on the merged tree:

- signal tripinfo: MATCH
- signal fcd: MATCH
- signal statistics: PASS
- acceptance: `signal tripinfo=PASS statistics=PASS => PASS`

This branch does not contain that fix; it lands with the merge that carries it.

## Hypothesis eliminated: RNG stream

Do not repeat this investigation.

- Instrumented `RandHelper::rand(SumoRNG*)` in the C++ build and `SumoRNG::rand` in the
  Rust port (env-gated, temporary). Both sides produced 91 draws over 0-40 s, identical
  (rng id, count, value) for every draw up to step 34000 (86 draws).
- The first difference was one extra draw on `lanes_15` at step 35000, from
  `dawdle2` via `patch_speed_before_lc`. The following `lanes_7` draw had the same value
  on both sides. That extra call followed from the diverged vehicle state, not from a
  different seed, RNG object, or draw order.
- Seeding matches: `thread-rngs` 64, lane RNG index `numericalID % 64`, pool seeded
  `seed + i`, default seed 23423 on both sides.

## Process lessons

- When a formula and its draw both match but the result still differs, check the state
  the formula is fed (here the integration method) before the random stream.
- The shared `sumo-rs-target` directory is overwritten by other agents' builds. A
  `cargo build` can also reuse a stale `utils` rlib when a source file's mtime is older
  than the rlib. Touch the edited file and copy the binary out right after the build, then
  verify the copy contains the change before trusting a run.
