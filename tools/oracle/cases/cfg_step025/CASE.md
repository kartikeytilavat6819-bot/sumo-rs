# cfg_step025

Processing option `--step-length 0.25`: a quarter-second simulation step changes fcd sampling, departures and the statistic timestep count. Existing cases cover 0.1 and 0.5; this covers 0.25. One-lane 600 m edge `A`.

Sim: `--end 60 --step-length 0.25`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_step025; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
