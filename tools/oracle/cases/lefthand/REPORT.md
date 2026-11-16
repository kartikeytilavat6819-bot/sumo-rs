# Left-hand — verdict (2026-11-16)

| Stage | Verdict | Detail |
|---|---|---|
| net | FAIL (harness) | `run_all.sh` netconvert lacks `--lefthand`; mirrored shapes on lanes `100_0`/`100_1` and junctions `1`/`2` |
| sim (`ref`) | **PASS** | tripinfo MATCH, fcd MATCH, stat 0 differences |

Lane change in left-hand mode (C++, matched by ours): `b1` 100_0 → 100_1 at t=7.00, back to 100_0 at t=29.00.
4 of 4 vehicles arrive in the reference.

The sim stage passing is only meaningful because the reference net carries `lefthand="true"` and ours reads it
(`nl_handler.rs:635`).
