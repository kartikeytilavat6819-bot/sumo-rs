# cfg_routesteps

Processing option `--route-steps`: how far ahead routes are loaded from the route file. Two variants: `ref` (`--route-steps 0`, load everything up front) and `ref_v50` (`--route-steps 50`, incremental loading in 50 s windows). Departures are spread over 0–140 s. One-lane 600 m edge `A`.

Sim: `--end 160 --route-steps 0` / `--end 160 --route-steps 50` (`sim_v50.args`).

Compared by run_all: tripinfo, fcd and stat for both variants (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_routesteps; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
