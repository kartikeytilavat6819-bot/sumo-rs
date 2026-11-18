# rail_portion_split

Portion working — the split half. `main` (100 m) stops at busStop `B` with
`split="part"`; `part` (50 m) has `depart="split" departPos="last"` and is inserted
at the back of `main` when the split happens (matching the official SUMO 1.28
portion-working test pattern). The existing `rail_portion_join` / `train_join`
cases only exercise coupling, not splitting.

Sim: `--end 300 --time-to-teleport -1 --additional-files .../split.add.xml`.
