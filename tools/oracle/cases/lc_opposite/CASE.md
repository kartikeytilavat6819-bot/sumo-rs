# lc_opposite

Opposite-direction overtaking (`lcOpposite`, LC2013) on a two-way rural road: edges `E0` (n0→n1) and `E1` (n1→n0),
1 lane each, 600 m. Netconvert guesses the opposite pairing with `--opposites.guess true`.

- `slow0`, `slow1` (`lcOpposite="0"`, 6 m/s) on `E0` are overtaken by `fast0`, `fast1` (`lcOpposite="1"`, 20 m/s).
- `on0..on3` (15 m/s) on `E1` provide oncoming traffic.
- Sim: `--end 120`. Reference: `make_sim_refs.sh lc_opposite` (pristine C++ 1.28.0). Net: `net_ref.sh`.
