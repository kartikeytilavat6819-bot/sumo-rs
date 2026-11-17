# fringe_maxveh

Four-arm priority junction `j` whose outer nodes `w`, `e`, `s`, `n` are `fringe="outer"`.
Eleven cars depart in a burst at t=0–10 s on two routes (`EW0 EW1`, `NS0 NS1`).

- Sim: `--end 200 --max-num-vehicles 3`: insertion is delayed so at most 3 vehicles are in the network.
- Reference: `make_sim_refs.sh fringe_maxveh` (pristine C++ 1.28.0). Net: `net_ref.sh`.
