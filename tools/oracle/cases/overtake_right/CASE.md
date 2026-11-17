# overtake_right

Three-lane motorway edge `R0` (800 m, 33.33 m/s). Trucks `slow*` run in lane 1 (middle); cars `fast*` start in lane 0 (right)
and must pass them. Overtaking on the right is only allowed with the option.

- Sim: `--end 200 --lanechange.overtake-right true`. Reference: `make_sim_refs.sh overtake_right` (pristine C++ 1.28.0).
- Net: `net_ref.sh`.
