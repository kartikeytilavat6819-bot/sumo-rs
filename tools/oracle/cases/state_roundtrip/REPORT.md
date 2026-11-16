# State round trip: `--save-state` / `--load-state`

Cases: `junction`, `signal`. Save at t=10 (`--save-state.times 10`), load in a fresh process (`--load-state`).
Script: `roundtrip.sh <bin_dir> <out_dir>` runs straight, save and load, then `compare.py`.
Kaggle run: `tools/kaggle/roundtrip_st01.ps1` (kernel `tools/kaggle/roundtrip_kernel_st01.py`), commit `0c9a2c9b`.
Reference: `cpp/` holds outputs of the official SUMO 1.28.0 build (`~/sumo-pristine`, identical to an uninstrumented build from the official tarball `~/sumo-trace`).

## Results (commit `0c9a2c9b`)

| check | junction | signal |
|---|---|---|
| state file == official C++ state (header and `<configuration>` ignored) | IDENTICAL (40 lines) | IDENTICAL (41 lines) |
| ours load == official C++ load, vehicles running at t=10 | IDENTICAL (5 vehicles) | IDENTICAL (4 vehicles) |
| ours load == ours straight, same vehicles | 4 of 5 differ (`timeLoss`, `arrivalSpeed`, `v0` arrival 19 vs 20 s) | 3 of 4 differ (`timeLoss`, `arrivalSpeed`) |
| C++ load == C++ straight (reference, same vehicles) | same 4 of 5 differ | same 3 of 4 differ |

The straight-vs-load drift is present in the official C++ reference too, so the acceptance bar for loading is C++ parity, not identity with the straight run.

## Root cause of the earlier state-file difference

Our `MSLane::save_state` wrote `<approaching>` entries for internal lanes. The official 1.28.0 `MSLane::saveState` writes them only for rail-junction lanes (`saveApproaching = toRailJunction`). The internal-lane term existed only in a locally edited source tree, which the port had copied. Fixed in `ms_lane.rs`.

The live simulation was already correct: an instrumented probe showed the same approach entries in C++ at the save point, which the official rail-only save condition does not write.

## Reproduce

```
bash tools/oracle/cases/state_roundtrip/roundtrip.sh <dir with sumo> /tmp/rt
```
