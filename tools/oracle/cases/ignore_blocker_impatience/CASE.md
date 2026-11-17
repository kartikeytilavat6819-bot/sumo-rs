# ignore_blocker_impatience

Priority junction `j` with a through road (`M0`→`M1`) and a side road (`S0`→`S1`). `M1` holds a stopped vehicle
`st` (150 m, 60 s), so vehicles that enter the junction on `M1` queue there.

- Side vehicles `s1`, `s4` have `impatience=1.0`, `s2` has `impatience=0.0`, `s3` default.
- Sim: `--end 200 --ignore-junction-blocker 5 --time-to-impatience 20`.
- Reference: `make_sim_refs.sh ignore_blocker_impatience` (pristine C++ 1.28.0). Net: `net_ref.sh`.
