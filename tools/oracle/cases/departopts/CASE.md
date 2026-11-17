# departopts

Single 400 m lane `D`. Eight vehicles bunch at depart 5.00 / 5.25 / 8.40 s (some with explicit `departPos`), plus flow `fl` every 0.5 s from 10 to 30 s. Insertion is contested on the lane.

- `ref` (`sim.args`): `--max-depart-delay 4 --eager-insert` (vehicles waiting over 4 s are skipped).
- `ref_extrap` (`sim_extrap.args`): `--extrapolate-departpos`.
- `ref_rdo` (`sim_rdo.args`): `--random-depart-offset 3 --seed 5`.
- `ref_sloppy` (`sim_sloppy.args`): `--sloppy-insert` (deprecated, prints a warning; output should match the default).

Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
