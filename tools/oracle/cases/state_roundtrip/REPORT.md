# State round trip: `--save-state` / `--load-state`

Cases: `junction`, `signal`. Save at t=10 (`--save-state.times 10`), load in a fresh process (`--load-state`).
Script: `roundtrip.sh <bin_dir> <out_dir>` runs straight, save and load, then `compare.py`.
Kaggle run: `tools/kaggle/roundtrip_st01.ps1` (kernel `tools/kaggle/roundtrip_kernel_st01.py`), commit `7578b6c0`.
Reference: pristine C++ outputs committed in `cpp/` (produced with `~/sumo-pristine/bin/sumo`).

## Results

| check | junction | signal |
|---|---|---|
| ours load == C++ load, vehicles running at t=10 | IDENTICAL (5 vehicles) | IDENTICAL (4 vehicles) |
| ours load == ours straight, same vehicles | 4 of 5 differ (`timeLoss`, `arrivalSpeed`, `v0` arrival 19 vs 20 s) | 3 of 4 differ (`timeLoss`, `arrivalSpeed`) |
| C++ load == C++ straight (reference, same vehicles) | same 4 of 5 differ | same 3 of 4 differ |
| ours state file == C++ state file (header and `<configuration>` ignored) | DIFFERENT: 12 lines | DIFFERENT: 18 lines |

The straight-vs-load drift is in the reference too. The C++ save run itself is identical to its straight run apart from output paths.
So the acceptance bar for the load is C++ parity, not identity with the straight run.

## Open differences

1. **Save side (state file):** our state writes `<approaching>` entries under `<lane>`/`<link>` that the C++ state does not contain. Loading still gives C++-identical tripinfo here, so the extra entries are not visible in these two runs. The cause is not yet traced.
2. **Load after save:** the `loadState` path is the C++ one for the handler, lane, link, approaching, TLS and `updateBestLanes` restores. Device and reminder restore covers tripinfo devices only (see `ms_state_handler.rs`).
3. Not run: the standard gate on the final commit (see the commit message), and meso.

## Reproduce

```
bash tools/oracle/cases/state_roundtrip/roundtrip.sh <dir with sumo> /tmp/rt
```
