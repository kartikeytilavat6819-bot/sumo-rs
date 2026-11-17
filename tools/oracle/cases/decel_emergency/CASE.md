# decel_emergency

Braking deviations on one 800 m lane: `l0` and `l1` stop on the lane, followers brake behind them.

- `wimpy` (decel 1.0, emergencyDecel 9, apparentDecel 1.5): must emergency-brake behind the hard stop of `l0`.
- `bad` (emergencyDecel 2 < decel 4.5): C++ warns and raises emergencyDecel to decel at load time.
- `hard` (decel 6, apparentDecel 6, emergencyDecel 9): the stopping leader `l1`.
- `normal` (default-like, decel 4.5): `f3` follows the queue.
- Sim: `--end 200`. Reference: `make_sim_refs.sh decel_emergency` (pristine C++ 1.28.0). Net: `net_ref.sh`.
