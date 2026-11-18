# rail_single_meet

Single-track line `w - r1 - r2 - e` (no passing loop). `r1` and `r2` are `rail_signal`
junctions, `w`/`e` are dead ends, and every edge is `allow="rail" spreadType="center"`
so the eastbound and westbound edges pair as bidirectional driveways.

`e1` departs from the west at t=0, `w1` from the east at t=3. With no loop the two
trains cannot pass: one must be held at a rail signal until the other clears the
single-track block. This exercises rail-signal block occupancy on a single track
(the existing `rail` case always offers a passing loop).

Sim: `--end 300 --time-to-teleport -1`.
