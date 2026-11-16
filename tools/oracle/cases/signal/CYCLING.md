# Traffic-light phase cycling: robustness run

Reference: C++ SUMO 1.28.0 (`$HOME/sumo-src/bin/sumo`, WSL).
Candidate: Rust `C:\Users\karti\sumo-rs-target-main\debug\sumo.exe` (prebuilt, not rebuilt).

Net: `ref.net.xml` used for BOTH sides (the case has no `net.xml`; candidate netconvert not exercised).
Signal: one `static` tlLogic `1802245009`, offset 0, phases `GGGGg` 82 s, `yyyyy` 3 s, `rrrrr` 5 s (cycle 90 s).
Common flags: `--precision 17 --seed 42 --end 900 --no-step-log`, plus `--tripinfo-output`, `--fcd-output`, `--statistic-output`.

## 1. `--tls-state-output`

Not a SUMO option. Both binaries reject it:
- Rust: `Error: called Result::unwrap() on an Err value: Process("Could not parse commandline options.")`
- C++: `Switch or parameter name expected. Error: Could not parse commandline options.`

Phase observation therefore used `--save-state.times 50,84,86,88,92,100,140`.

## 2. Runs (end 900)

| Load | Routes | Ref arrivals | Cand arrivals |
|---|---|---|---|
| base (5 veh, depart 0-4) | `routes.rou` | 5 | 5 |
| load20 (20 veh, depart 0-57) | generated | 20 | 20 |
| queue (20 veh, depart 75-151) | generated | 20 | 20 |
| multi (20 veh, depart 0-855, 45 s apart) | generated | 20 | 20 |

All four Rust runs and reference runs exit 0. Rust prints a SUMO_HOME warning on stderr (env var unset on Windows; XML validation disabled).

## 3. Comparisons

- **Tripinfo** (`compare_sim.py --tripinfo`): first diff is `departSpeed` of v0 (`10.5356...` vs `10.54`) in every run. Candidate ignores `--precision 17` and prints about 2 decimals.
- **Per-vehicle arrival** (own script, rounded): base 0 s total diff; multi 0 s; load20 1 s (v17); queue 2 s (v2, 94 vs 92).
- **Stats** (`compare_stat.py`): differences in `speed`, `routeLength`, `duration`, `timeLoss`, `totalTravelTime`; all rounding except queue/load20 `totalTravelTime` (361 vs 359, 364 vs 363), which follow the arrival diffs.
- **FCD** (`compare_sim.py --fcd`): first divergence in base at `t=0.000` on v0 `x` (`39.22005` vs `39.22`, precision again). Real behaviour divergence in queue at **t=87 s, v2**: ref 10.99 m/s entering `:335683858_4_0`; cand 11.83 m/s.

## 4. Does the light switch / does it hold vehicles?

Reference (from `--save-state` `tlLogic` elements):

| t (s) | phase | until (ms) | meaning |
|---|---|---|---|
| 50 | 0 | 82000 | green |
| 84 | 1 | 85000 | yellow |
| 86 | 2 | 90000 | red |
| 88 | 2 | 90000 | red |
| 92 | 0 | 172000 | green (cycle 2) |
| 100 | 0 | 172000 | green |
| 140 | 0 | 172000 | green |

The reference cycles correctly. Reference vehicle v2 brakes 11.1 → 3.5 m/s between t=86 and t=89 for the red.

Candidate:
- `--save-state.times` writes `st_50.00.xml.gz` (note the name differs from the reference's `st_50.000`) and **panics** on the next write:
  `shortcut: the global OptionsCont (getOptions) is not ported yet, so the vehicle parameter write cannot run` (`crates/sumo_core/src/microsim/ms_base_vehicle.rs:2997`). Exit 1.
- The candidate's snapshot contains **no `tlLogic` element**, so the phase cannot be read directly.
- Plain runs complete, and vehicles do pass the junction, so the light is not stuck. But v2 does **not** brake for the red (11.8 m/s through t=87-89) and the same vehicle arrives 2 s early.
- Zero vehicles stop at the light in any run on either side (`waitingTime` 0 everywhere), so the queue load did not produce a queue, even on the reference.

## Verdict

**Not verified; the candidate does not reproduce reference signal behaviour.**
- Reference: cycles correctly (green → yellow → red → green, 90 s).
- Candidate: no crash in plain 900 s runs, but its vehicles ignore the red at t=85-90 (v2), so its phases differ from the reference. The save-state path **crashes** on an unported `OptionsCont` shortcut, which also blocks reading phases.
- Not checked: whether the candidate's tlLogic cycles at all (no state output available); the approach-lane controlled-link mapping; the multi load's red windows (no vehicle stops on the reference either).

## Scratch

Inputs and outputs in `C:\Users\karti\AppData\Local\Temp\opencode\rb04\` (`routes20.rou`, `routes_queue.rou`, `routes_multi.rou`, and `<load>/{ref,cand}/`).
