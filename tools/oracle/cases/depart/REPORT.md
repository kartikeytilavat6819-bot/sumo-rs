# Departure / insertion matrix (junction net)

Net: `tools/oracle/cases/junction/ref_simple.net.xml`. Routes: the five vehicles from
`cases/junction/routes.rou` (r0 x2, r1, r2, r3), departs 0..4, each vehicle carrying one departure
attribute. Run: `--seed 42 --end 120 --no-step-log`.

- Reference: C++ SUMO v1.28.0 in WSL (`run_ref.sh`).
- Ours: prebuilt Rust `sumo.exe` (`run_ours.ps1`).
- Compare: `compare_sim.py --tripinfo` and `compare_stat.py` on `statistic.xml`.

## Results

| Variant | Ours (arrivals, first diff) | Reference (arrivals) | Verdict |
|---|---|---|---|
| departPos=base | 5, none | 5 | PASS |
| departPos=5 | 5, none | 5 | PASS |
| departPos=free | 5, none | 5 | PASS |
| departPos=random | 5, none | 5 | PASS |
| departPos=max | 0, exit 1: `Invalid departPos definition for vehicle 'v0'` | 0, same error, exit 1 | PASS (error parity; `max` is not a valid SUMO value) |
| departLane=first | 5, none | 5 | PASS |
| departLane=best | 5, none | 5 | PASS |
| departLane=allowed | 5, none | 5 | PASS |
| departLane=random | 5, none | 5 | PASS |
| departLane=1 (explicit index) | 4 (v3 silently dropped, exit 0), no error | 0, exit 1: `Invalid departLane definition for vehicle 'v3'.` | FAIL |
| departSpeed=0 | 5, v0 arrival 25.00 | 5, v0 arrival 26.00 | FAIL |
| departSpeed=max | 5, none | 5 | PASS |
| departSpeed=desired | 5, none | 5 | PASS |
| departSpeed=speedLimit | 5, none | 5 | PASS |
| departSpeed=last | 5, none | 5 | PASS |

## Findings

- **No panics.** Every ours run exited 0, except `departPos=max`, which exited 1 with the same
  message as the reference.
- **departLane=1 on a one-lane edge (FAIL).** The reference aborts the run. Ours runs to completion,
  drops `v3` without any message, and reports 4 arrivals. The validation path is
  `has_valid_route_start` in `crates/sumo_core/src/microsim/ms_vehicle.rs:2786`. Its
  `Invalid departLane definition for vehicle '…'.` branch at line 2793 never fires for this case.
  I did not trace why `v3` is dropped silently.
- **departSpeed=0 (FAIL).** Only `v0` differs, arriving at 26.00 (ref) vs 25.00 (ours). Totals
  differ by 1 s (`totalTravelTime` 113 vs 112). Cause not investigated.
- **Random modes match exactly** (`departPos=random`, `departLane=random`), so the RNG stream is
  shared with the reference.

## Summary

- 13 of 15 variants pass (`departPos=max` counts as error parity).
- Broken modes:
  1. `departLane=<index>` where the index is out of range for the start edge: ours inserts the other
     vehicles and drops the bad one silently; the reference aborts.
  2. `departSpeed=0`: a 1 s arrival difference on `v0`.
- `departLane` takes an index, not a lane id, so the "explicit lane id" variant was run as index `1`.
  A lane id such as `-103224#0_1` is not valid SUMO syntax, so it was not tested.

## Notes

- Ours' `stderr.txt` files include a PowerShell `NativeCommandError` wrapper around the SUMO_HOME
  warning. That is the harness, not a SUMO message.
