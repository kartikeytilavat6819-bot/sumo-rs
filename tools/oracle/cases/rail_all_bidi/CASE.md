# rail_all_bidi

Only eastbound edges are declared in the edge file (`w_r1 r1_r2 r2_e`); netconvert's
`--railway.topology.all-bidi` generates the westbound bidi counterparts named
`-w_r1 -r1_r2 -r2_e`. `r1` and `r2` are `rail_signal` junctions, `w`/`e` are dead ends.

`e1` runs east at t=0 and `w1` west on the generated bidi edges at t=4. They share a
single bidirectional track with no loop, so rail-signal driveways must hold one train
while the other occupies the block. This differs from `rail_bidi_platoon`, which
declares both directions by hand and has a passing loop.

Sim: `--end 300 --time-to-teleport -1`.
