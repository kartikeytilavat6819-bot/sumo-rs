# real_dense: extra tripinfo entry at horizon 120 s

## Finding

At `--end 120` ours wrote 39 tripinfo entries, the reference 38. The extra id is **v43**, not v23
(v23 is missing from both sides, so it is not part of the difference).

- Insertion is identical: both sides insert all 48 vehicles, and v43 enters lane `682360554#0_3` at t=54.00 on both.
- Reference stat at 120 s: `loaded=48 inserted=48 running=10`. Ours: `48 / 48 / 9`.
- The reference FCD still has v43 on its final lane (`226041026#1_0`, pos 6.90) at t=119.
- The reference reaches v43's arrival at **t=120.00**, one step after the horizon. That is confirmed by a 125 s reference run, which lists v43 with `arrival="120.00"`.
- Ours arrives at 119.00 because it is about 2.3 m ahead of the reference by t=118 (movement divergence that already shows in FCD x at t=40).

Cause: a horizon artefact. v43 finishes one step earlier in ours than in the reference, so it lands
just inside the `--end 120` cut on our side only. This is a timing difference, not an extra insertion.

## Fix

The case horizon is moved to `--end 125` on both sides, and the reference (tripinfo, stat, fcd) is
regenerated at 125 s. At 125 s both sides have the same 40 arrived vehicles and no ours-only or ref-only ids.
The next boundary vehicle (v27) was not visible at 120 s, so the horizon is a moving target: any
fixed end time can cut some vehicle. 125 s is chosen because the id sets match there.

## Remaining (not horizon)

With the horizon fixed, acceptance no longer reports id differences. It still reports FAIL on tripinfo:
`arrivalSpeed` differs by more than 0.5 m/s for v21 (4.80 vs 5.59), v25 (11.27 vs 12.01), v31 (15.96 vs 15.30).
Statistics are CLOSE (3 aggregate groups within 1 %). These come from the movement divergence and are
the open work for this case, not the horizon.
