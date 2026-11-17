# train_join

Train coupling on a 700 m single-track line `T0 T1`: leader `lead` (rail, 50 m) stops at 150 m for 30 s;
`follow` stops at 130 m with `triggered="join" join="lead"` and couples to it. A tram (`rail_urban`, 25 m) runs at t=20.

- Split is not exercised (1.28 stop attribute `split` needs a coupled train; not in this minimal case).
- Sim: `--end 200 --railway.max-train-length 80`. Reference: `make_sim_refs.sh train_join` (pristine C++ 1.28.0). Net: `net_ref.sh`.
