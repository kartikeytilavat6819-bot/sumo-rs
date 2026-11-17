# runtimeopts

Two 500 m edges `A B`, two lanes each. Vehicle `slow0` (4 m/s) holds lane 0; flow `fFast` (lane 0) overtakes it via lane change; flow `fFastR` runs on lane 1.

- `ref` (`sim.args`): `--lanechange.duration 3 --threads 1` (explicit single thread).
- `ref_ballistic` (`sim_ballistic.args`): `--step-method.ballistic true` (ballistic position update instead of semi-implicit Euler).
- `ref_hw` (`sim_hw.args`): `--time-to-teleport.highways 5 --emergencydecel.warning-threshold 0.5`. Edge speed 13.89 m/s is below the 69 km/h highway threshold, so the teleport option is expected to stay inert here.

Sim seed 2, `--end 120`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
