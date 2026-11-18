# rail_double_track

Two physically separate one-way tracks 50 m apart: the north track runs eastbound
(`wN - n1 - n2 - eN`) and the south track westbound (`eS - s2 - s1 - wS`). Each track
has `rail_signal` junctions at its ends and dead-end terminals, all edges
`allow="rail" spreadType="center"`.

Four trains: two eastbound on the north track (t=0, t=20) and two westbound on the
south track (t=5, t=25). The opposing trains never share a block, so unlike
`rail_single_meet` and `rail` no train should be held by a rail signal; this checks
rail-signal driveway computation on a double-track layout with independent tracks.

Sim: `--end 300 --time-to-teleport -1`.
